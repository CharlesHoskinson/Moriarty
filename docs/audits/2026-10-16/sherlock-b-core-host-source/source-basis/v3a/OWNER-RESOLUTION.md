# OWNER RESOLUTION — PROPOSED, SOURCE-ONLY, NOT ADOPTED

Top finding: the stored helper-origin is already decided; only the StepEvent observation side was ambiguous. The patch below decides that side, fixes `StepEventC: Copy`, and fixes the foreign-rule rendering to one literal. No trace, expected count, or implementation choice was supplied or requested from the two normal-oracle workers.

Status: proposed clarification only. Fresh scoped agreements and Root review are required before Root adoption. After adoption, O authors expectations independently. Native authority remains 0; C4 and all counters/charges remain unchanged; nothing was compiled, executed, or tested in this adjudication.

## (a) Existing source versus NEW decision

### Existing source already decides

- Stored value: every `.entry` row installs `originReferenceCursor: pre.referenceCursor`, e.g. the Scalar entry effect in [rows-supplement.json](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/b-phase0-convergence/review-candidate-r4/rows-supplement.json).
- [schema-supplement.json](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/b-phase0-convergence/review-candidate-r4/schema-supplement.json) SI-08: `originReferenceCursor = referenceCursor` while a helper is present.
- Cursor advancement: P7 says `referenceCursor += 1 iff E.eventKind = LegacyComplete`; SI-04 says `referenceCursor = consumedWork - helperWork`.
- Phase fields already literally say `pre.helper` and `post.helper`: `helperPhaseBefore` is `Opt<phase or status of pre.helper>` and `helperPhaseAfter` is `Opt<phase or status of post.helper>`.
- Result staging: `HelperResult` is `present only between a helper exit or failure and the legacy completion that consumes it`; SI-09 requires helper absent when result is present.
- P1 requires `pre.helper absent and pre.result absent`. P2 retains `helper, result and tasks`. P0 emits no event; Startup emits no StepEvent.

### Genuinely ambiguous in existing source

- [schema-supplement.json](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/b-phase0-convergence/review-candidate-r4/schema-supplement.json) `step_event/fields/originReferenceCursor` says only `Opt<ReferenceCursor> of the live helper`, with no entry/exit/legacy/Publish side.
- `helperKind: Opt<HelperKind>` does not say whether a legacy completion consuming a staged result reports the producer interval kind, a live-helper kind, or none.

### NEW normative decision

Separate the five fields:

- `helperKind`: interval kind. Pre-helper kind if present; else post-helper kind for an applied entry; else, for a staged-result legacy completion, `helperPlan(site(pre),pre)`; else, for a refused helper entry at P3/P6, the Selected attempted kind; else absent.
- `helperPhaseBefore/After`: strictly `pre.helper` / `post.helper`; therefore entry has absent Before, exit/failure has absent After, and staged-result legacy completion has absent/absent.
- `helperCursorBefore/After`: strictly the mapped cursor of `pre.helper` / `post.helper`; staged-result legacy completion has absent/absent. `Some(0)` remains distinct from absent.
- `originReferenceCursor`: pre-helper origin if present; else post-helper origin on applied entry; else `pre.referenceCursor` when `pre.result` is present or a helper entry is refused at P3/P6; else absent. For a staged result, `pre.referenceCursor` equals the producer helper’s stored origin because no LegacyComplete can intervene between its entry and consumption, and only LegacyComplete advances the cursor.
- Publish: P1 is absent. P2 uses the retained pre-helper or staged-result branch above; otherwise absent. P0 and Startup have no StepEvent, not a null-filled event.

For Q3, the sole foreign-family rendering is `WrongVariant`, clause `None`, rule `None`, locator `Some("foreign rule for family")`. This follows the two concordant V3-03/V3-17 dispositions and the `HostViolationKind` comment in [shared.proposed.rs](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/b-phase0-convergence/shared-rust-contract-candidate-v3/shared.proposed.rs). The generic RuleIdC comment saying to copy into `HostViolation.rule` is superseded for this case. The same literal applies to private family free functions.

## (b) Exact patch artifact

