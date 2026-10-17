# First substantial compiler batch — frozen proposal for final review

Status: DRAFT additive B04 contract, 2026-10-17. Required v1 scope is unchanged. This batch implements a deliberately narrow public source-to-admitted-host-program route, not constraints, native compilation or financial effects. Root must bind final integrated source plus current dirty QA afterimages; baseline HEAD alone is insufficient. Final03 and the receipt bindings beside this contract record241 ordinary tests, stream8/216 and actual333 comparisons; PR168 merged at011284561884346412a7c7eaa36a195310924878. No rerun occurred in this drafting lane, and generic continuity/native/target acceptance remains open.

## Exact proposed command surface

Extend existing `mori compile FILE --target TARGET --out NEWDIR` with:

- `--execution-profile sherlock-helper-microstep/1`: select the existing B host consumer. The selector omitted preserves current Core /1 and /2 compile behavior and artifact bytes. Recognize `sherlock-k-a/1` and `sherlock-quint-a/1` as unsupported compiler selections in this first batch, rather than silently selecting B. Later A support is a separate batch.
- `--action NAME`: required with execution-profile; exact source action name, no numeric/name fallback. Public Core source has actions, not source function syntax. The prepared wrapper function is an implementation representation, not advertised source-function support.
- `--execution-config FILE`: required with execution-profile; exact JSON contract below, no numeric defaults.
- `--observe`: optional, explicit request for existing bounded B host execution after successful preparation/admission. Without it, evaluation_performed=false and no run is launched. With it, evaluation_performed=true only after a real run starts; all existing H1/command classification/owner controls still apply. The compile-only and observe modes have separate observed predicates.

No input/scenario flag in this batch. Selected action must declare ZERO inputs; the exact map supplied to the existing consumer is an empty `BTreeMap<String, crate::Value>`. Any action parameter is named unsupported before attempting to replace it with a default/literal. State/assets, finance, /2, recursion/source functions, generic ADTs and non-scalar values remain explicitly unsupported on this narrow selected route. Existing ordinary Core compiler support is unaffected.

Selected Sherlock mode branches BEFORE calling backend::source_projection::compile_reported / stage::compile_staged; it does not emit legacy artifact.json or .zkir files. `--target` remains required by existing Args but is a supplied descriptor only; it does not create HostOnly native compatibility and cannot invoke SDK operations. Report its exact bytes/source identity using existing repository file digest conventions. Missing target or unreadable config is an input error, not an admitted source artifact. Input/source parse/validation errors retain the actual existing Core diagnostic.

## Real production types and call path

Production module roots publicly register `sherlock::helper_machine::core_action_consumer` and `source_input_bridge`; their old unregistered/cfg(test) file comments are stale. Do not create duplicate modules. Reuse `crate::parse`/`validate`, `sherlock_source::prepare_scalar_action`, `ScalarActionMap {function, report: ScalarActionReport}`, real `sherlock::Program`, `source_input_bridge::from_program`, fixed `host_owner_engine()` and its `OwnerEngine::admit`/`admit_run`. Actual B types are `Admitted`, `MachineImage`, `ProgramView`, `ConsumerConfig`, `SourceHostBounds`, `RunSummary` and owner-bound `HostObservation`; no type named BoundedProgramImage was found in this path. HostOnly is not a bounded native image.

Extract/reuse the existing nonsecurity orchestration in core_action_consumer into a public preparation helper returning an owner-bound admitted pair plus unchanged preparation report, without replacing or weakening the existing security-owned preflight/generated-AST/config checks. `run_scalar_action` then consumes that same preparation helper; the CLI observe branch calls that existing run consumer with the empty input map. The helper prepares and admits actual source shape; it never executes Core to get a result. Entry is the real zero-argument Call to the generated function, whose body contains source lets/operators. Observations serialize the actual owner-bound view, not reconstructed expected tables. In --observe mode serialize the pair returned by that actual run; never combine a first preparation token with a second independently admitted image. Compile-only serializes its own prepared/admit_run pair without running it. Applicable security review covers affected composition; this contract authors no replacement guard or security policy.

## execution-config JSON: `sherlock-host-config/1`

All listed keys are REQUIRED; unrecognized keys, invalid enum tags and missing keys yield a named config parse error. This is a new serde/wire adapter to real types, not a claim those types currently implement serde. Numeric storage is decimal strings, matching `^(0|[1-9][0-9]*)$`; checked conversion refuses values outside the actual target type (usize or u128), with no truncation/default/clamping. Layout bounds are a SINGLE shared object; no duplicated divergent bound copy is supplied.

