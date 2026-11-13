# Pairing Workflow for High-risk Changes

Use for bootloader, partition/image writer, privilege boundary, secret handling and release-image changes.

1. Author records intended change, risk surface and validation plan.
2. Implement smallest safe slice.
3. Reviewer independently checks target identity, failure behavior, rollback/recovery and test evidence.
4. No physical-media/release step before VM/static/unit gates pass.
5. Resolve review findings and rerun affected checks.

## Single-maintainer independent AI code review (owner-authorized)

A sole human maintainer may delegate **independent technical code review** to a separately executing AI reviewer (e.g. GitHub Copilot Code Review) when the owner explicitly authorizes it. The code-authoring assistant may not self-review and call that independent. Never invent another human reviewer; successful execution of a reviewer CI check does not mean that the reviewer approved the change.

1. State review scope, threats and expected evidence for the PR's exact HEAD SHA. Trigger a **separate AI reviewer execution** and retain its report and inline review comments in the PR.
2. Treat substantive findings and missing proof as merge blockers. Fix the code and tests, and respond to each thread with concrete commit/CI evidence. Never resolve merely by assertion.
3. Re-run all required VM/CI checks after every changed code HEAD, and request reviewer re-execution for that exact HEAD. Consider its latest written findings, not only a green reviewer workflow conclusion.
4. After validated fixes, resolve actionable review threads and record a concise PR disposition. Before merging, verify final head/base, clean diff, all required exact-HEAD checks, no unresolved actionable threads, and merge with an expected-HEAD lease.
5. Preserve **separate mandatory human sign-offs** for real hardware, firmware, production security, destructive writes, authenticated recovery, source licenses/SBOM and releases. The AI reviewer cannot authorize those, and a VM-only engineering merge must not be represented as a product release.
6. If a genuinely separate reviewer cannot run or the report is inaccessible, the independent-review gate is BLOCKED; do not silently substitute an author self-check.

Evidence/example: [I009a PR #14](https://github.com/wjhuang88/vita/pull/14): code head `b6ead5d050e79bea7ce11221a1093882a872951e`, 12/12 exact-head checks passed, multiple distinct Copilot review executions with resolved inline findings, squash merge `40703ff942cd1eeb8bf051b62d9cbfcc97c9d60a`. [Human acceptance](../validation/I009a-HUMAN-ACCEPTANCE.md) remains entirely pending.
