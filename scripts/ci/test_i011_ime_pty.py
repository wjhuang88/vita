#!/usr/bin/env python3
"""Real keyboard/resize/signal PTY smoke with *screen* assertions, no network."""
import codecs
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
import unicodedata

TIMEOUT = 12.0
MAX_OUTPUT = 1024 * 1024


class TerminalScreen:
    """Small VT screen needed for Ratatui's incremental cell-by-cell drawing.

    This is NOT a full ANSI terminal or a production parser. CI uses a fixed
    local fixture with no model output; interpret cursor movement, SGR, clear,
    alt screen and UTF-8 CJK cell widths for concrete visible-state assertions.
    """

    def __init__(self, width=80, height=24):
        self.decoder = codecs.getincrementaldecoder("utf-8")("replace")
        self.width = width
        self.height = height
        self.cells = [[" "] * width for _ in range(height)]
        self.x = 0
        self.y = 0
        self.state = "text"
        self.params = ""

    def resize(self, width, height):
        self.width = width
        self.height = height
        self.cells = [[" "] * width for _ in range(height)]
        self.x = 0
        self.y = 0

    def text(self):
        return "\n".join("".join(row).rstrip() for row in self.cells)

    @staticmethod
    def _number(params, i, default):
        fields = params.lstrip("?").split(";")
        if i >= len(fields) or not fields[i]:
            return default
        return int(fields[i]) if fields[i].isdigit() else default

    def csi(self, final, params):
        first = self._number(params, 0, 1)
        if final in ("H", "f"):
            self.y = max(0, min(self.height - 1, first - 1))
            self.x = max(0, min(self.width - 1, self._number(params, 1, 1) - 1))
        elif final == "G":
            self.x = max(0, min(self.width - 1, first - 1))
        elif final == "A":
            self.y = max(0, self.y - first)
        elif final == "B":
            self.y = min(self.height - 1, self.y + first)
        elif final == "C":
            self.x = min(self.width - 1, self.x + first)
        elif final == "D":
            self.x = max(0, self.x - first)
        elif final == "J" and first == 2:
            self.cells = [[" "] * self.width for _ in range(self.height)]
        elif final == "K":
            end = self.width if first == 0 else self.width
            for i in range(self.x, end):
                self.cells[self.y][i] = " "
        elif final == "h" and params in ("?1049", "?1047"):
            self.cells = [[" "] * self.width for _ in range(self.height)]
            self.x = self.y = 0
        # m, ?25h/?25l, ?2004h, and other cosmetic codes intentionally ignored.

    def feed(self, payload):
        for ch in self.decoder.decode(payload):
            if self.state == "text":
                if ch == "\x1b":
                    self.state = "escape"
                elif ch == "\r":
                    self.x = 0
                elif ch == "\n":
                    self.x = 0
                    self.y = min(self.height - 1, self.y + 1)
                elif ch == "\b":
                    self.x = max(0, self.x - 1)
                elif ch >= " " and ch != "\x7f":
                    if unicodedata.combining(ch):
                        continue
                    cell_width = 2 if unicodedata.east_asian_width(ch) in ("W", "F") else 1
                    if self.x < self.width:
                        self.cells[self.y][self.x] = ch
                        if cell_width == 2 and self.x + 1 < self.width:
                            # Continuation occupies a terminal cell, not a
                            # literal visible space between Chinese glyphs.
                            self.cells[self.y][self.x + 1] = ""
                    self.x += cell_width
            elif self.state == "escape":
                if ch == "[":
                    self.state = "csi"
                    self.params = ""
                elif ch == "]":
                    self.state = "osc"
                else:
                    self.state = "text"
            elif self.state == "csi":
                if "@" <= ch <= "~":
                    self.csi(ch, self.params)
                    self.state = "text"
                else:
                    self.params += ch
                    if len(self.params) > 50:
                        raise AssertionError("unexpected long ANSI control")
            elif self.state == "osc":
                if ch == "\x07":
                    self.state = "text"
                elif ch == "\x1b":
                    self.state = "osc_escape"
            elif self.state == "osc_escape":
                self.state = "text" if ch == "\\" else "osc"


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


def read_more(fd, output, screen, seconds=0.1):
    if select.select([fd], [], [], seconds)[0]:
        try:
            chunk = os.read(fd, 32768)
        except OSError as exc:
            if exc.errno == errno.EIO:
                chunk = b""
            else:
                raise
        if len(output) + len(chunk) > MAX_OUTPUT:
            raise AssertionError("PTY output exceeded limit")
        output.extend(chunk)
        screen.feed(chunk)


