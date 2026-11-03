# Iteration Workflow

Lifecycle: Planned → Active → Review → Complete / Partial / Blocked / Superseded.

During execution:
- Preserve the published baseline; changes go through `CHANGE-CONTROL.md`.
- Keep evidence with the iteration: commands, test results, VM/physical checks and important manual observations.
- Every iteration must aim for a runnable/testable deliverable, not disconnected internal code.
- If a capability is built but not reachable through the user/runtime path, report `partial`.
- Update user-facing README/reference docs in the same iteration when observable behavior changes.

Closure:
- Complete only if acceptance, evidence, status synchronization and residual registration are all satisfied.
- Update backlog/iteration owners, then the board.
