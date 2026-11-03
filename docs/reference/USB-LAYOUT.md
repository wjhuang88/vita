# USB Layout — Phase 1 Contract

ADR-0004 selects Ventoy as the Phase 1 boot substrate. The exact partition style and firmware support matrix remain subject to VITA-001-A QEMU validation, but the responsibility boundary is now defined.

## Physical responsibility boundary

Ventoy owns its required boot/data partition arrangement. Vita must not resize, reorder, or repurpose Ventoy-required partitions after installation unless an upstream-supported migration path is explicitly validated.

Phase 1 does not require an additional Vita partition. Portable product data lives on the ordinary Ventoy data partition so that ISO/config/workspace data can survive boot-substrate and Vita Live-image upgrades.

If later requirements need encrypted/private persistence or a dedicated writable system partition, introduce that as a separate decision rather than silently changing this contract.

## Logical data layout exposed to the operator

```text
/iso/
  vita/
    vita-live.iso      # project-owned default Live OS image
  ...                  # user-provided ISO/IMG/etc. boot images

/talos/config/         # portable model/session configuration
/talos/skills/         # optional Talos skills/extensions
/talos/workspace/      # writable operator workspace
/talos/logs/           # optional persisted redacted logs/audit, if enabled

/ventoy/ventoy.json    # Vita-owned Ventoy product configuration
```

Ventoy image discovery is constrained to `/iso`, which prevents runtime config/workspace files from being treated as boot assets. The Vita image path is initially `/iso/vita/vita-live.iso` and is configured as the default selection; user boot images remain dynamically discoverable under `/iso`.

## Medium discovery from the Live OS

The Live OS must not assume a transient device name such as `/dev/sdb1` or a particular device-mapper path. It must locate the Vita medium using stable evidence such as a filesystem label plus a project marker/config path, and must fail closed when multiple candidate media are ambiguous.

Read-only discovery may happen automatically. Any operation that writes a bootloader, repartitions media, formats a filesystem, or changes another disk must follow the target-identity and explicit-confirmation gates in `../sop/PRIVILEGED-OPERATIONS.md` and `../sop/BOOT-MEDIA-VALIDATION.md`.

## Upgrade invariant

Normal Vita upgrades should replace project-owned artifacts such as `vita-live.iso` and product configuration without deleting `/iso` user images or `/talos` portable data. Ventoy upgrades must use the upstream-supported data-preserving path and remain version/hash pinned by Vita release automation.
