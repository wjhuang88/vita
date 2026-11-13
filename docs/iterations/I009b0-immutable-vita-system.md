# I009b0 — Immutable Vita application tree and writable runtime proof

Status: **Active (published design baseline; guest evidence pending)**
Published: 2026-11-14
Story: [VITA-001-C](../backlog/active/VITA-001-C-live-os.md); parent [I009 Issue #12](https://github.com/wjhuang88/vita/issues/12)
Depends on: [I009a](I009a-init-lifecycle.md) VM-only Complete, merged [PR #14](https://github.com/wjhuang88/vita/pull/14).
Decisions: [ADR-0008](../decisions/ADR-0008-linux-init-service-supervision.md), [Vita OS design contract](../reference/VITA-OS-DESIGN.md).

## Intent and precise ownership

I009b's overall goal is a **minimal, complete Linux foundation** with a genuinely immutable system and separate writable state. The first bounded step, **I009b0**, converts the existing I009a initramfs image into an explicit **read-only Vita application subtree** without prematurely claiming the **entire root filesystem** has become read-only.

The kernel and initramfs stay unchanged in provenance; the Rust PID 1 continues as a **candidate**, not accepted production init. The shipped Talos binary remains in `/vita`. Early PID 1 mounts `/vita` as a self-bind then bind-remounts it read-only; guest checks its actual mount table and rejects a write attempt with `EROFS` as **root** and again from the **UID 1000** session launcher. A file-mode-only check does not satisfy this proof. `/run` and `/tmp` remain distinct explicitly bounded tmpfs; no persistent state or host disk is mounted.

## Machine-observable acceptance

- [ ] Exact pinned Alpine virt 3.24.3 kernel, static-musl Rust Vita init and embedded Talos SDK included in **Vita-owned GRUB ISO**, with root-owned initramfs entries; all input image/binary hashes recorded.
- [ ] Both BIOS and OVMF UEFI QEMU **boot from the actual ISO** to native non-root Talos, perform two read-only Tool turns and a handled error. No QEMU guest disk, network, physical USB write or privileged recovery shell.
- [ ] Inside the Linux guest, the `/vita` mount itself shows `ro` in the kernel mount table; a root-level attempt to create `/vita/.vita-immutable-probe` fails with `EROFS`, not merely `EACCES` due to permission bits. Talos session UID 1000 independently observes `EROFS`.
- [ ] Guest `/run` and `/tmp` remain `tmpfs` with explicit `size=16m` each; session can write into a defined `/tmp` scratch file, then remove it. No write is attempted to host/internal storage.
- [ ] Verify intentional static architecture: `/init`, `/vita/vita-interaction-prototype` owned by root, no group/world writes; no recovery shell bundled. No extra normal runtime daemon, Python/Node/JVM, or shell command for the default Agent session.
- [ ] Run existing I001/I005/I006/I007/I008/I009a regression and complete separately executing single-maintainer AI code review on exact final PR SHA; record proof and errors. An untested assumption does not count as Complete.

## Exclusions and next steps

**I009b0 is NOT a read-only rootfs**: the initramfs root itself remains a writable RAM-based rootfs under PID 1, and kernel/rootfs verification, `switch_root`, cgroups/device events, real multi-user accounts, Wi-Fi/filesystems, and software/firmware inventories remain **I009b1+**. An entire versioned immutable SquashFS image and separate volatile/persistent-data policy require another reviewed design and boot test. Do **not** claim A/B rollback, hardware secure boot, product-grade PID 1 or an authenticated recovery path.

No new permanent package dependency or destructive media operations are authorized by this iteration. Fault/recovery tests must preserve the already merged I009a evidence and keep human security/licensing/hardware gates **PENDING**.

## Evidence

PENDING; commit and precise QEMU CI run must be attached before moving to Review/Complete.
