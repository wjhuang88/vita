# I009b2a — read-only ISO media source for the Vita SquashFS root

Status: **Active / implementation candidate, no guest evidence yet**
Parent: [I009b2 Issue #21](https://github.com/wjhuang88/vita/issues/21), [I009 #12](https://github.com/wjhuang88/vita/issues/12), [VITA-001-C](../backlog/active/VITA-001-C-live-os.md)
Prerequisite: I009b1 [PR #19](https://github.com/wjhuang88/vita/pull/19) merged. ADR: [ADR-0008](../decisions/ADR-0008-linux-init-service-supervision.md).

## Bounded architecture

Keep the validated Alpine kernel, Rust-only stage1/permanent PID1, native non-root Talos SDK, SHA-verified SquashFS and existing lifecycle tests. **Change only the root image source**: the image is a separate file under `/vita/system.squashfs` on the Vita-owned **readonly ISO9660 media**, and is **absent from stage1 initramfs**. The Rust early init has a compile-time-only `VITA_ROOT_SOURCE=media` mode; its old I009b1 embedded-image mode remains the default and must pass its regression.

The zero-guest-disk QEMU fixture uses the explicit optical device `/dev/sr0`, never a disk scan or Agent/model-supplied path. Before accepting it, stage1 checks the ISO9660 primary volume descriptor and the fixed volume label `VITA_I009B2`, mounts the fixed media read-only, independently checks its kernel mountinfo, SHA-256 hashes the exact SquashFS file against the hash **embedded into stage1**, then attaches a read-only loop device and moves both the media and OS mounts across the PID1 root handoff.

The permanent PID1 still verifies the **actual** readonly SquashFS root and root/UID1000 EROFS, then starts Talos under a controlling TTY. The pinned Alpine ISO's matching kernel modules for ISO9660, optical block, ATA/ATAPI, loop and SquashFS are extracted with source provenance and loaded in dependency order. We do not assume this module set is complete until actual firmware guest proof passes.

## Exact acceptance (not yet met)

- [ ] BIOS and OVMF UEFI QEMU boot from actual Vita ISO containing a **separate readonly SquashFS file**, absent from the initramfs. Guest confirms ISO9660 `/dev/sr0`, volume ID, readonly mount, image SHA, root `/` squashfs ro, stage1 PID1 continuity and native UID1000 Talos Tool success.
- [ ] Existing original I009b1 embedded-image flow still passes both firmware and fault regressions unchanged.
- [ ] Actual media-backed ISO lifecycle: Ctrl+C, bounded crash restart, injected spawn failure, fatal adopted child reaping and running-session SIGTERM all pass without shell/root bypass.
- [ ] Negative media ISO: alter `/vita/system.squashfs` **after compiling stage1**; guest must fail SHA check with no stage2 or Talos.
- [ ] Measured sizes, module inventory, kernel/Rust binaries SHA and guest serial evidence archived against the exact PR HEAD. Separate independent AI reviewer findings resolved and all required CI green.

## Explicit non-goals / human gates

**The firmware boot path/GRUB ISO is not a cryptographically verified trust chain**, regardless of embedded root SHA. No production USB support, arbitrary optical media discovery, user persistence, encrypted state, updates, full device support, poweroff policy, unattended host-disk access or release claim. Later I009b2 slices own boot latency/RSS/daemon/footprint comparisons and Rust vs Dinit/Finit under a *common* guest workload, not this first media-source slice. [Issue #5](https://github.com/wjhuang88/vita/issues/5) remains independent. Source/SBOM, security, physical USB and firmware human acceptance stay **PENDING**.

## Proof log

PENDING — commit/CI and independent-review evidence must be recorded before closure.
