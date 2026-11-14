//! I009a disposable QEMU prototype: minimal reaping PID 1 and independent
//! non-root session. Not a complete service manager, released rootfs or OS.
#![forbid(unsafe_code)]

use nix::errno::Errno;
use nix::mount::{MsFlags, mount};
use nix::sys::signal::{Signal, kill};
use nix::sys::wait::{WaitPidFlag, WaitStatus, waitpid};
use nix::unistd::{Gid, Pid, Uid, chown, geteuid, setsid};
use signal_hook::{consts::signal::{SIGINT, SIGTERM}, flag};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::os::unix::fs::MetadataExt;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::{Duration, Instant};

const SESSION_BINARY: &str = "/vita/vita-interaction-prototype";
const INIT_BINARY: &str = "/init";
const VITA_UID: u32 = 1000;
const VITA_GID: u32 = 1000;
const MAX_LAUNCHES: u32 = 3;
const WINDOW: Duration = Duration::from_secs(60);
const POLL: Duration = Duration::from_millis(50);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Fixture {
    crash_session: bool,
    test_orphan: bool,
    spawn_failure: bool,
    fatal_failure: bool,
    test_active_shutdown: bool,
}

fn parse_boot_options(cmdline: &str) -> io::Result<Fixture> {
    let mut fixture = Fixture::default();
    let mut crash_seen = false;
    let mut orphan_seen = false;
    let mut spawn_failure_seen = false;
    let mut fatal_seen = false;
    let mut active_shutdown_seen = false;
    let mut recovery_seen = false;
    for word in cmdline.split_ascii_whitespace() {
        if let Some(value) = word.strip_prefix("vita.recovery=") {
            if recovery_seen || value != "0" {
                return Err(io::Error::other("recovery cannot be enabled in I009a ISO"));
            }
            recovery_seen = true;
        }
        if let Some(value) = word.strip_prefix("vita.i009.test_crash=") {
            if crash_seen || value != "1" {
                return Err(io::Error::other("invalid duplicate or crash fixture token"));
            }
            crash_seen = true;
            fixture.crash_session = true;
        }
        if let Some(value) = word.strip_prefix("vita.i009.test_orphan=") {
            if orphan_seen || value != "1" {
                return Err(io::Error::other("invalid duplicate or orphan fixture token"));
            }
            orphan_seen = true;
            fixture.test_orphan = true;
        }
        if let Some(value) = word.strip_prefix("vita.i009.test_spawn_failure=") {
            if spawn_failure_seen || value != "1" {
                return Err(io::Error::other("invalid or duplicate spawn failure fixture token"));
            }
            spawn_failure_seen = true;
            fixture.spawn_failure = true;
        }
        if let Some(value) = word.strip_prefix("vita.i009.test_fatal=") {
            if fatal_seen || value != "1" {
                return Err(io::Error::other("invalid or duplicate fatal fixture token"));
            }
            fatal_seen = true;
            fixture.fatal_failure = true;
        }
        if let Some(value) = word.strip_prefix("vita.i009.test_active_shutdown=") {
            if active_shutdown_seen || value != "1" {
                return Err(io::Error::other("invalid or duplicate active shutdown fixture token"));
            }
            active_shutdown_seen = true;
            fixture.test_active_shutdown = true;
        }
    }
    if fixture.fatal_failure && !fixture.test_orphan {
        return Err(io::Error::other("fatal fixture needs an adopted orphan"));
    }
    if fixture.test_active_shutdown
        && (fixture.crash_session || fixture.spawn_failure || fixture.fatal_failure)
    {
        return Err(io::Error::other("active shutdown fixture must start a normal session"));
    }
    Ok(fixture)
}

fn serial(message: &str) {
    if let Ok(mut console) = OpenOptions::new().write(true).open("/dev/console") {
        let _ = writeln!(console, "{message}");
        let _ = console.flush();
    } else {
        eprintln!("{message}");
    }
}

fn mount_one(from: &str, path: &str, typ: &str, flags: MsFlags, options: Option<&str>) -> io::Result<()> {
    mount(Some(from), path, Some(typ), flags, options)
        .map_err(|error| io::Error::other(format!("mount {typ} at {path} failed: {error}")))
}