```json
{
  "artifact": "OWNER-RESOLUTION-event-side-and-v3-errata-PROPOSED_UNADOPTED_SOURCE_ONLY",
  "governance": {
    "native_authority": 0,
    "c4_unchanged": true,
    "adopted": false,
    "execution_claims": []
  },
  "operations": [
    {
      "target": "review-candidate-r4/schema-supplement.json",
      "operation": "replace",
      "pointer": "/step_event/fields/helperKind",
      "old": "Opt<HelperKind>",
      "new": "Opt<HelperKind>: interval kind, distinct from live-helper presence. Source in priority order: (1) kind of pre.helper if present; (2) kind of post.helper if present (Applied HelperEnter); (3) if pre.result is present, helperPlan(site(pre), pre), which is Some for every consuming site in site_result_sets (Applied LegacyComplete consuming a staged result); (4) if the Selected row is HelperEnter/HelperVisit/HelperExit/HelperFail/Bookkeeping under an Applied, RefusedHelperWorkUncharged or RefusedCapCharged disposition, Selected.helperKind (covers a P3/P6-refused entry with no helper installed); (5) otherwise absent. Publish uses branches (1) or (3) for a retained helper/result and is otherwise absent; a helper-free LegacyComplete is absent."
    },
    {
      "target": "review-candidate-r4/schema-supplement.json",
      "operation": "replace",
      "pointer": "/step_event/fields/helperPhaseBefore",
      "old": "Opt<phase or status of pre.helper> (catalog phase)",
      "new": "Opt<phase or status of pre.helper> (catalog phase): present iff pre.helper is present. Scalar maps to its ScalarPhase; EnvGet, EnvPut, Find, Validate and Equal map to TriStatus; Bind maps to BindPhase; Append maps to AppendPhase; Match maps to MatchPhase. Absent for HelperEnter, for LegacyComplete including staged-result consumption, and whenever pre.helper is absent. Never sourced from post.helper or from a staged result."
    },
    {
      "target": "review-candidate-r4/schema-supplement.json",
      "operation": "replace",
      "pointer": "/step_event/fields/helperPhaseAfter",
      "old": "Opt<phase or status of post.helper>",
      "new": "Opt<phase or status of post.helper>: present iff post.helper is present, using the same per-kind phase/status mapping as helperPhaseBefore. Absent for Applied HelperExit and HelperFail (helper cleared), for LegacyComplete including staged-result consumption, and whenever post.helper is absent. Never sourced from pre.helper or from a staged result."
    },
    {
      "target": "review-candidate-r4/schema-supplement.json",
      "operation": "replace",
      "pointer": "/step_event/fields/helperCursorBefore",
      "old": "Opt<Nat> (catalog cursorBefore): Scalar phase ordinal; EnvGet cursor index (absent if None); EnvPut newRoot; Find rowCursor; Bind pCursor; Append srcCursor; Validate and Equal taskLive; Match fieldIdx",
      "new": "Opt<Nat> (catalog cursorBefore), sourced only from pre.helper: Scalar phase ordinal in declaration order (First=0, Second=1, Done=2); EnvGet cursor index (absent if its cursor is absent); EnvPut newRoot (absent if absent); Find rowCursor; Bind pCursor; Append srcCursor; Validate and Equal pre.taskLive; Match fieldIdx. Some(0) is a present zero and is distinct from absent. Absent if pre.helper is absent (including LegacyComplete consuming a staged result) or if the EnvGet/EnvPut mapped option is absent. Never sourced from post.helper or from a staged result."
    },
    {
      "target": "review-candidate-r4/schema-supplement.json",
      "operation": "replace",
      "pointer": "/step_event/fields/helperCursorAfter",
      "old": "the same quantity on the post-image",
      "new": "Opt<Nat>: the same per-kind quantity sourced only from post.helper, with Validate and Equal using post.taskLive. Absent if post.helper is absent (including Applied HelperExit, HelperFail and LegacyComplete) or if the EnvGet/EnvPut mapped option is absent. Never sourced from pre.helper or from a staged result."
    },
    {
      "target": "review-candidate-r4/schema-supplement.json",
      "operation": "replace",
      "pointer": "/step_event/fields/originReferenceCursor",
      "old": "Opt<ReferenceCursor> of the live helper",
      "new": "Opt<ReferenceCursor>: interval origin, sourced in priority order: (1) pre.helper.originReferenceCursor if pre.helper is present; (2) post.helper.originReferenceCursor if post.helper is present (Applied HelperEnter; equals pre.referenceCursor by the .entry effects); (3) pre.referenceCursor if pre.result is present, or if the Selected row is a .entry row refused at P3 or P6 with no helper installed; (4) otherwise absent. In case (3), pre.referenceCursor equals the producer helper's stored origin because referenceCursor advances only on an Applied LegacyComplete (P7, SI-04) and no LegacyComplete intervenes between the staging HelperExit/HelperFail and the consuming LegacyComplete or a Publish event retaining the result. P1 Publish and a helper-free, result-free LegacyComplete or Publish are absent. P0 and Startup emit no StepEvent."
    },
    {
      "target": "shared-rust-contract-candidate-v3/shared.proposed.rs",
      "operation": "unique-text-replace",
      "old": "#[derive(Clone, Copy, PartialEq, Eq, Debug)]\npub struct StepEventC {",
      "new": "#[derive(Clone, PartialEq, Eq, Debug)]\npub struct StepEventC {"
    },
    {
      "target": "shared-rust-contract-candidate-v3/shared.proposed.rs",
      "operation": "unique-text-replace",
      "old": "    // Copy shared rule carrier in R4 RuleId order. Owner alone maps &RawRuleId<->RuleIdC exhaustively; raw pin unchanged. Families match RuleIdC exhaustively and copy it into HostViolation.rule.",
      "new": "    // Copy shared rule carrier in R4 RuleId order. Owner alone maps &RawRuleId<->RuleIdC exhaustively; raw pin unchanged. Families match RuleIdC exhaustively; for a foreign RuleIdC they use the sole foreign-rule rendering at FamilyPlans and do not copy that RuleIdC into HostViolation.rule."
    },
    {
      "target": "shared-rust-contract-candidate-v3/shared.proposed.rs",
      "operation": "unique-text-replace",
      "old": "// Every method receives the authoritative Selected. A method presented a\n// foreign RuleId MUST return HostViolation{ForeignRuleId}. Families never\n// recompute site/helper-plan/dispatch, charges, caps, or refusals.",
      "new": "// Every method receives the authoritative Selected. A method presented a\n// foreign RuleId MUST return HostViolation { kind: HostViolationKind::WrongVariant, clause: None, rule: None, locator: Some(\"foreign rule for family\".to_string()) }.\n// The same literal rendering applies to private family free functions. Families never\n// recompute site/helper-plan/dispatch, charges, caps, or refusals."
    },
    {
      "target": "shared-rust-contract-candidate-v3/shared.proposed.rs",
      "operation": "unique-text-replace",
      "old": "// Foreign RuleIdC for the family: HostViolation{WrongVariant,None,None,Some(\"foreign rule for family\")}.",
      "new": "// Foreign RuleIdC for the family (sole rendering): HostViolation { kind: HostViolationKind::WrongVariant, clause: None, rule: None, locator: Some(\"foreign rule for family\".to_string()) }."
    }
  ]
}
```

The `StepEventC` span removes only `Copy`. `ExpandedStepIdC` holds `Arc<RunIdC>` and `LocatorC` holds `String`/`Vec`, so `Copy` was impossible; all semantic fields, order, and other derives are preserved. No recursive-AST trait is added.

## (c) Gates, unchanged state, limits

- Unchanged: all rows, deltas, guards, effects, charges, dispositions, outcomes, helper-work accounting, cursor advancement, SI-04, the 96 Enabled / 112 total counts, capacities, and core state semantics.
- No new `HostViolationKind`, enum variant, cost, state field, engine body, stub, or API is created.
- This is source-only. The future SDK-free Rust compiler pass by Root is still required to catch any remaining rendering errors; no compilation is claimed here.
- Generic ADT, finance, continuation, nested-helper, boundary, and full-B scope remain mandatory and separate. Actual engine APIs remain future work.
- Coverage limit: this adjudication covers only the listed event fields, the `StepEventC` derive, and the foreign-family rendering. It does not adopt R4 or v3 and creates no oracle expectation.

End tail quoted exactly:

`MUSE_HOST_EVENT_SIDE_ERRATA_FULL_TAIL_20261016`
