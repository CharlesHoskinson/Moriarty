# Grounded change proposal 01: obligation 127

## Obligation and scope

Obligation 127 records:

> K parse04 actual run finished; no live exec98449 remains. Terra roadmap_k_parse_terra source-only diagnosis of trace106 kore-expand-macros segfault in progress. Preserve failed frozen build and evidence; cumulative8compiler219krun. No retry until grounded change/proposal and fresh Astra medium plus Grok high audits then root admission. Prior obligation126 binding supplement completed; full115 suite not accepted.

This document supplies only the grounded change/proposal prerequisite. It does not retry the frozen case, request or grant admission, provide either fresh audit, or establish that obligation 127 is satisfied.

Task 4.1 is to pin the retained failure and admit one bounded reproduction. Task 4.2 is to “Resolve the reproduced backend defect without narrowing the accepted language.” The accepted boundary therefore remains fixed: retained `ConstructNone` has 5947 nested `Option` metadata and exactly 65,536 Core bytes; metadata depth 5948 produces 65,547 bytes and rejects `INPUT_BOUND`. Metadata depth is distinct from expression/value depth. A repair must not impose a 1500 metadata ceiling, cut the 113-case historical domain, skip trace-106, erase type metadata, or replace independent K typing with host validation.

## Observed failure

The task brief’s shorthand—“a segmentation fault in kore-expand-macros during K parsing, tracked as macro05 trace106”—is respectfully but materially imprecise. The retained evidence records two distinct trace-106 SIGSEGV observations at different pipeline stages and under different builds/attempts:

- **parse04:** `krun` used default macro expansion. Trace-106 returned 139, did not time out, and ran for 2.461 seconds. Stderr explicitly reports SIGSEGV while running `kore-expand-macros <definition> <temporary-input>`; stdout is empty. This is the kore-expand-macros-stage event named by obligation 127.
- **macro05:** `krun` added `--no-expand-macros`. Trace-106 returned 113, did not time out, and ran for 3.927 seconds. Stderr reports the shell’s segmentation-fault line followed by the failed command, `<definition>/interpreter <temporary-input> -1 <result.kore>`. A later `kore-print` step reports “Unexpected end of file while parsing” as it tries to read `result.kore`. That EOF is downstream missing or incomplete interpreter output; it is not independent evidence that the original input was malformed.

Disabling macro expansion did not fix the underlying problem. It moved the observed crash from `kore-expand-macros` to the LLVM interpreter and thus showed that the problem is not confined to macro expansion. The two crashes must not be merged into one event, and macro05 must not be described as narrowing the defect to `kore-expand-macros`.

Both attempts use K release `v7.1.337`, upstream commit `4a46d1231473b599c699160132fd6e76a5c46406`, package `k-7.1.337-4a46d1231473b599c699160132fd6e76a5c46406`, rooted at `/nix/store/y63xkr8pk2bqd5lh4889rlwldw26v9f4-k-7.1.337-4a46d1231473b599c699160132fd6e76a5c46406`.

### Position on the prior diagnosis

This proposal agrees with `k-failure-diagnosis.md` on the central finding. Excessive native recursive traversal depth caused by transport amplification is the strongest current hypothesis and could exhaust the process stack, but a stack-limit diagnosis is not proved. Each `Option` becomes `ejArray(ejCons(ejString("Option"),ejCons(T,ejNil())))`, adding roughly three K term layers and six JSON object/array layers per token; trace-106 has 5947 such tokens. `ConstructNone`, `exTypeShape`, and `exResolve` are supported, so this is not an unsupported constructor and a parser/grammar gap is not the leading hypothesis.

The prior diagnosis already says that the failure moved from `kore-expand-macros` to the LLVM interpreter when macro expansion was disabled. This proposal agrees and makes the stage distinction explicit: parse04 is the macro-expansion crash; macro05 is a later interpreter crash. Macro bypass alone has failed and is not proposed again.

