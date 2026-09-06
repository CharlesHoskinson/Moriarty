The audit verdict is **REVISE**. I could not save `experiments/zkir-k/docs-surge-2026-09-05/audits/08-fm-codex.md` because this session permits only filesystem reads. The report follows.

# Audit of 08-constraints-and-verdicts.md (formal methods expert)

Verdict: REVISE

Checks performed:

- Compared all six constraint constructors, emission rules, outcomes, and named helpers against the K sources.
- Checked coordinate operations, widths, commitments, and chip selection against the pinned Rust sources.
- Inspected midnight-circuits 7.2.4 width assertions and coordinate construction.
- Compared runner and harness descriptions with their Python implementations.
- Verified recorded comparison totals and divergence outcomes.
- Recomputed the example’s eight emitted constraints from its program; did not execute the example command.
- Confirmed one chapter title, no em-dashes, and no placeholder or drafting terminology.

## Findings

### F1 blocking: Coordinate checks attributed to `into_coordinates`

Claim: Chapter line 123 says both `from_coordinates` and `into_coordinates` re-check on-curve and subgroup facts.

Evidence: Repository observation: `experiments/zkir-k/semantics/zkir-constraints.k:368–376` checks chip availability, rejects Weierstrass identities, and compares extracted coordinates. `experiments/zkir-k/semantics/zkir-ops.k:154–162` extracts coordinates without testing curve or subgroup membership. The pinned Rust implementation, `repos/_build/ledger-92e8bdd3/zkir-v3/src/ir_instructions/into_coordinates.rs:101–132`, likewise extracts coordinates, adding nonzero assertions only for the two Weierstrass curves.

Fix: Replace the sentence with: “`from_coordinates` checks exact curve and subgroup membership. `into_coordinates` compares extracted coordinates and rejects Secp256k1 and Secp256r1 identities; it does not independently re-check curve or subgroup membership.”

### F2 blocking: Subgroup membership “after cofactor clearing”

Claim: Chapter line 57 says `#fromCoordsPt` demands subgroup membership “after cofactor clearing.”

Evidence: Repository observation: `experiments/zkir-k/semantics/zkir-curves.k:139–141` returns the original `pt(X, Y)`. `experiments/zkir-k/semantics/zkir-constraints.k:398–400` tests `inSubgroup(C, Q)` on that unchanged point. For Edwards curves, `zkir-curves.k:82–84` tests whether multiplying the original point by the subgroup order yields the identity. No cofactor clearing occurs in this path. The recorded order-two counterexample is rejected at `evidence/zkir-k-divergence-tests-2026-09-05b.txt:25–27`.

Fix: Replace “after cofactor clearing” with “the original point must already be in the prime-order subgroup.” Keep any explanation of the crate’s assignment mechanism separate from the K predicate.

### F3 blocking: Incorrect `less_than` outcome

Claim: Chapter line 128 describes `holds` as expected for `less_than` whose off-circuit bit bound failed.

Evidence: Repository observation: `experiments/zkir-k/semantics/zkir-vm.k:385–394` stops before writing the output when the bound fails. `zkir-constraints.k:314–315` compares against that output, and `zkir-constraints.k:195` yields `unknown` when it is absent. The recorded example explicitly reports `gate=less_than:unknown` at `evidence/zkir-k-divergence-tests-2026-09-05b.txt:7–9`. Chapter line 67 already describes this correctly.

Fix: State that the padded relation could accept an appropriately completed witness, but the actual failed witness run produces `unknown` because its output register is absent.

### F4 major: “Successful run” conflates status with constraint acceptance

Claim: Chapter line 147 defines a successful run as finished `ok` with an empty `violations` list.

Evidence: Repository observation: `experiments/zkir-k/tools/zkir_run.py:310–324` determines status independently of verdicts, which are collected at lines 336–350. `tools/diff_test.py:227–229` explicitly records successful `ok` runs with non-holding gates. The recorded `f05` example has `K=ok` and `commGate:violated` at `evidence/zkir-k-divergence-tests-2026-09-05b.txt:10–12`.

Fix: Replace the opening with: “For evidence that the honest witness satisfies the modelled relations, require both a finished `ok` run and an empty `violations` list.”

## Coverage

- Constraint constructors and emission: covered.
- Outcomes and named rule shapes: partly; correct F2 and F3.
- Chip gating and K4: covered.
- Width limits: covered.
- Witness-side versus circuit-side commitment: covered.
- Verdict guarantees and abstraction limits: partly; correct F1.
- Harness use of verdicts: partly; correct F4.