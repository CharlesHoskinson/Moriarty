# First continuation-model increment — unexecuted

This draft models initialization and **one successful prefix transition** for each independently frozen escrow/reserve normal specimen. It imports no Moriarty evaluator, compiler, emitted artifact or observed candidate result. Input identity is `ORACLE-CONTINUATIONS.json` SHA-256 `76075b6350f83ba2776a4466ed9eb4dada7e9d1da0c509a366026e6b98ec9640`. `INPUTS.json` pins the oracle, its explanatory/source files and the inspected local Quint entry sources.

The governing requirements flow is quint-modeling plus the quint-lang CLI/test/simulation references. The explicit task authorizes this single draft increment while execution remains withheld. The skill requires actual typecheck **and run** before expanding: neither has occurred, so this is not a completed executable model and no suffix transition is drafted.

## Shape and normal coverage

One invocation owns one cohesive `FinancialFrame` record. The two specimens are initialization choices, not interacting processes; accounts are balance-map keys. This is a single-invocation shared financial state model, so plain Quint applies without Choreo or a message medium. There is no time, crash, message delivery, adversarial participant or concurrency model in this increment.

`initialFrame`, `canStart` and `applyStart` are pure. `initCase` assigns the record, `init` chooses Escrow or Reserve, and the only business transition `start` combines its guard with one record assignment. `step` selects that transition. After reaching site 1, this first increment intentionally stops with no enabled next transition; no stutter fallback is added.

| Oracle rows | Draft observation |
|---|---|
| `escrow.opening`, `escrow.prefix_waiting` | Input10; alice100→90, escrow0→10, bob0/observer42/marker77 preserved; absent duty becomes present10; captures `[10]`; attempted/committed5; ordered transfer then liability effect. |
| `reserve.opening`, `reserve.prefix_waiting` | Input2; held40+2=42 captured before basis40→35; allocated0/marker77/all balances preserved; existing duty5→47; captures `[2,42]`; attempted/committed11; ordered state then liability effect. |

The separate test file contains two **normal example** tests with literal oracle expectations. `firstBoundaryReached` is a positive witness, false at initialization. There are no security invariants or adversarial tests. Detailed value/type/ordinal interpretation is in `ADAPTER.md`.

This draft omits successful resumes, false guard observations, commercial predicate rejection, accepted-event capture, later fees/discharge, interleaving, serialization, target/ledger/proof correspondence and all authorization/consumption analysis. Those omissions are not assertions about the implementation. No model/Rust adapter program or runtime harness is introduced.

## Local tool discovery and proposed checks

Read-only shell lookup found `/home/charl/.npm-global/bin/quint`, a symlink to the installed JS CLI; its package file records **0.32.0**. Installed CLI source explicitly registers `typescript`/`rust` backend choices for run/test, with Rust as default. The proposals select `--backend=typescript` explicitly. The absolute Node target and CLI source paths below were discovered without executing either program. Typecheck has no evaluation-backend flag. Imported/transitive JS package behavior and environment controls remain for Opus classification.

`PROPOSED-CHECKS.json` records exact argv/cwd/bounds. The following commands are **candidates for later Opus classification/root dispatch only**; none ran. Execute sequentially, stop on first failure, and do not expand the model before the first typecheck/run succeeds. A four-sample first run is for executability/witness inspection, not confirmation of any property. Two explicit example tests select both specimens; four nondeterministic traces alone do not guarantee both were sampled.

```bash
timeout --signal=TERM --kill-after=5s 30s /home/charl/.foreman/tools/fnm/node-versions/v24.21.0/installation/bin/node /home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js typecheck continuation_prefix.qnt
```

```bash
timeout --signal=TERM --kill-after=5s 30s /home/charl/.foreman/tools/fnm/node-versions/v24.21.0/installation/bin/node /home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js run continuation_prefix.qnt --main continuation_prefix --backend=typescript --max-steps=1 --max-samples=4 --seed=20261015 --witnesses firstBoundaryReached --verbosity=2
```

```bash
timeout --signal=TERM --kill-after=5s 30s /home/charl/.foreman/tools/fnm/node-versions/v24.21.0/installation/bin/node /home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js typecheck continuation_prefix_test.qnt
```

```bash
timeout --signal=TERM --kill-after=5s 30s /home/charl/.foreman/tools/fnm/node-versions/v24.21.0/installation/bin/node /home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js test continuation_prefix_test.qnt --main continuation_prefix_test --backend=typescript --match '^(escrowStartTest|reserveStartTest)$' --max-samples=1 --seed=20261015 --verbosity=2
```

The runner uses default `init` and `step`. All command outputs, selected-test counts and traces are still absent. No `verify`, Rust backend, native proof/VM, installation or network command is proposed. Any future sampled trace reports only this abstract model's sampled behavior; it cannot establish Rust/UInt128, arbitrary-witness, compiler or ledger correspondence. All U/M/MC/SP/G exits remain open.
