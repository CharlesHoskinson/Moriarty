// Exact typed literal port of SAFE/INPUTS.json. No lane vectors or expected data.
use super::*;
pub fn helper_cases() -> Vec<(&'static str, Option<HelperControlC>)> {
    vec![
        ("helper-none", None),
        ("helper-env-get", Some(HelperControlC::EnvGet { common: HelperCommon { site: SiteTag::EvalVar, origin_reference_cursor: 340282366920938463463374607431768211444u128, parent_resume: Some(137) }, binder: 211, cursor: Some(223), found: Some(RefC::Data(227)), status: TriStatus::Positive })),
        ("helper-env-put", Some(HelperControlC::EnvPut { common: HelperCommon { site: SiteTag::RetLet, origin_reference_cursor: 340282366920938463463374607431768211442u128, parent_resume: None }, binder: 229, value: RefC::Const(233), base: Some(239), new_root: Some(241), status: TriStatus::Negative })),
        ("helper-find", Some(HelperControlC::Find { common: HelperCommon { site: SiteTag::Invoke, origin_reference_cursor: 340282366920938463463374607431768211438u128, parent_resume: Some(251) }, function_name: 257, row_cursor: 263, found: Some(269), status: TriStatus::Positive })),
        ("helper-bind", Some(HelperControlC::Bind { common: HelperCommon { site: SiteTag::Enter, origin_reference_cursor: 340282366920938463463374607431768211436u128, parent_resume: None }, function: 271, params: SliceC { start: 277, len: 5 }, args: SliceC { start: 283, len: 7 }, p_cursor: 3, a_cursor: 4, partial: Some(293), dup_cursor: Some(307), phase: BindPhase::UnwindDup, status: BindStatus::Ok })),
        ("helper-append", Some(HelperControlC::Append { common: HelperCommon { site: SiteTag::RetArgs, origin_reference_cursor: 340282366920938463463374607431768211432u128, parent_resume: Some(311) }, src: SliceC { start: 317, len: 5 }, value: RefC::Data(331), src_cursor: 3, dest_start: 337, staged: Some(RefC::Const(347)), phase: AppendPhase::Tail })),
        ("helper-validate", Some(HelperControlC::Validate { common: HelperCommon { site: SiteTag::EvalLiteral, origin_reference_cursor: 340282366920938463463374607431768211426u128, parent_resume: None }, root: RefC::Data(349), status: TriStatus::Negative })),
        ("helper-equal", Some(HelperControlC::Equal { common: HelperCommon { site: SiteTag::EvalBinary, origin_reference_cursor: 340282366920938463463374607431768211424u128, parent_resume: Some(353) }, left: RefC::Const(359), right: RefC::Data(367), status: TriStatus::Positive })),
        ("helper-match", Some(HelperControlC::Match { common: HelperCommon { site: SiteTag::Cases, origin_reference_cursor: 340282366920938463463374607431768211418u128, parent_resume: Some(373) }, scrutinee: RefC::Data(379), arm: 383, saved_env: Some(389), partial: None, pat_tag: PatternTag::PCons, x: 397, y: 401, field_idx: 2, staged: Some(RefC::Data(409)), phase: MatchPhase::Field, status: TriStatus::Pending })),
        ("helper-plan", Some(HelperControlC::Plan { common: HelperCommon { site: SiteTag::Cases, origin_reference_cursor: 340282366920938463463374607431768211414u128, parent_resume: None }, source_match: 419, scrutinee: RefC::Const(421), saved_env: Some(431), partial: Some(433), node: 439, phase: PlanPhase::Bind, after_walk: PlanAfterWalk::Bind, cur: RefC::Data(443), path: SliceC { start: 449, len: 5 }, path_cursor: 3, arm: Some(457), status: TriStatus::Positive })),
        ("helper-build", Some(HelperControlC::Build { common: HelperCommon { site: SiteTag::Materialize, origin_reference_cursor: 340282366920938463463374607431768211412u128, parent_resume: Some(461) }, type_id: TypeIdC(701), ctor_id: CtorIdC(709), src: SliceC { start: 463, len: 5 }, src_cursor: 3, field_start: 467, staged: Some(RefC::Const(479)), node: Some(RefC::Data(487)), phase: BuildPhase::Copy })),
    ]
}
pub fn result_cases() -> Vec<(&'static str, Option<HelperResultC>)> {
    vec![
        ("result-none", None),
        ("result-validated", Some(HelperResultC::Validated { ok: true })),
        ("result-env-found", Some(HelperResultC::EnvFound { value: RefC::Data(503) })),
        ("result-env-missing", Some(HelperResultC::EnvMissing)),
        ("result-env-updated", Some(HelperResultC::EnvUpdated { cell: 509 })),
        ("result-find-found", Some(HelperResultC::FindFound { function: 521 })),
        ("result-find-missing", Some(HelperResultC::FindMissing)),
        ("result-bind-ok", Some(HelperResultC::BindOk { env: Some(523) })),
        ("result-bind-bad", Some(HelperResultC::BindBad)),
        ("result-append-done", Some(HelperResultC::AppendDone { seq: SliceC { start: 541, len: 5 } })),
        ("result-scalar-value", Some(HelperResultC::ScalarValue { value: RefC::Const(547) })),
        ("result-scalar-error", Some(HelperResultC::ScalarError { code: 10 })),
        ("result-scalar-selector", Some(HelperResultC::ScalarSelector { sel: BoolSelector::NotBool })),
        ("result-scalar-type-test", Some(HelperResultC::ScalarTypeTest { ok: false })),
        ("result-equal-done", Some(HelperResultC::EqualDone { value: RefC::Data(557) })),
        ("result-match-matched", Some(HelperResultC::MatchMatched { env: Some(563) })),
        ("result-match-no-match", Some(HelperResultC::MatchNoMatch)),
        ("result-plan-matched", Some(HelperResultC::PlanMatched { arm: 569, env: Some(571) })),
        ("result-plan-no-match", Some(HelperResultC::PlanNoMatch)),
        ("result-build-done", Some(HelperResultC::BuildDone { value: RefC::Data(577) })),
    ]
}
