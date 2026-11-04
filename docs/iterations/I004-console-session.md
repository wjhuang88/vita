# I004 — Rust native console proof on Talos Runtime

Status: Complete
Completed: 2026-11-04
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

### GitHub Actions proof

[Run 41789071247](https://github.com/wjhuang88/vita/actions/runs/41789071247), source head `6a65ba884104475a2a08c2db24441953add9b10a`, completed successfully:

- `governance`: **success**.
- `boot-chain-prototype`: **success**, existing disposable-image Ventoy BIOS, UEFI and additional boot-stub ISO regression unchanged.
- `talos-runtime-prototype`: **success**, I003 `vita-system-facts` 4/4 tests, new console parser/rendering 2/2 tests, old `VITA_I001_TALOS_RUNTIME_OK` SDK smoke, and scripted multi-turn run.
- Scripted multi-turn: two `[turn:ok]` turns with registered `[tool] system_summary`, an intervening `[turn:error]` for `demo:error` and successful `VITA_CONSOLE_EXIT_OK`.
- Scripted active-turn cancellation: `demo:slow` → `[interrupt requested]` → `[turn:cancelled]` → successful subsequent `system_summary` and `[turn:ok]` → `VITA_CONSOLE_EXIT_OK`.

### Closure

**I004 host-console slice Complete**; `VITA-001-D` remains Active because a real Live OS default session, production model/config, independent recovery and actual TTY/PTY usability evidence are not yet provided.

[Human acceptance checklist](../validation/I004-HUMAN-ACCEPTANCE.md) is **PENDING** and may be completed later in another conversation. No physical USB or high-risk operation was performed.

The earlier CI failure at [run 41788596295](https://github.com/wjhuang88/vita/actions/runs/41788596295) came from dropping queued prompts when a piped `:quit` arrived during the first turn. The test exposed the bug; the fix drains queued prompts before graceful exit and the later run passed both multi-turn and cancellation tests.
