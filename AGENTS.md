# AGENTS.md

## Project Mission
Build a portable USB system that combines multi-ISO boot with an Agent-native Live OS. In the Live OS, Talos Core is the primary interactive system interface, not merely a CLI application.

## Hard Constraints
1. **Talos interaction position:** the default interactive user session must be Talos Core-driven. Do not design the product as “Linux + terminal + bundled talos CLI”. A conventional shell is compatibility/recovery only.
2. **Destructive operations are never implicit:** disk writes, partitioning, filesystem formatting, bootloader installation, credential changes, firewall/network reconfiguration, package removal and equivalent high-impact actions require capability classification, a clear plan, target identification, explicit user confirmation, and audit evidence.
3. **No raw natural-language-to-root-shell bridge:** privileged execution must pass through the capability broker/policy layer. Shell execution is an adapter of last resort, not the architecture.
4. **Target-device identity must be proven before writes:** any USB/image installer must distinguish the source USB, target disk and mounted filesystems and must refuse ambiguous targets.
5. **Secrets are not committed:** model API keys and credentials must not be stored in tracked config. Config may reference runtime environment, prompt-on-use, OS key store, or encrypted removable-media secrets.
6. **Boot changes require VM validation first:** bootloader, partition layout, initramfs, kernel or Live image changes must pass QEMU smoke tests before physical-media validation.
7. **Routing before process work:** requirement, iteration, decision, release and governance changes must follow the owning SOP before implementation begins.
8. **Owner documents are authoritative:** update backlog/iteration/decision/reference owners before the derived `docs/BOARD.md`.
9. **Lessons and decisions write back:** recurring failures go to `EVOLUTION.md`; architectural tradeoffs affecting Soft/Assumption constraints go to `docs/decisions/`.
10. **Governance refresh:** when local governance predates the installed `agent-project-governance` skill, audit the affected capability before trusting stale SOPs.
11. **Rust-first Vita-owned system capabilities:** use Rust and native Linux interfaces for Vita interaction, providers, broker, config and audit wherever safe and feasible. No new app-language runtime or shell-command chains on the normal interaction path without a reviewed, documented exception. See ADR-0005.

## Coding Behavior
- State material assumptions before implementing.
- Implement only requested behavior; do not add speculative features, premature abstractions, or unrelated refactors.
- Match established repository style and module boundaries.
- Define verifiable success criteria before coding.
- Remove only artifacts made obsolete by the current change.
- Prefer explicit typed capability contracts over shell-string construction for OS operations.
- Evaluate existing Talos/Rust crates and native kernel interfaces before adding subprocess dependencies; inventory shipped non-Rust tools and justify exceptions.
- Treat boot code, storage operations and privilege transitions as high-risk surfaces requiring focused tests and review. For single-maintainer independent AI review, follow the separate-execution and exact-head evidence policy in `docs/sop/PAIRING-WORKFLOW.md`; AI review never substitutes for human product/security/release approval.

## Git Rules
- Work on a feature branch for non-trivial changes; inspect branch/worktree state before editing.
- Review the staged diff before commit; do not use broad staging when unrelated files exist.
- For every Agent-authored or Agent-assisted commit, the model tag is mandatory: `type(scope): description (#story-id) [model:<model-name>]`
- `scope` identifies the affected subsystem, e.g. `boot`, `live`, `interaction`, `capability`, `config`, `governance`, `build`.
- Example: `docs(governance): initialize high-risk project controls (#VITA-001) [model:gpt-5.6-sol]`.
- Never rewrite shared history without explicit user direction.

## Task Router
| Task | Read first |
| --- | --- |
| Project orientation / architecture | `docs/README.md`, `docs/reference/SYSTEM-ARCHITECTURE.md` |
| New requirement / scope change | `docs/sop/REQUIREMENT-INTAKE.md` |
| Start iteration | `docs/sop/START-ITERATION.md` |
| Execute iteration | `docs/sop/ITERATION-WORKFLOW.md` |
| In-flight scope change | `docs/sop/CHANGE-CONTROL.md` |
| New feature | `docs/sop/NEW-FEATURE.md` |
| Long-running autonomous work | `docs/sop/LONG-RUNNING-TASK.md` |
| Tests / validation | `docs/sop/TESTING.md` |
| Boot/image/storage work | `docs/sop/BOOT-MEDIA-VALIDATION.md` |
| Privileged OS operation design | `docs/sop/PRIVILEGED-OPERATIONS.md` |
| Security-sensitive work | `docs/sop/SECURITY.md` |
| Git / branch / commit | `docs/sop/GIT-WORKFLOW.md` |
| Release / image publishing | `docs/sop/RELEASE.md` |
| Pairing / staged high-risk change | `docs/sop/PAIRING-WORKFLOW.md` |
| Documentation maintenance | `docs/sop/DOC-CHECK.md` |
| Architecture decision | `docs/decisions/README.md` |
| Repeated failure / correction | `docs/sop/EVOLUTION-FEEDBACK.md` |

## Definition of Done
A slice is complete only when requested behavior is implemented, relevant automated/manual validation evidence is recorded, status owners are synchronized, user-facing docs are updated for observable behavior, and residual gaps are either explicitly out of scope or registered in the backlog.

## Session End Checklist
- [ ] Did the requested story/iteration acceptance criteria pass?
- [ ] Is validation evidence recorded in the owning story/iteration?
- [ ] Are important claims classified as confirmed fact, inference, assumption, or unknown where needed?
- [ ] Did backlog and iteration state change? Update owners first.
- [ ] If owner state changed, is `docs/BOARD.md` synchronized afterward?
- [ ] Did observable behavior change? Update README/reference/user guidance.
- [ ] Did we make a technical choice affecting a Soft/Assumption constraint? Record/update an ADR.
- [ ] Did we discover a recurring trap or corrected omission? Update `EVOLUTION.md`.
- [ ] Is residual work registered rather than hidden in prose?
- [ ] Is the staged diff focused and commit-ready?

## Current Known Traps
- Treating Talos as a CLI wrapper would violate the product architecture.
- A generic root shell exposed to model output would violate the privileged-operation boundary.
- USB source/target ambiguity can destroy the wrong disk.
- An image that boots in one firmware mode is not evidence that the portable boot contract is satisfied.
