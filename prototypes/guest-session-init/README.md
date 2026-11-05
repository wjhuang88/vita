# I006 — Linux guest starts the Vita/Talos Rust session

**Experimental QEMU boot-to-interaction proof only.** This is not a deployable Live OS, an ISO image, a firmware matrix or a USB installer.

The script `scripts/ci/validate_i006_guest_session.sh`:

1. Authenticates Alpine virt 3.24.3 ISO and extracts the exact verified Linux kernel.
2. Statically compiles this Rust guest PID 1 and the **existing** Vita/Talos SDK host (I004) for x86_64 Linux/musl.
3. Mounts guest `/proc` with Vita's native Rust `nix` API; no packaged BusyBox or boot-time shell process.
4. Uses Linux QEMU, no disk device and no network, to boot Rust PID 1, mount /proc and run the Talos session host directly.
5. Sends live `inspect`, `demo:error`, `inspect again`, `:quit` prompts through the Linux guest serial socket and verifies read-only tool turns and controlled termination.

No Talos CLI, scripting language inside the guest, unrestricted root shell, remote LLM or production model key is involved. The mount target and flags are fixed in Vita-owned Rust code and not exposed to the model. Test host Python and shell packages are build-only.

The guest PID 1 is a temporary test shim; it does **not** provide production signal handling, reaping, service supervision or an independently available offline recovery shell. See [I006 human acceptance](../../docs/validation/I006-HUMAN-ACCEPTANCE.md).
