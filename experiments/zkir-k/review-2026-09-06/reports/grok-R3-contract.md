# R3: the target contract and the static typing

VERDICT: BLOCKED — `targetContract` is not equivalent to unknown-witness keygen in either direction, so chapter 16 cannot be quoted as “the circuit can be built and keyed”.

## Findings

### Blocker — alignment operand arity is not a tier-one obligation

- **Where.** `zkir-contract.k:160` (`defect(align(Op), …)`), `zkir-static.k:214-230` (`#onlyAtoms`, `#hasCompress`, `alignmentOf`). Crate: `ir_vm.rs:79-178` (`fab_decode_to_bytes` / `fab_decode_to_bytes_atom`) at midnight-ledger 92e8bdd3, same helper at midnight-zkir 2ffe2d1 `ir_vm.rs:104`.
- **What K says.** `alignment.persistent_hash`, `alignment.keccak256` and `alignment.sha512` fail only on an `option` segment or a `compress` atom. They do not count Native operands against the layout. `circuit.static.dispatch` for those instructions only requires each operand to convert to Native (`zkir-contract.k:298-300`, `zkir-ext.k:453`).
- **What the crate says.** `AlignmentAtom::Field` consumes one assigned Native or returns `Synthesis("cannot decode field element from no data")` (`ir_vm.rs:134-137`). `AlignmentAtom::Bytes { length }` consumes `length/31 + (length%31 != 0)` Natives or returns `Synthesis("cannot decode bytes from to little data")` (`ir_vm.rs:145-159`). Both are independent of the witness. `optimal_k` unwraps them, so `--keygen` is `panic`.
- **Why it matters.** A compiler can emit a well-typed hash whose operand list is too short. The contract reports every obligation `met` or `notApplicable`. Keygen cannot build the circuit.

  Program A (scratchpad `align_field_no_inputs.zkir`): `persistent_hash` with alignment `[field]` and `inputs: []`. Contract: `ok (4 met, 10 n/a)`. Keygen: `panic`, message `cannot decode field element from no data`.

  Program B (scratchpad `align_bytes32_one_input.zkir`): `persistent_hash` with alignment `bytes length 32` (needs two field elements) and one Native input. Contract: `ok (4 met, 10 n/a)`. Keygen: `panic`, message `cannot decode bytes from to little data`.

  The same gap applies to `keccak256` on both surfaces and to `sha512` on 2ffe2d1, which share `fab_decode_to_bytes`. Surplus operands are ignored by the decoder and are not a keygen failure.
- **Fix.** Add a static expected-arity function from the alignment (1 per `field` atom, `ceil(length/31)` per `bytes` atom) and fail `alignment.*` when `lenOperands` is strictly smaller. Keep option/compress as they are. Put a program of each shape in `NEGATIVE_CONTROLS`.

### Major — `wf` and the 248-bit width obligations reject programs that keygen accepts

