# Testing Policy

Testing is layered by risk.

## Required layers
- Unit tests for parsing, policy/risk classification and pure capability logic.
- Integration tests for Talos Core ↔ capability broker ↔ provider contracts.
- VM tests for boot/session behavior and storage/network simulations.
- Physical USB smoke tests only after VM gates pass.

## Minimum evidence by area
- Boot changes: QEMU boot log + expected interactive target.
- Config changes: valid/invalid/redaction cases.
- Capability changes: allow/deny/confirm/audit cases.
- Destructive-storage code: loopback/sparse disk tests; never the workstation system disk.
- Release: checksum + firmware-mode matrix.

A passing unit test is not completion if the feature is not reachable through the Live OS interaction path.
