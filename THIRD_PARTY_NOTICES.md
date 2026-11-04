# Third-party software and redistribution

Vita-authored source code is licensed under [Apache License 2.0](LICENSE), but this does **not** relicense external projects or imply that a future bootable Vita image can be distributed under a single license.

## Present source repository

| Project | Role | Source / licensing |
| --- | --- | --- |
| [Ventoy](https://www.ventoy.net/) | Selected Phase 1 early-boot and ISO compatibility substrate | Ventoy authors' modules/tools: **GPLv3+**; Ventoy includes multiple third-party components under their own terms. [Official license inventory](https://www.ventoy.net/en/doc_license.html) and [upstream license directory](https://github.com/ventoy/Ventoy/tree/master/License). |
| [Talos](https://github.com/wjhuang88/talos) | Rust SDK/runtime dependency of the proof | Confirm the license and any transitive license obligations from the pinned upstream Talos revision; not relicensed by Vita. |
| Linux kernel / minimal userspace / GRUB / QEMU and crates | Future operating system or development-test components | Each component retains its own license. Development-time dependencies are not automatically redistributed by this source repository. |

**Current repository state:** Ventoy's executable release is downloaded and hash-checked by the CI smoke script at test time; the repository does **not** vendor a Ventoy binary or publish a bootable media release today. The deterministic probe ISO build uses separately installed development tools.

## Required before a binary / USB image release

- Inventory every bundled binary, library, firmware/driver, source component, notice and license (including transitive components).
- Preserve copyright, license texts, required notices and attribution for all redistributed components.
- Review applicable GPLv3+ corresponding-source and distribution obligations for any bundled Ventoy materials, alongside the distinct terms of its constituent third-party projects.
- Document where and how recipients obtain the corresponding source when required.
- Confirm whether any combined/linked components impose further conditions beyond the license for Vita-authored files.
- Ensure product release artifacts ship their own complete license and attribution package, verified against the actual contents.

This is a technical compliance checklist, not a legal conclusion about a future distribution configuration. License obligations must be re-evaluated when the image composition is defined.
