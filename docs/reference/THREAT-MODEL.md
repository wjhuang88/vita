# Initial Threat Model

## Assets at risk
- Host disks and partitions.
- User files on attached storage.
- Model/API credentials.
- Network configuration and connectivity.
- Boot firmware/bootloader state.
- Audit/history that may contain sensitive system output.

## Primary threats
1. Wrong-disk writes caused by ambiguous device enumeration.
2. Model hallucination or prompt injection causing unsafe system actions.
3. Malicious/untrusted ISO or USB-resident config/skill content.
4. Credential leakage through logs, prompts or persisted history.
5. Privilege confusion between Talos runtime, capability providers and recovery shell.
6. Compromised network/model endpoint influencing system actions.

## Required mitigations for first release
- Explicit target identity for writes.
- Typed capability allowlist and risk classes.
- Confirmation for destructive actions.
- No secret values in ordinary logs.
- Clear trust boundary for `/talos/skills` and config.
- Recovery path independent of external model availability.
- VM-first boot/media validation.

Open questions belong in backlog/Spikes rather than being silently assumed safe.