- **Where.** `zkir-syntax.k:357-364` (`#checkArity` for `div_mod_power_of_two`, `reconstitute_field`, `constrain_bits`, `less_than`), `zkir-syntax.k:336-337` (reassignment), `zkir-contract.k:156-159` (`width.div_mod_power_of_two`, `width.reconstitute_field`), `zkir-contract.k:154-155` (`width.constrain_bits`). Chapter 16 table rows `wf`, `width.div_mod_power_of_two`, `width.reconstitute_field`, `width.constrain_bits`. Header claim `zkir-contract.k:44-47`.
- **What K says.** `targetContract` fails when `wf` fails. `wf` rejects reassignment, `constrain_bits`/`less_than` with `bits >= 255`, and `div_mod`/`reconstitute_field` with `bits > 248`. The width obligations repeat the 248 bound and reject `constrain_bits` only when `bits > 255`.
- **What the crate says.** Unknown-witness `Relation::circuit` does not enforce those three bounds.

  | Program | Contract | Keygen |
  |---|---|---|
  | `corpus/handmade-negative/reassignment.zkir` (copy `%a` to `%b`, then `add` into `%b`) | `wf: reassignment of %b` | `accepted` at k=4 |
  | `corpus/handmade-negative/excessive_bits.zkir` (`constrain_bits` 255) | `wf: constrain_bits: excessive bit bound` | `accepted` at k=9 |
  | `div_mod_power_of_two` bits=249, two outputs | `wf` and `width.div_mod_power_of_two` | `accepted` at k=9 |
  | `reconstitute_field` bits=249 | `wf` and `width.reconstitute_field` | `accepted` at k=9 |

  Reassignment: `circuit` overwrites through `mem_insert` (`ir_vm.rs:761-780`), the same as `preprocess`. Single assignment is a K-only static check (CLM-0711, `zkir-syntax.k:287-288`).

  `constrain_bits` 255: `assigned_to_le_bits` asserts `nb_bits <= F::NUM_BITS` (`native_gadget.rs:894-897`). `F::NUM_BITS` is 255, so 255 is legal and canonicity is enforced (`native_gadget.rs:906-917`). `preprocess` rejects 255 (`ir_vm.rs:268-269`, `n >= FR_BITS`). `width.constrain_bits` matches the circuit (`N > 255`). `wf` matches `preprocess` (`B >=Int #frBits`). They disagree on 255.

  `div_mod` bits=249: the circuit takes 255 bits from `assigned_to_le_bits(..., None, true)` and slices at `bits` (`ir_vm.rs:1005-1023`). There is no `FR_BYTES_STORED * 8` test. The 248 test is only in `preprocess` (`ir_vm.rs:399-400`).

  `reconstitute_field` bits=249: the circuit does `assert_lower_than_fixed` against `1 << (FR_BITS - bits)` and `1 << bits` (`ir_vm.rs:1036-1045`). That is a witness bound, not a construction failure. The 248 test is only in `preprocess` (`ir_vm.rs:418-419`).
- **Why it matters.** Chapter 16 line 30 and `zkir-contract.k:44-47` state that unknown-witness keygen “performs exactly these checks and no other”, so a failed obligation names a synthesis failure. That is false. A Moriarty compiler that emits `constrain_bits` 255, `div_mod` 249, or a register reuse is rejected by the contract and accepted by `keygen_vk`. The other direction (finding 1) is worse. This direction means the contract is not a keygen oracle.

  `tools/provability.py:66-86` never keys a `wf` failure unless it is in `NEGATIVE_CONTROLS`. `reassignment.zkir`, `excessive_bits.zkir` and any 249-bit program are “not keyed”. Receipt `evidence/zkir-k-provability-2026-09-06b.txt:209-211` (“0 contradictions”) does not test these programs.
- **Fix.** Split the layers. Keep `wf` as “this definition will run `checkedJob`”, and say so. For the keygen-equivalent slice, drop the 248-bit width obligations (or mark them `info` as preprocess-only), and change `wf`’s `constrain_bits` test to `B >Int #frBits` so 255 matches `assigned_to_le_bits`. Put `reassignment.zkir` and `excessive_bits.zkir` in the provability sweep as named extra-strict cases, not as negative controls for keygen. Cite `preprocess` for 248 and `assigned_to_le_bits` / `Relation::circuit` for 255.

### Major — source-of-truth citations mix `preprocess` with `circuit`

- **Where.** Chapter 16 table (`docs/16-compilation-target-contract.md:11-27`): rows `wf`, `width.div_mod_power_of_two`, `width.reconstitute_field`. `zkir-contract.k:17-18`.
- **What K says.** Those width rows are grouped with chip gating, `less_than` 253 and `constrain_bits` 255 as “the program-only half of the synthErr outcomes” (`zkir-contract.k:41-43`).
- **What the crate says.** `width.less_than` is a real construction assert (`bounded_of_element`, `MAX_BOUND_IN_BITS = F::NUM_BITS - 2 = 253`, `native_gadget.rs:330-344`, `ir_vm.rs:1131`). `width.constrain_bits` is a real construction assert (`assigned_to_le_bits`, `native_gadget.rs:894-897`). `width.div_mod_power_of_two` and `width.reconstitute_field` are `preprocess` `bail!("Excessive bit count")` only. `wf`’s single-assignment and `constrain_bits >= 255` are not `IrSource::load` (`ir.rs:969-970` accepts `minor: 0..=0`) and not `circuit`.
- **Why it matters.** A compiler specification that quotes the table will treat 248-bit `div_mod` as a keygen obligation. Finding 2 shows keygen accepts 249. Chip gating, alignment option/compress, `circuit.static.*` and `used_chips` are cited against the right crate functions (see “What I checked”).
- **Fix.** Split the table into “`IrSource::load` / `wf`”, “`preprocess` structural”, and “`Relation::circuit` / `used_chips` / stdlib asserts”. Move 248-bit rows to the preprocess column.

