You are an independent Grok 4.7 read-only Moriarty reviewer. Worktree: /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929. Repository status: SP01.6 blocked by stale binding/candidate, missing accounting and unavailable live state. No dispatch. This is a pre-collected read-only packet. Do not edit files, run tests, dispatch campaigns, use web, or spawn agents. The full candidate text follows below: experiments/moriarty-language/formal/mil4/{semantics-contract.md,coverage.tsv,decisions.md,projection.md}, docs/superpowers/plans/2026-09-29-mil4-k-quint-completion.md, deliverables/mil4-successor-2026-09-29/DESIGN-MIL4-WORKING.md, and the K surge result. Candidate SHA-256: semantics 3416040e4c6746f39dddfa3fb92eb89c181364d0fba98f7ca2186d0f70321201; coverage 04a9123a8190c69f9808c4a9b49b2b427dd4592fa054cba5a88bf3adbf3f2c69; decisions 806cbe7809a9c0353f3ad19e4c789fd2acdc7d572a43defece7715c5d4047030; projection 9516510e2d245ed570a367a043fd3f469279284c28a000cf5f7c4c2ed98147a2. Focus: Review K→Quint abstraction for unsound erasure, missing state/actors/failures and S0 action grain. Give a concrete counterexample if possible. Check that proposed Quint work cannot be mistaken for admission or proof. Give a substantive vote or abstention on abstraction adequacy only. Report actual model identity if available and cite paths/lines. Return a concise audit.


=== BEGIN SOURCE PACKET ===

--- FILE: experiments/moriarty-language/formal/mil4/semantics-contract.md ---
# MIL/4 semantic contract candidate

**Status:** Sprint 0 working candidate, 2026-09-29. No language adoption, K proof, Quint simulation, native proof or ledger acceptance follows from this document. The [completion plan](../../../../docs/superpowers/plans/2026-09-29-mil4-k-quint-completion.md) defines the intended exit.

## Version boundary

Propose **Source/6 → Core/5** for the MIL/4 first-profile language. Keep Source/5 → Core/4 byte and API behavior unchanged. The Source/5 contract explicitly closes its Core/4 constructor set, and its compiler invokes the Core/4 evaluator. Generic `Record` or `Emit` values cannot stand for a new authenticated financial transition. A legacy record enters MIL/4 only through a field-by-field lossless embedding. Otherwise return a named version rejection. Do not infer a `/4` signed encoding from the Source/6 or Core/5 labels.

S0 still has proposed `/3` intent bytes. W-D1 and W-D2 must select the signature scheme, verifier locus, digest and canonical encoding before any `/4` migration is stated. Unknown tags reject. Migration must bind the source record, selected Core program, signed digest and all effect obligations to one new record. No compatibility proof is inherited from Source/5 or Core/4.

## State and constructor classes

Core/5 must represent these classes as typed constructors or typed records with closed tags and total validation:

| Class | Required content |
| --- | --- |
| Header and selection | Version/profile, executing domain, selected program and policy digest, current authenticated head, predecessor, validity, stage and episode IDs. |
| Typed values | Nominal asset/domain IDs, checked atomic quantities, deltas, oriented prices, shares, positions, instants and durations. |
| Intent and completion | One signed template, fixed fields, typed holes, permitted source and finite cell range, narrowing fill, selected branch, gross/fee/net bounds, recipients, surplus and residue. |
| Formula | Closed Φ₀ guard and outcome terms with distinct pre/post access; general Φ₁ and uncertified Ω reject. A closed certified Ω needs a versioned numeric and cost contract. |
| Authenticated state | Balance, supply, obligation, encumbrance, lock, pool, managed assets, shares, observation, grant, policy, receipt, claim, replay, episode head and tombstone cells, plus profile-specific aggregates. |
| Effects and duties | Complete ordered typed debit/credit, mint/burn, lock/unlock, debt, claim/receipt, budget and authority consumption, work and retained duty lines. Product remainders are metadata until a whole-unit ledger effect is derived. |
| Evidence and authority | Evidence policy, selected observation, provenance premise, signer/key/digest/verifier locus, grant scope/epoch/budget and replay identity. |
| Transition and result | Prepared stage, pending/unknown/recovery/terminal episode phase, accepted success or signed accepted failure, complete observation, or atomic rejection with judgment and code. |

The executing program derives the full read/write footprint after holes resolve. It reads each current cell from one authenticated head. It computes every post-cell and effect line. A supplied effect hash alone does not authenticate omitted endpoints, aggregates, recipients, reserves or duties.

## Stage relation and observation

Use one selected program and one signed intent per initial stage. Establish the six judgments in this order, pending W-D3's final rejection precedence: `stage`, `intent`, `effect`, `authority`, `history`, `failure`. `stage` checks selection and typed authenticated pre-state. `intent` checks exact signed scope and valid filling. `effect` checks the complete canonical prepared vector, conservation and post-state. `authority` checks signer, grant and budget. `history` checks current head, predecessor, replay and successor. `failure` checks the signed terminal branch and retained duties.

An accepted result contains the version/profile, signed scope digest, selected program, authenticated pre-head, prepared effect vector, all derived reads/writes, post-head and post-state, phase, remaining duty and work, consumed authority and replay, and judgment result. A local rejection contains a judgment and stable code, with no published post-state or effects. A ledger phase failure can retain only effects and duties named in the signed failure branch. Native qualification, signature verification and ledger atomicity remain explicit external premises until their interfaces are pinned.

## S0 candidate domain

W-D0 proposes one signer, one executing domain, one settlement asset, a literal-fee transfer, and funded AccrualFirst repayment of an existing obligation. Transfer debits `v+f`, credits `v` to the fixed recipient and `f` to the fixed fee recipient. Repay debits and credits `n`; it reduces accrued interest by `min(n,a)` and principal by the remainder. Both consume allowance and replay once and advance the authenticated head. S0 has no division, rounding, reserve posting, mint, foreign evidence or accepted fee-bearing failure.

Use checked UInt128 atomic balances and counters. Bound current Core/4 obligation values to `2^127−1` until a new consumer demonstrates a wider schema. The exact source/Core/K comparison domain and near-bound cases remain W-D4 obligations. S0 cannot be called admitted until W-D0–W-D4 have selected bytes, signature binding, ordered rejection, exact effect lines and widths.

## First-profile and deferred boundary

