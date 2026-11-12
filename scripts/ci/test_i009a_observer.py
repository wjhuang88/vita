#!/usr/bin/env python3
"""Serial read boundary tests for I009a fatal VM observer."""
import unittest

from observe_i009a_crash import validate_guest_error


class GuestErrorReadBoundaries(unittest.TestCase):
    def test_prefix_without_colon_is_not_yet_a_complete_marker(self):
        for partial in (
            "VITA_I006_GUEST_SESSION_ERR",
            "VITA_I006_GUEST_SESSION_ERROR",
            "VITA_I006_GUEST_SESSION_ERROR:",
        ):
            validate_guest_error(partial, "fatal")

    def test_expected_fatal_error_may_be_split_before_newline(self):
        for partial in (
            "VITA_I006_GUEST_SESSION_ERROR: VM-only injected",
            "VITA_I006_GUEST_SESSION_ERROR: VM-only injected fatal boot_pid1 failure",
            "VITA_I006_GUEST_SESSION_ERROR: VM-only injected fatal boot_pid1 failure\n",
        ):
            validate_guest_error(partial, "fatal")

    def test_reject_real_fatal_error_after_complete_line(self):
        with self.assertRaisesRegex(RuntimeError, "unexpected fatal boot error"):
            validate_guest_error("VITA_I006_GUEST_SESSION_ERROR: unexpected root failure\n", "fatal")

    def test_other_modes_remain_fail_closed(self):
        with self.assertRaisesRegex(RuntimeError, "Linux init failure"):
            validate_guest_error("VITA_I006_GUEST_SESSION_ERROR: bad mount", "spawn")


if __name__ == "__main__":
    unittest.main()
