// owner_si21_closure.rs — SI-21 helper clauses for Scalar/EnvGet/EnvPut/Find/Append/Validate/Equal, SI-21R for every non-Bind/Match result, SI-22 helper_state_space for all nine kinds.
// Denotations restated from main.k:229-319 and schema step_inv SI-21/SI-21R/helper_state_space over checked PreImageView reads. Explicit bounded loops/worklists; no native recursion; no producer-row replay.
// ADJ-1 (Scalar Done staged = K denotation of the installed operands) and ADJ-2 (Append Copy staged = vslot(src, srcCursor)) adjudicate SI-21 where it is silent; fresh consequential votes required.
use super::*;

fn cv(loc: &str) -> HostViolation { hv(HostViolationKind::WrongVariant, None, None, loc) }
fn ret_value(pre: &PreImage) -> Option<RefC> { match pre.control() { ControlC::Return { value } => Some(*value), _ => None } }
fn var_binder(pre: &PreImage) -> Result<Option<usize>, HostViolation> {
    match pre.control() { ControlC::Eval { expr } => Ok(match pre.expr(expr)? { ExpressionRowC::Var { binder } => Some(*binder), _ => None }), _ => Ok(None) }
}
fn literal_const(pre: &PreImage) -> Result<Option<usize>, HostViolation> {
    match pre.control() { ControlC::Eval { expr } => Ok(match pre.expr(expr)? { ExpressionRowC::Literal { constant } => Some(*constant), _ => None }), _ => Ok(None) }
}
fn invoke_name(pre: &PreImage) -> Option<usize> { match pre.control() { ControlC::Invoke { function_name, .. } => Some(*function_name), _ => None } }
// First cell with `binder` on the newest-first chain from `root`; bounded by active env cells.
fn first_on_chain(pre: &PreImage, root: EnvRootC, binder: usize) -> Result<Option<(usize, RefC)>, HostViolation> {
    let mut cur = root; let mut n = 0usize;
    while let Some(id) = cur {
        if n >= pre.active().env { return Err(cv("env chain bound")); }
        let cell = *pre.env_cell(&id)?;
        if cell.binder == binder { return Ok(Some((id, cell.value))); }
        cur = cell.previous; n += 1;
    }
    Ok(None)
}
// True iff `cursor` is on the chain from `root` (absent = chain end) and no strictly newer cell has `binder`.
fn cursor_on_chain(pre: &PreImage, root: EnvRootC, cursor: EnvRootC, binder: usize) -> Result<bool, HostViolation> {
    let mut cur = root; let mut n = 0usize;
    loop {
        if cur == cursor { return Ok(true); }
        let id = match cur { Some(id) => id, None => return Ok(false) };
        if n >= pre.active().env { return Err(cv("env chain bound")); }
        let cell = *pre.env_cell(&id)?;
        if cell.binder == binder { return Ok(false); }
        cur = cell.previous; n += 1;
    }
}
fn least_row_named(pre: &PreImage, name: usize) -> Result<Option<usize>, HostViolation> {
    let n = pre.admitted_function_count(); let mut i = 0usize;
    while i < n { if pre.function_row(&i)?.function_name == name { return Ok(Some(i)); } i += 1; }
    Ok(None)
}
fn slot_at(pre: &PreImage, s: &SliceC, i: usize) -> Result<RefC, HostViolation> {
    if i >= s.len { return Err(cv("vslot index")); }
    pre.value_slot(&s.start.checked_add(i).ok_or_else(|| arith("vslot pos"))?)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum KPrim { U128(u128), BoolV(bool), Node(NodeKind), Error(u8) }
fn binary_mode(op: BinaryOp) -> Option<ScalarMode> {
    match op { BinaryOp::Add => Some(ScalarMode::Add), BinaryOp::Sub => Some(ScalarMode::Sub), BinaryOp::Mul => Some(ScalarMode::Mul),
        BinaryOp::Lt => Some(ScalarMode::Lt), BinaryOp::Cons => Some(ScalarMode::Cons), BinaryOp::Branch => Some(ScalarMode::Branch), BinaryOp::Eq => None }
}
fn unary_mode(op: UnaryOp) -> ScalarMode { match op { UnaryOp::Leaf => ScalarMode::Leaf, UnaryOp::Not => ScalarMode::Not } }
// uiBinaryResult / uiUnaryResult (main.k:247-278) over kind(); exact checked u128 gives uiChecked error 10.
fn k_prim(pre: &PreImage, mode: ScalarMode, a: &RefC, b: Option<&RefC>) -> Result<KPrim, HostViolation> {
    use LogicalKind as K;
    let ka = pre.kind_of(a)?;
    let kb = match b { Some(r) => Some(pre.kind_of(r)?), None => None };
    Ok(match (mode, b, kb) {
        (ScalarMode::Add | ScalarMode::Sub | ScalarMode::Mul, Some(rb), Some(K::UInt)) if ka == K::UInt => {
            let (x, y) = (pre.u128_of(a)?, pre.u128_of(rb)?);
            let r = match mode { ScalarMode::Add => x.checked_add(y), ScalarMode::Sub => x.checked_sub(y), _ => x.checked_mul(y) };
            match r { Some(n) => KPrim::U128(n), None => KPrim::Error(10) } }
        (ScalarMode::Lt, Some(rb), Some(K::UInt)) if ka == K::UInt => KPrim::BoolV(pre.u128_of(a)? < pre.u128_of(rb)?),
        (ScalarMode::Cons, Some(_), Some(K::Nil | K::Cons)) if ka == K::UInt => KPrim::Node(NodeKind::Cons),
        (ScalarMode::Branch, Some(_), Some(K::Leaf | K::Branch)) if matches!(ka, K::Leaf | K::Branch) => KPrim::Node(NodeKind::Branch),
        (ScalarMode::Add | ScalarMode::Sub | ScalarMode::Mul | ScalarMode::Lt | ScalarMode::Cons | ScalarMode::Branch, Some(_), _) => KPrim::Error(11),
        (ScalarMode::Leaf, None, _) => if ka == K::UInt { KPrim::Node(NodeKind::Leaf) } else { KPrim::Error(11) },
        (ScalarMode::Not, None, _) => if ka == K::Bool { KPrim::BoolV(!pre.bool_of(a)?) } else { KPrim::Error(11) },
        _ => return Err(cv("scalar operand shape")),
    })
}
fn selector_of(pre: &PreImage, v: &RefC) -> Result<BoolSelector, HostViolation> {
    Ok(if pre.kind_of(v)? == LogicalKind::Bool { if pre.bool_of(v)? { BoolSelector::True } else { BoolSelector::False } } else { BoolSelector::NotBool })
}
fn has_type(pre: &PreImage, v: &RefC, t: TypeIdC) -> Result<bool, HostViolation> { pre.scalar_test_type_ok(v, &t) }

// ---- Scalar: the payload b.helper.Scalar.entry installs from site, top and v.
struct ScalarEntry { site: SiteTag, mode: ScalarMode, a: RefC, b: Option<RefC>, expected: Option<TypeIdC> }
fn scalar_entry(pre: &PreImage) -> Option<ScalarEntry> {
    let v = ret_value(pre)?;
    match pre.top_frame()? {
        FrameRowC::RightF { op, left, .. } => Some(ScalarEntry { site: SiteTag::RetRight, mode: binary_mode(*op)?, a: *left, b: Some(v), expected: None }),
        FrameRowC::UnaryF { op, .. } => Some(ScalarEntry { site: SiteTag::RetUnary, mode: unary_mode(*op), a: v, b: None, expected: None }),
        FrameRowC::IfF { .. } => Some(ScalarEntry { site: SiteTag::RetIf, mode: ScalarMode::TestBool, a: v, b: None, expected: None }),
        FrameRowC::ReturnF { return_type, .. } => Some(ScalarEntry { site: SiteTag::RetCall, mode: ScalarMode::TestType, a: v, b: None, expected: Some(*return_type) }),
        FrameRowC::ConstructorFieldsF { field_type, checked: false, .. } => Some(ScalarEntry { site: SiteTag::RetCtorFields, mode: ScalarMode::TestType, a: v, b: None, expected: Some(*field_type) }),
        _ => None,
    }
}
// ADJ-1: the Done staging K assigns to the installed operands.
fn scalar_den(pre: &PreImage, e: &ScalarEntry) -> Result<ScalarStaged, HostViolation> {
    Ok(match e.mode {
        ScalarMode::TestBool => ScalarStaged::Selector { sel: selector_of(pre, &e.a)? },
        ScalarMode::TestType => ScalarStaged::TypeTest { ok: has_type(pre, &e.a, e.expected.ok_or_else(|| cv("TestType expected"))?)? },
        m => match k_prim(pre, m, &e.a, e.b.as_ref())? {
            KPrim::U128(n) => ScalarStaged::U128Value { n }, KPrim::BoolV(b) => ScalarStaged::BoolValue { b },
            KPrim::Node(tag) => ScalarStaged::PendingNode { tag }, KPrim::Error(code) => ScalarStaged::PrimError { code } },
    })
}
fn scalar_si21(pre: &PreImage) -> Result<bool, HostViolation> {
    let (common, mode, a, b, expected, phase, staged) = match pre.helper() {
        Some(HelperControlC::Scalar { common, mode, operand_a, operand_b, expected, phase, staged, .. }) => (*common, *mode, *operand_a, *operand_b, *expected, *phase, *staged),
        _ => return Err(cv("Scalar helper absent")) };
    let e = match scalar_entry(pre) { Some(e) => e, None => return Ok(false) };
    if common.site != e.site || mode != e.mode || a != e.a || b != e.b || expected != e.expected { return Ok(false); }
    Ok(phase != ScalarPhase::Done || staged == scalar_den(pre, &e)?)
}
fn envget_si21(pre: &PreImage) -> Result<bool, HostViolation> {
    let (binder, cursor, found, status) = match pre.helper() {
        Some(HelperControlC::EnvGet { binder, cursor, found, status, .. }) => (*binder, *cursor, *found, *status), _ => return Err(cv("EnvGet helper absent")) };
    if var_binder(pre)? != Some(binder) || !cursor_on_chain(pre, pre.env_root(), cursor, binder)? { return Ok(false); }
    Ok(match status {
        TriStatus::Positive => match cursor { Some(c) => { let cell = *pre.env_cell(&c)?; cell.binder == binder && found == Some(cell.value) } None => false },
        TriStatus::Negative => first_on_chain(pre, pre.env_root(), binder)?.is_none(),
        TriStatus::Pending => true })
}
fn envput_si21(pre: &PreImage) -> Result<bool, HostViolation> {
    let (binder, value, base, new_root, status) = match pre.helper() {
        Some(HelperControlC::EnvPut { binder, value, base, new_root, status, .. }) => (*binder, *value, *base, *new_root, *status), _ => return Err(cv("EnvPut helper absent")) };
    let (tb, saved) = match pre.top_frame() { Some(FrameRowC::LetF { binder, saved_env, .. }) => (*binder, *saved_env), _ => return Ok(false) };
    if Some(value) != ret_value(pre) || binder != tb || base != saved { return Ok(false); }
    if status != TriStatus::Positive { return Ok(true); }
    match new_root { Some(n) if pre.active().env.checked_sub(1) == Some(n) => Ok(*pre.env_cell(&n)? == EnvCellC { binder, value, previous: base }), _ => Ok(false) }
}
fn find_si21(pre: &PreImage) -> Result<bool, HostViolation> {
    let (name, row_cursor, status) = match pre.helper() {
        Some(HelperControlC::Find { function_name, row_cursor, status, .. }) => (*function_name, *row_cursor, *status), _ => return Err(cv("Find helper absent")) };
    if invoke_name(pre) != Some(name) { return Ok(false); }
    let n = pre.admitted_function_count(); let mut i = 0usize;
    while i < row_cursor && i < n { if pre.function_row(&i)?.function_name == name { return Ok(false); } i += 1; }
    Ok(status != TriStatus::Positive || (row_cursor < n && pre.function_row(&row_cursor)?.function_name == name))
}
fn append_si21(pre: &PreImage) -> Result<bool, HostViolation> {
    let (src, value, src_cursor, dest_start, staged, phase) = match pre.helper() {
        Some(HelperControlC::Append { src, value, src_cursor, dest_start, staged, phase, .. }) => (*src, *value, *src_cursor, *dest_start, *staged, *phase), _ => return Err(cv("Append helper absent")) };
    let completed = match pre.top_frame() {
        Some(FrameRowC::ArgsF { completed, .. } | FrameRowC::ConstructorFieldsF { completed, checked: true, .. }) => *completed,
        _ => return Ok(false),
    };
    if src != completed || Some(value) != ret_value(pre) || src_cursor > src.len { return Ok(false); }
    let dest = SliceC { start: dest_start, len: src_cursor.checked_add(if phase == AppendPhase::Done { 1 } else { 0 }).ok_or_else(|| arith("append dest len"))? };
    let mut i = 0usize;
    while i < src_cursor { if slot_at(pre, &dest, i)? != slot_at(pre, &src, i)? { return Ok(false); } i += 1; }
    if phase == AppendPhase::Done && (src_cursor != src.len || slot_at(pre, &dest, src.len)? != value) { return Ok(false); }
    // ADJ-2: a staged copy value is exactly the source slot at srcCursor.
    Ok(match staged { Some(s) => phase == AppendPhase::Copy && src_cursor < src.len && s == slot_at(pre, &src, src_cursor)?, None => true })
}
fn validate_si21(pre: &PreImage) -> Result<bool, HostViolation> {
    let (root, status) = match pre.helper() { Some(HelperControlC::Validate { root, status, .. }) => (*root, *status), _ => return Err(cv("Validate helper absent")) };
    let k = match literal_const(pre)? { Some(k) => k, None => return Ok(false) };
    if root != RefC::Const(k) { return Ok(false); }
    let desc = const_descendants(pre, k)?;
    for t in &pre.img.tasks { match t { HelperTaskC::ValidateTask { r: RefC::Const(id), .. } if desc.contains(id) => {}, _ => return Ok(false) } }
    Ok(match status { TriStatus::Positive => pre.marker_free(&k)?, TriStatus::Negative => !pre.marker_free(&k)?, TriStatus::Pending => true })
}
fn equal_si21(pre: &PreImage) -> Result<bool, HostViolation> {
    let (left, right, status) = match pre.helper() { Some(HelperControlC::Equal { left, right, status, .. }) => (*left, *right, *status), _ => return Err(cv("Equal helper absent")) };
    let tl = match pre.top_frame() { Some(FrameRowC::RightF { op: BinaryOp::Eq, left, .. }) => *left, _ => return Ok(false) };
    if left != tl || Some(right) != ret_value(pre) { return Ok(false); }
    Ok(match status { TriStatus::Positive => structurally_equal(pre, left, right)?, TriStatus::Negative => !structurally_equal(pre, left, right)?, TriStatus::Pending => true })
}
pub(super) fn si21_helper_ok(pre: &PreImage) -> Result<bool, HostViolation> {
    match pre.helper() {
        Some(HelperControlC::Scalar { .. }) => scalar_si21(pre), Some(HelperControlC::EnvGet { .. }) => envget_si21(pre),
        Some(HelperControlC::EnvPut { .. }) => envput_si21(pre), Some(HelperControlC::Find { .. }) => find_si21(pre),
        Some(HelperControlC::Append { .. }) => append_si21(pre), Some(HelperControlC::Validate { .. }) => validate_si21(pre),
        Some(HelperControlC::Equal { .. }) => equal_si21(pre),
        Some(h @ (HelperControlC::Build { .. } | HelperControlC::Plan { .. })) => si21_adt_helper_ok(pre, h),
        Some(HelperControlC::Bind { .. } | HelperControlC::Match { .. }) => Err(cv("Bind/Match SI-21 is owner_si21_bind_match")),
        None => Err(cv("SI-21 without helper")),
    }
}

// ---- SI-21R
fn scalar_result_prim(pre: &PreImage) -> Result<Option<(KPrim, RefC, Option<RefC>)>, HostViolation> {
    let v = match ret_value(pre) { Some(v) => v, None => return Ok(None) };
    Ok(match pre.top_frame() {
        Some(FrameRowC::RightF { op, left, .. }) => match binary_mode(*op) { Some(m) => Some((k_prim(pre, m, left, Some(&v))?, *left, Some(v))), None => None },
        Some(FrameRowC::UnaryF { op, .. }) => Some((k_prim(pre, unary_mode(*op), &v, None)?, v, None)),
        _ => None })
}
// Data row j decodes to the K value: scalar payload, or node kind with fields structurally equal to the operands in Scalar.exit field order.
fn data_row_is(pre: &PreImage, j: usize, p: KPrim, a: RefC, b: Option<RefC>) -> Result<bool, HostViolation> {
    let r = RefC::Data(j); let k = pre.kind_of(&r)?;
    Ok(match p {
        KPrim::U128(n) => k == LogicalKind::UInt && pre.u128_of(&r)? == n,
        KPrim::BoolV(x) => k == LogicalKind::Bool && pre.bool_of(&r)? == x,
        KPrim::Node(NodeKind::Leaf) => k == LogicalKind::Leaf && structurally_equal(pre, pre.field_at(&r, &0)?, a)?,
        KPrim::Node(nk) => {
            let want = if nk == NodeKind::Cons { LogicalKind::Cons } else { LogicalKind::Branch };
            match b { Some(b) => k == want && structurally_equal(pre, pre.field_at(&r, &0)?, a)? && structurally_equal(pre, pre.field_at(&r, &1)?, b)?, None => false } }
        KPrim::Error(_) => false })
}
fn newest_data(pre: &PreImage, value: &RefC) -> Option<usize> {
    match value { RefC::Data(j) if pre.active().data.checked_sub(1) == Some(*j) => Some(*j), _ => None }
}
pub(super) fn si21_result_ok(pre: &PreImage, r: &HelperResultC) -> Result<bool, HostViolation> {
    Ok(match r {
        HelperResultC::Validated { ok } => match literal_const(pre)? { Some(k) => pre.marker_free(&k)? == *ok, None => false },
        HelperResultC::EnvFound { value } => match var_binder(pre)? { Some(x) => first_on_chain(pre, pre.env_root(), x)?.map(|c| c.1) == Some(*value), None => false },
        HelperResultC::EnvMissing => match var_binder(pre)? { Some(x) => first_on_chain(pre, pre.env_root(), x)?.is_none(), None => false },
        HelperResultC::EnvUpdated { cell } => match (pre.top_frame(), ret_value(pre)) {
            (Some(FrameRowC::LetF { binder, saved_env, .. }), Some(v)) => pre.active().env.checked_sub(1) == Some(*cell)
                && *pre.env_cell(cell)? == EnvCellC { binder: *binder, value: v, previous: *saved_env },
            _ => false },
        HelperResultC::FindFound { function } => match invoke_name(pre) { Some(n) => least_row_named(pre, n)? == Some(*function), None => false },
        HelperResultC::FindMissing => match invoke_name(pre) { Some(n) => least_row_named(pre, n)?.is_none(), None => false },
        HelperResultC::AppendDone { seq } => match (pre.top_frame(), ret_value(pre)) {
            (Some(FrameRowC::ArgsF { completed, .. } | FrameRowC::ConstructorFieldsF { completed, checked: true, .. }), Some(v)) => {
                if completed.len.checked_add(1) != Some(seq.len) || seq.start.checked_add(seq.len) != Some(pre.active().value_slots) { return Ok(false); }
                let mut i = 0usize;
                while i < completed.len { if slot_at(pre, seq, i)? != slot_at(pre, completed, i)? { return Ok(false); } i += 1; }
                slot_at(pre, seq, completed.len)? == v }
            _ => false },
        HelperResultC::ScalarValue { value } => match (newest_data(pre, value), scalar_result_prim(pre)?) {
            (Some(j), Some((p, a, b))) => data_row_is(pre, j, p, a, b)?, _ => false },
        HelperResultC::ScalarError { code } => matches!(scalar_result_prim(pre)?, Some((KPrim::Error(c), _, _)) if c == *code),
        HelperResultC::ScalarSelector { sel } => match (pre.top_frame(), ret_value(pre)) {
            (Some(FrameRowC::IfF { .. }), Some(v)) => selector_of(pre, &v)? == *sel, _ => false },
        HelperResultC::ScalarTypeTest { ok } => match (pre.top_frame(), ret_value(pre)) {
            (Some(FrameRowC::ReturnF { return_type: want, .. } | FrameRowC::ConstructorFieldsF { field_type: want, checked: false, .. }), Some(v)) =>
                has_type(pre, &v, *want)? == *ok,
            _ => false },
        HelperResultC::EqualDone { value } => match (newest_data(pre, value), pre.top_frame(), ret_value(pre)) {
            (Some(j), Some(FrameRowC::RightF { op: BinaryOp::Eq, left, .. }), Some(v)) => {
                let r0 = RefC::Data(j);
                pre.kind_of(&r0)? == LogicalKind::Bool && pre.bool_of(&r0)? == structurally_equal(pre, *left, v)? }
            _ => false },
        HelperResultC::BuildDone { .. } | HelperResultC::PlanMatched { .. } | HelperResultC::PlanNoMatch =>
            si21_adt_result_ok(pre, r)?,
        HelperResultC::BindOk { .. } | HelperResultC::BindBad | HelperResultC::MatchMatched { .. } | HelperResultC::MatchNoMatch =>
            return Err(cv("Bind/Match SI-21R is owner_si21_bind_match")),
    })
}

// ---- SI-22 helper_state_space
fn scalar_state_ok(pre: &PreImage, h: &HelperControlC) -> Result<bool, HostViolation> {
    use ScalarMode as M; use ScalarStaged as S;
    let (mode, a, b, expected, phase, reg_a, staged) = match h {
        HelperControlC::Scalar { mode, operand_a, operand_b, expected, phase, reg_a, staged, .. } => (*mode, *operand_a, *operand_b, *expected, *phase, *reg_a, *staged),
        _ => return Err(cv("Scalar state")) };
    let binary = matches!(mode, M::Add | M::Sub | M::Mul | M::Lt | M::Cons | M::Branch);
    if b.is_some() != binary || expected.is_some() != (mode == M::TestType) { return Ok(false); }
    let ka = pre.kind_of(&a)?;
    let numeric = matches!(mode, M::Add | M::Sub | M::Mul | M::Lt) && ka == LogicalKind::UInt;
    let reg_ok = if numeric { reg_a == pre.u128_of(&a)? } else { reg_a == 0 };
    Ok(match phase {
        ScalarPhase::First => staged == S::None && reg_a == 0,
        ScalarPhase::Second => staged == S::None && reg_ok && match mode {
            M::Add | M::Sub | M::Mul | M::Lt | M::Cons => ka == LogicalKind::UInt,
            M::Branch => matches!(ka, LogicalKind::Leaf | LogicalKind::Branch),
            _ => false },
        ScalarPhase::Done => reg_ok && matches!((mode, staged),
            (M::Add | M::Sub | M::Mul, S::U128Value { .. } | S::PrimError { code: 10 | 11 })
            | (M::Lt | M::Not, S::BoolValue { .. } | S::PrimError { code: 11 })
            | (M::Cons, S::PendingNode { tag: NodeKind::Cons }) | (M::Branch, S::PendingNode { tag: NodeKind::Branch })
            | (M::Leaf, S::PendingNode { tag: NodeKind::Leaf }) | (M::Cons | M::Branch | M::Leaf, S::PrimError { code: 11 })
            | (M::TestBool, S::Selector { .. }) | (M::TestType, S::TypeTest { .. })),
    })
}
fn tasks_ok(pre: &PreImage, equal: bool) -> Result<bool, HostViolation> {
    for t in &pre.img.tasks {
        let ok = match (equal, t) {
            (false, HelperTaskC::ValidateTask { visit: TaskVisitC::Unvisited, cursor, .. }) | (true, HelperTaskC::EqualTask { visit: TaskVisitC::Unvisited, cursor, .. }) => *cursor == 0,
            (false, HelperTaskC::ValidateTask { r, visit: TaskVisitC::Fields, cursor }) => { let ar = pre.checked_arity(&pre.kind_of(r)?)?; ar > 0 && *cursor < ar }
            (true, HelperTaskC::EqualTask { left, right, visit: TaskVisitC::Fields, cursor }) => { let k = pre.kind_of(left)?; let ar = pre.checked_arity(&k)?; k == pre.kind_of(right)? && ar > 0 && *cursor < ar }
            _ => false };
        if !ok { return Ok(false); }
    }
    Ok(true)
}
fn match_state_ok(pre: &PreImage, h: &HelperControlC) -> Result<bool, HostViolation> {
    use PatternTag as P;
    let (pat_tag, x, y, field_idx, staged, phase, status, partial, saved) = match h {
        HelperControlC::Match { pat_tag, x, y, field_idx, staged, phase, status, partial, saved_env, .. } => (*pat_tag, *x, *y, *field_idx, *staged, *phase, *status, *partial, *saved_env),
        _ => return Err(cv("Match state")) };
    let pending = status == TriStatus::Pending; let at_saved = partial == saved;
    Ok(match phase {
        MatchPhase::Arm => pending && pat_tag == P::PVar && x == 0 && y == 0 && field_idx == 0 && staged.is_none() && at_saved,
        MatchPhase::Tag => pending && matches!(pat_tag, P::PNil | P::PCons | P::PLeaf | P::PBranch) && staged.is_none() && at_saved,
        MatchPhase::Distinct => pending && matches!(pat_tag, P::PCons | P::PBranch) && staged.is_none() && at_saved,
        MatchPhase::Field => pending && matches!(pat_tag, P::PLeaf | P::PCons | P::PBranch) && staged.is_none() && match field_idx {
            0 => at_saved,
            1 => matches!(pat_tag, P::PCons | P::PBranch) && match partial { Some(p) => pre.env_cell(&p)?.previous == saved, None => false },
            _ => false },
        MatchPhase::Bind => pending && staged.is_some() && ((pat_tag == P::PVar && field_idx == 0) || matches!(pat_tag, P::PLeaf | P::PCons | P::PBranch)),
        MatchPhase::Ready => pending && staged.is_none(),
        MatchPhase::NoMatchReady => pending && staged.is_none() && at_saved,
        MatchPhase::Done => staged.is_none() && match status { TriStatus::Positive => true, TriStatus::Negative => at_saved, TriStatus::Pending => false },
    })
}
pub(super) fn si22_helper_ok(pre: &PreImage) -> Result<bool, HostViolation> {
    let tl = pre.task_live();
    let h = match pre.helper() { Some(h) => *h, None => return Err(cv("SI-22 without helper")) };
    Ok(match h {
        HelperControlC::Scalar { .. } => tl == 0 && scalar_state_ok(pre, &h)?,
        HelperControlC::EnvGet { cursor, found, status, .. } => tl == 0 && match status {
            TriStatus::Pending => found.is_none(), TriStatus::Positive => found.is_some() && cursor.is_some(), TriStatus::Negative => found.is_none() && cursor.is_none() },
        HelperControlC::EnvPut { new_root, status, .. } => tl == 0 && match status {
            TriStatus::Pending => new_root.is_none(), TriStatus::Positive => new_root.is_some(), TriStatus::Negative => false },
        HelperControlC::Find { row_cursor, found, status, .. } => { let n = pre.admitted_function_count(); tl == 0 && match status {
            TriStatus::Pending => found.is_none() && row_cursor <= n, TriStatus::Positive => found == Some(row_cursor) && row_cursor < n,
            TriStatus::Negative => found.is_none() && row_cursor == n } }
        HelperControlC::Bind { params, args, p_cursor, a_cursor, partial, dup_cursor, phase, status, .. } => {
            let unwound = p_cursor == params.len && a_cursor == args.len && params.len == args.len && tl >= 1;
            pre.img.tasks.iter().all(|t| matches!(t, HelperTaskC::PairTask { .. })) && match (phase, status) {
                (BindPhase::Pairing, BindStatus::Ok) => p_cursor == tl && a_cursor == tl && p_cursor <= params.len && a_cursor <= args.len && partial.is_none() && dup_cursor.is_none(),
                (BindPhase::UnwindType | BindPhase::UnwindInsert, BindStatus::Ok) => unwound && dup_cursor.is_none(),
                (BindPhase::UnwindDup, BindStatus::Ok) => unwound && dup_cursor.is_some(),
                (BindPhase::Propagate, BindStatus::Bad) => tl >= 1 && dup_cursor.is_none(),
                (BindPhase::Finished, _) => tl == 0 && dup_cursor.is_none(),
                _ => false } }
        HelperControlC::Append { src, src_cursor, dest_start, staged, phase, .. } => {
            let done = if phase == AppendPhase::Done { 1 } else { 0 };
            let len_ok = dest_start.checked_add(src_cursor).and_then(|x| x.checked_add(done)) == Some(pre.active().value_slots);
            tl == 0 && len_ok && match phase {
                AppendPhase::Copy => src_cursor <= src.len && (staged.is_none() || src_cursor < src.len),
                AppendPhase::Tail | AppendPhase::Done => src_cursor == src.len && staged.is_none() } }
        HelperControlC::Validate { status, .. } => match status { TriStatus::Positive => tl == 0, TriStatus::Negative => true, TriStatus::Pending => tasks_ok(pre, false)? },
        HelperControlC::Equal { status, .. } => match status { TriStatus::Positive => tl == 0, TriStatus::Negative => true, TriStatus::Pending => tasks_ok(pre, true)? },
        HelperControlC::Match { .. } => tl == 0 && match_state_ok(pre, &h)?,
        HelperControlC::Build { .. } | HelperControlC::Plan { .. } => tl == 0 && si22_adt_helper_ok(pre, &h)?,
    })
}
