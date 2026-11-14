//! I009b1: disposable early Linux init. Linux PID 1 execs permanent Rust
//! vita-init after moving a real, read-only squashfs root onto /.
//! Never a privileged Agent tool or a production recovery implementation.
#![deny(unsafe_op_in_unsafe_fn)]

use nix::errno::Errno;
use nix::kmod::init_module;
use nix::mount::{mount, MsFlags};
use nix::unistd::{chroot, getpid};
use std::ffi::CStr;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use sha2::{Digest, Sha256};
use std::os::fd::AsRawFd;
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::thread;
use std::time::Duration;

const IMAGE: &str = "/system.squashfs";
const EXPECTED_ROOT_IMAGE_SHA256: &str = env!("VITA_ROOTFS_SHA256");
const LOOP_DEVICE: &str = "/dev/loop0";
const NEW_ROOT: &str = "/newroot";
// Linux loop.h LOOP_SET_FD; the only unsafe call is this fixed, early-boot
// ioctl on an in-initramfs device inode. No user paths or untrusted args.
const LOOP_SET_FD: nix::libc::c_int = 0x4c00;

fn serial(message: &str) {
    if let Ok(mut f) = OpenOptions::new().write(true).open("/dev/console") {
        let _ = writeln!(f, "{message}");
        let _ = f.flush();
    } else {
        eprintln!("{message}");
    }
}

fn mount_fs(src: &str, dest: &str, kind: &str, flags: MsFlags, data: Option<&str>) -> io::Result<()> {
    mount(Some(src), dest, Some(kind), flags, data)
        .map_err(|e| io::Error::other(format!("mount {kind} {dest}: {e}")))
}

fn early_mounts() -> io::Result<()> {
    let safe = MsFlags::MS_NOSUID | MsFlags::MS_NODEV;
    mount_fs("proc", "/proc", "proc", safe | MsFlags::MS_NOEXEC, None)?;
    mount_fs("sysfs", "/sys", "sysfs", safe | MsFlags::MS_NOEXEC | MsFlags::MS_RDONLY, None)?;
    mount_fs("devtmpfs", "/dev", "devtmpfs", MsFlags::MS_NOSUID, Some("mode=0755"))?;
    mount_fs("tmpfs", "/run", "tmpfs", safe, Some("mode=0755,size=16m"))?;
    mount_fs("tmpfs", "/tmp", "tmpfs", safe | MsFlags::MS_NOEXEC, Some("mode=1777,size=16m"))?;
    fs::create_dir_all("/run/vita")?;
    // Preserve the existing I006 Talos observer's meaningful readiness
    // contract: Linux procfs and volatile guest mounts have *actually*
    // been established before the prompt becomes interactive.
    if fs::read_to_string("/proc/uptime")?.trim().is_empty() {
        return Err(io::Error::other("early procfs is unavailable"));
    }
    serial("VITA_I006_GUEST_PROC_READY");
    serial("VITA_I007_RUNTIME_MOUNTS_READY proc=proc sys=sysfs run=tmpfs tmp=tmpfs");
    Ok(())
}

fn load_pinned_modules() -> io::Result<()> {
    let order = fs::read_to_string("/modules/load.order")?;
    let empty = CStr::from_bytes_with_nul(b"\0").map_err(io::Error::other)?;
    let mut count = 0usize;
    for line in order.lines() {
        let name = line.trim();
        // The build provides only numeric, generated filenames, never paths
        // or module parameters from an Agent or a kernel command line.
        if name.len() != 6 || !name.ends_with(".ko") ||
            !name[..3].chars().all(|c| c.is_ascii_digit()) {
            return Err(io::Error::other("invalid pinned kernel module manifest"));
        }
        let image = fs::read(format!("/modules/{name}"))?;
        if !image.starts_with(b"\x7fELF") {
            return Err(io::Error::other(format!("not an ELF kernel module: {name}")));
        }
        match init_module(&image, empty) {
            Ok(()) | Err(Errno::EEXIST) => count += 1,
            Err(e) => return Err(io::Error::other(format!("load pinned module {name}: {e}"))),
        }
    }
    serial(&format!("VITA_I009B1_KERNEL_MODULES_READY count={count}"));
    Ok(())
}

