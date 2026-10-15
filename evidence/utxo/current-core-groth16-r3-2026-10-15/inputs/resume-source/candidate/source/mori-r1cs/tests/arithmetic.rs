//! UInt128 range/overflow, comparison and strict (non-short-circuit) evaluation.
//! Vectors A1-A18 and tamper rows T4/T7/T8 are [AUTHOR] (AUTHOR-EXPECTATIONS.json,
//! ADDENDUM-1 E-T4); rejected rows claim what a wrapping, field-wrapping or
//! short-circuit evaluator would produce [HOSTILE].
mod common;

use common::*;
use mori_core::UInt;
use mori_r1cs::*;

fn acc_rel(action: &str, cap: CapBinding) -> CompiledRelation {
    compile_ok(ARITH, options(action, cap))
}

/// Claim for add/sub/mul: inputs (a, b), acc pre -> post, one State effect.
fn acc_claim(a: u128, b: u128, pre: u128, post: u128, cap: Option<u128>) -> TransitionValues {
    TransitionValues {
        inputs: vec![u(a), u(b)],
        pre: BodyValues { states: vec![u(pre)], cells: vec![], duties: vec![] },
        post: BodyValues { states: vec![u(post)], cells: vec![], duties: vec![] },
        effects: vec![EffectValues::State { before: u(pre), after: u(post) }],
        cap: cap.map(UInt),
    }
}

#[test]
fn checked_addition_boundaries() {
    let max = constant("u128_max");
    let p127 = constant("two_pow_127");
    let add = acc_rel("add", CapBinding::NotBound);
    assert_eq!(add.plan().logical_work, UInt(4));
    assert!(accepts(&add, acc_claim(max, 0, 0, max, None))); // A1
    assert!(!accepts(&add, acc_claim(max, 1, 0, 0, None))); // A2: wrapping claim
    assert!(!accepts(&add, acc_claim(p127, p127, 0, 0, None))); // A3
    assert!(accepts(&add, acc_claim(p127, constant("two_pow_127_minus_1"), 0, max, None))); // A4
}

#[test]
fn logical_cap_boundary() {
    let max = constant("u128_max");
    let add = acc_rel("add", CapBinding::PublicCap);
    assert!(accepts(&add, acc_claim(max, 0, 0, max, Some(4)))); // A1 with cap 4
    assert!(!accepts(&add, acc_claim(max, 0, 0, max, Some(3)))); // A1-cap3
}

#[test]
fn checked_subtraction_boundaries() {
    let max = constant("u128_max");
    let sub = acc_rel("sub", CapBinding::NotBound);
    assert!(!accepts(&sub, acc_claim(5, 6, 0, max, None))); // A5: wrapping claim
    assert!(accepts(&sub, acc_claim(max, max, 0, 0, None))); // A6
}

#[test]
fn checked_multiplication_keeps_the_256_bit_intermediate_exact() {
    let max = constant("u128_max");
    let p64 = constant("two_pow_64");
    let mul = acc_rel("mul", CapBinding::NotBound);
    assert!(!accepts(&mul, acc_claim(p64, p64, 0, 0, None))); // A7: 2^128 wraps to 0
    assert!(accepts(&mul, acc_claim(constant("two_pow_64_minus_1"), constant("two_pow_64_plus_1"), 0, max, None))); // A8
    assert!(accepts(&mul, acc_claim(max, 1, 0, max, None))); // A9
    assert!(accepts(&mul, acc_claim(0, max, 0, 0, None))); // A10
}

#[test]
fn field_wrapped_product_is_refused_for_the_honest_and_the_hostile_witness() {
    // A11 / T7: a = u128_max, b = ceil(r / a); a*b mod r < 2^128 although a*b >= 2^128.
    let max = constant("u128_max");
    let probe_b = constant("wrap_probe_b");
    let wrapped = constant("wrap_probe_product_mod_r");
    let mul = acc_rel("mul", CapBinding::NotBound);
    let claim = acc_claim(max, probe_b, 0, wrapped, None);
    assert!(!accepts(&mul, claim.clone()));
    // Hostile prover: force the product output bits to the field-wrapped value, leave
    // every other witness honest. The a1*b1 = 0 check and the exact recomposition both
    // refuse; no choice of the limb products can help (gadgets::mul).
    let e = root(&mul, 0);
    let tamper = uint_bits(Tamper::none(), &format!("s0/e{e}/mul.c"), wrapped);
    let out = accepts_tampered(&mul, claim, tamper);
    assert_eq!(out.overrides_used, 128, "the mutation reached every product bit");
    assert!(!out.satisfied);
}

