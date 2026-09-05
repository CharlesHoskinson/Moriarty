---
id: zkir.k-definition
type: semantics
title: The ZKIR semantics in K, as built
status: active
updated_at: 2026-09-05T21:10:00Z
sources:
  - SRC-0023
  - SRC-0025
  - SRC-0006
---

# The ZKIR semantics in K, as built

This page records what the executable K definition of ZKIR v3 under `experiments/zkir-k/` is, how it was checked, and what the checks found. The decisions it implements are on [../zkir-k-semantics-plan.md](../zkir-k-semantics-plan.md); the instruction set, types and VM it formalises are described on [zkir-instruction-set.md](zkir-instruction-set.md), [zkir-type-system.md](zkir-type-system.md) and [zkir-vm-semantics.md](zkir-vm-semantics.md). Everything here was reproduced on 2026-09-05 with K v7.1.337 and the receipts named in each claim.

## Layout

The definition is one K definition split into modules, each with the Rust file it follows in its header (CLM-0731; experiments/zkir-k/semantics; repository observation; reproduced; high; S1):

| Module | File | Follows | Content |
|---|---|---|---|
| `ZKIR-SYNTAX`, `ZKIR-WF` | `zkir-syntax.k` | `ir.rs`, `ir_types.rs` | 34 instructions, 13 types, operands, guards, alignments; `reads`, `writes`, `encodedLen`; static well-formedness |
| `ZKIR-FIELD` | `zkir-field.k` | `transient-crypto/src/curve.rs`, midnight-curves 0.3.1 | eight moduli, modular arithmetic on K `Int`, Tonelli-Shanks square roots |
| `ZKIR-CURVES` | `zkir-curves.k` | midnight-curves 0.3.1, midnight-circuits 7.2.4 `ecc/curves.rs` | affine twisted-Edwards (Jubjub, Curve25519) and Weierstrass (secp256k1, secp256r1) arithmetic, subgroup tests, Jubjub decompression |
| `ZKIR-VALUES` | `zkir-values.k` | `ir_instructions/encode.rs`, midnight-circuits `Instantiable` | the 13 value constructors, `encode_offcircuit`, `from_public_input`, input decoding |
| `ZKIR-CONSTANTS` | `zkir-constants.k` (generated) | midnight-circuits `hash/poseidon/constants/blstrs.rs`, `ecc/hash_to_curve/mtc_params.rs` | 204 Poseidon round constants, the MDS matrix, the Shallue-van de Woestijne parameters |
| `ZKIR-HASH` | `zkir-hash.k` | midnight-circuits `poseidon_cpu.rs`, `htc_gadget.rs`, `mtc_cpu.rs`; FIPS 180-4; Keccak | Poseidon sponge (fixed and variable length), hash-to-curve, SHA-256, Keccak-256 |
| `ZKIR-OPS` | `zkir-ops.k` | `ir_instructions/*.rs` off-circuit halves, `transient-crypto/src/{fab,repr}.rs` | one function per `*_offcircuit`, boolean and bit-bound resolution, aligned byte layout for the standard hashes |
| `ZKIR-VM` | `zkir-vm.k` | `ir_vm.rs` `preprocess` | the configuration, sequencing, one witness rule per instruction, constraint emission, transcript and commitment checks, generation mode |
| `ZKIR-CONSTRAINTS` | `zkir-constraints.k` | `ir_vm.rs` `circuit`, `ir_instructions/*.rs` in-circuit halves, `used_chips` | the in-circuit relation of each gate, evaluated on the final witness |
| `ZKIR-EXT` | `zkir-ext.k`, `zkir-sha512-constants.k` | `midnight-zkir` 2ffe2d1 `zkir/src/` | Bool, Byte, Bytes<n>; and, or, xor, concat, slice, nth, reverse, load_constant, sha512 |

Programs enter through `tools/zkir_kast.py`, which reads the JSON artifact and builds the `Program` term with pyk, mirroring the serde layer of `ir.rs` (variables start with `%`, immediates are little-endian hex below r, `guard: null`, tagged alignments); the same script rejects what `IrSource::load` rejects (CLM-0732; experiments/zkir-k/tools/zkir_kast.py; repository observation; reproduced; high; S1). A run is `job(Program, Preimage)`, where the preimage mirrors `ProofPreimage`: raw inputs, binding input, optional commitment and opening, private transcript, public transcript inputs and outputs (CLM-0732).

