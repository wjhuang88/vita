# I011 — Offline Chinese IME and pure TUI feasibility

Status: **Active (2026-11-21)** — Stage 1 branch implementation;
no passed exact-head CI or independent AI review claimed yet.
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
  different key events. Host-IME-completed Unicode paste is not fed
  back into Pinyin; ASCII fallback and safe terminal restoration work.
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
  deletion, Unicode paste and ANSI/control/bidi stripping.
- Host-only terminal frontend under crossterm/ratatui; no Talos SDK,
  no secrets, network, external IME service, host disk/USB operations.
- The standard Enter key confirms a candidate and only a later Enter
  submits a prepared buffer to a **demo-only** list; the UI does not
  send Tool or Agent actions.
- Focused tests, self-test and Python real PTY smoke with Ctrl+C and
  termios restore, plus fixed 1.99.0 Rust and musl compilation in
  `.github/workflows/i011-chinese-ime-tui.yml`.
- Readonly source research: [inputx public API](https://docs.rs/inputx-pinyin/1.4.0/inputx_pinyin/);
  [session commit and L0 semantics](https://docs.rs/inputx-pinyin/1.4.0/inputx_pinyin/session/struct.Session.html);
  [Kmscon upstream](https://github.com/kmscon/kmscon).
  Dictionary lineage and dependencies are not yet approved for release.

## Stage 1 validation log

**Not yet executed on GitHub at baseline authoring:** source is
published on a PR branch to trigger exact-code-head compilation,
unit tests, PTY smoke and musl target checks. CI pass/error and
reviewer findings must be appended here rather than guessed.

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
the new PR (link to be recorded), its exact HEAD checks/reviewer
comments, this iteration log and the runnable
[prototype README](../../prototypes/ime-input-spike/README.md).
