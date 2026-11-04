# I005 — Real Linux kernel -> Vita Rust PID 1 proof

**Experimental only.** This is the first Vita-owned Linux guest program. It is a marker probe, **not** a distribution, a production init replacement, a Talos interactive session or a release ISO.

The CI script `scripts/ci/validate_i005_live_init.sh` fetches the original Alpine Linux virt 3.24.3 ISO pinned by SHA-256, extracts its upstream kernel, statically compiles this Rust (standard-library-only) program for x86_64 musl, and packages only the program and console device node into a disposable initramfs. QEMU must emit `VITA_I005_LINUX_RUST_INIT_OK pid=1 arch=x86_64` from the **guest** serial console.

No physical medium is enumerated or written. No Python/Node/JVM is present inside the initramfs; build tooling lives on the host. Running this code as PID 1 is a smoke-proof convenience only: Vita's final Live OS is expected to use a proven minimal Linux init/service stack with the **Talos Rust session host as the default human interaction**, not reimplement an init system.

Run on a disposable Linux development VM with QEMU, Rust and sudo:

```sh
bash scripts/ci/validate_i005_live_init.sh
```

For manual validation and still-open release/firmware matters, see [I005 human checks](../../docs/validation/I005-HUMAN-ACCEPTANCE.md).
