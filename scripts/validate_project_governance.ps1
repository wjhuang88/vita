param([string]$Root = ".")
$ErrorActionPreference = "Stop"
$required = @(
"AGENTS.md","CLAUDE.md","GEMINI.md","EVOLUTION.md",".agent-governance/manifest.yaml","docs/README.md",
"docs/backlog/PRODUCT-BACKLOG.md","docs/iterations/README.md","docs/decisions/README.md","docs/roadmap/IMPLEMENTATION-ROADMAP.md",
"docs/proposals/README.md","docs/reference/README.md","docs/sop/REQUIREMENT-INTAKE.md","docs/sop/START-ITERATION.md",
"docs/sop/ITERATION-WORKFLOW.md","docs/sop/CHANGE-CONTROL.md","docs/sop/LONG-RUNNING-TASK.md","docs/sop/LOCAL-DEV.md",
"docs/sop/NEW-FEATURE.md","docs/sop/DOC-CHECK.md","docs/sop/PAIRING-WORKFLOW.md","docs/sop/TESTING.md",
"docs/sop/EVOLUTION-FEEDBACK.md","docs/sop/GIT-WORKFLOW.md","docs/sop/RELEASE.md")
$failed=$false
foreach($f in $required){ if(-not (Test-Path (Join-Path $Root $f))){ Write-Host "MISSING $f"; $failed=$true } }
$agents=Get-Content (Join-Path $Root "AGENTS.md") -Raw
foreach($s in @("Hard Constraints","Coding Behavior","Git Rules","Task Router","Session End Checklist")){ if(-not $agents.Contains($s)){ Write-Host "AGENTS missing section: $s"; $failed=$true } }
if($failed){ exit 1 }
Write-Host "governance validation: PASS"
