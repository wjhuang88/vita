#!/usr/bin/env python3
"""Host-only VM harness. Projects model config into a temporary readonly share.
No Python is shipped in Vita. No physical disks, tap devices or host forwarding.
"""
import argparse
import json
import os
from pathlib import Path
import signal
import socket
import subprocess
import tempfile
import time
import tomllib
from urllib.parse import urlsplit


def project_config(source, environ):
    """Only selected connection fields; deliberately exclude hooks/skills/history."""
    if len(source) > 1024 * 1024:
        raise ValueError("configuration too large")
    config = tomllib.loads(source)
    model = config["model"]
    provider = config["providers"][config["provider"]]
    protocol = provider["protocol"]
    if protocol not in ("openai-chat", "openai-responses"):
        raise ValueError("unsupported protocol")
    if provider.get("tool_protocol", "native") != "native":
        raise ValueError("only native tools are supported")
    endpoint = provider["base_url"]
    url = urlsplit(endpoint)
    if url.scheme != "https" or not url.hostname or url.username or url.password or url.query or url.fragment:
        raise ValueError("HTTPS endpoint without embedded credentials required")
    key = environ.get(provider.get("api_key_env", "")) or provider.get("api_key")
    if not isinstance(key, str) or not key.strip() or len(key) > 8192 or any(ord(c) < 32 or ord(c) == 127 for c in key):
        raise ValueError("credential missing or invalid")
    if not isinstance(model, str) or not model or len(model) > 200 or not model.isprintable():
        raise ValueError("invalid model")
    result = dict(protocol=protocol, model=model, base_url=endpoint, api_key=key)
    settings = provider.get("models", {}).get(model, {})
    if settings.get("reasoning"):
        result["reasoning"] = settings["reasoning"]
    if provider.get("timeout"):
        result["timeout"] = provider["timeout"]
    if len(json.dumps(result).encode()) > 32768:
        raise ValueError("projection too large")
    return result


def stop(process):
    if process and process.poll() is None:
        process.terminate()
        try:
            process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            process.kill()
            process.wait()


