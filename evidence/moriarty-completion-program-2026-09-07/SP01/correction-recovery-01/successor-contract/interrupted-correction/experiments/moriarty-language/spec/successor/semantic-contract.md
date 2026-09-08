# SP01.3 successor semantic and signing contract

Status: **PROPOSAL**. Not a semantic freeze. Not registered. Not accepted.
Pending SP01.2 challenge-map reconciliation and user-selected cross-provider
majority for consequential choices. Independent GPT-6 reviews this output.
Full source grammar is SP02. Executable K Core is SP03. This file defines
the interfaces those sprints consume.

Author: Grok 4.6 high. Worktree only. No runtime, proof, native, or old-profile
edits. Old `moriarty-bounded-atomic/1` bytes remain a distinct experimental
profile. Their SHA domains are not successor crypto.

## 0. Classification and pins

Every sentence below is one of: source fact, repository observation, inherited
constraint, proposal, open check. Unmarked normative tables in sections A-F
are **proposal**.

Pinned inputs (SHA-256 of exact worktree bytes):

| Path | SHA-256 |
| --- | --- |
| `deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md` | `bd9a7af78e400da7d8ba4d57abc9693665579ea78e7fdb98004f231ecbfd0aa8` |
| `deliverables/defi-language-design-2026-09-07/action-targets.csv` | `7b9b245b97a4d63f6cc946c9dd7e7f6c3773df4cf47b2540d82b5ffc6cfe3d2f` |
| `openspec/REPORT-RECONCILIATION-2026-09-07.md` | `a1a1661fc0d554d8e469651fdbd42c41e7c9e2115c8550f369742dcee94bd5da` |
| `openspec/sprints/sp01-financial-contract-and-execution-admission.md` | `fdb586d9bf05f2fbf7e9c2294be49854528db20e575faf9aaa1144b033908172` |
| `openspec/sprints/sp02-complete-mori-authoring-frontend.md` | `d5a0bfca8edf795e7724b1a61b0ab959851fdba9cfe0c3b3f84ac289b2dfee28` |
| `openspec/sprints/sp03-executable-bounded-semantics-in-k.md` | `1bbe0401f74cf236c2150486bfec719db027648d7aad52ea97952e6b56919ab9` |
| `evidence/moriarty-design-sprint-2026-09-06/worked-examples.json` | `46aee15de568ce1bab7372588866aac5f3e62ca7ecac73ee1ee23de0610ecf2b` |
| `evidence/moriarty-completion-program-2026-09-07/SP01/loan-swap-subset-01/design.md` | `4ec089205fc69b94b80cd68b94a82b17c16d98e0240355b9977506983da9d7ca` |
| `experiments/moriarty-language/spec/bounds.json` (old profile, immutable) | `b548641a1a9d74bab68ba699ffb1e2350fa0889d61b8704e98216f9d4a6c3664` |
| `experiments/moriarty-language/src/types.ts` (old profile, immutable) | `5cc7356e9a38a74f1a202c7c6c8d7ac6d1ef6eac8fe0d8013beb242e3de604d0` |
| `experiments/moriarty-language/src/codec.ts` (old profile, immutable) | `8575d98e630fed5b6e68b23c574aec450fc2bbce1b83b057ce6154e43610b7e8` |
| `experiments/moriarty-language/spec/numeric-profile.json` | `6d88f694bf8af8c5b7dd75fce76f58fe0d14fc68c2782dd0d3fb885ca4a7eb15` |

Inherited constraints (RP01, LANGUAGE-DESIGN, program rules): finite registered
bounds; four mandatory acceptance claims; Preview sole public target; no host
boolean as acceptance; `Prepared` is not acceptance; no unbounded
strings/arrays/maps/recursion/callbacks; refunds cannot restore gross debit;
fees count against net goals; residual duties survive partial progress; unknown
extensions and revoked keys reject. These are not reopened by this proposal.

SP01.2 live work is not an input. No row here implies that map is approved.

## A. Named finite domains

Proposal: every value inhabits a named finite domain. Profile admission names
the simultaneous maxima. Cartesian products of maxima are not encodable.

### A.1 Primitive domains (proposed widths, not registered)

