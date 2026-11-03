# I001 — Foundation: boot decision + Talos interaction boundary

Status: Planned
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
- Talos Core integration test or compile-time proof recorded.
- Governance validator passes.

## Exit criteria
- Boot strategy ADR exists and unblocks B/C/H.
- Talos integration is proven at Core/runtime level, not only CLI wrapping.
- Residual unknowns become explicit backlog items.

## Evidence
Not started.
