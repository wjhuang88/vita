# VITA-002-K — 无桌面中文输入：纯 Rust 拼音 + CJK 控制终端

Type: Spike / staged engineering feasibility. Priority: P0.
Status: **Active Stage 1; Stage 2/3 pending**.
Owner: [#40](https://github.com/wjhuang88/vita/issues/40);
parent: [VITA-002](VITA-002-os-interaction-settings.md);
related general terminal capability [VITA-002-A #30](VITA-002-A-terminal-spike.md).
Iteration: [I011](../../iterations/I011-chinese-ime-tui-spike.md).

## Approved design direction, not a shipping selection (2026-11-21)

Owner approved advancing a **no X11/Wayland, no desktop** approach:
Vita's non-root Rust TUI owns IME composition and passes *committed*
Unicode text to the local editor; Kmscon (DRM/KMS) is the primary
candidate for physical console CJK rendering; FbTerm + Fcitx5/Rime
is a reference control, not the mandatory product stack. Do not claim
Kmscon/Alpine/musl/real GPU verification before measured guest evidence.
The existing Rust/Talos SDK OS interaction boundary still applies;
never launch Talos CLI inside bash as the primary product.

### Delivery and dependency gates

1. **Stage 1, executable PTY proof (in progress).** Small independent
   Rust inputx-pinyin 1.4.0/rust 1.99 implementation; mode/preedit/
   candidates/selection/paging/backspace, direct host-IME Unicode not
   double-converted, bounded buffers, **Ctrl+D/F4 submit only** (Enter
   cannot submit, even on auto-repeat), and no model/privileged actions.
   Live bracketed-paste decoding deliberately disabled due upstream
   unbounded allocation; bounded streaming paste remains a product gap.
   Actual
   host PTY runtime, resize and terminal recovery must be checked in CI.
   This stage does not close #30; it supplies IME-specific evidence.
2. **Stage 2, Linux native console proof (not started).** Disposable
   QEMU BIOS + OVMF UEFI run actual DRM/KMS Kmscon+fonts+the same TUI
   with direct guest keyboard event. Measure Alpine v3.24
   packaging/musl, seat/device access, PTY/SID/foreground group and
   fallback ASCII console, compare FbTerm/Fcitx5, quantify
   image bytes/RSS/idle CPU/input p95. **A host PTY or serial screenshot
   is insufficient.** No host disks/USB write.
3. **Stage 3, choice/integration (not started).** Publish ADR decision
   based on VM pass/fail, dependency, maintenance, license and
   operator experience; only then integrate IME into VITA-002-B's
   native Talos SDK UI. If inputx quality is insufficient, evaluate
   librime; preserve no-X/Wayland target unless evidence forces a
   separately reviewed design change. Product physical-media/security/
   license/release approval remains human pending.

## Acceptance checklist

- [ ] Stage 1 code + exact-head unit/PTY/musl checks pass.
- [ ] Independent AI review of the exact code head and all actionable
  feedback resolved; no author self-review approval.
- [ ] Stage 2 both firmware guests display actual Unicode CJK locally
  without desktop and safely recover on terminal/IME failure.
- [ ] File license/SBOM/ABI/footprint and source dependency tradeoffs.
- [ ] Stage 3 ADR, owner acceptance and native Vita OS interaction path.
- [ ] Real physical console/hardware security/human release gates.

## Explicit exclusions

No keyboard/input interception for other applications, root shell,
secret input, model execution, unsigned ISO changes, third-party
driver installation on a user's device, automatic learned dictionary
persistence or privileged UI. Do not update status to Complete merely
because a standalone host PTY demo works.

Progress and resumed-task checkpoint: [#40](https://github.com/wjhuang88/vita/issues/40)
and [I011](../../iterations/I011-chinese-ime-tui-spike.md).