| Domain | Width | Inclusive range | Wire |
| --- | --- | --- | --- |
| `UInt64` | 64 | `0 .. 2^64-1` | canonical unsigned decimal |
| `UInt128` | 128 | `0 .. 2^128-1` | canonical unsigned decimal |
| `SInt64` | 64 | `-2^63 .. 2^63-1` | canonical signed decimal |
| `SInt128` | 128 | `-2^127 .. 2^127-1` | canonical signed decimal |
| `Bool` | 1 | `{false,true}` | JSON boolean |
| `Identifier` | 1..64 | ASCII `[A-Za-z][A-Za-z0-9_]*` | UTF-8 ASCII |
| `Party` | Identifier | finite roster per instance | Identifier |
| `AssetId` | Identifier | finite roster per instance | Identifier |
| `Time` | UInt64 | UTC epoch seconds | unsigned decimal |
| `Date` | UInt32 | days since Unix epoch | unsigned decimal |
| `EventOrder` | UInt64 | instance-local event ordinal | unsigned decimal |
| `Digest` | 256 bits | SHA-256 digest | 64 lowercase hex |
| `Scale` | UInt8 | `0 .. 18` | unsigned decimal |

Canonical integer spelling: `0` or `-?[1-9][0-9]*`. Reject leading zeros, `+`,
`-0`, separators, exponents, JSON numbers. Intermediates use the same width as
the result domain unless a named `checked_wide` form is admitted. Overflow,
underflow, and division by zero reject the whole action. Arithmetic is not
reassociated without an intermediate-bound preservation argument.

Adjustable parameters (majority before freeze): integer widths, Scale maximum,
collection capacities, predecessor fan-in, ordinary and recovery work, source
and signing byte ceilings. Admission checks all of them before allocation or
expensive proof.

### A.2 Indexed financial domains

`Amount<Asset>` is a nonnegative token quantity. Record:
`{tag:"Amount", asset:AssetId, value:UInt128}`. Asset is a ledger identity, not
a unit name. Quantum conversion is explicit settlement: `value mod quantum = 0`
and `ledgerAmount = value / quantum` in UInt128.

`Debt<Denomination>` is signed in orientation, not as a hidden integer sign.
Record: `{tag:"Debt", debtId:Identifier, debtor:Party, creditor:Party,
denomination:Identifier, outstanding:UInt128, status:DebtStatus}`. Outstanding
is nonnegative. Debtor/creditor names the direction. `SInt128` is for P&L,
funding, and signed price deltas. It is not a substitute for missing parties.
Creating or increasing debt requires liability authority. Transfer authority
never implies DebtCreate.

`Shares<Vault>`: `{tag:"Shares", vault:Identifier, value:UInt128}`. Distinct
from the vault's underlying `Amount<Asset>`.

`Rate`: `{tag:"Rate", mantissa:SInt128, scale:Scale}` meaning
`mantissa * 10^(-scale)`. `Price<Base,Quote>`:
`{tag:"Price", base:AssetId, quote:AssetId, mantissa:UInt128, scale:Scale}`.
Exchange uses an explicit Price or Rate plus a dust Amount of the output asset.
Remainder strictly below dust stays in the source reserve. No implicit unit
coercion. No floating literals. No plain `/`.

Identities (unique in the instance lifetime, including terminal tombstones):
`PositionId`, `ObligationId`, `RequestId`, `MessageId`. Records occupy capacity
after cancel, settle, claim, or expire. Identity reuse rejects.

Finite records, enums, options, and collections have profile-bound capacity and
keys. Keys are Identifier or Digest. No unbounded strings, arrays, maps,
recursion, or callbacks.

### A.3 Amount predicates

Every protected quantity names Min or Exact:

- `Min{amount}`: result `>= amount` (over-delivery allowed).
- `Exact{amount}`: result `== amount` (over-delivery rejects).

The admitted atomic swap uses Min (`min_out`). Exact-output is a distinct
predicate (DA01). Do not treat Min as Exact.

### A.4 Source versus Core (proposal choice)

Distinguish source sugar from typed Core. Source has `agreement`, `action`,
`requires`, `let`, `next.field=`, `emit`, `ensures`, declarations, and
composition keywords. Core is the finite constructor set in C.3. SP02 elaborates
source to Core. SP03 executes Core. S-expressions and JSON are diagnostic
interchange, not trusted compiler inputs. This distinction is a consequential
proposal (SD-SUGAR-CORE).

