# Cardano Webs catalog labels and field wiring

Advisory documentation only, based on Git `64c6dedff1d99b33c23832330d7911a5fad9ed59`. No patch is authored here: `dwu_audit_conformance_pcd` owns the loader/UI/build patch. This packet inventories its existing candidate interface and proposes only copy/data labels. Root alone integrates. No runtime, build, browser, compiler, cryptographic or ledger operation was performed.

## What the current display omits

The pinned `docs/webs/app/build.mjs:17–28` builds explanatory data and `App.jsx:22–42` renders `data.scenarios`; neither loads the retained predicate catalogue. The generated data has **five** examples: invoice, exchange orders, token issuance, loan repayment and treasury allocation. Each has three role labels. The explorer intro at `App.jsx:32` names only the first three. The proposed label correction names all five; it changes no financial model or values.

The retained `CATALOGUE-v1.2.json` contains **143 versions, 89 source heads and four typed statement-schema rows**. These are source membership counts. The candidate browser supplies all 143 options, using exact `versionedId`, rather than discarding 54 superseded versions. A source head means that no retained row explicitly names that version in `supersedes`; it does not mean selected, accepted, compiled, deployed or latest by a version-number sort.

This is a display/discovery gap. The five example cards do not enumerate language constructs; the 143 catalogue entries do not enumerate callable compiler features. The experimental `mori-k2` consumer takes a source program and selected action/profile, not a catalogue ID (`experiments/moriarty-cardano-rust/README.md:88–110`). Its pinned documentation names `moriarty/1`, `moriarty-core/1`, `SettlementHashV2`, `NotBound`, `GrossDebitAndFees` and `TouchedOnly` and describes its specific construct subset. This source description is not a new execution result.

The workspace `mori` capabilities source is a different interface (`crates/mori/src/commands/capabilities.rs:87–129`). It describes Core/1 and Core/2 checks/reference consumers and native-feature compilation to distinct ZKIR artifacts, with Midnight target metadata. Its `SOURCE_REVISION` is a curated inspection basis, not this binary's identity. The Cardano drawer currently documents the experimental `mori-k2` path, so label it accordingly; do not silently use workspace Core/2 availability to fill Cardano capability labels. No executable or feature flag was queried in this task.

## Exact field contract

`FIELD-MAP.json` records every field consumed by the candidate `catalogue.ts`, its projection and display use. The loader consumes top-level `catalogueId`, `catalogueVersion`, `profiles` (keys only) and `rows`. It retains each full original row as `sourceRow`; additional row attributes remain visible there. Top-level catalogue rules/metadata are not projected into a full inspector. The candidate is a row browser, not a complete catalogue validator.

All four original `statementSchema` objects are preserved. `encoding`, ordered `fields`, each complete field object, `note` and `binding_constraints` are displayed. Source field attributes observed are `name`, `type`, `source`, `digest_of`, `index_of`, `role`, `tx_relation`, `min`, `max`, `value` and `note`. The interface also permits optional `context_item`, but none appears in these four source schemas. Missing schemas retain public-input prose rather than acquiring invented typed fields.

`resultSchema` is copied as a description string; `outputs.typedFields` remains null. The label **Source result description** is therefore more precise than **Result schema**. The labels **Recorded implementation evidence** and **Recorded first blocker** identify copied historical prose without suggesting a fresh assessment. `LABEL-PROPOSAL.json` gives exact current strings, proposed strings and candidate/base source locations. Claim text, source statuses, categories, assumptions and security-authored paragraphs stay untouched by this proposal.

The candidate's `profile.selected`, `target.selected`, predicate digest and setup identity fields remain null. “Not supplied to display” is display metadata; it cannot be reworded as a claim that an implementation, key or artifact does not exist. The source-file SHA-256 is a file identity and remains labelled as such. No profile/target or catalogue-to-action mapping is supplied by this display contract.

## Wiring and review handoff

Use the loader author's proposed `generated.catalogueText` for the exact retained public UTF-8 input and `generated.catalogueSource` for repository-relative path, SHA-256 and explicit commit/null. Keep this text outside the existing `clean()` transformation, which changes `path` keys to basenames. `CatalogBrowser` calls `loadForDisplay`, which projects rows and renders full source fields. It performs JSON parsing and a TypeScript assertion, not runtime schema validation or compilation.

Keep the application explorer and experimental compiler documentation independently navigable. Optional linking copy: “Browse retained claim versions below. The application examples and experimental compiler interfaces are documented separately.” Do not add “143 supported proofs,” certified counts, proof generation buttons or per-entry compiler availability from this source projection.

`EXPECTATIONS.json` contains the exact ordered 143 version identities, 89 source-head identities, all four ordered field-name lists and canonical schema digests, plus the five existing example/role memberships. They were extracted from the immutable source data, not from running the proposed loader. Candidate source was read before extraction; this is disclosed source-membership corroboration, not an independent financial oracle or independent review.

After the loader author's patch is integrated, the supplied ordinary command for root is:

```sh
timeout 30s node --experimental-strip-types --test --test-isolation=none --test-concurrency=1 docs/webs/app/src/catalogue.test.mjs
```

Expected: three ordinary conservation tests covering all 143 IDs, 89 source heads and four complete statement schemas. **Unrun here.** This command comes from the loader author's inspected-candidate handoff; this author did not inspect that newly authored test body. Root should review its final exact bytes before executing it. Build/static-render/browser checks and any copy acceptance remain separate. Existing passing suites were not rerun.

A useful UI predicate for later review is: selecting any retained version changes only the displayed original row and its original schema, while every source version remains selectable and recorded values remain unchanged. A prose-only row must show its description without a fabricated field table. A superseded row must stay selectable. These are ordinary display expectations, not hostile cryptographic tests.

## Files and limitations

- `SOURCE-PINS.json`: full Git blob/SHA-256 pins for baseline files and SHA-256 pins for the unapplied candidate loader/browser/build sources.
- `FIELD-MAP.json`: consumed fields, raw-row-only fields, absent display metadata and wiring contract.
- `LABEL-PROPOSAL.json`: seven precise optional copy changes and one navigation paragraph; no patch applied.
- `EXPECTATIONS.json`: exact retained source/display memberships; no proof catalogue claims added.

No public cryptographic statement, setup class, authentication claim, owner decision or ledger outcome was reviewed or changed. Existing 148 requirements, 292 scenarios, 20 families and U/M/MC/SP/G, finance, composition and target obligations remain outside this copy increment and are not reduced by it. A static source result is not product acceptance. No private or excluded historical data was read.
