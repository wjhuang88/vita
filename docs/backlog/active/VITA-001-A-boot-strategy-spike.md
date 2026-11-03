# VITA-001-A — Spike: select portable boot strategy

Status: Active

## Question
Which boot architecture best supports both ISO loop/chain boot and a project-owned Live OS while keeping installation, upgrade and recovery maintainable?

## Timebox
One focused investigation iteration.

## Options compared
- Reuse/integrate Ventoy as the ISO boot substrate.
- GRUB2-based project-owned boot menu and loopback/chainloading strategy.
- Hybrid architecture where a third-party boot substrate handles ISO compatibility and the project owns Live OS/session integration.
- Fully custom substrate only if evidence shows a material advantage.

## Evidence gathered
See `../../reference/BOOT-STRATEGY-RESEARCH.md`.

Evidence snapshot on 2026-11-03 supports a Phase 1 hybrid architecture:
- pinned, unmodified Ventoy owns early boot and heterogeneous image compatibility;
- Vita owns the default Live ISO, boot-menu configuration, portable runtime data, OS interaction layer and release policy;
- pure project-owned GRUB is rejected for Phase 1 because generic ISO loopback boot still depends on distro/initramfs behavior, creating a compatibility burden unrelated to Vita's core differentiation;
- Secure Boot remains a separate validation/product-claim track.

Decision: `../../decisions/ADR-0004-hybrid-ventoy-boot-substrate.md`.

## Current prototype
- `../../../prototypes/boot/ventoy/ventoy.json` demonstrates the planned product configuration boundary.
- `/iso` is the Ventoy search root.
- `/iso/vita/vita-live.iso` is the default image path.
- User ISO images remain dynamically discoverable under `/iso`.

## Validation blocker
The execution environment used for this pass does not provide `qemu-system-x86_64`, GRUB tooling, `xorriso`, or squashfs tooling. No VM boot result is inferred from documentation.

The next spike action is a disposable raw-disk QEMU proof. Physical-media writes remain prohibited until that passes.

## Output / Acceptance
- [x] Recommended approach and rejected options with reasons.
- [ ] QEMU proof-of-concept for the minimum boot path or documented blocker. **Blocker documented; proof still required before Complete.**
- [x] ADR records the selected boot substrate and reversal trigger.
- [x] VITA-001-B/C/H dependencies remain explicit: B/C stay blocked until VM proof closes this Spike; H owns the later reproducible build/release matrix.

## Residual unknowns
- BIOS versus UEFI-first product support matrix.
- Secure Boot end-to-end behavior and signing policy.
- Exact automated raw-image construction path.
- Physical firmware compatibility after VM gates pass.
