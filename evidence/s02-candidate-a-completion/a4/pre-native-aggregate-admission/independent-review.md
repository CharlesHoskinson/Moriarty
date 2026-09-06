# A4 preliminary aggregate compact package: independent review

Spec verdict: PASS. Evidence-preservation verdict: PASS.
The package faithfully preserves root's admitted amended preliminary114 gate,
its exact original receipt bindings and stated limits. It does not establish
A4 completion, actual native package acceptance, final115, Council or H1.

Package:
`evidence/s02-candidate-a-completion/a4/pre-native-aggregate-admission/`.
All 11 package files were inspected or hashed; no package original was modified.

## Source inspection and actual standalone audit

Read audit.py, retained_checks.py, build_archive.py, README and package/index
metadata before execution. The audit has an explicit non-assert optimization
guard. Its only non-standard local import is the reviewed retained_checks.py,
whose bytes are verified through packageSupport before import. The helper reads
tar members into memory, rejects unsafe/nonregular/duplicate/oversized members,
and uses archived Python only as AST/literal data. Neither audit source imports
or executes archived recorders, helpers, tests, prior audit scripts or semantic
code. It does not read external generated trees or the external runtime archive.
build_archive.py was reviewed as source only and was not run by this reviewer.

Executed the standalone audit from /tmp with Python optimization explicitly
disabled and bytecode writes disabled. Actual tool chunk5a7495 exited0. The
complete original invocation and actual returned tool object are preserved
below. Its stdout is byte-identical to the package's audit-report.json, SHA256
`f701b464c2d59c39d4e729575a11bf64d79f5b5316380814edadac10889bdd81`.
This independent audit does not replace the package's earlier actual audit
receipt: that retains chunka2e154/exit0 and has SHA256
`aa0288a1e90eed9c94a5778c0b04f2d68cf7b80c55c55801ac1229674b64e621`.
The earlier optimized-invocation rejection remains separately retained; this
review did not rerun that already recorded negative audit control.

## Original membership and pin closure

- The archive has exactly 141 unique regular original members, 37,270,487
  uncompressed original bytes, and compressed size 3,636,702 bytes. All member
  paths, sizes and hashes agree with index.json.
- All 21 original fresh top-level files are included. The complete original
  generated-file manifest selects exactly 100 regular basetemp files at most
  2,000,000 bytes each, totaling 32,550,779 bytes: 64 child-control records and
  36 small fixtures. No other basetemp file substitutes for that exact set.
- The prior compact retained17 package is nested losslessly. Its 153 original
  members, 59-member admission source archive, 46-member original source
  archive, retained control records, prior source/decision bindings and limits
  validate. Including its build_archive.py closes the existing prior index's
  package-support pins; that file is retained as data and never executed.
- The fresh source/support archive has 59 exact members. All original source,
  support, root decision and per-node small pins are present in the combined
  archive closure. The aggregate references exactly 135 distinct small pins.
- An additional independent read/hash check compared every one of the 141
  archived small members byte-for-byte with its still-retained live original
  and the archive index. It completed in chunk5484b9, exit0. No large external
  generated file was read by this check.

## Exact admitted scope and preserved gaps

The audit verifies the ordered 97 original JUnit nodes, complete original
collection/pytest/outer argv and terminal bindings, zero fresh parent exits,
all 16 fresh subprocess control identities/reports/exits, and their authentic
archived worker string. It verifies the 17 retained controls and every row of
the exact disjoint 17+97=114 aggregate against its own retained or fresh receipt.
Counts are accompanied by identities and per-node pin bindings.

Root-admission114, the independent original intake and their exact hashes are
preserved. The original interrupted parent's exit remains null. Its partial
eighteenth child receives no retained credit, while the whole required fresh
control remains in the admitted complement. Historical assertion execution
retains the accepted source-bound inference and its missing-flag limitations.
Endpoint hashes do not establish continuous historical immutability.

The outer evidence preserves 87 actually returned responses and the explicit
monitor-cell50 polling-metadata gap. It does not claim a complete polling
history or reconstruct missing responses. The original independent intake's
first audit-schema mistake and corrected PASS remain in the archived review;
no standalone historical audit script or missing tool response is fabricated.

