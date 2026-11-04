#!/usr/bin/env python3
"""VM-only guest serial observer. Does not accept physical block devices."""
import argparse
import os
import re
import socket
import stat
import subprocess
import sys
import tempfile
import time
from pathlib import Path

def observe(args):
    source = Path(args.iso or args.disk).resolve(strict=True)
    if not stat.S_ISREG(source.stat().st_mode):
        raise ValueError("QEMU input must be a regular image file, not a block device")
    logs = Path(args.log_dir).resolve()
    logs.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="vita-i002-serial-") as temp:
        socket_path = os.path.join(temp, "serial.sock")
        command = [
            "qemu-system-x86_64", "-machine", "accel=tcg", "-m", "768", "-smp", "2",
            "-nic", "none", "-display", "none", "-monitor", "none", "-no-reboot",
            "-serial", f"unix:{socket_path},server=on,wait=off",
        ]
        if args.iso:
            command += ["-boot", "order=d", "-cdrom", str(source)]
        else:
            command += ["-drive", f"file={source},format=raw,if=ide,snapshot=on"]

        serial_log = logs / f"{args.label}.serial.log"
        qemu_log = logs / f"{args.label}.qemu.log"
        started = time.monotonic()
        transcript = bytearray()
        login_sent = command_sent = False
        serial = None
        with serial_log.open("wb") as serial_file, qemu_log.open("wb") as qemu_file:
            process = subprocess.Popen(command, stdout=qemu_file, stderr=subprocess.STDOUT)
            try:
                while time.monotonic() - started < args.timeout:
                    if process.poll() is not None:
                        raise RuntimeError(f"QEMU exited before guest proof (exit={process.returncode})")
                    if serial is None:
                        candidate = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
                        candidate.settimeout(0.35)
                        try:
                            candidate.connect(socket_path)
                            serial = candidate
                        except (OSError, socket.timeout):
                            candidate.close()
                            time.sleep(0.2)
                            continue
                    try:
                        data = serial.recv(65536)
                        if not data:
                            raise RuntimeError("guest serial channel disconnected")
                        serial_file.write(data)
                        serial_file.flush()
                        transcript.extend(data)
                        if len(transcript) > 2_000_000:
                            del transcript[:-1_000_000]
                    except socket.timeout:
                        pass
                    output = transcript.decode("utf-8", errors="replace").replace("\r", "\n")
                    output = re.sub(r"\x1b\[[0-?]*[ -/]*[@-~]", "", output)
                    if not login_sent and re.search(r"login:\s*$", output):
                        serial.sendall(b"root\r")
                        login_sent = True
                        print("VITA_I002_LOGIN_PROMPT_OK", flush=True)
                    if login_sent and not command_sent and re.search(r"(?m)^[^\n]*#\s*$", output):
                        serial.sendall(b"uname -s; printf 'VITA_I002_GUEST_OK\\n'\r")
                        command_sent = True
                    if command_sent and re.search(r"(?m)^\s*Linux\s*$", output) and re.search(
                        r"(?m)^\s*VITA_I002_GUEST_OK\s*$", output
                    ):
                        print(f"VITA_I002_ALPINE_GUEST_OK label={args.label}", flush=True)
                        return
                raise TimeoutError(f"guest Linux proof not observed within {args.timeout} seconds")
            finally:
                if serial is not None:
                    serial.close()
                process.terminate()
                try:
                    process.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    process.kill()
                    process.wait()
                print(f"VITA_I002_SERIAL_LOG={serial_log}", flush=True)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    inputs = parser.add_mutually_exclusive_group(required=True)
    inputs.add_argument("--iso")
    inputs.add_argument("--disk")
    parser.add_argument("--label", required=True)
    parser.add_argument("--log-dir", required=True)
    parser.add_argument("--timeout", type=int, default=150)
    try:
        observe(parser.parse_args())
    except Exception as exc:
        print(f"VITA_I002_GUEST_ERROR: {exc}", file=sys.stderr, flush=True)
        sys.exit(1)
