# I003 — Rust-native, read-only system facts through Talos

Status: Active
Published baseline: 2026-11-04

## Objective

Turn the I001 `system_summary` demonstration into a reusable, typed Rust-native system provider with standalone tests. Continue the Ready VITA-001-D interaction-runtime story in a bounded, independently runnable slice.

## Iteration coordination

The parallel I002 real-ISO investigation on separate Draft PR #4 is Partial with issue #5 as a compatibility blocker. I003 starts independently from clean `main` and does **not** claim to solve that ISO problem or supersede I002 evidence.

## Selected work

- Create `crates/vita-system-facts`, a Rust 2024 crate with only `std`; prohibit unsafe code.
- Inspect `/etc/os-release` and bounded Linux `/proc` fields, returning typed optional values on unavailable/malformed/oversized facts.
- Parse uptime seconds and total RAM in KiB with explicit units and bounds.
- Inject fake fixture paths for unit tests to avoid requiring privileged or hardware-dependent integration.
- Integrate through Vita's existing `talos-runtime` `AgentTool` with `ToolNature::Read` and no shell/CLI wrapper.
- Extend I001 workflow with unit tests and crate dependency triggers; preserve the QEMU boot baseline.

## Runnable acceptance

- `cargo +1.99.0 test --manifest-path crates/vita-system-facts/Cargo.toml` passes.
- `cargo +1.99.0 run --manifest-path prototypes/interaction-runtime/Cargo.toml` executes `system_summary` under Talos and emits `VITA_I001_TALOS_RUNTIME_OK`.
- `sh scripts/validate_project_governance.sh .` passes.
- Existing I001 QEMU boot-chain job remains green when this PR runs.
- No shell, privileged operation, physical disk changes, new language runtime, or external model provider.

## Exclusions and residual ownership

This slice does **not** make a Vita Live OS, console/PTY, crash recovery or privileged broker usable. These remain VITA-001-C/D/F/G acceptance gates. No numeric performance/footprint claim is made without measurement.

## Evidence

Awaiting GitHub Actions execution. Do not mark I003 Complete until actual tests pass and owner status is synchronized.
