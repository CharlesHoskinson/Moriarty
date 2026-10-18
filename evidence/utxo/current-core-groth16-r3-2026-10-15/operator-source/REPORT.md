# r3 operator exception repair: source report

Author: `claude-opus-5-5` (requested effort high; I cannot observe the actual effort), 2026-10-15, as the security source author for this minimal repair. I started in a new context, with no subagents. I used only Read, Write and Edit. Every author, reviewer and voter in this lineage is the same model family. Independence is context-only. It is not independence between vendors.

The bytes are **UNREVIEWED, UNPARSED and NEVER RUN.** I make no claim about tested behaviour or syntax. Every statement below comes from my reading of the text. I have no hashing tool. Root hashes and diffs the result after this task ends.

## 1. Callable predicate and answer

*Do the repaired operator bytes implement exactly E-1, E-2 (a–d) and the optional E-3 from `current-core-groth16-r3-operator-independent-effect-review-20261015/proposals/OPERATOR-MINIMUM-REPAIRS.md`? Do they preserve all unrelated source, steps, controls, limits and interfaces, so that a fresh reviewer can assess exception recording and refusal before the marker?*

**My reading: yes.** Root's mechanical diff must confirm it against the seed `d2ddbc3c…`. I applied each edit to the seeded copy with an exact-match Edit, in the order E-1, E-2a, E-2b, E-2c, E-2d, E-3. Each single edit matched its `old` text uniquely. The E-2c replace-all ran after E-2a and E-2b. Afterwards I read back the whole `main()`/`attempt()` region:
- E-2c turned exactly ten eight-space `return finish(result)` lines into bare `return`s, at lines 573, 578, 583, 588, 593, 609, 614, 625, 634 and 637.
- `main()` keeps its one four-space `return finish(result)` (line 565).
- `attempt()` ends with the step loop (line 652).

No other byte was edited.

## 2. What changed (repaired `operator-proposal/run_r3.py`)

| Edit | Location (repaired file) | Change |
|---|---|---|
| E-1 | `materialize().put()`, lines 302–307 | The source read now sits inside `try/except OSError`. A missing or unreadable source appends `source unreadable: <src>: <repr>` to `stops` and returns, so the loop goes on to the remaining entries. The existing `Option A materialization identity STOP` in `attempt()` then stops everything later. |
| E-2a | end of the former `main()` | The trailing `return finish(result)` after the step loop is removed. The loop is now the last statement of `attempt()`. |
| E-2b | `main()` lines 550–565; `attempt()` from line 568 | Before the marker: if `sys.flags.isolated` is false, `T.name != TASK_NAME` or `SELF != OPERATOR`, the operator prints one `refused_before_marker` JSON line to stdout and returns 2. Then comes `RUN.mkdir()`, kept outside the `try`. The `result` dict is built next. Inside `try`: the `start` journal line and `attempt(result)`. `except Exception` records `operator_exception` and one STOP. `finish(result)` is called once, and `main()` returns its value. |
| E-2c | ten early returns in `attempt()` | `return finish(result)` → `return`. |
| E-2d | module docstring, Effects list | Two lines that record the refusal before the marker and the exception-to-STOP rule. |
| E-3 (optional, applied) | `post_run()` `hw_rule`, lines 737–739 | POST-RESULT now quotes Q-1's general rule for all entries: ALL-REFS-OK "never implies that adversarial internal witnesses were executed for any HW entry". The sentence about RELATION-PARTIAL is kept. |

**Unchanged by my reading:**
- every constant, `STEPS`, `DEPS`, `ENV`, `WRAP` and `AS`/`WALL`;
- every argv, `EXPECT`, `KNOWN`, `FREEZE_KEYS` and the controls and their check functions;
- `prestart()` (it keeps its own `-I` and placement STOPs as the record), `proposal_mismatches()`, `run()`, `e0_step()`, `control()`, `finish()` and `post_run()` apart from the E-3 string;
- the header docstring outside the E-2d lines. It still names the earlier operator source author and says "UNREVIEWED and NEVER RUN". That remains true, and changing it was not part of the minimum repair.

The other six seeded files are untouched. Root's hash comparison is authoritative.

## 3. How the repair works, for a fresh reviewer

**Refusal before the marker (OR-02).** The three conditions are the same ones `prestart()` uses. They are evaluated before any filesystem write. `SELF` is `Path(__file__).resolve()` and `T` derives from it, so a symlink to the real placed file resolves to the real path. A copy at any other path, or a run without `-I` (the author packet included), exits 2 and creates no file or directory. "Writes nothing" means nothing on the filesystem. One line goes to stdout.

**Exceptions after the marker (OR-01, R-C.7).**
- `RUN.mkdir()` stays outside the `try` on purpose. If `run/` already exists, the once-only refusal must not reach `finish()`. Otherwise `snapshot('after')` would `write_text` over a prior attempt's listing files and could add further records to that attempt's `run/`.
- Any `Exception` raised from the start journal line through the last step goes to the handler:
  - E-1 now covers the unreadable-source case;
  - other examples are `toolchain()` on a missing pin file, `snapshot('before')` on a missing `settings.toml` or a walk race, the `xb` log opens, `journal()`, `json.loads(plan_b)`, and a destination-side `FileExistsError` in `put()`.
- The handler records `operator_exception` and a STOP. No later control or step runs, because `attempt()` has been left.
- `finish()` then runs its six guarded postchecks, the guarded finish journal line, the O_EXCL `RESULT.json` and the guarded `post_run()`, exactly as on designed paths.
- `post_run()` still spawns ANALYZE only under its existing conditions: `prestart_ok`, three re-hashes and the scratch trace digest.
- `attempt()` reads and writes no module global except through `prestart()` (`global LIBTEST`). `PINNED` is mutated, not rebound. Every former local of `main()` (`plan_b`, `pin_dest`, `collide`, `r`, `j`, `err`, `ok`, `stops`, `files`, `tc`, `status`) is now local to `attempt()`, and nothing outside used them.

