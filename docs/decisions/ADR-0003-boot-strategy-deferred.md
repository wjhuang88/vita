# ADR-0003 — Do not select the boot substrate before the boot-strategy Spike

Status: Superseded by `ADR-0004-hybrid-ventoy-boot-substrate.md`

## Context
The product resembles Ventoy in one dimension, but multi-ISO compatibility, licensing, upgrade behavior, UEFI/BIOS support and ownership of the Live OS path may favor different architectures.

## Constraint Decomposition
| Constraint | Type | Source | Can Change? |
| --- | --- | --- | --- |
| USB must boot stored ISO images and the project Live OS | Hard | Product requirement | No |
| Reuse Ventoy specifically | Assumption | Inspiration, not requirement | Yes |
| UEFI-first | Soft | Initial target | Yes |
| Legacy BIOS / Secure Boot support | Assumption | Not yet decided | Yes |

## Reasoning
Prematurely choosing Ventoy or GRUB would make later storage/layout and release work depend on unvalidated compatibility assumptions.

## Decision
Defer final boot-substrate selection to VITA-001-A. No implementation story may claim the boot architecture is settled until that Spike records evidence and supersedes/amends this ADR.

## Supersession
VITA-001-A gathered enough upstream evidence on 2026-11-03 to select the Phase 1 architecture. `ADR-0004-hybrid-ventoy-boot-substrate.md` supersedes this decision. QEMU proof remains an implementation gate for closing the Spike; the architecture selection is reversible if that proof invalidates a critical assumption.

## Reversal Trigger
Superseded: see ADR-0004.
