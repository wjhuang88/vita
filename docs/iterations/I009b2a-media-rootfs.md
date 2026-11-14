# I009b2a — read-only ISO media source for the Vita SquashFS root

Status: **Complete — disposable QEMU media-root engineering slice; human product/release gates PENDING**
Completed: 2026-11-14
Parent: [I009b2 Issue #21](https://github.com/wjhuang88/vita/issues/21), [I009 #12](https://github.com/wjhuang88/vita/issues/12), [VITA-001-C](../backlog/active/VITA-001-C-live-os.md)
Prerequisite: I009b1 [PR #19](https://github.com/wjhuang88/vita/pull/19) merged. ADR: [ADR-0008](../decisions/ADR-0008-linux-init-service-supervision.md).

## Bounded architecture

Keep the validated Alpine kernel, Rust-only stage1/permanent PID1, native non-root Talos SDK, SHA-verified SquashFS and existing lifecycle tests. **Change only the root image source**: the image is a separate file under `/vita/system.squashfs` on the Vita-owned **readonly ISO9660 media**, and is **absent from stage1 initramfs**. The Rust early init has a compile-time-only `VITA_ROOT_SOURCE=media` mode; its old I009b1 embedded-image mode remains the default and must pass its regression.

The zero-guest-disk QEMU fixture uses the explicit optical device `/dev/sr0`, never a disk scan or Agent/model-supplied path. Before accepting it, stage1 checks the ISO9660 primary volume descriptor and the fixed volume label `VITA_I009B2`, mounts the fixed media read-only, independently checks its kernel mountinfo, SHA-256 hashes the exact SquashFS file against the hash **embedded into stage1**, then attaches a read-only loop device and moves both the media and OS mounts across the PID1 root handoff.

The permanent PID1 still verifies the **actual** readonly SquashFS root and root/UID1000 EROFS, then starts Talos under a controlling TTY. The pinned Alpine ISO's matching kernel modules for ISO9660, optical block, ATA/ATAPI, loop and SquashFS are extracted with source provenance and loaded in dependency order. We do not assume this module set is complete until actual firmware guest proof passes.

## Exact acceptance (VM engineering only, all five proven)

- [x] BIOS and OVMF UEFI QEMU boot from actual Vita ISO containing a **separate readonly SquashFS file**, absent from the initramfs. Guest confirms ISO9660 `/dev/sr0`, volume ID, readonly mount, image SHA, root `/` squashfs ro, stage1 PID1 continuity and native UID1000 Talos Tool success.
- [x] Existing original I009b1 embedded-image flow still passes both firmware and fault regressions unchanged.
- [x] Actual media-backed ISO lifecycle: Ctrl+C, bounded crash restart, injected spawn failure, fatal adopted child reaping and running-session SIGTERM all pass without shell/root bypass.
- [x] Negative media ISO: alter `/vita/system.squashfs` **after compiling stage1**; guest must fail SHA check with no stage2 or Talos.
- [x] Measured sizes, module inventory, kernel/Rust binaries SHA and guest serial evidence archived against the exact PR HEAD. Separate independent AI reviewer findings resolved and all required CI green.

## Explicit non-goals / human gates

**The firmware boot path/GRUB ISO is not a cryptographically verified trust chain**, regardless of embedded root SHA. No production USB support, arbitrary optical media discovery, user persistence, encrypted state, updates, full device support, poweroff policy, unattended host-disk access or release claim. Later I009b2 slices own boot latency/RSS/daemon/footprint comparisons and Rust vs Dinit/Finit under a *common* guest workload, not this first media-source slice. [Issue #5](https://github.com/wjhuang88/vita/issues/5) remains independent. Source/SBOM, security, physical USB and firmware human acceptance stay **PENDING**.

## Proof log

**Verified.** [Code PR #22](https://github.com/wjhuang88/vita/pull/22) merged to `main` as `f0788fc04b281a4030a61052fbefbf892cc241d8`. Final code HEAD `29297be88ea00790ac2243564fb89a6635e50dc2` passed **19/19 exact-HEAD checks** and all nine workflow runs, including separate Copilot code review with no unresolved issues. The last reviewer said **"Needs a closer look"** for product-level PID1/security concerns, not "human approved"; release/manual checks remain unchecked.

**Actual guest and provenance:** [I009b2a run 43558241910](https://github.com/wjhuang88/vita/actions/runs/43558241910) and [artifact 14099281966](https://github.com/wjhuang88/vita/actions/runs/43558241910/artifacts/14099281966) retain BIOS/UEFI serial, module SHA/ABI manifests, exact root SHA, image contents and byte counts.
- Both BIOS and OVMF UEFI emit `VITA_I009B2_MEDIA_ROOTFS_GUEST_OK mode=bios|uefi`. Guest `VITA_I009B2_MEDIA_SOURCE_OK device=/dev/sr0 fstype=iso9660 readonly=1` confirms explicit readonly optical media with `VITA_I009B2` volume ID; the **root-image file is absent from early initramfs**, remains on ISO, and survives `VITA_I009B2_MEDIA_MOUNT_MOVED_OK` across PID1 handoff.
- Root image SHA-256 inside stage1: `2a6d06c06d70a45dff575463973b6c5a277400eee88773496921cb2d480689fd`; real SquashFS `/` mounted `ro`, `EROFS` on root and UID1000 writes. Embedded Talos executes two readonly Tool turns and a handled error, with proper nonroot controlling TTY.
- The **media-backed** path passes Ctrl+C, three-crash budget, injected spawn failure/reaper, fatal adopted-child reaping (distinct `468,469`), and active Talos SIGTERM without restart: `VITA_I009B1_SWITCHED_ROOT_LIFECYCLE_VM_OK`.
- A modified **ISO-resident** SquashFS fails the embedded SHA check: `VITA_I009B1_TAMPERED_ROOT_REJECTED_VM_OK`, `VITA_I009B1_STAGE1_FAIL_CLOSED: immutable root image SHA-256 mismatch`, no stage2/Talos.
- The unchanged I009b1 embedded-root route also passed all corresponding regressions on the same code HEAD. Source module inventory: pinned Alpine kernel `6.18.58-0-virt`; `loop.ko`, `squashfs.ko`, `isofs.ko`, `cdrom.ko` and `sr_mod.ko` (duplicate cdrom dependency permitted); `ata_piix` built in. SHA/ABI manifests retained.

**Measured built artifacts (identical project source and root image contents):**

| Artifact | I009b1 embedded | I009b2a ISO-backed |
| --- | ---: | ---: |
| root SquashFS | 5,074,944 B (4.84 MiB) | 5,074,944 B (4.84 MiB) |
| early initramfs | 5,429,179 B (5.18 MiB) | 501,184 B (0.48 MiB) |
| full Vita ISO | 30,988,288 B (29.55 MiB) | 31,137,792 B (29.69 MiB) |

Initramfs shrank **90.77%**, while complete ISO grew **149,504 bytes (+0.48%)**. This measures **artifact sizes only**; runtime peak/idle RAM, boot-to-Talos latency, daemon counts and real hardware are **not yet measured**. Those are required in the remaining [I009b2 #21](https://github.com/wjhuang88/vita/issues/21), along with comparable Rust/Dinit/Finit guest workloads and a separate init-selection ADR supplement.

**Safety/approval:** no writable guest disks, guest network, physical USB writes, host disk scanning, model-root shell, production security signing or automatic persistence. Optical device `/dev/sr0` is a **QEMU fixed-device fixture**, not general hardware discovery. Firmware verified boot, source-license/SBOM, authenticated recovery, human security, portability and release approvals remain **PENDING**.

## Integration note — ISO9660 vs isofs module

Initial media CI failed before QEMU because the module manifest mistakenly requested `iso9660` as a **module name**, while Linux uses the `isofs` driver for the filesystem type `iso9660`. The dependency-plan request now uses `isofs`; the guest mount remains `iso9660`. This was the **initial failure diagnosis**; the corrected final code HEAD subsequently passed BIOS/UEFI media guest CI (see Proof log).

## Independent AI review round 1 — ISO creation and lifecycle labeling

The separately running Copilot reviewer caught an invalid `grub-mkrescue -V` invocation (it prints **version**, not volume ID), missing fixed volume labels on the crash/spawn/fatal/shutdown test ISOs, and an omitted direct CI dependency on the shared serial observer. The code now forwards xorriso's `-volid VITA_I009B2` via `grub-mkrescue ... -- -volid ...` for **all** six generated ISO variants; module naming uses `isofs` (mount filesystem type remains `iso9660`). The media workflow also triggers on `observe_i006_guest_session.py`. **Historical note:** these were pending during implementation; the final exact-HEAD CI and independent re-review later completed successfully (see Proof log).
