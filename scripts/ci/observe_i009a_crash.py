#!/usr/bin/env python3
"""I009a QEMU crash-loop proof: real ISO guest, no network/host disks."""
from __future__ import annotations

import argparse
import socket
import subprocess
import tempfile
import time
from pathlib import Path

SUCCESS = "VITA_I009_CRASH_BUDGET_EXHAUSTED attempts=3"
FAILSAFE = "VITA_I009_FAILSAFE_NO_ROOT_SHELL"


def run(iso: str, log_dir: str, timeout: int, mode: str = "crash") -> None:
    iso_path = Path(iso).resolve(strict=True)
    if not iso_path.is_file() or iso_path.stat().st_size == 0:
        raise ValueError("expected a nonempty regular engineering ISO file")
    logs = Path(log_dir).resolve()
    logs.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="vita-i009a-crash-") as temp:
        sock = str(Path(temp) / "serial.sock")
        cmd = [
            "qemu-system-x86_64", "-machine", "accel=tcg", "-m", "1024", "-smp", "2",
            "-nic", "none", "-display", "none", "-monitor", "none", "-no-reboot",
            "-serial", f"unix:{sock},server=on,wait=off",
            "-cdrom", str(iso_path), "-boot", "order=d",
        ]
        data = bytearray()
        client = None
        started = time.monotonic()
        with (logs / "guest-serial.log").open("wb") as serial, (logs / "qemu.log").open("wb") as qemu:
            process = subprocess.Popen(cmd, stdout=qemu, stderr=subprocess.STDOUT)
            try:
                while time.monotonic() - started < timeout:
                    if process.poll() is not None:
                        raise RuntimeError(f"QEMU quit prematurely: {process.returncode}")
                    if client is None:
                        candidate = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
                        candidate.settimeout(0.5)
                        try:
                            candidate.connect(sock)
                            client = candidate
                        except OSError:
                            candidate.close()
                            time.sleep(0.1)
                            continue
                    try:
                        chunk = client.recv(65536)
                    except socket.timeout:
                        continue
                    if not chunk:
                        raise RuntimeError("serial disconnected")
                    serial.write(chunk)
                    serial.flush()
                    data.extend(chunk)
                    if len(data) > 2_000_000:
                        raise RuntimeError("serial log exceeds 2 MiB bound")
                    output = data.decode("utf-8", errors="replace").replace("\r", "")
                    if "VITA_I006_GUEST_SESSION_ERROR" in output:
                        raise RuntimeError("Linux init failure\n" + output[-2000:])
                    if "VITA_I007_RECOVERY_READY" in output or "vita> " in output:
                        raise RuntimeError("crash fixture unexpectedly opened a session/recovery")
                    if mode == "spawn":
                        # Failed session creation must preserve the main PID 1
                        # loop: it must still reap an adopted grandchild and
                        # process a subsequent guest-delivered SIGTERM.
                        required = (
                            "VITA_I009_SESSION_START_FAILED",
                            "VITA_I009_FAILSAFE_NO_ROOT_SHELL",
                            "VITA_I009_ORPHAN_REAPED_OK",
                            "VITA_I009_SHUTDOWN_REQUESTED",
                        )
                        if all(marker in output for marker in required):
                            if "VITA_I009_TALOS_UID=1000" in output or "VITA_I009_SESSION_RESTART" in output:
                                raise RuntimeError("failed-start fixture spawned an unintended session")
                            print("VITA_I009A_SPAWN_FAILURE_REAPER_VM_OK", flush=True)
                            return
                        continue
                    if SUCCESS in output and FAILSAFE in output:
                        starts = output.count("VITA_I009_SESSION_START attempt=")
                        crashes = output.count("VITA_I009_SIMULATED_SESSION_CRASH")
                        if starts != 3 or crashes != 3:
                            raise RuntimeError(f"unbounded/incomplete crash policy: starts={starts}, crashes={crashes}")
                        print(f"VITA_I009A_CRASH_POLICY_VM_OK starts={starts} crashes={crashes}")
                        return
                raise TimeoutError("timed out waiting for bounded crash-loop marker")
            finally:
                if client is not None:
                    client.close()
                process.terminate()
                try:
                    process.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    process.kill()
                    process.wait()


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--iso", required=True)
    parser.add_argument("--log-dir", required=True)
    parser.add_argument("--timeout", type=int, default=150)
    parser.add_argument("--mode", choices=("crash", "spawn"), default="crash")
    args = parser.parse_args()
    run(args.iso, args.log_dir, args.timeout, args.mode)
