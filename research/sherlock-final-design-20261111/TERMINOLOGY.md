# Sherlock terminology

Owner, 2026-11-11: "What should we call Sherlock? ... We need some terminology", then "don't lose the terminology".

The design these terms describe is `research/sherlock-processor-final-20261111/PROPOSAL.md` as applied in `ARCHITECTURE.md`. Section numbers below are sections of `PROPOSAL.md`.

## Category

Sherlock is a **universal, elastic proving co-processor for Midnight**.

- **Co-processor.** Midnight is the main processor. Sherlock runs the computation off-chain, on the user's machine, and Midnight accepts the result by checking one proof.
- **Universal.** One universal key set and one wrap key accept every program (GP-1 to GP-4). Native Compact needs one verifier key per entry point.
- **Elastic.** The same design runs on the user's own devices and GPUs, from a laptop to a GPU host the user owns. Work grows by adding fixed-size shards; the statement and the universal keys do not change with the hardware. Only devices and hosts the user owns take part (OD-9).
- **Proving.** The co-processor produces a claim that the chain checks.
- **Zero knowledge.** A hard requirement (owner, 2026-11-11: "we need to make this zero knowledge or else its useless"). Every STARK layer runs on the user's machine and the published proof is the blinded wrap. Zero knowledge is a property of the co-processor, not a word of its name.

Words kept out of the name: "recursion" (the internal mechanism of leaves, nodes and `L`) and "modular" (how the design is built, in kits). Both stay correct inside the architecture documents.

One line: Sherlock is a universal, elastic proving co-processor for Midnight. It runs any program off-chain, on the user's own hardware, and Midnight accepts the result with one universal key set through the ledger 9.1.0.1 verifier; whether that verifier changes depends on owner decision OD-24.

## Hard rules

Never k = 22; every proof is a fixed-size shard; per-program, witness-independent preprocessing bound to `prog_d`; zero knowledge end to end, with every STARK layer on the user's machine and no helper that sees the plain witness; proven 100-bit security with no unproved conjecture, claimed only after the security qualification gate G1 to G5 passes (status: not passed); GP-1 to GP-4 never narrowed; one universal key set; the ledger 9.1.0.1 verifier unchanged and a verifier key of at most 50,000 bytes.

## Terms

| Term | Meaning |
|---|---|
| Sherlock | The whole system, the co-processor |
| Sherlock machine | The abstract machine and its language: the interpreted rules for general programs, the straight-line family `R_SL_*` for Compact Stage 1, `prog_d`, `input_d`. `machine_version` inside `prog_d` names the rule family a program was planned under (`ARCHITECTURE.md` section 1.7) |
| Sherlock chip | The AIRs that prove the machine: the per-program leaf chips, the universal node, `L`, and the wrap circuit |
| Leaf, node, `L`, wrap | The proof stages: leaf shards prove the program's steps; nodes of fixed arity verify their children; `L` verifies the node root; the wrap verifies `L` (section 3.1) |
| Shard | One fixed-size proof of the tree. Work grows by adding shards, never by growing a circuit |
| Shard plan | The witness-free output of `sherlock plan`, once per program: instruction table, chip sets and leaf shapes, cut points, log layout, discharge placement and per-program keys, bound to `prog_d` through `plan_d` and `key_root` (section 4) |
| Path C | The shortest path through the tree for a one-leaf call: one leaf, one leaf layer at internal parameters, no internal node, then `L` and the wrap (section 3.1) |
| Universal key set | The internal-recursive key, the `L` key and the wrap key, the same for every program. Per-program keys (app, leaf-layer, internal-for-leaf) sit under `key_root` inside `prog_d` |
| Sherlock prover | The user's own proof server, off-chain, a resident daemon on a local socket with one-use blinding slots (OD-13) |
| Sherlock contract | A Compact contract whose exported entries hold the universal wrap key, with the anchor cell; for general runs, the `acceptRun` entry |
| Run | One execution of a program on an input |
| Claim | What the chain accepts: the public statement of a run or a call |
| Off-chain, On-chain | Where a part lives: the user's machine, or Midnight |

## The bridge to Midnight

