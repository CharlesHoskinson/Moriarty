# Exact integration changes and limits

This candidate is a proposed authority judgment over financial schema03, whose
137-file candidate is 7942eb0af701fc9bebeb796b979a56fdd37b2311a89787b90a35bfe755af36e1.
Schema JSON: 693559219ae197637f509c70485ccc6946403372f1ddfdbb04b7fcbd90a5d526.
Bindings: ccd83dacf0f9b12dc640ad83cd7acc644cf7ceb5135182cfec687fc57dc6ac94.
Consumers: 9db0a9cea5e725effc1f9e1ad062c589d7909405b18049f837115182c16db716.
The copied bytes match the financial author's immutable files. That candidate is
not accepted: the author reports independent findings and is preparing schema04.

## Proposed ABI changes, not edits to schema03

1. Add the closed Grant, Target, Occurrence, Seed and Premises records in
   authority-schema.json as a separately versioned authority layer. Do not silently
   add fields to an old signed record. Keep each existing financial value at its
   65,536-byte/4,096-node bound; the proposed complete authority document has a
   separate262,144-byte/16,384-node/depth64 bound,128 grants and64 transitions.
   These new wrapper bounds need independent design votes; they do not increase
   the published expression/Sigma limits.
2. Bind every child grant to an existing AuthorityDefinition ID, principal,
   delegation, program/action/operation lists, policy and time window. Its recipient
   and operation scope cannot exceed the parent. Parent resource gross/fee and
   nominal creation budgets remain mandatory in addition to per-grant lifetime
   use. Current code accepts parent kind OutcomeIntent with exactPlanHash=None
   only. Exact child grants then narrow it with one-use occurrences. Legacy parent
   ExactPlan records reject LEGACY_EXACT_PLAN; there is no migration of old
   signatures, reusable plan-hash membership or assumed wire compatibility.
3. The financial evaluator must supply its **derived**, complete typed transition,
   not a caller-created Effect array. Its independent transition judgment binds
   the exact profile/source, ordinary/recovery mode, reason/evidence commitments,
   operands, effects, full state, time and predecessor in this document. The
   authority checker independently derives targets and compares claims, movements,
   origins and counters. It does not replace financial arithmetic, receipt use,
   rule execution, collateral allocation or complete financial frame verification.
4. Initialize the authority ledger from an authenticated seed containing the exact
   financial state, fixed grant-set commitment, per-grant usage and occurrence
   consumption. Load parent spending from the actual AuthorityCounter and
   NominalCounter rows. New actions increment the appropriate totals and must
   reproduce the corresponding financial after-counters exactly. A new grant
   cannot make already spent nominal/cash capacity available again. Capitalization
   debits the debtor's creation capacity; creditor consent is a different right,
   not a fictitious second nominal creation counter.
5. Integrate signing06 currentness with these full typed transition hashes. Its
   external context must establish the exact seed and authentic grant documents,
   active policy status, ledger-current predecessor and consumed occurrence set.
   Branch integration must partition the actual residual grant budgets and merge
   common ancestry once under the signing composition rules. This checker supports
   a bounded linear sequence and a trusted seed; it does **not** yet implement the
   full38 split/join adapter. Do not feed arbitrary branch outputs as trusted seeds.
6. Use signing_bundle for developer display: retain the complete canonical signed
   grant bytes **and** the referenced profile preimage, including parent limits.
   A display hash alone does not show those limits. The ExactPlan transition hash
   binds all numerical values; OutcomeIntent target identity projection preserves
   typed roles/IDs/rules while quantities are checked against independent budgets
   and the admitted financial relation. No argument-name heuristic or host rule
   decides which fields are financial. Exact operation-index/source/action origins
   are validated before deriving grants.

## Claim and policy mechanics

All38 rows in operation-authority-map.json define a base right and an actual
operand/state principal selector. Additional rights come from concrete effects:
cash and fees, fresh/increased duties, paid/discharged/carry claims, both old and
new assignment parties, changed debt terms, capitalization consent, collateral
and represented supply. A Carry to a fresh ID is inspected against the old source
and actual successor; it cannot evade assignment by avoiding in-place mutation.
The scalar grant debit uses the exact typed quantity. An explicitly zero claim
uses zero monetary capacity; the separately required base-operation grant still
consumes one operation use. No zero-entitlement field becomes a positive amount.

The fixed type walk collects policy-bearing resources from actual operands and
effects. The exact profile path must include their source and destination asset/
vault policies, plus the transition policy. Current policy hashes/windows and
revocation context are checked. Rule-specific helper dispatch remains owned by
the financial evaluator, not an arbitrary authority callback.

## Necessary schema04 reconciliation before combined acceptance

The financial author is adding exact movement/helper contexts and per-operation
policy dispatch, complete Request/Message reservations, conditional/branch duties,
share entitlement/backing, imported supply and deterministic Genesis/Migrate
initialization. This candidate cannot initialize missing schema03 fields. Once
schema04 freezes, update only a successor authority candidate: copy/pin its schema,
regenerate structural JSON definitions, bind new effects/claim-map fields and
exercise actual generated transitions for every affected operation family.

The current examples are full typed source records with manually stated external
financial/authentication premises. They are not an execution of all38 financial
consumers. Required follow-up is a combined interpreter-derived funded creation,
repayment, request fulfillment/cancel and migration history, plus the branch
adapter and work/mode relation. Actual signatures, legal authority, oracle truth,
native identity/fees, PCD and Midnight acceptance remain separate mandatory gates.
The48 financial requirements,40 published expression constructors, full ACTUS/DeFi
corpora, five composition operators and asset-study cases are unchanged.
