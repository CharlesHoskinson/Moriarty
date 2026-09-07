# Candidate A: one no-flatten compiler contrast

Status: source-only proposed design for independent root review. No code, control, compiler, parser, solver or helper was executed for this design. No model, runtime, receipt or existing capture source changes are proposed. Execution needs a separate reviewed concrete plan and root dispatch. The brainstorming skill supplied the design workflow; the delegated design-only scope excludes its default commit and implementation steps.

## Evidence and question

The admitted new phase diagnostic ended at its 900-second bound. Its seven original trace rows show load, parse and typecheck resolving Right, then compile entry with no completion row. The traced public compile interval is incomplete; this does not locate an internal operation or prove where execution was at timeout. The time-wrapped child exit was -15, with complete forced cleanup. Recorder/outer zero describe preservation. Empty stdout/stderr/resources supply no generated compiler JSON or CPU/RSS measurement. These conclusions belong to this invocation, not the historical uninstrumented timeout.

Original references, relative to `.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/`:

| Original | SHA256 |
| --- | --- |
| `full/full-observed/trace.jsonl` | `d3b05f37c9976dfa99a2f34897aced0289060c7278c36517dc06f15f9adee536` |
| `full/full-observed/process-terminal.json` | `8d00cbc294996d789c7815e5c8590e159f3224a5a27775bcf2283719582f4e35` |
| `review-final.md` | `22d592b2b9c5f74d57c6929d656b816e50bc9ed23e72875bdbaafc92c76ae24b` |
| `freeze.json` | `59a2ce768c0e26683ebec421d7ef091b9c3a8f5894b844ad6f8c57e599d9e405` |

Question: can one new invocation finish the same original-view JSON compilation when the existing CLI skips full flattening? The result is an observation about that altered command, never an H1 result or substitute solver input.

## Alternatives and choice

1. **Recommend the existing `--flatten=false` contrast.** It changes one documented compiler switch, retains identical semantic source bytes and all model selections, and needs no observer. The installed implementation still creates `q::init`, `q::step` and `q::inv`, performs the second import/name resolution, and calls incremental analysis before the early return. Successful JSON output therefore establishes those operations completed in this new command. The unflattened representation is deliberately different; failure or timeout has no new phase localization.
2. **Deeper internal hooks.** Observing exported `parsePhase3importAndNameResolution`, `analyzeInc` and `flattenModules` could separate compile subintervals while retaining flattening. Those exports also occur on other routes or recursively; a new design would need call-context attribution, nesting, bounded trace volume, synchronous receiver/argument/value/throw identity and fresh fault/CLI equivalence controls. The existing six-export observer cannot supply those conclusions unchanged. This is the more invasive next option if the contrast proves uninformative, not part of this authorization.

The source supports a useful conditional distinction, not a cost attribution: no-flatten success while the admitted flattened observation timed out is evidence consistent with work omitted by the switch being material. Separate executions, observer removal, scheduling and different capture intervals prevent a proof of causation or a subtraction-based timing claim. No-flatten timeout leaves both pre-flatten work and output work possible; it does not prove flattening irrelevant or that this invocation reached compile.

## Fixed command and budgets

Retain the existing original compilation view entry `.superpowers/sdd/a5-factoring-receipts/pilot-compilation-view/view/specs/quint/s02/factored_verification/candidate_a_funding_pilot.qnt`, main `candidate_a_funding_pilot`, target `json`, invariant `pilotSafety`, verbosity `0`, and original default init/step. Invoke the pinned Node directly with `--max-old-space-size=4096`, then the pinned CLI, `compile`, the absolute entry path, `--main=candidate_a_funding_pilot`, `--target=json`, `--invariant=pilotSafety`, `--verbosity=0`, and exactly the added `--flatten=false`. Do not add `--out`, switch target, change invariants or run a solver. Original compiler stdout is the diagnostic JSON artifact.

Node: `/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin/node`, SHA256 `f3432a45b03b2da0d270095fdd8813dc34cbea73f5fc8b18c7a384b7cf9b333a`. CLI: `/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js`, SHA256 `ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501`.

Allow exactly one fresh tiny capability control, at an RH002 outer bound of 120 seconds, followed only after independent control admission by exactly one full contrast at an RH002 outer bound of 900 seconds. Node old-space remains 4096 MiB for both. This is not a total RSS limit. The outer bound includes recorder preparation, four version probes, runtime/source capture and child execution; it therefore supplies at most 900 seconds to the full compiler, unlike the preceding observer's 900-second child interval. Record this difference; do not increase the bound to compensate. No retries are included. A4 native activity must be terminal, with prior owned groups absent, before either control or full dispatch; serialize all native work.

## Minimal unchanged capture stack

