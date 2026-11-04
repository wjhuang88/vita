# I004 — Rust native console proof on Talos Runtime

Status: Active
Published baseline: 2026-11-04
Selected work: VITA-001-D (Active); I003 reusable Rust facts already Complete
Risk: host-only offline tool/console fixture; no bootloader, privileges or media writes.

## Objective

Prove a real, multi-turn host console over the supported `talos-runtime` SDK, including typed event rendering, read-only system-summary tool execution, control commands, cancellation and basic recovery from provider errors. Do not couple Vita's session to Talos CLI/TUI.

## Runnable deliverable

- `prototypes/interaction-runtime` has a separate `--console-fixture` mode using a deterministic offline model with **actual Talos SDK** turn, tool and permission handling.
- Console consumes canonical `SessionEvent::TurnEvent` events; renders text and tool name only, not raw tool arguments, tool content or thinking.
- Support `:help`, `:cancel`, `:quit`, Ctrl+C and EOF; queue text entered mid-turn.
- Explicit `demo:error` provider error fixture and `demo:slow` cancellation fixture.
- Preserve normal no-argument I001 run and existing read-only `vita-system-facts`.
- Unit and scripted full SDK integration tests, plus governance and original BIOS/UEFI probe boot CI.

## Acceptance / evidence

```sh
cargo +1.99.0 test --manifest-path prototypes/interaction-runtime/Cargo.toml
cargo +1.99.0 run --manifest-path prototypes/interaction-runtime/Cargo.toml
printf 'inspect\ndemo:error\ninspect again\n:quit\n' | cargo +1.99.0 run --manifest-path prototypes/interaction-runtime/Cargo.toml -- --console-fixture
```

Scripted session must show two `[turn:ok]`, one `[turn:error]`, a Talos `[tool] system_summary`, and `VITA_CONSOLE_EXIT_OK`. Governance and QEMU I001 regression must pass.

**Human-only manual acceptance is NOT part of the automated CI and is tracked separately at [I004-HUMAN-ACCEPTANCE.md](../validation/I004-HUMAN-ACCEPTANCE.md).** Leave unchecked items explicitly pending.

## Scope exclusions

This is not a real Live OS, a privileged broker, a production model provider/configuration path, a PTY-tested default shell or independent recovery shell. Those belong to VITA-001-C/D/E/F/G. No physical USB work.

I002 real Alpine ISO compatibility is still Blocked under [Issue #5](https://github.com/wjhuang88/vita/issues/5) and remains isolated in Draft PR #4.

## Execution evidence

Pending PR #7 final CI results. Retain failed-run evidence and do not mark I004 Complete until the exact final head passes all required automated gates.
