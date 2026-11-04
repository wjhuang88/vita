//! First real Linux-user-space boot marker owned by Vita.
//! NOT the final Linux init, interactive session, or a production OS.
#![forbid(unsafe_code)]

use std::fs::OpenOptions;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    // The exact PID 1 assertion distinguishes Linux guest init from a host
    // tool printing an expected string. The serial console is kernel-provided.
    let marker = format!(
        "VITA_I005_LINUX_RUST_INIT_OK pid={} arch={}",
        std::process::id(),
        std::env::consts::ARCH
    );
    match OpenOptions::new().write(true).open("/dev/console") {
        Ok(mut device) => {
            let _ = writeln!(device, "{marker}");
            let _ = device.flush();
        }
        Err(_) => {
            let _ = writeln!(io::stdout(), "{marker}");
        }
    }

    // This is a boot probe, not a service manager. Keep PID 1 alive long
    // enough for the VM smoke tester to observe the real guest output.
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}