fn mount_runtime() -> io::Result<()> {
    let safe = MsFlags::MS_NOSUID | MsFlags::MS_NODEV;
    mount_one("proc", "/proc", "proc", safe | MsFlags::MS_NOEXEC, None)?;
    if fs::read_to_string("/proc/uptime")?.trim().is_empty() {
        return Err(io::Error::other("Linux /proc/uptime missing"));
    }
    serial("VITA_I006_GUEST_PROC_READY");
    mount_one("sysfs", "/sys", "sysfs", safe | MsFlags::MS_NOEXEC | MsFlags::MS_RDONLY, None)?;
    mount_one("tmpfs", "/run", "tmpfs", safe, Some("mode=0755,size=16m"))?;
    mount_one("tmpfs", "/tmp", "tmpfs", safe | MsFlags::MS_NOEXEC, Some("mode=1777,size=16m"))?;
    fs::create_dir_all("/run/vita")?;
    let mounted = fs::read_to_string("/proc/mounts")?;
    for (path, fstype) in [("/proc", "proc"), ("/sys", "sysfs"), ("/run", "tmpfs"), ("/tmp", "tmpfs")] {
        if !mounted.lines().any(|line| {
            let mut parts = line.split_ascii_whitespace();
            parts.next().is_some() && parts.next() == Some(path) && parts.next() == Some(fstype)
        }) {
            return Err(io::Error::other(format!("missing real {fstype} mount on {path}")));
        }
    }
    serial("VITA_I007_RUNTIME_MOUNTS_READY proc=proc sys=sysfs run=tmpfs tmp=tmpfs");
    Ok(())
}

/// Parse Linux mountinfo (ID parent device root mountpoint options ...) for
/// an actual read-only mount. Directory permissions alone are insufficient.
fn has_readonly_mount(mountinfo: &str, path: &str) -> bool {
    mountinfo.lines().any(|line| {
        let fields: Vec<_> = line.split_ascii_whitespace().collect();
        fields.len() > 6
            && fields[4] == path
            && fields[5].split(',').any(|option| option == "ro")
    })
}

fn reject_system_write() -> io::Result<()> {
    let probe = "/vita/.vita-i009b0-write-probe";
    match OpenOptions::new().write(true).create_new(true).open(probe) {
        Err(error) if error.raw_os_error() == Some(Errno::EROFS as i32) => Ok(()),
        Ok(_) => {
            // Avoid leaving a writable probe behind if the isolation failed.
            let _ = fs::remove_file(probe);
            Err(io::Error::other("security violation: /vita write was allowed"))
        }
        Err(error) => Err(io::Error::other(format!(
            "expected EROFS from immutable /vita, got {error}"
        ))),
    }
}

fn mount_immutable_vita() -> io::Result<()> {
    // In direct I009b0 initramfs boots this protects the application tree,
    // not '/'. In I009b1 stage2 the entire SquashFS root has already been
    // validated read-only; this extra bind mount preserves older contracts.
    mount(Some("/vita"), "/vita", None::<&str>, MsFlags::MS_BIND, None::<&str>)
        .map_err(|e| io::Error::other(format!("bind /vita failed: {e}")))?;
    mount(
        None::<&str>, "/vita", None::<&str>,
        MsFlags::MS_BIND | MsFlags::MS_REMOUNT | MsFlags::MS_RDONLY
            | MsFlags::MS_NOSUID | MsFlags::MS_NODEV,
        None::<&str>,
    ).map_err(|e| io::Error::other(format!("readonly remount /vita failed: {e}")))?;
    let mounts = fs::read_to_string("/proc/self/mountinfo")?;
    if !has_readonly_mount(&mounts, "/vita") {
        return Err(io::Error::other("kernel mountinfo lacks immutable /vita"));
    }
    reject_system_write()?;
    serial("VITA_I009B0_SYSTEM_MOUNT_READONLY_OK path=/vita result=EROFS");
    Ok(())
}

/// The disposable initramfs archive must never preserve host-user ownership.
/// Otherwise UID 1000 could mutate trusted OS files before or after the
/// session starts. This validates Linux's *actual* guest inode ownership.
fn verify_rootfs_ownership() -> io::Result<()> {
    for path in ["/init", "/vita", SESSION_BINARY, "/dev"] {
        let metadata = fs::metadata(path)?;
        if metadata.uid() != 0 || metadata.gid() != 0 || metadata.mode() & 0o022 != 0 {
            return Err(io::Error::other(format!(
                "unsafe rootfs ownership/permissions at {path}: uid={} gid={} mode={:o}",
                metadata.uid(), metadata.gid(), metadata.mode() & 0o7777
            )));
        }
    }
    serial("VITA_I009_ROOTFS_OWNER_OK uid=0 gid=0");
    Ok(())
}

