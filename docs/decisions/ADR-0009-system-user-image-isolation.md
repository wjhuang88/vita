# ADR-0009 — Separate Vita system images from user boot images

Status: **ACCEPTED product storage/trust boundary (2026-11-16); physical partition/default boot implementation NOT YET VALIDATED**.
Owner: VITA-001 / VITA-001-B / VITA-001-C; [physical-layout and boot feasibility #27](https://github.com/wjhuang88/vita/issues/27).
Amends the **product data-placement decision** in [ADR-0004](ADR-0004-hybrid-ventoy-boot-substrate.md) and [USB-LAYOUT](../reference/USB-LAYOUT.md), without invalidating I001's historical disposable probe or replacing the selected pinned, unmodified Ventoy compatibility substrate.

## Requirement and conflict

Vita's own operating-system payloads are **product-controlled boot and update assets**. User-imported ISO/IMG/WIM/VHD files are **untrusted, user-controlled boot content**. They must not be stored in the same user-managed image library or rely on the same writable/update domain. The old Phase 1 path `/iso/vita/vita-live.iso` alongside `/iso/<user images>` is therefore **superseded for the product**. A rename to `/iso/system` would not satisfy the desired storage/update isolation.

Existing I001 CI wrote both test ISOs to Ventoy's P1 and demonstrated a historical **boot-substrate probe**, not the future shipping disk layout. I009b2a/b2b booted a QEMU optical ISO (`/dev/sr0`), not a physical multi-partition USB; these experiments also remain valid within their declared scope.

## Accepted invariants

1. **Three ownership domains, not one image directory.** Keep Vita-owned boot files, versioned immutable system rootfs, metadata, and fallback/recovery images in a **dedicated Vita-system storage domain**. Keep user-uploaded boot images in the **Ventoy user-image domain**. Keep portable Talos configuration/workspace/logs in a **distinct user-state namespace**; this may reside on the user-data partition initially, but system updates must never reclassify it as system content.
2. **Partition isolation is the preferred physical product topology.** Preserve Ventoy's first user-data partition **P1** and required Ventoy EFI partition **P2**. Provision a dedicated **P3/VITA_SYSTEM** using an explicitly reserved tail area at initial media creation, not by repurposing P2 or silently repartitioning existing Ventoy media. Treat P3 as protected system-owned storage; normal runtime mounts of its actual OS root are readonly. A dedicated user-state P4 is a separate future decision, not required here.
3. **Small boot-control metadata is not a system image.** Ventoy configuration (e.g., `/ventoy/ventoy.json`) can remain in its upstream-supported location on P1. It may reference an independently verified Vita entry, but **the OS image/kernel/rootfs itself must not be supplied from the user-ISO namespace**. No automatic search for a user filename to replace an absent or corrupt Vita system image.
4. **Separate operations and identity.** User image import/delete/rename operations may change only the user-image domain. System installation/update/rollback targets only an explicitly identified Vita-system volume with ownership, expected disk/partition identity, version and digest validation, and rollback policy. Changes to boot pointers must be staged and validated; never overwrite a user ISO as part of an OS update.
5. **Fail closed.** Missing/ambiguous/tampered P3 must not cause Vita to boot a user ISO as a trusted OS. A malicious image named `vita-live.iso`, a symlink, or a tampered user Ventoy config cannot count as Vita integrity evidence. Bootloader and firmware verification remain separate, mandatory product/security work.
6. **No implicit physical writes.** Media initialization, reserved-space partitioning, upgrade/migration, physical boot tests and any destructive fallback require disposable-loop proof first, then actual media identification, backup/consent and separately documented human approval. No existing user's disk should be changed by this ADR.

## Proposed physical media map — not yet a boot-verified build

```text
USB device
├── P1  VENTOY_DATA       (user-managed storage, ordinary user-writeable)
│   ├── /iso/            user-imported ISO/IMG/WIM/VHD only
│   ├── /talos/          user settings, skills, workspace and opted-in logs
│   └── /ventoy/         upstream-required Ventoy config / small boot pointer
├── P2  VTOYEFI           (Ventoy-owned EFI partition; do not modify)
└── P3  VITA_SYSTEM       (Vita-managed; separate volume)
    ├── boot/            versioned, authenticated boot resources (future)
    ├── versions/        immutable root image and its signed manifest (future)
    ├── recovery/        separately governed recovery assets (future)
    └── state/           system boot-health/version pointers (future)
```

**Names and subdirectories are a logical target contract, not a promise of on-disk binaries or a fixed partition number on already provisioned media.** Filesystem type, disk GUID labels, signing key custody, operational write access and on-media boot-entry layout are open under #27. Do not equate a writable partition with a verified immutable operating system; independently establish the trust chain and an authorized updater.

## Ventoy compatibility and boot feasibility gate

Official Ventoy documentation supports preserving trailing space **at installation time** and creating additional partitions there, while retaining its required Part1/Part2 arrangement: [disk layout](https://www.ventoy.net/en/doc_disk_layout.html). Ventoy's normal ISO discovery, global search root and `ventoy.json` reside on Part1: [search configuration](https://www.ventoy.net/en/doc_search_path.html), [plugin configuration](https://www.ventoy.net/en/faq.html). This means **P3 existence does not imply Vita on P3 boots by default**.

Evaluate **documented Ventoy-compatible boot-chain integration** (e.g., carefully scoped GRUB handoff via supported hooks versus an explicitly controlled parallel Vita entry). The Ventoy menu extension documented at [menu extension](https://www.ventoy.net/en/plugin_grubmenu.html) is ordinarily invoked with F6 and is **not itself proof of automatic/default Vita boot**. Ventoy's `vlnk` "local disk" feature is documented for a different disk, so **do not assume it supports P3 of the same Ventoy USB**: [Ventoy vlnk](https://www.ventoy.net/en/doc_vlnk.html). Alternative loader selection must respect ADR-0004's unmodified substrate constraint or be separately proposed as a new decision.

**Gate #27:** using *only verified disposable file-backed loop media*, prove correct P1/P2/P3 isolation, BIOS+UEFI default Vita boot from P3, selectable user ISO on P1, negative identity/tamper cases, preserving user data across safe upgrades, source/license evidence, and exact-head independent review. No migration of the current `prototypes/boot` fixture or Vita-on-P1 product release before the gate.

## Consequences and decision rationale

Benefits: distinct ownership and visibility; user-image operations cannot accidentally replace boot roots; bounded system-updater target; easier integrity/version/rollback governance; clearer user mental model.

Costs: a separate P3 consumes reserved capacity, complicates installation/migration and default boot integration, and needs explicit checks for MBR/GPT/firmware combinations and existing Ventoy media. Unsupported existing layouts may require an **opt-in new-device provision/migration**, never a silent in-place repartition. Disk separation protects against accidental cross-domain updates but **does not itself provide encryption, authentication or verified boot**.

## Reversal and validation

The **image-domain separation invariant is a hard product requirement**; if the P3 default-boot method proves unsupported, reconsider the loader/layout with a new ADR rather than silently restoring Vita OS into the user `/iso` directory. The existing I001 boot proof remains historical evidence for Ventoy compatibility, not proof of this new product partition contract. Physical writes, source licensing, Secure Boot and production release remain human-gated.
