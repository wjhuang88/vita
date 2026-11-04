# I002 — Real Linux ISO compatibility slice

Status: Partial — real ISO negative compatibility proof; acceptance blocked by #5
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
- **VITA-001-B is Blocked** by the Ventoy/Alpine guest boot compatibility failure, tracked in [Issue #5](https://github.com/wjhuang88/vita/issues/5). Visible menu selection and invalid-image failure handling also remain unverified.
- Python is CI-only observer code, not an added Vita runtime dependency.

## Evidence state

Initial evidence was recorded in four subsequent CI experiments below. The full I002 acceptance was **not** met.

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

## Third CI observation: UEFI loader reached but Linux guest did not (2026-11-04)

Run [41769831996](https://github.com/wjhuang88/vita/actions/runs/41769831996):
- The **same unmodified Alpine ISO** boots directly under OVMF x86_64 UEFI to serial login, a root shell, `uname -s` = `Linux`, and `VITA_I002_GUEST_OK`.
- On the Ventoy-backed UEFI disk the ISO's GRUB reached `Booting 'Linux virt'` but produced no subsequent Linux guest serial login/command marker before the 210-second bound.
- I001 probe BIOS/UEFI and runtime CI regression checks remain green.
- Both BIOS ISOLINUX and UEFI GRUB observations are tracked in [Issue #5](https://github.com/wjhuang88/vita/issues/5), rather than being treated as supported behavior.

**Bounded fourth experiment:** Ventoy officially documents a GRUB2 alternate mode for problematic Linux ISOs and an `_VTGRUB2.iso` filename identifier. Keep the official Alpine ISO bytes unchanged (verify its copied SHA-256 equals the original) and select the specially named copy through Ventoy. This is a boot-mode experiment, not a product decision to rename arbitrary user files or to claim generic Linux compatibility.

Primary reference: https://www.ventoy.net/en/doc_name_identifier.html ; https://www.ventoy.net/en/doc_grub2boot.html .

If the documented fallback does not boot a real guest, treat this fixture as a demonstrated compatibility blocker; do not close VITA-001-B or promote I002 to a successful real-ISO deliverable.

## Fourth CI observation and I002 partial closeout (2026-11-04)

Run [41770647347](https://github.com/wjhuang88/vita/actions/runs/41770647347), commit `360731f46c367b2cb04275be2ca6d0d0314b437d`:
- Governance validation: **success**; independent I001 baseline workflow [41770647291](https://github.com/wjhuang88/vita/actions/runs/41770647291): **success** (governance, Talos embedding and probe boot).
- Official Alpine 3.24.3 SHA-256 still pinned: `cdc1bc0efc64d8d611bf3733f54868172a545888dfd726f518bf279bcb0ff1af`.
- Direct QEMU Alpine BIOS and OVMF/UEFI: **success**, both logged into guest and executed `uname -s` (`Linux`) and `VITA_I002_GUEST_OK`.
- Ventoy / OVMF with a byte-identical Alpine ISO copy renamed `_VTGRUB2.iso`: **failure**. Guest reached GNU GRUB 2.04 and `Booting 'Linux virt'`, then no Alpine login or executed marker within 210 seconds.
- Evidence archive: [i002-real-linux-serial](https://github.com/wjhuang88/vita/actions/runs/41770647347/artifacts/13294702714), including direct and Ventoy guest serial logs. This is a real system boot compatibility defect, **not** a passing result.

**Disposition: Partial.** The test harness, source integrity checks, positive controls, exact observed handoff symptoms and bounded failure evidence are delivered. The core acceptance of booting a representative Linux distribution ISO from Ventoy is **not** met. VITA-001-B remains Blocked with [Issue #5](https://github.com/wjhuang88/vita/issues/5) as the diagnosis owner. Existing PR #4 remains Draft/unmerged; do not weaken CI or mislabel this as a full compatible distro boot. Future attempts should investigate the VM/Ventoy-to-kernel ISO media handoff based on concrete boot logs, not arbitrary menu mode retries.
