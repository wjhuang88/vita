# ADR-0008 — Vita Linux init and service supervision selection

Status: **Proposed — small-and-complete Linux + Rust-native control plane is the accepted product direction; permanent PID 1/OS stack selection remains under I009 evaluation; no release approval**
Date: 2026-11-07
Story: VITA-001-C; follow-up [I009 Issue #12](https://github.com/wjhuang88/vita/issues/12).
Dependencies: ADR-0001, ADR-0002, ADR-0005, ADR-0006, ADR-0007; first firmware ISO proof I008 PR #11.

## Current product direction — minimal, complete, Rust-owned (2026-11-07, latest clarification; authoritative)

**Hard constraints, confirmed by product owner:** Vita must remain **small and beautiful** as a portable Agent-native Linux. Linux capabilities must be functionally complete, but Vita need not bundle a large conventional distribution, desktop, shell-tool management stack or systemd. Native Rust implementation of bounded foundational capabilities is acceptable and desirable when it improves typed control, memory safety and product integration. *Do not equate a complete Linux capability surface with shipping a general-purpose distro.*

This latest clarification **supersedes** earlier same-day proposed Debian/systemd-first and full general-purpose-distro priority, without erasing historical deliberation. The Alpine/musl/minimal-initramfs kernel work (I005–I008) remains the starting foundation, not a discarded throwaway. Debian/systemd and NixOS are now **optional external-interface/reference benchmarks**, not I009 mandatory implementations or selection leaders.

### Preferred layering and ownership

```text
Firmware -> GRUB / Vita Live ISO -> Linux kernel + modules + firmware
  -> narrow early boot /init -> minimal read-only rootfs + bounded /run /tmp
    -> PID 1 (I009 selection: constrained Vita Rust init OR proven lightweight init)
       -> supervised noninteractive services: Vita Broker, hardware/network helpers
       -> TTY/session leader: non-root native Vita/Talos SDK (primary user UI)
       -> independent authorized offline recovery (NOT model shell, not default)
    -> Vita OS Core (Rust-first; separate from PID 1 if it improves survivability)
       -> typed provider interfaces: process, filesystem, storage, network, hardware,
          user/session, services, logs, update/package and diagnostics
       -> narrowly privileged broker; target validation, confirm, audit, progress, policy
    -> opt-in standard Linux applications / compatibility and extension components
```

A **minimal Linux** means an intentionally small shipped base with **complete extensible Linux capabilities**: normal POSIX/ELF compatibility, real drivers/modules, proper device/event management, networking/Wi-Fi, filesystems, user/TTY management, system calls, software updates and offline recovery. Some services can be modular or installed on demand; no claim of complete hardware coverage before testing.

### PID 1 shortlist, grounded in currently available upstreams

1. **Vita Rust minimal init prototype (valid candidate, not selected yet):** a *small, independent PID 1* that owns child reaping, signals, bounded service restart, orderly shutdown, explicit bootstrap handoff and nothing Agent-specific. Reuse stable Linux interfaces and maintained Rust crates (`nix`, `rustix`); keep richer service policies and the Talos/Agent loop outside PID 1. Do **not** simply keep the current sleeping Rust proof as a release init.
2. **Dinit or Finit (external proven lightweight comparison):** Dinit's integrated dependencies and service management or Finit's TTY/appliance support offer lower PID 1 implementation risk; evaluate actual Alpine/musl integration, terminal behavior, shutdown, rescue and binary dependency size.
3. **Alpine BusyBox init + OpenRC (integration control):** the known low-risk distro baseline, not a mandatory final architecture. Use it to benchmark how much safety and integration a custom init must match.
4. **Rust-native upstream exploratory references:** `threefoldtech/zinit` (Rust/Tokio, Unix socket and JSON-RPC, used in Zero-OS but project-specific); `rinit-org/rinit` (Rust, explicitly work-in-progress); `youhide/oxinit` (pre-alpha). These are sources to study, **not automatically trusted production dependencies**. Tiny container init `tini`/`dumb-init` alone cannot supply Live OS lifecycle.
5. **systemd/NixOS:** retain only as *capability benchmark or opt-in interoperability*, e.g., a well-defined system-interface behavior to emulate rather than a required service dependency. No blanket ban if a real feature needs it, but no unconditional whole-stack migration.

### What to implement natively vs reuse

| Capability | Vita Rust-owned | Mature external Linux building block |
| --- | --- | --- |
| Agent intent, tool schemas, session, policy, confirmation, audit, typed API and orchestration | **Yes** | Do not delegate authorization to the LLM or shell |
| Process/service state, supervised tasks, diagnostics, startup configuration | **Prefer** narrow Rust service interface/manager; optional dedicated Rust PID 1 | Linux PID/FD, signals/cgroups, libc/kernel primitives; comparison with Dinit/Finit |
| Process, mount, storage inventory and readonly system info | **Yes** typed `/proc`/`/sys`/netlink/udev monitoring | Kernel namespaces, filesystem and device interfaces; vetted library where required |
| Networking configuration/link/route state | **Yes** Rust netlink and typed policy adapters | Kernel rtnetlink; reuse proven DHCP/DNS, Wi-Fi (iwd/wpa_supplicant), firewall and crypto stack rather than rewrite protocols |
| Block mounting, filesystems, partitioning and hardware safety | **Yes** typed state and strict device identity/authorization, audited helpers | Kernel VFS and trusted fsck/mkfs/cryptsetup/blkid/libmount tools/libraries; no model-generated destructive commands |
| Device events/firmware/hotplug | **Yes** typed inventory and event routing | devtmpfs + mdev/eudev where needed, kernel modules and firmware |
| Terminal, account and privilege boundary | **Yes** session orchestration and an unprivileged Vita host | Kernel TTY/session/uid/gid/PAM/auth where appropriate; do not hand-write credentials/security primitives casually |
| Update, signed extension and recovery plan | **Yes** policy, version/state contracts, verified update orchestration | Stable package/image tooling and well-reviewed cryptographic implementations; offline authorized rescue |

**Reliability constraint:** self-implementation is *allowed*, not a purity requirement. An own PID 1 is a safety-critical independent process and must pass the same child-reaping, fault injection, user-tty, crash-loop, upgrade, graceful shutdown and recovery matrix as mature alternatives, without embedding Talos or an async Agent runtime inside PID 1 by default.

### Revised I009 near-term scope

- **A.** Preserve Alpine/musl + existing Vita kernel/ISO baseline; build a reproducible minimal rootfs, inspect shipped dependencies/licenses and modules, boot on BIOS and OVMF UEFI in QEMU (never write real USB).
- **B.** Compare **two genuine PID 1 implementations** under identical kernel, initramfs, CPU/firmware, terminal and service workload: a narrowly scoped Rust Vita init prototype vs Dinit (or Finit if it provides a concrete TTY integration advantage). Keep BusyBox/OpenRC as control evidence. Bench image size, idle RAM, boot time, static/dynamic dependencies and failure behavior, not just source LOC.
- **C.** Native non-root Vita/Talos SDK via real controlling TTY; a separately supervised broker fixture with explicit crash recovery; background processes, zombie reaping, dependency ordering, shutdown, bad mount, bad recovery, unavailable model/network.
- **D.** Implement *only two read-only Rust-native provider proof points* first: service/process status and network link/device inventory (using stable kernel APIs, no human CLI output scraping). Then one typed, **allowlisted, confirmed and audited** non-destructive service action through broker; keep disk/network writes disabled.
- **E.** Finalize selected PID 1 only after measured results and independent review; record decision + reversal criteria, preserve all previous I001–I008 CI, Ventoy Issue #5 blocker and manual licensing/security/physical gates. Do not build Debian/NixOS in I009 solely for comparison.

### Small-and-beautiful design tests

- Minimize **shipped** binaries/processes, global mutable state and long-lived background daemons, **not** safety checks or supported Linux behaviors.
- Maintain one typed Rust OS capability model across CLI-less local Agent UI, optional graphical/remote UI and future providers.
- Keep PID 1 and the privileged broker independently survivable from Talos/model crashes.
- Support optional packages/standard Linux program execution without making Bash the default UI.
- Every feature must specify its capability boundary, dependencies, permission context, observable behavior, failure/recovery and evidence.
- Measure image size, boot speed and RAM; **do not invent numeric budget before baseline measurements**.

### Newly considered reference projects

- Rust/Tokio PID 1 `zinit`, existing JSON-RPC control: https://github.com/threefoldtech/zinit
- Rust `rinit` (upstream work-in-progress): https://github.com/rinit-org/rinit
- Rust `oxinit` (upstream pre-alpha): https://github.com/youhide/oxinit
- Lightweight `nitro` supervisor (0BSD, statically linked musl possible): https://github.com/leahneukirchen/nitro
- Alpine/musl base rationale: https://www.alpinelinux.org/about/
- Rust network link/address/route APIs: https://docs.rs/rtnetlink/latest/rtnetlink/
- Dinit/Finit earlier comparison and licensing remains in historical sections below.

## Earlier product-goal correction (historical, further refined above)

**Hard product intent:** Vita is a functionally complete Linux OS (drivers, networking, storage, users/sessions, packages, services, security, recoverability) with **native Agent-first interaction** as the primary way humans operate it. The differentiator is *intent-to-governed-system-capability*, not the smallest possible PID 1 or an agent CLI running in a terminal. The earlier Alpine BusyBox+OpenRC recommendation was based on a narrower minimum-Linux-appliance assumption and is now a **historical low-overhead baseline, not the selected product architecture**. I005-I008 remain valid experimental proofs; do not change their previously accepted scope or claim a distro migration has happened.

The full-agent-operation target affects ADR-0006's **soft** Alpine base choice. That ADR remains the factual record for the I005/I008 experiments but **does not constrain the final base to musl/Alpine**. Before implementation of a new product foundation, test glibc/systemd alternatives in disposable VM/ISO, compare named outcomes, and record a subsequent accepted ADR. Change class: *decision reopened and comparative scope increased*; existing iteration/CI baseline remains intact.

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

## Previous proposed decision (I009 target — reopened; NOT final)

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


## Historical change-control addendum: earlier proposed systemd/full-distro path (superseded by the latest minimal-Rust direction)

The user clarified that **intelligent ease of use and complete Linux functionality** outrank a minimal image footprint. Therefore the previous Alpine/OpenRC-first decision is **on hold** pending system-platform comparative evidence. I009 Issue #12 is re-scoped to evaluate the operating system base **and** the machine interfaces exposed to the Agent; it is not a commitment to replace Rust PID 1 with BusyBox init.

### Revised evaluation dimensions, in priority order

1. **System API completeness and stability:** service/process lifecycle, resources, network, device discovery, storage/filesystems, users/sessions, logs, software/updates, recovery; can Rust call typed APIs without shell-output scraping?
2. **Safe autonomy:** least privilege, polkit/IPC authorization, per-operation target identity and confirmation, observable progress and audit, bounded execution, rollback/compensation where possible. Never let arbitrary model-produced methods/parameters reach system D-Bus as root.
3. **Full general-purpose Linux viability:** compatible drivers, kernel modules, POSIX applications, package management, remote/local network, consoles and future graphical session; no required conventional desktop, but no deliberate loss of Linux functionality.
4. **Bootable removable Live OS:** read-only/ephemeral root, optional persistence, x86_64 BIOS and UEFI QEMU proof, secure/offline recovery and portable-media compatibility tested independently; no silent physical writes.
5. **Maintenance and delivery:** stable distro support, immutable/reproducible build, patching, image/lifecycle and source-license review.
6. **Footprint/boot speed:** measure but treat as optimization rather than first-order decision.

### Platform candidates, not merely init binaries

| Candidate product foundation | Agent-native leverage | Key costs/unknowns | Stage |
| --- | --- | --- | --- |
| **Debian stable (13/trixie) + systemd + glibc + custom Live ISO** | Mature PID1/unit DBus, cgroups, journald, udev/logind; strong integration with NetworkManager, UDisks2, polkit and native Rust D-Bus; broad software/driver support; Debian live-build path | Larger footprint, rebuild of Alpine/musl rootfs assumptions, live persistence/upgrade contract remains work | **Provisional primary PoC** |
| **NixOS + systemd** | Same Linux API layer plus declarative system configuration, generations and rollback for high-risk Agent-applied system changes | Nix store/build/update semantics and Live USB portability add substantial project complexity; still requires policy and safe target selection | **Strategic challenger** |
| **Alpine + BusyBox init + OpenRC** | Known light Alpine/musl stack; I005-I008 kernel prototype reused; typed APIs possible from independent services | OpenRC core largely shell-based service scripts and lacks systemd manager's rich common native D-Bus API; integration burden shifts to Vita | **Lightweight control baseline** |
| **Chimera/Dinit or Alpine with Dinit** | Dependency and supervision control socket and small service model | Fewer ready-made standardized general-Linux management API integrations; distro and session/udev integration need proof | **Conditional challenger** |
| **Finit / OpenWrt procd** | Embedded appliance functionality; Finit integrates service+getty, procd offers event/service ubus | Tailored to appliance/router assumptions; less obvious broad general-Linux stack and platform API ecosystem | **Specialized alternatives** |

**Superseded provisional recommendation:** systemd appeared better for Vita's **complete OS capability surface** than an init-only minimalism contest; **Debian stable + systemd + native Rust Broker** is the lead engineering experiment, not a final migration decision. NixOS may win if versioned system-state transactions/rollbacks are a primary product need and its resource/delivery cost is acceptable. Neither is proven in Vita yet.

### What the Agent should actually control

```text
User (local TTY / future graphical interface)
  -> Vita native Rust/Talos Runtime (unprivileged primary UI)
     -> Intent -> typed capability request -> plan/dry-run/target binding
        -> Vita policy + confirmation + audit (never model-provided root commands)
           -> narrowly privileged Vita system broker
              -> systemd1 D-Bus: units, transient bounded jobs, scoped resources
              -> NetworkManager D-Bus: devices, links, profiles
              -> UDisks2 D-Bus: storage discovery / controlled mount
              -> logind D-Bus: seats, sessions, sleep/power
              -> journald API: structured diagnostics
              -> udev native monitor: devices / stable identity
              -> package/update adapter (distro-specific, separate authorization)
           -> progress + exact changes + structured results + appropriate rollback
Linux kernel + mature system services supply real functionality below.
```

- Vita does **not** need to implement networking, logind, init, disk scanning or package management from scratch, nor replace all system functions with command wrappers.
- Native Rust D-Bus via `zbus` is a candidate mechanism; final interfaces and crate versions must be pinned, policy-validated and tested. systemd is a service manager and system API **substrate**, not Vita's Agent policy or conversational UI.
- Only operations with a supported rollback contract can claim rollback; power off, disk format, firmware and physical-device writes are **not** generally reversible. Confirmation and target verification remain mandatory.
- Run the primary Vita/Talos session non-root, with a real controlling TTY in early VM proofs. A graphical Agent UI is a later independent human-interface layer; using systemd/logind never mandates GNOME/KDE or launching Talos inside bash.
- Use controlled noninteractive service health/activation to keep the broker responsive independently of Talos/model availability; offline recovery remains separate and human authorized.

### Revised I009 comparative acceptance

1. Produce disposable pinned **Debian-systemd** Live ISO and **Alpine-OpenRC** rootfs/ISO control baseline; optionally test **NixOS-systemd** separately when system-state rollback is demonstrably beneficial. Do not mix direct QEMU kernel proofs with ISO-backed proof claims.
2. Boot each qualifying candidate in BIOS and OVMF UEFI QEMU into the same **native** Talos session on an unprivileged controlling TTY, with actual read-only Tool calls.
3. Demonstrate typed read-only introspection of **service state, device identity, network interfaces and structured logs**, capturing API contracts and implementation dependencies.
4. Demonstrate one allowlisted, reversible service restart with broker authorization, before/after observability, failure injection and audit; prohibit arbitrary unit names, root shells and unconfirmed disk/network mutations.
5. Measure rootfs/ISO bytes, cold boot time, idle RAM, integration effort and package/firmware coverage on identical guest hardware and reproducible pinned inputs.
6. Test systemd service crash/restart, child reaping, session recovery, no-network/local-mode operation, and offline rescue isolation. Preserve I001-I008 and separate Ventoy Issue #5.
7. If proposing systemd final base, explicitly supersede the **future-base** clause of ADR-0006 with evidence; keep Alpine I005-I008 provenance intact. No release without package/GPL license inventory and real-device/manual gates.

### Primary upstream basis

- systemd manager typed D-Bus, units and jobs: https://www.freedesktop.org/software/systemd/man/latest/org.freedesktop.systemd1.html
- systemd login/session D-Bus: https://www.freedesktop.org/software/systemd/man/latest/org.freedesktop.login1.html
- systemd journal structured API: https://freedesktop.org/software/systemd/man/latest/sd-journal.html
- systemd resource control and sandboxing: https://freedesktop.org/software/systemd/man/latest/systemd.exec.html
- Native Rust zbus: https://z-galaxy.github.io/zbus/
- NetworkManager stable D-Bus: https://www.networkmanager.dev/docs/developers/
- UDisks2 D-Bus/polkit: https://wiki.freedesktop.org/www/Software/udisks/
- Debian live-build and image tooling: https://wiki.debian.org/SystemBuildTools
- Debian 13 stable support: https://www.debian.org/releases/
- NixOS generations and rollback: https://nixos.org/manual/nixos/stable/
