# I008 — Vita-owned GRUB ISO boots Linux/Talos under BIOS and UEFI

Status: Review (original ISO head passed; expanded exact-head I001 regression pending)
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

## Evidence / closure (verified 2026-11-06; follow-up exact-head regression pending)

Verified source head `11ba5b08f2f34092c7e88d7825c3079735c807f8` on [I008 run 42180463023](https://github.com/wjhuang88/vita/actions/runs/42180463023):
- `bios-uefi-live-iso` and `governance`: success.
- Generator built a real GRUB ISO, pinned source Alpine ISO/kernel and statically linked Rust guest binaries, recorded local ISO SHA-256/size, and captured boot config plus two Linux guest serial logs in [evidence artifact 13485341649](https://github.com/wjhuang88/vita/actions/runs/42180463023/artifacts/13485341649).
- BIOS: `VITA_I006_LINUX_TALOS_GUEST_OK turns=2 errors=1` and `VITA_I008_ISO_GUEST_TALOS_OK mode=bios`.
- OVMF UEFI: `VITA_I006_LINUX_TALOS_GUEST_OK turns=2 errors=1` and `VITA_I008_ISO_GUEST_TALOS_OK mode=uefi`.
- Final combined host marker: `VITA_I008_LINUX_TALOS_ISO_VM_OK`.
- Exact original head regressions: [I005 run 42180462929](https://github.com/wjhuang88/vita/actions/runs/42180462929) success; [I006 run 42180462894](https://github.com/wjhuang88/vita/actions/runs/42180462894) success; [I007 run 42180462926](https://github.com/wjhuang88/vita/actions/runs/42180462926) success, including both guest Talos and isolated recovery. All four relevant governance checks passed.
- I001 source boot substrate was unchanged by I008. **Final change-control extension:** add a dedicated I001 disposable-loop regression job to the I008 workflow. Its exact new PR-head result is still **PENDING**. Do not merge before it and the repeated firmware test pass.

Separate human/legal/physical gates remain **PENDING** per I008 human handoff. No claim of product Live OS or distributable binary.
