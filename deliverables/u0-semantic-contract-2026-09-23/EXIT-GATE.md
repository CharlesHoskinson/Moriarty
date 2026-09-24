# U0 exit gate — 2026-09-23

**U0 contract recorded; capabilities open**

| Item | Evidence file(s) | Evidence RECORDED | Capability status | Counts | Open reason |
| --- | --- | --- | --- | --- | --- |
| Versioned source/Core embeddings | `deliverables/u0-semantic-contract-2026-09-23/source-core-embeddings.json`, `openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json` | yes | OPEN | 1 present, 17 partial, 66 absent | Source/Core leaf embeddings are incomplete |
| Stage/intent/effect/authority/history/failure judgments | `deliverables/u0-semantic-contract-2026-09-23/judgments.json`, `deliverables/u0-semantic-contract-2026-09-23/source-core-embeddings.json` | yes | OPEN | 6 judgments; 17 partial, 66 absent embeddings | Judgments are recorded; source/Core realization remains partial |
| Numeric profile | `deliverables/u0-semantic-contract-2026-09-23/numeric-profile.json` | yes | OPEN | 17 primitives, 6 open gaps; reserve absent | Primitive conformance or reserve posting remains open |
| K-reference reconciliation | `deliverables/u0-semantic-contract-2026-09-23/k-reconciliation.json` | yes | OPEN | 0 covered, 5 partial, 18 not covered | K reference coverage remains incomplete |
| Compiler/ZKIRv3/verifier/key/ledger pins | `deliverables/u0-semantic-contract-2026-09-23/target-pins.json` | yes | OPEN | 11 historical, 10 unresolved | Historical pins are not reverified; compatible tuple or source alignment is open |
| Field-by-field enforcement map | `deliverables/u0-semantic-contract-2026-09-23/enforcement-map.json` | yes | OPEN | 0 enforced, 84 unenforced | Native enforcement is not established for every leaf |
| Trust and unresolved interface premises | `deliverables/u0-semantic-contract-2026-09-23/trust-premises.json` | yes | OPEN | 9 premises, 6 open | Unresolved trust or interface premises remain |
| Next-backend requirement matrix | `deliverables/u0-semantic-contract-2026-09-23/backend-requirement-matrix.json` | yes | OPEN | 24 rows, 24 specified-only | Backend requirements are specified, not demonstrated |

## Checker runs

| Command | Exit code | OK line |
| --- | --- | --- |
| `python3 scripts/check_u0_stage_schema.py --root .` | 0 | OK: 84 leaf fields, 1 present, 17 partial, 66 absent |
| `python3 scripts/check_u0_numeric_profile.py --root .` | 0 | OK: 17 primitives, 6 open conformance gaps |
| `python3 scripts/check_u0_k_reconciliation.py --root .` | 0 | OK: 23 rows, 0 covered, 5 partial, 18 not-covered |
| `python3 scripts/check_u0_target_pins.py --root .` | 0 | OK: 11 historical, 0 absent, compatible tuple NOT established; absent means no pin found by this recorded search; not a proof that none exists |
| `python3 scripts/check_u0_enforcement_map.py --root .` | 0 | OK: 84 leaf fields, 0 enforced, 0 host-only, 84 NOT_ENFORCED |
| `python3 scripts/check_u0_trust_backend_matrix.py --root .` | 0 | OK: 24 backend rows specified-only, 9 trust premises |
| `python3 scripts/build_u0_backend_matrix.py --root . --check` | 0 | OK: backend matrix matches source |

## Limitations

- `deliverables/u0-semantic-contract-2026-09-23/source-core-embeddings.json`: “absence means not found by this recorded search; it is not a proof of non-realisation. Semantic adequacy of a cited declaration is a reviewed claim; the checker proves declaration, context, and profile membership only.”
- `deliverables/u0-semantic-contract-2026-09-23/k-reconciliation.json`: “Coverage is a reviewed claim; the checker proves citations and evidence quotes only.”
- `deliverables/u0-semantic-contract-2026-09-23/target-pins.json`: “absent means no pin found by this recorded search; not a proof that none exists”
- `deliverables/u0-semantic-contract-2026-09-23/enforcement-map.json`: “Whether a cited line truly enforces the field is a reviewed claim. Mechanisms cite no line. Signed-intent authentication was not found in a circuit, a bound ledger primitive, or another native boundary under the declared native roots. experiments/moriarty-language/compact is included. Its arithmetic helpers and generated loan and swap kernels constrain anonymous Uint fields of two restricted Core programs. They do not name a canonical stage-relation leaf. Copies under deliverables/preview-loan-2026-09-17 are snapshots, not native roots.”
- `deliverables/u0-semantic-contract-2026-09-23/trust-premises.json`: “Semantic fit of a quote to its statement label is a reviewed claim, not mechanically proven. The checker verifies structure, literal quote occurrence, identifier resolution, required topics, label length, one sentence without ', and' or a semicolon, and closure-word absence.”
