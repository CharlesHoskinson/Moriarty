# N1, Aiken and V13: portable captured evidence

This leaf preserves reviewed source, captured local results and their exact adoption records.
Publication adds byte checks and a source navigation map.
Publication does not add a catalogue, Aiken, native, proof or ledger execution result.

## Start here

| Content | Path | Use |
|---|---|---|
| Reviewed N1 source | [source/catalogue](source/catalogue/README.md) | Inspect the exact 14-file patched Python package. |
| Aiken package and pinned dependency sources | [source/aiken](source/aiken/README.md) | Inspect the exact 16-file package, lock and 38 stdlib files. |
| N1 adoption | [N1-C5-ADOPTION.json](captured/cardano-catalogue-v12-n1-run-20261015/N1-C5-ADOPTION.json) | Read the scoped structural decision and retained gaps. |
| N1 actual-result review | [Opus report](captured/cardano-catalogue-v12-n1-c5-independent-actual-result-review-20261015/REPORT.md) | Read the unchanged judgments and coverage limits. |
| Aiken adoption | [ADOPTION.json](captured/current-cardano-p3-r1-aiken-root-run-records-20261015/ADOPTION.json) | Read the limited local decision and PB-01–PB-12. |
| Aiken actual-result review | [Opus report](captured/current-cardano-p3-r1-aiken-independent-actual-result-review-20261015/REPORT.md) | Read the unchanged local results and target blockers. |
| Actual Aiken logs | [A1.log](captured/current-cardano-p3-r1-aiken-local-execution-20261015/logs/A1.log) | Inspect captured output, not a publication rerun. |
| Graph navigation | [graph/README.md](graph/README.md) | Separate the archival graph from the derived analysis view. |
| Byte preservation | [SEED-COPY-MANIFEST.json](preservation/SEED-COPY-MANIFEST.json) | Check all 2,698 original public bodies and their portable destinations. |
| Mechanical verification | [VERIFICATION.json](preservation/VERIFICATION.json) | Read measured checks and unrun stages. |

## Separate the evidence stages

| Stage | Status in this leaf |
|---|---|
| Source | Exact byte copies. Direct source views duplicate selected captured files. |
| Captured local results | Original N1 C0–C4/C6, C5 and Aiken H/A1/A2 records remain unchanged. |
| Publication layout | Byte checked. Catalogue and Aiken execution in this layout remain UNRUN. |
| Graphify | Local build, clustering, analysis and exports ran on already-extracted data. |
| Fresh publication audit | PENDING. Meta Muse Wonder high must review these new bytes before delivery. |
| Security judgments | Attributed only to unchanged named Opus reports. The publication author supplies none. |
| Native, proof and ledger acceptance | Not established or authorized by this publication. |

The N1 adoption is `ADOPT_SCOPED_STRUCTURAL_UNIT` for the whole-sibling schema guard.
The Aiken adoption covers local CEK library agreement and compiled identity records only.
No historical source-only report has been relabeled as a current runtime pass.
The [report](REPORT.md) gives the retained qualifications and publication limits.

## Portable consumption

Read files directly after clone. No archive extraction is required.
Use [source/PORTABLE-LAYOUT.json](source/PORTABLE-LAYOUT.json) to compare direct source views with captured bodies.
Use [graph/SOURCE-PATH-MAP.json](graph/SOURCE-PATH-MAP.json) to locate each V13 citation after clone.
Original absolute paths inside opaque records remain unchanged.
Those historical paths are evidence references, not portable command arguments.

Run the read-only byte verifier from this leaf:

```bash
python3 tools/verify_publication.py
```

This verifier reads source and JSON as data. It does not import catalogue or Aiken code.
The verifier does not execute historical operators.
Read each operator and its authority record before any separately authorized reproduction.
Captured operator paths can contain authority-sensitive commands and once-only placement checks.

The existing [R3 Rust snapshot](../current-core-groth16-r3-2026-10-15/README.md) remains a separate frozen closure.
The branch root has no live Rust crate delivery from this publication.
The existing [Aiken placement evidence](../p3-r1-package-placement-2026-10-15/README.md) remains unchanged.
All 16 package bodies here equal that published package.

Moriarty remains permissionless for public developers.
Maintainer reviews and campaign records never become public compile, prove or deploy admission.
