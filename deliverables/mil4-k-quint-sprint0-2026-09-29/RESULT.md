# MIL/4 K and Quint Sprint 0 result

**Status:** in progress, not frozen. This sprint began the semantic boundary and constructor inventory, then added an isolated provisional Source/6 presentation, K S0 definition, Quint S0 model and TypeScript candidate preparer. It did not implement a Source/6 parser or ledger admission, run tests, dispatch a campaign, or change the guarded source/Core runtime. SP01.6 remained blocked by stale binding/candidate inputs, missing accounting and unavailable live resource state. No public transaction was reported.

## Concrete outputs

- [Semantic contract candidate](../../experiments/moriarty-language/formal/mil4/semantics-contract.md): Source/6 → Core/5 direction, S0 transfer/repay stage, six judgments, fail-closed first-profile boundary and historical K disposition.
- [Constructor inventory](../../experiments/moriarty-language/formal/mil4/coverage.tsv): 267 provisional rows. It includes 48 Source/5 grammar productions, 60 Core/4 expression constructors, Source/6/Core/5 candidates, S0 leaves, historical lifecycle and K files, later-profile rejects and eight first profiles. `unmapped`, `missing`, `to-classify` and `map-or-reject` rows are explicit incomplete work, not coverage.
- [Decision register](../../experiments/moriarty-language/formal/mil4/decisions.md): W-D0–W-D6 and M4-C1–M4-C5 remain open. Two independent external reviews support the Source/6 → Core/5 direction only.
- [K-to-Quint projection](../../experiments/moriarty-language/formal/mil4/projection.md) and [common state design](../../experiments/moriarty-language/formal/quint/mil4/common-state.md): preserve signed endpoints, gross effect lines, bound creditor, spent counters, replay, head, first rejection and accepted phase failures.
- [S0 expected outcomes](../../experiments/moriarty-language/formal/mil4/s0-discriminators.md) and [proof register](../../experiments/moriarty-language/formal/mil4/proof-register.md): independent positive/hostile cases and 15 open MIL/4 claim domains. No proof is discharged.
- [Source/6 grammar](../../experiments/moriarty-language/spec/successor/financial-agreement-source-v6-grammar.ebnf) and [companion contract](../../experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md): closed one-stage S0 presentation and proposed Source/6 → Core/5 mapping. This is not parsed by the runtime.
- [K S0 prototype](../../experiments/moriarty-language/formal/k/mil4/s0.k) and [Quint S0 model](../../experiments/moriarty-language/formal/quint/mil4/s0.qnt): separate definitions for the provisional transfer and funded repayment slice. External signatures, snapshots, native qualification and ledger compare-and-consume remain premises.
- [TypeScript S0 preparer](../../experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts): computes a local candidate and returns `PreparedUnqualified` with the external premises still required. It never returns ledger admission.

## Nine-seat review

Three GPT-6 Sol internal seats independently inventoried Source/Core, historical K and Quint abstraction. They performed read-only work and ran no tests. Three Opus 5.5 CLI reviews returned `claude-opus-5-5`. Three Grok 4.7 CLI reviews returned `grok-4.7-build`. Their raw receipts and prompts are in [audits](audits/). Grok plan-mode attempts ended `cancelled`; the completed reviews used a fixed text packet with tools disabled. The completed Opus sessions had read-only file tools but could not run guarded status or recompute digests. The root agent inspected guarded status and fixed the candidate hashes before review.

| Lens | Opus finding | Grok finding | Disposition |
| --- | --- | --- | --- |
| Source/Core boundary | Agree on Source/6 → Core/5 direction; current bytes cannot freeze. | Agree on direction; wire encoding remains separate. | Candidate direction retained; version row remains open after revisions. |
| K scope | No inherited proof or family admission, but S0 judgment/effect/rejection/proof detail is incomplete. | Same inadequacy vote; first-profile adapters still reject. | Added S0 leaves, expected outcomes and proof register, then a provisional K S0 definition; no proof claimed. |
| Quint projection | Rejected netted gross debits and missing bound parties, signer, signed scope, head and replay. | Rejected erased first judgment, spent/work, signed endpoints and accepted-failure duties. | Rewrote the design map and added a provisional `.qnt` model; no simulation claimed. |

The initial frozen review candidate hashes were: semantic contract `3416040e4c6746f39dddfa3fb92eb89c181364d0fba98f7ca2186d0f70321201`, coverage `04a9123a8190c69f9808c4a9b49b2b427dd4592fa054cba5a88bf3adbf3f2c69`, decisions `806cbe7809a9c0353f3ad19e4c789fd2acdc7d572a43defece7715c5d4047030`, projection `9516510e2d245ed570a367a043fd3f469279284c28a000cf5f7c4c2ed98147a2`. The revised files have different hashes and have **not** received a fresh full-candidate audit.

## Implementation slice and audit

The [S0 implementation contract](../../experiments/moriarty-language/formal/mil4/s0-implementation-contract.md) fixes a provisional comparison slice: signed literal-fee transfer, funded AccrualFirst repayment, a complete ordered effect vector, six first-failure judgments, replay, authenticated round/head premises, and one unit of work on success. Source/6 is a closed EBNF draft, not a parser. K can produce an S0 observation only when its trusted external cell supplies authentication; the TypeScript function returns a local `PreparedUnqualified` candidate with the missing premises listed. Quint models signed and submitted actions separately, but its integer head successor is abstract. None of these implements the ledger compare-and-consume path.

An independent [cross-artifact audit](audits/s0-cross-artifact-implementation-review.md) found and drove corrections to signed amount binding, nonzero-round handling, authenticated current round, exact effect vectors, repayment endpoints, nominal bounds, allowance width, work consumption, and validity endpoints. The audit was static and did not validate transition behavior. The final revised bytes have not received a new nine-seat vote.

Final syntax and type formation commands all exited 0 with no diagnostics: `quint typecheck experiments/moriarty-language/formal/quint/mil4/s0.qnt`, `kompile experiments/moriarty-language/formal/k/mil4/s0.k --backend kore --main-module MIL4-S0 --syntax-module MIL4-S0-SYNTAX -o /tmp/mil4-s0-final-kompiled`, and `npm --prefix experiments/moriarty-language run typecheck`. These commands do not establish K execution, Quint simulation, cross-model agreement, or ledger admission.

## Blocking work before Sprint 0 exit

1. Implement and audit the actual Source/6 parser, lexical and version reject rules. Expand every new constructor, cell, effect and result class to an exact row. Resolve each `unmapped`, `missing`, `to-classify` and `map-or-reject` row.
2. Choose W-D0–W-D4 exact S0 bytes, signature and snapshot-authentication premise, rejection precedence, fee/alias/zero-line behavior, replay namespace and numeric domain. Reconcile the older Source/5/Core/4 S0 packet as a historical input.
3. Freeze the revised full candidate and obtain fresh substantive votes on its exact bytes. Existing boundary votes support direction only. Keep dissent and unavailable premises visible.
4. Reconcile and audit the provisional K, Quint and TypeScript S0 implementations against the same typed inputs and observations. Build `common.qnt` and a mechanized K-to-Quint projection. The old surge escrow and DeFi arithmetic projections do not satisfy either deliverable. Native and ledger acceptance remain separate gates.

TypeScript `tsc`, K `kompile` and Quint `typecheck` were used for syntax and type formation of the prototype. No tests, semantic traces, Quint simulations, model checks, or proofs were run. The compile and typecheck results do not validate the semantics. Earlier `git diff --check` found no whitespace errors in tracked changes; that check does not validate the new semantics or untracked files.