The eight first profiles are `amm-cp/1`, `loan-fixed/1`, `cdp/1`, `opt-capped/1`, `obs/1`, `gov/1`, `bridge-pair/1` and `vault/1`. Each needs its own typed cells, authenticated transition, complete effects, positive trace and otherwise-well-formed hostile trace. The current [K surge](../../../../deliverables/mil4-k-surge-2026-09-29/RESULT.md) supplies arithmetic and effect sketches only. Its `m4AdmitFamily` rejects every first family with `FAMILY_STAGE_ADAPTER_ABSENT`. Preserve that fail-closed boundary until the corresponding full stage adapter exists.

Defer LP mint/burn, variable credit, unbacked/rebasing issuance, uncapped options and general derivatives, median/TWAP, general voting and multisigner admission, bonded fast fill and foreign verifier assurance, complex vault withdrawals/slashing, general Φ₁ and uncertified Ω. Give each an explicit versioned rejection. A broad family label never admits its later profile.

## Historical K disposition

Archive old bounded `moriarty.k` behavior, original fixtures, toolchain locks and SP03 receipts as historical domain evidence. Adapt the expression and lifecycle modules, codecs, runner and surge static/formula/escrow rules only after a documented Source/6/Core/5 mapping. Replace their composition roots and version-specific state signatures. Treat family arithmetic/effect projections as sketches. The old runner's proof entry returns `PROOF_UNIMPLEMENTED`; no old theorem is inherited. Record each file-level disposition in the constructor register before Sprint 1.

## Open decision gate

The [decision register](decisions.md) owns W-D0–W-D6 and M4-C1–M4-C5. The [constructor inventory](coverage.tsv) is provisional, and the [projection](projection.md) is a design map. Sprint 0 cannot exit until every source production and Core constructor has a version/profile and a rule or named rejection, S0 leaves have exact dispositions, and two substantive independent votes agree on the frozen candidate. A changed signature or state transition opens new proof obligations.


--- FILE: experiments/moriarty-language/formal/mil4/coverage.tsv ---
id	origin	version	first_profile	static_rule	k_rule_or_reject	quint_relevance	fixture	proof_status	old_disposition	status
SRC5:program	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:profileDecl	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:agreementDecl	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:declaration	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:unitDecl	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:partyDecl	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:assetDecl	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:recordDecl	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:recordField	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:operationDecl	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:uninitializedStateDecl	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:actionDecl	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:parameters	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:parameter	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:statement	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:requirement	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:binding	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:update	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:emission	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:effectFields	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:effectField	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:postcondition	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:type	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:typeArgs	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:typeArgument	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:signedInteger	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:expression	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:conditional	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:disjunction	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:conjunction	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:negation	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:comparison	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:comparisonOp	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:sum	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:product	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:postfix	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:ordinaryPrimaryName	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:primary	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:genericCall	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:dynamicOrLiteral	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:financialGeneric	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:financialRead	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:financialPostRead	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:recordLiteral	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:arguments	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:identifier	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:integerToken	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
SRC5:stringToken	financial-agreement-source-v5-grammar.ebnf	Source/5	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ConstructShares	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ConstructAmount	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ConstructVariant	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ProjectVariant	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ConvertUInt	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:Select	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:LitUInt	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:LitBool	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:LitAmount	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:LitQuantity	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:LitShares	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:LitRate	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:LitPrice	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ReadLocal	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ReadArg	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ProjectField	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ProjectIndex	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ConstructRecord	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ConstructEnum	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ConstructSome	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ConstructCollection	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:Add	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:Eq	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:Not	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:Require	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:NextWrite	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:Emit	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ReadOutstanding	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ReadPrincipal	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ReadAccrued	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ReadBalance	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ReadAllowanceRemaining	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ReadAllowanceSpent	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ReadPostOutstanding	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ReadPostPrincipal	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ReadPostAccrued	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ReadPostBalance	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ReadPostAllowanceRemaining	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ReadPostAllowanceSpent	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ProjectSome	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ScalarValue	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:LitSInt	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:LitText	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ReadPre	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ReadObs	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:AccessField	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:AccessIndex	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:ConstructNone	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:Sub	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:Mul	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:FloorDiv	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:CeilDiv	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:Lt	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:Lte	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:Gt	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:Gte	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:And	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:Or	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:Let	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
CORE4:Ensure	financial-expression-v1.ts	Core/4	legacy	unmapped	unmapped	unknown	none	open	map-or-reject	inventory
MIL4:Header	semantics-contract.md	Source/6→Core/5 candidate	shared	proposed	missing	to-classify	none	open	new	proposed
MIL4:SelectedProgram	semantics-contract.md	Source/6→Core/5 candidate	shared	proposed	missing	to-classify	none	open	new	proposed
MIL4:SignedIntent	semantics-contract.md	Source/6→Core/5 candidate	shared	proposed	missing	to-classify	none	open	new	proposed
MIL4:Hole	semantics-contract.md	Source/6→Core/5 candidate	shared	proposed	missing	to-classify	none	open	new	proposed
MIL4:Fill	semantics-contract.md	Source/6→Core/5 candidate	shared	proposed	missing	to-classify	none	open	new	proposed
MIL4:PhiGuard	semantics-contract.md	Source/6→Core/5 candidate	shared	proposed	missing	to-classify	none	open	new	proposed
MIL4:PhiOutcome	semantics-contract.md	Source/6→Core/5 candidate	shared	proposed	missing	to-classify	none	open	new	proposed
MIL4:CertifiedOmega	semantics-contract.md	Source/6→Core/5 candidate	shared	proposed	missing	to-classify	none	open	new	proposed
MIL4:AuthenticatedCell	semantics-contract.md	Source/6→Core/5 candidate	shared	proposed	missing	to-classify	none	open	new	proposed
MIL4:Evidence	semantics-contract.md	Source/6→Core/5 candidate	shared	proposed	missing	to-classify	none	open	new	proposed
MIL4:Grant	semantics-contract.md	Source/6→Core/5 candidate	shared	proposed	missing	to-classify	none	open	new	proposed
MIL4:Budget	semantics-contract.md	Source/6→Core/5 candidate	shared	proposed	missing	to-classify	none	open	new	proposed
MIL4:CompleteEffect	semantics-contract.md	Source/6→Core/5 candidate	shared	proposed	missing	to-classify	none	open	new	proposed
MIL4:RetainedDuty	semantics-contract.md	Source/6→Core/5 candidate	shared	proposed	missing	to-classify	none	open	new	proposed
MIL4:Stage	semantics-contract.md	Source/6→Core/5 candidate	shared	proposed	missing	to-classify	none	open	new	proposed
MIL4:Episode	semantics-contract.md	Source/6→Core/5 candidate	shared	proposed	missing	to-classify	none	open	new	proposed
MIL4:Observation	semantics-contract.md	Source/6→Core/5 candidate	shared	proposed	missing	to-classify	none	open	new	proposed
MIL4:AcceptedFailure	semantics-contract.md	Source/6→Core/5 candidate	shared	proposed	missing	to-classify	none	open	new	proposed
MIL4:AtomicRejection	semantics-contract.md	Source/6→Core/5 candidate	shared	proposed	missing	to-classify	none	open	new	proposed
PROFILE:amm-cp/1	DESIGN-MIL4-WORKING.md	Source/6→Core/5 candidate	amm-cp/1	proposed	FAMILY_STAGE_ADAPTER_ABSENT	yes	none	open	adapt-projection	proposed
PROFILE:loan-fixed/1	DESIGN-MIL4-WORKING.md	Source/6→Core/5 candidate	loan-fixed/1	proposed	FAMILY_STAGE_ADAPTER_ABSENT	yes	none	open	adapt-projection	proposed
PROFILE:cdp/1	DESIGN-MIL4-WORKING.md	Source/6→Core/5 candidate	cdp/1	proposed	FAMILY_STAGE_ADAPTER_ABSENT	yes	none	open	adapt-projection	proposed
PROFILE:opt-capped/1	DESIGN-MIL4-WORKING.md	Source/6→Core/5 candidate	opt-capped/1	proposed	FAMILY_STAGE_ADAPTER_ABSENT	yes	none	open	adapt-projection	proposed
PROFILE:obs/1	DESIGN-MIL4-WORKING.md	Source/6→Core/5 candidate	obs/1	proposed	FAMILY_STAGE_ADAPTER_ABSENT	yes	none	open	adapt-projection	proposed
PROFILE:gov/1	DESIGN-MIL4-WORKING.md	Source/6→Core/5 candidate	gov/1	proposed	FAMILY_STAGE_ADAPTER_ABSENT	yes	none	open	adapt-projection	proposed
PROFILE:bridge-pair/1	DESIGN-MIL4-WORKING.md	Source/6→Core/5 candidate	bridge-pair/1	proposed	FAMILY_STAGE_ADAPTER_ABSENT	yes	none	open	adapt-projection	proposed
PROFILE:vault/1	DESIGN-MIL4-WORKING.md	Source/6→Core/5 candidate	vault/1	proposed	FAMILY_STAGE_ADAPTER_ABSENT	yes	none	open	adapt-projection	proposed


