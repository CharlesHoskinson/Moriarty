I adopted the new test code with one small fix to how it creates temp files. Nothing was compiled or run (Cargo was out of scope), so every expected value in the new tests is still unchecked.

**Identity and owned source hash:** `claude-opus-5-5` (Opus 5.5), acting as the author-finisher and classifier. This is not an independent review.
- Before: `b52300688fd14654c2665942d3611b5eb1bcbb7a3c910a06219250b72191203d` (874 lines, +551 / −0 against e9a2b80b)
- After: `e8d3af1257d447aad9f14b566b6eb9b3005a929f26887cbe3863a546f93054a7` (883 lines, +560 / −0)
- The first 323 lines match e9a2b80b exactly (sha `56f04e4f…`). Core and lexer production, the Cargo and toolchain files, and `escrow.mori` are unchanged. The oracle `normal-oracle.json` still hashes to `76075b63…`.

**Tests, counted from the source:** the original 10 functions (9 `#[test]` plus 1 proptest with 32 cases) are untouched and not weakened. There are 12 new tests, for 22 in total:
- `h6`, `h7`, `h9`, `h10`, `h11`, `h12`, `h14`
- `h24` (×2: in-memory caps, and file reads via `read_bounded`)
- `f2` (×3: typed inputs, typed states, liability ordering)

H14 checks that a tampered capture is **accepted**. That documents a known gap, not a protection.

**Symbols:** every symbol the tests use is already public, so there were no visibility edits. That covers `stage::decode::{continuation, event, read_bounded, CONTINUATION_TOO_LARGE, EVENT_TOO_LARGE}`, the stage constants, `json::parse`, `Effect`, `Error.step` and `Balance`. There are no SDK imports, new dependencies or Cargo changes, and the expected error messages match the strings in the production code.

**The fix:** `TempInput` wrote to a predictable `/tmp` path with `fs::write`, so it would follow a planted symlink or overwrite an existing file, and its cleanup could then delete a file it never created. It now uses `create_new(true)`, which refuses any file or symlink already at that path, so it only ever removes its own files.

**Running the check:** the filesystem is a new kind of effect compared with the earlier four-class Core classification. The test creates and deletes 4 files named `mori-core-stage-hostile-<pid>-*` in `temp_dir()`, about 4.2 MiB at peak. Before running index 3, the check needs four additions: a pinned `TMPDIR`, a pre-run check that no such files exist, a post-run check for leftovers, and the new source hash.

**Verdicts:**
- **Adopting the source:** APPROVE_ORDINARY.
- **Running index 3:** APPROVE_ORDINARY once those four additions are made, otherwise NEEDS_SOURCE.

**Still unverified:**
- whether the file compiles;
- the step numbers and work counters the tests expect;
- E2's `[7,8]` cost;
- which admission rule fires first in the F2 and `ghost` cases;
- behaviour at exactly the size limits.

I left two harmless problems in H6 unfixed: a misleading comment and one mutation listed twice.

The final independent whole-candidate reviews by Opus and Muse are still owed. The report is under 1,100 words, at `R/OPUS-CORE-HOSTILE-FINISH.md`.
