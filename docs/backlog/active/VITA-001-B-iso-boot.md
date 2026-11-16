# VITA-001-B — User can boot an ISO stored on the USB

Status: Blocked — representative Linux ISO fails via Ventoy (#5)


As an operator,
I want the boot environment to discover ISO images in the USB ISO directory,
so that I can carry multiple installers/rescue systems without reflashing the device.

Scope:
- Define ISO directory contract.
- Discover supported image files.
- Present selectable entries.
- Boot at least one representative Linux ISO in the selected firmware mode.

Out of scope:
- Universal compatibility with every ISO format/vendor.
- Network boot.

Acceptance:
- Given a supported ISO exists in the configured ISO directory
  When the machine boots from the USB
  Then the ISO appears as a selectable boot target
  And selecting it transfers control successfully to the ISO boot path.
- Given a corrupt/unsupported ISO
  When discovery occurs
  Then the system does not silently attempt a destructive fallback
  And the failure is diagnosable.

Dependencies: VITA-001-A.
Minimum validation: QEMU boot evidence plus checksum/test fixture record.
State owners: backlog, iteration, boot reference, release evidence.

## Image discovery/ownership change (2026-11-16)

**Product requirement:** discover only user-provided boot images on Ventoy P1 `/iso/`; Vita-owned OS images are **not** part of the user image library and reside on a separate protected system domain (P3 target) under [ADR-0009](../../decisions/ADR-0009-system-user-image-isolation.md). The default Vita boot entry must not be selected by masquerading as a user ISO. [#27](https://github.com/wjhuang88/vita/issues/27) will prove P3 firmware default boot and user ISO on P1 under identical disposable VM media. Ventoy's representative third-party distro compatibility blocker [#5](https://github.com/wjhuang88/vita/issues/5) remains separate and unresolved. Historical I001 co-location is not new product acceptance.

## I002 observed compatibility blocker

Unmodified Alpine Linux virt 3.24.3, SHA-256 `cdc1bc0efc64d8d611bf3733f54868172a545888dfd726f518bf279bcb0ff1af`, directly boots to a Linux guest shell in both BIOS and OVMF UEFI. On the same tested environment, Ventoy BIOS stops after ISOLINUX, and Ventoy UEFI normal and GRUB2 fallback stop after Alpine's GRUB handoff without a Linux guest login. See [Issue #5](https://github.com/wjhuang88/vita/issues/5) and the unmerged [Draft PR #4](https://github.com/wjhuang88/vita/pull/4) for actual QEMU/serial evidence.

VITA-001-B is **Blocked** pending a verifiable Ventoy-backed guest command, visible menu evidence and corrupt/unsupported ISO failure behavior. I001's deterministic boot-stub success is not proof of this Story's real-distro acceptance.
