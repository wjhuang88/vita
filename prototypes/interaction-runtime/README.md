# I001 Talos Runtime Embedding Prototype

This prototype proves one narrow requirement: Vita can host the Talos turn loop through the supported `talos-runtime` SDK without invoking `talos-cli` or `talos-tui`.

It pins Talos to `261602242e49e6f921f5d3e4eac96eebbe92f8b3` and performs a deterministic, network-free turn:

1. construct `RuntimeBuilder`;
2. register a Vita-owned `system_summary` `AgentTool`;
3. allow only the Read nature required by that tool;
4. use a deterministic fake `LanguageModel` to request the tool;
5. read Linux facts directly from `/etc` and `/proc` without shell execution;
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
