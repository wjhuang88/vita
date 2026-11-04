# I004 human acceptance handoff

**Status:** HUMAN VALIDATION PENDING
**Scope:** Vita host-side Rust console prototype. This is a handoff for a *different* human-led validation session; it does not claim a production Live OS or a complete Talos-powered system console.

## Automated evidence (no human action)

I004 PR #7 uses GitHub Actions to check parser/event-renderer unit tests, two successful Talos SDK read-only Tool turns separated by a provider error, clean exit, the original I001 SDK smoke, governance checks, and QEMU BIOS/UEFI boot-probe regression. Record the final successful run and head SHA after automation passes.

## HUMAN-001 — interactive console and cancellation

- [ ] **NEEDS HUMAN**: On a disposable or trusted Linux development machine with Rust 1.99.0, check out the reviewed Vita PR #7 commit.
- [ ] Run `cargo +1.99.0 run --manifest-path prototypes/interaction-runtime/Cargo.toml -- --console-fixture`.
- [ ] Enter `inspect` twice, each time verify `[tool] system_summary` and `[turn:ok]`, without duplicated output.
- [ ] Enter `demo:error`, verify `[turn:error]` and the next `inspect` still works; no secret/error internals printed.
- [ ] Enter `demo:slow`, press **Ctrl+C** within 15 seconds, verify cancellation terminates the active turn and the next `inspect` can succeed.
- [ ] Repeat `demo:slow`, type `:cancel` while it is active, verify cancellation and next `inspect`.
- [ ] Test `:help`, `:quit` and stdin EOF (Ctrl+D); successful graceful exit should include `VITA_CONSOLE_EXIT_OK`.
- [ ] Record observed results, terminal transcript and failure cases (redact all system identifiers, usernames and credentials).

**Pass criteria:** Ctrl+C and `:cancel` reliably interrupt without exiting the entire host console, a subsequent turn can run, a provider error is contained and exit is clean. If any fail, mark FAIL and open an issue with precise redacted reproducer.

## HUMAN-002 — TTY / PTY environment matrix

- [ ] **NEEDS HUMAN**: Repeat HUMAN-001 once in a physical Linux virtual console (TTY) and once in a terminal emulator (PTY); capture distro, kernel, TERM and observed differences.
- [ ] **NOT IMPLEMENTED / DEFERRED to VITA-001-C/D**: Verify that a real Vita Live system opens the Talos session as its native default interaction host, rather than invoking a Talos CLI from an interactive shell.
- [ ] **NOT IMPLEMENTED / DEFERRED to VITA-001-G**: Test the independently accessible recovery shell when the Vita session fails to start.

## HUMAN-003 — permissions and provider integration

- [ ] **NEEDS HUMAN REVIEW**: Inspect that only `system_summary` is exposed and it remains `ToolNature::Read`; no unrestricted model-to-root-shell path is enabled.
- [ ] **NOT IMPLEMENTED / DEFERRED to VITA-001-E/F**: Review live model/provider credentials, USB-local config, privileged broker approvals, target identity and audit/redaction once implemented.
- [ ] **NOT REQUIRED FOR I004**: Real USB media validation. Do not write physical media based on a host console fixture. Physical-target gate remains mandatory.

## HUMAN-004 — unrelated ISO compatibility blocker

- [ ] **BLOCKED / NOT AN I004 GATE**: Alpine virt real ISO fails to reach Linux via Ventoy in BIOS and UEFI. See [Issue #5](https://github.com/wjhuang88/vita/issues/5) and unmerged [Draft PR #4](https://github.com/wjhuang88/vita/pull/4). Handle in a separate validation workstream.

## Evidence form for the other session

- Operator / date:
- Exact commit SHA / CI URL:
- Linux distro, kernel, TERM and terminal program:
- HUMAN-001: PASS / FAIL / BLOCKED + redacted transcript:
- HUMAN-002: PASS / FAIL / DEFERRED + environment matrix:
- HUMAN-003: PASS / FAIL / DEFERRED + review findings:
- Issue / PR references for findings:
- Overall human verdict: **PENDING / PASS / FAIL / BLOCKED**

**Never mark an unchecked item as human-validated.** I004's automated host proof may be delivered independently; overall VITA-001-D acceptance requires Live OS integration and human checks.
