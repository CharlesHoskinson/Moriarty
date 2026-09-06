# A5 compile timeout: bounded local source diagnosis

Status: source-backed diagnostic options only. No compiler, replay, solver, or diagnostic experiment executed. No runtime, model, recorder, or Foreman source changed. This is independent native review, not Council adjudication or adoption of H1.

The completed original-view invocation timed out with empty verbosity-0 stdout/generated JSON, stderr, and GNU-time output. Those observations do not locate a compiler phase. The earlier independent typecheck success does not establish that this compile invocation completed typechecking. Retain the terminal distinctions and limits in `a5-pilot-compilation-view-terminal-review.md`; the source inspection below adds no timing, RSS, or semantic result.

All source references below are relative to `/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/`. Package metadata identifies version 0.32.0. Read-only SHA-256 checks matched all 18 listed files against the frozen runtime manifest `.superpowers/sdd/a5-factoring-receipts/tool-store/manifest.json`, SHA-256 `fa3e9838c66480557ed6d928a2d159ebc074d05f67f15ef08ab20388d9c49e0b`. This was a selected-source check, not a new whole-runtime/archive audit.

## Observed source path

`cli.js:119–124` chains load → parse → typecheck → compile → outputCompilationTarget → outputResult. Parse itself includes source resolution, import/name resolution, and topological sorting (`cliCommands.js:121–155`). Typecheck invokes module analysis (`:162–172`). After typecheck, the JSON compile path is:

| Boundary | Installed implementation |
| --- | --- |
| Add selected entry definitions | `cliCommands.js:408–420` parses and appends `q::init`, `q::step`, and requested invariant/temporal definitions. |
| Resolve names again | `:424–429` reruns import/name resolution and replaces the lookup table. A resolution error returns here. |
| Incremental analysis | `:430` calls `analyzeInc`; `quintAnalyzer.js:44–79` creates a new analyzer, transforms lookup definitions, then analyzes the added declarations. |
| Optional flattening | `cliCommands.js:432–444` skips flattening only for explicit boolean `flatten=false` and a non-TLA+ target. Otherwise `:447` invokes `flattenModules`. |
| Flattening internals | `flattening/fullFlattener.js:44–76` unshadows names, inlines aliases, then loops through import/export flattening, instance transformation, and repeated name-resolution passes. Its helper at `:86–92` invokes parser phase 3; the nearby topological-sorting comment alone is not evidence of an additional phase-4 call. |
| Serialize and write | `cliCommands.js:506–510` builds the entire JSON string before writing stdout. `cliReporting.js:196–235` serializes selected stage fields including modules, lookup table, types, and effects. JSON output does not call the Apalache branch at `cliCommands.js:512`. |

## Source locations that could warrant measurement

These are structural observations, not measured hotspots or a timeout explanation.

- Import/name collection clones imported module tables (`names/collector.js:112`) and copied definitions (`names/base.js:33–39`); instance/export paths also clone tables (`collector.js:79,149`). Resolution occurs both before and after initial typechecking, and within flattening.
- Incremental analysis still constructs `TypeApplicationResolver`, which traverses and replaces every lookup-table definition (`quintAnalyzer.js:60–65`; `types/typeApplicationResolution.js:29–40`). Type applications resolve aliases and substitute arguments (`:60–114`).
- Alias inlining transforms modules, table definitions, and inferred types separately (`types/aliasInliner.js:30–56`); `resolveAlias` recursively follows aliases (`:115–122`).
- `ir/IRTransformer.js:75,95,253,308,379,458` deep-clones modules, types, declarations, definitions, expressions, and rows before recursive transformation. Its callers include alias inlining and instance transformation (`flattening/instanceFlattener.js:37–40`). Actual instances additionally copy and rename prototype definitions with fresh IDs (`:104–119`); this inspection does not assert the pilot exercised that conditional branch.
- `flattening/flattener.js:136–145` optionally namespaces a definition, then walks nested references **before** checking whether that definition was already collected. Namespacing calls the definition transformer (`ir/namespacer.js:27–30`). Thus the deduplication shown does not prevent that preceding nested walk from repeating; no frequency or cost was measured.

## Existing controls and smallest useful next diagnostic

`--verbosity` is declared at `cli.js:95–99`, but the inspected JSON compile path has no routine stage-progress log calls. The console/debug calls found in its parsing/flattening sources concern errors or unexpected parser states. Increasing verbosity alone therefore supplies no source-backed promise of phase boundaries. `--out` forces derived verbosity to zero (`cliHelpers.js:123–124`) and writes a completed/error stage through `outputResult` (`cliCommands.js:533–561`); it is not a live progress checkpoint. `--source-map` is declared for **parse**, not compile (`cli.js:64–69`, strict command parsing), and is written after source resolution but before name resolution (`cliCommands.js:139–143`). A separate parse/typecheck receipt cannot locate the historical compile timeout.

