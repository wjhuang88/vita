# I005 — human validation and later-stage handoff

**Status: HUMAN REVIEW / FUTURE HARDWARE VALIDATION PENDING**
**Scope:** I005 isolated Linux kernel / Rust PID 1 boot proof. This is **not** a usable Vita Live OS image or a USB installer. Intended for completion in another conversation.

## MANUAL-001 — independent kernel/initramfs review

- [ ] **HUMAN REVIEW** — Inspect [ADR-0006](../decisions/ADR-0006-alpine-live-base.md) and confirm Alpine/musl is an acceptable initial development base for the future Live OS.
- [ ] **HUMAN REVIEW** — Check the upstream Alpine kernel/package provenance, redistribution license obligations and a repeatable source-compliance plan before publishing any image.
- [ ] **HUMAN REVIEW** — Examine CI script for unintended host-device writes: it must create only temporary initramfs files and execute QEMU, with no `dd`/disk installer/physical device target.

## MANUAL-002 — end-user system (deferred until implemented)

- [ ] **NOT IMPLEMENTED / DEFERRED** — Boot an actual Vita Live ISO on supported x86_64 UEFI machine and reach a project-owned Talos default interaction session.
- [ ] **NOT IMPLEMENTED / DEFERRED** — Verify an independent recovery shell when the Talos host cannot start, without an online model.
- [ ] **NOT IMPLEMENTED / DEFERRED** — Confirm actual writable runtime/persistent data mounts and preservation of user ISO/config data.
- [ ] **NOT IMPLEMENTED / DEFERRED** — Verify real-device boot under explicit source/target identity and confirmed physical media safety gates; I005 must not be used to write an external drive.
- [ ] **NOT IMPLEMENTED / DEFERRED** — Secure Boot support decision and firmware/device matrix.

## MANUAL-003 — actual console behavior

- [ ] **SEPARATELY PENDING** — Complete real TTY/PTY Ctrl+C/exit and security review in [I004 checklist](I004-HUMAN-ACCEPTANCE.md); I005 does not replace this work.

## Evidence template for the separate acceptance session

- Reviewer / date:
- Vita branch, commit and workflow URL:
- Alpine version and exact ISO/kernel SHA-256:
- Source/license review outcome:
- Safety/code review findings:
- Firmware/device matrix (only when separately authorized and implemented):
- Failures, reproductions, GitHub issues:
- Overall verdict: **PENDING / PASS / FAIL / BLOCKED**

Do not mark any unchecked or not-implemented item complete on the strength of the QEMU Rust-init smoke proof.
