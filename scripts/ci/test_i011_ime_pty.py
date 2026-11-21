#!/usr/bin/env python3
"""Real PTY smoke for the isolated Vita IME TUI; no model, QEMU, or USB access."""
import errno
import fcntl
import os
import pty
import re
import select
import signal
import struct
import subprocess
import sys
import termios
import time

TIMEOUT = 20.0


def run(exe: str) -> None:
    master, slave = pty.openpty()
    proc = None
    try:
        fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack("HHHH", 24, 80, 0, 0))
        original = termios.tcgetattr(slave)

        def acquire_tty():
            # Session leader must claim the slave as its controlling tty;
            # setsid alone leaves /dev/tty unavailable to terminal libraries.
            os.setsid()
            fcntl.ioctl(0, termios.TIOCSCTTY, 0)

        proc = subprocess.Popen(
            [exe],
            stdin=slave,
            stdout=slave,
            stderr=slave,
            preexec_fn=acquire_tty,
            close_fds=True,
        )
        deadline = time.monotonic() + TIMEOUT
        observed = bytearray()
        sent = False
        resized = False
        stopped = False
        sent_at = None
        resize_at = None
        while time.monotonic() < deadline:
            if select.select([master], [], [], 0.2)[0]:
                try:
                    chunk = os.read(master, 65536)
                except OSError as exc:
                    if exc.errno != errno.EIO:
                        raise
                    break
                if not chunk:
                    break
                observed.extend(chunk)
            # Terminal painters may interleave cursor/style sequences
            # with title bytes; readiness is *raw input + first draw*,
            # not a fragile substring in an ANSI screen stream.
            raw_ready = not (termios.tcgetattr(slave)[3] & termios.ICANON)
            if not sent and observed and raw_ready:
                # Discrete keypresses: first Enter confirms Pinyin;
                # the second submits a line, and must not call any Agent.
                os.write(master, b"zhongguo")
                time.sleep(0.15)
                os.write(master, b"\r")
                time.sleep(0.15)
                os.write(master, b"\r")
                sent = True
                sent_at = time.monotonic()
            # Ratatui moves the cursor and changes styles between glyphs.
            # A terminal byte stream need not contain contiguous "中国".
            # Keep a bounded settle window and validate UTF-8 glyphs after
            # the process exits, rather than blocking resize/Ctrl+C on
            # contiguous bytes that never need to occur.
            if sent and not resized and time.monotonic() - sent_at >= 0.6:
                fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack("HHHH", 12, 40, 0, 0))
                # Explicit resize feedback; serial bytes alone do not resize.
                os.kill(proc.pid, signal.SIGWINCH)
                resized = True
                resize_at = time.monotonic()
            if resized and not stopped and time.monotonic() - resize_at >= 0.3:
                os.write(master, b"\x03")  # Ctrl+C under crossterm raw mode
                stopped = True
            if stopped and proc.poll() is not None:
                break
        if proc.poll() is None:
            # Safe fixed-fixture transcript only: no credentials/model data.
            debug = bytes(observed[-1200:]).decode("utf-8", errors="replace")
            debug = debug.replace("\x1b", "<ESC>")
            raise AssertionError(
                f"PTY UI did not exit after Ctrl+C; sent={sent}, "
                f"resized={resized}, stopped={stopped}, "
                f"raw_ready={not (termios.tcgetattr(slave)[3] & termios.ICANON)}, "
                f"transcript_head={bytes(observed[:400]).decode('utf-8', errors='replace')!r}, "
                f"transcript_tail={debug!r}"
            )
        output = bytes(observed)
        assert proc.returncode == 0, f"PTY UI exit status {proc.returncode}"
        assert sent, "TUI never rendered its initial screen"
        assert resized and stopped, "PTY resize/Ctrl+C were not actually exercised"
        # The initial mode label contains "中", but not "国".
        # "国" verifies that submitted pinyin produced a distinct CJK glyph.
        assert "国".encode("utf-8") in output, "Pinyin candidate glyph missing in PTY output"
        # Strip ANSI cursor/style sequences before checking contiguous words:
        # terminal renderers can position each grapheme separately.
        readable = re.sub(rb"\x1b\[[0-?]*[ -/]*[@-~]", b"", output).decode(
            "utf-8", errors="replace"
        )
        assert "中国" in readable, "Chinese candidate/commit missing after ANSI normalization"
        # The terminal must restore canonical input and echo on exit.
        restored = termios.tcgetattr(slave)
        masks = termios.ICANON | termios.ECHO | termios.ISIG
        assert (original[3] & masks) == (restored[3] & masks), "termios not restored"
        print("VITA_IME_PTY_OK utf8_candidate resize ctrl_c termios_restore")
    finally:
        if proc is not None and proc.poll() is None:
            proc.kill()
            proc.wait(timeout=5)
        os.close(master)
        os.close(slave)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit("usage: test_i011_ime_pty.py /path/to/vita-ime-input-spike")
    run(sys.argv[1])
