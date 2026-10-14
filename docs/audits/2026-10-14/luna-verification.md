# Luna verification map

Pinned source: `cb8280ef54082853dd8bb6c6a52050328d0fc6fa` (tree `ce05a5c777e5e60f9bebfb1f5a80df78a2556096`). Scope is read-only public tracked source, tests and specs. No test or native action was performed. Native authority was zero; native artifacts were metadata-only. The requested `pluginstatus --json` is not a subcommand in this checkout; guarded `status --json` reports the old SP01.6 campaign blocked by stale/missing operational evidence. That status is context, not a dispatch or source verdict.

## Map findings

The evidence is split across four boundaries: the standalone beta TypeScript package and its `moriarty-beta-tests/1` harness; Rust `mori` CLI/core with `moriarty-tests/1`; the experimental successor source CLI/evaluators and finite lifecycle corpus; and separate Quint acceptance/custody models plus K executable definitions and comparison runners. README explicitly keeps beta, native language/1, successor source/5 and older bounded-atomic/1 profiles distinct. Their passing status cannot be transferred between profiles.

Actual child processes are exercised by beta CLI tests, experimental successor CLI tests, and Rust integration tests that invoke the built public `mori` executable. FeePool integration tests state that finite expected outcomes are independent and exercise public CLI consumers. Some successor CLI tests calculate expected JSON from the same API and compare the wrapper to it; those establish CLI/API agreement, not independent semantics. Beta manifest expected results provide fixed expected post/effect data, but the test runner feeds them to the same simulator under test. The K lifecycle corpus cites independent financial and work expectation files and compares complete result envelopes; its README limits that evidence to finite corpus behavior and explicitly says it is not a correspondence theorem. Quint supplies finite adversarial acceptance/custody models; its own README says witnesses are not automatically replayed through Rust/ledger and key verifier booleans remain premises.

The only CI workflow mapped here is the Pages/site workflow. SP12 lays out cold-start, clean-builder and release evidence gates as specified-only work. Source path presence, historical receipts and current docs are not fresh runtime proof, release qualification, financial settlement or product acceptance.

## Tested-profile and oracle matrix from source inspection

| Profile/evidence layer | Source-visible tests/oracle | Key limit |
|---|---|---|
| Beta authoring/local preparation | Spawned CLI tests; bounded `moriarty-beta-tests/1`; fixed expected post/effects | Expected cases execute against same evaluator; result is explicitly `PreparedUnqualified` |
| Rust native language/1 | Rust unit/integration/snapshot/property test inventory; actual subprocess checks of built `mori`; `moriarty-tests/1`; FeePool finite independent CLI expectations | Source-visible process tests and oracle setup are mapped; this task did not run them or establish ZKIR/native/ledger correspondence |
| Successor source/5 and Core/4 | Spawned CLI wrapper tests; Source/Core chained lifecycle corpus with separate outcome and work files | wrapper/API equality can share logic; fixture corpus is bounded |
| Quint | Acceptance, custody, lifecycle, signed/call models and deterministic tests described in QA README | Finite/sampled QA; external verifier booleans are assumptions; no automatic Rust trace replay |
| K | Bounded expression/lifecycle/numeric definitions, fixture runner and independent expected output files | Defined subsets and finite rows; not full Core semantics or theorem |
| FeePool mapped static | Source fixture matrix, component integration source and documented compile/inspect commands | Local source-static support does not establish authenticated authority, native proof or deployed acceptance |
| Release | Site CI; SP12 planned release/checker/clean-builder obligations | Site CI is not language package or product release qualification |

## Strong audit questions and gaps

1. Bind every claim to exact profile, source digest, public consumer, oracle provenance and acceptance boundary; keep U0–U7/M1–M17/MC/SP/G denominators intact.
2. Separate model truth from implementation correspondence. Add trace adapters and mutation checks for each mapped field/effect, plus a named premise ledger.
3. Break shared-oracle dependencies with independent hand-derived small financial cases, metamorphic laws, independent work counts and separately adjudicated acceptance expectations.
4. Attack each profile independently: serialization and unknown fields, duplicate keys, paths/symlinks, stale and replayed predecessors, effect omission/reordering, fee/refund gross/net accounting, liabilities, rounding boundaries, Unicode, late outcomes, atomic refusal and binding mismatches.
5. Audit installed/packed public CLI cold-start paths and clean builder reproducibility as ordinary local checks; keep native proving, private material and network/ledger actions on their separately blocked lane.
6. Classify evidence as source-inspected, specified-only, historical-run, current-run or externally accepted. Do not convert counts, source paths, model samples, campaign records or site publication into acceptance claims.

The machine graph contains 34 nodes, 39 typed edges, 7 capability records and 8 concrete coverage gaps, with line-linked claims and SHA-256 hashes for inspected files. See `luna-verification.json` for the full graph, capability status, questions, and citations.