def until(fd, output, screen, predicate, label, timeout=TIMEOUT):
    end = time.monotonic() + timeout
    while time.monotonic() < end:
        read_more(fd, output, screen)
        if predicate():
            return
    raise AssertionError(
        f"PTY did not reach {label}; screen={screen.text()!r}, "
        f"suffix={bytes(output[-180:])!r}"
    )


def tty_raw(slave):
    return not bool(termios.tcgetattr(slave)[3] & termios.ICANON)


def assert_tty_restored(slave, original):
    restored = termios.tcgetattr(slave)
    flags = termios.ICANON | termios.ECHO | termios.ISIG
    assert (restored[3] & flags) == (original[3] & flags), \
        "PTY process did not restore ICANON/ECHO/ISIG"


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
    output = bytearray()
    screen = TerminalScreen()
    try:
        until(master, output, screen,
              lambda: tty_raw(slave) and "S=0 E=0" in screen.text(),
              "initial raw TUI with empty editor")
        os.write(master, b"zhongguo")
        until(master, output, screen,
              lambda: "中国" in screen.text() and "zhongguo" in screen.text(),
              "visible pinyin candidates and composition")
        os.write(master, b"\r")  # select first candidate, no submission
        until(master, output, screen,
              lambda: "S=0 E=2" in screen.text()
              and "文本: 中国" in screen.text()
              and "zhongguo" not in screen.text(),
              "committed Chinese in editor without submission")
        # Enter repeat, a real hazard on legacy PTYs, cannot submit.
        os.write(master, b"\r\r")
        read_more(master, output, screen, 0.25)
        assert "S=0 E=2" in screen.text(), \
            "auto-repeated Enter wrongly submitted committed text"

        os.write(master, b"\x04")  # Ctrl+D must be an explicit distinct gesture
        until(master, output, screen,
              lambda: "S=1 E=0" in screen.text()
              and "中国" in screen.text(),
              "explicit submission shown in the submitted panel")

        # Clear model before actual resize. A new screen repaint must
        # populate the state; a mere SIGWINCH does NOT fulfill the check.
        screen.resize(40, 12)
        fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack("HHHH", 12, 40, 0, 0))
        os.kill(proc.pid, signal.SIGWINCH)
        until(master, output, screen,
              lambda: "S=1 E=0" in screen.text()
              and "Editor" in screen.text(),
              "new visible frame after PTY resize")

        os.write(master, b"\x03")
        proc.wait(timeout=5)
        assert proc.returncode == 0, f"Ctrl+C status {proc.returncode}"
        assert_tty_restored(slave, original)
        print("VITA_IME_PTY_OK ime_commit explicit_submit resize ctrl_c termios")
    finally:
        close_session(master, slave, proc)


def unsolicited_paste_run(exe):
    # Even without requesting bracketed paste, a remote/unsolicited
    # terminal may send ESC[200~ and omit its closing ESC[201~.
    # The stream parser must never buffer the whole payload or swallow ^C.
    master, slave, original, proc = open_session(exe)
    output = bytearray()
    screen = TerminalScreen()
    try:
        until(master, output, screen,
              lambda: tty_raw(slave) and "S=0 E=0" in screen.text(),
              "unsolicited-paste initial TUI")
        os.write(master, b"\x1b[200~")
        # Separate marker and contents, ensuring the open marker enters
        # the reader rather than relying on terminal paste negotiation.
        time.sleep(0.10)
        os.write(master, b"Z" * 4096)
        for _ in range(6):
            read_more(master, output, screen, 0.10)
        assert "S=0 E=0" in screen.text(), \
            "unsolicited incomplete paste mutated committed editor"
        assert "拼音:" in screen.text(), "terminal lost original UI"
        os.write(master, b"\x03")
        proc.wait(timeout=5)
        assert proc.returncode == 0, \
            f"Ctrl+C swallowed by unfinished paste; status={proc.returncode}"
        assert_tty_restored(slave, original)
        print("VITA_IME_UNSOLICITED_PASTE_OK bounded_discard ctrl_c termios")
    finally:
        close_session(master, slave, proc)


def signal_run(exe):
    master, slave, original, proc = open_session(exe)
    output = bytearray()
    screen = TerminalScreen()
    try:
        until(master, output, screen,
              lambda: tty_raw(slave) and "S=0 E=0" in screen.text(),
              "signal-case initial TUI")
        os.kill(proc.pid, signal.SIGTERM)
        proc.wait(timeout=5)
        assert proc.returncode == 0, f"SIGTERM status {proc.returncode}"
        assert_tty_restored(slave, original)
        print("VITA_IME_SIGNAL_OK sigterm termios_restore")
    finally:
        close_session(master, slave, proc)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit("usage: test_i011_ime_pty.py /path/to/vita-ime-input-spike")
    keyboard_run(sys.argv[1])
    unsolicited_paste_run(sys.argv[1])
    signal_run(sys.argv[1])
