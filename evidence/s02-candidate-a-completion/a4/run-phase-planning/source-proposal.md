# A4 case014: next diagnostic source proposal

**Recommendation: prepare a bounded run-phase observation design before another costly invocation.** A case014-only typecheck is the smallest command change, but the existing admitted final aggregate already successfully typechecked the same semantic sources under the same recorded runtime. A new observed run can answer the still-open question directly: does a new case014 invocation complete public typecheck and enter public runSimulator before failing? This is a changed diagnostic question, not authorization to repeat the consumed canonical export stage.

Status: source/evidence inspection only. No native/test/parser/helper import, retry, structural intake, implementation, source edit or commit. The failed014 original remains immutable;044 and all78 remain closed.

## Existing evidence is stronger than a stale parser result

The final Task2 aggregate typecheck is `.superpowers/sdd/a4-producer-receipts/task2-all-wrappers-types/receipt.json`, SHA-256 `106d55614a0fdfbb23f2509f123e59e18901afa98e334fe857269db4d49744dd`. Actual requested argv is `quint typecheck specs/quint/s02/candidate_a_integrated_wrappers_typecheck.qnt`; resolved argv uses the same pinned Node/CLI as failed014, with no extra heap option. Child/recorder exited0 in849.015616322seconds, with empty original stdout/stderr.

I compared every one of its104 Quint source pins and208 original before/after Quint copies against failed014/current source bytes. All match, including case014, corrected case032 and corrected lowering. Its entire runtime-before/after maps equal failed014's8,102-pin/four-tree maps. The99-module aggregate import closure includes case014; case014 alone has20 recursive modules. Thus final coverage includes the corrected semantic sources, although aggregate entry and standalone entry have different module sets and allocation histories.

The final Task2 Rust test receipt `task2-quint-green/receipt.json`, SHA-256 `2436fd4efc72c2b0dd1dee176a0bcf1253665c857e9ad5c1cbe354a7e499fa1a`, also has those matching104 sources/208 copies and identical runtime maps. It reports child/recorder0 in717.581312148seconds; original stdout lists all12 passing tests, including route/boundary/native-start checks. This is not a full28-state case014 trace. The tiny successful literal probe (`task2-literal-probe-run`, receipt `e390770bbcc07583bc42f19b38e3d8dd781fa46fa407937f1a76cc0050699a85`) proves only its distinct small carrier route; its then-current Candidate A source map predates many later changes.

Do not describe the historical120-source capture as the current complete121-source freeze. Two Python support files changed afterward (`record_s02_candidate_a_integrated.py`, `test_s02_candidate_a_integrated_export.py`) and the exporter was added. Historical final jobs retain different before/after HEADs, e221eaff→0d2727e7, while their source maps are stable. Root producer Task2 admission explicitly records those root-only commits and local-unit limits. No missing historical outer/GNU-resource receipt is supplied from inner exit0. The newly admitted final-source parser is parsing evidence only; it is not the reason the stronger historical typecheck is applicable.

## Source-supported phase distinction

Installed `cli.js:308` runs `load → parse → typecheck → runSimulator → outputResult`. It does **not** call the public compile/full-flatten chain. `cliCommands.js:266–330` prepares arguments and resolves init/step/invariant/witness expressions, then calls Rust simulation using the resolver table. `cliHelpers.js:33–45` parses and resolves those expressions; it does not compile the model there. `rust/commandWrapper.js:214–255` first obtains the evaluator path, then serializes the request with `json-bigint.stringify(input, bigintCheckerReplacer)` before spawning Rust. This is not a measured serialization failure: subsequent Rust output collection, parsing and ITF handling are also inside the runSimulator interval, and the existing OOM has no phase markers. Empty stdout/noITF does not prove Rust was never spawned.

The actual JavaScript OOM and historical frontend successes make work after typecheck a reasonable hypothesis to discriminate, not an established historical location. An A5 no-flatten flag is not an evidence-backed A4 remedy: this run chain has no compile callback to skip.

