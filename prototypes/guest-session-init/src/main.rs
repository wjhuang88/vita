//! I006 Linux guest PID 1: directly launch the Vita/Talos Rust console.
//! QEMU-only experiment, not a production init/service manager.
#![forbid(unsafe_code)]

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const MOUNT_TIMEOUT: Duration = Duration::from_secs(5);
const SESSION_BINARY: &str = "/vita/vita-interaction-prototype";

fn serial_line(line: &str) {
    if let Ok(mut console) = OpenOptions::new().write(true).open("/dev/console") {
        let _ = writeln!(console, "{line}");
        let _ = console.flush();
    } else {
        eprintln!("{line}");
    }
}

fn wait_bounded(child: &mut Child, limit: Duration) -> io::Result<bool> {
    let deadline = Instant::now() + limit;
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(status.success());
        }
        if Instant::now() >= deadline {
            child.kill()?;
            let _ = child.wait();
            return Ok(false);
        }
        thread::sleep(Duration::from_millis(20));
    }
}

fn boot() -> io::Result<()> {
    if std::process::id() != 1 {
        return Err(io::Error::other("guest init must be Linux PID 1"));
    }
    serial_line("VITA_I006_GUEST_PID1_OK");

    // A fixed, boot-only guest mount command. This tool is not available to
    // the agent and cannot affect host mount namespaces or any disk target.
    let mount_stdout = OpenOptions::new().write(true).open("/dev/console")?;
    let mount_stderr = mount_stdout.try_clone()?;
    let mut mount = Command::new("/bin/busybox")
        .args(["mount", "-t", "proc", "proc", "/proc"])
        .stdin(Stdio::null())
        .stdout(Stdio::from(mount_stdout))
        .stderr(Stdio::from(mount_stderr))
        .spawn()?;
    if !wait_bounded(&mut mount, MOUNT_TIMEOUT)? {
        return Err(io::Error::other("guest /proc mount failed or timed out"));
    }
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
        assert_eq!(MOUNT_TIMEOUT, Duration::from_secs(5));
        assert_eq!(SESSION_BINARY, "/vita/vita-interaction-prototype");
    }
}
