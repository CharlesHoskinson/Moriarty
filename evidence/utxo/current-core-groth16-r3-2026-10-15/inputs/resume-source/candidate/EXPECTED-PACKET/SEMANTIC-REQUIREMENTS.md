# Semantic requirements for the current-Core R1CS compiler (frozen before implementation)

Status: **source author's own derivation, specified-only.** Author: Claude Opus 5.5
(`claude-opus-5-5`), the compiler source author. This is **not** an independent
implementation oracle. No parser, evaluator, compiler, cargo, rustc, test, SDK, VM,
key, proof or ledger command ran. Fresh non-author held-out vectors and hostile
witness mutations are still required before any execution is scored.

Source tree read: `/home/charl/Moriarty/.worktrees/utxo-pr54-currentness-20261015`,
HEAD `6c7b1fda` (parent `9c63048a`). `crates/mori-core` has no uncommitted changes in
that checkout; the uncommitted files are all under `crates/mori-cardano`. Per-file
digests are pinned in `../SOURCE-PINS.json`. Line references below are to that tree.

Labels: **[SRC]** read from current source; **[DER]** my derivation from [SRC];
**[ORA]** taken from a frozen non-author oracle packet (pins in `REUSED-FIXTURES.json`).

## A. Admission and identity

- **SR-01 [SRC] Profile routing.** `mori_core::language` routes `moriarty/1` and
  `moriarty/2` only (`parser.rs:352-377`). `mori_core::parse` admits `/1` and runs
  `validate` before returning (`parser.rs:157-159`). `validate` refuses any program
  whose language/semantics differ from `moriarty/1`/`moriarty-core/1`
  (`runtime.rs:120-126`) or that carries `Await` (`runtime.rs:237-239`).
  `stage::require_core1` repeats both refusals (`stage/profile.rs:10-23`). `/2` is
  admitted only by `stage::load` (`stage/mod.rs:71-88`).
- **SR-02 [SRC] Raw-source vs Program identity.** Source identity is SHA-256 of the
  raw UTF-8 source bytes (`mori-cardano/src/export.rs:27`,
  `source_sponsor.rs:153`). Program identity is SHA-256 of
  `intent::canonical(program)` (recursively key-sorted compact JSON;
  `intent.rs:72-88`, `export.rs:22`, `stage/digest.rs:20-22`). The two differ and
  must be pinned separately.
- **SR-03 [SRC] Validation facts the compiler may rely on** (`runtime.rs:120-322`):
  unique non-reserved identifiers; inputs/lets never shadow states, inputs, lets or
  asset names; every expression reference points to an earlier node (acyclic DAG,
  depth ≤ 64); every statement is well typed; transfer/fee/liability amounts are
  UInt; predicates are Bool; `Next` matches its state's type; asset identities are
  unique; every `liability_present/amount` name has a source `Liability`
  introduction with one identity (`runtime.rs:325-372`).

## B. Values and expression semantics

- **SR-04 [SRC] Types.** `UInt` is `u128` atomic units with canonical decimal JSON
  (`ir.rs:11-32`); `Bool` is a JSON boolean (`ir.rs:33-43`).
- **SR-05 [SRC] Name environment.** The environment starts as pre-state values plus
  the action inputs (`runtime.rs:882-883`). `Let` adds a binding (`runtime.rs:716-719`).
  `Next` replaces both the snapshot state and the environment binding, so later
  `Name` reads see the new value (`runtime.rs:735-746`).
- **SR-06 [SRC] Strict evaluation, per-statement memoisation.** `Binary` evaluates
  `lhs` then `rhs` unconditionally, including `And`/`Or` (`runtime.rs:451-453`,
  `476-477`). **There is no short-circuit.** Each statement evaluation creates a
  fresh cache (`runtime.rs:399-406`); a cache hit returns without re-evaluation or
  charge (`runtime.rs:423-425`). A node shared by two statements is re-evaluated in
  the second statement against the then-current environment and snapshot.
- **SR-07 [DER] Success condition of an expression.** Because every node reachable
  from a statement root is evaluated, a successful statement requires that **every**
  reachable node evaluates without error, independent of Boolean values. Evaluation
  order matters only for which error is reported.
