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

## System/user image boundary (2026-11-16)

Vita system image/kernel/rootfs/rollback artifacts **must not share the user-imported ISO namespace or update target**. [ADR-0009](../decisions/ADR-0009-system-user-image-isolation.md) requires a separately identified P3 Vita-system domain and P1 user-image domain; neither a matching `vita-live.iso` name nor a user-writable Ventoy menu entry is system identity. Treat P1 boot-menu/config manipulation, malicious user images, absent/ambiguous P3, unexpected symlinks, tampered manifests, interrupted updates and cross-partition copy/write bugs as explicit threats. The system entry must fail closed and system update must not write user ISO files; no automatic host-disk scan/mount. Separate partitions are not signature verification or full-disk security. Disposable-loop proof, human physical-media and Secure Boot acceptance remain required under [#27](https://github.com/wjhuang88/vita/issues/27).

## Required mitigations for first release
- Explicit target identity for writes.
- Typed capability allowlist and risk classes.
- Confirmation for destructive actions.
- No secret values in ordinary logs.
- Clear trust boundary for `/talos/skills` and config.
- Recovery path independent of external model availability.
- VM-first boot/media validation.

Open questions belong in backlog/Spikes rather than being silently assumed safe.
