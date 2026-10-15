# Final nonsecurity continuation contract

Sol **unconditionally agrees with C1–C10 as amended by DESIGN-OPUS-CLARIFICATION.md**. This is the second substantive language/compiler vote; it adds no design condition. Implementation still requires the actual same-packet Opus final signoff, fresh Muse design audit and root dispatch. Opus owns all security requirements/code/tests/judgments. DESIGN-OPUS-FINAL.md's forthcoming exact backend DTO/context/frame interfaces are incorporated verbatim when delivered, not supplied by Sol. No implementation or execution occurred.

This document supersedes conflicting earlier Sol plan language. MF1/3/5 are resolved by Opus's clarification, not by Sol interpretations. MF4's exact security artifact/context contract belongs to Opus's final amendment. The following fixes the nonsecurity parts of MF1–MF9.

## Compiler stack and compatibility freeze

The selected public path is `commands/compile.rs → mori_backend::compile_stages → stage/lower.rs → existing Builder/lower_action machinery → ZKIRv3`. Correct Opus clarification §3's module locations to **`crates/mori-backend/src/stage/{context,frame,artifact,lower}.rs`**: context/frame/artifact are Opus-authored; raw lower is Sol-authored and consumes frame's API. `mori-ledger` gets only required refusal/source-closure hunks; its native-call `relation/compiler.rs` is not this stage compiler. Root must obtain Opus signoff on these corrected paths in the same final packet.

Source facts: backend `lib.rs::digest` hashes `intent::canonical(value)`. Its program hash hashes Program; `signed_relation.rs::context_hash` retains its existing ordered18-element preimage (versions, registry, Program, Target, action, public projection, financial frames, replay steps). These backend files and backend `proving.rs` have **no** `source_closure`, `compiler_hash` or `compiler_id` dependency. Thus the closure-regeneration whitelist for backend `/3 artifact.json` paths, derived digests/context and `.zkir` bytes is **empty**. Require exact legacy artifact/instruction/role/operation bytes; do not regenerate goldens. This corrects earlier Sol speculation about backend hash churn.

Separately, `mori-ledger/src/proving.rs:150` manually enumerates `include_str!` in source_closure; ledger build/compiler-hash-bearing records can refresh under Opus §1.2. That is not a `/3` backend exception. Opus owns closure inclusion, exclusions and tests.

## `/1` wire and validator split

Keep legacy `validate(&Program)` `/1`-only. Add `validate_source(&Program)` dispatching source/profile validation; parser calls it, and staged APIs call it plus Opus `require_core2`. Legacy APIs retain `require_core1` before evaluation/layout. Opus authors acceptance helpers; Sol integrates calls without duplicating their predicates.

Freeze `ir::v1` Program/Action/Statement derives verbatim, and nested Step/Execution wire decoding, with conversions into shared in-memory types. Preserve all existing non-Await serialization. Shared Await is source2/in-memory, not an extension of legacy JSON decoding. Every legacy public Deserialize path for Program, Action, Statement, Step and Execution must route through the frozen wire, including generic serde calls, rather than relying only on CLI profile checks. Opus owns this wire hunk.

Decode inventory: direct Program IR JSON (`mori-core/tests/json_export.rs` and external serde callers); standalone/nested Step/Execution; backend `replay.rs::ReplayRecord::from_json` (`ReplayBody.execution`); backend `proving.rs` execution-bearing receipt/proof DTOs; generic `core_io::read_json<T>` at their callers. Source `core_io::program` parses text, not Program JSON. Core replay functions accept in-memory Execution; their callers inherit its frozen decoder. Legacy Scenario/Snapshot/Type/Value fields remain unchanged. No staged output decoder may bypass this rule by deriving Deserialize over shared Await-containing Statement. Inventory and H23 fixtures are delivered to Opus for its compatibility hunk.

## Shared evaluator and match closure

Sol exposes exactly one non-admitting runner:

```text
run_segment(program, plan, action, entry_site, seed_env,
            boundary: Snapshot, meter)
  -> Result<(Execution, exit_site, out_env), Error>
```

Opus `stage/admit.rs` validates and wraps the runner atomically. Opus owns `stage/{validate,digest,cost,profile,decode,admit}.rs`; Sol owns `stage/plan.rs`, parser grammar and shared evaluator extraction. Root/Sol as designated integrator alone applies shared-file hunks verbatim and presents their final diff to Opus. No second interpreter or local consumption registry.

Exhaustive/wildcard checklist: Core runtime typing/execution/duty collectors; backend lib touched-key scan/emitter; backend proving statement scans; backend signed_relation label scan; ledger ops step-root; ledger relation/mod root/provenance; ledger relation/compiler both root/emitter matches; ledger signed/mod statement admission; ledger layout admit_bindings_inner wildcard; CLI inspect statement_rule/expression_constraints. Unsupported consumers get explicit typed refusal behind Opus's `/1` gate; source-only roots select guard. Also gate Core intent/sponsor/replay/logical wrappers, backend replay/proving, ledger static/layout/local-deployment/signed preparation/context. Preserve legacy error wrappers; Opus pins required error identity per entry.

## Exact steps, work and fixtures

Reached exit-await charges entry1 and stops **before guard and without an exit Await Step**. Start Execution contains prefix steps only. Resume charges dispatch1 once, evaluates guard with a fresh per-statement cache, and on true emits Await Step at original ordinal with Bool(true), pre snapshot and no effects. That dispatch is the Await statement charge: no second statement charge. Segment Execution.semantics is `moriarty-core/2`. False yields no segment, unchanged canonical continuation/residual, attempted-only cost. Ordinary segment rejection publishes no effects/continuation.

N awaits yield N+1 circuits; zero-await actions yield one s0/0→0 circuit and Complete/empty frame. Exact PendingFrame/StageSegment and None-cap rules are Opus clarification §2, accepted verbatim. Static costs count distinct expression DAG nodes per statement; both operands are eager. Total-cap preflight and committed constants are Opus-owned, not mutable counter ABI.

Keep distinct oracles: one-await ReserveAllocation remains costs[13,20], committed33, captured funded12, final balances88/0/12/7 and reserve12/released15. Delivery is[5,6], committed11; false attempted2 retains committed5. Root's separate **two-await reserve** is[11,15,10], committed36; do not replace or conflate fixtures. A false observation never increases committed work.

## CLI and JSON contract

| Invocation | Arguments |
|---|---|
| Start | `simulate FILE --action A --scenario S [--logical-cap C]` |
| Resume | `simulate FILE --resume C --event E [--logical-cap C]` |
| Resume with action | Optional `--action A` must equal Continuation.action, else stage_site |

Replace only simulate's flattened ActionInput with command-local args. Preserve legacy missing-required-argument behavior and stdout bytes. Opus supplies pre-read argument-conflict hunk: resume requires event and conflicts with scenario/trace/target/authorization; event without resume is usage error. Shared ActionInput elsewhere stays unchanged.

Legacy read_json/parse remains1MiB. Opus provides exact staged strict-reader helpers for4MiB Continuation and64KiB StageEvent, duplicate rejection and unsorted-boundary rejection; do not raise legacy limits or sort resume frames. Resume reads Continuation only, never prior history/stdout. Every `/2` object has qualification=local-reference-stage, authentication=none, consumption=untracked. Waiting/Complete/false exit0; rejected1. Complete never says settled. Residual lists positive post liabilities; false uses unchanged boundary.

Final backend wire types/signatures, context order, gates, Output treatment and resource accounting come only from Opus's final amendment. This is the sole remaining security-interface delivery, not another Sol design choice. All full roadmap exits remain open.
