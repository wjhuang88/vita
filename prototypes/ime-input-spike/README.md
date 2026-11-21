# I011 — Offline Chinese IME / PTY Rust spike

**Experimental component only.** This is not the Vita Live OS UI, not a
production input method, and not a replacement for Linux's physical console.
Tracking: [VITA-002-K #40](https://github.com/wjhuang88/vita/issues/40)
and [I011](../../docs/iterations/I011-chinese-ime-tui-spike.md).

The code proves whether the independent Rust `inputx-pinyin` engine can
feed a single-user, non-root Rust TUI without an external IME daemon,
without Talos/model access, and without committing a composing candidate
as an Agent command. It does **not** render fonts itself or configure DRM/KMS.

## Run from a Unicode-capable host terminal

Requires Rust **1.99.0** and a PTY terminal supporting UTF-8 and key events.

```sh
cargo +1.99.0 test --manifest-path prototypes/ime-input-spike/Cargo.toml
cargo +1.99.0 run --manifest-path prototypes/ime-input-spike/Cargo.toml
cargo +1.99.0 run --manifest-path prototypes/ime-input-spike/Cargo.toml -- --self-test
```

The session starts in Chinese mode. Enter `zhongguo` to see candidates;
choose using **1–5**, **Space** or **Enter**. **Enter never submits**;
press **Ctrl+D** or **F4** to submit committed text to the *local*
demonstration panel. This distinct gesture is intentional: legacy PTYs
cannot reliably distinguish a second key press from Enter auto-repeat.
**F2** or **Ctrl+Space** (when the terminal sends it) toggles English mode;
**Tab/PageDown** and **Shift+Tab/PageUp** navigate candidate pages,
**Esc** cancels preedit, **Backspace** deletes preedit or a whole Unicode
grapheme, and **Ctrl+C** exits and restores terminal state.

Host-IME committed Unicode characters are handled as direct text input,
**not re-interpreted as pinyin**. **Bracketed paste is intentionally
DISABLED in this PTY prototype:** crossterm's upstream Event::Paste parser
can buffer arbitrarily large/incomplete payloads before an application
callback. Do not claim large-paste acceptance; a bounded streaming decoder
is a follow-up product requirement. The library's explicit `Input::Paste`
method is separately unit-tested for control/bidi filtering, newline
normalization, and a 4,096-character event cap, but is not wired to the
PTY parser. The editor is bounded to 32 ASCII pinyin letters per
composition, five candidates/page and 8 KiB committed text.

**Security:** no network/model, shell calls, file writes, credential
inputs, logs, or persistent learning. The engine's in-memory L0 ranking
may change on candidate selection and is destroyed on exit. **Never**
reuse this prototype's ordinary text editor for credentials, privileged
approval, terminal recovery or system-wide keyboard interception.

## Reproducible engineering checks

```sh
python3 scripts/ci/test_i011_ime_pty.py \
  prototypes/ime-input-spike/target/debug/vita-ime-input-spike
```

The CI uses real PTY key input with staged Enter-confirm and Ctrl+D-submit,
a bounded resize, Ctrl+C and a distinct SIGTERM process shutdown;
it checks changed submitted state, UTF-8 glyph output and termios
restoration, then compiles a Rust
`x86_64-unknown-linux-musl` binary. A passing host PTY check **does not**
prove a Linux kernel VT or Kmscon physical console.

Kmscon/DRM and a genuinely independent ASCII recovery path are **Stage 2**
and must be boot-tested with real BIOS+UEFI QEMU guests, Alpine/musl,
font licenses, GPU/seat access, PTY session ownership and crash cases.
It is a hypothesis, not a selected shipped OS component.

## Dependencies and distribution policy

Pinned for this experiment: `inputx-pinyin =1.4.0`, `ratatui =0.30.0`,
`crossterm =0.29.0`, `unicode-segmentation =1.12.0`, and
`unicode-width =0.2.0`.
`inputx-pinyin` source is MIT OR Apache-2.0, but its embedded dictionary
has a **separate upstream source/attribution chain** (Unihan, pypinyin,
jieba, Leipzig, SUBTLEX). The entire resolved dependency/license and
dictionary data audit must be completed before embedding it into a
distribution image. This experiment only records a dependency tree and
compilation evidence; it does not license or approve a Vita release.
See [source](https://github.com/goliajp/inputx) and
[crate docs](https://docs.rs/inputx-pinyin/1.4.0/inputx_pinyin/).
