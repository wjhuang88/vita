//! VM-only Vita guest Linux bootstrap, NOT a production init/service manager.
//! Default: embedded Rust/Talos SDK session; explicit kernel cmdline recovery:
//!           vita.recovery=1 -> credential-free, offline, root BusyBox shell.
#![forbid(unsafe_code)]

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use nix::mount::{MsFlags, mount};

const SESSION_BINARY: &str = "/vita/vita-interaction-prototype";
const RECOVERY_BINARY: &str = "/bin/busybox";
fn mount_flags() -> MsFlags {
    MsFlags::MS_NOSUID | MsFlags::MS_NODEV
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GuestMode {
    Talos,
    Recovery,
}

/// Recovery is exclusively a kernel boot-argument choice, never a prompt,
/// model output, environment variable or automatic exception fallback.
/// Ambiguous or malformed settings fail closed without opening a root shell.
fn select_mode(cmdline: &str) -> io::Result<GuestMode> {
    let mut selected = None;
    for token in cmdline.split_ascii_whitespace() {
        if let Some(value) = token.strip_prefix("vita.recovery=") {
            if selected.is_some() {
                return Err(io::Error::other("ambiguous recovery boot selection"));
            }
            selected = Some(match value {
                "0" => GuestMode::Talos,
                "1" => GuestMode::Recovery,
                _ => return Err(io::Error::other("invalid recovery boot selection")),
            });
        }
    }
    Ok(selected.unwrap_or(GuestMode::Talos))
}

fn serial_line(line: &str) {
    if let Ok(mut console) = OpenOptions::new().write(true).open("/dev/console") {
        let _ = writeln!(console, "{line}");
        let _ = console.flush();
    } else {
        eprintln!("{line}");
    }
}

fn mount_fs(
    source: &str,
    target: &str,
    kind: &str,
    flags: MsFlags,
    options: Option<&str>,
) -> io::Result<()> {
    mount(Some(source), target, Some(kind), flags, options).map_err(|error| {
        io::Error::other(format!("guest {kind} mount at {target} failed: {error}"))
    })
}

fn mount_runtime() -> io::Result<()> {
    // These are constants in the guest initramfs; never accept external or
    // model-provided paths, devices, or mounts.
    mount_fs(
        "proc",
        "/proc",
        "proc",
        mount_flags() | MsFlags::MS_NOEXEC,
        None,
    )?;
    if fs::read_to_string("/proc/uptime")?.trim().is_empty() {
        return Err(io::Error::other("guest /proc/uptime unavailable"));
    }
    serial_line("VITA_I006_GUEST_PROC_READY");

    mount_fs(
        "sysfs",
        "/sys",
        "sysfs",
        mount_flags() | MsFlags::MS_NOEXEC | MsFlags::MS_RDONLY,
        None,
    )?;
    mount_fs(
        "tmpfs",
        "/run",
        "tmpfs",
        mount_flags(),
        Some("mode=0755,size=16m"),
    )?;
    mount_fs(
        "tmpfs",
        "/tmp",
        "tmpfs",
        mount_flags() | MsFlags::MS_NOEXEC,
        Some("mode=1777,size=16m"),
    )?;
    fs::create_dir_all("/run/vita")?;

    // Verify the *actual Linux mount table*, not only successful syscall
    // return values, before allowing either interactive entry.
    let mounts = fs::read_to_string("/proc/mounts")?;
    for (target, fs_type) in [
        ("/proc", "proc"),
        ("/sys", "sysfs"),
        ("/run", "tmpfs"),
        ("/tmp", "tmpfs"),
    ] {
        if !mounts.lines().any(|line| {
            let mut fields = line.split_whitespace();
            fields.next().is_some()
                && fields.next() == Some(target)
                && fields.next() == Some(fs_type)
        }) {
            return Err(io::Error::other(format!(
                "guest runtime mount validation failed: {target}"
            )));
        }
    }
    serial_line("VITA_I007_RUNTIME_MOUNTS_READY proc=proc sys=sysfs run=tmpfs tmp=tmpfs");
    Ok(())
}

fn serial_stdio() -> io::Result<(Stdio, Stdio, Stdio)> {
    let input = File::open("/dev/ttyS0")?;
    let output = OpenOptions::new().write(true).open("/dev/ttyS0")?;
    let error = output.try_clone()?;
    Ok((Stdio::from(input), Stdio::from(output), Stdio::from(error)))
}

fn run_talos() -> io::Result<()> {
    serial_line("VITA_I007_DEFAULT_TALOS_SELECTED");
    let (stdin, stdout, stderr) = serial_stdio()?;
    let status = Command::new(SESSION_BINARY)
        .arg("--console-fixture")
        .current_dir("/")
        .stdin(stdin)
        .stdout(stdout)
        .stderr(stderr)
        .status()?;
    if !status.success() {
        return Err(io::Error::other(format!(
            "Vita/Talos host failed with status {status}; reboot with vita.recovery=1 for offline rescue"
        )));
    }
    serial_line("VITA_I006_GUEST_TALOS_SESSION_OK");
    serial_line("VITA_I007_DEFAULT_TALOS_EXIT_OK");
    Ok(())
}

fn run_recovery() -> io::Result<()> {
    if !nix::unistd::geteuid().is_root() {
        return Err(io::Error::other("guest recovery shell must be local root"));
    }
    serial_line("VITA_I007_RECOVERY_READY uid=0");
    serial_line("VITA_I007_RECOVERY_NOTICE explicit offline root shell; no model or USB disks");
    let (stdin, stdout, stderr) = serial_stdio()?;
    // Local-only, deliberate boot selection. No agent may invoke this binary.
    // Do not import model credentials or arbitrary environment variables.
    let status = Command::new(RECOVERY_BINARY)
        .args(["sh", "-i"])
        .current_dir("/")
        .env_clear()
        .env("PATH", "/bin")
        .env("HOME", "/")
        .env("TERM", "vt100")
        .stdin(stdin)
        .stdout(stdout)
        .stderr(stderr)
        .status()?;
    if !status.success() {
        return Err(io::Error::other(format!(
            "offline recovery shell exited: {status}"
        )));
    }
    serial_line("VITA_I007_RECOVERY_EXIT_OK");
    Ok(())
}

fn boot() -> io::Result<()> {
    if std::process::id() != 1 {
        return Err(io::Error::other("guest init must be Linux PID 1"));
    }
    serial_line("VITA_I006_GUEST_PID1_OK");
    mount_runtime()?;

    let cmdline = fs::read_to_string("/proc/cmdline")?;
    match select_mode(&cmdline)? {
        GuestMode::Talos => run_talos(),
        GuestMode::Recovery => run_recovery(),
    }
}

fn main() {
    if let Err(error) = boot() {
        serial_line(&format!("VITA_I006_GUEST_SESSION_ERROR: {error}"));
    }
    // A minimal VM probe, not a child reaper or permanent PID 1.
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_explicit_kernel_argument_can_enable_recovery() {
        assert_eq!(select_mode("console=ttyS0").unwrap(), GuestMode::Talos);
        assert_eq!(
            select_mode("console=ttyS0 vita.recovery=0").unwrap(),
            GuestMode::Talos
        );
        assert_eq!(
            select_mode("console=ttyS0 vita.recovery=1").unwrap(),
            GuestMode::Recovery
        );
        assert_eq!(
            select_mode("console=ttyS0 xvita.recovery=1").unwrap(),
            GuestMode::Talos
        );
    }

    #[test]
    fn ambiguous_and_invalid_boot_selection_fail_closed() {
        for cmdline in [
            "vita.recovery=yes",
            "vita.recovery=01",
            "vita.recovery=1 vita.recovery=0",
            "vita.recovery=0 vita.recovery=1",
            "vita.recovery=",
        ] {
            assert!(select_mode(cmdline).is_err(), "{cmdline}");
        }
    }
}
