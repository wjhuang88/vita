# ADR-0006 — Alpine/musl as first Vita Linux Live base

Status: Accepted for I005 experimental boot foundation, **not a distribution release claim**
Date: 2026-11-04

## Context and classification

The VITA-001-C Story was in Refinement after VITA-001-A resolved the Ventoy substrate. The missing decision was a concrete Live Linux kernel, initramfs and userspace build direction that can be proven independently from the existing Ventoy/third-party ISO compatibility blocker (#5). Vita's value remains its Rust-owned, Talos-powered default interaction session.

| Constraint | Class | Decision |
| --- | --- | --- |
| A real Linux kernel/initramfs/rootfs must eventually boot Vita's session | Hard | Start with independently testable Linux kernel + Rust guest program |
| Talos Runtime must be the default interaction host in the complete Live OS | Hard | I005 marker is **not** the final session |
| No implicit destructive disk writes / physical USB installer | Hard | QEMU direct kernel/initramfs only, no block target |
| Vita-owned core code is Rust-first; minimize other app-language runtimes | Hard/Soft per ADR-0005 | Use static Rust musl in the first guest; no Python, Node or JVM |
| Alpine Linux as initial minimal kernel/userspace packaging base | Soft | Select Alpine Linux virt 3.24.3 as initial source, revisit if unsuitable |
| An all-Rust PID 1 is necessary | Assumption (rejected) | Do not commit to Rust PID 1; this is a throwaway proof |
| UEFI and Ventoy image boot compatibility are solved | Assumption (false/unproven) | Keep firmware and Ventoy gates separate |

## Decision

For the **first experimental Linux boot slice**, use the **upstream Alpine virt 3.24.3 x86_64** release kernel (extracted from the exact previously verified ISO SHA-256 `cdc1bc0efc64d8d611bf3733f54868172a545888dfd726f518bf279bcb0ff1af`) plus a custom disposable initramfs containing only Vita's statically compiled Rust init marker. The prototype proves *real guest Linux user-space execution*, not the final distribution architecture.

The intended next build step is a minimal Alpine/musl userspace (mature upstream init/recovery stack, read-only root base, explicitly mounted writable runtime/data), with the Vita Rust host embedding Talos Runtime as the default interactive process. The exact Alpine package inventory, kernel modules, license manifest and reproducible rootfs format require separate implementation proof before claiming VITA-001-C complete.

The first QEMU smoke boots kernel and initramfs directly, **without Ventoy**, to isolate this new Linux+Rust integration boundary from [Issue #5](https://github.com/wjhuang88/vita/issues/5). A later gate must boot a complete Vita ISO through BIOS/UEFI and from the selected Ventoy substrate. Neither is implied by a direct `-kernel/-initrd` test.

## Runtime dependency inventory for I005

| Component | Shipped in experimental guest | License / provenance |
| --- | --- | --- |
| Linux kernel `vmlinuz-virt` | Yes, extracted from SHA-256-pinned Alpine ISO | Linux GPL-2.0-only kernel distribution; upstream Linux/Alpine packaging and source obligations apply |
| `/init` Vita Rust program | Yes, static musl, Rust standard library | Vita Apache-2.0 code; Rust standard library and musl have their own license obligations |
| `/dev/console` device node | Yes | Linux device, not executable |
| BusyBox/OpenRC/recovery shell | **No**, not yet implemented | Future package/license inventory required |
| Talos runtime / provider / model secrets | **No**, not yet integrated | Future default session |
| xorriso, cpio, curl, QEMU, cargo, toolchain | **Build/CI host only** | Not packaged into initramfs |

No distributable image or source-compliance claim is made from this one-off test fixture.

## Validation, human boundary, reversal

- Require exact upstream ISO SHA-256 before extracting the kernel, logged kernel SHA-256, no dynamically linked interpreter for Rust `/init`, QEMU guest serial marker with `pid=1`, and retained logs.
- Require subsequent CI for the actual minimal rootfs, Talos session launch, UEFI/BIOS menu and upgrade path. A direct Linux kernel proof cannot close the Story.
- Human/live-device questions belong to [I005 manual handoff](../validation/I005-HUMAN-ACCEPTANCE.md); keep any physical write behind the independent target-identity and VM gates.
- Reverse the Alpine-base choice if its licensing, security lifecycle, hardware modules, Talos dependencies, package footprint or live-image boot compatibility fails product requirements. Do not silently switch distributions or pretend a Linux init proof resolves the Ventoy issue.

Related: [ADR-0004](ADR-0004-hybrid-ventoy-boot-substrate.md), [ADR-0005](ADR-0005-rust-first-system-capabilities.md).
