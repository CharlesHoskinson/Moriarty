# K64c6 catalogue consumer and isolated loader regression

This unapplied patch refreshes the actual catalog consumer candidate against immutable source `64c6dedff1d99b33c23832330d7911a5fad9ed59` in K. Existing App/build/checker/generated-data files are byte-identical to the previous packet's base; `SOURCE-PINS.json` records exact Git blob IDs and SHA-256. Every source was resolved through this immutable commit. Current working-tree movement does not change those pins.

The real path remains `build.mjs → src/data.generated.json → App.jsx → CatalogBrowser.jsx → catalogue.ts:loadForDisplay`. The build embeds the retained catalog's exact public UTF-8 bytes and file provenance in generated page data. App renders the inspector within its existing technical notes. Every143 versioned row and89 source head remains selectable; statement definitions and original source rows remain intact. No website redesign, copy-label revision, predicate interpretation or acceptance rule is introduced. JSX/copy bytes are preserved from the preceding consumer candidate; advisory Astra owns label proposals separately.

`files/` contains complete bodies for six consumer candidate files and two new regression files. `consumer.patch` carries the same changes against exact K64c6. The earlier schema/consumer packets remain untouched. No copied bulk review packet is needed: source pinning, the literal expectation fixture, patch and this note identify the candidate.

The new `src/catalogue.test.mjs` calls the same typed loader used by the actual inspector, with the same `data.generated.json` input used by App. Its three ordinary tests check exact catalog transport/provenance; all143 literal versioned IDs/89 source heads; and full original row plus ordered typed statement-schema conservation. The fixture preserves all four typed versions and their8/12/8/12 field membership. This is source-data conservation, without hostile cases, proof/security claim validation, setup judgments or ledger assertions.

Expectations come directly from the retained immutable catalog, not from candidate output. The author previously inspected the catalogue and authored both earlier proposals; this is disclosed source exposure, not fresh independent authorship, an independent arithmetic oracle or a review verdict. Original source strings—including statuses, assumptions, setup language and bindings—are compared as data, without judging their truth or adequacy.

After root integrates the eight files into the exact base, the smallest intended check is, from repository root with Node24:

```bash
timeout 30s node --experimental-strip-types --test --test-isolation=none --test-concurrency=1 docs/webs/app/src/catalogue.test.mjs
```

The command uses Node's built-in test runner/type stripping and public JSON reads; it imports no React, esbuild, SDK, key/proof/native or provider module. It requires no package installation and does not build the page or write generated artifacts. Expected result: three passing ordinary conservation tests. It was **not run** in this source-only wave. The Node24 command/interface assumption remains to be observed by root; no installed-version or runtime claim is supplied here.

Pure Python JSON/source/hash/patch reconstruction checks are recorded separately in `SOURCE-COMPARISON.json`. They do not substitute for the unrun JavaScript regression. Full page build, HTML/PAGE-REPORT regeneration, static/browser/keyboard checks and fresh applicable review remain later integration work. Existing passing checks were not rerun. Original148 requirements/292 scenarios/20 families and U/M/MC/SP/G/financial/composition/target acceptance remain unchanged and open at their own scopes.
