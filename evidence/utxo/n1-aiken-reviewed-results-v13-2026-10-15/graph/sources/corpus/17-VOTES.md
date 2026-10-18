# Literal votes: RR-03 constraint profile, MR3-R and addenda A1–A3 (seat V13)

**Voter:** `claude-opus-5-5` (runtime-stated). Requested effort is high; I cannot observe the
actual effort. This is a fresh context. I did not author any text voted in §1–§5. Every voter
in this lineage (V8 P3S1 author, V9, V10 R1 author, V11 R1 reviewer, V12 freeze author, V13
me) is the same model family, so independence is context-level only. "V13" is my proposed
label; the root assigns seats.

**Counting rule (the root counts).** A vote counts only for the exact quoted text. AGREE means
agreement with that text. It does not approve any implementation. It confers no execution
authority and adopts nothing by itself. I did not assume that earlier votes were right; I
re-derived every claim I rely on (`REPORT.md` §2–§3). Dissent is preserved. No missing voter is
counted. A qualification listed under a vote does not change the voted text, and my vote does
not depend on it.

## 1. P3R1-AP1-r (V11 text; earlier votes: V11, V12)

Quoted verbatim from
`current-cardano-p3-repair-independent-source-review-20261015/VOTES.md` §3:

> "P3R1-AP1-r: Alternative-plan admissibility is provided by a separately versioned Core profile
> `moriarty-owner-constraints/1` and a Cardano mapping `moriarty-cardano-intent-domain/3`. The
> signer signs: a non-empty set of admissible actions, each with its own target string (for
> Cardano, `hex(predicate):hex(vk):hex(shape)` of that action's catalogue entry); a commitment
> to the pre-snapshot rows the consumer authenticates, with every other represented row a
> premise equal to that account's gross debit in the step; optional per-input bounds;
> per-asset recipient sets; per-asset limits extended by a creditor-side `receivable_floor`
> over closing duties whose creditor is the signer; and every other `/2` authority field
> (network, anchor, program address, signer key, nonce, predecessor, window). A plan is
> admissible when, for every required signer, the action and its target are in the signed
> set, the pre-snapshot commitment matches, bounded inputs are in bounds, Core
> `check_effects` passes unchanged, closing receivables meet the floor and every debit of the
> signer goes to an admitted recipient. Each signer record names its profile; a step may mix
> `/2` and `/3` records, each checked under its own profile. Substitution predicates, partial
> completion across stages and recovery remain open obligations. `moriarty-owner-intent/2` and
> domain `/2` are unchanged."

**V13: AGREE (literal).**

Reasons, from source:
1. One signed record carries a target for each action. S2 compares only the selected action's
   catalogue entry. Because `pre_hash` covers only datum rows, the records for `partial(35)` and
   `finish()` are byte-identical (P-A, P-B, PC-1, TG-1, TG-2).
2. `partial(20)` is refused by Core `check_effects`, called unchanged: escrow net 18 < 30 and
   seller net 20 < 30. My hand-derived work is 28/20/28, which agrees with the non-author Sol
   schedules.
3. A creditor-side bound exists and is observable at its boundary (CF-55-A/B, CF-55-36, CF-56-A).
4. The text's premise rule equals S2's existing premise construction (`thread.ak:419-430`).
   R3's required-signer classification equals S2's (`thread.ak:296-313`).
5. No plan list, scenario-hash set, whitelist or solver authority appears. A third plan, a
   boundary plan and a second program are admitted by the same rule. This is general
   admissibility, and the language stays permissionless.

The text is a design direction. It is compatible with A1 ("per-input bounds" can be scoped by
action), with A2 and A4 (they add limit fields without removing any), and with A5 (the `/2`
authority fields are named here; how Core obtains the expected values is not fixed by this
text).

## 2. MR3-R (V12 text; first vote V12)

Quoted verbatim from `current-cardano-rr03-constraints-profile-independent-freeze-20261015/VOTES.md` §2:

> "MR3-R: (R1) Step 4 calls `check_effects` once with every `l.limit` of the signer in signed
> order. (R2) In Core the represented balance rows are the rows of the Core pre snapshot;
> committed rows are those in `PreCommitment.balances` and must equal the pre rows with the same
> `(asset, account)`; every other pre row must equal that `(asset, account)`'s gross debit
> over the step's Transfer and Fee effects, self-transfers included; `PreCommitment.state` and
> `PreCommitment.liabilities` must equal the pre state and the complete pre liability list.
> (R3) Required signers are every `from` of a Transfer or Fee, the debtor of every effect of a
> `Liability` statement and the creditor of every effect of a `Discharge` statement,
> classified by statement kind and computed by the checker, never supplied by a caller.
> (R4) Every provided record must verify and pass under its profile, required or not, and a
> step carries at most one record per signer."

**V13: AGREE (literal).**

- **R1:** correct. `check_effects` refuses any debited or owed asset that has no limit in the
  same call (`intent.rs:264-285`). LM-3 discriminates.
- **R2:** correct.
  - "Must equal the pre rows with the same `(asset, account)`" requires such a row to exist; I
    read it that way.
  - Two consequences need explicit pins. One is PC-9. The other is Core's normalization before
    execution: an empty state map means the declared initial values, and rows are sorted.
    R6 in MR3-R5 states both. They interpret the text; they do not change it.
- **R3:** correct, and exact.
  - Each Liability or Discharge statement emits exactly one Liability effect
    (`runtime.rs:787-860`).
  - S2 uses the catalogue-pinned `discharge` flag.
  - A zero-amount accrual still requires the debtor (H1 warranty 0→0).
- **R4:** correct, and equals ABI-V2 §7.3 rule 5 (RS-3).
  - The vote text leaves out "strictly ascending by signer". S2 already enforces that order,
    and the contract states it (AR-07, Info).

## 3. MR3-A1 (V12 text; first vote V12)

> "MR3-A1: `InputBound` carries `action`; bounds are strictly ascending by `(action, name)`;
> preparation requires `action` in `targets` and `name` an input of that action with the
> bound's type; step 3 reads: every bound whose `action` equals the selected action names an
> input that is present, typed and in bounds; bounds of other actions are ignored."

**V13: AGREE (literal).**

- **IB-1 counterexample.** Re-traced. Inputs are per action (`runtime.rs:572-585`), and
  `finish` has none. MR-3 as worded therefore refuses `finish()` once any `amount` bound
  exists. A1 is the smallest fix: one field, one ordering rule, and one scoped step-3
  sentence. `min ≤ max` stays from the base preparation rule.
- **Qualification (AR-05; does not change the text).** S2 cannot check name or type for bounds
  on non-selected actions. The catalogue has no input declarations, and only the selected
  shape is present. Core refuses such records; S2 admits them, and those bounds have no
  effect on the step. R7 (MR3-R5) records this as a permitted divergence, DV-1.

## 4. MR3-A2 (V12 text; first vote V12)

> "MR3-A2: `ConstraintLimit` carries `position_floor`; after step 4, (4b) every Liability effect
> whose creditor is the signer and whose `after` is below its `before` requires a limit for its
> asset, and (5b) for each limit, the beneficiary's net (credit minus debit minus the signer's
> fees, exactly as `check_effects` computes it for that limit) plus the closing liabilities of
> that asset whose creditor is the beneficiary is at least `position_floor`. A floor of 0 keeps
> the MR-3 behaviour. The field must be fixed before the first implementation of
> `moriarty-owner-constraints/1`."

