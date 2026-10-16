# Adopted reading references

This is a verbatim quotation index, not a new review. The complete Opus source review and Muse reconciliation are preserved in `reviews/`; Root’s adoption binds them. The unchanged proposal and first Muse review retain historical wording superseded by those readings.

## Owner field split

Source: `reviews/opus-source/RESULT.text.gz`, §1.

- **G4-FIELD-SRC [SF+MD], adoptable as a requirement statement.** The three Rust-side encodings agree, and the literal meets every bound above.
- **G4-FIELD-PRIME [RA].** Primality is not derived here. TestEq soundness and Assert soundness depend on it, because they need a zero-divisor-free ring with inverses.
- **G4-FIELD-REAL [UK].** This covers the executed blst arithmetic, its modulus, the feature and cfg resolution, the binary identity, and the prover and verifier.

## B1–B13 reconciliation

Source: `reviews/muse-owner-reading-disposition/RESULT.text.gz`, “Per-reading disposition”. These quotations index the complete Opus B1–B13 readings; they do not replace the full owner text.

- **B1 — AGREES.** Split G4-FIELD into G4-FIELD-SRC [SF+MD], G4-FIELD-PRIME [RA], and G4-FIELD-REAL [UK]. The Rust modulus encodings agree and satisfy the stated bounds, but reading the literal proves neither primality nor the executed `blst` arithmetic, Montgomery correctness, `cfg(ci_build)`/`portable` resolution, feature selection, build identity, or artifact behavior.
- **B2 — AGREES.** `TestEq` Booleanity rests solely on `(x−y)·aux = 1−res`, `(x−y)·res = 0`, and the prime-field assumption. The subsequent `ConstrainToBoolean` is an instruction present in emitted code but a constraint no-op for the already bound-cached cell; no reviewer or certificate may count it as an additional Boolean gate.
- **B3 — AGREES.** Every G4 predicate is a semantic cell property, not a per-call gate count. `NativeGadget::constrained_cells`, keyed by cell identity, records bound provenance; `convert` and `assert_lower_than_fixed` may use `convert_unsafe` or skip work when a sufficient bound exists, and `assert_equal` propagates bounds across copy-equal cells. By source reading each cached bound is backed by a gate, lookup, assignment construction, or equality to such a cell, conditional on the unobserved permutation/lookup backend. `assigned_to_le_bits` itself neither reads nor writes that cache. Constraint counts, N1–N16, and IO-3 equivalence must model this history dependence.
- **B4 — AGREES.** For `0 ≤ w ≤ 254`, the recombined sum is `< 2^w ≤ 2^254 < p`, forcing the canonical representative in `[0,2^w)`. For `w = 255`, `enforce_canonical` makes the bit decomposition unique but the range condition is vacuous because every field element fits below `2^255`. For `w > 255`, `assigned_to_le_bits` panics on the host; off-circuit, `w ≥ 255` is rejected. G4-ISA-4 is therefore adopted only for `w ≤ 254`, covering the used widths 1, 8, 12, and 128.
- **B5 — AGREES.** The actual in-circuit width is `n = max(bits + bits % 2, 4)`, so odd or tiny `bits` values give a looser in-circuit domain than the documented/off-circuit domain; for `bits = 128`, `n = 128` exactly. At 128 bits, `lower_than` bounds both operands through `bounded_of_element`/`assert_less_than_pow2`, assigns Boolean `b`, and range-checks `z = 2by − b + x − 2bx − y` below `2^128`; the adopted sufficient field condition is `p ≥ 2^129`, which the literal satisfies by a large margin. The upstream documentation claiming a compile-time restriction around `(NUM_BITS−1)/2`, and the comparison module’s smaller advised bound, are inconsistent with the collected code’s actual `n ≤ NUM_BITS−2 = 253` assertion and remain [RA] for E5 consumers, not aux/2 premises.
- **B6 — AGREES.** In-circuit `Assert` is non-zero only: `assert_non_zero` enforces an inverse relation such as `cond·r = 1`, so any non-zero field element passes. Off-circuit it requires exactly `1`. Therefore every Assert consumer must supply a constrained Boolean; `enabled_fetch` does so for its guarded prefix condition.
- **B7 — AGREES.** The Add citation is refined: `arithmetic.rs:67` declares `linear_combination`; the default `add` calls it. Identity, zero-filtering, single-term, and constant shortcuts may return an existing or fixed cell without an `arith_gate` row, while preserving the semantic output relation.
- **B8 — AGREES.** `circuit_field.rs:331` is the test-module alias, not the production instantiation; the source instantiation is `impl_circuit_field_le!(midnight_curves::Fq, 32)`. The `literal_step.rs:293` call-site line cannot be verified from the unnumbered body and is consistent with `emit_literal_step` at `:253`.
- **B9 — AGREES.** Equality enablement in `NativeChip::configure` and the collected `load_table` source/call in the full synthesis path are source facts; the actual runtime lookup argument, permutation backend, and table loading in an executed build remain [UK]. `cpu_utils` supplies witness computation only and is not a soundness premise.
- **B10 — AGREES.** My prior wording that a free selector, carry, product, column sum `S_k`, or TestEq inverse “breaks determinism (L-5) even if each gate is locally sound” is superseded by the owner’s stronger correction: accepting such a value as an unconstrained substitute is “a **soundness** failure, not only a failure of determinism (L-5).” Supplied witnesses remain permissible only when the forcing equations and ranges still constrain them.
- **B11 — AGREES.** The generic `TableShape`/`constrain` path states only `UInt` as `ConstrainBits{128}` and `Bool`; it never states an 8-bit premise. Byte ranges for fetched carriers must come from per-lane L4 ranges on row cells and robustly be re-imposed on the selected output, as `literal_step` does with `image_ranges` and `b.ranges(&expression.lanes, &selected)`. Range inheritance through the `CondSelect` fold requires Boolean selectors with at most one set. G4-FETCH must state this re-range requirement explicitly and is not established by the generic fetch.
- **B12 — AGREES.** The local `gated_eq` lowering is sound only under its preconditions: when enabled it gives field equality `L ≡ R (mod p)`, and integer meaning requires `L` and `R` to be built from ISA-1 Add/Mul expressions with ranges bounding `|L−R|`; when disabled it is vacuous, but unconditional constraints on feeding temporaries—such as ungated `bits(carry, 9)`, LessThan operand ranges, and the final Assert/borrow condition—must still be satisfiable by disabled witnesses. Soundness is required, not mere determinism of temporaries.
- **B13 — AGREES.** Preprocessing is bypassable as evidence: `IrSource::prove_unchecked` accepts caller-supplied `Preprocessed`, the `mem_insert`/`pi_push` mismatch checks are host `Value` checks rather than constraints, and in-circuit `PublicInput`/`PrivateInput` ignore the guard while assigning unconstrained Native cells. Only synthesis gates, copy constraints, decomposition lookups, and conversions count.