Two bounded options could be reviewed, with different evidentiary value:

1. **Existing CLI contrast:** keep the exact semantic source inputs and compile selections, but set `--flatten=false` for JSON. A successful new invocation would establish completion of its own parse, typecheck, second resolution, incremental analysis, and serialization while omitting full flattening. It changes the compiler output, so it is not an admissible replacement for the flattened pilot or evidence of equivalent solver input. Either success or timeout still would not locate the historical run's phase. It requires a new reviewed command/receipt plan; do not reuse the closed six-stage authorization or its success predicate unchanged.
2. **Minimal direct phase observation candidate:** a separately pinned wrapper around the existing exported CLI stages (`cliCommands.js:47–56`) could emit bounded monotonic entry/return/error markers, while preserving the exact stage order, arguments/defaults, and mutable object threading. Markers must distinguish a returned error from success. Do not serialize intermediate objects, rerun stages, or infer completion from an entry marker. This would distinguish parse/typecheck/compile/output intervals in that new invocation; it would not distinguish second resolution from alias inlining inside `compile`. Deeper instrumentation is a separate, more intrusive design and review decision.

The second option is the smallest direct phase-measurement candidate identified here that can retain the same transformations. Its source preservation is a design requirement, not an implementation verdict: no wrapper was written or approved. A reviewed design must fix argument handling and interception, archive the wrapper, retain authentic child/outer exits and bounded cleanup, and use a separate diagnostic stream or explicitly revise the empty-stderr acceptance rule. Preserve the existing timeout/memory limits, immutable 24-file view and 30 originals, runtime pins, and fresh receipt paths; disclose instrumentation overhead. Changing target to TLA+ would add backend work and is not a diagnostic of this JSON-only path. No resource increase, compiler patch, flattening optimization, H1 adoption, or new native invocation follows from this brief.

## Exact inspected source pins

Paths in this table are relative to the package root (including `dist/src/` where applicable).

| File | SHA-256 |
| --- | --- |
| `package.json` | `08355263da6adb7d5578c2cddb9b0e9634d79cfb2f73e4679888031067c332a4` |
| `dist/src/cli.js` | `ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501` |
| `dist/src/cliCommands.js` | `b18672d656782aefde254fe1021dc13fdffdcf8f4ad8709e3126fb5fa293362a` |
| `dist/src/cliHelpers.js` | `4ad6b6364d2a5d07389f2518aa80013bfd01dbdbab6201b53fd6b042eb8a91dc` |
| `dist/src/quintAnalyzer.js` | `fd53e14aafb5b2e6c5846eddc8ef9f175b76a53b55fa187c0a519706472b512a` |
| `dist/src/types/typeApplicationResolution.js` | `7a48a13ecabdfdb0e46b7a68581c5fcfd558adb37f067127753eddf8b40d2535` |
| `dist/src/flattening/fullFlattener.js` | `91bb56f07bb49ed354617f2307fa7949921e224a0be300c4fc397e894b071a1b` |
| `dist/src/flattening/flattener.js` | `92dc7f04d859d0ffef83145ad8f6ebbcde60d561a404fa880b9d33750cf5fe7e` |
| `dist/src/flattening/instanceFlattener.js` | `b39b544ae2470f602bdadb11773935da863b1cd4a2dbefbce13faa5362eae01e` |
| `dist/src/types/aliasInliner.js` | `dbdbc0c1952ce5d5cf275862457498ab2539c995c3d51743ae5082f7af897428` |
| `dist/src/ir/IRTransformer.js` | `47251acd2fcc7710deca82d8d97c79ad77e998be1413c1f1608b5c700a123d31` |
| `dist/src/ir/namespacer.js` | `b975783afb58853b33926ef56eec18070a9f09828a197a232571f1d790a1fcdf` |
| `dist/src/names/collector.js` | `5882527bc8d879537babfb88786714e4d048d3b884f228d2aaa4459170e00e35` |
| `dist/src/names/base.js` | `31e89f38ddbf5a834086901b49072c38c970c69398018a5676e2f26189dcefc0` |
| `dist/src/cliReporting.js` | `823889ffe67fa0132f51f2a40d40bb663d8b6280c4c1077dfe9dfb77be9ba66e` |
| `dist/src/verbosity.js` | `3a0a9857b248b3e0d3a14e70ea04bf78f6e4cee57370bce5eafe6c8c29024423` |
| `dist/src/parsing/quintParserFrontend.js` | `4d22b256295adf521c1af60865eb0f8254e65c024c5e8b2271d1b7f8701d7b85` |
| `dist/src/parsing/ToIrListener.js` | `7d7fcfe5c2bbbe2d48e8e921f2be58d1c723848f487f63e42944b6b2bee81104` |
