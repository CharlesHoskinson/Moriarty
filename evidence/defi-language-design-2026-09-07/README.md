# DeFi and language-design research evidence

This directory records the research dossier and `.mori` extension migration. It does not establish a successor parser, K definition, usability result, native proof or financial settlement.

The source map and cards retain consulted URLs, section locators, dates, source IDs, payload hashes where available, and access/coverage limits. New raw capsules are under `.raw/captured/defi-language-design-2026-09-07/`. They contain researcher paraphrases and provenance, not the complete downloaded papers. Original payloads and extracts remain in the ignored local inbox. The ISO record is a metadata-only web inspection with no original HTML hash; Marlowe reuses the existing PDF and receipt. Source cards must not be described as independently certified paper claims. PDF acquisition receipts retain their original pre-extraction status; `inspection-record.json` records the later text extraction and observed text digests. Capsule-file digest checks verify the portable capsule, not an unavailable original payload.

Scrapling 0.4.15 fetched the new public source payloads. The first extraction attempt used an unavailable `css_first` method; a bounded retry used `css(...)[0]`. Both attempt receipts remain in the local inbox. Existing static arXiv/PDF site patterns applied; no new credential or cookie storage was needed. `acquire.py` accepts a JSON list of source objects with `id` and `url`, and stores local receipts. All acquisition was public and unauthenticated.

Three parallel research agents covered DeFi taxonomies, empirical syntax use and financial/resource semantics. The coordinator reconciled findings, acquired durable provenance and inspected critical distinctions. No existing ACTUS fixture or DeFi target row was removed.

`extension-migration.json` records the old/new example paths and source digests. The source bytes and exact registered bounds are unchanged. Future materialization output names use `.mori`; historical output names remain recoverable from their recorded commits. Run materialization into a fresh directory when comparing with retained evidence.

Verification:

- `language-tests.txt`: existing language suite, including frontend, evaluator and lowering tests.
- `language-build.txt` and `demo.txt`: TypeScript build and executable source demo.
- `verification-final.json`: source-byte preservation, source mapping, graph endpoints and maintained document links. `verify.py` reproduces these checks without writing artifacts.
- `vault-lint-final.json`: final deterministic strict wiki/provenance lint.
- `intake-*` and `polish-*`: inspected portable vault transactions. The legacy source inventory is outside the core's permitted write paths; the repository update appends matching rows while preserving the original bytes.
- `audits/`: candidate manifests, packets, independent model identity receipts and verdicts. Initial feedback is retained; final admission applies only to its bound candidate.

The initial GPT-6 review rejected a draft that conflated outcome authorization with a concrete execution commitment. The corrected design signs constraints in outcome mode and proves refinement of the chosen execution. Exact-plan mode can additionally commit to the execution body. Initial receipts and counts describe the earlier candidate; use the final verification and admission records for this change.
