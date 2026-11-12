#!/usr/bin/env bash
set -euo pipefail

# I009a VM-only candidate Rust PID1, nonroot Talos tty and crash/reap fixture; NO disk/USB targets.
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
WORK_DIR="$(mktemp -d /tmp/vita-i009a-live.XXXXXX)"
LOG_DIR="$ROOT_DIR/i009a-init-logs"
ISO_NAME="alpine-virt-3.24.3-x86_64.iso"
ISO_URL="https://dl-cdn.alpinelinux.org/alpine/v3.24/releases/x86_64/$ISO_NAME"
ISO_SHA256="cdc1bc0efc64d8d611bf3733f54868172a545888dfd726f518bf279bcb0ff1af"
KERNEL_SHA256="17fbd3efd16f2e7cb1b945f06423b37399ba7d5ac8cdd3665e7f13f363500e85"

fail() { echo "VITA_I009A_ERROR: $*" >&2; exit 1; }
cleanup() { rm -rf "$WORK_DIR"; }
trap cleanup EXIT
[[ "$(uname -m)" == "x86_64" ]] || fail "x86_64 Linux host only"
mkdir -p "$LOG_DIR"

echo "[I009a] Install host-only QEMU and static Rust build tools"
sudo apt-get update -qq
sudo DEBIAN_FRONTEND=noninteractive apt-get install -y -qq \
  qemu-system-x86 ovmf grub-common grub-pc-bin grub-efi-amd64-bin \
  xorriso mtools cpio gzip binutils musl-tools curl cmake clang pkg-config build-essential >/dev/null
echo "[I009a] Verify Alpine virt 3.24.3 ISO and exact I005 kernel"
curl --fail --location --retry 3 --silent --show-error "$ISO_URL" -o "$WORK_DIR/$ISO_NAME"
echo "$ISO_SHA256  $WORK_DIR/$ISO_NAME" | sha256sum --check --status \
  || fail "Alpine ISO does not match pinned SHA-256"
xorriso -osirrox on -indev "$WORK_DIR/$ISO_NAME" \
  -extract /boot/vmlinuz-virt "$WORK_DIR/vmlinuz-virt" >"$LOG_DIR/xorriso.log" 2>&1 \
  || fail "kernel extraction failed"
echo "$KERNEL_SHA256  $WORK_DIR/vmlinuz-virt" | sha256sum --check --status \
  || fail "Alpine kernel differs from I005 pinned hash"
echo "VITA_I009A_KERNEL_SHA256=$KERNEL_SHA256" >"$LOG_DIR/kernel.sha256"

echo "[I009a] Cross-build static Vita init and genuine Talos SDK host"
rustup toolchain install 1.99.0 --profile minimal
rustup target add --toolchain 1.99.0 x86_64-unknown-linux-musl
export CC_x86_64_unknown_linux_musl=musl-gcc
cargo +1.99.0 build --manifest-path "$ROOT_DIR/prototypes/vita-init/Cargo.toml" \
  --release --target x86_64-unknown-linux-musl
cargo +1.99.0 test --manifest-path "$ROOT_DIR/prototypes/vita-init/Cargo.toml"
cargo +1.99.0 build --manifest-path "$ROOT_DIR/prototypes/interaction-runtime/Cargo.toml" \
  --release --target x86_64-unknown-linux-musl

INIT="$ROOT_DIR/prototypes/vita-init/target/x86_64-unknown-linux-musl/release/vita-init"
TALOS="$ROOT_DIR/prototypes/interaction-runtime/target/x86_64-unknown-linux-musl/release/vita-interaction-prototype"
for binary in "$INIT" "$TALOS"; do
  [[ -x "$binary" ]] || fail "guest Rust executable missing"
  if readelf -l "$binary" | grep -q 'INTERP'; then
    fail "dynamic ELF interpreter present in guest Rust binary"
  fi
  sha256sum "$binary" >>"$LOG_DIR/guest-binaries.sha256"
done

echo "[I009a] Build isolated initramfs (no physical target or guest drive)"
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
  find . -print0 | cpio --null -o --format=newc --owner=0:0 2>"$LOG_DIR/cpio.log" \
    | gzip -9 >"$WORK_DIR/vita-guest-initramfs.cpio.gz"
)
[[ -s "$WORK_DIR/vita-guest-initramfs.cpio.gz" ]] || fail "initramfs generation failed"
du -h "$WORK_DIR/vita-guest-initramfs.cpio.gz" >"$LOG_DIR/initramfs-size.txt"