**V13: AGREE (literal).**

Reasons:
- **H71-R and DX-1, re-traced.**
  - Under MR-3, every check is a monotone box over per-plan quantities. The hull of the
    admitted plans (30,30), (89,89) and finish therefore contains (30,89).
  - With the seller's `position_floor` at 90:
    - P-A, P-B, P-A89, P-A32 and P-A40 each total exactly 90 and pass (SX-01);
    - H71 totals 35 and is refused;
    - DX-1 (30,89) totals 31 and is refused.
- **4b.** It mirrors the debtor rule (`intent.rs:271-275`) and closes LM-9. It uses
  `after < before`, which fits `check_constraints`'s effects-only input. It is equivalent to
  "nonzero Discharge" because accruals never decrease (`runtime.rs:809-814`).
- **5b's `net`.** It is defined whenever step 4 has passed. Reusing `check_effects` arithmetic
  avoids a second definition. The escrow's view subtracts the escrow's own fees; SX-02 pins
  that.
- **Timing.** The argument is right: a signed field added after v1 needs `/2`.

Qualifications (none of them changes the text):
- (a) "At least" is an exact comparison. R5 pins Core to a saturating sum and S2 to unbounded
  integers (AR-03, OV-1).
- (b) The debtor-side dual is still open (AR-02). I propose it separately as MR3-A4, so that
  this text can be counted on its own.
- (c) Position is beneficiary-relative and debtor-agnostic (AR-06, Info).
- (d) A2 bounds what a creditor can lose per unit received. It does not make program rules
  hold without the proof.

## 5. MR3-A3 (V12 text; first vote V12)

> "MR3-A3: `PreCommitment.balances` are strictly ascending by `(asset, account)`, `liabilities`
> strictly ascending by name, and `state` holds exactly the declared states; `limits` are
> strictly ascending by asset; `recipients` strictly ascending by asset; `InputBound` is
> serialized with an internal `type` tag; every new structure rejects unknown fields."

