#!/usr/bin/env bash
set -euo pipefail
# I009b1 VM-only: pinned Linux kernel and its modules -> Rust stage1 ->
# verified read-only SquashFS '/' -> Rust permanent init -> Talos UID1000.
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
WORK="$(mktemp -d /tmp/vita-i009b1.XXXXXX)"
MEDIA_MODE="${VITA_I009B2_MEDIA_MODE:-0}"
[[ "$MEDIA_MODE" == 0 || "$MEDIA_MODE" == 1 ]] || { echo "Invalid I009b2 mode" >&2; exit 1; }
LOG="$ROOT/i009b1-rootfs-logs"
if [[ "$MEDIA_MODE" == 1 ]]; then
  LOG="$ROOT/i009b2-media-logs"
fi
ISO="alpine-virt-3.24.3-x86_64.iso"
URL="https://dl-cdn.alpinelinux.org/alpine/v3.24/releases/x86_64/$ISO"
ISO_HASH="cdc1bc0efc64d8d611bf3733f54868172a545888dfd726f518bf279bcb0ff1af"
KERNEL_HASH="17fbd3efd16f2e7cb1b945f06423b37399ba7d5ac8cdd3665e7f13f363500e85"
fail() { echo "VITA_I009B1_ERROR: $*" >&2; exit 1; }
trap 'rm -rf "$WORK"' EXIT
[[ "$(uname -m)" == "x86_64" ]] || fail "x86_64 Linux host only"
mkdir -p "$LOG"
sudo apt-get update -qq
sudo DEBIAN_FRONTEND=noninteractive apt-get install -y -qq \
  qemu-system-x86 ovmf grub-common grub-pc-bin grub-efi-amd64-bin \
  xorriso mtools cpio gzip binutils musl-tools curl cmake clang pkg-config build-essential \
  squashfs-tools kmod zstd xz-utils >/dev/null

echo "[I009b1] Verify pinned upstream Alpine ISO and matching kernel"
curl --fail --location --retry 3 --silent --show-error "$URL" -o "$WORK/$ISO"
echo "$ISO_HASH  $WORK/$ISO" | sha256sum -c --status || fail "upstream ISO hash mismatch"
sha256sum "$WORK/$ISO" >"$LOG/upstream-iso.sha256"
xorriso -osirrox on -indev "$WORK/$ISO" \
  -extract /boot/vmlinuz-virt "$WORK/vmlinuz" >"$LOG/kernel-extract.log" 2>&1 || fail "kernel missing"
echo "$KERNEL_HASH  $WORK/vmlinuz" | sha256sum -c --status || fail "kernel hash mismatch"

echo "[I009b1] Extract matching-kernel module metadata from the *same* verified ISO"
xorriso -osirrox on -indev "$WORK/$ISO" \
  -extract /boot/initramfs-virt "$WORK/initramfs-virt" >"$LOG/initfs-extract.log" 2>&1 \
  || fail "Alpine initramfs-virt missing"
sha256sum "$WORK/initramfs-virt" >"$LOG/upstream-initramfs.sha256"
SRC="$WORK/kmodules"
mkdir -p "$SRC"
# Record actual archive names before selecting; Alpine cpio entries may
# contain a leading ./, so do not assume a hard-coded path prefix.
gzip -dc "$WORK/initramfs-virt" | cpio -it --quiet >"$LOG/upstream-initramfs-entries.txt" \
  2>"$LOG/upstream-cpio-list.err" || fail "invalid source initramfs"
grep -E '(^|/)lib/modules/' "$LOG/upstream-initramfs-entries.txt" \
  >"$LOG/upstream-module-entries.txt" || fail "pinned initramfs contains no module entries"
