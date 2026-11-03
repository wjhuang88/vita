# ADR-0001 — Talos Core is the primary OS interaction runtime

## Context
The product must feel Agent-native at the operating-system interaction layer. Launching a Talos CLI from a conventional shell would keep the shell as the real interaction model and reduce Talos to a bundled application.

## Constraint Decomposition
| Constraint | Type | Source | Can Change? |
| --- | --- | --- | --- |
| Talos occupies shell/session interaction position | Hard | Product intent | No for current product |
| Linux remains the underlying OS/kernel/userspace base | Soft | Initial engineering direction | Yes |
| Existing Talos CLI is sufficient for this role | Assumption | Initial wording | Rejected / must be proven otherwise |

## Reasoning
Talos Core/runtime components should be embedded into a dedicated session process that owns the interactive loop and invokes structured system capabilities. This preserves the Agent-native experience without requiring Talos to replace the kernel or init system.

## Decision
Build a project-specific OS interaction runtime on Talos Core. The default interactive session launches this runtime directly. Conventional shell access exists only as compatibility/recovery. Do not define the final product as a wrapper around the existing Talos CLI.

## Reversal Trigger
Revisit only if Talos itself evolves a reusable shell/session runtime whose architecture satisfies the same default-session, capability-governance and recovery requirements without CLI wrapping.
