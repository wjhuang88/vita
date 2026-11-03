# Product Backlog

## Required Reads
- `../reference/SYSTEM-ARCHITECTURE.md`
- `../reference/THREAT-MODEL.md`
- `../reference/BOOT-STRATEGY-RESEARCH.md`
- `../decisions/ADR-0001-talos-primary-interaction-runtime.md`
- `../decisions/ADR-0004-hybrid-ventoy-boot-substrate.md`

## Active
| ID | Priority | Type | Title | Decision Context | Required Reads | State |
| --- | --- | --- | --- | --- | --- | --- |
| VITA-001 | P0 | Epic | Agent-native portable USB OS | Product foundation | Architecture + ADR-0001 | Refinement |
| VITA-001-A | P0 | Spike | Select portable boot strategy | Hybrid Ventoy/Vita selected; VM proof still gates completion | Boot research + ADR-0004 | Active |
| VITA-001-B | P0 | Story | Discover and boot ISO images | Depends on A VM proof | A output | Blocked |
| VITA-001-C | P0 | Story | Boot a directly usable Live OS | Depends on A VM proof | A output | Blocked |
| VITA-001-D | P0 | Story | Talos Core becomes the primary interactive session | `talos-runtime` embedding boundary confirmed | Talos integration + ADR-0001 | Ready |
| VITA-001-E | P0 | Story | Load model and Agent configuration from the USB | Security boundary | Config contract + threat model | Refinement |
| VITA-001-F | P0 | Story | Execute system operations through a governed capability broker | ADR-0002 | Privileged operations SOP | Refinement |
| VITA-001-G | P1 | Story | Provide explicit compatibility/recovery shell | ADR-0001 | Talos integration reference | Refinement |
| VITA-001-H | P0 | Technical Story | Produce reproducible USB image and boot validation matrix | Release gate + third-party boot substrate | Boot validation SOP + ADR-0004 | Refinement |

## Later
- Hardware inventory and firmware diagnostics packs.
- Offline/local-model mode.
- Remote support/session handoff.
- Signed extension/skill bundles.
- Optional graphical Agent workspace.

## Archive
Completed or superseded items move to `archive/` and remain indexed here.
