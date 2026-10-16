# REPORT: independent design and oracle review of the RR-03 constraint profile and its addenda

Reviewer: `claude-opus-5-5` (runtime-stated). Requested effort is high; I cannot observe the
actual effort. This is a fresh context. I wrote none of P3S1, P3S1-R1, ALTERNATIVE-PLANS-RR03,
the V11 review, MR-3, P3R1-AP1(-r), the freeze packet (V12), its expectations, or any fixture.
Every author, reviewer and voter in this lineage is the same model family, so my independence
is at the context level only. Seat label used below: **V13** (my proposal; the root assigns seats).

Tools: Read, Write and Edit only. I ran no Bash, Grep or Glob. I listed no directory, hashed
nothing, and ran no parser, interpreter, compiler, SDK, crypto, network, Git, key, proof, wallet
or ledger action. I started no subagent. I opened files only through named paths, manifests and
explicit references. I did not open any provider, private, stderr, credential, prompt, process,
final or wrapper file, any live A1 draft or any mutant output. Every number below is my hand
arithmetic over source text. None of it is an actual result. Every hash I cite was copied from a
manifest; I verified none of them myself.

## 0. Separate axes

| Axis | Result |
|---|---|
| **PREDICATE** | **Not yet complete. Yes once the root records five decisions.** The frozen design, MR3-R and A1–A3 are correct as far as they go. Every oracle number I re-derived matches. Before an implementer can build v1 without guessing, the root needs to record: (1) the A1/A2/A3 tallies; (2) a Core step-facts binding for `verify_plan` (my AR-01 / MR3-A5), without which records authenticate themselves at Core and eight frozen refusal rows cannot be produced there; (3) a decision on a debtor-side field (AR-02 / MR3-A4), which has to be settled before v1 for the same reason as A2's field; (4) the arithmetic, normalization and permitted-divergence readings (AR-03/04/05 / MR3-R5). None of these changes a frozen expectation byte. |
| **VOTES** | **P3R1-AP1-r: AGREE. MR3-R: AGREE. MR3-A1: AGREE. MR3-A2: AGREE. MR3-A3: AGREE.** All are literal votes; see `VOTES.md`. I propose three new texts (MR3-R5, MR3-A4, MR3-A5), each carrying only my vote. |
| **FZ-01..07 (security judgments)** | All seven **confirmed**. FZ-02 is extended by its debtor-side dual (AR-02). FZ-07 is only partly confirmed: I did not read `build_identity.rs`. §4. |
| **New findings** | AR-01 (Medium): Core does not bind domain or key, and the expected-intent reconstruction for `/3` records is unspecified. AR-02 (Low/Medium): debtor overpayment / under-discharge. AR-03 (Low): step 5/5b arithmetic is not pinned. AR-04 (Low): committed-row existence and pre-execution normalization. AR-05 (Low): S2 cannot check other-action bound metadata. AR-06, AR-07, AR-08 (Info). §5. |
| **EXPECTATIONS** | **Consistent on every value I re-derived**: 5 positive plans, 1 bad plan, all 70 boundary/variant rows and the 6 second-program cases. I found two **layer inconsistencies** (OC-1: eight RP rows cannot be produced at Core under the specified API; OC-2: the TG-8 refusal reason depends on the scenario). The correction proposals keep the original bytes unchanged. I also give 19 supplementary rows (my lineage, root adoption needed) in `FINDINGS.json`. |
| **CONTRACT** | The smallest concrete v1 field and check contract is in §8. **No implementation** and no execution authority. |
| **ADOPTION / ACCEPTANCE** | None by me. RR-03 stays High and open. §9. |

## 1. Startup and routing

I read the installed `develop/SKILL.md` and `references/execution-focus.md`. From the
full-integration checkout I read `AGENTS.md`, `docs/MORIARTY-PRODUCT-CONTRACT.md` and
`docs/audits/2026-10-14/SECURITY-ROUTING.md`. All security judgments in this task belong to
Opus. Native and SDK authority is zero. Consequential design decisions need two agreeing
substantive votes on identical text. The product is permissionless: the consumer's account and
key map is asset-owner authorization, not a developer whitelist. "Independently valid
alternative plans within the same signed constraints remain admissible" is the requirement under
test. The root refreshed guarded status. I did not run the CLI (tool scope).

