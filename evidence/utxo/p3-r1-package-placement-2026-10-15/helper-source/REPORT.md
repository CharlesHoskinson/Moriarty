# P3S1-R1 Aiken package placement source report

Status: SOURCE CANDIDATE WRITTEN; STATIC COMPARISONS OBSERVED; HELPER AND PLACEMENT UNRUN; FRESH MUSE REVIEW PENDING.

This is the fully specified nonsecurity successor of the reviewed P3S1 package byte-copy helper. The falsifiable source predicate is that the new helper equals the old helper with only the supplied five-line R1 patch, and that its `read_package` consumer selects exactly the 16 manifest rows recorded in `EXPECTATIONS.json`. No runtime pass or source approval is claimed.

Requested author routing is `gpt-6.1-sol`, high effort. Observed interface is this delegated Codex implementation session. The parent assigned that routing; this worker has no independently authenticated provider/model/effort receipt and makes no self-authenticated identity or effort claim. Root must obtain a fresh actual Meta Muse Wonder (`muse-wonder-1.0`) high independent nonsecurity audit for this exact candidate. This worker did not invoke another model or agent. All security work and judgments belong to Opus 5.5; this report makes no security judgment.

## Inputs and boundaries

I read the installed `moriarty-dev:develop` skill and its execution-focus reference, checkout `AGENTS.md`, product contract and security routing, the entire old helper, old source manifest and old report, the R1 patch note, exact diff, proposed full helper, R1 manifest and the Opus-authored eligibility boundaries. Root reported guarded status refreshed for this task; no repository helper or status CLI was run by this worker. Historical SP01.6 did not select this work.

Old helper path: `/home/charl/moriarty-eutxo-validation-2026-10-12/sol-p3-aiken-package-placement-source-20261015/candidate/place_p3_aiken_package.py`. Its old source manifest declares 5284 bytes and SHA256 `3e961fdd8ca60de70b123d47c2d30089e06564c0a5921fbd7992cb96779b1819`. These are preserved input metadata, not a fresh hash by this worker. Prior review and exercise of the 15-file package are parent-provided historical context; they do not establish R1 execution.

R1 manifest path: `/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-p3-review-repair-source-20261015/SOURCE-MANIFEST-R1.json`. Supplied frozen SHA256: `a54de5695c7bc29bab50376a92506a3247127a0c1b1341b7c0067e1fac10a957`. Observed length: 19836 bytes. Root reported verification of all 98 candidate rows and all 218 packet outputs; the worker did not repeat those input hashes. The manifest's own candidate label remains `P3S1-R1 successor (PROPOSED, unreviewed, uncompiled, unexecuted)` in the preserved raw metadata. No raw metadata was rewritten to reconcile later review status.

All writes are confined to `/home/charl/moriarty-eutxo-validation-2026-10-12/sol-p3-r1-aiken-package-placement-source-20261015`. No old source/output or shared dirty checkout was changed. No wrapper, private/provider file, PROCESS/PROMPT/FINAL artifact or credential was read or changed, and no Git operation was performed. Public source and checkout guidance reads are listed above. No folder enumeration, network/Git, install, helper/candidate/test/native/Aiken/Rust execution, key/proof/wallet/ledger operation or nested delegation was performed. Only self-authored standard-library data read/copy/diff/hash scripts were run. Hashing is restricted to this task's explicit public output allowlist. The checkpoint skill was read for handoff, but its helper and database mutations were not run because the explicit task authorizes only this folder and prohibits helper execution; the requested source report and manifest form the scoped handoff.

## Exact implementation and static evidence

`candidate/place_p3s1r1_aiken_package.py` is 5289 bytes, SHA256 `f65a6372dc87b6045f357d87f7188d4b681f329c8aa199c2ebe080cf37eb2272` (computed on the new own output). Its bytes equal the supplied `.proposed` text exactly. Replacing exactly one occurrence of each of the five authorized old lines reproduces it. A standard-library unified byte/text diff with the proposal's headers equals the supplied `PLACEMENT-R1-PATCH.diff` exactly:

```diff
--- sol-p3-aiken-package-placement-source-20261015/candidate/place_p3_aiken_package.py
+++ proposed/place_p3s1r1_aiken_package.py
@@ -1,5 +1,5 @@
 #!/usr/bin/env python3
-"""Place only the frozen P3S1 Aiken package; never execute package code."""
+"""Place only the frozen P3S1-R1 Aiken package; never execute package code."""
 
 import argparse
 import hashlib
@@ -8,10 +8,10 @@
 import sys
 
 
-MANIFEST_SHA256 = "5c5dc86d04e0ed0ba93e35d5fa45fd0a256f8f2492f5667f7d3883e8a7055d7f"
-MANIFEST_FORMAT = "p3s1-source-manifest/1"
+MANIFEST_SHA256 = "a54de5695c7bc29bab50376a92506a3247127a0c1b1341b7c0067e1fac10a957"
+MANIFEST_FORMAT = "p3s1r1-source-manifest/1"
 PACKAGE_PREFIX = "aiken/"
-PACKAGE_FILE_COUNT = 15
+PACKAGE_FILE_COUNT = 16
 RECEIPT_NAME = "PLACEMENT-RECEIPT.json"
 
 
@@ -70,7 +70,7 @@
     output = require_fresh_output(output, source_root)
     manifest_bytes, package = read_package(source_root, manifest_path)
 
-    # All 15 inputs have been checked and retained in memory at this point.
+    # All 16 inputs have been checked and retained in memory at this point.
     # mkdir refuses an output created since the freshness check as well.
     output.mkdir()
     copied = []
```

