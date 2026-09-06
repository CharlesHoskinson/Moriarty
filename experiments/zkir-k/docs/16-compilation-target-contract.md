# The compilation target contract

The contract is the list of obligations a compiler must meet for its ZKIR output to be well formed, provable and semantically defined by this definition. It is stated once, as K functions over the program and the configuration, and a specification of a compiler that targets ZKIR quotes it rather than restating the crate's checks. It has three tiers. Tier one is decided from the program alone and answers whether the circuit can be built and keyed. Tier two needs a preimage and answers whether the witness computation succeeds on it. Tier three runs the crate's circuit on that preimage under the MockProver and answers whether the circuit accepts it. The observable semantics `[[P]](pre)`, the triple a compiler-correctness statement talks about, is a fourth piece of the same module set.

## Tier one: program-only obligations

`targetContract(P)` in `zkir-contract.k` (module `ZKIR-CONTRACT`) is a total function from `Program` to `Obligations`, a list of `obligation(name, status)` with status `met()`, `failed(msg)`, `notApplicable()` when no instruction of that kind occurs, or `info(fact)` for a fact the later tiers quote. The main module `ZKIR-CONTRACT-MAIN` in `zkir-contract-main.k` rewrites a `Program` in `<k>` to that list; it imports `ZKIR-EXT-CONTRACT` from `zkir-ext.k` so the same compiled definition serves both surfaces. Twenty obligations are emitted, in this order.

| Obligation | What is decided | Source of truth in the crate |
|---|---|---|
| `wf` | the static check `wf` of `ZKIR-WF` (10-well-formedness-and-static-checks.md) | `IrSource::load` and the structural checks of `preprocess` in `ir_vm.rs` |
| `version` | info: `3.<minor>` | `IrSource::load` accepts version 3.0 only |
| `surface` | info: base, extension, or common to both; failed when `reverse_bytes` meets an extension type or instruction | the instruction and type enums of `ir.rs` and `ir_types.rs` at each commit |
| `chips.enabled` | info: the chip set the program initialises (`usedChips`, `zkir-static.k`) | `IrSource::used_chips` in `ir_vm.rs` |
| `chips.gating` | no instruction's gate asks for a chip that set omits (`chipNeeds`) | the `ZkStdLib` accessors of midnight-zk-stdlib 2.3.5 `src/lib.rs`, which panic with `ZkStdLibArch must enable ...` |
| `width.less_than` | the padded width `max(bits + bits mod 2, 4)` is at most 253 | `bounded_of_element` in midnight-circuits 7.2.4 `field/native/native_gadget.rs`, `MAX_BOUND_IN_BITS` |
| `width.constrain_bits` | width at most 255 | `assigned_to_le_bits` |
| `width.div_mod_power_of_two` | bits at most 248 | `FR_BYTES_STORED * 8` in `ir_vm.rs` |
| `width.reconstitute_field` | bits at most 248 | the same |
| `alignment.persistent_hash`, `alignment.keccak256`, `alignment.sha512` | no `option` segment, no `compress` atom | `fab_decode_to_bytes` in `ir_vm.rs` |
| `circuit.static.output` | the arity of `output` matches the signature, and every operand whose static type is known has the declared type | the `I::Output` arm of `Relation::circuit` in `ir_vm.rs` |
| `circuit.static.bool_gate` | `and`, `or`, `xor` have at least one input, and every input of known type is a `Bool` | `resolve_bit_list` and `AssignedBit::try_from` (midnight-zkir 2ffe2d1 `ir_vm.rs`) |
| `circuit.static.constant` | every `load_constant` encoding decodes on the strict decoder, `decodeStrict(_, _, true)` of `ZKIR-VALUES` with the arms of `ZKIR-EXT-VALUES` | the `I::LoadConstant` arm, which calls `decode_offcircuit` |
| `circuit.static.bytes_bounds` | `nth` and `slice` index within the static length of their byte string, a slice length is at least 1, and the operand is a byte string | the `I::Nth` and `I::Slice` arms and `TryFrom<CircuitValue>` for `Vec<AssignedByte>` |
| `circuit.static.dispatch` | no instruction's in-circuit dispatch is unsupported for the static types of its operands | the `_ =>` arms of the `*_incircuit` functions in `ir_instructions/*.rs`, `TryFrom<CircuitValue>` in `ir_types.rs`, the `I::EcMulGenerator` and `I::Encode` arms |
| `commitment` | info: the value of `do_communications_commitment` and what the preimage must carry | the commitment block of `preprocess` and `circuit` |
| `inventory.instructions`, `inventory.types` | info: instruction counts by op, the types declared | the program |

