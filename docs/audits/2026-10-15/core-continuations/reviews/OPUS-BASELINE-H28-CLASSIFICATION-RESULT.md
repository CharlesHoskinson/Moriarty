I approved check A (the H28 coverage test) as ordinary execution. I could not approve either legacy byte command: my request to read the baseline source with `git show 95df7355…` wasn't approved, so both are NEEDS_SOURCE. I ran nothing and changed no source; the only file written is `R/OPUS-BASELINE-H28-CLASSIFICATION.md`.

**A, `source_closure_coverage`: APPROVE_ORDINARY**, if the preconditions below hold.
- **Checked:** the test SHA is `184f35a5…`. Its 61 baseline entries match `H28-BASELINE-VERIFICATION.json` exactly and in order, followed by the 8 stage files for 69 total.
- **What it does:** it embeds `proving.rs` with `include_str!` and parses that text. At runtime it only resolves paths and lists `mori-core/src`; it never opens file contents. It makes no `mori_ledger`, SDK or native calls and writes nothing to source.
- **What gets compiled:** the `mori-ledger` dependencies plus the `futures` dev-dependency. No workspace crate has a `build.rs`. Third-party build scripts and proc macros from Cargo.lock still run, and that residual isn't waived. You can reuse the prior CLI build premise only if its closure already covered these dependencies at the current Cargo.lock.
- **Preconditions:**
  - Add the test to the source manifest.
  - Pin the current `proving.rs` and the list and SHAs of `mori-core/src/**/*.rs`, including the untracked `stage/` files.
  - Confirm there is no Cargo config and no symlinks leading outside the worktree.
  - Unset the wrapper and other `CARGO*`/`MORI*`/`PROPTEST*` variables without printing them.
  - Run it once.

**B1, baseline-mori compile: NEEDS_SOURCE.**
- The source and target hashes check out. The target is unsigned ledger-9 with local anchor `legacy-byte-oracle`, not a Fenrir identity; its SHA `8cd664ce…` needs to be added as a pin.
- The current `compile.rs` has been modified (it adds a staged `moriarty/2` path), so I didn't treat it as the baseline.
- The report lists the files I need from `95df7355`: `compile.rs`, `proof_io.rs`, `core_io.rs`, the CLI dispatch, the backend `Target`/`Artifact` definitions, and a diff of backend/core against `eb2577d6`. If they match the current unsigned path, I expect to approve it with these preconditions:
  - `baseline-export` doesn't exist yet.
  - The binary hash is the same before and after.
  - The inputs are re-verified.
  - It runs once.

**B2, the ignored comparator: NEEDS_SOURCE**, because it depends on B1.
- The test reads only the fixture and the candidate's compile output and writes nothing, so the candidate never supplies expected bytes.
- **Gap, not waived:** it never checks that the fixture's set of files equals the manifest plus the candidate's `.zkir` files. Until a non-security author adds that check (with a Muse audit), you would need to record every fixture file's hash and compare that set against what the baseline exported.

The full Muse and Opus reviews of the candidate and its results are still owed.
