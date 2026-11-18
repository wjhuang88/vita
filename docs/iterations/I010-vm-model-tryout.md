# I010 — VM-only real model tryout

Status: Active (2026-11-17). Owner: VITA-001-E / VITA-001-D.

## Authorized outcome

The maintainer requests loading the local Talos model configuration into the running Vita VM and explicitly selected implementing guest model support. This is a new bounded engineering slice; prior offline iterations remain unchanged.

## Constraints and plan

- Hard: secrets never enter Git, ISO, build logs or model context; host config remains unchanged.
- Hard: real Talos SDK provider, nonroot session, only existing read-only system_summary tool; no shell or privileged model tool.
- Hard: explicit opt-in VM networking and readonly configuration share; no physical disk or host network reconfiguration.
- Soft: reuse pinned talos-provider and Rust native Linux APIs rather than subprocess network setup.
- Assumption: pinned Alpine virt modules support virtio-net and virtiofs; verify before guest acceptance.
- Prepare a private runtime directory containing only selected provider/model settings, mount readonly via virtiofs. Do not share ~/.talos (history, skills, other credentials).
- Opt-in build feature and boot token enable fixed QEMU guest networking; offline default remains available.
- Validate input/redaction/failure behavior, BIOS/UEFI offline regression, online guest reply and native tool round trip; independently review changed secret/boot surfaces.

## Acceptance

Given the operator explicitly launches the model-test VM with a private configuration share,
when Vita starts,
then it remains UID1000 on a readonly root and loads only selected model/provider settings.
When prompted to inspect its system, the real model invokes system_summary and answers from the guest result.
Missing/invalid config fails closed with generic diagnostics and never falls back to a different endpoint.
The secret is absent from tracked files, ISO and ordinary logs; the share is readonly; closing the VM removes transient configuration.

## Exclusions

Production USB config schema, full broker, host tools, skills/MCP/hooks, authenticated recovery, signed boot, hardware validation, release approval. This is not final product configuration or init selection.

## Evidence

Implementation and local test evidence are recorded below; complete acceptance remains pending the listed residual gates.

## Independent review and bounded test limitation

Separate reviewer found redirect-policy limitation in pinned Talos provider; see [open transport-policy residual](../backlog/active/VITA-001-E-transport-policy.md). Current trial uses only public demonstration prompts and VM system facts with the operator-selected HTTPS provider; strict transport confinement and security/release acceptance remain blocked. Provider errors do not select a fallback provider. Tool smoke now additionally requires a trusted native execution flag and expected guest kernel evidence rather than only a model ToolCall.

## Local validation checkpoint (2026-11-18)

Status: **Partial engineering slice; usable BIOS real-model trial delivered.** User accepted the local trial and authorized commit/push on 2026-11-18. No merge, production security or release approval.

- Branch `feat/vita-001-e-vm-model-tryout`, base `49c774a`; final local ISO SHA256 `5f155cb4c87693dabf536d038552a1cfe3b6f24cba23c2991f1d98c2514e81a1`. Arch dependency setup replaces apt in a temporary script copy, not the repository CI script.
- `cargo +1.99.0 test --manifest-path prototypes/interaction-runtime/Cargo.toml --features model-test`: 4 passed; init equivalent: 9 passed; `python3 scripts/dev/test_model_projection.py`: 2 passed. Default-feature unit tests also run; see local logs.
- `python3 scripts/dev/run_model_vm.py --iso target/vm-model-20261117/vita-model.iso --logs target/vm-model-20261117/bios-retry --smoke`: **PASS**, actual native tool execution marker plus kernel `6.18.58-0-virt`, memory `994744 KiB`, model answer. Guest readonly share/root and UID1000 verified.
- Synthetic malformed configuration VM: **PASS**, fail-closed generic error, no secret echo or prompt. Successful and failed smoke exits removed their private runtime share directories.
- Initial parallel BIOS/UEFI requests returned `[turn:error]`; sequential BIOS retry passed. UEFI retry booted readonly root, readonly config, network and UID1000, but model request still failed. Cause **unknown**; do not claim UEFI real-model acceptance.
- BIOS offline regression of the same payload with model boot token removed: **PASS**, two tool turns and expected error recovery, no network or shared configuration. UEFI offline result recorded separately below.
- Source and saved-log scan against the active credential: **PASS** (values never printed). Original local Talos config was not modified. Secret is projected only after building the ISO into per-user tmpfs, not supplied to the build.
- Separate reviewer `review_vm_model` performed two read-only executions: tool-evidence finding fixed and validated by final BIOS run; transport-policy finding remains OPEN, detailed in owner backlog. This is not exact-head PR approval.
- Local artifacts: `target/vm-model-20261117/` (ignored), including build/test/serial logs and resolved dependency locks. Interactive launcher intentionally does not record conversation content.

Remaining: UEFI online error diagnosis; provider no-redirect/HTTPS-only public transport API; complete fault/negative matrix and exact-head independent review/CI before merge. Product USB configuration, hardware and release gates remain open.

Final offline checks: BIOS and UEFI **PASS** on same model-capable payload with opt-in boot token removed, two turns plus expected error recovery. Default-feature tests: console 2/2, init 9/9. Governance validator and `git diff --check`: PASS. Interactive BIOS model VM launched with GTK and no QEMU startup errors; conversation logging disabled.
