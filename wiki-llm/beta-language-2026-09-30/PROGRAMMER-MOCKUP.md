# Moriarty programmer-facing beta and full-language horizon

**Status:** repaired design mockup after v2 review, 2026-09-30; specified only. This is a
successor proposal, not a grammar freeze, an implemented beta claim or financial
acceptance. [FULL-LANGUAGE-HORIZON.md](FULL-LANGUAGE-HORIZON.md) supplies the typed
full-language examples, contracts and requirements checklist. Read this page for
the smaller authoring surface and its exact relationship to the existing local
baseline.

## Three distinct surfaces

| Surface | Status and evidence | What a programmer can infer |
| --- | --- | --- |
| Existing `moriarty-financial-agreement-source/6` → Core/5 S0 | **local** parser/preparer; [contract](../../experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md), [wrapper](../../experiments/moriarty-language/src/successor/mil4-s0-source-v6.ts), [tests](../../experiments/moriarty-language/tests/mil4-s0-source-v6.test.mjs) | Closed atom-valued transfer or funded AccrualFirst repayment; `PreparedUnqualified` remains a preparation candidate. |
| Proposed `moriarty-beta/1` brace records | **specified** by [BETA-DESIGN](BETA-DESIGN.md) and [CONVERGENCE](CONVERGENCE.md); no command receipt is claimed by this mockup | Exact quantity sugar and a closed name/argument registry are planned. Only transfer/repay may lower. Recognition of another operation is not financial validation. |
| Proposed `moriarty-horizon/0.1` contract notation | **specified interface; open implementation/proof** in [FULL-LANGUAGE-HORIZON](FULL-LANGUAGE-HORIZON.md) | Typed cells, holes, authority, observations, outcomes, profile relations and episodes are proposed constructs. This notation is not accepted beta grammar. |

The status labels are scoped: **local** means the inspected existing S0 path;
**specified** means this written proposal; **open** means the implementation,
proof or qualified evidence has not been established here. New beta code requires
its own measured command results before its status changes. Hashes, names,
matching fixtures and service acknowledgements authenticate nothing by themselves.

## Syntax tour: proposed beta records

A `profile` selects a version. An `agreement` names a nominal source scope.
Declarations bind identities or immutable pure values; named arguments expose
roles and units. An `intent` fixes the request, limits and explicit unsupported
scope fields. An `action` selects an intent. The scenario is separate untrusted
text; it supplies snapshot cells and a candidate head, never new recipients,
amounts, bounds or selected programs.

Both examples below are **specified beta syntax**, with an existing local S0
semantic target. Include this common prelude inside each agreement; this is a
documentation convention, not an implemented import:

```mori
  domain Preview = { id: "Midnight", chain: "midnight", network: "preview" };
  account Buyer = { domain: Preview, id: "Owner" };
  account Seller = { domain: Preview, id: "Recipient" };
  account Treasury = { domain: Preview, id: "Fee" };
  asset USD = { domain: Preview, id: "A", representation: "canonical",
                scale: 2, symbol: "USD" };
```

### Transfer: exact value and explicit literal fee

```mori
profile "moriarty-beta/1";
agreement Invoice {
  // Insert the common prelude above.
  const price: Qty<USD> = 10.00 USD;
  const fee: Qty<USD> = 0.10 USD;
  intent Payment = {
    domain: Preview, asset: USD, signer: Buyer, key: "key1", nonce: "n1",
    pre_head: "h0", valid: rounds(domain: Preview, from: 100, to: 200),
    gross_cap: price + fee, fee_cap: fee, net_floor: price,
    operation: transfer(from: Buyer, to: Seller, fee_to: Treasury,
                        value: price, fee: fee),
    source_hash: "src1", policy_digest: "policy1", failure: SuccessOnly,
    observations: [], disclosures: [], retained_effects: [], retained_duties: [],
    delegation: None, recovery: None,
  };
  action pay uses Payment;
}
```

### Funded repayment: creditor and debt come from the obligation cell

```mori
profile "moriarty-beta/1";
agreement Repayment {
  // Insert the common prelude above. Buyer is this obligation's debtor.
  obligation Loan = { domain: Preview, asset: USD, id: "Loan" };
  const payment: Qty<USD> = 30.00 USD;
  intent Payment = {
    domain: Preview, asset: USD, signer: Buyer, key: "key1", nonce: "r1",
    pre_head: "h0", valid: rounds(domain: Preview, from: 100, to: 200),
    gross_cap: payment, fee_cap: 0.00 USD, net_floor: 0.00 USD,
    operation: repay(obligation: Loan, payer: Buyer, amount: payment),
    source_hash: "src1", policy_digest: "policy1", failure: SuccessOnly,
    observations: [], disclosures: [], retained_effects: [], retained_duties: [],
    delegation: None, recovery: None,
  };
  action repay_loan uses Payment;
}
```