fn cmp_claim(a: u128, b_: u128, acc: u128, flags: [bool; 6]) -> TransitionValues {
    TransitionValues {
        inputs: vec![u(a), u(b_)],
        pre: BodyValues { states: vec![u(acc)], cells: vec![], duties: vec![] },
        post: BodyValues { states: vec![u(acc)], cells: vec![], duties: vec![] },
        effects: flags.iter().map(|f| EffectValues::Emit(b(*f))).collect(),
        cap: None,
    }
}

#[test]
fn comparisons_and_their_complements() {
    let max = constant("u128_max");
    let cmp = acc_rel("cmp", CapBinding::NotBound);
    assert_eq!(cmp.plan().logical_work, UInt(24));
    // order: lt, le, gt, ge, eq, ne
    let a12 = [false, true, false, true, true, false];
    let a13 = [false, false, true, true, false, true];
    let a14 = [true, true, false, false, false, true];
    assert!(accepts(&cmp, cmp_claim(3, 3, 9, a12)));
    assert!(accepts(&cmp, cmp_claim(max, 0, 9, a13)));
    assert!(accepts(&cmp, cmp_claim(0, max, 9, a14)));
    // Any single flipped flag is refused.
    for i in 0..6 {
        let mut wrong = a12;
        wrong[i] = !wrong[i];
        assert!(!accepts(&cmp, cmp_claim(3, 3, 9, wrong)), "flag {i}");
    }
    // Hostile prover flips the Lt flag witness of `a < b` (statement 0).
    let e = root(&cmp, 0);
    let out = accepts_tampered(&cmp, cmp_claim(3, 3, 9, [true, true, false, true, true, false]), Tamper::none().set(format!("s0/e{e}/lt.f#0"), fr(1)));
    assert_eq!(out.overrides_used, 1);
    assert!(!out.satisfied);
}

fn strict_claim(flag: bool, a: u128, b_: u128) -> TransitionValues {
    TransitionValues {
        inputs: vec![b(flag), u(a), u(b_)],
        pre: BodyValues { states: vec![u(9)], cells: vec![], duties: vec![] },
        post: BodyValues { states: vec![u(9)], cells: vec![], duties: vec![] },
        effects: vec![],
        cap: None,
    }
}

#[test]
fn or_evaluates_both_operands() {
    let strict = acc_rel("strict", CapBinding::NotBound);
    assert_eq!(strict.plan().logical_work, UInt(8));
    // A15 / T8: flag true but a - b underflows; a short-circuit lowering would accept.
    assert!(!accepts(&strict, strict_claim(true, 0, 1)));
    assert!(accepts(&strict, strict_claim(true, 1, 1))); // A16
    assert!(accepts(&strict, strict_claim(false, 2, 2))); // A17
    assert!(!accepts(&strict, strict_claim(false, 3, 2))); // A18
}

#[test]
fn limbs_and_bools_are_canonical() {
    // E-T4: (lo + 2^64, hi - 1) on input `a` = u128_max denotes the same integer under
    // a naive recomposition; the bit-sum binding refuses it.
    let max = constant("u128_max");
    let add = acc_rel("add", CapBinding::NotBound);
    let at = offset(&add, "input.a");
    assert!(!accepts_public(&add, acc_claim(max, 0, 0, max, None), |p| {
        p[at] += fr(1u128 << 64);
        p[at + 1] -= fr(1);
    }));
    // A Bool slot set to 2.
    let cmp = acc_rel("cmp", CapBinding::NotBound);
    let flag = offset(&cmp, "effect.0.emit.lt.value");
    assert!(!accepts_public(&cmp, cmp_claim(3, 3, 9, [false, true, false, true, true, false]), |p| p[flag] = fr(2)));
}
