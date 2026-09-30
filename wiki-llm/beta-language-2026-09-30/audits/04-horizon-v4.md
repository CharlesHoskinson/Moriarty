# Independent PL audit 04 — full horizon v4

## Identity, frozen input and scope

Requested seat: GPT-6.1 Sol, medium effort, seat 4. This session provides no independent host receipt establishing the returned model identity or effort. The requested routing is recorded; actual model/effort verification is unavailable. This is a substantive scoped review, not a provider-attestation receipt.

Reviewed exact bytes:

| Artifact | SHA-256 |
| --- | --- |
| `PROGRAMMER-MOCKUP.md` | `66a0afd0bc127eaf81ec58de16a86c484852bccdc7b849313302d684a036acc7` |
| `FULL-LANGUAGE-HORIZON.md` | `3824645eb83d8671244f583de2637ec14b46cbb679a818358bdcc7a46936f251` |

Hashes were independently recomputed before reporting. Startup restored AGENTS.md and the repository development skill and refreshed guarded status. Status has no pending transactions and retains SP01.6's stale-input/accounting/resource/history blocks. Those blocks confer no product acceptance and do not prevent this authorized document review.

Scope: the whole displayed typed interface and financial lifecycle, including sections 1–16, the mockup's source/scenario mapping and status boundaries, and this seat's prior findings. I did not read peer audit or repair reports. References to peers inside the candidate were treated as author claims, not independent evidence. No production code, proofs, native authentication or ledger results were reviewed for approval. Only this audit file was written.

## Verdict

**Concur with the stated reconciliation, scheduled first-failure and Deposit/Reward repairs as proposed source semantics. Request one further medium interface repair before whole-interface concurrence.** No remaining high finding was identified in this document scope. This is not implementation, proof, financial-settlement or usability approval.

The previous v3 M1 is repaired: a signed observational action now has Query identity, its own checkpoint rule and service journal, no economic head/nonce/control consumption, and cannot become Swap's economic predecessor. Original accepted Claim evidence must supply that predecessor.

## M1 — Recovery qualification has no specified producer

**Severity:** medium. **Kind:** proposed typed-interface completeness. **Locations:** horizon section 3, especially lines 411, 454–455, 546–550, 560–575; financial callers in sections 11 and 13, including lines 1973 and 1985.

The qualification enum distinguishes Planning, Signing, NativeComplete, Observational and Recovery. Core.prepare and `readyOrPublishFirstRejection` explicitly yield a Planning scope with native/finality/atomic-commit premises still pending. OneDomain then specifies separate Core.qualifySigning and Core.qualifyNative steps. Observation separately specifies Core.prepareObservation. Recovery's expansion instead says that the same Core.prepare's qualified result enters Kernel.recover; the method rule then explicitly requires Recovery qualification. No step producing that opaque Recovery qualification is defined.

**Exact discriminator:** take OriginRefund after accepted Escrow p0→p1. Supply valid signed USD100 refund terms, complete finalized nonreceipt and future-delivery exclusion, coherent effects/footprint/duties, valid parent, grant/resource signatures and expiry, correct p1 predecessor, and a fresh durable attempt. All scheduled preparation predicates pass. Core.prepare still yields the stated Planning qualification. Passing this directly to Kernel.recover cannot satisfy the stated Recovery-kind check. The native payload/signature is still an explicit pending premise, so changing only the kind also cannot authorize a native refund dispatch. DestinationReturn likewise changes receipt/journal cells and uses this unresolved recovery entry point, even though it performs no wallet transfer.

The text consistently requires signed recovery branches and native-complete dispatch; this finding does **not** establish a bypass or exploit in any implementation. It establishes that the displayed sugar cannot yet be expanded into the declared typed protocol without an unstated qualification rule. The late assertion that recover requires signed economic branch qualification identifies the condition but does not specify when/how Core produces that distinct result or what effects a Recovery receipt permits.