## B. Lexical and grammar contract

Complete EBNF is SP02. This section is the contract SP02 must satisfy. The
LANGUAGE-DESIGN action fragment is an excerpt, not a full grammar. No ellipsis
is labeled full BNF.

### B.1 Lexical

- File encoding: well-formed UTF-8 without BOM. Lone surrogates reject.
- Identifiers: ASCII `[A-Za-z][A-Za-z0-9_]*`, length 1..64, case-sensitive.
  Reserved words cannot be identifiers. Ban `constructor`, `prototype`,
  `__proto__`.
- Keywords are ASCII and case-sensitive.
- Integers: canonical unsigned or signed decimal as A.1. No float tokens.
- Strings: JSON-compatible, Unicode scalar validation. Escapes if any:
  `\"`, `\\`, `\/`, `\b`, `\t`, `\n`, `\f`, `\r`, and `\u00xx` for other
  U+0000..U+001F. No other escape. Identifiers have no escapes.
- Whitespace: ASCII space, tab, CR, LF between tokens.
- Comments: `//` to end of line; non-nesting `/* ... */`. Counted in source-byte
  limits. Comments do not alter Core, authorization, or claims.
- Unicode normalization: none. NFC/NFKC is not applied. Identifiers are ASCII,
  so this hits only string literals and comments.
- Token priority: longest match for `==`, `<=`, `>=`. No `!=`.
- Source hash: SHA-256 of original UTF-8 bytes, separate from semantic hashes.

### B.2 Keyword categories

Declaration: `agreement`, `profile`, `lifetime`, `horizon`, `unit`, `const`,
`state`, `observation`, `settlement`, `policy`, `status`, `effect`, `action`,
`reserve`, `recovery`, `enum`, `option`, `record`, `finite`.
Type: `UInt64`, `UInt128`, `SInt64`, `SInt128`, `Bool`, `Amount`, `Debt`,
`Shares`, `Rate`, `Price`, `Time`, `Date`, `Party`, `AssetId`, `Text`.
Statement: `requires`, `let`, `next`, `emit`, `ensures`.
Composition (not Boolean): `seq`, `par`, `interleave`, `atomic`, `message`.
Boolean: `not`, `and`, `or`, `true`, `false`.
Rounding: `floor_div`, `ceil_div`.
Admin: `genesis`, `activate`, `revoke`, `migrate`, `pause`.
Reserved binders: `pre`, `post`, `remaining`.

`and` is Boolean conjunction only. It is not financial composition.

### B.3 Precedence (tightest to loosest)

1. Parentheses, calls, field projection.
2. Multiplication.
3. Addition, subtraction.
4. One non-chained comparison (`== < <= > >=`).
5. `not`.
6. `and`.
7. `or`.

So `not a == b` means `not (a == b)`. This differs from the atomic profile.
Comparisons do not chain. `and`/`or` short-circuit left to right. Conservative
static bounds still apply to both branches. Evaluation is lexical left to right.
The first failed clause is the diagnostic. `requires`/`let`/`next`/`emit` may
interleave. All `ensures` form a suffix. `pre` is immutable. `next.field` writes
at most once. RHS cannot read `next`. `post` only in `ensures`.

### B.4 Source constructs required by DA01-DA24

SP02 must give a production for each: agreement header (profile, lifetime,
horizon); unit, const, state, observation, settlement, field policy, status,
effect schema, action, reserve, recovery-reserve; enum, option, record, finite
collection declarations; parameters; `requires`, `let`, `next.field=`, `emit`,
`ensures`; literals; binary and unary operators; `floor_div`, `ceil_div`;
amount/debt/shares/rate/price constructors; Min/Exact predicates; composition
forms `seq`, `par`, `interleave`, `atomic`, `message`; admin actions listed in
E.4. Missing production is a SP02 defect, not an implementation default.

### B.5 Required operations and errors

Every Core op in C.3 names its reject code. Closed diagnostic families:

| Family | When |
| --- | --- |
| `LEX_*` / `PARSE_*` | encoding, token, grammar |
| `TYPE_*` | unit, width, constructor mismatch |
| `BOUND_*` | profile capacity, bytes, depth, nodes |
| `GUARD_FAILED` | `requires` false |
| `ENSURES_FAILED` | `ensures` false |
| `ARITH_*` | overflow, underflow, div-zero, noncanonical |
| `AUTH_*` | missing authority, attenuation, principal |
| `INTENT_*` | debit cap, uncapped debit, net goal, debt cap, recipient, call, exact mismatch |
| `OBLIGATION_*` | unknown, duplicate, mismatch, partial-rule, excess, already-terminal |
| `REQUEST_*` / `MESSAGE_*` | illegal status transition, replay |
| `WORK_*` | ordinary exhausted, recovery exhausted, cloned residual |
| `OBS_*` | unauthentic, stale, wrong role, missing assumption |
| `SPEC_*` | unknown extension, revoked key/spec, activation window |
| `HISTORY_*` | duplicate predecessor, consumed, order, migration replay |
| `ACCEPT_*` | missing claim, proof-context mismatch |
| `LEDGER_*` | currentness, unique consumption, finality (ledger, not K) |

First error in lexical evaluation order wins. Financial rollback discards all
tentative writes and effects. Submission fees are outside that rollback.

## C. Typing, effects, Core

Judgments (proposal):

```text
Γ; Δ ⊢ e : τ ! ε
Γ; Δ ⊢ stmt ⇒ Γ'; Δ' ! E
Γ; Δ ⊢ action ⇒ Δ' ! E
⊢_bound profile, program
```

`Γ` is ordinary bindings and scope. `Δ` is owned resources and capabilities.
`ε`/`E` are effect sets. Pure expressions have empty `ε`. Typing is paired with
a resource-bound judgment and visibility (public/private) constraints.
Unauthorized declassification rejects.

Scope: `let` binds a sequential immutable local to end of action. Parameters,
consts, `pre` fields, and observations are immutable. `next` is write-only
staging. Duplicate `next.field` rejects. RHS of any expression cannot mention
`next`. `post` is elaborated only after all writes assemble.

Resource use: identities in `Δ` cannot be duplicated or dropped except by a
named constructor. Quantitative conservation is a separate accounting judgment,
not ownership.

### C.1 Staged evaluation

```text
Eval(profile, program, pre, action, observations, authority, predecessors)
  = Reject(code, span)
  | Prepared(post, effects, duties, residualAuthority, successors,
             remainingWork, claims)
```

Rules: evaluate finite expressions; check `requires`; stage each write/effect;
assemble post (unwritten fields copy `pre`); check `ensures` suffix; on any
reject, discard tentative financial changes. `Prepared` is a candidate. K tests
and host flags are not acceptance. Ledger fees of a rejected submission are not
in the language rollback.

### C.2 Proposed numeric and shape bounds

Not registered. Not a freeze. Majority and SP02 `bounds.json` before use.

| Parameter | Proposed ceiling |
| --- | --- |
| source UTF-8 bytes | 65536 |
| identifier characters | 64 |
| signing document UTF-8 bytes | 65536 |
| decoded depth | 16 (signing/eval), 40 (AST) |
| keys per record | 64 |
| array length | 128 |
| effects per action | 16 |
| writes per action | 64 |
| predecessors | 8 |
| ordinary lifetime work | UInt128, instance-frozen |
| recovery lifetime work | UInt128, instance-frozen, distinct |
| claim count | 16 |
| observation fields | 64 |
| collection capacity | 128 including tombstones |
| expression depth | 16 |
| Scale | 18 |
| unit-vector components | 8 |
| absolute unit exponent | 16 |

Checks run before allocation or expensive proof: bytes, depth, nodes, claim
count, predecessor fan-in, sidecar bytes, total verification-work budget, spec
activation, revocation, unknown fields.

### C.3 Typed Core operations

Each row: constructor, domain, preconditions, effect, consumption, output,
financial reference. Unsupported constructors reject. No hidden semantic branch.

**Pure arithmetic**