(
  cd "$SRC"
  gzip -dc "$WORK/initramfs-virt" |
    cpio -idm --quiet --no-absolute-filenames --no-preserve-owner '*lib/modules/*'
) || fail "matching kernel module metadata extraction failed"
# Alpine v3.24 initramfs ships modules under usr/lib/modules,
# not lib/modules. Present the normal merged-/usr path expected by kmod
# without changing or downloading any kernel module bytes.
[[ -d "$SRC/usr/lib/modules" ]] || {
  find "$SRC" -maxdepth 6 -type f | head -n 25 >"$LOG/upstream-extracted-paths.txt"
  fail "pinned usr/lib/modules tree missing"
}
ln -s usr/lib "$SRC/lib"
KVER="$(find "$SRC/usr/lib/modules" -mindepth 1 -maxdepth 1 -type d -printf '%f\n' | head -1)"
[[ -n "$KVER" ]] || fail "kernel version not found"
[[ "$(find "$SRC/usr/lib/modules" -mindepth 1 -maxdepth 1 -type d | wc -l)" == 1 ]] ||
  fail "ambiguous kernel module versions"
echo "kernel_release=$KVER" >"$LOG/kernel-modules.txt"
# Without --all, modprobe interprets 'squashfs' as a parameter to
# 'loop' rather than a second requested kernel module.
drivers=(loop squashfs)
if [[ "$MEDIA_MODE" == 1 ]]; then
  # The Linux ISO is explicitly /dev/sr0 in the diskless QEMU fixture.
  # Resolve all dependencies for optical media, ISO9660, ATA/ATAPI and loop.
  # The Linux filesystem type is 'iso9660', but its kernel driver
  # is named 'isofs'. Require module identity, not filesystem type.
  drivers+=(isofs sr_mod ata_piix)
fi
modprobe --all -d "$SRC" -S "$KVER" --show-depends "${drivers[@]}" \
  >"$LOG/modprobe-plan.txt" 2>"$LOG/modprobe-error.txt" ||
  fail "pinned module dependencies for squashfs/loop could not be resolved"
if grep -Eq '^(install|remove|softdep) ' "$LOG/modprobe-plan.txt"; then
  fail "unexpected executable module helper in pinned manifest"
fi
for required in "${drivers[@]}"; do
  grep -Eq "(^|/)$required[.]ko|^builtin .*$required" "$LOG/modprobe-plan.txt" ||
    fail "dependency plan does not include mandatory module $required"
