#!/usr/bin/env bash
set -euo pipefail

# I007: two boot modes in disposable QEMU: default Talos and explicit offline recovery.
# NO disk/USB targets, no host block device, no guest network.
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
WORK_DIR="$(mktemp -d /tmp/vita-i007-guest.XXXXXX)"
LOG_DIR="$ROOT_DIR/i007-guest-logs"
ISO_NAME="alpine-virt-3.24.3-x86_64.iso"
ISO_URL="https://dl-cdn.alpinelinux.org/alpine/v3.24/releases/x86_64/$ISO_NAME"
ISO_SHA256="cdc1bc0efc64d8d611bf3733f54868172a545888dfd726f518bf279bcb0ff1af"
KERNEL_SHA256="17fbd3efd16f2e7cb1b945f06423b37399ba7d5ac8cdd3665e7f13f363500e85"

fail() { echo "VITA_I007_ERROR: $*" >&2; exit 1; }
cleanup() { rm -rf "$WORK_DIR"; }
trap cleanup EXIT
[[ "$(uname -m)" == "x86_64" ]] || fail "x86_64 Linux host only"
mkdir -p "$LOG_DIR"

echo "[I007] Install host-only QEMU and build tools"
sudo apt-get update -qq
sudo DEBIAN_FRONTEND=noninteractive apt-get install -y -qq \
  qemu-system-x86 xorriso cpio gzip binutils musl-tools libarchive-tools \
  curl cmake clang pkg-config build-essential >/dev/null

# Ubuntu's BusyBox static binary SIGILLs on QEMU's x86_64 baseline CPU.
# Instead use a target-architecture Alpine v3.24 static package. Keep a
# manifest digest and pin it after the exploratory no-network QEMU proof.
RECOVERY_APK="busybox-static-1.37.0-r31.apk"
RECOVERY_URL="https://dl-cdn.alpinelinux.org/alpine/v3.24/main/x86_64/$RECOVERY_APK"
curl --fail --location --retry 3 --silent --show-error \
  "$RECOVERY_URL" -o "$WORK_DIR/$RECOVERY_APK"
sha256sum "$WORK_DIR/$RECOVERY_APK" >"$LOG_DIR/recovery-package.sha256"
RECOVERY="$WORK_DIR/busybox.static"
bsdtar -xOf "$WORK_DIR/$RECOVERY_APK" bin/busybox.static >"$RECOVERY" \
  || fail "Alpine static BusyBox package extraction failed"
[[ -s "$RECOVERY" ]] || fail "Alpine static BusyBox binary missing"
chmod 0755 "$RECOVERY"
if readelf -l "$RECOVERY" | grep -q 'INTERP'; then
  fail "Alpine BusyBox recovery shell must be statically linked"
fi
"$RECOVERY" --list | grep -qx sh || fail "Alpine BusyBox sh applet missing"
"$RECOVERY" --list | grep -qx id || fail "Alpine BusyBox id applet missing"
sha256sum "$RECOVERY" >"$LOG_DIR/recovery-binary.sha256"
echo "[I007] Verify Alpine virt 3.24.3 ISO and exact I005 kernel"
curl --fail --location --retry 3 --silent --show-error "$ISO_URL" -o "$WORK_DIR/$ISO_NAME"
echo "$ISO_SHA256  $WORK_DIR/$ISO_NAME" | sha256sum --check --status \
  || fail "Alpine ISO does not match pinned SHA-256"
xorriso -osirrox on -indev "$WORK_DIR/$ISO_NAME" \
  -extract /boot/vmlinuz-virt "$WORK_DIR/vmlinuz-virt" >"$LOG_DIR/xorriso.log" 2>&1 \
  || fail "kernel extraction failed"
echo "$KERNEL_SHA256  $WORK_DIR/vmlinuz-virt" | sha256sum --check --status \
  || fail "Alpine kernel differs from I005 pinned hash"
echo "VITA_I006_KERNEL_SHA256=$KERNEL_SHA256" >"$LOG_DIR/kernel.sha256"

echo "[I006] Cross-build static Vita init and genuine Talos SDK host"
rustup toolchain install 1.99.0 --profile minimal
rustup target add --toolchain 1.99.0 x86_64-unknown-linux-musl
export CC_x86_64_unknown_linux_musl=musl-gcc
cargo +1.99.0 build --manifest-path "$ROOT_DIR/prototypes/guest-session-init/Cargo.toml" \
  --release --target x86_64-unknown-linux-musl \
  --features experimental-root-recovery
