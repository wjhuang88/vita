# Privileged Operations

## Capability risk classes
- **R0 Read-only:** inspect files/metadata, processes, hardware, network state.
- **R1 Mutating user-space:** bounded changes in workspace/user configuration.
- **R2 Privileged:** service, package, network or protected-file changes.
- **R3 Destructive:** disk partition/format/write, bootloader install, credential destruction, irreversible delete.

## Execution contract
Each capability declares input schema, resolved targets, privilege requirement, risk class, reversibility, timeout and audit behavior.

For R2/R3:
- show the action plan and resolved target;
- validate preconditions;
- require confirmation according to policy; R3 always requires explicit confirmation;
- execute through a bounded provider;
- record redacted audit evidence;
- surface partial failure and recovery steps.

Do not let model-generated free-form shell text bypass this contract.
