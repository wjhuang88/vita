# VITA-001-C — Operator can boot the project Live OS

Status: Active (I008 BIOS+UEFI engineering ISO proof verified; complete Live OS open)


As an operator,
I want a directly bootable Live OS on the USB,
so that I can diagnose and operate a machine even when no external ISO is selected.

Scope:
- Kernel/initramfs/rootfs delivery.
- Read-only base system plus defined writable runtime/data areas.
- Required system tools and Talos runtime prerequisites.
- Choose a minimal Linux userspace and document shipped binaries, native libraries, app-language runtimes, licenses and build-only dependencies.
- Run the project-owned Talos session host as a native Rust process, without a scripting-language runtime requirement.
- Boot-to-session handoff.

Out of scope:
- General-purpose desktop distribution parity.
- Persistent package mutation before persistence design is accepted.

Acceptance:
- Given a supported machine/VM boots the USB
  When the Live OS entry is selected
  Then the system reaches a usable local session without another installation step
  And the Talos interaction service can start.
- Given a Live OS build is produced
  When its runtime dependencies are inspected
  Then normal Vita session startup requires no Python, Node.js or JVM unless an accepted exception is documented
  And the recovery path remains usable without a network model endpoint.

Decisions: [ADR-0005](../../decisions/ADR-0005-rust-first-system-capabilities.md), [ADR-0006](../../decisions/ADR-0006-alpine-live-base.md).

Dependencies: VITA-001-A.

## Readiness resolution and I005 scope (2026-11-04)

The missing minimal-Linux-base decision is resolved for the first experiment by ADR-0006: Alpine virt 3.24.3 kernel/musl, with Vita-owned Rust host and a mature init/recovery path in the eventual full Live image. The dependency VITA-001-A is already Complete. This Story can advance to Active, but I005 is only a **direct QEMU kernel-to-Rust process proof**, not its full acceptance.

- Implemented and proven in [CI 41791034664](https://github.com/wjhuang88/vita/actions/runs/41791034664): verified Alpine kernel -> throwaway initramfs -> statically compiled Vita Rust PID 1 -> real Linux guest serial marker. This is a boot proof, not a full OS.
- Unimplemented: complete rootfs/service manager, default Talos session, writable runtime/data, physical/UEFI firmware boot, independent offline recovery and distributable image.
- Manual decisions/reviews: [I005 handoff](../../validation/I005-HUMAN-ACCEPTANCE.md). Separate Ventoy real ISO blocker remains [Issue #5](https://github.com/wjhuang88/vita/issues/5).

## I006 guest Talos integration (verified QEMU slice)

- A new minimal initramfs packages a statically compiled Vita Rust PID 1 and the actual existing Vita/Talos SDK console to demonstrate **Linux guest boots into Talos interaction**, not a shell/CLI.
- Linux kernel and guest tools are protected by fixed SHA-256 and ELF checks. Serial input drives actual Tool turns in QEMU; [CI 41860037097](https://github.com/wjhuang88/vita/actions/runs/41860037097) passed actual guest-side tool turns and recovery.
- Full Alpine rootfs, reliable PID 1 service supervision, independent recovery, ISO/UEFI/Ventoy, persistence and physical device release are **not** included.
- The human acceptance checklist is [I006](../../validation/I006-HUMAN-ACCEPTANCE.md). The third-party ISO compatibility problem remains Issue #5.

## I007 volatile mounts and explicit offline recovery (VM proof verified)

The experimental guest Rust PID 1 now mounts /proc, readonly sysfs, bounded volatile /run and /tmp and validates the actual Linux mount table. Default launch remains the direct Rust/Talos SDK console; an exact vita.recovery=1 kernel option selects a separate offline **local root shell** (not a model Tool) with cleared inherited environment. Both independent QEMU boots have passed in [CI 41970334220](https://github.com/wjhuang88/vita/actions/runs/41970334220), with fixed SHA-256 digests for the Alpine recovery APK and executable. The root shell requires an explicitly compiled experimental-root-recovery feature (disabled by default) as well as the boot token. This is not a full rootfs, secure product boot/recovery selector or physical-media release. See I007 iteration and docs/validation/I007-HUMAN-ACCEPTANCE.md; Story remains Active.

## I008 project-owned GRUB ISO (VM boot proof verified)

Produce a hashed engineering ISO packaging the existing pinned Linux kernel and Rust/Talos SDK guest. Prove actual ISO-mediated BIOS and OVMF UEFI boot to the read-only multi-turn Talos session, not just a menu marker or QEMU direct kernel boot. This slice excludes the experimental root-shell recovery, full rootfs, production configuration and real/third-party Ventoy boot. Acceptance and human review live in [I008](../../iterations/I008-live-iso-boot.md); full Story C stays Active until product-level requirements are met.

Source head `11ba5b08` passed [I008 CI 42180463023](https://github.com/wjhuang88/vita/actions/runs/42180463023), with BIOS and OVMF UEFI both booting the generated ISO into actual Linux/Talos Tool turns. I005, I006 and I007 original-head regressions passed. Expanded exact-head I001 boot-substrate regression is pending before PR merge. This does not close VITA-001-C; the product rootfs, secure offline recovery, config and real-device integration are still open.
