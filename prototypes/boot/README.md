# I001 Boot Prototype Staging

This directory contains the non-destructive configuration side of the selected Phase 1 boot architecture.

## Selected shape

- Ventoy is an unmodified, pinned third-party boot substrate.
- Vita owns `ventoy.json`, the Vita Live ISO, and everything after that ISO starts.
- User boot images live under `/iso/` and remain dynamically discoverable.
- The default Vita image path is `/iso/vita/vita-live.iso`.
- Portable Talos/Vita data remains outside the Ventoy search root under `/talos/`.

`ventoy/ventoy.json` is a staging template for the future image builder. It does not write to a disk.

## QEMU proof still required

VITA-001-A remains open until an automated or controlled-host proof:

1. installs the pinned Ventoy release into a disposable raw disk image;
2. copies this configuration plus a minimal Vita bootable ISO into the data partition;
3. boots the raw image in QEMU through the initial BIOS/UEFI matrix;
4. verifies the default image reaches a deterministic Vita marker;
5. verifies a second user ISO remains selectable;
6. records the exact version, hash, commands, firmware mode, and result.

The environment used for the first research pass does not provide QEMU, GRUB tooling, xorriso, or squashfs tools, so no VM success is claimed yet.

## Safety

Do not turn this prototype into a script that accepts an arbitrary block-device path. Physical media installation remains prohibited until the disposable-image VM path passes and target-device identity/refusal rules are implemented.