## 2. Independent derivation first, then agreement grading

I derived these values from `installment-invoice.mori`, its IR (`expected-program.json`, one
node per occurrence, ids e26–e58), `H1-settle.mori`, Core `runtime.rs` (charge 1 per statement,
then 1 per uncached node before its children, with no short-circuit) and `intent.rs:252-350`.
Only after deriving them did I open the non-author vectors.

| Item | My derivation | Graded against | Agreement |
|---|---|---|---|
| partial schedule | 6,2,4,2,2,2,4,6 = 28 | `invoice-partial.json` `logical_schedule`; freeze | equal |
| finish schedule | 2,2,2,2,4,2,6 = 20 | `invoice-finish.json` schedule (a different pre state; work is static); freeze | equal |
| settle schedule | 6,4,2,2,2,2,4,4,7,3 = 36 | HELDOUT `H1-P1` `work_used`; freeze | equal |
| PR-0 / PR-90 | 6 (statement 0) / 12 (statement 2) | freeze | equal |
| WK-1 cap 19 / WK-2 cap 27 | refused at statement 6, used 19 / statement 7, used 27 | freeze | equal |
| P-A effects, Sol post | Fee 2, Transfer 35, invoice 90→55, paid 0→35; 93/72/16/48/41 | `invoice-partial.json` | equal |
| P-A escrow / seller | 37/2/35/0/33/69 · net 35, receivable 55 | freeze, ALT-PLANS | equal |
| P-B | Transfer 90, invoice 90→0 present-zero, paid 0→90, opened t→f; escrow 90/0/90/0/90/14 | freeze, ALT-PLANS | equal |
| N-C | escrow net 18 < 30, seller net 20 < 30; work 28, cost reported after execution | freeze | equal |
| P-A89 / P-A32 / P-A40 | gross 91/34/42, net 87/30/38, obligations 15/72/64; custody 62M/119M/111M | freeze | equal |
| Cardano lovelace | (109+41)·10^6 + 3·10^6 = 153M; P-A 116M; P-B 63M; payouts equal the custody decrease | freeze | equal |
| H1-A | Transfer 248750, Fee 1250, PTS self 5, warranty created 0, released 248750, rounds 1, open t→t; post 750000/1253/248750 | HELDOUT `H1-P1` (effects, post, pre, work) | equal |
| H1-B/C/D/PR/RF | ADMIT / seller obligations 10001 > 10000 / seller net 199999 < 200000 / statement 0, work 6 / floors 10000 and 10001 | freeze | equal |
| CF, LM, RC, IB, TG, PC, MX, RS rows | per-row hand checks (see `FINDINGS.json` `oracleAudit`) | freeze | equal (layer notes below) |
| DX-1 claim | every check is a monotone box, so the hull of admitted plans contains (30,89) | freeze | proof confirmed |
| PC-CUSTODY canonical string | key order, adjacent `Value` tag, `scale` number, compact separators | freeze | equal to `intent::canonical` rules |

Classification: these are derivative agreements within one model family over the same source
text. The non-author Sol vectors and H1-P1 are the only independent inputs, and every value I
checked against them agrees.

## 3. Topic-by-topic assessment

- **Design binding.** At Cardano, S2 derives every domain field and the key from the ledger, the
  datum and the catalogue (`thread.ak:570-607`, `intent.ak:218-274`), so `/3` binds them.
  At Core, the specified `verify_plan(program, source_hash, action, target, scenario,
  commitment, records, at, cap)` has no source for network, anchor, program address,
  predecessor, nonce or the account's key. Its `SignerRecord::V3` carries only the signed
  intent, and the expected intent for `verify_constraints` is never defined. A record signed
  under a foreign key, for another thread, or after a Revoke therefore verifies at Core.
  **AR-01**, fixed by MR3-A5.
