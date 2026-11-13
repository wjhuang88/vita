# I009a — Minimal independent PID 1 and Agent session lifecycle

Status: Complete (VM-only engineering slice; product/physical/manual acceptance pending)
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

## Independent AI review round 1 (2026-11-12)

A separately executed [GitHub Copilot Code Review](https://github.com/wjhuang88/vita/actions/runs/43249443015) on source `5b40d3848c5cde64a897e677e83c2378fe002b78` returned **Changes recommended** with three unresolved findings: (HIGH) PID 1's sleep-only fatal path stopped reaping/signals, (MEDIUM) foreground launcher Ctrl+C caused false session crashes, and (MEDIUM) normal QEMU observer raced the separate orphan-reaping marker. This follows the single-maintainer delegation decision in [PR #14](https://github.com/wjhuang88/vita/pull/14).

Follow-up fix scope:
- Preserve a reaping/signals loop on failure; no restart after a session spawn error, include a guest-injected spawn failure followed by orphan reaping and PID 1 SIGTERM.
- Catch SIGINT in the session launcher (not Talos) so native Talos Ctrl+C handling remains functional; add BIOS ISO Ctrl+C guest proof.
- Require the orphan marker before QEMU observer exit; eliminate the flaky late-marker race.
- **All revised exact-head CI and AI re-review remain PENDING** until independently checked, and all human/legal/physical gates remain unapproved.

## Independent AI review round 2 (2026-11-12)

Copilot's [second exact-head review](https://github.com/wjhuang88/vita/actions/runs/43251358633) marked **all three first-round findings resolved**, then flagged:
1. Missing guest coverage of the *actual* fatal `boot_pid1() -> Err` / fallback reaper.
2. The signal helper PID could be falsely counted as the adopted orphan.
3. A pending shutdown during the 120 ms restart backoff might still launch another session.

Follow-up changes:
- A dedicated fatal boot token returns an error from PID 1 **after forking the orphan-parent fixture**; the adopted grandchild itself signals PID 1. A separate BIOS guest observer requires the fatal diagnostic, both the known direct parent and a **distinct reaped grandchild PID**, and subsequent SIGTERM handling.
- Track the `--signal-parent` helper's PID separately in the nonfatal fixture; never count it as an adopted orphan.
- Check for pending stop before retry-budget allocation and again after restart backoff; do not launch an additional Talos session once shutdown is requested.

This is still a **VM-only engineering candidate**. Final exact-head CI, third AI re-review and human/manual approval gates remain **PENDING** until verified. Do not close story C or declare product init complete.

## Independent AI review round 3 — ownership / read-boundary hardening (2026-11-12)

[Third independent Copilot review](https://github.com/wjhuang88/vita/pull/14) confirmed the signal-helper PID fix and the retry/shutdown guard. Its remaining substantive findings:
- **HIGH:** initramfs `cpio` previously retained the build host's UID/GID; on a UID 1000 host the non-root session could own system binaries or `/dev`. **Fix:** use `cpio --owner=0:0` and assert actual guest inodes for `/init`, `/vita`, Talos executable and `/dev` are root-owned and not group/other writable *before* setting the dedicated serial device owner.
- **MEDIUM:** a serial chunk ending before the colon in `VITA_I006_GUEST_SESSION_ERROR:` could trigger an indexing error. **Fix:** match the complete delimiter, add host-side unit tests for split-message boundaries.
- The earlier fatal fallback fixture executed on [I009a CI 43253474750](https://github.com/wjhuang88/vita/actions/runs/43253474750) and emitted `VITA_I009A_FATAL_BOOT_REAPER_VM_OK parent=442 reaped=[442, 443]` (two distinct reaped child PIDs) plus shutdown-signal evidence.

After these changes the **new exact-head CI and independent AI re-review remain PENDING**. This prototype is never a shipping recovery or physical USB approval.

## Independent AI review round 4 — live-session shutdown gate (2026-11-12)

Fourth separate [Copilot Code Review](https://github.com/wjhuang88/vita/pull/14) confirmed the cpio `root:root` ownership isolation fix and arbitrary serial-read boundary fix. It flagged a **previously untested live-session shutdown** path for closer assessment: PID 1 receives SIGTERM while Talos owns the controlling TTY, and must not respawn or abandon its child. A dedicated **BIOS guest ISO** now starts a real non-root Talos console, delivers SIGTERM to PID 1 via a guest-local helper while the session is active, and requires `VITA_I009_SHUTDOWN_REQUESTED` plus `VITA_I009_SHUTDOWN_SESSION_REAPED` and **exactly one session start**. Do not claim this gate passed or select production Rust init before exact-head CI plus independent AI re-review. I009b still owns real shutdown/reboot and product-grade device/rootfs work.

## Final I009a closure evidence — 2026-11-12

The bounded VM-only engineering slice was merged in [PR #14](https://github.com/wjhuang88/vita/pull/14). Exact reviewed code HEAD: `b6ead5d050e79bea7ce11221a1093882a872951e`; main squash merge: `40703ff942cd1eeb8bf051b62d9cbfcc97c9d60a`.

- **12/12 exact-head GitHub checks successful**, including 11 workflow/regression jobs and the separately executed Copilot code-review check. Copilot execution success is *not* a human approval or proof of zero risk: read its real PR reports. Independent reviewer findings were resolved with code/guest evidence and all inline threads were closed.
- [Final I009a guest run 43256546630](https://github.com/wjhuang88/vita/actions/runs/43256546630) passed 6 Rust unit tests and BIOS/UEFI **real Vita ISO** boot to a non-root controlling-TTY Talos session. Evidence markers include `VITA_I009A_ISO_GUEST_TALOS_OK mode=bios` and `mode=uefi`, `VITA_I009A_CTRL_C_GUEST_OK`, `VITA_I009A_CRASH_POLICY_VM_OK starts=3 crashes=3`, `VITA_I009A_SPAWN_FAILURE_REAPER_VM_OK`, `VITA_I009A_FATAL_BOOT_REAPER_VM_OK parent=440 reaped=[440, 441]`, `VITA_I009A_ACTIVE_SESSION_SHUTDOWN_VM_OK`, `VITA_I009A_MINIMAL_INIT_LIFECYCLE_VM_OK`.
- The guest also checks `VITA_I009_ROOTFS_OWNER_OK` for root-owned, non-group/world-writable critical image inodes. Tests cover adopted grandchild reaping, error/fatal PID1 fallback, Ctrl+C handling and SIGTERM shutdown of a running Talos session without respawn. Old I001/I005/I006/I007/I008 regression checks all passed on the same head.
- In independent Copilot review rounds, defects in fatal-path reaping, Ctrl+C, orphan proof race, child-PID identity, shutdown race, initramfs ownership and socket-marker framing were addressed. Latest reviewer noted risk of high-privilege PID1 code and the need for *separate human product/security examination*.
- **No product acceptance:** [I009a manual checklist](../validation/I009a-HUMAN-ACCEPTANCE.md) remains unchecked. Final Rust PID1 vs Dinit/Finit choice, full immutable rootfs/device services, real poweroff/reboot, physical USB, firmware, GPL/SBOM, broker and authenticated recovery remain I009b/I009c/I009d work. Do not close Story C or Issue #12.
