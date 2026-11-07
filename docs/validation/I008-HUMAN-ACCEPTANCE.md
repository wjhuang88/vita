# I008 — Human acceptance: GRUB ISO firmware and real guest Talos

Status: HUMAN / SECURITY / LICENSING / HARDWARE REVIEW PENDING.
Automated QEMU completion does not check these boxes.

- [ ] HUMAN REQUIRED — Review kernel/Alpine, GNU GRUB, Rust/musl/nix/Talos SDK provenance and source/redistribution obligations before any release of this ISO.
- [ ] HUMAN REQUIRED — Verify the actual controlling TTY/PTY experience: terminal echo, Ctrl+C, session foreground process group, signal handling and error recovery.
- [ ] DEFERRED — Add complete read-only rootfs, mature init/reaper and real model/config with secret redaction.
- [ ] DEFERRED — Add a secure independent recovery path; **the I008 ISO deliberately omits** the experimental unauthenticated I007 root BusyBox shell.
- [ ] DEFERRED — Physical BIOS/UEFI matrix, Secure Boot policy, USB identity verification, persistence, release images and signed checksums.
- [ ] BLOCKED SEPARATELY — Ventoy third-party ISO handoff Issue #5; a Vita-owned ISO is not evidence Alpine third-party ISO compatibility.

Reviewer/date:
Exact Vita commit and CI:
Physical firmware/terminal matrix:
License/source/security findings:
Open defects and follow-ups:
Verdict: PENDING / PASS / FAIL / BLOCKED.