--- FILE: experiments/moriarty-language/formal/mil4/decisions.md ---
# MIL/4 Sprint 0 decision register

**Status:** all rows open. Recommendations are review candidates, not adopted semantics. Each final row needs alternatives, exact candidate digest, discriminator, two substantive independent votes, dissent and final disposition.

| ID | Candidate direction | Required discriminator or evidence | State |
| --- | --- | --- | --- |
| W-D0 | S0 is one-signer, one-domain literal-fee transfer and existing-obligation AccrualFirst repayment. | Show complete source/Core/K effect and hostile corpus, or justify a narrower slice. | Open |
| W-D1 | Bind one exact digest across signature, proof public statement and ledger effect commitment. | Fix scheme, bytes, digest, verifier locus and recipient substitution rejection. | Open |
| W-D2 | Closed canonical `/3` S0 tags; define `/4` migration separately. | Source/Core round trip, unknown-tag rejection and byte-level substitution. | Open |
| W-D3 | One authenticated pre-head; six ordered judgments; complete prepared effects; atomic local rejection. | Distinguish omitted effect, stale head and valid signed failure on exact bytes. | Open |
| W-D4 | S0 checked UInt128, first obligation cap `2^127−1`, no rounding or reserve; certify later Ω separately. | Near-bound S0 corpus, AMM fee and vault remainder cases on pinned targets. | Open |
| W-D5 | Foreign evidence and nonreceipt require qualified verifier and finality premises. | Partial delivery, timeout-unknown and conflicting late receipt. | Open |
| W-D6 | Admit each first family only after complete authenticated adapter. | One positive and one rule-reaching hostile trace per family. | Open |
| M4-C1 | Propose Source/6 → Core/5; canonical `Price<Base,Quote,Scale>` and explicit `/4` boundary. | Lossless old-record mapping or named rejection, inverted price and renamed field. | Open |
| M4-C2 | Distinguish fractional product remainder from a posted whole-unit reserve effect. | Nonzero fraction and one-unit residue with complete balancing lines. | Open |
| M4-C3 | Closed certified Ω only inside selected transition; reject general Φ₁. | Feasible result, forged quotient and overflow on pinned target. | Open |
| M4-C4 | Derive complete authenticated cell footprint after hole resolution. | Omit one endpoint, aggregate, grant, reserve or duty from otherwise-valid stage. | Open |
| M4-C5 | Sign recipients, fee scope, duties, evidence and failure branch; retain unknown partial progress. | Recipient substitution, grant revoke, receipt replay and duty omission. | Open |

Current review evidence: three GPT-6 Sol read-only seats inventoried Source/Core, historical K and Quint abstraction. Their findings support a **candidate** Source/6 → Core/5 boundary and reject unchanged semantic reuse. They are not votes on frozen bytes. Grok 4.7 readiness and Opus 5.5 full-candidate audits remain separate seats. No decision is closed by this register.


--- FILE: experiments/moriarty-language/formal/mil4/projection.md ---
# K to Quint projection candidate

**Status:** design map only. No Quint model or simulation has been run. Define `α : KState → QuintState` only for an authenticated, admitted K stage. The current K surge's family projections are not admitted stages.

