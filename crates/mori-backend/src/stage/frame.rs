//! Source-derived segment layouts and the only emitter of pending-frame roles.
//! Every pending role is an ordinary public binding created by
//! `Builder::public`; this module edits no Builder field except appending the
//! advance guard's `Instruction::Assert`. The relation
//! binds the executing action's own frame only. Linking a stage-in frame to the
//! latest accepted stage-out frame, and isolation of other actions' frames, is
//! the unperformed `pending-frame-predecessor-binding` gate.
use super::PendingPositions;
use crate::{Builder, Error, Wire, error, immediate};
use midnight_zkir_v3::Instruction;
use midnight_zkir_v3::ir::Operand;
use mori_core::stage::{
    CaptureSlot, MAX_AWAITS_PER_ACTION, MAX_CAPTURE_SLOTS, MAX_EVENT_BINDERS, StageBinder,
    StagedProgram,
};
use mori_core::{ExprId, Statement, Type};
use std::ops::Range;

pub(crate) const PRE_SITE_ROLE: &str = "pre_pending_site";
pub(crate) const PRE_CAPTURE_ROLE: &str = "pre_pending_capture";
pub(crate) const EVENT_ROLE: &str = "event";
pub(crate) const POST_SITE_ROLE: &str = "post_pending_site";
pub(crate) const POST_CAPTURE_ROLE: &str = "post_pending_capture";
pub(crate) const PENDING_ROLES: [&str; 5] = [
    PRE_SITE_ROLE,
    PRE_CAPTURE_ROLE,
    EVENT_ROLE,
    POST_SITE_ROLE,
    POST_CAPTURE_ROLE,
];

/// One circuit: segment j of an action with N awaits, in global index order.
/// `slots` is the width-L layout `captures(N)`; slot i >= len is zero padding.
pub(crate) struct SegmentLayout {
    pub action: usize,
    /// Declared action name; the pending site roles are named by it.
    pub action_name: String,
    pub segment: u16,
    pub entry_site: u16,
    pub exit_site: u16,
    /// Original statement ordinals lowered by this circuit.
    pub statements: Range<usize>,
    pub guard: Option<ExprId>,
    pub slots: Vec<CaptureSlot>,
    pub len_in: u16,
    pub len_out: u16,
    pub event: Vec<StageBinder>,
    /// Emitted local steps: the true entry Await counts, the exit Await does not.
    pub replay_steps: u32,
}

/// Pre-pending wires bound into the advance environment. The recorded
/// positions stay private so only `emit_post` can turn them into the ABI.
pub(crate) struct PreFrame {
    /// Live entry captures `slots[0..len_in]`, named by slot.
    pub captures: Vec<(String, Wire)>,
    /// Event binders of the entry site, in declaration order.
    pub events: Vec<(String, Wire)>,
    pre_site_pos: u32,
    pre_capture_pos: Vec<u32>,
    event_pos: Vec<u32>,
}

fn refuse(message: impl Into<String>) -> Error {
    error("artifact", message)
}

