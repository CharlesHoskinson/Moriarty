All three are approved as ordinary execution, each with the conditions below. The report is in `R/OPUS-CHECK-DELTAS.md`, about 1,150 words. I ran nothing, changed no source and took no native actions (still 0).

I couldn't refresh `status --json`: the permission layer refused the guarded CLI. I loaded AGENTS and the develop skill. Every earlier premise and caveat still applies, including C1–C7, D1–D3 and the H28 A.4 preconditions.

**A. Backend checks rerun: approved.**
- **Why the old run failed:** `stdout.txt` shows the failing test panicked at `stages.rs:45:99`. That's the compile helper's `unwrap`, so the run died at load time before any assertion. That fits your lexer diagnosis.
- **The fix:** the three new lines at `stages.rs:120-122` mean the same thing as the old ones. The assertions, the test list and the single ignored test are unchanged.
- **Conditions:**
  - The only pin you refresh is `stages.rs`, to `217c75af…`.
  - Run the same two commands one after the other and stop at the first failure.
  - Expect `5 passed; 1 ignored` from `stages`, then `4 passed; 0 ignored` from `stage_relation`.
  - Don't pass `--ignored`.
  - Record the run under a new check ID so `backend-normal-01` stays as failure evidence.

**B1. Baseline compile: approved.** It no longer needs source.
- **Old source checked:**
  - `compile.rs` takes the unsigned branch: the target has no `signed_policy` and uses the `legacy-byte-oracle` anchor.
  - It serializes everything before creating the output folder.
  - `publish_new` creates new files only and never overwrites.
  - The old backend `compile` only builds the ZKIR and parses it back; it doesn't touch the environment, other files, proving or keygen.
- **Not re-read:** the `core_io.rs`, `cli.rs` and `main.rs` bodies. Whether the binary matches this source rests on the R53 provenance record.
- **Conditions:**
  - The binary SHA is `89ae66c4…` before and after.
  - `baseline-export` doesn't exist beforehand.
  - The input hashes and the target pin are re-verified.
  - It runs once.

**B2. Ignored comparator: approved if B1 succeeds.** The comparator body is unchanged, and the old `to_string_pretty` output is byte-for-byte the same as the test's `to_vec_pretty`.
- **The gap stays open in code:** the test never checks that the fixture's set of files is exactly what the baseline exported. Root has to cover it:
  - Hash every exported file and confirm the set is exactly `artifact.json` plus one `NNN-<action>.zkir` per circuit.
  - Copy without overwriting.
  - Check that the fixture and the export match exactly, in both directions, before the candidate run and again after it.
- Run once with the exact command; expect `1 passed`.

**C. H28 after the statement-root fix: approved.**
- **Conditions:**
  - The only pin you refresh is `relation/mod.rs`, to `e1f0fb62…`. Root needs to confirm that applying the patch to the old file reproduces that hash.
  - The A.4 preconditions still apply.
  - The 61 baseline entries and the test SHA (`184f35a5…`) are unchanged.
  - No other production file has changed.
  - Run once and stop on failure.

None of this accepts the feature. The full Muse and Opus code and results reviews are still owed, and they now also cover Sol's fixture edit and the patch. I may have run slightly over the 320-second limit.
