# I007 — Volatile Linux runtime and explicit offline recovery

Status: Active
Published baseline: 2026-11-05
Selected scope: VITA-001-C Active; preparatory VM-only slice for VITA-001-G, which remains in Refinement.
Dependency: I006 Linux guest/Talos SDK proven and merged.
Risk: Linux boot/mounts, local root shell. QEMU diskless and networkless only.

## Executable goal

Start the *same* SHA-verified Alpine-kernel Linux guest in two separate QEMU modes:
1. Default: initialize /proc, readonly /sys and bounded volatile /run and /tmp; verify the real Linux mount table; launch the native Vita/Talos Rust SDK host and drive the established I006 read-only multi-turn Tool test.
2. Explicit recovery: kernel argument vita.recovery=1, and **only** that token, selects a separate local root BusyBox sh shell. Drive an actual shell command through the guest serial port, confirm guest uid=0 and observed command output, then exit. Talos must not start.

Both use the same disposable initramfs built with static musl Rust binaries and a distro static BusyBox reserved for *recovery only*, not model tools or boot mount helper. The Cargo feature experimental-root-recovery MUST be explicitly enabled for this VM fixture; ordinary builds leave the root-shell dispatch disabled. Recovery inherits no model secrets (env_clear) and has no network/disk target. Duplicate, malformed or unknown recovery parameters must fail closed.

## Automatic acceptance

- cargo +1.99.0 test --manifest-path prototypes/guest-session-init/Cargo.toml
- bash scripts/ci/validate_i007_runtime_recovery.sh
- Default Linux guest serial: I006 multi-turn Tool/error-recovery markers; I007_RUNTIME_MOUNTS_READY and I007_DEFAULT_TALOS_EXIT_OK; no recovery marker.
- Separate explicit recovery Linux guest serial: I007_RECOVERY_READY uid=0, I007_RECOVERY_ROOT_OK *assembled by guest shell execution*, I007_RECOVERY_EXIT_OK; no Vita/Talos console.
- Existing I001, I005, I006 CI plus governance remain passing. Preserve guest logs/provenance on failures.

Never claim Complete before exact-PR-head CI has confirmed both genuine QEMU guest paths.

## Exclusions and risk

This is an isolated engineering proof, **not** full Alpine rootfs, immutable read-only root, mature init/reaper, shipping recovery menu, authenticated real-device rescue, firmware/UEFI/Ventoy, persistence, LLM provider/config or USB installer. The initramfs root is still writable and ephemeral. The explicit recovery shell is a local **unauthenticated root shell**, which is unacceptable as an assumed production security design without separate review. Do not close C/D/G Stories based on this test.

## Evidence

Pending CI. Human security, legal, terminal and device validation all remain PENDING in docs/validation/I007-HUMAN-ACCEPTANCE.md.

## I007 acceptance harness change control — final-marker race and compile gate

[Initial CI 41968027990](https://github.com/wjhuang88/vita/actions/runs/41968027990) verified real Linux/Talos guest multi-turn behavior, but the original I006 serial observer terminated QEMU immediately after the I006 success marker, racing the following I007 clean-exit marker. The I007 gate **was not weakened**: the observer now accepts an optional final-marker argument and waits until that marker is genuinely received before stopping QEMU. Existing I006 calls remain unchanged.

Before any release, the risk of an unauthenticated root shell deserves an additional default-deny build boundary: I007 explicitly enables the experimental-root-recovery Cargo feature in its **diskless VM-only** build. Default builds reject recovery even if the kernel argument is provided. This supplements, not replaces, the required human/firmware authorization review.

## I007 change control — shell CPU baseline and package provenance

[CI 41968628065](https://github.com/wjhuang88/vita/actions/runs/41968628065) passed the default Linux/Talos guest and the new post-session marker, but **failed recovery**: Ubuntu 26.04 busybox-static 1.37.0 exited with SIGILL under the QEMU default x86_64 CPU model. The [failed guest serial artifact](https://github.com/wjhuang88/vita/actions/runs/41968628065/artifacts/13388896385) is preserved; the kernel and native mounts were working.

Do not weaken the CPU-baseline target by silently switching to QEMU -cpu max. Instead use Alpine **v3.24/main/x86_64/busybox-static-1.37.0-r31.apk** from the distro matching the pinned kernel. Extract only its static /bin/busybox.static, log source package and executable SHA-256 and test with the unchanged generic QEMU CPU. The APK download is initially an exploratory build input: **a fixed expected SHA-256 for the APK must be added before declaring I007 Complete/merge-ready**. No real host or guest disks are exposed by this experiment.

Earlier CI 41968027990 failed solely because I006's serial observer returned before the adjacent I007 final marker; that race was corrected by a backward-compatible optional final-marker gate.

### Source/license gate

BusyBox is GPL-2.0-only; the eventual Live ISO must satisfy source-redistribution obligations. The pinned download alone is not legal approval; HUMAN-002 stays unchecked.
