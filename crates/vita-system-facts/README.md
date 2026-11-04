# vita-system-facts

Small, read-only Linux system facts provider for Vita's Rust/Talos integration layer.

- Uses only the Rust standard library; **no shell, subprocess, FFI or additional runtime**.
- Reads `/etc/os-release` and bounded `/proc` files.
- Produces typed, optional facts: OS metadata, hostname, kernel release, architecture, family, uptime seconds and total RAM (KiB).
- Missing/invalid/oversized data is returned as `None`, never as a guessed value.
- Explicit paths enable deterministic fixture tests without requiring root privileges.

This is a foundational *read-only* provider, not a privileged capability broker, device inventory or finished Vita console. The enclosing `talos-runtime` permission rule still grants only the `Read` nature.

Run standalone unit tests:

```sh
cargo +1.99.0 test --manifest-path crates/vita-system-facts/Cargo.toml
```

Integration is exercised by `prototypes/interaction-runtime` and GitHub Actions I001 Foundation.