## What a run produces

The configuration carries the seven components of the VM page as cells (`<mem>`, `<pi>`, `<skips>`, `<pubInIdx>`, `<pubOutIdx>`, `<privIdx>`, `<outputs>`) plus `<constraints>`, `<status>`, `<chips>` and `<verdicts>` (CLM-0733; experiments/zkir-k/semantics/zkir-vm.k; repository observation; reproduced; high; S1). Each instruction first appends its gate to `<constraints>` and then runs its witness half; the first witness error sets `<status>` and freezes the witness cells while every later instruction still emits its gate, so the constraint set at the end is always the whole circuit's (CLM-0733). At the end, the transcripts must be fully consumed and, when the program declares a communications commitment, the commitment must equal the Poseidon commitment of the raw inputs and the encoded outputs, exactly as in `preprocess` (CLM-0733). Finally every gate is evaluated against the witness and `<verdicts>` records, per gate, whether it holds, is violated, cannot be synthesised (a missing in-circuit dispatch arm or an uninitialised chip), or is unknown because the witness stopped before the register existed (CLM-0733).

Gates are instruction-level relations, not PLONKish gate rows. `gate(add(a, b, o))` states that register `o` equals the sum of `a` and `b` under the type dispatch of `add_incircuit`; `gate(assert(c))` states `c` is non-zero; `piGate(i, g, x)` states that public input `i` equals `x` when the boolean `g` is 1 and 0 otherwise (CLM-0734; experiments/zkir-k/semantics/zkir-constraints.k; repository observation; reproduced; high; S1). Because the relation is evaluated on the concrete witness, a run establishes that the honest witness satisfies the emitted constraints, never that no other witness does; soundness stays with the arc-zkir theorems, as the plan says (CLM-0734).

## How it was checked

Three layers, each with its receipt:

1. Unit checks against an independent Python implementation of the same formulas (`tools/unit_values.py`): field inverses and square roots, the non-residuosity of both Edwards `d` constants, the Jubjub subgroup generator as eight times the curve generator, scalar multiples on all four curves, the order of each generator, encode and decode round trips for every type, and the parity-only behaviour of Jubjub decompression. 41 of 41 pass (CLM-0726).
2. Known-answer checks against the Rust crate for the hashes (`tools/unit_hash.py`): Poseidon on zero to three inputs, hash-to-curve on zero to three inputs, SHA-256 and Keccak-256 through one-, 32- and 232-byte aligned preimages, and the aligned byte layout itself. 18 of 18 pass; Poseidon and hash-to-curve matched on the first run, which also confirms that the crate's paired partial rounds are an algebraic rewrite of the plain round function (CLM-0726).
3. Differential runs against the crate's own `preprocess` (`tools/diff_test.py`). The harness draws typed raw inputs, runs the K definition in generation mode until the transcript needs are stable, then runs K and `zkir-oracle` on the same preimage and compares status, every register's variant and encoding, the public-input vector and the skip vector. Over the 65 programs of the corpus, 314 comparisons agree and none disagree; 37 are successful witness runs, the rest compare error paths (CLM-0728). All gates hold on every successful run (CLM-0729).

The corpus is the 43 inline programs of the crate's tests at 92e8bdd3 (seeded with their literal inputs where the tests give them), the six version-3 micro-dao precompiles of midnight-zkir 2ffe2d1, the seven Moriarty escrow and swap artifacts, nine handmade programs that exercise every instruction on every type it supports, and seven handmade negatives for the static checks (CLM-0724, CLM-0728). The escrow, swap and micro-dao programs never reach a successful witness with generated inputs, because their assertions need a consistent transaction context; for them the harness compares error paths only (CLM-0728).

## What the checks found