The creditor is not a solver hole. The stipulated cell binds `Loan` to Buyer,
Seller, USD and its accounting fields. Repayment has identity conversion and
AccrualFirst only; no conversion/fee argument or default debt component is added.
`Qty` expressions/scalars use checked UInt128 `0..2^128−1`; S0 value, fee,
amount, caps, floors and principal/accrued/outstanding narrow to `0..2^127−1` at
field lowering. Balances, allowance counters, rounds, gross sum and work use
UInt128. An out-of-range field is an authoring/formation error. No decimal
rounding, implicit asset conversion or floating point is proposed for beta.

## Independent transfer and repayment trace

These are explicit expected observations derived from the S0 contract, without
calling the proposed beta compiler. All quantities below are **atoms** of A at
scale 2. Each column is an independent stage; neither uses the other's post-head.

| Step | Transfer `Invoice.pay` | Repayment `Repayment.repay_loan` |
| --- | --- | --- |
| Fixed source intent | Owner→Recipient value 1000; Fee receives 10; gross cap1010/fee cap10/net floor1000; `n1`, h0, rounds100..200 | Owner pays Loan 3000; gross cap3000/fee cap0/net floor0; `r1`, h0, rounds100..200 |
| Completion/scenario | Head h0, predecessor genesis, round150; balances Owner10000, Recipient200, Fee50; allowance Owner remaining5000/spent100; replay unused; work remaining10/spent2; candidate h1 | Head h0, predecessor genesis, round150; balances Owner10000, Recipient200; allowance remaining5000/spent100; Loan debtorOwner/creditorRecipient/assetA/principal10000/accrued1000/outstanding11000/Outstanding; replay unused; work remaining10/spent2; candidate h1 |
| Typed request | `TransferLiteralFee`; signed and submitted endpoints/value/fee identical | `RepayAccrualFirst`; payer=debtor=signer; creditor is bound Recipient; `IdentityConversion(1,0,None)` |
| Required kernel facts, **open** | Exact intent signature + source/policy binding; same-head balances/allowance/work/round; replay unused; authenticated predecessor/successor; atomic compare-and-consume | Same, plus same-head Loan/creditor/asset/accounting authentication |
| Ordered derived effects | `Debit(Owner,1010)`, `Credit(Recipient,1000)`, `Credit(Fee,10)`, `UseAllowance(Owner,1010)`, `UseReplay(Midnight,Owner,n1)`, `AdvanceHead(h0,h1)` | `Debit(Owner,3000)`, `Credit(Recipient,3000)`, `SetObligation(Loan,8000,0,8000,Outstanding)`, `UseAllowance(Owner,3000)`, `UseReplay(Midnight,Owner,r1)`, `AdvanceHead(h0,h1)` |
| Candidate post | Balances8990/1200/60; allowance3990/spent1110; work9/spent3; replay consumed; h1; no duty | Balances7000/3200; principal8000/accrued0/outstanding8000/Outstanding; allowance2000/spent3100; work9/spent3; replay consumed; h1; no S0 duty |
| Existing local result | `PreparedUnqualified`, not committed | `PreparedUnqualified`, residual debt remains in Loan, not committed |
| Hostile completion | Omit Fee credit → Effect `S0_EFFECT_MISMATCH`; also stale h0 does not move History before Effect | Substitute creditor in a well-formed submitted vector → Effect `S0_EFFECT_MISMATCH`; changed signed/submitted payer → Intent `S0_INTENT_SCOPE` |
| Rejection publication | First judgment/code, diagnosticWork1, publishedPost/publishedEffects null, no published successor | Same; no debt-only reduction or partial creditor payment |
| Qualified commitment, **open** | A verifier must establish all four premises and all bindings, then atomically publish the entire vector | Same; absent/failed evidence never promotes local preparation to settlement |

All S0 stages use `stage → intent → effect → authority → history → failure`.
Source formation rejects before these judgments. The local result retains four
required premises: `canonical-intent-signature`, `snapshot-to-head`,
`head-extension`, `atomic-ledger-compare-and-consume`. It also retains four
unverified wrapper bindings: `agreement-id`, `selected-program`, `asset-scale`,
`authenticated-predecessor`. There is no canonical `/3` signing codec implied by
this source spelling.

