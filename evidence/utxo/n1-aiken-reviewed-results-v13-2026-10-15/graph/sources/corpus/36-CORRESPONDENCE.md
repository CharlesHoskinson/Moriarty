# CORRESPONDENCE: RR-03 candidate versus Core, the voted texts and S2

Reviewer: `claude-opus-5-5` (runtime-stated), fresh context, not V12, V13 or V14. Source
reading and hand arithmetic only. Core was read at
`/home/charl/Moriarty/.worktrees/utxo-pr54-currentness-20261015/crates/mori-core`; I cannot
confirm byte equality with the frozen-70 pins (closure `f47318a2…35ed`), except that `lib.rs`
counts 462 bytes by hand, the pinned size (IR-12).

## 1. Copied private Core bodies (fresh correspondence audit)

| Candidate | Core reference | Result |
|---|---|---|
| `keys.rs` `unhex` | `intent.rs:92-110` | Identical logic: length `2·len`, lowercase hex only, pairwise `from_str_radix`. Error code `mori::constraints::signature` instead of `mori::core::signature`; messages equal. |
| `keys.rs` `check_key(scheme, key_hex)` | `intent.rs:111-127` | Identical branch structure and k256 calls (`schnorr::VerifyingKey::from_bytes` on 32 bytes; 33 bytes, prefix 2 or 3, `ecdsa::VerifyingKey::from_sec1_bytes`). Only the argument form differs. |
| `keys.rs` `verify_signature` | `intent.rs:214-247` | Same order: `check_key`, `unhex(sig, 64)`, scheme branch, `Signature::try_from` / `from_slice`, `normalize_s().is_some()` refuses high-S, `Verifier::verify(message, sig)` on the unhashed message (k256 hashes with SHA-256 inside the verifier). The message is computed by the caller before `check_key` in `verify_constraints`; this changes only error identity when a key is invalid and the payload is oversized, which preparation already excludes. |
| `accounting.rs` `core_sums` | `intent.rs:286-336` | Identical per-limit loop: Transfer `from == signer` adds gross, `to == beneficiary` adds credit, `from == beneficiary` adds debit; Fee `from == signer` adds gross and fees, `from == beneficiary && from != signer` adds debit; obligations = closing rows with `asset == limit.asset && debtor == signer`, checked. Error code `mori::constraints::accounting` instead of `mori::core::authority`. |
| `accounting.rs` `net` | `intent.rs:337` | `credit.checked_sub(debit).and_then(|v| v.checked_sub(fees))`, identical. |
| `check.rs` `normalized_pre` | `runtime.rs:548-571, 628, 661` | Partial copy: empty state map → declared initials; balances sorted by `(asset, account)`; liabilities sorted by name. Core's validation (state completeness and types, duplicates, asset identities, row existence for transfer endpoints) is not copied. Non-authoritative: the commitment is compared again with `Execution.pre` after execution. A wrong normalization can only change error identity and reported cost. |

Reference range: the copies agree with Core on every input for which Core `check_effects`
returns `Ok`, which is the only range in which `accounting()` is reached inside
`steps_after_execution` (step 4 runs first). The CO-2 equivalence tests compare acceptance with
Core's public `prepare_intent` and `verify_intent` on valid, uppercase, short, long, all-`ff`,
all-zero, non-hex, cross-scheme, uncompressed-prefix and high-S inputs. CO-10 compares only on
the P-A effects at four boundaries; the copy is otherwise established by reading.

Native authority: zero. The only key operations in the candidate are in tests: secp256k1 keys
derived in memory from public labels (`SHA-256("rr03-owner-constraints public test
key/" || label)`) and in-memory signing. No key file, wallet path or stored material.

## 2. Core API and `verify_plan` order