**Repair:** choose and state one complete expansion. Either specify Core's scheduled Recovery qualification, its exact input/output premises, its relationship to Planning/native signing, and the receipt-to-acceptance path; or define Kernel.recover as a proof-only service with no financial/control writes, then route the actual refund/return transition through the same build/sign/NativeComplete/observe/accept sequence as OneDomain. In either case preserve Stage→Intent→Effect→Authority→History→Failure and the native signature conjunction. Do not let a recovery-service receipt establish effects or erase a duty.

**Required source tests after repair:** valid refund p1 and DestinationReturn f1 have a well-typed complete path; unsigned branch fails Failure after earlier predicates pass; expired recovery grant fails Authority; simultaneous cap/overflow plus expired grant still fails Intent/Effect first; absent exact native signature prevents dispatch; Unknown/conflict remains Pending without spend, replay reset, head advance or duty discharge. These are specified tests, not tests executed here.

## Scoped positive decisions

### Reconciliation and continuation — agree

Sections 2, 3, 9, 11 and 13 consistently distinguish Economic and Observational HeadFacts. Both reconciliation actions bind their signed QueryId, selector, Pair, verifier, original-fact selectors and bounded attempts; they publish no financial or ledger-control effects. Request qualification and response proof qualification are separate scheduled terms. A missing response proof before query dispatch is not silently treated as a verified response.

For Escrow p0→p1, lost Claim response f0→f1 and later observation checkpoint fx, the query can record evidence about fx but cannot substitute fx for f1. Qualifying the original Claim once does not remint or advance the head again. OriginRefund independently resolves p1 and requires qualified nonreceipt plus future-delivery exclusion. Count zero and timeout alone remain Unknown. The authenticated service journal charges the signed quota and cumulative parent work once per dispatched query attempt without resetting ledger controls; a third query or consumed attempt must reject in the scheduled Authority/History checks. This resolves this seat's original v3 M1.

### First failure and conjunctive authority — agree as specification

ScopeRequest is a total bounded collector. Deferred absent/invalid facts reach Core; no early `.requireOk()` chooses an Authority failure. The common schedule explicitly places Stage before Intent before Effect before Authority before History before Failure. The same schedule governs direct preparation, economic callers, governance and observational request/response qualification.

The section 16 probes are consistent: at round301 with parent [250,400] and grant expiry300, debit2/cap1 fails Intent; legal debit1 with recipient UInt128.max+1 fails Effect; invalid Stage binding wins over both; only expiry fails Authority; parent expiry300 fails Intent first. This is agreement with the specified order, not evidence that an evaluator executes it.

OneDomain requires separate Planning, Signing and NativeComplete qualifications. Payload correspondence, parent/grant/resource authority, fresh expiry/revocation/history and exact returned native signatures remain conjunctive. Successful signing does not establish dispatch or ledger acceptance. Raw native signatures cannot be assumed before the native bytes exist. The remaining recovery-kind gap above does not negate these explicit obligations.

### Deposit/Reward registry and liability preservation — agree

Section 12 reads the complete finite withdrawal and current-duty registries at the authenticated head and binds them to the tier book. Missing or partial completeness/coherence evidence is a Stage failure. NoActiveWithdrawals rejects Pending, MatureUncommitted, unburned zero-owed claims or outstanding duties before any Deposit/Reward book recomputation. Historical Paid records with zero active shares/owed and no duty remain unchanged. Bound16 is explicit; truncation cannot certify an empty registry.

This is a useful financial restriction with a future bounded active-claim update relation left open. It avoids the old case's 90-cent claim/duty divergence without claiming support for active withdrawals during deposit or reward.

### Proportional loss, receipt scopes, custody and language coverage — agree within named profiles

The signed SameTierProportional holder-tier policy separates proportional exposure from external collateral lock priority. Pending and MatureUncommitted claims remain exposed; Paid records are excluded; protected senior custody does not enter the active-tier denominator; claim/book/duty updates and rounding dust are explicit. A claim-first certificate cannot certify the displayed proportional witness.