- **SR-08 [SRC] Checked arithmetic.** `Add` = `checked_add` (refusal
  `mori::core::overflow`), `Sub` = `checked_sub` (`mori::core::underflow`), `Mul` =
  `checked_mul` (`mori::core::overflow`) (`runtime.rs:455-469`). [DER] Success of
  `Mul` requires the mathematical product of two values < 2^128 to be < 2^128; the
  256-bit intermediate must not be reduced modulo any field prime.
- **SR-09 [SRC] Comparisons and Booleans.** `Eq`/`Ne` compare equal-typed values
  (`runtime.rs:470-471`); `Lt/Le/Gt/Ge` order UInt (`runtime.rs:472-475`); `And`,
  `Or` are Boolean conjunction/disjunction after both operands are evaluated;
  `Not` negates a Bool (`runtime.rs:447-450`).
- **SR-10 [SRC] Duty reads.** `liability_present(n)` is true iff a row named `n`
  exists in the **current** snapshot, including a present row of amount 0;
  `liability_amount(n)` is that row's amount or 0 when absent (`runtime.rs:430-441`).

## C. Statement semantics (successful path)

- **SR-11 [SRC] Positional predicates.** `Requires` and `Ensures` are evaluated at
  their position, against the current environment and snapshot; any value other
  than `true` refuses with `requires`/`ensures` (`runtime.rs:721-734`). An `Ensures`
  before a `Next` sees the old value.
- **SR-12 [SRC] Next.** Effect `State{name, before, after}`; repeated `Next` of one
  state is legal and each produces its own effect (`runtime.rs:735-746`).
- **SR-13 [SRC] Emit.** Effect `Emit{name, value}` of either type; no snapshot change
  (`runtime.rs:747-754`).
- **SR-14 [SRC] Transfer and Fee** (`runtime.rs:755-786`, `move_funds` `668-700`).
  The amount is evaluated first. Debit check `from.amount ≥ amount`
  (`Insufficient balance`) precedes the credit check. When `from == to` (same asset
  and account) the balance is unchanged **but the debit check still applies**. A
  credit that exceeds u128 refuses (`overflow`). An effect is produced even for
  amount 0. Fee and Transfer have identical balance semantics and differ only in
  effect kind.
