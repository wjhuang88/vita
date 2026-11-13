# I009b0 — Immutable Vita application tree and writable runtime proof

Status: **Complete (VM-only immutable /vita application tree proof; full readonly rootfs and human gates remain OPEN)**
Completed: 2026-11-14
Published: 2026-11-14
Story: [VITA-001-C](../backlog/active/VITA-001-C-live-os.md); parent [I009 Issue #12](https://github.com/wjhuang88/vita/issues/12)
Depends on: [I009a](I009a-init-lifecycle.md) VM-only Complete, merged [PR #14](https://github.com/wjhuang88/vita/pull/14).
Decisions: [ADR-0008](../decisions/ADR-0008-linux-init-service-supervision.md), [Vita OS design contract](../reference/VITA-OS-DESIGN.md).

## Intent and precise ownership

I009b's overall goal is a **minimal, complete Linux foundation** with a genuinely immutable system and separate writable state. The first bounded step, **I009b0**, converts the existing I009a initramfs image into an explicit **read-only Vita application subtree** without prematurely claiming the **entire root filesystem** has become read-only.

The kernel and initramfs stay unchanged in provenance; the Rust PID 1 continues as a **candidate**, not accepted production init. The shipped Talos binary remains in `/vita`. Early PID 1 mounts `/vita` as a self-bind then bind-remounts it read-only; guest checks its actual mount table and rejects a write attempt with `EROFS` as **root** and again from the **UID 1000** session launcher. A file-mode-only check does not satisfy this proof. `/run` and `/tmp` remain distinct explicitly bounded tmpfs; no persistent state or host disk is mounted.

## Machine-observable acceptance

- [x] Exact pinned Alpine virt 3.24.3 kernel, static-musl Rust Vita init and embedded Talos SDK included in **Vita-owned GRUB ISO**, with root-owned initramfs entries; all input image/binary hashes recorded.
- [x] Both BIOS and OVMF UEFI QEMU **boot from the actual ISO** to native non-root Talos, perform two read-only Tool turns and a handled error. No QEMU guest disk, network, physical USB write or privileged recovery shell.
- [x] Inside the Linux guest, the `/vita` mount itself shows `ro` in the kernel mount table; a root-level attempt to create `/vita/.vita-immutable-probe` fails with `EROFS`, not merely `EACCES` due to permission bits. Talos session UID 1000 independently observes `EROFS`.
- [x] Guest `/run` and `/tmp` remain `tmpfs` with explicit `size=16m` each; session can write into a defined `/tmp` scratch file, then remove it. No write is attempted to host/internal storage.
- [x] Verify intentional static architecture: `/init`, `/vita/vita-interaction-prototype` owned by root, no group/world writes; no recovery shell bundled. No extra normal runtime daemon, Python/Node/JVM, or shell command for the default Agent session.
- [x] Run existing I001/I005/I006/I007/I008/I009a regression and complete separately executing single-maintainer AI code review on exact final PR SHA; record proof and errors. An untested assumption does not count as Complete.

## Exclusions and next steps

**I009b0 is NOT a read-only rootfs**: the initramfs root itself remains a writable RAM-based rootfs under PID 1, and kernel/rootfs verification, `switch_root`, cgroups/device events, real multi-user accounts, Wi-Fi/filesystems, and software/firmware inventories remain **I009b1+**. An entire versioned immutable SquashFS image and separate volatile/persistent-data policy require another reviewed design and boot test. Do **not** claim A/B rollback, hardware secure boot, product-grade PID 1 or an authenticated recovery path.

No new permanent package dependency or destructive media operations are authorized by this iteration. Fault/recovery tests must preserve the already merged I009a evidence and keep human security/licensing/hardware gates **PENDING**.

## Evidence — original implemented source head `24899c9e83618c154a426aee45b2e89da221f52e`

- **Guest run:** [I009b0 real-ISO QEMU workflow 43463575936](https://github.com/wjhuang88/vita/actions/runs/43463575936) completed **success** on exact source HEAD `24899c9`. Both BIOS and OVMF UEFI booted the generated Vita-owned ISO and emitted `VITA_I006_LINUX_TALOS_GUEST_OK turns=2 errors=1` followed by `VITA_I009B0_ISO_GUEST_TALOS_OK mode=bios` or `mode=uefi`, and `VITA_I009B0_IMMUTABLE_VITA_ISO_VM_OK`.
- **Actual guest invariants required by the CI observer:** `VITA_I009_ROOTFS_OWNER_OK`, `VITA_I009B0_SYSTEM_MOUNT_READONLY_OK path=/vita result=EROFS`, `VITA_I009B0_NONROOT_SYSTEM_WRITE_DENIED uid=1000 errno=EROFS`, `VITA_I009B0_VOLATILE_TMP_OK uid=1000`, `VITA_I009_TALOS_UID=1000`, `VITA_I009_CONTROLLING_TTY_OK` and `VITA_I009_ORPHAN_REAPED_OK`. The markers are emitted **only after** Linux mountinfo and actual write syscall checks, not by synthetic test-side printing.
- **Build/provenance:** pinned Alpine virt 3.24.3 kernel SHA, non-dynamic static-musl Vita init/Talos host binaries, root-owned cpio, generated ISO checksum and bounded guest serial logs retained in workflow artifacts. 7 Rust unit tests passed.
- **Existing regressions:** I001, I005, I006, I007, I008 and full I009a (including crash, fatal, TTY, UID and shutdown) all passed on the original code head; original 14/14 GitHub check conclusions success.
- **Independent review:** the first Copilot Code Review reported no inline issues but requested actual VM proof; the second review, after the VM proof, flagged an inconsistent Story/iteration/PR evidence status rather than a runtime defect. That documentation inconsistency is being corrected here. **Final documentation HEAD `f57c53d45f88f6f96a7d930cc1043ded8246cb45` passed all 14 exact-HEAD checks**, including full I001/I005/I006/I007/I008/I009a/I009b0 regressions and separate Copilot review execution. The reviewer's latest text was **'Needs a closer look'** (not an AI or human approval), with **no open issues**; the requested final-HEAD guest/lifecycle results were independently verified and recorded in the PR merger disposition. The Story/iteration status inconsistency was corrected and its thread resolved. [PR #16](https://github.com/wjhuang88/vita/pull/16) was merged as `a7a241cb417fed68485de94e4c3380eb7c23f71a`. Human physical/security/GPL/firmware/release gates remain **PENDING**.
- **Explicit distinction:** this validates a **read-only `/vita` bind mount** and volatile `/tmp`; it does **not** verify immutable `/` / SquashFS, `switch_root`, a product image, Dinit/Finit selection, real hardware, or security/legal/release manual gates. Full rootfs follow-up [I009b1 Issue #17](https://github.com/wjhuang88/vita/issues/17) remains open.


## Remaining work ownership

True immutable root `/` with separately built SquashFS (or justified EROFS), early initramfs and `switch_root`, kernel module and firmware provenance, and root EROFS tests are owned by [I009b1 Issue #17](https://github.com/wjhuang88/vita/issues/17). Dinit/Finit comparison, full services/network/device support, persistence, authorization and offline recovery are not part of this VM-only slice. I009 / VITA-001-C remain open; no permanent Rust PID1 selection or product release.
