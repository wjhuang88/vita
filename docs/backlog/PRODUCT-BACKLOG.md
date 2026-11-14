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
| VITA-001 | P0 | Epic | Agent-native portable USB OS | Product foundation; I001 architecture gates proven | Architecture + ADR-0001 + ADR-0004 | Active |
| VITA-001-B | P0 | Story | Discover and boot ISO images | I002 partial; real Alpine ISO Ventoy handoff blocked (#5) | Issue #5 + PR #4 | Blocked |
| VITA-001-C | P0 | Story | Boot a directly usable Live OS | I008-I009b2a VM proof merged; readonly ISO-backed SquashFS / proven (19/19 exact-head checks), initramfs 0.48 MiB; services, measurable init comparison, security and release OPEN | ADR-0008 + I009a | Active |
| VITA-001-D | P0 | Story | Talos Core becomes the primary interactive session | I006 Linux guest Talos SDK offline session verified; production provider, TTY and recovery remain open | I004 + I006 | Active |
| VITA-001-E | P0 | Story | Load model and Agent configuration from the USB | Security boundary | Config contract + threat model | Refinement |
| VITA-001-F | P0 | Story | Execute system operations through a governed capability broker | ADR-0002 | Privileged operations SOP | Refinement |
| VITA-001-G | P1 | Story | Provide explicit compatibility/recovery shell | I007 QEMU offline root recovery proven; firmware/TTY/security/product recovery open | ADR-0007 + I007 | Refinement |
| VITA-001-H | P0 | Technical Story | Produce reproducible USB image and boot validation matrix | A proven; release artifact still depends on C | Boot validation SOP + ADR-0004 | Refinement |

## Completed / Archived
| ID | Type | Result | Archive |
| --- | --- | --- | --- |
| VITA-001-A | Spike | Selected and VM-proved the Phase 1 Ventoy substrate + Vita-owned Live OS boundary | `archive/VITA-001-A-boot-strategy-spike.md` |

## Later
- Hardware inventory and firmware diagnostics packs.
- Offline/local-model mode.
- Remote support/session handoff.
- Signed extension/skill bundles.
- Optional graphical Agent workspace.

Completed or superseded item detail lives in `archive/`; this file remains the prioritization surface.


**I009b1 VM engineering checkpoint (2026-11-14):** [PR #19](https://github.com/wjhuang88/vita/pull/19) merged, 17/17 final-head checks; actual readonly SquashFS `/` and stage1/PID1 handoff proven in QEMU. VITA-001-C remains Active for media mapping, services/drivers, final init comparison and human release gates.

**I009b2a VM media-source checkpoint (2026-11-14):** [PR #22](https://github.com/wjhuang88/vita/pull/22) merged after 19/19 exact-head checks and independent AI technical review; [BIOS/UEFI evidence](https://github.com/wjhuang88/vita/actions/runs/43558241910) proves readonly ISO9660 source for the complete readonly SquashFS `/`, no root image in initramfs, negative media-image tamper rejection and PID1/Talos fault lifecycle. [I009b2 #21](https://github.com/wjhuang88/vita/issues/21) stays open for boot/RAM benchmark and init comparison. No physical/release approvals.
