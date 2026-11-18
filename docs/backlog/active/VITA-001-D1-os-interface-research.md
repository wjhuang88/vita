# VITA-001-D1 — OS interaction and settings discovery

Type: Spike / scope decomposition. Parent: VITA-001-D; partners: VITA-001-C, E, F, G.
Priority: P0 candidate. Status: **Complete — research and task decomposition only** (2026-11-18). Implementation/experiments transferred to VITA-002.

## Outcome and scope

Operator needs a primary Vita/Talos interface supporting ordinary interaction and model/basic OS configuration, including when the model is unavailable. User requested research on 2026-11-18, not implementation of all settings.

Required reads: [research proposal](../../proposals/OS-INTERACTION-AND-SETTINGS-RESEARCH.md), [Talos boundary](../../reference/TALOS-INTEGRATION.md), [config contract](../../reference/CONFIG-CONTRACT.md), [ADR-0002](../../decisions/ADR-0002-governed-capability-broker.md), [ADR-0005](../../decisions/ADR-0005-rust-first-system-capabilities.md), [I010](../../iterations/I010-vm-model-tryout.md).

Hard: Talos-primary nonroot session; all privileged mutations use typed broker, target/plan/confirmation/audit; no secrets in model context or Git; boot changes require VM proof. Basic local configuration must not depend on successful model connection (proposed product requirement pending baseline acceptance).
Soft: Rust TUI with reusable components; terminal backend and network service choice require evidence before ADR.
Assumptions: PTY/serial provides adequate Chinese/resize behavior; components can be reused within footprint; service backends expose recoverable operations. All remain to be tested.

## Research acceptance and evidence

- [x] Inventory existing SDK, TUI dependencies and Vita capability gaps.
- [x] Compare approaches, preserve accepted SDK/Talos-primary boundaries.
- [x] Map model and OS settings, offline flow, privilege and config transaction requirements.
- [x] Identify unvalidated terminal, secret persistence and service-backend assumptions.
- [x] Propose dependent slices and observable acceptance, without declaring them ready iterations.

Commands: `sh scripts/validate_project_governance.sh .`; `git diff --check`. Both PASS on 2026-11-18.
No UI prototype, VM changes, network/credential changes, architectural approval, commit or merge performed as part of this research.

## Remaining discovery / readiness gates

1. S1 terminal spike: PTY/serial resizing, keyboard/Chinese/paste, single renderer, crash cleanup; measure CPU/RSS/latency and compare candidate component boundary.
2. Select network/service configuration backend with dependency/license/service footprint evidence, not assumed systemd APIs.
3. Specify revisioned configuration transaction, credential reference/lifecycle, offline confirmations and backend authentication; separate durable storage from temporary session.
4. Bound first implemented settings (proposed: model connection plus basic status), write Given/When/Then acceptance and route through iteration intake.
5. Preserve I010 UEFI online failure and transport policy open gates; this Spike does not supersede or close them.

Candidate S1-S6 decomposition lives in the proposal; no implementation story is Ready until scope/dependencies/evidence are accepted. Independent hardware/security/release gates remain required.

## Development handoff (2026-11-18)

User authorized development task records and GitHub Issues. [VITA-002](VITA-002-os-interaction-settings.md) owns executable child acceptance and remaining discovery; original research history above is preserved. This closes desk research, not TUI implementation or security validation.
