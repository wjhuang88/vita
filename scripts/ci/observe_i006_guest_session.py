#!/usr/bin/env python3
"""VM-only serial proof: Linux guest -> Vita Rust PID1 -> Talos SDK console."""
from __future__ import annotations

import argparse
import os
import socket
import stat
import subprocess
import sys
import tempfile
import time
from pathlib import Path

MAX_LOG_BYTES = 2_000_000


def require_regular(path: str) -> str:
    target = Path(path).resolve(strict=True)
    if not stat.S_ISREG(target.stat().st_mode):
        raise ValueError("kernel and initramfs must be regular files, not block devices")
    if target.stat().st_size == 0:
        raise ValueError("kernel or initramfs is empty")
    return str(target)


def observe(kernel: str, initrd: str, log_dir: str, timeout: int) -> None:
    kernel_path = require_regular(kernel)
    initrd_path = require_regular(initrd)
    logs = Path(log_dir).resolve()
    logs.mkdir(parents=True, exist_ok=True)
    serial_log = logs / "guest-serial.log"
    qemu_log = logs / "qemu.log"
    with tempfile.TemporaryDirectory(prefix="vita-i006-serial-") as temp:
        socket_path = os.path.join(temp, "serial.sock")
        command = [
            "qemu-system-x86_64", "-machine", "accel=tcg",
            "-m", "1024", "-smp", "2", "-nic", "none",
            "-kernel", kernel_path, "-initrd", initrd_path,
            "-append", "console=ttyS0 rdinit=/init panic=-1 loglevel=5",
            "-display", "none", "-monitor", "none", "-no-reboot",
            "-serial", f"unix:{socket_path},server=on,wait=off",
        ]
        client = None
        step = 0
        transcript = bytearray()
        started = time.monotonic()
        with serial_log.open("wb") as serial_file, qemu_log.open("wb") as qemu_file:
            process = subprocess.Popen(command, stdout=qemu_file, stderr=subprocess.STDOUT)
            try:
                while time.monotonic() - started < timeout:
                    if process.poll() is not None:
                        raise RuntimeError(f"QEMU quit before guest proof ({process.returncode})")
                    if client is None:
                        candidate = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
                        candidate.settimeout(0.5)
                        try:
                            candidate.connect(socket_path)
                            client = candidate
                        except OSError:
                            candidate.close()
                            time.sleep(0.2)
                            continue
                    try:
                        chunk = client.recv(65536)
                    except socket.timeout:
                        continue
                    if not chunk:
                        raise RuntimeError("guest serial disconnected")
                    serial_file.write(chunk)
                    serial_file.flush()
                    transcript.extend(chunk)
                    if len(transcript) > MAX_LOG_BYTES:
                        raise RuntimeError("guest serial output exceeded 2MB limit")
                    output = transcript.decode("utf-8", errors="replace").replace("\r", "")
                    if "VITA_I006_GUEST_SESSION_ERROR:" in output:
                        raise RuntimeError("Linux guest init reported session startup failure")
                    if step == 0 and "VITA_I006_GUEST_PROC_READY" in output and "vita> " in output:
                        client.sendall(b"inspect\n")
                        step = 1
                    elif step == 1 and output.count("[turn:ok]") >= 1:
                        client.sendall(b"demo:error\n")
                        step = 2
                    elif step == 2 and "[turn:error]" in output:
                        client.sendall(b"inspect again\n")
                        step = 3
                    elif step == 3 and output.count("[turn:ok]") >= 2:
                        client.sendall(b":quit\n")
                        step = 4
                    elif step == 4 and "VITA_I006_GUEST_TALOS_SESSION_OK" in output:
                        expected = [
                            "VITA_I006_GUEST_PID1_OK",
                            "VITA_I006_GUEST_PROC_READY",
                            "Vita offline console fixture",
                            "[tool] system_summary",
                            "[turn:error]",
                            "VITA_CONSOLE_EXIT_OK",
                        ]
                        missing = [item for item in expected if item not in output]
                        if missing or output.count("[turn:ok]") != 2:
                            raise RuntimeError(f"incomplete Talos guest proof: {missing}")
                        print("VITA_I006_LINUX_TALOS_GUEST_OK turns=2 errors=1", flush=True)
                        return
                raise TimeoutError(f"guest session proof not reached within {timeout}s (step={step})")
            finally:
                if client is not None:
                    client.close()
                process.terminate()
                try:
                    process.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    process.kill()
                    process.wait()
                print(f"VITA_I006_SERIAL_LOG={serial_log}", flush=True)


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
        print(f"VITA_I006_GUEST_ERROR: {error}", file=sys.stderr, flush=True)
        return 1


if __name__ == "__main__":
    sys.exit(main())
