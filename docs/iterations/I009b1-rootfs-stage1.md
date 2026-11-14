# I009b1 — genuine read-only Linux rootfs and stage1/PID1 continuity

Status: **Complete — VM-only engineering slice; product/hardware/release acceptance PENDING**
Completed: 2026-11-14
Published: 2026-11-14
Owner: [I009b1 Issue #17](https://github.com/wjhuang88/vita/issues/17), [I009 #12](https://github.com/wjhuang88/vita/issues/12), [VITA-001-C](../backlog/active/VITA-001-C-live-os.md)
Prerequisite: [I009b0](I009b0-immutable-vita-system.md) Complete and merged PR #16.

## Immutable bounded scope

Create a **separate SquashFS root image** with only the root-owned permanent Rust init, native Talos SDK host, normal userspace mountpoints and minimal identities. Include it in a disposable cpio stage1 payload for a zero-guest-disk QEMU proof. A distinct Rust stage1 validates the exact SHA-256 embedded at compile time, loads only kernel-ABI-matched modules from the **same SHA-pinned Alpine ISO**, attaches a fixed read-only loop, mounts SquashFS, moves the early Linux mounts and \`exec\`s the permanent Rust init **without changing PID1**.

The stage2 Rust init independently checks the kernel mount table identifies / as SquashFS, confirms root write attempts return EROFS, and then launches the native non-root Talos session. No shell or model in stage1, no automatic persistence or physical disk discovery.

## Required executable evidence

- [x] Real BIOS and OVMF UEFI boot from the Vita-owned generated ISO, not QEMU -kernel. Native Talos UID1000, controlling TTY, two Tool turns and handled error.
- [x] Actual guest image SHA-256 confirmation; root filesystem mount type SquashFS, mount option ro, and root write EROFS; PID1 continuity across switch_root and kernel module ABI inventory.
- [x] Bounded /run and /tmp tmpfs independent from read-only root; separate root image, cpio and ISO SHA manifests, exact kernel/module provenance.
- [x] Existing I001/I005/I006/I007/I008/I009a/I009b0 regressions, independent AI reviewer on exact code HEAD, CI and review comments closed.
- [x] Recorded image bytes, early stage/module inventory, Rust runtime binary count and explicit human license/security/firmware/USB gates kept OPEN.

## Excluded / future iterations

This is not a full Alpine userspace or production Live OS. Root image is still **held inside initramfs memory** for first proof, not mapped efficiently from read-only ISO media. No device hotplug, Wi-Fi, package manager, A/B recovery, production real-device poweroff, signing or upstream-source compliance sign-off. Rust PID1 still must be compared with Dinit/Finit under shared workload. Ventoy third-party ISO blocker Issue #5 is distinct.

## Evidence

**COMPLETE for the bounded diskless-QEMU engineering scope** — evidence below; broader Linux/human gates remain OPEN.

## Explicit root vs non-root EROFS proof

The permanent PID 1 verifies that root's attempt to create a file directly under `/` returns EROFS, and the UID1000 Talos session helper checks a **test-only mode 0777 directory** embedded inside SquashFS. If the root had been incorrectly mounted read-write, the latter creation would be permitted by file permissions; accepting only EROFS therefore excludes a misleading EACCES-only check. The test-only directory must not be incorporated into the production filesystem schema. Both guest markers are mandatory on BIOS and UEFI, and passed on the final exact code HEAD.

## Independent AI review round 1: required negative and lifecycle verification

The separate GitHub Copilot reviewer flagged three actionable issues: (HIGH) `modprobe` needs `--all` to resolve **both** loop and squashfs dependencies, (MEDIUM) a negative guest whose root image is altered *after* embedding the original SHA in stage1, and (MEDIUM) full I009a-style Ctrl+C, crash budget, spawn failure, fatal-reaper and running-session SIGTERM tests on the **new stage1/SquashFS** path. All three scenarios subsequently passed exact-HEAD QEMU, including a changed image's SHA mismatch and no stage2/Talos markers. Copilot's latest independent review reports **no open issues** but retains a product-level human safety review recommendation.

## Final I009b1 evidence and precise closure (2026-11-14)

- [Implementation PR #19](https://github.com/wjhuang88/vita/pull/19) **merged** to main as `96713752b2b157760543d695bbb2fee7f6ad76a3`. Reviewed exact source HEAD: `b1253740e0538a7d10286f85e9984d96556eeac0`. **17/17 exact-head GitHub checks and 9 workflow runs succeeded**; all Copilot review threads resolved. Latest independent reviewer text: **"Needs a closer look", no open issues** — not human product/security approval.
- [Real ISO guest run 43555621207](https://github.com/wjhuang88/vita/actions/runs/43555621207): BIOS/OVMF UEFI reached native Talos with two read-only Tool turns and one handled error, and verified actual SHA-256-matched `squashfs` root filesystem mounted read-only at `/`. Both root and UID 1000 writes returned kernel `EROFS`; PID1 continued through stage1 `MS_MOVE + chroot + exec`, and the non-root Talos controlling TTY survived.
- **Negative proof:** deliberately modify the root image after compiling stage1 with the original SHA, then boot a separate ISO. `VITA_I009B1_TAMPERED_ROOT_REJECTED_VM_OK` and `VITA_I009B1_STAGE1_FAIL_CLOSED: immutable root image SHA-256 mismatch` were observed; no successful stage2/Talos marker.
- **Post-handoff lifecycle:** `VITA_I009B1_SWITCHED_ROOT_LIFECYCLE_VM_OK` covers Ctrl+C, bounded three-session crash policy, injected spawn failure, fatal reaper (two distinct PIDs), and SIGTERM of a running Talos session without restart. Seven permanent-init and two stage1 unit tests passed; I001/I005/I006/I007/I008/I009a/I009b0 exact-head regression passed.
- **Provenance/footprint:** pinned Alpine `6.18.58-0-virt` kernel with exactly `loop` and `squashfs` ABI-matched loadable modules (source/module SHA manifests retained). Root SquashFS **5,074,944 bytes / 4.84 MiB**; stage1 initramfs **5,429,179 bytes / 5.18 MiB**; generated ISO **30,988,288 bytes / 29.55 MiB**. Binary/image/module manifests stored in the QEMU workflow artifact.
- **Limits:** image still resides in RAM-backed initramfs rather than efficient ISO/USB-media mapping. Full driver/network/hotplug service coverage, measured boot latency and idle RAM, Dinit/Finit init comparison, true firmware verified boot, product recovery/permissions, license/SBOM obligations, hardware and release signoffs are **NOT COMPLETE**. [Issue #17](https://github.com/wjhuang88/vita/issues/17) and Story C remain OPEN for those gates.
