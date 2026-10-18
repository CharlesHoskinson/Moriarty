# P3S1-R1 ordinary Aiken package placement

This snapshot preserves the revised copy helper, its expectations, one actual copy result, and the source and result reviews.
The helper copied 16 opaque package files totaling 1,162,873 bytes plus a receipt.
Root recorded one invocation with exit 0 on October 15, 2026, at 20:39:01Z.
Publication did not run the helper or compile Aiken.

## Evidence

- [Helper source](helper-source/candidate/place_p3s1r1_aiken_package.py) preserves the exact 5,289-byte helper.
- [Historical helper README](helper-source/candidate/README.md) retains its earlier source-only status.
- [Frozen expectations](helper-source/EXPECTATIONS.json) preserve all 16 exact package rows, including raw order and role fields.
- [Helper manifest](helper-source/SOURCE-MANIFEST.json) identifies the four source outputs.
- [Parent R1 manifest](package-source/SOURCE-MANIFEST-R1.json) retains all 98 parent rows.
- [Opaque package bodies](package-source/aiken/aiken.toml) include only its 16 `aiken/` members.
- [Actual command record](actual/PRE-EXECUTION.json), [result](actual/RESULT.json), [receipt](actual/PLACEMENT-RECEIPT.json), and [output manifest](actual/OUTPUT-MANIFEST.json) retain the root observations.
- [Meta source review](reviews/source/REPORT.md) and [source adoption](reviews/source/ROOT-SOURCE-ADOPTION.json) precede the copy.
- [Meta result review](reviews/actual-result/REPORT.md), [input manifest](reviews/actual-result/INPUT-MANIFEST.json), and [result adoption](reviews/actual-result/ROOT-RESULT-ADOPTION.json) record the later result scope.
- [Provenance](PROVENANCE.json) records full source and reviewer identities.

The result review returned `muse-wonder-1.0-contributor` from Meta.
Both reviews requested high effort. Actual effort was not independently observed.
The result reviewer read nine public inline inputs and did not independently rehash the package bodies.
Source preservation remains root-observed.

## Portable placement inputs

The copied helper still requires the original R1 manifest digest and exactly 16 `aiken/` rows.
Its preserved inherited help text names an older manifest filename.
The flag accepts the exact R1 manifest supplied here.

Use these inputs when preparing a separate ordinary copy:

| Helper argument | Snapshot-relative path |
|---|---|
| `--source-root` | `package-source` |
| `--source-manifest` | `package-source/SOURCE-MANIFEST-R1.json` |
| `--output` | A fresh absent directory under an existing stable parent, outside `package-source` |

The other 82 parent manifest members are omitted.
The helper selects only the included 16 `aiken/` members.
Each published package body equals the actual placed body and the original frozen input.
Publication checks compared content bytes only.
No published-layout helper invocation occurred.

The accepted scope assumes a trusted pinned manifest and stable filesystem ancestry.
A late I/O failure can leave partial output. The helper has no rollback and refuses output reuse.
It does not copy modes or timestamps.
The parent ABI2 package remains uncompiled and unexecuted within this evidence scope.
This copy result supplies no Aiken, native, proof, UTXO, ledger, or Preprod execution result.
Opus 5.5 owns all security judgments. The immutable reviews retain their exact scope.
Moriarty remains permissionless for public developers.