### Major — `obs(Status, Outputs, Pi)` is not the Compact prove interface

- **Where.** `zkir-vm.k:542-556` (`observable` / `#observable`), `zkir-constraints.k:144-147` (`#encodeAll`), `zkir_run.py:293-307`, `zkir_kast.py:474-488` (`tier_two` copies `observable` only). Crate: `ir.rs:82-102` (`Zkir::prove` returns `(Proof, Vec<Fr>, Vec<Option<usize>>)`), `ir_vm.rs:508-523` (`pi_skips`).
- **What K says.** `[[P]](pre) = obs(status, encoded outputs, public inputs)`. Chapter 16 line 51: a compiler-correctness statement quotes that triple. Completeness mentions status `ok`, circuit accept, and public inputs. Soundness quantifies over memories in `witnessSpace` with their public-input vector. `<skips>` is a VM cell (`zkir-vm.k:75`) and is existentially abstracted in `claims/spec-compiled-observable.k:18`.
- **What the crate says.** `prove` returns the PI vector and `pi_skips`. A guarded-off impact pushes zeros and `Some(count)`. An active impact of zeros pushes zeros and `None`. The two are equal in `Pi` and distinct in `pi_skips`. `Zkir::check` returns only the skip vector. The binding input is `Pi[0]`. The communications commitment, when the flag is set, is `Pi[1]`. The opening is witness-only.
- **Why it matters.** Compact reconstructs the public transcript from PIs plus skips. A source execution that impacts 0 is not the same as a skipped impact. `obs` cannot state that distinction. Completeness as written also omits encoded-output equality, even though the third component of `obs` is exactly those encodings (`encodeValue`, the commitment preimage). Soundness as written is not a statement about `obs` at all. It ranges over `<mem>` and `<unconstrainedRegs>`, which `zkir_kast.py contract --preimage` reports next to `observable` but not inside it.
- **Fix.** Either add `skips` to `Observable` (`obs(Status, List, List, List)` or a named record) and pin `<skips>` in `spec_compiled_observable`, or state in chapter 16 that Compact compiler-correctness has a fifth component outside `[[P]]`. Name encoded outputs in the completeness sentence. Keep the opening out of `obs`. Keep the binding input and the commitment as `Pi[0]` and `Pi[1]`.

### Major — compiler-correctness is not stated at the strength the definition supports

