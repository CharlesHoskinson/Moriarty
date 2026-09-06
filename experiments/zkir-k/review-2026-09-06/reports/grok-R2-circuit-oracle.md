# R2: circuit oracle and comparison table

VERDICT: WARNING — the oracle builds the stdlib `MidnightCircuit` the crate keys and proves, with the same instance layout the verifier sees, but D3 names a C outcome for `inv` of zero that the crate realises as W, the generic `ok/violated` cell treats C/W/X as agreement, and the taint walk leaves several named D3 cases as not-comparable.

## Findings

### 1. major — D3 lists `inv` of 0 as constraint-failure; the crate reports witness-consistency first

`experiments/zkir-k/plan-iter3/circuit-comparison-table.md:140-147` (D3, C sub-cell) says an injection that makes an `inv` operand 0 is `C`. The same section's W list (`circuit-comparison-table.md:132-139`) already includes “arithmetic”, which covers `inv`.

What the crate does (`zkir-v3/src/ir_vm.rs:946-949`): `inv_incircuit` then `mem_insert`. Native inversion (`midnight-circuits-7.2.4/src/field/native/native_chip.rs:960-967` and `assign_with_shifted_inverse` at `367-386`) uses `(x - shift).invert().unwrap_or(F::ZERO)` with `shift = 0`, so a zero operand yields hinted inverse `0` without panicking, then `mem_insert` compares `0` to the honest inverse still in the preprocessed memory (`ir_vm.rs:761-777`). `error_if_known_and` (`midnight-proofs-0.8.2/src/circuit/value.rs:92-96`) aborts synthesis with `Error::Synthesis("error_if_known_and")`. `MockProver::run` never reaches `verify`, so the `x * r = 1` gate is not observed.

On every honest `ok` run `preprocess` has already rejected `inv` of zero, so the witness output is a non-zero inverse. Zeroing the operand is therefore `W`, not `C`.

The executable walk does not implement the table's C prediction. `circuit_compare.py:362-366` appends a violation and raises `Undecided`, so `judge` returns `N/C` (`circuit_compare.py:649-651`) and the case is not counted. A walk that followed the markdown would expect `C` and fail against the oracle's `W`.

Fix: delete “an `inv` whose operand became 0” from the D3 C list; treat it as W (recomputed output stored through `mem_insert`). Make the walk `return self.store(o, 0, None, at)` instead of `Undecided`.

### 2. major — generic `ok/violated` cell accepts C, W or X

`circuit-comparison-table.md:70` and `circuit_compare.py:207-212`: any first non-holding verdict other than `commGate` / `fromCoordinates` maps to `Expect(frozenset({C, W, X}))`. `judge` then counts any of those three oracle outcomes as `AGREE`.

D1 (`circuit-comparison-table.md:79-107`) is right that the crate does not expose “unsatisfiable relation” versus “prover abort” as one enum. It is not right to treat that as agreement in the harness. A K `violated` that the crate realises only as `assert_equal` (should be `C`) would still pass if the oracle panics (`X`) or hits `mem_insert` (`W`). That is exactly a modelling error the table exists to catch.

`commGate` is pinned to `C` (`circuit_compare.py:208-209`); native `from_coordinates` is split by `jubjub_from_xy` (`circuit_compare.py:168-185`). Divergence cases pin the known programs (`divergence_tests.py:204-226`). The 2026-09-06c corpus never hit the generic cell (`oracle 2: 0 successful K runs with a non-holding gate`). The cell is still the default for any new `ok/violated` program.

Fix: default `ok/violated` to a single crate realisation per op (constraint ops → `{C}`, `mem_insert` ops → `{W}`, hint-unwrap ops → `{X}`), and keep the C/W/X union only behind an explicit named difference with a decoder, as D1 already does for Jubjub `from_coordinates`.

### 3. major — D3 taint walk does not implement several C cases it names; those injections are not comparable

`circuit-comparison-table.md:140-147` names these as `C`: non-boolean `cond_select` / `not` / `impact` guard or bit; `less_than` operands above the padded bound; `inv` of 0 (finding 1). The walk turns each of them into `Undecided`:

- non-boolean bit/guard: `circuit_compare.py:316-321` (`bool_check`) and `493-497`
- `less_than` above the padded bound: `circuit_compare.py:375-381`
- `inv` of 0: `circuit_compare.py:362-366`

`judge` on `outcomes is None` returns `N/C` (`circuit_compare.py:649-651`). Receipts: `evidence/zkir-k-circuit-differential-92e8bdd3-2026-09-06c.txt` “1 not comparable” (`test_jubjub_point_cond_select_fails_when_bit_zero.zkir inject-input`, `%bit` 1→2); extension receipt two such rows (`test_bool_ops.zkir`, same `cond_select` program). D3 says that injection is `C`. The crate converts the bit (`ir_vm.rs:836-838`, `std.convert` to `AssignedBit`) and would produce a definite outcome.

