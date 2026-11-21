# Operating Board

> Derived operating view only. Backlog and iteration owner documents define truth.

| Item | State | Owner Doc | Gate |
| --- | --- | --- | --- |
| VITA-001 | Now / Active | `backlog/active/VITA-001-agent-native-usb-os.md` | Advance executable child slices without bypassing their own readiness gates |
| VITA-001-B | Blocked | `backlog/active/VITA-001-B-iso-boot.md` | Issue #5: Alpine direct ISO works; Ventoy BIOS/UEFI guest handoff fails |
| VITA-001-D | Now / Active | `backlog/active/VITA-001-D-talos-interaction-runtime.md` | I006 guest Talos console proven in QEMU; model/config, TTY and recovery open |
| VITA-001-C | Now / Active | `backlog/active/VITA-001-C-live-os.md` | I008 through I009b2b VM slices merged (I009b2b 20/20 exact-head checks; 12 BIOS/UEFI boots; same-runner source benchmark recorded); Rust/Dinit/Finit selection (#25), post-prompt RAM, full Linux services and separate Vita-system P3 versus user ISO P1 boot/layout gate #27 OPEN; human hardware/security/release gates OPEN |
| VITA-001-G | Later / Refinement | `backlog/active/VITA-001-G-recovery-shell.md` | I007 VM-only precursor verified; physical recovery/security approval not available |

| VITA-001-E | Now / Active | `backlog/active/VITA-001-E-usb-config.md` | I010 BIOS real-model/tool trial passes; UEFI online error unresolved; transport policy security gate OPEN |

| VITA-001-D1 | Complete (research) | `backlog/active/VITA-001-D1-os-interface-research.md` | Research handed to VITA-002 child development records |

| [VITA-002 #29](https://github.com/wjhuang88/vita/issues/29) | Next / Refinement | `backlog/active/VITA-002-os-interaction-settings.md` | A/C/F Ready; B/D/E/G/H/I/J depend on earlier validation; no iteration started |
| [VITA-002-K #40](https://github.com/wjhuang88/vita/issues/40) | Now / Active Stage 1 | `backlog/active/VITA-002-K-chinese-ime.md` | I011: isolated Rust Pinyin + host PTY prototype in review/CI; Kmscon local CJK BIOS/UEFI proof, production integration and human release approval OPEN |
