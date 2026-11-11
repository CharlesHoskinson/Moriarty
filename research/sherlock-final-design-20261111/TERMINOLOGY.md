# Sherlock terminology

Owner, 2026-11-11: "What should we call Sherlock? ... We need some terminology", then "don't lose the terminology".

## Category

Sherlock is a **universal, elastic proving co-processor for Midnight**.

- **Co-processor.** Midnight is the main processor. Sherlock runs the computation off-chain, on the user's hardware, and Midnight accepts the result by checking one proof.
- **Universal.** One key set and one contract accept every program (GP-1 to GP-4). Native Compact needs one verifier key per entry point.
- **Elastic.** The same chip scales from a laptop to a GPU host to a cluster. The statement and the keys do not change with the hardware.
- **Proving.** The co-processor produces a claim that the chain checks.
- **Zero knowledge.** A hard requirement (owner, 2026-11-11: "we need to make this zero knowledge or else its useless"). The word joins the category name when the zero-knowledge design (recalibration round, section 4a) is in the architecture: "a universal, elastic, zero-knowledge proving co-processor for Midnight".

Words kept out of the name: "recursion" (the internal mechanism of leaf, node and decider) and "modular" (how the chip is built, in kits). Both stay correct inside the architecture documents.

One line: Sherlock is a universal, elastic proving co-processor for Midnight. It runs any program off-chain, on the hardware the user has, and Midnight accepts the result with one key set and one contract.

## Terms

| Term | Meaning |
|---|---|
| Sherlock | The whole system, the co-processor |
| Sherlock machine | The abstract machine and its language: the CEK machine, the 120 rules, `prog_d`, `input_d` |
| Sherlock chip | The circuit that proves the machine: leaf chips, the node, the decider, the one key set |
| Leaf, node, decider | The three proof stages |
| Sherlock prover | Our own proof server, off-chain, API-compatible with the stock Midnight proof server |
| Sherlock contract | `SherlockRun` on Midnight; `acceptRun` is its entry point |
| Run | One execution of a program on an input |
| Claim | What the chain accepts: the public statement, recorded under `claim_d` |
| Off-chain, On-chain | Where a part lives: the user's machine (and helper hosts), or Midnight |