- **SR-15 [SRC] Liability** (`runtime.rs:787-836`). Amount evaluated; if a row with
  that name exists its identity must equal the statement's; `after = before +
  amount` (checked, `overflow`); an absent row is created present with `after =
  amount`, **even when amount is 0** (present-zero). Effect `Liability{…, before,
  after}`; for a created row `before = 0`.
- **SR-16 [SRC] Discharge** (`runtime.rs:837-860`). The named row must exist
  (`Unknown residual liability`); `after = before − amount` (checked,
  `Discharge exceeds residual liability`); the row is kept at 0 (present-zero).
  Effect uses the row's own identity.
- **SR-17 [SRC] Atomic outcome.** `simulate` either returns the complete
  `Execution{pre, post, steps, effects}` or an error with no committed effects
  (`runtime.rs:865-912`).

## D. Scenario premises

- **SR-18 [SRC] Scenario normalisation** (`runtime.rs:528-667`): ≤ 4,096 balances and
  ≤ 1,024 liabilities; `(states + balances + liabilities + Liability statements + 1)
  × (statements + 2) ≤ 65,536`; state map empty (use initial values) or complete and
  typed; inputs exactly the action's, typed; every balance asset declared, accounts
  1..1024 bytes and unique per asset; **every (asset, account) named by any Transfer/Fee
  of the selected action must have an explicit balance**; balances sorted by
  `(AssetId, account)`, liabilities by name; a liability whose name has a source
  introduction must carry that identity.
- **SR-19 [DER] Frame.** Rows not named by the action (untouched balances, untouched
  liabilities) are carried unchanged from `pre` to `post`.

## E. Logical work

- **SR-20 [SRC] /1 logical cost.** `simulate_metered` charges one unit per statement
  dispatch (`runtime.rs:710`) and one unit per uncached node visit, charged before the
  node's children (`runtime.rs:426-428`), under a caller-trusted unsigned cap
  (`logical_cost.rs:33-55`). `Meter::charge` refuses when `used + 1 > cap`
  (`policy.rs:37-53`). [DER, confirmed by `stage/cost.rs:1-8,21-68`] A successful
  `/1` action uses exactly `W = Σ_statements (1 + |distinct nodes reachable from the
  root|)`, a value-independent constant, so success under cap `c` iff `W ≤ c`.
  Circuit cost is a different quantity and is not inferred from `W`.

## F. Owner intent (separate layer)

- **SR-21 [SRC] check_effects** (`intent.rs:252-350`) computes per-asset gross debit,
  fees, credit, debit and closing obligations from the effect list and closing
  liabilities. It is an owner-authority check outside `simulate`; it is not part of
  the transition relation. All of its inputs are the ordered effects and post
  liabilities.

## G. Staged `/2` (plan inputs only; first slice refuses)

- **SR-22 [SRC]** `stage::load` = parse_v2 → require_core2 → validate_v2 → plan →
  plan invariants → cost fill → cost invariants → program/plan hashes
  (`stage/mod.rs:71-88`). Segment `j` runs from entry site `j` to the next await,
  paying that await's entry charge 1; a resume first evaluates the site guard as its
  own Step with the unchanged boundary (`runtime.rs:950-976`). A false guard is
  `NotAdvanced` with no segment (`stage/admit.rs:103-121`). Start refuses when
  `plan_total > cap` (`stage/cost.rs:164-179`); resume requires
  `committed == committed_before[site]` and `committed + cost ≤ effective cap`, the
  CLI cap never raising the stored cap (`stage/cost.rs:184-217`). `residual` lists
  post liabilities with positive amount (`stage/mod.rs:179-187`). Captures are
  inputs, lets and earlier event binders, in plan order (`stage/plan.rs:56-105`).
  R7 canonical boundary predicates are in `stage/decode.rs:94-227`.

## H. Target projection facts (not compiler semantics)

- **SR-23 [SRC]** The atomic Cardano layout refuses `Emit` program-wide
  (`mori-cardano/src/profile.rs:11-13`); the staged UNIT codec admits Emit
  (`staged_projection.rs:192-232`). Projection bodies order states by name, framed
  cells (ScriptCustodied × assets) by `(asset, account)`, and duties by name with
  absence exactly `(false, 0)` (`projection.rs:46-65,338-375`). KeyHeldExternal
  balances are supplied as unauthenticated premises (`projection.rs:415-430`).

## Circuit requirements derived from the above (CR)

| ID | Requirement for an accepted assignment | From |
|---|---|---|
| CR-01 | Every UInt wire is an integer in [0, 2^128) proven by 64-bit limb bit decomposition; every Bool wire is 0/1 | SR-04 |
| CR-02 | Add/Sub/Mul outputs equal the exact integer result and the success relation is unsatisfiable when Core would overflow/underflow; no field reduction stands in for u128 | SR-08 |
| CR-03 | Every node reachable from each statement root is lowered and constrained; no short-circuit gating | SR-06, SR-07 |
| CR-04 | Within one statement a node is lowered once; across statements it is lowered again against the current environment | SR-06 |
| CR-05 | Name reads, Let, Next and duty reads follow SR-05/SR-10 statement by statement | SR-05, SR-10 |
| CR-06 | Requires/Ensures predicates are constrained true at their position | SR-11 |
| CR-07 | Ordered effects are bound in statement order with exact kinds, constants and values | SR-12–SR-16 |
| CR-08 | Transfer/Fee enforce debit-before-credit success, self-transfer debit check, and credit overflow | SR-14 |
| CR-09 | Duty cells carry (present, amount) with absent ⇒ amount 0; Liability sets present; Discharge requires present and keeps present-zero | SR-15, SR-16 |
| CR-10 | Post state, post balances and post duties are bound; untouched represented cells equal pre | SR-17, SR-19 |
| CR-11 | Logical work `W` is a relation constant computed from the IR; with a bound cap the circuit enforces `W ≤ cap` | SR-20 |
| CR-12 | Program hash, profile, relation version, action, statement shape and compiler build identity are bound as constants in-circuit; source hash is recorded separately | SR-02 |
| CR-13 | Public encoding has exact slot count; each limb < 2^64 and each Bool ∈ {0,1} is forced by constraints; digest halves < 2^128 | CR-01 |
| CR-14 | Rejected inputs have no satisfying assignment; refusal semantics are declared, not proven, in the first slice | SR-17 |
| CR-15 | Unsupported constructs are refused at compile time, never host-computed | task contract |