| Op | Domain | Pre | Eff | Cons | Out | Ref |
| --- | --- | --- | --- | --- | --- | --- |
| `Add`/`Sub` | same Amount, UInt, or SInt | no overflow/underflow | none | none | same | DA01,DA06 |
| `Mul` | numeric; unit exponents add | no overflow | none | none | result type | DA01 |
| `FloorDiv` | numeric; unit exponents subtract | denom > 0 | none | none | floor | DA01,DA06,DA17 |
| `CeilDiv` | same | denom > 0 | none | none | ceil | DA17 |
| `Eq`/`Lt`/`Lte`/`Gt`/`Gte` | same domain | comparable | none | none | Bool | all |
| `Not`/`And`/`Or` | Bool | — | none | none | Bool | — |

**Staging**

| Op | Domain | Pre | Eff | Cons | Out | Ref |
| --- | --- | --- | --- | --- | --- | --- |
| `Require` | Bool | true else `GUARD_FAILED` | none | none | unit | LANGUAGE-DESIGN |
| `Let` | x:τ = e | x unbound | none | none | Γ,x | LANGUAGE-DESIGN |
| `NextWrite` | field:τ, e:τ | unwritten; e from pre/locals/args/obs/const | stages | write slot | unit | staged semantics |
| `Ensure` | Bool | suffix; true else `ENSURES_FAILED` | none | none | unit | LANGUAGE-DESIGN |
| `Emit` | effect record | schema, policy, authority | appends E | effect slot | unit | DA21 |

**Assets**

| Op | Domain | Pre | Eff | Cons | Out | Ref |
| --- | --- | --- | --- | --- | --- | --- |
| `Transfer` | Amount<Asset>, from, to | balance; transfer authority; not DebtCreate | Transfer | amount at from | balances | DA01 |
| `Fee` | Amount<Asset>, from, to | fee policy; counts in gross and net | Fee | amount | balances | DA21 |
| `Exchange` | Amount in, Price/Rate, dust, Min/Exact out | reserves; rounding named; dust explicit | two Transfers | in amount | out amount | DA01 |
| `Mint`/`Burn` | Amount, authority | mint/burn cap | Mint/Burn | none / amount | supply | DA02,DA10,DA17 |
| `Lock`/`Unlock` | Amount or Shares, locker | lock rule | Lock/Unlock | lock slot | encumbrance | DA05 |

**Debt and shares**

| Op | Domain | Pre | Eff | Cons | Out | Ref |
| --- | --- | --- | --- | --- | --- | --- |
| `DebtCreate` | Debt record | liability authority; debt cap; unique id | DueCreated | debt cap | obligation | DA06,DA09,DA10 |
| `DebtAccrue` | obligation, Rate, Time/Date, rounding | exists; calendar named | Accrual | none | outstanding+ | DA06,DA11 |
| `DebtRepay` | amount, allocation rule | outstanding > 0 | Transfer + DueSettled | cash + debt | outstanding- | DA06 |
| `DebtWriteOff` | amount, loss authority | named; residual remains unless allocated | WriteOff | none | residual debt | DA07 |
| `DebtCapitalize` | amount | liability authority | Capitalize | none | principal+ | DA09 |
| `ShareMint`/`ShareBurn` | Shares, Amount, rounding direction | vault rule; zero-supply policy | Share* | asset or shares | shares | DA02,DA17 |
| `ShareConvert` | in, out, method rounding | method-specific floor/ceil | Convert | in | out | DA17 |

**Positions, requests, messages**

| Op | Domain | Pre | Eff | Cons | Out | Ref |
| --- | --- | --- | --- | --- | --- | --- |
| `PositionOpen`/`Adjust`/`Close` | PositionId, range/tick bounds | finite traversal; direction rounding | Position* | position slot | position | DA03,DA11 |
| `RequestOpen` | RequestId, committed amount | unique; work remaining | RequestOpen | assets + work | Pending | DA18,DA15 |
| `RequestFulfill` | RequestId, fill amount | Pending; cumulative cap | Fulfill | fill | Pending or Claimable | DA18 |
| `RequestClaim` | RequestId, amount | Claimable; not double claim | Claim | entitlement | Claimed/residual | DA18 |
| `RequestCancel` | RequestId | controller; not resurrect Claimed | Cancel | residual authority | Cancelled | DA18,DA21 |
| `MessageSend`/`Receive`/`Refund` | MessageId, payload bound | provenance; finality assumption | Message* | message slot | status | DA23 |