N/C is not fail-closed against a wrong K model: it is “not counted”. D8 (`ok/unsupported`) is a justified non-comparison (K makes no claim). These D3 N/C rows are convenience: the oracle would still emit one of the six outcomes.

The extension `and` / `or` / `xor` arms (`midnight-zkir-2ffe2d1/zkir/src/ir_vm.rs:1109-1123`) store through `mem_insert` and are unmodelled (`circuit_compare.py:528` `Undecided`). `slice` / `nth` / `concat` over tainted bytes raise `Undecided` (`circuit_compare.py:432-433`) even though 2ffe2d1 uses `mem_insert` for them (`ir_vm.rs:1317-1321`, `1357-1361`, `1388`).

Fix: evaluate the named C/W cases (non-boolean convert → `C`; `inv` of 0 → `W` as in finding 1; `and`/`or`/`xor` as boolean arithmetic + `mem_insert`). Keep `Undecided` only for gadgets whose hint the walk cannot run (hashes with a broken alignment, byte `slice`/`nth` over unknown bytes). If a program’s only Native candidate is undecided, skip the column (`n/r`) rather than emitting N/C.

### 4. major — D3’s “witness-assigned public_input / private_input” column is not what the harness injects

`circuit-comparison-table.md:126-128` and `42-48`: inject-input is “a declared Native input register, or `public_input` / `private_input` output”. `choose_injections` (`circuit_compare.py:614-623`) injects declared Native inputs, then computed outputs, and **skips** `public_input` / `private_input` (`618-619`). Those registers are injected only when K lists them under `unconstrained` (D5, guard 0).

A guarded-on Native `public_input` is witness-assigned in circuit (`ir_vm.rs:978-1003`, `guard: _`, value from `preproc.memory`) and is the D3 case the spike used on `expire` `%t.0` (`evidence/zkir-k-circuit-oracle-spike-2026-09-06.txt:117-119`, `pi_push`). The `--circuit` harness never repeats that injection. D5 only covers the guard-off subset.

Fix: add a `inject-public` candidate: the first Native `public_input`/`private_input` output that is not in `k['unconstrained']`, predicted by the same walk (typically `W` via `pi_push` or a later `mem_insert`).

### 5. minor — witness-consistency versus synthesis is only a tracing substring

`ir_vm.rs:773` and `791` log `error!(..., "Misalignment between `prepare` and `synthesize` runs. This is a bug.")` and then `error_if_known_and` returns `Error::Synthesis("error_if_known_and")`. The stdlib wraps that as `Relation::circuit error: ...` (`midnight-zk-stdlib-2.3.5/src/lib.rs:1883-1887`). The two classes are the same `Err` at the MockProver API.

The oracle (`ledger-92e8bdd3/src/main.rs:75, 194-201, 552-555`) installs a global `tracing` layer and classifies `MockProver::run` `Err` as `witness-consistency-error` iff any ERROR event contains `Misalignment between \`prepare\` and \`synthesize\``. Confirmed on `std_hashes.zkir --inject inject_a.json`: outcome `witness-consistency-error`, `trace_errors` holds that sentence, `message` is `Synthesis("error_if_known_and")`.

If the layer is not the global default, or the crate log string changes, every `W` becomes `S`. No K row currently expects `S` (`circuit-comparison-table.md:114-124`), so that misclassification would `DISAGREE` on inject-computed (`W`) rather than silently pass. It would still scramble `--prove` (`main.rs:446-449`) and any future cell that included `S`.

`catch_unwind` (`main.rs:633-647`) does not hide this path: `mem_insert` returns `Err`, it does not panic. `catch_unwind` does hide oracle panics (lock unwrap, `optimal_k` internals) as crate `panic`, which D2 then matches on the substring `Synthesis` (`circuit_compare.py:206`).

Fix: classify W by the stashed `circuit.take_error()` string `error_if_known_and` **and** the tracing event, and fail closed (unknown-synthesis) if they disagree. Do not treat an oracle-internal panic as D2.

### 6. minor — `--inject` parse failures are reported as `preprocess-error`

`main.rs:508-511`: `inject()` `Err` (non-Native type, non-canonical decimal, missing file) sets `outcome = "preprocess-error"`. That is not `IrSource::preprocess`. The Python harness only writes Native decimals (`circuit_compare.py:77-79`), so the corpus is unaffected. A bad `--inject` file would land in the D9 cell.

Fix: a distinct outcome, or exit 2 as for load errors (`main.rs:600-603`).

### 7. minor — `--pis` / `--binding-input` cannot produce a constraint failure