| K observation | Quint state or observation | Constraint |
| --- | --- | --- |
| Version, selected Core program and resolved intent | Immutable signed scope | Preserve profile, program, endpoints, bounds, failure branch and evidence choice. Filling may narrow scope only. |
| Authenticated pre/post cells and derived footprint | Typed maps keyed by domain, asset, account, obligation, episode or claim | Preserve every value read or written by the action. Do not merge nominal IDs. |
| Complete ordered effect vector | Canonical per-cell delta and consume set | Compare the whole vector before the abstract transition. Keep fee beneficiary distinct. |
| Balances, custody and supply | Per-domain, per-asset balances, episode custody and supply | Enforce conservation after alias resolution. Cross-domain backing needs a separate assumption. |
| Principal, accrued, claim and retained duty | Obligation components, entitlement and duty | A partial payment changes debt only with matched creditor credit. A duty survives partial progress. |
| Grant, allowance, budget and work | Remaining counters, gross use and authority epoch | Refund does not restore consumed gross authority. |
| Predecessor/head and replay/nullifier | Opaque head link and consumed ID sets | K must authenticate underlying commitments. The ledger must compare and consume atomically. |
| Phase and failure result | Pending, unknown, recovery or terminal status and no-commit rejection observation | A rejected stage has no Quint successor and no published effects. |

One Quint action represents one atomic accepted K stage. Competing proposals can interleave before admission, but only one wins a current head. Time is a finite round or epoch. Timeout does not prove foreign nonreceipt. Signature validity, native proof truth, oracle provenance and foreign finality enter only as named qualified premises. Quint cannot establish their cryptography.

Start `s0.qnt` with transfer and repay over balances, allowance, obligation principal/accrued, head and replay. The model must guard UInt128 limits and the first obligation cap because Quint integers do not overflow. Later models add only state needed for escrow episodes, pool and vault reserves, issuance and claims, selected observations, governance epochs, and bridge entitlement. Each action needs a reachable witness and a safety invariant. Run `quint typecheck` and sampled `quint run` when implementation begins; sampled traces are not a refinement proof. Do not run `quint verify` without an explicit model-checking request.

For each compared accepted transition, require `α(preK) = preQ`, `α(postK) = postQ`, and exact effect-vector agreement under the declared mapping. For K rejection, compare only the no-commit observation. The projection excludes lexical errors that have no Quint action. Record any omitted K field and the reason its omission preserves the checked property.


--- FILE: deliverables/mil4-successor-2026-09-29/DESIGN-MIL4-WORKING.md ---
# Moriarty Intent Language (MIL/4) — successor working design

**Date:** 2026-09-29. **Status:** working successor proposal. This document starts from the [MIL/3 specified-only draft](../mil3-design-draft-2026-09-29/DESIGN-MIL3-DRAFT.md) and uses its [decision and implementation workflow](../mil3-design-draft-2026-09-29/FINALIZATION-AND-IMPLEMENTATION-WORKFLOW.md). It records a proposed semantic delta; it is not an adopted language profile, a U0 exit, a proof result or a Midnight ledger result. Earlier MIL/2 and MIL/3 texts and review receipts remain intact.

## 1. Role of MIL/4

MIL/4 is a successor contract for the eight DeFi families previously reviewed by separate category agents. It keeps MIL/3's six U0 judgments (`stage`, `intent`, `effect`, `authority`, `history`, `failure`), selected-program rule, exact prepared effects, typed resources and persistent duties as the starting proposal. It names missing state, arithmetic, authority and admission rules as live issues with discriminators; none is disposed by this working text. Every family has a first profile and a separate deferred boundary. A specified profile is not automatically an admitted, implemented or native accepted profile.

The first implementation slice remains the [U0 S0 proposal](../u0-study-2026-09-28/UNIFIED-PROPOSAL.md): literal-fee transfer and funded AccrualFirst repayment of an existing obligation. Originate and accrue in `loan-fixed/1` are outside S0. The MIL/3 workflow's W-D0–W-D3 and S0 portion of W-D4, design gate M3-D, reference gate M3-R and native gate M3-N remain open. MIL/4 cannot inherit a passing result from those gates by changing the version number. Broader DeFi profiles enter only after their own source/Core/K/native/ledger evidence under U1–U6.

## 2. Shared successor decisions

These are live MIL/4 decisions, not implicit changes to the recorded U0 policy. Each issue record will retain its alternatives, affected clauses, evidence, two reviewer verdicts, dissent and final disposition. None is disposed by this working text. M4-C1–C5 supplement the MIL/3 workflow's W-D0–W-D6 register. W-D0–W-D3 and the S0 part of W-D4 gate S0. The Ω, AMM-fee and vault remainder of W-D4, the cross-domain part of W-D5, and W-D6 gate their applicable family profiles. A category cannot inherit a favorable review of another row.

| ID and inherited issue | Proposed direction and live alternative | Discriminator before selection |
| --- | --- | --- |
| M4-C1 / W-D2 | **Canonical names and bytes.** Reuse U0's base-per-quote orientation. Write `Price<Base,Quote,Scale>` everywhere, name both nominal assets and domains, and reject implicit reciprocals. Alternative: keep MIL/3's shorter notation but prove its `b,q` positions map to base and quote without ambiguity. S0 review-candidate bytes remain `/3`; `/3`→`/4` encoding, unknown-tag rejection, migration and source/Core round trips are open MIL/4 decisions, not a new S0 default. | Invert a typed price, rename a constructor or add an unknown tag under both versions; require one canonical digest or a deterministic reject/migration result. |
| M4-C2 / W-D4 | **Rounding and dust.** Retain the [U0 numeric decision](../../docs/decisions/u0-numeric-profile-decision.md) as controlling policy. The beneficiary remains the protocol reserve. A fractional quotient remainder is not a ledger Qty. Open: per-primitive direction and effect-line representation, including the reserve cell, whole-unit identification and matching conservation. Alternative: a new versioned owner amendment if an intended beneficiary differs. | Compute a nonzero fractional remainder and a separate one-unit allocation residue, then name every integer effect and reserve change. A hash-bound beneficiary change needs a new decision. |
| M4-C3 / W-D4 | **Closed financial arithmetic.** Propose a closed certified Ω in the selected program while general nonlinear intent formulas remain deferred. Alternative: defer all Ω to U4 as MIL/2 proposed. Input/intermediate widths, total rejection, exact output and target cost are open. | One feasible result and one otherwise valid forged quotient or overflow must reach the arithmetic rule on the pinned target. Host computation alone is insufficient. |
| M4-C4 / W-D3 | **State and footprints.** Register every aggregate, reserve, custody, supply, claim, receipt and policy cell; derive complete footprints after holes resolve and authenticate the required heads. Alternative: a smaller first profile with fixed-empty cells and explicit rejection. | Delete one derived endpoint, aggregate or policy read from an otherwise valid stage; reject before acceptance. |
| M4-C5 / W-D1, W-D5 | **Authority, evidence and partial progress.** The signature contract must name scheme, bytes, digest, verifier locus and public inputs before it can bind recipients, fee scope, duties, evidence and failure branches. Imported evidence names a verifier premise. Alternative: defer a foreign-dependent profile until a qualified verifier exists. | With a valid envelope, substitute one signed recipient byte, revoke a grant, replay a receipt or omit a retained duty. Name the rejecting judgment and accepted-state result. |

