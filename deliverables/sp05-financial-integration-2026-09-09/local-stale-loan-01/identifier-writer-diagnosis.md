# Durable adverse-result writer: identifier width mismatch

Reproduced against unchanged `sp05-adverse-integration/experiments/moriarty-midnight-financial/ledger/launch-local.mjs`.

`hex` at line25 accepts exactly 64 lowercase hexadecimal characters. `validatePublicAdverseValue` at line148 uses it for each `reservations.identifiers` item. Actual retained SDK/indexer identifiers in `ledger/fixtures/historical-dust/response.json` are 66 hexadecimal characters. The actual native/public-transaction writer (line99) and event writer (line164) already use a different bounded variable-length identifier predicate. Transaction hashes and transaction identifiers were conflated in the new reservation writer.

Minimal connected reproduction: `/tmp/moriarty-adverse-identifiers-reproduction.test.mjs`; command `node --test /tmp/moriarty-adverse-identifiers-reproduction.test.mjs`; raw `/tmp/moriarty-adverse-identifiers-red.tap`. Complete controlled INCOMPLETE result with synthetic64 identifier writes successfully. The same result with actual retained public66 identifiers throws `LAUNCH_ADVERSE_RESERVATIONS` before `integration-result.json` retention. Result: 1 pass / 1 expected failure.

This establishes a concrete sufficient cause for a missing integration result when the actual reservations contain66 identifiers. It does not recover or establish the actual after-state or nonmutation result. Current attempt's public copy was not present at the initial check; this reproduction uses existing historical public identifiers and explicitly controlled other result fields. Candidate and terminal rejection evidence remain independently preserved by their earlier writers. No new submission is needed to reproduce or repair this defect.

Next narrow repair should use an explicit SDK transaction-identifier validator, preserve strict64 hash checks for hashes, and test actual SDK producer identifiers through the complete durable writer, including malformed controls. Do not silently synthesize a missing actual integration result from this fixture. Any actual after-state not retained remains unknown pending independent public evidence.

Only /tmp test/report/log and owned temporary fixture directories were written; fixture directories were cleaned. Frozen source/evidence, wallets and services were untouched.
