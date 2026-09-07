# MC01 replacement worker after a skill-routing abort

Status: concrete proposal, awaiting explicit authorization.

## Observed failure

Worker 04 exited before reading the Moriarty audits or editing any source.
It selected a skill restricted to the active bridge repository.
It then stopped because Moriarty has no bridge package files.
All nine source hashes match the rejected candidate. The worktree is clean.
The host retains the full 480-second charge despite the 68-second early exit.
The Fable canary passed. No substantive review was run on unchanged source.

## Proposed exception

Authorize one replacement worker using the existing 330-second headroom.
Increase MC01's dispatch allowance from four to five for this startup failure only.
Increase the master dispatch allowance from 24 to 25. Do not borrow another package's allowance.
Keep the existing MC01 80-minute and master 510-minute ceilings.
Retain every prior charge and the existing verification and audit allocations.
The Fable 600-second charge already includes its completed readiness canary.
Its substantive review must fit the unused portion of that same bound.

## Concrete command correction

Use the same isolated worktree, absolute Codex Sol executable, strong containment and systemd memory/CPU limits.
Use a 310-second launcher timeout and a 320-second service limit within the 330-second full-bound charge.
Add this explicit scope statement before the existing five-part correction task:

> This task concerns Moriarty, not the bridge repository. The bridge-formal-methods skill explicitly applies only in the active bridge repository. It does not apply here. Do not install its repository-local files. Apply Moriarty AGENTS.md and docs/FOOTGUNS.md. This is the same approved source-profile correction, with no Lean or Quint editing. A skill for another repository does not supersede these task instructions.

Retain the original owned paths, corrections, checks, report requirements and independent Fable/GPT-6 reviews.
Write corrected files and the report within 300 seconds.
Do not change installed skills, Codex configuration or Foreman.
This is a proposed route correction, not an executed or proven fix.
Any terminal failure stops the pass. No further worker allowance is implicit.
