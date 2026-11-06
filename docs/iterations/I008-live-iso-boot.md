# I008 — Vita-owned GRUB ISO boots Linux/Talos under BIOS and UEFI

Status: Active (exact-head CI pending)
Published baseline: 2026-11-06
Selected work: VITA-001-C (Active); precursor to VITA-001-H (not yet Ready).
Dependency: I007 VM-only runtime and recovery merged through PR #10.
Risk: firmware/boot and GPL provenance; strictly disposable QEMU, no physical host disk.

## Immutable objective
Build an ephemeral `vita-live-experimental.iso` from the SHA-256-pinned Alpine virt 3.24.3 kernel and statically compiled Vita Rust PID 1 plus actual embedded Talos SDK host. Both QEMU legacy BIOS and OVMF x86_64 UEFI **must boot the generated ISO via GRUB**, not via QEMU `-kernel`/`-initrd`, reach real Linux guest user-space, drive two read-only Tool turns separated by a handled error, and exit cleanly.

## Executable acceptance
- Run `bash scripts/ci/validate_i008_live_iso.sh` on a disposable x86_64 Ubuntu CI host.
- Enforce pinned Alpine source ISO and extracted kernel hashes. Reject dynamic Rust binaries. Record produced Vita ISO SHA-256, byte size and GRUB configuration.
- Build the ISO via `grub-mkrescue` with BIOS/EFI boot modules. QEMU must attach **only the read-only optical ISO**; UEFI uses private temporary OVMF firmware variables, no guest network, writable host disk or USB device.
- In **each separate firmware boot**, observe guest Linux PID 1, mounted /proc and runtime, Vita/Talos SDK console, two `[turn:ok]`, one `[turn:error]`, clean `VITA_I007_DEFAULT_TALOS_EXIT_OK`, no recovery/error markers.
- Retain serial/QEMU/provenance/ISO hashes even on failure, but **do not publish the experimental ISO binary** as a release artifact.
- Preserve I001/I005/I006/I007 regression gates and governance. Do not declare Complete until actual **exact PR-head** CI passes.

## Explicit exclusions
Not an immutable Alpine rootfs, mature init/reaper, actual model API, persistent /talos/config, policy broker, shipping recovery shell or release. Compile-time experimental-root-recovery stays disabled; no BusyBox in the ISO fixture. Real hardware, Secure Boot, Ventoy compatibility (Issue #5), recovery authentication, licensing and physical writing are **separate human/release gates**. Do not close VITA-001-C, -G or -H from this iteration.

## Human handoff
All items at [I008-HUMAN-ACCEPTANCE](../validation/I008-HUMAN-ACCEPTANCE.md) remain unchecked until independently reviewed.

## Evidence / closure
PENDING: exact-head run(s), successful firmware-mode serial logs, artifacts and source SHA must be appended after verification. Do not infer success from ISO creation or an isolated GRUB menu.
