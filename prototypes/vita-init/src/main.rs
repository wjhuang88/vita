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
}

fn parse_boot_options(cmdline: &str) -> io::Result<Fixture> {
    let mut fixture = Fixture::default();
    let mut crash_seen = false;
    let mut orphan_seen = false;
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
    if crash {
        serial("VITA_I009_SIMULATED_SESSION_CRASH");
        std::process::exit(42);
    }
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

fn orphan_parent() -> io::Result<()> {
    // A grandchild exits AFTER its direct parent: Linux must reparent it to
    // PID 1. The VM reaper asserts observing this different, adopted PID.
    Command::new(INIT_BINARY)
        .arg("--orphan-child")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(())
}

fn orphan_child() {
    thread::sleep(Duration::from_millis(300));
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
    if uid_from_proc(pid)? != VITA_UID {
        let _ = kill(pid, Signal::SIGKILL);
        return Err(io::Error::other("Talos child would run as root"));
    }
    serial(&format!("VITA_I009_TALOS_UID={VITA_UID} pid={pid}"));
    Ok(pid)
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

fn boot_pid1() -> io::Result<()> {
    if std::process::id() != 1 || !geteuid().is_root() {
        return Err(io::Error::other("Vita init must run as Linux root PID 1"));
    }
    serial("VITA_I006_GUEST_PID1_OK");
    // /proc must be mounted before reading the kernel command line.
    mount_runtime()?;
    let flags = parse_boot_options(&fs::read_to_string("/proc/cmdline")?)?;
    configure_fixture_tty()?;
    serial("VITA_I009_PID1_MINIMAL_INIT_READY");

    let stopping = Arc::new(AtomicBool::new(false));
    flag::register(SIGTERM, stopping.clone())?;
    flag::register(SIGINT, stopping.clone())?;
    let mut budget = Budget::new(Instant::now());
    let mut orphan_parent_pid = None;
    let mut expect_orphan = flags.test_orphan;
    if flags.test_orphan {
        let child = Command::new(INIT_BINARY)
            .arg("--orphan-parent")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        orphan_parent_pid = Some(Pid::from_raw(child.id() as i32));
    }
    let mut session_pid = None;
    if let Some(attempt) = budget.allocate(Instant::now()) {
        serial(&format!("VITA_I009_SESSION_START attempt={attempt}"));
        session_pid = Some(spawn_session(flags.crash_session)?);
    }

    loop {
        // Handle each termination request exactly once; a latched flag would
        // otherwise spam the serial console while PID 1 remains in safe mode.
        if stopping.swap(false, Ordering::Relaxed) {
            serial("VITA_I009_SHUTDOWN_REQUESTED");
            if let Some(pid) = session_pid {
                // The session launcher is the process-group leader after setsid.
                // The experimental crash fixture never claims group ownership.
                if !flags.crash_session {
                    let _ = kill(Pid::from_raw(-pid.as_raw()), Signal::SIGTERM);
                }
                let _ = kill(pid, Signal::SIGTERM);
            }
            session_pid = None;
        }
        let mut changed = false;
        loop {
            match waitpid(Pid::from_raw(-1), Some(WaitPidFlag::WNOHANG)) {
                Ok(WaitStatus::StillAlive) | Err(Errno::ECHILD) => break,
                Ok(WaitStatus::Exited(pid, code)) => {
                    changed = true;
                    if session_pid == Some(pid) {
                        session_pid = None;
                        if code == 0 {
                            serial("VITA_I006_GUEST_TALOS_SESSION_OK");
                            serial("VITA_I007_DEFAULT_TALOS_EXIT_OK");
                        } else if let Some(attempt) = budget.allocate(Instant::now()) {
                            serial(&format!("VITA_I009_SESSION_RESTART attempt={attempt} exit={code}"));
                            thread::sleep(Duration::from_millis(120));
                            serial(&format!("VITA_I009_SESSION_START attempt={attempt}"));
                            session_pid = Some(spawn_session(flags.crash_session)?);
                        } else {
                            serial(&format!("VITA_I009_CRASH_BUDGET_EXHAUSTED attempts={MAX_LAUNCHES}"));
                            serial("VITA_I009_FAILSAFE_NO_ROOT_SHELL");
                        }
                    } else if Some(pid) == orphan_parent_pid {
                        orphan_parent_pid = None;
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
                        if let Some(attempt) = budget.allocate(Instant::now()) {
                            serial(&format!("VITA_I009_SESSION_START attempt={attempt}"));
                            session_pid = Some(spawn_session(flags.crash_session)?);
                        } else {
                            serial(&format!("VITA_I009_CRASH_BUDGET_EXHAUSTED attempts={MAX_LAUNCHES}"));
                            serial("VITA_I009_FAILSAFE_NO_ROOT_SHELL");
                        }
                    }
                }
                Ok(_) => { changed = true; }
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
        None => boot_pid1(),
        Some("--session-child") => session_child(args.get(2).is_some_and(|s| s == "--fixture-crash")),
        Some("--orphan-parent") => orphan_parent(),
        Some("--orphan-child") => { orphan_child(); Ok(()) },
        _ => Err(io::Error::other("unsupported Vita init mode")),
    };
    if let Err(e) = result {
        serial(&format!("VITA_I006_GUEST_SESSION_ERROR: {e}"));
        if std::process::id() != 1 {
            std::process::exit(1);
        }
        loop {
            thread::sleep(Duration::from_secs(60));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boot_options_reject_recovery_and_ambiguous_fixture() {
        assert_eq!(parse_boot_options("console=ttyS0").unwrap(), Fixture::default());
        assert_eq!(parse_boot_options("vita.i009.test_crash=1 vita.i009.test_orphan=1").unwrap(),
            Fixture { crash_session: true, test_orphan: true });
        for line in ["vita.recovery=1", "vita.recovery=yes", "vita.recovery=1 vita.recovery=0",
            "vita.i009.test_crash=2", "vita.i009.test_crash=1 vita.i009.test_crash=1",
            "vita.i009.test_orphan=1 vita.i009.test_orphan=0"] {
            assert!(parse_boot_options(line).is_err(), "{line}");
        }
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