- **Where.** `docs/16-compilation-target-contract.md:53`, `plan-iter3/PLAN.md:51-52`, `zkir-vm.k:548-550`, `claims/spec-compiled-observable.k`, `zkir_kast.py:491-505` (tier three is the oracle binary).
- **What is written.** Completeness: an honest source run yields a preimage such that `[[P]](pre)` has status `ok`, the circuit accepts `pre` (tier three), and the public inputs match the source. PLAN.md: “This is what the oracle tests.” Soundness: every memory in the modelled witness space (with its PI vector, in chapter 16) corresponds to a source execution, residual list from chapter 08.
- **What the definition supports.** `observable` is a total function of `<status>`, `<outputs>`, `<pi>` after `job`. That supports “`job(P, Pre)` reaches `obs(ok, encoded outputs, pis)`”, which is the claims template. It does not contain a K predicate “the circuit accepts”. Tier three is `zkir-circuit-oracle` (`MockProver::verify`). PLAN.md’s “the oracle tests” completeness is false: `diff_test.py --circuit` compares K verdicts to MockProver on a preimage. It does not compare a source semantics to `[[P]]`. Soundness as written needs a source semantics and a quantification over `witnessSpace`. Neither is a function of `obs`. Chapter 16 line 117 already says the three tiers do not decide soundness. PLAN.md still presents both directions as the statement Moriarty will make at this interface.
- **Why it matters.** A compiler specification that quotes chapter 16 will think `obs` plus `targetContract` is enough for both directions. Completeness as a K claim is reachability of `obs`. Circuit accept is an empirical side condition. Soundness is a claim about `witnessSpace`, not about `obs`, and it is unproved.
- **Fix.** Write completeness as: source honest preimage `pre` implies `[[P]](pre) = obs(ok, enc(outs), pis)` with `pis` and `skips` equal to the source prediction, and separately “MockProver accepts `(P, pre)` on the corpus”. Write soundness as: every `mem` with `witnessSpace(verdicts, mem)` (residual list of chapter 08) implies a source execution, and say that this is not a statement about `obs`. Do not say the oracle tests completeness.

### Minor — `wf` and `width.constrain_bits` disagree on 255

- **Where.** `zkir-syntax.k:361-362` (`B >=Int #frBits`), `zkir-contract.k:154-155` (`N >Int 255`).
- **What K says.** `constrain_bits` 255 fails `wf` and meets `width.constrain_bits`.
- **What the crate says.** Circuit accepts 255. `preprocess` rejects 255.
- **Why it matters.** Two obligations in the same `targetContract` list, both presented as keygen checks, give opposite answers on one program (`excessive_bits.zkir`).
- **Fix.** Same as finding 2: `wf` uses `>`, width stays `> 255`, and the chapter says which layer each belongs to.

### Minor — a non-empty output signature with no `output` instruction keys

- **Where.** `zkir-contract.k:148, 187-200` (`applies(outputSig)` only on `output(_)`). Crate: `I::Output` (`ir_vm.rs:1400-1418` at 2ffe2d1, same arm at 92e8bdd3).
- **What K says.** `circuit.static.output` is `notApplicable` when no `output` instruction occurs. A program with `outputs: ["Scalar<BLS12-381>"]` and `instructions: []` is `ok (2 met, 12 n/a)`.
- **What the crate says.** Keygen accepted that program at k=4. The signature is checked only when an `output` instruction runs.
- **Why it matters.** A compiler can declare return types that the circuit never constrains. Not a keygen contradiction. It is a missing “signature is realised” obligation if Moriarty wants `output` to be the compiled result.
- **Fix.** Fail `circuit.static.output` when `lenIrTypes(Outs) > 0` and no `output` instruction occurs, or document that the signature is advisory until `output` appears.

### Nit — `version` is `info`, not a pin

- **Where.** `zkir-contract.k:73`, `zkir-syntax.k:310-311`, `ir.rs:969-970`.
- **What K says.** `version` is always `info("3." + minor)`. `wf` rejects `minor != 0`. The preprocessor rejects `3.1` as format (`handmade-negative/wrong_version.zkir`).
- **What the crate says.** `IrSource::load` accepts `major=3, minor=0` only.
- **Why it matters.** The table row cites `IrSource::load` as the source of truth for an `info` fact. The pin is `wf` plus the preprocessor. Harmless if those stay in the contract.
- **Fix.** Either fail `version` when `minor != 0`, or cite `wf` in the table.

### Nit — `#firstKnown` binds a type when only one operand is known

- **Where.** `zkir-static.k:93-95, 120, 138-139`.
- **What K says.** `add`, `mul` and `cond_select` bind the first known operand type. `defect(dispatch)` for those instructions fires only when both types are known (`zkir-contract.k:251-254`).
- **What the crate says.** The in-circuit arm requires equal supported types. If the unknown operand is a different type, synthesis fails at that instruction.
- **Why it matters.** This is a guess. On the corpus, unknown types are rare: inputs are declared, and most `bindOuts` arms are total in the operand types. I did not exhibit a well-formed program that reaches `unknownT` without an earlier skipped dispatch. The skip of unknown in `defect` is conservative in the direction chapter 16 claims (no fail on a guess). A later obligation that trusted the guessed type could fail too strictly. I did not find such a program.
- **Fix.** Bind only when both operand types are known and equal (and in `selectT`/`arithT`/`fieldT`). Otherwise leave the output unknown.

