# Security Procedure

Security review is mandatory for changes touching credentials, model endpoints, skill loading, privilege transitions, remote access or persisted history.

Checks:
- No credentials in Git, image layers, examples or ordinary logs.
- Redaction tests exist for token-like values.
- USB config/skills are treated as untrusted input until trust/signing policy is defined.
- External model output never directly acquires root authority.
- Capability allowlists and confirmations remain enforceable without relying on model compliance.
- Recovery mode does not silently inherit remote/model credentials.
- Network behavior is explicit and diagnosable.
