# VITA-001-C — Operator can boot the project Live OS

Status: Active (I005 — real kernel / Rust init proof only)


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

- Implemented path (pending CI): verified Alpine kernel -> throwaway initramfs -> statically compiled Vita Rust PID 1 -> real guest serial marker.
- Unimplemented: complete rootfs/service manager, default Talos session, writable runtime/data, physical/UEFI firmware boot, independent offline recovery and distributable image.
- Manual decisions/reviews: [I005 handoff](../../validation/I005-HUMAN-ACCEPTANCE.md). Separate Ventoy real ISO blocker remains [Issue #5](https://github.com/wjhuang88/vita/issues/5).
