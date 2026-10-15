I couldn't approve scope A for source, actual results or docs. All three are **INCOMPLETE**: I ran out of the time budget, and the permission layer refused to let me decompress the retained captures (`zcat` and `gzip -dc`). The report is at `R/OPUS-SUCCESSOR-A.md`, and it does not claim the whole feature is approved.

**Source (Core):** I read all 8 Core stage modules, the runtime/parser/json/lib/logical-cost diffs and the helpers they reach, and I found no required defect. The main security properties hold in source:
- `/2` programs can't reach the `/1` entry points, and `/1` programs are refused on resume.
- Resume checks run in order (R0 to R7), and a rejection leaves the caller's input untouched.
- The continuation is bound to its program and plan by exact digest.
- Captures and events must match the pending site exactly by name, order and type.
- Committed work must match the plan, and the invocation cap can never raise the stored cap.
- The resumed boundary is checked as written, never re-sorted.
- Input files are size-capped before parsing, and duplicate keys and non-canonical numbers are refused.
- The three labels are inserted verbatim. H14 (tampered capture accepted) is correctly framed as a known limitation of unauthenticated input, not as protection.
- Optional F2 (typed inputs, typed states and liability order) is enforced in source.

**Evidence:** `sha256sum -c` passes for all 417 entries in `SHA256SUMS`, and the oracle hash matches `76075b63…9640`. I confirmed `stage_hostile.rs` has 22 tests. I read the H6, H7, H9, H10 and H14 tests and the file-size test in full, but only the opening of H11.

**Findings (none required):**
- **A-1 (low):** at `stage_hostile.rs:413-414` the comment says the case uses a JSON escape, but the payload has none, so a duplicate key written with an escape is untested.
- **A-2 (info):** at `stage_hostile.rs:425-426` one H6 case is listed twice and adds nothing.
- **A-3 (info):** the stored cap can be changed by the caller, which fits "authentication: none". The docs should say it is advisory; I didn't check that.
- **A-4 (info):** at `decode.rs:122` the `captures` parameter is deliberately unused, which could mislead readers.

**Unread required coverage (listed in the report):**
- **Public CLI:** the source (`simulate`, `inspect`, `compile`, `check`, `core_io`, `proof_io`, `cli.rs`) and its 7 tests.
- **Trace-path bodies:** `public_field`, `authorization_rule`, `unknown_premises`, and `signed_relation::reconstruct`/`accept`.
- **Ledger:** the changed entry guards, plus the source-closure test (48 files, 3 exclusions) and its 6 tests.
- **Other Core tests:** the normal (5), compat (4) and static-cost (13) tests, the rest of the hostile bodies, and the temp-file helper.
- **Actual results:**
  - All decompressed captures and the 57/78 totals.
  - `CAPTURE-PIN-COMPARISON-CURRENT.json` and its deltas.
  - The 84-path current manifest, which is separate from `SHA256SUMS`.
  - `SOURCE-SCOPE-CURRENT.json` and `OBSERVATIONS-CURRENT.json`.
- **Normative docs:** all of them, so I haven't checked the frozen H1–H28 list against the tests. H15–H23, H27 and H28 aren't named in `stage_hostile.rs`.
- **Quint and `/1` comparator:** the Quint runs, and the checks that the `/1` comparator is unchanged and `lower.rs` is not called.

A follow-up pass would need permission to read the `.gz` captures, plus time for the unread items.
