# Human acceptance and deferred validation

Required human interaction, real hardware, terminal compatibility and security judgment are tracked explicitly here rather than counted as automated success.

- [I004 Rust Console Human Acceptance](I004-HUMAN-ACCEPTANCE.md): real terminal Ctrl+C, cancellation, PTY/TTY matrix, security review, and deferred boot/recovery requirements.

Each check should record the exact commit SHA, environment, redacted evidence and PASS / FAIL / BLOCKED status. Unchecked checks are never treated as complete.
- [I005 Linux/Rust Boot Human Acceptance](I005-HUMAN-ACCEPTANCE.md): Alpine base review, source-license obligations, future UEFI/USB/rootfs/Talos session gates.
- [I006 Linux Guest Talos Session Human Acceptance](I006-HUMAN-ACCEPTANCE.md): guest mount helper and read-only policy review, licensing, TTY, live/rootfs, recovery and device gates.
- [I007 Volatile Runtime and Offline Recovery](I007-HUMAN-ACCEPTANCE.md): mandatory security review of unauthenticated local root rescue, licensing, terminal/firmware validation.
