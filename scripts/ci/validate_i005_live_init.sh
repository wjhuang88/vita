#!/usr/bin/env bash
set -euo pipefail

# I005: builds a Linux initramfs and boots a real Rust PID 1 in QEMU.
# No disk installer, loop device, host /dev write or physical USB target.
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
ALPINE_VERSION="3.24.3"
ISO_NAME="alpine-virt-$ALPINE_VERSION-x86_64.iso"
ISO_BASE="https://dl-cdn.alpinelinux.org/alpine/v3.24/releases/x86_64"
ISO_SHA256="cdc1bc0efc64d8d611bf3733f54868172a545888dfd726f518bf279bcb0ff1af"
WORK_DIR="$(mktemp -d "/tmp/vita-i005-live.XXXXXX")"
LOG_DIR="$ROOT_DIR/i005-live-logs"
QEMU_PID=""
mkdir -p "$LOG_DIR"

fail() {
  echo "VITA_I005_ERROR: $*" >&2
  exit 1
}
cleanup() {
  set +e
  if [[ -n "$QEMU_PID" ]] && kill -0 "$QEMU_PID" 2>/dev/null; then
    kill "$QEMU_PID" 2>/dev/null
    wait "$QEMU_PID" 2>/dev/null
  fi
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT INT TERM

[[ "$(uname -m)" == "x86_64" ]] || fail "I005 fixture currently targets x86_64 only"
command -v sudo >/dev/null || fail "sudo is needed to install CI build dependencies"
echo "[I005] Install host-only QEMU, archive and ELF inspection tools"
sudo apt-get update -qq
sudo DEBIAN_FRONTEND=noninteractive apt-get install -y -qq \
  qemu-system-x86 xorriso cpio gzip binutils musl-tools curl >/dev/null

echo "[I005] Fetch original Alpine virt ISO with exact pinned SHA-256"
curl --fail --location --retry 3 --silent --show-error "$ISO_BASE/$ISO_NAME" -o "$WORK_DIR/$ISO_NAME"
echo "$ISO_SHA256  $WORK_DIR/$ISO_NAME" | sha256sum --check --status \
  || fail "untrusted or changed Alpine ISO"
echo "VITA_I005_KERNEL_SOURCE iso=$ISO_NAME sha256=$ISO_SHA256"

echo "[I005] Extract kernel from verified upstream Alpine release"
xorriso -osirrox on -indev "$WORK_DIR/$ISO_NAME" \
  -extract /boot/vmlinuz-virt "$WORK_DIR/vmlinuz-virt" >"$WORK_DIR/xorriso.log" 2>&1 \
  || { cat "$WORK_DIR/xorriso.log" >&2; fail "Alpine ISO does not contain /boot/vmlinuz-virt"; }
[[ -s "$WORK_DIR/vmlinuz-virt" ]] || fail "extracted guest kernel is empty"
KERNEL_SHA256="$(sha256sum "$WORK_DIR/vmlinuz-virt" | awk '{print $1}')"
echo "VITA_I005_KERNEL_EXTRACTED sha256=$KERNEL_SHA256"
printf '%s  vmlinuz-virt\n' "$KERNEL_SHA256" > "$LOG_DIR/kernel.sha256"

echo "[I005] Compile Vita-owned Rust init probe for static musl x86_64 Linux"
rustup toolchain install 1.99.0 --profile minimal
rustup target add --toolchain 1.99.0 x86_64-unknown-linux-musl
cargo +1.99.0 build --manifest-path "$ROOT_DIR/prototypes/live-init/Cargo.toml" \
  --release --target x86_64-unknown-linux-musl

BINARY="$ROOT_DIR/prototypes/live-init/target/x86_64-unknown-linux-musl/release/vita-live-init-probe"
[[ -x "$BINARY" ]] || fail "Vita Rust init binary was not created"
if readelf -l "$BINARY" | grep -q 'INTERP'; then
  fail "Vita init has a dynamic ELF interpreter; static musl is required"
fi
file "$BINARY" || true

echo "[I005] Construct a minimal throwaway Linux initramfs"
mkdir -p "$WORK_DIR/root/dev"
cp "$BINARY" "$WORK_DIR/root/init"
chmod 0755 "$WORK_DIR/root/init"
# The char device exists only inside the newly-created initramfs tree.
sudo mknod -m 0600 "$WORK_DIR/root/dev/console" c 5 1
(
  cd "$WORK_DIR/root"
  find . -print0 | cpio --null -o --format=newc 2>"$WORK_DIR/cpio.log" | gzip -9 >"$WORK_DIR/vita-initramfs.cpio.gz"
)
[[ -s "$WORK_DIR/vita-initramfs.cpio.gz" ]] || fail "initramfs creation failed"

echo "[I005] Boot actual Linux kernel and run Vita's own Rust PID 1"
qemu-system-x86_64 \
  -machine accel=tcg -m 768 -smp 2 -nic none \
  -kernel "$WORK_DIR/vmlinuz-virt" \
  -initrd "$WORK_DIR/vita-initramfs.cpio.gz" \
  -append "console=ttyS0 rdinit=/init panic=-1 loglevel=5" \
  -display none -serial "file:$LOG_DIR/guest-serial.log" \
  -monitor none -no-reboot \
  >"$LOG_DIR/qemu.log" 2>&1 &
QEMU_PID="$!"
FOUND=0
for attempt in $(seq 1 100); do
  if grep -Fq 'VITA_I005_LINUX_RUST_INIT_OK pid=1 arch=x86_64' "$LOG_DIR/guest-serial.log"; then
    FOUND=1
    break
  fi
  if ! kill -0 "$QEMU_PID" 2>/dev/null; then
    cat "$LOG_DIR/qemu.log" >&2
    tail -100 "$LOG_DIR/guest-serial.log" >&2 || true
    fail "QEMU exited before guest Rust init proof"
  fi
  sleep 1
done
if [[ "$FOUND" != "1" ]]; then
  tail -100 "$LOG_DIR/guest-serial.log" >&2 || true
  cat "$LOG_DIR/qemu.log" >&2
  fail "Linux guest did not execute Vita's Rust PID 1 within 100 seconds"
fi
echo "VITA_I005_LINUX_RUST_INIT_OK pid=1 arch=x86_64"
echo "VITA_I005_LIVE_KERNEL_PROOF_OK alpine=$ALPINE_VERSION kernel_sha256=$KERNEL_SHA256"
