#!/usr/bin/env python3
"""Diskless QEMU proof of explicitly selected local, offline root recovery shell.

The response marker must be assembled by a command executed in the guest,
so an echoed command alone is insufficient to pass this test.
"""
from __future__ import annotations

import argparse
import os
import socket
import subprocess
import sys
import tempfile
import time
from pathlib import Path

from observe_i006_guest_session import require_regular

MAX_SERIAL_BYTES = 2_000_000


def observe(kernel: str, initrd: str, log_dir: str, timeout: int) -> None:
    kernel = require_regular(kernel)
    initrd = require_regular(initrd)
    logs = Path(log_dir).resolve()
    logs.mkdir(parents=True, exist_ok=True)
    started = time.monotonic()
    with tempfile.TemporaryDirectory(prefix="vita-i007-serial-") as temporary:
        socket_path = os.path.join(temporary, "serial.sock")
        argv = [
            "qemu-system-x86_64", "-machine", "accel=tcg", "-m", "1024",
            "-smp", "2", "-nic", "none", "-kernel", kernel,
            "-initrd", initrd,
            "-append", "console=ttyS0 rdinit=/init panic=-1 loglevel=5 vita.recovery=1",
            "-display", "none", "-monitor", "none", "-no-reboot",
            "-serial", f"unix:{socket_path},server=on,wait=off",
        ]
        connection = None
        step = 0
        received = bytearray()
        with (logs / "guest-serial.log").open("wb") as serial_file, (
            logs / "qemu.log"
        ).open("wb") as qemu_file:
            guest = subprocess.Popen(argv, stdout=qemu_file, stderr=subprocess.STDOUT)
            try:
                while time.monotonic() - started < timeout:
                    if guest.poll() is not None:
                        raise RuntimeError(f"QEMU exited early: {guest.returncode}")
                    if connection is None:
                        candidate = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
                        candidate.settimeout(0.45)
                        try:
                            candidate.connect(socket_path)
                            connection = candidate
                        except OSError:
                            candidate.close()
                            time.sleep(0.15)
                            continue
                    try:
                        chunk = connection.recv(65536)
                    except socket.timeout:
                        continue
                    if not chunk:
                        raise RuntimeError("guest serial socket unexpectedly closed")
                    serial_file.write(chunk)
                    serial_file.flush()
                    received.extend(chunk)
                    if len(received) > MAX_SERIAL_BYTES:
                        raise RuntimeError("guest serial exceeded the hard 2MB cap")
                    output = received.decode("utf-8", errors="replace").replace("\r", "")
                    if "VITA_I006_GUEST_SESSION_ERROR:" in output:
                        tail = output.split("VITA_I006_GUEST_SESSION_ERROR:", 1)[1]
                        if "\n" in tail:
                            raise RuntimeError(f"guest init error: {tail.split(chr(10), 1)[0]}")
                    if "Vita offline console fixture" in output:
                        raise RuntimeError("Talos fixture started in recovery mode")
                    if step == 0 and all(x in output for x in [
                        "VITA_I007_RUNTIME_MOUNTS_READY",
                        "VITA_I007_RECOVERY_READY uid=0",
                        "VITA_I007_RECOVERY_NOTICE",
                    ]):
                        # The assembled marker is NOT in this input/echo.
                        connection.sendall(
                            b"test \"$(/bin/busybox id -u)\" = 0 && "
                            b"printf 'VITA_I007_%s_%s\\n' RECOVERY ROOT_OK\n"
                        )
                        step = 1
                    elif step == 1 and "VITA_I007_RECOVERY_ROOT_OK" in output:
                        connection.sendall(b"exit\n")
                        step = 2
                    elif step == 2 and "VITA_I007_RECOVERY_EXIT_OK" in output:
                        print("VITA_I007_GUEST_RECOVERY_SHELL_OK uid=0", flush=True)
                        return
                raise TimeoutError(f"recovery guest proof exceeded {timeout}s (step={step})")
            finally:
                if connection is not None:
                    connection.close()
                guest.terminate()
                try:
                    guest.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    guest.kill()
                    guest.wait()
                print(f"VITA_I007_RECOVERY_SERIAL_LOG={logs / 'guest-serial.log'}", flush=True)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--kernel", required=True)
    parser.add_argument("--initrd", required=True)
    parser.add_argument("--log-dir", required=True)
    parser.add_argument("--timeout", type=int, default=140)
    args = parser.parse_args()
    try:
        observe(args.kernel, args.initrd, args.log_dir, args.timeout)
        return 0
    except Exception as error:
        print(f"VITA_I007_RECOVERY_ERROR: {error}", file=sys.stderr, flush=True)
        return 1


if __name__ == "__main__":
    sys.exit(main())
