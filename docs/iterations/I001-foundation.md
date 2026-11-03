# I001 — Foundation: boot decision + Talos interaction boundary

Status: Active
Published baseline: 2026-11-03

## Objective
Reduce the two largest architecture uncertainties before broad implementation: portable boot substrate and Talos Core integration boundary.

## Selected work
- VITA-001-A boot strategy Spike.
- Refine VITA-001-D with evidence from the current Talos repository/core API shape.
- Confirm or amend ADR-0001/0002 assumptions.

## Runnable/testable deliverable
A minimal VM-bootable proof that reaches a project-owned Live environment or boot stub, plus a host-runnable Talos interaction prototype that demonstrates a structured system-inspection capability. If the two cannot yet be combined, the iteration may finish `partial` only with blockers and next story registered.

## Validation plan
- QEMU/firmware smoke command recorded.
- Talos Core/runtime integration test or compile-time proof recorded.
- Governance validator passes.

## Exit criteria
- Boot strategy ADR exists and unblocks B/C/H.
- Talos integration is proven at Core/runtime level, not only CLI wrapping.
- Residual unknowns become explicit backlog items.

## Execution evidence — 2026-11-03

### Boot strategy
- Upstream Ventoy and GNU GRUB documentation reviewed.
- `docs/reference/BOOT-STRATEGY-RESEARCH.md` records the evidence and option comparison.
- ADR-0004 selects the Phase 1 hybrid architecture: pinned/unmodified Ventoy substrate + Vita-owned Live OS/runtime.
- Non-destructive staging config added at `prototypes/boot/ventoy/ventoy.json`.
- VM proof remains open. The current execution environment lacks `qemu-system-x86_64`, GRUB tooling, `xorriso`, and squashfs tooling, so no boot success is claimed.

### Talos integration boundary
- Talos inspected at `261602242e49e6f921f5d3e4eac96eebbe92f8b3`.
- Current Talos reference contract confirms `talos-runtime` as the supported standalone embedding facade and explicitly keeps CLI/TUI outside that contract.
- `docs/reference/TALOS-INTEGRATION.md` now records the concrete Vita composition boundary.
- `prototypes/interaction-runtime/` provides a deterministic, network-free Rust proof using `RuntimeBuilder`, a Vita-owned read-only `AgentTool`, and an explicit Read permission rule.
- `.github/workflows/i001-foundation.yml` runs the governance validator and the Talos runtime prototype on the PR.

## Current disposition

- VITA-001-A: Active; architecture selected, QEMU proof still required.
- VITA-001-D: Ready; supported embedding boundary is confirmed, host prototype awaiting CI evidence.
- VITA-001-B/C remain blocked until A's VM proof closes the boot-substrate gate.

## Residual work before I001 can close

1. Obtain successful CI execution of `prototypes/interaction-runtime`.
2. Implement and run the disposable-image QEMU proof for the Ventoy-backed default Vita boot path.
3. Record the exact boot commands/firmware matrix and update VITA-001-A evidence.
4. Run the governance validator on the final iteration state.
5. If either proof invalidates an architectural assumption, amend the owning ADR before further implementation.