## Coverage gaps

Instructions and checks the corpus plus `provability.py` do not exercise, relative to what keygen decides without a witness:

- Hash alignment operand count versus layout (`field` / `bytes` atoms) on `persistent_hash`, `keccak256`, `sha512`. Finding 1. No corpus file.
- `constrain_bits` 255 and 0. 255 is `excessive_bits.zkir` (wf only, never keyed). 0 keys and the contract passes (scratchpad).
- `div_mod_power_of_two` / `reconstitute_field` with `249 <= bits <= 255` (keygen accepts, contract fails).
- Reassignment (`handmade-negative/reassignment.zkir`) as a keygen control.
- Non-empty `outputs` signature with no `output` instruction.
- `keccak256` compress atom (scratchpad `compress_align.zkir` fails `alignment.keccak256` and keygen, good, not in `EXPECTED`).
- `concat` result length above `MAX_BYTES_LEN` with statically known lengths (`zkir-ext.k:463-465`). The crate test corpus does not include it.
- `nth` / `slice` of `Bytes<32>` (`bytes32()` in the static environment) versus `Bytes<n>` for `n != 32`. `staticBytesLen` treats `bytes32()` as 32 (`zkir-ext.k:96-97`). 2ffe2d1 stores both as `CircuitValue::Bytes`. Not a suspected defect. Not in the contract corpus as a distinct case.
- `and` / `or` / `xor` with a known non-Bool input (bool_gate). Empty inputs are covered (`test_bool_gate_empty_inputs_fails.zkir`).
- `load_constant` of a chipped type other than Jubjub (`k08` is Jubjub only).
- `ec_mul` of mismatched point/scalar pairs other than `f12` (P256 generator).
- Two `output` instructions against the same signature (each is checked, keygen still succeeds).
- Unknown static types feeding later `circuit.static.*` checks (chapter 16’s stated residual). No corpus program relies on `unknownT`.

Error paths that are witness-dependent (tier two, correctly excluded from tier one): non-boolean guards, `inv` of zero, transcript length, commitment mismatch, `constrain_bits` value bounds, off-curve points.

Preimage shapes: `contract_corpus.py` runs tiers two and three only on the seven Moriarty contexts. The rest of the corpus is tier one only. That is enough for the keygen claim if the obligation set is complete. It is not (finding 1).

## Questions for the authors

1. Is `wf` allowed to be strictly stronger than keygen in the compiler contract (single assignment, `constrain_bits` 255, 248-bit `div_mod` / `reconstitute_field`)? If yes, chapter 16 and `zkir-contract.k:44-47` must stop saying keygen performs exactly these checks. If no, those tests have to move out of the keygen-equivalent slice.
2. Should alignment operand arity be a failed obligation or an accepted residual of “unknown / not modelled”? Keygen already decides it without a witness.
3. Is Compact compiler-correctness supposed to include `pi_skips`? If yes, `obs` is the wrong interface as defined.
4. Should completeness mention encoded outputs, or are Moriarty circuits specified to have empty `output` lists (the seven contexts all have 0 output elements)?
5. Is “the circuit accepts `pre`” a conjunct of the K-quoted statement, or an empirical condition on the oracle? The definition cannot state it.
6. For `#firstKnown` and `slice`/`nth`/`and` binding an output type when an operand is unknown: is the intended invariant “assigned type is the circuit type if the instruction succeeds”, or “assigned type is unknown unless every operand type is known”?

## What I checked and how

