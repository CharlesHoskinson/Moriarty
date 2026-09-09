# Research wiki root-bundle delta audit

**PASS — bounded delta only.** GPT-6 Codex `/root/research_publication_review`, independent publication reviewer, 2026-09-09. This supplements `publication-gpt6-final.md` and preserves that audit's application limits.

Reviewed `deliverables/language-design-2026-09-09/wiki-draft/wiki-transaction-root.json`, SHA-256 `559641e8b28afc38b55fdcc2151096c4d42fd492ccd038220796c3dad6b1b96d`, against previously reviewed `wiki-transaction-final.json`, SHA-256 `737fd0a2aedccdcc1c8c21b838f8124ca28d4cc1c3a7bef10dae186364a159dc`.

The exact changes are a supporting link to the grammar-review dossier in the architecture page and the source/claim ledgers' generated_at values, from midnight to `2026-09-09T18:58:55.690346Z`. The grammar claim itself is unchanged. All top-level transaction metadata, expected hashes, write paths/modes, other page text, collection bytes, source records, claim records and ID mappings are unchanged. Structural comparison after removing generated_at confirms both ledgers equal the reviewed originals. Thus the original source/claim preservation and CSV-companion consistency conclusions carry forward.

No material regression found. No canonical apply performed. Reinspect against merged main, preserve unrelated edits and check final README/grammar-review link targets before apply; this note does not authorize reusing another worktree's inspection receipt. Guarded status was refreshed and still reports unresolved implementation history with no pending transactions. No acquisition, source execution, proof run or financial call occurred.

## Final binding after portable-inspection corrections

**PASS on final root bundle SHA-256 `1c99a1bb49fe9227908e2b342c2d61bcfad6de07cb5bc760928b66b26c8153f7`.** This supersedes the intermediate `559641…` binding above; the earlier observation remains historical. Independently rechecked all eight per-write SHA-256 values against their content. Ledger timestamps are now canonical UTC seconds, `2026-09-09T18:58:55Z`. Relative to the original reviewed final bundle, the only content deltas remain these two timestamps and the grammar-review citation. Source/claim entries, collection and other content are unchanged. The stored root inspection (`wiki-root-inspect.json`) reports valid, and its complete hash map matches these eight content values. Earlier pre-write inspection failures are preserved separately; they are not successful applies. Canonical application still belongs to the parent and is not claimed by this audit.

## Final link-only repair

**PASS — `wiki-link-repair.json` SHA-256 `8cb3d40027b4a8e48833d05ada274c44eabec0876e107bcda16cf0ce9971cda0`.** Compared its sole write, wiki/moriarty-architecture.md, directly with the audited root bundle. Its entire content equals the previous content with exactly two `../README.md#small-step-semantics-implemented-repayment-subset` targets replaced by `../README.md`. Labels, claims and all other bytes are unchanged. The resulting architecture page bytes observed in the research checkout match that replacement. Source/claim ledgers are outside this repair and remain unchanged by it. This removes dependence on differing GitHub/Obsidian anchor normalization without changing the cited document or semantic scope. Final lint/apply status remains reported by the parent workflow rather than inferred from this content check.
