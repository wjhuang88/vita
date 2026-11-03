# Pairing Workflow for High-risk Changes

Use for bootloader, partition/image writer, privilege boundary, secret handling and release-image changes.

1. Author records intended change, risk surface and validation plan.
2. Implement smallest safe slice.
3. Reviewer independently checks target identity, failure behavior, rollback/recovery and test evidence.
4. No physical-media/release step before VM/static/unit gates pass.
5. Resolve review findings and rerun affected checks.
