# Design votes: generic current-Core R1CS compiler (r2 source author)

**Voter:** `claude-opus-5-5` as stated by the runtime. "High" effort was requested; I cannot observe it. I am the compiler source author, so these votes are design judgments from my own reading of current source. They are **not** an independent review of this candidate. Every earlier voter is the same model family, so independence is at the context level only. I consulted no other model or agent.

**Counting rule** (claim-utxo-abi-v1 VOTES.md): a count applies to the exact text of its row only. An amended agreement is not a vote for the literal text. These votes confer no execution, setup, key or adoption authority. Rosters are preserved below; the root tallies.

## 1. Agreement with the existing audits on the four questions

| Question | Audit position | My vote | Reason (current source) |
|---|---|---|---|
| Source-bound program, IR and compiler identity | X7 review §7 F2 and R-7; ABI-v1 VOTES §5 P4 item 3 | **AGREE, with amendment** | The relation must bind the canonical Program hash, profile (language, semantics, logical-cost profile, relation format/version), action, statement shape and compiler build **as in-circuit constants**, not only in a catalogue row. The raw source hash is a separate provenance identity and is never bound (export.rs:19-30). A "compiler commit" is not enough: the build identity must cover the exact Core closure bytes. The incoming `pub mod sherlock;` changes Core `lib.rs` and therefore the identity, though the financial files are unchanged. I agree with F2 that an R1CS digest names matrices but binds no VK without a setup transcript. Implemented: identity.rs, statement.rs header, `ProbeReport::r1cs_sha256`, CORE-CLOSURE-PINS.json. |
| Constrained semantics | P4 items 2 and 5; product contract §ZKIRv3 ("witness generation correctness is insufficient") | **AGREE** | One generic lowering over the typed IR, no per-program circuits. Every reachable node is lowered with no short-circuit (runtime.rs:451-453). Each gadget is satisfiable exactly when Core's checked operation succeeds (MATH §2). I add a requirement the audits did not state: the success relation must be **neither stricter nor weaker** than `simulate`. For example, the accounting sums use exact 144-bit integers rather than checked u128, because Core never refuses on an effect total. |
| Full ordered effects, state, duties, work and range limits | P4 items 2-5; ABI-v1 VOTES P3-8 and P3-10 | **AGREE** | Ordered effects with constant kinds and fields; positional Requires/Ensures; Next rebinding; present-zero distinct from absent, with an in-circuit canonical absent `(false, 0)`; registry identities as constants; `W` as a constant with `W ≤ cap` when bound; 64-bit limb ranges and Bool canonicality on every public field. MSB0/MEB0 cannot carry this (P3-8), so the ABI v1 mapping needs a new encoding. |
| Independent differential oracles | P4 item 4; Aiken R1 review §11; UNI-020 | **AGREE on direction; DISSENT on one clause of the literal text** | Agree: frozen independent expectations before execution, boundary vectors, and full-outcome comparison. Dissent: "StageRejection codes must equal the first violated constraint region's mapped code" does not fit a success relation. Unsatisfied constraints carry no Core refusal order, and arkworks reports the first unsatisfied *constraint index*, which depends on synthesis order. For a success relation the differential predicate should be "satisfiable iff the reference succeeds, with equal outputs", plus hostile-witness controls. Refusal codes need a separate rejection relation. |

## 2. Second votes on the one-vote ABI-v1 contracts (VOTES.md)

### P4 contract items (one vote, V5)

| Item | Literal text (abridged) | My vote |
|---|---|---|
| 1 | Input is exactly `stage::load` output | **AMEND.** That holds for `/2`. For `/1` the actual admission is `parse` (which validates) plus `require_core1`, because `stage::load` refuses `/1` (stage/profile.rs:26-31). Implemented that way in frontend.rs. |
| 2 | Versioned generic lowering L, written once, no per-program circuits; M1 the only hand-lowered row | **AGREE (literal)** |
| 3 | Catalogue binds program/plan digests, action, site, lowering version and source digest, compiler commit, R1CS digest, P3 maps; statement carries program and plan digests as constants | **AMEND.** As above: the profile, relation version, shape digest and compiler build digest (including the Core closure) also belong in-circuit, and "compiler commit" becomes a byte manifest. |
| 4 | Differential harness incl. refusal codes = first violated region | **AMEND** (see §1). |
| 5 | Coverage of all 20 families, every effect kind incl. Liability, Await, Emit; otherwise refuse, never approximate | **AGREE (literal)** |
| 6 | No claims until 1-5 pass review and an authorized run | **AGREE (literal)** |

With V5, items 2, 5 and 6 reach two agreeing votes on their literal text. Items 1, 3 and 4 stay at one vote on the literal text, with my amended texts as new first votes.

### P3 contract items P3-1 to P3-12 (one vote, V5)

