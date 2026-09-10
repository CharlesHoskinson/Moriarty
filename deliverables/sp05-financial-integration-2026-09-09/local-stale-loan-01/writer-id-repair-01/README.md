# Adverse reservation identifier retention repair

Source-only correction after local-stale-loan-01. Original candidate196 and admission214 file pins were verified before mutation; original writer and preservation map are in `../original-writer-source/`. Original manifests, admission and attempt outcomes remain unchanged.

The writer confused SDK transaction identifiers with 64-hex transaction hashes. Its reservation validator now uses the same bounded lowercase-hex identifier predicate (1–256 characters) as its existing native submission and public event writers, centralized to prevent drift. Hash checks remain exactly64 hex. No new transaction, result recovery or allocation is performed.

The new `ledger/adverse-writer-identifiers.test.mjs` calls the actual native candidate writer and complete integration-result writer using this attempt's public native candidate and actual66-character identifiers. Other result fields are controlled INCOMPLETE values, never a reconstruction of lost actual after-state. A synthetic64 control still works; actual66 now works; malformed, uppercase, prefixed, oversized and non-string identifiers reject before durable result creation. Temporary test outputs alone are cleaned.

Connected RED:2pass/1fail exact LAUNCH_ADVERSE_RESERVATIONS. GREEN:32pass/0fail/0skip across new identifiers, launch-local and stale-loan-integration tests. Earlier historical reproduction also retained. Command from repo root:

```
node --experimental-test-module-mocks --test experiments/moriarty-midnight-financial/ledger/adverse-writer-identifiers.test.mjs experiments/moriarty-midnight-financial/ledger/launch-local.test.mjs experiments/moriarty-midnight-financial/ledger/stale-loan-integration.test.mjs
```

`git diff --check` passed. Current actual run remains failed/missing integration report, with actual after-state unknown. Independent source audit pending. No private inputs, services, proofs or submissions accessed.