For every family profile `p`, instantiate the inherited MIL/3 stage relation as six explicit obligations:

```
stage_p    : selected program, authenticated heads, typed pre-state, prepared transition
intent_p   : exact signed bytes, filling, recipients, bounds, failure and disclosure
effect_p   : complete canonical lines, derived footprint, conservation and post-state
authority_p: current grant, budget, issuer or verifier right, and replay consumption
history_p  : authenticated predecessor, head extension and retained claim identity
failure_p  : named rejection with no accepted writes, or signed retained phase effects/duties
```

These lines are an obligation map, not six proved judgments. A profile must give the exact cells, effects, rejection precedence and positive/hostile traces for each line before its design gate. Changing only a predecessor commitment or an unpaid duty in an otherwise feasible stage must select `history_p` or `failure_p`, respectively, rather than pass through `effect_p` alone.

The [September 23 U0 exit](../u0-semantic-contract-2026-09-23/EXIT-GATE.md) retains its recorded absent embeddings, uncovered K rows, unenforced native leaves and target-pin gap. MIL/4 decisions do not re-score it. A later result record must distinguish source, Core, K, native proof and Preview ledger effects.

## 3. Eight DeFi profile comparisons

The [eight MIL/2 category reports](../mil2-deep-research-2026-09-29/CATEGORY-COVERAGE.md) and their Opus recommendation syntheses are research inputs. The MIL/3 §8 schemas are the specified-only starting point. The following are proposed MIL/4 contract additions and evidence cases; they are not coverage verdicts.

| Family and proposed first profile | MIL/3 already specifies | MIL/4 decision to close | First positive / well-formed hostile case | Later boundary |
| --- | --- | --- | --- | --- |
| **AMM and exchanges** `amm-cp/1` | One-hop exact-input constant-product quote, fresh pool head, four balance effects, signed fee scope and surplus. | Bind authenticated reserves to custody and pool head, fee units and retained value, exact quotient/tightness, widths, pool authority and full footprint. Select exact rational fee within the quote or upfront rounded fee as a separate W-D4 decision. | Funded 11 A for exact 23 B at 1000/2200 reserves and `f=3,F=1000`; reject otherwise valid 24 B output despite the trader's 20 B floor. A separate otherwise identical 23 B case distinguishes fee policies because upfront one-unit fee permits only 21 B. Equivalent `30/10000` is a separate encoding/width comparison, not a changed rate. | Exact output, LP issue/burn, routes, partial fills, maker orders and multisigner clearing. |
| **Lending and borrowing** S0 repay, then `loan-fixed/1` and `loan-coll/1` | Funded AccrualFirst repayment of an existing obligation is S0; MIL/3 also proposes originate/accrue and collateral liquidation. | Keep S0 restricted to repay. For later `loan-fixed/1`, fix period/rate/rounding, obligation status, funded originate/accrue and one-shot consent. Separately fix aggregate lock transitions, selected current price, close factor, bounded seizure, surviving shortfall and loss waterfall for `loan-coll/1`. | S0 repays 30 from principal 1000/accrued 10 to principal 980/accrued 0 with 30 paid; reject the same debt reduction without the creditor payment. | Variable rates, pooled shares, flash liquidity and portfolio liquidation. |
| **Stablecoins and synthetics** `cdp/1` | Positive mint with issue grant, matching debt/lock, global ceiling, exact supply and balance, typed price policy. | Register authenticated aggregate ceiling cell; define canonical mint/burn effects, grant quota and per-line authority, atomic debt/lock/supply correspondence and bounded health comparison. | Lock 150 collateral at 3:2 requirement and mint 100 units under current cap; reject positive supply without matching debt or lock. | Redemption duty and settlement burn, reserve-backed issuance, shutdown, rebasing, shared debt and cross-domain supply. |
| **Derivatives** `opt-capped/1` | Finite funded call reserve, capped payoff, selected fixing, one-shot exercise and retained unpaid duty. | Fix instrument/claim IDs, premium and writer funding, exact selected round, integer payoff/rounding, explicit versus automatic exercise and state transitions through payment/reclaim. | Proposed integer case: size 1,500,000 micro-underlying, strike 3,000,000,000 and cap 3,500,000,000 micro-USDC per whole underlying, selected fixing 3,123,456,789, funded reserve 750,000,000 micro-USDC. Floor payout is 185,185,183 micro-USDC, leaving 564,814,817 reserved before a permitted reclaim; reject a one-micro-USDC underpayment with otherwise conserving effects. Fixing policy and premium still require exact terms. | Perpetual funding, margin, liquidation and social loss. |
| **Oracles and observations** `obs/1` | Typed observation admission, selected current round, policy and verified provenance label. | Bind canonical observation tuple and exact bytes, verification locus, key epoch, authenticated time/status and branch-independent admission. | Admit selected current typed round; reject a still-fresh but nonselected older round with a valid financial envelope. | Bounded median/TWAP and imported verifier profiles. |
| **Governance** `gov/1` | Queue/execute/cancel, grant epoch and policy head, protected existing duties. | Fix distinct approval set and consumption, exact queue/action digest, execution-time grant/head check, pause mask and policy migration of duties. | Apply authorized amendment under current head; reject execution after grant revocation with unchanged signed action bytes. | Voting snapshot and general multisigner stage admission. |
| **Bridges and cross-domain settlement** `bridge-pair/1` | Unique source claim; `destReceive` reduces remaining entitlement, but `destTimeout` is defined only when no receipt exists. Refund requires a named nonreceipt premise. | Fix claim/message/nullifier bytes, verifier/finality and epoch premise, installment count and remaining entitlement, exact conversion, authenticated destination terminal state and refund after partial delivery. If no qualified foreign verifier exists, this profile remains specified-only. | Propose source lock of 100 and verified one-shot destination delivery of 99 under a fee bound of 1; reject source refund after time elapses while destination nonreceipt is unknown. | Bonded fast fill, additional representation ratios and reorg-loss policy. |
| **Staking, restaking and yield** `vault/1` | One-asset/one-class conversion, virtual offset, custody surplus, authorized recognition and later withdrawal/slash lifecycle. | Fix share issue authority, exact floor/ceil widths, donation and recognition beneficiary, virtual value ownership, managed/custody/surplus accounting and complete effects. Treat withdrawal, rewards and slash as later profiles. | With managed=999, supply=1998, virtual offsets=1 and deposit=500, accept 999 shares with remainder 500; reject claimed 998 shares/remainder 1500 because remainder must be below divisor 1000. | Validator/AVS integration, reward indexing, withdrawal and slash lifecycles, strategy mandates, principal/yield splits. |

