# Independent GPT-6 high R2 helper audit

**REQUEST_CHANGES.** The seven original probes now pass, but H1d remains incomplete in two reproduced cases. H1a/H1b/H1c/H3a/H3b and the latest-start ordering correction pass the scoped review. H2 remains byte-identical and approved.

1. **Response after abort — medium.** `invocation-handoff.mjs:93` awaits `buildResponse` without rechecking settlement. Abort returns `HANDSHAKE_ABORTED`, then late fulfillment attempts one response write on the destroyed socket. Add the guard after this await and regress abort/timeout during response building. This is an exported protocol API defect; current executor supplies a synchronous builder, and no actual peer receipt is claimed.
2. **Pending replay persistence hidden — medium.** `invocation-handoff.mjs:78,83` detaches replay denial persistence; `executor-caller.mjs:190` derives `outstandingWork` only from setup state. A successful handshake followed by a pending replay denial returns `setup: settled`, `outstandingWork: false`, and cleanup removed while one denial write remains outstanding. Track protocol persistence independently and expose its pending status in results and errors; preserve late completion honestly while suppressing new work on abort.

Independent checks: **115/115 tests passed**, 1775.6 ms; `git diff --check` passed. Eleven negative-probe scenarios produced twelve observation rows, including all seven prior probes, plus three timing replicates. Blocking result reads and cleanup returned within the configured 40 ms total plus a 10 ms scheduling allowance. This external 10-second watchdog was not used as the timing criterion.

Real nonce failure closed and removed its acquired socket. A real socket acquired before the timeout but delivered afterward remained explicitly owned through `lateAllocation`; its late cleanup removed server, socket and directory with zero spawns. Wall rollback did not extend the bound, delayed boot rejected without spawn, and the prover latest-start boundary rejected after boot.

The author's 50 ms timing allowance is looser than needed and exceeds its 40 ms outer bound, contrary to the report's wording. Its abort tests cover state confirmation but omit the final response await; its replay test does not check pending persistence after a successful handshake. Existing `afterEach` teardown was already in base. The historical five-minute hang remains unconfirmed and is not claimed fixed by new teardown.

All eight live and immutable hashes matched before and after review, binding manifest `9332d441df5a1d452cc237fe7973262e6a57fa87d4b106fd90a6735f397b380b`. Root reports author terminal completion with the same hashes. Root also reports full guarded gate queue 1657 passed at 18:54:19 UTC with unchanged hashes. That gate success does not override these source findings.

No task 5.2, production or release approval. C wrapper source exists; broader Python consumer/accounting/financial acceptance and C wrapper integration are outside this repair. No network, proving, Docker, K, real store mutation, implementation, or delegation occurred. Large financial/language suites and C wrapper outcomes are author evidence only.

Artifacts: `/tmp/moriarty-handoff-fable-r2-audit-gpt6-high.json`, `/tmp/moriarty-handoff-fable-r2-probes-gpt6-high.mjs`, `/tmp/moriarty-handoff-fable-r2-probes-gpt6-high.jsonl`, `/tmp/moriarty-handoff-fable-r2-tests-gpt6-high.tap`.

## Frozen SHA-256 manifest

- `experiments/moriarty-midnight-financial/ledger/executor-caller.mjs`: `a4b54aad06d74f6b286f3c1463d359fb0bda40626c277ef91f9784c912c9f1dc`
- `experiments/moriarty-midnight-financial/ledger/executor-caller.test.mjs`: `019723faffdf41e2d6be8c74c2c8806f113378fe8a11f6cbb9c069be40319223`
- `experiments/moriarty-midnight-financial/ledger/fault-matrix.mjs`: `1d6b52248b8b1748e554395cfd22ee0075141de261e082edd5063a7fda4619dc`
- `experiments/moriarty-midnight-financial/ledger/fault-matrix.test.mjs`: `3a2fc296104a0928095e5e413e6329353be0fb5a99d6eff00688fb278ef857b6`
- `experiments/moriarty-midnight-financial/ledger/invocation-handoff.mjs`: `b47797546c5982a6f8f1b5b05d5e320fd9f410ab83148a2d1130bd42ff205519`
- `experiments/moriarty-midnight-financial/ledger/invocation-handoff.test.mjs`: `70f2b1cd1eabe05d18331d36420415d45c56700305a305bcc89dd623da338f5f`
- `experiments/moriarty-midnight-financial/ledger/prover-lifetime.mjs`: `a046a77a17cd38ab4d0a97e3df601add97ebde8443e3303d1dfc1a9bb8967cdb`
- `experiments/moriarty-midnight-financial/ledger/prover-lifetime.test.mjs`: `693f88c809668f0daf513f8b5ed12826d4facfea72a66a9eda160ac373d5e6ed`
