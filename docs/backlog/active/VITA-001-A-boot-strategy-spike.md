# VITA-001-A — Spike: select portable boot strategy

## Question
Which boot architecture best supports both ISO loop/chain boot and a project-owned Live OS while keeping installation, upgrade and recovery maintainable?

## Timebox
One focused investigation iteration.

## Options to compare
- Reuse/integrate Ventoy as the ISO boot substrate.
- GRUB2-based project-owned boot menu and loopback/chainloading strategy.
- Hybrid architecture where a third-party boot substrate handles ISO compatibility and the project owns Live OS/session integration.
- Other approach only if evidence shows a material advantage.

## Evidence to gather
- License and redistribution implications.
- UEFI/legacy BIOS behavior.
- Secure Boot implications.
- ISO compatibility burden.
- Ability to keep user ISO/config data intact during project upgrades.
- Partition/layout constraints.
- Automation and reproducible image-building support.
- Failure recovery path.

## Output / Acceptance
- [ ] Recommended approach and rejected options with reasons.
- [ ] QEMU proof-of-concept for the minimum boot path or documented blocker.
- [ ] ADR records the selected boot substrate and reversal trigger.
- [ ] VITA-001-B/C/H dependencies are updated.
