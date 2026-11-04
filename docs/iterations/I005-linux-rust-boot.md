# I005 — Real Linux kernel boot into Vita-owned Rust user-space

Status: Complete
Completed: 2026-11-04
Published baseline: 2026-11-04
Selected work: VITA-001-C (Refinement -> Ready via ADR-0006 -> Active for this slice)
Risk: Linux kernel/initramfs change; QEMU only; no installation or physical-media writes.

## Objective

Prove that a **real Linux kernel**, rather than a GRUB test-stub ISO, enters a Vita-owned Rust user-space process inside QEMU. Fix the minimal-base contract while keeping the full Live OS and Talos session acceptance separate.

## Executable deliverable

- New Rust standard-library-only `prototypes/live-init` crate, compiled as a static musl Linux executable.
- Build a disposable cpio initramfs containing `/init` and `/dev/console`; extract the genuine upstream Alpine `vmlinuz-virt` kernel from a fixed SHA-256-pinned ISO.
- QEMU boots **directly** using `-kernel` and `-initrd`, captures serial logs and requires `VITA_I005_LINUX_RUST_INIT_OK pid=1 arch=x86_64` from the guest.
- Build tooling is host-only. There is no model endpoint, shell bridge, physical USB device or downloadable product image.
- New independent CI workflow with error artifacts, unchanged baseline governance gates.

## Acceptance and exclusions

Run: `bash scripts/ci/validate_i005_live_init.sh`; also `cargo +1.99.0 fmt --manifest-path prototypes/live-init/Cargo.toml --all -- --check`.

This slice is **not** complete before the actual QEMU guest marker and CI success are observed. A Rust init marker is not a usable Vita Live session. No boot claim through Ventoy, UEFI firmware, Secure Boot or physical USB hardware is made.

The complete VITA-001-C Story additionally requires a real minimal rootfs, Talos default native session, read-only base/writable runtime contract, offline recovery and user-facing ISO. I002/Issue #5 retains independent real ISO compatibility diagnosis.

## Required human/reviewer work

See [I005 human acceptance](../validation/I005-HUMAN-ACCEPTANCE.md). Any high-risk media/boot review and physical checks remain explicitly unchecked; the build fixture alone is safe for isolated CI and has no installer target.

## Evidence

## Final CI execution — 2026-11-04

[GitHub Actions run 41791034664](https://github.com/wjhuang88/vita/actions/runs/41791034664), source `9191b0081026350956893ce4a2c97004d74392a8`:

- `governance`: **success**.
- `rust-linux-boot`: **success**. `bash -n` and pinned Rust 1.99 `cargo fmt --check` passed.
- Retrieved official Alpine virt 3.24.3 ISO and matched hard-pinned SHA-256 `cdc1bc0efc64d8d611bf3733f54868172a545888dfd726f518bf279bcb0ff1af`.
- Extracted upstream kernel SHA-256 `17fbd3efd16f2e7cb1b945f06423b37399ba7d5ac8cdd3665e7f13f363500e85`.
- Statically built Vita Rust `/init` for `x86_64-unknown-linux-musl`; the ELF interpreter check passed.
- Generated an isolated cpio initramfs; real Linux QEMU serial output contained exactly `VITA_I005_LINUX_RUST_INIT_OK pid=1 arch=x86_64`. End marker: `VITA_I005_LIVE_KERNEL_PROOF_OK`.
- Original attempt [41790880344](https://github.com/wjhuang88/vita/actions/runs/41790880344) failed before boot because minimal Rust 1.99 lacked `rustfmt`; fixed by explicitly installing the pinned component. It was not a Linux boot failure.

## Closure

**I005 Complete** for the *direct Linux kernel -> Vita Rust PID 1 proof*. **VITA-001-C remains Active and incomplete**, because the boot probe is not a usable Live OS: no normal init/service manager, rootfs, Talos default session, offline recovery, firmware/USB boot or distribution image. Future I006 must integrate an actual minimal rootfs and Talos session, then prove boot via product ISO and Ventoy separately.

Manual review and device-related gates remain **PENDING** in [I005 human checklist](../validation/I005-HUMAN-ACCEPTANCE.md). Nothing physical was flashed or inspected.
