# Independent GPT-6 result review

Candidate `c6249e14c06907946ff47f0815b1540abf4bb65409a234ce3a94ddb611a87f52`.
Reviewer: OpenAI GPT-6 Astra (`gpt-6-astra`), independent source review in task `/root/ledger_effects_successor02_review`.

**Scoped verdict: REJECT / R6 INCOMPLETE. Overall SP05: BLOCKED.** This is diagnostic review of a failed author run, not acceptance. Grok reached its configured 48-turn limit, exited 1 with `stopReason: cancelled`, and left partial source. `failure-diagnosis.json` and the stopped ledger remain controlling. No retry or acceptance is authorized by this review.

## Decisive reproduced findings

1. **High: material contract effects escape comparison across binding.** Native `ContractCall` fixtures with a guaranteed transcript pass the unchanged control. Changing that transcript's unshielded mint from empty to seven units or its compute gas from zero to one changes native serialization. Same-phase comparison rejects both. Binding the changed native transaction makes `recheckFinalizedAgainstRecipe` accept both against the original recipe. `compareSignatureData` returns early across binding phases (providers.mjs:362); `compareActions` checks only address, entry point, communication commitment and transcript presence (527). It never compares transcript effects, gas or program. Native maintenance updates similarly accept a changed replay counter or replacement authority after binding. Thus unsupported action families are not consistently rejected: material changes are accepted. These are offline comparator counterexamples with synthetic UTXO references and copied signatures; they do not establish valid proofs, ledger acceptance or a submitted exploit.

2. **High: the ordinary dust-fee funding path remains unsupported.** `fundUnshielded` calls pinned facade `balanceUnboundTransaction` with only `{ttl}`. Facade defaults `tokenKindsToBalance` to `all`, invokes dust balancing, and returns its fee transaction in `balancingTransaction` (facade/dist/index.js:357–385). Dust `Transacting.balanceTransactions` creates an intent with `DustActions(...spends...)` (dust-wallet/dist/v1/Transacting.js:300). Candidate gross inspection rejects every nonempty dust spend before reservation/finalization (providers.mjs:183). Consequently a normal recipe requiring a fee spend cannot traverse this driver. That rejection prevents admission; it does not fulfill R6. `DustSpend.vFee` is explicitly exposed as bigint in the pinned ledger declarations (1485). Lack of exposed shielded input values is not a reason to call dust fee amounts unobservable. DUST fees and the selected asset's gross debit need separate units and explicit budget semantics.

3. **Medium: malformed counter state can bypass gross admission.** Through the actual public funding function and an inert finalization stub, a native signed input of 100 reaches the boundary with cap 50 when `reservedGross=-1000`; the resulting reservation is -900. Omitting `counters.grossSpend` also reaches the boundary. `reserveBudget` validates requested deltas but not counter presence/type/nonnegativity (141). Normal driver defaults initialize well-formed counters, so these are public API validation failures, not demonstrated corruption of the default driver's internal counters.

4. **Accounting and custody coverage is incomplete.** The repair now sums all guaranteed/fallible unshielded inputs in recognized recipe parts and segments, keeps refunds from reducing gross, and rejects underdeclared gross and zero submission allowances. It rejects shielded inputs/transients and multiple assets instead of accounting for them. The custody bindings distinguish USD_micro and swap asset A/B; one untyped aggregate does not express those units or fee duties. Native dust registrations with `allowFeePayment=900` pass gross inspection with only the 100-unit unshielded input counted. This supplemental fixture does not validate the registration's own signature or establish an actual 900-unit debit; it demonstrates that this fee authorization family is neither accounted for nor explicitly rejected. Do not sum its value into the other asset's gross or claim a proven spend exploit.

## Verified capability and boundary

The fixed scope includes native unchanged unshielded comparisons, guaranteed/fallible Zswap output comparisons, network changes, disjoint recipe merges, and complete unshielded gross aggregation in tested parts. The previous underdeclaration and base-plus-balancing counterexamples now reject. The positive public funding control reserves 100 gross and one submission before the inert finalization boundary, and retains both when finalization throws. The zero-submission control rejects with no reservation. Reservation updates are synchronous and check both caps before either increment for well-formed counters. The submit wrapper retains a per-wrapper no-retry guard and does not release reservations on errors; this is not a broader durable/shared retry or deadline guarantee.

Production order is balance → sign → verify unshielded input signatures → inspect gross → check caller allowances → reserve actual gross and one submission → finalize → compare → submit. Native facade finalization also books pending transactions before returning. The review did not invoke that finalization or any real wallet, proof or network. Gross/allowance checks occur after balance/sign, so the evidence does not establish that malformed allowance calls cause no wallet preparation effects.

## New evidence and verification

`gpt6-independent-probes.mjs` imports the frozen candidate and records 14 cases in `gpt6-independent-results.json`. Two unchanged native action controls pass; four changed native action cases reject in the same phase but are unexpectedly accepted across native binding. Native construction errors: zero. Funding controls include valid admission, zero submission, malformed counter state, a fee-registration accounting observation, and a 300-unit two-part gross check.

A bounded alternative was experimentally checked: `eraseProofs().serialize()` yields equal bytes for unchanged maintenance/call bind controls and an unchanged disjoint merge/bind control, while all four altered action cases yield unequal bytes. This supports examining canonical proof-erased native transaction comparison after native recipe merge. It does not prove normalization across real proof generation, all SDK families, ordering choices or genuine finalized transactions. No implementation was changed.

Commands retained in `gpt6-verification.json` and `gpt6-check-*.stdout/stderr`:

- `node gpt6-independent-probes.mjs` (final output additionally retained in `gpt6-independent-probes.stdout`).
- `node --test experiments/moriarty-midnight-financial/ledger/receipt.test.mjs`: 36/36 pass, run from the original worktree after hashes were checked.
- `node --test root-effects.test.mjs`: 7/7 pass.
- `node --test root-sdk-effects.test.mjs`: 3/3 pass.
- `node --test root-sdk-recipe-shape.test.mjs`: 3/3 pass.
- `node --test root-gross-admission.test.mjs`: 3/3 pass.
- `node gpt6-replayed-predecessor-probes.mjs`: all 38 predecessor expectations met, 5 positive and 33 negative. This replay has new output paths and preserves all `retained-gpt6-*` evidence.

The candidate hash, all eight frozen files and all binding source hashes were checked. Supplemental directly inspected SDK source digests are recorded in the JSON review. Source and pins were unchanged after execution. The supplied suite uses the original worktree because it relies on its custody fixtures; focused probes import the frozen provider. Native `bind`, `merge`, serialization and proof erasure on synthetic objects were used. No prove/mockProve, wallet restore, actual facade finalization, actual submission, network, source edits or other agents were used.

## Report disposition and remaining gates

`FOREMAN_REPORT.json` is stale predecessor evidence: it says gross still scans base segment 1 and trusts caller gross, although this source has changed that behavior. Its scope/results cannot attest this hash. The newer Markdown correctly leaves R6 incomplete, but its description of unsupported contract transcript internals must not be read as fail-closed enforcement; the reproduced comparator accepts their mutations. Preserve both reports as failed-run evidence and reconcile before any later source acceptance.

R1–R5/R7–R8, proof and binding compatibility beyond these synthetic tests, financial Preview settlement, PCD, full SP05 and the twelve-sprint program remain open. No merge, release, proof dispatch or product acceptance follows from the passing offline tests.