Each first profile requires a complete constructor and cell register, a positive trace and a hostile trace that reaches the intended semantic rule. A broad family name does not admit the later boundary. Imported primary sources are comparative anchors, not evidence that a Moriarty rule works.

### Numeric and effect conditions on the proposed traces

- **AMM.** The 23 B result uses an exact rational 3/1000 fee inside the pool quote. The charged fee is 0.033 A as a rational calculation, not an atomic 0.033 A ledger transfer. An upfront ceil fee of 1 A gives the alternative 21 B quote. The issue record must name the fee-cap unit, beneficiary and whether the fee stays in pool custody or causes an additional protocol-transfer effect. Four balance effects suffice only for the retained-in-pool form.
- **Loan.** The 30-unit repayment has no fee, debits the debtor 30, credits the creditor 30 and changes debt from principal 1000/accrued 10 to principal 980/accrued 0. An impairment entry cannot substitute for the funded discharge.
- **CDP.** The 150-for-100 equality case assumes one collateral asset, one synthetic asset, compatible declared decimals, `Price<Synthetic,Collateral,S>=1` at declared scale `S`, a 3/2 requirement and a `≥` health rule. Those assumptions, their exact scaled comparison, grant use, price policy, global debt head and any haircut/fee must be signed and authenticated before the case can count as positive.
- **Option.** With six-decimal underlying size, `1,500,000·(3,123,456,789−3,000,000,000) = 185,185,183·1,000,000 + 500,000`. Thus the floor payout is 185,185,183 micro-USDC. The cap spread requires a funded 750,000,000 micro-USDC reserve. The stage still needs a declared premium, selected fixing round, exercise rule, exact payout and residual reserve/reclaim effects. A one-micro-unit underpayment isolates exact payoff while preserving gross conservation.
- **Bridge.** A 100-unit source lock and 99-unit destination credit do not by themselves explain the remaining one unit. The first profile must name representation ratio/decimals, whether it is a charged fee or still-undelivered entitlement, its beneficiary, cumulative delivery and the maximum later refund. A partial 99-unit receipt needs a state distinct from both no receipt and completed delivery; destination terminal nonreceipt must refer to the remaining entitlement.
- **Vault.** In the deposit case, `500·1999 = 999·1000 + 500`; the remainder 500 is in product units, not an independent 500-unit asset or share payment. Post managed assets are 1499 and share supply 2997. Custody, surplus, virtual value ownership and the beneficiary of retained rounding must be stated. A claimed quotient 998 with remainder 1500 satisfies the equation but violates `0≤r<1000`.

