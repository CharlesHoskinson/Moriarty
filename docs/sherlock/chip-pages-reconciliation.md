# Chip animation + Pages reconciliation (sherlock-s3, 2026-10-26)

Reconciles `visualization/` (the chip animation) and the Pages site (`site/`,
built by `.github/workflows/site.yml`) with the current Sherlock design.
Docs-only; ledger 9, the node, the Midnight SDK and the proof server unchanged.

## Current facts reflected

- U031 export plans: constraint and witness plans are separate typed artifacts;
  `forbid_op_prefixes` controls apply at rules 149/195. U031 PASS
  (sherlock-s9 461e76d8c, run 20261026T034302Z-U031; evaluator confirmed at
  holdout E=7f0942c3d).
- GP recursion: universal gated leaf proves S consecutive steps per leaf
  (`step_i.post_d == step_{i+1}.pre_d` inside the leaf); IVC node k=20
  (531,985 rows measured); run decider k=20; public instance is the
  5/6-lane digest instance [binding, prog_d, pre_d, post_d, rule, h_pre];
  arity-4 Poseidon. B group 9/9 on the consolidation line.
- Holdout: E=7f0942c3d (evaluator-owned `qa/holdout`, HMERGE committed
  9e5617e6d into sherlock-s10).
- Consolidation: sherlock-s10 (re-verify line) merges into sherlock-final.
  C2 and C5 PASS on that line (runs 20261026T034453Z); C3/C4 remain blocked
  on U031 ledger re-run / U023 respectively — see
  [c-group-rollup.md](c-group-rollup.md).
- Financial spec: `FinancialItems.metadata.md` is the current financial
  specification; refinement closure is not inferred from it. Lean contracts
  at sherlock-lean d70956d1b+.

## Files changed

- `visualization/index.html` — proof-machinery row now states the GP
  recursion design (S steps per leaf, IVC node k20, decider k20, digest
  instance, arity-4 Poseidon).
- `site/src/components/Roadmap.tsx` — tracks note updated from the stale
  "open after k17 row exhaustion" to the current GP design and B 9/9.
- This file.

## Verification

Pages build verified locally in `site/` (`npm run check`, `npm test`,
`npm run build`); no deploy performed (site workflow is manual-dispatch,
main-only).

## Final docs pass on sherlock-final (sherlock-s3, 2026-10-26)

Consolidation facts: sherlock-final at `c7d3c48a3` (fast-forward line carrying
the sherlock-s10 re-verification merges); evaluator HMERGE `9e5617e6d` set
`qa/holdout` to E=`7f0942c3d`; F4 PASS run `20261026T040123Z`; U024 closed
`035738Z`, U031 PASS `20261026T035253Z` and C3 PASS `20261026T035312Z` on the
consolidation line; Lean locked build 19 run `20261026T035951Z-sherlock-lean-build`
exit 0 (pin: sherlock-lean `0115ab1cf`). U023 status: fold-path fix
`d570c75cc` landed on sherlock-s6, fragments re-running on s2/s7/s4, and the
U023 qualification record remains pending the real stagenet run. A second
small docs pass follows when U023/C4 close. Pages rebuild verified locally
on this branch (typecheck, fixed-set tests, production build); no deploy.