The `circuit.static` obligations are decided on the forward type environment of `ZKIR-STATIC` (`inputEnv`, `bindOuts`, `stype`, `zkir-static.k`): inputs carry their declared type, every instruction whose output type is fixed by its operand types binds its outputs, and a register whose type needs a value stays unknown. A check on an unknown type is never reported; it stays with the gate that evaluates it on the witness (08-constraints-and-verdicts.md). The dispatch tables are type predicates: `arithT` is the set of `add.rs`, `fieldT` the set of `mul.rs`, `inv.rs`, `into_bytes32.rs` and `from_bytes32.rs`, `eqT` adds `Bytes32` for `constrain_eq.rs` and `eq.rs`, `selectT` follows `select.rs`, and `ZKIR-EXT-CONTRACT` extends `negT`, `selectT` and `eqT` with the `Bool`, `Byte` and `Bytes<n>` arms of 2ffe2d1. Two byte strings of different lengths are different types in the environment, so `constrain_eq` on `Bytes<4>` and `Bytes<5>` fails the obligation as it fails synthesis.

Unknown-witness key generation performs exactly these checks before it has a witness. `tools/provability.py` runs `zkir-circuit-oracle --keygen` on every corpus program whose tier one is met and on every program whose tier one fails as a negative control; the receipt `evidence/zkir-k-provability-2026-09-06b.txt` records 122 programs keyed, all accepted, and 16 negative controls, all failing with a panic or a synthesis error that names the same instruction as the failed obligation. The corpus receipt `evidence/zkir-k-contract-corpus-2026-09-06c.txt` lists the expected failures: the chip-gating, width and alignment cases of `corpus/divergence/`, the ten programs whose keygen fails on an in-circuit static check, and the ill-formed programs.

## Tier two: the preimage-dependent conditions

Tier two is the run of `job(P, Pre)` on `ZKIR-VM`, the raw entry point that models `preprocess` (06-configuration-and-run-lifecycle.md). Its conditions are the final `<status>` being `ok()` and the `<witnessSpace>` cell being true: every emitted relation holds or is `unconstrained`, and every register holds a well-typed value. The registers in `<unconstrainedRegs>` are reported with it, because a compiler that relies on such a register being determined by the circuit relies on something the model does not claim. `tools/zkir_kast.py contract --preimage` reports the tier as the object `preimage_dependent` with the members `status`, `error` when the status is not `ok`, `witness_space`, `unconstrained`, `violations`, `observable` and `met`.

## Tier three: circuit acceptance

Tier three is the outcome of `zkir-circuit-oracle` on the program and the preimage: `preprocess`, `MidnightCircuit::new` with the known instance and witness, `MockProver::run` and `verify`. The six outcomes are `preprocess-error`, `witness-consistency-error`, `synthesis-error`, `constraint-failure`, `panic` and `accepted` (12-oracles-and-differential-testing.md). The contract reports it as the object `circuit` with `outcome`, `message`, `k`, `elapsed_ms` and `met`, and as `not run` with a `reason` when the binary of the surface's workspace under `repos/_build/` does not exist. The comparison of this outcome with the tier-two verdicts, on every corpus preimage and on the seven transaction contexts of `corpus/moriarty-contexts/`, is the `--circuit` mode of `tools/diff_test.py` against `plan-iter3/circuit-comparison-table.md`.

## The observable semantics

The observable result of a run is one term computed by one function over the final configuration:

```k
syntax Observable ::= obs(Status, List, List) [symbol(obs)] | noObs() [symbol(noObs)]
syntax Observable ::= observable(Status, List, List) [function, total]
rule observable(St, Os, Pi) => obs(St, #encodeAll(Os), Pi)
```

`[[P]](pre) = observable(status, outputs, pi)` is the status of the run, the values returned by `output` in order, each encoded as `encode_offcircuit` encodes it (`encodeValue`, the encoding the communications commitment hashes) and concatenated, and the public-input vector. The rule for `#observable` in `zkir-vm.k` writes it to the `<observable>` cell after `#witnessSpace`, and after a failed static check of `checkedJob` without a run. The runner exposes the cell as `observable` with `status`, `error`, `outputs` and `pis`. The memory, the verdicts and the witness space are the model's evidence for the triple, not part of the observation.