| Obligation | Source | Verdict |
|---|---|---|
| A5 facts: network, anchor, program address, predecessor, nonce, per-account scheme and key | `types.rs` `StepFacts`, `AccountKey`; `plan.rs` `before_execution` | Implemented. An absent key refuses (`binding`, cost 0) for both profiles. |
| `/3` expected intent rebuilt from facts, program, source hash and commitment; only signer, targets, limits, recipients, input bounds and window from the record | `plan.rs:78-117` | Implemented. A field pre-check against the facts precedes the rebuild; it refuses exactly when the rebuilt intent would differ on those fields, so only the error identity changes (`binding` instead of a preparation code). |
| `/2` record: five facts, caller target, account key; then unchanged `prepare_intent` + `verify_intent` | `plan.rs:61-77` | Implemented. |
| No caller-supplied required signers, premises or ok flags | whole crate | None exist. `required_signers` is computed from `Execution.steps[i].statement`; premises are computed from effects. |
| Records strictly ascending by signer, one per signer, at least one | `plan.rs:152-161` | Implemented before any record check (cost 0). |
| Pre-execution (cost 0): preparation, signature, window, step 1 with the caller target, R6 committed rows against the normalized scenario, step 3 | `plan.rs:110-120` | Implemented. |
| Execution: unchanged `simulate_metered`; its error passes through with Core's cost | `plan.rs:168` | Implemented. |
| Post-execution (execution cost): R3 coverage; per `/3` record R6 against `Execution.pre`, R2 premises, step 4 (one `check_effects` call), 4b, 5, 5b, 5c, 6; per `/2` record unchanged `check_effects` | `plan.rs:169-208`, `check.rs:206-275` | Implemented in that order. |
| R1 one call | `check.rs:214-215` | `check_effects(signer, &all_inner_limits, effects, closing)`. |
| R3 classification by statement kind | `check.rs:20-61` | Transfer/Fee `from`; Liability-statement debtor; Discharge-statement creditor; zero amounts included. If step effects do not concatenate to `execution.effects`, both parties of every Liability effect are added (refusal-only). |
| R4 every provided record passes | `plan.rs:185-208` | Implemented (RS-3). |
| Present-zero versus absent duties | Core `Execution` | Unchanged Core semantics; present-zero rows add 0 to every sum; absence is visible in `duty_evolution` tests (warranty absent → present-zero). |
| Frame | Core `simulate` | Unchanged; untouched rows and the `legacy` duty keep their values (H1-A asserts). |

**Consumer trust (IR-06).** Core cannot authenticate any unsigned `verify_plan` input:
`program`, `source_hash`, `action`, `target`, `scenario` (state, inputs, balances,
liabilities), `commitment`, `facts`, `at`, `cap`. On Cardano S2 derives every one of them from
the ledger, the spent datum, the catalogue and the redeemer-bound proof statement. Host-side
`verify_plan` is a checker over consumer-authenticated inputs; host simulation never
authenticates ledger state. The candidate documents this for `StepFacts` only.

## 3. Arithmetic, re-derived

Notation: `MAX = 2^128 − 1`, `H = 2^127`.

**R5 floors (saturating is exact).** For a floor `f ≤ MAX` and true sum `S ≥ 0`:
`S ≥ f ⇒ min(S, MAX) ≥ f` because `f ≤ MAX`; `S < f ⇒ S < MAX` so `min(S, MAX) = S < f`.
Composition: `min(min(a, MAX) + b, MAX) = min(a + b, MAX)` for `a, b ≥ 0`, so the
candidate's `net.saturating_add(Σ saturating)` equals the saturated true position. Checked
addition ("overflow is an error") is not exact for floors.

**SX-03 (OV-1).** Seller receivables `55 + H + H = 2^128 + 55 > MAX`. Saturated: `MAX ≥ MAX`,
admit. Position `35 + sat(2^128 + 55) = MAX ≥ MAX`, admit. Escrow obligations `55 + 14 = 69`
(x1, x2 have debtor `someone`). Escrow exposure `37 + 69 = 106 ≤ 92 + 104`. Work 28. A checked
receivable sum overflows (the test asserts `None`). S2 sums unbounded: `2^128 + 55 ≥ MAX`.

**A4 cap (checked is exact; saturating is not).** For a cap `c ≤ MAX` and true sum
`S = gross + obligations` with each term `≤ MAX` after step 4: checked addition returns `None`
exactly when `S > MAX ≥ c`, which the candidate treats as exceeded; otherwise it compares `S`
exactly. Saturating addition at `c = MAX` returns `MAX ≤ MAX` for every `S > MAX` and admits.
Unbounded S2 refuses `S > MAX`. So Core-checked equals S2 for every input; Core-saturating would
fail open at `c = MAX` (author OS-01; V13 C5's "(saturating)" is the imprecise word, "exact" is
the operative one).

**OV-2.** Escrow 2^127 + 2, invoice H + 1, reserve H, plan `partial(H)`. Program: `H > 0`,
`H < H + 1`; Fee 2 → escrow H; Transfer H → escrow 0; Discharge → invoice 1; paid H; ensures
`1 == (H + 1) − H`. Escrow step 4: gross `H + 2 ≤ MAX`, fees `2 ≤ 2`, credit H, net `H − 2 ≥ 0`,
obligations `1 + H ≤ MAX`: passes. 4b: no reduced receivable of escrow. 5: `0 ≥ 0`.
5b: `(H − 2) + 1 = H − 1 ≥ 0`. 5c: `(H + 2) + (H + 1) = 2^128 + 3` → `None` → refuse
`exposure`, cost 28. `sat = MAX` would admit.

**OV-2b.** Reserve `H − 4`: obligations `1 + H − 4 = H − 3`; exposure
`H + 2 + H − 3 = 2^128 − 1 = MAX ≤ MAX`, admit. Seller: net H, receivables 1, exposure
`0 ≤ 0`. Work 28.

