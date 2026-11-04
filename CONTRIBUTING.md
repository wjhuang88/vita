# Contributing to Vita

Thanks for your interest in Vita. This is a pre-alpha, Rust-first, agent-native portable operating environment. We value **small verifiable changes and explicit safety boundaries** over broad implementation claims.

## Before starting

- Read [AGENTS.md](AGENTS.md) for project rules and [docs/README.md](docs/README.md) for documentation ownership.
- Check the [product backlog](docs/backlog/PRODUCT-BACKLOG.md), [iteration evidence](docs/iterations/) and [architecture decisions](docs/decisions/).
- Prefer an issue or focused design discussion for a new boot/storage/runtime architecture, with an acceptance test and rollback story.
- Keep contributions scoped; do not introduce a new runtime or privileged subprocess without documenting the need and reviewing [ADR-0005](docs/decisions/ADR-0005-rust-first-system-capabilities.md).

## Development and verification

On Linux with Rust installed:

```sh
sh scripts/validate_project_governance.sh .
rustup toolchain install 1.99.0 --profile minimal
cargo +1.99.0 run --manifest-path prototypes/interaction-runtime/Cargo.toml
```

CI validates the I001 foundation in [GitHub Actions](https://github.com/wjhuang88/vita/actions/workflows/i001-foundation.yml). For the disposable-image QEMU boot validation, use a controlled Linux VM/runner and read [BOOT-MEDIA-VALIDATION.md](docs/sop/BOOT-MEDIA-VALIDATION.md) before running `scripts/ci/validate_i001_boot.sh`.

**Do not test write/format/bootloader code on a workstation disk or someone else's USB medium.** All destructive-media automation must reject arbitrary block devices and verify loop backing file identity. Never commit model credentials, disk identifiers or personal system logs.

## Pull request expectations

1. State the issue/story or architectural decision and what observable result changes.
2. Include relevant tests and clear evidence; for boot work, distinguish BIOS/UEFI, guest-marker proof and physical testing.
3. Disclose newly introduced shipped dependencies, external binaries, licensing and privilege boundaries.
4. Keep README/reference docs accurate when implemented behavior changes. Update backlog/iteration owners before their derived BOARD when state changes.
5. High-risk PRs require independent review of device identity, failure behavior, rollback and test evidence. See [PAIRING-WORKFLOW.md](docs/sop/PAIRING-WORKFLOW.md).

Contributions are submitted under the project's [Apache License 2.0](LICENSE) unless a file explicitly states another applicable license. Third-party materials must retain their original license and attribution.

## Good first contributions

Issues and design/test contributions are welcome in minimal Linux image assembly, QEMU matrices, structured Rust system inspection, terminal UX, portable configuration and documentation clarity. Ask before assuming an unimplemented public API is stable.
