# VITA-001 — Agent-native portable USB OS

## Type
Epic

## Outcome
Create one portable USB medium that can both boot stored ISO images and boot its own directly usable Agent-native Live OS.

## Product identity
The Live OS is not “a toolbox containing Talos”. Talos Core provides the primary interactive system session and coordinates OS capabilities through a governed execution layer.

## Scope
- Portable boot foundation.
- ISO discovery/boot path.
- Direct Live OS boot path.
- Talos Core-based interaction runtime.
- USB-local model/Agent configuration.
- Governed system capability execution.
- Recovery shell.
- Reproducible image build and validation.

## Out of scope for first round
- GUI desktop as a primary interaction model.
- Autonomous destructive remediation without confirmation.
- Fleet management/cloud control plane.
- Full secure-boot support unless selected by the boot-strategy spike.
- Every Linux distribution/ISO compatibility guarantee.

## Child work
VITA-001-A through VITA-001-H.

## Epic acceptance
- A test USB image can boot in the agreed firmware modes.
- Stored ISO images are discovered and at least one representative ISO boots.
- The project Live OS boots and enters Talos interaction without requiring the user to first launch a terminal command.
- Talos can inspect system state and perform at least one safe system operation through the capability broker.
- A destructive test operation is blocked until explicit confirmation and produces audit evidence.
- Model/Agent config is loaded from the agreed USB directory without requiring secrets in Git.
- Recovery shell remains reachable when Talos is unavailable.
- Build and test evidence is recorded in an iteration/release record.