The prior document’s smallest discriminator remains **specified-only and not executed**. After the required audits and root admission, it would use the exact macro05 input, parser, artifact, and argv in one bounded diagnostic, without compiling or raising limits. Native traversal frames near stack exhaustion, semantic-evaluation frames, or another failure location would support different repairs. Its result is unknown, so every repair option below is contingent on evidence that does not yet exist.

The refinement added here is CLM-0524’s independent stage-level corroboration of parse04, together with a sharper limit on what that corroboration says about macro05.

## Upstream relation

CLM-0521 records the k-rust project’s canonical `runtimeverification/k` reference as release `v7.1.337`, commit `4a46d1231473b599c699160132fd6e76a5c46406`. Moriarty’s retained Nix package pins that identical release and commit, not merely the same version number.

CLM-0524 transcribes the k-rust exclusion for KEVM `evm-execution`: “The pinned reference execution path fails in kore-expand-macros before producing a configuration, so exact backend comparison cannot reach either executor.” KEVM’s `optimizations.md` and Moriarty trace-106 are different macro-heavy corpora for different definitions, but on the identical canonical K build they fail at the same stage: `kore-expand-macros` before configuration construction.

This is strong, not conclusive, evidence that parse04 belongs to an already documented upstream defect class for this exact K build rather than being a defect newly introduced by `expression-v1.k` or Moriarty’s transport. It establishes stage-level recurrence across at least two independent macro-heavy inputs. For admission framing, parse04 is at minimum in the exclusion class already documented by CLM-0524; a fresh admission request need not present it as a novel discovery requiring a from-scratch upstream report before it can be treated as grounded.

It does **not** establish a shared mechanism. KEVM may trigger the failure through macro-rule volume or complexity, while Moriarty may trigger it through `ConstructNone` metadata nesting depth. CLM-0524 documents an exclusion, not a root-cause diagnosis. Most importantly, CLM-0524 does not cover macro05: macro05’s SIGSEGV occurs in the interpreter after macro expansion is disabled.

If parse04 is in this stage-level class, the identical canonical `runtimeverification/k` build is already effectively treated by k-rust’s pinned-reference exclusion as unreliable in `kore-expand-macros` for sufficiently large, macro-heavy, or deeply nested inputs. The owner should treat that stage as known-hostile territory on this pin. A fix likely belongs either in a semantic-preserving Moriarty input/transport structure that avoids pathological recursion or in a newer upstream K release not yet pinned here. Task 4.2 calls for working around the class without narrowing the language; the documented exclusion is sufficient grounding for that posture even though it does not identify the common mechanism.

## Options

### Option A — semantic-preserving iterative traversal or non-amplifying transport

After the admitted discriminator localizes the failure, replace the implicated recursive native traversal with an iterative route or adopt a reviewed transport representation that avoids depth amplification. Retain every metadata constructor, the depth-5947/Core-65,536 boundary, and independent K validation. The change requires representation correspondence tests, rebuilt bindings/artifacts, and complete applicable conformance. Confirmation requires the exact retained maximum to produce its complete expected value/type/work result, while depth 5948 still rejects `INPUT_BOUND` and all historical/current conformance remains intact. This option does not narrow the accepted language, but it requires owner approval before implementation.

### Option B — reviewed bounded child-stack increase

Only if the discriminator supports native stack exhaustion, test a finite stack allowance for the single exec-shim child with unchanged input, artifact, memory, CPU, deadline, and language bounds. This is lower-code-cost but carries material resource and portability risk: it could mask recursion, fail to cover the admitted domain safely, or only move the crash. Confirmation must include the complete semantic result, not exit zero alone, a safe bound for the byte-bounded domain, unchanged `INPUT_BOUND` rejection at depth 5948, and full conformance. This option does not narrow the language but requires owner approval. It is explicitly not the primary recommendation.

### Option C — newer pinned K release and upstream engagement

