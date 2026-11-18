# Local real-model VM trial (I010)

This is an opt-in engineering test, not a released Live OS or USB configuration contract.
Build the existing media-rootfs fixture with `VITA_MODEL_TEST=1`, `VITA_I009_BENCH_BUILD_ONLY=1`, and a pre-created `/tmp/vita-i009b2b.*` export directory. The build needs the same tools as I009b2 plus public CA certificates. It packages Rust Talos provider adapters and virtio-net/virtiofs modules, never model credentials.

Launch as your ordinary desktop user (requires QEMU GTK/user-network/virtiofs support and Rust virtiofsd):

```sh
python3 scripts/dev/run_model_vm.py --iso /path/to/media.iso --logs target/model-vm
```

By default reads `~/.talos/config.toml`; `--config` selects another local file. Only selected model/protocol/HTTPS URL, selected-model reasoning, bounded timeouts, and resolved credential are projected. A nonempty configured environment variable takes precedence over inline credential. Supported protocols: `openai-chat`, `openai-responses`, native tools only. Hooks, skills, MCP, history, other providers and host workspaces are excluded.

A private per-user runtime directory holds the projected JSON mode 0600. virtiofsd serves it readonly in a namespace sandbox; the guest mounts it readonly/nosuid/nodev/noexec. Closing the QEMU window terminates the share daemon and removes temporary configuration. Force-killing the launcher or host crash may leave a private runtime file until logout/reboot; inspect `/run/user/<uid>/vita-model-*` after abnormal termination. Never package these directories.

Only a compiled `model-test` feature plus exact `vita.model-test=1` boot token enables guest network setup. Fixed guest NIC identity and addresses are configured via Rust netlink. QEMU uses user-mode outbound networking with no forwarded ports, physical disks, tap/bridge or host network changes. This permits outbound access including host/LAN; it is not a destination allowlist.

The normal interactive path does not log conversations. `--smoke` sends a non-sensitive VM system-inspection prompt and saves a credential-redacted serial transcript. `--smoke --invalid-config-test` uses synthetic malformed configuration and never reads the real credential. `--firmware uefi` uses OVMF. Native guest tool execution, readonly root/share and UID1000 remain required.

Try asking `请调用 system_summary，告诉我当前虚拟机的内核和内存。` The only available tool is readonly system_summary; no shell/filesystem/network-modification tools. `:cancel` interrupts and `:quit` ends the session. Close the window to stop the VM.

Security limitation: pinned Talos clients may follow redirects; HTTPS is validated initially but endpoint confinement/downgrade prevention is not proven. Use only non-sensitive trial prompts with your trusted configured provider. [Transport follow-up](../../docs/backlog/active/VITA-001-E-transport-policy.md) remains an open security/release gate.
