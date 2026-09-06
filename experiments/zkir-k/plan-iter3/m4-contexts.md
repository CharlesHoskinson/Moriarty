# M4. Real transaction contexts for the Moriarty artifacts

Date: 2026-09-06. Worktree `zkir-k-iter3`. Receipt: `evidence/zkir-k-moriarty-contexts-2026-09-06.txt`.

## 1. What the assertions need

The seven artifacts are the compiled circuits of two Compact 0.26 contracts
(compactc 0.34.100, runtime 0.19.100, `--feature-zkir-v3`): the escrow
(`fund`, `release`, `refundAfterTimeout`) and the atomic swap (`fundAlice`,
`fundBob`, `decide`, `expire`). Every circuit is a sequence of ledger reads,
kernel calls and one witness call, each of which the compiler lowers to the
same three-part pattern in ZKIR:

- a ledger read (`phase`, `deadline`, an authority, a token colour, an
  amount): one or two `public_input` instructions consuming
  `public_transcript_outputs` (the `popeq` result; `Bytes<32>` values arrive
  as two field elements, the top byte constrained to 8 bits and the remaining
  248 bits), followed by `impact` instructions that emit the VM opcodes of the
  read (`dup`, `idx path`, `popeq`) and echo the read value into the
  program-verified `public_transcript_inputs`;
- a kernel call (`blockTimeLt`, `receiveUnshielded`, `sendUnshielded`): only
  `impact` instructions, except `blockTimeLt`, whose result is one more
  `public_input` fed back to an `assert`;
- a witness call (`aliceSecret`, `bobSecret`, `partySecret`): two
  `private_input` instructions (`Bytes<32>` split as above), a
  `persistent_hash`, `bytes32_into_low_high`, and equality tests against the
  authority read from the ledger, then `assert`.

`public_transcript_inputs` is therefore entirely determined by the program
and the transcript outputs (the K generation mode records it as `pubIn`
needs, so the differential generator never fails on it). What the generator
cannot guess are the *relations* between the other values:

| Assertion (Compact) | ZKIR form | What the preimage must satisfy |
|---|---|---|
| `phase == Phase.X` / `phase != Phase.Y` | `test_eq %t.0 imm; assert` on the first transcript output | the first ledger read is the phase the circuit expects (0 for `fundAlice`/`fund`, 1 for `fundBob`/`release`/`refundAfterTimeout`, 2 for `decide`; not 3 or 4 for `expire`) |
| `!blockTimeGte(deadline)` / `blockTimeGte(deadline)` | `lt` kernel op; its result is a `public_input` asserted directly (or negated) | the `lt` result must be boolean and consistent with `time < deadline` for the deadline read; the generator's random field element fails `Expected boolean` and a random bit fails the assertion half the time |
| `authority == persistentHash(secret)` | two `private_input`s, `persistent_hash`, `bytes32_into_low_high`, two `test_eq`, `cond_select`, `assert` | the ledger's authority cell must equal the Poseidon-based persistent hash of the witness's 32 bytes; unreachable by random choice |
| `receiveUnshielded` / `sendUnshielded` | kernel `impact`s only on the effects cells (indices 6, 7, 8 of the kernel state) | no transcript output; only the colour and amount reads above |
| communications commitment | `commGate` on `poseidon(rand ++ encode(inputs) ++ encode(outputs))` | recorded by the generation mode; already satisfied |

This is why the differential receipt `zkir-k-differential-92e8bdd3-2026-09-05c.txt`
shows `Failed direct assertion` at `%t.1` (the phase test) or `Expected
boolean` for six artifacts, and one lucky success for `expire`: it needs only
`phase` in {0,1,2}, a boolean `lt` result of 0, and no witness.

## 2. Path taken: the compiled contract driven through the Compact runtime

The repository has no `node_modules`, but the Nix store holds
`@midnight-ntwrk/compact-runtime` 0.19.100 (the exact version
`output/contract/index.js` checks with `checkRuntimeVersion`) at
`/nix/store/*-midnight-ntwrk-compact-runtime_0.19.100/lib/node_modules/@midnight-ntwrk/compact-runtime`,
with its `@midnightntwrk/onchain-runtime-v4` 4.0.0-rc.3 WASM dependency
bundled. midnight-ledger 92e8bdd3 (`onchain-runtime-wasm/Cargo.toml`) is
onchain-runtime-wasm 4.0.0-rc.3 as well, so the runtime that executes the
contract and the crate that runs `preprocess` are the same onchain-runtime
version.

`tools/moriarty_preimages.mjs` (Node 24, ESM; a `module.registerHooks`
resolver maps the bare specifier to the Nix path so the generated `index.js`
loads unchanged) does, per contract:

1. instantiate `Contract` with deterministic witnesses (`aliceSecret` =
   32 × 0xa1, `bobSecret` = 32 × 0xb0, `partySecret` = 32 × 0xe5) and compute
   the authorities with the runtime's `persistentHash(CompactTypeBytes(32), ·)`;