```text
{
 format: "sherlock-host-config/1",
 profile: "sherlock-helper-microstep/1",
 grammar: "DedicatedSorts", finance: "Absent",
 bounds: {
  code_rows,function_rows,code_child_slots,parameter_slots,pattern_rows,
  match_plan_rows,match_arm_rows,type_rows,constructor_rows,constructor_field_slots,
  max_arity,constant_nodes,constant_field_slots,data_nodes,data_field_slots,
  environment_cells,frames,helper_tasks,serialization_bytes: usize_decimal,
  max_reference_work,max_helper_work,step_cap: u128_decimal
 },
 layout: {
  layout_version: [string,string],
  field_id: null, widths: null, scalar_layout_id: null,
  capacities: {name_rows,name_bytes,occurrence_path_slots,value_slots: usize_decimal},
  boundary_capacities_present: false
 },
 fuel: {kind:"Nat",value:u128_decimal} | {kind:"Negative"} | {kind:"NatAboveU128"},
 stack: same RunIntC tagged union,
 host: {
  max_source_actions,max_source_expressions,max_source_statements,
  max_action_inputs,max_supplied_inputs,max_projection_occurrences,
  max_generated_ast_nodes,max_ast_nesting,max_input_name_bytes,
  max_total_input_name_bytes: usize_decimal
 }
}
```

Map exactly to existing `ConsumerConfig`: profile=SherlockHelperMicrostep1, grammar=DedicatedSorts, finance=Absent; layout.bounds=bounds; raw RunIntC classes preserve actual owner dispositions, including Negative/NatAboveU128. Null fields select the current HostOnly layout, never a fabricated field. Existing `validate_config`/admission determines valid host extents/capacities; all host extents remain caller-stated and positive by its existing rule. This wire schema adds no accepted numerical policy profile or enlarged limit. No runtime inputs can hide in this file.

## Output artifact JSON: `sherlock-host-artifact/1`

Write `sherlock-artifact.json` to the requested NEWDIR through the existing publish-new contract only after all selected mode work and serialization succeeds; refuse preexisting output as existing publication does. This is an ordinary host preparation/report artifact, not ZKIR, a proof input, a canonical checkpoint ABI or an installable contract. Reopening it as any of those formats must be unsupported.

Required top-level fields (no placeholder artifacts):

```text
format = "sherlock-host-artifact/1"
qualification = "host-only-scalar-value-projection; no effects/constraints/native/target qualification"
source = {language, semantics, agreement, action, action_index, source_digest}
execution_profile = "sherlock-helper-microstep/1"
input_policy = {kind:"parameter-free", declared:[], supplied:[]}
config = exact supplied config fields plus exact file digest
preparation = unchanged ScalarActionReport fields
admission = {status:"admitted", representation:"HostOnly", identity: actual IdentityC fields,
             program_view: actual ProgramView wire projection, layout: actual LayoutObservation}
plans = {
 witness: {status:"not-implemented", artifact:null, reason:"no exported WitnessPlan in this batch"},
 constraint: {status:"not-implemented", artifact:null, reason:"no Sherlock constraint emission in this batch"},
 export: {status:"unsupported", artifact:null, reason:"HostOnly source route has no native exporter"}
}
observation = {status:"not-requested", evaluation_performed:false}
           | {status:"observed", evaluation_performed:true,
              summary: actual RunSummary, events_forwarded: actual counter,
              image: actual complete owner-bound ImageObservation wire projection,
              resource_report: actual CoreActionRun::resource_report}
           | {status:"failed", evaluation_performed:true, stage, actual_error,
              events_forwarded: actual counter, completed:false}
evidence = {source_preparation,host_observation,native:"unrun",target:"unrun",acceptance:"none"}
```

For failed/refused preparation/admission the command returns nonzero and emits a structured diagnostic on stderr with stage (`parse|select|config|source-projection|host-guard|admit|admit-run|run|observation|publish`), unchanged actual diagnostic payload and available source/action/statement/expression locator. It publishes no successful artifact. A semantically completed Rejected/Exhausted owner run is still an observed owner outcome and is not relabelled as a tooling failure or success-valued computation. A partial Run failure must not fabricate a completed summary; keep forwarded event count and original failure evidence. Publication behavior for a run failure is diagnostic-only, no artifact, so the failed observation alternative above describes the diagnostic payload, not a successful file.

