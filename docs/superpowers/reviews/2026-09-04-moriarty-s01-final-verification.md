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