/// Stage2 is reached via stage1's switch_root + exec at PID 1, not a new
/// process. Verify the *root mount* is genuinely SquashFS and read-only,
/// rather than accepting the I009b0 /vita-only bind-remount proof.
fn verify_stage2_immutable_root() -> io::Result<()> {
    if std::process::id() != 1 {
        return Err(io::Error::other("stage2 was not exec'd into PID 1"));
    }
    let mounts = fs::read_to_string("/proc/self/mountinfo")?;
    let found = mounts.lines().any(|line| {
        let words: Vec<_> = line.split_ascii_whitespace().collect();
        words.len() > 8 && words[4] == "/" &&
        words[5].split(',').any(|opt| opt == "ro") &&
        line.contains(" - squashfs ")
    });
    if !found {
        return Err(io::Error::other("stage2 '/' is not a read-only squashfs mount"));
    }
    match OpenOptions::new().write(true).create_new(true).open("/.vita-rootfs-write-probe") {
        Err(error) if error.raw_os_error() == Some(Errno::EROFS as i32) => {}
        Ok(_) => {
            let _ = fs::remove_file("/.vita-rootfs-write-probe");
            return Err(io::Error::other("stage2 rootfs unexpectedly writable"));
        }
        Err(error) => return Err(io::Error::other(format!(
            "unexpected root write-probe errno: {error}"
        ))),
    }
    let mounted = fs::read_to_string("/proc/mounts")?;
    for (path, typ) in [("/proc", "proc"), ("/sys", "sysfs"),
        ("/dev", "devtmpfs"), ("/run", "tmpfs"), ("/tmp", "tmpfs")] {
        if !mounted.lines().any(|line| {
            let w: Vec<_> = line.split_ascii_whitespace().collect();
            w.len() > 2 && w[1] == path && w[2] == typ
        }) {
            return Err(io::Error::other(format!("stage2 mount not moved: {path}")));
        }
    }
    serial("VITA_I009B1_ROOT_MOUNT_READONLY_OK path=/ fstype=squashfs errno=EROFS pid=1");
    serial("VITA_I009B1_SWITCH_ROOT_PID1_CONTINUITY_OK");
    Ok(())
}

/// This is an explicit VM-only device-owner handoff; production login/TTY ACL
/// policy belongs to the separately tested session manager.
fn configure_fixture_tty() -> io::Result<()> {
    chown("/dev/ttyS0", Some(Uid::from_raw(VITA_UID)), Some(Gid::from_raw(VITA_GID)))
        .map_err(|e| io::Error::other(format!("fixture tty ownership failed: {e}")))
}

/// The Linux /proc stat fields after "(comm)" start with state, ppid, pgrp,
/// session, tty_nr, tpgid. Merely redirecting stdio does NOT prove a TTY.
fn controlling_tty_info(stat: &str, expected_pid: i32) -> io::Result<(i32, i32, i32)> {
    let (_, fields) = stat.rsplit_once(") ").ok_or_else(|| io::Error::other("malformed /proc/self/stat"))?;
    let rest: Vec<&str> = fields.split_ascii_whitespace().collect();
    if rest.len() < 6 {
        return Err(io::Error::other("short /proc/self/stat"));
    }
    let pgrp = rest[2].parse::<i32>().map_err(|_| io::Error::other("invalid pgrp"))?;
    let sid = rest[3].parse::<i32>().map_err(|_| io::Error::other("invalid sid"))?;
    let tty_nr = rest[4].parse::<i32>().map_err(|_| io::Error::other("invalid tty_nr"))?;
    let tpgid = rest[5].parse::<i32>().map_err(|_| io::Error::other("invalid tpgid"))?;
    if sid != expected_pid || pgrp != expected_pid || tty_nr == 0 || tpgid != pgrp {
        return Err(io::Error::other(format!(
            "no controlling foreground tty: sid={sid} pgrp={pgrp} tty={tty_nr} tpgid={tpgid}"
        )));
    }
    Ok((sid, tty_nr, tpgid))
}

