# S01 final verification

Classification: repository and experiment observation. This record concerns
the S01 specification package and local transfer experiment, not the 24 XML
release gates. Whole-package independent review was still in progress when
these commands ran; its disposition belongs in the review ledger.

## Tested boundary

- Commit: `89848aef54616dbc3833e37fceef3a1d541168f2`.
- Original S01 branch base: `40f1e5a40ac939770d5fc5a0520b060d3313ec95`.
- Working directory: `/home/charl/Moriarty/.worktrees/s01-audit-start`.
- Interpreter: `/home/charl/Moriarty/.venv/bin/python`.
- Active semantic scope: `0.0.0-e00.2`.
- Scope SHA-256: `9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a`.

## Controller verification

`/home/charl/Moriarty/.venv/bin/python scripts/validate_s01_intent_evidence.py`
exited 0. Output status was `recomputed-package-gate-passed`; every named gate
from `S01-01` through `S01-10` was true. The command was read-only.

`/home/charl/Moriarty/.venv/bin/python -m pytest -q` exited 0:

```text
278 passed in 9.31s
```

The protected-input comparison exited 0 with no output:

```bash
git diff --exit-code 40f1e5a40ac939770d5fc5a0520b060d3313ec95..HEAD -- moriarty/core.py moriarty/swap.py moriarty/compact.py moriarty/backend.py evidence/semantic-scope experiments/moriarty-core-swap
```

The placeholder scan exited 1 with no matches:

```bash
rg -n 'TB[D]|TO[D]O|FIXM[E]|implement late[r]|fill i[n]' openspec/changes/s01-intent-theorem-freeze schemas/intent evidence/s01-intent-theorem-freeze moriarty/intent.py scripts/validate_s01_intent_evidence.py
```

`git status --short` had no output after tests. In particular, the tests made
no new change to the historical graph receipt.

## Explicit whitespace exception

The unmodified default whole-branch `git diff --check` exited 2 for six extra
blank lines at end of file: the ambiguity, assumption, hard-predicate,
observation, and preference registry JSON files, plus the OpenSpec metadata
file. These are previously reviewed S01 bytes, not new Task 6 whitespace.
Five of the files are independently pinned normative outputs. Preserve their
reviewed bytes instead of replacing their pins for a style-only change.

The following command retained all other whitespace checks and exited 0 with
no output:

```bash
git -c core.whitespace=-blank-at-eof diff --check 40f1e5a40ac939770d5fc5a0520b060d3313ec95..HEAD
```

The independent reviewer agreed that this explicit exception has no material
effect on S01 gate soundness. It is not a claim that the default check passed.

## Boundary preserved

The candidate theorem remains unmechanized. The baseline and extra-effect
mutant both deny signing. Complete authenticated effect extraction, the full
runtime verifier, proof/backend/ledger correspondence, ACTUS completeness,
human preference, and S02 architecture selection are not established here.

S02 preparation uses Quint and Apalache under the user's direction. No S02
model result or selected architecture follows from this verification record.

## Corrected final boundary

The whole-package review found a settlement-process/receipt alias mismatch.
Commit `5d3863793660d551b3e30a88e322d5c9a497d33c` corrected it. Independent
re-review approved all code and normative artifacts, checked the four new
regressions, and verified all 20 pins against committed bytes. Only the
terminology pin changed; the other 19 remained unchanged. There is no remaining
material code or normative finding.

The corrected manifest file SHA-256 is
`b1a5ac79236fd4fc95a689e3e78e32ad0dfab9a7db3c84a54919e63749adf8ef`.
The prior manifest remains available as SRC-0032 at its archived local path;
SRC-0034 identifies the corrected current manifest. Both local certificates
still deny signing. The schema and Core scope did not change.

After the provenance and S01 completion-status edits, the controller repeated
the read-only validator and full suite in the same worktree. The validator
again exited 0 with all ten gates true. The full-suite command above exited 0:

```text
282 passed in 9.58s
```

The completion-status test first failed on the old in-progress README, then
all three OpenSpec tests passed after the status correction. Separate read-only
checks verified unique source IDs, the actual SHA-256 of SRC-0032/33/34, changed
wiki metadata and local links, and exact program inventories of 15 sprints,
24 release gates, and 22 deliverables against the XML. Every release gate
remained unassessed with an empty evidence list. The register moves from
completed S01 to in-progress S02, without recording any S02 model result.

The uncommitted-diff whitespace check passed. The whole-branch check with the
documented EOF-only exception and the protected Core/backend/scope/E00
comparison also exited 0. Test runs made no new graph-receipt change.

This completes Task 7's bounded S01 verification. The next obligation is the
reviewed S02 contract subproject followed by the four Quint model experiments.