The [U0 study's remainder classes](../u0-study-2026-09-28/UNIFIED-PROPOSAL.md) distinguish a charged liability increment, sub-unit residual and whole atomic-unit protocol reserve. MIL/4 must reconcile them with the owner's [U0 numeric decision](../../docs/decisions/u0-numeric-profile-decision.md) for every primitive. A protocol-reserve line is possible only for an identified whole unit with matching conservation effects. Any different beneficiary is an explicit versioned policy override; it is not silently inferred from floor or ceil.

The category-specific agent assignments used these retained baseline reports and five-review recommendation sets. Each current assignment compared MIL/3 with its category; the recommendations are research input, not the two independent full-candidate design votes:

| Family | Baseline report | Recommendation synthesis |
| --- | --- | --- |
| AMM and exchanges | [MIL/2 category 1](../mil2-deep-research-2026-09-29/category-review/01-amm-exchange.md) | [AMM](../mil2-deep-research-2026-09-29/opus55-amm-recommendations/SYNTHESIS.md) |
| Lending and borrowing | [MIL/2 category 2](../mil2-deep-research-2026-09-29/category-review/02-lending.md) | [Lending](../mil2-deep-research-2026-09-29/opus55-lending-recommendations/SYNTHESIS.md) |
| Stablecoins and synthetics | [MIL/2 category 3](../mil2-deep-research-2026-09-29/category-review/03-stablecoins.md) | [Stablecoins](../mil2-deep-research-2026-09-29/opus55-stablecoins-recommendations/SYNTHESIS.md) |
| Derivatives | [MIL/2 category 4](../mil2-deep-research-2026-09-29/category-review/04-derivatives.md) | [Derivatives](../mil2-deep-research-2026-09-29/opus55-derivatives-recommendations/SYNTHESIS.md) |
| Oracles and observations | [MIL/2 category 5](../mil2-deep-research-2026-09-29/category-review/05-oracles.md) | [Oracles](../mil2-deep-research-2026-09-29/opus55-oracles-recommendations/SYNTHESIS.md) |
| Governance | [MIL/2 category 6](../mil2-deep-research-2026-09-29/category-review/06-governance.md) | [Governance](../mil2-deep-research-2026-09-29/opus55-governance-recommendations/SYNTHESIS.md) |
| Bridges and cross-domain settlement | [MIL/2 category 7](../mil2-deep-research-2026-09-29/category-review/07-bridges.md) | [Bridges](../mil2-deep-research-2026-09-29/opus55-bridges-recommendations/SYNTHESIS.md) |
| Staking, restaking and yield | [MIL/2 category 8](../mil2-deep-research-2026-09-29/category-review/08-staking-yield.md) | [Staking and yield](../mil2-deep-research-2026-09-29/opus55-staking_yield-recommendations/SYNTHESIS.md) |

## 4. Decision, implementation and audit order

1. Complete the MIL/3 workflow's S0 decision contract. Record any MIL/4 change to an S0 rule as a new versioned issue, not an inherited approval.
2. For each MIL/4 category row, let its assigned agent produce an independent need-by-need comparison against MIL/3, the prior category report and primary sources. The lead records disagreement and the smallest discriminating example. A formal semantics reviewer checks the combined stage relation, and a financial reviewer checks economics and units.
3. Freeze one full candidate and send the same bytes to fresh Grok 4.6 high and GPT-6 Astra auditors. Record requested and returned identity, effort, digest, terminal status, verdict and dissent. Two substantive agreeing votes are required for a consequential design choice under `AGENTS.md`. Changed normative bytes require new audits.
4. Implement only the first admitted profile in the actual source/5 → Core/4 consumer after its design gate and one-active-delivery-item check. Compare TypeScript and K on the same feasible corpus. Then certify each used primitive at U1 and bind proof, current state and complete ledger effects at U2. Repeat qualification for each later family under U6.
5. Keep SP01.6 blocked history and the existing guarded campaign rules. Independent source research is allowed; a new delivery implementation item cannot occupy the current active slot without an existing guarded transition. No missing native interface, reviewer or result is replaced by a host assertion.

## 5. Evidence status

This first MIL/4 text is a working proposal. It records no design vote, no profile admission and no implemented or native result. Eight category-specific assignments and formal and financial preparation reviews informed it. Their current-turn findings are summarized above; prior raw Opus receipts remain under the linked recommendation directories. Full-candidate review of the earlier bytes is recorded separately. No test, proof, compiler or Preview transaction was run for this document.

## 6. Subsequent K implementation surge

A later [specified-only K surge](../mil4-k-surge-2026-09-29/RESULT.md) produced a composable definition and an [audit reconciliation](../mil4-k-surge-2026-09-29/AUDIT-RECONCILIATION.md) from three GPT-6 Sol implementation seats, three Opus 5.5 reviews and three Grok 4.7 audits. The K definition compiles. Its six-judgment acceptance relation covers a bounded one-domain, one-asset escrow; eight DeFi first profiles have arithmetic and effect projections, while an explicit family-admission entry rejects until authenticated stage adapters exist. The surge does not dispose M4-C1–C5, change this document's working status, or establish a source/Core, native, or ledger result.


--- FILE: deliverables/mil4-k-surge-2026-09-29/RESULT.md ---
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


--- FILE: deliverables/mil4-successor-2026-09-29/S0-DECISION-PACKET-WORKING.md ---
# S0 decision packet for the MIL/3 → MIL/4 successor path

**Date:** 2026-09-29. **Status:** working issue packet; no decision vote or design freeze. This packet instantiates the first action in the [MIL/3 workflow](../mil3-design-draft-2026-09-29/FINALIZATION-AND-IMPLEMENTATION-WORKFLOW.md#6-first-decision-packet-and-next-action). The [MIL/4 working draft](DESIGN-MIL4-WORKING.md) retains all eight DeFi families as later profile questions. The [U0 study](../u0-study-2026-09-28/UNIFIED-PROPOSAL.md#4-the-first-slice-s0) supplies the proposed S0 laws. The September 23 U0 exit counts remain unchanged.

## 1. One live issue register

| ID | Proposed disposition for review | Live alternative and decisive evidence | Current state |
| --- | --- | --- | --- |
| W-D0 first slice | Admit only one-signer, one-domain, one-asset literal-fee transfer and funded AccrualFirst repayment of an existing obligation as S0. | Replace S0 with an evidenced narrower or broader slice only after comparing source/Core path, U1 primitive cost and U2 contrasting-program need. Transfer alone is a sub-slice. | Open; proposed, not voted. |
| W-D1 signature | Prefer one ledger-bound signing contract with exact `/3` intent digest, signer key, program/head/replay/effect public binding. | In-circuit verification may be selected only with a named scheme, verifier circuit and target certificate. Native ledger signature checking over *the same proved digest* is not demonstrated for the preferred alternative. | Open; no scheme, byte encoding or verifier locus frozen. |
| W-D2 canonical language | Use closed typed tags, explicit empty fields, nominal IDs, sorted unique collections, a `/3` domain tag and deterministic rejection of unknown tags. Keep `/4` migration separate. | A different encoding may be used if it proves one byte representation per typed value, source/Core round trips and stable signed digest. Measure finite caps before freezing values. | Open; the legacy `/1` stage schema is an inventory, not `/3` canonical bytes. |
| W-D3 stage/failure | Select the source/5 action and Core/4 program at an authenticated pre-head; compare complete prepared and submitted effects, derived footprints, post-state, authority, history and signed terminal failure policy. Reject atomically with no retained effects. | Any alternate relation must give the same six U0 judgment keys, full effect and failure traces, and exact source/Core/K/native binding. | Open; relation, error order and byte codes not frozen. |
| W-D4 S0 numeric | Use exact checked UInt128 operations on asset-qualified atomic units, with no S0 division, rounding or reserve posting. For the first current-Core demonstration, also bound obligation fields by `2^127−1`. | Widen obligation fields only with a versioned schema and consumer change and a near-bound source/Core/K comparison. AMM/vault Ω and fee placement do not gate S0. | Open; no width correspondence or implementation claim. |

W-D5 cross-domain evidence and W-D6 category admission remain later profile issues. The parts of W-D4 concerning AMM fees, vault conversion and general Ω stay outside S0. No issue in this table has two substantive agreeing design votes.

## 2. Proposed S0 source and effect contract

**Formation.** A candidate names `/3` profile, domain, one signer and signer key reference, one nominal settlement asset with smallest-unit scale, source hash, selected action, Core program identity, pre-head, predecessor commitment, replay ID, validity interval, gross/fee/net bounds, fixed recipients and a terminal failure policy. Supply changes, observations, disclosures and retained effects/duties are explicit empty sets; a nonempty value rejects. Delegation and recovery are `none`. Every sort, tag, count and size has a finite profile cap. The exact wire encoding and domain tag remain W-D2 decisions.

**Literal-fee transfer.** Let principal transfer `v>0` and literal fee `f≥0` be atomic units of the same asset. Owner debit is `v+f`, signed recipient credit is `v`, and signed fee-recipient credit is `f`; aliasing recipients is resolved before effect comparison. `f≤feeCap`, `v+f≤grossDebitCap`, and credited net to the intended recipient meets the signed floor. The fee line is an actual balance transfer, not a computed fractional pool fee. Sender allowance, balance and cumulative work/replay counters change exactly once. The existing Core transfer does not already implement this fee constructor, so the eventual callable path must explicitly prepare and consume it.

**Funded repayment.** Pre-state has existing obligation `o` with principal `p`, accrued `a` and outstanding `p+a`; all are nonnegative and within the first-slice obligation cap. For `0<n≤p+a`, exact conversion `(mantissa=1, scale=0, rounding=none)` gives settlement `n`. Set `dA=min(n,a)`, `dP=n−dA`, `a'=a−dA`, `p'=p−dP`, `outstanding'=p'+a'`. The same stage debits the payer `n`, credits the creditor `n`, consumes allowance and allocation replay state, and writes the obligation. The debt cannot fall without this funded transfer; impairment or a deadline does not count as discharge.

**Complete footprints.** Transfer reads/writes owner, recipient and fee-recipient balances after alias resolution, sender allowance, transfer replay state and work counters, plus ordinary `ReadPre`, `NextWrite` and `Ensure` dependencies. Repay also reads/writes obligation components and status, allocation replay, the funded transfer cells and its ordinary dependencies. Declared reads/writes contain the derived sets. Every post-cell outside derived writes equals its authenticated pre-value. A two-balance transfer footprint or an omitted creditor credit rejects.

**Six judgments.** `stage` checks selected program and typed pre-state; `intent` checks exact signed terms and validity; `effect` checks canonical prepared lines, conservation and post-state; `authority` checks signer, allowance and current budget; `history` checks authenticated predecessor, replay and head extension; `failure` distinguishes pre-accept rejection with no writes from the only admitted terminal success. The exact order and wire rejection codes remain W-D3 decisions. A failed local evaluator call cannot become an accepted fee-bearing phase.

## 3. Signature decision that must be made

MIL/2 §11 proposes `Poseidon(domain_tag || canonical(intent))` as a public digest and says wallet or ledger signature checking is possible; it explicitly does not claim in-circuit Ed25519. The U0 study instead prefers in-circuit authentication. The local evaluator consumes caller-supplied `signatureValid`, and its legacy SHA-256 authority digest is not a `/3` wallet signature. ZKIR curve operations do not themselves verify a signature.

**Alternative A, proposed for first review:** a versioned ledger-bound signature profile. Freeze exact `/3` intent bytes, domain tag, digest algorithm, signature scheme and key encoding. Let `D` be the digest of those bytes. The proof's public statement includes `D`, signer identity/key commitment, domain/profile/program IDs, pre-head, predecessor, replay value and complete effect commitment. Ledger acceptance checks a signature over exactly `D`, verifies a proof with the same `D` and effect commitment, checks the current head/replay, and applies those effects atomically. The recorded Midnight ledger API has BIP-340 key and signature-checking types; it has not established arbitrary Moriarty-intent digest checking or this proof/ledger equality. BIP-340 and Poseidon are candidates until that interface is pinned.

**Alternative B:** the same signed statement is checked by an identified in-circuit signature scheme and concrete verifier gadget. The native proof must constrain the signature and every public binding above; a named target certificate must cover valid and hostile witnesses and cost. This cannot be inferred from generic curve instructions.

Under either alternative, start with a valid signed transfer, substitute one recipient byte while retaining the signature and otherwise feasible envelope, and require `intent`/authentication rejection with zero accepted effects. Repeat for a one-unit fee-cap change and a substituted proof-public digest. A malformed signature envelope alone is not the required negative control. W-D1 cannot be disposed until exact bytes, scheme, digest, verifier locus and accepted-statement equality are selected and audited.

## 4. Numeric discriminators and current consumer boundary

Let `U=2^128−1` and `S=2^127−1`. S0 uses checked add/subtract, comparisons and `min`; no field-modular wrap. The current Core parser bounds principal, accrued and outstanding by `S`, while balances and allowances may reach `U`. A first executable corpus must retain the `S` caller precondition rather than assert that the current source/Core accepts all `u128` debts. K uses unbounded `Int`, so a near-bound TypeScript/K comparison is required before claiming correspondence.

The small positive repayment has `p=1000`, `a=10`, `outstanding=1010`, `n=30`, `dA=10`, `dP=20`, `p'=980`, `a'=0`, `outstanding'=980`, payer debit 30 and creditor credit 30. A first near-bound positive candidate uses `p=S−1`, `a=1`, `outstanding=S`, `n=1`, payer balance 1, creditor balance `U−1`, allowance remaining 1 and spent `U−1`; the expected creditor balance and spent counter are `U`, with debt `S−1`. This trace still requires all other invariant fields in the fixture. Hostile controls include `p=S,a=1` (opening parse/invariant failure), `n>outstanding`, payer overdraft, receiver-credit overflow, recipient change, `f=feeCap+1`, missing effect line, stale predecessor and replayed ID. Each hostile envelope must otherwise be valid and reach its intended rule.

All S0 operations are exact, so rounding direction is `none` and S0 dust class is `none`. The owner [U0 numeric decision](../../docs/decisions/u0-numeric-profile-decision.md) remains binding for later computed primitives. Charged liability increments, non-postable sub-unit residuals and whole-unit protocol-reserve amounts are separate later decisions; none is silently posted by S0.

## 5. Exit and audit boundary

This packet is a review input, not M3-D or U0-F. To dispose it, freeze the `/3` grammar/encoding and complete S0 stage/failure rules, pin the chosen signature contract at the specification level, give every S0 schema leaf a dynamic/fixed/empty-with-rejection/deferred disposition, and obtain fresh Grok 4.6 high and GPT-6 Astra full-candidate audits of identical bytes. A consequential rule needs two substantive agreeing votes, with dissent preserved. Local implementation then needs the actual source/5 → Core/4 consumer, TypeScript/K agreement and separate native/Preview evidence. The blocked SP01.6 delivery item is not bypassed by this paper.

No test, proof, compiler, campaign dispatch or Preview transaction was run to write this working packet.


=== END SOURCE PACKET ===
Return the requested audit now. State if the packet is insufficient for a vote.
