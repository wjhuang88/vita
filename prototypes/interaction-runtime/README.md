# I001 Talos Runtime Embedding Prototype

This prototype proves one narrow requirement: Vita can host the Talos turn loop through the supported `talos-runtime` SDK without invoking `talos-cli` or `talos-tui`.

It pins Talos to `261602242e49e6f921f5d3e4eac96eebbe92f8b3` and performs a deterministic, network-free turn:

1. construct `RuntimeBuilder`;
2. register a Vita-owned `system_summary` `AgentTool`;
3. allow only the Read nature required by that tool;
4. use a deterministic fake `LanguageModel` to request the tool;
5. read typed, bounded Linux facts from the reusable `vita-system-facts` crate (`/etc` and `/proc`, including optional uptime and total RAM) without shell execution;
6. assert the turn succeeds and the structured capability actually executed;
7. shut the runtime down normally.

Expected terminal marker:

```text
VITA_I001_TALOS_RUNTIME_OK
```

Run from this directory:

```sh
cargo run
```

The embedding prototype compiled and ran successfully in [GitHub Actions run 41660883827](https://github.com/wjhuang88/vita/actions/runs/41660883827) using Rust 1.99.0. The command above is equivalent to the tested CI entrypoint.

This prototype is not the final Vita interaction UI, capability broker, provider configuration, or privileged execution path.

## I003 test evidence

The read-only provider is now extracted into `crates/vita-system-facts`, with 4 tests covering parsing and missing/invalid/oversized fixtures. [CI run 41772155509](https://github.com/wjhuang88/vita/actions/runs/41772155509) compiled and executed this Rust prototype and emitted `VITA_I001_TALOS_RUNTIME_OK` under the Read permission rule.

## I004 offline Rust console fixture

Run the host-side **deterministic Talos Runtime console**, not a production model:

```sh
cargo +1.99.0 run --manifest-path prototypes/interaction-runtime/Cargo.toml -- --console-fixture
```

Enter `inspect` to exercise the registered read-only `system_summary` Tool through Talos, stream its response, and continue to another prompt. `:help`, `:cancel`, `:quit` and Ctrl+C are available. `demo:error` deliberately fails one fixture turn to test recovery; `demo:slow` lets a human test interrupting an active turn. Input typed during a turn is queued; EOF finishes the active turn first.

The session does not connect to an LLM, write disks, expose a root shell, boot a Vita ISO or provide durable recovery. The [manual acceptance checklist](../../docs/validation/I004-HUMAN-ACCEPTANCE.md) records items requiring a real TTY/PTY or human review.
