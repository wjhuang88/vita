# I009b1 — genuine read-only Linux rootfs and stage1/PID1 continuity

Status: **Active (engineering implementation, CI and independent review pending)**
Published: 2026-11-14
Owner: [I009b1 Issue #17](https://github.com/wjhuang88/vita/issues/17), [I009 #12](https://github.com/wjhuang88/vita/issues/12), [VITA-001-C](../backlog/active/VITA-001-C-live-os.md)
Prerequisite: [I009b0](I009b0-immutable-vita-system.md) Complete and merged PR #16.

## Immutable bounded scope

Create a **separate SquashFS root image** with only the root-owned permanent Rust init, native Talos SDK host, normal userspace mountpoints and minimal identities. Include it in a disposable cpio stage1 payload for a zero-guest-disk QEMU proof. A distinct Rust stage1 validates the exact SHA-256 embedded at compile time, loads only kernel-ABI-matched modules from the **same SHA-pinned Alpine ISO**, attaches a fixed read-only loop, mounts SquashFS, moves the early Linux mounts and \`exec\`s the permanent Rust init **without changing PID1**.

The stage2 Rust init independently checks the kernel mount table identifies / as SquashFS, confirms root write attempts return EROFS, and then launches the native non-root Talos session. No shell or model in stage1, no automatic persistence or physical disk discovery.

## Required executable evidence

- [ ] Real BIOS and OVMF UEFI boot from the Vita-owned generated ISO, not QEMU -kernel. Native Talos UID1000, controlling TTY, two Tool turns and handled error.
- [ ] Actual guest image SHA-256 confirmation; root filesystem mount type SquashFS, mount option ro, and root write EROFS; PID1 continuity across switch_root and kernel module ABI inventory.
- [ ] Bounded /run and /tmp tmpfs independent from read-only root; separate root image, cpio and ISO SHA manifests, exact kernel/module provenance.
- [ ] Existing I001/I005/I006/I007/I008/I009a/I009b0 regressions, independent AI reviewer on exact code HEAD, CI and review comments closed.
- [ ] Recorded image bytes, early stage/module inventory, Rust runtime binary count and explicit human license/security/firmware/USB gates kept OPEN.

## Excluded / future iterations

This is not a full Alpine userspace or production Live OS. Root image is still **held inside initramfs memory** for first proof, not mapped efficiently from read-only ISO media. No device hotplug, Wi-Fi, package manager, A/B recovery, production real-device poweroff, signing or upstream-source compliance sign-off. Rust PID1 still must be compared with Dinit/Finit under shared workload. Ventoy third-party ISO blocker Issue #5 is distinct.

## Evidence

**PENDING** — actual QEMU, exact HEAD and independent review required. Never infer success from a compile or ISO creation alone.

## Explicit root vs non-root EROFS proof

The permanent PID 1 verifies that root's attempt to create a file directly under `/` returns EROFS, and the UID1000 Talos session helper checks a **test-only mode 0777 directory** embedded inside SquashFS. If the root had been incorrectly mounted read-write, the latter creation would be permitted by file permissions; accepting only EROFS therefore excludes a misleading EACCES-only check. The test-only directory must not be incorporated into the production filesystem schema. Both guest markers are mandatory on BIOS and UEFI. Not verified until new exact-head CI succeeds.

## Independent AI review round 1: required negative and lifecycle verification

The separate GitHub Copilot reviewer flagged three actionable issues: (HIGH) `modprobe` needs `--all` to resolve **both** loop and squashfs dependencies, (MEDIUM) a negative guest whose root image is altered *after* embedding the original SHA in stage1, and (MEDIUM) full I009a-style Ctrl+C, crash budget, spawn failure, fatal-reaper and running-session SIGTERM tests on the **new stage1/SquashFS** path. All three scenarios have been added to the branch CI, but none may be marked Complete until the **final exact-head QEMU evidence and independent re-review** pass. The tamper fixture must show SHA mismatch and no stage2/Talos markers.
