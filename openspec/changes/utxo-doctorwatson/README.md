# `doctorwatson<UTXO>`

Use `./scripts/doctorwatson-utxo.sh` for local requirement integrity and reporting regressions. Use `--requirement EUC-012` to inspect a clause, `--gate EUC-012` to test its declared closure, and `--next` to list the next consumers and blockers. A valid inventory can contain only open requirements.

[Current evidence](current-evidence.json) preserves three separate inventories: 119 UTXO clauses with 238 scenarios, 11 first-contract clauses with 16 scenarios, and 18 retained Midnight PCD clauses with 38 scenarios. Their original normative files remain authoritative. The [field schema](evidence.schema.json) describes record structure; the reader additionally checks actual source membership, byte pins and declared dependencies. Neither check authenticates semantic acceptance.

[Reporting requirements](requirements.md) apply to this tool. The [route reconciliation](route-reconciliation.md) proposes additive ordinary requirements for current Rust artifacts and fixed-verifier consumers. It does not replace the original generated-Aiken or generated-spending requirements.

The [design and assessment](../../../docs/utxo/doctorwatson/DESIGN.md), [all-EARS source manifest](../../../docs/utxo/doctorwatson/audit/SOURCE-MANIFEST.json) and post-build audit retain broader language, financial, formal and release obligations. The broader prototype goal remains paused. This entrypoint invokes no compiler, prover, wallet or ledger consumer.
