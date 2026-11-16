#!/usr/bin/env python3
"""Deterministic offline tests for I009b2b summary and grouping semantics."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from benchmark_i009b2b import summarize, verify_source_identity
from observe_i006_guest_session import read_qemu_memory, parse_guest_stats


class BenchmarkTest(unittest.TestCase):
    def test_grouping_and_medians(self):
        rows = []
        for firmware in ("bios", "uefi"):
            for source in ("embedded", "media"):
                for i in range(3):
                    rows.append({
                        "firmware": firmware, "source": source,
                        "tty_ms": 10 + i, "prompt_ms": 20 + i,
                        "host_rss_at_prompt_kib": 100 + i,
                        "host_peak_hwm_kib": 200 + i,
                        "guest_stats": {
                            "stage": "pre_session", "mem_available_kib": 900 - i,
                            "processes": 3 + i, "uptime_s": float(i),
                        },
                    })
        groups = summarize(rows)
        self.assertEqual(set(groups), {
            "bios/embedded", "bios/media", "uefi/embedded", "uefi/media"
        })
        self.assertEqual(groups["bios/embedded"]["repetitions"], 3)
        self.assertEqual(groups["bios/embedded"]["metrics"]["prompt_ms"],
                         {"median": 21, "min": 20, "max": 22})
        self.assertEqual(
            groups["uefi/media"]["metrics"]["guest_pre_session_mem_available_kib"]["median"], 899
        )

    def test_split_serial_stats_are_ignored_until_newline(self):
        prefix = "VITA_I009_BENCH_GUEST_STATS stage=pre_session "
        incomplete = prefix + "mem_total_kib=1024 "
        self.assertIsNone(parse_guest_stats(incomplete))
        almost = incomplete + "mem_available_kib=768 processes=4 uptime_s=12."
        self.assertIsNone(parse_guest_stats(almost))
        self.assertEqual(parse_guest_stats(almost + "34\n"), {
            "stage": "pre_session", "mem_total_kib": 1024,
            "mem_available_kib": 768, "processes": 4, "uptime_s": 12.34,
        })

    def test_root_source_identity_rejects_duplicate_media_iso(self):
        media = ("VITA_I009B2_MEDIA_SOURCE_OK\n"
                 "VITA_I009B2_MEDIA_MOUNT_MOVED_OK\n")
        verify_source_identity("media", media)
        verify_source_identity("embedded", "VITA_I009B1_ROOT_MOUNT_READONLY_OK\n")
        with self.assertRaises(ValueError):
            verify_source_identity("embedded", media)
        with self.assertRaises(ValueError):
            verify_source_identity("embedded", "VITA_I009B2_MEDIA_SOURCE_OK\n")
        with self.assertRaises(ValueError):
            verify_source_identity("media", "VITA_I009B2_MEDIA_SOURCE_OK\n")
        with self.assertRaises(ValueError):
            verify_source_identity("unknown", "")

    def test_memory_probe_invalid_pid(self):
        self.assertEqual(read_qemu_memory(-42), {})


if __name__ == "__main__":
    unittest.main()