/// N + 1 layouts per action, actions in declaration order, segments ascending.
/// The sealed plan is re-checked against the program it was derived from.
pub(crate) fn layouts(sp: &StagedProgram) -> Result<Vec<SegmentLayout>, Error> {
    let program = sp.program();
    let plan = sp.plan();
    if plan.actions.len() != program.actions.len() {
        return Err(refuse("Stage plan does not cover every program action."));
    }
    let mut layouts = Vec::new();
    for (index, (action, ap)) in program.actions.iter().zip(&plan.actions).enumerate() {
        let count = action.statements.len();
        let n = ap.sites.len();
        if ap.action != action.name || usize::from(ap.statement_count) != count {
            return Err(refuse(format!(
                "Stage plan differs from action {:?}.",
                action.name
            )));
        }
        if n > MAX_AWAITS_PER_ACTION
            || ap.segment_cost.len() != n + 1
            || ap.committed_before.len() != n + 1
        {
            return Err(refuse(format!(
                "Stage plan for {:?} lacks exactly N + 1 segments.",
                action.name
            )));
        }
        // Every source Await is a plan site, in order, and nothing else is.
        let awaits: Vec<_> = action
            .statements
            .iter()
            .enumerate()
            .filter_map(|(ordinal, statement)| match statement {
                Statement::Await {
                    label,
                    binders,
                    guard,
                } => Some((ordinal, label, binders, *guard)),
                _ => None,
            })
            .collect();
        if awaits.len() != n {
            return Err(refuse(format!(
                "Stage plan sites differ from the awaits of {:?}.",
                action.name
            )));
        }
        for (k, (site, (ordinal, label, binders, _))) in ap.sites.iter().zip(&awaits).enumerate() {
            if usize::from(site.site) != k + 1
                || usize::from(site.ordinal) != *ordinal
                || site.label != **label
                || site.binders.len() != binders.len()
                || site.binders.len() > MAX_EVENT_BINDERS
                || site
                    .binders
                    .iter()
                    .zip(binders.iter())
                    .any(|(s, b)| s.name != b.name || s.ty != b.ty)
            {
                return Err(refuse(format!(
                    "Stage site {} of {:?} differs from its await.",
                    k + 1,
                    action.name
                )));
            }
        }
        let slots = ap
            .sites
            .last()
            .map_or_else(Vec::new, |site| site.captures.clone());
        if slots.len() > MAX_CAPTURE_SLOTS {
            return Err(refuse("Capture layout exceeds the slot bound."));
        }
        // Prefix monotonicity fixes each slot's name and type across sites.
        let mut previous = 0;
        for site in &ap.sites {
            let len = site.captures.len();
            if len < previous || site.captures[..] != slots[..len] {
                return Err(refuse(format!(
                    "Capture layouts of {:?} are not prefix-monotone.",
                    action.name
                )));
            }
            previous = len;
        }
        let ordinal = |k: usize| usize::from(ap.sites[k - 1].ordinal);
        let width = |k: usize| -> Result<u16, Error> {
            u16::try_from(ap.sites[k - 1].captures.len())
                .map_err(|_| refuse("Capture layout exceeds the slot bound."))
        };
        for j in 0..=n {
            let entry = j;
            let exit = if j == n { 0 } else { j + 1 };
            let start = if entry == 0 { 0 } else { ordinal(entry) + 1 };
            let end = if exit == 0 { count } else { ordinal(exit) };
            let replay = end
                .checked_sub(if entry == 0 { 0 } else { ordinal(entry) })
                .filter(|_| start <= end)
                .ok_or_else(|| refuse("Stage ordinals are not increasing."))?;
            layouts.push(SegmentLayout {
                action: index,
                action_name: action.name.clone(),
                segment: j as u16,
                entry_site: entry as u16,
                exit_site: exit as u16,
                statements: start..end,
                guard: (entry != 0).then(|| awaits[entry - 1].3),
                slots: slots.clone(),
                len_in: if entry == 0 { 0 } else { width(entry)? },
                len_out: if exit == 0 { 0 } else { width(exit)? },
                event: if entry == 0 {
                    Vec::new()
                } else {
                    ap.sites[entry - 1].binders.clone()
                },
                replay_steps: u32::try_from(replay)
                    .map_err(|_| refuse("Replay step count overflow."))?,
            });
        }
    }
    Ok(layouts)
}

/// Zero-await test: holds iff N = 0, since advance segments have entry >= 1.
fn zero(l: &SegmentLayout) -> bool {
    l.entry_site == 0 && l.exit_site == 0
}

fn check_layout(l: &SegmentLayout) -> Result<(), Error> {
    let width = l.slots.len();
    if usize::from(l.len_in) > width
        || usize::from(l.len_out) > width
        || (l.entry_site == 0 && (l.len_in != 0 || !l.event.is_empty() || l.guard.is_some()))
        || (l.entry_site != 0 && (l.entry_site != l.segment || l.guard.is_none()))
        || (l.exit_site == 0 && l.len_out != 0)
        || (l.exit_site != 0 && l.exit_site != l.segment + 1)
    {
        return Err(refuse("Segment layout is inconsistent with its sites."));
    }
    Ok(())
}