Cross-reference or report the stage-level class upstream as useful, and separately assess whether a newer `runtimeverification/k` release removes both relevant crash sites without semantic drift. Any assessment requires its own review and admission, a complete toolchain re-pin, artifact rebuild, and full requalification. It may fix parse04 while leaving macro05, introduce other semantic/toolchain changes, or provide no fix. Confirmation requires pinned upstream change evidence or admitted execution of the exact retained maximum with full semantics and unchanged rejection boundaries. This option does not narrow the accepted language and requires owner approval.

### Option D — explicit narrowing as a last resort

Changing the accepted metadata boundary or omitting the retained case would supersede task 4.2 and invalidate existing acceptance assumptions. Only evidence that no safe semantic-preserving traversal, representation, resource, or upstream-release route can cover the admitted byte-bounded domain could motivate owner consideration. That evidence does not exist because the smallest discriminator has not run. This option narrows the language, requires explicit owner approval, and is not recommended.

## Recommendation to the owner

Recommend that the owner consider **Option A** as the primary repair candidate because it directly addresses transport/native depth amplification while preserving the accepted language and independent K validation.

This is a recommendation, not a decision or authorization. Before implementing it, obtain fresh Astra-medium and Grok-high audits of this grounded proposal and then root admission for the already-specified bounded smallest-discriminator diagnostic. The diagnostic outcome is unknown. Native-traversal frames would support an iterative/non-amplifying repair; semantic-evaluation frames would require the corresponding iterative semantic repair; another location would require a newly grounded targeted option. Option B remains secondary and becomes eligible only if the discriminator supports stack exhaustion.

Any later repair must accept the retained `ConstructNone` with 5947 nested `Option` metadata at exactly 65,536 Core bytes; keep depth 5948/65,547 bytes rejecting `INPUT_BOUND`; preserve all metadata constructors and independent K typing; and pass correspondence plus complete applicable conformance.

## Open questions

1. Do parse04’s `kore-expand-macros` crash and macro05’s interpreter crash share a root cause, or are they independent defects triggered by the same oversized/deeply nested input? This remains unresolved without the smallest discriminator.
2. Is CLM-0524’s stage-level corroboration on the identical K release and commit sufficient to treat parse04 as an already documented upstream exclusion class for admission, or do reviewers require a separate `runtimeverification/k` bug report?
3. Who authorizes the already-specified, not-yet-executed smallest discriminator, and under which obligation, given that it is itself a bounded retry variant of the frozen case?
4. Can Option A be implemented and correspondence-tested entirely in the transport/codec/native traversal without modifying `formal/k/expression-v1.k`, or does task 4.2 also constrain which K semantics files may change?
5. Should K `v7.1.337` at commit `4a46d1231473b599c699160132fd6e76a5c46406` be reconsidered against newer upstream releases that may not exhibit this `kore-expand-macros` behavior, independently of a Moriarty-side fix?
6. What exact content and acceptance threshold must the fresh Astra-medium and Grok-high audits meet before root admission may be requested for a diagnostic retry?

## Evidence pins

All hashes in this table were recomputed from the named retained bytes. The external k-rust clone was at HEAD `687ccd01e4708c495358c834c3685f297ecc8ac6` when its source was read.

