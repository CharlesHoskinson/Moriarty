//! Hostile claims and witness tampering against [ORACLE] invoice vectors:
//! T1 (with erratum E-T1), T1-cap, T2, T3, T5 and R-PARTIAL-EQ [AUTHOR].
mod common;

use common::*;
use mori_core::{UInt, Value};
use mori_r1cs::*;

fn open() -> (CompiledRelation, TransitionValues) {
    let rel = compile_ok(INVOICE, options("open", CapBinding::PublicCap));
    let oracle = oracle_case("invoice-open");
    let values = oracle_values(&rel, &oracle, Some(oracle.cap));
    (rel, values)
}

fn bump(v: &mut Value, high: bool) {
    match v {
        Value::UInt(UInt(n)) => *n = n.wrapping_add(if high { 1u128 << 64 } else { 1 }),
        Value::Bool(x) => *x = !*x,
    }
}

fn bump_uint(n: &mut UInt, high: bool) {
    n.0 = n.0.wrapping_add(if high { 1u128 << 64 } else { 1 });
}

/// Every single-slot change of a claimed body value, as a new claim.
fn mutations(v: &TransitionValues) -> Vec<(String, TransitionValues)> {
    let mut out = Vec::new();
    for high in [false, true] {
        let tag = if high { "hi" } else { "lo" };
        for i in 0..v.inputs.len() {
            if high && matches!(v.inputs[i], Value::Bool(_)) {
                continue;
            }
            let mut m = v.clone();
            bump(&mut m.inputs[i], high);
            out.push((format!("input{i}.{tag}"), m));
        }
        for (phase, post) in [("pre", false), ("post", true)] {
            let body = if post { &v.post } else { &v.pre };
            for i in 0..body.states.len() {
                if high && matches!(body.states[i], Value::Bool(_)) {
                    continue;
                }
                let mut m = v.clone();
                bump(if post { &mut m.post.states[i] } else { &mut m.pre.states[i] }, high);
                out.push((format!("{phase}.state{i}.{tag}"), m));
            }
            for i in 0..body.cells.len() {
                let mut m = v.clone();
                bump_uint(if post { &mut m.post.cells[i] } else { &mut m.pre.cells[i] }, high);
                out.push((format!("{phase}.cell{i}.{tag}"), m));
            }
            for i in 0..body.duties.len() {
                let mut m = v.clone();
                let d = if post { &mut m.post.duties[i] } else { &mut m.pre.duties[i] };
                if high {
                    bump_uint(&mut d.amount, true);
                    out.push((format!("{phase}.duty{i}.amount.hi"), m));
                } else {
                    d.present = !d.present;
                    out.push((format!("{phase}.duty{i}.present"), m.clone()));
                    let mut m2 = v.clone();
                    let d2 = if post { &mut m2.post.duties[i] } else { &mut m2.pre.duties[i] };
                    bump_uint(&mut d2.amount, false);
                    out.push((format!("{phase}.duty{i}.amount.lo"), m2));
                }
            }
        }
        for i in 0..v.effects.len() {
            for part in 0..2 {
                let mut m = v.clone();
                let changed = match &mut m.effects[i] {
                    EffectValues::State { before, after } => {
                        let x = if part == 0 { before } else { after };
                        if high && matches!(x, Value::Bool(_)) { false } else { bump(x, high); true }
                    }
                    EffectValues::Amount(a) => part == 0 && { bump_uint(a, high); true },
                    EffectValues::Duty { before, after } => {
                        bump_uint(if part == 0 { before } else { after }, high);
                        true
                    }
                    EffectValues::Emit(x) => {
                        part == 0 && !(high && matches!(x, Value::Bool(_))) && { bump(x, high); true }
                    }
                };
                if changed {
                    out.push((format!("effect{i}.part{part}.{tag}"), m));
                }
            }
        }
    }
    out
}

