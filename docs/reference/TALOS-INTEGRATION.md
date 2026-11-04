# Talos Integration Contract

Evidence snapshot: 2026-11-03
Talos repository inspected at: `wjhuang88/talos@261602242e49e6f921f5d3e4eac96eebbe92f8b3`

## Confirmed product requirement

- Talos is the default interaction runtime of the Live OS.
- Vita consumes supported Talos runtime/Core surfaces rather than execing the existing CLI as the product shell.
- Talos receives structured system context and invokes governed capabilities.
- A conventional shell remains an explicit recovery/compatibility path, not the normal interaction root.

## Confirmed current Talos embedding boundary

The current Talos architecture already provides the boundary Vita needs:

- `talos-runtime` is the supported embeddable SDK facade.
- Embedders are directed to `RuntimeBuilder` / `RuntimeHandle` rather than `talos-cli`, `talos-tui`, or direct `talos-agent` constructors.
- `RuntimeBuilder::new()` is intentionally minimal; shared built-in tool composition is explicit rather than automatic.
- The runtime facade exposes/re-exports the provider, tool, permission, sandbox, session-event, approval, and durable-session types required by a standalone host.
- Custom `AgentTool` implementations can be registered directly.
- Permission rules and an `ApprovalHandler` are first-class embedding surfaces; unresolved approvals fail closed.
- A custom `SandboxProvider` or the canonical sandbox factory can be supplied.
- Durable session composition exists without CLI/TUI ownership.
- The current Talos repository contains an external SDK fixture that constructs a runtime, registers a custom read-only tool, applies permission rules, submits a turn, consumes completion, and shuts down without using CLI/TUI.

Primary Talos evidence:
- `docs/reference/ARCHITECTURE.md`
- `docs/reference/RUNTIME-SDK-CONTRACT.md`
- `crates/talos-runtime/src/lib.rs`
- `crates/talos-runtime/Cargo.toml`
- `tests/fixtures/runtime-sdk-external/src/main.rs`
- `tests/fixtures/runtime-sdk-external/src/provider.rs`

## Vita composition decision

Vita should initially depend on the supported `talos-runtime` facade, pinned to an exact tested Talos revision during pre-1.0 development.

Do not make Vita depend on:
- `talos-cli` as the interaction implementation;
- `talos-tui` as the OS UI contract;
- direct `talos-agent` constructors unless a missing SDK capability is demonstrated and separately decided.

Target composition:

```text
console / future UI
      |
      v
vita-interaction
      |
      v
talos-runtime (RuntimeBuilder / RuntimeHandle)
      |
      +--> model provider
      +--> Vita system capability tools
      +--> permission / approval boundary
      +--> sandbox boundary where applicable
      +--> durable session support
```

## Capability-broker relationship

Talos already contains a permission pipeline, which Vita should reuse rather than bypass. It is necessary but not by itself the entire Vita capability broker.

Vita's broker remains responsible for system-specific policy that is outside a generic Agent runtime contract, including:
- target-device identity and ambiguity refusal;
- destructive-operation plan/preview semantics;
- privileged helper/service boundary;
- OS-specific capability taxonomy;
- audit records suitable for system administration;
- confirmation UX for high-impact changes;
- recovery behavior when the Agent/runtime is unavailable.

The broker should expose typed `AgentTool` capabilities to Talos and use Talos permission/approval surfaces as one enforcement layer. It must not turn model output into unrestricted root shell strings.

## First structured system capability

The I001 prototype uses a read-only `system_summary` capability implemented as an `AgentTool`. It reads structured/local OS facts directly (for example `/etc/os-release` and `/proc` data) without shelling out and runs under a `ToolNature::Read` allow rule.

The prototype intentionally uses a deterministic fake `LanguageModel`; the goal is to prove the embedding and tool path independently of API keys or network access.

## Model/provider configuration implication

Vita does not need Talos CLI configuration parsing to host the runtime. The OS interaction layer can load Vita's removable-media config, construct/select a provider, and pass it to `RuntimeBuilder::provider`.

Provider secrets remain runtime inputs and must not be baked into the image or repository.

## Session lifecycle implication

The host owns the session lifecycle:
1. resolve Vita medium/config;
2. construct provider and capability registrations;
3. build Talos runtime;
4. submit operator input;
5. consume typed `SessionEvent` output;
6. render to console/UI;
7. perform bounded shutdown or recovery fallback.

This is compatible with Talos being the default OS interaction surface without making Talos PID 1 or replacing Linux init/service management.

## Current status

Confirmed:
- Talos can be embedded below the CLI/TUI boundary through a supported project API.
- Custom structured tools and explicit permission policy are supported.
- Vita can own console/UI and lifecycle while using Talos for the Agent turn loop.
- The Vita Rust embedding prototype compiled and executed under Rust 1.99.0 in the completed I001 GitHub Actions run `41660883827`, emitting `VITA_I001_TALOS_RUNTIME_OK`.

Still to validate in Vita:
- event rendering suitable for a real console/PTY;
- cancellation and crash recovery behavior in a Live OS session;
- privileged broker integration beyond the initial read-only capability;
- final provider/config construction contract.

The repository contains `prototypes/interaction-runtime/` as the first compile/run proof. I001 CI provides reproducible host-level SDK integration evidence; the runnable Live OS session remains future work.
