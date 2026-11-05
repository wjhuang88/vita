#!/usr/bin/env bash
set -euo pipefail

# VM-only Linux -> Rust PID1 -> Talos SDK session; NO disk/USB targets.
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
WORK_DIR="$(mktemp -d /tmp/vita-i006-guest.XXXXXX)"
LOG_DIR="$ROOT_DIR/i006-guest-logs"
ISO_NAME="alpine-virt-3.24.3-x86_64.iso"
ISO_URL="https://dl-cdn.alpinelinux.org/alpine/v3.24/releases/x86_64/$ISO_NAME"
ISO_SHA256="cdc1bc0efc64d8d611bf3733f54868172a545888dfd726f518bf279bcb0ff1af"
KERNEL_SHA256="17fbd3efd16f2e7cb1b945f06423b37399ba7d5ac8cdd3665e7f13f363500e85"

fail() { echo "VITA_I006_ERROR: $*" >&2; exit 1; }
cleanup() { rm -rf "$WORK_DIR"; }
trap cleanup EXIT
[[ "$(uname -m)" == "x86_64" ]] || fail "x86_64 Linux host only"
mkdir -p "$LOG_DIR"

echo "[I006] Install host-only QEMU and static Rust build tools"
sudo apt-get update -qq
sudo DEBIAN_FRONTEND=noninteractive apt-get install -y -qq \
  qemu-system-x86 xorriso cpio gzip binutils musl-tools \
  curl cmake clang pkg-config build-essential >/dev/null
echo "[I006] Verify Alpine virt 3.24.3 ISO and exact I005 kernel"
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
  --release --target x86_64-unknown-linux-musl
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
  "$WORK_DIR/root/run" "$WORK_DIR/root/tmp" "$WORK_DIR/root/vita" "$WORK_DIR/root/etc"
cp "$INIT" "$WORK_DIR/root/init"
cp "$TALOS" "$WORK_DIR/root/vita/vita-interaction-prototype"
chmod 0755 "$WORK_DIR/root/init" "$WORK_DIR/root/vita/vita-interaction-prototype"
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
  --log-dir "$LOG_DIR" --timeout 140

echo "VITA_I006_GUEST_BOOT_TO_TALOS_OK"