fn session_child(crash: bool) -> io::Result<()> {
    if geteuid().as_raw() != VITA_UID {
        return Err(io::Error::other("Talos session launcher did not drop root privileges"));
    }
    // Independently verify the runtime state from the unprivileged session,
    // not only from root PID1. A permission-denied error is not sufficient.
    reject_system_write()?;
    serial("VITA_I009B0_NONROOT_SYSTEM_WRITE_DENIED uid=1000 errno=EROFS");
    // The I009b1 SquashFS guest has a test-only, otherwise UID1000-writable
    // directory. It prevents EACCES from masquerading as readonly-root EROFS.
    let rootinfo = fs::read_to_string("/proc/self/mountinfo")?;
    if rootinfo.lines().any(|line| {
        let w: Vec<_> = line.split_ascii_whitespace().collect();
        w.len() > 8 && w[4] == "/" && line.contains(" - squashfs ")
    }) {
        const FIXTURE: &str = "/.vita-i009b1-write-fixture";
        let metadata = fs::metadata(FIXTURE)?;
        if metadata.mode() & 0o777 != 0o777 {
            return Err(io::Error::other("nonroot rootfs probe fixture permissions are insufficient"));
        }
        match OpenOptions::new().write(true).create_new(true)
            .open("/.vita-i009b1-write-fixture/uid1000-probe") {
            Err(e) if e.raw_os_error() == Some(Errno::EROFS as i32) => {
                serial("VITA_I009B1_NONROOT_ROOT_WRITE_DENIED uid=1000 errno=EROFS");
            }
            Ok(_) => {
                let _ = fs::remove_file("/.vita-i009b1-write-fixture/uid1000-probe");
                return Err(io::Error::other("nonroot could write permanent rootfs"));
            }
            Err(e) => return Err(io::Error::other(format!(
                "nonroot rootfs write must return EROFS, got: {e}"
            ))),
        }
    }
    fs::write("/tmp/.vita-i009b0-volatile-probe", b"vita-i009b0")?;
    if fs::read("/tmp/.vita-i009b0-volatile-probe")? != b"vita-i009b0" {
        return Err(io::Error::other("volatile /tmp read/write mismatch"));
    }
    fs::remove_file("/tmp/.vita-i009b0-volatile-probe")?;
    serial("VITA_I009B0_VOLATILE_TMP_OK uid=1000");
    if crash {
        serial("VITA_I009_SIMULATED_SESSION_CRASH");
        std::process::exit(42);
    }
    // Ctrl+C targets the entire foreground process group, including this
    // launcher. Catch SIGINT here so Talos' own Ctrl+C cancellation handler
    // can complete and the launcher can wait/reap it normally. Caught signal
    // dispositions reset on exec into the actual Talos binary.
    flag::register(SIGINT, Arc::new(AtomicBool::new(false)))?;
    // Become a new session leader BEFORE opening the serial tty so the tty
    // can become this session's controlling terminal (no shell/getty).
    let sid = setsid().map_err(io::Error::other)?;
    let tty = OpenOptions::new().read(true).write(true).open("/dev/ttyS0")?;
    let stat = fs::read_to_string("/proc/self/stat")?;
    let (_sid, tty_nr, _tpgid) = controlling_tty_info(&stat, std::process::id() as i32)?;
    serial(&format!("VITA_I009_CONTROLLING_TTY_OK sid={sid} tty_nr={tty_nr} uid={VITA_UID}"));
    let status = Command::new(SESSION_BINARY)
        .arg("--console-fixture")
        .current_dir("/")
        .env_clear()
        .env("HOME", "/")
        .env("TERM", "vt100")
        .env("PATH", "/vita")
        .stdin(Stdio::from(tty.try_clone()?))
        .stdout(Stdio::from(tty.try_clone()?))
        .stderr(Stdio::from(tty))
        .status()?;
    if status.success() {
        serial("VITA_I009_TALOS_NATIVE_EXIT_OK");
        Ok(())
    } else {
        Err(io::Error::other(format!("native Talos exited with {status}")))
    }
}

fn orphan_parent(signal_pid1: bool) -> io::Result<()> {
    // A grandchild exits AFTER its direct parent: Linux must reparent it to
    // PID 1. Fatal fixture uses *this adopted child* to signal PID 1, with
    // no separate signal-helper child that could fake the orphan proof.
    let mut cmd = Command::new(INIT_BINARY);
    cmd.arg("--orphan-child");
    if signal_pid1 {
        cmd.arg("--signal-pid1");
    }
    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(())
}