echo "[I009a] Assemble Vita-owned GRUB ISO booting real Alpine kernel and Rust/Talos SDK"
ISO_TREE="$WORK_DIR/vita-iso"
VITA_ISO="$WORK_DIR/vita-live-experimental.iso"
mkdir -p "$ISO_TREE/boot/grub"
cp "$WORK_DIR/vmlinuz-virt" "$ISO_TREE/boot/vmlinuz-virt"
cp "$WORK_DIR/vita-guest-initramfs.cpio.gz" "$ISO_TREE/boot/vita-initramfs.cpio.gz"
cat >"$ISO_TREE/boot/grub/grub.cfg" <<'GRUB'
set default=0
set timeout=0
serial --unit=0 --speed=115200
terminal_input serial console
terminal_output serial console
menuentry "Vita experimental Linux/Talos session" {
    linux /boot/vmlinuz-virt console=ttyS0 rdinit=/init vita.i009.test_orphan=1 panic=-1 loglevel=5
    initrd /boot/vita-initramfs.cpio.gz
}
GRUB
grub-mkrescue -o "$VITA_ISO" "$ISO_TREE" >"$LOG_DIR/grub-mkrescue.log" 2>&1 \
  || fail "GRUB ISO creation failed"
[[ -s "$VITA_ISO" ]] || fail "empty Vita ISO"
sha256sum "$VITA_ISO" >"$LOG_DIR/vita-iso.sha256"
stat -c '%s' "$VITA_ISO" >"$LOG_DIR/vita-iso.bytes"
cp "$ISO_TREE/boot/grub/grub.cfg" "$LOG_DIR/grub.cfg"
# Pre-release QA only: deliberately exclude feature-gated root shell.
[[ ! -e "$WORK_DIR/root/bin/busybox" ]] || fail "unexpected recovery shell in ISO"

for mode in bios uefi; do
  echo "[I009a] Boot generated ISO through $mode firmware into actual guest Talos session"
  python3 "$ROOT_DIR/scripts/ci/observe_i006_guest_session.py" \
    --iso "$VITA_ISO" --firmware "$mode" --log-dir "$LOG_DIR/$mode" \
    --timeout 210 --final-marker VITA_I007_DEFAULT_TALOS_EXIT_OK \
    --required-marker VITA_I009_ORPHAN_REAPED_OK
  grep -Fq 'VITA_I007_RUNTIME_MOUNTS_READY' "$LOG_DIR/$mode/guest-serial.log" \
    || fail "missing runtime mount proof under $mode"
  grep -Fq 'VITA_I007_DEFAULT_TALOS_EXIT_OK' "$LOG_DIR/$mode/guest-serial.log" \
    || fail "missing Talos exit marker under $mode"
  if grep -Eq 'VITA_I007_RECOVERY_READY|VITA_I006_GUEST_SESSION_ERROR:' "$LOG_DIR/$mode/guest-serial.log"; then
    fail "unexpected recovery or init error under $mode"
  fi
  for marker in VITA_I009_ROOTFS_OWNER_OK VITA_I009_PID1_MINIMAL_INIT_READY VITA_I009_TALOS_UID=1000 VITA_I009_CONTROLLING_TTY_OK VITA_I009_ORPHAN_REAPED_OK; do
    grep -Fq "$marker" "$LOG_DIR/$mode/guest-serial.log" || fail "missing $marker under $mode"
  done
  echo "VITA_I009A_ISO_GUEST_TALOS_OK mode=$mode"
done
echo "VITA_I009A_LINUX_TALOS_ISO_VM_OK"

echo "[I009a] Prove Ctrl+C preserves Talos launcher and exits cleanly"
python3 "$ROOT_DIR/scripts/ci/observe_i006_guest_session.py" \
  --iso "$VITA_ISO" --firmware bios --log-dir "$LOG_DIR/ctrl-c" \
  --timeout 180 --ctrl-c-exit
grep -Fq VITA_I007_DEFAULT_TALOS_EXIT_OK "$LOG_DIR/ctrl-c/guest-serial.log" \
  || fail "Ctrl+C did not produce a clean Talos exit"

