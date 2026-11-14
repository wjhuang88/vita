# VITA-001-C — Operator can boot the project Live OS

Status: Active (I008 ISO firmware proof merged; I009a minimal init/session lifecycle selected; full Live OS remains open)


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

## Accepted architecture and next slice (2026-11-07)

[ADR-0008](../../decisions/ADR-0008-linux-init-service-supervision.md) and the [Vita OS design contract](../../reference/VITA-OS-DESIGN.md) establish a **small yet functionally complete**, Rust-first Agent-native Linux product. PID 1, non-root Talos session and privileged Broker are separate failure/trust domains. Alpine/musl remains the experimental base; **Rust PID 1 vs Dinit/Finit remains an open comparative decision**; do not declare Debian/systemd a default. ChromeOS is a layering and immutable-system reference only. [I009a](../../iterations/I009a-init-lifecycle.md) is the next bounded VM-only slice. I008 was merged in [PR #11](https://github.com/wjhuang88/vita/pull/11) after all nine exact-head checks passed. This does not close Story C or human/security/physical/Ventoy gates.

## I009a historical development record (superseded by final closure below)

Separate minimal Rust PID 1 candidate with non-root session/controlling TTY, bounded crash budget, orphan reaping, and real ISO firmware guest tests is proposed by the I009a PR. **No successful I009a guest proof is claimed until exact-head CI completes.** The decision between Rust PID 1 and Dinit/Finit stays OPEN, and full rootfs/recovery/physical gates remain unchanged.

## I009a historical original-head proof (final CI later passed; see closure below)

Initial source `333e8da54c59bbd13750726ccc8782c2a7ae99b6` passed [I009a CI 42373604370](https://github.com/wjhuang88/vita/actions/runs/42373604370), including BIOS/UEFI ISO Talos Tool execution, non-root controlling TTY, orphan reaping and bounded crash fail-safe; all I001/I005/I006/I007/I008 original-head regressions passed. The evidence and security boundary are in [I009a](../../iterations/I009a-init-lifecycle.md). A small final source/documentation follow-up requires repeated exact-head CI before merging PR #14. Story C remains Active; final PID1 choice and product-level rootfs/recovery open.

## I009a VM engineering slice complete (2026-11-12)

[I009a PR #14](https://github.com/wjhuang88/vita/pull/14) merged as `40703ff942cd1eeb8bf051b62d9cbfcc97c9d60a`, after multiple independent Copilot review rounds, resolved inline issues and 12/12 final-head checks. [Final I009a CI](https://github.com/wjhuang88/vita/actions/runs/43256546630) verifies the real BIOS/UEFI ISO, non-root Talos controlling TTY, child/orphan/fatal reaping, bounded crash policy, root-owned system image, Ctrl+C and live-session SIGTERM shutdown with no respawn. The single-maintainer AI review policy is now documented in [PAIRING-WORKFLOW](../../sop/PAIRING-WORKFLOW.md).

**Story C remains Active.** This closes only I009a's disposable diskless VM proof. I009b rootfs/services/device/init benchmarking and selection, I009c typed OS capability/broker, and I009d recovery/firmware/hardware/release are not done. Independent AI code review is not physical/security or release approval.

## I009b0 boundary checkpoint (2026-11-14)

First I009b increment: [I009b0 iteration](../../iterations/I009b0-immutable-vita-system.md) implements a **read-only `/vita` application mount**, with root and UID 1000 `EROFS` rejection and bounded volatile `/run` and `/tmp`. Real BIOS/UEFI guest evidence on **original code HEAD `24899c9`** passed in [I009b0 CI 43463575936](https://github.com/wjhuang88/vita/actions/runs/43463575936) including two Tool turns/one handled error; the **final documentation HEAD `f57c53d`** also passed 14/14 checks, and [PR #16](https://github.com/wjhuang88/vita/pull/16) merged to `main` as `a7a241cb417fed68485de94e4c3380eb7c23f71a` after separately executed Copilot reviews and verified guest evidence (no outstanding actionable threads). It is **not** a whole immutable Linux rootfs: that belongs to I009b1+ with actual image mounting/switch-root, driver/module selection and rootfs-provenance proof. I009a stays Complete; Story C stays Active. Final PID 1 choice versus Dinit/Finit remains open. Security, source-license and physical-USB review remain pending.


## I009b0 status closure (2026-11-14)

[I009b0 iteration](../../iterations/I009b0-immutable-vita-system.md) is **Complete only for the VM-only read-only `/vita` subtree**; kernel mounts deny root and UID1000 writes with `EROFS`, while volatile runtime remains usable and native Talos completes both firmware ISO boots. Original and final source heads each passed 14/14 checks. Full rootfs `/` is **still writable RAM-based initramfs**; [I009b1 #17](https://github.com/wjhuang88/vita/issues/17) tracks actual immutable image/switch_root and module inventory. Story C stays **Active**; the Rust PID1 candidate is not the final selection. Security, licensing, hardware and physical-media human acceptance remain **PENDING**.

## I009b1 independent SquashFS root and Rust handoff (2026-11-14)

[Issue #17](https://github.com/wjhuang88/vita/issues/17) / [I009b1 owner](../../iterations/I009b1-rootfs-stage1.md): genuine read-only rootfs and tiny Rust stage1->permanent PID1 handoff are being implemented and **not yet accepted** until BIOS/UEFI real-ISO guest plus exact-head AI review and CI. Input kernel and module sources are fixed to the existing Alpine ISO. The image is carried in RAM inside initramfs for a diskless fixture; not a full hardware rootfs, signed boot or final init selection.
