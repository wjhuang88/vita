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

## Cross-cutting implementation gates
- Apply ADR-0005: Vita-owned runtime and system capability logic are Rust-first with typed Linux-native APIs where feasible.
- Stage 1: inventory all shipped executables/services/language runtimes and select a minimal Linux base; distinguish build-only tooling.
- Stage 2: embed Talos Runtime directly into a Rust session process, rather than wrapping its CLI.
- Stage 3: validate native read-only providers first, then admit reviewed external IPC/subprocess exceptions under broker control.
- Derive footprint/performance budgets from actual measured Live image builds, not unsupported assumptions.

Sequencing principle: prove boot and Talos Core interaction boundaries before broad tool integration.