## 4. Residual limitations (recorded; not fixed by this repair)

1. **Exception after a spawn.** Suppose `run()` raises between `Popen` and the return of `wait4`. In practice that means an OSError from `wait4`/`killpg`, or an error while hashing logs after the child exits.
   - The child can still be alive in its own session. Nothing kills its process group.
   - That step's record is lost, because `result['steps'][sid] = r` runs only after `run()` returns. Its log files exist and `operator_exception` names the error.
   - `finish()` can then hash a lock file or parse a partial E4/E5 stdout while the child is still writing.
   - If `prestart_ok` holds and the scratch trace is intact, `post_run()` may spawn ANALYZE beside the live child.
   - If the second `xb` open fails, the first leaves a stdout log with no record.

   This is no worse than the unrepaired code, where the operator died and left the child running with no RESULT.json. Killing the group would be a new control, which is outside this minimum repair.
2. **Partial finish exceptions.** Each postcheck is guarded, so a failing one leaves partial fields and an entry in `postcheck_errors`. Examples:
   - `listings()` can set `snapshot_after` and fail before the diffs;
   - if `snapshot('before')` raised after writing its listing files, `listings()` sets both diffs and then raises a `KeyError` on the missing `snapshot_before`, which is recorded.

   If `write_new(RESULT.json)` fails, `finish()` prints `result_write_failed` and returns 1, and no post-run happens. If `post_run()` raises partway, `post/` (and possibly `analysis/`) can exist without `POST-RESULT.json`. The error goes to stdout only.
3. **Exceptions the handler does not catch.**
   - `except Exception` does not catch `KeyboardInterrupt`, `SystemExit` or `GeneratorExit`.
   - Signals such as SIGTERM, SIGHUP and SIGKILL end the process without `finish()`.

   This is deliberate. An operator abort should not go on to spawn the post-run analysis. A-8's "any unexpected exception" should be read as Python `Exception` subclasses (VOTES).
4. **Limits of the refusal before the marker.** It is placement hygiene, not a defence against code injection. Module-level stdlib imports and constant construction run before the check. Without `-I`, a planted module on `sys.path` would be imported before the refusal could act. The check does not verify bytes either. A wrong-bytes copy at the right path passes it, creates `run/` and then STOPs in `prestart()` on the FREEZE comparison. That is a designed, recorded path.
5. **Build-script filesystem exposure (R-05), unchanged.** There is no mount namespace. Build scripts and proc-macros of the union crates and the local crates can read and write anything the user can: BASE lanes, `<task>`, `~/.cargo`, keys and private files. The operator detects writes to the pinned task files, the toolchain pins, the scratch trace and listing metadata. It cannot detect reads.
6. **Per-step resource scope (OR-06), unchanged.**
   - RLIMIT_AS 8 GiB, CPU 60/900 s, CORE 0 and nice 10 apply per process.
   - The 1800 s wall limit and the 3 GiB disk gate (checked before the spawn only) apply per spawn.
   - Nothing bounds the whole run. The worst case stays at 17 spawns × 1800 s.
   - The repair adds no spawn. An exception path can reach ANALYZE only under its existing conditions.
7. **Carried items.** OR-04, OR-05/Q-2, OR-07, OR-08 TOCTOU, IR-09b/R-B.1, R-B.3, IR-10, and every item in the review's §12, are carried unchanged.
8. **Untested bytes (OR-09).** No interpreter has parsed these bytes. E-2 adds a new `try/except` and a new function, which strengthens the case for P-AST.

## 5. Votes

Votes are in `VOTES.md`: A-8 FOR, E-3 FOR and P-AST FOR, each as a second substantive vote, with no vote on Q-2. I cannot review or approve my own bytes. The repaired operator needs a fresh non-author exact-byte review, a FREEZE regenerated over its new digest, the P-AST precheck and root authorization. The existing votes and the parent dissent (W-OR-01 DISSENT) and qualifications are not changed.

## 6. Process record

- **Startup.** Read the installed `SKILL.md`, `references/execution-focus.md`, the full-integration checkout's `AGENTS.md` and `docs/MORIARTY-PRODUCT-CONTRACT.md`. `docs/SECURITY-ROUTING.md` does not exist at that path, so I read `docs/audits/2026-10-14/SECURITY-ROUTING.md`, the path the skill names. Root refreshed guarded status. I ran no CLI. Historical SP01.6 is not this task. Native/SDK authority is 0.
- **Read in full.**
  - Prior `operator-proposal/run_r3.py` (`d2ddbc3c…`, per seed);
  - the review's `REPORT.md`, `FINDINGS.json`, `VOTES.md`, `READ-COVERAGE.json` and `proposals/OPERATOR-MINIMUM-REPAIRS.md`;
  - `ROOT-PRE-EDIT-SEED.json`;
  - my seeded `operator-proposal/run_r3.py`, before editing;
  - lines 546–655 of the repaired file, read back to confirm the replace-all.
- **Not read.** The other six seeded files. They were not edited and are outside this scope.
- **Never accessed.** No PROMPT, PROCESS, wrapper, FINAL or PREPARATION file. No provider, private or stderr file. No credentials or keys. I did no directory listing, glob, grep, hash, execution, import or parse.
- **Writes.** Only the paths in `OWN-OUTPUT-ALLOWLIST.json`.

This report grants no execution, native authority, proof, budget, counter reset or acceptance. No r3 Rust result exists. U0–U7, M1–M17 and every MC/SP/G exit remain open.