Owner, 2026-11-11: "name the mathematical structure we are developing or modifying", then "yes". The textbook category is proof composition with field switching (STARK-to-SNARK wrapping).

| Term | Meaning |
|---|---|
| Sherlock bridge | The whole structure: small-field STARKs over BabyBear and its degree-4 extension, with WHIR as the proximity test, composed with one PLONK-KZG proof over the BLS12-381 scalar field Fr, joined at one layer |
| Bridge layer, `L` | The last STARK layer. Its trace stays in BabyBear; its Merkle tree and Fiat-Shamir transcript use Midnight Poseidon over Fr, fed by 8 BabyBear elements per Fr element. One shape, height at most 2^20, parameters of section 3.4 (OD-3) |
| Wrap | A blinded midnight-proofs PLONK circuit with KZG over BLS12-381 and the Blake2b transcript. It verifies `L` as a private witness, replays the deferred field log natively, and publishes only the call vector. The only proof that leaves the user's machine |
| G4 wrap | The wrap shape: 4 identical lanes of 8 advice columns, every gate and lookup of degree at most 5, a 19-row Poseidon layout, one fixed 2^16-row range table, k the smallest of 19, 20 and 21 whose rows fit the 85 percent gate (section 8). G4 names this shape only; the fourth check of the security gate is written "gate check G4" |
| Deferred field log | The BLS12-381 Fr work a program asks for, recorded as entries `(op, inputs, outputs)` under a tagged onion digest. `L` binds it into its public values and the wrap replays it natively. A discretionary operation that does not fit goes to tier E (section 3.2) |
| Tier E | Inline emulation of Fr work in leaf chips (FIELD-EMU, POSEIDON-FR-EMU, JUBJUB-EMU). It proves the same `Valid(e)` as a log entry |
| `W_pv` | The fixed length of the public-value vector `pv` that `L` exposes for the log: `2B + 9V` words, an entry table of `B` rows of 2 words and a value table of `V` rows of 9 limbs; 2,816 at `B = V = 256` (OD-17) |
| Reservation `R` | The mandatory log entries of a run: the anchor `DIGEST` entries `R_A` plus the public-digest entries of its statement. The planner places `R` before any discretionary operation, so a full log never sends a mandatory entry to tier E (OD-17) |
| Placement V, V+ | E-1 entry-label binding: the routine `bind_label` computes `binding_input = le31(SHA256(m))` on the leaf's `sha2` chip, and the wrap requires call-vector lane 0 to equal it (V). V+ reads the domain bytes and label offset from the tape and pins their digest `layout_d` to a wrap key constant, so a ledger layout change replaces one key constant and no `prog_d` (OD-5) |
| `L_TX` | The fixed number of call-vector lanes after `binding_input` and `comm`: the anchor prefix, the effect words and constrained zero padding. The call vector has `2 + L_TX` words for every program, Compact and general runs alike: 1,026 at `L_TX` = 1,024 (OD-7, OD-15) |
| `C_P` | The anchor value of a deployed program, `C_P = H(SHK_PROGC, r_P, prog_d)` with one salt `r_P` per deployment. The anchor cell holds `C_P`, never plain `prog_d` |
| Anchor statement | The words `(mode, j, k, a_in, a_out)` that every call carries among the statement words of `L`: `mode` is ORD or CHUNK, `a_in` is the value the `popeq` lane reads, `a_out` the value the call leaves. The one wrap checks it for ordinary calls and chunks alike (OD-16) |
| STARK engine | The prover of leaves, nodes and `L`: OpenVM stark-backend v2.1.0 with its CUDA prover (owner, 2026-11-11: "openvm seems to be the winner"); Plonky3 with Plonky3-recursion is the second choice |
| Security qualification gate, G1 to G5 | The five checks that must all pass before the design claims proven 100-bit security and before any universal key freezes: G1 parameters frozen, G2 calculator run D3 on the frozen sets, G3 two-OOD argument and transcript vectors, G4 Lean restatement, G5 theorem-to-configuration mapping. Status: not passed; D3 passes the calculator part of G2 (`ARCHITECTURE.md` section 1.9) |
