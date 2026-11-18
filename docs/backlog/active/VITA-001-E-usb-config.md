# VITA-001-E — Talos loads model and Agent configuration from the USB

As an operator,
I want the Live OS to load portable Talos/model configuration from a documented USB directory,
so that the same device can connect to my selected model provider without rebuilding the image.

Initial contract under validation:
- USB data root: `/talos/`
- Model config: `/talos/config/models.toml`
- Agent/session config: `/talos/config/agent.toml`
- Optional skills/extensions: `/talos/skills/`
- Runtime workspace: `/talos/workspace/`
- Plaintext API secrets are not required or encouraged in tracked/example config.

Acceptance:
- Given valid config exists on the USB data partition
  When the Live OS starts Talos interaction
  Then provider/model/session settings are loaded and the source path is visible in diagnostics.
- Given config is missing or invalid
  When Talos starts
  Then the system provides an actionable local setup path and does not expose secret material in logs.
- Given a config refers to a credential source
  When diagnostics are shown
  Then credential values are redacted.

Dependencies: VITA-001-C, VITA-001-D.

## I010 VM-only model tryout (2026-11-17)

Active bounded precursor: [I010](../../iterations/I010-vm-model-tryout.md). User authorized real model access using a minimal readonly runtime projection of local Talos config. Product USB config remains unfinished.

Open security gate: [provider transport policy](VITA-001-E-transport-policy.md).

2026-11-18 checkpoint: I010 BIOS real-model native tool trial usable; UEFI online calls returned generic errors (cause unknown), and upstream redirect-policy security gate remains open. See I010 evidence and residual work; product Story stays Active.