**Neutral values (OS-07).** After step 4, `gross ≤ gross_cap` and `obligations ≤ liability_cap`,
so `gross + obligations ≤ gross_cap + liability_cap ≤ exposure_cap` and 5c never refuses; when
`gross_cap + liability_cap ≤ MAX` the checked sum cannot overflow. `common::neutral` panics if
the neutral value does not exist; no frozen row reaches that (largest frozen pair 300,000 + 0).

**Fees and self-transfers.** H1 seller PTS limit (beneficiary seller): self-transfer 5 adds
gross 5, credit 5, debit 5, net 0; exposure `5 + 0 ≤ 5`; premise row PTS/seller = gross debit
5 (CO-4). InstallmentInvoice escrow: Fee 2 counts in gross and fees, and 5b's escrow view is
`(p − 2) + (90 − p) = 88` for every partial and `90 + 0` for finish (SX-02).

**Net-floor order (R5 last clause).** Core runs `check_effects` for every limit first, then 4b,
then `accounting()` (5, 5b, 5c), so `net` is `Some` whenever 5b is evaluated. S2 reports
`position` and `exposure` True when the record's `check_effects` is False; the gate is False
either way (OS-08).

**Debtor and creditor sides.** `obligations`: closing rows with `debtor == signer` (A4 and
`liability_cap`). `receivables`: closing rows with `creditor == signer` (step 5). Position:
closing rows with `creditor == beneficiary` (A2). 4b: Liability effects with
`creditor == signer && after < before` need a limit for the asset. Each matches the voted text
and S2 (`creditor_sum`, `debtor_sum`, `creditor_ok`).

**Typed 128-bit fields.** Rust: every amount, cap, floor, window and nonce is `UInt(u128)`,
decoded from a canonical decimal string (`ir.rs:19-31`), so out-of-range or `"035"` values fail
decoding (PF-7). Aiken: `limits_ok` checks the four `/2` caps; `limits_ordered` checks
`receivable_floor`, `position_floor`, `exposure_cap`; `bounds_ok` checks `min`/`max`;
`constraint_intent_json` checks `not_before ≥ 0`, `not_before ≤ not_after`, `is_u128(not_after)`;
`juint` halts outside u128 in every emitted value; effect amounts are range-checked by
`core.effect_view`.

## 4. Rust and Aiken step correspondence

| Step | Core (`check.rs`, `plan.rs`) | S2 (`constraints.ak`, `thread.ak`) | Agreement |
|---|---|---|---|
| Message | canonical JSON of `ConstraintIntent`, prefix `moriarty-owner-constraints/1\0`, u32 BE length, ≤ 65,536 | `constraint_intent_json` keys in canonical order at every level, `constraint_signing_message` | Equal key sets and order (checked against serde field names); CO-6 author vector shared by both tests |
| `pre_hash` | hex SHA-256 of canonical caller `PreCommitment` | SHA-256 of `pre_commitment_json` over the spent datum (state in catalogue order = name order, all custody rows, all liabilities incl. present-zero) | CO-5 frozen text at both |
| Domain fields and key | `StepFacts` | ledger/datum/catalogue (`own_hash`, `thread_id`, `own_ref`, `seq`, `acc.key`) | Same bindings |
| Step 1 | selected action's entry equals caller target | `targets_ok`: entry for `entry.name` equals `target_hash_v2(dom)`; all targets non-empty, ordered, core strings, catalogue actions | Equal except DV-2/DV-3 (refuse-only) |
| Step 2 | R6 committed rows; R2 premises | datum-derived by construction; premise rows built by S2 as gross debit | Equal by construction |
| Step 3 | own-action bounds present, typed, in bounds; all bounds prepared (action in targets, input exists with type, min ≤ max) | `bounds_ok`: order, action in targets, `min ≤ max` in u128, own-action bounds via the selected shape | Equal except DV-1 (other-action name/type unchecked; no effect on the step) |
| A3 limit order | `prepare_constraints` (`order`) | `limits_ordered` (index order = AssetId order by `well_formed`) | Equal |
| Step 4 | one `check_effects` call | `core.check_effects` port over inner limits (R1 port, unchanged) | Equal (P3 review compared the port line by line) |
| 4b | `creditor == signer && after < before` needs limit | `creditor_ok` | Equal |
| 5 | saturating Σ ≥ floor | unbounded Σ ≥ floor | Equal (R5) |
| 5b | `net` + saturating Σ(creditor == beneficiary) ≥ floor | `cr − d − f + creditor_sum(beneficiary)` ≥ floor, guarded by step 4 | Equal (R5) |
| 5c | checked `gross + obligations` ≤ cap, overflow = exceeded | unbounded `g + debtor_sum` ≤ cap, guarded by step 4 | Equal (OS-01) |
| 6 | every Transfer/Fee `from == signer` has `to` in its asset's set | `recipients_ok` (also rule order, rule has limit, account order) | Equal except DV-2 |
| Window | `at ∈ [not_before, not_after]` | whole finite validity interval inside the window | S2 stricter by interval (as `/2`) |
| R3, R4 | computed; every record passes; strict order | `required_signers`, `authority_shape`, every checked record | Equal |
| `intents_digest` | off-chain: blake2b over `lp(signer) ‖ sha256(message)` | same for both profiles | Equal |

