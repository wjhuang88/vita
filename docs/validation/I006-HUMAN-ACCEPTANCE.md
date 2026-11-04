# I006 — separate human verification and deferred acceptance

**Status: HUMAN VALIDATION PENDING**
**Owner:** independent operator/reviewer, possibly guided by a separate ChatGPT conversation.
**Purpose:** do not conflate successful QEMU Linux/Talos fixture integration with shipping a product Live OS.

## HUMAN-001 — independent boot and privilege audit

- [ ] **NEEDS HUMAN** — Inspect `scripts/ci/validate_i006_guest_session.sh`, the Python serial observer and `prototypes/guest-session-init/src/main.rs`. Verify no physical device target, host mount, destructive disk writer, unverified network code or credential is introduced.
- [ ] **NEEDS HUMAN** — Review the constrained Rust-native `nix` mount of guest `/proc` with `nodev/noexec/nosuid` flags, and verify no generic mount authority is exposed to Talos.
- [ ] **NEEDS HUMAN** — Confirm only the read-only Talos `system_summary` tool is enabled; default denial of unsafe operations is not bypassed by the guest host.
- [ ] **NEEDS HUMAN** — Review Alpine Linux kernel, Rust/musl and `nix` dependency license/source provenance before publishing an image. CI source hashes are not a production-grade SBOM.

## HUMAN-002 — operator experience and recovery

- [ ] **DEFERRED / NOT IMPLEMENTED** — Integrate a mature Linux init/service manager and independent offline recovery shell; make recovery accessible when Talos cannot launch or crashes.
- [ ] **NEEDS HUMAN / I004** — In a real TTY and PTY, test Ctrl+C, live input, stdin echo, cancellation, terminal handling, and redacted system information. See [I004 validation](I004-HUMAN-ACCEPTANCE.md).
- [ ] **DEFERRED / NOT IMPLEMENTED** — Real LLM provider configuration and secret handling from a writable Vita USB data area; test signed/approved privileged capabilities.

## HUMAN-003 — actual portable boot

- [ ] **DEFERRED / NOT IMPLEMENTED** — Boot a complete Vita Linux ISO through selected Ventoy substrate in UEFI and any declared BIOS mode, and confirm the Talos primary session launches without starting bash.
- [ ] **DEFERRED / NOT IMPLEMENTED** — Validate persistent data layout, read-only base, recovery/upgrade, Secure Boot policy, physical USB media identity and target confirmation.
- [ ] **BLOCKED SEPARATELY** — Real third-party Alpine ISO under Ventoy remains Issue #5 / Draft PR #4; do not reopen this by copying I006's direct kernel proof.

## Evidence handoff (for another conversation)

- Date, reviewer, exact Vita SHA and PR/CI URL:
- Alpine ISO/kernel SHA, Rust/musl/nix source and licenses:
- Console TTY/PTY environment and redacted transcript:
- Boot-script / privilege review PASS/FAIL:
- Recovery shell and Live ISO validation PASS/FAIL/NOT IMPLEMENTED:
- Physical device matrix (ONLY when separately authorized and safe):
- Issues/PRs raised, remaining acceptance gates:
- **Final human verdict: PENDING / PASS / FAIL / BLOCKED**

All unchecked items stay Pending. Automated CI is not a substitute for a legal/physical/security review.