Use the existing RH002 CLI around `scripts/record_s02_candidate_a_integrated.py`, which launches the direct Node/CLI command in the inherited owned process group. Its `subprocess.run` does not create another session. RH002 supplies the new session, wall bound, group cleanup, launch/resource/terminal receipts and actual time-wrapped child exit. The integrated recorder supplies an authentic inner receipt on normal completion and full shared runtime checks. Its existing four tool version probes remain disclosed setup work. No globals, source roots, stage constants, environment functions or acceptance predicates are overridden.

The integrated recorder requires a fresh stage under `.superpowers/sdd/a4-producer-receipts/`, `-B`, and its exact fresh `-X pycache_prefix=<stage>/python-cache`. Use reserved stage names `a5-noflat-tiny-20260906` and `a5-noflat-full-20260906`; these names identify A5 diagnostics despite the fixed A4 storage parent. Their original `stdout.bin`, `stderr.bin` and `receipt.json` remain at these paths. Its ITF-only artifact glob produces `{}` for this command: the original stdout JSON needs an explicit external artifact pin. Do not name JSON output as an ITF artifact to force inclusion.

Reserve new root-owned metadata under `.superpowers/sdd/a5-no-flatten-diagnostic-20260906/`, with separate `tiny-outer/`, `full-outer/`, parent-cache directories, source snapshots, current dispatch data, original command transport and final admission files. The inner receipt must be outside its RH002 outer directory. Root transport originals are saved incrementally outside any hash-finalized input directory. The later plan must enumerate the exact exclusive paths and command arrays before creation. No generic harness, new recorder or wrapper source is needed.

Rejected shortcuts:

- Direct Node under RH002 alone cannot meet its positive receipt gate: `capture()` requires a real inner receipt and independently validates none of its body. A fabricated receipt or precreated placeholder is unacceptable.
- The old A5 `record()` helper creates its own new session and has incomplete descendant cleanup. Nesting it inside RH002 can escape the outer owned group. Do not reuse it for execution.
- The closed observer `launch.cjs` requires exactly eight effective arguments. It rejects the extra flag unchanged; this design does not patch or reuse that launch path.

## Source and runtime evidence closure

The integrated recorder's 121-source snapshot does **not** include the A5 copied view. Keep this snapshot honestly labeled, and add separate exact before/after original-byte evidence for all 24 immutable view files plus all 30 frozen originals: 54 distinct paths, not an inferred import subset. Check every entry against the existing view manifest and task2 freeze, including the exact two copied adapter import additions and every unchanged view file. The actual input is the old view path, never a regenerated view or current model substitute.

Also bind all four frozen alias-control sources; the existing view manifest, task2 freeze, old original-compile input, alias index, phase diagnostic freeze/admission, this design and later reviewed plan, RH002, integrated recorder and its Python dependency closure, current root dispatch, and the shared runtime references. Preserve exact source archive membership and bytes, resolving overlaps once by path and rejecting conflicting hashes. Source archives and preparation snapshots are taken before dispatch and rechecked after confirmed cleanup, not supplied retrospectively from unverified current files.

Use the admitted shared runtime verification interfaces unchanged, with their historical runtime bootstrap/dispatch arguments. Before each invocation, check the full expected runtime inventories, membership, external archive pins and selected actual executables; after it, check them again. The normal integrated receipt provides these checks, but a timeout may not. A separately reviewed root data-only endpoint capture using the existing runtime routines must close that gap after the group is absent. Record both actual current preparation/dispatch HEAD and old source/runtime bases in separate fields. Endpoint equality supports endpoint stability, not continuous historical immutability.

Known retained bindings: view manifest SHA256 `5b158d684be97052eb362f217c25f1fbaf03583fbd67fd46c5972b0662dcf1a2`; task2 source freeze `97fad04118219a75017a738eb7dc4fb013d351161892cfa05e26c26528f55ab2`; alias index `7dc430fd098e7fded8ebac492cfbda9d486ffa863802bf830e21ef330e563fde`; shared runtime manifest `fa3e9838c66480557ed6d928a2d159ebc074d05f67f15ef08ab20388d9c49e0b`. The implementation plan must enumerate all referenced paths and remaining pins from those originals; this design is not an executable freeze.

## Tiny control and terminal admission

