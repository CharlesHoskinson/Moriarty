# Offline lifecycle comparison

The `lifecycle-corpus` command executes the complete loan lifecycle through the public Source/5 and Core/4 APIs. Each path consumes its own preceding output. Both use the existing TypeScript implementation; the independent financial oracle is the retained September 12 fixture, not a second semantics implementation.

```sh
npm --prefix experiments/moriarty-language run lifecycle-corpus
```

Use Node.js 24. The command needs no wallet, K runtime or network service. It prints complete JSON observations and exits nonzero on source/evidence failure, incomplete execution or mismatched results. The emitted report explicitly says `kExecuted: false` and `kStatus: "not-executed"`.

Four stages preserve debt 0 → 100 → 110 → 80 → 0, complete effects, liabilities, identities, unrelated state, and cumulative work. Nine negative cases cover duplicate accrual, repeated/early periods, the incurred-liability cap after partial repayment, failed funding, settled-source rejection, late false ensures, malformed unrelated allowance and work mismatch. Eight cases have valid continuation/retry controls; a settled loan has no further valid payment. These nine cases do not exhaust the wider task 4 requirements.

The corpus checks complete rejection envelopes, including source spans and work where applicable, and refuses published state/effects on rejection. Diagnostic spans are derived directly from source text. Source and Core input isolation and comparator failures have deliberate mutation controls.

Validation on September 17:

- Full language suite: 890 passed, 0 failed, 0 skipped; retained in `full-tests.txt`.
- Typecheck and strict OpenSpec validation passed.
- `corpus-result.json` retains the actual command result.
- `independent-probes.mjs` and its output verify full observations and deliberately divergent Core state.
- The initial fresh-machine baseline had 879/883 passing because Compact tools were absent. Restoring official Compact 0.31.1 / language 0.23.0 / runtime 0.16.0 resolved all four failures without changing or skipping their tests. `environment.json` records versions and downloads.

Implementation started from the preserved corpus at commit `55011b8f8f8d69ca5ba78dc65e87baab5ee72c1c`, recovered before branch cleanup. The new implementation removes the cross-worktree evidence fallback, independently chains both paths, executes negative cases, and makes failures affect CLI exit status.

This is an offline comparison prerequisite. It does not execute K, reproduce the historical native crash, prove correspondence, close task 4, or establish authenticated Docker/Preview lifecycle execution. The old compiled trace106 artifacts and pinned K runtime are absent from this checkout. The recovered source alone cannot reproduce that historical compiled experiment.

The frozen candidate is listed in `candidate.json`. Independent audit receipts, when present, apply only to their recorded candidate and scope.