/// The position `Builder::public` assigns is the binding count before it.
fn public_at(
    b: &mut Builder,
    role: &str,
    name: &str,
    ty: Type,
    expected: Option<Operand>,
    constant: Option<String>,
) -> Result<(u32, Wire), Error> {
    let before = b.bindings.len();
    let position =
        u32::try_from(before).map_err(|_| error("limit", "Public ABI size overflow."))?;
    let wire = b.public(role, name, ty, expected, constant, None);
    if b.bindings.len() != before + 1 {
        return Err(refuse("Pending role was not appended at its position."));
    }
    Ok((position, wire))
}

/// Pre-pending block, inserted after the pre-liability positions. The site is
/// constrained to the entry constant and every pre slot >= len_in to 0.
pub(crate) fn emit_pre(b: &mut Builder, l: &SegmentLayout) -> Result<Option<PreFrame>, Error> {
    if zero(l) {
        if !l.slots.is_empty() || l.len_in != 0 || l.len_out != 0 || !l.event.is_empty() {
            return Err(refuse("A zero-await circuit has no pending frame."));
        }
        return Ok(None);
    }
    check_layout(l)?;
    let entry = u128::from(l.entry_site);
    let (pre_site_pos, _) = public_at(
        b,
        PRE_SITE_ROLE,
        &l.action_name,
        Type::UInt,
        Some(immediate(entry)),
        Some(entry.to_string()),
    )?;
    let len_in = usize::from(l.len_in);
    let mut captures = Vec::with_capacity(len_in);
    let mut pre_capture_pos = Vec::with_capacity(l.slots.len());
    for (i, slot) in l.slots.iter().enumerate() {
        let live = i < len_in;
        let (position, wire) = public_at(
            b,
            PRE_CAPTURE_ROLE,
            &slot.name,
            slot.ty,
            (!live).then(|| immediate(0)),
            (!live).then(|| "0".to_string()),
        )?;
        pre_capture_pos.push(position);
        if live {
            captures.push((slot.name.clone(), wire));
        }
    }
    let mut events = Vec::with_capacity(l.event.len());
    let mut event_pos = Vec::with_capacity(l.event.len());
    for binder in &l.event {
        let (position, wire) = public_at(b, EVENT_ROLE, &binder.name, binder.ty, None, None)?;
        event_pos.push(position);
        events.push((binder.name.clone(), wire));
    }
    Ok(Some(PreFrame {
        captures,
        events,
        pre_site_pos,
        pre_capture_pos,
        event_pos,
    }))
}

/// An advance segment exists only for a true guard; a false guard has no circuit.
pub(crate) fn assert_guard(b: &mut Builder, guard: Wire) -> Result<(), Error> {
    if guard.ty != Type::Bool {
        return Err(refuse("An await guard must be Bool."));
    }
    b.instructions.push(Instruction::Assert {
        cond: guard.operand,
    });
    Ok(())
}

