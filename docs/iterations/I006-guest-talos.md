# I006 — Real Linux guest boots the Vita/Talos Rust session

Status: Complete
Completed: 2026-11-04
Published baseline: 2026-11-04
Selected work: VITA-001-C + VITA-001-D (both Active; I004/I005 foundation Complete)
Risk: initramfs, kernel and guest process boot; disposable QEMU only; no physical disk writes.

## Objective

Connect the two independently validated halves of Vita's design: I005 real Linux -> Rust PID 1, and I004 Vita-native Rust console embedding the supported Talos Runtime SDK. A **Linux guest**, not the CI host, must run the real Talos SDK, receive serial prompts, invoke read-only system facts, recover from a fixture error, and shut down.

## Published executable baseline

- Pinned, upstream Alpine virt 3.24.3 ISO and its exact I005 verified kernel SHA-256.
- Two statically linked Rust/musl executables: Vita guest PID 1 and the existing Vita interaction prototype. The guest launches the **Vita SDK binary**, not a Talos CLI/TUI or an unrestricted shell.
- A fixed Linux `/proc` mount performed directly by the guest Rust PID 1 through the maintained `nix` API with `nodev/noexec/nosuid` flags; no packaged BusyBox or boot shell, and no model-exposed mount tool.
- Disposable, minimal in-memory initramfs and QEMU direct -kernel/-initrd path. No host block device target, disk image, guest network, Ventoy or USB writes.
- Host Python interacts with the actual guest's serial port: inspect -> demo:error -> inspect again -> :quit. Verify Linux PID 1, mounted /proc, two [turn:ok], one [turn:error], Tool system_summary, Vita console clean shutdown, then guest init's success marker.
- Preserve CI logs on any outcome. The exact guest marker is VITA_I006_LINUX_TALOS_GUEST_OK turns=2 errors=1.

## Acceptance gate

The full script `bash scripts/ci/validate_i006_guest_session.sh` must compile both guest programs for musl, reject dynamic ELF interpreters and observe the Talos turn transcript from the QEMU guest socket. The GitHub governance job and syntax/static formatting gates must pass on the exact PR head.

**Do not declare I006 Complete** until that evidence exists. A successful cargo build alone, host process, bootloader marker or unverified QEMU spawn is insufficient.

## Exclusions and manual work

This is not a distribution-grade Linux rootfs, mature init/reaper, automatic login/TTY manager, actual online model provider, broker, recovery shell, ISO/UEFI/Ventoy boot or physical USB release. Full VITA-001-C/D Stories remain Active even if this experiment passes. Manual validation, Alpine kernel and Rust dependency licensing, TTY/PTY and firmware gates are recorded at [I006 human acceptance](../validation/I006-HUMAN-ACCEPTANCE.md). Third-party ISO compatibility remains separately Blocked in Issue #5.

## Evidence

**Verified** in [CI 41860037097](https://github.com/wjhuang88/vita/actions/runs/41860037097), source head `ec600f001d03ee154f3bc7c9f3f4269b9a9e1677`:

- `governance`: success.
- `guest-boot-to-talos`: success. Alpine virt 3.24.3 exact ISO SHA-256 and extracted Linux kernel SHA-256 matched pinned hashes.
- Vita guest PID 1 and the actual Vita/Talos Runtime SDK host were both cross-compiled for `x86_64-unknown-linux-musl` and checked for absent ELF dynamic interpreters.
- Real Linux guest serial artifact [13339188349](https://github.com/wjhuang88/vita/actions/runs/41860037097/artifacts/13339188349) contains `VITA_I006_GUEST_PID1_OK`, `VITA_I006_GUEST_PROC_READY`, the native Vita offline console banner, two distinct `[tool] system_summary`, two `[turn:ok]`, one `[turn:error]`, `VITA_CONSOLE_EXIT_OK` and `VITA_I006_GUEST_TALOS_SESSION_OK`.
- The Python observer only emitted `VITA_I006_LINUX_TALOS_GUEST_OK turns=2 errors=1` after observing all required output **from the guest**, not a host-side simulation.
- Existing I001 and I005 regression workflows also passed against the I006 branch. The first two failed BusyBox mount attempts remain documented under change control.

**I006 host/VM experiment Complete.** The complete VITA-001-C and VITA-001-D Stories remain **Active**: no distributable Live ISO/rootfs, production model/config, mature init/recovery or verified TTY/physical-device support. Manual items remain **PENDING** in the I006 handoff checklist.

## I006 change control — guest mount initialization

[Initial CI 41857471016](https://github.com/wjhuang88/vita/actions/runs/41857471016) and [diagnostic CI 41858640799](https://github.com/wjhuang88/vita/actions/runs/41858640799) both compiled Vita/Talos SDK for Linux/musl and booted Rust PID 1. The temporary static BusyBox `/proc` mount step then failed. The second run preserved complete serial error: `guest /proc mount failed or timed out`.

This is an **implementation correction**, not a changed product target: replace the failed BusyBox subprocess with the safe Rust `nix` mount wrapper, hardcoded to guest `/proc` and restrictive flags. BusyBox is removed from the test initramfs and is not part of the delivered I006 runtime. The actual QEMU guest acceptance gate passed under the corrected native Rust mount, as recorded above.
