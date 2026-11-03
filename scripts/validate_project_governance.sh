#!/bin/sh
set -eu
ROOT="${1:-.}"
fail=0
required='AGENTS.md CLAUDE.md GEMINI.md EVOLUTION.md .agent-governance/manifest.yaml docs/README.md docs/backlog/PRODUCT-BACKLOG.md docs/iterations/README.md docs/decisions/README.md docs/roadmap/IMPLEMENTATION-ROADMAP.md docs/proposals/README.md docs/reference/README.md docs/sop/REQUIREMENT-INTAKE.md docs/sop/START-ITERATION.md docs/sop/ITERATION-WORKFLOW.md docs/sop/CHANGE-CONTROL.md docs/sop/LONG-RUNNING-TASK.md docs/sop/LOCAL-DEV.md docs/sop/NEW-FEATURE.md docs/sop/DOC-CHECK.md docs/sop/PAIRING-WORKFLOW.md docs/sop/TESTING.md docs/sop/EVOLUTION-FEEDBACK.md docs/sop/GIT-WORKFLOW.md docs/sop/RELEASE.md'
for f in $required; do
  if [ ! -f "$ROOT/$f" ]; then echo "MISSING $f"; fail=1; fi
done
for section in 'Hard Constraints' 'Coding Behavior' 'Git Rules' 'Task Router' 'Session End Checklist'; do
  if ! grep -q "$section" "$ROOT/AGENTS.md"; then echo "AGENTS missing section: $section"; fail=1; fi
done
if ! grep -q 'profile: "high-risk"' "$ROOT/.agent-governance/manifest.yaml"; then echo 'manifest profile mismatch'; fail=1; fi
if ! grep -q 'skill_version: "2.0.0"' "$ROOT/.agent-governance/manifest.yaml"; then echo 'manifest skill version missing'; fail=1; fi
if ! grep -q 'Talos Core' "$ROOT/docs/decisions/ADR-0001-talos-primary-interaction-runtime.md"; then echo 'Talos architecture decision missing'; fail=1; fi
if ! grep -q 'explicit confirmation' "$ROOT/docs/sop/PRIVILEGED-OPERATIONS.md"; then echo 'privileged confirmation gate missing'; fail=1; fi
if [ "$fail" -ne 0 ]; then exit 1; fi
echo 'governance validation: PASS'
