# Moriarty UTXO work

This branch contains the UTXO research, requirements, experiments, and implementation work in progress.

## Start here

- [Requirements index](requirements-index.md)
- [Requirements coverage](requirements-coverage.json)
- [Experimental backend](../../experiments/moriarty-eutxo/)
- [Current source manifest](publication/source-manifest.json)
- [Source adoption receipt](publication/source-adoption.json)
- [UNIT fixture provenance](publication/UNIT-FIXTURE-PROVENANCE.json)
- [First Cardano sprint: OpenSpec/EARS and approved expected outcomes](../../openspec/changes/cardano-first-contract/proposal.md)
- [Current implementation results and remaining gaps](../../evidence/utxo/current-implementation-417/REPORT.md)
- [Research publication and source inventory](../../research/utxo/PUBLICATION.md)
- [Five experiment designs](../../evidence/utxo/designs/README.md)
- [Muse Wonder model documentation](../../evidence/utxo/model-docs/USAGE.md)

## Acceptance status

The code is an experimental candidate. This publication does not establish UTXO compatibility or source approval.

No public pattern family has complete source-to-UPLC-to-real-ledger validation in this UTXO campaign. Private and proof-carrying support remain open.

The current production gap is policy, script, budget, and minimum-ADA binding in the connected transaction consumer. UNIT fixtures cannot establish production execution.

Two fresh independent Muse Wonder audits must review the same full compiler and ledger candidate before native execution. Real ledger results require a separate independent review.

The requirements cover twenty public pattern families. All families remain in the end-to-end validation scope.

## Publication contents

Readable research indexes, public reviews, experiment designs, and model documentation are published with per-file hashes in [the readable publication manifest](../../evidence/utxo/READABLE-PUBLICATION.json).

The full research and historical evidence archive is being filtered and verified for a separate commit. Its manifest will preserve original paths and content hashes.

Private keys, credentials, private model reasoning, dependency caches, and downloaded runtime binaries do not belong in the publication. Original evidence remains on the local host.

The three embedded signing-key inputs identified in the provenance report are known deterministic UNIT fixtures. They are not wallet credentials.
