# R3: the target contract and the static typing

Focus: experiments/zkir-k/semantics/zkir-static.k (forward static type environment `bindOuts`, `stype`, `chipNeeds`, chip and width helpers), zkir-contract.k (`targetContract`, every obligation including `circuit.static.*`), zkir-contract-main.k, tools/zkir_kast.py (`contract` command), tools/contract_corpus.py, chapter experiments/zkir-k/docs/16-compilation-target-contract.md, and the receipts zkir-k-contract-corpus-2026-09-06c.txt and zkir-k-provability-2026-09-06b.txt.

Questions:
1. Is the static type environment sound (never assigns a wrong static type) and where it is incomplete (unknown), does every obligation stay conservative in the direction the chapter claims (a program is never failed on a guess)? Find a program that the contract passes but keygen rejects, or that the contract fails but keygen accepts; the provability receipt says zero contradictions, try to break it.
2. Is every obligation's source of truth in the crate cited correctly (used_chips, the chip limits, the alignment restrictions, the in-circuit static arms), for both surfaces?
3. Is the obligation set complete for tier one: what else does keygen decide without a witness that the contract does not state?
4. Tiers two and three: are the preimage-dependent and circuit fields of the contract output exactly what they claim, and is the observable semantics `obs(Status, Outputs, Pi)` the right interface for a compiler-correctness statement (what is missing from it: skips? the commitment? the binding input?).
5. The compiler-correctness statement in chapter 16 and PLAN.md: are the two directions stated at exactly the strength the definition supports?
