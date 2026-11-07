# I009a — Minimal independent PID 1 and Agent session lifecycle

Status: Active (published design baseline; code/guest CI evidence pending)
Published: 2026-11-07
Story: VITA-001-C (Active); [Issue #12](https://github.com/wjhuang88/vita/issues/12)
Requires: I008 merged as `f24c1498` after all nine exact-head CI checks.
Decision owners: [ADR-0008](../decisions/ADR-0008-linux-init-service-supervision.md) and [Vita OS design contract](../reference/VITA-OS-DESIGN.md).

## Immutable goal

Move beyond the I008 Rust-PID1 sleep loop into a **bounded VM-only process-lifecycle proof** without changing established firmware/kernel or embedded-Talos behavior. PID 1 is only an independent reliable bootstrap/reaper; Talos lives as a separate unprivileged session process. This is *not* a commitment to a permanent custom Rust init before comparison with Dinit/Finit.

## Machine-observable acceptance

- Boot a **Vita-built ISO** through both BIOS and OVMF UEFI in separate disposable diskless QEMU guests; not just direct `-kernel`, not just a menu.
- Guest asserts PID 1 identity; spawns native embedded-Talos session as non-root with explicit UID/GID and no inherited root credentials; proves real read-only Tool success/error/retry and clean exit.
- Reap exited child processes including at least one short-lived/orphan descendant; test bounded crash restart and fail-safe when budget exhausted; handle termination/shutdown signals rather than silently sleeping after errors.
- Ensure PID 1 and privileged broker are separate from Talos/model session. No root shell in default ISO; I007 recovery feature remains excluded.
- Record correct `/proc`/`/sys`/`run`/`tmp` mounts, exact kernel/ISO/binary SHA, serial evidence and failures. Preserve I001/I005/I006/I007/I008 checks.
- **Real controlling TTY** (session leadership, correct ioctl and foreground group) remains an explicit follow-on gate if not independently verified in this slice. Do not claim it passed from stdio via /dev/ttyS0.
- No physical devices or media writes; retain manual licensing, firmware, TTY, recovery/security and full rootfs gates as PENDING.

## Excluded

No finished read-only rootfs, persistent USB config, Dinit/Finit final decision, real device compatibility, OTA/A-B updates, privileged model tools, background broker release or distribution artifacts.

## Proof log

PENDING — add exact-head CI runs and guest markers after implementation, keep State Active/Review until then.