cargo +1.99.0 build --manifest-path "$ROOT_DIR/prototypes/interaction-runtime/Cargo.toml" \
  --release --target x86_64-unknown-linux-musl

INIT="$ROOT_DIR/prototypes/guest-session-init/target/x86_64-unknown-linux-musl/release/vita-guest-session-init"
TALOS="$ROOT_DIR/prototypes/interaction-runtime/target/x86_64-unknown-linux-musl/release/vita-interaction-prototype"
for binary in "$INIT" "$TALOS"; do
  [[ -x "$binary" ]] || fail "guest Rust executable missing"
  if readelf -l "$binary" | grep -q 'INTERP'; then
    fail "dynamic ELF interpreter present in guest Rust binary"
  fi
  sha256sum "$binary" >>"$LOG_DIR/guest-binaries.sha256"
done

echo "[I006] Build isolated initramfs (no physical target or guest drive)"
mkdir -p "$WORK_DIR/root/dev" "$WORK_DIR/root/proc" "$WORK_DIR/root/sys" \
  "$WORK_DIR/root/run" "$WORK_DIR/root/tmp" "$WORK_DIR/root/bin" "$WORK_DIR/root/vita" "$WORK_DIR/root/etc"
cp "$INIT" "$WORK_DIR/root/init"
cp "$TALOS" "$WORK_DIR/root/vita/vita-interaction-prototype"
cp "$RECOVERY" "$WORK_DIR/root/bin/busybox"
chmod 0755 "$WORK_DIR/root/init" "$WORK_DIR/root/vita/vita-interaction-prototype" "$WORK_DIR/root/bin/busybox"
# ONLY character-device nodes under the brand-new disposable initramfs tree.
sudo mknod -m 0600 "$WORK_DIR/root/dev/console" c 5 1
sudo mknod -m 0600 "$WORK_DIR/root/dev/ttyS0" c 4 64
sudo mknod -m 0666 "$WORK_DIR/root/dev/null" c 1 3
(
  cd "$WORK_DIR/root"
  find . -print0 | cpio --null -o --format=newc 2>"$LOG_DIR/cpio.log" \
    | gzip -9 >"$WORK_DIR/vita-guest-initramfs.cpio.gz"
)
[[ -s "$WORK_DIR/vita-guest-initramfs.cpio.gz" ]] || fail "initramfs generation failed"
du -h "$WORK_DIR/vita-guest-initramfs.cpio.gz" >"$LOG_DIR/initramfs-size.txt"

echo "[I006] QEMU Linux guest receives actual Talos prompts on serial"
python3 "$ROOT_DIR/scripts/ci/observe_i006_guest_session.py" \
  --kernel "$WORK_DIR/vmlinuz-virt" \
  --initrd "$WORK_DIR/vita-guest-initramfs.cpio.gz" \
  --log-dir "$LOG_DIR/default" --timeout 140 \
  --final-marker VITA_I007_DEFAULT_TALOS_EXIT_OK

grep -Fq "VITA_I007_RUNTIME_MOUNTS_READY" "$LOG_DIR/default/guest-serial.log" \
  || fail "guest runtime mount validation absent"
grep -Fq "VITA_I007_DEFAULT_TALOS_EXIT_OK" "$LOG_DIR/default/guest-serial.log" \
  || fail "guest default Talos exit marker missing"
if grep -Fq "VITA_I007_RECOVERY_READY" "$LOG_DIR/default/guest-serial.log"; then
  fail "recovery shell entered without explicit boot selection"
fi
echo "VITA_I007_DEFAULT_TALOS_OK"

echo "[I007] Verify offline root shell selected only with explicit kernel argument"
python3 "$ROOT_DIR/scripts/ci/observe_i007_recovery.py" \
  --kernel "$WORK_DIR/vmlinuz-virt" \
  --initrd "$WORK_DIR/vita-guest-initramfs.cpio.gz" \
  --log-dir "$LOG_DIR/recovery" --timeout 140
echo "VITA_I007_OFFLINE_RECOVERY_OK"
