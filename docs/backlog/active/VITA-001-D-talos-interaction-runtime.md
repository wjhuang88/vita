# VITA-001-D — Talos Core becomes the primary interactive OS session

Status: Ready

As an operator,
I want the system to enter a Talos Core-driven interaction environment by default,
so that system intent, inspection and operations are mediated by an Agent-native interface rather than a conventional shell-first workflow.

Scope:
- Build a dedicated OS interaction runtime on the supported `talos-runtime` embedding API.
- Own session lifecycle, prompt/input loop, context, tool/capability invocation and result rendering in Vita.
- Obtain system context from structured providers/tools.
- Reuse Talos permission/approval/sandbox surfaces as enforcement layers inside the broader Vita capability-broker boundary.
- Provide explicit escape/recovery path to a conventional shell.

Out of scope:
- Treating the existing Talos CLI binary as the final shell implementation.
- Depending on `talos-tui` as Vita's OS interaction contract.
- Directly coupling Vita to unsupported `talos-agent` constructors unless a missing SDK capability is first demonstrated and decided.
- Replacing the Linux kernel/init system with Talos.
- Giving the model unrestricted root command execution.

Acceptance:
- Given the Live OS reaches the interactive target
  When the operator session starts
  Then Talos interaction starts without first invoking it from bash/zsh/etc.
  And ordinary inspection requests are fulfilled through structured system capabilities.
- Given the Talos interaction runtime crashes or cannot initialize
  When the recovery path is selected
  Then a conventional local shell remains available for repair.

## I001 integration evidence

Talos repository inspected at `261602242e49e6f921f5d3e4eac96eebbe92f8b3`.

Confirmed:
- `talos-runtime` is the supported embeddable SDK facade.
- `RuntimeBuilder` / `RuntimeHandle` own the intended embedding path.
- custom `AgentTool`, `LanguageModel`, permission rules, approval handling, sandboxing and durable sessions are available without CLI/TUI ownership.
- Talos itself ships an external SDK fixture proving this composition shape.

Vita proof source: `../../../prototypes/interaction-runtime/`.

The prototype registers a Vita-owned read-only `system_summary` capability and runs a deterministic network-free turn through Talos Runtime. CI execution evidence is required before the I001 proof is considered passed.

Decision links: `../../decisions/ADR-0001-talos-primary-interaction-runtime.md`, `../../decisions/ADR-0002-governed-capability-broker.md`.
Decision constraint: Talos is the primary interaction runtime; privileged actions pass through the broker.
Minimum validation: host runtime compile/run proof + PTY/console integration test + Live VM smoke test.