fn verify_image_hash() -> io::Result<()> {
    let mut file = File::open(IMAGE)?;
    let mut hasher = Sha256::new();
    let mut buf = [0_u8; 64 * 1024];
    let mut total = 0_u64;
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 { break; }
        total += n as u64;
        if total > 128 * 1024 * 1024 {
            return Err(io::Error::other("unexpectedly large immutable root image"));
        }
        hasher.update(&buf[..n]);
    }
    let actual = format!("{:x}", hasher.finalize());
    if actual != EXPECTED_ROOT_IMAGE_SHA256 {
        return Err(io::Error::other(format!(
            "immutable root image SHA-256 mismatch: {actual}"
        )));
    }
    serial(&format!(
        "VITA_I009B1_ROOT_IMAGE_SHA256_OK bytes={total} digest={actual}"
    ));
    Ok(())
}

fn mount_root_image() -> io::Result<()> {
    verify_image_hash()?;
    let source = File::open(IMAGE)?;
    let loop_fd = OpenOptions::new().read(true).open(LOOP_DEVICE)?;
    // SAFETY: LOOP_SET_FD takes a second argument holding an open fd.
    // Both fds point to fixed, root-owned initramfs objects. This is the
    // one narrowly audited unsafe syscall boundary in stage-1.
    let result = unsafe {
        nix::libc::ioctl(loop_fd.as_raw_fd(), LOOP_SET_FD, source.as_raw_fd())
    };
    if result != 0 {
        return Err(io::Error::other(format!(
            "attach readonly loop image: {}", io::Error::last_os_error()
        )));
    }
    mount_fs(LOOP_DEVICE, NEW_ROOT, "squashfs",
             MsFlags::MS_RDONLY | MsFlags::MS_NOSUID | MsFlags::MS_NODEV, None)?;
    let mountinfo = fs::read_to_string("/proc/self/mountinfo")?;
    if !mountinfo.lines().any(|line| {
        let cols: Vec<_> = line.split_ascii_whitespace().collect();
        cols.len() > 8 && cols[4] == NEW_ROOT &&
        cols[5].split(',').any(|opt| opt == "ro") &&
        line.contains(" - squashfs ")
    }) {
        return Err(io::Error::other("kernel did not mount the actual squashfs read-only"));
    }
    if !fs::metadata("/newroot/init")?.is_file() {
        return Err(io::Error::other("permanent Rust init missing from rootfs"));
    }
    serial("VITA_I009B1_STAGE1_SQUASHFS_MOUNTED readonly=1");
    Ok(())
}

fn move_mount(from: &str, to: &str) -> io::Result<()> {
    mount(Some(from), to, None::<&str>, MsFlags::MS_MOVE, None::<&str>)
        .map_err(|e| io::Error::other(format!("move mount {from} to {to}: {e}")))
}

fn stage1() -> io::Result<()> {
    if getpid().as_raw() != 1 {
        return Err(io::Error::other("stage1 must be Linux PID 1"));
    }
    serial("VITA_I009B1_STAGE1_PID1_READY");
    early_mounts()?;
    load_pinned_modules()?;
    mount_root_image()?;
    for part in ["proc", "sys", "dev", "run", "tmp"] {
        move_mount(&format!("/{part}"), &format!("/newroot/{part}"))?;
    }
    // The root of a Linux initramfs is rootfs and cannot be pivot_root'ed.
    // Follow the switch_root MS_MOVE + chroot pattern while retaining PID 1.
    std::env::set_current_dir(NEW_ROOT)?;
    mount(Some("."), "/", None::<&str>, MsFlags::MS_MOVE, None::<&str>)
        .map_err(|e| io::Error::other(format!("switch_root MS_MOVE: {e}")))?;
    chroot(".").map_err(io::Error::other)?;
    std::env::set_current_dir("/")?;
    // Command::exec retains PID 1 and never launches a shell or another
    // supervisory parent. The permanent init revalidates '/' before Talos.
    let err = Command::new("/init").arg("--stage2").env_clear().exec();
    Err(io::Error::other(format!("exec permanent Rust init failed: {err}")))
}

fn main() {
    if let Err(e) = stage1() {
        serial(&format!("VITA_I009B1_STAGE1_FAIL_CLOSED: {e}"));
        // Stage1 has no interactive root shell, network or disk write target.
        // A product fail-safe/reboot policy requires separate human review.
        loop { thread::sleep(Duration::from_secs(60)); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loop_device_constant_is_linux_loop_set_fd() {
        assert_eq!(LOOP_SET_FD, 0x4c00);
    }

    #[test]
    fn manifest_rejects_ambiguous_module_names() {
        for name in ["../../x.ko", "001.ko.zst", "abc.ko", "001.ko/foo", "000.ko/"] {
            assert!(name.len() != 6 || !name.ends_with(".ko") ||
                    !name[..3].chars().all(|c| c.is_ascii_digit()));
        }
    }
}
