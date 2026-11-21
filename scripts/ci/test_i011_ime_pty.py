#!/usr/bin/env python3
"""End-to-end PTY keyboard/resize/signal smoke; no model or physical disk."""
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

TIMEOUT = 15.0
MAX_OUTPUT = 1024 * 1024


def open_session(exe):
    master, slave = pty.openpty()
    fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack("HHHH", 24, 80, 0, 0))
    original = termios.tcgetattr(slave)

    def acquire_tty():
        os.setsid()
        fcntl.ioctl(0, termios.TIOCSCTTY, 0)

    proc = subprocess.Popen(
        [exe], stdin=slave, stdout=slave, stderr=slave,
        preexec_fn=acquire_tty, close_fds=True,
    )
    return master, slave, original, proc


def read_more(fd, data, seconds=0.1):
    if select.select([fd], [], [], seconds)[0]:
        try:
            block = os.read(fd, 32768)
        except OSError as exc:
            if exc.errno == errno.EIO:
                block = b""
            else:
                raise
        if len(data) + len(block) > MAX_OUTPUT:
            raise AssertionError("PTY output exceeded limit")
        data.extend(block)


def until(fd, data, predicate, label, timeout=TIMEOUT):
    end = time.monotonic() + timeout
    while time.monotonic() < end:
        read_more(fd, data)
        if predicate():
            return
    raise AssertionError(
        f"PTY never reached {label}; output suffix={bytes(data[-450:])!r}"
    )


def tty_raw(slave):
    return not bool(termios.tcgetattr(slave)[3] & termios.ICANON)


def assert_tty_restored(slave, original):
    restored = termios.tcgetattr(slave)
    flags = termios.ICANON | termios.ECHO | termios.ISIG
    assert (restored[3] & flags) == (original[3] & flags), \
        "PTY child did not restore ICANON/ECHO/ISIG"


def close_session(master, slave, proc):
    try:
        if proc.poll() is None:
            proc.kill()
            proc.wait(timeout=5)
    finally:
        os.close(master)
        os.close(slave)


def keyboard_run(exe):
    master, slave, original, proc = open_session(exe)
    observed = bytearray()
    try:
        until(master, observed, lambda: tty_raw(slave) and b"S=0" in observed,
              "initial raw TUI with no submitted text")
        os.write(master, b"zhongguo")
        # "国" does not occur in the initial mode label.
        until(master, observed, lambda: "国".encode() in observed,
              "pinyin candidate actually rendered")
        before_commit_count = observed.count("国".encode())
        os.write(master, b"\r")  # only confirm candidate, not submit
        until(master, observed,
              lambda: observed.count("国".encode()) > before_commit_count
              and b"E=2" in observed,
              "committed two-grapheme candidate rendered in editor")
        assert b"S=1" not in observed, \
            "Enter mistakenly submitted the input to the Agent/demo"
        # Repeating Enter cannot submit: only an explicit different gesture.
        os.write(master, b"\r\r")
        read_more(master, observed, 0.2)
        assert b"S=1" not in observed, \
            "Enter auto-repeat caused an unsafe submission"

        os.write(master, b"\x04")  # Ctrl+D: explicit submit in raw mode
        until(master, observed, lambda: b"S=1" in observed,
              "visible submitted-count increment following Ctrl+D")

        # Resizing must produce a *new frame*, not merely a SIGWINCH.
        resized_from = len(observed)
        fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack("HHHH", 12, 40, 0, 0))
        os.kill(proc.pid, signal.SIGWINCH)
        until(master, observed,
              lambda: b"S=1" in observed[resized_from:],
              "resized TUI frame with submitted state")

        os.write(master, b"\x03")  # actual Ctrl+C key byte (raw-mode input)
        proc.wait(timeout=5)
        assert proc.returncode == 0, f"Ctrl+C exit status {proc.returncode}"
        assert_tty_restored(slave, original)
        print("VITA_IME_PTY_OK candidate commit explicit_submit resize ctrl_c termios")
    finally:
        close_session(master, slave, proc)


def signal_run(exe):
    master, slave, original, proc = open_session(exe)
    observed = bytearray()
    try:
        until(master, observed, lambda: tty_raw(slave) and b"S=0" in observed,
              "signal-case initial TUI")
        os.kill(proc.pid, signal.SIGTERM)
        proc.wait(timeout=5)
        assert proc.returncode == 0, f"SIGTERM exit status {proc.returncode}"
        assert_tty_restored(slave, original)
        print("VITA_IME_SIGNAL_OK sigterm termios_restore")
    finally:
        close_session(master, slave, proc)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit("usage: test_i011_ime_pty.py /path/to/vita-ime-input-spike")
    keyboard_run(sys.argv[1])
    signal_run(sys.argv[1])
