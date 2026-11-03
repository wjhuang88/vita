# Evolution Log

This file records reusable lessons discovered through failures, corrections and unexpectedly effective remedies. Mandatory current rules belong in `AGENTS.md` or the owning SOP.

## Index
| ID | Area | Trigger | Promoted to |
| --- | --- | --- | --- |
| EV-0001 | Architecture | Talos positioning clarified during initialization | `AGENTS.md`, ADR-0001 |
| EV-0002 | Boot validation | I001 exposed hidden unattended-boot and clean-runner dependencies | `docs/sop/BOOT-MEDIA-VALIDATION.md`, I001 CI harness |
| EV-0003 | Media safety | Ventoy installer proof required destructive semantics without physical-device risk | `docs/sop/BOOT-MEDIA-VALIDATION.md`, I001 CI harness |

## EV-0001 — Talos must not be reduced to a bundled CLI
- **Trigger:** Initial project framing described “a system with an integrated Talos”.
- **Symptom:** The architecture could drift toward a conventional Linux distribution that merely launches `talos cli`.
- **Cause:** “Integrated tool” language did not express the intended interaction boundary.
- **Remedy:** Define Talos Core as the primary OS interaction runtime and keep conventional shell as compatibility/recovery.
- **Prevention:** Enforce the constraint in `AGENTS.md`, architecture reference and ADR-0001; reject stories that only wrap the existing CLI.
- **Promotion target:** `AGENTS.md`, `docs/decisions/ADR-0001-talos-primary-interaction-runtime.md`.

## EV-0002 — Boot capability is not proven until the unattended handoff reaches a guest marker
- **Trigger:** I001 boot CI initially failed before and during the first real Ventoy handoff.
- **Symptom:** `grub-mkrescue` failed on a clean runner because `mformat` was absent; after that was fixed, Ventoy reached its menu but did not enter the Vita ISO because the secondary boot menu is enabled by default.
- **Cause:** Documentation-level feature support and package names did not expose all runtime dependencies or interactive menus in the complete path.
- **Remedy:** Install `mtools` explicitly for GRUB rescue-image generation, disable Ventoy's secondary menu on Vita's unattended default path, and require a marker emitted from inside the selected ISO rather than treating QEMU/Ventoy startup as success.
- **Prevention:** Keep the end-to-end BIOS/UEFI marker checks in `scripts/ci/validate_i001_boot.sh` and require equivalent guest-reachable evidence for future boot-path changes.
- **Promotion target:** `docs/sop/BOOT-MEDIA-VALIDATION.md`, `scripts/ci/validate_i001_boot.sh`.

## EV-0003 — Destructive installer tests need an independently verified disposable target
- **Trigger:** VITA-001-A needed to run the real Ventoy installer in CI.
- **Symptom:** A normal installer accepts a block-device path and correctly warns that all data will be lost; a typo or device-discovery mistake could therefore make a boot test dangerous on a less isolated host.
- **Cause:** The installer's own confirmation is not a sufficient identity proof for Vita's high-risk media workflow.
- **Remedy:** Create a temporary raw file, attach it to `/dev/loopN`, reject partitions/non-loop devices, and verify the kernel-reported loop backing file exactly matches the newly created raw image before invoking the installer.
- **Prevention:** Never let automated destructive-media validation accept an arbitrary external block-device argument. Physical targets use the separate explicit target-identity procedure.
- **Promotion target:** `docs/sop/BOOT-MEDIA-VALIDATION.md`, `scripts/ci/validate_i001_boot.sh`.
