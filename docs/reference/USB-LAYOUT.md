# USB Layout — Initial Contract Under Validation

The final partition layout depends on VITA-001-A. Do not treat partition count/filesystems as final yet.

Logical data layout exposed to the operator:

```text
/iso/                 # user-provided ISO images
/talos/config/        # portable model/session configuration
/talos/skills/        # optional Talos skills/extensions
/talos/workspace/     # writable operator workspace
/talos/logs/          # optional persisted redacted logs/audit, if enabled
```

The boot implementation may use hidden/system partitions. Upgrades should avoid overwriting `/iso/` and `/talos/` data unless the release procedure explicitly states otherwise.
