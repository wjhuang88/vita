//! Read-only, bounded Linux facts for Vita-owned system providers.
//!
//! This crate deliberately uses only the Rust standard library and Linux
//! pseudo-filesystems. It never invokes a shell, child process or privileged API.
//! Missing/unreadable/oversized fields become None instead of aborting the Agent.

#![forbid(unsafe_code)]

use std::fs::File;
use std::io::Read;
use std::path::Path;

const SMALL_FIELD_LIMIT: u64 = 4096;
const MEMINFO_LIMIT: u64 = 64 * 1024;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemFacts {
    pub os_release: Option<String>,
    pub hostname: Option<String>,
    pub kernel_release: Option<String>,
    pub architecture: &'static str,
    pub family: &'static str,
    pub uptime_seconds: Option<u64>,
    pub memory_total_kib: Option<u64>,
}

/// Collect bounded system facts from the current Linux environment.
///
/// All filesystem operations are read-only. No shell command is executed.
pub fn inspect_linux() -> SystemFacts {
    inspect_paths(Path::new("/etc/os-release"), Path::new("/proc"))
}

/// Collect from explicit roots so tests can use fake /proc and /etc fixtures.
///
/// This does not imply that the source paths are trusted for privileged
/// operations. It only reads ordinary text files and limits their sizes.
pub fn inspect_paths(os_release_file: &Path, proc_root: &Path) -> SystemFacts {
    let os_release = read_bounded(os_release_file, SMALL_FIELD_LIMIT);
    let hostname = read_bounded(&proc_root.join("sys/kernel/hostname"), SMALL_FIELD_LIMIT);
    let kernel_release = read_bounded(&proc_root.join("sys/kernel/osrelease"), SMALL_FIELD_LIMIT);
    let uptime_seconds = read_bounded(&proc_root.join("uptime"), SMALL_FIELD_LIMIT)
        .and_then(|text| parse_uptime_seconds(&text));
    let memory_total_kib = read_bounded(&proc_root.join("meminfo"), MEMINFO_LIMIT)
        .and_then(|text| parse_memory_total_kib(&text));

    SystemFacts {
        os_release,
        hostname,
        kernel_release,
        architecture: std::env::consts::ARCH,
        family: std::env::consts::FAMILY,
        uptime_seconds,
        memory_total_kib,
    }
}

/// Fail closed on missing files, invalid UTF-8, unexpected size, or empty text.
fn read_bounded(path: &Path, max_bytes: u64) -> Option<String> {
    let mut reader = File::open(path).ok()?.take(max_bytes + 1);
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes).ok()?;
    if bytes.len() as u64 > max_bytes {
        return None;
    }
    let trimmed = String::from_utf8(bytes).ok()?;
    let trimmed = trimmed.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

fn parse_uptime_seconds(text: &str) -> Option<u64> {
    let seconds = text.split_whitespace().next()?.parse::<f64>().ok()?;
    // u64::MAX rounds up to 2^64 in f64; do not permit a saturated cast.
    if !seconds.is_finite() || seconds < 0.0 || seconds >= (u64::MAX as f64) {
        return None;
    }
    Some(seconds.floor() as u64)
}

fn parse_memory_total_kib(text: &str) -> Option<u64> {
    for line in text.lines() {
        let mut tokens = line.split_whitespace();
        if tokens.next()? != "MemTotal:" {
            continue;
        }
        let size = tokens.next()?.parse::<u64>().ok()?;
        if tokens.next()? != "kB" || tokens.next().is_some() {
            return None;
        }
        return Some(size);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn parses_uptime_without_shell() {
        assert_eq!(parse_uptime_seconds("1234.99 856.10"), Some(1234));
        assert_eq!(parse_uptime_seconds("0.00 0.00"), Some(0));
        assert_eq!(parse_uptime_seconds("-1.0 0.0"), None);
        assert_eq!(parse_uptime_seconds("NaN 0.0"), None);
        assert_eq!(parse_uptime_seconds("inf 0.0"), None);
        assert_eq!(parse_uptime_seconds("garbage"), None);
        assert_eq!(parse_uptime_seconds("18446744073709551616.0"), None);
    }

    #[test]
    fn parses_memtotal_as_kib_only() {
        assert_eq!(
            parse_memory_total_kib("MemFree:        10 kB\nMemTotal:   16384 kB\n"),
            Some(16_384)
        );
        assert_eq!(parse_memory_total_kib("MemTotal: 1 MB"), None);
        assert_eq!(parse_memory_total_kib("MemTotal: 1 kB extra"), None);
        assert_eq!(parse_memory_total_kib("MemTotal: nope kB"), None);
        assert_eq!(parse_memory_total_kib("MemFree: 3 kB"), None);
    }

    #[test]
    fn reads_bounded_fixture_and_survives_missing_fields() {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "vita-system-facts-{}-{nanos}",
            std::process::id()
        ));
        let proc = root.join("proc");
        fs::create_dir_all(proc.join("sys/kernel")).unwrap();
        fs::write(root.join("os-release"), "NAME=Vita Test\n").unwrap();
        fs::write(proc.join("sys/kernel/hostname"), "vita-test\n").unwrap();
        fs::write(proc.join("uptime"), "42.79 21.0\n").unwrap();
        fs::write(proc.join("meminfo"), "MemTotal: 2048 kB\n").unwrap();

        let facts = inspect_paths(&root.join("os-release"), &proc);
        assert_eq!(facts.os_release.as_deref(), Some("NAME=Vita Test"));
        assert_eq!(facts.hostname.as_deref(), Some("vita-test"));
        assert_eq!(facts.kernel_release, None);
        assert_eq!(facts.uptime_seconds, Some(42));
        assert_eq!(facts.memory_total_kib, Some(2048));

        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn rejects_oversized_and_invalid_utf8_fields() {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "vita-system-facts-invalid-{}-{nanos}",
            std::process::id()
        ));
        fs::create_dir_all(&root).unwrap();
        let input = root.join("field");
        fs::write(&input, vec![b'x'; 4097]).unwrap();
        assert_eq!(read_bounded(&input, 4096), None);
        fs::write(&input, [0xff, 0xfe]).unwrap();
        assert_eq!(read_bounded(&input, 4096), None);
        fs::write(&input, " \n\t ").unwrap();
        assert_eq!(read_bounded(&input, 4096), None);
        fs::remove_dir_all(&root).unwrap();
    }
}
