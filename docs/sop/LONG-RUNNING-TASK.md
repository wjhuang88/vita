# Long-running Task

Use for work spanning 3+ ordered phases, multiple systems/worktrees, or durable resume checkpoints.

Before starting, record:
- final outcome and stop condition;
- complete phase inventory;
- dependencies and destructive/privileged steps;
- verification per phase;
- durable checkpoint location.

Execution rules:
- Advance autonomously through unblocked phases.
- Persist checkpoints in the owning issue/iteration/story, not private scratch state.
- Stop on a safety confirmation gate, missing permission, contradictory requirement or failed prerequisite that makes continued work invalid.
- On interruption, resume from recorded evidence rather than reconstructing from memory.
