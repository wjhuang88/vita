# VITA-001-G — Operator can enter a compatibility/recovery shell

As an operator,
I want an explicit conventional shell fallback,
so that I can repair the Live OS when Talos or model connectivity is unavailable.

Acceptance:
- Given Talos cannot initialize or the operator requests recovery mode
  When the recovery action is chosen
  Then a local shell is available with clearly documented privilege state.
- The recovery shell is not the default happy-path interaction entry.

Dependencies: VITA-001-C, VITA-001-D.
