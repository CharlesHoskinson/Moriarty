# Consolidated review findings, 2026-09-06

Eight independent reviews of the third iteration of the ZKIR-in-K semantics:
four on Claude Fable 5.1 (`reports/fable-R1..R4.md`) and four on Grok 4.6
through the grok CLI (`reports/grok-R1..R4.md`; the Codex GPT-6 Astra lane was
unavailable at launch, `reports/LANE-SUBSTITUTION.md`), two per scope: R1 the
modelled witness space and the soundness statement, R2 the circuit oracle and
the comparison table, R3 the target contract and the static typing, R4 the
symbolic backend, the claims and the tooling.

Every finding was checked against the cited source (the K modules, the tools,
the crates under `repos/_build/`, midnight-circuits 7.2.4 and midnight-zk-stdlib
2.3.5 in the cargo registry) and, where a circuit behaviour was claimed, against
`zkir-circuit-oracle` on a program and preimage written for the purpose (under
the session scratchpad `consolidate/`, summarised in the verification notes of
each item). Findings of the 2026-09-05 audit are not repeated; none regressed.

## Verdicts

| Reviewer | Verdict | Raised | Accepted | Fix substituted | Rejected | Deferred part |
|---|---|---|---|---|---|---|
| fable-R1 | WARNING | 5 | 5 (items 3, 4, 5, 8, 9) | option (a) of its finding 2 not taken | 0 | malicious-prover harness (D1) |
| grok-R1 | WARNING | 6 | 6 (items 2, 3, 4, 6, 7, 8) | "restrict the claim to `job` memories" not taken | 0 | 0 |
| fable-R2 | WARNING | 9 | 9 (items 10 to 18) | 0 | 0 | key comparison (D2) |
| grok-R2 | WARNING | 8 | 8 (items 10, 11, 12, 14, 15, 16) | "drop `--pis`" not taken; optimal_k panic vocabulary kept | 0 | 0 |
| fable-R3 | BLOCKED | 9 | 9 (items 1, 19 to 24, 26, 27) | 0 | 0 | circuit-size fact (D5) |
| grok-R3 | BLOCKED | 9 | 9 (items 1, 19, 21, 22, 25, 26, 27) | `wf` bound change and dropping the 248-bit rows not taken; output-signature obligation recorded as advisory | 0 | 0 |
| fable-R4 | WARNING | 11 | 11 (items 28, 30 to 38) | 0 | 0 | 0 |
| grok-R4 | WARNING | 9 | 9 (items 27, 28, 29, 31, 32, 33, 34, 36, 38) | compilation relation in the template deferred (D3) | 0 | D3 |

Both R3 verdicts stand: item 19 is a blocker. Both families found the same
three central defects independently (the alignment field count, the
`transientCommit` barrier, the `inv`-of-zero cell), and the two families
disagreed with each other only on the two questions settled below.

## The two disagreements