- The thirteen findings of the arc-zkir divergence review reproduce as programs (`tools/divergence_tests.py`): for each, the off-circuit status agrees between K and the crate, and the gate outcome is the one the review describes, for example `assert` of 2 fails off-circuit while its gate holds, `cond_select` on Bytes32 and `constrain_eq` on JubjubScalar succeed off-circuit while their gates are synthesis errors, `bytes32_from_low_high` with a foreign `high` succeeds off-circuit while its gate is a synthesis error, `less_than` with a 3-bit bound rejects 8 off-circuit while the padded 4-bit gate would accept it, a program touching secp256k1 only through `from_bytes32` succeeds off-circuit while its gate reports the chip uninitialised, and the order-2 Curve25519 point is rejected on both sides (CLM-0730).
- One divergence not in the review: Jubjub `from_coordinates` off-circuit goes through point decompression and uses only the parity of `x`, so `(x+2, y)` yields the true point on both K and the crate while the exact-coordinate gate is violated; the Agda trust base assumes the exact behaviour (CLM-0735; evidence/zkir-k-divergence-tests-2026-09-05.txt case `k01` and repos/input-output-hk/arc-zkir/src/zkir-v3/Assumptions.agda `fromCoordsJ-coordsJ`; executed test; reproduced; high; S1). Recorded in [../contradictions.md](../contradictions.md).
- The Rust `preprocess` does not enforce single assignment; the K static check does, so a program that rewrites a register is rejected by K before the run and accepted by the crate (CLM-0725). No program in the corpus does this.
- Error messages agree in substance; the only textual difference is the crate printing field elements as little-endian hex where K prints decimals (CLM-0728).

## The extension surface

`ZKIR-EXT` is a second main module on the same definition for the standalone `midnight-zkir` crate at 2ffe2d1: the types Bool, Byte and Bytes<n> (Bytes<32> keeps the base representation so every base rule applies), the instructions `and`, `or`, `xor` over Bool, `concat`, `slice`, `nth`, `reverse` over byte strings, `load_constant` from a typed raw encoding, `sha512`, logical `neg` on Bool, and the canonical-form check that 2ffe2d1's decoder performs by re-encoding every decoded value; `reverse_bytes` is rejected at the preprocessor as the crate removed it (CLM-0739; experiments/zkir-k/semantics/zkir-ext.k; repository observation; reproduced; high; S1). Against a second `zkir-oracle` built from that crate, 61 of its inline test programs, the six micro-dao precompiles and the handmade programs give 366 comparisons that all agree, with 40 successful runs and no non-holding gate (CLM-0737). The canonical-form check is the visible behavioural difference between the two crates: a Jubjub point input with a wrong `x` of the right parity is accepted at 92e8bdd3 and rejected at 2ffe2d1 (CLM-0737).

A second new finding came out of this run (K2): both crates slice the transcript directly, so an unguarded `public_input` or `private_input` whose transcript is too short panics with an index-out-of-range rather than returning an error; K reports an error, and the divergence test `k02` reproduces the panic through the oracle (CLM-0738).

## The other oracles

The arc-zkir v3 Agda development type-checks in full with its own Nix toolchain, fifteen modules under `--safe` (CLM-0736; evidence/arc-zkir-agda-typecheck-2026-09-05.txt; executed test; reproduced; high; S1). It cannot execute programs: it is parametric over an `Assumptions` record that supplies the field, curve and hash primitives, and the repository has no concrete instance of that record, so oracle 3 of the plan remains a correspondence by reading, not by running (CLM-0736; repos/input-output-hk/arc-zkir/src/zkir-v3/Assumptions.agda; repository observation; reproduced; high; S4).

Midnight's `k-rust` (0.4.0, built from the pinned checkout) runs the K tutorial lesson in three seconds and parses terms of `ZKIR-SYNTAX` in under four, but neither `krust krun` on a one-instruction program nor `krust kcompile` of the full definition finished: the compile was killed after 25 minutes of CPU with no output, where canonical K compiles the same definition in twenty seconds (CLM-0740; evidence/k-rust-compatibility-2026-09-05.txt; executed test; reproduced; high; S1). The definition uses nothing k-rust documents as unsupported, so the gap is performance of its frontend pipeline on a definition of this size, not a feature boundary; running the semantics inside the Midnight TypeScript tooling stays a goal, as the plan said, not a gate (CLM-0740).
