# I009b2b — controlled same-runner readonly-root-source benchmark

Status: **Implementation submitted for CI / independent review; not yet validated**
Parent: [I009b2 #21](https://github.com/wjhuang88/vita/issues/21) and VITA-001-C.
Prerequisite: I009b2a media-root source [PR #22](https://github.com/wjhuang88/vita/pull/22) and documentation [PR #23](https://github.com/wjhuang88/vita/pull/23), both merged.

## Immutable baseline and acceptance

**Objective:** determine actual boot-to-Talos and memory cost of the I009b1 embedded-root vs I009b2a ISO-backed root using the same pinned Alpine kernel, immutable SquashFS, Talos SDK, Rust PID1, runner and guest workload. A smaller initramfs is **not** itself evidence of faster boot or lower RAM.

**Boundaries:** disposable QEMU only; read-only CD-ROM, no guest networking, no host disk passthrough, no physical USB writes. Rust PID1 remains **a candidate**, not final selection. Do not distribute/release generated ISO. Human firmware/security/source licenses/SBOM/recovery/hardware checks stay **PENDING**.

**Build:** reuse the existing exact SHA-verified Alpine ISO/kernel and matching kernel modules. In opt-in benchmark mode only, export one *temporary* embedded-root ISO and one temporary media-root ISO. Runtime rootfs contents must be equal in bytes. The default I009b1/I009b2a full fault/tamper suite remains unchanged; benchmark build-only is restricted to a new disposable /tmp directory.

**Boot:** one GitHub-hosted x86_64 runner, same QEMU flags: TCG, 1024 MiB RAM, two vCPUs, no NIC, no monitor, no reboot. Separate BIOS and OVMF UEFI, three successful repetitions per source/firmware (12 successful runs total). Alternate embedded/media order each repetition to reduce systematic first-run bias. Real guest session runs two read-only Talos Tool turns, one handled model-error turn and quits cleanly; each run must prove correct readonly root, TTY and media mount markers.

## Metrics and honest interpretation

- Host monotonic milliseconds from QEMU **Popen** to first observed VITA_I009_CONTROLLING_TTY_OK and first vita> prompt. This includes firmware and serial/socket observation delay; it is *not* a guest-uptime-only or pure kernel boot measurement.
- Host QEMU VmRSS at the first prompt and maximum sampled kernel VmHWM during the completed workload; this is host-process resident memory, **not guest available RAM**.
- Guest /proc/meminfo MemTotal and MemAvailable, /proc/uptime, /proc process count emitted **before Talos session spawn** from an opt-in Rust PID1 read-only probe. This is **pre-session baseline**, NOT idle/post-Talos memory. Per-process daemon breakdown, actual guest post-prompt idle/peak RAM and rootfs-memory attribution are **not yet covered**.
- Exact ISO, root image and compressed initramfs byte counts; SHA-256 of both disposable ISOs. Kernel/module and Rust binary provenance are inherited from existing I009b1/I009b2a logs.
- Keep raw-runs.jsonl, per-run metrics.json, QEMU logs, serial transcripts, and summary.json reporting median/min/max per source/firmware. ISO binaries remain only in /tmp and are deleted. No performance win should be asserted before CI captures raw data.

## Validation and governance

- [ ] Offline syntax and unit tests passing
- [ ] Exact PR-head build and 12 successful firmware/Tool-session runs
- [ ] Original I009b1/I009b2a lifecycle/tamper/legacy regressions passing
- [ ] Independent separately running AI technical review, no unresolved actionable findings
- [ ] Exact-head CI and evidence recorded; merge only after required gates
- [ ] Human product/hardware/security/license/release signoff separately **PENDING**

**Next I009b2c gate:** shared Rust/Dinit/Finit experiment on identical guest workload, including TTY, SIGCHLD adoption, bounded restarts and failure shutdown. Do **not** select init yet. Guest *post-prompt* idle/peak and boot-health measurement still need a separately scoped instrumentation slice.

Engineering-only, no product release approval.
