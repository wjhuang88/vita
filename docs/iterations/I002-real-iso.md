# I002 — Real Linux ISO compatibility slice

Status: Active
Published baseline: 2026-11-04
Selected Ready work: VITA-001-B
Risk profile: high-risk boot/media workflow; VM-only

## Objective

Prove an **unmodified, representative third-party Linux ISO** reaches actual Linux userspace from the same Ventoy-backed medium as Vita's default boot probe. This advances beyond I001's GRUB-only test images.

## Runnable deliverable

- Extend I001's existing safety-checked temporary loop-backed disk harness behind an opt-in `VITA_I002_REAL_ISO=1` mode; ordinary I001 behavior remains unchanged.
- Download Alpine Linux virt 3.24.3 x86_64 from its version-specific official URL, verify upstream SHA-256, and record the observed digest in CI logs/artifacts.
- Require a positive-control direct Alpine ISO boot with serial login and an executed Linux shell command.
- Stage the **same unmodified ISO** under `/iso/` alongside `/iso/vita/vita-live.iso`; select it with Ventoy's existing default-image setting and require the same executed guest command.
- Preserve all target-device guards and capture guest serial logs even on test failure.

## Verification commands

```sh
sh scripts/validate_project_governance.sh .
VITA_I002_REAL_ISO=1 bash scripts/ci/validate_i001_boot.sh
```

Workflow: `.github/workflows/i002-real-iso.yml` and Actions artifact `i002-real-linux-serial`.

Expected marker for both direct-ISO and Ventoy-backed boots: `VITA_I002_ALPINE_GUEST_OK`; full slice marker: `VITA_I002_REAL_ISO_BOOT_OK`.

## Explicit exclusions and residual acceptance

- Not a complete Vita Live OS, Talos console, privilege broker, Secure Boot solution or physical USB installer.
- Does not certify all Linux distributions or images.
- **VITA-001-B will remain Active** until independently validated interactive menu visibility and corrupt/unsupported ISO failure behavior are recorded. A successful auto-selection alone does not close those acceptance conditions.
- Python is CI-only observer code, not an added Vita runtime dependency.

## Evidence state

I002 execution evidence pending the initial GitHub Actions run. Do not infer Linux boot success from file staging or upstream documentation.