Wire rule for all actual owner records (excluding private zero-sized `_sealed` construction markers, dereferencing Arc ownership): struct keys use exact Rust field names; enum values are externally tagged by exact Rust variant name, tuple positional order retained; Option=None is null, Some(x) is x; all usize/u128/i64 counters/IDs use exact decimal strings, bool remains JSON boolean; vectors retain order and multiplicity. `IdentityC`, `ProgramView`, its expression/function/parameter/occurrence/constant rows, `LayoutObservation`, `RunSummary`, and `ImageObservation` cover EVERY actual field defined at the frozen source pins; no Debug-string record substitutes for typed output. First-batch supported ProgramView uses scalar Literal/Var/Let/Binary/Unary/If/zero-argument Call; match/constructor tables remain empty and are still serialized as empty arrays. Source/shared enum/tag schemas must be generated or individually inventoried against those exact pins before authoring; a changed owner type invalidates this schema candidate. Existing DigestC/identity records are serialized as their actual typed values, not redefined as authenticity claims. `preparation` and `resource_report` retain their existing serde formats unchanged, including their existing numeric conventions; the new owner-record wire rule does not rewrite them. Resource unmeasured fields remain unmeasured. Source/config/target file digests use existing `mori_core::intent::hash_bytes` as source identity metadata, not native authenticity.

## Supported grammar rules and refusal boundaries

Reuse exact current `prepare_scalar_action` support, further narrowing only the explicitly selected first host route to zero inputs: language moriarty/1, semantics moriarty-core/1, no states/assets; ordered lets and exactly one final emit value; UInt/Bool literals, bound names, Not; Add/Sub/Mul/Eq/Lt direct; Ne/Ge via existing Not(Eq/Lt); Gt/Le/And/Or via current hygienic eager lowering in source_adapter.rs. Preserve source left-to-right evaluation and eager boolean semantics. Keep generated lowering/source-occurrence fields intact. The emit's label/position/role is metadata; its ordered financial/Core effect is NOT executed here. Every unsupported construct reports its actual earliest available source locator. Core validation remains prerequisite and unchanged.

## Two contrasting new positive source programs

New parameter-free specimens (not existing weighted/threshold fixtures):

```mori
language "moriarty/1";
agreement FixedQuote {
  action quote() {
    let base = 7 * 11;
    let total = base + 5;
    emit "quoted"(total);
  }
}
```

Independent mathematical value: UInt82. Prepared/admitted function body must retain two lets, Mul and Add; it must not be Literal82.

```mori
language "moriarty/1";
agreement FixedEligibility {
  action inspect() {
    let low = 3 < 9;
    let high = 9 > 3;
    let eligible = low and high;
    emit "eligible"(eligible);
  }
}
```

Independent mathematical value: Booltrue. Actual lowered function must retain Lt, hygienic eager Gt/And lowering, lets and source occurrence mapping. Only one compiler path handles both; fixture names/source digests cannot select implementation branches. Non-author O must freeze complete admitted tables and selected full B event/image/work expectations before implementation execution; these value literals alone do not establish complete-prefix or financial correctness. Existing accepted source/compiler regression fixtures remain unchanged.

## Falsifiable batch exits and exact owners

Integrator I alone edits `crates/mori/src/commands/compile.rs`, `crates/mori/src/cli.rs` if needed, shared public exports and wire artifact module registration. One compiler owner P edits a NEW ordinary wire/report leaf module plus nonsecurity core_action_consumer preparation factoring; existing security-owned module is read-only to P. Oracle O alone writes the two new specimen/expected-record fixtures. QA T writes ordinary CLI/wire regressions on disjoint files after I freezes helper membership. Docs D updates affected command reference/how-to/example/profile/limitation pages. I binds final candidate/import closure; fresh applicable non-author reviews cover source and actual results. No ABI/dispatcher writes by leaf owners.

Compile-only exit: actual CLI prepares both specimens; artifact actual owner ProgramView preserves source operators and zero-argument call; evaluation_performed=false; witness/constraint/export statuses above; no legacy ZKIR/native operation entered. Observe exit: under the existing designated-owner H1 disposition and inspected ordinary closure, both real run_scalar_action results match independent typed table/event/image expectations, yielding82 andtrue. Contract/context/native/resource gates are not changed. Cross-profile A/B clock equality and native target acceptance remain unclaimed.

Required ordinary regressions: omitted selector reproduces old Core compile artifact bytes on its existing fixture under existing admissible check scope; parameterized selected action is unsupported with no manufactured default; state/assets/finance,/2, unknown action/profile, bad/missing config/overflow/truncated decimal and existing destination have exact named diagnostics; bool path retains eager operand order; complete typed output/member presence; compile-only never invokes run; --observe reaches actual fixed B engine; meaningful wrong normal output fails the independent comparison. Checks must be inspected for transitive SDK calls before execution; no blanket Cargo/native test permission is inferred.

Wake/dependency: canonical acceptance of final03 and this exact schema plus current H1 scope disposition allow eligible implementation; missing native0/C4/R2/60460 facts do not block this SDK-free source preparation. Existing native gates still block their downstream actions. No hidden hard-blocker resolution driver or new scheduler is added.