The authoritative manifests retain the fresh 274-file/6,237,424,864-byte tree
and retained 318-file/9,795,796,111-byte tree, with their directory and symlink
inventories. Large files and the admitted 3,329-file Python archive remain
external, hash-bound originals. This review relies on their already completed
admission and manifest bindings; no mismatch required another 16-GB tree rehash.
No runtime archive was duplicated or rehashed by this package review.

Actual78-case/1,557-event native package/admission, remaining required controls,
final115, direct native CLI, shared regressions and remaining independent review
are still open. The package's source and output consistently retain these limits.

## Package digest inventory

<!-- package-digests -->

| Package file | Bytes | SHA256 |
| --- | ---: | --- |
| README.md | 3412 | `51f7e3b79582c0897e221268d7ab0691286febb26e5992a933d6b09b9d5f4c17` |
| audit-report.json | 833 | `f701b464c2d59c39d4e729575a11bf64d79f5b5316380814edadac10889bdd81` |
| audit-tool-receipt.json | 1284 | `aa0288a1e90eed9c94a5778c0b04f2d68cf7b80c55c55801ac1229674b64e621` |
| audit.py | 20621 | `b7bc57895b42f1f164945371a28a09d8b6d63c2df6c0bfc23fa501df1a4beba3` |
| build_archive.py | 5921 | `5363c274ac57c6017ef1011aaf32b4f43ac275d4ea9671b22aa5a2015faa5c97` |
| index.json | 72722 | `e4f688c816d1074eef2f52bdd29bf56fae2ede3fe0b6a1f54d5fa1970bd45bef` |
| optimization-rejection-tool-receipt.json | 717 | `978da770511307c44e4cbe41bf1c9393f52e60abc636a3a07108984084640c27` |
| original-small-evidence.tar.gz | 3636702 | `20f9932bfd8c164e5f9de007452c855ae2f4af9c9e9aaf4380f493bb1224cacc` |
| packaging-report.json | 158 | `d516d79b7189385d6b8da539726bed97dfd8a038629892f8864fb59c7a6f2a2d` |
| packaging-tool-receipt.json | 549 | `31da002fa1b71a837f76ad0e623e9ee680c2c7d6db025cf5e25966d0e89c3f8f` |
| retained_checks.py | 17433 | `049f18599cfa19204e87dd49b62da26ff49486cf22c9fcab84a3c38b370bc8a9` |

## This review's authentic audit tool invocation and result

<!-- audit-tool-receipt -->

```json
{
  "args": {
    "cmd": "PYTHONOPTIMIZE=0 python3 -B /home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-candidate-a-completion/a4/pre-native-aggregate-admission/audit.py",
    "workdir": "/tmp",
    "max_output_tokens": 2200
  },
  "result": {
    "chunk_id": "5a7495",
    "wall_time_seconds": 0.085577379,
    "exit_code": 0,
    "original_token_count": 209,
    "output": "{\n  \"a4Complete\": false,\n  \"aggregateDistinctSmallPins\": 135,\n  \"aggregateNodes\": 114,\n  \"archiveMembers\": 141,\n  \"archiveSha256\": \"20f9932bfd8c164e5f9de007452c855ae2f4af9c9e9aaf4380f493bb1224cacc\",\n  \"archivedCodeExecuted\": false,\n  \"freshChildControls\": 16,\n  \"freshJUnitInstances\": 97,\n  \"freshSourceArchiveMembers\": 59,\n  \"largeRawTreesRehashed\": false,\n  \"nativeFinal115StillRequired\": true,\n  \"nestedRetainedArchiveMembers\": 153,\n  \"ok\": true,\n  \"optimization\": 0,\n  \"originalParentExit\": null,\n  \"pollingMetadataGapPreserved\": true,\n  \"preservedOuterResponses\": 87,\n  \"retainedControls\": 17,\n  \"runtimeArchiveRehashed\": false,\n  \"scope\": \"root-admitted amended preliminary114 only\",\n  \"selectedSmallOriginalBytes\": 32550779,\n  \"selectedSmallOriginals\": 100,\n  \"testsExecuted\": false,\n  \"uncompressedOriginalBytes\": 37270487\n}\n"
  }
}
```

Only this new review file was written. No source/runtime/original receipt edit,
test/native/semantic replay, archived-code execution or commit occurred.
