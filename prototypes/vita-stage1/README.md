# I009b1 — minimal Rust early init (candidate)

Disposable Linux PID1 stage-1, **not** an agent-facing or production init.
Firmware ISO -> pinned Linux kernel -> Rust stage1 -> load verified same-kernel modules -> verify exact root-image SHA -> read-only loop SquashFS -> move proc/sys/dev/run/tmp -> Linux MS_MOVE+chroot+exec -> permanent vita-init PID1 -> non-root Talos.

Only the kernel and static Rust guest programs are required in the user-facing stack. Stage1 has one narrowly scoped unsafe Linux LOOP_SET_FD ioctl; no model-generated calls, host disk scanning or recovery shell. The separate SquashFS image lives in **initramfs RAM** for this first zero-disk QEMU proof. Moving it to a read-only ISO media source, full Linux userspace, kernel module/firmware coverage, real shutdown, and final PID1-vs-Dinit/Finit choice are later gates.

The digest is embedded at build time and checked inside the guest, which is integrity consistency, **not** a firmware trust chain, signing or verified boot. Exact source + module licenses require human release review.