/// Post-pending block, appended after the last post-liability position. The
/// site is the exit constant; live slots equal the wires the segment actually
/// computed and slots >= len_out equal 0.
pub(crate) fn emit_post(
    b: &mut Builder,
    l: &SegmentLayout,
    pre: Option<&PreFrame>,
    live: &[(&str, Wire)],
) -> Result<Option<PendingPositions>, Error> {
    let pre = match (zero(l), pre) {
        (true, None) => {
            if !live.is_empty() {
                return Err(refuse("A zero-await circuit has no pending captures."));
            }
            return Ok(None);
        }
        (false, Some(pre)) => pre,
        _ => return Err(refuse("Pending pre and post frames disagree.")),
    };
    check_layout(l)?;
    let len_out = usize::from(l.len_out);
    if pre.pre_capture_pos.len() != l.slots.len()
        || pre.event_pos.len() != l.event.len()
        || pre.captures.len() != usize::from(l.len_in)
    {
        return Err(refuse("Pending pre frame differs from the segment layout."));
    }
    if live.len() != len_out
        || live
            .iter()
            .zip(&l.slots)
            .any(|((name, wire), slot)| *name != slot.name || wire.ty != slot.ty)
    {
        return Err(refuse("Live captures differ from the exit layout."));
    }
    let exit = u128::from(l.exit_site);
    let (post_site, _) = public_at(
        b,
        POST_SITE_ROLE,
        &l.action_name,
        Type::UInt,
        Some(immediate(exit)),
        Some(exit.to_string()),
    )?;
    let mut post_captures = Vec::with_capacity(l.slots.len());
    for (i, slot) in l.slots.iter().enumerate() {
        let (expected, constant) = match live.get(i) {
            Some((_, wire)) => (wire.operand.clone(), None),
            None => (immediate(0), Some("0".to_string())),
        };
        let (position, _) = public_at(
            b,
            POST_CAPTURE_ROLE,
            &slot.name,
            slot.ty,
            Some(expected),
            constant,
        )?;
        post_captures.push(position);
    }
    Ok(Some(PendingPositions {
        pre_site: pre.pre_site_pos,
        pre_captures: pre.pre_capture_pos.clone(),
        events: pre.event_pos.clone(),
        post_site,
        post_captures,
    }))
}

// Test-only Builder introspection (H21/H27 structure). Not executed until the
// exact closure is classified; no SDK check, model, VM or proving is used.
#[cfg(test)]
mod tests {
    use super::*;
    use midnight_transient_crypto::curve::Fr;

    const ESCROW: &str = r#"language "moriarty/2";
agreement ContinuationEscrowOracle {
  asset PAY = { network: "local", issuer: "continuation-escrow-oracle", reference: "PAY", scale: 0 };
  state marker: UInt = 77;
  action deliver(amount: UInt) {
    transfer(PAY, "alice", "escrow", amount);
    liability("release", PAY, "escrow", "bob", amount);
    await delivery(ready: Bool) when ready;
    transfer(PAY, "escrow", "bob", amount);
    discharge("release", amount);
  }
}
"#;
    const RESERVE: &str = r#"language "moriarty/2";
agreement ReserveContinuationOracle {
  asset CREDIT = { network: "local", issuer: "continuation-reserve-oracle", reference: "UNIT", scale: 0 };
  state basis: UInt = 40;
  state allocated: UInt = 0;
  state marker: UInt = 77;
  action allocate(extra: UInt) {
    let held = basis + extra;
    next basis = basis - 5;
    liability("allocation", CREDIT, "reserve", "target", held);
    await approval(quote: UInt) when quote >= held;
    next allocated = held;
    transfer(CREDIT, "reserve", "target", held);
    discharge("allocation", held);
    next basis = basis + quote;
    await settlement(ready: Bool) when ready;
    requires quote <= 50;
    fee(CREDIT, "reserve", "treasury", extra);
    emit "settled_quote"(quote);
  }
}
"#;
    const MIXED: &str = r#"language "moriarty/2";
agreement Mixed {
  state n: UInt = 0;
  action a(x: UInt) { await go(ready: Bool) when ready; next n = x; }
  action b() { next n = n + 1; }
}
"#;

    fn load(src: &str) -> StagedProgram {
        mori_core::stage::load(src).expect("frozen oracle loads")
    }
    fn zero_imm() -> Operand {
        Operand::Immediate(Fr::from(0u128))
    }
    fn constrained_to(b: &Builder, wire: &Operand, value: &Operand) -> bool {
        b.instructions.iter().any(|i| {
            matches!(i, Instruction::ConstrainEq { a, b } if a == wire && b == value)
        })
    }
    fn wire_of(b: &Builder, position: u32) -> Operand {
        b.wires[position as usize].clone()
    }
    fn site(l: &[SegmentLayout], action: usize, segment: u16) -> &SegmentLayout {
        l.iter()
            .find(|s| s.action == action && s.segment == segment)
            .unwrap()
    }

