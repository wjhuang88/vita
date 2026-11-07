# I009a — Minimal Init / Session: Human Acceptance

All items remain **PENDING**, unless a reviewer records an exact commit and evidence. QEMU-only proof never authorizes real USB writes or release.

- [ ] Inspect whether the candidate Rust PID 1 truly reaps *orphaned descendants*, forwards/shuts down signals and survives Talos/model failure; examine panic, crash-loop, timeout and no-terminal behavior.
- [ ] Validate session UID/GID, credentials and controlling TTY / foreground process group on actual physical and USB serial/TTY hardware; serial fd redirection alone is insufficient.
- [ ] Review local offline recovery authentication, secure-boot and firmware trust claims, no implicit root shell, persistence state security and capability broker separation.
- [ ] Review full source/license/SBOM obligations for Linux kernel, firmware, Rust static binary/toolchain, any future BusyBox or Dinit/Finit components.
- [ ] Approve physical removable-device target identity and destructive-write SOP only after independent VM gates and explicit hands-on authorization.
- [ ] Inspect standard Linux device/network/Wi-Fi module, user applications, boot menu, poweroff and crash recovery beyond the minimal QEMU guest.

State: Not approved. I009a and I009 full Live OS remain engineering work.
