# I001 — Foundation: boot decision + Talos interaction boundary

Status: Complete
Published baseline: 2026-11-03
Completed: 2026-11-03

## Objective
Reduce the two largest architecture uncertainties before broad implementation: portable boot substrate and Talos Core integration boundary.

## Selected work
- VITA-001-A boot strategy Spike.
- Refine VITA-001-D with evidence from the current Talos repository/core API shape.
- Confirm or amend ADR-0001/0002 assumptions.

## Runnable/testable deliverable
A minimal VM-bootable proof that reaches a project-owned Live environment or boot stub, plus a host-runnable Talos interaction prototype that demonstrates a structured system-inspection capability.

## Exit criteria
- [x] Boot strategy ADR exists and releases the VITA-001-B/C/H architecture dependency.
- [x] Talos integration is proven at Core/runtime level, not only CLI wrapping.
- [x] Residual unknowns have explicit owner stories or later-stage destinations.
- [x] Synchronized owner docs passed governance validation before closure.

## Execution evidence — 2026-11-03

### Boot strategy
- `docs/reference/BOOT-STRATEGY-RESEARCH.md` records the Ventoy/GRUB evidence and option comparison.
- ADR-0004 selects the Phase 1 hybrid architecture: pinned/unmodified Ventoy substrate + Vita-owned Live OS/runtime.
- `prototypes/boot/ventoy/ventoy.json` owns the Vita product configuration boundary.
- `scripts/ci/validate_i001_boot.sh` constructs a target-guarded disposable raw disk, verifies its loop backing identity, verifies the pinned Ventoy archive, stages deterministic ISO fixtures, and executes QEMU smoke paths.
- GitHub Actions run `41659336430`, job `boot-chain-prototype`, passed at commit `68c25efd89aa12ca0a611fbce4ea6605907cbebd` with:

```text
VITA_BOOT_PROBE_OK mode=bios marker=VITA_BOOT_OK
VITA_BOOT_PROBE_OK mode=uefi marker=VITA_BOOT_OK
VITA_BOOT_PROBE_OK mode=bios marker=VITA_USER_ISO_BOOT_OK
VITA_I001_BOOT_CHAIN_OK ventoy=1.1.18
```

### Talos integration boundary
- Talos inspected and pinned for the prototype at `wjhuang88/talos@261602242e49e6f921f5d3e4eac96eebbe92f8b3`.
- `talos-runtime` is confirmed as the supported standalone embedding facade; CLI/TUI remain outside Vita's embedding contract.
- `prototypes/interaction-runtime/` runs a deterministic network-free turn through `RuntimeBuilder`, a Vita-owned read-only `system_summary` `AgentTool`, and an explicit Read permission rule.
- GitHub Actions run `41659336430`, job `talos-runtime-prototype`, compiled Talos Runtime v0.10.1 from the pinned revision under Rust 1.99.0 and finished with:

```text
VITA_I001_TALOS_RUNTIME_OK
```

### Governance and synchronized closure
- Run `41659336430` passed `sh scripts/validate_project_governance.sh .` while the executable proofs were green.
- Owner docs were then synchronized: VITA-001-A archived as Complete, B dependency released, I001 moved to Review, backlog updated, then the derived board updated.
- GitHub Actions run `41660190624` revalidated that synchronized Review state; `governance`, `talos-runtime-prototype`, and `boot-chain-prototype` all concluded `success`.
- Reusable failure lessons were promoted to `EVOLUTION.md` and `docs/sop/BOOT-MEDIA-VALIDATION.md` before final closure.

## Final disposition

- VITA-001-A: Complete and archived.
- VITA-001-B: Ready for a representative real-ISO compatibility slice.
- VITA-001-C: dependency released but remains in Refinement until the concrete Live OS base/build contract is selected.
- VITA-001-D: Ready; the supported Talos embedding boundary is proven, while real console/Live integration remains implementation work.
- VITA-001-H: remains Refinement and depends on the final Live OS from C for release-grade image production.

## Residual work

I001 deliberately does not claim:
- physical USB/firmware validation;
- Secure Boot support;
- real third-party distro ISO compatibility beyond deterministic probe images;
- a final Vita Live OS rootfs/initramfs;
- Talos console/PTY rendering, privileged capability broker integration, or crash recovery in the Live system.

These are owned by VITA-001-B/C/D/F/H and later release hardening. No residual item invalidates the two architecture decisions proven by I001.

## Retrospective

The most useful I001 finding was that documentation-level capability claims were insufficient for both major decisions. Running the real paths exposed clean-runner dependencies, an interactive Ventoy secondary menu and the need for stronger disposable-target identity checks. Those lessons are now durable controls rather than tribal knowledge.
