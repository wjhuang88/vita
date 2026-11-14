#!/usr/bin/env bash
# I009b2a: the exact same firmware and fault workload as I009b1, but the
# immutable root image must remain on the read-only Vita-owned ISO.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
export VITA_I009B2_MEDIA_MODE=1
exec bash "$ROOT/scripts/ci/validate_i009b1_rootfs.sh"