## 5. Divergences

| Id | Direction | Status |
|---|---|---|
| DV-1 | S2 admits a record whose other-action bound is mistyped; no effect on the step | Permitted by voted R7 |
| DV-2 | S2 halts on strings outside the escape-free alphabet | Inherited from `/2`; documented by V13 beside R7; refuse-only |
| DV-3 | S2 requires every target action to be a catalogue action | Refuse-only; outside the voted R7 text (IR-01) |
| Error codes | Copies use `mori::constraints::*` | Verdict-neutral |
| Pre-check order in `/3` binding | `binding` reported before preparation codes | Verdict-neutral |
| Cost | Pre-execution 0; Core attempts; post-execution full cost | As frozen design table §6 |
| Window | Interval versus point | As `/2` |

## 6. `/2` behaviour preservation

- `intent.ak`, `core.ak`, `claim.ak`, `groth16.ak`, `bytes.ak`, `catalogue.ak`, `c2.ak` and every
  R1 test file are hash-equal R1 copies (root materialization). `types.ak` only appends types and
  `SettleV3`. `s2.ak` adds one `SettleV3` branch to the same `thread.settle`.
- In `thread.ak` (compared with the R1 file in full), the R1 `Settle` path reaches the RecV2
  branch, which is the R1 per-intent computation verbatim; `signers`, `required`,
  `authority_shape`, `scenario_hash`, `dom`, `intents_digest`, statement, continuation, payouts
  and every other component are R1 text. The only R1-visible change is eight extra components
  that are True for `/2` records.
- Every R1 settle test asserts `false_parts`, digests or messages, so its expectation is
  unchanged by True components. The 146 historical CEK agreements (ADOPTION A1) are on the old
  `thread.ak`; they must be re-observed. Expected E-2 membership: 178 tests (146 R1 + 20 + 12).
- Cost and size change for `/2` steps too (IR-09). R1 S2 was already 30,764 B against an
  observed `maxTxSize` of 16,384 B; nothing here claims Preprod fit.

## 7. Syntax and API reading (nothing compiled)

No predictable compile failure found. Points that need E-1/E-2 to confirm, with fallbacks in
`proposals/MINIMAL-REPAIRS.md` §2:

- Aiken `let StepV3 { …11 fields… } = step_of(r)`: single-constructor record pattern in `let`
  (R1 compiles `let Pair(a, b) = p`).
- Aiken `fail @"not a settle redeemer"` in a `when` branch: R1 `core.ak:285` uses the same form.
- Aiken `transaction.placeholder`, `list.repeat`, `interval.between`, `assets.lovelace_of`:
  stdlib v2.x names, not checked against the 38 pinned files.
- Rust `ecdsa::SigningKey::from_bytes(&secret.into())` (`From<[u8; 32]>` for the 32-byte
  `GenericArray`), schnorr `Signer<Signature>`, `-*sig.s().as_ref()` with
  `Signature::from_scalars`, serde `deny_unknown_fields` on the tagged enums (same pattern as
  Core `Effect`/`Value`), closures `before`/`after` that capture `Copy`/borrowed data and are
  reused with `map_err` (same pattern as Core `verify_and_simulate_metered`).
- `aiken build` type-checks every `lib/` module, so a failure in `constraints_fixtures.ak` would
  also block compiling S2.

## 8. Cargo dependency closure

Direct registry dependencies: `serde 1.0.229` (derive), `serde_json =1.0.151`, `k256 =0.13.4`
(`schnorr`, `ecdsa`), all present in the Core closure with the same features; no
dev-dependencies. The transitive set is mori-core's plus mori-lexer's registry set; mori-core's
dev-dependencies (proptest, git `midnight-*`) are not resolved for a non-member path dependency,
and its workspace `[patch]` is ignored. Reuse of the known reviewed registry set is plausible but
unverified here: I did not read that lock. Conditions: the lock contains `serde_json 1.0.151`
exactly (or the requirement is relaxed to the caret form Core uses), no package is added, sources
are vendored, the toolchain is set explicitly (the crate lies outside `<root>/moriarty`, so
`rust-toolchain.toml` does not apply), and no ancestor `.cargo/config*` exists. No install or
fetch is proposed.
