# Portable Configuration Contract — v0

Status: first-round requirement; schema not yet frozen.

Expected logical paths:
- `/talos/config/models.toml`
- `/talos/config/agent.toml`

Model config may contain:
- provider identifier
- base URL / endpoint
- model name
- timeout/retry policy
- credential **reference**, not a required plaintext secret

Agent config may contain:
- default system profile/persona
- enabled capability groups
- confirmation policy overrides within hard safety bounds
- workspace path
- optional skill directories

Secrets:
- Never committed to the repository.
- Diagnostics must redact credentials/tokens.
- Plaintext removable-media secrets, if supported at all, require an explicit threat-model decision and warning.
