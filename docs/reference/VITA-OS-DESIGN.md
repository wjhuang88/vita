# Vita OS design contract (2026-11-07)

**Status: accepted product architecture; implementation maturity: engineering prototype only.**
Owner: [ADR-0008](../decisions/ADR-0008-linux-init-service-supervision.md); work tracked under [VITA-001-C](../backlog/active/VITA-001-C-live-os.md) and [I009 Issue #12](https://github.com/wjhuang88/vita/issues/12).

## Product invariant

**A small, beautiful, functionally complete Linux whose primary human interface is the native Vita/Talos Agent.** Full Linux functionality means kernel/driver/device/network/storage/users/process/software capability, not a large packaged distro or desktop. Minimal shipped software and Rust-native system capabilities are first-class goals alongside safety and reliability.

### Trust and process boundaries

```text
Firmware -> GRUB / owned Live ISO [Ventoy compatibility is independent]
  -> Linux kernel + modules / firmware
     -> early boot -> verified, read-only, versioned rootfs + bounded volatile runtime
        -> minimal permanent PID 1 [selection OPEN: Rust vs Dinit/Finit, BusyBox/OpenRC baseline]
           |-> independently supervised OS services and Vita privileged Broker
           |-> Vita Session Manager -> unprivileged native Rust/Talos SDK UI
           |                                 -> typed Tool -> policy/confirm/audit -> Broker
           |-> separate offline recovery path [authorization design PENDING]
           +-> Linux device/TTY/network/storage services and optional applications
        -> explicit optional writable user state [no implicit host-disk writes]
```

1. **PID 1:** tiny and reliable; reaping, signals, restart budget, shutdown. Never host LLM logic or user policy.
2. **Session:** real controlling terminal/session identity, non-root Talos host, independent restart on crash, recovery still works without model.
3. **Control plane:** Rust providers for typed process, service, device, network, filesystem, software, logs and session APIs. Avoid model-written shell pipelines.
4. **Privilege:** target identity, allowlist, plan, explicit human approval for risks, broker execution, bounded response, auditable outcomes. Never model → root directly.
5. **Persistence:** OS image separate from trusted configuration, user files, skills, memories and logs. Updates are versioned and guarded by boot-health checks; fallback not yet implemented.
6. **Full Linux compatibility:** retain POSIX/ELF programs and real kernel/hardware support. Use mature device/security/protocol components when self-implementation would add risk.
7. **Recovery:** local/offline separate interface, not automatically accessible from an Agent prompt. An unauthenticated root recovery shell is *not* a release feature.

## Accepted / open / disproven

| Subject | Status |
| --- | --- |
| Rust-first Vita OS control plane and embedded Talos as primary UI | **ACCEPTED** |
| Minimal but fully capable Linux rather than mainstream large distro | **ACCEPTED** |
| Alpine/musl as the first engineering base | **ACCEPTED for prototypes**, reassess only with evidence |
| Independent PID 1 / session / broker trust domains | **ACCEPTED** |
| Rust PID 1 vs Dinit/Finit/BusyBox+OpenRC | **OPEN**, measurable I009 decision |
| Read-only rootfs, versioned images and separated user data | **ACCEPTED target**, **NOT IMPLEMENTED** |
| BIOS/UEFI guest boot via Vita-owned ISO and Talos SDK fixture | **VERIFIED I008 VM-only** |
| Production controlling TTY, service reaper/supervisor, drivers/hotplug | **NOT YET VERIFIED** |
| Physical USB / Secure Boot / third-party Ventoy ISO / release | **NOT VERIFIED** |
| Recovery authentication / GPL-source fulfillment / hardware testing | **HUMAN GATES PENDING** |

## How implementation must proceed

I009a: bounded process-reaper and service/session separation with VM-only evidence; I009b: complete small rootfs and real controlling TTY; I009c: real Linux native Rust provider → governed broker Tool action; I009d: versioned image/persistence/recovery.

**Do not lose the original I008 evidence**: [PR #11](https://github.com/wjhuang88/vita/pull/11) / [9 passing exact-head checks](https://github.com/wjhuang88/vita/actions/runs/42367193854). Separate [Ventoy Issue #5](https://github.com/wjhuang88/vita/issues/5) remains unresolved.

Borrow from ChromeOS only its stable base, session-supervisor separation, immutable OS/writable user-state split, and rollback/failsafe approach; not the browser, board-specific partition scheme, Upstart binary or verified-firmware assumption.
