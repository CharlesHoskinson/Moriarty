# Advisory consult on the ZKIR-in-K plan, 2026-09-03

Lane: judgment. The configured advisor (Claude Fable 5.1, `foreman-advisor`) failed three times with API 529 overload on 2026-09-03, so the consult was routed to the Antigravity CLI `agy` 1.1.24 with model `gemini-3.8-flash-high` in plan mode (read-only), reading `wiki/zkir-k-semantics-plan.md`, `wiki/zkir/zkir-vm-semantics.md` and `wiki/zkir/zkir-v3-divergence-review.md`. This is a vendor substitution and is recorded as such. Duration 60 s. Verbatim response follows.

---

On the five architectural decisions in zkir-k-semantics-plan.md:

Disagree with the single coupled configuration; unifying witness reduction and constraint emission in one rewrite prevents testing prover slack where inactive guards take arbitrary values (Finding 3), and obscures divergences where preprocessing halts while constraints pass (Finding 2).

Agree with targeting the 34-instruction `92e8bdd3` surface with an `EXT` module; it anchors the formal semantics to the only machine-checked Agda specification while isolating unverified upstream drift.

Agree with K `Int` and `modInt`; arbitrary-precision integers with Euclidean modulo correctly model prime field arithmetic and Fermat inversion without power-of-two wraparound bugs.

Disagree with JSON builtin parsing inside K; desugaring large JSON trees via K rewrite rules is notoriously slow, fragile, and bloated, whereas AST parsing belongs in an external harness.

Agree with `k-rust` compatibility as a goal rather than a gate; gating on an evolving, unofficial toolchain introduces artificial blockers absent from canonical K.

The two biggest risks the plan misses are: first, SMT intractability during symbolic execution, as Z3 cannot decide non-linear modular field arithmetic or cryptographic hashes without custom abstraction lemmas; second, false confidence regarding constraint soundness, because checking constraints solely against honest preprocess witnesses tests only completeness (W satisfies C) and cannot detect underconstrained circuits where malicious witnesses also satisfy the constraints.

Before milestone 2, eliminate the in-K JSON parser in favor of a lightweight external script or pyk preprocessor that converts JSON circuits directly into KAST terms, keeping `ZKIR-SYNTAX` strictly focused on AST sorts and well-formedness checks.