**Observations and admin**

| Op | Domain | Pre | Eff | Cons | Out | Ref |
| --- | --- | --- | --- | --- | --- | --- |
| `Observe` | typed field, anchor, freshness, role | authentic; not treated as proven truth | none | obs slot | value + assumption | DA20,DA12 |
| `Genesis` | initial state, lifetime, work, claims | unique instance | Genesis | genesis slot | state | E.4 |
| `Activate`/`Pause`/`Revoke`/`Migrate` | spec/key/window | admin authority; no work reset; no consumed revive | Admin | admin slot | spec status | DA22 |

Rounding policy is named per op: floor, ceil, none. Remainder disposition is
named: discarded-with-record, stays-in-reserve, allocated-to-party. ACTUS
day-count, calendar, and business-day names are financial definitions, not
host clocks. `obs.now` is an authenticated observation.

### C.4 Observation assumptions

Every observation carries: `anchor` (Digest), `freshness` (notAfterExclusive),
`role` (who may submit), `authenticity` (wrapper predicate), `truthAssumption`
(`required_observation` | `external_unproven` | `ledger_predicate`). Oracle
truth, custody, finality, witness availability, key trust, and proof soundness
are named assumptions or separate obligations. They are not implied by a valid
transition.

## D. State, effects, work, composition

State is all of:

- balances: `Amount<Asset>` by (party, asset)
- nominal liabilities and accrual: `Debt` records and accrued fields
- shares, positions, requests, messages
- status: episode and agreement
- event order
- claims in force
- locks and residual authority
- consumed histories
- conserved ordinary Work and conserved recovery Work

Effects are typed complete records. A candidate that omits an emitted effect
rejects. Duties are residual obligations, requests, messages, and locks.
Residual authority is an `allowedEffectSet` plus caps. Successors are identified
continuation artifacts. `remainingWork` partitions ordinary and recovery.
`claims` lists the four mandatory kinds plus named ContractInvariant instances.

Closure exhaustion never erases debt. Partial payments preserve creditor and
controller identity. Split/join partitions; no duplication of identities,
residual authority, or remaining work. Cancellation, revocation, and migration
cannot resurrect consumed state or replenish spent budget.

Reserve recovery work at genesis. If required future steps cannot fit ordinary
plus recovery, reject admission. When ordinary work is zero, only recovery
constructors under the recovery reserve may run. Recovery cannot mint ordinary
work.

Continuation: `continuationOwner` is a Party. Only that party, or a named
delegate inside residual authority, may submit the next Prepared. Attenuation:
`allowedEffectSet(child) ⊆ allowedEffectSet(parent)`, and every numeric cap of
the child is `<=` the parent cap of the same key. Risk metrics (volatility,
utilization, health factor) are not invariants of attenuation.

### D.1 Five composition operators

`and` is not an operator. Proof split/join is not an operator. Async is not
proofjoin. Each operator has its own read, write, authority, prefix, and
history contract. MC06 owns interface/authority/history with MC07 cases.

| Op | Read | Write | Authority | Prefix | History |
| --- | --- | --- | --- | --- | --- |
| `seq` | later may read earlier post | exclusive in order | same or attenuated | earlier prefix visible to later | predecessors chain |
| `par` | disjoint footprints | disjoint writes | partition; no copy | no shared prefix mutation | fan-in <= bound; join cannot replenish work |
| `interleave` | shared reads; conflict rule named | locked regions | shared caps consume monotonically | observable prefixes declared | each step consumes residual |
| `atomic` | all or nothing | all or nothing | one authority set | no external prefix | one history step |
| `message` | local plus authenticated message | local; remote via message | sender residual; receiver own | message not local post | message id consumed once; finality assumed |

### D.2 Request and privacy states

Request: `Pending -> Claimable -> Claimed`, with `Cancelled` and `Expired` from
Pending/Claimable only. Claimed cannot cancel. Expired does not erase a Claimed
payment duty (DA12). Private fields need authorized declassification.
Commitments do not publish private data. Private succession must deliver usable
witness material, predecessor evidence, residual duties/authority/work, and
recovery information (MC06). A commitment alone is insufficient.

