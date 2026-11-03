# Release

A release is the distribution of a bootable image/artifact, not merely a Git tag.

Required before release:
- Selected stories/iteration are complete with evidence.
- Boot/media validation matrix passes for required modes.
- Talos default interaction path is reachable.
- Recovery shell is reachable.
- Secrets are absent from artifacts/logs/examples.
- Destructive capability confirmation/audit tests pass.
- Artifact SHA-256 is published.
- Upgrade/write instructions clearly identify the target device and data-preservation behavior.
- Release notes list known firmware/ISO compatibility limits.
