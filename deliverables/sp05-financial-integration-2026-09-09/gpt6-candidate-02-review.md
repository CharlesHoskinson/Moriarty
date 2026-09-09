# Independent candidate 02 source review

**Verdict: PASS WITH LIMITS for the fixed source scope.** No blocking source defect was established. This fresh GPT-6 Astra review is independent of implementation and does not accept the candidate or authorize publication/dispatch. The required Opus review is still absent per handoff.

Reviewed commit: `cb21b30d7f1161447f2ab95d6b5ac62dfae7a600`. Candidate manifest SHA-256: `2033b9641b07a603e49e382a385ecfc4d2007e350415697efebd8431dc1a8725`. All 32 listed file hashes matched.

The inspected production path is `integrate-local` → genuine proven assets → single-use prepared deployment → exact native asset budget bindings → durable providers → driver → native finalized observation → full financial comparator. Installed SDK code confirms the prove/balance/submit/watch sequence, actual `tokenKindsToBalance` option, UNBOUND_TRANSACTION recipe, proof marker transitions and block-hash query configuration. No fictional wallet flags or expected-value fallback were found.

Balancing preserves original financial outputs, contract actions, transcripts and metadata before signing. Added funding/change and the DUST-only transaction are constrained. Gross input values, signed owners, native DUST vFee and submission counters are charged cumulatively before finalization; refund outputs do not erase charges. Durable locks, allocations and unresolved-operation stops prevent same-allocation reconstruction from resetting budgets. The asset loader checks external receipt/source bindings and exact import bytes. The observer and comparator retain actual public values separately from independent expectations.

The original expectations remain preserved. The self-mint correction adds only the two actual swap-initialize native input quantities and provenance; economic participant changes and reserves remain unchanged. Retained generated code reproduces this representation.

## Independent offline checks

- Baseline: 52 passed, 0 failed (`npm test`).
- Ledger: 146 passed, 0 failed (`npm run test:ledger`).
- Retained generated runtime: 3 passed, 0 failed (`npm run test:compiled` with the handed-off custody artifact directory).

The companion JSON records commands and output digests. These runs did not compile, prove, create/use a live wallet, submit transactions or access network services.

## Required followups and claim limits

1. **Outer process containment remains mandatory.** `providers.mjs:422` returns `containmentComplete:false`; `run-local.mjs:89-93` consequently throws `DRIVER_CLEANUP_INCOMPLETE` and reports `INCOMPLETE` after otherwise successful stages. `integrate-local.mjs:174-177` preserves that result. Inspect these predicates and run the ledger cleanup/deadline controls to reproduce the source behavior. Before an actual launch, review and demonstrate process termination, bounded pending operations and retained ambiguous-submission evidence. This false field is not waived by this review.
2. **The genuine whole path remains empirically open.** `proven-assets.mjs:127,195` rejects synthetic receipts for executable loading. No newly compiled genuine financial build, actual proof service, real wallet recipe or financial ledger execution was tested. The compiled suite uses retained generated artifacts with simulated context and transport. A separately admitted full build and local experiment must establish the corresponding claims.
3. **Acceptance gates remain closed.** `integrate-local.mjs:174` explicitly leaves financial, proof and network acceptance false. Obtain the actual independent Opus audit, preserve resource/admission gates, and retain actual local/Preview settlement, rollback and PCD evidence before claiming those outcomes. The existing status CLI operational-history stop is not cleared by this source review.

This bounded audit is not an exhaustive proof or transitive dependency attestation. Existing wallet handles, callbacks, admitted receipt authenticity and external RPC/indexer observations remain explicit trust dependencies. No source was edited.