| Item | My vote |
|---|---|
| P3-1 total asset map, no rounding, unsatisfiable outside [0, 2^63) | AGREE (literal) |
| P3-2 custody/external account map | AGREE (literal) |
| P3-3 in-circuit `ledger_delta` (payouts, deposits, custody totals, gross and net per (account, asset)) | **AMEND.** Agree on (a)-(c) and gross. Net per cell equals `post − pre` of that cell, which is already in the statement; a separate signed net field needs an explicit canonical sign encoding first. My source implements gross debit and fees only (AccountingBinding::GrossDebitAndFees). |
| P3-4 full unnetted fees | AGREE (literal) |
| P3-5 exact conservation at settle | AGREE (literal) |
| P3-6 tagged payouts | AGREE (literal) |
| P3-7 debit authority for every external account with gross debit > 0 | AGREE (literal). Note: a self-transfer counts toward gross debit (intent.rs:297-306), so it needs the signature although nothing moves. |
| P3-8 full liability rows, present-zero distinct | AGREE (literal); implemented in the relation |
| P3-9 one-time consumption | AGREE (literal) |
| P3-10 staged frame semantics | AGREE (literal); see STAGED-LOWERING-PLAN.md |
| P3-11 Emit chained or tagged, never dropped | AGREE (literal). Consistent with the adopted whole-program Emit refusal of the initial target: refusing is not dropping. |
| P3-12 wording discipline ("datum transition certified") | AGREE (literal) |

## 3. Consequential choices in this source needing second votes (my first votes)

| Id | Choice | Vote | Note |
|---|---|---|---|
| C1 | One circuit-specific relation per (program, action, options) with fresh setup each; not a universal circuit | AGREE | Permitted by the task. It is not a program registry: anyone can compile, and the relation is determined by the program. |
| C2 | Header slots as public constants (version, program hash, profile, action, shape, build, W) | AGREE | The VK is specialized anyway. Constants put the identities into the verified statement. |
| C3 | Layout v0 order and encodings (UInt `[lo64, hi64]`, digest `[hi128, lo128]`, wide `[lo64, mid64, top16]`) | AGREE | Test-only adapter; the ABI v1 mapping decides final bytes |
| C4 | Represented snapshot = touched cells + explicit frame cells + the whole duty registry; extra rows are a target premise | AGREE | Core's trace bound counts all rows. Frame completeness is listed as a target obligation |
| C5 | External duties refused in the first slice | AGREE | A dynamic identity cell needs its own design |
| C6 | An action that can never succeed is refused at compile time rather than emitted as an unsatisfiable relation | AGREE | Either is sound; refusal is more informative |
| C7 | `CapBinding` explicit, no default | AGREE | — |
| C8 | `AccountingBinding::GrossDebitAndFees` as an option, no default | AGREE | Partial P3-3(d) |
| C9 | Build identity = BUILD-MANIFEST (Cargo.toml + src) + Core closure digest | AGREE | Documentation edits do not change the relation; Core byte changes do |
| C10 | R1CS digest format `mori-r1cs-matrices/1` under `OptimizationGoal::Constraints` | AGREE | Must be re-checked against ark-groth16 0.4 setup (UNVERIFIED-LIB) |
| C11 | Direct field adapter is test-only; the ABI v1 mapping is pending and refuses | AGREE | No second claim ABI is created |
| C12 | Emit lowered by the compiler; whole-program refusal at the initial ledger target | AGREE | A third vote for the already adopted policy |
| C13 | D-13: the compiler selects no reconstruction cap; the stored cap is an explicit public slot | AGREE on the slot; **ABSTAIN** on the D-13 value | The value is a codec decision and stays split (`Some(plan_total)` vs `None`) |
| C14 | Keep the exact arkworks 0.4 pins of the prototype (ark-ff =0.4.2, ark-relations =0.4.0, ark-bls12-381/groth16/snark =0.4.0) | AGREE | Root correction. The prior partial manifest's 0.5 line is replaced, not adopted |
| C15 | Core dependency by relative path into an isolated pinned copy; no live worktree path | AGREE | — |

## 4. Dissent and records kept

- The prior partial source (`current-core-groth16-compiler-source-20261015`) named arkworks `0.5`. Root corrected this to 0.4, and the r2 manifest uses the prototype's exact 0.4 pins. The prior bytes are kept in ROOT-PARTIAL-SOURCE-COPY.json; nothing was rewritten silently.
- AUTHOR-EXPECTATIONS T1 and T4 (prior session, same author) were wrong in two places. ADDENDUM-1 (E-T1, E-T4) records both, frozen before any r2 assertion was written, and the original packet bytes are unchanged.
- Not voted, and not inferred from silence: any execution, setup, key, Preprod or ledger action; adoption of ABI v1 as a whole; the D-13 value; the Sherlock typed host or Candidate B, for which no compiler or target support is claimed.
