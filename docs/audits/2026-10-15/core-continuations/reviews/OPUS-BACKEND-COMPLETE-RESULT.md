**SOURCE_WRITTEN.** I'm Opus 5.5 (`claude-opus-5-5`), the security author on this job. Nothing was compiled, formatted, tested, modelled or proved, and there's no commit. I'm not claiming any approval. Full U/M/MC/SP/G remain open.

**Files:**
- **`crates/mori-backend/src/stage/{mod,artifact,frame,context}.rs`:** the earlier job had already finished all four, so I left them unchanged.
  - `frame` and `context` are `pub(crate)`, and `PendingPositions` is re-exported as an `Option`.
  - The decoder is strict, `context` builds the 29-element context, and a signed Target is refused there.
- **`crates/mori-backend/tests/stage_relation.rs` (new):** four focused tests, not run.
  1. Pending roles appear only at the recorded positions, with the right site and padding constants (H27).
  2. Every circuit has its own context, and all ten gates stay unperformed.
  3. The decoder rejects six forgeries (H20/H21).
  4. The `/3` compiler refuses a moriarty/2 program with a typed error (H18). The H mapping is my own reading, not an audited classification.
- **`R/OPUS-BACKEND-SHARED.patch`:** exact diff hunks for Sol to apply:
  - **`lib.rs`:** `require_core1` goes first in `compile`, `compile_signed` and `compile_mode`, and the main lowering match gets an `Await` arm that refuses with a typed error.
  - **`signed_relation.rs`:** `admit` checks the profile first and gets the same `Await` arm. The reconstruction step also checks the profile after parsing, which covers replay `generate`.
  - **`stage/lower.rs`:** a signed Target is refused before the program loads and before any output directory.
- **`R/OPUS-BACKEND-DETAIL.md`:** the public report.

**Remaining qualifiers:**
- **Test 4 will fail** until Sol applies the `lib.rs` hunks.
- **Diff line numbers** are counted by hand against today's uncommitted worktree, so they may need a small offset.
- **Visibility is unchecked:** the tests assume certain fields and constructors are public. If one isn't, it's a compile-only fix.
- **`proving.rs:735/982`:** I didn't check whether these statement matches need an `Await` arm.
- **Not verified:** that the command-line tool calls `compile_staged` before it reserves the output directory.
- **`/3` identity** is untouched: no snapshots changed and the EMPTY whitelist is preserved. The authenticated ledger binding and all ten gates are unperformed, and I'm drawing no conclusion about host security.
- **Startup:** given the time limit, I read only the start of `AGENTS.md`. I didn't load the `moriarty-dev:develop` skill or run `status --json`.
