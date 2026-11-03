# Evolution Log

This file records reusable lessons discovered through failures, corrections and unexpectedly effective remedies. Mandatory current rules belong in `AGENTS.md` or the owning SOP.

## Index
| ID | Area | Trigger | Promoted to |
| --- | --- | --- | --- |
| EV-0001 | Architecture | Talos positioning clarified during initialization | `AGENTS.md`, ADR-0001 |

## EV-0001 — Talos must not be reduced to a bundled CLI
- **Trigger:** Initial project framing described “a system with an integrated Talos”.
- **Symptom:** The architecture could drift toward a conventional Linux distribution that merely launches `talos cli`.
- **Cause:** “Integrated tool” language did not express the intended interaction boundary.
- **Remedy:** Define Talos Core as the primary OS interaction runtime and keep conventional shell as compatibility/recovery.
- **Prevention:** Enforce the constraint in `AGENTS.md`, architecture reference and ADR-0001; reject stories that only wrap the existing CLI.
- **Promotion target:** `AGENTS.md`, `docs/decisions/ADR-0001-talos-primary-interaction-runtime.md`.
