# I001 — Ventoy / QEMU Boot Probe

The selected Phase 1 design uses a pinned, unmodified Ventoy boot compatibility substrate, with Vita owning the future Live OS and its default boot entry.

- Product boot configuration: [ventoy/ventoy.json](ventoy/ventoy.json)
- Default configured ISO path: `/iso/vita/vita-live.iso`
- User ISO discovery root: `/iso`
- Portable Talos/Vita data: `/talos` (outside the ISO search root)

## Verified I001 proof

CI installed Ventoy 1.1.18 on a verified temporary loop-backed raw image, copied a Vita GRUB probe ISO and a second probe ISO, and booted via QEMU:

```text
VITA_BOOT_PROBE_OK mode=bios marker=VITA_BOOT_OK
VITA_BOOT_PROBE_OK mode=uefi marker=VITA_BOOT_OK
VITA_BOOT_PROBE_OK mode=bios marker=VITA_USER_ISO_BOOT_OK
VITA_I001_BOOT_CHAIN_OK ventoy=1.1.18
```

Evidence: [GitHub Actions run 41660883827](https://github.com/wjhuang88/vita/actions/runs/41660883827), [archived VITA-001-A](../../docs/backlog/archive/VITA-001-A-boot-strategy-spike.md).

Run the test script on a **disposable Linux VM/CI runner** only:

```sh
bash scripts/ci/validate_i001_boot.sh
```

The script must keep its hard-coded temporary raw-file/loop-device identity checks. Do not modify it to accept a physical disk path.

**Not proven by this test:** Linux rootfs, interactive Vita session, third-party distro ISO compatibility, Secure Boot or physical USB hardware. Those are later stories.
