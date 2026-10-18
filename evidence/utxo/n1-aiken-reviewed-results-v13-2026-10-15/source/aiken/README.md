# Aiken source and dependency snapshot

This directory retains the exact 16 package files used by the captured local run.
It also supplies the captured `aiken.lock` and package map.
All 38 captured stdlib source and metadata files remain under `build/packages/aiken-lang-stdlib/`.
The [layout manifest](../PORTABLE-LAYOUT.json) identifies all 56 Aiken snapshot files.
The [public identity comparison](../AIKEN-PUBLIC-SOURCE-IDENTITY.json) checks the 16 package bodies against existing placement evidence.

The local run used Aiken `v1.1.24+bacbeb3` and stdlib `v2.2.0`.
The source view contains no Aiken executable or executable-bit requirement.
No compiled blueprint or execution output is mixed into this direct source view.
Captured blueprints remain under `captured/current-cardano-p3-r1-aiken-local-execution-20261015/` at the leaf root.
The same captured tree retains full help, journal, log and receipt bodies.
Root launch and observation records remain in the adjacent captured root-record directory.

The publication layout has not run `aiken check`, `aiken build` or blueprint operations.
The direct source layout matches the captured run structure, including pinned dependency bodies.
It is an unrun reproduction input, not a new package installation result.
Separately reproduce only with the pinned executable and inspected command settings.
The old operator steps retain absolute paths and their original authority limits.

The [Opus actual-result report](../../captured/current-cardano-p3-r1-aiken-independent-actual-result-review-20261015/REPORT.md) supplies the judgments.
The [root adoption](../../captured/current-cardano-p3-r1-aiken-root-run-records-20261015/ADOPTION.json) retains PB-01–PB-12.
A1 captured 146 passing CEK library tests, including 37 expected-failure tests.
It did not establish compiled-wrapper behavior, signature positives, Core correspondence, Groth16 or ledger acceptance.
The applied S2 size is 30,764 bytes against observed `maxTxSize` 16,384.
Opus reports host-derived estimates of 2.8–3.7 times memory and 1.4–1.9 times steps.
Those host estimates are not ledger ExUnits.

The [license supplement](../../supplements/stdlib-license/MANIFEST.json) captures Apache-2.0 source attribution at stdlib commit `779e9025356a85484475e06d5a14acb032bcdcc9`.
Its [LICENSE](../../supplements/stdlib-license/LICENSE) retains the exact 11,340-byte source body.
The supplement adds license provenance, not installed or deployed dependency qualification.

The unchanged report contains the older `297-entry` phrase.
The later [count qualification](../../supplements/PARAMETER-COUNT-QUALIFICATION.json) records 350 values for the named snapshot.
That correction does not settle parameter ordering, evaluator selection, authenticated parameters or deployed identity.
No funds, proof, deployment or Preprod result follows from this snapshot.
