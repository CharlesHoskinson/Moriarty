# Opus 5.5 Quint check classification: bounded ordinary only

Reviewer: Opus 5.5 (`claude-opus-5-5`), high effort, security owner. I read source only and ran nothing: no Quint, Node, Cargo, build, test, SDK, VM, proof, keygen, ledger or network command. The only file I wrote is this one. Native authority is still 0.

## Verdicts

| # | Command (cwd `R/quint-continuations`) | Verdict |
|---|---|---|
| 1 | `typecheck continuation_prefix.qnt` | **APPROVE_ORDINARY** |
| 2 | `run … --backend=typescript --max-steps=1 --max-samples=4 --seed=20261015 --witnesses firstBoundaryReached --verbosity=2` | **APPROVE_ORDINARY** |
| 3 | `typecheck continuation_prefix_test.qnt` | **APPROVE_ORDINARY** |
| 4 | `test … --backend=typescript --match '^(escrowStartTest|reserveStartTest)$' --max-samples=1 --seed=20261015 --verbosity=2` | **APPROVE_ORDINARY** |

Run the four in order and stop at the first nonzero exit or timeout. Each verdict holds only for the exact argv in `PROPOSED-CHECKS.json` and only once the prerequisites below are met. These verdicts say nothing about model correctness, correspondence with the Rust implementation, or security acceptance.

## What I checked (repository observation)

- **Pins I could verify.** I re-hashed these files myself and they match `INPUTS.json`: `ORACLE-CONTINUATIONS.json` `76075b63…9640`, `ORACLE-CONTINUATIONS.md` `7758…471b`, `oracle-escrow-v2.mori` `2eb9…60a`, `oracle-reserve-v2.mori` `260f…530`, `continuation_prefix.qnt` `f303…d7f` and `continuation_prefix_test.qnt` `b99f…21`.
- **Pins I could not verify.** My sandbox would not hash anything under `~/.npm-global`. So the four tool pins (`package.json` `0835…`, `cli.js` `ac12…`, `cliCommands.js` `b186…`, `builtin.qnt` `8d1f…`) are recorded but not re-verified by me. `package.json` does read version 0.32.0.
- **The model files.** `continuation_prefix.qnt` imports nothing. `continuation_prefix_test.qnt` imports only `./continuation_prefix`. No Rust, candidate, compiler or evaluator code is imported or invoked. The literals match the README/ADAPTER oracle rows: escrow alice 100→90, escrow account 0→10, duty 10, captures `[10]`, work 5; reserve basis 40→35, duty 5→47, captures `[2,42]`, work 11.
- **Call path.** `cli.js` hands typecheck to `load→parse→typecheck→outputResult`, run to `…→runSimulator`, and test to `…→runTests`.
  - **Reads:** `load` reads the input file. `sourceResolver.js:42` reads imports relative to the input's directory. `builtin.js:27` reads `dist/src/builtin.qnt`. Nothing touches SRS files, keys, HOME config or `~/.quint` on these paths.
  - **Backend selection:** `runSimulator` (`cliCommands.js:306`) and `runTests` (`:231`) take the Rust `CommandWrapper` only when `backend === 'rust'`. That wrapper is what reaches `binaryManager.getRustEvaluatorPath`, which can download from `api.github.com` into `QUINT_HOME`/`~/.quint` and then spawn the binary. With `--backend=typescript` the in-process `Evaluator` runs instead. The default is `rust`, so the explicit flag is required (commands 2 and 4 pass it). Typecheck has no backend and never reaches an evaluator.
  - **Network and processes:** `apalache.js` loads grpc, tar and `child_process` at module load, but its download, spawn and gRPC code sits inside functions reached only through verify/compile. I found nothing that runs on load. I saw no other fetch or spawn on these four paths.
  - **Writes:** files are written only through `--out`, `--out-itf`/`--output` or `--source-map`, and none of these commands pass them. All output goes to stdout/stderr.
  - **Environment reads:** `QUINT_HOME` (`config.js:30`, Rust/Apalache paths only) and `QUINT_RETRY_NONDET_SMALLER_THAN` (`nondet.js:107`, which changes how nondeterministic choices are sampled). Node itself reads `NODE_OPTIONS` (`--require`/`--import` can inject code) and `NODE_PATH`. Chalk reads color variables, which only changes how output looks.
- **Bounds.** `--seed` together with `--max-samples` fixes the sample count. `n-traces` defaults to 1, within the limit of 4. `--max-steps=1` gives exactly one `start`. Quint has no internal timeout (its issue #633 is commented out), so the outer `timeout 30s` + `--kill-after=5s` is the only limit on how long each command can run.

## Prerequisites (ROOT, before command 1)

1. Clear `NODE_OPTIONS`, `NODE_PATH`, `QUINT_HOME`, `QUINT_RETRY_NONDET_SMALLER_THAN` and any other `QUINT_*` or tool-specific override, without logging their values. Leave HOME/CODEX_HOME alone and TMPDIR at its default.
2. Re-hash the four tool pins and the six model/oracle pins against `INPUTS.json`. Stop on any mismatch.
3. Record one SHA-256 tree digest of `quint/dist/src/**` and `quint/node_modules/**`, plus a hash of the absolute Node binary, both before and after the run. The tree and the binary's own version are not pinned today; `/usr/local/bin/node` is only described as resolving to v24.21.0.
4. Use the absolute Node path from the argv, not a `node` or `quint` found through PATH. Set cwd exactly to `R/quint-continuations`.
5. Capture stdout, stderr, exit code and elapsed time to `R/checks/` only.
6. Before and after, list `R/quint-continuations` and check that `~/.quint` is absent or unchanged. Any new file, any `out_*.itf.json`, or any change under `~/.quint` is a failure.
7. Run with network denied if the host allows it. That turns an accidental Rust or Apalache path into an error instead of a download.

## Residuals (not closed by this classification)

- The JS dependency closure (yargs, chalk, lodash, antlr4ts, seedrandom, immutable, grpc and the others) is checked only by digest. I did not read its source.
- I did not run the Node binary, so its version and integrity are unverified until step 3.
- Four samples cannot guarantee that both specimens appear. Coverage of both comes from the two explicit tests. The witness count is about whether the model executes, not about any property.
- The model uses mathematical `int`. It does not model UInt128, serialization, ledger state, proofs or adversaries. Passing runs cannot establish Rust correspondence or any U/M/MC/SP/G exit.
- The full Muse and Opus reviews of the candidate and of the results are still owed. No parent exit is closed. Do not expand the model until commands 1–2 succeed and their outputs have been reviewed.

## Missing files

None of the missing files block these commands. Before the next increment, add a pinned digest manifest covering `dist/src` + `node_modules` and the Node binary to `INPUTS.json`.
