# Implementation Roadmap

## Stage 0 — Governance and architecture convergence
- Establish high-risk governance baseline.
- Prove boot substrate choice.
- Inspect Talos Core integration surface.

## Stage 1 — Bootable foundation
- Build USB image layout.
- Boot project Live OS in QEMU.
- Discover/boot representative ISO.

## Stage 2 — Agent-native interaction
- Build Talos Core-based session runtime.
- Add structured system-state providers.
- Add recovery shell.

## Stage 3 — Governed system operations
- Capability broker and risk classes.
- Confirmation/audit pipeline.
- Storage/network/process/filesystem providers.

## Stage 4 — Portable configuration and persistence
- `/talos/config` contract.
- Credential-source abstraction and redaction.
- Workspace/persistence rules.

## Stage 5 — Release hardening
- Firmware/boot matrix.
- Physical-device matrix.
- Upgrade strategy preserving ISO/config data.
- Signed/checksummed artifacts and release notes.

Sequencing principle: prove boot and Talos Core interaction boundaries before broad tool integration.