fn orphan_child(signal_pid1: bool) -> io::Result<()> {
    thread::sleep(Duration::from_millis(if signal_pid1 { 650 } else { 300 }));
    if signal_pid1 {
        kill(Pid::from_raw(1), Signal::SIGTERM).map_err(io::Error::other)?;
    }
    Ok(())
}

fn signal_parent_fixture(active: bool) -> io::Result<()> {
    // A longer guest-local delay gives the nonroot native Talos console time
    // to claim its controlling TTY before this fixture sends SIGTERM to PID1.
    thread::sleep(Duration::from_millis(if active { 1750 } else { 650 }));
    kill(Pid::from_raw(1), Signal::SIGTERM).map_err(io::Error::other)
}

fn uid_from_proc(pid: Pid) -> io::Result<u32> {
    let status = fs::read_to_string(format!("/proc/{pid}/status"))?;
    let line = status.lines().find(|line| line.starts_with("Uid:"))
        .ok_or_else(|| io::Error::other("no child UID in proc"))?;
    line.split_ascii_whitespace().nth(1)
        .ok_or_else(|| io::Error::other("no child real UID"))?
        .parse().map_err(|_| io::Error::other("bad child UID"))
}

fn spawn_session(crash: bool) -> io::Result<Pid> {
    let c = OpenOptions::new().write(true).open("/dev/console")?;
    let child = Command::new(INIT_BINARY)
        .arg("--session-child")
        .args(if crash { &["--fixture-crash"][..] } else { &[][..] })
        .current_dir("/")
        .env_clear()
        .env("HOME", "/")
        .env("PATH", "/vita")
        .uid(VITA_UID)
        .gid(VITA_GID)
        .stdin(Stdio::null())
        .stdout(Stdio::from(c.try_clone()?))
        .stderr(Stdio::from(c))
        .spawn()?;
    let pid = Pid::from_raw(i32::try_from(child.id()).map_err(io::Error::other)?);
    // Failure to inspect a child (e.g., when it exits during /proc lookup)
    // must not escape with a potentially live, untracked session process.
    match uid_from_proc(pid) {
        Ok(uid) if uid == VITA_UID => {},
        result => {
            let _ = kill(pid, Signal::SIGKILL);
            let _ = waitpid(pid, None);
            return Err(io::Error::other(format!(
                "Talos session UID could not be verified: {result:?}"
            )));
        }
    }
    serial(&format!("VITA_I009_TALOS_UID={VITA_UID} pid={pid}"));
    Ok(pid)
}

fn start_or_failsafe(crash: bool, injected_failure: bool) -> Option<Pid> {
    let result = if injected_failure {
        Err(io::Error::other("VM-only injected session spawn failure"))
    } else {
        spawn_session(crash)
    };
    match result {
        Ok(pid) => Some(pid),
        Err(error) => {
            serial(&format!("VITA_I009_SESSION_START_FAILED: {error}"));
            serial("VITA_I009_FAILSAFE_NO_ROOT_SHELL");
            None
        }
    }
}

fn fatal_pid1_reaper(error: io::Error) -> ! {
    serial(&format!("VITA_I006_GUEST_SESSION_ERROR: {error}"));
    serial("VITA_I009_FATAL_PID1_REAPER_ACTIVE");
    serial("VITA_I009_FAILSAFE_NO_ROOT_SHELL");
    let stopping = Arc::new(AtomicBool::new(false));
    if let Err(e) = flag::register(SIGTERM, stopping.clone()) {
        serial(&format!("VITA_I009_FATAL_SIGNAL_HANDLER_ERROR: {e}"));
    }
    if let Err(e) = flag::register(SIGINT, stopping.clone()) {
        serial(&format!("VITA_I009_FATAL_SIGNAL_HANDLER_ERROR: {e}"));
    }
    loop {
        if stopping.swap(false, Ordering::Relaxed) {
            serial("VITA_I009_SHUTDOWN_REQUESTED");
        }
        match waitpid(Pid::from_raw(-1), Some(WaitPidFlag::WNOHANG)) {
            Ok(WaitStatus::StillAlive) | Err(Errno::ECHILD) => thread::sleep(POLL),
            Ok(WaitStatus::Exited(pid, _)) | Ok(WaitStatus::Signaled(pid, _, _)) => {
                serial(&format!("VITA_I009_FATAL_REAPED pid={pid}"));
            }
            Ok(_) | Err(Errno::EINTR) => {},
            Err(e) => {
                serial(&format!("VITA_I009_FATAL_WAIT_ERROR: {e}"));
                thread::sleep(POLL);
            }
        }
    }
}

