# I009b2a — ISO media root: human/product acceptance (PENDING)

This checklist is **not satisfied by PR #22 AI code review or QEMU CI**. Owner: VITA-001-C / [I009b2 #21](https://github.com/wjhuang88/vita/issues/21). Record each decision with a human reviewer, exact SHA, hardware and evidence.

- [ ] Confirm firmware and Secure Boot trust chain and signed boot assets on actual target hardware; the current embedded image SHA alone is **not** trust anchoring.
- [ ] Review optical/USB root source selection beyond the **fixed QEMU-only `/dev/sr0`** device, including device identity, malicious media, path swap and loss/disconnect behavior.
- [ ] Verify correct initramfs and kernel-module provenance, source license/GPL/firmware obligations, supply-chain/SBOM, and update authenticity.
- [ ] Validate product recovery authorization, poweroff/reboot, secure offline fallback and persisted state policy; no unauthenticated shipping root shell.
- [ ] Benchmark real RAM and boot-to-Talos timing, resource exhaustion, module failures and media removal; prove behavior on representative BIOS/UEFI physical machines.
- [ ] Approve any actual removable/USB writes only after unambiguous device identity and explicit user confirmation; PR #22 performed **none**.
- [ ] Reassess Rust PID1 versus Dinit/Finit using shared guest workload before final production init choice.

Status: **All PENDING; engineering CI 43558241910 is not human acceptance.**
