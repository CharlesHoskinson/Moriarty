// owner_si21_bind_match.rs — SI-21 Bind/Match denotations only. Finite iterative checked reads; no native recursion, no producer-output oracle.
// K sources: uiBind/uiBindOne main.k:288-298 and uiMatching/uiKind main.k:299-319, restated over the v3a checked PreImageView reads.
use super::*;

fn bv(loc: &str) -> HostViolation {
    hv(HostViolationKind::WrongVariant, None, None, loc)
}
fn sort_accepts(pre: &PreImage, arg: &RefC, want: TypeIdC) -> Result<bool, HostViolation> {
    let k = pre.kind_of(arg)?;
    Ok(pre.checked_sort(&k).map(|s| s == want).unwrap_or(false))
}
fn chain_segment(
    pre: &PreImage,
    head: EnvRootC,
    base: EnvRootC,
) -> Result<Vec<EnvCellC>, HostViolation> {
    let mut out: Vec<EnvCellC> = Vec::new();
    let mut cur = head;
    while cur != base {
        let id = cur.ok_or_else(|| bv("env chain terminated before base"))?;
        if out.len() >= pre.active().env {
            return Err(bv("env chain bound"));
        }
        let cell = *pre.env_cell(&id)?;
        out.push(cell);
        cur = cell.previous;
    }
    Ok(out)
}
fn segment_matches(
    pre: &PreImage,
    head: EnvRootC,
    base: EnvRootC,
    bindings_newest_last: &[(usize, RefC)],
) -> Result<bool, HostViolation> {
    let seg = chain_segment(pre, head, base)?;
    if seg.len() != bindings_newest_last.len() {
        return Ok(false);
    }
    for (cell, want) in seg.iter().zip(bindings_newest_last.iter().rev()) {
        if cell.binder != want.0 || cell.value != want.1 {
            return Ok(false);
        }
    }
    Ok(true)
}
// UnwindDup: dupCursor d is a cell id on the partial chain and no strictly newer cell binds the top PairTask binder.
fn dup_scan_ok(
    pre: &PreImage,
    head: EnvRootC,
    d: usize,
    binder: usize,
) -> Result<bool, HostViolation> {
    let mut cur = head;
    let mut n = 0usize;
    while let Some(id) = cur {
        if n >= pre.active().env {
            return Err(bv("env chain bound"));
        }
        if id == d {
            return Ok(true);
        }
        let cell = *pre.env_cell(&id)?;
        if cell.binder == binder {
            return Ok(false);
        }
        cur = cell.previous;
        n += 1;
    }
    Ok(false)
}
fn bind_is_bad(pre: &PreImage, params: &SliceC, args: &SliceC) -> Result<bool, HostViolation> {
    if params.len != args.len {
        return Ok(true);
    }
    let mut seen: Vec<usize> = Vec::new();
    let mut i = 0usize;
    while i < params.len {
        let pp = params
            .start
            .checked_add(i)
            .ok_or_else(|| arith("bind param pos"))?;
        let ap = args
            .start
            .checked_add(i)
            .ok_or_else(|| arith("bind arg pos"))?;
        let pr = *pre.parameter(&pp)?;
        let arg = pre.value_slot(&ap)?;
        if seen.contains(&pr.binder) {
            return Ok(true);
        }
        seen.push(pr.binder);
        if !sort_accepts(pre, &arg, pr.sort)? {
            return Ok(true);
        }
        i += 1;
    }
    Ok(false)
}
fn pair_task_matches(
    pre: &PreImage,
    idx: usize,
    params: &SliceC,
    args: &SliceC,
) -> Result<bool, HostViolation> {
    let pp = params
        .start
        .checked_add(idx)
        .ok_or_else(|| arith("bind task param"))?;
    let ap = args
        .start
        .checked_add(idx)
        .ok_or_else(|| arith("bind task arg"))?;
    let pr = *pre.parameter(&pp)?;
    let arg = pre.value_slot(&ap)?;
    Ok(match pre.img.tasks.get(idx) {
        Some(HelperTaskC::PairTask {
            binder,
            sort,
            arg: a,
        }) => *binder == pr.binder && *sort == pr.sort && *a == arg,
        _ => false,
    })
}
pub(super) fn si21_bind_helper_ok(pre: &PreImage) -> Result<bool, HostViolation> {
    let (function, params, args, p_cursor, a_cursor, partial, dup_cursor, phase, status) =
        match pre.helper() {
            Some(HelperControlC::Bind {
                function,
                params,
                args,
                p_cursor,
                a_cursor,
                partial,
                dup_cursor,
                phase,
                status,
                ..
            }) => (
                *function,
                *params,
                *args,
                *p_cursor,
                *a_cursor,
                *partial,
                *dup_cursor,
                *phase,
                *status,
            ),
            _ => return Err(bv("Bind helper absent")),
        };
    match pre.control() {
        ControlC::Enter {
            function: Some(f),
            args: ca,
        } if *f == function && *ca == args => {}
        _ => return Ok(false),
    }
    if pre.function_row(&function)?.params != params {
        return Ok(false);
    }
    let bad = bind_is_bad(pre, &params, &args)?;
    if pre.task_live() > params.len || pre.task_live() > args.len {
        return Ok(false);
    }
    let mut ti = 0usize;
    while ti < pre.task_live() {
        if !pair_task_matches(pre, ti, &params, &args)? {
            return Ok(false);
        }
        ti += 1;
    }
    let inserted: Vec<(usize, RefC)> = if phase == BindPhase::Pairing {
        Vec::new()
    } else {
        let mut v: Vec<(usize, RefC)> = Vec::new();
        let mut i = pre.task_live();
        while i < params.len {
            let pp = params
                .start
                .checked_add(i)
                .ok_or_else(|| arith("bind inserted param"))?;
            let ap = args
                .start
                .checked_add(i)
                .ok_or_else(|| arith("bind inserted arg"))?;
            v.push((pre.parameter(&pp)?.binder, pre.value_slot(&ap)?));
            i += 1;
        }
        v.reverse();
        v
    };
    Ok(match (phase, status) {
        (BindPhase::Pairing, BindStatus::Ok) => {
            p_cursor == a_cursor
                && p_cursor == pre.task_live()
                && p_cursor <= params.len
                && a_cursor <= args.len
                && partial.is_none()
                && dup_cursor.is_none()
        }
        (BindPhase::UnwindType, BindStatus::Ok) => {
            p_cursor == params.len
                && a_cursor == args.len
                && pre.task_live() >= 1
                && dup_cursor.is_none()
                && segment_matches(pre, partial, None, &inserted)?
        }
        (BindPhase::UnwindDup, BindStatus::Ok) => {
            p_cursor == params.len
                && a_cursor == args.len
                && pre.task_live() >= 1
                && dup_cursor.is_some()
                && segment_matches(pre, partial, None, &inserted)?
                && match (dup_cursor, pre.top_task()) {
                    (Some(d), Some(HelperTaskC::PairTask { binder, .. })) => {
                        dup_scan_ok(pre, partial, d, *binder)?
                    }
                    _ => false,
                }
        }
        (BindPhase::UnwindInsert, BindStatus::Ok) => {
            p_cursor == params.len
                && a_cursor == args.len
                && pre.task_live() >= 1
                && dup_cursor.is_none()
                && segment_matches(pre, partial, None, &inserted)?
        }
        (BindPhase::Propagate, BindStatus::Bad) => {
            pre.task_live() >= 1 && dup_cursor.is_none() && bad
        }
        (BindPhase::Finished, BindStatus::Ok) => {
            pre.task_live() == 0
                && dup_cursor.is_none()
                && !bad
                && segment_matches(pre, partial, None, &{
                    let mut v: Vec<(usize, RefC)> = Vec::new();
                    let mut i = 0usize;
                    while i < params.len {
                        let pp = params
                            .start
                            .checked_add(i)
                            .ok_or_else(|| arith("bind finished param"))?;
                        let ap = args
                            .start
                            .checked_add(i)
                            .ok_or_else(|| arith("bind finished arg"))?;
                        v.push((pre.parameter(&pp)?.binder, pre.value_slot(&ap)?));
                        i += 1;
                    }
                    v.reverse();
                    v
                })?
        }
        (BindPhase::Finished, BindStatus::Bad) => {
            pre.task_live() == 0 && dup_cursor.is_none() && bad
        }
        _ => false,
    })
}
pub(super) fn si21_bind_result_ok(
    pre: &PreImage,
    r: &HelperResultC,
) -> Result<bool, HostViolation> {
    let (function, args) = match pre.control() {
        ControlC::Enter {
            function: Some(f),
            args,
        } => (*f, *args),
        _ => return Ok(false),
    };
    let params = pre.function_row(&function)?.params;
    let bad = bind_is_bad(pre, &params, &args)?;
    match r {
        HelperResultC::BindOk { env } => {
            if bad {
                return Ok(false);
            }
            let mut v: Vec<(usize, RefC)> = Vec::new();
            let mut i = 0usize;
            while i < params.len {
                let pp = params
                    .start
                    .checked_add(i)
                    .ok_or_else(|| arith("bind result param"))?;
                let ap = args
                    .start
                    .checked_add(i)
                    .ok_or_else(|| arith("bind result arg"))?;
                v.push((pre.parameter(&pp)?.binder, pre.value_slot(&ap)?));
                i += 1;
            }
            v.reverse();
            segment_matches(pre, *env, None, &v)
        }
        HelperResultC::BindBad => Ok(bad),
        _ => Ok(false),
    }
}
enum MatchDen {
    NoMatch,
    Matched(Vec<(usize, RefC)>),
}
fn match_denotation(
    pre: &PreImage,
    pat: &PatternRowC,
    scrutinee: &RefC,
) -> Result<MatchDen, HostViolation> {
    let view = super::pure_view(pre)?;
    let mut stack: Vec<(PatternRowC, RefC)> = vec![(*pat, *scrutinee)];
    let mut bindings: Vec<(usize, RefC)> = Vec::new();
    let mut budget = view.patterns.len().saturating_add(1);
    while let Some((cur_pat, cur_val)) = stack.pop() {
        if budget == 0 {
            return Err(bv("pattern tree bound"));
        }
        budget -= 1;
        let kind = pre.kind_of(&cur_val)?;
        match cur_pat {
            PatternRowC::Wildcard => {}
            PatternRowC::PVar { x } => {
                if bindings.iter().any(|(b, _)| *b == x) {
                    return Ok(MatchDen::NoMatch);
                }
                bindings.push((x, cur_val));
            }
            PatternRowC::PNil => {
                if kind != LogicalKind::Nil {
                    return Ok(MatchDen::NoMatch);
                }
            }
            PatternRowC::PCons { x, y } => {
                if kind != LogicalKind::Cons
                    || x == y
                    || bindings.iter().any(|(b, _)| *b == x || *b == y)
                {
                    return Ok(MatchDen::NoMatch);
                }
                bindings.push((x, pre.field_at(&cur_val, &0)?));
                bindings.push((y, pre.field_at(&cur_val, &1)?));
            }
            PatternRowC::PLeaf { x } => {
                if kind != LogicalKind::Leaf || bindings.iter().any(|(b, _)| *b == x) {
                    return Ok(MatchDen::NoMatch);
                }
                bindings.push((x, pre.field_at(&cur_val, &0)?));
            }
            PatternRowC::PBranch { x, y } => {
                if kind != LogicalKind::Branch
                    || x == y
                    || bindings.iter().any(|(b, _)| *b == x || *b == y)
                {
                    return Ok(MatchDen::NoMatch);
                }
                bindings.push((x, pre.field_at(&cur_val, &0)?));
                bindings.push((y, pre.field_at(&cur_val, &1)?));
            }
            PatternRowC::Constructor {
                type_id,
                ctor_id,
                children,
            } => {
                let kind_matches = match kind {
                    LogicalKind::Nil => type_id == TypeIdC::List && ctor_id.0 == 0,
                    LogicalKind::Cons => type_id == TypeIdC::List && ctor_id.0 == 1,
                    LogicalKind::Leaf => type_id == TypeIdC::Tree && ctor_id.0 == 2,
                    LogicalKind::Branch => type_id == TypeIdC::Tree && ctor_id.0 == 3,
                    LogicalKind::Constructor {
                        type_id: kt,
                        ctor_id: kc,
                    } => kt == type_id && kc == ctor_id,
                    _ => false,
                };
                if !kind_matches {
                    return Ok(MatchDen::NoMatch);
                }
                let arity = pre.checked_arity(&kind)?;
                if children.len != arity {
                    return Err(bv("pattern constructor arity"));
                }
                let end = children
                    .start
                    .checked_add(children.len)
                    .ok_or_else(|| arith("pattern children end"))?;
                let slice = view
                    .pattern_children
                    .get(children.start..end)
                    .ok_or_else(|| oob("prog.patternChildren"))?;
                for idx in (0..children.len).rev() {
                    let child_pat = *pre.pattern(&slice[idx])?;
                    let child_val = pre.field_at(&cur_val, &idx)?;
                    stack.push((child_pat, child_val));
                }
            }
        }
    }
    Ok(MatchDen::Matched(bindings))
}
fn match_context(
    pre: &PreImage,
) -> Result<Option<(RefC, usize, EnvRootC, PatternRowC, Vec<(usize, RefC)>, bool)>, HostViolation> {
    let (scrutinee, arm, saved, _partial) = match pre.helper() {
        Some(HelperControlC::Match {
            scrutinee,
            arm,
            saved_env,
            partial,
            ..
        }) => (*scrutinee, *arm, *saved_env, *partial),
        _ => return Ok(None),
    };
    match pre.control() {
        ControlC::Cases {
            value,
            arms,
            saved_env,
        } if *value == scrutinee && arms.start == arm && *saved_env == saved => {}
        _ => return Ok(None),
    }
    if !matches!(
        pre.expr(&pre.control_origin())?,
        ExpressionRowC::Match { plan: None, .. }
    ) {
        return Ok(None);
    }
    let pat = *pre.pattern(&pre.arm(&arm)?.pattern)?;
    if matches!(pat, PatternRowC::Wildcard | PatternRowC::Constructor { .. }) {
        return Ok(None);
    }
    match match_denotation(pre, &pat, &scrutinee)? {
        MatchDen::Matched(b) => Ok(Some((scrutinee, arm, saved, pat, b, true))),
        MatchDen::NoMatch => Ok(Some((scrutinee, arm, saved, pat, Vec::new(), false))),
    }
}
pub(super) fn si21_match_helper_ok(pre: &PreImage) -> Result<bool, HostViolation> {
    let (pat_tag, x, y, field_idx, staged, phase, status, partial, saved) = match pre.helper() {
        Some(HelperControlC::Match {
            pat_tag,
            x,
            y,
            field_idx,
            staged,
            phase,
            status,
            partial,
            saved_env,
            ..
        }) => (
            *pat_tag, *x, *y, *field_idx, *staged, *phase, *status, *partial, *saved_env,
        ),
        _ => return Err(bv("Match helper absent")),
    };
    let (scrutinee, _, saved2, pat, bindings, matched) = match match_context(pre)? {
        Some(v) => v,
        None => return Ok(false),
    };
    if saved != saved2 {
        return Ok(false);
    }
    if phase == MatchPhase::Arm {
        return Ok(status == TriStatus::Pending
            && pat_tag == PatternTag::PVar
            && x == 0
            && y == 0
            && field_idx == 0
            && staged.is_none()
            && partial == saved);
    }
    let (etag, ex, ey) = match pat {
        PatternRowC::PVar { x } => (PatternTag::PVar, x, 0),
        PatternRowC::PNil => (PatternTag::PNil, 0, 0),
        PatternRowC::PCons { x, y } => (PatternTag::PCons, x, y),
        PatternRowC::PLeaf { x } => (PatternTag::PLeaf, x, 0),
        PatternRowC::PBranch { x, y } => (PatternTag::PBranch, x, y),
        PatternRowC::Wildcard | PatternRowC::Constructor { .. } => return Ok(false),
    };
    if pat_tag != etag || x != ex || y != ey {
        return Ok(false);
    }
    let made: usize = match phase {
        MatchPhase::Arm | MatchPhase::Tag | MatchPhase::Distinct | MatchPhase::NoMatchReady => 0,
        MatchPhase::Field => usize::from(field_idx),
        MatchPhase::Bind => {
            if etag == PatternTag::PVar {
                0
            } else {
                usize::from(field_idx)
            }
        }
        MatchPhase::Ready => bindings.len(),
        MatchPhase::Done => {
            if status == TriStatus::Positive {
                bindings.len()
            } else {
                0
            }
        }
    };
    if !segment_matches(pre, partial, saved, &bindings[..made.min(bindings.len())])? {
        return Ok(false);
    }
    Ok(match phase {
        MatchPhase::Tag => {
            status == TriStatus::Pending && staged.is_none() && etag != PatternTag::PVar
        }
        MatchPhase::Distinct => {
            status == TriStatus::Pending
                && staged.is_none()
                && match etag {
                    PatternTag::PCons => pre.kind_of(&scrutinee)? == LogicalKind::Cons,
                    PatternTag::PBranch => pre.kind_of(&scrutinee)? == LogicalKind::Branch,
                    _ => false,
                }
        }
        MatchPhase::Field => {
            status == TriStatus::Pending
                && staged.is_none()
                && matched
                && matches!(
                    etag,
                    PatternTag::PLeaf | PatternTag::PCons | PatternTag::PBranch
                )
        }
        MatchPhase::Bind => {
            status == TriStatus::Pending
                && matched
                && staged.is_some()
                && (etag != PatternTag::PVar || field_idx == 0)
                && staged == bindings.get(usize::from(field_idx)).map(|b| b.1)
        }
        MatchPhase::Ready => status == TriStatus::Pending && staged.is_none() && matched,
        MatchPhase::NoMatchReady => {
            status == TriStatus::Pending && staged.is_none() && !matched && partial == saved
        }
        MatchPhase::Done => {
            staged.is_none()
                && ((status == TriStatus::Positive && matched)
                    || (status == TriStatus::Negative && !matched && partial == saved))
        }
        _ => false,
    })
}
pub(super) fn si21_match_result_ok(
    pre: &PreImage,
    r: &HelperResultC,
) -> Result<bool, HostViolation> {
    let (scrutinee, arms, saved) = match pre.control() {
        ControlC::Cases {
            value,
            arms,
            saved_env,
        } if arms.len > 0 => (*value, *arms, *saved_env),
        _ => return Ok(false),
    };
    if !matches!(
        pre.expr(&pre.control_origin())?,
        ExpressionRowC::Match { plan: None, .. }
    ) {
        return Ok(false);
    }
    let pat = *pre.pattern(&pre.arm(&arms.start)?.pattern)?;
    if matches!(pat, PatternRowC::Wildcard | PatternRowC::Constructor { .. }) {
        return Ok(false);
    }
    match match_denotation(pre, &pat, &scrutinee)? {
        MatchDen::Matched(bindings) => match r {
            HelperResultC::MatchMatched { env } => segment_matches(pre, *env, saved, &bindings),
            _ => Ok(false),
        },
        MatchDen::NoMatch => Ok(matches!(r, HelperResultC::MatchNoMatch)),
    }
}