- **Required signers.** R3 equals ABI-V2 §7.3 rules 1–2. S2 already classifies by the shape's
  `discharge` flag, which comes from the catalogue-pinned statement kind
  (`thread.ak:296-313`). Core can classify exactly from `Execution.steps[i].statement`, because
  each Liability or Discharge statement emits exactly one Liability effect
  (`runtime.rs:787-860`). A zero-amount accrual (H1 warranty 0→0) still requires the debtor.
  Confirmed.
- **Mixed `/2` + `/3`, `/2` unchanged.** The prefixes differ from byte 15 onward, so the two
  message spaces are disjoint. R4 allows one record per signer. MX-1..3 hold. Under MR3-A5, a
  `/2` record in `verify_plan` must also carry the caller's target and the step facts.
  Otherwise a validly signed `/2` record for another verifier key passes at Core (supplementary
  TG-9).
- **Every provided record verified.** R4 is correct and matches S2 rule 5. RS-3 discriminates.
- **Per-action targets and inputs.** Step 1 compares only the selected action's entry. This is
  correct, and TG-1 shows it. FZ-01 is real (IB-1). A1 is the minimal fix. A residual remains
  at S2, which cannot check the input name or type of a bound on another action (AR-05).
- **Signed commitment, unsigned premises.** R2 is right. Premise = gross debit cannot cause an
  insufficient-balance failure. Gross debit sums every debit, so the premise is never smaller
  than what the account pays out. Two gaps remain. First, "must equal the pre rows with the
  same `(asset, account)`" has to mean that the row exists (PC-9). Second, the pre-execution
  comparison needs Core's normalization: an empty scenario state map means initial values, and
  `scenario_snapshot` is `pub(crate)` (PC-10). **AR-04**.
- **Beneficiary net (fees included) plus closing receivables, and overflow.** A2's 5b reuses
  `check_effects`'s `net = credit − debit − signer fees` (`intent.rs:286-337`). It is defined
  whenever step 4 has passed. Neither step 5's nor 5b's sum pins its overflow behaviour. Rust
  `u128` checked arithmetic would refuse plans that the literal ≥ comparison admits, and Aiken's
  unbounded integers would admit them, so Core and S2 would diverge (OV-1, reachable with two
  2^127 receivables). **AR-03**. Reading R5 pins exact comparison: saturating at Core, unbounded
  at S2. That is the literal meaning of "at least", so it supports A2 as worded. The escrow's
  view of the seller's position includes the escrow's own fees: every partial gives
  (p−2)+(90−p) = 88 and finish gives 90 (supplementary SX-02).
- **Complete liability, zero and residual duties.** `closing` is the complete post list at Core
  (`execution.post.liabilities`) and at S2 (`new_liabilities`). Present-zero rows count as zero
  receivable. The reserve residual of 14 blocks Close. Duty frames are preserved. Confirmed.
