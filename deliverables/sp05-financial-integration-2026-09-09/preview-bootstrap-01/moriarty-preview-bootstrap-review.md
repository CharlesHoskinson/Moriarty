# Callable existing-wallet Preview bootstrap

Source frozen in `/home/charl/Moriarty/.worktrees/sp05-preview-owner`; manifest `/tmp/moriarty-preview-bootstrap-source.json`. Own four files: new `ledger/preview-bootstrap.mjs`, `preview-bootstrap.test.mjs`, `preview-swap-composition.test.mjs`, and package scripts. All owner4/RPC3/driver2/integration4 source files remain byte-identical to reviewed candidates. Boundary dependency is corrected02, SHA bbb9c9c8f6f75e16996183d96f2db10a3f15ca95a45dd1b5437afa92de835c27. No publication or operational admission.

`launchPreviewFinancialCase(plan)` uses the closed Preview plan/writer, exported existing private-file/password/runtime-inspection/submission helpers, and the inspected SDK restore sequence. `preflightPreviewLaunch(plan)` checks retained build/runtime and explicit Preview RPC genesis before private preparation. Output and new contract-state directory are exclusive, with owner-only existing parents. All3 existing `.midnight-wallet-state/preview/{shielded,unshielded,dust}.json` envelopes must be present and valid before SDK HD/facade/restore. `wallet.stateDirectory` names the runtime ROOT, not its hidden child directory. Missing or invalid state has no create-from-seed fallback.

The existing seed derives the same original Preview payer; network, Bech32/native address and signing key are checked before wallet start, then the restored unshielded state identity is checked. Strict complete child synchronization, available DUST and no pending unshielded/DUST state precede integration. Native public transaction bytes/identifiers are retained before submit via the existing writer. New integration callbacks retain public stages/events/result with actual producer schemas. Bootstrap accepts no injected runtime/provider adapters. Tests use Node module mocks only.

## Persistence correction

Actual wallet-sdk-facade stop at dist/index.js663–669 does not persist snapshots. Existing wallet.ts172+ explicitly calls child serializeState and wallet-state.ts uses version1 envelopes; its convenience path suppresses failures and overwrites without preserving originals. The bootstrap uses those same actual serializeState/string/envelope interfaces with stricter retention. Source hashes are in `/tmp/moriarty-preview-bootstrap-persistence-provenance.json`.

During owned wallet stop, serialize all3 states under a5s cleanup bound, check original private bytes unchanged, preserve them in an exclusive private sibling `.moriarty-backup-<allocationId>`, stage/fsync all replacements, then durably create `.moriarty-persistence-pending.json` before any replacement. Rename each, verify all3 saved bytes and fsync the directory, then remove/fsync the pending marker. A marker at next startup prevents all private reads/SDK restore; no automatic repair. Any interrupted mixed set remains explicitly blocked with originals preserved outside public output. Wallet stop runs in finally even if persistence fails; outer containment remains required. This is a file-set consistency mechanism, not a claim all SDK children serialized the identical chain height. Exclusive admitted wallet use and trusted local filesystem remain assumptions.

## Checks and scope

`npm --prefix experiments/moriarty-midnight-financial run test:preview`:73pass,0fail,0skip, final raw `/tmp/moriarty-preview-bootstrap-final-02.tap`. Includes actual existing restore-call ordering under controlled SDK modules; all missing/invalid persisted children; wrong restored network; reused output; wrong genesis; private persistence success/failure; interrupted second rename retaining marker+backups; pending-marker startup refusal; actual integration-to-writer tests; full swap native observer/comparator/mint-gate composition. No original private files, live network/services/proofs/transactions were used. Synthetic test directories alone were changed/cleaned.

Full swap control uses retained LOCAL native transactions, reconstructed public financial state with only role capability commitments rebound to synthetic secrets, Preview owner encoding, native balance containers and actual independent comparator. It passes all4 stages; changed close recipient fails exact PARTICIPANT_NET_DELTA; pending minted wallet state stops before swap callback. Initial assertion expected a different family of rejection code; retained first log records that fixture correction. This is source evidence, not new Preview settlement.

Raw bootstrap RED/firstGREEN/persistenceRED/combinedGREEN logs remain in /tmp. The source test runner emits its module-mocking experimental/deprecation notices. `git diff --check` passed.

## Actual entry and remaining admission

The package now exposes `preview` for the same explicit hashed-plan CLI:

```
npm --prefix experiments/moriarty-midnight-financial run preview -- --run --plan /ABSOLUTE/ADMITTED/plan.json --sha256 EXACT_PLAN_SHA256
```

This command was NOT run. Public API permits one planned case; it does not allocate another attempt. Before any real run, bind the mandatory Preview hard gate, both source/resource reviews, original wallet and role provenance, exact currently observed chain/protocol/build compatibility, fresh private/output paths/password preparation, gross/DUST/submission limits, and independent shutdown timer. Historical protocol1000000 is not current-head protocol evidence. Preserve all prior reservations/fees/runtime and failed artifacts. Actual wallet persistence/results must be independently checked after execution; no Preview financial acceptance is claimed by these tests.
