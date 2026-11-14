# ADR-0008 — Vita small-and-complete Agent-native Linux OS

Status: **Accepted product architecture and boundaries (2026-11-07); PID 1 implementation OPEN pending I009 measurements**
Story: VITA-001-C / VITA-001-D / VITA-001-F; [I009 Issue #12](https://github.com/wjhuang88/vita/issues/12).
References: [Vita OS design contract](../reference/VITA-OS-DESIGN.md), ADR-0001, ADR-0002, ADR-0004, ADR-0005, ADR-0006, ADR-0007.

## Context and superseded proposals

The project-owner clarified three constraints across the init and ChromeOS discussion:
1. **Small and beautiful is a hard product constraint**, not an optional optimization. Keep the shipped OS and number of permanent daemons intentionally small.
2. **Complete Linux ability is equally hard**: normal Linux applications, drivers, networking, local storage, user sessions and hardware operation must remain possible; completeness does **not** mean bundling a general-purpose desktop distro.
3. **Native Agent-first operation is the defining user experience**. The embedded Rust Vita/Talos Runtime is the default OS interface. Standard shells remain independent compatibility/recovery tools, not the main UI.

The earlier BusyBox init + OpenRC preference, and the subsequent Debian/systemd/NixOS-first recommendation, were **proposals, not accepted irreversible choices**. The owner rejected the loss of minimalism and is willing to implement bounded infrastructure in Rust. This ADR supersedes those recommendations **as product-selection defaults**. The decision history remains in the commits of [PR #13](https://github.com/wjhuang88/vita/pull/13) and the dated history of [Issue #12](https://github.com/wjhuang88/vita/issues/12). No source evidence or experiment is erased.

Alpine/musl **remains the chosen first engineering base** (ADR-0006), subject to demonstrated ABI, hardware, licensing or security limitations. Existing I005–I008 boot work remains valuable. We do **not** import the ChromeOS browser, board firmware, Upstart or its exact partition layout.

## Accepted decisions

**D1 — System layering.** Firmware/GRUB (unmodified Ventoy compatibility substrate for the multi-ISO product) → Linux kernel/modules/firmware → small early boot and read-only Vita rootfs → independent init/service lifecycle → unprivileged Vita session manager and embedded Talos runtime → typed native OS capabilities → separately privileged broker with policy and audit. Talos can crash or be unavailable without killing PID 1 or the offline recovery environment.

**D2 — Rust ownership.** Vita's control plane (session management, typed providers, service view, broker, confirmations, audit, updates and state model) is Rust-first. Linux kernel, filesystem tools, Wi-Fi protocols, firmware, proven crypto and other well-vetted components may remain external where safer. Never equate full Linux capability with preinstalling giant stacks. No normal-model-to-root-shell or arbitrary model-issued D-Bus/service-manager method calls.

**D3 — PID 1 is a deliberately small, independent failure domain.** PID 1 must reliably reap orphan children, own signals, supervise critical services with bounded restart, handle clean shutdown, and fail safely. **No embedded Talos/LLM loop or broad broker privileges inside PID 1.** The currently merged I008 Rust PID 1 is only a sleeping proof and **not a production init**.

**D4 — Candidate shortlist, not final selection.** Compare a bounded Vita Rust init against **Dinit or Finit**, using Alpine BusyBox init + OpenRC as a known risk-control benchmark. Tini/dumb-init/catatonit are only container/single-process init helpers. ChromeOS Upstart is a design reference, not a dependency. Debian/systemd/NixOS are capability benchmarks, not mandatory shipped stacks. Choose based on measured reliability, boot/TTY behavior, ownership cost, system completeness, dependency footprint and recovery; never by Rust purity alone.

**D5 — Session ownership.** A dedicated Vita session manager creates a **real controlling TTY** (serial first; graphical console later), runs Talos as a **non-root** process, handles exit/crash/respawn with a bounded budget, and exposes explicit service/session states. Background OS services are independently supervised; an Agent crash cannot be mistaken for OS shutdown. A raw serial file descriptor is not proof of a controlling terminal.

**D6 — Immutable OS, separated state.** First product rootfs is a versioned read-only image (candidate squashfs) with bounded tmpfs runtime and *explicitly selected* optional persisted user/config/skill/agent-state volumes. Never discover-and-mount arbitrary writable host disks. Later: versioned images, boot-success acknowledgements, bounded fallback; on USB this may be two ISO/image versions rather than ChromeOS A/B GPT slots. Update/rollback not considered implemented until boot tests demonstrate it.

**D7 — Typed operating-system capabilities.** Prefer safe Rust interfaces to /proc, /sys, netlink, kernel device notifications and typed IPC. Define normalized process/service, network, disk, device, session, diagnostics and software/update providers with scoped permissions and structured results. Start with read-only process/service and network/device inspection. All privileged mutations require typed allowlists, verified targets, explicit human confirmation, plan/result comparison and audit. Rollback may only be promised for operations with real reversible semantics.

**D8 — Offline recovery and isolation.** A local, independently bootable/available recovery path must not require Talos, network or model credentials. I007's local unauthenticated root BusyBox shell is a feature-gated VM experiment **disabled in normal/I008 images**, not a shipping recovery policy. Use scoped Linux capabilities, seccomp, namespaces and resource isolation for untrusted tools or privileged helpers after explicit threat testing. Borrow ChromeOS's service separation, not its hardware verified-boot claims on arbitrary PCs.

## Explicit unknowns / approval boundaries

- Final PID 1 (Rust vs Dinit/Finit vs BusyBox/OpenRC): **OPEN**; verify against I009 acceptance before selection. If Rust init is chosen, its process-state tests and recovery must match mature implementations.
- Rootfs format and size budget, hardware and firmware driver inventory, secret storage, actual TTY behavior, agent/provider authorization model, long-lived storage layout, update/rollback and secure boot: **OPEN**.
- Real device write targets, Linux/BusyBox and third-party license source obligations, recovery authentication, signing and local physical-access policy: **HUMAN REVIEW PENDING**. Do not claim product release.
- Third-party Ventoy ISO compatibility [Issue #5](https://github.com/wjhuang88/vita/issues/5) remains an independent blocker even if Vita's own GRUB ISO boots.

## Sequence of bounded execution

- **I008 (complete VM-only):** self-owned GRUB ISO boots BIOS+UEFI into native Talos SDK under pinned Alpine kernel, all 9 exact-head checks successful; [PR #11](https://github.com/wjhuang88/vita/pull/11), merged as `f24c1498`. The experimental PID 1 is still incomplete.
- **I009a (first executable gate):** prototype a *separate* minimal Vita Rust PID 1, explicit service/session state transitions, bounded restart, SIGCHLD reaping and shutdown, and keep actual Talos interaction independent. Test in QEMU ISO. If controlling TTY is not solved in this slice, mark the gap **PENDING**, not Complete.
- **I009b (Linux product foundation):** minimal read-only Alpine rootfs, correct login/TTY session, device events, Linux application compatibility, OS service separation; compare Dinit/Finit with the same measured workload.
- **I009c (Agent operation):** Rust native readonly providers and independently authorized broker action, progress/audit and failure policy; actual Tool traversal inside Linux guest.
- **I009d (portable durability):** explicitly approved persistence, image-version rollout, boot-health and fallback, offline authenticated recovery, later hardware/manual release gates.

Each slice must preserve the I001–I008 regression matrix, publish exact-head QEMU evidence and never write actual USB/host disks without a separate identity/confirmation gate.

## Evaluation and reversal

Reject/replace a candidate if it fails PID 1 process supervision, independent Talos recovery, hard security boundaries, real TTY, normalized Agent APIs, required hardware, reproducible packaging, licensing or the small-system target. Collect measured image bytes/RAM/boot time before setting numeric budgets. A changed product constraint needs a new recorded ADR rather than another unmarked contradictory paragraph.

## Verified design sources

- ChromiumOS boot and system architecture: https://www.chromium.org/chromium-os/chromiumos-design-docs/boot-design/ and https://www.chromium.org/chromium-os/chromiumos-design-docs/software-architecture/
- ChromiumOS read-only root/image update: https://www.chromium.org/chromium-os/chromiumos-design-docs/filesystem-autoupdate/
- ChromiumOS session manager and update_engine: https://chromium.googlesource.com/chromiumos/platform2/+/HEAD/login_manager/README.md and https://chromium.googlesource.com/chromiumos/platform2/+/HEAD/update_engine/
- Linux initramfs and PID 1 boot behavior: https://www.kernel.org/doc/html/latest/filesystems/ramfs-rootfs-initramfs.html
- Alpine minimal Linux: https://www.alpinelinux.org/about/
- Dinit / Finit / OpenRC: https://davmac.org/projects/dinit/ ; https://troglobit.com/finit/ ; https://github.com/OpenRC/openrc

## Dated implementation evidence — 2026-11-14 (does not change the accepted decision)

[I009a PR #14](https://github.com/wjhuang88/vita/pull/14) demonstrated a minimal Rust PID1 engineering **candidate** with nonroot controlling-TTY Talos, child reaping, bounded restart and fault paths in real BIOS/UEFI ISO guests. [I009b0 PR #16](https://github.com/wjhuang88/vita/pull/16) now also demonstrates actual readonly `/vita` application mount (root + UID1000 `EROFS`) and bounded volatile scratch, with 14/14 exact final PR-head checks. **Neither proof finalizes the PID1 selection nor establishes a readonly `/` rootfs.** True rootfs, service/device lifecycle and Dinit/Finit comparison remain open. The latest implementation evidence supersedes only the dated experimental-status descriptions above; D1–D8 decisions and human safety gates are unchanged.

## I009b1 measured VM candidate (2026-11-14)

[PR #19](https://github.com/wjhuang88/vita/pull/19), merged `96713752b2b157760543d695bbb2fee7f6ad76a3`, proved a **genuine readonly SquashFS `/`** under BIOS/UEFI QEMU guest ISO with a SHA-checking Rust stage1, PID1 continuity to native Talos, both root/UID1000 EROFS and deliberate root-image tamper rejection. [Final run 43555621207](https://github.com/wjhuang88/vita/actions/runs/43555621207), 17/17 checks, independent Copilot review and zero unresolved code findings. **This updates the engineering evidence only:** Rust vs Dinit/Finit/BusyBox+OpenRC final selection, service/device lifecycle, physical storage mapping, persistence, source-license inventory, real recovery and human product/security approval remain OPEN. No change to the adopted architectural boundaries.
