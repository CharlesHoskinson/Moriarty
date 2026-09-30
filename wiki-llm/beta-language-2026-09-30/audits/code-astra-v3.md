# Independent whole-code result audit — v3

Date: 2026-09-30. Verdict: **PASS for the bounded beta code candidate and entry to developer trials.** No high, medium or low defect was established in this review. This is not financial, proof, authentication, ledger or full-language acceptance.

## Identity and independence

Requested reviewer: fresh `gpt-6-astra`, medium effort, per the delegated task and repository routing. Observable identity: this fresh delegated agent context `/root/code_astra_v3`; no separate provider-returned model or effort attestation is exposed to this reviewer. The requested identity is therefore recorded without inventing a returned provider receipt. No peer audit, prior audit or repair report was read. Candidate tests named `repair.test.mjs` were inspected as current executable tests, not as historical review evidence.

Loaded the Moriarty development skill and ran the required guarded `status --json` from the specified checkout. It returned no pending transactions and reported unresolved historical campaign accounting/admission. This read-only code review did not dispatch that campaign or alter its acceptance state.

## Frozen scope

Checkout: `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`.
Baseline: `870f998b36ecda04622fa4274132e74902942d0b`.
Manifest: `audits/code-candidate-v3.json`, 58 files.
Verified aggregate: `c66b683b47aec11f3670f93510a73f706ee37369bb848164559edf122287ade9`.

Both initial and final checks recomputed every file SHA256. Aggregate computation sorted entries lexicographically by path and hashed their UTF8 `sha256`, two spaces, path, LF lines. All 58 file hashes and the aggregate matched.

Inspected the complete package production implementation: frontend, bounded JSON parser, bridge, public index, CLI, starter, framing and stdio servers, build and package exports/declaration generation, editor client and packaging, AI adapter assets, eight family examples, and all current package test files. Inspected CONVERGENCE and IMPLEMENTATION-PLAN as the controlling code contract. Inspected the unchanged Source/6 wrapper and relevant parser/lowering boundaries consumed by the package. `git diff` against the recorded baseline showed no changes under `experiments/moriarty-language/src`.

The separate proposed typed horizon documents were excluded as assigned; the executable horizon declarations and examples were included. Dependency manifests and lockfile package inventories were inspected; dependencies themselves were not independently security-audited.

## Findings and result observations

No actionable severity finding remains from this review, so there is no defect file/line reproduction to request a repair against.

- Exact quantities retain nominal asset identity and checked UInt128 intermediates, with S127 narrowing at S0 assignments. Economic IDs reach Source/6; scenario cells are exact and ordered. Scenario objects have closed schemas and raw decoded duplicate keys reject.
- Transfer and funded AccrualFirst repayment call the existing preparation wrapper. Results retain complete effects, post-state, work/replay updates and external premises. Formation rejection remains distinct from Core rejection. Automatic overpayment uses the documented unchanged debt placeholder; explicit candidate effects suppress the automatic-proposal note. First economic failure remains Core-owned within the representable comparison domain.
- Inspection attaches conditional Core coverage only to actual LocalS0 actions. Empty, declaration-only and SpecifiedOnly sources report open financial relations without S0 premises. Mixed sources expose the S0 action names and scope the narrow bounds appropriately.
- Check results retain references and fieldUses without serializing declaration values. Origin dependency closures include use sites and transitive definitions. Source/scenario digests bind the exact input bytes. Inspection and expansion bound their serialized artifacts and do not publish partial artifacts on their bound failures.
- CLI human check output and explicit JSON output are distinct. Case manifests have finite counts, closed fields and contained real paths. Build exports include public declarations and bundle the existing preparation implementation. The actual packed-consumer test exercises installation outside the checkout, CLI/API behavior, emitted declarations, and equality of the bundled editor server with the distributed CLI.
- Incoming and outgoing editor positions share CR/LF/CRLF and UTF16 semantics. Oversize Full changes invalidate previous results, return empty/null editor results without attempting obsolete positions, and recover only at a newer valid generation. Framing, source/document and output bounds are explicit. MCP tools dispatch to the same text APIs.

## Verification

Observed the supplied current execution logs: `beta-tests-code-v3.txt` reports 69 tests passed, zero failed, including the actual packed-consumer check; `typecheck-code-v3.txt` reports the typecheck invocation with no diagnostics. These are parent-run observations, not claimed as independently rerun here.

Independently executed 13 focused discriminators against the frozen source imports: S127 assignment versus UInt128 arithmetic, parenthesized/transitive quantity references, exact input digests, formatter semantic/idempotent behavior under all three newline conventions, escaped/nested duplicate JSON keys, combined stale/expired/exhausted/replayed first-failure comparison with independently written Source/6, empty/declaration-only inspection, dense reference summaries, and expired overpayment. All 13 passed. The dense valid case retained 4000 references from 40083 source bytes; its JSON-safe check summary was 396601 bytes, below 524288, and declarations exposed no values.

Independently ran:

```sh
node --test --test-name-pattern='over-budget Full|identical CR' tests/protocol.test.mjs
```

Both real subprocess tests passed, zero failed. They exercised oversize Full invalidation/recovery and CR/CRLF/LF navigation/completion. No broad suite was repeated without a concrete remaining risk.

## Limits

This finite audit does not prove correctness for all admitted inputs. Real VS Code activation and provider activation remain unperformed. The reported packed installation and typecheck were inspected through current tests and supplied execution logs; this audit did not independently repeat packing or installation. General financial horizon relations, canonical authentication, native proof/correspondence and atomic financial ledger acceptance remain open. The six developer trials remain subsequent work and are not established by this audit.

Only this audit report was written to the checkout. No product edit, configuration change or commit was made.
