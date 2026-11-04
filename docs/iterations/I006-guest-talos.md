# I006 — Real Linux guest boots the Vita/Talos Rust session

Status: Active
Published baseline: 2026-11-04
Selected work: VITA-001-C + VITA-001-D (both Active; I004/I005 foundation Complete)
Risk: initramfs, kernel and guest process boot; disposable QEMU only; no physical disk writes.

## Objective

Connect the two independently validated halves of Vita's design: I005 real Linux -> Rust PID 1, and I004 Vita-native Rust console embedding the supported Talos Runtime SDK. A **Linux guest**, not the CI host, must run the real Talos SDK, receive serial prompts, invoke read-only system facts, recover from a fixture error, and shut down.

## Published executable baseline

- Pinned, upstream Alpine virt 3.24.3 ISO and its exact I005 verified kernel SHA-256.
- Two statically linked Rust/musl executables: Vita guest PID 1 and the existing Vita interaction prototype. The guest launches the **Vita SDK binary**, not a Talos CLI/TUI or an unrestricted shell.
- An allowlisted, statically packaged BusyBox binary **only** to mount /proc at boot, with fixed args and 5-second timeout. This is a documented temporary boot exception to the preferred Rust-native boundary, not an Agent tool or production recovery design.
- Disposable, minimal in-memory initramfs and QEMU direct -kernel/-initrd path. No host block device target, disk image, guest network, Ventoy or USB writes.
- Host Python interacts with the actual guest's serial port: inspect -> demo:error -> inspect again -> :quit. Verify Linux PID 1, mounted /proc, two [turn:ok], one [turn:error], Tool system_summary, Vita console clean shutdown, then guest init's success marker.
- Preserve CI logs on any outcome. The exact guest marker is VITA_I006_LINUX_TALOS_GUEST_OK turns=2 errors=1.

## Acceptance gate

The full script `bash scripts/ci/validate_i006_guest_session.sh` must compile both guest programs for musl, reject dynamic ELF interpreters and observe the Talos turn transcript from the QEMU guest socket. The GitHub governance job and syntax/static formatting gates must pass on the exact PR head.

**Do not declare I006 Complete** until that evidence exists. A successful cargo build alone, host process, bootloader marker or unverified QEMU spawn is insufficient.

## Exclusions and manual work

This is not a distribution-grade Linux rootfs, mature init/reaper, automatic login/TTY manager, actual online model provider, broker, recovery shell, ISO/UEFI/Ventoy boot or physical USB release. Full VITA-001-C/D Stories remain Active even if this experiment passes. Manual validation, Alpine and BusyBox licensing, TTY/PTY and firmware gates are recorded at [I006 human acceptance](../validation/I006-HUMAN-ACCEPTANCE.md). Third-party ISO compatibility remains separately Blocked in Issue #5.

## Evidence

Pending initial GitHub CI; maintain Complete/Partial classification only after inspecting exact guest output.
