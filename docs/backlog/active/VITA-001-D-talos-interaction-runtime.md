# VITA-001-D — Talos Core becomes the primary interactive OS session

Status: Active (I004 — offline native console)

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

The I001 prototype registered a Vita-owned read-only `system_summary` capability and ran a deterministic network-free turn through Talos Runtime. I003 now extracts its Linux facts into `crates/vita-system-facts`; all 4 crate tests and the actual Talos tool execution passed in [CI run 41772155509](https://github.com/wjhuang88/vita/actions/runs/41772155509). Full console and Live OS integration remain unimplemented.

Decision links: `../../decisions/ADR-0001-talos-primary-interaction-runtime.md`, `../../decisions/ADR-0002-governed-capability-broker.md`.
Decision constraint: Talos is the primary interaction runtime; privileged actions pass through the broker.
Minimum validation: host runtime compile/run proof + PTY/console integration test + Live VM smoke test.

## I003 native-facts slice

- Extract Linux-only, bounded, read-only system facts into `crates/vita-system-facts` with no additional language or library runtime.
- Cover missing or malformed `/proc` fields, size boundaries and deterministic fixtures in Rust unit tests.
- Feed the typed facts into the existing Talos `system_summary` tool without changing its Read permission classification or introducing shell execution.
- The I003 test/embedding gate is **not the full Story acceptance**: real console/PTY and Live OS boot-to-session remain open, so do not close VITA-001-D merely from host tests.

## I004 host console slice

- Add a standalone `--console-fixture` Rust console over `talos-runtime`, preserving the I001 one-shot proof.
- Stream canonical structured turn events, queue subsequent input, support `:cancel`, Ctrl+C, `:quit`, EOF, bounded turns and controlled SDK shutdown.
- Deliberate `demo:error` and `demo:slow` fixture cases exercise error recovery and cancellation; the only registered system tool remains `system_summary` with explicit Read classification.
- Unit and scripted CI tests are required; do not claim a production model, a Live OS default session, or recovery before VITA-001-C/D/G gates.
- Real TTY/PTY, review and other deferred human checks: [I004 manual acceptance](../../validation/I004-HUMAN-ACCEPTANCE.md). This Story stays Active.
