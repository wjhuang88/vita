# VITA-001-G — Operator can enter a compatibility/recovery shell

Status: Refinement (I007 diskless QEMU-only precursor; C/D dependencies still Active)

As an operator,
I want an explicit conventional shell fallback,
so that I can repair the Live OS when Talos or model connectivity is unavailable.

Acceptance:
- Given Talos cannot initialize or the operator requests recovery mode
  When the recovery action is chosen
  Then a local shell is available with clearly documented privilege state.
- The recovery shell is not the default happy-path interaction entry.

Dependencies: VITA-001-C, VITA-001-D.

## I007 verified experimental recovery precursor

One exact kernel argument vita.recovery=1 selects an offline local root BusyBox shell in a diskless, networkless QEMU guest; the default remains the Talos SDK session. The kernel-argument parsing fails closed on invalid or repeated values. No normal model interaction can launch this fallback. [CI 41970334220](https://github.com/wjhuang88/vita/actions/runs/41970334220) confirms this VM-only path and a real executed command. An explicit Cargo experimental-root-recovery build feature is disabled in normal builds. Production boot-menu selection, physical root-shell authentication, mature init, reliable job control and actual offline repair workflow remain unimplemented. See ADR-0007 and docs/validation/I007-HUMAN-ACCEPTANCE.md. Do not close Story G from I007.