    #[test]
    fn escrow_layouts_are_source_derived_with_original_ordinals() {
        let l = layouts(&load(ESCROW)).unwrap();
        let shape: Vec<_> = l
            .iter()
            .map(|s| {
                (
                    s.segment,
                    s.entry_site,
                    s.exit_site,
                    s.statements.clone(),
                    s.len_in,
                    s.len_out,
                    s.replay_steps,
                    s.guard.is_some(),
                )
            })
            .collect();
        assert_eq!(
            shape,
            vec![(0, 0, 1, 0..2, 0, 1, 2, false), (1, 1, 0, 3..5, 1, 0, 3, true)]
        );
        for s in &l {
            assert_eq!(s.action_name, "deliver");
            assert_eq!(
                s.slots.iter().map(|c| c.name.as_str()).collect::<Vec<_>>(),
                vec!["amount"]
            );
        }
        assert!(l[0].event.is_empty());
        assert_eq!(
            l[1].event,
            vec![StageBinder {
                name: "ready".into(),
                ty: Type::Bool
            }]
        );
    }

    #[test]
    fn reserve_layouts_cover_three_segments() {
        let l = layouts(&load(RESERVE)).unwrap();
        let shape: Vec<_> = l
            .iter()
            .map(|s| (s.statements.clone(), s.len_in, s.len_out, s.replay_steps))
            .collect();
        assert_eq!(
            shape,
            vec![(0..3, 0, 2, 3), (4..8, 2, 3, 5), (9..12, 3, 0, 4)]
        );
        assert_eq!(
            l[0].slots.iter().map(|c| c.name.as_str()).collect::<Vec<_>>(),
            vec!["extra", "held", "quote"]
        );
    }

    #[test]
    fn zero_await_action_has_one_segment_and_emits_no_pending_roles() {
        let sp = load(MIXED);
        let l = layouts(&sp).unwrap();
        assert_eq!(
            l.iter()
                .map(|s| (s.action, s.segment, s.entry_site, s.exit_site))
                .collect::<Vec<_>>(),
            vec![(0, 0, 0, 1), (0, 1, 1, 0), (1, 0, 0, 0)]
        );
        let zero_layout = site(&l, 1, 0);
        assert_eq!(zero_layout.statements, 0..1);
        assert_eq!(zero_layout.replay_steps, 1);
        let mut b = Builder::default();
        let pre = emit_pre(&mut b, zero_layout).unwrap();
        assert!(pre.is_none());
        assert!(emit_post(&mut b, zero_layout, None, &[]).unwrap().is_none());
        assert!(b.bindings.is_empty() && b.instructions.is_empty());
    }

    #[test]
    fn start_segment_emits_zero_constrained_pre_frame_and_live_post_frame() {
        let sp = load(ESCROW);
        let l = layouts(&sp).unwrap();
        let mut b = Builder::default();
        let amount = b.public("input", "amount", Type::UInt, None, None, None);
        let pre = emit_pre(&mut b, &l[0]).unwrap().unwrap();
        assert!(pre.captures.is_empty() && pre.events.is_empty());
        let live = [("amount", amount.clone())];
        let p = emit_post(&mut b, &l[0], Some(&pre), &live).unwrap().unwrap();
        assert_eq!(
            (p.pre_site, p.pre_captures.clone(), p.events.clone(), p.post_site, p.post_captures.clone()),
            (1, vec![2], vec![], 3, vec![4])
        );
        let roles: Vec<_> = b
            .bindings
            .iter()
            .map(|x| (x.role.as_str(), x.name.as_str(), x.ty, x.constant.as_deref()))
            .collect();
        assert_eq!(
            roles,
            vec![
                ("input", "amount", Type::UInt, None),
                (PRE_SITE_ROLE, "deliver", Type::UInt, Some("0")),
                (PRE_CAPTURE_ROLE, "amount", Type::UInt, Some("0")),
                (POST_SITE_ROLE, "deliver", Type::UInt, Some("1")),
                (POST_CAPTURE_ROLE, "amount", Type::UInt, None),
            ]
        );
        assert!(constrained_to(&b, &wire_of(&b, p.pre_site), &zero_imm()));
        assert!(constrained_to(&b, &wire_of(&b, p.pre_captures[0]), &zero_imm()));
        assert!(constrained_to(
            &b,
            &wire_of(&b, p.post_site),
            &Operand::Immediate(Fr::from(1u128))
        ));
        // The post capture equals the actual input wire, not a host value.
        assert!(constrained_to(&b, &wire_of(&b, p.post_captures[0]), &amount.operand));
    }

