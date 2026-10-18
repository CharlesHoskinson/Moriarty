# Votes: exception repair source author

Voter: `claude-opus-5-5`, the security source author of the repaired `operator-proposal/run_r3.py` in this directory, 2026-10-15. I worked in a new context. I authored none of the reviewed operator `d2ddbc3c…`, the effect review or its proposals. I did author the repaired bytes, so I cannot review or approve them. Every seat in this lineage is the same model family, so independence is context-only.

A vote counts only for the exact text it names. These votes add to the existing votes. They do not change or replace any existing vote, dissent or qualification. W-OR-01 DISSENT, Q-2's single first position and every carried limitation stand as recorded. No vote here grants execution, native authority, budget or a counter reset. Root counts the tally.

## A-8

Literal text voted on (from `proposals/OPERATOR-MINIMUM-REPAIRS.md`, "New clause text for the amendment (A-8)"):

> **A-8. Failure propagation and pre-marker refusal.** The operator refuses with exit 2 and writes nothing unless it was started with `python3 -I` as `<task>/tools/run_r3.py`. After it creates `run/`, any unexpected exception becomes a recorded STOP (`operator_exception`). No later step runs, and `finish()` runs exactly once, so the postchecks, the O_EXCL RESULT.json and the guarded post-run still happen. A missing or unreadable Option A source is an identity STOP. None of this is a retry or a limit change.

**Vote: FOR** (second substantive vote; the reviewer's FOR is the first).

Reasons:
- **The clause matches the bytes.** I checked each sentence against the repaired source:
  - refusal before the marker at lines 553–556;
  - marker outside the `try` at 557;
  - `except Exception` recording `operator_exception` and a STOP at 562–564;
  - a single `finish` call at 565;
  - `attempt()` left on the exception, so no later control or step;
  - the E-1 `source unreadable` STOP feeding the existing Option A STOP.

  Nothing in it touches a step, argv, limit, pin, control or helper, so "not a retry or a limit change" holds.
- **It enforces R-C.7 on the paths that matter.** The once-only attempt can no longer lose its RESULT.json and postchecks to a plausible environmental fault, such as a garbage-collected crate, a missing toolchain file or a listing race. The fail-closed property is kept: no later step runs.
- **Keeping the marker outside the `try` is right.** If it were inside, a second invocation that found `run/` already present would reach `finish()`. That `finish()` would `write_text` over the first attempt's listing files and add further records to its `run/`.
- **Catching `Exception` and not `BaseException` is right.** An operator abort (`KeyboardInterrupt`) or `SystemExit` should end the process. It should not go on to spawn ANALYZE.

My reading of the clause:
- "any unexpected exception" means Python `Exception` subclasses. It excludes `BaseException`-only types and signals.
- "writes nothing" means no filesystem write. One `refused_before_marker` line goes to stdout.
- "No later step runs" covers the controls and E0r3…E5-r3. A-4's post-run ANALYZE may still spawn under its existing conditions.

If a later voter reads the clause more broadly than this, the bytes do not meet that reading, and that voter should vote CHANGE. The residuals in REPORT §4 are outside the clause and stay recorded limitations: an exception after a spawn with the child still alive, partial finish exceptions, R-05 build-script exposure and per-step resource scope.

## E-3

Literal text: the replacement `hw_rule` string in `OPERATOR-MINIMUM-REPAIRS.md` E-3.

**Vote: FOR** (second substantive vote). It brings the POST-RESULT string into line with Q-1's general rule for all entries, so a reader of POST-RESULT alone cannot take ALL-REFS-OK on a RELATION-EXECUTED entry that rests on public-field refs (IR-03) as adversarial-witness evidence. It is a string-only change and has no effect on control flow.

## P-AST

Literal text: the root AST-only parse precheck exactly as scoped in the effect review's `REPORT.md` §8:
- the three exact pinned `.py` files to be placed: this repaired `run_r3.py`, `copy_verify_r3.py` `b37bbc1d…` and `libtest_rows.py` `466aeb8a…`;
- each digest compared with its FREEZE record before parsing;
- `/usr/bin/python3 -I -B`, with a root-owned check script outside `<task>` and every packet, and a cwd outside all three;
- `ast.parse(..., mode='exec')`, optionally followed by an in-memory `compile(tree, path, 'exec', dont_inherit=True)` that is then discarded;
- never `import`, `exec`, `eval`, `runpy`, `py_compile`/`compileall`, `__pycache__`, any write in `<task>`, or creation of `run/`/`controls/`/`post/`/`analysis/`/the scratch root;
- a root-owned record of the interpreter version, the three digests and pass/fail per file.

**Vote: FOR** (second substantive vote).

Reasons:
- This repair adds new syntax (`try/except` in `main()` and in `put()`, and a new `def attempt`) that no interpreter has parsed. A syntax error in `run_r3.py` fails before the marker. One in `copy_verify_r3.py` fails C-1 and consumes the once-only attempt.
- The check can only lower that risk. It needs no import, exec or bytecode file, writes nothing in `<task>` and creates no marker. It must run on the bytes root actually places, after the FREEZE is regenerated over the repaired digest.
- A pass means only "parses under that interpreter". It is not behaviour, name-resolution or effect evidence. A failure goes back to fresh authoring, with no placement. The check consumes no counter and grants no authority.

## Not voted

| Item | Reason |
|---|---|
| Q-2 (HW-24) | Outside this source scope. I did not read `HW-TRACEABILITY-2.json` in this context. |
| W-OR-01 (run unrepaired bytes) | I add no vote. The reviewer's DISSENT stands. |
| Repaired operator bytes | I authored them. They need a fresh non-author exact-byte review. |
| A-1…A-7, Q-1, placement, FREEZE shape | Already voted by others. Not reopened. |

## Tally note (my reading; root counts)

| Item | Votes on the identical text after this file |
|---|---|
| A-8 | 2: reviewer FOR (first position) and mine FOR. Both are `claude-opus-5-5`, independent by context only. |
| E-3 | 2: reviewer FOR and mine FOR |
| P-AST | 2: reviewer FOR and mine FOR |
| Q-2 | 1 (reviewer); unchanged |
| W-OR-01 | 1 DISSENT (reviewer); unchanged |
| Repaired `run_r3.py` | No review yet. Execution not authorized. It needs the fresh exact-byte review, the regenerated FREEZE, a P-AST pass and root authorization. |
