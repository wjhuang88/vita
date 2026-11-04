# System Architecture

## Product shape

```text
Firmware (UEFI / optional BIOS)
        |
        v
Portable Boot Layer
   |                    \
   |                     +--> ISO discovery / chain or loop boot
   v
Project Live OS
   |
   +--> Linux kernel + init + base services
   |
   +--> Talos OS Interaction Runtime   <--- default interactive session
          |
          +--> Talos Core: model/session/planning/tool orchestration
          |
          +--> System Context Providers
          |
          +--> Governed Capability Broker
                    |
                    +--> filesystem provider
                    +--> process/service provider
                    +--> storage provider
                    +--> network provider
                    +--> diagnostics/hardware provider
                    +--> constrained shell compatibility adapter
          |
          +--> Audit / confirmation / policy

Explicit fallback: conventional recovery shell
```

## Architectural boundary
Talos does not replace the Linux kernel or necessarily PID 1. It replaces the **default human interaction model** that would otherwise be an interactive shell or desktop launcher. The session runtime owns intent input, context, execution planning, capability invocation and result presentation.

## Rust-first system stack

Vita-owned interaction runtime, system context providers, governed broker, config and audit are Rust-first. Prefer native Linux interfaces through maintained Rust crates over parsing shell/CLI output. Do not add Python, Node.js or JVM as routine Vita runtime requirements without an explicit exception.

This does not imply replacing Linux, firmware, the boot substrate, required native libraries or recovery tools. A mature external service or tightly controlled subprocess may be safer for particular privileged operations; the same target-identity, least-privilege and audit rules apply. See [ADR-0005](../decisions/ADR-0005-rust-first-system-capabilities.md) and [Rust stack guide](RUST-NATIVE-SYSTEM-STACK.md).

## Why not just Talos CLI
A CLI launched from bash leaves bash as the primary environment and tends to inherit CLI-era assumptions. The project instead needs a reusable runtime boundary built on Talos Core so boot/session lifecycle, system context, privileges, policy and audit are first-class.

## Boot boundary
The exact ISO boot substrate remains unresolved pending VITA-001-A. The product requirement is capability-based, not implementation-based: stored ISO boot + project Live OS + upgrade path that preserves user data where feasible.