`main.rs:520-543, 25-28`: after `inject`, `instance = pre.pis.clone()`, so a `--pis` or `--binding-input` change is copied into both the witness `Preprocessed` and the MockProver instance column. `pi_push` (`ir_vm.rs:783-798`) then fires `W` before `constrain_as_public_input` (`ir_vm.rs:1207-1209`). `--instance` is the only path to D6.

Documented, and `diff_test.py --circuit` never passes `--pis`. The flags still exist as a footgun: a caller who thinks `--pis` perturbs “the instance the verifier sees” independently of the witness gets `W` for a disagreement that a real verifier would see as `C`.

Fix: either drop `--pis`/`--binding-input`, or stop copying `pre.pis` into the MockProver instance when they are set.

### 8. minor — `ok/synthErr` message check `Synthesis` is a substring, not a class

`circuit_compare.py:201-206`: chip gating requires `'must enable'`; width requires `'Cannot bound'`; every other `synthErr` requires `'Synthesis'` in the panic message. D2 (`circuit-comparison-table.md:109-124`) is right that `optimal_k`’s `DevAssembly::run(...).unwrap()` (`midnight-zk-stdlib-2.3.5/src/lib.rs:2130-2146` via `cost_model`) turns a witness-independent `Error::Synthesis` into a panic. The substring also matches a panic whose payload merely mentions `Synthesis` for another reason (finding 5). Divergence cases pin the chip/width strings (`divergence_tests.py:209-224`).

Fix: require `called \`Result::unwrap()\`` plus `Synthesis(` for the unwrap class, as D2’s prose already describes.

## Coverage gaps

- Guarded-on Native `public_input` / `private_input` outputs: D3 names them; `choose_injections` never selects them (finding 4). The spike’s `expire --inject %t.0` is not in the `--circuit` receipt.
- D5 is Native-only. `transcripts_guard_off.zkir` reports `unconstrained=%pp,%ps` (`Point<Jubjub>`, `Scalar<Secp256k1>`). Circuit still assigns those cells (`ir_vm.rs:978-1003`) and `encode` of `%pp` (`transcripts_guard_off.zkir:78-84`) reads `%pp`. No injection column exists for that consumer. JubjubScalar canonicity (`native_bytes.zkir` `%c`) is documented as untestable; unconstrained foreign points are not.
- Extension corpus: 0 `inject-unconstrained` rows (`zkir-k-circuit-differential-ext-2ffe2d1-2026-09-06c.txt`). D5 is untested on 2ffe2d1.
- `and` / `or` / `xor`, and tainted `slice`/`nth`/`concat`: walk `Undecided` (finding 3).
- `--circuit` does not exercise `--pis`, `--binding-input`, `--model-only`, `--keygen`, or `--prove`. Provability is a separate 13-proof sample (`provability.py:94-102, 272-278`): seven Moriarty contexts plus six spike preimages. `micro-dao__buyIn` keys at k=16 and is never proved. Injected memories never go through `prove_unchecked`.
- Honest `ok/violated` / `ok/synthErr` never occur on the generated corpus (receipts: `oracle 2: 0`). Those table rows are discharged only by `divergence_tests.py`.
- `synthesis-error` as an oracle outcome is not reached on the corpus (spike finding 6). The class exists for a missing injected register and for `--prove` `Err` without the misalignment log.
- Instance perturbation always does `pis[i] + 1` (`circuit_compare.py:642-646`). No length change, no perturbation of the empty committed-instance column.

## Questions for the authors

1. For an injection that zeroes an `inv` operand, do you want the table to follow `mem_insert` (W, finding 1) or the unsatisfiable `x * r = 1` gate (C, which `MockProver::run` does not report)? The markdown currently says both.
2. `Relation::circuit` ignores `_instance` (`ir_vm.rs:709`). Public inputs enter the constraint system only through `pi_push` then `constrain_as_public_input`. Is `--instance` therefore defined to be “the MockProver instance columns”, not “the `Value<Instance>` handed to the relation”? That matches `midnight_zk_stdlib::prove` (`lib.rs:2004-2009`: `&[com_inst.as_slice(), &pi]`), but it is not the `MidnightCircuit` instance field.
3. `optimal_k` (`lib.rs:2130-2146`) searches `k = 9..=25` and may return a cost-model `k` below 9 (receipts: `native_identity` keygen `k=4`). MockProver then uses `RowSizer::min_k` for the row count (`midnight-proofs-0.8.2/src/dev/mod.rs:811-824`) while chips are configured with `(used_chips(), k-1)` (`lib.rs:1853-1854`). The spike recorded `rows = 2^k` on every run it printed. Is a MockProver accept at that `k` claimed to be the same constraint system as `setup_vk` on `bls_midnight_2p{k}`, or only “close enough for accept/reject”?
4. `--prove` verifies with `VerifierKey::verify(&PARAMS_VERIFIER, &proof, pis)` (`main.rs:460`), which is `midnight_zk_stdlib::verify::<DummyRelation, ...>` (`transient-crypto/src/proofs.rs:554-566, 437-460`). That is the ledger’s check (`ledger/src/structure.rs` uses the same `PARAMS_VERIFIER`). `PARAMS_VERIFIER` is `s_g2` from `bls_midnight_2p14` (`proofs.rs:121-128`; KZG verifier params carry no `k`, `midnight-proofs-0.8.2/src/poly/kzg/params.rs:235-267`). Is a k=15/16 proof that verifies here (receipt: `std_hashes` k=15, `test_curve25519_ec_mul` keys at 16) intended to be “the ledger would accept this proof and these PIs”, and not “this Midnight transaction would succeed”?
5. Should `catch_unwind` around `optimal_k` (D2) stay classified as `panic`, matching `setup_vk`’s `expect("keygen_vk should not fail")` (`lib.rs:1952`), or be mapped to `synthesis-error` so that MockProver mode and `--keygen` share a vocabulary? `--keygen` already documents that the `Err` arm is dead (`provability.py:203-204`); negative controls are all `panic`.

