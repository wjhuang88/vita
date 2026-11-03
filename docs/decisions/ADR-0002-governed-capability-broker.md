# ADR-0002 — Privileged system actions pass through a governed capability broker

## Context
An Agent-native shell needs system-level power, but direct natural-language-to-root-shell execution creates unacceptable ambiguity for disk, boot, credential and network operations.

## Constraint Decomposition
| Constraint | Type | Source | Can Change? |
| --- | --- | --- | --- |
| Destructive operations require explicit confirmation | Hard | Safety requirement | No |
| Actions must be auditable and target-resolved | Hard | Safety/diagnostic requirement | No |
| Prefer typed capability providers over shell commands | Soft | Architecture choice | Yes |
| A shell adapter may be needed for coverage gaps | Assumption | Compatibility expectation | Yes |

## Reasoning
A capability broker can expose typed operations, classify risk, require target identity, obtain confirmation and produce audit evidence. It also allows providers to evolve independently from the language model/planner.

## Decision
Talos invokes registered capabilities through a broker/policy layer. Shell execution may exist as a constrained adapter but is not the default privileged execution substrate.

## Reversal Trigger
Revisit if the underlying OS provides an equivalent typed, policy-enforced, auditable action API that fully replaces the broker responsibilities.
