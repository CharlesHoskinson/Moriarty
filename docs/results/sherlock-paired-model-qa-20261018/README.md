# Selected paired-observation checker evidence

The existing Sherlock QA checker now has an explicit selected supplied K/Quint comparison path. The promoted source is exactly `check.py`, `test_paired_projection.py` and the literal F03 Quint observation driver. The [fresh R253 revision02 review](review/r253-revision02/REPORT.md) approved these three files with the retained findings below, accepted the corrected single-test result and supported the default-reader/two-hash compatibility rebind. Actual init selected `gemini-4-argon-high`, conversation `85b594b0-dc05-4de0-b2ef-04af6bf540db`; one fresh no-tools review completed successfully. This records observed selection, not backend-weight attestation.

The new mode compares supplied observations against both the frozen independent expectation and each other. It retains raw provenance and states which rows are omitted. Its unit fixtures do not execute their named model producers. The [complete public thirteen-file snapshot](actual/r248-onecase/snapshot/) preserves the exact sources/oracles used for the corrected case. No model, compiler, native, circuit or ledger result follows from these Python QA observations.

| Actual record | Executed scope | Observed result |
|---|---|---|
| [R241](history/r241/ACTUAL-RESULT.json) | Ten original Python unittest methods | Nine pass; one fails |
| [R248 corrected case](actual/r248-onecase/RESULT.json) | Only the formerly failing `test_selected_cli_api_keeps_default_full_corpus_counts` | One pass, exit0; all thirteen selected inputs unchanged |

The original failure is retained in [its exact output](history/r241/ACTUAL-stderr.txt). Two fixture literals incorrectly used `uiReturn(uiU(7))`; the [correction](history/r244-correction/FIXTURE.PATCH) uses canonical `uiRet(uiU(7))`, as declared by unchanged `main.k` and the frozen N01 prefix oracle. The production model/comparator was not changed to fit a bad fixture. R253 inspected why that correction leaves the other nine historical passing methods applicable. This is nine historical passes plus reviewed compatibility and one corrected actual pass, **not a fresh ten-test execution**. The unchanged historical 254-test cohort was not rerun.

The corrected case observes the normal eight-program/216-prefix corpus checks together with selected supplied N01 pairing. The standard requirement-evidence function remains byte-identical by the fresh source review. Separate [Root integration evidence](root/r268/CHECKER-PROMOTION.json) records the exact three source files and two current UIREQ-025 checker-reference cells; no new requirement closure was added.

The [approved guarded JSON patch](integration/CURRENT-EVIDENCE-CHECKER-REBIND.jsonpatch) is rooted in ledger `6df32dc2…` and replaces only those two checker hashes after testing their row/path/old-hash preconditions. It is not a whole-ledger replacement. Root's first serialization produced an intermediate `e9bcb443…` with equivalent JSON values but changed Unicode escape spelling. The [additive byte-preservation correction](root/r268/CHECKER-REBIND-BYTE-PRESERVATION.json) withdraws that intermediate as current and records authoritative `9ff83b17…`, preserving every original byte except the two approved 64-byte hash literals. Do not reapply the patch blindly or overwrite concurrent R254 metadata.

The [existing reader output](root/r268/CURRENTNESS-AFTER-CHECKER-REBIND.json) recorded `current`, ten current adopted originals out of35, and no findings at its intermediate/pre-profile-source cut. It is preserved verbatim with that ledger identity. The later byte correction and R254 source installation are separate events; this archive claims no newly executed reader result for those later cuts and adopts none of R254's pending Rust references. File-currentness checks do not grant semantic acceptance.

Retained findings from the raw R253 opinion:

- **NB-01:** `compare_paired_observations` directly indexes a missing `observations` field and can raise `KeyError` rather than `ValueError`; `main` catches the error. No API-error-contract fix is claimed here.
- **NB-02:** A first difference under `/programs/0/...` lacks run identifiers needed by the top-level raw-context locator. That locator reports unavailable while each layer's raw context remains populated.
- **NB-03:** The F03 file-loading branch and its two repository-relative frozen oracle inputs are outside the thirteen-file unit snapshot and unexercised by the N01 tests. The Quint driver is literal source only; its text was inspected, not executed.

Original **UIREQ-021 remains open**. Actual F03 K and Quint traces at initial fuel93/stack64 remain unobserved. Supplied finite-trace comparison is no formal refinement proof, producer authentication, generic-language coverage, Candidate-B/helper schedule correspondence, financial effect/duty or await/resume evidence, or native/target qualification. These maintainer evidence records add no public developer permission requirement.