| Path | SHA-256 |
| --- | --- |
| `.moriarty-dev/k-macro05-trace106-diagnostic.py` | `edf6e965b7a3d0c8bcde12890b20862d23c9e1f46dcf78d6b3606a73f4d25e9f` |
| `.moriarty-dev/k-macro05-trace106-pins.json` | `bf29e8630b21726198a6bdb93613567236cb7c2460f83dd1c5b76644e4563bf7` |
| `.moriarty-dev/k-macro05-trace106-requisites.json` | `ec4998d7dee6d2f1df15f4ed9314e5948d6a984b40ce5df3eae07b2dd6919b0e` |
| `.moriarty-dev/k-macro05-trace106-k-package-map.json` | `195ebcab58cb73cd5906a996156dbfdba72e458fa68396044ca16492212b7140` |
| `.moriarty-dev/k-macro05-trace106-ownership.json` | `c1780f8f78de547043923431955c3c330e6ced5dc03de10d9b4def8b533e373a` |
| `.moriarty-dev/test_k_macro05_trace106_diagnostic.py` | `10ef821157c9d0025b418b3a65a6717c3d012d6ebcc8534130ff5a91c42fdd08` |
| `.moriarty-dev/k-macro05-trace106-host-evidence-path.txt` | `6b92efedfae56758fb70da5fb33e9e7c9398ddca056c2fec331240a1b911a292` |
| `wiki/zkir/midnight-k-tooling.md` | `ba8b0ae47a066c3730fe72a7fa572998db587e16b457ba174fac342bb22627c4` |
| `openspec/changes/language-to-ledger-lifecycle/tasks.md` | `e08d279c61de767cf68954e4ede1a36babb230851618563062b9335c66c8b79f` |
| `deliverables/language-to-ledger-2026-09-12/design-review/k-failure-diagnosis.md` | `153619948f21c9a957b7461f6bd5bc15893e5757db3940377f60b80a76e6cd42` |
| `deliverables/language-to-ledger-2026-09-12/design-review/lifecycle-k-expectations.md` | `fe723b5b44abd380e925234dda12161ce1237a6a6c1f11651cd126f7a0fd7783` |
| `deliverables/language-to-ledger-2026-09-12/k-admission/root-exec-shim-freeze-04.json` | `9f06aaf1ace8e456b76bd38b0d30c64db35e158d0b0e35ba31c8d56168b0ac03` |
| `/home/charl/Moriarty/.worktrees/sp03-expression-k-macro05/experiments/moriarty-language/formal/k/.build-expression-v1/trace-106.stderr` | `5a5ef35093ea3f40495a70a519233de90f3108670e226132d8150ed7c277ff21` |
| `/home/charl/Moriarty/.worktrees/sp03-expression-k-macro05/experiments/moriarty-language/formal/k/.build-expression-v1/trace-106.command.json` | `22662c12bc58ac6c5bad58bc3e2f2fed50c73e73ccca0611f4f43826e6dd7f32` |
| `/home/charl/Moriarty/.worktrees/sp03-expression-k-macro05/experiments/moriarty-language/formal/k/.build-expression-v1/binding.json` | `4c663a141042a0fd4ea2be1b00e95c0ea4c5b9e744cd940e0d5352feaafbbb27` |
| `/home/charl/Moriarty/.worktrees/sp03-expression-k-macro05/experiments/moriarty-language/formal/k/.build-expression-v1/trace-106.input` | `6c29dc49a828956f5056605b672b519a515129383c7dd46b0dc34bba174c3c1b` |
| `/home/charl/Moriarty/.worktrees/sp03-expression-k-macro05/experiments/moriarty-language/formal/k/expression-parser.py` | `8420d151ea9e510b96fc941e73f496b27d53e1e1913e3d270b8ce4f4c0701f1f` |
| `/home/charl/Moriarty/.worktrees/sp03-expression-k-macro05/experiments/moriarty-language/formal/k/expression_codec.py` | `9d8f32cfa778e246900873aa5c015b45ab1ae8bec067fb26709f4a2e2996590d` |
| `/home/charl/Moriarty/.worktrees/sp03-expression-k-macro05/experiments/moriarty-language/formal/k/run.py` | `9adc7dcf0a66f9fdf9d754906f65df7ca1e64e0f10d0de360384c28c35ed4a79` |
| `/home/charl/Moriarty/repos/midnightntwrk/k-rust/scripts/reference-differential.toml` | `937d88be618ee045bcd07cbf7251359e229320fd28d3175a48c59b78acc9d995` |

## Not authorised

This document does not retry the K run, grant or request admission, or satisfy obligation 127 by itself. It provides only the grounded change/proposal prerequisite. Fresh Astra-medium and Grok-high audits and subsequent root admission remain required and were not attempted here.
