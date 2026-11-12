# mca-lean: a Lean proof of the Reed-Solomon MCA bound that the Sherlock calculator uses

Owner decision OD-19 option (b), 2026-11-11.

## Target

Prove in Lean 4 the mutual correlated agreement (MCA) bound for Reed-Solomon codes beyond the
unique decoding radius that the soundness calculator uses
(`repos/stark/openvm-org_stark-backend/crates/stark-backend/src/soundness/calculator.rs:534`,
`whir_proximity_gap_security` and `log2_a_bound_bchks25`). With this proof, the internal and `L`
layers of the Sherlock design meet "proven 100 bits, no conjecture".

The calculator bound is the BCHKS25 linear-in-`n` bound:

- error `d * ceil(a) / q`, with `a = 2 D_X D_Y^2 D_Z + (gamma n + 1) D_Y`;
- `D_X = (m + 1/2) n sqrt(rho)`, `D_Y = (m + 1/2) / sqrt(rho)`, `D_Z = (m + 1/2)^2 / (3 rho)`;
- `m = 3`, agreement `(4/3) sqrt(rho)` (that is, `gamma = 1 - (4/3) sqrt(rho)`);
- `d = 1` for folding (the generator `Gen(2; alpha) = (1, alpha)`);
- `d = w - 1` for batching (the generator `Gen(w; mu) = (1, mu, ..., mu^(w-1))`, a degree `w - 1`
  curve).

The MCA event is WHIR (eprint 2024/1586) Definition 4.9. The paper proof is in
`research/sherlock-mca-proof-20261111/` (STATEMENT.md, AUDIT.md, PROOF.md). The Lean statements
follow STATEMENT.md. Where this project and STATEMENT.md differ, STATEMENT.md wins and this project
changes.

## Definition of done

1. 0 `sorry` in the project.
2. `#print axioms` on each main theorem shows only `propext`, `Classical.choice` and `Quot.sound`.
3. The hard lemma (the Johnson-range MCA exception bound) is never a hypothesis, a structure field
   or a premise of a main theorem. It is proved.
4. An instantiation theorem at the frozen `L` and internal parameters proves the bit bound that the
   calculator uses.

Note (coordinator, 2026-11-11): OD-19 now runs unique decoding in production. This proof is for
bringing list decoding back later (k = 19). The definition of done is unchanged.

## Layout

- `MCA/Defs.lean`: definitions written from the papers over Mathlib only (RS code on an evaluation
  domain, relative Hamming distance and agreement, correlated agreement, MCA as WHIR Def 4.9, the
  fold generator, the degree `w - 1` curve generator).
- `MCA/Proof.lean`: the PROOF.md targets: Theorem 7.1 (`thm_7_1`, `|E| ≤ a*_M` under (H1) to
  (H6) for every linear subcode and curve degree), Corollary 8.2 (`cor_8_2`, affine subcodes) and
  Lemma 9.0 (`lemma_9_0`, (H1) to (H6) at the 26 frozen rows).
- `MCA/Statements.lean`: the printed BCHKS25 Thm 4.6 form (target T), bad-set cardinality bounds.
- `MCA/Bridge.lean`: equivalence between `MCA/Defs.lean` and ArkLib's `IsMCA` / `mcaError`.
- `MCA/Instantiation.lean`: the bit bound at the frozen parameters (stub until STATEMENT.md pins
  them).
- `MCA/Axioms.lean`: `#print axioms` checks. It is a classic file (no `module` header), because a
  module importer does not load proof bodies.
- `SURVEY.md`: the libraries that exist, with pinned commits, versions, axioms and sorry status.

## Build

Builds run in `heavy.slice`, under the shared lock, with logs in `~/.cache/mca-lean/`:

```
systemd-run --user --unit=mca-lean-build-N --collect \
  -p WorkingDirectory=$PWD \
  flock ~/.cache/cost-opt/measure/heavy.lock /home/hoskinson/.local/bin/heavy-run lake build
```

Each Lean command gets at most 25 minutes, except `lake exe cache get`. Lake resumes an
interrupted build, so a timed-out build continues with the next unit number.
