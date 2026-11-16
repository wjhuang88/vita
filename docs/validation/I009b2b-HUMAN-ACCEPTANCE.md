# I009b2b — VM benchmark vs human/product validation (PENDING)

I009b2b is **Complete only as a VM engineering measurement slice** after [PR #24](https://github.com/wjhuang88/vita/pull/24), merged as `8f88e3f2872d9010f25362368c01af91818f98dd`. Its [final 20/20 exact-head CI](https://github.com/wjhuang88/vita/actions/runs/43797290307) and independently executed Copilot code review demonstrate **no product-level acceptance**. Owner: VITA-001-C / [I009b2 #21](https://github.com/wjhuang88/vita/issues/21). Fill in human reviewer, specific hardware/firmware, signed commits and redacted evidence for each item.

- [ ] Review the statistical significance, warm/cold cache effects and real hardware boot-to-Talos measurements on representative BIOS and UEFI machines. Three QEMU/TCG repeats per source are engineering evidence only.
- [ ] Measure **actual guest post-Talos idle/peak** RAM, daemon/service inventory, kernel/module footprint and failure/resource-exhaustion behavior, and do not substitute the host QEMU `VmHWM` or guest *pre-session* `MemAvailable`.
- [ ] Validate source selection and signed boot trust beyond fixed QEMU optical `/dev/sr0`, including device identity, tampered media, physical disconnect and Secure Boot.
- [ ] Review third-party kernel/firmware/source licensing, GPL obligations, reproducible build inputs, SBOM and authenticity of product update artifacts.
- [ ] Demonstrate authenticated local offline recovery, explicit persistence/poweroff/reboot, version rollback and disk-write target safety, with unambiguous human confirmation for every actual USB/host write.
- [ ] Complete [I009b2c #25](https://github.com/wjhuang88/vita/issues/25) Rust/Dinit/Finit equivalent-root, real controlling TTY, service supervision and fault/reaper study, then separately approve the **final** PID 1 choice.
- [ ] Record human product, security, physical firmware and release acceptance with exact final SHA and evidence before distributing media.

Status: **ALL HUMAN GATES PENDING**. No physical disk/USB was written by the I009b2b CI or approved by this document. Parent #21 remains **open**; no production acceptance is implied.