A compiler-correctness statement quotes the triple in two directions. Completeness: an honest run of the source semantics yields a preimage `pre` such that `[[P]](pre)` has status `ok`, the circuit accepts `pre` (tier three), and the public inputs of `[[P]](pre)` are the ones the source semantics predicts. Soundness: every memory in the modelled witness space with its public-input vector corresponds to a source execution. Soundness quantifies over the modelled space, which over-approximates what the real circuit accepts wherever a constraint is unmodelled, so the residual list of the soundness claim in 08-constraints-and-verdicts.md (auxiliary cells, copy wiring, hash gadget internals, prover-side panics, JubjubScalar canonicity, guarded-off inputs) is a stated residual of the theorem. The claims template `spec_compiled_observable` in `tools/run_claims.py` states one instance of the completeness direction as a reachability claim: `job(P, Pre)` from the initial configuration reaches `<observable> obs(status, outputs, pis)` with every other cell existential. The generated instance `claims/spec-compiled-observable.k` proves it for `add %a 1 -> %b ; impact 1 [%b]` on a symbolic input; `evidence/zkir-k-claims-2026-09-06c.txt` is the receipt.

## Running the contract

The definition is built with `make -C experiments/zkir-k zkir-contract`. The program-only form:

```
uv run --group zkir-k python experiments/zkir-k/tools/zkir_kast.py contract experiments/zkir-k/corpus/divergence/f12_ec_mul_generator_p256.zkir
```

prints `program`, `surface` and the twenty obligations as `{name, status, detail}`. Two of them from that output, the chip fact and the failing dispatch:

```json
    {
      "name": "chips.enabled",
      "status": "info",
      "detail": "p256"
    },
    ...
    {
      "name": "circuit.static.dispatch",
      "status": "failed",
      "detail": "instruction 0: Unsupported EcMulGenerator for scalar of type Secp256r1Scalar"
    },
```

With a preimage, on the swap contract's `expire` circuit and its transaction context:

```
uv run --group zkir-k python experiments/zkir-k/tools/zkir_kast.py contract experiments/moriarty-core-swap/output/zkir/expire.zkir --preimage experiments/zkir-k/corpus/moriarty-contexts/swap-expire.pre.json
```

The output keeps the program-only members and adds `tier_one`, a summary of the obligations, and the two tiers; `observable.pis` has 394 entries and is shortened here:

```json
  "tier_one": {
    "met": true,
    "failed": [],
    "counts": {"met": 3, "failed": 0, "notApplicable": 11, "info": 6}
  },
  "preimage_dependent": {
    "status": "ok",
    "witness_space": true,
    "unconstrained": ["%color.11", "%color.12", "%amount.13", "%value.14", "%value.15"],
    "violations": [],
    "observable": {"status": "ok", "outputs": [], "pis": ["0", "9620025776880078320084866401708960911381310259544672466178048295274965038202", "48", ...]},
    "met": true
  },
  "circuit": {
    "outcome": "accepted",
    "binary": "/home/charl/Moriarty/repos/_build/ledger-92e8bdd3/target/release/zkir-circuit-oracle",
    "message": "MockProver::verify ok",
    "k": 9,
    "elapsed_ms": 13,
    "met": true
  }
```

`--ext` selects the extension surface for both the preprocessor and tier two, and `--vm-definition DIR` points tier two at another compiled definition. `uv run --group zkir-k python experiments/zkir-k/tools/contract_corpus.py` runs tier one over the corpus and all three tiers over the seven Moriarty contexts against an expectation table, and `uv run --group zkir-k python experiments/zkir-k/tools/provability.py --out evidence/zkir-k-provability-<date>.txt` keys every program whose tier one is met.

## What the contract does not decide

Tier one names no witness-dependent condition: a bound on a value (`constrain_bits`, `less_than`, `reconstitute_field`), a non-boolean guard, an inverse of zero, a point not on its curve, a transcript of the wrong length, or a communications commitment that does not match. Those are tier two, on one preimage, and say nothing about other preimages. Tier one also decides nothing where the static type is unknown: an operand produced by `concat` of operands of unknown length, or by an instruction whose own dispatch is unsupported, is skipped, and the gate reports the case on a witness.

Tier two is a statement about the modelled relations, not about the circuit: the residual list of chapter 08 applies to every `witness_space` true. Tier three is a statement about one preimage under the MockProver, which checks the constraint system without producing a proof; keygen, proving and verification with real parameters are the receipts of `tools/provability.py`, not part of the contract's output. Nothing in the three tiers decides the soundness direction of a compiler-correctness statement, which quantifies over memories no test enumerates; the claims of `claims/` are where that obligation begins.
