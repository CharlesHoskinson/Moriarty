# Catalogue consumer presentation cleanup

This source-only review packet contains the full original eight candidate bodies under `history/original-eight/`, the prior full patch/pins under `history/`, and eight complete candidate bodies under `files/`. Only `docs/webs/app/src/catalogue.ts` changes from the prior candidate. The prior packet remains unchanged.

`consumer.patch` is the full eight-file candidate against immutable main71c22. `presentation-only.patch` is the one-file cleanup against the prior candidate. Root may use the full patch on main71, or the delta on an already integrated exact prior candidate; they are alternatives, not successive patches. `PINS.json` binds all original/candidate bodies and base blobs.

The minimal cleanup removes synthetic internal `ownerDependencies` fields and their model/process strings, and changes the local display envelope from `display-proposal/1` / `unapplied-source-proposal` to `display/1` / `source-display`. Related source comments describe the actual read-only adapter. These fields are not part of the retained catalogue source, predicate identity, statement schema or acceptance rule.

Inspection found that CatalogBrowser did not render the old top-level format/status or either ownerDependencies field. They nevertheless entered the browser-bound view. Removing them avoids carrying internal model/process metadata into the product. No new badge, approval wording, proof status or visual redesign is added. The raw source's own status/provenance strings are not globally scrubbed.

The existing public source path remains build→generated catalogueText/catalogueSource→App-mounted CatalogBrowser→typed `loadForDisplay`. App, CatalogBrowser, build/checker, generated data, fixture and three Node test bodies are byte-identical to the prior candidate. All143 versions/89 source heads/four complete typed schemas and every original source row/value remain unchanged.

`PRESERVED-DEPENDENCIES.json` gives exact new adapter and current JSX line references for source status/property/assumptions, statement schemas, profile/target/setup display fields and acceptance-related declarations. Those fields were deliberately preserved rather than interpreted or rewritten. Acceptance-related and assumption-assessment adapter declarations are not read by the current inspector. Audit/review/model-related strings in37 retained source-value locations remain quoted source provenance; their exact JSON pointers and source file hash are recorded. Removing those would change source data and is outside this cleanup.

The existing ordinary command from repository root after assigned integration remains:

```bash
timeout 30s node --experimental-strip-types --test --test-isolation=none --test-concurrency=1 docs/webs/app/src/catalogue.test.mjs
```

Expected: three data-conservation tests pass. **Unrun here.** Tests and fixture are unchanged. Their scope is original catalogue/generated data/loader conservation, not proof/security/acceptance validation. No new tests mirror the presentation literals; static exact-delta checks cover this small metadata cleanup.

`STATIC-CHECKS.json` records only own-directory public source/JSON/hash checks and in-memory patch reconstruction. No provider, product/Node/SDK/model/native/proof runtime, browser, build, key/wallet/ledger action, shared write or Git mutation occurred. The packet is unapplied; runtime, generated HTML/report, fresh applicable review and integration remain pending. Author prior implementation exposure is disclosed; no independent review or security judgment is claimed. All EARS/family/roadmap obligations remain independent and unchanged.
