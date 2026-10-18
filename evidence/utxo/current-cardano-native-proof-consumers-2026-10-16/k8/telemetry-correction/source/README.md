# K8 captured-telemetry validator (ordinary parser correction)

## What this is

`validate_capture.py` closes the remaining K8 record-metadata gap using only
already-captured public data. Two earlier source audits expected
`succeed_immediately` for the plain-`fail` test `k8_short_vk_is_refused`;
the Aiken parser actually maps plain `fail` to `succeed_eventually` and
`fail once` to `succeed_immediately` (a missing annotation defaults to
`fail_immediately`). The captured telemetry was right; the expectation was
wrong. This script encodes the parser-correct mapping and strict typed
checks over the captured JSON. It is a small (~190-line) stdlib-only checker,
not a new framework, authority, campaign, or self-test house.

## What it is not

It generates no proof, reruns no native/Aiken test, imports no operator,
SDK or native code, and uses no subprocess, socket, RNG or key material.
It does not overwrite or relabel the historical record: the original
wrapper status stays FAILED. Genuine-proof acceptance is the previously
recorded scoped adoption (root record 99541... adopting the 85e... review:
1 genuine True plus N1/N2/N3 False, 9 normal rows, the malformed-length
unit classified separately); this validator acknowledges that predicate
and recomputes none of it. Execution units remain Aiken VM measurements,
not qualified ledger ExUnits. Ledger, Preprod, the C2 compiled wrapper and
whole-language coverage remain open, as do all 20 UTxO families, ACTUS/DeFi
and general obligations, recursion, private handoff, bounded multi-parent
PCD and universal certificates. No compatibility percentage is claimed.

## Inputs (all pinned, bounded, read before parsing)

| Input | Bytes | SHA-256 |
|---|---|---|
| staged/lib/pvc2/k8_genuine_proof_test.ak | 4782 | d25c3811132ee99dab4bba9862ec87147931c80cd516e6eae5940e2088551eec |
| run/logs/step2-k8-tests.stdout.log | 2914 | 9ac4871e8923873843d607b42648f31987834ac1fdbf79cf5b7607898a015800 |
| run/receipts/RESULT.json | 4875 | 5bc3033c0274cb50f2d3e473c74974a547bd4a896c5e7cd1d4e82c47b45ece8f |

Each input is read at most 65536 bytes, then checked for exact pinned size
and SHA-256 before any parsing. The original result is additionally checked
as status FAILED with its original single error wording unchanged; its
giant native-run guard schema is deliberately not duplicated here.

## Output

Sole output: `VALIDATION.json` inside a fresh, previously absent
`--output-dir`, created exclusively (no overwrite of old files). It records
the three input pins, the 10 rows with expected and actual modes, and the
flags ordinaryValidationOnly=true, nativeReexecuted=false,
newNativeAuthority=false, originalWrapperStatus=FAILED,
originalWrapperStatusChanged=false, rawCapturedPassedUnits=10,
normalRows=9, pairingOutcomesCount=4. On any validation failure the script
writes to stderr, exits nonzero, and creates no output directory.

## Authority

No execution is authorised by this README. The one bounded ordinary run in
EXECUTION-PROPOSAL.json requires a fresh non-author review and AGREE on the
materialised bytes first. The separately authored K9 native 7-case operator
and its pending source audit are out of scope here.
