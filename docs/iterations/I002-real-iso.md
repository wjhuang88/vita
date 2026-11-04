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

## First CI observation and defect correction (2026-11-04)

Run [41768331775](https://github.com/wjhuang88/vita/actions/runs/41768331775) at `c03077eb`:
- governance and syntax checks **passed**;
- I001 disposable Ventoy / BIOS / UEFI / user-probe smoke paths **passed**;
- official unmodified Alpine 3.24.3 ISO SHA-256 verified as `cdc1bc0efc64d8d611bf3733f54868172a545888dfd726f518bf279bcb0ff1af`;
- direct QEMU Alpine positive control **passed**: serial login, `uname -s` returned `Linux`, and `VITA_I002_GUEST_OK` printed from the guest shell;
- Ventoy path reached **ISOLINUX 6.04 `boot:` prompt** in the guest serial log but did not auto-advance during the 180-second observation window.

This is a real boot-menu handoff observation, not a checksum/VM failure. Correction: the guest observer now sends a single carriage return **only when it sees the exact ISOLINUX `boot:` prompt**, modeling an operator accepting the ISO's default boot entry, and records `VITA_I002_ISOLINUX_BOOT_PROMPT_ACCEPTED`. The independently observed image digest is now also hard-pinned after the upstream manifest check.

Retest remains required; these findings do **not** establish successful Ventoy-backed Linux boot until a later run reaches the executed guest command marker.

## Second CI observation and firmware-specific follow-up (2026-11-04)

Run [41769007122](https://github.com/wjhuang88/vita/actions/runs/41769007122):
- Alpine direct BIOS control passed again, including login and executed command.
- Ventoy's BIOS path visibly reached the ISOLINUX `boot:` prompt. The observer sent a single Enter only after seeing this prompt (marker: `VITA_I002_ISOLINUX_BOOT_PROMPT_ACCEPTED`).
- Even after Enter, the serial stream remained at ISOLINUX and no Linux guest command proof appeared. This is an **unresolved BIOS compatibility outcome**; extending the timeout or treating the prompt as success would be misleading.

Vita's primary firmware target is x86_64 UEFI. The next targeted experiment adds a positive-control **direct OVMF boot of the original Alpine ISO**, then a Ventoy-backed OVMF boot of that same ISO. The test emits the same in-guest `uname -s` and `VITA_I002_GUEST_OK` proof. BIOS compatibility remains explicitly open for future diagnosis and is not silently counted as passing.
