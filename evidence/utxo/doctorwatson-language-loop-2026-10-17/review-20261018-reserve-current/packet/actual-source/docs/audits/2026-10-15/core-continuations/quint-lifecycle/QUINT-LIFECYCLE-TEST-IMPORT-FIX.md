# Lifecycle test import correction — source only

Actual failure: `checks/quint-lifecycle-03` exited1 during test-module name resolution, with QNT404 at15 Escrow/Reserve constructor uses. Its stdout/stderr/result captures remain unchanged. The lifecycle module's own typecheck and32-trace TypeScript run previously exited0; this task does not repeat them or infer any Rust correspondence.

Root cause: the test imported `continuation_lifecycle.*` and assumed that module's prefix constructor imports were transitive. Installed Quint0.32.0 `dist/src/names/collector.js:140` explicitly documents that imported names are hidden from downstream imports unless exported. The lifecycle model directly imports the constructors and therefore resolved them; the test module did not. The installed `enterImport` path supports direct single-name imports, already used by the passed model.

The only correction adds direct `continuation_prefix.Escrow` and `continuation_prefix.Reserve` imports to the test file. All nine tests, literal expected states, financial effects/work/ordinal expectations and model semantics remain unchanged. No export, model, prefix, oracle, product source or historical manifest is edited.

- Old test SHA-256: `9a923041c30dba26a97e80cbd9966da195328ec57867a73b6d75a87a3fe07fc9`.
- New test SHA-256: `dc39c999921e51737bb3db17b5349e3a78fb8687a04408ff01a042261c161d6e`.
- Exact old bytes archived outside the model directory: `/home/charl/Moriarty/.moriarty-dev/runtime/core-continuations-20261015/quint-lifecycle-test-before-import-fix.qnt`.
- Minimal diff: `QUINT-LIFECYCLE-TEST-IMPORT-FIX.patch`.
- New standalone delta pins: `QUINT-LIFECYCLE-TEST-IMPORT-FIX-PINS.json`.

The original six files still match their prior pins. The lifecycle model also matches its previous manifest. `LIFECYCLE-INPUTS.json` remains immutable; the new delta records only the updated test-file identity and pins all unchanged neighboring files.

Only previous commands3 and4 are proposed below, unchanged apart from the corrected test input bytes. `QUINT-LIFECYCLE-TEST-FIX-CHECKS.json` is the new exact packet. Both remain unexecuted pending Opus delta classification/root dispatch; stop on the first failure. Passed model typecheck/simulation are excluded from this retry.

```bash
timeout --signal=TERM --kill-after=5s 30s /home/charl/.foreman/tools/fnm/node-versions/v24.21.0/installation/bin/node /home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js typecheck continuation_lifecycle_test.qnt
```

```bash
timeout --signal=TERM --kill-after=5s 30s /home/charl/.foreman/tools/fnm/node-versions/v24.21.0/installation/bin/node /home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js test continuation_lifecycle_test.qnt --main continuation_lifecycle_test --backend=typescript --match '^(escrowFalseTest|escrowTrueTest|escrowFalseThenTrueTest|reserveFalseApprovalTest|reserveTrueApprovalTest|reserveFalseSettlementTest|reserveTrueSettlementTest|reserveFalseThenTrueTrajectoryTest|reserveBusinessRejectionTest)$' --max-samples=1 --seed=20261016 --verbosity=2
```

No Quint, Cargo, formatter, SDK, native, ledger or product command ran in this correction task. The hypothesis is supported by the actual failure and installed resolver source; success of the correction is not yet observed. No security judgment or parent acceptance is supplied.
