# Lead note for Round 2 (2026-11-11)

The owner asked for one converged final design. The two studies meet at memory, so Round 2 of this study must address the run-length study directly.

Inputs (read before writing round2.md):
- `research/sherlock-runlength-consensus-20261111/lanes/*/round2.md` and, once it exists, `research/sherlock-runlength-consensus-20261111/CONSENSUS.md`. Four of five lanes there converged on dense tail-buffered memory (`len` advances only by real appends, an 8-cell tail carried in registers, a depth-61 Merkle tree for 2^64 cells, FRAMES and TASKS as Poseidon hash-stacks), with about 4,480 rows per old-memory read (ESTIMATE).
- This study's `lanes/B/round1.md` proposes write-once LogUp across the run (`WO-LogUp` / `ZM-LogUp`), which would replace that Merkle tree.

Each lane's round2.md must also answer:
1. Merkle (run-length consensus) vs run-wide write-once LogUp: which one should the final design use for memory, PROG and type tables, and why? If LogUp: how do 2^64 cells, SRC-023 partition independence, C6 outcome-only publication and GP-2 to GP-4 (runs consuming other runs' claims, foreign proofs, any shape) still hold?
2. The two-pass cost: all leaf commitments must exist before the challenges. What does that do to proving latency, parallel proving, pause and resume (the run-length study's checkpoint tier), and to a run that is proved while it executes? Is there a sound one-pass alternative on this stack (for example the EC multiset hash, b2)?
3. `VerifierGadget` single phase (`verifier_gadget.rs:308-310`): confirm the exact constraint and that the proposed realization does not need a fork of midnight-circuits.
4. Raising S from 16 to 64 or 128: the run-length lanes rejected large S for now (SRC-014). Reconcile.

CONSENSUS.md of this study must state one recommended memory design that is compatible with, or explicitly replaces, the run-length consensus, with the reason.

## Added 2026-11-11: modularity (owner)

Read `research/sherlock-final-design-20261111/MODULARITY.md`. Round 2 positions and CONSENSUS.md must describe the recommended design as modules with frozen bus interfaces, and must put the memory backend (Merkle vs write-once LogUp) and the boundary commitment (Poseidon vs Horner) behind swappable interfaces.
