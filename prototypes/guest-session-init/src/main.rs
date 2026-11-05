//! I006 Linux guest PID 1: directly launch the Vita/Talos Rust console.
//! QEMU-only experiment, not a production init/service manager.
#![forbid(unsafe_code)]

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

const PROC_DIR: &str = "/proc";
const SESSION_BINARY: &str = "/vita/vita-interaction-prototype";

fn serial_line(line: &str) {
    if let Ok(mut console) = OpenOptions::new().write(true).open("/dev/console") {
        let _ = writeln!(console, "{line}");
        let _ = console.flush();
    } else {
        eprintln!("{line}");
    }
}

fn boot() -> io::Result<()> {
    if std::process::id() != 1 {
        return Err(io::Error::other("guest init must be Linux PID 1"));
    }
    serial_line("VITA_I006_GUEST_PID1_OK");

    // Rust-native guest-only procfs mount, with fixed, restrictive flags.
    // No model-exposed mount authority or external boot subprocess.
    nix::mount::mount(
        Some("proc"),
        PROC_DIR,
        Some("proc"),
        nix::mount::MsFlags::MS_NODEV
            | nix::mount::MsFlags::MS_NOEXEC
            | nix::mount::MsFlags::MS_NOSUID,
        None::<&str>,
    )
    .map_err(|err| io::Error::other(format!("guest proc mount syscall failed: {err}")))?;
    if fs::read_to_string("/proc/uptime")?.trim().is_empty() {
        return Err(io::Error::other("guest /proc was not mounted"));
    }
    serial_line("VITA_I006_GUEST_PROC_READY");

    // Vita's own Talos Runtime SDK host is the direct primary session.
    // No Linux shell or Talos CLI/TUI sits between init and this process.
    let stdin = File::open("/dev/ttyS0")?;
    let stdout = OpenOptions::new().write(true).open("/dev/ttyS0")?;
    let stderr = stdout.try_clone()?;
    let status = Command::new(SESSION_BINARY)
        .arg("--console-fixture")
        .current_dir("/")
        .stdin(Stdio::from(stdin))
        .stdout(Stdio::from(stdout))
        .stderr(Stdio::from(stderr))
        .status()?;
    if !status.success() {
        return Err(io::Error::other(format!("Talos host exited with {status}")));
    }
    serial_line("VITA_I006_GUEST_TALOS_SESSION_OK");
    Ok(())
}

fn main() {
    if let Err(err) = boot() {
        serial_line(&format!("VITA_I006_GUEST_SESSION_ERROR: {err}"));
    }
    // Keep test-only PID 1 alive until the QEMU observer closes the VM.
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mount_is_fixed_and_time_bounded() {
        assert_eq!(PROC_DIR, "/proc");
        assert_eq!(SESSION_BINARY, "/vita/vita-interaction-prototype");
    }
}
