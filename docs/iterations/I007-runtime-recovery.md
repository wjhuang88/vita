# I007 — Volatile Linux runtime and explicit offline recovery

Status: Active
Published baseline: 2026-11-05
Selected scope: VITA-001-C Active; preparatory VM-only slice for VITA-001-G, which remains in Refinement.
Dependency: I006 Linux guest/Talos SDK proven and merged.
Risk: Linux boot/mounts, local root shell. QEMU diskless and networkless only.

## Executable goal

Start the *same* SHA-verified Alpine-kernel Linux guest in two separate QEMU modes:
1. Default: initialize /proc, readonly /sys and bounded volatile /run and /tmp; verify the real Linux mount table; launch the native Vita/Talos Rust SDK host and drive the established I006 read-only multi-turn Tool test.
2. Explicit recovery: kernel argument vita.recovery=1, and **only** that token, selects a separate local root BusyBox sh shell. Drive an actual shell command through the guest serial port, confirm guest uid=0 and observed command output, then exit. Talos must not start.

Both use the same disposable initramfs built with static musl Rust binaries and a distro static BusyBox reserved for *recovery only*, not model tools or boot mount helper. Recovery inherits no model secrets (env_clear) and has no network/disk target. Duplicate, malformed or unknown recovery parameters must fail closed.

## Automatic acceptance

- cargo +1.99.0 test --manifest-path prototypes/guest-session-init/Cargo.toml
- bash scripts/ci/validate_i007_runtime_recovery.sh
- Default Linux guest serial: I006 multi-turn Tool/error-recovery markers; I007_RUNTIME_MOUNTS_READY and I007_DEFAULT_TALOS_EXIT_OK; no recovery marker.
- Separate explicit recovery Linux guest serial: I007_RECOVERY_READY uid=0, I007_RECOVERY_ROOT_OK *assembled by guest shell execution*, I007_RECOVERY_EXIT_OK; no Vita/Talos console.
- Existing I001, I005, I006 CI plus governance remain passing. Preserve guest logs/provenance on failures.

Never claim Complete before exact-PR-head CI has confirmed both genuine QEMU guest paths.

## Exclusions and risk

This is an isolated engineering proof, **not** full Alpine rootfs, immutable read-only root, mature init/reaper, shipping recovery menu, authenticated real-device rescue, firmware/UEFI/Ventoy, persistence, LLM provider/config or USB installer. The initramfs root is still writable and ephemeral. The explicit recovery shell is a local **unauthenticated root shell**, which is unacceptable as an assumed production security design without separate review. Do not close C/D/G Stories based on this test.

## Evidence

Pending CI. Human security, legal, terminal and device validation all remain PENDING in docs/validation/I007-HUMAN-ACCEPTANCE.md.
