# R115 populated independent two-program expectations

`EXPECTED.json` is directly shaped as R112 TwoProgramExpectations. Both records contain source_digest, full source, full preparation and all twelve ProgramView fields. There are no wildcards, omitted arrays or selected static field gaps under the pinned proposed schema. See DERIVATION.md and TYPE-AND-JOIN-ACCOUNTING.json for every row/path/span/type premise.

Only inspected stdlib literal/JSON/hash checks have run. No source parser/checker/bridge/owner/compiler/model/native execution or actual artifact/output was used. Source/preprocessing integration and admitted execution remain Root-owned separate work.

For a later independently captured artifact under this exact schema:

```sh
python3 compare_expected.py artifact.json --case chain-total
python3 compare_expected.py artifact.json --case branch-total
```

Exit0 means the complete R112 selected interface matches; exit1 means a selected mismatch; exit2 is input/pin failure. It does not compare other admission fields, config/target/genesis/plans or runtime/native results. The existing R112 Rust comparator can also deserialize this same EXPECTED.json; that Rust path was not built or run in this lane.44 synthetic normal golden-copy/mutation checks pass.
