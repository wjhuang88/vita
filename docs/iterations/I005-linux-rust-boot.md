# I005 — Real Linux kernel boot into Vita-owned Rust user-space

Status: Active
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

Pending first CI execution.