echo "[I009a] Prove bounded failure without enabling a root recovery shell"
sed -i 's/vita.i009.test_orphan=1/vita.i009.test_crash=1/' "$ISO_TREE/boot/grub/grub.cfg"
CRASH_ISO="$WORK_DIR/vita-crash-engineering.iso"
grub-mkrescue -o "$CRASH_ISO" "$ISO_TREE" >"$LOG_DIR/grub-crash.log" 2>&1 || fail "crash fixture ISO failed"
sha256sum "$CRASH_ISO" >"$LOG_DIR/crash-iso.sha256"
python3 "$ROOT_DIR/scripts/ci/observe_i009a_crash.py" --iso "$CRASH_ISO" \
  --log-dir "$LOG_DIR/crash" --timeout 150
grep -Fq VITA_I009_FAILSAFE_NO_ROOT_SHELL "$LOG_DIR/crash/guest-serial.log" \
  || fail "fail-closed crash proof missing"
echo "[I009a] Inject a session-spawn failure after creating an orphan and a signal sender"
sed -i 's/vita.i009.test_crash=1/vita.i009.test_orphan=1 vita.i009.test_spawn_failure=1/' "$ISO_TREE/boot/grub/grub.cfg"
SPAWN_FAIL_ISO="$WORK_DIR/vita-spawn-failure-engineering.iso"
grub-mkrescue -o "$SPAWN_FAIL_ISO" "$ISO_TREE" >"$LOG_DIR/grub-spawn-failure.log" 2>&1 \
  || fail "spawn-failure fixture ISO failed"
sha256sum "$SPAWN_FAIL_ISO" >"$LOG_DIR/spawn-failure-iso.sha256"
python3 "$ROOT_DIR/scripts/ci/observe_i009a_crash.py" --mode spawn --iso "$SPAWN_FAIL_ISO" \
  --log-dir "$LOG_DIR/spawn-failure" --timeout 150

echo "[I009a] Exercise actual PID 1 fatal fallback after adopting a descendant"
sed -i 's/vita.i009.test_spawn_failure=1/vita.i009.test_fatal=1/' "$ISO_TREE/boot/grub/grub.cfg"
FATAL_ISO="$WORK_DIR/vita-fatal-reaper-engineering.iso"
grub-mkrescue -o "$FATAL_ISO" "$ISO_TREE" >"$LOG_DIR/grub-fatal.log" 2>&1 \
  || fail "fatal-reaper fixture ISO failed"
sha256sum "$FATAL_ISO" >"$LOG_DIR/fatal-iso.sha256"
python3 "$ROOT_DIR/scripts/ci/observe_i009a_crash.py" --mode fatal --iso "$FATAL_ISO" \
  --log-dir "$LOG_DIR/fatal" --timeout 150
grep -Fq VITA_I009_FATAL_PID1_REAPER_ACTIVE "$LOG_DIR/fatal/guest-serial.log" \
  || fail "fatal PID1 reaper fallback was not entered"
echo "[I009a] Exercise PID 1 shutdown while Talos owns the foreground TTY"
sed -i 's/vita.i009.test_orphan=1 vita.i009.test_fatal=1/vita.i009.test_active_shutdown=1/' "$ISO_TREE/boot/grub/grub.cfg"
SHUTDOWN_ISO="$WORK_DIR/vita-active-session-shutdown.iso"
grub-mkrescue -o "$SHUTDOWN_ISO" "$ISO_TREE" >"$LOG_DIR/grub-active-shutdown.log" 2>&1 \
  || fail "active shutdown fixture ISO creation failed"
sha256sum "$SHUTDOWN_ISO" >"$LOG_DIR/active-shutdown-iso.sha256"
python3 "$ROOT_DIR/scripts/ci/observe_i009a_crash.py" --mode shutdown --iso "$SHUTDOWN_ISO" \
  --log-dir "$LOG_DIR/active-shutdown" --timeout 150
grep -Fq VITA_I009_SHUTDOWN_SESSION_REAPED "$LOG_DIR/active-shutdown/guest-serial.log" \
  || fail "active Talos child was not reaped after PID1 SIGTERM"
echo "VITA_I009A_MINIMAL_INIT_LIFECYCLE_VM_OK"