**V13: AGREE (literal).**

- **S2 already produces these orders.**
  - Datum custody rows are strictly ascending by `(AssetId, account)` (`thread.ak:127-155`).
  - Duties are strictly ascending by name (`thread.ak:159-184`).
  - Catalogue asset order equals `AssetId` order, so limit indices sort the same way.
- **The struct definitions fix the rest.** The `limit` object is nested, and every field is
  serialized, including empty arrays. The contract states these explicitly for interop.
- **Liveness only.** An ordering mismatch still fails closed, so A3 is a liveness fix; LM-2 and
  PC-7 discriminate.
- **Tag collision.** The internal tag collides with no `InputBound` field name. `Value` uses
  its own adjacent `type`/`value` tagging inside the bound's fields.

## 6. New proposals (my vote only; one vote each)

**MR3-R5 (readings; no signed-field change):**

> "MR3-R5: (R5) The sums compared with `receivable_floor` and `position_floor` are exact
> integers: Core adds with saturating u128 arithmetic, which decides every comparison with a
> floor below 2^128 exactly, and S2 uses unbounded integers; the beneficiary's net is computed
> only after step 4 has passed for every limit. (R6) Each committed balance row must be a row of
> the Core pre snapshot with the same `(asset, account)` and amount; before execution the
> commitment is compared with the scenario normalized as Core normalizes it (balances sorted by
> `(asset, account)`, liabilities by name, an empty state map replaced by the declared initial
> values), and after execution it is compared again with `Execution.pre`. (R7) Every consumer
> applies every preparation rule it can evaluate from signed data and authenticated facts; the
> only permitted difference is that S2 does not check the input name and type of a bound whose
> action is not the step's action, and such a bound has no effect on the step."

V13: PROPOSE (AGREE). Cases: SX-03 (OV-1), SX-16 (PC-9), SX-17 (PC-10), SX-19 (IB-7/DV-1).

**MR3-A4 (debtor exposure cap; adds a signed field):**

> "MR3-A4: `ConstraintLimit` carries `exposure_cap`; after step 5b, (5c) for each limit, the
> signer's gross debit of that asset (fees and self-transfers included, exactly as
> `check_effects` computes `gross`) plus the closing liabilities of that asset whose debtor is
> the signer (exactly as `check_effects` computes `obligations`) is at most `exposure_cap`. An
> `exposure_cap` not below `gross_cap + liability_cap` keeps the MR-3 behaviour. The field must
> be fixed before the first implementation of `moriarty-owner-constraints/1`."

V13: PROPOSE (AGREE). Counterexamples, which are the debtor-side dual of FZ-02:
- **H72-R (SX-05).** Fee 2, Transfer 89, invoice 90→60. The escrow is at 91 + 74 = 165. Every
  MR-3 and A2 check passes.
- **DX-1 (89,30) (SX-06).** Under A4, escrow `exposure_cap` 106 for InstallmentInvoice holds
  every honest partial at exactly 106 and finish at 104. With the seller's A2 floor of 90,
  every partial must then pay exactly what it discharges. Edge: if `gross_cap + liability_cap`
  exceeds 2^128 − 1, no cap value fully disables 5c. Realistic caps are far below that.

**MR3-A5 (Core consumer facts; API binding, no signed-field change):**

> "MR3-A5: Core `verify_plan` takes consumer-authenticated step facts — network, anchor,
> program address, predecessor, nonce and each account's scheme and public key — beside the
> existing program, source hash, action, target, scenario, commitment, time and cap. For a
> `/3` record it rebuilds the expected `ConstraintIntent` from those facts, the program, the
> source hash and the commitment, taking only the signer, targets, limits, recipients, input
> bounds and window from the record, and requires the signed intent to equal it before checking
> the signature. For a `/2` record it requires the record's authority to carry the same five
> facts, the caller's target and the account's key. An account without a key in the facts is
> refused. S2 already derives every one of these facts from the ledger, the datum and the
> catalogue."

V13: PROPOSE (AGREE). Counterexamples:
- KY-1 (SX-07);
- PG-1 and PG-2 (SX-08, SX-09);
- RP-2..RP-6 and RP-8 at Core (SX-10..SX-14);
- TG-9 (SX-15).

The key map is the program instance's own account-authority map, as in the S2 catalogue. It is
not a developer whitelist and admits no program.

## 7. Not voted

I cast no vote on P3R1-G1-r, M5-a, M7-a, P3S-M8, P3R1-B1 or any other text outside this
task's scope. I cast no vote on execution, setup, keys, signatures, proofs, Preprod, funds or
adoption. No vote here closes RR-03. The smallest concrete v1 implementation contract, which
follows these votes and depends on the root's tally, is in `REPORT.md` §8.
