# Git Workflow

Current mode: simple feature branches; worktrees recommended for parallel Agent work but not mandatory.

- Branch from current `main` after checking remote state.
- Prefer `feat/<story-id>-<slug>`, `fix/<story-id>-<slug>`, `docs/<story-id>-<slug>`.
- Inspect staged diff before commit.
- Agent-authored/assisted commit format: `type(scope): description (STORY-ID) [model:<model-name>]`.
- Keep one coherent story/slice per PR when practical.
- Do not force-push shared branches unless explicitly authorized.
- High-risk PRs follow `PAIRING-WORKFLOW.md`.
