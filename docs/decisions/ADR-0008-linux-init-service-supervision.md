# ADR-0008 — Vita Linux init and service supervision selection

Status: **Proposed, selected engineering baseline for I009; not verified for release**
Date: 2026-11-07
Story: VITA-001-C; follow-up [I009 Issue #12](https://github.com/wjhuang88/vita/issues/12).
Dependencies: ADR-0001, ADR-0002, ADR-0005, ADR-0006, ADR-0007; first firmware ISO proof I008 PR #11.

## Context

Vita is an agent-native portable Live Linux OS. Its differentiator is a native **Rust Vita/Talos SDK interaction session**, typed capability broker, configuration and audit—not reimplementing a kernel, init, service manager, hardware-event daemon or root shell.

**Known facts:** I005-I007 boot the real Alpine virt 3.24.3 kernel with a disposable static Rust PID 1; I006/I007 mount /proc, /sys, /run and /tmp and run genuine Talos SDK Tool turns. The Rust guest bootstrap explicitly lacks production child reaping, system shutdown, supervision and correct interactive controlling TTY. I008 source SHA `11ba5b08...` passed generated-ISO BIOS and OVMF UEFI boot [CI #42180463023](https://github.com/wjhuang88/vita/actions/runs/42180463023). The fixture has no complete Alpine rootfs and is not a release.

**Unverified:** full rootfs startup, source-redistribution compliance, actual TTY ownership, module/hotplug coverage, physical media, model configuration, authenticated recovery and production privilege separation. Rust-first under ADR-0005 explicitly **does not mandate Rust PID 1**.

## Constraints and evaluation

Hard: default native Talos session (not shell-launched Talos CLI); no implicit model-to-root commands; mature PID 1 lifecycle/reaping/shutdown; supervised and rate-limited noninteractive services; correct terminal login ownership; unprivileged session plus broker-controlled privileged work; independently tested BIOS/UEFI guest; GPL/source review before publication; no physical disk writes from CI.

Soft: Alpine/musl package/ecosystem reuse, small/observable boot, minimal runtime dependencies, deterministic QEMU reproducibility, future testable Rust capability interface.

| Candidate | PID 1 and service management | Benefits | Risks / Vita fit |
| --- | --- | --- | --- |
| **BusyBox init + OpenRC supervise-daemon** | BusyBox PID 1, OpenRC dependencies and runlevels, optional supervised noninteractive foreground daemons | Alpine's established default integration, minimal additional platform work; practical stage-by-stage replacement | Two components; OpenRC uses shell scripts for boot; supervision must be configured per service. **SELECTED** |
| **Dinit** | Native PID 1, dependency-aware supervisor, bounded service restarts | Clean unified model; Apache-2.0; simple service descriptions | Nondefault Alpine init; must provide own boot, device and shutdown integration. **Primary challenger** |
| **s6-linux-init + s6-rc** | s6-svscan PID 1, daemon supervision, oneshot/longrun dependency state | Excellent supervision, readiness and ISC license | More packages, generator/compiled service graph, higher integration burden |
| **runit** | PID 1 with three lifecycle stages, runsv supervision | Simple, small, BSD-style license | Dependency ordering needs script coordination; fewer built-in platform services |
| **openrc-init** | OpenRC also provides alternate Linux PID 1 | Can consolidate chosen service semantics | Not Alpine's typical BusyBox-PID1 setup; no evidence yet to justify switch |
| **systemd** | Comprehensive PID 1/activation/desktop/service system | Rich ecosystem and dependency/health controls | Large departure from minimal Alpine/musl path and current engineering scope |
| **Custom Rust PID 1** | New process reaper, signal, supervisor, startup/shutdown, device and TTY work | Maximum local ownership | High safety and maintenance risk, no differentiating product benefit. **REJECTED** |

This matrix is a qualitative architecture assessment. **No measured candidate-size, speed or memory benchmark exists yet**; do not present one as fact.

## Decision (I009 target)

**Select Alpine Linux BusyBox `/sbin/init` as permanent PID 1 plus OpenRC as the service/runlevel manager.** Use OpenRC `supervise-daemon` only for **noninteractive** foreground services, initially a dummy privileged-broker service; configure crash limits, start/stop ordering and health checks.

- Kernel -> temporary early `/init` (prefer audited Alpine mkinitfs or equivalent narrowly scoped bootstrap) -> discover and mount verified Live rootfs -> `exec switch_root` into **BusyBox `/sbin/init` as PID 1**. First I009 VM smoke may use an initramfs-contained Alpine root without switch_root but must identify this restriction; neither design leaves the current Rust proof program as the permanent init.
- Alpine OpenRC handles `sysinit`/`boot`/`default` services and lifecycle. Pin all packages, scripts and architecture-specific binaries in the build manifest, with verified APK/ELF hashes and a source/license bill of materials.
- Adopt Alpine **mdev** for baseline device nodes/uevents and required kernel modules/firmware. Evaluate **eudev** if specific tested hardware or desktop requirements demand it; neither choice is the PID 1 service manager.
- Preserve the **native Rust Vita/Talos SDK host** as the default interactive program, but **do not put the foreground terminal session under OpenRC supervise-daemon**. A supervised daemon may detach its controlling TTY. Spawn a dedicated terminal login through BusyBox `inittab` + an audited getty/agetty, initially on QEMU `ttyS0`, and execute the native Vita session as the non-root `vita` login program (not through an interactive shell). Add `tty1` in a later physical/graphics slice after tests. Prove controlling terminal, session leadership, signals, foreground input and clean respawn. Do not autologin root; unprivileged local autologin is **VM-only pending physical security review**.
- Use a separately launched service for the future privileged broker, allowlisted IPC and audit. Do not allow arbitrary LLM-supplied command strings, service names or raw shell. Native Talos capability providers are Rust-first, as before.
- Offline recovery remains an **independent boot and authorization policy**, not a generic fallback from a failed Talos session. I007 `vita.recovery=1` root BusyBox shell is explicitly compile-time gated and **not packaged in I008**, and remains non-shipping without human authentication and legal review.
- Boot scripting provided by Alpine/OpenRC is a **Linux platform exception**, not a scripting runtime/command pipeline in Vita's ordinary agent interaction path. Production interaction does not need Python, Node, JVM or shell invocation.
- Do not treat `openrc-init` and OpenRC service management as synonymous: this decision keeps **BusyBox as PID 1**; OpenRC's optional alternate PID 1 is not required.

### Intended boundary

```
BIOS/UEFI -> GRUB -> Linux kernel -> early /init [rootfs discovery]
                                           -> switch_root /sbin/init [BusyBox PID 1]
                                              +-> OpenRC [boot + dependencies]
                                              |     +-> mdev / networking / logs
                                              |     +-> supervise-daemon -> noninteractive broker
                                              +-> inittab getty [ttyS0 / tty1]
                                                    -> non-root vita login
                                                    -> native Rust Vita/Talos session [controlling TTY]
                                                         -> typed capabilities -> broker IPC
                                              +-> separately authorized offline recovery
```

No automatic privileged shell is in the normal UI path.

## Executable gate / next slice

[Issue #12](https://github.com/wjhuang88/vita/issues/12) records the following proof, which **must not be represented as completed**:

1. Pinned Alpine rootfs and all executable/shared-library inventory; ISO/rootfs size, boot time, memory, license/source plan. Explicit BusyBox GPL and OpenRC BSD-2-Clause review before redistribution.
2. BIOS **and** OVMF UEFI boot through the **actual generated ISO**; observe `/proc/1/comm`, child reaping and permanent PID 1 identity, and runlevels. If rootfs handoff is in scope, test the real `switch_root`.
3. Controlled non-root TTY login and a native Talos Tool session with input, errors and interactive termination; active controlling TTY validated rather than just `/dev/ttyS0` file handles.
4. Supervised noninteractive service crashes and recovery (bounded restart/backoff), dependency ordering, unclean session exit and safe shutdown/reboot. Test mount, missing binary, broker crash and no-network/model paths.
5. Preserve I001-I008 regressions, feature-disabled root shell default, real BIOS/UEFI guest logs and disposable-disk-only CI; do not close VITA-001-C or Ventoy Issue #5.

Security/licensing/physical-device, firmware and Secure Boot acceptance remain **PENDING**. No distribution license/source clearance is asserted by this ADR.

## Reversal

Run a bounded Dinit challenger PoC only if the selected stack shows measured unacceptable footprint, poor supervised recovery, incorrect TTY behavior, distro service incompatibility, or unsatisfactory security/maintenance overhead. Select Dinit only after documented repeatable VM comparison and a superseding ADR; do not select purely for code-language preference.

## Verified upstream references (2026-11-07)

- Alpine default musl/BusyBox/OpenRC: https://www.alpinelinux.org/about/
- Alpine OpenRC: https://docs.alpinelinux.org/user-handbook/0.1a/Working/openrc.html
- OpenRC service supervision: https://github.com/OpenRC/openrc/blob/master/supervise-daemon-guide.md
- Alpine getty TTY/autologin and physical security warning: https://wiki.alpinelinux.org/wiki/TTY_Autologin
- Alpine serial login: https://wiki.alpinelinux.org/wiki/Enable_Serial_Console_on_Boot
- Alpine mdev/eudev: https://wiki.alpinelinux.org/wiki/Device_Manager
- Alpine mkinitfs: https://github.com/alpinelinux/mkinitfs
- Linux initramfs semantics: https://www.kernel.org/doc/html/latest/filesystems/ramfs-rootfs-initramfs.html
- BusyBox inittab actions: https://lists.busybox.net/pipermail/busybox-cvs/2019-November/039509.html
- Dinit (Apache-2.0): https://davmac.org/projects/dinit/
- s6-linux-init / s6-rc (ISC): https://www.skarnet.org/software/s6-linux-init/ and https://www.skarnet.org/software/s6-rc/
- runit: https://smarden.org/runit/
- systemd: https://freedesktop.org/software/systemd/man/latest/init.html