No other helper byte changed. This preserves the `aiken/` selector and prefix stripping, all CLI flags and parsing, every path guard and read/write effect, all errors and refusals, exclusive package/receipt creation, stdout summary, and the receipt schema/name `p3-aiken-placement-receipt/1` / `PLACEMENT-RECEIPT.json`. The inherited manifest help wording names the old manifest filename; it is deliberately unchanged and does not prevent supplying the R1 path. No unrelated behavior was repaired.

Actual consumer inspection: `read_package` reads manifest bytes, checks its fixed digest, parses `manifest['format']` and `manifest['files']`, selects `path.startswith('aiken/')`, requires the fixed member count, then reads `path`, `bytes` and `sha256` from each selected row. The R1 format matches the new constant. All 98 observed rows have `path` and `sha256` strings and integer `bytes`, plus integer `order` and string `role`; the extra metadata is not consumed by this helper. Exactly 16 unique selected rows total 1162873 bytes. They are one `aiken.toml`, 13 `lib/pvc2/*.ak` files and two validators. `EXPECTATIONS.json` retains the original selected row objects and all source metadata outside `files`, and separately records the exact prefix-stripped destination rows in source order. No interface mismatch was observed by this schema comparison. Package contents were not rehashed; root's verified packet is the input identity evidence.

The only helper change to ordinary execution effects is the frozen package selected by the pin/format/count. All selected bytes would be checked and retained before output creation; each member would be exclusively created and read back; the unchanged receipt would be written last. The output parent must exist, the output leaf must be absent and the output must be outside the resolved input root. Path locations and ancestry must remain stable. Copies do not preserve filesystem metadata. A later I/O failure may leave partial output; no rollback or reuse is provided. These are source-inspected operational limits, not execution observations or security conclusions.

## Frozen future command and expected outputs

The proposed command in the README and `EXPECTATIONS.json` is **UNRUN, root authority only**, after fresh full-candidate Muse source review:

```bash
python3 /home/charl/moriarty-eutxo-validation-2026-10-12/sol-p3-r1-aiken-package-placement-source-20261015/candidate/place_p3s1r1_aiken_package.py --source-root /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-p3-review-repair-source-20261015/candidate --source-manifest /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-p3-review-repair-source-20261015/SOURCE-MANIFEST-R1.json --output /home/charl/moriarty-eutxo-validation-2026-10-12/sol-p3-r1-aiken-package-placement-source-20261015/placed-package-after-muse-root-only
```

The future output path is recorded but not created. Expected membership is the 16 declared byte-identical placed files plus only the added receipt. Expected package count is 16 and expected package byte total is 1162873; receipt bytes are outside that total. A future root operator must independently compare actual file membership, lengths/digests, input identities and receipt fields against the frozen expectations and record the actual command, exit and outputs. No output directory, placement receipt, helper import, Python syntax compilation, `--help`, test or unit suite was produced or run here. No constant-mirroring tests were added.

Opus's `ELIGIBILITY-BOUNDARIES.md` says A-1 also needs the separately reviewed 38-file stdlib placement and exact isolated command, among its other prerequisites. Those preparations, actual A-1 and actual-result review are outside this task. The packet's qualified source-review status establishes no runtime or security result. Root owns promotion, actual placement, fresh Muse source/result audits and eventual publication.

## Deliverables and acceptance limits

The explicit public output manifest allowlist is `candidate/place_p3s1r1_aiken_package.py`, `candidate/README.md`, `EXPECTATIONS.json`, and `REPORT.md`. `SOURCE-MANIFEST.json` records each output's own observed byte length and SHA256. It excludes itself and `SOURCE-MANIFEST.sha256`; the latter pins the manifest without a self-hash cycle. No provider output, raw wrapper, task private file or unlisted path is hashed or included. No raw capture or source metadata was altered.

Implementation is the exact requested source candidate. Evidence consists of source inspection, byte equality with the full proposal, exact diff equality, schema/member comparisons and own-output manifest checks. Acceptance remains pending the fresh non-author source audit; ordinary placement and result review remain unobserved. No source, runtime, security, native or target acceptance is asserted. Current native/SDK authority remains zero. This changes no financial/proof duty, all20 family, recursion/private/multi-parent obligation, resource counter, public developer permission, design/resource vote or complete U0-U7, M1-M17, MC/SP/G exit.
