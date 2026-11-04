# ADR-0005 — Rust-first Vita-owned system capabilities

Status: Accepted for future implementation slices
Date: 2026-11-04

## Context

Talos Core and its supported embedding facade are Rust-based. Vita's differentiator is the Agent-native system interaction model, not a new kernel or a collection of command wrappers. Requiring Python/Node/Java sidecars or shell-command pipelines for ordinary OS operations would add runtime, packaging and privilege complexity.

Rust-first is a concrete product engineering constraint, not a claim that all firmware, boot code, Linux components or drivers must be written in Rust.

## Constraint classification

| Constraint | Type | Decision |
| --- | --- | --- |
| Vita-owned interaction/runtime, system providers, capability broker, config and auditing use Rust as primary implementation | Hard | Reuse Talos Runtime and typed Rust contracts |
| Talos is the primary OS interaction layer, not a CLI wrapper | Hard | Keep ADR-0001 |
| Destructive and privileged actions pass through broker, confirmation and audit | Hard | Keep ADR-0002 |
| Extra application-language runtimes and shell-process chains are minimized in the shipped Live OS | Soft | Require explicit exception review |
| Linux kernel, minimal init/userspace and mature boot compatibility software may remain | Soft | Decide per boundary based on safety and maintenance |
| Every Linux system operation has a sufficiently mature safe Rust implementation | Assumption | Unproven: evaluate by operation |

## Decision

1. **Own Vita's interaction and system layer in Rust.** The host process embeds Talos Runtime; its session lifecycle, system context adapters, typed capability broker, config loader and audit path remain Vita-owned Rust modules.
2. **Prefer Linux-native interfaces via maintained Rust crates.** Evaluate /proc, /sys, netlink, file descriptors, sockets, namespaces, mount/ioctl and similar kernel interfaces, rather than invoking and parsing human-facing output from ps, lsblk, ip, mount, systemctl or general shells. Reuse existing Talos crates before adding new packages.
3. **Provide a constrained exception mechanism.** Where a native API is immature or unsafe, use reviewed typed IPC or a specific allowlisted subprocess. Define structured arguments (never model-generated shell code), exact binary, privilege context, stable parsing contract, target validation, timeout, bounded output, redaction and audit.
4. **Default to no extra application-language runtime.** Normal Vita/Talos interaction must not require Python, Node.js, JVM or other interpreter/VM unless the exception is justified by demonstrable reliability, footprint, packaging and recovery benefits. Build-only tooling does not count as shipped runtime dependency.
5. **Maintain privilege isolation independently of language.** Rust memory safety does not grant authorization or prevent wrong-disk writes. The broker and least-privilege helpers are mandatory even for Rust-only providers.
6. **Do not rewrite mature safety-critical software merely for purity.** Linux kernel, firmware/bootloader, drivers, cryptography, filesystem tools and minimal recovery utilities may remain where replacing them raises risk or maintenance cost. Native FFI libraries and external daemons must still be inventoried.
7. **Avoid premature platform rewriting.** This ADR does not mandate a Rust PID 1, a Rust kernel, universal kernel portability, handwritten unsafe blocks or reimplementation of an established ISO boot compatibility engine.

## Provider selection order

1. Rust standard library or existing Talos libraries.
2. Maintained Rust crate using stable Linux-native APIs.
3. Typed IPC into a mature service or vetted FFI library with explicit lifecycle/privilege boundaries.
4. Narrow, allowlisted subprocess adapter for an exceptional capability.
5. Explicit shell for operator recovery or build tooling only; never unrestricted language-model-to-root execution.

This is a review order, not a categorical ban on mature non-Rust components.

## Verification gates

- Record each provider's API, input/output schema, risk class, target resolution, privilege boundary, dependency footprint, failure semantics, tests and whether it starts processes.
- Implement representative read-only system inspection through a native Rust provider without shell execution and test allow/deny/confirm/audit through the broker.
- Publish a shipped runtime dependency inventory for the Live OS: binaries, language runtimes, libraries, licenses and any build-only exceptions.
- Run offline/missing-service failure-path and recovery tests in a VM; maintain existing strict physical-media gates.
- Establish numeric image-size and performance budgets only after a measured prototype baseline exists.
- Record and review every runtime-level exception in the affected story/decision.

## Consequences and story ownership

- VITA-001-C must select a minimal Live OS base and document shipped dependencies.
- VITA-001-D must embed Talos Runtime in the project's Rust session process without making Talos CLI or another language runtime the primary host.
- VITA-001-F must prefer typed native providers and make fallback adapters exceptional and governed.
- Proven boot architecture and Linux compatibility remain separate architectural boundaries.

See ../reference/RUST-NATIVE-SYSTEM-STACK.md for candidate interfaces and verification guidance.

## Reversal trigger

Reconsider a particular provider or base component if a native Rust implementation is less secure, less reliable or materially harder to maintain than a reviewed alternative. Such an exception does not repeal Rust-first ownership of Vita's core.
