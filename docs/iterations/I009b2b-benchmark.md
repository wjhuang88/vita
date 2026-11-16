# I009b2b — controlled same-runner readonly-root-source benchmark

Status: **Complete — VM-only benchmark/provenance slice (2026-11-16); human/product acceptance PENDING**
Parent: [I009b2 #21](https://github.com/wjhuang88/vita/issues/21) and VITA-001-C.
Prerequisite: I009b2a media-root source [PR #22](https://github.com/wjhuang88/vita/pull/22) and documentation [PR #23](https://github.com/wjhuang88/vita/pull/23), both merged.

## Immutable baseline and acceptance

**Objective:** determine actual boot-to-Talos and memory cost of the I009b1 embedded-root vs I009b2a ISO-backed root using the same pinned Alpine kernel, immutable SquashFS, Talos SDK, Rust PID1, runner and guest workload. A smaller initramfs is **not** itself evidence of faster boot or lower RAM.

**Boundaries:** disposable QEMU only; read-only CD-ROM, no guest networking, no host disk passthrough, no physical USB writes. Rust PID1 remains **a candidate**, not final selection. Do not distribute/release generated ISO. Human firmware/security/source licenses/SBOM/recovery/hardware checks stay **PENDING**.

**Build:** reuse the existing exact SHA-verified Alpine ISO/kernel and matching kernel modules. In opt-in benchmark mode only, export one *temporary* embedded-root ISO and one temporary media-root ISO. Runtime rootfs contents must be identical by both byte size and SHA-256 digest; mismatch fails the entire comparison. The default I009b1/I009b2a full fault/tamper suite remains unchanged; benchmark build-only is restricted to a new disposable /tmp directory.

**Boot:** one GitHub-hosted x86_64 runner, same QEMU flags: TCG, 1024 MiB RAM, two vCPUs, no NIC, no monitor, no reboot. Separate BIOS and OVMF UEFI, three successful repetitions per source/firmware (12 successful runs total). Alternate embedded/media order each repetition to reduce systematic first-run bias. Real guest session runs two read-only Talos Tool turns, one handled model-error turn and quits cleanly; each run must prove correct readonly root, TTY and media mount markers.

## Metrics and honest interpretation

- Host monotonic milliseconds from QEMU **Popen** to first observed VITA_I009_CONTROLLING_TTY_OK and first vita> prompt. This includes firmware and serial/socket observation delay; it is *not* a guest-uptime-only or pure kernel boot measurement.
- Host QEMU VmRSS at the first prompt and maximum sampled kernel VmHWM during the completed workload; this is host-process resident memory, **not guest available RAM**.
- Guest /proc/meminfo MemTotal and MemAvailable, /proc/uptime, /proc process count emitted **before Talos session spawn** from an opt-in Rust PID1 read-only probe. This is **pre-session baseline**, NOT idle/post-Talos memory. Per-process daemon breakdown, actual guest post-prompt idle/peak RAM and rootfs-memory attribution are **not yet covered**.
- Exact ISO, root image and compressed initramfs byte counts; SHA-256 of both disposable ISOs. Kernel/module and Rust binary provenance are inherited from existing I009b1/I009b2a logs.
- Keep raw-runs.jsonl, per-run metrics.json, QEMU logs, serial transcripts, and summary.json reporting median/min/max per source/firmware. ISO binaries remain only in /tmp and are deleted. Raw data are captured; three runs per stratum are insufficient to prove a stable performance win across runner classes.

## Validation and governance

- [x] Offline syntax and unit tests passing
- [x] Exact PR-head build and 12 successful firmware/Tool-session runs
- [x] Original I009b1/I009b2a lifecycle/tamper/legacy regressions passing
- [x] Independent separately running AI technical review, no unresolved actionable findings
- [x] Exact-head CI and evidence recorded; merge only after required gates
- [ ] Human product/hardware/security/license/release signoff separately **PENDING**

## Final exact-head VM evidence (2026-11-16)

[PR #24](https://github.com/wjhuang88/vita/pull/24) merged via squash as `8f88e3f2872d9010f25362368c01af91818f98dd` after **20/20 exact code-head** checks at `a7dc7db6e320fd9e071f5a07ba1a57eb1d32a3cb`. [Final benchmark workflow 43797290307](https://github.com/wjhuang88/vita/actions/runs/43797290307) includes a complete **12/12** BIOS/OVMF UEFI matrix: 3 embedded + 3 media boots per firmware. The [raw CI artifact #14195141657](https://github.com/wjhuang88/vita/actions/runs/43797290307/artifacts/14195141657) retains `summary.json`, `raw-runs.jsonl`, each `metrics.json`, 12 serial transcripts, and pinned kernel/module/ISO provenance; no ISO binaries were published. Original I009b1/I009b2a fault/reaper/shutdown/tamper regression checks passed on the same exact head.

An independent Copilot code-review execution was performed on `a7dc7db`: **no open code issues**, all four historical inline threads resolved. Its advisory that VM runtime CI was still running was closed by the subsequently verified 20/20 exact-head checks, not by author assertion. No human security, hardware, source-licensing, or release acceptance is implied.

**One-runner measurements:** GitHub-hosted Linux `7.0.0-1019-azure`, QEMU `10.2.1`, `accel=tcg`, 1024 MiB/2 vCPUs, no guest NIC, identical read-only Talos workload. Each cell below is the **median of 3 runs**:

| Firmware / readonly root source | Popen → first `vita> ` | Host QEMU VmHWM | Guest pre-session MemAvailable |
| --- | ---: | ---: | ---: |
| BIOS / embedded initramfs | 5,606 ms | 267,276 KiB | 875,008 KiB |
| BIOS / readonly ISO media | 5,399 ms | 265,480 KiB | 884,332 KiB |
| UEFI / embedded initramfs | 5,784 ms | 318,204 KiB | 863,684 KiB |
| UEFI / readonly ISO media | 5,846 ms | 310,020 KiB | 876,156 KiB |

- Both root images: **5,079,040 B** and identical SHA-256 `71760aa4a412a82ec1dba97e1609a6d0846d819c224660812c5e7cd2601d4621`. Embedded initramfs **5,433,116 B**; media-backed **501,180 B**. Full embedded ISO **30,992,384 B**; media ISO **31,141,888 B**.
- I verified every serial transcript against the stated source: 6 embedded boots contain **no** ISO-media markers; 6 media boots contain **both** source and moved-mount markers. All 12 include real read-only root/TTY evidence and exactly two successful Tool turns + one handled error. Exact-head benchmark script also rejects mislabeled source boots; fixed temporary exports cannot follow destination symlinks.
- Host QEMU memory **is not guest RAM**; guest memory values above were captured **before** Talos starts, **not at idle/post-prompt/peak**. Firmware, cold/warm cache and runner scheduling remain confounders. Historic `8f7e0f6` CI ran on a different AWS Linux runner; **do not compare absolute boot times across runner classes**, assert statistically robust superiority or set product performance budgets from these tiny samples.

**Boundary and residuals:** I009b2b is **Complete only as a controlled QEMU measurement harness**. [I009b2c #25](https://github.com/wjhuang88/vita/issues/25) owns Rust-vs-Dinit/Finit equivalent-workload/failure/service comparisons. Guest post-prompt idle/peak RAM and daemon inventory, on-device driver/firmware, source-license/SBOM, version/recovery/security, real USB writes and release signoffs are **OPEN / HUMAN PENDING**; see [I009b2b human acceptance](../validation/I009b2b-HUMAN-ACCEPTANCE.md). Parent [I009b2 #21](https://github.com/wjhuang88/vita/issues/21) and Story VITA-001-C remain **Active**, not closed.

**Next I009b2c gate:** shared Rust/Dinit/Finit experiment on identical guest workload, including TTY, SIGCHLD adoption, bounded restarts and failure shutdown. Do **not** select init yet. Guest *post-prompt* idle/peak and boot-health measurement still need a separately scoped instrumentation slice.

Engineering-only, no product release approval.
