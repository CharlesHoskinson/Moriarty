# ACTUS public Core case source proposal

The candidate adds one real financial trace to the existing `crates/mori-core/tests/financial_pair.rs` integration target: pinned `stk04`, using the public Rust `parse -> validate -> simulate` consumer in two linked calls. Complete before/after files and `proposal.patch` are supplied. The worktree and implementation repository were not edited. No runtime, Cargo, SDK, VM, proof, wallet or network command was run. This is source-inspected, unevaluated preparation only.

Predicate: at the exact frozen candidate, normal `stk04` purchase and termination must produce all seven present ordered ACTUS result fields, all canonical pre/post and statement-prefix snapshots, and every local and aggregate ordered effect from the frozen literals. The actual first poststate must be the second call's scenario. No Fee/Liability effect, missing/additional result field or extra transfer/event may be accepted by the test projection. Non-author review and an authorized future actual test result are still required; no pass is asserted.

Future command, not run:

    cargo test -p mori-core --locked --test financial_pair actus_stk04_cashflows_match_every_present_result_field_and_linked_prefix -- --exact

The command names the existing `financial_pair` target and only its added ordinary Core API case. The patch adds no new framework, crate/dependency, production dispatch or public IR interface. Cargo target/dependency behavior was source inspected only. The parent retains control of future execution scope.

## Actual importer/test/public-path findings

At Moriarty main 9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb, scripts/acquire_actus_public_sources.py:233–255 reads every tests/actus-tests-*.json object to inventory membership, contract type, result keys and event kinds. It is an acquisition/inventory importer. It does not execute a Moriarty program or compare financial semantics. Its prospective repository paths do not exist in the evidence worktree; the corresponding pinned local source repositories under /home/charl/Moriarty/repos supplied git-show bytes.

The supplied comparative actus-core 42451170dc61c5c11c4144bd5a69046f149e8016 test/Spec.hs omits seven named fixtures, four executable types and the analysis-date file; test/Spec/TestFramework.hs:184–197 compares only event type/date/payoff with a binary32 downcast. Its importer uses Double, parses only a subset of possible fields and can return an empty case list on a parse failure. None of that comparator's derived values or test results was used as an oracle. No Moriarty ACTUS conformance test consumer was found in the inspected pinned scripts/crates/tests source path. SP07's conformance adapter paths remain proposed in the sprint document.

The real public Rust Core/1 consumer already parses source, validates public IR and simulates ordered source statements with exact integral asset transfers and snapshots. Its existing financial_pair.rs tests already consume independently frozen literal financial specimens through that API. The proposal extends this target, uses its existing fixture macro, and uses no Sherlock/native/backend/ledger path. This is a local cashflow correspondence specimen, not schedule, compiler or target correspondence.

## Requirement retention

basis/actus-tests/tests contains every original fixture file, unchanged. basis/retained-membership.json records all 277 filename-qualified members, full-case canonical hashes and every ordered event's present field keys. Source inventory covers 276 contract fixtures plus one analysis-date fixture. The patch retains the complete STK file, including unselected STK cases; none is deleted, excluded or relabelled passing. Only stk04 is proposed for this bounded test. All 277 remain unexecuted by this lane.

The 18 executable type requirements remain PAM, LAM, LAX, NAM, ANN, CLM, UMP, CSH, STK, COM, FXOUT, SWPPV, SWAPS, CAPFL, OPTNS, FUTUR, CEG and CEC. Original 32 taxonomy dispositions are preserved in the pinned requirements CSV and taxonomy bytes in basis. One STK trace does not complete STK or reduce the denominator.

DS-01 business-day/calculation-payment shifts, DS-02 SCMP/CSMP identifier discrepancy, DS-03 ANN initialization, DS-04 COM quantity/sign/TD erratum, DS-05 CLM redemption, DS-06 FXOUT event identity, and DS-07 FUTUR/taxonomy breadth retain their recorded dispositions. The existing design-study text and SP07 requirements are copied in basis. This proposal resolves none of them. Even the supplied stk04 EOM term is retained without implementing an EOM cycle generator; this finite trace has explicit dates and no recurring schedule.

## Limitations and review boundary

The source hardcodes this contract's two agreed prices and explicit dates, as a developer-authored finite contract. It is not a general ACTUS importer/package, does not derive schedules or handle unsupported constructs by host flags, and does not implement equity delivery, dividends, rates/day-count, decimal rounding, market observations, calendars, mint/burn or outstanding duty lifecycles. Its asset and account binding and starting funds are explicit local harness premises. Unsigned timestamp equality has no authenticated clock meaning. Source normal lifecycle guard choices require review and have no independent normative repeat-call oracle here.

Expectations were authored from supplied normative fixture/spec and frozen without execution. They are author-derived, not a fresh non-author oracle. No independent reviewer, provider returned identity or actual result is claimed. No security analysis/test/judgment is supplied by this lane. No U0–U7, M1–M17, MC/SP/G, all-family finance, composition, private/recursive, native/Preview/Fenrir/proof/ledger/release exit changes. Public developer use remains permissionless.
