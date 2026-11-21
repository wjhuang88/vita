#!/usr/bin/env python3
"""Real PTY smoke for the isolated Vita IME TUI; no model, QEMU, or USB access."""
import errno
import fcntl
import os
import pty
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
        proc = subprocess.Popen(
            [exe],
            stdin=slave,
            stdout=slave,
            stderr=slave,
            start_new_session=True,
            close_fds=True,
        )
        deadline = time.monotonic() + TIMEOUT
        observed = bytearray()
        sent = False
        resized = False
        stopped = False
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
            if not sent and b"Vita IME PTY spike" in observed:
                # Discrete keypresses: first Enter confirms Pinyin;
                # the second submits a line, and must not call any Agent.
                os.write(master, b"zhongguo")
                time.sleep(0.1)
                os.write(master, b"\r")
                time.sleep(0.1)
                os.write(master, b"\r")
                sent = True
            if sent and not resized and "中国".encode() in observed:
                fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack("HHHH", 12, 40, 0, 0))
                # Explicit resize feedback; serial bytes alone do not resize.
                os.kill(proc.pid, signal.SIGWINCH)
                resized = True
                time.sleep(0.15)
            if resized and not stopped:
                os.write(master, b"\x03")  # Ctrl+C under crossterm raw mode
                stopped = True
            if stopped and proc.poll() is not None:
                break
        if proc.poll() is None:
            raise AssertionError("PTY UI did not exit after Ctrl+C")
        output = bytes(observed).decode("utf-8", errors="replace")
        assert proc.returncode == 0, f"PTY UI exit status {proc.returncode}"
        assert sent, "TUI never rendered its initial screen"
        assert resized, "Chinese candidate/commit not rendered"
        assert "中国" in output, "Pinyin candidate missing in PTY transcript"
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
