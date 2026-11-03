# VITA-001-F — Talos performs system operations through a governed capability broker

As an operator,
I want Talos to inspect and operate the local system safely,
so that high-value system work can be performed conversationally without converting model output directly into unrestricted root shell commands.

Scope:
- Typed capability interface.
- Risk classes: read-only, mutating, privileged, destructive.
- Plan/target display before high-impact execution.
- Explicit confirmation for destructive operations.
- Audit event with intent, resolved target, capability, result and redacted error/output.
- First provider set: filesystem, process/service, storage inspection, network inspection, diagnostics; one controlled mutating capability.

Out of scope:
- Unlimited arbitrary root shell execution.
- Silent autonomous disk formatting/partition changes.

Acceptance:
- Given a read-only inspection request
  When Talos resolves it to a registered capability
  Then it can execute without destructive confirmation and returns structured evidence.
- Given a destructive request
  When Talos prepares execution
  Then the exact target and action are shown
  And execution does not begin until explicit confirmation.
- Given a capability is not registered or policy denies it
  When Talos attempts invocation
  Then the action is rejected and audited.

Decision link: `../../decisions/ADR-0002-governed-capability-broker.md`.
