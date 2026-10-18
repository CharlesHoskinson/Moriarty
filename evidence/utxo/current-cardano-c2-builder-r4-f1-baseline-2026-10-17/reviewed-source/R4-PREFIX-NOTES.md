# R4 test prefix: notes

Author: claude-opus-5-5 HIGH (the actual runtime model). This is a bounded source fragment only. Nothing was run, imported, compiled, hashed or reviewed. Authority is 0. This is not an acceptance, not sourceReady and not a runtime permission.

## Coverage
- `R4-TEST-PREFIX.py.part` contains imports, frozen literals, pinned fixture loading (`setUpModule`), the R3-derived independent formatters, the lexists-gated lazy R4 loader, `R4TestBase` helpers, and exactly one test: `TestR00Prerequisites.test_r00_prerequisites`.
- It has no R01-R17 methods and no `unittest.main`. The full 18-test suite has **not** been delivered, and no RED or GREEN has been observed.
- `R4-TEST-INTERFACE.json` lists every global, function and base method, the MISSING phrase and loader behaviour, the assembly order and the part boundaries.
- The prefix is about 400 lines, longer than the ~250 aim. Most of the extra length is verbatim literals: labels, limitations, audit pins and script-hash strings.

## Effects (proposed, when executed later)
- Reads only pinned envelope files, through a bounded no-follow read.
- Imports legacy, host_codec and r2. It may load R4 once, if R4 is present.
- R00 calls r2 once, on deep-copied K6/K2 inputs.
- The helpers write only under per-test `scratch/<method>` directories.
- `redirect_stdout`/`redirect_stderr` swap `sys.stdout`/`sys.stderr`, which is the historical pattern. They do not patch os, builtins, json or R4.
- There is no network, subprocess, native, prover, key or VM action. Lints are lints only, and nothing here makes a sandbox claim.

## Gaps and choices for review
- **Governance pins.** The R4 contract/API, the CP r2 contract/API, the conditional-adoption pins and the ROOT-CP08 pins are recorded only as `GOVERNANCE_PINS` literals. The API envelope layout does not include those documents, so R00 cannot byte-check them; root must check them at assembly. The ROOT-CP08 pin is the packet MANIFEST pin. R00 does byte-check the adopted r2 bytes against the ROOT-CP08 candidate pin.
- **B9.** The B9 body was not supplied or read. The prefix pins B9 at 308672 B with sha256 10baa179... and STOPs if it is absent; it never synthesizes B9. Its shape is inspected only at future R00 run time, through `legacy.extract_compiled`, plus an independent blake2b-224 policy recompute. Whether POLICY matches B9 is unverified here.
- **Additions beyond the R3 pattern** (all STOP guards):
  - a `__pycache__` absence check;
  - module `__file__` samefile checks;
  - `find_spec` absence of v1;
  - the platform flag and dir_fd checks from contract section 0.
  Root may drop any of these.
- **Label order.** R00 compares the r2 labels in order as well as by value. This rests on the adopted bytes and S34 order.
- **R00 timing.** R00's scratch-empty check assumes it runs first, which sorted class names give.
- **Unverified expectations.** `expected_case`, `expected_inputs` and `MANIFEST_KEYS` follow the R3 test, the legacy main layout and the API, but have not been checked against an implementation. The manifest values for `compiledValidatorTitle`, `scriptHexFile`, `scriptCompiledBytes` and `scriptInnerFlatBytes` are not restated in the R4 API, so no literals are given for them. Later parts must not guess them.
- **Unused import.** `copy` is imported for later parts and is unused in this prefix.
- **Not proved by tests.** The source-only properties are not proved by any test: CP04 (no encode or copy before the scan), no host alias or patching, and no hidden or cached state.
- **No claims.** There are no Rust, curve, pairing, Groth16, VK-profile, authenticated-ledger, fee, ExUnits, fit or network claims.
- **K6 and K2.** K6 is a captured public genuine local proof, but its context is untrusted. The K2 setup is single-party, TEST ONLY and forgeable. Circuit/VK correspondence is OPEN.

## Scope
- **Historical votes.** The Opus 4.8 R4 votes (CB401-CB412 AGREE) stay a scoped, conditional test basis, attributed to claude-opus-4-8. Those reviewers did not see the r2 bytes.
- **CP08.** CP08 adopts r2 for R4 test authoring only. It does not permit R4 execution, R4 implementation or builder reliance.
- **Frozen bytes.** The frozen contract bytes were not rewritten. The root record governs the current scope.
- **Muse seat.** Gemini4 Argon High replaces Muse only for non-security (Sol glue) review.
- **Still OPEN:** all 20 UTxO families, finance, ACTUS, DeFi, recursion, private handoff, multiparent PCD and Preprod.
- **Before any test runs:** the future R01-R17 parts, the assembled whole file, a fresh whole-code and effects review, and a root decision.

## Unread
- Only the MANIFEST and its 11 named files were read.
- Not read: the originalPublicSource paths, the B9 body, the CP r2 contract/API bodies, DOWNSTREAM-R4-CONDITIONAL-ADOPTION.json, the Sol INTEGRATION-CONTRACT.md and the R3 C2-BUILDER-API.
