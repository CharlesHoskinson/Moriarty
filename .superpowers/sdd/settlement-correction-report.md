# S01 settlement terminology correction report

## Status

DONE

Base revision: `ee27ee8`

## Correction

The XML v1.3 term at lines 147–149 defines `settlement` as the process that
turns fills and proofs into final, spendable outcomes or an authorized recovery
path. The previous registry assigned that alias to `SettlementReceipt`, which
is evidence that a settlement step produced a recorded state.

The corrected registry:

- keeps W4 `TERM-W4-025` `SettlementReceipt` in the `evidence` category and
  gives it no source alias;
- adds `TERM-S01-043` `SettlementProcess` in the `process` category;
- assigns the XML alias `settlement` only to `SettlementProcess`;
- excludes the process itself from the receipt's meaning and excludes receipt
  evidence from the process's meaning.

Gate 01 now freezes 70 exact nouns and IDs and rejects a category inversion or
an alias transfer between `SettlementProcess` and `SettlementReceipt`.

## Hash transition

Terminology artifact and independently embedded validator pin:

```text
old: b4572303254f932aba311ddc324cf774ad2501b01fe054a309cc75eb6ea935c3
new: c16997ab1ab6445ea5fb3e76e910b621110bb025537e93c29ea10d5795354c3b
```

Validator executable:

```text
old: de45186c892878fc91dd1f345a1aa46783eddd0f4e51ce866d467bf5c2d3d478
new: 2aaf4881f7334801bb513bc3203c54a495dadaf60bafaa52703560dbaf4448a4
```

Resealed manifest:

```text
old file SHA-256: e58efb60c4c8e85d9ff2de878bc0b9628c08b79430a5aa00a0ce397618d507bd
new file SHA-256: b1a5ac79236fd4fc95a689e3e78e32ad0dfab9a7db3c84a54919e63749adf8ef
old manifest self-hash: fc4e73bdae57a52eccc1d9cd3b5f9dfb439f95b25b1fad46a26da6e1fc12b168
new manifest self-hash: ea183aeeeaaf0a7688fe4e9399c1f3cabc9fa8ae7a5ecf38adc73a70bdb7103e
```

The explicit writer also regenerated `validation-report.json`. Its bytes remain
unchanged at
`c29abe17d9004dd06ab3efc8ce4893477b0ef52c82ebeeb13a6d31b8ace5b88a`
because all ten recomputed gate results and the evidence boundary remain the
same.

## TDD evidence

The semantic regression test failed before the registry correction:

```text
FAILED tests/test_s01_registries.py::test_settlement_process_is_distinct_from_receipt_evidence
AssertionError: assert 'SettlementReceipt' == 'SettlementProcess'
1 failed in 0.05s
```

After the registry correction, the focused semantic test passed. Three
validator mutation cases then failed with the pre-fix fallback error
`S01 in-memory artifact override differs from on-disk evidence`, proving Gate
01 did not detect the semantic corruption. After the Gate 01 correction:

```text
tests/test_s01_intent_evidence.py::test_gate_01_enforces_settlement_process_receipt_semantics
tests/test_s01_registries.py::test_settlement_process_is_distinct_from_receipt_evidence
4 passed in 0.19s
```

## Evidence-write order

Before writing receipts, the semantic gate was computed with:

```text
/home/charl/Moriarty/.venv/bin/python -c 'import json; from scripts.validate_s01_intent_evidence import _recompute_semantic_gate; print(json.dumps(_recompute_semantic_gate(), indent=2, sort_keys=True))'
```

Result: `S01-01` through `S01-10` were true and status was
`recomputed-package-gate-passed`.

Only then was the explicit writer invoked:

```text
/home/charl/Moriarty/.venv/bin/python scripts/validate_s01_intent_evidence.py --write-evidence
```

The default read-only CLI was run immediately after the write and again after
the final test suites:

```text
/home/charl/Moriarty/.venv/bin/python scripts/validate_s01_intent_evidence.py
```

Each run reported all ten gates true and
`recomputed-package-gate-passed`.

## Final verification

Focused S01 suite:

```text
/home/charl/Moriarty/.venv/bin/python -m pytest -q tests/test_s01_openspec.py tests/test_s01_registries.py tests/test_intent_verifier.py tests/test_s01_intent_evidence.py
204 passed in 8.51s
```

Full repository suite:

```text
/home/charl/Moriarty/.venv/bin/python -m pytest -q
282 passed in 9.68s
```

The atomic-swap vector remains unchanged. Both local expected certificates
still set `signing_request_permitted` to `false`.

## Scope and schema

No schema change was required. The existing strict terminology record schema
already supports the new record, and the exact semantic relationship belongs
in the independently pinned Gate 01 checks.

No frozen XML, approved design, audit supplement, semantic scope, Core, swap,
backend, E00, OpenSpec normative output, source inventory, wiki, archive,
execution document, review ledger, or checkpoint file was changed. Independent
approval is not claimed by these local checks; the root reviewer owns the final
re-review.

## Changed tracked paths

- `.superpowers/sdd/settlement-correction-report.md` (force-added from the
  ignored SDD report directory)
- `evidence/s01-intent-theorem-freeze/evidence-manifest.json`
- `evidence/s01-intent-theorem-freeze/terminology.json`
- `scripts/validate_s01_intent_evidence.py`
- `tests/test_s01_intent_evidence.py`
- `tests/test_s01_registries.py`
