# Catalogue loader refresh — main71c22

Unapplied eight-file patch against immutable `71c22f8a477231be869786eb42f04e3b50d1b3a7`. Exact Git blob/SHA pins, new-target absences and current public consumer sources are in `PINS.json`. `before/` holds four complete existing bodies; `files/` holds all eight complete candidate bodies. Earlier packets remain unchanged.

The actual main page still imports `main.jsx → App.jsx → data.generated.json`; no catalogue consumer is installed there. The candidate retains the existing build→generated catalogue text→App-mounted CatalogBrowser→typed loader wiring, source data and copy labels. Seven candidate bodies are unchanged; only the frozen fixture's commit provenance is refreshed. No website redesign or security interpretation is added.

Falsifiable predicate: After integration, generated.catalogueText/catalogueSource must reach the App-mounted CatalogBrowser and its real catalogue.ts:loadForDisplay; the existing three Node cases must retain143 ordered versioned identities,89 explicit source heads, all four complete typed statement schemas and original row values while preserving the eight existing page data groups. A missing import/carrier, relabelled or omitted identity, dropped field attribute or changed original value falsifies this data-conservation predicate. No proof/security/truth/ledger assertion follows.

The same three focused ordinary tests call the same typed loader with the actual generated page input. They preserve exact catalogue bytes/provenance,143 versions/89 heads, and every original row plus the four complete8/12/8/12 statement schemas. They validate data conservation only. Author prior source/implementation exposure is disclosed in `PINS.json`; no independent arithmetic/security oracle or audit is claimed.

Root's minimal intended Node24 argv after integration is:

```bash
timeout 30s node --experimental-strip-types --test --test-isolation=none --test-concurrency=1 docs/webs/app/src/catalogue.test.mjs
```

Expected: three tests pass. **Not executed here.** Imports are Node standard-library modules and the pure typed loader, without React, esbuild, SDK/provider/proof/ledger modules or package installation. Build/browser/HTML/report regeneration and fresh applicable review remain pending. The existing Node24 flags/interface assumption is unobserved in this task.

`STATIC-CHECKS.json` records fresh mechanical own-directory source/JSON/hash checks and in-memory patch reconstruction against the immutable base. No shared or Git writes, provider, build, product/native/SDK/proof execution occurred. All143 version identities,89 heads, four schemas, original statuses/setup language and prior copy labels remain source data; no substantive claim adequacy was assessed. All retained EARS/family/roadmap exits remain independent obligations.