    #[test]
    fn final_segment_binds_entry_captures_and_events_and_zeroes_post_frame() {
        let sp = load(ESCROW);
        let l = layouts(&sp).unwrap();
        let mut b = Builder::default();
        let pre = emit_pre(&mut b, &l[1]).unwrap().unwrap();
        assert_eq!(
            pre.captures.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>(),
            vec!["amount"]
        );
        assert_eq!(
            pre.events.iter().map(|(n, w)| (n.as_str(), w.ty)).collect::<Vec<_>>(),
            vec![("ready", Type::Bool)]
        );
        let guard = pre.events[0].1.clone();
        let before = b.instructions.len();
        assert_guard(&mut b, guard.clone()).unwrap();
        // The guard is one direct Assert on the event wire, not a ConstrainEq.
        assert_eq!(b.instructions.len(), before + 1);
        assert!(matches!(
            &b.instructions[before],
            Instruction::Assert { cond } if *cond == guard.operand
        ));
        assert!(!constrained_to(
            &b,
            &guard.operand,
            &Operand::Immediate(Fr::from(1u128))
        ));
        let p = emit_post(&mut b, &l[1], Some(&pre), &[]).unwrap().unwrap();
        assert_eq!(b.bindings[p.events[0] as usize].role, EVENT_ROLE);
        assert!(b.instructions.iter().any(|i| matches!(
            i,
            Instruction::ConstrainToBoolean { val } if *val == wire_of(&b, p.events[0])
        )));
        assert_eq!(b.bindings[p.post_site as usize].constant.as_deref(), Some("0"));
        assert!(constrained_to(&b, &wire_of(&b, p.post_site), &zero_imm()));
        assert_eq!(
            b.bindings[p.post_captures[0] as usize].constant.as_deref(),
            Some("0")
        );
        assert!(constrained_to(&b, &wire_of(&b, p.post_captures[0]), &zero_imm()));
        // A live pre capture is range-checked, never zero-constrained.
        assert_eq!(b.bindings[p.pre_captures[0] as usize].constant, None);
        assert!(!constrained_to(&b, &wire_of(&b, p.pre_captures[0]), &zero_imm()));
    }

    #[test]
    fn mismatched_frames_and_live_lists_refuse() {
        let sp = load(ESCROW);
        let l = layouts(&sp).unwrap();
        let mut b = Builder::default();
        // A pending layout without its pre frame refuses.
        assert_eq!(
            emit_post(&mut b, &l[0], None, &[]).unwrap_err().code,
            "mori::backend::artifact"
        );
        let pre = emit_pre(&mut b, &l[0]).unwrap().unwrap();
        let wrong = b.public("input", "other", Type::UInt, None, None, None);
        for live in [
            vec![],
            vec![("other", wrong.clone())],
            vec![("amount", Wire { operand: wrong.operand.clone(), ty: Type::Bool })],
            vec![("amount", wrong.clone()), ("amount", wrong.clone())],
        ] {
            assert_eq!(
                emit_post(&mut b, &l[0], Some(&pre), &live).unwrap_err().code,
                "mori::backend::artifact"
            );
        }
        let not_bool = Wire { operand: zero_imm(), ty: Type::UInt };
        let before = b.instructions.len();
        assert_eq!(
            assert_guard(&mut b, not_bool).unwrap_err().code,
            "mori::backend::artifact"
        );
        assert_eq!(b.instructions.len(), before);
        // A zero-await layout refuses a pre frame from another layout.
        let mixed = load(MIXED);
        let ml = layouts(&mixed).unwrap();
        assert_eq!(
            emit_post(&mut b, site(&ml, 1, 0), Some(&pre), &[]).unwrap_err().code,
            "mori::backend::artifact"
        );
    }
}