2. run `initialState` (the Compact constructor) with those authorities,
   distinct parties, tokens and amounts, deadline 1 800 000 000 s;
3. execute the circuits in scenario order on the state each leaves behind
   (`createCircuitContext(circuit, dummyContractAddress(), coinPublicKey,
   state, privateState, …, time)`, then `contract.circuits[c](ctx, …args)`),
   with block time 1 700 000 000 (before the deadline) or 1 900 000 000 (after);
4. take the root call's `CallProofData` from `callProofDataTrace`, hand it to
   the onchain runtime's `proofDataIntoSerializedPreimage(input, output,
   publicTranscript, privateTranscriptOutputs)`, and decode the resulting
   tagged-serialized `ProofPreimage` with `preimage-json`, a 40-line binary
   added to the ledger-92e8bdd3 workspace (`tools/preimage-json/`), into the
   harness JSON (decimal strings).

| artifact | scenario | time |
|---|---|---|
| swap-fundAlice | fresh contract, WaitingAlice | before |
| swap-fundBob | after fundAlice, WaitingBob | before |
| swap-decide | after fundBob, WaitingDecision, `decision = 1` | before |
| swap-expire | after fundBob, WaitingDecision (both deposits refunded) | after |
| escrow-fund | fresh contract, Waiting | before |
| escrow-release | after fund, Funded | before |
| escrow-refundAfterTimeout | after fund, Funded | after |

The generator is deterministic (two runs give byte-identical files). The
runtime path succeeded on the first attempt for all seven artifacts; the
hand-built fallback (path 3) was not needed.

## 3. Results

`tools/moriarty_contexts.py --ext` (receipt above): for every artifact, on
both surfaces (ZKIR with the ledger-92e8bdd3 binaries, ZKIR-EXT with the
midnight-zkir-2ffe2d1 binaries),

- K `checkedJob` and the crate's `preprocess` both succeed and agree on
  status, every register's type and encoding, the public-input vector and the
  skip vector; every emitted verdict holds;
- the circuit oracle reports `accepted` (MockProver verify ok): k = 13 for the
  five circuits with a witness hash (2^13 rows, 80–120 ms, 34 MB), k = 9 for
  `expire`, k = 8 for `refundAfterTimeout`;
- the honest `public_transcript_inputs` produced by the runtime's
  `Op::field_repr` are the ones the program's `impact` instructions encode:
  a mismatch would have failed with `Public transcript input mismatch`;
  the compiler's and the runtime's transcript encodings agree at these
  versions;
- the negative controls are rejected on all three sides with the same error
  class: `wrong-phase` (first ledger read + 1, transcript inputs regenerated
  by the K generation mode so only the ledger value is wrong) and
  `wrong-witness` (last private transcript element + 1) both end in
  `Failed direct assertion`; the circuit oracle classifies them as
  `preprocess-error`, since a witness the preprocessor rejects never reaches
  the constraint system.

## 4. What is fixed by construction, and what remains

- `binding_input` is 0 and the communications-commitment randomness is 0:
  `proofDataIntoSerializedPreimage` sets both to 0, and the ledger overwrites
  them at transaction assembly (the binding input is the hash of the
  transaction the call is part of). The ZKIR programs only push the binding
  input as their first public input and check the commitment against the
  given randomness, so any values are equally consistent; the harness's
  existing `wrong-comm` perturbation already covers the commitment check.
  A preimage with the ledger's real binding input needs a full unproven
  transaction (`ContractCallPrototype` → `Transaction`), which the runtime
  package alone does not build; that is transaction assembly, not circuit
  semantics, and is left out.
- The transaction-level checks are outside the circuit: the effects cells
  (`receiveUnshielded` / `sendUnshielded` claims) reach the ledger through
  the public transcript and are validated when the transaction is applied,
  not by the proof. The preimages here are consistent with the contract
  state; whether the surrounding transaction carries the unshielded coins is
  not represented.
- `decide` is exercised only with `decision = 1` (settle); `decision = 0`
  follows the other branch of the same circuit with the same transcript
  shape and is a one-line change to the generator.
- The runtime is the Nix-store package; if it is absent, set
  `COMPACT_RUNTIME` to any copy of `@midnight-ntwrk/compact-runtime`
  0.19.100. Other versions fail `checkRuntimeVersion` in the generated
  `index.js`.

## 5. Files

- `experiments/zkir-k/tools/moriarty_preimages.mjs`: the generator (Node).
- `experiments/zkir-k/tools/preimage-json/`: the decoder crate (source copy;
  built in `~/Moriarty/repos/_build/ledger-92e8bdd3`, one workspace-member
  line added there).
- `experiments/zkir-k/corpus/moriarty-contexts/*.pre.json`, `manifest.json`:
  the seven preimages and how each was produced.
- `experiments/zkir-k/tools/moriarty_contexts.py`: the runner (K checked,
  zkir-oracle, zkir-circuit-oracle, negative controls; `--ext` for the second
  surface).
- `evidence/zkir-k-moriarty-contexts-2026-09-06.txt`: the receipt.
