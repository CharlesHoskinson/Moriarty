# Historical UTXO research and validation archive

This package preserves the selected public bytes from the original October 12–14 research and validation capture. It contains historical successes, failures, rejected reviews and provenance. It is not the current Rust/Groth16 prototype, a current language acceptance, native Cardano acceptance or a private PCD result. See [HISTORICAL-SCOPE.md](HISTORICAL-SCOPE.md).

The main generation records 77,016 original paths representing 13,009,194,205 bytes, deduplicated into 35,490 gzip objects and 17 chunks totaling 787,898,112 bytes. Its strict public-text extraction ledger has 9,854 records, of which 216 have published text. These figures describe the main generation only.

The public-source restoration supplement preserves another 7,035 exact original paths, 79,662,736 bytes, in 5,510 objects and one 14,188,916-byte chunk. Its independent acquisition profile and root verification receipt pin public Git commit blobs and two official documents. The main omission ledger remains unchanged. [The cross-index](provenance/RESTORATION-CROSS-INDEX.json) links every restored path and digest to its main omission reason and acquisition proof.

| Generation | Location | Purpose |
|---|---|---|
| Main | This directory | Frozen historical research, project sources and validation evidence; repeated contents deduplicated |
| Public sources | [supplements/public-source-restoration](supplements/public-source-restoration/) | Exact proven public source and document bytes conservatively withheld by the main classifier |
| Helper review history | [supplements/helper-repair-history](supplements/helper-repair-history/) | Original failed reviews, reproduced defects and repaired helper evidence |
| Review 426 | [supplements/review-426](supplements/review-426/) | Selected frozen public review packet |
| Post-capture public selection | [supplements/post-capture-public](supplements/post-capture-public/) | Explicitly selected public late artifacts, with raw provider captures withheld |
| Omission metadata repair | [supplements/omission-metadata-repair](supplements/omission-metadata-repair/) | Original reconstruction failure, reviewed strict unknown-digest metadata repair, tests and adoption provenance |
| Depth repair | [supplements/depth-repair-history](supplements/depth-repair-history/) | Actual failed depth traversal and independently reviewed bounded refusal repair evidence |

Each generation owns its manifests, chunks and reconstructor. See [RECONSTRUCTION.md](RECONSTRUCTION.md) for exact commands and the historical canonical graph digest. No dependency installation or model access is needed to reconstruct the published bytes.

`paths.jsonl` maps original roots/paths to hashes and sizes. `objects.jsonl` maps hashes to gzip ranges in chunks. `exclusions.jsonl` records omissions rather than promising withheld bytes are present. `public-log-extractions.jsonl` records the exact source identity and extraction status where known. An uninventoried, payload-free late omission has an explicitly unknown source digest and an auditable metadata adoption receipt; it is not a reconstructible object. Original filter/finalizer attestations retain their original scope and hashes.

Original raw model/provider streams, private reasoning, signing keys, wallet stores, authentication material, runtime/dependency payloads and caches are withheld. Public-source restoration is exact provenance-bound recovery, not a general exemption for captures or key material. Conservative content labels also catch some legitimate public syntax and test fixtures; the restoration cross-index and separately classified exact public test/review sources document those dispositions. Rejected reviews and original failures retain their actual verdicts and identities; later scoped helper approval is not approval of earlier bytes or the corpus.

Approachable readable research, reports, designs and model documentation are published separately under `research/utxo/` and `evidence/utxo/{reports,designs,model-docs}/`. Current source integration and prototype reviews belong to separate publications and are not silently included here.