**Guard-on transcript input registers.** Fable R1 (finding 2) says the
circuit never binds a `public_input` / `private_input` register at any guard;
the spike, the comparison table (D3) and the receipts say an injection into a
guard-on input register is a witness-consistency error. Source: the
`PublicInput | PrivateInput` arm of `Relation::circuit`
(`zkir-v3/src/ir_vm.rs:977-1003`) matches `guard: _`, assigns the register
from `preproc.memory` through `assign_incircuit` and stores it with
`mem_insert`, whose comparison is against the same witness entry it was
assigned from; no `pi_push` and no equality against the transcript exists for
either instruction. Oracle: a guard-1 `public_input` Native register that no
instruction reads, injected 2 -> 3, is `accepted`
(`consolidate/e1a_pub_guard1_noconsumer.zkir`); the same with a guard-1
`private_input` is `accepted` (`e1c`); the same register consumed by an
`impact` is `witness-consistency-error` at `pi_push` index 1 (`e1b`); a
guard-0 register consumed by an `impact` is likewise `W` (`e1d`). So Fable R1
is right about the gate: the input instruction pins the cell to its type only,
for every guard and every type. The spike and the table are right about the
observation: the `W` they record comes from the consumer, not from the input
gate. Grok R1 (finding 2) makes the same point from the other side, that a
guard-0 register with a consumer is not free. Resolution: `unconstrained` on
these gates is a gate-level marker ("this relation does not pin the cell
beyond its type; the witness holds the type's default and the transcript has
no entry for it"), not a claim that the cell is free in the circuit; global
freedom is "no later relation reads the register", which the D5 walk decides.
The transcript binding is a residual: the public transcript is enforced by the
ledger's re-execution, the private transcript by nobody. Item 4.

**Direction of the approximation.** Chapter 08 and the header of
`zkir-constraints.k` state "any memory in the modelled witness space, with its
public-input vector, is accepted by the circuit up to the residual list"
(modelled ⊆ accepted). Chapter 16, PLAN.md and CLM-0763 state that the
modelled space "over-approximates what the real circuit accepts wherever a
constraint is unmodelled" (accepted ⊆ modelled). Fable R3 (finding 4) and
Grok R1 (question 2) are right that these are different statements, and that
the residual list as written was assembled for the first while the
compiler-soundness direction needs the second. Decision, from what a
compiler proof uses and from what the evidence supports: the primary claim is
accepted ⊆ modelled. Let `Acc(P, pi)` be the cell assignments the circuit of
`P` accepts with instance `pi`, `proj` their projection to the named registers
(a K memory when every projected cell is a typed value), and
`Mod(P, pi) = { M | witnessSpace(verdicts(P, M, pi), M) }`. The soundness
direction of compiler correctness quantifies over `Mod`; for that to cover
the circuit, `proj(Acc) ⊆ Mod` is what is needed and what the receipts
support (no `violated` or `synthErr` on any circuit-accepted witness,
`evidence/zkir-k-circuit-differential-92e8bdd3-2026-09-06c.txt:835`,
`...-ext-2ffe2d1-2026-09-06c.txt:920`). Its residual, the places where the
circuit accepts a cell assignment whose projection is not in `Mod` or is not a
K memory at all, is: JubjubScalar non-canonical bits (existing item), foreign
limb non-canonical tuples and foreign point coordinates (item 3), and the
per-chip assumption behind the typing clause (item 2). The converse,
`Mod ⊆ Acc`, is the tightness claim: the model admits no memory the circuit
rejects; it is what D5 (`inject-unconstrained` → `A`) and the `violated` →
`C`/`W`/`X` cells test, and its residual is the existence of auxiliary cells,
prover-side panics, hash gadget internals and copy wiring. Chapter 08 and the
definition header must state both claims with the two residual lists on their
own sides and name the first as the one a compiler proof uses; chapter 16,
PLAN.md and CLM-0763 keep "over-approximates" and cite that text. Item 1.

## Findings accepted, by area

Each item names the reports that raised it, the defect, the fix (file, change)
and how the fix is verified. Severity in brackets.

### Witness space and soundness statement

1. **[major] The two inclusions are stated in opposite directions** (fable-R3
   #4, grok-R1 Q2, grok-R3 #5). Fix: rewrite the "Soundness claim" paragraph of
   `semantics/zkir-constraints.k:46-66` and of
   `docs/08-constraints-and-verdicts.md:128-137` as two claims: (a) soundness
   direction, `proj(Acc) ⊆ Mod`, residual: JubjubScalar canonicity, foreign
   limb canonicity, per-chip typing assumption; (b) tightness, `Mod ⊆ Acc`,
   residual: auxiliary cells, prover-side panics, hash gadget internals, copy
   wiring; state which evidence column tests each. Make
   `docs/16-compilation-target-contract.md:53`, `plan-iter3/PLAN.md:53` and
   `wiki/zkir-k-semantics-plan.md` CLM-0763 quote (a) verbatim. Verify: the
   four texts contain the same sentence (grep), and the residual items appear
   on one side only.
2. **[major] `wellTyped` is a K-sort check, not the semantic predicate the
   header describes** (grok-R1 #1). `wellTyped(ListItem(_:Value) L)`
   (`zkir-constraints.k:180-183`) accepts `native(#r)`, `jubjubPoint(pt(0, 0))`,
   a 3-byte `bytes32`, `secp256k1Base(p)`; the constructors of
   `zkir-values.k:27-39` carry no invariant, only `decodeValue` does. Fix: a
   semantic `wellTyped`: `0 <= native < #r`, `0 <= jubjubScalar < #rJ`,
   `lengthBytes(bytes32) == 32`, points `onCurve` and `inSubgroup`
   (`zkir-curves.k`), foreign elements below their modulus, extension
   `boolV`/`byteV`/`bytesV` bounds; keep the sentence that decoder-produced
   values satisfy it and delete "by construction" from the header and from
   `docs/08:120`; state the typing clause as "the circuit constrains each
   assigned cell to its type (cited per chip: `native_gadget.rs:565-604`
   bytes, `edwards_chip.rs:842-856` cofactor, `field_chip.rs:424-455` limbs)".
   Verify: a unit test (`tools/unit_values.py` or a K test module) that
   `witnessSpace` is false on the four malformed memories above; the twelve
   claims still prove (their `requires 0 <= X < #r` discharges the new
   condition).
3. **[major] Foreign-field limb canonicity is a residual the list does not
   name** (fable-R1 #1, grok-R1 #3a). `FieldChip::assign`
   (`field_chip.rs:424-455`) range-checks limbs against
   `well_formed_log2_bounds` (`:263-278`), chosen so that
   `m <= base^(n-1) * msl < 2m`; the assigned integer is not proven below the
   modulus, and `as_public_input`, `encode_incircuit` and the commitment hash
   the raw limbs. Foreign points inherit it through their coordinates. Fix:
   `#canonicity` (`zkir-constraints.k:404-406`) returns `unconstrained` for
   the six foreign field types and the three foreign point types with a
   message naming the limbs or coordinates; residual item next to the
   JubjubScalar one in the header, chapter 08, the table's D5 paragraph
   (`circuit-comparison-table.md:241-245`) and its "What the crate cannot
   distinguish" list (`:294-297`); projection item (3) gains "for the foreign
   field chip `assert_equal` is limb equality after conditional
   normalisation, which is value equality on canonical representatives".
   Verify: `zkir_run.py` on `corpus/handmade/curve_secp256k1.zkir` reports the
   five foreign inputs `unconstrained`; the contract-corpus and divergence
   expectations updated for every program with foreign inputs; the receipts
   regenerated.
4. **[major] What `unconstrained` means on the input gates** (fable-R1 #2,
   grok-R1 #2, fable-R2 #3a, grok-R2 #4). Resolution above. Fix: reword the
   "Guarded-off inputs" residual (header `:62-66`, `docs/08:51, :135`) as a
   "Transcript inputs" item: the `public_input` / `private_input` gate pins
   the cell to its type only, for every guard; `unconstrained` is reported at
   guard 0, where the witness holds the type's default and the transcript
   has no entry; whether the cell is free in the circuit is decided by its
   consumers (D5); the public transcript is bound by the ledger's
   re-execution, the private transcript by nobody, and a specification must
   not read "holds the ledger's value" from a `holds` on these gates. Document
   `unconstrainedRegs` (`:185-189`) as "registers whose assigning relation
   does not pin the cell beyond its type". Add the harness column
   `inject-transcript` (item 12) and extend `predict_unconstrained` to the
   non-Native `unconstrained` registers so that `A` is claimed only when no
   consumer reads the cell (today `transcripts_guard_off.zkir` `%pp` is
   `encode`d, grok-R1). Verify: receipt rows `inject-transcript: A` on a
   register without a consumer and `W` on one with an `impact` consumer, on
   both surfaces.
5. **[minor] A hash input that overflows its alignment atom is `synthErr` in K
   and a rejection of an existing circuit** (fable-R1 #3). `alignedBytesCircuit`
   (`zkir-constraints.k:546-556`) reuses `#alignedBytes`, whose byte-range
   failure becomes `vErr` and then `synthErr`. Oracle: `persistent_hash` over a
   `bytes 1` atom with the operand injected to 300 is
   `witness-consistency-error` (`consolidate/e10_align_overflow.zkir`: the
   decomposition hint truncates, the recomputed digest differs from the witness
   at `mem_insert` of the output), not a synthesis error and not, as the report
   guessed, a constraint failure. Fix: a distinct constructor for the range
   failure in the circuit-side decoder mapped to `violated` in `#std4`; keep
   the "did not match alignment" arity failure as `synthErr`; the walk predicts
   `W` for this injection (the hash output is a `mem_insert` output). Verify: a
   divergence case with `--inject` (K `violated`, oracle `W`).
6. **[minor] Residual and projection omissions** (grok-R1 #3, #5). Fix: in the
   projection paragraph name the extension `reverse` with the four base
   `memory.insert` instructions (`midnight-zkir-2ffe2d1/zkir/src/ir_vm.rs:1290-1294`)
   and state that for these outputs the modelled register is the recomputed
   cell, not the `Preprocessed.memory` slot (D4); under "auxiliary cells" name
   decomposition witnesses, lookup rows and hash padding cells (`#shaPad`,
   `#kPad`) explicitly. Verify: text.
7. **[nit] `#canonicity` comment on the last hop** (grok-R1 #4).
   `scalar_from_le_bytes` (`edwards_chip.rs:1362-1380`) still constructs
   `enforced_canonical: false`; canonicity of `jubjub_scalar_from_native` is
   the `div_rem` remainder constraint, not the flag. Fix the comment at
   `zkir-constraints.k:394-403`.
8. **[nit] `#unsat` is dead; chapter 08 cites the `06b` receipts** (fable-R1
   #4, grok-R1 #6). Fix: delete `#unsat` (`zkir-constraints.k:207-210`) or use
   it for item 5; point `docs/08:137` at the regenerated receipts.
9. **[nit] Declared-type mismatch on `inputGate` is `violated` in K,
   `Error::Synthesis` in the crate** (fable-R1 #5). Reachable only under `job`
   for a program that overwrites an input register. Fix: `#typed`
   (`zkir-constraints.k:407-410`) returns `synthErr` with the `convert_values`
   text, and the table gains the row; or document the row as `job`-only.

### Circuit oracle and comparison table

10. **[major] D3 names constraint failures where the crate panics or
    misaligns** (fable-R2 #1, grok-R2 #1, #3). Oracle, on one-instruction
    programs: `inv` with the operand injected to 0 is `W` (`mem_insert` of the
    hinted inverse 0 against the honest inverse, `consolidate/e2_inv.zkir`);
    `bytes32_from_low_high` with the high operand 256 is `panic` "Trying to
    convert ... to AssignedByte" (`e3_b32.zkir`), with the low operand
    `2^248` it is `C`; `less_than` with `bits 4` and an operand 16 is `panic`
    "Trying to convert ... to an AssignedBounded less than 2^4" (`e4_lt.zkir`).
    Fix: `circuit-comparison-table.md:141-147` moves `inv` of 0 to the `W`
    list and the two conversions to a new `X` bullet with their message
    classes "AssignedByte" and "AssignedBounded", added to "What the crate
    cannot distinguish" next to D1c; `Walk.step` in `tools/circuit_compare.py`
    returns `W` for `inv` of 0 (`:362-366`, store 0), `X` with the class for
    `less_than` above the padded bound (`:376-381`) and for a high operand
    above 255 (`:518-521`), `C` for a low operand with byte 31 set. Verify:
    the three programs as divergence cases with `--inject`, `AGREE` on the
    predicted cell.
11. **[major] The generic `ok/violated` cell and the first-non-holding rule
    let a real disagreement pass** (fable-R2 #2, grok-R2 #2).
    `expected_preimage_cell` (`circuit_compare.py:207-212`) accepts any of
    `C`, `W`, `X` with no message class; `first_bad` (`:98-103`) orders by
    emission, but every `synthErr` is witness-independent and fires inside
    `optimal_k` before `MockProver::run`, so a `violated` at instruction 3 and
    a `synthErr` at instruction 7 is expected `{C, W, X}` and the crate's
    unrelated `X` counts as agreement. Latent today (both receipts: `0
    successful K runs with a non-holding gate`). Fix: a `synthErr` anywhere in
    `all_verdicts` decides the row with its message class; for `violated`
    derive the realisation from the gate (`commGate` → `C`;
    `fromCoordinates` → existing decompression logic; `bytes32_from_low_high`
    high → `X` "AssignedByte", low → `C`; `less_than` → `X` "AssignedBounded";
    `inv` → `W`; `constrain_*`, `assert`, `piGate`/`guardGate` booleanity →
    `C`; other `mem_insert` gates → `W`); reserve the three-way set for
    unclassified gates and print those as `N/C`, never `AGREE`. Verify: a
    `divergence_tests.py` case with a `violated` followed by a `synthErr`
    expects `X` with the unwrap class; the table text at `:35-38` and `:69`
    rewritten.
12. **[major] The injection columns never reach a guard-on transcript register
    or any boundary of the walk** (fable-R2 #3, grok-R2 #4). `choose_injections`
    (`circuit_compare.py:614-623`) takes declared inputs and skips
    `public_input`/`private_input`; every value is `old + 1`. Fix: a fourth
    kind `inject-transcript` (the first Native `public_input`/`private_input`
    output not in `unconstrained`), predicted by the walk; for every chosen
    register a second value chosen by its first consumer: `1 << bits` for
    `constrain_bits`/`div_mod_power_of_two`/`reconstitute_field`, `2` for a
    boolean consumer, `256` and `1 << 248` for `bytes32_from_low_high`, `x + 1`
    and `x + 2` for native `from_coordinates`, `0` for `inv`; table
    `:126-129` says what the column is. Verify: the cells `inject: X` and the
    D3 range sub-cells hit at least once per surface in the regenerated
    receipts.
13. **[major] The prove-and-verify receipt is not ledger acceptance and the
    residuals are unstated** (fable-R2 #4). `--prove` verifies the `pis`
    returned by `Zkir::prove`; the ledger's statement
    (`ledger/src/verify.rs:1956-1970`) is `[binding_input, communication_commitment,
    field_repr(guaranteed), field_repr(fallible)]` with the commitment pushed
    unconditionally, so a program without `do_communications_commitment` can
    never satisfy a ledger statement, and the verifier key is regenerated,
    never compared with a deployed key. Fix: state the three residuals
    (statement vector, unconditional commitment, key identity) in the header
    of `tools/provability.py`'s receipt and in chapter 16 tier three
    (`docs/16:117`); report a distinct `ledger.commitment` fact (`met` false
    when a program presented as a contract entry point lacks the commitment)
    rather than a failed tier-one obligation. Verify: receipt header and
    chapter text; `zkir_kast.py contract` on `expire.zkir` shows the fact.
14. **[minor] `--pis` and `--instance` can produce an `accepted` the verifier
    would reject; a shortened vector is `C`** (fable-R2 #5, grok-R2 #7).
    Confirmed on `f03_guard_uncoupled.zkir`: `--pis '["42","5"]'` and
    `--instance '["42","5"]'` are `accepted` (the extra instance cell is
    constrained by nothing; `pi_push` compares only while `idx <
    preproc.pis.len()`), `--pis '[]'` and `--instance '[]'` are
    `constraint-failure`. Fix: after `MockProver::run` compare the instance
    length with the number of pushes and report `instance-length-mismatch`;
    correct the doc comment (`main.rs:19-28`) and spike finding 5; document
    `--pis`/`--binding-input` as witness-side perturbations that surface as
    `W`, or remove them. Verify: the two commands above produce the new
    outcome.
15. **[minor] Undecided walk cases that are decidable** (fable-R2 #8, grok-R2
    #3). `cond_select`, `not` and `impact` with a bit or guard of 2 are `C`
    with two equality failures and no misalignment (the `convert` hint maps
    the non-boolean value to the honest branch and only booleanity fails;
    confirm in `native_chip.rs` `convert` for `AssignedBit` before relying on
    it); a tainted `y` into native `from_coordinates` is `X` when
    `jubjub_from_xy` is `None`, else `W`; extension `and`/`or`/`xor` are
    boolean arithmetic stored by `mem_insert`; `slice`/`nth`/`concat` use
    `mem_insert` at 2ffe2d1. Fix: `bool_check` (`circuit_compare.py:316-321`),
    the guard branch (`:493-497`), the `y` branch (`:416-418`) and the
    extension arms return the outcome instead of raising `Undecided`. Verify:
    zero `N/C` rows in both receipts.
16. **[minor] Outcome classification is looser than the vocabulary** (grok-R2
    #5, #6, #8; fable-R2 #7). `W` rests on a tracing substring alone; an
    `--inject` parse failure is reported as `preprocess-error`
    (`main.rs:508-511`); the D2 unwrap class is the substring `Synthesis`; the
    `panic` row accepts any panic. Fix: classify `W` by both the stashed
    `error_if_known_and` error and the event, else `synthesis-error`; inject
    errors exit 2 (`inject-error`); the unwrap class requires "called
    `Result::unwrap()`" and "Synthesis("; the `panic` row carries the two D10
    message classes. Verify: `divergence_tests.py` message classes; a bad
    inject file exits 2.
17. **[minor] D1(b) and `_native_from_coordinates_cell` are wrong for the
    foreign Edwards chip** (fable-R2 #6). `point_from_coordinates`
    (`ecc/foreign/edwards_chip.rs:1087-1105`) returns
    `Error::Synthesis("invalid coordinates")` from the hint (`S`) and
    `into_subgroup` panics on torsion (`X`); Weierstrass is `W` as stated.
    Fix: table `:89-93` and `circuit_compare.py:179` split Curve25519 from
    secp256k1/secp256r1. Unreachable today (preprocess rejects first).
18. **[nit] A panic drops the partial `Output`** (fable-R2 #9). `k`,
    `preprocess_ms`, `optimal_k_ms` are absent from every `panic` line. Fix:
    build `Output` outside the `catch_unwind` closure (`main.rs:632-647`).

### Contract and static typing

19. **[blocker] The alignment obligations omit the field-count check, so a
    program the contract passes fails keygen** (fable-R3 #1, grok-R3 blocker).
    `#alignDefect` (`zkir-contract.k:160-168`) sees the alignment alone;
    `fab_decode_to_bytes_atom` (`ir_vm.rs:133-137, 156-159`) needs one
    operand per `field` atom and `ceil(length / 31)` per `bytes` atom and
    returns `Error::Synthesis` otherwise, witness-independently, on both
    surfaces (`sha512` included). Confirmed: `persistent_hash` with
    `[field, field]` and one input, and `keccak256` with `[bytes 40]` and one
    input, both `contract` met and `--keygen` `panic` "cannot decode field
    element from no data" / "cannot decode bytes from to little data"
    (`consolidate/e5_align_short.zkir`, `e5b_align_bytes40.zkir`). Fix:
    `#alignFields(Segments)` in `ZKIR-STATIC` (1 per `fieldAtom`, `(N + 30) / 31`
    per `bytesAtom(N)`, 0 for compress, options already fail); `#alignDefect`
    takes the operand list and fails with the Rust text when
    `lenOperands < #alignFields`; surplus operands stay accepted; the same
    check in `ZKIR-WF` (`zkir-syntax.k:353-365`), since `preprocess` rejects
    it for every preimage; the two programs under `corpus/handmade-negative/`,
    in `EXPECTED` of `tools/contract_corpus.py` and `NEGATIVE_CONTROLS` of
    `tools/provability.py`. Verify: contract fails on both, keygen panics on
    both, receipts regenerated.
20. **[major] `reconstitute_field` with `bits = 0` passes tier one and panics
    keygen** (fable-R3 #2). `assert_lower_than_fixed(divisor, 1 << 255)`
    trips `assert!(bit_length < F::NUM_BITS)` in
    `field/decomposition/chip.rs:412`; off-circuit `preprocess` bails
    "Excessive bit bound" on every value. Confirmed (`consolidate/e6_rf0.zkir`:
    contract met, keygen `panic` "assertion failed: (bit_length as u32) <
    F::NUM_BITS"). Fix: reject `bits = 0` in `#checkArity(reconstituteField)`
    and in `defect(rfWidth(), ...)` with the Rust text; negative case; record
    as candidate upstream finding K7 (load accepts, both stages reject, one by
    an assertion). Verify: contract fails, keygen panics, receipt.
21. **[major] "Keygen performs exactly these checks" is false in the other
    direction, and the provability method cannot see it** (fable-R3 #3, #6;
    grok-R3 #2, #3, minor). Confirmed: `reassignment.zkir` (k=4),
    `excessive_bits.zkir` (`constrain_bits 255`, k=9), `div_mod_power_of_two`
    249 and `reconstitute_field` 249 (k=9) all key with the contract failed;
    the 248-bit and the `>= 255` bounds are `preprocess` checks
    (`ir_vm.rs:268, 399, 418`), single assignment is K-only (CLM-0711), and
    `width.constrain_bits` (`> 255`) is unreachable behind `wf` (`>= 255`).
    `provability.py` keys only programs whose contract is met plus a curated
    list (`:67`, `:229-235`), so "0 contradictions" is by construction. Fix:
    every obligation carries a stage (`keygen`, `preprocess`, `both`) as a
    field of `obligation` exported by `zkir_kast.py contract`; rewrite
    `zkir-contract.k:41-49` and `docs/16:31` as "a program with every
    keygen-stage obligation met or not applicable keys; a program with a failed
    preprocess-stage obligation is rejected on every preimage"; split the
    chapter 16 table by stage and cite `preprocess` for the 248-bit rows and
    `wf`, `assigned_to_le_bits`/`bounded_of_element` for the others; tag
    `width.constrain_bits` as unreachable behind `wf` or drop it;
    `provability.py` keys every parseable program, expects `accepted` when
    only preprocess-stage obligations fail and a panic or synthesis error
    naming the instruction otherwise. `wf` keeps its `preprocess` bounds
    (the K static layer stays strictly stricter, prior audit item 2). Verify:
    the regenerated provability receipt lists the five programs above as
    keyed-with-preprocess-stage-failure and the two of item 19 and the one of
    item 20 as keygen failures.
22. **[minor] `#firstKnown` binds a guessed type after an unsupported pair**
    (fable-R3 #7, grok-R3 nit). `bindOuts` for `add`, `mul`, `cond_select`
    (`zkir-static.k:120, 138-139`) binds the first known operand type when
    the two known types differ; a later `circuit.static.output` then reports
    the guess, which `docs/16:29` says never happens. Fix: bind `unknownT()`
    unless both operand types are known and equal. Verify: the mixed-type
    program of the report reports one defect only.
23. **[minor] A missing oracle binary is a failed tier three** (fable-R3 #8).
    `zkir_kast.py:498` returns `met: False`; `contract_corpus.py:147` counts it
    as `UNEXPECTED`. Fix: `met: None` and a "not run" count.
24. **[nit] The negative-control check is weaker than the chapter's sentence**
    (fable-R3 #9). `NEGATIVE_OK = {'panic', 'synthesis-error'}`
    (`provability.py:87, :290`); chapter 16 says the failure names the
    instruction. Fix: compare the oracle message with the obligation's detail.
25. **[nit] Two chapter-16 rows to reword** (grok-R3 nits). `version` is `info`
    while the pin is `wf` plus the preprocessor: cite `wf`, not
    `IrSource::load`. A non-empty output signature with no `output`
    instruction keys and the contract says `notApplicable`: state that the
    signature is advisory until an `output` instruction occurs.

### Observable semantics

26. **[major] `obs(Status, Outputs, Pi)` drops `pi_skips`** (fable-R3 #5,
    grok-R3 #4). `Preprocessed.pi_skips` (`ir_vm.rs:508-523`, `Some(count)`
    per guarded-off `impact`) is returned by `prove` and turned into
    `Op::Noop { n }` in the transaction's transcripts
    (`ledger/src/prove.rs:276-290`); two preimages with equal `(status,
    outputs, pis)` and different skips are different transactions, and the
    `impact` guard is the one place the compiler encodes control flow into the
    public inputs. The definition already has `<skips>` (`zkir-vm.k:75, 485`).
    Fix: `obs(Status, List, List, List)` with `<skips>` as the fourth
    component (`zkir-vm.k:551-556`), `#observable`, `spec_compiled_observable`
    in `tools/run_claims.py`, `zkir_run.py`'s `observable`, the tier-two
    object of `zkir_kast.py`; chapter 16 states that `pi[0]` is the binding
    input, `pi[1]` the commitment when enabled, that the opening is
    existentially quantified in any statement, and why the memory is not
    observable. Verify: the claims re-prove with the four-place `obs`;
    `zkir_run.py` prints `skips`; the contexts receipt shows the skip list of
    `expire` (`Some` entries).
27. **[major] The compiler-correctness statement is not at the strength the
    definition supports** (fable-R3 #4 second paragraph, grok-R3 #5, grok-R4
    #2 in part, fable-R4 answer 3). `docs/16:53` and `PLAN.md:53` mix a
    reachability claim the definition proves with a per-preimage oracle
    outcome, omit the encoded outputs and the skips, and PLAN.md's "This is
    what the oracle tests" is false of completeness (the oracle compares K
    verdicts with MockProver on a preimage; it has no source semantics). Fix:
    completeness = "a source-honest preimage `pre` gives `[[P]](pre) =
    obs(ok, enc(outs), pis, skips)` equal to the source prediction" (a K
    reachability claim, the template) plus "the circuit accepts `(P, pre)`",
    an empirical side condition discharged by tier three on the corpus;
    soundness = a statement over `witnessSpace` (item 1), not over `obs`;
    delete the oracle sentence from PLAN.md and CLM-0763; the template gains a
    `witness_space` parameter that pins `<witnessSpace>` and
    `<unconstrainedRegs>` (usable once item 28 is fixed), its docstring says
    `targetContract(P)` is discharged separately and that it runs `job`, not
    `checkedJob`; rename the generated instance to an observable instance
    (`claims/README.md`). Verify: text; the instance re-proves with
    `witness_space` pinned.

### Symbolic claims

28. **[major] `transientCommit` and `poseidonHash` are two uninterpreted
    symbols for one hash** (fable-R4 #1, grok-R4 #1). `zkir-hash.k:81` carries
    `[concrete]`; `#commCheck` (`zkir-vm.k:582-584`) uses `transientCommit`,
    `commGate` (`zkir-constraints.k:132`) uses `poseidonHash`, so on a symbolic
    input the witness side and the gate never meet and no verdict-pinned claim
    with `do_communications_commitment` (every Moriarty artifact) can prove;
    reproduced by fable-R4 (`comm-sym.k`, residual `#commOutcome(...,
    transientCommit(ListItem(A), 5), ...)`). Fix: drop `[concrete]` from the
    `transientCommit` rule (it is a definitional alias; the barrier stays on
    `poseidonHash`); add a claim with the commitment flag and verdicts pinned;
    update `claims/README.md:30-40` and the `zkir-symbolic.k` header. Verify:
    the new claim and the twelve existing ones prove under `run_claims.py`.
29. **[minor] `[concrete]` does not cover the helpers; "functionality is the
    only property" is too strong** (grok-R4 #7). `absorbAll`, `permute`,
    `#htc`, `mapToCurve`, `#shaPad`, `#shaBlocks`, `#kPad`, `#kAbsorb`,
    `#ecMulAcc` have no attribute; `ecMul(C, _, K) => identity(C) requires K
    <= 0 [concrete]` still fires on a symbolic point with a concrete scalar;
    totality and the result sort remain. Fix: `[concrete]` on the helpers;
    README narrowed to "no collision, injectivity or range lemma; totality
    and the sort remain".
30. **[minor] `constrain_to_boolean-ok` and `assert-ok` are concrete runs;
    the `[2, r)` branch of `assert` is unstated** (fable-R4 #6). The natural
    disjunctive form does not prove (`asBool` needs a case split). Fix: one
    claim per boolean value under one module, and an error claim for
    `assert` on `[2, r)` ("Expected boolean, found: C").
31. **[minor] No vacuity check; `#Top` is a substring test** (fable-R4 #4,
    grok-R4 #5). An unsatisfiable `requires` proves anything (reproduced,
    `vacuous-add.k`). Fix: per module a companion probe with the same LHS and
    an unreachable RHS that must fail, or detection of `WarnTrivialClaim`;
    success requires a line that is exactly `#Top`.
32. **[nit] Claim hygiene** (fable-R4 #11, grok-R4 #8). Pin `<observable>` in
    the eleven verdict claims; fix the stale header of `claims/add-spec.k:4`
    (four gates, not two).

### Tooling and receipts

33. **[major] `make` never fails on a failing layer** (fable-R4 #5, grok-R4
    #3, #9). Every recipe is `tool | tee receipt` under `/bin/sh`, so the
    status is `tee`'s (reproduced with `false | tee`); `DATE` has no suffix so
    a same-day rerun overwrites the audited receipt; there is no `claims`,
    `drift` or `zkir-symbolic` target, so the two receipts of R4 were
    assembled by hand. Fix: `SHELL := /bin/bash`, `.SHELLFLAGS := -eu -o
    pipefail -c`; a `SUFFIX` variable; targets `zkir-symbolic`, `claims`,
    `drift`; `check` echoes only after every layer returned 0. Verify: a
    recipe with a failing tool exits non-zero; `make claims` regenerates the
    claims receipt including its header lines.
34. **[minor] Runner exit codes and messages** (fable-R4 #2, #3; grok-R4 #4).
    `run_claims.py:138-145` tests `[Error]` before the residual-goal branch,
    so every refuted claim is a "kprove error"; `--only` with an unknown name
    prints `0/0 claims proved` and exits 0; `moriarty_contexts.py:136` matches
    `--only` as a substring and exits 0 on no match. Fix: match the prover's
    "cannot be rewritten further" line as residual; reject unknown names with
    exit 2 and `total == 0` with exit 1; whole-name match.
35. **[minor] The contexts negatives never reach the circuit** (fable-R4 #8).
    Every negative row is `preprocess-error`, honest and inherent, but the
    docstring says "rejected on all three sides" and `outcome != 'accepted'`
    would also pass a `load-error`. Fix: require `preprocess-error`; reword
    the docstring, the exit criterion and the receipt summary; note that the
    regenerated public inputs are truncated at the failing assertion.
36. **[minor] `upstream_drift.py` is blind to body and type changes**
    (fable-R4 #9, grok-R4 #6). Only serde names, field names and three
    dependency versions are compared. Fix: field types in the payload; per
    comparison `git diff --stat <pin> <head> -- <crate>/src/ir_vm.rs
    <crate>/src/ir_instructions/ transient-crypto/src/proofs.rs`, counted as
    drift when non-empty.
37. **[nit] Receipt provenance** (grok-R4 Q4, fable-R4 #10). Archive the
    kprove logs next to the claims receipt; correct the manifest sentence on
    the commitment randomness (the DApp supplies it in the call prototype,
    `ledger/src/construct.rs:565-568`; the ledger overwrites only the binding
    input).

### Documentation

38. **[minor] Stale statements** (fable-R4 #7, grok-R4 #9). `claims/README.md:52-56`
    and `zkir-symbolic.k:30-35` say the triple cannot be packaged into one
    term (the `<observable>` cell exists since M5b);
    `wiki/zkir/zkir-k-definition.md` CLM-0746 says the uninterpreted-hash
    treatment is not implemented; `docs/11-tooling-reference.md` documents
    none of `run_claims.py`, `upstream_drift.py`, `moriarty_contexts.py`,
    `moriarty_preimages.mjs`, `preimage-json`. Fix all four, and add the M3
    claim record to the definition page with the regenerated receipt.

Counts: 1 blocker, 14 major, 15 minor, 8 nit (38 items).

## Rejected, with reasons

- **Mark every `public_input` / `private_input` register `unconstrained`
  regardless of guard** (fable-R1 #2, option a). Rejected: it would list every
  transcript register in `unconstrainedRegs` and lose the distinction the
  Compact default-on-guard-0 pattern relies on; the gate-level meaning plus the
  transcript residual (item 4) states the same fact without that loss.
- **Restrict the soundness claim to memories `job` produces** (grok-R1 #1,
  alternative fix). Rejected in favour of the semantic predicate (item 2): the
  claims quantify over symbolic memories, and a predicate that is only true of
  decoder output cannot be quoted by a specification.
- **`#guardedOff` should require `defaultValue(T)`** (grok-R1 #2, sub-point).
  Rejected: at guard 0 `job` writes the default itself, as `preprocess` does,
  so a non-default value cannot arise from a run; the marker records the guard.
- **Change `wf`'s `constrain_bits` bound to `> 255`** (grok-R3 #2, minor).
  Rejected: `wf` mirrors `preprocess`, which rejects 255 (`ir_vm.rs:268`);
  the K static layer is strictly stricter than keygen by design (prior audit
  item 2). The stage tag of item 21 removes the contradiction in the prose.
- **Drop the 248-bit width obligations** (grok-R3 #2). Rejected: a program that
  fails them is rejected on every preimage, which a compiler must know; they
  become preprocess-stage obligations (item 21).
- **Drop `--pis` and `--binding-input`** (grok-R2 #7). Rejected: they are the
  documented way to perturb the witness-side vector; item 14 documents them as
  such and adds the instance-length check.
- **Map `optimal_k` panics to `synthesis-error`** (grok-R2 Q5). Rejected:
  `setup_vk` panics on the same error, so `panic` is the crate's own
  vocabulary for keygen; the message class (item 16) tells the cases apart.
- **A failed obligation when the output signature is non-empty and no
  `output` instruction occurs** (grok-R3 minor). Rejected as an obligation:
  keygen accepts the program, so the contract would contradict it; recorded
  as advisory text (item 25).
- **`commitment` as a failed tier-one obligation** (fable-R2 #4, part).
  Rejected as tier one: the program keys; a distinct `ledger.commitment` fact
  is adopted instead (item 13).
- **A compilation relation `compile(S) = P` and `sourcePreimage(S, ctx)` as
  template parameters** (grok-R4 #2). Rejected for this definition: those are
  the objects of the Moriarty specification, not of the ZKIR semantics; the
  observable-instance rename and the `witness_space` pin are adopted (item 27),
  the rest is deferred (D3).
- **MockProver at `k` below 9 versus `setup_vk`** (grok-R2 Q3). Not a finding:
  the provability receipt checks `pk_k == k` on all 122 keyed programs, and
  MockProver's row count does not change the constraint system.

## Deferred, with reasons

- **D1. A malicious-prover harness for non-canonical JubjubScalar bits and
  foreign limb tuples** (fable-R1, grok-R1 coverage). Needs a patched
  `Relation` writing limb advice independently of `assign_incircuit`; the
  residual is stated (items 1, 3) rather than tested.
- **D2. Byte-for-byte comparison of `--keygen`'s verifier key with the
  compactc keys** (fable-R2 #4c). No key files exist under the Moriarty
  `output/` directories; `--vk FILE` is specified for when they do.
- **D3. A compiler obligation with a compilation relation and source
  executions** (grok-R4 #2). Belongs to the Moriarty specification; the
  template's shape is fixed by item 27 so that it can be instantiated there.
- **D4. Claims on the extension surface and a Moriarty artifact as a `kprove`
  claim** (both R4 coverage). `ZKIR-SYMBOLIC` imports `ZKIR-VM` only; the
  smallest artifact is 49 instructions with 84 transcript inputs and a
  `persistent_hash`; the commitment claim of item 28 is the first step.
- **D5. A circuit-size fact in tier one** (fable-R3 Q5). Deliberately an
  environment concern until a compiler needs to state provability against a
  named SRS.
- **D6. Checkpoint ops in the seven Moriarty circuits** (fable-R4 Q2). Whether
  `partition_transcripts` re-encodes a `ckpt` op is to be settled with an
  M4 follow-up on the runtime's op list; no artifact receipt depends on it.
- **D7. Upstream report K7** (`reconstitute_field bits = 0`, item 20) is
  recorded for the next upstream batch with K1 to K6.

## Fix plan, by dependency

Two implementers. A owns the definition, the claims and the prose; B owns the
tools, the oracle, the table and the receipts. Kompile after each A step
(`make kompile`, `make zkir-contract`, and the Haskell definition).

**Step 1, independent.**
A: items 19, 20, 21, 22, 9 in `zkir-static.k`, `zkir-contract.k`,
`zkir-syntax.k`, `zkir-constraints.k` (`#typed`), plus the stage field in
`zkir_kast.py`'s export (shared with B, A writes it); the two negative
programs. B: items 33, 34, 35, 36, 37 (Makefile, `run_claims.py`,
`moriarty_contexts.py`, `upstream_drift.py`, the manifest sentence), item 18
and the `inject-error` exit of item 16 (`circuit-oracle/*/src/main.rs`).

**Step 2, A first.**
A: items 26 (four-place `obs`, `zkir-vm.k`), 2 (semantic `wellTyped`), 3
(`#canonicity` on foreign types), 5 (range failure → `violated`), 6, 7, 8 in
`zkir-constraints.k`; `zkir_run.py` exposes `skips`. Then B: items 10, 11, 12,
14, 15, 16, 17 in `circuit_compare.py`, `diff_test.py`, `divergence_tests.py`
and `main.rs` (item 12's `inject-transcript` column and the non-Native walk of
item 4 depend on item 3's new `unconstrained` registers; the divergence cases
of items 5, 10, 11 depend on the K verdict changes), and `provability.py`
(item 21's stage-aware sweep, item 13's receipt header and `ledger.commitment`
fact, items 23, 24) with `contract_corpus.py`.

**Step 3, A.**
Items 28, 29, 30, 31, 32 in `zkir-hash.k`, `zkir-curves.k`, `zkir-symbolic.k`,
`claims/*.k`, `run_claims.py`'s template (item 27's `witness_space`
parameter); re-kompile the Haskell definition; `make claims`.

**Step 4, prose, after the code settles.**
A: items 1, 4, 13 (chapter text), 25, 27, 38 in `docs/08`, `docs/16`,
`docs/11`, `PLAN.md`, `claims/README.md`, the two wiki pages (CLM-0746,
CLM-0763, a new claim for the residual split). B: `circuit-comparison-table.md`
(items 3, 4, 10, 11, 12, 14, 15, 17) and the receipts: `make check`, `make
contract`, `make contexts`, `make circuit`, `make provability`, `make claims`,
`make drift` with a new suffix; both receipts of `--circuit` with zero `N/C`
and the new columns, the provability receipt with the stage-aware sweep.

**Step 5, both.** Cross-check: the inclusion sentence identical in the four
texts; every residual item on one side; `unit_values.py` malformed-memory
tests; the counts of the regenerated receipts recorded in the definition page.