## Options and proposed decision

| Option | New observation | Cost and limits |
| --- | --- | --- |
| One unchanged-source `quint typecheck specs/quint/s02/candidate_a_integrated_case_014.qnt`, unchanged RH002+integrated recorder,900-second outer bound/no extra heap | Whether a new standalone20-module frontend invocation completes under that capture | No new observer or launcher; smallest operational change. Success adds entry-specific resource/terminal evidence but cannot locate the historical OOM or guarantee a subsequent run. Failure locates nothing finer than this new frontend route. Existing99-module aggregate success limits its information gain. |
| **Recommended: independently designed and controlled public run-phase observation** | Whether the new invocation completes typecheck and enters runSimulator, or leaves an earlier public interval incomplete | Reuse the reviewed non-async observation mechanics, exact receiver/args/value/Promise identity, Left skipping, bounded trace and failure74 contract. A separately frozen run-specific phase profile must include runSimulator; a new narrow launcher must retain the exact run argv. The old immutable six-phase observer and eight-argument compile launcher do not supply this unchanged. Fresh tiny identity/failure/run-equivalence controls and independent admission precede any full diagnostic. An incomplete runSimulator interval still does not identify serialization, Rust execution, output conversion or an inner hotspot. |
| Deeper serialization/spawn observation or a representation change | Could discriminate later suboperations once a public run interval is established | More invasive controls and attribution requirements; no suitable already-admitted implementation identified. Defer. No heap escalation, TypeScript backend switch, omitted fields, case subdivision, alternate semantic source or export acceptance bypass is proposed. |

Before the recommended option can execute, root must adopt a separate exact design and plan: new exclusive diagnostic stage/output path (never canonical014 reuse); immutable original source/runtime and helper closure; phase profile/launcher code and source pins; exact original-argv contrast limited to diagnostic output routing; unchanged wall/default-heap settings unless separately reviewed; own-group cleanup; truthful separate endpoint handling on missing inner receipt; strict actual trace/transport/index predicates; tiny-control admission; one explicit full dispatch and stop after its terminal. Reusing mechanics does not transfer the old compile controls' admission to runSimulator. A successful diagnostic ITF would remain diagnostic evidence until separately reviewed; it cannot silently replace the failed export.

## Source pins and inspection record

| Source | SHA-256 |
| --- | --- |
| Installed `dist/src/cli.js` | `ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501` |
| Installed `dist/src/cliCommands.js` | `b18672d656782aefde254fe1021dc13fdffdcf8f4ad8709e3126fb5fa293362a` |
| Installed `dist/src/cliHelpers.js` | `4ad6b6364d2a5d07389f2518aa80013bfd01dbdbab6201b53fd6b042eb8a91dc` |
| Installed `dist/src/rust/commandWrapper.js` | `1f26d3f1c31529572b5be32276d2d94c19a3d0298dbf444891802f744e337e1c` |
| Existing integrated recorder | `dc30b764e2603dfe3c39ef6e2063a7082d116dc9d3eb9a012072f468beb8f8e9` |
| Existing RH002 runner | `d8973d3541269d1be2ce524f2482e5f5b858f15f7d9117e3fe6d5cd2171d660d` |
| Producer Task2 `validation.json` | `8d87711d0cd801c3abca9b29b78e340dcdee8d37581d4d196a59f1addc9674dd` |
| Producer Task2 `review.md` | `8d0197841d4a952c014e90f37bfdda4c989920de9dc9115790a8874975a930ad` |

Small source/evidence comparison actual tool chunk e398be exited0: both historical final source sets and416 combined original copies match; both runtime maps equal the failed014 maps; exact closures20/99 and six listed selected-source/admission hashes were independently checked. This comparison did not rehash runtime archives or execute archived code. Earlier search commands encountered two absent guessed paths and an overbroad `.map` match; these were read-only search misses, not native diagnostic attempts. The original producer archive remains the already-admitted3,230-member archive98b132da; this source-only proposal did not rerun its full archive audit.