## What was checked

Read, not merely listed:

- `experiments/zkir-k/tools/circuit-oracle/ledger-92e8bdd3/src/main.rs` and the 2ffe2d1 copy (differ only in `midnight_zkir_v3` vs `midnight_zkir` imports; `diff -u` of the two `main.rs` files).
- `experiments/zkir-k/tools/circuit_compare.py` (walk, table cells, `choose_injections`, `judge`).
- `experiments/zkir-k/tools/diff_test.py` `--circuit` (`217-241`, `293-331`, `350-390`).
- `experiments/zkir-k/tools/divergence_tests.py` `CIRCUIT` (`204-226`) and the comparison at `289-292`.
- `experiments/zkir-k/tools/provability.py`.
- `experiments/zkir-k/plan-iter3/circuit-comparison-table.md`, `PLAN.md`.
- Receipts: `evidence/zkir-k-circuit-oracle-spike-2026-09-06.txt`, `zkir-k-circuit-differential-92e8bdd3-2026-09-06c.txt` (464 comparisons, 463 AGREE, 0 DISAGREE, 1 N/C), `zkir-k-circuit-differential-ext-2ffe2d1-2026-09-06c.txt` (498, 496 AGREE, 2 N/C), `zkir-k-provability-2026-09-06b.txt` (122 keyed, 13 proved, 16/16 negative controls), `zkir-k-divergence-tests-2026-09-06c.txt`.
- Crate: `repos/_build/ledger-92e8bdd3/zkir-v3/src/ir.rs` (`Zkir::prove`/`k`/`keygen`, `prove_unchecked`), `ir_vm.rs` (`preprocess`, `Relation::circuit`, `mem_insert`, `pi_push`, `used_chips`, bypass `memory.insert` at 1115-1149), `ir_instructions/inv.rs`, `from_coordinates.rs`; `midnight-zkir-2ffe2d1/zkir/src/ir_vm.rs` (extension `and`/`or`/`xor`/`slice`/`nth`/`concat`/`Reverse`).
- stdlib 2.3.5: `MidnightCircuit::new`/`synthesize` (table load, `used_chips` via `params()`), `prove` instance layout, `setup_vk`/`optimal_k`; `circuit-params` enabled on `midnight-proofs` by stdlib (`Cargo.toml` features `circuit-params`, `committed-instances`), so `MockProver::run` uses `configure_with_params`, not `ZkStdLibArch::default()`.
- `midnight-proofs-0.8.2` `MockProver::run` / `RowSizer::min_k`; `error_if_known_and`; KZG `ParamsVerifierKZG`.
- `transient-crypto/src/proofs.rs` `PARAMS_VERIFIER`, `VerifierKey::verify`, `DummyRelation`.
- `zkir-v3/tests/common/mod.rs` `TestParams` / seed `[42; 32]`.
- Docs 08 (witness space, D5), 12, 16 (tier three vs provability).
- Prior audit `review-2026-09-05/CONSOLIDATED.md`: none of these oracle/table items are regressions of accepted 2026-09-05 findings.

Commands:

- `diff -u` of the two oracle `main.rs` files (crate-name imports only).
- `/home/charl/Moriarty/repos/_build/ledger-92e8bdd3/target/release/zkir-circuit-oracle` on `handmade/std_hashes.zkir` + spike `std_hashes.json` `--inject inject_a.json` → `witness-consistency-error` with the misalignment trace on `%s1` (output redirected under `/tmp/claude-1000/-home-charl/7cf55fca-6b70-4fce-8aa0-2e793cb52ef2/scratchpad/inject_a.out`).

Did not re-run the full `--circuit` corpus or `provability.py`. Did not re-kompile.
