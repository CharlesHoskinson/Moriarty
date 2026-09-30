# MIL/4 K surge result

**Status:** specified-only implementation draft, 2026-09-29. This directory is isolated from the active Moriarty K implementation. It does not adopt MIL/4 or satisfy U0, native, or Preview gates. The guarded SP01.6 lane was blocked when this work began and no transaction was dispatched.

The seven K files contain 1,715 lines. `kompile --backend kore --main-module MIL4-PROPOSAL --syntax-module MIL4-PROPOSAL-SYNTAX --output-definition /tmp/moriarty-mil4-proposal-kompiled mil4-proposal.k` completed with exit 0 on K 7.1.337 after the audit-driven edits. The 30 compiler warnings concern unused pattern variables and are retained in `compile.stderr.log`. No K execution or semantic test was run.

## Composed entry points

`mil4-proposal.k` is the composition root. Its entry points expose these K relations:

| Entry point | Rules | Scope |
| --- | --- | --- |
| `m4Check` | `M4S-STATIC` | Nominal type and formula formation, bounded arithmetic and explicit rejection of open Oracle, Ω and Φ₁ terms. |
| `m4Evaluate` | `M4F-FORMULA` | Concrete Φ₀ truth or named rejection over typed, head-bound finite values. Authentication of the supplied values is an external premise. |
| `m4Step` | `M4T-TRANSITION` | A one-domain, one-asset conditional escrow slice, including cumulative accounting and explicit pending/unknown/recovery outcomes. |
| `m4Accept` | `M4C-CONTRACT` | Ordered stage, intent, effect, authority, history and failure judgments for that same escrow slice. Stage now evaluates the concrete outcome formula and rejects false or undefined results. Native qualification and atomic ledger consumption are premises. |
| `m4FamilyProjection` | `M4D-DEFI` | Pure arithmetic and evidence-shape projections for eight first DeFi profiles. Its `m4dProjected` value is a local calculation over supplied data. |
| `m4AdmitFamily` | `M4D-DEFI` | Explicit fail-closed boundary: every first family rejects with `FAMILY_STAGE_ADAPTER_ABSENT`; later profiles reject as unsupported. |
| `m4FamilyEffects` | `M4E-EFFECTS` | Exact finite read/write/effect list shapes for the eight first profiles. It does not authenticate values or apply the effects. |

## Eight family boundaries

| Area | First relation | Explicitly open before complete admission |
| --- | --- | --- |
| AMM and exchanges | Exact-input constant-product quote with exact rational fee and finite effect schema | Signed fee policy, authenticated reserves/custody, target widths and the full stage adapter. |
| Lending | Existing AccrualFirst funded S0 repayment with creditor credit | Existing obligation authentication, principal/accrued post-state adapter, and native settlement. Originate/accrue remain outside S0. |
| Stablecoins | One collateralized mint with cap/health guard and supply/debt schema | Price provenance, issue grant, aggregate authentication and complete burn/redemption policy. |
| Derivatives | Capped funded call payoff and one-shot exercise schema | Signed premium, fixing selection, reclaim and full reserve/claim state. |
| Oracles | Selected round/freshness and observation schema | Canonical tuple, verifier locus, key epoch and actual provenance verification. |
| Governance | Current grant epoch/threshold and policy-duty schema | Distinct approval verification, queue/cancel semantics, protected duty migration and pause policy. |
| Bridges | Source lock and receipt arithmetic with partial entitlement shape | Cross-domain issue/conservation, verifier/finality, bytes/nullifiers, cumulative receipt state and terminal nonreceipt policy. Refund is rejected by the effect schema. |
| Vaults | One-asset share floor/remainder and deposit schema | Share issue authority, donation/recognition ownership, custody/surplus authentication and full stage adapter. |

## Current admission limit

The six-judgment `M4C` relation is defined only for `M4T/1`. The `M4D` arithmetic and `M4E` effect schema are separate projections; no rule joins their quantities, old values, post values, authority, and observed ledger effects. Consequently no DeFi family is admitted by this proposal. `m4AdmitFamily` enforces this limit in K. K compilation checks parsing and module consistency only; it is not evidence of semantic soundness or native enforcement.

## Recommended decisions and implementation order

These are recommendations for the open MIL/4 decision register, not adopted defaults.

| Decision | Recommendation | Required discriminator |
| --- | --- | --- |
| M4-C1, names and bytes | Version a canonical `/4` source/Core encoding with explicit domain and asset IDs, `Price<Base,Quote,Scale>`, one selected-program digest, and unknown-tag rejection. Keep `/3` input separate until an explicit migration rule exists. | Rename a field, invert a price, add an unknown tag, and compare the exact digest or rejection. |
| M4-C2, fee and rounding | Use the retained-in-pool exact rational fee for the first AMM profile; represent quotient remainders as product metadata. Credit the protocol reserve only when a separate whole atomic unit and balancing effect are identified. Keep the escrow slice at zero fee until a signed fee formula and beneficiary are defined. | The 11 A, 1000/2200 pool gives 23 B under the retained rational fee and 21 B under an upfront one-unit fee. The vault's remainder 500 must create no asset or share transfer. |
| M4-C3, arithmetic | Introduce closed, versioned Ω constructors only for certified first-profile primitives. Bind widths, canonical quotient/remainder and exact rejection to source/Core/K/native. Continue rejecting general Φ₁ and uncertified Oracle or Ω terms. | An otherwise valid forged quotient and overflow must reject on the pinned target, not only in host K arithmetic. |
| M4-C4, cells and effects | Derive the complete read/write footprint from the selected operation and resolved holes. Authenticate every pre-value and calculate every post-value and effect line; never accept a caller-supplied effect digest as sufficient. | Delete one recipient, aggregate, reserve, grant, duty or policy cell from an otherwise valid stage; reject before any prepared state. |
| M4-C5, authority and evidence | Sign canonical bytes covering program, scope, recipients, failure branch, duties and effect bounds. Specify the signature scheme, digest, verifier locus and public inputs. Treat foreign receipts and terminal nonreceipt as qualified verifier facts; `unknown` remains a durable state. | Substitute a recipient byte, revoke a grant, replay a receipt, or omit a retained duty; each must reach the named judgment. A timeout alone cannot refund a bridge claim. |

The implementation order is S0 funded repayment, then the local escrow slice with a closure reserve, then AMM with a complete pool-state adapter. CDP, option, oracle, governance and vault each need their registered cells and verified evidence before their numeric projections can join the six judgments. Bridge settlement stays specified-only until source/destination conservation, partial claim state and a qualified foreign verifier are available. This order follows the existing U0-first roadmap and avoids treating a kernel or venue receipt as a language financial effect.

## Independent-review obligations

The first Opus 5.5 transition review (`opus55-transition.md`, G1–G19) identified concrete counterexamples that the first escrow slice does not close: gross caps and fees in the MIL/2 showcase, direct delivery followed by refund giving the owner both assets, incomplete actor-relative footprints, missing budgets/refund authority, exhausted closure slots, signature reuse across episodes, custody aliases, and conserving but redirected effects. These are required design repairs, not accepted behavior. Its recommendation to key custody by episode and reserve closure work is particularly relevant to the next transition profile. Later review results and compilation status must be recorded before this result is treated as final.
