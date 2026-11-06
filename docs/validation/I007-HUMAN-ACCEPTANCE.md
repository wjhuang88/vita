# I007 — Human acceptance: runtime mounts and offline root recovery

Status: HUMAN / SECURITY / LEGAL REVIEW PENDING.
This list is for a separate guided acceptance chat and must remain unchecked after automated CI.

**AUTOMATED VM-ONLY VALIDATION: PASS** — [run 41970334220](https://github.com/wjhuang88/vita/actions/runs/41970334220), source `2b8c9ceded2ddb50eba7ff508532643702d85ebb`, normal Talos/Tool and offline recovery both exercised in different QEMU Linux guests. SHA-256-pinned Alpine static BusyBox package and ELF used. This does **not** constitute a human/physical/security approval.

## HUMAN-001 — privilege and recovery security

- [ ] HUMAN REQUIRED — Evaluate the physical-access and authentication consequences of kernel argument vita.recovery=1 enabling an **unauthenticated root shell**. Review boot-menu lock, local attacker and Secure Boot policies before any hardware distribution.
- [ ] HUMAN REQUIRED — Audit the compile-time experimental-root-recovery feature gate (off by default) AND the exact token parser: malformed/duplicate values fail closed; no model prompt, tool output, environment variable or Talos failure can silently enter recovery.
- [ ] HUMAN REQUIRED — Review that recovery environment is cleared of API credentials and that the QEMU test exposes no guest network, physical media or host disk.
- [ ] HUMAN REQUIRED — Inspect restricted /proc, readonly /sys, bounded tmpfs /run and /tmp. This is NOT an immutable read-only rootfs.
- [ ] HUMAN REQUIRED — Design non-root Talos interactive session privilege and governed broker isolation before shipping.

## HUMAN-002 — licensing and provenance

- [ ] HUMAN REQUIRED — Review SHA-pinned Alpine kernel and Alpine v3.24 BusyBox-static APK (1.37.0-r31), exact package + extracted binary SHA-256, source and GPL redistribution obligations. Also account for the earlier Ubuntu BusyBox SIGILL under QEMU's default x86_64 CPU; do not claim real hardware portability without CPU-matrix evidence. CI package logging is not production approval.
- [ ] HUMAN REQUIRED — Assess Rust/musl/nix/Talos license/source inventory for eventual distribution.

## HUMAN-003 — product integration and real hardware

- [ ] DEFERRED / NOT IMPLEMENTED — A product-owned Live ISO with explicit recovery choice, firmware menu and validated local root-shell authentication.
- [ ] DEFERRED / NOT IMPLEMENTED — Mature init/service supervision, actual offline recovery when Talos cannot start, cleanup/reboot and fail-closed physical security.
- [ ] HUMAN REQUIRED — **TTY/PTY and job control:** recovery serial log reported `sh: can't access tty; job control turned off`; reproduce on real TTY/PTY, check controlling-terminal/session setup, prompt/echo, Ctrl+C, signals and shell child control. A functional serial test command does not validate interactive operator usability (also see I004).
- [ ] DEFERRED / NOT IMPLEMENTED — Real USB identity/write safety, persistent state, UEFI/Ventoy boot, Secure Boot decision and full hardware matrix.
- [ ] BLOCKED SEPARATELY — Issue #5: real Alpine third-party ISO Ventoy handoff.

## Independent evidence handoff

Reviewer/date and Vita SHA/CI:
Linux/TTY environment, kernel and BusyBox hashes:
- Pinned Alpine BusyBox-static APK SHA-256: `b0d6cfc585d7fdd12df359cc5d38d5e45894e9e9e78c4059eb41577d022ec734`
- Extracted static BusyBox executable SHA-256: `1690ed6db71fa35621537580135deb8de4f55f30a50d9d591050256e334cd8b9`
Source/license/security review findings:
Redacted default/recovery terminal transcripts:
Firmware/physical matrix status:
Open defects/issues/PRs:
Verdict: PENDING / PASS / FAIL / BLOCKED.

Automated guest success alone never clears a human box.
