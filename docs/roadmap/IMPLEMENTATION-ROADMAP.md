# Implementation Roadmap

## Stage 0 — Governance and architecture convergence
- Establish high-risk governance baseline.
- Prove boot substrate choice.
- Inspect Talos Core integration surface.

## Cross-cutting disk image separation change (2026-11-16)

[ADR-0009](../decisions/ADR-0009-system-user-image-isolation.md) requires Vita-owned OS images/rootfs/rollback assets to reside in a **protected system domain (P3 target)** distinct from user ISO/IMG content on Ventoy P1 `/iso`. [Issue #27](https://github.com/wjhuang88/vita/issues/27) must prove separately bootable P3 Vita in BIOS/UEFI, user ISO launch on P1, no cross-domain updates, reserved-space partitioning under disposable loop targets, and safe migration before any physical provisioning/release claims. The legacy co-located I001 boot probe remains historical only.

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
