# VITA-001-B — User can boot an ISO stored on the USB

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