Read, did not modify: `experiments/zkir-k/semantics/zkir-static.k`, `zkir-contract.k`, `zkir-contract-main.k`, `zkir-ext.k` (modules `ZKIR-EXT-STATIC`, `ZKIR-EXT-CONTRACT`), `zkir-syntax.k` (`ZKIR-WF`), `zkir-vm.k` (`observable`, `<skips>`), `zkir-values.k` (`decodeStrict`), `zkir-constraints.k` (`#encodeAll`). Tools: `zkir_kast.py` (`contract`, `tier_one`/`two`/`three`), `contract_corpus.py` (`EXPECTED`, `NEGATIVE` programs), `provability.py`. Docs: `docs/16-compilation-target-contract.md`, `docs/08-constraints-and-verdicts.md`, `plan-iter3/PLAN.md`, `claims/spec-compiled-observable.k`, `claims/README.md`. Receipts: `evidence/zkir-k-contract-corpus-2026-09-06c.txt`, `evidence/zkir-k-provability-2026-09-06b.txt`. Prior audit: `review-2026-09-05/CONSOLIDATED.md` (not re-reported).

Crate, both surfaces: `used_chips` at 92e8bdd3 `ir_vm.rs:1212-1262` and 2ffe2d1 `ir_vm.rs:1461-1508` (K `usedChips` matches: input types, `public_input`/`private_input` types, `hash_to_curve`/`transient_hash`/`persistent_hash`/`keccak256`/`sha512`, commitment Poseidon. Not `from_bytes32`, `load_constant`, `jubjub_scalar_from_native`, native `from_coordinates`). `Relation::circuit` dispatch and static arms: `I::Output`, `resolve_bit_list`, `I::LoadConstant` + `decode_offcircuit` (canonical re-encode at 2ffe2d1), `I::Nth`/`I::Slice`/`I::Concat`, `fab_decode_to_bytes`, `add.rs`/`select.rs`/`neg.rs`/`mul.rs` type sets, `I::EcMulGenerator`, `assigned_to_le_bits`, `bounded_of_element`. `ir.rs:82-102` prove/skips. `ir_types.rs` `MAX_BYTES_LEN`, `CircuitValue` at 2ffe2d1 (no separate `Bytes32` variant).

Commands (from the repository root, output under `/tmp/claude-1000/-home-charl/7cf55fca-6b70-4fce-8aa0-2e793cb52ef2/scratchpad/r3/`):

```
uv run --group zkir-k python experiments/zkir-k/tools/zkir_kast.py contract <program>
/home/charl/Moriarty/repos/_build/ledger-92e8bdd3/target/release/zkir-circuit-oracle <program> --keygen --params /home/charl/Moriarty/repos/_build/params
```

on `align_field_no_inputs.zkir`, `align_bytes32_one_input.zkir`, `constrain_bits_0.zkir`, `constrain_bits_255.zkir`, `compress_align.zkir`, `output_sig_no_instr.zkir`, `divmod_249.zkir`, `reconstitute_249.zkir` (scratchpad), and corpus `reassignment.zkir`, `excessive_bits.zkir`, `undefined_variable.zkir`.

Static-type soundness: `bindOuts` / `stype` / `chipNeeds` against the in-circuit output types and `ZkStdLib` accessors. I did not find a program where a *known* binding is a different type than the circuit would assign on success. Incompleteness (`unknownT`) is skipped by `defect` as chapter 16 claims. The contradictions above are missing or extra obligations, not a wrong `known(T)`.

Citation check that held: `chips.enabled` / `chips.gating` versus `used_chips` and the stdlib `must enable` panics (`f13`, `k01b`, `k04`, `k08`). `width.less_than` versus `bounded_of_element` (`k05`). Alignment option (`k07`) and compress (scratchpad). `circuit.static.output` versus `I::Output` (`output_operand_type_mismatch`). `circuit.static.bool_gate` versus `resolve_bit_list`. `circuit.static.constant` versus `decode_offcircuit`. `circuit.static.bytes_bounds` versus `I::Nth`/`I::Slice`. `circuit.static.dispatch` versus the `_ =>` arms sampled (`f06`, `f07`, `f08`, `f12`). Both surfaces: `ZKIR-EXT-CONTRACT` extends `negT`/`selectT`/`eqT` with Bool/Byte/`Bytes<n>`, and `usedChips` grows `sha2_512` only on `sha512`.
