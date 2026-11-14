#!/usr/bin/env python3
"""Diskless real-ISO negative test: stage1 must reject a changed root image."""
from __future__ import annotations

import argparse
import socket
import subprocess
import tempfile
import time
from pathlib import Path


def validate(iso: str, log_dir: str, timeout: int) -> None:
    image = Path(iso).resolve(strict=True)
    if not image.is_file() or image.stat().st_size == 0:
        raise ValueError("missing, empty or nonregular tampered Vita ISO")
    out = Path(log_dir).resolve()
    out.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="vita-i009b1-tamper-") as temp:
        sock = str(Path(temp) / "serial.sock")
        cmd = [
            "qemu-system-x86_64", "-machine", "accel=tcg", "-m", "1024", "-smp", "2",
            "-nic", "none", "-display", "none", "-monitor", "none", "-no-reboot",
            "-serial", f"unix:{sock},server=on,wait=off", "-cdrom", str(image),
            "-boot", "order=d",
        ]
        client = None
        data = bytearray()
        started = time.monotonic()
        with (out / "guest-serial.log").open("wb") as serial, (out / "qemu.log").open("wb") as qemu:
            proc = subprocess.Popen(cmd, stdout=qemu, stderr=subprocess.STDOUT)
            try:
                while time.monotonic() - started < timeout:
                    if proc.poll() is not None:
                        raise RuntimeError(f"guest quit before stage1 fail-closed proof: {proc.returncode}")
                    if client is None:
                        conn = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
                        conn.settimeout(0.5)
                        try:
                            conn.connect(sock)
                            client = conn
                        except OSError:
                            conn.close()
                            time.sleep(0.1)
                            continue
                    try:
                        chunk = client.recv(65536)
                    except socket.timeout:
                        continue
                    if not chunk:
                        raise RuntimeError("guest serial closed unexpectedly")
                    serial.write(chunk)
                    serial.flush()
                    data.extend(chunk)
                    if len(data) > 2_000_000:
                        raise RuntimeError("bounded serial log exceeded 2 MB")
                    output = data.decode("utf-8", errors="replace").replace("\r", "")
                    for forbidden in (
                        "VITA_I009B1_STAGE1_SQUASHFS_MOUNTED",
                        "VITA_I009B1_SWITCH_ROOT_PID1_CONTINUITY_OK",
                        "VITA_I009_TALOS_UID=1000",
                        "VITA_I009B1_ROOT_MOUNT_READONLY_OK",
                        "vita> ",
                    ):
                        if forbidden in output:
                            raise RuntimeError(f"tampered root reached unauthorized state: {forbidden}")
                    if (
                        "VITA_I009B1_ROOT_IMAGE_SHA256_OK" in output
                        or "VITA_I009B1_IMMUTABLE_ROOTFS_GUEST_OK" in output
                    ):
                        raise RuntimeError("tampered root incorrectly passed SHA verification")
                    if (
                        "VITA_I009B1_STAGE1_PID1_READY" in output
                        and "VITA_I009B1_STAGE1_FAIL_CLOSED:" in output
                        and "immutable root image SHA-256 mismatch" in output
                    ):
                        print("VITA_I009B1_TAMPERED_ROOT_REJECTED_VM_OK", flush=True)
                        return
                raise TimeoutError("tampered root was not rejected within guest timeout")
            finally:
                if client is not None:
                    client.close()
                proc.terminate()
                try:
                    proc.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    proc.kill()
                    proc.wait()


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--iso", required=True)
    ap.add_argument("--log-dir", required=True)
    ap.add_argument("--timeout", type=int, default=150)
    a = ap.parse_args()
    validate(a.iso, a.log_dir, a.timeout)
