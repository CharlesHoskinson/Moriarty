You are an independent soundness reviewer of one design point of a proposed zero-knowledge circuit. Work in the current directory (a git checkout). Run no cargo, no proofs, no local models. Do not commit.

Context: research/sherlock-processor-analogy-20261110/study.md (the "Sherlock leaf as a chip" design: a CPU CORE of register words that issues micro-ops; chips DEC, PAGE, MEM, ALU, HASH; one bus of tuples (chip, key, a, b, c) tied by a halo2 `lookup_any`, which is SET INCLUSION with no multiplicities; write-once memory as an append log; S steps per leaf; consecutive leaves chained by `post_d == pre_d`). The proposed design text is openspec/changes/sherlock-vm-consensus-20261107/design.md section 2.7; its constraints D1 to D8 are in 2.7.8. Your own earlier model of this design is in qa/gemini-quint/chipleaf/ (chipleaf.qnt, REPORT.md; see findings F12 and F13). Do NOT read models/sherlock/quint/chipLeaf*.qnt: this review must be independent of that model.

Design point D7 (frame protocol) is open. Model the CACHED frame protocol, exactly as follows:
- CORE registers include `top` (the value of the top frame, held in registers), `fp` (the address of the memory entry that holds the frame BELOW the top), `depth` (number of frames) and `len` (memory log length). At depth 0, `top = 0` and `fp = NIL`.
- PUSH v: append the entry `(top, fp)` at address `len` (a MEM bus request `(CELL, len, top, fp)`), then `fp := len`, `len := len + 1`, `top := v`, `depth := depth + 1`. At depth 0 this spills the sentinel `(0, NIL)`. The stored link is the core's `fp` register, never a bus field.
- POP (depth >= 1): the result is the `top` register. Load the entry at `fp` (bus request `(CELL, fp, a, b)` with `fp < len`, served by MEM for addresses appended in this leaf and by PAGE, against the pre-leaf memory root, for older ones). Then `top := a`, `fp := b`, `depth := depth - 1`. At depth 1 it loads the sentinel. POP at depth 0 is an error.
- REWRITE v (depth >= 1; a rule that changes the top frame): `top := v` only. No bus request, no append, `fp` and `len` unchanged.
- The leaf statement `pre_d`/`post_d` includes `top`, `fp`, `depth`, `len` and the memory root. The next leaf starts from `post_d` of the previous leaf, so the cached top crosses the leaf boundary in the statement registers.

Use the quint-modeling and quint-lang skills. Write qa/gemini-quint/chipleaf-d7/cached.qnt and tests in qa/gemini-quint/chipleaf-d7/cached_test.qnt. Avoid built-in names such as `fields`, `next`, `variant`, `to`. Model: the CORE step relation for a small ISA (at least LIT, PUSH, POP, REWRITE, HALT), the bus with inclusion, MEM and PAGE with D1 to D8 applied, S slots per leaf, and AT LEAST TWO CHAINED LEAVES (leaf 2's pre registers and memory equal leaf 1's post registers and committed memory). Write a reference machine with an abstract stack (a list of values).

Main invariant (frame-stack refinement): over the chained leaves, the sequence of POP results of the circuit equals the sequence of POP results of the reference stack, and the final depth and top agree. Give witnesses (programs) that push in leaf 1 and pop in leaf 2, with at least three frames and distinct values.

Mutants; each must be caught by the invariant with a concrete witness:
1. `rewrite_appends` (your F12): REWRITE also appends `(v, fp)` at `len` and sets `fp := len`.
2. `link_from_bus` (your F13): PUSH stores the link from a free bus field instead of the `fp` register.
3. `pop_no_reload`: POP does not set `top` from the loaded entry (keeps the old `top`).
4. `top_dropped`: the leaf statement does not carry `top`; leaf 2's `pre.top` is chosen freely by the prover.
Also say whether the cached protocol needs any constraint beyond these (for example on the sentinel, on `depth`, or on POP reading an entry that PAGE serves), and give a witness for each.

Then compare with the UNCACHED protocol (no `top` register; `fp` points at the top entry itself; PUSH appends `(v, fp)` and sets `fp := len`; POP loads `mem[fp]`, returns its value and sets `fp` to its link; a rewrite loads `mem[fp]` for the link, appends `(v, link)` and sets `fp := len`). Count, per PUSH, POP and REWRITE, the MEM appends and the MEM or PAGE reads of each protocol, and the registers each adds to the leaf statement. Say which you recommend for D7 and why.

Write qa/gemini-quint/chipleaf-d7/REPORT.md last, with: a SOUND verdict for the cached protocol with D1 to D8; every finding numbered with its witness (the Quint definition name) and the constraint that closes it; the cost comparison table; your recommendation; and the exact `quint` commands the lead should run. The lead runs quint typecheck, test and run on your model afterwards.