#[test]
fn every_single_slot_change_is_refused_except_the_valid_present_zero_pre_state() {
    let (rel, honest) = open();
    assert!(accepts(&rel, honest.clone()));
    let reserve = rel.plan().duty_index("reserve").unwrap();
    // E-T1 [AUTHOR ADDENDUM-1]: flipping the pre reserve presence gives the valid
    // present-zero pre-state; Core accepts it with identical post and effects.
    let present_zero = format!("pre.duty{reserve}.present");
    let all = mutations(&honest);
    assert!(all.len() > 40);
    for (label, claim) in all {
        match rel.assignment::<ark_bls12_381::Fr>(claim.clone()) {
            // An absent duty with a nonzero amount is not encodable (only duty
            // mutations can produce one); the public-vector forms are refused below.
            Err(MappingError::Shape(_)) => assert!(label.contains(".duty"), "{label}"),
            Err(other) => panic!("{label}: {other}"),
            Ok(a) => {
                let satisfied = run(&rel, a, Tamper::none()).satisfied;
                assert_eq!(satisfied, label == present_zero, "{label}");
            }
        }
    }
    // Duty fields edited in the public vector only (non-canonical forms included).
    for phase in ["pre", "post"] {
        for d in &rel.plan().duties {
            let present = offset(&rel, &format!("{phase}.duty.{}.present", d.name));
            let amount = offset(&rel, &format!("{phase}.duty.{}.amount", d.name));
            assert!(!accepts_public(&rel, honest.clone(), |p| p[amount] += fr(1)), "{phase} {} amount", d.name);
            assert!(!accepts_public(&rel, honest.clone(), |p| p[present] = fr(1) - p[present]), "{phase} {} present", d.name);
        }
    }
}

#[test]
fn every_header_field_is_a_bound_constant() {
    // T3: any header slot changed (program hash half, profile, version, action, shape,
    // compiler build, W) is refused.
    let (rel, honest) = open();
    for slot in rel.layout().slots.iter().filter(|s| s.kind.is_header() && s.label != "header.cap") {
        for k in 0..slot.encoding.width() {
            let at = slot.offset + k;
            assert!(!accepts_public(&rel, honest.clone(), |p| p[at] += fr(1)), "{} field {k}", slot.label);
        }
    }
}

#[test]
fn public_cap_is_a_claimed_upper_bound_on_w() {
    let (rel, honest) = open();
    let mut more = honest.clone();
    more.cap = Some(UInt(39));
    assert!(accepts(&rel, more)); // T1-cap
    let mut less = honest;
    less.cap = Some(UInt(37));
    assert!(!accepts(&rel, less)); // T2
}

#[test]
fn comparison_flag_and_strict_less_than_cannot_be_forged() {
    let rel = compile_ok(INVOICE, options("partial", CapBinding::PublicCap));
    let oracle = oracle_case("invoice-partial");
    let honest = oracle_values(&rel, &oracle, Some(oracle.cap));
    // T5: flip the Lt flag witness of `amount < opening` (statement 2).
    let e = root(&rel, 2);
    let label = format!("s2/e{e}/lt.f#0");
    let out = accepts_tampered(&rel, honest.clone(), Tamper::none().set(label.clone(), fr(0)));
    assert_eq!(out.overrides_used, 1);
    assert!(!out.satisfied);

    // R-PARTIAL-EQ [HOSTILE]: amount 90 equals the opening 90; Core refuses at the
    // `requires amount < opening`. Claim the post a predicate-skipping evaluator
    // would produce, honestly and with the Lt flag forced to true.
    let cell = |name: &str| rel.plan().cells.iter().position(|c| c.account == name).unwrap();
    let invoice = rel.plan().duty_index("invoice").unwrap();
    let paid = rel.plan().state_index("paid").unwrap();
    let mut eq = honest;
    eq.inputs = vec![u(90)];
    eq.post.cells[cell("escrow")] = UInt(17);
    eq.post.cells[cell("fees")] = UInt(16);
    eq.post.cells[cell("seller")] = UInt(103);
    eq.post.duties[invoice] = DutyValue { present: true, amount: UInt(0) };
    eq.post.states[paid] = u(90);
    eq.effects = vec![
        EffectValues::Amount(UInt(2)),
        EffectValues::Amount(UInt(90)),
        EffectValues::Duty { before: UInt(90), after: UInt(0) },
        EffectValues::State { before: u(0), after: u(90) },
    ];
    assert!(!accepts(&rel, eq.clone()));
    let forced = accepts_tampered(&rel, eq, Tamper::none().set(label, fr(1)));
    assert_eq!(forced.overrides_used, 1);
    assert!(!forced.satisfied);
}
