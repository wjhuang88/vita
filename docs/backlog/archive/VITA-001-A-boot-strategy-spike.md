# VITA-001-A — Spike: select portable boot strategy

Status: Complete
Completed: 2026-11-03

## Question
Which boot architecture best supports both ISO loop/chain boot and a project-owned Live OS while keeping installation, upgrade and recovery maintainable?

## Timebox
One focused investigation iteration.

## Options compared
- Reuse/integrate Ventoy as the ISO boot substrate.
- GRUB2-based project-owned boot menu and loopback/chainloading strategy.
- Hybrid architecture where a third-party boot substrate handles ISO compatibility and the project owns Live OS/session integration.
- Fully custom substrate only if evidence shows a material advantage.

## Decision
Select the Phase 1 hybrid architecture recorded in `../../decisions/ADR-0004-hybrid-ventoy-boot-substrate.md`:
- pinned, unmodified Ventoy owns early boot and heterogeneous image compatibility;
- Vita owns the default Live ISO, boot-menu configuration, portable runtime data, OS interaction layer and release policy;
- pure project-owned GRUB is rejected for Phase 1 because generic ISO loopback boot still depends on distro/initramfs behavior;
- Secure Boot remains a separate validation/product-claim track.

Detailed evidence: `../../reference/BOOT-STRATEGY-RESEARCH.md`.

## Executed proof

GitHub Actions run `41659336430`, `boot-chain-prototype`, at commit `68c25efd89aa12ca0a611fbce4ea6605907cbebd`:

```text
Install Ventoy to /dev/loop0 successfully finished.
VITA_BOOT_PROBE_OK mode=bios marker=VITA_BOOT_OK
VITA_BOOT_PROBE_OK mode=uefi marker=VITA_BOOT_OK
VITA_BOOT_PROBE_OK mode=bios marker=VITA_USER_ISO_BOOT_OK
VITA_I001_BOOT_CHAIN_OK ventoy=1.1.18
```

The test uses only a temporary raw file bound to a `/dev/loopN` device and refuses the target unless the kernel-reported backing file exactly matches the temporary image. No physical-media write occurred.

## Output / Acceptance
- [x] Recommended approach and rejected options with reasons.
- [x] QEMU proof-of-concept validates the minimum default Vita boot path in BIOS and x86_64 UEFI.
- [x] A second user ISO remains bootable through the same Ventoy-backed medium.
- [x] ADR records the selected boot substrate and reversal trigger.
- [x] VITA-001-B/C/H dependency gate is released; their own acceptance and remaining refinements still apply.

## Residual work destination
- Representative real Linux ISO compatibility: VITA-001-B.
- Final Live OS kernel/initramfs/rootfs and session handoff: VITA-001-C.
- Reproducible release image, wider firmware/device matrix and physical media: VITA-001-H.
- Secure Boot chain/signing/enrollment: release hardening; no support claim yet.

No remaining item keeps this Spike open.
