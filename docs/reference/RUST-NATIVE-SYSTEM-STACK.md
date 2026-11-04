# Rust-native System Stack — First-round Selection Guide

Status: candidate interfaces only. Individual crates/versions are not selected dependencies before testing.

## Architecture boundary

Firmware + boot substrate -> Linux kernel / minimal userspace -> Vita-owned Rust session runtime -> Talos Runtime -> Rust capability broker -> Rust system providers -> Linux native interfaces and reviewed external services.

The recovery shell is an explicit operator fallback. Normal Agent operations do not synthesize shell commands.

## Candidate implementation matrix

| Vita capability | Rust / native API candidates | Risk and compatibility notes |
| --- | --- | --- |
| Process and system facts | std, procfs, sysinfo, rustix, /proc | Respect PID namespaces; avoid localized command output |
| Files and permissions | std::fs, cap-std, rustix | Prefer capability-scoped handles; check symlinks and TOCTOU |
| Block inventory | /sys/class/block, /proc, vetted ioctls via rustix/nix | Confirm device identity, partitions and mounts before any mutation |
| Network inspection and control | rtnetlink, sockets, netlink libraries | Namespace-specific view; mutations require policy |
| Services | Typed IPC such as zbus where the target image has D-Bus | Service manager is an explicit distribution decision |
| Storage operations | rustix/nix mount/ioctl where safe; reviewed helper otherwise | Partitioning, formatting, encryption remain high-risk |
| Config, async and TLS | serde, toml, tokio, rustls, tracing | Prefer reuse of Talos implementation; avoid credential logs |
| Console/PTY | Rust terminal/PTY libraries where needed | The default session is a Rust process, not a parent shell |

A Rust crate using libudev or another native library is not automatically pure Rust. Assess transitive dependencies, architecture support and maintenance before adoption.

## Dependency and exception budget

The Live OS build records shipped binary/package, its role, native libraries, any application-language runtime, license, platform support and build-only versus runtime classification. Runtime use of Python, Node.js or JVM to implement Vita-owned core behavior needs an approved exception.

If a provider cannot safely use a Rust-native API, its narrow adapter declares executable/IPC identity, typed arguments, bounded output, locale/format expectations, privileges, timeouts and error/audit semantics. There is no generic run_shell_as_root capability.

## First extracted provider (I003)

`crates/vita-system-facts` is an implemented no-dependency Rust library for bounded, read-only Linux system summary fields. It is invoked through `prototypes/interaction-runtime` with Talos' explicit Read permission rule and tested with deterministic fixture paths. This is **not** a complete capability broker, service manager or disk-inventory API.

## Validation

- system_summary and first read-only inventory execute without a shell subprocess.
- Provider tests cover permissions, missing native facilities, target identity and structured failures.
- VM tests validate broker allow/deny/confirm/audit and offline/recovery behavior.
- Do not publish arbitrary byte-size or latency claims before measuring the selected Live image.

See [ADR-0005](../decisions/ADR-0005-rust-first-system-capabilities.md), [Privileged Operations SOP](../sop/PRIVILEGED-OPERATIONS.md) and [Architecture](SYSTEM-ARCHITECTURE.md).