## Both retained R2-4 prongs

Source: `G4-ROOT-SOURCE-REQUIREMENT-ADOPTION-r1.json`, `muse_R2_4_both_prongs_verbatim`.

- **R2-4 can govern only with its stated qualification.** Candidate bytes already place WrongRepresentation before EqScratch.rows. The Opus reading may classify host refusals in that order, and `Q_eq+1` may be reported only as an explicitly labelled lower bound, consistent with the candidate's `needed:">Q_eq"`. If a future host format requires `needed` to be the exact unfolded row count, that interpretation would not govern and would require new reviewed bytes.
I preserve one dissent from any broader reading of the Opus vote: R2-4 must not be read as selecting an exact-need reporting format or a circuit ordering mechanism. It is accepted only as the bounded refusal-classification reading above.

## Retained owner bindings

Source: `reviews/opus-source/RESULT.text.gz`, §7.

The following remain unchanged and binding:
- E1–E7 and E-FUEL;
- IO-1–IO-8;
- P3 conditions 1–8;
- errata R2-1 and R2-3–R2-6, with the Muse R2-4 dissent preserved verbatim in both prongs;
- full39 conditions 1–10 and F1–F7;
- L4;
- C4: no dependent circuit source edit before an admitted A102 compile, and a compile failure means registration revert;
- the old 90s/30s caps, counters and terminal failures;
- the full MoriartyV1 U0–U7 / M1–M17 roadmap.

There is no retry, no new campaign, no resource grant, no command, and native authority stays 0. This review makes no claim about emitters, native code, the SDK, tools, builds, registry authenticity, caller validity, proofs or targets. The complete transitive build, proof backend, table loading at runtime, features and C bindings remain unknown.
