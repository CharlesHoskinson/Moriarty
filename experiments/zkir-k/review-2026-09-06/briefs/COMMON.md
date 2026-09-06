# Common brief for all reviewers of the ZKIR-in-K semantics

You are a formal-methods expert reviewing an executable K Framework (v7.1.337)
semantics of ZKIR v3, Midnight's zero-knowledge intermediate representation.
Priority: find real defects and unsound modelling choices; second, missing
coverage; third, K engineering quality. Authorized scope: read anything under
the repository; do not modify any file except your own report. Delegation:
none, do the review yourself. Verification: proportional; you may run
`kompile`, `krun` and the Python tools listed below to check a suspicion, but
cite exact file:line for every finding.

Repository root: /home/charl/Moriarty/.worktrees/zkir-k-iter3
Definition:      experiments/zkir-k/semantics/*.k  (main modules ZKIR, ZKIR-EXT, ZKIR-CONTRACT-MAIN in zkir-contract-main.k, ZKIR-SYMBOLIC in zkir-symbolic.k for the Haskell backend; ZKIR-STATIC and ZKIR-CONTRACT are new this iteration)
This iteration:  experiments/zkir-k/plan-iter3/PLAN.md (goal and milestones), circuit-comparison-table.md, m4-contexts.md; the circuit oracle under experiments/zkir-k/tools/circuit-oracle (built in the two _build workspaces, binaries target/release/zkir-circuit-oracle); claims under experiments/zkir-k/claims; docs chapters experiments/zkir-k/docs/08 and 16
Tools:           experiments/zkir-k/tools/*.py (run with `uv run --group zkir-k python <tool>` from the repo root)
Corpus:          experiments/zkir-k/corpus/
Receipts:        evidence/zkir-k-*-2026-09-06c.txt (and the 2026-09-06 spike, provability, drift and claims receipts)
Design record:   wiki/zkir-k-semantics-plan.md, wiki/zkir/zkir-k-definition.md
Ground truth:    the Rust crate at midnight-ledger commit 92e8bdd3, extracted under
                 /home/charl/Moriarty/repos/_build/ledger-92e8bdd3/zkir-v3/src/ (and midnight-zkir 2ffe2d1 under /home/charl/Moriarty/repos/_build/midnight-zkir-2ffe2d1/zkir/src/)
                 (ir.rs, ir_types.rs, ir_vm.rs, ir_instructions/*.rs); midnight-circuits 7.2.4 and
                 midnight-curves 0.3.1 sources under ~/.cargo/registry/src/index.crates.io-*/
Spec:            /home/charl/Moriarty/repos/input-output-hk/arc-zkir/docs/zkir-v3-spec.md and
                 src/zkir-v3/*.agda (Assumptions.agda is the trust base)
Kompiled definitions exist already (experiments/zkir-k/semantics/*-kompiled); do not delete them. The prior audit's accepted findings and fixes are in experiments/zkir-k/review-2026-09-05/CONSOLIDATED.md; do not re-report them unless they regressed.

Report format (Markdown, write it in full, no placeholders):
1. Verdict line: `VERDICT: APPROVED | WARNING | BLOCKED` with one sentence.
2. Findings, most severe first. Each finding: severity (blocker / major / minor /
   nit), title, file:line, what the K code says, what the source of truth says,
   why it matters (which program or preimage would expose it), and a concrete fix.
3. Coverage gaps: instructions, types, error paths or preimage shapes the tests
   do not exercise.
4. Questions for the authors (things you could not decide from the sources).
5. What you checked and how (commands run, files read), so the architect can
   reproduce your reasoning.
Do not restate the design; do not praise. Be specific. Length: as long as the
findings need, no longer.
