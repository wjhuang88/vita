# Boot Strategy Research — I001 / VITA-001-A

Evidence snapshot: 2026-11-03

## Question

Which boot architecture best supports both arbitrary user ISO images and a project-owned Vita Live OS while keeping upgrades, recovery, licensing, and future product control manageable?

## Evidence

### Ventoy as the compatibility substrate

Confirmed from current upstream documentation/release material:

- Ventoy directly boots ISO/WIM/IMG/VHD(x)/EFI images and supports Legacy BIOS plus multiple UEFI architectures.
- The first data partition is ordinary storage; users copy image files rather than rebuilding the medium.
- Updating Ventoy preserves files in the first partition.
- `VTOY_DEFAULT_SEARCH_ROOT` can restrict image discovery to a Vita-owned `/iso` tree.
- `VTOY_DEFAULT_IMAGE` plus menu timeout can make the Vita Live image the default path while still exposing user images.
- `VTOY_SECONDARY_BOOT_MENU=0` removes the otherwise-default secondary mode menu from Vita's unattended default boot path.
- Menu aliases can present a product-facing name instead of a raw ISO filename.
- Ventoy reserves its first two partitions; optional reserved space can be created at the end of the disk for extra partitions.
- Ventoy is GPLv3+; Vita release packaging must preserve the applicable license/corresponding-source obligations.
- Secure Boot support exists, but upstream explicitly documents firmware/key-enrollment caveats. Vita must not equate this with a complete signed-boot-chain guarantee.

Observed latest upstream release on this evidence date: Ventoy 1.1.18. The Linux archive published SHA-256 is `d86ff9de63d94c8b8f1f6b14d23c55af4b88e8207bf2893890892a6a97b1ad54`.

Primary references:
- https://www.ventoy.net/en/doc_start.html
- https://www.ventoy.net/en/doc_disk_layout.html
- https://www.ventoy.net/en/plugin_control.html
- https://www.ventoy.net/en/doc_secondary_boot_menu.html
- https://www.ventoy.net/en/doc_search_path.html
- https://www.ventoy.net/en/plugin_menualias.html
- https://www.ventoy.net/en/doc_secure.html
- https://github.com/ventoy/Ventoy/releases
- https://github.com/ventoy/Ventoy/blob/master/License/license-ventoy.txt

### Pure GRUB2 ownership

GNU GRUB 2.14 can read images through loopback and can chain-load on BIOS/EFI platforms. However, the GRUB manual explicitly notes that the operating system inside an image still has to discover its root; whether a distro can boot from loopback depends on that distro/initramfs and loader behavior. Several loader classes cannot be made generic through loopback alone.

Therefore a Vita-owned GRUB menu would give maximum bootloader ownership but would also make Vita responsible for a growing compatibility database and distro-specific boot recipes. That is the wrong initial optimization because ISO compatibility is a product requirement but not Vita's differentiating capability.

Primary references:
- https://www.gnu.org/software/grub/manual/grub/html_node/Loopback-booting.html
- https://www.gnu.org/software/grub/manual/grub/html_node/Chain_002dloading.html
- https://www.gnu.org/software/grub/manual/grub/grub.html

## Option comparison

| Option | ISO compatibility burden | Vita product ownership | Upgrade/data safety | Initial complexity | Decision |
| --- | --- | --- | --- | --- | --- |
| Pure Ventoy product shell | Low | Low; product risks becoming a themed Ventoy | Good | Low | Reject |
| Pure project-owned GRUB2 | High | High | Project-owned | High and grows with distro matrix | Reject for Phase 1 |
| Hybrid: Ventoy substrate + Vita-owned Live OS/runtime | Low | High above boot substrate | Good; Live ISO can be replaced independently | Moderate | **Select for Phase 1** |
| Fully custom boot substrate | Very high | Maximum | Project-owned | Highest | Reject until a concrete need appears |

## Recommended Phase 1 architecture

```text
Firmware
  -> unmodified/pinned Ventoy substrate
      -> /iso/vita/vita-live.iso       (default)
      -> /iso/... user ISO images      (discovered dynamically)

Vita Live ISO
  -> Linux kernel / base system
  -> Vita system services
  -> Talos-runtime-based interaction runtime (default session)
  -> governed system capabilities
  -> explicit recovery shell

Ventoy data partition
  -> /iso/      boot images only
  -> /talos/    portable config, skills, workspace, logs
  -> /ventoy/   product boot-menu configuration
```

This is a substrate boundary, not a product dependency inversion: Ventoy owns image compatibility and early boot; Vita owns the Live OS, interactive session, system policy, configuration, user experience, and release lifecycle above that boundary.

## Initial Ventoy integration rules

1. Consume a pinned upstream Ventoy release without source modifications in Phase 1.
2. Keep user images under `/iso`; keep Talos/Vita runtime data outside that search root.
3. Put Vita Live ISO at `/iso/vita/vita-live.iso`, configure it as the default image, and disable the secondary boot-mode menu on the default unattended path.
4. Do not use the image-list whitelist for normal operation because users must be able to add ISO files without rebuilding configuration.
5. Do not depend on a transient Linux device node to locate the portable data partition. The Live OS must identify the Vita medium using a filesystem label and/or project marker with ambiguity checks.
6. Treat Secure Boot as a separate validation track. No "Secure Boot supported" product claim until Vita's complete chain and enrollment behavior are tested and documented.
7. Releases embedding Ventoy must carry third-party license/source compliance material; modifications to Ventoy require a separate decision and review.

## Executed validation

GitHub Actions run `41659336430`, job `boot-chain-prototype`, executed on a GitHub-hosted Ubuntu 26.04.1 runner at Vita commit `68c25efd89aa12ca0a611fbce4ea6605907cbebd`.

The test is intentionally disposable and target-guarded:
- creates a 768 MiB temporary raw file;
- attaches only a `/dev/loopN` device whose kernel backing file is re-verified against that exact raw file;
- downloads Ventoy 1.1.18 and verifies SHA-256 before extraction/use;
- installs Ventoy only after the loop/backing-file safety checks pass;
- generates deterministic GRUB probe ISOs;
- stages Vita configuration plus both Vita and user ISO fixtures;
- boots the raw disk under QEMU and requires serial markers.

Observed evidence:

```text
Install Ventoy to /dev/loop0 successfully finished.
VITA_BOOT_PROBE_OK mode=bios marker=VITA_BOOT_OK
VITA_BOOT_PROBE_OK mode=uefi marker=VITA_BOOT_OK
VITA_BOOT_PROBE_OK mode=bios marker=VITA_USER_ISO_BOOT_OK
VITA_I001_BOOT_CHAIN_OK ventoy=1.1.18
```

This proves the minimum Phase 1 chain:
- Ventoy can be safely staged onto a verified loop-backed disposable disk in CI;
- Vita's configured default image automatically reaches the project-owned ISO under legacy BIOS;
- the same default image reaches the project-owned ISO under x86_64 UEFI/OVMF;
- a second user ISO in `/iso` remains independently bootable after changing the selected default.

## Remaining validation outside VITA-001-A

Not yet claimed:
- physical USB/firmware compatibility;
- Secure Boot end-to-end behavior and signing/enrollment policy;
- representative third-party Linux distribution compatibility beyond the deterministic probe ISO;
- the final Vita Live OS kernel/initramfs/rootfs rather than the I001 boot stub;
- a release-grade firmware/device matrix.

Those belong to VITA-001-B/C/H and later release-hardening work. The host environment used to author the first pass did not include the boot toolchain; CI now provides the authoritative VM proof instead of inferring success from documentation.
