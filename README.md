# Vita

> **Vita** is a portable Agent-native operating environment powered by Talos Core.

Vita is a portable USB operating environment with two first-class boot paths:

1. **Multi-ISO boot** — discover ISO images stored on the USB drive and boot them without rewriting the device for every image.
2. **Agent-native Live OS** — boot a directly usable Linux environment whose primary user interaction layer is powered by **Talos Core**.

Talos is **not** treated as a bundled CLI tool. The target is an OS interaction runtime that occupies the role normally held by an interactive shell/session: the user expresses intent, Talos reasons over system state, and privileged capabilities are invoked through a policy-controlled, auditable capability broker. A conventional shell remains available as an explicit compatibility and recovery path.

## First-round goals

- UEFI-first portable USB boot with an explicit BIOS-compatibility decision.
- ISO discovery and selectable boot entries.
- A directly bootable Live OS image.
- Talos Core-based primary interaction runtime.
- USB-local configuration discovery for model endpoint/model/agent policy.
- System capabilities for files, processes, storage, network and diagnostics.
- Explicit confirmation and audit for destructive or security-sensitive actions.
- Reproducible image build and QEMU-based smoke validation before physical-device tests.

See [`docs/README.md`](docs/README.md) and [`docs/backlog/PRODUCT-BACKLOG.md`](docs/backlog/PRODUCT-BACKLOG.md).