## Field-by-field Source/6 mapping

| Beta source or scenario origin | Existing Source/6 spelling | Core meaning / restriction |
| --- | --- | --- |
| Profile/agreement name | Header `/6`, `agreement Invoice` or `Repayment` | Proposed beta header changes presentation; agreement binding stays unverified. |
| `Preview.id`, `USD.id`, `USD.scale` | `domain Midnight; settlement A scale 2;` | Emit economic IDs, never declaration names/ticker; representation/network are unverified metadata. |
| Transfer operation | `selected TransferLiteralFee source_hash "src1" digest "policy1"` | `programId` is selected action ID, not agreement ID. |
| Repay operation | `selected RepayAccrualFirst source_hash "src1" digest "policy1"` | Fixed selection; no solver-selected accounting program. |
| `signer`, `key`, `nonce` | `signer Owner key "key1"; nonce "n1";` or `"r1"` | Replay key includes domain+signer+nonce. |
| `pre_head`, `valid` | `pre_head "h0"; valid 100..200;` | Exact source-fixed head/window. |
| `gross_cap`, `fee_cap`, `net_floor` | Transfer1010/10/1000; repay3000/0/0 | Explicit S127 narrowing after pure expression evaluation. |
| Transfer arguments | Both `signed_action transfer from Owner to Recipient fee_to Fee value 1000 fee 10;` and identical `submit transfer ...;` | Pairwise distinct endpoints, owner=signer; no endpoint defaults. |
| Repay arguments | Both `signed_action repay obligation Loan payer Owner amount 3000 conversion identity;` and identical `submit repay ...;` | Creditor absent from request; authenticated obligation binds it. |
| `source_hash`, `policy_digest` | Selected `source_hash`, selected `digest` | Policy digest is not the signed-intent digest. |
| `failure: SuccessOnly` | `failure success_only;` | Terminal success only; no accepted retained-cost failure. |
| `observations`, `disclosures` | `observations empty; disclosures empty;` | No authentication/observation escape. |
| `retained_effects`, `retained_duties` | `retained_effects empty; retained_duties empty;` | No persistent S0 episode/duty state. |
| `delegation`, `recovery` | `delegation none; recovery none;` | No delegated signer or recovery branch. |
| Scenario head/predecessor/round | `authenticated { head "h0"; predecessor "genesis"; round 150; ... }` | The word `authenticated` names a local claim until external premise checks. |
| Scenario balances | Three ordered transfer rows; two ordered repay rows | Every cell required, including Fee when fee0; never create missing receiver0. |
| Scenario allowance/work | Owner remaining5000/spent100; work_remaining10/work_spent2 | Both spent counters explicit; never reset. |
| Scenario Loan | debtorOwner/creditorRecipient/assetA/principal10000/accrued1000/outstanding11000/status outstanding | All seven accounting/identity fields plus ID supplied; sum and bound enforced. |
| Scenario replay | `replay unused;` | Closed unused/consumed transport only. |
| Derived candidate effects | Complete ordered `effects { ... }` above | Proposal only; Core independently derives/compares all economic/admin lines. |
| Scenario post_head | `post_head "h1";` plus `advance_head "h0" "h1";` | Unverified successor; no fixture-created ledger commit. |

Source byte spans and scenario JSON pointers must explain every generated field.
Generated profile selection is a lowering rule, not fabricated source evidence.
The exact generated vector is compared only on representable Source/6 inputs;
formation/Core separation is part of the observation, not a silent coercion.

## Horizon registry and construct status

The beta's specified closed registry contains only the names and fixed argument
schemas recorded in [CONVERGENCE](CONVERGENCE.md). Non-S0 recognition must report
`SpecifiedOnly`; attempted expansion/simulation must reject
`BETA_PROFILE_UNSUPPORTED` without effects. Unknown operation/argument names
reject formation. Typed horizon transition declarations are a different proposed
grammar and are not inserted into beta records as opaque strings.

