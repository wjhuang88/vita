# Talos Integration Contract

## Confirmed product requirement
- Talos is the default interaction runtime of the Live OS.
- The implementation should consume Talos Core/runtime components rather than merely exec the existing CLI as the product shell.
- Talos receives structured system context and invokes governed capabilities.

## Integration surfaces to validate in I001
- Which Talos crates/modules can be embedded without CLI/TUI coupling.
- Session lifecycle API.
- Model/provider configuration API.
- Tool/capability registration API.
- Event/stream output contract suitable for a console session.
- Cancellation, timeout and recovery behavior.

## Target project-side components
Names are provisional:
- `interaction-runtime` — session lifecycle and console/UI boundary.
- `capability-broker` — registration, risk classification, confirmation and audit.
- `system-providers` — typed Linux/system operations.
- `config-loader` — removable-media config discovery and secret references.

## Non-goals
- Forking Talos purely to remove its CLI UI.
- Routing every system operation through string shell commands.
- Hiding privilege escalation from the user.
