# ADR-0004 — Hybrid boot architecture: Ventoy substrate + Vita-owned Live OS

Status: Accepted for Phase 1; implementation remains gated by QEMU proof in VITA-001-A.

## Context

Vita must boot arbitrary ISO images from removable media and also boot a project-owned Live OS. ISO compatibility is required, but the differentiating product surface is the Vita Live environment and Talos-based interaction runtime rather than bootloader compatibility engineering.

VITA-001-A compared a project-owned GRUB2 path with Ventoy reuse and a hybrid design. GNU GRUB supports loopback image access, but generic ISO boot still depends on the guest OS/initramfs finding its root. Ventoy already owns a broad image-compatibility layer, user-file discovery, BIOS/UEFI support, and data-preserving upgrades.

## Constraint Decomposition

| Constraint | Type | Source | Can Change? |
| --- | --- | --- | --- |
| Stored user ISO images must remain directly bootable | Hard | Product requirement | No |
| Vita Live OS must be a first-class/default boot target | Hard | Product requirement | No |
| User ISO/config data should survive Vita upgrades | Hard | Product requirement | No |
| Talos/Vita runtime architecture must not depend on Ventoy internals | Hard | Product boundary | No |
| Use Ventoy specifically | Soft | Selected Phase 1 implementation | Yes |
| Keep Ventoy unmodified | Soft | Complexity/license-risk reduction | Yes |
| Secure Boot product support | Assumption | Not yet validated end-to-end | Yes |
| Legacy BIOS remains a supported product target | Assumption | Not yet fixed by product scope | Yes |

## Decision

For Phase 1, use a pinned, unmodified Ventoy release as the removable-media boot substrate.

Vita owns everything above that substrate:
- Live OS image and its update lifecycle;
- `/ventoy/ventoy.json` product configuration;
- default boot selection and product naming;
- portable `/talos/` configuration/workspace data;
- Talos-based interactive OS runtime;
- system capability broker and security policy;
- Vita release/test matrix.

The primary data partition exposes `/iso/` for boot images. The Vita Live image lives at a stable path under that tree (initial target `/iso/vita/vita-live.iso`) and is selected as the default through Ventoy global-control configuration. Arbitrary user images remain discoverable under `/iso/` without a generated whitelist.

Do not fork or patch Ventoy in this phase. Releases that redistribute Ventoy must include the applicable GPLv3+ license/source-compliance materials.

Secure Boot is explicitly **not** considered solved by selecting Ventoy. It remains a separate validation/product-claim decision.

## Consequences

Positive:
- Vita avoids owning a distro-specific ISO loopback compatibility matrix.
- The Vita Live image can be developed and replaced independently of the boot substrate.
- User ISO/config data can remain on the ordinary data partition across updates.
- Vita retains full ownership of the Agent-native system experience.

Costs:
- Vita inherits an upstream boot dependency and must pin/test updates.
- Ventoy partition-layout constraints must be respected.
- Third-party license/source compliance becomes part of Vita release engineering.
- Secure Boot behavior depends on both Ventoy and Vita's own image/signing chain and needs separate proof.

## Validation Gate

Before VITA-001-A can be Complete:
1. build/stage a Ventoy-backed Vita disk image in automation or a controlled VM host;
2. boot it through QEMU BIOS and x86_64 UEFI paths selected by the initial support matrix;
3. verify automatic/default boot reaches the Vita Live boot stub/image;
4. verify at least one additional user ISO remains selectable;
5. record exact Ventoy version/hash and commands.

Physical-media testing comes only after VM gates pass.

## Reversal Trigger

Revisit this decision if:
- the QEMU/physical matrix exposes an unacceptable Ventoy limitation;
- Vita requires a measured/signed boot chain Ventoy cannot satisfy;
- upstream maintenance/licensing becomes incompatible with Vita distribution;
- Vita's supported ISO set becomes narrow enough that a project-owned loader is materially simpler;
- product requirements demand boot behavior that cannot be implemented cleanly at the Ventoy configuration/substrate boundary.