| Visible construct | Proposed beta support | Currently evidenced / intended boundary |
| --- | --- | --- |
| profile/agreement/action; domain/account/asset/obligation | specified nominal declarations | Source/6 has a distinct local closed presentation; [source contract](../../experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md). |
| Qty literals/atoms/+/-/*/min/max/rounds | specified exact pure authoring | Source/6 locally consumes explicit atom integers/windows; full elaboration equivalence open. |
| intent/transfer/repay; snapshot/scenario | specified lowering and local preparation | Existing [Core/5](../../experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts) derives complete effects; frontend command evidence needed. |
| Empty/None/SuccessOnly | specified explicit wire scope | Existing S0 locally rejects nonempty scope; [implementation contract](../../experiments/moriarty-language/formal/mil4/s0-implementation-contract.md). |
| Named non-S0 registry calls | specified name/reference/quantity recognition | Financial relations, evidence and execution open; [typed horizon](FULL-LANGUAGE-HORIZON.md). |
| Struct/ADT/function/library/generic/match; Delta/Position/Price/Clock | not beta grammar; specified horizon interface | Typed total elaboration, dimensional/numeric correspondence open. |
| SignedIntent/Grant/Hole/Completion/Cell/pre/post/Footprint | not beta grammar; specified horizon interface | Signed refinement, framing and external qualification open. |
| Observation/Evidence/KernelReceipt/Outcome/Duty/stage/episode | beta record names where registered; typed contract grammar specified separately | All authentication, persisted continuation and atomic consumption remain open; [kernel recommendation](../kernel-api-recommendation-2026-09-29.md). |

## Provenance and next profile

The initial supplementary mockup reviewed by audit04 had SHA-256
`5ef082e9369506bcd4a7a5f1b1e8ec7a5b1ee8e9ba6f95c2ac233fd4736a7bfb`.

Its review remains immutable in [audit04](audits/04-language-horizon.md). The frozen
v1 beta design had SHA-256
`6af4ca5356ed3fd58818b5717401d5b218b672b65769e342f708d4a4e0cd061a`.
This repair supersedes the mockup's prose-only interfaces and premature beta
status labels; it does not revise old audit receipts or count them as approval
of these new bytes. The controlling requirements are [the original mockup
requirements](../../docs/language/PROGRAMMER-FACING-MOCKUP-REQUIREMENTS-2026-09-30.md).

The smallest next programmer-visible executable profile is the specified beta
transfer and funded repayment above. It preserves the larger horizon by exposing
its interfaces without weakening S0 or pretending to execute other lifecycles.
The full examples, unresolved choices, formal obligations and final requirements
checklist are in [FULL-LANGUAGE-HORIZON.md](FULL-LANGUAGE-HORIZON.md).


The [v2 follow-up reviews](audits/04-horizon-v2.md) and
[independent correspondence review](audits/05-horizon-v2.md) reviewed this page at
`89ab5354e9ff0de7d0793318a8c9df93d617b71bfac13da120289402dc61dc65` and the horizon at
`7d2673703dfd271dd641b625c799dee3f536803827dacb7d66bc938cc82ec26f`.
Their remaining findings are proposed repairs in
[FULL-LANGUAGE-HORIZON §16](FULL-LANGUAGE-HORIZON.md#16-focused-v2-review-repair-and-independent-arithmetic-discriminators):
explicit proportional slash allocation, signed branch/milestone receipt scopes,
per-stage continuation heads, exact kernel scopes, conjunctive signatures and
expiry/renewal, pledged-custody effects, and a bounded pure function example.
The positive receipt floors and cumulative caps remain explicit; the S0 trace
above retains its original single-stage meaning. Checked coverage boxes in the
horizon mean visible proposed notation, with fresh exact-byte review pending.

The [v3 interface review](audits/04-horizon-v3.md) and
[v3 correspondence review](audits/05-horizon-v3.md) inspected this page at
`16d419cb4e8f26571d9aab79406e5e8864e7db623028b98929274c727e5d601e` and the horizon at
`f5c947822879f55d25a06efc48eec77eb82149c9cadb262b31e6274f683db488`.
Their three remaining findings have focused v4 proposed dispositions in
[FULL-LANGUAGE-HORIZON §16](FULL-LANGUAGE-HORIZON.md#v4-focused-dispositions-of-the-v3-reviews):
signed observational reconciliation with no economic head consumption; total
collection of deferred scope facts followed by the shared ordered Core judgments;
and a finite, authenticated NoActiveWithdrawals financial precondition for
Deposit/Reward. The new literal cases include an unknown Claim receipt, expired
grant combined with invalid cap/effect, and the100.10→101.00 pending-liability
mismatch. These are proposed typed interfaces and independently checked literal
arithmetic, with fresh exact-byte review pending. This page's existing Source/6
trace and field mapping retain their original local qualification limits.
