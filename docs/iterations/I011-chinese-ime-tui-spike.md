# I011 — Offline Chinese IME and pure TUI feasibility

Status: **Active (2026-11-21)** — Stage 1 code VM-independent PTY and
musl CI proof passed at pinned candidate head; **independent final-head
re-review / PR merge OPEN**. Stages 2/3 unstarted.
Owner: [VITA-002-K](../backlog/active/VITA-002-K-chinese-ime.md) /
[#40](https://github.com/wjhuang88/vita/issues/40).
Related but independent: [VITA-002-A #30](https://github.com/wjhuang88/vita/issues/30).

## Immutable intake / user-authorized outcome

The owner approved advancing the *candidate* native Rust TUI +
`inputx-pinyin` + Kmscon architecture, **not preapproving** Kmscon as a
final display layer, physical hardware claims or any root/USB writes.
Baseline source main: `966f40df8d9cb34094b6063ea2ceecf66aefe905`.
Development branch: `feat/vita-002-k-ime-spike`.

### Constraints

- **Hard:** no X11/Wayland/desktop or external online model for IME.
  Input remains inside the unprivileged Vita session, outside PID1,
  Broker, privileged confirmations, secrets and recovery.
- **Hard:** IME selection and the *separate* text-submit action are
  different key gestures. Standard Enter **never submits** (auto-repeat
  cannot be safely distinguished on legacy PTY); Ctrl+D/F4 does. Normal
  host-IME committed Unicode bypasses Pinyin; bracketed-paste parsing is
  disabled pending a bounded streaming decoder. ASCII fallback and
  signal-safe terminal restoration are required.
- **Hard:** current rootfs/boot chain and QEMU ISO fixtures unchanged,
  no physical device writes, no automatic learning persistence.
- **Soft:** prefer lightweight Rust native crate and reusable Ratatui
  interface; measure rather than assume CJK font/compositor cost.
- **Assumptions to test:** `inputx-pinyin 1.4.0` compiles in pinned
  Rust/musl; candidate quality sufficient; Kmscon has suitable
  Alpine 3.24 packaging, DRM/seat/PTY and font support.

## Stage 1 work selected for this PR

- Separate published runnable `prototypes/ime-input-spike` whose
  Rust input state machine has candidate selection, paging, mode
  changes, raw composition cancellation, input bounds, grapheme
  deletion, direct Unicode character editing and library-side test-only
  bounded paste/control/bidi stripping; no live bracketed-paste claims.
- Host-only terminal frontend under crossterm/ratatui; no Talos SDK,
  no secrets, network, external IME service, host disk/USB operations.
- Enter can confirm a candidate but **never submits** a line; only an
  explicit Ctrl+D/F4 commits prepared text to a **demo-only** list.
  Repeated Enter cannot invoke Tool or Agent actions.
- Focused tests, self-test and Python real PTY smoke with Ctrl+C and
  termios restore, plus fixed 1.99.0 Rust and musl compilation in
  `.github/workflows/i011-chinese-ime-tui.yml`.
- Readonly source research: [inputx public API](https://docs.rs/inputx-pinyin/1.4.0/inputx_pinyin/);
  [session commit and L0 semantics](https://docs.rs/inputx-pinyin/1.4.0/inputx_pinyin/session/struct.Session.html);
  [Kmscon upstream](https://github.com/kmscon/kmscon).
  Dictionary lineage and dependencies are not yet approved for release.

## Review-driven corrections (2026-11-21)

Independent reviewer [PR #41](https://github.com/wjhuang88/vita/pull/41)
identified six actionable failures on an earlier head: page-offset
selection, SIGTERM terminal restoration, wrapped text cursor, legacy
Enter repeat, unbounded upstream bracketed-paste buffering, and PTY
test observability. They are real merge blockers.

The updated branch fixes page-relative Space/Enter, requires explicit
Ctrl+D/F4 for submit, adds cursor-following Unicode viewport and
catchable termination flags, **disables live bracketed paste** instead
of using an unbounded decoder, and exercises real PTY state transitions,
resize, Ctrl+C and SIGTERM. The code and PTY tests passed on the pinned
candidate HEAD shown below; final documentation HEAD / independent
re-review must still be separately validated. Real Kmscon
rendering/keyboard/boot remains a separate [Stage-2 #42](https://github.com/wjhuang88/vita/issues/42).

## Stage 1 validation log

**Stage 1 verified candidate code HEAD (2026-11-21):**
`2ef7bb0fbcc56dfaf0d5206b933e8532ae4a05d8`;
[I011 run 44887334764](https://github.com/wjhuang88/vita/actions/runs/44887334764)
**2/2 successful jobs** (governance and ime-pty), using Rust **1.99.0**:

- `cargo fmt --check`, `cargo clippy --all-targets -D warnings`,
  10 input/editor library tests, 2 terminal-viewport tests: **PASS**.
- `--self-test`:
  `VITA_IME_SELFTEST_OK pinyin=zhongguo commit=中国 submit=explicit`.
- **Actual nonroot PTY** TUI with a minimal ANSI screen reconstruction:
  `VITA_IME_PTY_OK ime_commit explicit_submit resize ctrl_c termios`.
  Confirms screen-visible Pinyin candidate, separate Enter confirmation
  from Ctrl+D submission, displayed submitted count, resize redraw,
  actual Ctrl+C key-byte exit and terminal attribute restore.
- **Independent signal path**:
  `VITA_IME_SIGNAL_OK sigterm termios_restore`. It is not silently
  replaced by a SIGINT signal injection.
- `x86_64-unknown-linux-musl --release` builds a **static-pie** ELF
  successfully; binary SHA-256:
  `6ad7f37e1afc6fa4a643048b824d9ee45e9150c9442a00124cc2df61d3e7e568`.
  This is a compile/host-PTY proof, **not** a Kmscon/guest/native display
  or a product Alpine image validation.
- [I011 CI provenance artifact #14699903691](https://github.com/wjhuang88/vita/actions/runs/44887334764/artifacts/14699903691)
  includes dependency tree, build metadata and release SHA/size.
- Copilot review on earlier `f08d10e3` surfaced six substantive
  findings. Changes described above address them, but **final HEAD
  fresh review and resolved thread verification remain mandatory**.
  Do not treat the earlier review as approval.

Later documentation/PR-head commits must re-run exact-HEAD CI;
the candidate result does not automatically validate a changed SHA.

Manual/physical Chinese input validation: **NOT DONE**.
BIOS/UEFI guest native DRM/KMS display: **NOT DONE**.
Kmscon packaging/seat/cjk-font/driver footprint: **NOT MEASURED**.
FbTerm/Fcitx5 comparator: **NOT IMPLEMENTED**.
Production license/SBOM, firmware, signing and physical-media gate:
**HUMAN PENDING**.

## Stage 2/3 stop conditions and follow-up

Do not claim full #40 or VITA-002-A/B completion until Kmscon native
BIOS + UEFI guest results, terminal failure recovery, measured
memory/image/latency and independent review are retained. No changing
production boot/init defaults in this iteration. If Alpine ABI or
product display compatibility is unsupported, preserve failure logs
and issue evidence, compare the FbTerm baseline, and trigger a
documented technical choice instead of silently adding Wayland.

## Durable resume checkpoint

For continuation after PR creation, use [Issue #40](https://github.com/wjhuang88/vita/issues/40),
[PR #41](https://github.com/wjhuang88/vita/pull/41), its exact HEAD checks/reviewer
comments, this iteration log and the runnable
[prototype README](../../prototypes/ime-input-spike/README.md).
