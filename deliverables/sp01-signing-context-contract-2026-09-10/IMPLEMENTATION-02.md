# Executable source contract 02

This successor preserves all six draft01 files unchanged. The full signing
contract remains proposed. The new closed example schema and contextual checker
implement a bounded executable part of it; they do not register a signature or
financial runtime ABI. Full38 financial wire definitions are still absent in the
parallel operation draft, whose mathematical types remain explicitly unresolved.
Unsupported operation tags reject schema admission. No opaque Any payloads occur.

`schema-02.json` closes every document, genesis input, authority, plan, argument,
literal operation operand, full state and transition record. `profile-02.json`
owns the finite parties/assets, actors, obligation, declared entry bodies and
operation schedule. Account maps have every profile account key, including zeros.
The checker does not branch on example IDs or intended failure labels. Profile
changes require schema regeneration and new candidate review; callers cannot
inject a type registry or reinterpret existing signed bytes.

The profile is bound by `digest(profile,{profile, schema})`. Every hash is SHA256
of UTF8(`MORIARTY-SIGNING-CONTEXT-EXAMPLE/2/`+recordKind), a NUL, and exact sorted-key
compact UTF8 JSON. Record kinds are profile, genesis, authority, plan and state.
`decode_canonical` rejects duplicate object keys and noncanonical spelling. The
schema forbids numeric JSON values and closes arrays/records. UInt128 domains
are checked before relation evaluation. Whole document bound is65,536 bytes;
32 steps,16 operations per step,8 arguments,16 funding inputs,8 duties and256
sidecar bytes are explicit EXAMPLE bounds, not inherited production parameters.
Signature hashing algorithms/native field compatibility remain unimplemented.

Hash graph: genesis funding/declaration/work → genesis hash → authority → authority
hash. Authority has profile and (ExactPlan only) selected plan hashes. Plans contain
no authority/state hash, so no cycle. Genesis-derived initial full state has its
own hash. Each step binds authority hash, predecessor full-state hash, complete
plan/hash, full resulting state/hash and charges. The next step binds the prior
state hash. All bodies/preimages occur in the document; omitted or stale bodies
cannot be supplied through an unchecked external registry. Typed authority here
is assumed by the caller's explicit test context, not cryptographic evidence.

Actual predicate order is the implementation: finite schema/domain; genesis and
authority/profile hashes; environment target/revocation/window; initial funding
and state hash; each step's authority/plan/post hashes and named argument checks;
mode/activation/current predecessor and uniqueness; declared body/operation
authority, derived movements/debt/backing, work and caps; claimed charge equality;
full state equality; terminal net/duty checks. First failing predicate is returned
from these relations. A rejection label cannot instruct the checker. Some financial
guards occur while deriving effects before work comparison; this explicit order
supersedes draft01's idealized order for this executable subset only.

Financial relation: Transfer and Fee debit source and credit destination exactly
once; both count toward gross and Fee additionally toward fee consumption, matching
the current financial draft. This differs from the optional excluded-fee policy
mentioned in draft01. All balances/counters/frame fields are compared. Loan
creation matches an actual same-plan advance; repayment matches cash and cannot
reuse one transfer for multiple Repay operations. Accrual is a signed bounded
addition amount in this example, NOT a validated rate/day-count calculation.
Cancellation derives goal-receipts-plus-fees deficit, checks current payer balance
and remaining authority, reserves a surviving duty and prevents duplicate backing.
Migration changes only version/work; all financial state remains. Terminal success
requires net goal, zero debt and no duties. Receiving net uses the exact symmetric range −(2^128−1)..+(2^128−1), derived from two UInt128 counters. This matches the financial author’s proposed NetAmount<A> dependency; it is not narrowed to SInt128 or added to the frozen expression types. Fees/receipts use named profile assets;
no comparison between unrelated units is introduced.

Costs are derived from each complete operation term and literal occurrence plus
ceil(sidecar bytes/256), with separate recovery for Cancel. This literal-only term
format is NOT the full frozen40-constructor expression representation. Integrating
that expression evaluator/cost derivation is still required before full SP01.3
closure. No Boolean semantics proposal is substituted. The archived certificate
summation defect is exercised through actual charge/sidecar mutations.

`validate_partition` checks exact generic source partition records with distinct
identities, one origin, after-charge ordinary/recovery conservation, every account's
gross partition and unique duty assignment. It is independent of example names.
It does not establish full multi-input history validity: financial debt partitions,
policy/goal compatibility, shared-prefix ledger merging, actual native signatures
and branch/join acceptance remain required. Linear histories and this partition
predicate cannot be promoted to full MC06 composition.

Two retained RED commands evaluate the exact archived AST function/condition with
inert dependencies: first_false_stage returns an intended label on unchanged data;
metadata checks argument0 instead of the enclosing debt_id binding. These are
narrow reproductions, not a rerun of the archived full generator. The new connected
tests use complete hand-written initial/post states and compare exact first codes;
mutations rebuild all dependent hashes where appropriate. Removing the mutation
recovers acceptance. The exact-plan permutation rejects; outcome selection permits
the reordered named bindings while preserving ObligationId metadata.

Run `python3 context-checker-02.test.py`. The retained outputs are source checks
only. No runtime, wallet, service, proof, signing, network or ledger action occurs.

## Required closure before full contract freeze

1. Financial owner must supply closed profile-owned records for all38 operations,
   liabilities, requests/messages, policies and observations; current mathematical
   signatures are insufficient. Pin their schema and implement the selected rules.
2. Replace the example literal term format with the reviewed expression dependency
   and its complete source/Core/action derivations, retaining exact work schedules.
3. Extend generic identities/partition checks to full policy, cumulative ledger,
   recovery and duty histories over all admitted operators; retain independent
   complete funded positive and negative serialized histories.
4. Specify actual signature suite, domains and display envelope, registry/observation
   authentication, and implement four mandatory claim validation. This checker
   always reports signatureVerified=false and ledgerAccepted=false.

These are concrete remaining interfaces, not acceptance claims. Draft01 is not
rewritten, archived candidate04 is not reused as current authority, and none of
the expression or financial owner's frozen/source files is modified.
