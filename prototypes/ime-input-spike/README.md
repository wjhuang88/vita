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

Host-IME committed Unicode characters are treated as direct text,
**not re-interpreted as pinyin**. This standalone TUI **does not call
Crossterm's input event parser at all**: even without requesting bracketed
paste, an unsolicited `ESC[200~` can trigger its default-enabled unbounded
paste parser and swallow Ctrl+C. Instead, a fixed 512-byte raw-input
reader and a bounded, backpressured 16-slot channel feed a small
stream decoder with capped UTF-8/escape state. Complete or incomplete
bracketed-paste sequences are **discarded without committing text**;
Ctrl+C is always recognized. This is deliberately NOT support for arbitrary
bracketed-paste text. The library's bounded `Input::Paste` method remains
test-only and is not wired into the PTY path. Full safe streaming paste
is a future product requirement. The editor remains bounded to
32 ASCII pinyin letters per composition, five candidates/page
and 8 KiB committed text.

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
a bounded resize, Ctrl+C, a distinct SIGTERM process shutdown,
and an unsolicited unfinished bracketed-paste stream plus Ctrl+C.
It checks reconstructed terminal states, Unicode glyphs, terminal
restoration and paste discard, then compiles a Rust
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