- **Position scope; creditors without limits.** A2's 4b closes LM-9. 5b closes H71 and DX-1
  for the creditor. The **debtor-side dual stays open**. The escrow can be made to pay 89
  while only 30 is discharged (forged H72, or DX-1's (89,30)): every MR-3 + A2 check passes,
  and no parameter choice that admits (30,30), (89,89) and finish excludes it. **AR-02**.
  MR3-A4 (`exposure_cap`: gross + obligations ≤ cap) closes it. Together with A2, every partial
  of InstallmentInvoice is then forced to pay exactly what it discharges, within the fee slack.
  Info items: position is beneficiary-relative, so a creditor who routes cash to a separate
  payee cannot express a position on its own receivable, and receivables count at face value
  from any debtor (**AR-06**).
- **Canonical encoding, unknown tags, duplicates, refusals.** `json::parse` refuses duplicate
  keys and trailing data. `UInt` refuses leading zeros and non-strings. `deny_unknown_fields`
  and serde enum tags refuse unknown fields and variants. The verifier compares structures and
  then hashes `canonical(expected)`, so input formatting never matters. A3 pins the remaining
  list orders. The struct definitions already fix everything else (nested `limit` object, no
  skipped empty arrays); §8 C2 states these pins for interop. Strings outside S2's safe alphabet
  are refused only at S2. That is fail-closed and documented (DV-2).
- **Same authorization: partial(35) and finish admitted, partial(20) refused.** Confirmed. Both
  plans use byte-identical `/3` records, because `pre_hash` covers only datum rows and those are
  identical for both plans.
- **Held-out second program.** All 6 cases confirmed, including R1's one-call requirement
  (LM-3) and the seller's self-transfer recipient rule (RC-5).

## 4. Security judgments on FZ-01..07 (author findings)

| Id | Judgment | Severity | Reason |
|---|---|---|---|
| FZ-01 | **Confirmed** | Medium (product); safety none | IB-1 traced: name-only bound and "present" refuse `finish()`. The failure is closed, but it defeats alternative plans across actions as soon as any bound exists. A1 fixes it. |
| FZ-02 | **Confirmed, extended** | Low/Medium | H71-R and DX-1 traced. The dual (debtor overpays) is AR-02. Both are defence in depth under a forged proof, and program-valid only for programs that decouple discharge from payment. |
| FZ-03 | **Confirmed** | Low | Per-limit calls only fail closed (each call demands authority for every asset touched). R1 is correct. |
| FZ-04 | **Confirmed** | Low | Orderings that differ between tools produce signature mismatch. This is liveness only. |
| FZ-05 | **Confirmed** | Low | PC-2 traced: buyer 93, fees 14, seller 13 ≠ gross debit 0. |
| FZ-06 | **Confirmed** | Info | Core has no required-signer function. R3 resolves this. |
| FZ-07 | **Confirmed in part** | Info | I verified that `intents_digest` is a free `ContextV2` field (`abi2.rs:40-52`) and that C2 and the statement layout are untouched. For the `lib.rs` → build-identity chain I rely on the V11 reading of `build_identity.rs`, which I did not read. |

## 5. New findings (counterexample and minimal fix)

| Id | Sev | Finding | Counterexample | Minimal fix |
|---|---|---|---|---|
| AR-01 | Medium | `verify_plan` has no consumer-authenticated step facts, and the expected `/3` intent is undefined. Records authenticate themselves at Core. | KY-1 (seller record under a foreign key), PG-1 (signed over another program hash), RP-2..6 and RP-8 (after Revoke the datum rows are unchanged, so `pre_hash` still matches), TG-9 | MR3-A5: `StepFacts` + expected-intent reconstruction |
| AR-02 | Low/Med | No debtor-side combined bound. Any parameters that admit the intended plans also admit overpayment. | H72 (Fee 2, Transfer 89, invoice 90→60): escrow 91+74 = 165; DX-1 (89,30) | MR3-A4 `exposure_cap` (decide before v1) |
| AR-03 | Low | Step 5/5b sums are not pinned: Rust checked u128 vs Aiken unbounded. | OV-1: receivables 55 + 2·2^127, floor 2^128−1 | R5: exact comparison |
| AR-04 | Low | Committed-row existence, and normalization before execution | PC-9 (commitment holds spectator, scenario omits it); PC-10 (scenario state `{}`) | R6 |
| AR-05 | Low | S2 cannot check the input name or type of another action's bound (A1). Core refuses where S2 admits. | IB-7: Bool bound on `(partial, amount)` with plan `finish()` | R7: a permitted, documented divergence (DV-1); the bound has no effect on the step |
| AR-06 | Info | Position is beneficiary-relative and debtor-agnostic | separate-payee creditor; credit substitution (bounded by `/1` source-literal identities) | none for v1; record it |
| AR-07 | Info | MR3-R's R4 text leaves out "strictly ascending by signer". The `intents_digest` order depends on it. | — | S2 rule 5 already enforces it; the §8 contract states it |
| AR-08 | Info | TG-8's open-plan refusal is `mori::core::input` (cost 0) on CARDANO-partial, not `requires` | — | Correction proposal OC-2 |

## 6. Oracle consistency and correction proposals (frozen bytes unchanged)

- **OC-1 (layer).** RP-2, RP-3, RP-4, RP-5, RP-6 and RP-8 predict "REFUSE signature" or
  "REFUSE (nonce/predecessor)". The predicate is exact at CARDANO, where S2 derives the facts.
  At CORE it is producible only if MR3-A5 is adopted. Without A5, Core `verify_plan` admits
  RP-8 (Revoke changes neither the state, the rows nor the liabilities) and has nothing to
  compare against in RP-2..6. RP-1's Core refusal is still produced, through `pre_hash`.
  **Proposal:** test these rows at both layers if A5 is adopted, at CARDANO only otherwise.
  Lineage: freeze V12 → this review V13.
- **OC-2 (precision).** TG-8: with the CARDANO-partial scenario, an `open` plan is refused by
  `scenario_snapshot` input-count validation (`mori::core::input`, cost 0, `runtime.rs:572-577`).
  The `requires not opened` refusal (statement 0, work 3: Not + Name + dispatch) needs a
  scenario with `open`'s inputs and buyer/escrow/fees rows. The verdict (REFUSE) stands.
- **Cost pins (no conflict).** PC-4, PC-6, PC-7 and PC-8 are refused before execution (cost 0)
  under the specified order. PC-2 and PC-5 are refused after execution (cost 28).

The 19 supplementary rows (SX-01..SX-19) are in `FINDINGS.json`, derived by hand by me. They
apply only if the root adopts them and the matching text is tallied.

## 7. Votes (summary; literal texts in `VOTES.md`)

P3R1-AP1-r **AGREE**. MR3-R **AGREE** (I read R2's "equal" as requiring the row to exist; R6
makes that explicit). MR3-A1 **AGREE**. MR3-A2 **AGREE** (exact semantics, per R5).
MR3-A3 **AGREE**. New proposals, one vote each: **MR3-R5**, **MR3-A4**, **MR3-A5**.

## 8. Smallest concrete v1 implementation contract (follows the votes; no authority)

**C0 Decision gates.**
- **Base:** MR-3 verbatim + MR3-R. A1, A2 and A3 are each implemented only if the root's tally
  shows two agreeing substantive votes.
- **A2 and A4:** they add signed fields, so both must be decided before the first byte of v1.
  If A4 is not tallied, v1 omits `exposure_cap`, and the debtor dual becomes a recorded residual
  that only `moriarty-owner-constraints/2` can close.
- **A5:** if A5 is not tallied, the Core `verify_plan` documentation must state that it
  authenticates no domain field and no key, and OC-1 rows are tested at CARDANO only.
- **R5:** if R5 is not tallied, OV-1, PC-9, PC-10 and IB-7 are excluded from tests and
  recorded as open.

**C1 Types (serde derive, field names as written, no rename/flatten/default/skip/alias):**
```rust
pub struct ActionTarget { pub action: String, pub target_hash: String }
pub struct ConstraintLimit { pub limit: AssetLimit, pub receivable_floor: UInt,
    pub position_floor: UInt /* A2 */, pub exposure_cap: UInt /* A4 iff tallied */ }
pub struct PreCommitment { pub state: BTreeMap<String, Value>, pub balances: Vec<Balance>, pub liabilities: Vec<Liability> }
#[serde(tag = "type", deny_unknown_fields)]
pub enum InputBound { UInt { action: String /* A1 */, name: String, min: UInt, max: UInt },
                      Bool { action: String /* A1 */, name: String, value: bool } }
pub struct RecipientRule { pub asset: AssetId, pub accounts: Vec<String> }
pub struct ConstraintAuthorization { network, anchor, program_address, targets: Vec<ActionTarget>,
    signer, public_key_hex, scheme: Scheme, nonce: UInt, predecessor, not_before: UInt,
    not_after: UInt, limits: Vec<ConstraintLimit>, recipients: Vec<RecipientRule> }
pub struct ConstraintIntent { profile, source_hash, program_hash, semantics, pre_hash,
    input_bounds: Vec<InputBound>, assets: Vec<AssetId>, authority: ConstraintAuthorization }
pub struct SignedConstraints { pub intent: ConstraintIntent, pub signature_hex: String }
pub enum SignerRecord { V2 { authority: Authorization, signed: SignedIntent }, V3(SignedConstraints) } // unsigned transport
pub struct AccountKey { pub scheme: Scheme, pub public_key_hex: String }                        // A5
pub struct StepFacts { pub network: String, pub anchor: String, pub program_address: String,     // A5, unsigned,
    pub predecessor: String, pub nonce: UInt, pub keys: BTreeMap<String, AccountKey> }            // consumer-authenticated
```
Every struct and enum has `#[serde(deny_unknown_fields)]`. Every field is always serialized,
including empty arrays.

**C2 Encoding.**
- Decode with `mori_core::json::parse`, then `serde_json::from_value`.
- `canonical` = `mori_core::intent::canonical`; `pre_hash` = `hash_bytes(canonical(commitment))`.
- Message = `b"moriarty-owner-constraints/1\0" || u32_be(len) || payload`, with payload ≤ 65,536.
- Orders (MR-3 + A1 + A3):
  - targets strictly ascending by action;
  - bounds strictly ascending by `(action, name)`;
  - assets sorted;
  - limits strictly ascending by `limit.asset` (Core `AssetId` `Ord`);
  - recipients strictly ascending by asset, and each rule's accounts strictly ascending;
  - commitment balances strictly ascending by `(asset, account)`, liabilities strictly ascending
    by name, and state holding exactly the declared states with their types.
- The signed JSON's text form is irrelevant. The verifier compares structures, then hashes
  `canonical(expected)`.

**C3 Preparation** (`prepare_constraints`, and pre-execution in `verify_plan`):
- every `prepare_intent` check on the shared fields (`intent.rs:135-176`);
- targets: non-empty, ordered, each a program action, each passing the `/2` string rule;
- bounds: under A1, `action` ∈ targets, `name` an input of that action with matching type,
  `min ≤ max`;
- recipients: each asset unique and carrying a limit;
- the commitment rules of C2;
- key acceptance equal to Core `check_key`/`unhex`, and ECDSA high-S refused (CO-2).

**C4 `verify_plan(program, source_hash, action, target, scenario, commitment, facts, records, at, cap)`.**

1. **Before execution (cost 0).**
   - Decode. Records must be strictly ascending by signer, one per signer.
   - For each **`/2`** record:
     - `authority.{network, anchor, program_address, predecessor, nonce}` equal `facts`;
     - `authority.target_hash == target`;
     - `(scheme, public_key_hex) == facts.keys[signer]`;
     - then the unchanged `prepare_intent` + `verify_intent`.
   - For each **`/3`** record:
     - Rebuild `expected = prepare_constraints(program, source_hash, commitment,
       signed.input_bounds, a)`. Here `a` is `signed.authority` with the five domain fields
       taken from `facts` and `(scheme, public_key_hex)` taken from `facts.keys[signer]`.
       An absent key is refused.
     - Run `verify_constraints(expected, signed, at)`: full struct equality, profile and
       semantics constants, window, key, signature.
     - Step 1 with the caller's `target`.
     - Step 2, committed part: the commitment equals the scenario normalized as Core does it
       (rows sorted; an empty state map → declared initials); every committed row exists with
       an equal amount (R6).
     - Step 3 (A1 scoped).
2. **Execution.** `mori_core::simulate_metered(program, action, scenario, cap)`. Errors pass
   through unchanged.
3. **After execution (cost = the execution's cost).**
   - `required_signers(&execution) ⊆ signers` (R3).
   - Then, per record in ascending order:
     - `/3`: commitment == `execution.pre` (R6 defence); premise rows == gross debit; step 4
       (one call, R1); 4b; 5; 5b; 5c; 6.
     - `/2`: unchanged `check_effects`.
   - The first failure is reported.

**C5 Step arithmetic (R5).**
- Step 5: Σ closing liabilities with creditor == signer for the asset, saturating u128 add,
  compared ≥ `receivable_floor`.
- 5b: `net_l` = credit − debit − signer fees, exactly as in `intent.rs:286-337`. Defined
  because step 4 passed. Then `net_l` saturating-add Σ closing liabilities with creditor ==
  beneficiary, compared ≥ `position_floor`.
- 4b: any Liability effect with creditor == signer and `after < before` requires a limit for
  its asset.
- 5c (A4): `gross` + `obligations`, exactly as `check_effects` computes them, compared
  ≤ `exposure_cap` with an exact (saturating) sum.
- S2 uses unbounded integers for these sums. The existing `add128` in the `check_effects` port
  is unchanged.

**C6 Errors.**
- Codes are `mori::constraints::{decode, order, prepare, binding, target, commitment, bound,
  signers, premise, receivable, creditor_authority, position, exposure, recipient}`.
- `check_effects`, `simulate` and signature errors pass through.
- `Error` is built directly (its fields are public; `Error::new` is `pub(crate)`).

**C7 S2 `/3` deltas** to `IMPLEMENTATION-SCOPE.md` §3:
- Steps 4b, 5b and 5c become per-record Boolean fields. Components `intent_creditor` and
  `intent_exposure` are added, along with `intent_targets`, `intent_bounds`,
  `intent_receivables` and `intent_recipients`.
- S2 applies every C3 rule it can evaluate from signed data, the catalogue and the selected
  shape (R7). It cannot evaluate other-action bound metadata (DV-1).
- Strings outside the safe alphabet are refused (DV-2).
- PreCommitment JSON is built from the datum. The key comes from the catalogue. The domain
  comes from ledger facts.

**C8 Tests.**
- Every frozen row is asserted in its tallied variant column, with the layer per OC-1.
- SX rows are included only after root adoption.
- CO-1..CO-10 as in `IMPLEMENTATION-SCOPE.md`, plus:
  - CO-11: the A5 binding (KY-1, PG-1, PG-2, DM rows, TG-9);
  - CO-12: R5 exactness (OV-1 at Core and S2);
  - CO-13: R6 (PC-9, PC-10).
- Expectation bytes are never edited. A mismatch is recorded as a failed prediction.

**C9 Forbidden (unchanged), plus:** taking `expected` from the record itself; caller-supplied
required signers, premises or flags; plan lists; scenario-hash sets; whitelists.

## 9. What stays open

RR-03 (High) stays open until an implementation passes the frozen oracle and an independent
actual-result review accepts the outcome. Also open:
- RR-03 design scope: O-D-2 (signer display); staged partial completion and recovery (O-D-5);
  substitution predicates; multi-step plans.
- Composition: recursion/PCD; private handoff; multi-parent composition.
- Families and conformance: all twenty pattern families; ACTUS 277/18/32/DS01–07; DeFi
  72/DA01–24, held-outs, intents and regressions.
- Exits: every U0–U7, M1–M17, MC01–MC08, SP01–SP12 and G01–G24 exit.
- Preprod.

**Not claimed:**
- that a position floor or exposure cap is a full financial law;
- that program rules (`discharge == payment`) hold without proof soundness;
- anything about partial stages or recovery.

Native authority stays zero.

## 10. Coverage limits

- I read Core from the currentness checkout. I cannot confirm that it equals the frozen-70
  closure (`f47318a2`). Sherlock files outside the 70 are not covered.
- I read the HELDOUT vectors only at lines 1–120 and 255–859, `invoice-finish.json` only from
  line 1000 to the end, and selected ranges of `plan.rs` and `abi2.rs`.
- I did not read `catalogue.ak`, `bytes.ak`, the tests, `build_identity.rs` or
  `CORE-COMPARISON.json`.
- Exact modes are in `READ-COVERAGE.json`.

Files: `REPORT.md`, `FINDINGS.json`, `VOTES.md`, `ELIGIBILITY-BOUNDARIES.md`,
`READ-COVERAGE.json`, `OWN-OUTPUT-ALLOWLIST.json` (written last). The root hashes them.
