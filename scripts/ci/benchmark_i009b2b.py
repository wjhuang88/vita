#!/usr/bin/env python3
"""I009b2b: same-runner, real BIOS/UEFI ISO boot-to-Talos comparison.

Runs only disposable QEMU with optical ISO input and no guest network/disk.
Guest memory is sampled by PID1 *before* Talos starts; host QEMU memory
is measured throughout the actual two-ok/one-error Talos workload.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import platform
import statistics
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OBSERVER = ROOT / "scripts/ci/observe_i006_guest_session.py"


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as reader:
        for chunk in iter(lambda: reader.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def summarize(records: list[dict]) -> dict:
    groups = {}
    for firmware in ("bios", "uefi"):
        for source in ("embedded", "media"):
            selected = [r for r in records
                        if r["firmware"] == firmware and r["source"] == source]
            if not selected:
                continue
            metrics = {}
            for key in ("tty_ms", "prompt_ms", "host_rss_at_prompt_kib",
                        "host_peak_hwm_kib"):
                numbers = [r[key] for r in selected]
                metrics[key] = {
                    "median": statistics.median(numbers),
                    "min": min(numbers),
                    "max": max(numbers),
                }
            # Guest metrics are a PRE-session baseline, not idle/post-prompt.
            for key in ("mem_available_kib", "processes", "uptime_s"):
                numbers = [r["guest_stats"][key] for r in selected]
                metrics["guest_pre_session_" + key] = {
                    "median": statistics.median(numbers),
                    "min": min(numbers),
                    "max": max(numbers),
                }
            groups[f"{firmware}/{source}"] = {
                "repetitions": len(selected), "metrics": metrics,
            }
    return groups


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--embedded", type=Path, required=True)
    parser.add_argument("--media", type=Path, required=True)
    parser.add_argument("--log-dir", type=Path, required=True)
    parser.add_argument("--repetitions", type=int, default=3)
    args = parser.parse_args()
    if args.repetitions < 3:
        parser.error("require >=3 repetitions per source per firmware")
    for path in (args.embedded, args.media):
        if not path.is_file() or path.is_symlink() or path.stat().st_size == 0:
            parser.error("each ISO must be a nonempty regular file, not a link/device")
    logs = args.log_dir.resolve()
    logs.mkdir(parents=True, exist_ok=True)
    source_files = {"embedded": args.embedded.resolve(), "media": args.media.resolve()}
    build_manifest = {
        key: {
            "iso_bytes": path.stat().st_size,
            "iso_sha256": sha256(path),
            "initramfs_bytes": int(path.with_name(f"{key}-initramfs.bytes").read_text().strip()),
            "root_squashfs_bytes": int(path.with_name(f"{key}-root.bytes").read_text().strip()),
            "root_squashfs_sha256": path.with_name(f"{key}-root.sha256").read_text().split()[0],
        }
        for key, path in source_files.items()
    }
    if build_manifest["embedded"]["root_squashfs_bytes"] != build_manifest["media"]["root_squashfs_bytes"]:
        raise ValueError("comparison root images must have equal byte size")
    if build_manifest["embedded"]["root_squashfs_sha256"] != build_manifest["media"]["root_squashfs_sha256"]:
        raise ValueError("comparison root images must be byte-identical by SHA-256")
    context = {
        "schema_version": 1,
        "runner": platform.uname()._asdict(),
        "qemu_version": subprocess.check_output(
            ["qemu-system-x86_64", "--version"], text=True,
        ).splitlines()[0],
        "qemu_profile": "-machine accel=tcg -m 1024 -smp 2 -nic none -display none -monitor none -no-reboot -cdrom",
        "workload": "two read-only Talos Tool turns and one handled model error, then :quit",
        "boot_clock": "host time.monotonic from QEMU Popen to first observed serial marker/prompt",
        "guest_memory_sampling": "pre_session only; NOT idle or post-Talos peak",
        "host_memory_sampling": "/proc/<qemu-pid>/status VmRSS at prompt, VmHWM maximum during workload",
        "cache_caveat": "single runner, alternating source order; cache warming and host contention not eliminated",
        "artifacts": build_manifest,
    }
    records = []
    # Rotate source order to avoid permanent first-run advantage.
    for firmware in ("bios", "uefi"):
        for repetition in range(1, args.repetitions + 1):
            order = ("embedded", "media") if repetition % 2 else ("media", "embedded")
            for source in order:
                case_dir = logs / firmware / source / f"repeat-{repetition:02d}"
                case_dir.mkdir(parents=True, exist_ok=True)
                metrics = case_dir / "metrics.json"
                cmd = [
                    sys.executable, str(OBSERVER),
                    "--iso", str(source_files[source]), "--firmware", firmware,
                    "--log-dir", str(case_dir), "--benchmark-json", str(metrics),
                    "--timeout", "240", "--final-marker", "VITA_I007_DEFAULT_TALOS_EXIT_OK",
                    "--required-marker", "VITA_I009_ORPHAN_REAPED_OK",
                    "--required-marker", "VITA_I009B1_ROOT_MOUNT_READONLY_OK",
                    "--required-marker", "VITA_I009_CONTROLLING_TTY_OK",
                ]
                if source == "media":
                    cmd.extend([
                        "--required-marker", "VITA_I009B2_MEDIA_SOURCE_OK",
                        "--required-marker", "VITA_I009B2_MEDIA_MOUNT_MOVED_OK",
                    ])
                print(f"VITA_I009B2B_RUN firmware={firmware} source={source} repetition={repetition}", flush=True)
                subprocess.run(cmd, check=True)
                raw = json.loads(metrics.read_text())
                required = ("tty_ms", "prompt_ms", "host_rss_at_prompt_kib",
                            "host_peak_hwm_kib", "guest_stats")
                if not raw["passed"] or any(raw.get(field) is None for field in required):
                    raise RuntimeError(f"incomplete benchmark proof: {case_dir}")
                if raw["host_peak_hwm_kib"] <= 0 or raw["guest_stats"]["stage"] != "pre_session":
                    raise RuntimeError(f"invalid memory metric semantics: {case_dir}")
                records.append({
                    "firmware": firmware, "source": source, "repetition": repetition,
                    "tty_ms": raw["tty_ms"], "prompt_ms": raw["prompt_ms"],
                    "host_rss_at_prompt_kib": raw["host_rss_at_prompt_kib"],
                    "host_peak_hwm_kib": raw["host_peak_hwm_kib"],
                    "guest_stats": raw["guest_stats"],
                    "raw_log": str(metrics.relative_to(logs)),
                })
                (logs / "raw-runs.jsonl").write_text(
                    "".join(json.dumps(record, sort_keys=True) + "\n" for record in records)
                )
    report = {**context, "successful_runs": len(records), "groups": summarize(records)}
    (logs / "summary.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(f"VITA_I009B2B_BENCH_OK runs={len(records)}", flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
