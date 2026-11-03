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
