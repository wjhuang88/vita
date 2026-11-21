# I011 — Offline Chinese IME / PTY Rust spike

**Experimental component only.** This is not the Vita Live OS UI, not a
production input method, and not a replacement for Linux's physical console.
Tracking: [VITA-002-K #40](https://github.com/wjhuang88/vita/issues/40)
and [I011](../../docs/iterations/I011-chinese-ime-tui-spike.md).

The code proves whether the independent Rust \`inputx-pinyin\` engine can
feed a single-user, non-root Rust TUI without an external IME daemon,
without Talos/model access, and without committing a composing candidate
as an Agent command. It does **not** render fonts itself or configure DRM/KMS.

## Run from a Unicode-capable host terminal

Requires Rust **1.99.0** and a PTY terminal supporting UTF-8 and key events.

\`\`\`sh
cargo +1.99.0 test --manifest-path prototypes/ime-input-spike/Cargo.toml
cargo +1.99.0 run --manifest-path prototypes/ime-input-spike/Cargo.toml
cargo +1.99.0 run --manifest-path prototypes/ime-input-spike/Cargo.toml -- --self-test
\`\`\`

The session starts in Chinese mode. Enter \`zhongguo\` to see candidates;
choose using **1–5**, **Space** or **Enter**, and press **Enter again**
to submit the committed text to the *local* demonstration panel.
**F2** or **Ctrl+Space** (when the terminal sends it) toggles English mode;
**Tab/PageDown** and **Shift+Tab/PageUp** navigate candidate pages,
**Esc** cancels preedit, **Backspace** deletes preedit or a whole Unicode
grapheme, and **Ctrl+C** exits and restores terminal state.

Paste is recognized through bracketed-paste events and treated as
**already-committed Unicode**, not re-interpreted as pinyin. Embedded
control/escape bytes and bidi overrides are removed from pasted editor
text; newlines/tabs become spaces. The editor is intentionally bounded:
32 ASCII pinyin letters in preedit, five candidates per page,
4,096 pasted characters per event and 8 KiB editor content.

**Security:** no network/model, shell calls, file writes, credential
inputs, logs, or persistent learning. The engine's in-memory L0 ranking
may change on candidate selection and is destroyed on exit. **Never**
reuse this prototype's ordinary text editor for credentials, privileged
approval, terminal recovery or system-wide keyboard interception.

## Reproducible engineering checks

\`\`\`sh
python3 scripts/ci/test_i011_ime_pty.py \
  prototypes/ime-input-spike/target/debug/vita-ime-input-spike
\`\`\`

The CI uses real PTY key input, a bounded resize and Ctrl+C, checks UTF-8
candidate bytes and termios restoration, then compiles a Rust
\`x86_64-unknown-linux-musl\` binary. A passing host PTY check **does not**
prove a Linux kernel VT or Kmscon physical console.

Kmscon/DRM and a genuinely independent ASCII recovery path are **Stage 2**
and must be boot-tested with real BIOS+UEFI QEMU guests, Alpine/musl,
font licenses, GPU/seat access, PTY session ownership and crash cases.
It is a hypothesis, not a selected shipped OS component.

## Dependencies and distribution policy

Pinned for this experiment: \`inputx-pinyin =1.4.0\`, \`ratatui =0.30.0\`,
\`crossterm =0.29.0\`, \`unicode-segmentation =1.12.0\`, and
\`unicode-width =0.2.0\`.
\`inputx-pinyin\` source is MIT OR Apache-2.0, but its embedded dictionary
has a **separate upstream source/attribution chain** (Unihan, pypinyin,
jieba, Leipzig, SUBTLEX). The entire resolved dependency/license and
dictionary data audit must be completed before embedding it into a
distribution image. This experiment only records a dependency tree and
compilation evidence; it does not license or approve a Vita release.
See [source](https://github.com/goliajp/inputx) and
[crate docs](https://docs.rs/inputx-pinyin/1.4.0/inputx_pinyin/).
