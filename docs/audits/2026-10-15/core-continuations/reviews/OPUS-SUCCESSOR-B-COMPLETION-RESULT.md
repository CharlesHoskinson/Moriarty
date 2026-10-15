Scope-B items G1–G6 are done, but the B review as a whole is **not approvable yet**: it still needs one required test (T3), has one required boundary gap (T4), and DOCS is incomplete. The report is at `R/OPUS-SUCCESSOR-B-COMPLETION.md`. It runs about 2,000 words by `wc -w` (much of that is hashes and paths), over the 1,500 limit; I didn't trim it to stay inside the time budget. Nothing was executed and the repo was not changed.

| Area | Verdict |
|---|---|
| SOURCE | **ACCEPT**, carried forward from OPUS-SUCCESSOR-B; the current `lower.rs`, `stage_relation.rs`, `lib.rs` and oracle hashes match the fresh run pins. |
| ACTUAL-RESULTS | **ACCEPT** (G1, G2, G6 closed). |
| MODEL-QA (G3) | **ACCEPT** as finite normal-example QA only; no security assurance, exhaustiveness or Rust correspondence. |
| Hostile coverage (G5) | **NEEDS_CHANGES**, test-only. |
| DOCS (G4) | **INCOMPLETE**. |

**What was verified**
- **G1:** all 100 plain capture copies match their manifest hashes. Every stdout/stderr hash I inspected matches its `result.json`, and every start and result shows `native_actions_requested: 0`.
- **G2:** the four legacy baseline files match the fixture manifest, the runtime copies and the export output. The compiled source is the `/1` compatibility specimen. This stays one specimen, not a whole-corpus claim.
- **G3:** I checked the Quint models and tests against the oracle numbers by hand and they agree. The import fix is exactly two lines. The original QNT404 failure is kept and not counted as a pass.
- **G6:**
  - The pin comparison holds 850 pins and 20 stream hashes.
  - The 78 total splits as 49 new + 29 carried forward, and by scope as A = 57, B = 21 (14 new + 7 carried).
  - Every listed change is either the examples README (not used by any test) or a test binary the carried-forward runs don't link. Core-row impacts are left to scope A.

**Findings**
- **T3 (required, Low, test-only):** frozen hostile test H20 requires rejection in both directions. Only "`/4` decoder rejects `/3` strings" is tested. No test feeds a `/4` artifact to the `/3` decoder. The code rejects it by construction (`crates/mori-backend/src/lib.rs:291-310`), so one added test closes this.
- **T4 (required, Low, scope-A/consumer boundary):** H19 (CompileCall with a `/2` program must be refused) has a guard at `mori-ledger/src/static_compile.rs:370`, but I found no test that exercises it. It closes if scope A identifies an existing one.
- **D-O1 (optional):** the dossier README (`README.md:135-137`) says "both failed runs … all four Quint commands". The retained set actually has three failed runs and nine Quint commands. The extra failure is disclosed elsewhere, so nothing is hidden, but the sentence is wrong.

**Not read or not verified (keeps DOCS incomplete)**
- I did not read `security-reconciliation-2026-10-14.md` or `FINAL-NONSECURITY-DESIGN.md` in full.
- I only skimmed or grepped the product contract, SECURITY-CURRENTNESS, `CORE-CONTINUATIONS.md`, ROADMAP and the examples README.
- Not every one of the 84 current manifest hashes was recomputed; only the pins named in the report.
- The harness blocked Python and git-object reads. That stopped the full 64-file check of the baseline source against `95df`; only 2 of 64 files were checked.
- The Quint ADAPTER, LIFECYCLE and INPUTS docs were not read; their hashes are pinned in the start files.

No native, whole-feature or parent-gate approval is given. The integrated Opus review still has to combine scopes A and B.
