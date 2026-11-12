#!/usr/bin/env python3
"""VM-only serial proof: Linux guest -> Vita Rust PID1 -> Talos SDK console."""
from __future__ import annotations

import argparse
import os
import shutil
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


def observe(
    kernel: str | None, initrd: str | None, log_dir: str, timeout: int,
    final_marker: str | None = None, iso: str | None = None, firmware: str = "bios",
    required_markers: tuple[str, ...] = (), ctrl_c_exit: bool = False,
) -> None:
    # I006: direct-kernel path remains intact. I008: genuine ISO boot.
    if iso is None:
        if kernel is None or initrd is None:
            raise ValueError("direct boot needs kernel and initrd")
        kernel_path = require_regular(kernel)
        initrd_path = require_regular(initrd)
        iso_path = None
    else:
        if kernel is not None or initrd is not None:
            raise ValueError("ISO path must not also supply direct kernel/initrd")
        iso_path = require_regular(iso)
        kernel_path = initrd_path = None
    if firmware == "uefi" and iso_path is None:
        raise ValueError("UEFI firmware requires ISO boot")
    logs = Path(log_dir).resolve()
    logs.mkdir(parents=True, exist_ok=True)
    serial_log = logs / "guest-serial.log"
    qemu_log = logs / "qemu.log"
    with tempfile.TemporaryDirectory(prefix="vita-i006-serial-") as temp:
        socket_path = os.path.join(temp, "serial.sock")
        command = [
            "qemu-system-x86_64", "-machine", "accel=tcg", "-m", "1024",
            "-smp", "2", "-nic", "none", "-display", "none",
            "-monitor", "none", "-no-reboot",
            "-serial", f"unix:{socket_path},server=on,wait=off",
        ]
        if iso_path is not None:
            command.extend(["-cdrom", iso_path, "-boot", "order=d"])
            if firmware == "uefi":
                codes = ["/usr/share/OVMF/OVMF_CODE_4M.fd", "/usr/share/OVMF/OVMF_CODE.fd"]
                variables = ["/usr/share/OVMF/OVMF_VARS_4M.fd", "/usr/share/OVMF/OVMF_VARS.fd"]
                code = next((p for p in codes if Path(p).is_file()), None)
                vars_source = next((p for p in variables if Path(p).is_file()), None)
                if not code or not vars_source:
                    raise FileNotFoundError("OVMF firmware images not found")
                vars_copy = os.path.join(temp, "OVMF_VARS.fd")
                shutil.copyfile(vars_source, vars_copy)
                # Isolated firmware pflash only; no target disks or USB devices.
                command.extend([
                    "-drive", f"if=pflash,format=raw,unit=0,readonly=on,file={code}",
                    "-drive", f"if=pflash,format=raw,unit=1,file={vars_copy}",
                ])
        else:
            command.extend([
                "-kernel", kernel_path, "-initrd", initrd_path,
                "-append", "console=ttyS0 rdinit=/init panic=-1 loglevel=5",
            ])
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
                        # Wait for the end of the failing guest line before
                        # terminating QEMU, otherwise a split recv() truncates
                        # the only actionable error.
                        tail = output.split("VITA_I006_GUEST_SESSION_ERROR:", 1)[1]
                        if "\n" in tail:
                            details = tail.split("\n", 1)[0].strip()
                            raise RuntimeError(f"Linux guest init failed: {details}")
                    if ctrl_c_exit:
                        # I009a: the terminal driver must deliver SIGINT to the
                        # foreground group; Talos exits cleanly and its parent
                        # launcher must not count Ctrl+C as a crash.
                        if step == 0 and "VITA_I009_CONTROLLING_TTY_OK" in output and "vita> " in output:
                            client.sendall(b"\x03")
                            step = 1
                        elif step == 1 and "VITA_I007_DEFAULT_TALOS_EXIT_OK" in output:
                            expected_interrupt = [
                                "VITA_I009_TALOS_UID=1000",
                                "VITA_I009_CONTROLLING_TTY_OK",
                                "[exit requested]",
                                "VITA_I009_TALOS_NATIVE_EXIT_OK",
                                "VITA_I006_GUEST_TALOS_SESSION_OK",
                            ]
                            absent = [x for x in expected_interrupt if x not in output]
                            if absent:
                                raise RuntimeError(f"Ctrl+C guest proof missing: {absent}")
                            forbidden = ["VITA_I009_SESSION_RESTART", "VITA_I009_SESSION_SIGNALLED",
                                         "VITA_I009_CRASH_BUDGET_EXHAUSTED"]
                            if any(x in output for x in forbidden):
                                raise RuntimeError("Ctrl+C incorrectly restarted/killed Talos launcher")
                            print("VITA_I009A_CTRL_C_GUEST_OK", flush=True)
                            return
                        continue
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
                        # An optional follow-up boot milestone must be observed
                        # *before* QEMU is terminated; this avoids the race
                        # between the I006 and I007 adjacent guest markers.
                        if final_marker is not None and final_marker not in output:
                            continue
                        # Do not race QEMU shutdown against independent guest
                        # orphan-reaping/late milestone markers.
                        if any(marker not in output for marker in required_markers):
                            continue
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
    parser.add_argument("--kernel")
    parser.add_argument("--initrd")
    parser.add_argument("--iso")
    parser.add_argument("--firmware", choices=("bios", "uefi"), default="bios")
    parser.add_argument("--log-dir", required=True)
    parser.add_argument("--timeout", type=int, default=140)
    parser.add_argument("--final-marker", default=None)
    parser.add_argument("--required-marker", action="append", default=[])
    parser.add_argument("--ctrl-c-exit", action="store_true")
    args = parser.parse_args()
    if args.iso is not None:
        if args.kernel or args.initrd:
            parser.error("--iso cannot be combined with --kernel or --initrd")
    elif not args.kernel or not args.initrd:
        parser.error("supply --iso or both --kernel and --initrd")
    if args.firmware == "uefi" and not args.iso:
        parser.error("--firmware uefi requires --iso")
    try:
        observe(
            args.kernel, args.initrd, args.log_dir, args.timeout,
            args.final_marker, args.iso, args.firmware,
            tuple(args.required_marker), args.ctrl_c_exit,
        )
        return 0
    except Exception as error:
        print(f"VITA_I006_GUEST_ERROR: {error}", file=sys.stderr, flush=True)
        return 1


if __name__ == "__main__":
    sys.exit(main())