#[derive(Debug)]
struct Budget {
    used: u32,
    since: Instant,
}

impl Budget {
    fn new(now: Instant) -> Self {
        Self { used: 0, since: now }
    }

    fn allocate(&mut self, now: Instant) -> Option<u32> {
        if now.duration_since(self.since) > WINDOW {
            self.since = now;
            self.used = 0;
        }
        if self.used == MAX_LAUNCHES {
            return None;
        }
        self.used += 1;
        Some(self.used)
    }
}

fn boot_pid1(stage2: bool) -> io::Result<()> {
    if std::process::id() != 1 || !geteuid().is_root() {
        return Err(io::Error::other("Vita init must run as Linux root PID 1"));
    }
    serial("VITA_I006_GUEST_PID1_OK");
    // Stage1 has moved these mounts into the separately mounted SquashFS.
    // The direct initramfs I009a/b0 regressions keep their existing path.
    if !stage2 {
        mount_runtime()?;
    } else {
        verify_stage2_immutable_root()?;
    }
    let flags = parse_boot_options(&fs::read_to_string("/proc/cmdline")?)?;
    verify_rootfs_ownership()?;
    mount_immutable_vita()?;
    configure_fixture_tty()?;
    serial("VITA_I009_PID1_MINIMAL_INIT_READY");

    let stopping = Arc::new(AtomicBool::new(false));
    flag::register(SIGTERM, stopping.clone())?;
    flag::register(SIGINT, stopping.clone())?;
    let mut budget = Budget::new(Instant::now());
    let mut orphan_parent_pid = None;
    let mut expect_orphan = flags.test_orphan;
    if flags.test_orphan {
        let mut cmd = Command::new(INIT_BINARY);
        cmd.arg("--orphan-parent");
        if flags.fatal_failure {
            cmd.arg("--signal-pid1");
        }
        let child = cmd
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        orphan_parent_pid = Some(Pid::from_raw(child.id() as i32));
    }
    if flags.fatal_failure {
        // The PID 1 function really RETURNS an error after a child exists.
        // main() must enter fatal_pid1_reaper, then reap both the known direct
        // parent and its adopted grandchild and process their SIGTERM.
        if let Some(pid) = orphan_parent_pid {
            serial(&format!("VITA_I009_ORPHAN_PARENT_PID={pid}"));
        }
        return Err(io::Error::other("VM-only injected fatal boot_pid1 failure"));
    }
    let mut session_pid = None;
    if let Some(attempt) = budget.allocate(Instant::now()) {
        serial(&format!("VITA_I009_SESSION_START attempt={attempt}"));
        session_pid = start_or_failsafe(flags.crash_session, flags.spawn_failure);
    }
    let mut signal_helper_pid = None;
    if flags.spawn_failure || flags.test_active_shutdown {
        // VM-only: prove that failed startup and *live* non-root sessions do
        // not stop PID1's signal handling and shutdown/reaping behavior.
        let mut cmd = Command::new(INIT_BINARY);
        cmd.arg("--signal-parent");
        if flags.test_active_shutdown {
            cmd.arg("--active");
        }
        let helper = cmd
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        signal_helper_pid = Some(Pid::from_raw(helper.id() as i32));
    }

    let mut shutting_down = false;
    loop {
        // Handle each termination request exactly once; a latched flag would
        // otherwise spam the serial console while PID 1 remains in safe mode.
        if stopping.swap(false, Ordering::Relaxed) {
            serial("VITA_I009_SHUTDOWN_REQUESTED");
            shutting_down = true;
            if let Some(pid) = session_pid {
                // The session launcher is the process-group leader after setsid.
                // The experimental crash fixture never claims group ownership.
                if !flags.crash_session {
                    let _ = kill(Pid::from_raw(-pid.as_raw()), Signal::SIGTERM);
                }
                let _ = kill(pid, Signal::SIGTERM);
            }
            // Keep tracking/reaping the terminated child. Never restart while
            // shutting down even if it exits by signal.
        }
        let mut changed = false;
        loop {
            match waitpid(Pid::from_raw(-1), Some(WaitPidFlag::WNOHANG)) {
                Ok(WaitStatus::StillAlive) | Err(Errno::ECHILD) => break,
                Ok(WaitStatus::Exited(pid, code)) => {
                    changed = true;
                    if session_pid == Some(pid) {
                        session_pid = None;
                        if shutting_down {
                            serial("VITA_I009_SHUTDOWN_SESSION_REAPED");
                        } else if code == 0 {
                            serial("VITA_I006_GUEST_TALOS_SESSION_OK");
                            serial("VITA_I007_DEFAULT_TALOS_EXIT_OK");
                        } else if stopping.load(Ordering::Relaxed) {
                            serial("VITA_I009_RESTART_ABORTED_SHUTDOWN");
                        } else if let Some(attempt) = budget.allocate(Instant::now()) {
                            serial(&format!("VITA_I009_SESSION_RESTART attempt={attempt} exit={code}"));
                            thread::sleep(Duration::from_millis(120));
                            // A stop can arrive DURING the retry backoff.
                            // Never start one more Talos session after SIGTERM.
                            if stopping.load(Ordering::Relaxed) {
                                serial("VITA_I009_RESTART_ABORTED_SHUTDOWN");
                            } else {
                                serial(&format!("VITA_I009_SESSION_START attempt={attempt}"));
                                session_pid = start_or_failsafe(flags.crash_session, false);
                            }
                        } else {
                            serial(&format!("VITA_I009_CRASH_BUDGET_EXHAUSTED attempts={MAX_LAUNCHES}"));
                            serial("VITA_I009_FAILSAFE_NO_ROOT_SHELL");
                        }
                    } else if Some(pid) == orphan_parent_pid {
                        orphan_parent_pid = None;
                    } else if Some(pid) == signal_helper_pid {
                        // Explicitly exclude this direct PID from adopted-child
                        // evidence even if it exits before the orphan does.
                        signal_helper_pid = None;
                    } else if expect_orphan {
                        serial(&format!("VITA_I009_ORPHAN_REAPED_OK pid={pid}"));
                        expect_orphan = false;
                    }
                }
                Ok(WaitStatus::Signaled(pid, sig, _)) => {
                    changed = true;
                    if session_pid == Some(pid) {
                        session_pid = None;
                        serial(&format!("VITA_I009_SESSION_SIGNALLED signal={sig}"));
                        if shutting_down {
                            serial("VITA_I009_SHUTDOWN_SESSION_REAPED");
                        } else if stopping.load(Ordering::Relaxed) {
                            serial("VITA_I009_RESTART_ABORTED_SHUTDOWN");
                        } else if let Some(attempt) = budget.allocate(Instant::now()) {
                            serial(&format!("VITA_I009_SESSION_START attempt={attempt}"));
                            session_pid = start_or_failsafe(flags.crash_session, false);
                        } else {
                            serial(&format!("VITA_I009_CRASH_BUDGET_EXHAUSTED attempts={MAX_LAUNCHES}"));
                            serial("VITA_I009_FAILSAFE_NO_ROOT_SHELL");
                        }
                    }
                }
                Ok(_) => { changed = true; }
                Err(Errno::EINTR) => continue,
                Err(e) => return Err(io::Error::other(format!("waitpid failed: {e}"))),
            }
        }
        if !changed {
            thread::sleep(POLL);
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let result = match args.get(1).map(String::as_str) {
        None => boot_pid1(false),
        Some("--stage2") => boot_pid1(true),
        Some("--session-child") => session_child(args.get(2).is_some_and(|s| s == "--fixture-crash")),
        Some("--orphan-parent") => orphan_parent(args.get(2).is_some_and(|s| s == "--signal-pid1")),
        Some("--orphan-child") => orphan_child(args.get(2).is_some_and(|s| s == "--signal-pid1")),
        Some("--signal-parent") => signal_parent_fixture(args.get(2).is_some_and(|s| s == "--active")),
        _ => Err(io::Error::other("unsupported Vita init mode")),
    };
    if let Err(e) = result {
        if std::process::id() == 1 {
            fatal_pid1_reaper(e);
        }
        serial(&format!("VITA_I006_GUEST_SESSION_ERROR: {e}"));
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boot_options_reject_recovery_and_ambiguous_fixture() {
        assert_eq!(parse_boot_options("console=ttyS0").unwrap(), Fixture::default());
        assert_eq!(parse_boot_options("vita.i009.test_crash=1 vita.i009.test_orphan=1").unwrap(),
            Fixture { crash_session: true, test_orphan: true, spawn_failure: false, fatal_failure: false, test_active_shutdown: false });
        for line in ["vita.recovery=1", "vita.recovery=yes", "vita.recovery=1 vita.recovery=0",
            "vita.i009.test_crash=2", "vita.i009.test_crash=1 vita.i009.test_crash=1",
            "vita.i009.test_orphan=1 vita.i009.test_orphan=0",
            "vita.i009.test_spawn_failure=0",
            "vita.i009.test_spawn_failure=1 vita.i009.test_spawn_failure=1",
            "vita.i009.test_fatal=0",
            "vita.i009.test_fatal=1",
            "vita.i009.test_orphan=1 vita.i009.test_fatal=1 vita.i009.test_fatal=1",
            "vita.i009.test_active_shutdown=0",
            "vita.i009.test_active_shutdown=1 vita.i009.test_crash=1",
            "vita.i009.test_active_shutdown=1 vita.i009.test_spawn_failure=1",
            "vita.i009.test_active_shutdown=1 vita.i009.test_active_shutdown=1"] {
            assert!(parse_boot_options(line).is_err(), "{line}");
        }
    }

    #[test]
    fn startup_failure_is_explicitly_fixture_gated() {
        assert_eq!(
            parse_boot_options("vita.i009.test_orphan=1 vita.i009.test_spawn_failure=1").unwrap(),
            Fixture { crash_session: false, test_orphan: true, spawn_failure: true, fatal_failure: false, test_active_shutdown: false }
        );
    }

    #[test]
    fn fatal_failure_requires_an_adopted_grandchild() {
        assert_eq!(
            parse_boot_options("vita.i009.test_orphan=1 vita.i009.test_fatal=1").unwrap(),
            Fixture { crash_session: false, test_orphan: true, spawn_failure: false, fatal_failure: true, test_active_shutdown: false }
        );
    }

    #[test]
    fn mountinfo_parser_requires_actual_ro_vita_mount() {
        let info = "21 1 0:18 / / rw,relatime - rootfs rootfs rw\n\
                    31 21 0:18 /vita /vita ro,nosuid,nodev,relatime - rootfs rootfs rw\n\
                    32 21 0:35 / /tmp rw,nosuid,nodev - tmpfs tmpfs rw\n";
        assert!(has_readonly_mount(info, "/vita"));
        assert!(!has_readonly_mount(info, "/tmp"));
        assert!(!has_readonly_mount(info, "/"));
        assert!(!has_readonly_mount(
            "31 21 0:18 /vita /vita rw,nosuid - rootfs rootfs rw",
            "/vita"
        ));
        assert!(!has_readonly_mount(info, "/sys"));
    }

    #[test]
    fn active_shutdown_is_independent_of_crash_and_recovery_modes() {
        assert_eq!(
            parse_boot_options("vita.i009.test_active_shutdown=1").unwrap(),
            Fixture { test_active_shutdown: true, ..Fixture::default() }
        );
    }

    #[test]
    fn budget_is_bounded_and_resets_only_after_window() {
        let t0 = Instant::now();
        let mut budget = Budget::new(t0);
        assert_eq!(budget.allocate(t0), Some(1));
        assert_eq!(budget.allocate(t0), Some(2));
        assert_eq!(budget.allocate(t0), Some(3));
        assert_eq!(budget.allocate(t0), None);
        assert_eq!(budget.allocate(t0 + WINDOW), None);
        assert_eq!(budget.allocate(t0 + WINDOW + Duration::from_millis(1)), Some(1));
    }

    #[test]
    fn controlling_tty_requires_session_leadership_and_foreground() {
        // Fields 3...8: state, ppid, pgrp, sid, tty_nr, tpgid
        assert_eq!(controlling_tty_info("25 (vita init) S 1 25 25 1088 25 0", 25).unwrap(),
            (25, 1088, 25));
        assert!(controlling_tty_info("25 (vita) S 1 25 25 0 25", 25).is_err());
        assert!(controlling_tty_info("25 (vita) S 1 7 25 1088 7", 25).is_err());
        assert!(controlling_tty_info("25 (vita) S 1 25 25 1088 5", 25).is_err());
    }
}