Cumulative gross/fees/work remain independent of source-signed milestone/terminal receipt gates. Loan origination, coin mint/redemption/emergency and composed Delivered/OriginRefund/DestinationReturn have distinct receipt scopes. Preexisting wallet assets cannot satisfy fresh delivery or claim-bound custody. Refund does not erase original fees/gross or create a GOLD delivery. Signed Initial/PreviousAcceptedHead continuation preserves exact accepted effect/claim identity and requires preserving fresh authorization after unrelated head movement.

Lending's DebitCustody addresses the pledged resource and exact lock, frames Alice's free GOLD unchanged and preserves other locks and residual debt. Parent, grant, resource and native signatures/epochs/expiry remain distinct requirements. Bounded pure functions cannot mutate effects or authority. The eight displayed families expose their typed lifecycle relations and residual duties; finite route/clearing, existing burned-claim emergency payment and active-claim Deposit/Reward extensions remain explicitly open rather than silently supplied by sugar.

The mockup keeps accepted beta syntax/registry separate from local Source/6/Core/5 and the proposed horizon. The independent transfer/repayment mapping still exposes required unverified bindings and premises; a visible example or checked coverage box is not authenticated success. I do not infer developer usability from source inspection; the separately requested developer trials remain separate evidence.

## Independently reproduced arithmetic

Executed a bounded inline Python integer-arithmetic probe against the literal values; no horizon parser, evaluator, certificate verifier, model, kernel or ledger was run.

| Probe | Reproduced result |
| --- | --- |
| B1210/S1100/L121; weights100/1000 | 99/990; losses11/110 |
| Same tier weights100/200/800 | 99/198/792; total1089 |
| B101/S3/L1 | 33.33/66.66 plus reserve dust0.01 |
| Distinct exposed B1110, loss121, weights100/1000 | 89.90/899.09 plus dust0.01 |
| Deposit100 into B1001/S100 | floor(10000×100/100100)=9 shares |
| Old unsupported active10 claim | 10010→10100 cents; difference90 |
| Supported free100/new9 after B1101/S109 | 101009/9090 cents plus dust1 cent |
| Loan500 less repayment30 | Whole-life delta470, distinct from origination milestone500 |
| Scoped credit520 less charge20; credit500 less20 | Net500 passes; net480 fails floor500 |
| Coin backing0.200 less redemption0.190 | Whole-life GOLD loss0.010; distinct receipt0.190 |
| Destination input99.70 plus fee0.30; source100 plus0.20 | Destination gross100; source gross100.20 |
| Pledged custody2 less sale1; debt475 less400 | Custody1 and residual debt75 |
| Call spot140/strike100; writer reserve500 | Payoff40; residual reserve460 |
| UInt128.max plus1 | 2^128, outside UInt128 |
| Parent[250,400], round301, grant expiry300 | Parent valid, grant expired |
| GOLD floor0.900 less actual0.899 | Missing0.001 |

Arithmetic does not certify allocation policy, evidence completeness, signatures, provenance, first-failure execution or atomicity. The section 16 tables state those semantic premises separately and continue to label their execution unperformed.

## Remaining obligations and abstention

Repair M1 and obtain fresh exact-byte whole-interface review. Proposed horizon grammar/elaboration and boundedness; all profile certificate equations; scheduled evaluator correspondence; canonical signing bytes and unforgeable qualifications; fresh native authorization; reservation, replay/head consumption and accepted-history linkage; query-journal authentication/deduplication; complete registry proofs; and native ledger effects/finality remain open. Beta implementation acceptance requires its own executable and empirical evidence. Developer trials must establish usability independently.

I abstain from implementation, proof, native-authentication, ledger-settlement, usability and release approval. Scoped concurrence attaches only to the two hashes above and does not replace the preserved prior audits on their original inputs.
