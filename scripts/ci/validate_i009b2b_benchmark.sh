#!/usr/bin/env bash
# I009b2b QEMU-only same-runner comparison; no guest disks, network, or USB writes.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
WORK="$(mktemp -d /tmp/vita-i009b2b.XXXXXX)"
trap 'rm -rf "$WORK"' EXIT
export VITA_I009_BENCH_BUILD_ONLY=1
export VITA_I009_BENCH_EXPORT_DIR="$WORK"
echo "[I009b2b] Build embedded SquashFS ISO (same pinned source)"
bash "$ROOT/scripts/ci/validate_i009b1_rootfs.sh"
echo "[I009b2b] Build media-backed SquashFS ISO (same pinned source)"
bash "$ROOT/scripts/ci/validate_i009b2a_media_rootfs.sh"
echo "[I009b2b] Alternate source order for real BIOS/UEFI QEMU boot sessions"
python3 "$ROOT/scripts/ci/benchmark_i009b2b.py" \
  --embedded "$WORK/embedded.iso" --media "$WORK/media.iso" \
  --log-dir "$ROOT/i009b2b-benchmark-logs" --repetitions 3
