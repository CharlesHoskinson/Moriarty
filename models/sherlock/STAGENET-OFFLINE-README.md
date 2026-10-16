# Offline stagenet input inventory and JSON readback comparison

`stagenet/offline.py` is a standalone Python standard-library tool. `prepare`
inventories existing Core/1 Artifact/3, Target, Scenario and frozen Readback
JSON. `compare` compares all fields of two supplied JSON documents. These
operations do not compile source, construct a witness, verify a proof, deploy
a contract or establish a network identity. The requested target label is
Midnight stagenet; an input target labelled local remains local.

Freeze the expected readback bytes independently and supply their SHA-256. A
digest identifies bytes; it does not establish that their expected semantics
are correct. For example, from the repository root:

```sh
python3 -B models/sherlock/stagenet/offline.py compare \
  --expected models/sherlock/stagenet/fixtures/pure-expected.json \
  --expected-sha256 6370ced2898896a914ee8191c44f2937b98bb9f232cb7c15a5a8569aa9cb0540 \
  --observed models/sherlock/stagenet/fixtures/pure-observed-eight.json
```

This example is intended to return exit 1 and report
`/execution/effects/0/value/value`, with expected `7` and observed `8`. Exit 0
means the supplied documents compare equal; the output still states
`acceptance: none`. Invalid input exits 2 through the handled input-error path;
malformed shapes outside that path can raise a traceback. The CLI example itself
was not executed in the captured normal suite.

Object keys are compared in sorted order and arrays retain their order. The
reported first difference is the first in that traversal, rather than the
earliest execution event. All supplied fields participate, including extensions,
preserved state and liabilities. Missing members remain distinct from present
null or zero. Container shape checks do not validate native hex, evidence claims
or state-transition semantics.

`prepare` accepts required `--artifact`, `--target`, `--scenario`, `--action`,
`--application-scope`, `--expected` and `--expected-sha256` arguments. The scope
is either `pure/no-financial-effects` or `financial`. Optional `--source`
inventories source bytes, while `--external-configuration` retains only that
JSON file's path, digest and size. Preparation retains missing target fields,
artifact/target differences and all adapter/accounting gaps. It does not fill
missing identities from an artifact or treat JSON data as a native call.

The [captured normal result](../../docs/audits/2026-10-15/sherlock-offline-readback/README.md)
records 16 passing assertions on the exact bound source. It covers ordinary
comparison and historical input retention. Hostile inputs, the pure preparation
branch, CLI behavior and the R2 configuration/acceptance corrections lack test
coverage in that capture. The pure fixtures use conspicuous fixture-only native
strings. The historical payment input remains prepared readback, with an
unchanged supplied ledger and no applied successor.

Native export, witness/ABI mapping, proof/key identity, actual verification,
deployment/call and complete observed readback on Midnight stagenet remain
required separate work. Gross debit, fees, net receipts and residual duties need
an explicit accounting adapter; this JSON tool supplies no financial acceptance.
It does not implement public `mori simulate`, `mori test` or `verify-intent`.