def smoke(serial_path, qemu, log_path, secret, invalid=False):
    transcript = bytearray()
    client = socket.socket(socket.AF_UNIX)
    deadline = time.monotonic() + 240
    sent = False
    try:
        while not Path(serial_path).exists():
            if qemu.poll() is not None or time.monotonic() > deadline:
                raise RuntimeError("guest unavailable")
            time.sleep(.1)
        client.connect(str(serial_path))
        client.settimeout(.5)
        while time.monotonic() < deadline:
            try:
                chunk = client.recv(16384)
            except socket.timeout:
                continue
            if not chunk:
                break
            transcript.extend(chunk)
            if len(transcript) > 2_000_000:
                raise RuntimeError("guest log limit")
            if invalid and b"model session failed (details suppressed)" in transcript:
                if secret.encode() in transcript or b"vita> " in transcript:
                    raise RuntimeError("invalid config leaked or reached prompt")
                print("VITA_I010_INVALID_CONFIG_FAIL_CLOSED_OK", flush=True)
                return
            if not sent and b"vita> " in transcript:
                client.sendall(b"Call system_summary to inspect this VM. Report the actual kernel version and memory, then say VITA_MODEL_OK.\n")
                sent = True
            if sent and b"[turn:ok]" in transcript and b"VITA_I010_NATIVE_SYSTEM_SUMMARY_EXECUTED" in transcript:
                text = transcript.decode(errors="replace")
                required = ("VITA_I010_CONFIG_READONLY_OK", "VITA_I010_GUEST_NETWORK_READY",
                            "VITA_I009_TALOS_UID=1000", "[tool] system_summary", "VITA_MODEL_OK", "[turn:ok]",
                            "VITA_I010_NATIVE_SYSTEM_SUMMARY_EXECUTED", "6.18.58-0-virt")
                if not all(marker in text for marker in required):
                    raise RuntimeError("model reply did not prove required guest tool roundtrip")
                print("VITA_I010_REAL_MODEL_TOOL_OK", flush=True)
                return
            if sent and b"[turn:error]" in transcript:
                raise RuntimeError("model turn failed (details suppressed)")
        raise RuntimeError("guest model smoke timed out")
    finally:
        client.close()
        log_path.write_text(transcript.decode(errors="replace").replace(secret, "[REDACTED]"))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--iso", required=True, type=Path)
    parser.add_argument("--config", type=Path, default=Path.home() / ".talos/config.toml")
    parser.add_argument("--logs", required=True, type=Path)
    parser.add_argument("--smoke", action="store_true")
    parser.add_argument("--invalid-config-test", action="store_true",
                        help="boot with malformed synthetic config; no model credential read")
    parser.add_argument("--firmware", choices=("bios", "uefi"), default="bios")
    args = parser.parse_args()
    if os.getuid() == 0:
        raise RuntimeError("run as the desktop user, not root")
    iso = args.iso.resolve(strict=True)
    if not iso.is_file():
        raise RuntimeError("ISO must be a regular file")
    # Read bounded input; no exception containing parser/source text escapes main.
    if args.invalid_config_test:
        if not args.smoke:
            raise RuntimeError("invalid-config test requires --smoke")
        config = {"api_key": "INVALID_CONFIG_CANARY"}
    else:
        with args.config.open("rb") as f:
            source = f.read(1024 * 1024 + 1).decode()
        config = project_config(source, os.environ)
    runtime = Path(f"/run/user/{os.getuid()}")
    if not runtime.is_dir() or runtime.stat().st_uid != os.getuid():
        raise RuntimeError("private runtime directory unavailable")
    args.logs.mkdir(parents=True, exist_ok=True)
    def interrupted(_signum, _frame):
        raise KeyboardInterrupt
    signal.signal(signal.SIGTERM, interrupted)
    daemon = qemu = None
    with tempfile.TemporaryDirectory(prefix="vita-model-", dir=runtime) as temp:
        temp = Path(temp)
        share = temp / "share"
        share.mkdir(mode=0o700)
        config_file = share / "config.json"
        fd = os.open(config_file, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        with os.fdopen(fd, "w") as f:
            json.dump(config, f)
        vhost = temp / "vhost.sock"
        try:
            with (args.logs / "virtiofsd.log").open("w") as dlog, (args.logs / "qemu.log").open("w") as qlog:
                daemon = subprocess.Popen(["/usr/lib/virtiofsd", "--shared-dir", str(share),
                    "--socket-path", str(vhost), "--readonly", "--sandbox", "namespace",
                    "--translate-uid", "map:1000:0:1", "--translate-gid", "map:1000:0:1"], stdout=dlog, stderr=dlog)
                for _ in range(100):
                    if daemon.poll() is not None:
                        raise RuntimeError("readonly file-share daemon failed")
                    if vhost.exists():
                        break
                    time.sleep(.05)
                else:
                    raise RuntimeError("file-share daemon timeout")
                command = ["qemu-system-x86_64", "-name", "Vita real model test", "-machine", "accel=tcg",
                    "-m", "1024", "-smp", "2", "-no-reboot", "-monitor", "none", "-cdrom", str(iso), "-boot", "order=d",
                    "-object", "memory-backend-memfd,id=mem,size=1G,share=on", "-numa", "node,memdev=mem",
                    "-chardev", f"socket,id=fs,path={vhost}", "-device", "vhost-user-fs-pci,chardev=fs,tag=vita-model",
                    "-netdev", "user,id=net", "-device", "virtio-net-pci,netdev=net,mac=52:54:00:76:69:74"]
                if args.firmware == "uefi":
                    import shutil
                    code = Path("/usr/share/edk2/x64/OVMF_CODE.4m.fd")
                    variables = Path("/usr/share/edk2/x64/OVMF_VARS.4m.fd")
                    if not code.exists():
                        code = Path("/usr/share/OVMF/OVMF_CODE_4M.fd")
                        variables = Path("/usr/share/OVMF/OVMF_VARS_4M.fd")
                    shutil.copyfile(variables, temp / "vars.fd")
                    command += ["-drive", f"if=pflash,format=raw,unit=0,readonly=on,file={code}",
                                "-drive", f"if=pflash,format=raw,unit=1,file={temp / 'vars.fd'}"]
                serial = temp / "serial.sock"
                if args.smoke:
                    command += ["-display", "none", "-serial", f"unix:{serial},server=on,wait=off"]
                else:
                    # Interactive model conversations are not logged to disk.
                    command += ["-vga", "none", "-display", "gtk,show-tabs=on", "-serial", "vc"]
                qemu = subprocess.Popen(command, stdout=qlog, stderr=qlog)
                print("Vita VM started: readonly model share, user-mode outbound network, no host disks", flush=True)
                if args.smoke:
                    smoke(serial, qemu, args.logs / "guest-serial.log", config["api_key"], args.invalid_config_test)
                elif qemu.wait() != 0:
                    raise RuntimeError("QEMU failed")
        finally:
            stop(qemu)
            stop(daemon)


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        pass
    except Exception:
        # Never echo TOML, provider response bodies, key values or credential URLs.
        raise SystemExit("VM model test failed; inspect sanitized VM/daemon logs and configuration schema")
