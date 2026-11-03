# VITA-001-D — Talos Core becomes the primary interactive OS session

As an operator,
I want the system to enter a Talos Core-driven interaction environment by default,
so that system intent, inspection and operations are mediated by an Agent-native interface rather than a conventional shell-first workflow.

Scope:
- Build a dedicated OS interaction runtime on Talos Core APIs/components.
- Own session lifecycle, prompt/input loop, context, tool/capability invocation and result rendering.
- Obtain system context from structured providers.
- Provide explicit escape/recovery path to a conventional shell.

Out of scope:
- Treating the existing Talos CLI binary as the final shell implementation.
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

Decision links: `../../decisions/ADR-0001-talos-primary-interaction-runtime.md`, `../../decisions/ADR-0002-governed-capability-broker.md`.
Decision constraint: Talos is the primary interaction runtime; privileged actions pass through the broker.
Minimum validation: PTY/console integration test + Live VM smoke test.
