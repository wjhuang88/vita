# Boot and Media Validation

## Before any physical write
1. Produce the image in a file.
2. Validate partition/filesystem structure on a loopback/sparse device where possible.
3. Boot in QEMU for every required firmware mode.
4. Require evidence from inside the selected guest/ISO path, not merely proof that QEMU or the boot menu started.
5. Capture exact tool versions, hashes, commands and results.

## Disposable installer-test gate

When an automated VM test must run a real disk/bootloader installer:
- create a new temporary raw image inside the test's private temporary directory;
- attach it with a loop device created by the test itself;
- accept only a whole `/dev/loopN` target, never an externally supplied block-device path;
- reject a target that the kernel exposes as a partition;
- query the loop mapping immediately before the destructive installer call and require its backing file to resolve exactly to the raw image created by this test;
- detach the loop device during cleanup even after failure;
- do not weaken these checks merely because the CI runner is disposable.

The I001 reference implementation is `scripts/ci/validate_i001_boot.sh`.

## Unattended boot proof

A boot smoke test is successful only when a deterministic marker or equivalent observable state is reached inside the intended selected image/guest path.

For Ventoy-backed unattended paths:
- configure the intended default image explicitly;
- configure a menu timeout explicitly;
- explicitly decide whether the secondary boot-mode menu is allowed. Vita's default unattended Live path disables it unless a later decision changes this;
- keep at least one test that proves an additional user image remains independently bootable/selectable;
- treat Secure Boot as a distinct matrix item rather than inferring it from ordinary UEFI success.

For GRUB rescue-image fixtures on clean Debian/Ubuntu-style CI runners, include `mtools` alongside GRUB/xorriso tooling; `grub-mkrescue` may require `mformat` while producing EFI-capable images.

## Physical target identity gate
Before `dd`, installer, partition, format or bootloader write:
- enumerate candidate block devices;
- display model, serial (when available), size, removable flag and mounted filesystems;
- explicitly identify source USB/data volume and intended target;
- refuse the current system/root disk;
- require explicit confirmation containing the resolved target device;
- unmount safely before writes.

The disposable loop-device proof does not waive or replace this physical-media gate.

## Release matrix
Record at minimum:
- x86_64 UEFI boot result for the declared initial target;
- legacy BIOS only if declared supported;
- Live OS boot result;
- representative real ISO boot result;
- persistence/config data preservation after upgrade path test;
- Secure Boot result only if Vita intends to claim support;
- physical-device results after all VM gates pass.
