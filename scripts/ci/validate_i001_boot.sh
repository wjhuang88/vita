#!/usr/bin/env bash
set -euo pipefail

VENTOY_VERSION="1.1.18"
VENTOY_SHA256="d86ff9de63d94c8b8f1f6b14d23c55af4b88e8207bf2893890892a6a97b1ad54"
VENTOY_URL="https://github.com/ventoy/Ventoy/releases/download/v${VENTOY_VERSION}/ventoy-${VENTOY_VERSION}-linux.tar.gz"
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
WORK_DIR="$(mktemp -d "${RUNNER_TEMP:-/tmp}/vita-i001-boot.XXXXXX")"
RAW_IMAGE="$WORK_DIR/vita-ventoy.raw"
MOUNT_DIR="$WORK_DIR/mnt"
LOOP_DEVICE=""

cleanup() {
  set +e
  mountpoint -q "$MOUNT_DIR" && sudo umount "$MOUNT_DIR"
  if [[ -n "$LOOP_DEVICE" ]] && sudo losetup "$LOOP_DEVICE" >/dev/null 2>&1; then
    sudo losetup -d "$LOOP_DEVICE"
  fi
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT INT TERM

fail() {
  echo "VITA_BOOT_SMOKE_ERROR: $*" >&2
  exit 1
}

assert_disposable_loop() {
  local expected backing
  expected="$(readlink -f "$RAW_IMAGE")"
  [[ "$LOOP_DEVICE" =~ ^/dev/loop[0-9]+$ ]] || fail "refusing non-loop target: $LOOP_DEVICE"
  [[ -b "$LOOP_DEVICE" ]] || fail "loop target is not a block device: $LOOP_DEVICE"
  [[ ! -e "/sys/class/block/${LOOP_DEVICE#/dev/}/start" ]] || fail "target is a partition: $LOOP_DEVICE"
  backing="$(sudo losetup --list --noheadings --output BACK-FILE "$LOOP_DEVICE" | xargs)"
  [[ -n "$backing" ]] || fail "loop target has no backing file"
  backing="$(readlink -f "$backing")"
  [[ "$backing" == "$expected" ]] || fail "loop backing mismatch: expected $expected, got $backing"
}

make_probe_iso() {
  local iso_path="$1" marker="$2" tree="$WORK_DIR/iso-$2"
  mkdir -p "$tree/boot/grub"
  cat > "$tree/boot/grub/grub.cfg" <<EOF
set timeout=0
set default=0
serial --unit=0 --speed=115200
terminal_output serial console
menuentry "Vita I001 probe" {
    echo ${marker}
    sleep 1
    halt
}
EOF
  grub-mkrescue -o "$iso_path" "$tree"
  [[ -s "$iso_path" ]] || fail "failed to create probe ISO: $iso_path"
}

find_first_file() {
  local candidate
  for candidate in "$@"; do
    [[ -f "$candidate" ]] && { printf '%s\n' "$candidate"; return 0; }
  done
  return 1
}

run_qemu_probe() {
  local mode expected_marker log
  mode="$1"
  expected_marker="$2"
  log="$WORK_DIR/qemu-${mode}-${expected_marker}.log"
  local -a firmware_args=()
  if [[ "$mode" == "uefi" ]]; then
    local code vars vars_copy
    code="$(find_first_file /usr/share/OVMF/OVMF_CODE_4M.fd /usr/share/OVMF/OVMF_CODE.fd)" || fail "OVMF code image not found"
    vars="$(find_first_file /usr/share/OVMF/OVMF_VARS_4M.fd /usr/share/OVMF/OVMF_VARS.fd)" || fail "OVMF vars image not found"
    vars_copy="$WORK_DIR/OVMF_VARS-${expected_marker}.fd"
    cp "$vars" "$vars_copy"
    firmware_args=(
      -drive "if=pflash,format=raw,unit=0,readonly=on,file=$code"
      -drive "if=pflash,format=raw,unit=1,file=$vars_copy"
    )
  fi

  set +e
  timeout 45s qemu-system-x86_64 \
    -machine accel=tcg -m 512 \
    "${firmware_args[@]}" \
    -drive "file=$RAW_IMAGE,format=raw,if=ide" \
    -display none -serial "file:$log" -monitor none -no-reboot
  local rc=$?
  set -e
  [[ "$rc" -eq 0 || "$rc" -eq 124 ]] || { cat "$log" >&2 || true; fail "QEMU $mode exited with $rc"; }
  grep -Fq "$expected_marker" "$log" || { cat "$log" >&2 || true; fail "QEMU $mode did not reach $expected_marker"; }
  echo "VITA_BOOT_PROBE_OK mode=$mode marker=$expected_marker"
}

set_default_image() {
  local image_path="$1"
  sudo python3 - "$MOUNT_DIR/ventoy/ventoy.json" "$image_path" <<'PY'
import json, pathlib, sys
path = pathlib.Path(sys.argv[1])
data = json.loads(path.read_text())
for entry in data.get("control", []):
    if "VTOY_DEFAULT_IMAGE" in entry:
        entry["VTOY_DEFAULT_IMAGE"] = sys.argv[2]
        break
else:
    raise SystemExit("VTOY_DEFAULT_IMAGE missing")
path.write_text(json.dumps(data, indent=2) + "\n")
PY
  sync
}

echo "[I001] Install host dependencies"
sudo apt-get update -qq
sudo DEBIAN_FRONTEND=noninteractive apt-get install -y -qq \
  qemu-system-x86 ovmf grub-common grub-pc-bin grub-efi-amd64-bin \
  xorriso mtools parted util-linux dosfstools exfatprogs curl python3 >/dev/null

echo "[I001] Build deterministic boot-stub ISOs"
make_probe_iso "$WORK_DIR/vita-live.iso" VITA_BOOT_OK
make_probe_iso "$WORK_DIR/user-probe.iso" VITA_USER_ISO_BOOT_OK

echo "[I001] Fetch and verify pinned Ventoy release"
curl --fail --location --silent --show-error "$VENTOY_URL" -o "$WORK_DIR/ventoy.tar.gz"
echo "$VENTOY_SHA256  $WORK_DIR/ventoy.tar.gz" | sha256sum --check --status || fail "Ventoy SHA-256 mismatch"
tar -xzf "$WORK_DIR/ventoy.tar.gz" -C "$WORK_DIR"
VENTOY_DIR="$WORK_DIR/ventoy-${VENTOY_VERSION}"
[[ -f "$VENTOY_DIR/Ventoy2Disk.sh" ]] || fail "unexpected Ventoy release layout"

echo "[I001] Create disposable raw disk and prove target identity"
truncate -s 768M "$RAW_IMAGE"
LOOP_DEVICE="$(sudo losetup --find --show --partscan "$RAW_IMAGE")"
assert_disposable_loop

echo "[I001] Install Ventoy only to the verified disposable loop device"
(cd "$VENTOY_DIR" && printf 'y\ny\n' | sudo bash ./Ventoy2Disk.sh -i -L VITA "$LOOP_DEVICE")
assert_disposable_loop
sudo partprobe "$LOOP_DEVICE"
sudo udevadm settle
PART1="${LOOP_DEVICE}p1"
[[ -b "$PART1" ]] || fail "Ventoy data partition missing: $PART1"
mkdir -p "$MOUNT_DIR"
sudo mount "$PART1" "$MOUNT_DIR"
sudo mkdir -p "$MOUNT_DIR/iso/vita" "$MOUNT_DIR/ventoy" "$MOUNT_DIR/talos/config"
sudo cp "$WORK_DIR/vita-live.iso" "$MOUNT_DIR/iso/vita/vita-live.iso"
sudo cp "$WORK_DIR/user-probe.iso" "$MOUNT_DIR/iso/user-probe.iso"
sudo cp "$ROOT_DIR/prototypes/boot/ventoy/ventoy.json" "$MOUNT_DIR/ventoy/ventoy.json"
sync
sudo umount "$MOUNT_DIR"
sudo losetup -d "$LOOP_DEVICE"
LOOP_DEVICE=""

echo "[I001] Validate default Vita image through BIOS and x86_64 UEFI"
run_qemu_probe bios VITA_BOOT_OK
run_qemu_probe uefi VITA_BOOT_OK

echo "[I001] Prove an additional user ISO remains bootable/selectable"
LOOP_DEVICE="$(sudo losetup --find --show --partscan "$RAW_IMAGE")"
assert_disposable_loop
sudo partprobe "$LOOP_DEVICE"
sudo udevadm settle
PART1="${LOOP_DEVICE}p1"
sudo mount "$PART1" "$MOUNT_DIR"
set_default_image /iso/user-probe.iso
sudo umount "$MOUNT_DIR"
sudo losetup -d "$LOOP_DEVICE"
LOOP_DEVICE=""
run_qemu_probe bios VITA_USER_ISO_BOOT_OK

echo "VITA_I001_BOOT_CHAIN_OK ventoy=${VENTOY_VERSION}"
