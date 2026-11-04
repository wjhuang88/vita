# VITA-001-B — User can boot an ISO stored on the USB

Status: Blocked — Ventoy/Alpine guest boot compatibility, GitHub Issue #5


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

## I002 implementation slice
- Validate unmodified Alpine Linux virt 3.24.3 with upstream SHA-256 from its fixed version-specific release path.
- Require both direct and Ventoy-backed QEMU serial login plus an executed Linux shell command; retain the Vita default probe ISO on the same medium.
- Store real guest logs as a workflow artifact. Do not interpret automatic ISO handoff alone as proof of visible menu listing.
- **Residual acceptance:** menu presentation/selection evidence and corrupt/unsupported ISO behavior still require explicit tests before B can close.
State owners: backlog, iteration, boot reference, release evidence.

## I002 outcome
- Positive controls passed: official Alpine ISO boots directly in BIOS and UEFI and runs a Linux guest shell command.
- Ventoy BIOS, Ventoy UEFI normal mode and officially documented GRUB2 fallback all fail to reach Linux userspace in CI; see [Issue #5](https://github.com/wjhuang88/vita/issues/5).
- Full acceptance remains open (actual real-ISO boot; menu visibility; corrupt-image diagnostics). I002 = Partial, PR #4 remains Draft and unmerged. This is not evidence of real-distro compatibility.
