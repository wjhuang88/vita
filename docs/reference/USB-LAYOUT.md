# USB Layout — System/User Image Isolation Contract

**Product constraint accepted on 2026-11-16:** Vita OS images and user-imported boot images have **separate ownership and storage domains**. [ADR-0009](../decisions/ADR-0009-system-user-image-isolation.md) **supersedes only the co-located image placement** in [ADR-0004](../decisions/ADR-0004-hybrid-ventoy-boot-substrate.md). This is a **design contract**, not an implemented/released bootable USB image. [Implementation/VM proof #27](https://github.com/wjhuang88/vita/issues/27) remains OPEN.

## Physical partition responsibility

```text
Vita USB disk — target topology, not yet implemented
P1  VENTOY_DATA    user image library and portable user data
    /iso/          only user-provided ISO/IMG/WIM/VHD files
    /talos/        user-managed config, skills, workspace and logs
    /ventoy/       required Ventoy plugin/config and small boot metadata
P2  VTOYEFI        upstream Ventoy-owned boot partition: never repurpose
P3  VITA_SYSTEM    Vita-managed system image/version/recovery domain
    /boot/         system boot assets
    /versions/     readonly, versioned immutable OS/rootfs and manifests
    /recovery/     product recovery images (authorization pending)
    /state/        controlled boot-health/version pointers (future)
```

P3 is **not** a generic ISO upload directory and must not be added to user-image discovery. The user data partition initially holds logically separate `/iso` and `/talos`; a P4/private-state design, encryption and writable system state need separate decisions. Filesystem layout, exact partitions/labels and default-boot implementation are not final.

Ventoy itself reserves P1/P2; its official [disk-layout documentation](https://www.ventoy.net/en/doc_disk_layout.html) supports allocating a **reserved tail space at install time** that can later host P3 without resizing P1/P2. **Do not create P3 by repartitioning an existing user device**. Physical partition creation will only be tested first against verified disposable file-backed loop images.

## Image discovery and boot-menu contract

- User image importer and menu image discovery operate on **P1 `/iso/` only**, never on P3 system paths. Files under `/talos/` and `/ventoy/` do not become user boot targets.
- Vita OS is a **separate named/default system entry** whose loader locates a verified P3 by stable identity, signed/hashed manifest and approved boot policy. It must **not** choose a user-supplied `/iso/vita-live.iso` or fallback to a matching filename from P1 when the real system volume is missing.
- Ventoy config `/ventoy/ventoy.json` remains on P1 because upstream requires that path. This is **boot-control metadata only**, not a trusted Vita OS image. A user-writable boot-control file is a product threat surface; ownership/authenticity and product boot policy are still security design gates.
- Ventoy normally scans ISO files on P1, **not P3**. Upstream's GRUB menu extension has manual F6 behavior; an auto/default separate P3 Vita boot entry must be *proven*, not assumed. The old `VTOY_DEFAULT_IMAGE=/iso/vita/vita-live.iso` is a historical I001 fixture, **not the product contract**.
- The first existing Ventoy boot proof [I001](../../prototypes/boot/README.md) intentionally writes a Vita test ISO and a user test ISO to P1. **Do not rewrite that archived proof without a separately validated P3 boot replacement**. It is not evidence of partition isolation.

## Separated updates and data retention

System-install/update/rollback may target **only** a strongly identified Vita-owned P3 with explicit authorization, signature/digest validation, nonrollback guarantees only where implemented and independently governed boot-health pointers. A user-image add/delete/rename must affect **only P1 `/iso`**. An upgrade may change a narrowly defined controlled boot-menu pointer but must not delete or overwrite user ISOs or `/talos` data.

Trust domains are distinct even if both partitions are on the same physical stick; **partition isolation alone is not cryptographic trusted boot**. Device identity ambiguity, injected media, missing P3, removable-drive loss, tampered manifest, P1 config attacks, conflicting version metadata and interrupted updates must fail safely with an explicit recovery path. Never mount or modify arbitrary host disks to search for a compatible image.

## Validation and human gates

[Issue #27](https://github.com/wjhuang88/vita/issues/27) must demonstrate on a disposable raw disk: P1/P2 preserved; P3 created from the reserved tail; BIOS+UEFI default boot Vita from **actual P3**; one user ISO on **P1** still selectable/bootable; user image cannot spoof Vita's OS; image import and system update cannot cross storage domains; a failed system update preserves user content. Require exact-head CI, independent review and non-destructive negative cases.

Real U-disk writes/repartition/migration, boot signing/Secure Boot, physical firmware variants, source-license/SBOM and release approval **remain separately human-gated**. Existing QEMU ISO-backed rootfs proof uses fixed optical `/dev/sr0`, not this physical USB layout.
