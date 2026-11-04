# Architecture Decisions

Write an ADR when a technical choice changes a Soft constraint, relies on an Assumption, or a Hard constraint forces a meaningful design choice. Each ADR must state constraints, reasoning, decision and reversal trigger.

Current:
- ADR-0001 — Talos Core as primary interaction runtime.
- ADR-0002 — Governed capability broker for system operations.
- ADR-0003 — Original boot-substrate deferral (superseded by ADR-0004).
- ADR-0004 — Selected Phase 1 hybrid Ventoy substrate + Vita-owned Live OS.
- ADR-0005 — Rust-first Vita-owned system capabilities; explicit Linux/boot/external dependency exceptions.
- ADR-0006 — Alpine Linux/musl first Live base and direct Linux kernel/Rust init prototype (not a release claim).