Use the unchanged `.superpowers/sdd/a5-factoring-receipts/alias-visibility-control/src/driver_direct.qnt`, SHA256 `5f786022cef03df2ec4daa9cf83a1d0350079d7f131cfa4a3926c972f2b6a386`, main `alias_visibility_direct`, invariant `safety`, with the same command structure and `--flatten=false`. Preserve all four frozen control source files, including the three-file actual direct import closure. Require actual inner child/recorder/RH002/tool exits zero, no timeout or forced cleanup, stable source/runtime closures, empty stderr, and bounded strict JSON stdout. Check stage `compiling`, clean warnings/errors, main and sole serialized module `alias_visibility_direct`, retained original imports and `Concrete` typedef, and the generated `q::init`, `q::step`, `q::inv` declarations referencing the selected inputs. Check these against the raw source and admitted flattened control output; require the representation difference, not byte equality across flatten settings. This is a capability/route control, not a new equivalence proof. Any failed control predicate closes the gate before the full invocation.

For full success, require the same process/receipt/source/runtime gates and strict JSON stdout with `compiling`, clean warnings/errors, main and sole serialized module `candidate_a_funding_pilot`, original unflattened declaration/import structure, and the three generated selections. Do not apply flattened declaration completeness or solver-readiness predicates to it. Bound data intake at 128 MiB; preserve larger output unchanged but do not admit its content under this design. The later plan must fix exact expected JSON fields and declaration structures from the pinned serializer/source before execution.

Timeout or nonzero exit can receive **failed diagnostic preservation** after authentic terminal, complete cleanup, original bytes and source/runtime endpoint checks. It cannot pass RH002 eligibility or the success/control predicate. On timeout RH002 may kill the integrated recorder before its after-snapshot and inner receipt; preserve the missing receipt as missing. Do not invent a Node exit from the time-wrapped process's -15 or manufacture inner success. Root's separate post-cleanup endpoint capture can admit preserved observations, not reconstruct an absent compiler phase or exit. If a group is unresolved, defer artifact hashing/admission even though RH002 itself may have emitted hashes. Preserve launch/poll/terminal failures and require explicit root takeover before continuation; no native retry follows.

No generated diagnostic output enters H1, A4 exports or a solver. No full/factored equivalence, factoring correctness, compiler cause, Council or A5 completion conclusion follows.

## Source basis and remaining plan contract

Installed runtime references below are relative to the pinned Quint package. This design reread and hashed these selected files; it did not execute them or repeat the whole-runtime audit.

| Source | Relevant interface | SHA256 |
| --- | --- | --- |
| `dist/src/cli.js:79–124` | boolean flatten option, JSON target, phase chain | `ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501` |
| `dist/src/cliCommands.js:400–459,489–514` | extra definitions, second resolution, analysis, early return, JSON output | `b18672d656782aefde254fe1021dc13fdffdcf8f4ad8709e3126fb5fa293362a` |
| `dist/src/cliReporting.js:196–239` | serialized output-stage fields | `823889ffe67fa0132f51f2a40d40bb663d8b6280c4c1077dfe9dfb77be9ba66e` |
| `dist/src/flattening/fullFlattener.js:44–92` | skipped transforms and repeated resolution | `91bb56f07bb49ed354617f2307fa7949921e224a0be300c4fc397e894b071a1b` |
| `dist/src/parsing/quintParserFrontend.js:289` | deeper-hook candidate export | `4d22b256295adf521c1af60865eb0f8254e65c024c5e8b2271d1b7f8701d7b85` |
| `dist/src/quintAnalyzer.js:44` | deeper-hook candidate export | `fd53e14aafb5b2e6c5846eddc8ef9f175b76a53b55fa187c0a519706472b512a` |

Capture sources: `scripts/record_s02_candidate_a_integrated.py:57–83,151–158,168–237`, SHA256 `dc30b764e2603dfe3c39ef6e2063a7082d116dc9d3eb9a012072f468beb8f8e9`; `evidence/s02-candidate-a-completion/a4/native-resources/runner.py:166–260`, SHA256 `d8973d3541269d1be2ce524f2482e5f5b858f15f7d9117e3fe6d5cd2171d660d`; `scripts/run_s02_candidate_a_factoring_pilot.py:172–232`, SHA256 `816c3ad79dc3a67ca9a03729af56d74188c161a7546b9c43e821c222a4bce7f2`. The closed observer launch pin is `f14dc497792d6e440efa4ec974cf838e884adb0945d51c8c1844ed43578af06b` (`:18` fixes eight effective arguments).

Before any dispatch, the implementation plan must concretely fix: exclusive root/inner/outer/cache/transport ownership; exact 54-plus-support archive membership and before/after routines; full runtime endpoint handling on missing inner receipt; exact native argv/environment comparison to the old command; strict output predicates and limits; original tool-response preservation; and stop/cleanup/takeover gates. RH002 lacks a parent signal handler and publishes the PGID only in its terminal: use its owned timeout, avoid external parent interruption, and require actual process inspection after transport/host loss. These are explicit existing-interface limitations, not permission to patch the harness. If the plan cannot satisfy any gate with the unchanged interfaces, return that concrete gap for design review before materialization.
