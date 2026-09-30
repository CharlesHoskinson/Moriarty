# K to Quint projection candidate

**Status:** revised design map after nine-seat Sprint 0 review and finite S0 witness execution. K and Quint have executed fixed local transfer, repayment and hostile cases. No ledger-committed K stage or complete K-to-Quint projection has run. The projection's committed domain remains empty.

## Domain and action grain

Define `α` on **ledger-committed K stages** whose authenticated pre-state, prepared effects and atomic head/replay consumption match one signed statement. K-local preparation alone is outside that domain. The intended initial relation is `α(initK) ⊆ initQ`. For each committed stage, require `α(preK)=preQ`, a matching enabled Quint commit action, `α(postK)=postQ`, and equality of the complete ordered effect and consumption records. A finite comparison is trace-correspondence evidence for that corpus, not a universal refinement proof.

The executed S0 corpus uses a narrower local observation relation `α_local`. Three separately written common positive fixtures align numeric balance, obligation, allowance, work and round inputs. Their literal complete pre/effect/post outputs agree after an explicit identifier map; the reviewed packet uses a manual map and has no executable general adapter. The older non-common K, Quint and TypeScript fixtures still use different initial allowance, work and round values. Hostile overlap compares first judgment and code in the current witnesses. Source formation failures, unverified signed-wire fields and external authentication premises are outside this local relation. The finite comparison and its limits are recorded in `deliverables/mil4-k-quint-sprint1-2026-09-29/S1-FINITE-CORRESPONDENCE.md`.

For these single-asset fixtures the local map supplies asset `A` to Quint money and allowance lines, whose constructors omit it; K debit, credit and allowance lines carry `A`, while TypeScript debit and credit carry `A` but `UseAllowance` omits it. Quint's obligation value carries `A`; K and TypeScript `SetObligation` lines omit it. Replay is a K typed key, Quint triple and TypeScript serialized triple. Round is adjacent to K state, inside TypeScript state and also in its stipulated tuple, and outside Quint's financial snapshot. These differences must be checked explicitly by any executable adapter. A requested pending phase is also outside the common S0 failure domain: Quint currently classifies it at Failure, TypeScript at Stage, and K has no such phase constructor.

Keep `sign(intent)` and `submit(intentId, fill, claimedPreHead, submittedEffects)` as distinct abstract actions. Signing stores immutable version, program, signer, payer/owner, debtor/creditor, recipients, fee beneficiary, caps, validity interval, failure branch and evidence choice. Submission checks that the fill narrows the signed scope, the claimed pre-head is current, the intent has not been consumed, and time is within the signed interval. One successful submission is an atomic K stage and head update. A stale competing proposal remains representable as a disabled commit with a semantic rejection observation. Do not create a no-op successor solely to model rejection.

## State and observation mapping

| K field or result | Quint image | Required distinction |
| --- | --- | --- |
| Version, selected Core program, signed template and resolved fill | Immutable intent table and selected fill | Keep every fixed endpoint, gross/fee/net bound, failure branch and policy digest. Compare supplied fields with signed fields. |
| Authenticated pre/post cells and derived footprint | Typed maps keyed by domain, asset, account, obligation, episode or claim | Keep each read/write value. Preserve nominal identities and absent-versus-present cells. |
| Complete ordered effect vector | Ordered typed lines **and** derived cell deltas | Keep line direction and gross debit before netting aliases. Track charged fees and their beneficiary. Compare zero-line policy explicitly. |
| Balances, custody, supply and aggregate cells | Per-domain asset/account balances, episode custody, supply and aggregate state | Enforce local conservation after alias resolution. Cross-domain backing is a separate qualified premise. |
| Obligation | Principal, accrued, outstanding, status, settlement asset, authenticated debtor and creditor | Only credit to that obligation's bound creditor can discharge debt. A caller cannot substitute a creditor. |
| Signer, allowance, grant, budget and work | Owner-keyed remaining **and spent** counters, grant epoch, gross use, remaining work and closure reserve | Tie the debited payer to the signed signer or an explicit grant. Refund does not restore gross use. |
| Predecessor/head, replay, receipt and nullifier | Opaque current head and consumed ID sets | Replay ID is the signed intent nonce/digest scoped by domain and signer. Separate legitimate partial repayments use distinct IDs. K authenticates commitments. |
| Validity, observation round and policy epoch | Finite abstract round or epoch | A fresh but nonselected older observation is distinct from the selected current round. Time alone never proves foreign nonreceipt. |
| Local rejection | `(judgment, stableCode, noCommit)` observation | Preserve the first failing semantic judgment and code for the comparison corpus. No state/effect successor exists. Lexical failures remain outside Quint's action model. |
| Accepted signed failure, pending or unknown | Separate committed phase observation with retained effects, custody, work and duties | These are not local rejections. Keep their authenticated successor state and continuing duty. |

The comparison adapter may compute a no-commit result for a disabled action. It must retain the K judgment and code; a generic disabled action is insufficient for rejection-precedence comparison. The S0 failure policy permits atomic local rejection and terminal success only. Later signed phase failures, escrow pending and bridge unknown have distinct transitions and observations.

## S0 model shape

Use one owner/signer and explicit signed recipient and fee recipient for transfer. The commit action debits gross `v+f`, credits `v` and `f`, consumes allowance remaining by `v+f`, increases allowance spent by `v+f`, consumes the signed replay ID and advances the head. Keep all three effect lines before alias resolution. The exact self-transfer and zero-fee canonical policies remain open W-D3 leaves; the model must reject or represent them explicitly.

Repayment reads an obligation that binds debtor, creditor, settlement asset, principal, accrued, outstanding and status. It debits the signed payer and credits that bound creditor by `n`, consumes gross allowance and replay, and applies AccrualFirst with identity conversion. The model checks `0<n≤outstanding`, `outstanding=principal+accrued`, the first-slice signed nominal bound, UInt128 balances and counters, and exact post status. Debt cannot fall if the matched creditor credit is absent.

Quint integers do not overflow. Guard every balance and counter at `2^128−1`, and every first-slice nominal amount and liability cap at `2^127−1`. The current lifecycle state fields are UInt128, while its source/kernel nominal admission applies the signed bound. Record those different domains rather than saying all obligation fields have the same wire cap.

## Limits and work order

The present K surge's `M4T/1` escrow state is not S0 transfer or repayment state. `M4D` family arithmetic and `M4E` effect shapes are disconnected from its six-judgment admission. `m4AdmitFamily` rejects every first family. Keep `α` undefined for those projections. A Quint model of their arithmetic is an independent design experiment, not K admission.

Extract `common.qnt` after the signed scope, replay namespace, result observations and state types are selected. The isolated `s0.qnt` is a typechecked, fixed-witness prototype. Three common positive fixtures compare complete literal financial observations; the hostile corpus covers gross debit, creditor binding, recipient substitution, stale head, replay and overflow on its stated overlapping domain. The separate head, failure and premise experiment has executed finite witnesses under stipulated tuples and remains provisional. A complete general comparison still needs one canonical input lowered through all artifacts and an executable general projection. No `quint verify` model check has run. Signature truth, native proof validity, oracle provenance and foreign finality remain named external premises.
