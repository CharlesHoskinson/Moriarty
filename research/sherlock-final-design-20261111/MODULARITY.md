# Modularity requirement for the final Sherlock leaf design (owner, 2026-11-11)

Owner: "Can we make this design modular so its much easier for AI to implement?"

The merged `FINAL-DESIGN.md` (and each study's CONSENSUS.md where it proposes structure) must be organised as modules, not as one circuit description.

1. Modules. At least: CORE (register file, rule micro-operations), DEC, ALU + RANGE, MEM (in-leaf window and tail), MEMORY BACKEND, HASH, VALUE DIGEST (`sherlock-value/1`), BOUNDARY COMMITMENT (`pre_d` / `post_d` or its replacement), CONTINUATION (segment tree and checkpoint), NODE INTEGRATION. Merge or split only with a stated reason.
2. Frozen interfaces. Modules communicate only through typed bus tuples `(chip, key, a, b, c)` or named copy-constrained cells. For each module give: the tuples it requests and provides, their field meanings and ranges, and the invariants a provider guarantees. No module may read another module's internal columns.
3. Swappable backends. MEMORY BACKEND (Merkle dense tree vs run-wide write-once LogUp) and BOUNDARY COMMITMENT (Poseidon digest vs Horner fingerprint) must sit behind one interface each, so both can be built and measured without changes to CORE or MEM.
4. Module kit. For each module: interface contract; soundness invariants; row budget (ESTIMATE); a small Quint model or the existing model it refines; MockProver positive and negative tests (named mutants); a code skeleton with per-function contracts (`research/skeleton-coding-20261108/`); and the files in `crates/mori-chip/src/` it owns. A kit must be implementable by one agent in one session without reading other modules' internals.
5. Build order. A dependency graph and the order in which modules are built and verified, marking which can be built in parallel.
6. Integration harness. One whole-leaf differential test against the reference machine (`sherlock-k-a/1`) that runs on every module change.
