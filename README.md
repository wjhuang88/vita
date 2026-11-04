# Vita

**The computer is the tool. Intent is the interface.**

Vita is an open-source, **Rust-first, agent-native portable operating environment**. The vision is a bootable USB that can start your own ISO images *or* enter a purpose-built Linux Live system where [Talos Core](https://github.com/wjhuang88/talos) becomes the primary way to understand and operate the machine.

**Not another Linux distribution with an AI CLI preinstalled.** Vita is being designed around a first-class agent interaction runtime, structured operating-system capabilities, and an independently accessible recovery path.

[![Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![I001 Foundation](https://github.com/wjhuang88/vita/actions/workflows/i001-foundation.yml/badge.svg)](https://github.com/wjhuang88/vita/actions/workflows/i001-foundation.yml)
[![Stage](https://img.shields.io/badge/stage-pre--alpha-orange.svg)](docs/roadmap/IMPLEMENTATION-ROADMAP.md)

[**简体中文**](README.zh-CN.md) · [Architecture](docs/reference/SYSTEM-ARCHITECTURE.md) · [Roadmap](docs/roadmap/IMPLEMENTATION-ROADMAP.md) · [Contributing](CONTRIBUTING.md)

> [!IMPORTANT]
> **Pre-alpha / engineering preview.** Vita does not yet ship an end-user Live OS ISO or a USB installer. The repository contains validated boot-chain **probe images**, a direct Linux kernel-to-Rust PID 1 VM proof, and a standalone Talos Runtime/console prototype. These are separate experiments, **not** a complete boot-to-Talos operating system.

## What we're building

| Boot an entire toolbox | Talk to your machine | Keep control |
| --- | --- | --- |
| Keep multiple ISO images on the same USB medium and select them at boot. | Start a Talos Core-powered system session as the intended default interface—not from inside bash. | Invoke typed, policy-governed capabilities. High-impact actions require target verification, confirmation and audit. |

Vita aims to serve portable diagnosis, recovery, administration and experimental agent-native computing. **The interface should understand intent; the system should make every consequential action inspectable.**

### A different system boundary

```text
                            VITA USB
                               │
                        BIOS / UEFI boot
                               │
                 Ventoy compatibility substrate
                     ┌─────────┴──────────┐
                     │                    │
             Your ISO images       Vita Live OS (planned)
                                          │
                                Linux kernel + minimal init
                                          │
                                Vita session host (Rust)
                                          │
                                   Talos Runtime
                                          │
                              Governed capability broker
                                          │
                          Rust-native Linux system providers
                                          │
                            /proc · /sys · netlink · ioctl

             Independent recovery shell (planned)
```

**Responsibility is deliberate:** [Ventoy](https://www.ventoy.net/) handles early boot and ISO compatibility in Phase 1. Vita owns its Live OS, agent session, policy, system capabilities, user experience and release lifecycle. Talos is embedded via the supported `talos-runtime` SDK rather than wrapped through `talos-cli`.

Read [ADR-0004](docs/decisions/ADR-0004-hybrid-ventoy-boot-substrate.md) (boot), [ADR-0001](docs/decisions/ADR-0001-talos-primary-interaction-runtime.md) (interaction) and [ADR-0005](docs/decisions/ADR-0005-rust-first-system-capabilities.md) (Rust-first implementation).

## Project status: what is real today?

| Capability | Status | Evidence / next gate |
| --- | --- | --- |
| Ventoy-backed disposable disk image | **Proven in CI** | Ventoy 1.1.18 installed only to a verified loop-backed temporary image |
| Default Vita probe ISO, legacy BIOS | **Proven in QEMU** | `VITA_BOOT_OK` guest marker |
| Default Vita probe ISO, x86_64 UEFI | **Proven in QEMU/OVMF** | `VITA_BOOT_OK` guest marker |
| Second user probe ISO, legacy BIOS | **Proven in QEMU** | `VITA_USER_ISO_BOOT_OK` guest marker |
| Embedded Talos Runtime + read-only `system_summary` | **Proven in CI** | Rust prototype executes a deterministic, offline agent turn without a shell |
| Representative third-party Linux ISO boot | **Next** | VITA-001-B |
| Real Linux kernel -> Vita Rust PID 1 | **Proven in QEMU (direct)** | I005; not a Live rootfs, Talos session or ISO |
| Usable Vita Live rootfs/default Talos session | **Not yet built** | VITA-001-C + D |
| Governed privileged operations, recovery, persistence | **Planned** | VITA-001-E/F/G |
| Secure Boot, physical-device matrix, releasable image | **Not yet validated** | VITA-001-H and later release hardening |

The completed **I001 foundation** is documented in [the iteration record](docs/iterations/I001-foundation.md), with executable source in [the boot prototype](prototypes/boot/) and [the Rust embedding prototype](prototypes/interaction-runtime/). See [GitHub Actions](https://github.com/wjhuang88/vita/actions/workflows/i001-foundation.yml) for current results. **A passing probe is not evidence that a full Live Linux system is available.**

## Try the engineering prototypes

The following commands are for contributors, **not** an end-user installation procedure.

### 1. Inspect and validate project governance

```sh
git clone https://github.com/wjhuang88/vita.git
cd vita
sh scripts/validate_project_governance.sh .
```

### 2. Run the Talos Runtime embedding proof

On Linux with Rust installed:

```sh
rustup toolchain install 1.99.0 --profile minimal
cargo +1.99.0 run --manifest-path prototypes/interaction-runtime/Cargo.toml
```

Expected success marker:

```text
VITA_I001_TALOS_RUNTIME_OK
```

The prototype pins a tested Talos Git revision, uses a deterministic fake model (no API key or network model endpoint), registers a read-only `system_summary` tool and accesses typed, bounded Linux facts through [vita-system-facts](crates/vita-system-facts/) without a shell. The crate's four tests and the Talos integration have passed CI. It does **not** provide the final interactive shell.

### 3. Validate the temporary-image boot chain

On a suitable **disposable Linux development VM or GitHub-hosted runner** with QEMU support:

```sh
bash scripts/ci/validate_i001_boot.sh
```

This script installs host testing dependencies, downloads and checksum-verifies a pinned Ventoy release, creates a temporary raw image, and uses privileged loop-device operations **only after verifying the device's backing file matches that image**. It boots probe ISOs with QEMU/OVMF and checks guest-emitted serial markers.

> [!CAUTION]
> The harness uses `sudo`, installs packages, and runs a boot-media installer against a temporary **file-backed loop device**. Review the script before executing it; use an isolated VM/runner. **Never substitute a real disk path** or treat this command as an installer for a physical USB drive.

## Offline native-console prototype (I004)

The host-side Rust console is an **offline deterministic fixture**, not a released operating system or real LLM client. It drives the actual Talos Runtime event stream and registered read-only system tool:

```sh
cargo +1.99.0 run --manifest-path prototypes/interaction-runtime/Cargo.toml -- --console-fixture
```

Use `:help` for commands; `demo:error` and `demo:slow` are fixtures for provider error and cancellation. Manual TTY/PTY, human security review and deferred Live OS recovery gates are tracked in the [I004 human acceptance checklist](docs/validation/I004-HUMAN-ACCEPTANCE.md).

## Planned portable data layout

The following is the **Phase 1 logical contract**, not a ready-to-flash image:

```text
/iso/
  vita/vita-live.iso       # default Vita image (not yet published)
  other-rescue.iso         # user-managed boot images
/talos/
  config/models.toml       # model/provider references
  config/agent.toml        # agent settings and policy
  skills/
  workspace/
  logs/
/ventoy/ventoy.json        # boot-menu configuration
```

The normal data-preserving upgrade goal is to replace Vita-owned images/configuration while retaining user ISO files and the `/talos` workspace. See [USB layout](docs/reference/USB-LAYOUT.md) and [configuration contract](docs/reference/CONFIG-CONTRACT.md).

## Design principles

- **Agent-native, not shell-first.** Talos Core drives the default interaction lifecycle; an independent local shell remains available for recovery.
- **Rust-first where Vita owns the implementation.** Prefer maintained Rust/Linux-native APIs over launching system utilities and parsing human-oriented output. Keep Linux and proven boot software where they are the safer boundary.
- **Capabilities, not arbitrary root commands.** Typed inputs/outputs, allowlists, risk classes and explicit confirmation are the intended privilege model.
- **Offline-aware and portable.** Recovery must work without a remote model. Secrets do not belong in Git or baked into images.
- **Evidence before hardware.** Validate boot and storage changes in disposable VM images before any real USB operation.

These are design requirements; not all enforcement layers exist yet. See [architecture](docs/reference/SYSTEM-ARCHITECTURE.md), [security threat model](docs/reference/THREAT-MODEL.md) and the [backlog](docs/backlog/PRODUCT-BACKLOG.md).

## Roadmap

1. **Foundation — complete:** choose and validate boot substrate; prove direct Talos Runtime embedding.
2. **Bootable Vita:** select minimal Linux base, create a real Live ISO, boot a representative third-party ISO, and start the default session.
3. **Agent-native system interface:** native console/session UX, structured hardware/system context and independent recovery.
4. **Governed system operations:** Rust system providers, policy enforcement, confirmation and audit.
5. **Portable product:** removable-media configuration/persistence, safe upgrade, validation matrices and release images.

Track work through [BOARD](docs/BOARD.md), [Product Backlog](docs/backlog/PRODUCT-BACKLOG.md) and [Implementation Roadmap](docs/roadmap/IMPLEMENTATION-ROADMAP.md).

## Contributing

Vita is early-stage and welcomes focused technical contributions: Rust-native system APIs, minimal Linux image building, firmware/ISO compatibility, security reviews, CI portability and documentation.

Start with [CONTRIBUTING.md](CONTRIBUTING.md) and [AGENTS.md](AGENTS.md). For boot/media, storage or privilege changes, include test evidence and follow the project's [high-risk review procedure](docs/sop/PAIRING-WORKFLOW.md).

## License and third-party software

**Vita-authored source code is licensed under the [Apache License 2.0](LICENSE).** See [NOTICE](NOTICE).

Vita's Phase 1 boot plan uses **Ventoy**, whose own work is licensed **GPLv3+**, along with other separately licensed components. Their terms are **not** replaced by Vita's Apache-2.0 license. This source repository does not vendor Ventoy binaries; the CI prototype fetches an upstream release. A distributable boot image must undergo its own third-party license and corresponding-source compliance review. See [third-party notices](THIRD_PARTY_NOTICES.md) and [Ventoy licensing](https://www.ventoy.net/en/doc_license.html).

---

**Vita is a project to make operating a machine feel native to intelligence—without surrendering system-level control.**

## Next Live Linux milestone (I005, experimental)

The [I005 direct Linux boot test](prototypes/live-init/) has **passed CI** with a verified Alpine kernel and a **static Rust init probe**; [run 41791034664](https://github.com/wjhuang88/vita/actions/runs/41791034664) captured the serial marker from real guest Linux PID 1. This is explicitly an engineering *kernel-to-Rust user-space* proof, **not** the final Vita Live ISO, a Talos session, or a physical-USB writer. See [ADR-0006](docs/decisions/ADR-0006-alpine-live-base.md) and the [human review checklist](docs/validation/I005-HUMAN-ACCEPTANCE.md). The separate Live rootfs and default Talos session remain to be built.
