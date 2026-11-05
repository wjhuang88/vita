# ADR-0007 — Explicit recovery boot boundary for Vita's VM Linux prototype

Status: Accepted for I007 *diskless QEMU engineering proof only*; not a production security or boot policy approval
Date: 2026-11-05

## Context and constraints

- Hard: embedded Talos Runtime SDK remains the default interaction, not a Linux shell or Talos CLI wrapper.
- Hard: recovery is independently accessible offline and not selectable by model output; no raw model-to-root-shell bridge.
- Hard: boot changes must pass QEMU and no physical disk writes/host mount are permitted during the experiment.
- Soft: conventional local recovery may use a minimal static distro-provided shell.
- Unresolved assumption: local physical access plus a kernel command-line selector is sufficient to authorize root rescue. This must NOT be assumed safe for actual USB systems.

## Decision

The experimental guest Rust PID 1 mounts /proc, readonly /sys and separate bounded volatile tmpfs /run and /tmp, and verifies the Linux mount table before entering any interactive session. The initramfs root itself is still writable/ephemeral; there is no claim of an immutable read-only OS base.

Default is the native Vita/Talos SDK host. Only if the experimental-root-recovery Cargo feature is explicitly built into the VM fixture may one exact token, vita.recovery=1, select a local static BusyBox root shell with serial stdin/stdout/stderr, sanitized environment and no model or network requirement. Duplicate/invalid boot settings fail closed. A failed Talos start must not silently hand root to the model or the operator; an explicit reboot/recovery choice is necessary.

BusyBox is packaged solely for the deliberate recovery branch, **not** invoked to mount /proc (the failed I006 BusyBox boot-mount experiment remains superseded by the native Rust nix mount). There is no model-accessible shell tool or guest disk/USB device in QEMU.

## Runtime inventory

- Upstream Alpine virt 3.24.3 kernel: exact pinned upstream ISO and kernel hash.
- Vita Rust boot process and Vita/Talos SDK host: static musl Rust executables; source pinned in this repository.
- Static BusyBox: included but not executed on default boot; only executed on explicit recovery, package version and checksum retained by CI. License/source compliance needs human verification.
- Linux virtual proc/sysfs and tmpfs: in-kernel.
- Python, shell build scripts, Rust compiler, QEMU: development/CI host only, not guest runtime.

## Validation, threats and reversal

Two distinct QEMU boots must demonstrate actual guest Talos Tool turns vs executed local root-shell commands, not just boot banners. All privileged mounts target hardcoded guest virtual filesystems, the recovery shell never inherits secrets or remote network access, and no host device target is accepted. Reviewers must separately assess physical-access authentication, Secure Boot, service isolation, recovery safety and license redistribution before any full Live ISO. Root-shell access is an explicit prototype limitation, not a product security feature.

Reconsider this approach if the threat model, source licensing, production init design or bootloader policy requires authenticated or privilege-reduced recovery. The mature init/rootfs, product ISO and Ventoy handoff are not selected by this ADR.

Related: ADR-0001/0002/0005/0006; docs/validation/I007-HUMAN-ACCEPTANCE.md.

## I007 CPU compatibility correction (2026-11-05)

The first QEMU default/Talos boot succeeded, but recovery execution from Ubuntu's static BusyBox package raised guest SIGILL on QEMU's baseline x86_64 CPU ([CI 41968628065](https://github.com/wjhuang88/vita/actions/runs/41968628065)). For portable recovery, do **not** mask the incompatibility with a higher CPU model. I007 instead selects the Alpine v3.24 x86_64 static BusyBox APK matching the Alpine kernel line. Record and enforce the exact APK digest before merge; retain the unchanged QEMU baseline instruction set. Source/license and hardware compatibility review remains separate.

## I007 validated VM boundary (2026-11-05)

[Final hash-pinned CI 41970334220](https://github.com/wjhuang88/vita/actions/runs/41970334220) confirms two separate Linux QEMU guest boots on the unchanged baseline CPU: native Vita/Talos structured read-only interaction by default, and an explicit feature-gated `vita.recovery=1` path that executed and exited a local root BusyBox shell. The Alpine static BusyBox APK SHA-256 is `b0d6cfc585d7fdd12df359cc5d38d5e45894e9e9e78c4059eb41577d022ec734`, and the extracted ELF SHA-256 is `1690ed6db71fa35621537580135deb8de4f55f30a50d9d591050256e334cd8b9`; both fail closed if changed. **Human approval remains required** for source redistribution, boot authentication, trusted recovery policy and terminal/job-control UX. Serial `sh: can't access tty; job control turned off` is a recorded limitation.
