# I009a — Minimal independent PID 1 and Agent session lifecycle

Status: Review (original guest and regression CI passed; follow-up exact-head housekeeping CI pending)
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

## Proof log — source `333e8da54c59bbd13750726ccc8782c2a7ae99b6` (2026-11-07)

- [I009a run 42373604370](https://github.com/wjhuang88/vita/actions/runs/42373604370): `init-session-guest` **SUCCESS**, `governance` **SUCCESS**.
- Static Rust/musl candidate and native Talos SDK built on GitHub-hosted x86_64; 3 Rust unit tests passed (boot-token fail-closed, restart window, TTY foreground requirement).
- From a **real generated GRUB ISO** (not direct QEMU kernel injection): BIOS and OVMF UEFI both returned `VITA_I006_LINUX_TALOS_GUEST_OK turns=2 errors=1`, `VITA_I009A_ISO_GUEST_TALOS_OK mode=bios|uefi`, and `VITA_I009A_LINUX_TALOS_ISO_VM_OK`.
- Each guest log independently verifies `VITA_I009_PID1_MINIMAL_INIT_READY`, `VITA_I009_TALOS_UID=1000`, `VITA_I009_CONTROLLING_TTY_OK` and `VITA_I009_ORPHAN_REAPED_OK`, followed by normal session exit.
- A separately built, diskless BIOS crash-fixture ISO returned `VITA_I009A_CRASH_POLICY_VM_OK starts=3 crashes=3`, `VITA_I009_CRASH_BUDGET_EXHAUSTED attempts=3` and `VITA_I009_FAILSAFE_NO_ROOT_SHELL`. **No actual root shell**, guest network or host writable disk.
- [I009a retained serial/provenance artifact 13574995716](https://github.com/wjhuang88/vita/actions/runs/42373604370/artifacts/13574995716): SHA-256 provenance, GRUB config, BIOS+UEFI guest serial/QEMU logs, bounded crash evidence. No ISO binary uploaded.
- All original-head regressions: [I001/I008 run 42373604351](https://github.com/wjhuang88/vita/actions/runs/42373604351), [I005 run 42373604145](https://github.com/wjhuang88/vita/actions/runs/42373604145), [I006 run 42373604179](https://github.com/wjhuang88/vita/actions/runs/42373604179), [I007 run 42373604333](https://github.com/wjhuang88/vita/actions/runs/42373604333) **SUCCESS**. Total 11 exact-original-head check runs all green.
- Final follow-up commit removes one unused Rust import, ensures shutdown signal state is processed once, and records this evidence. **Its own exact-head CI is PENDING**; keep PR Draft until repeated real guest/provenance and regressions pass.

**Boundary:** this is a dedicated disposable VM engineering candidate only; it is not the final Vita init design, immutable Linux rootfs, product-grade session/account policy, complete shutdown, privileged Broker or recoverable physical USB OS. All [I009a human reviews](../validation/I009a-HUMAN-ACCEPTANCE.md) remain unchecked.