## E. Acceptance

Four mandatory claims, always:

1. `ContractInvariant` (source/profile spelling `ContractProperty` in the atomic
   subset). Agreement properties of the declared model.
2. `IntentRefinement`. Concrete execution satisfies the signed ExactPlan or
   OutcomeIntent, including gross debit, fees, recipients, net goals, new
   liabilities, locks, residual, recovery.
3. `TransitionValidity`. Eval produced this Prepared from these inputs.
4. `HistoryCompliance` (atomic spelling `PredecessorHistory`). Compatible
   predecessors, policy, consumed authorization identity, no revival.

No source option disables them. Unsupported mandatory claims fail closed. No
boolean witness establishes acceptance. No optional history. No host simulation
fallback.

### E.1 ExactPlan versus OutcomeIntent

ExactPlan commits to the execution body. IntentRefinement checks equality with
that body. OutcomeIntent commits to constraints. A solver may select any
admitted plan. IntentRefinement proves the selected plan and complete output
satisfy the signed constraints. Outcome mode does not require a second ExactPlan
signature. Soft ranking is unsigned. An accepted route is not a global
optimality promise.

Signable intent is independent of the concrete selected route. Concrete
acceptance binds: signed intent digest, selected plan, observations, full
output effects, liabilities, work, predecessor IDs.

### E.2 Prepared is not acceptance

Acceptance additionally requires: canonical bounded inputs; authenticated
current authorization; admissible observations; unique current predecessor
consumption; native verification of the exact statement; exact binding to
ledger effects. Ledger owns unique consumption, currentness, order, and
finality. Unknown extensions and revoked keys reject even with a valid proof.
Dependencies are bounded and compatible. Decoding is strict.

### E.3 Partial fill and durable consumption

Consumption identity is `(network, deployment, principal, nonce)`. Persist
cumulative gross debits, fees, net delivered, outstanding liabilities, residual
rights, remaining work. Successors update that record. Refunds do not restore
gross. Fees count in gross and in net goals. Concurrent fill and cancel race on
ledger currentness of the residual. Reloading a solver does not reset the
record. Cloned residual work rejects.

### E.4 Genesis and admin

Genesis freezes profile, program, claims, lifetime, ordinary and recovery work,
horizon, initial state, observation and principal bindings. Activate enables a
spec/key inside a window. Revoke disables it; later valid proofs under that key
reject. Migrate cannot revive consumed history or replenish work. Pause stops
ordinary constructors except named admin/recovery. Parameter change cannot
downgrade claims.

### E.5 Proof context (data bound, not an API)

The statement binds, at least: program/spec/policy versions, signed
authorization digest, selected plan hash, predecessor identities, observations
hash, post state, complete effects, duties, residual authority, remaining work,
claim list. No proof bytes enter the payload they authenticate. No circular
statement hash. This names required fields. It does not invent a prover,
verifier, or deployment API. Native correspondence is an open check.

Local financial rollback is distinct from submission fees.

## F. Canonical signed bytes and display

Render only parsed canonical signed bytes. Every authority-impacting field has
required `x-display` `{label, path, unit, role}`. Unknown field, extension, or
display/signature mismatch rejects. No hidden wildcard.

### F.1 Proposed encoding (majority and native fit before freeze)

Codec name: `moriarty-canonical-json/succ-0` (**proposal**). Do not treat
`moriarty-canonical-json/1` or `MORIARTY-*-bounded-atomic/1` as accepted
successor domains.

Rules: objects, arrays, strings, booleans only. Reject JSON numbers, null,
duplicate keys, unknown keys, lone surrogates, non-ASCII object keys. Object
keys emit in increasing unsigned ASCII-byte order. Arrays keep schema order.
No whitespace. String escapes as B.1. No Unicode normalization. Decode then
re-encode must be byte-identical. Byte, depth, and node limits are checked
before hashing and before proof.

Integers on the wire are canonical decimal strings (A.1).

Host hash proposal: `SHA256(UTF8(domain) || 0x00 || preimage)`. Raw source and
raw proof bytes use `SHA256(bytes)` with empty domain. This is a **host
SHA-256 proposal**. It requires majority and native compatibility before freeze.
It is not accepted new crypto.

