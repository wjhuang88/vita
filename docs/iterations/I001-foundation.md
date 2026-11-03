# I001 — Foundation: boot decision + Talos interaction boundary

Status: Review
Published baseline: 2026-11-03

## Objective
Reduce the two largest architecture uncertainties before broad implementation: portable boot substrate and Talos Core integration boundary.

## Selected work
- VITA-001-A boot strategy Spike.
- Refine VITA-001-D with evidence from the current Talos repository/core API shape.
- Confirm or amend ADR-0001/0002 assumptions.

## Runnable/testable deliverable
A minimal VM-bootable proof that reaches a project-owned Live environment or boot stub, plus a host-runnable Talos interaction prototype that demonstrates a structured system-inspection capability.

## Validation plan
- QEMU/firmware smoke command recorded.
- Talos Core/runtime integration compile/run proof recorded.
- Governance validator passes.

## Exit criteria
- [x] Boot strategy ADR exists and releases the VITA-001-B/C/H architecture dependency.
- [x] Talos integration is proven at Core/runtime level, not only CLI wrapping.
- [x] Residual unknowns have explicit owner stories or later-stage destinations.

## Execution evidence — 2026-11-03

### Boot strategy
- Upstream Ventoy and GNU GRUB documentation reviewed.
- `docs/reference/BOOT-STRATEGY-RESEARCH.md` records evidence and option comparison.
- ADR-0004 selects the Phase 1 hybrid architecture: pinned/unmodified Ventoy substrate + Vita-owned Live OS/runtime.
- `prototypes/boot/ventoy/ventoy.json` owns the Vita product configuration boundary.
- `scripts/ci/validate_i001_boot.sh` constructs a disposable 768 MiB raw disk, verifies its `/dev/loopN` backing identity before any destructive operation, pins/verifies Ventoy 1.1.18, stages deterministic ISO fixtures, and boots the result with QEMU.
- GitHub Actions run `41659336430`, job `boot-chain-prototype`, passed at `68c25efd89aa12ca0a611fbce4ea6605907cbebd` with:

```text
VITA_BOOT_PROBE_OK mode=bios marker=VITA_BOOT_OK
VITA_BOOT_PROBE_OK mode=uefi marker=VITA_BOOT_OK
VITA_BOOT_PROBE_OK mode=bios marker=VITA_USER_ISO_BOOT_OK
VITA_I001_BOOT_CHAIN_OK ventoy=1.1.18
```

### Talos integration boundary
- Talos inspected and pinned for the prototype at `wjhuang88/talos@261602242e49e6f921f5d3e4eac96eebbe92f8b3`.
- Current Talos reference contract confirms `talos-runtime` as the supported standalone embedding facade and explicitly keeps CLI/TUI outside that contract.
- `docs/reference/TALOS-INTEGRATION.md` records the Vita composition boundary.
- `prototypes/interaction-runtime/` runs a deterministic network-free turn through `RuntimeBuilder`, a Vita-owned read-only `system_summary` `AgentTool`, and an explicit Read permission rule.
- GitHub Actions run `41659336430`, job `talos-runtime-prototype`, compiled Talos Runtime v0.10.1 from the pinned revision under Rust 1.99.0 and finished with:

```text
VITA_I001_TALOS_RUNTIME_OK
```

### Governance
- The same run's `governance` job passed `sh scripts/validate_project_governance.sh .`.
- Owner docs are being synchronized before the derived board as required by the iteration workflow.

## Disposition after I001

- VITA-001-A: Complete and archived; boot substrate decision/proof gate is closed.
- VITA-001-B: dependency released and Ready for a representative real-ISO compatibility slice.
- VITA-001-C: dependency released but remains in Refinement until the concrete Live OS base/build contract is selected.
- VITA-001-D: Ready; supported Talos embedding boundary is proven, while real console/Live integration remains implementation work.
- VITA-001-H: remains Refinement and depends on the final Live OS from C for release-grade image production.

## Residual work

I001 deliberately does not claim:
- physical USB/firmware validation;
- Secure Boot support;
- real third-party distro ISO compatibility beyond deterministic probe images;
- a final Vita Live OS rootfs/initramfs;
- Talos console/PTY rendering, privileged capability broker integration, or crash recovery in the Live system.

These are owned by VITA-001-B/C/D/F/H and later release hardening. No residual item invalidates the two architecture decisions proven by I001.

## Review gate

I001 can move from Review to Complete only after the synchronized owner-doc state is revalidated by project governance CI on the current branch head. The final status change must update owner docs first and then `docs/BOARD.md`.
