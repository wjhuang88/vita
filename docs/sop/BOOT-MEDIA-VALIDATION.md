# Boot and Media Validation

## Before any physical write
1. Produce the image in a file.
2. Validate partition/filesystem structure on a loopback/sparse device where possible.
3. Boot in QEMU for every required firmware mode.
4. Capture evidence.

## Physical target identity gate
Before `dd`, installer, partition, format or bootloader write:
- enumerate candidate block devices;
- display model, serial (when available), size, removable flag and mounted filesystems;
- explicitly identify source USB/data volume and intended target;
- refuse the current system/root disk;
- require explicit confirmation containing the resolved target device;
- unmount safely before writes.

## Release matrix
Record at minimum:
- UEFI boot result;
- legacy BIOS only if declared supported;
- Live OS boot result;
- representative ISO boot result;
- persistence/config data preservation after upgrade path test.
