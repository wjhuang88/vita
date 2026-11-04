# Operating Board

> Derived operating view only. Backlog and iteration owner documents define truth.

| Item | State | Owner Doc | Gate |
| --- | --- | --- | --- |
| VITA-001 | Now / Active | `backlog/active/VITA-001-agent-native-usb-os.md` | Advance executable child slices without bypassing their own readiness gates |
| VITA-001-B | Blocked | `backlog/active/VITA-001-B-iso-boot.md` | Issue #5: Alpine direct ISO works; Ventoy BIOS/UEFI guest handoff fails |
| VITA-001-D | Now / Active | `backlog/active/VITA-001-D-talos-interaction-runtime.md` | I004 host console Complete; TTY/PTY human checks and Live default session pending |
| VITA-001-C | Now / Active | `backlog/active/VITA-001-C-live-os.md` | I005 real Linux/Rust init smoke; full rootfs + Talos + UEFI still open |
| I005 | Now / Active | `iterations/I005-linux-rust-boot.md` | CI must observe real QEMU guest PID 1 serial marker |
