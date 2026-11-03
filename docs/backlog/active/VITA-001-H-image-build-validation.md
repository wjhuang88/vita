# VITA-001-H — USB image build reaches reproducible, testable state

To make boot and storage changes safe,
the maintainer/Agent needs a reproducible image build and firmware-mode validation matrix,
so that releases are tested before touching physical devices.

Scope:
- Deterministic image/rootfs assembly inputs where practical.
- QEMU smoke boot.
- Boot log capture.
- Image checksum.
- Physical-media test checklist after VM pass.

Acceptance:
- [ ] A documented command produces the image artifact.
- [ ] QEMU boot validation passes for required firmware modes.
- [ ] Failure logs are retained or summarized in iteration evidence.
- [ ] SHA-256 is generated for release artifacts.
- [ ] Physical USB write is gated by the target-device procedure.

Dependencies: VITA-001-A, VITA-001-C.