done
MODULES="$WORK/modules"
mkdir -p "$MODULES"
: >"$MODULES/load.order"
: >"$LOG/module-provenance.sha256"
count=0
while IFS= read -r line; do
  [[ -n "$line" ]] || continue
  case "$line" in
    builtin\ *) continue ;;
    insmod\ *)
      src_file="$(printf '%s\n' "$line" | awk '{print $2}')"
      case "$src_file" in
        "$SRC"/*) ;;
        /lib/modules/*) src_file="$SRC$src_file" ;;
        *) fail "module path escaped pinned source: $src_file" ;;
      esac
      [[ -f "$src_file" ]] || fail "missing module from pinned source: $src_file"
      name="$(printf '%03d.ko' "$count")"
      case "$src_file" in
        *.ko) cp "$src_file" "$MODULES/$name" ;;
        *.ko.gz) gzip -dc "$src_file" >"$MODULES/$name" ;;
        *.ko.xz) xz -dc "$src_file" >"$MODULES/$name" ;;
        *.ko.zst) zstd -dc "$src_file" >"$MODULES/$name" ;;
        *) fail "unsupported module compression: $src_file" ;;
      esac
      [[ "$(head -c 4 "$MODULES/$name" | od -An -tx1 | tr -d ' \n')" == 7f454c46 ]] ||
        fail "kernel module is not ELF: $name"
      ver="$(modinfo -F vermagic "$MODULES/$name" | awk '{print $1}')"
      [[ "$ver" == "$KVER" ]] || fail "kernel module ABI mismatch for $name: $ver"
      printf '%s\n' "$name" >>"$MODULES/load.order"
      sha256sum "$MODULES/$name" >>"$LOG/module-provenance.sha256"
      printf 'order=%s source=%s\n' "$name" "$src_file" >>"$LOG/kernel-modules.txt"
      count=$((count + 1))
      ;;
    *) fail "invalid modprobe plan entry: $line" ;;
  esac
done <"$LOG/modprobe-plan.txt"
echo "module_count=$count" >>"$LOG/kernel-modules.txt"

echo "[I009b1] Compile static Rust permanent init and native Talos"
rustup toolchain install 1.99.0 --profile minimal
rustup target add --toolchain 1.99.0 x86_64-unknown-linux-musl
export CC_x86_64_unknown_linux_musl=musl-gcc
cargo +1.99.0 build --manifest-path "$ROOT/prototypes/vita-init/Cargo.toml" \
  --release --target x86_64-unknown-linux-musl
cargo +1.99.0 test --manifest-path "$ROOT/prototypes/vita-init/Cargo.toml"
cargo +1.99.0 build --manifest-path "$ROOT/prototypes/interaction-runtime/Cargo.toml" \
  --release --target x86_64-unknown-linux-musl
INIT="$ROOT/prototypes/vita-init/target/x86_64-unknown-linux-musl/release/vita-init"
TALOS="$ROOT/prototypes/interaction-runtime/target/x86_64-unknown-linux-musl/release/vita-interaction-prototype"
for binary in "$INIT" "$TALOS"; do
  [[ -x "$binary" ]] || fail "static Rust guest binary missing"
  if readelf -l "$binary" | grep -q INTERP; then fail "unexpected dynamic interpreter"; fi
  sha256sum "$binary" >>"$LOG/guest-binaries.sha256"
done

echo "[I009b1] Build genuinely separate immutable SquashFS system image"
TREE="$WORK/rootfs"
mkdir -p "$TREE/dev" "$TREE/proc" "$TREE/sys" "$TREE/run" "$TREE/tmp" "$TREE/media" \
  "$TREE/vita" "$TREE/etc" "$TREE/root"
cp "$INIT" "$TREE/init"
cp "$TALOS" "$TREE/vita/vita-interaction-prototype"
chmod 0755 "$TREE/init" "$TREE/vita/vita-interaction-prototype"
# Disposable engineering-only probe: guarantees a UID 1000 file creation
# would succeed in this directory if the *root filesystem* were writable.
# It is intentionally absent from any product rootfs contract.
mkdir -p "$TREE/.vita-i009b1-write-fixture"
chmod 0777 "$TREE/.vita-i009b1-write-fixture"
printf 'root:x:0:0:root:/root:/bin/false\nvita:x:1000:1000:Vita Agent:/run/vita:/bin/false\n' >"$TREE/etc/passwd"
printf 'root:x:0:\nvita:x:1000:\n' >"$TREE/etc/group"
chmod 0644 "$TREE/etc/passwd" "$TREE/etc/group"
IMAGE="$WORK/vita-system.squashfs"
mksquashfs "$TREE" "$IMAGE" -noappend -no-progress -no-xattrs \
  -all-root -all-time 0 -mkfs-time 0 -processors 1 -comp gzip \
  >"$LOG/mksquashfs.log" 2>&1 || fail "SquashFS build failed"
[[ -s "$IMAGE" ]] || fail "root image empty"
sha256sum "$IMAGE" >"$LOG/root-image.sha256"
stat -c %s "$IMAGE" >"$LOG/root-image.bytes"
unsquashfs -stat "$IMAGE" >"$LOG/root-image-stat.txt" || fail "root image corrupt"
unsquashfs -ll "$IMAGE" >"$LOG/root-image-entries.txt" || fail "root image inaccessible"
grep -Fq '/vita/vita-interaction-prototype' "$LOG/root-image-entries.txt" ||
  fail "root image missing Talos"
ROOT_HASH="$(sha256sum "$IMAGE" | awk '{print $1}')"

echo "[I009b1] Pin root SHA inside independently compiled Rust stage1"
if [[ "$MEDIA_MODE" == 1 ]]; then
  export VITA_ROOT_SOURCE=media
else
  unset VITA_ROOT_SOURCE || true
fi
VITA_ROOTFS_SHA256="$ROOT_HASH" cargo +1.99.0 build \
  --manifest-path "$ROOT/prototypes/vita-stage1/Cargo.toml" \
  --release --target x86_64-unknown-linux-musl
VITA_ROOTFS_SHA256="$ROOT_HASH" cargo +1.99.0 test \
  --manifest-path "$ROOT/prototypes/vita-stage1/Cargo.toml"
STAGE="$ROOT/prototypes/vita-stage1/target/x86_64-unknown-linux-musl/release/vita-stage1"
[[ -x "$STAGE" ]] || fail "stage1 missing"
if readelf -l "$STAGE" | grep -q INTERP; then fail "stage1 not static"; fi
sha256sum "$STAGE" >>"$LOG/guest-binaries.sha256"

echo "[I009b1] Pack stage1 initramfs containing the independent root image"
EARLY="$WORK/early"
mkdir -p "$EARLY/dev" "$EARLY/proc" "$EARLY/sys" "$EARLY/run" \
  "$EARLY/tmp" "$EARLY/newroot" "$EARLY/modules"
cp "$STAGE" "$EARLY/init"
if [[ "$MEDIA_MODE" == 0 ]]; then
  cp "$IMAGE" "$EARLY/system.squashfs"
else
  [[ ! -e "$EARLY/system.squashfs" ]] ||
    fail "media-backed image must not be copied into initramfs"
  echo "media-backed rootfs absent from early initramfs" >"$LOG/image-not-in-initramfs.txt"
fi
cp "$MODULES"/* "$EARLY/modules/"
chmod 0755 "$EARLY/init"
sudo mknod -m 0600 "$EARLY/dev/console" c 5 1
sudo mknod -m 0600 "$EARLY/dev/ttyS0" c 4 64
sudo mknod -m 0666 "$EARLY/dev/null" c 1 3
sudo mknod -m 0600 "$EARLY/dev/loop0" b 7 0
(
  cd "$EARLY"
  find . -print0 | cpio --null -o --format=newc --owner=0:0 2>"$LOG/cpio.log" \
    | gzip -9 >"$WORK/stage1-initramfs.cpio.gz"
)
[[ -s "$WORK/stage1-initramfs.cpio.gz" ]] || fail "stage1 initramfs empty"
sha256sum "$WORK/stage1-initramfs.cpio.gz" >"$LOG/stage1-initramfs.sha256"
stat -c %s "$WORK/stage1-initramfs.cpio.gz" >"$LOG/stage1-initramfs.bytes"
[[ ! -e "$EARLY/bin/busybox" ]] || fail "unexpected recovery shell in default boot"

echo "[I009b1] Build actual BIOS/UEFI Vita ISO"
ISO_TREE="$WORK/iso"
VITA_ISO="$WORK/vita-i009b1.iso"
mkdir -p "$ISO_TREE/boot/grub"
cp "$WORK/vmlinuz" "$ISO_TREE/boot/vmlinuz-virt"
cp "$WORK/stage1-initramfs.cpio.gz" "$ISO_TREE/boot/stage1.cpio.gz"
if [[ "$MEDIA_MODE" == 1 ]]; then
  mkdir -p "$ISO_TREE/vita"
  cp "$IMAGE" "$ISO_TREE/vita/system.squashfs"
  sha256sum "$ISO_TREE/vita/system.squashfs" >"$LOG/iso-media-root-image.sha256"
fi
cat >"$ISO_TREE/boot/grub/grub.cfg" <<'GRUB'
set default=0
set timeout=0
serial --unit=0 --speed=115200
terminal_input serial console
terminal_output serial console
menuentry "Vita experimental SquashFS root" {
    linux /boot/vmlinuz-virt console=ttyS0 rdinit=/init vita.i009.test_orphan=1 panic=-1 loglevel=5
    initrd /boot/stage1.cpio.gz
}
GRUB
grub_args=()
if [[ "$MEDIA_MODE" == 1 ]]; then
  grub_args=(-V VITA_I009B2)
fi
grub-mkrescue "${grub_args[@]}" -o "$VITA_ISO" "$ISO_TREE" >"$LOG/grub.log" 2>&1 ||
  fail "firmware ISO build failed"
sha256sum "$VITA_ISO" >"$LOG/vita-iso.sha256"
stat -c %s "$VITA_ISO" >"$LOG/vita-iso.bytes"
cp "$ISO_TREE/boot/grub/grub.cfg" "$LOG/grub.cfg"
if [[ "$MEDIA_MODE" == 1 ]]; then
  xorriso -indev "$VITA_ISO" -pvd_info >"$LOG/iso-volume-id.txt" 2>&1 ||
    fail "cannot inspect ISO9660 volume descriptor"
  grep -Fq VITA_I009B2 "$LOG/iso-volume-id.txt" ||
    fail "generated media ISO volume label mismatch"
fi

for mode in bios uefi; do
  echo "[I009b1] Boot generated ISO using $mode firmware into genuine Talos"
  extra_markers=()
  if [[ "$MEDIA_MODE" == 1 ]]; then
    extra_markers=(--required-marker VITA_I009B2_MEDIA_SOURCE_OK
                   --required-marker VITA_I009B2_MEDIA_MOUNT_MOVED_OK)
  fi
  python3 "$ROOT/scripts/ci/observe_i006_guest_session.py" \
    --iso "$VITA_ISO" --firmware "$mode" --log-dir "$LOG/$mode" \
    "${extra_markers[@]}" \
    --timeout 240 --final-marker VITA_I007_DEFAULT_TALOS_EXIT_OK \
    --required-marker VITA_I009_ORPHAN_REAPED_OK \
    --required-marker VITA_I009B1_STAGE1_SQUASHFS_MOUNTED \
    --required-marker VITA_I009B1_ROOT_IMAGE_SHA256_OK \
    --required-marker VITA_I009B1_ROOT_MOUNT_READONLY_OK \
    --required-marker VITA_I009B1_NONROOT_ROOT_WRITE_DENIED \
    --required-marker VITA_I009B1_SWITCH_ROOT_PID1_CONTINUITY_OK
  for marker in VITA_I009B1_STAGE1_PID1_READY \
    VITA_I009B1_STAGE1_SQUASHFS_MOUNTED VITA_I009B1_ROOT_IMAGE_SHA256_OK \
    VITA_I009B1_ROOT_MOUNT_READONLY_OK VITA_I009B1_SWITCH_ROOT_PID1_CONTINUITY_OK \
    VITA_I009B1_NONROOT_ROOT_WRITE_DENIED VITA_I009_TALOS_UID=1000 VITA_I009_CONTROLLING_TTY_OK \
    VITA_I009_ORPHAN_REAPED_OK VITA_I007_DEFAULT_TALOS_EXIT_OK; do
    grep -Fq "$marker" "$LOG/$mode/guest-serial.log" ||
      fail "missing $marker under $mode"
  done
  if grep -Eq 'VITA_I009B1_STAGE1_FAIL_CLOSED|VITA_I006_GUEST_SESSION_ERROR:' \
    "$LOG/$mode/guest-serial.log"; then
    fail "fatal guest init or image error under $mode"
  fi
  if [[ "$MEDIA_MODE" == 1 ]]; then
    for marker in VITA_I009B2_MEDIA_SOURCE_OK VITA_I009B2_MEDIA_MOUNT_MOVED_OK; do
      grep -Fq "$marker" "$LOG/$mode/guest-serial.log" ||
        fail "missing readonly ISO-media marker $marker under $mode"
    done
    echo "VITA_I009B2_MEDIA_ROOTFS_GUEST_OK mode=$mode"
  fi
  echo "VITA_I009B1_IMMUTABLE_ROOTFS_GUEST_OK mode=$mode"
done
echo "[I009b1] Replay lifecycle faults through *stage1 + SquashFS*, not only direct initramfs"
python3 "$ROOT/scripts/ci/observe_i006_guest_session.py" \
  --iso "$VITA_ISO" --firmware bios --log-dir "$LOG/ctrl-c" \
  --timeout 180 --ctrl-c-exit
grep -Fq VITA_I009_TALOS_NATIVE_EXIT_OK "$LOG/ctrl-c/guest-serial.log" ||
  fail "Ctrl+C failed across the stage1 root handoff"

sed -i 's/vita.i009.test_orphan=1/vita.i009.test_crash=1/' "$ISO_TREE/boot/grub/grub.cfg"
CRASH_ISO="$WORK/vita-root-crash.iso"
grub-mkrescue -o "$CRASH_ISO" "$ISO_TREE" >"$LOG/grub-crash.log" 2>&1 ||
  fail "crash fixture ISO generation failed"
sha256sum "$CRASH_ISO" >"$LOG/crash-iso.sha256"
python3 "$ROOT/scripts/ci/observe_i009a_crash.py" --iso "$CRASH_ISO" \
  --log-dir "$LOG/crash" --timeout 180
grep -Fq 'VITA_I009_CRASH_BUDGET_EXHAUSTED attempts=3' "$LOG/crash/guest-serial.log" ||
  fail "lifecycle crash budget missing after root handoff"

sed -i 's/vita.i009.test_crash=1/vita.i009.test_orphan=1 vita.i009.test_spawn_failure=1/' \
  "$ISO_TREE/boot/grub/grub.cfg"
SPAWN_ISO="$WORK/vita-root-spawn-failure.iso"
grub-mkrescue -o "$SPAWN_ISO" "$ISO_TREE" >"$LOG/grub-spawn.log" 2>&1 ||
  fail "spawn failure fixture build failed"
sha256sum "$SPAWN_ISO" >"$LOG/spawn-iso.sha256"
python3 "$ROOT/scripts/ci/observe_i009a_crash.py" --mode spawn --iso "$SPAWN_ISO" \
  --log-dir "$LOG/spawn-failure" --timeout 180

sed -i 's/vita.i009.test_spawn_failure=1/vita.i009.test_fatal=1/' \
  "$ISO_TREE/boot/grub/grub.cfg"
FATAL_ISO="$WORK/vita-root-fatal-fallback.iso"
grub-mkrescue -o "$FATAL_ISO" "$ISO_TREE" >"$LOG/grub-fatal.log" 2>&1 ||
  fail "fatal reaper fixture ISO build failed"
sha256sum "$FATAL_ISO" >"$LOG/fatal-iso.sha256"
python3 "$ROOT/scripts/ci/observe_i009a_crash.py" --mode fatal --iso "$FATAL_ISO" \
  --log-dir "$LOG/fatal" --timeout 180

sed -i 's/vita.i009.test_orphan=1 vita.i009.test_fatal=1/vita.i009.test_active_shutdown=1/' \
  "$ISO_TREE/boot/grub/grub.cfg"
SHUTDOWN_ISO="$WORK/vita-root-active-shutdown.iso"
grub-mkrescue -o "$SHUTDOWN_ISO" "$ISO_TREE" >"$LOG/grub-shutdown.log" 2>&1 ||
  fail "active shutdown fixture ISO build failed"
sha256sum "$SHUTDOWN_ISO" >"$LOG/shutdown-iso.sha256"
python3 "$ROOT/scripts/ci/observe_i009a_crash.py" --mode shutdown --iso "$SHUTDOWN_ISO" \
  --log-dir "$LOG/active-shutdown" --timeout 180
echo "VITA_I009B1_SWITCHED_ROOT_LIFECYCLE_VM_OK"

echo "[I009b1] Negative proof: alter the root image AFTER embedding its digest in stage1"
sed -i 's/vita.i009.test_active_shutdown=1/vita.i009.test_orphan=1/' \
  "$ISO_TREE/boot/grub/grub.cfg"
if [[ "$MEDIA_MODE" == 1 ]]; then
  printf 'intentionally changed after stage1 hash was compiled' >>"$ISO_TREE/vita/system.squashfs"
else
  printf 'intentionally changed after stage1 hash was compiled' >>"$EARLY/system.squashfs"
fi
(
  cd "$EARLY"
  find . -print0 | cpio --null -o --format=newc --owner=0:0 2>"$LOG/cpio-tamper.log" \
    | gzip -9 >"$WORK/tampered-stage1.cpio.gz"
)
cp "$WORK/tampered-stage1.cpio.gz" "$ISO_TREE/boot/stage1.cpio.gz"
TAMPER_ISO="$WORK/vita-tampered-root.iso"
grub-mkrescue "${grub_args[@]}" -o "$TAMPER_ISO" "$ISO_TREE" >"$LOG/grub-tamper.log" 2>&1 ||
  fail "tampered ISO generation failed"
sha256sum "$TAMPER_ISO" >"$LOG/tampered-iso.sha256"
python3 "$ROOT/scripts/ci/observe_i009b1_tamper.py" --iso "$TAMPER_ISO" \
  --log-dir "$LOG/tampered" --timeout 180
if [[ "$MEDIA_MODE" == 1 ]]; then
  echo "VITA_I009B2_MEDIA_ROOTFS_VM_OK"
fi
echo "VITA_I009B1_IMMUTABLE_ROOTFS_VM_OK"
