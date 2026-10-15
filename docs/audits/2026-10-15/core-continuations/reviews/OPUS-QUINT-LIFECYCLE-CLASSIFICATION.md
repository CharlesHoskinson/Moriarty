# Opus 5.5 Quint lifecycle classification: second increment, bounded ordinary only

Reviewer: Opus 5.5 (`claude-opus-5-5`), high effort, security owner. I only read source. I ran no Quint, Node, Cargo, build, test, SDK, VM, proof, keygen, ledger, network or cache command. This is the only file I wrote. Native authority stays at 0.

## Verdicts

| # | Command (cwd `R/quint-continuations`) | Verdict |
|---|---|---|
| 1 | `typecheck continuation_lifecycle.qnt` | **APPROVE_ORDINARY** |
| 2 | `run continuation_lifecycle.qnt --main continuation_lifecycle --backend=typescript --max-steps=4 --max-samples=32 --seed=20261016 --witnesses resumeObserved falseObservationReached completionReached businessRejectionReached --verbosity=2` | **APPROVE_ORDINARY** |
| 3 | `typecheck continuation_lifecycle_test.qnt` | **APPROVE_ORDINARY** |
| 4 | `test continuation_lifecycle_test.qnt --main continuation_lifecycle_test --backend=typescript --match '^(9 named tests)$' --max-samples=1 --seed=20261016 --verbosity=2` | **APPROVE_ORDINARY** |

Each command runs under `timeout --signal=TERM --kill-after=5s 30s` with the absolute Node v24.21.0 path and `quint/dist/src/cli.js`. Run them in order and stop at the first nonzero exit or timeout. The verdicts cover only the exact argv in `PROPOSED-LIFECYCLE-CHECKS.json` (sha256 `22c6a8ec…3c3f`). They are not a judgement of model correctness, Rust correspondence or security acceptance.

## Repository observations

- **Pins I re-hashed, all matching `LIFECYCLE-INPUTS.json`:** the oracle JSON/MD and the escrow/reserve `.mori` files, the six original files (ADAPTER, INPUTS, PROPOSED-CHECKS, README, `continuation_prefix.qnt` `f303…d7f`, `continuation_prefix_test.qnt` `b99f…21`, all unchanged), `continuation_lifecycle.qnt` `f8690924…6b7d`, `continuation_lifecycle_test.qnt` `9a923041…fc9`, `LIFECYCLE.md` `2336…b7` and `PROPOSED-LIFECYCLE-CHECKS.json`.
- **New pins** (they cannot appear in their own manifest): `LIFECYCLE-INPUTS.json` = `33c574e4f35a15cecfcefbf5384f472fdce29aac124f91b01b685242923a847e`, `QUINT-EXECUTION-PINS.json` = `488f26152f76da9fc16156fec10e65b1a210287c00222872cd3a133677dbbb38`.
- **Imports.** `continuation_lifecycle.qnt` imports six names, all by relative path `"./continuation_prefix"`: `Specimen`, `Escrow`, `Reserve`, `FinancialFrame`, `initialFrame` and `applyStart`. Each is a type, a constructor or a `pure def`. It does not import `boundary`, `init`, `start` or `step`. `continuation_lifecycle_test.qnt` imports only `continuation_lifecycle.*` from `"./continuation_lifecycle"`. There is no `var` code, remote, URL or absolute-path import, file read, SDK, Midnight, Rust candidate, compiler or evaluator reference.
- **Model content.** The model adds one business action, `resume`, over a finite normal event domain: Delivery true/false, Approval 41/43/51, Settlement true/false. The four witnesses are positive reachability `val`s. There is no invariant and no adversarial or hostile domain. The test literals match the oracle-derived values in LIFECYCLE.md: escrow 5→11, reserve 11→26→36, the quote-51 business rejection at step 9, and false observations keep `c` unchanged.
- **Tool path.** All four commands use the same subcommands, the same Node path and the same `cli.js` as the four approved and passed prefix commands. With `--backend=typescript` they never reach the Rust `CommandWrapper` or `binaryManager`, so there is no download into `~/.quint`. Typecheck has no backend option. No `--out`, `--out-itf`, `--output` or `--source-map` is given, so nothing is written to disk. I am reusing the prior closure reading as an unchanged premise. It holds only if ROOT re-verifies it (preconditions 2–3).
- **Bounds.** 32 samples × at most 4 `resume` steps, 9 tests × 1 sample, and 30 s each, enforced by the outer timeout. Quint has no internal timeout.

## Preconditions (ROOT, before command 1)

1. Unset `NODE_OPTIONS`, `NODE_PATH`, `QUINT_HOME`, `QUINT_RETRY_NONDET_SMALLER_THAN` and every other `QUINT_*` variable without logging their values. Keep HOME and CODEX_HOME. Leave TMPDIR at its default.
2. Re-hash every pin in `LIFECYCLE-INPUTS.json` plus the two new pins above. Stop on any mismatch.
3. Recompute the `dist/src/**` + `node_modules/**` tree digest and the Node binary hash, and compare both to `QUINT-EXECUTION-PINS.json` before and after each command. Any drift is a failure.
4. Invoke the absolute Node and `cli.js` paths exactly, not through PATH. cwd must be exactly `R/quint-continuations`.
5. Write stdout, stderr, exit code and elapsed time only to new `R/checks/quint-lifecycle-01..04`.
6. Record the source directory file listing and the state of `~/.quint` (absent or digest) before and after. Any new file, including `out_*.itf.json`, or any `~/.quint` change is a failure.
7. Deny network if the host supports it.

## Residuals (not closed)

- **Possible ordinary failures, not safety issues.** (a) Quint imports are not re-exported unless declared with `export`, so `continuation_lifecycle_test.qnt` may fail to resolve `Escrow` and `Reserve` through `continuation_lifecycle.*`, and command 3 would fail. Do not patch this under this classification. A source fix needs a new pin and a new classification. (b) After `Complete` or at `site == 0`, `step` is disabled. How the TS simulator handles a 4-step trace that hits this (trace ends early, or deadlock reported) may affect command 2's exit status. Treat either outcome as an observation to review, not as acceptance.
- Witness counts show only that the model executes. Thirty-two random samples need not reach `businessRejectionReached`; the explicit test supplies that coverage.
- The model uses mathematical `int`, not UInt128. The adapter's limitations still apply: no serialization, ledger, proof, adversary or Rust correspondence. No U/M/MC/SP/G exit or parent exit is closed.
- I have still not read the JS dependency closure source; it is trusted only by digest.
- The full Muse and Opus feature audits of the candidate and of these results are still owed. Do not expand the model until all four outputs have been reviewed.
