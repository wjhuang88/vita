# I009a — Independent Rust init and non-root Talos session (engineering only)

This crate tests whether **a small Rust PID 1** is viable for Vita's small-and-complete Agent OS architecture [ADR-0008](../../docs/decisions/ADR-0008-linux-init-service-supervision.md). It is a **candidate**, not a final choice or a release init.

- Only an isolated disposable QEMU initramfs: Linux PID 1 mounts fixed proc/sysfs/tmpfs paths, validates real mount table and reaps children by \`waitpid(-1, WNOHANG)\`.
- Session helper (the same static binary in \`--session-child\` mode) drops to fixed UID/GID 1000, invokes \`setsid\`, opens \`ttyS0\` and verifies session/foreground controlling TTY state via \`/proc/self/stat\`; runs the actual embedded Talos SDK host, not Talos CLI.
- A bounded three-launch, one-minute restart policy prevents rapid crash loops and leaves an inert PID 1 failsafe with no root recovery shell. VM-only fixture tokens enable orphan and crash tests.
- No dynamic scripting language in guest, no LLM/model keys, no network or physical disk. I007 root rescue feature is **never** packaged.
- Does not yet implement full shutdown poweroff, independent authenticated recovery, initramfs switch_root, device events or product-grade privilege policy; explicit future human/VM gates remain.

CI entry: \`bash scripts/ci/validate_i009a_init_lifecycle.sh\`. Retains serial logs and hashes but does not release experimental ISO.

Independent AI code review found fatal-path reaping, Ctrl+C and late orphan-marker gaps on the first head. The follow-up branch adds a non-restarting safe reaper path, launcher-only SIGINT handling, and dedicated BIOS guest scenarios for Ctrl+C and injected session-spawn failure. These are **pending exact-head CI and AI re-review**, not evidence of a production init.

Follow-up independent review also required a true `boot_pid1() -> Err` test with orphan and signal after fallback, protection against confusing the helper PID with an orphan, and a shutdown guard inside the restart backoff. The new fatal fixture verifies that **two distinct child PIDs** are reaped, including the known orphan parent, before accepting SIGTERM. Final exact-head CI and AI re-review remain required.
