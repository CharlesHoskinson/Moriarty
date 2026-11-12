# Modularity requirement for the Sherlock design (owner, 2026-11-11)

Owner: "Can we make this design modular so its much easier for AI to implement?"

`ARCHITECTURE.md` is organised as modules (kits), not as one circuit description.

1. Modules. Machine level: CORE (register file, rule micro-operations), DEC, ALU and RANGE, MEM (in-leaf window and tail), MEMORY BACKEND, HASH, VALUE DIGEST (`sherlock-value/1`), BOUNDARY COMMITMENT, CONTINUATION, NODE INTEGRATION. Co-processor level: the bridge layer `L` and the wrap (kit L AND WRAP), LEDGER CONTRACT, ZERO KNOWLEDGE, COMPACT FRONTEND. Merge or split only with a stated reason.
2. Frozen interfaces. Modules communicate only through typed bus tuples `(chip, key, a, b, c)`, named copy-constrained cells, or the frozen co-processor interfaces: the deferred field log and its `pv` encoding, the call vector of `2 + L_TX` words, the anchor statement, and `prog_d` with `key_root`. For each module give the tuples or words it requests and provides, their field meanings and ranges, and the invariants a provider guarantees. No module reads another module's internal columns.
3. Swappable backends. MEMORY BACKEND (Merkle dense tree against run-wide write-once LogUp) and BOUNDARY COMMITMENT (Poseidon digest against Horner fingerprint) sit behind one interface each, so both can be built and measured without changes to CORE or MEM.
4. Module kit. For each module: interface contract; soundness invariants; cost budget (ESTIMATE until measured); a small Quint model or the existing model it refines; positive tests and named negative mutants; a code skeleton with per-function contracts (`research/skeleton-coding-20261108/`); and the files it owns. A kit is implementable by one agent in one session without reading other modules' internals.
5. Build order. A dependency graph and the order in which modules are built and verified, marking which can be built in parallel (`ARCHITECTURE.md` section 3).
6. Integration harness. One whole-run differential test against the reference machine that runs on every module change; for Compact, the frontend golden vectors (effect vector equal to the native public-input vector word for word).

Hard rules, kept by every module: never k = 22; every proof is a fixed-size shard; per-program, witness-independent preprocessing bound to `prog_d`; zero knowledge end to end, with every STARK layer on the user's machine and no helper that sees the plain witness; proven 100-bit security with no unproved conjecture, claimed only after the security qualification gate G1 to G5 passes (status: not passed); GP-1 to GP-4 never narrowed; one universal key set; the ledger 9.1.0.1 verifier unchanged and a verifier key of at most 50,000 bytes.