### F.2 Domain tags (proposed, unused by old profile)

| Name | Domain string | Preimage |
| --- | --- | --- |
| sourceHash | (empty) | original source UTF-8 |
| profileHash | `MORIARTY-SUCC-BOUNDS/0` | exact proposed bounds bytes |
| programHash | `MORIARTY-SUCC-PROGRAM/0` | canonical Core/manifest |
| claimRoot | `MORIARTY-SUCC-CLAIMS/0` | canonical requiredClaims |
| genesisHash | `MORIARTY-SUCC-GENESIS/0` | canonical GenesisBody |
| stateHash | `MORIARTY-SUCC-STATE/0` | canonical StateBody |
| actionHash | `MORIARTY-SUCC-ACTION/0` | canonical ActionCall |
| observationsHash | `MORIARTY-SUCC-OBS/0` | canonical ObservationSet |
| executionBodyHash | `MORIARTY-SUCC-EXEC-BODY/0` | canonical ExecutionBody (no sig, no auth digest, no proof) |
| exactPlanDigest | `MORIARTY-SUCC-EXACT-PLAN/0` | canonical unsigned ExactPlan |
| outcomeIntentDigest | `MORIARTY-SUCC-OUTCOME-INTENT/0` | canonical unsigned OutcomeIntent |
| authorityDigest | `MORIARTY-SUCC-AUTHORITY/0` | canonical Authority wrapper |
| proofContextHash | `MORIARTY-SUCC-PROOF-CTX/0` | canonical ProofContext |
| traceHash | `MORIARTY-SUCC-TRACE/0` | canonical Prepared body |
| acceptanceBind | `MORIARTY-SUCC-ACCEPTANCE/0` | intent digest + plan + obs + effects + liabilities + work + predecessor IDs |

Acyclic DAG: source -> program/claims/profile; executionBody ->
executionBodyHash -> ExactPlan; unsigned intent -> intentDigest -> Authority ->
acceptanceBind with observations, post, effects, work, predecessors. Proof is
after acceptanceBind. No cycle.

### F.3 Display obligations

Required visible fields: gross debit caps, fee caps, net goals (fees included),
recipients, debt caps versus transfer caps, locks, residual authority, recovery
rights, assumptions, network, deployment, principal, nonce, validity, spec,
policy, profile, Min versus Exact, dust, rate/price, continuation owner. Display
projection is a function of parsed canonical bytes only.

### F.4 Schema-inexpressible constraints

JSON Schema does not enforce: cap sums versus actual movements; unique compound
identities; hash preimage correspondence; relational validity windows versus
`obs.now`; attenuation subset; cumulative fill versus cap; conservation
equalities. Those live in this contract and `semantic-decisions.json`. The
schema must not claim them.

## G. SP02 and SP03 interfaces

SP02 consumes: domains A, lexical B, judgments C, construct list B.4, proposed
bounds C.2, sugar/Core split A.4. SP02 writes complete EBNF, lexer, successor
`bounds.json` (new file, not a mutation of the atomic file), frontend selected
by explicit profile hash. It keeps `src/frontend.ts` and atomic bytes stable.

SP03 consumes: Core constructors C.3, Eval C.1, state/effects D, reject codes
B.5, observation assumptions C.4. SP03 writes K configuration cells: control,
immutable pre, tentative writes, locals, obligations, ordered effects,
observations, authority, predecessors, remaining ordinary and recovery work,
result status. First slice: checked arithmetic, one guard, one staged update,
one transfer, then partial payment leaving residual debt (30 of 100 leaves 70).
K models Moriarty Core, not archived ZKIR.

Neither sprint invents a prover API. SP09 owns acceptance encoding freeze after
this proposal is reconciled and majority-approved.

## H. Open freeze gates

1. SP01.2 challenge-map reconciliation.
2. User-selected majority on majorityRequired decisions.
3. Native hash/domain correspondence.
4. Usability evaluation of source sugar (model opinion is not evidence).
5. Formal proofs of named claims (SP03 and later). Tests are not proofs.
6. Successor `bounds.json` registration (SP02), distinct from atomic bytes.

End of proposal.
