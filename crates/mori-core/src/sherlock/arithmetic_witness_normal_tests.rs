//! Literal normal specimens from aux/2 ARITHMETIC-FUEL-DESIGN.md §3.
//! These test host computation only, not constraints or machine acceptance.
use super::arithmetic_witness::{ArithmeticOp, materialize_arithmetic, materialize_fuel};

const ZERO: [u8; 16] = [0; 16];
const MAX: [u8; 16] = [255; 16];

fn small(x: u8) -> [u8; 16] {
    let mut bytes = ZERO;
    bytes[0] = x;
    bytes
}

fn at(i: usize, x: u8) -> [u8; 16] {
    let mut bytes = ZERO;
    bytes[i] = x;
    bytes
}

fn chain(first: usize, last: usize) -> [bool; 17] {
    let mut bits = [false; 17];
    for bit in &mut bits[first..=last] {
        *bit = true;
    }
    bits
}

#[test]
fn owner_add_a1_through_a5() {
    let mut carry_once = [false; 17];
    carry_once[1] = true;
    let mut doubled_max = MAX;
    doubled_max[0] = 254;
    let cases = [
        (ZERO, ZERO, ZERO, [false; 17], false),
        (MAX, ZERO, MAX, [false; 17], false),
        (small(255), small(1), at(1, 1), carry_once, false),
        (MAX, small(1), ZERO, chain(1, 16), true),
        (MAX, MAX, doubled_max, chain(1, 16), true),
    ];
    for (a, b, r, c, ovf) in cases {
        let w = materialize_arithmetic(Some(ArithmeticOp::Add), Some(&a), Some(&b));
        assert_eq!(w.add.r, r);
        assert_eq!(w.add.c, c);
        assert_eq!((w.sel_add, w.uu, w.ovf, w.lt_out), (true, true, ovf, false));
        for i in 0..16 {
            assert_eq!(
                u16::from(a[i]) + u16::from(b[i]) + u16::from(c[i]),
                u16::from(r[i]) + 256 * u16::from(c[i + 1])
            );
        }
    }
}

#[test]
fn owner_sub_s1_through_s4_and_lt_l1_through_l5() {
    let mut high_remainder = MAX;
    high_remainder[0] = 1;
    high_remainder[15] = 0;
    let mut reverse_remainder = ZERO;
    reverse_remainder[0] = 255;
    reverse_remainder[15] = 255;
    let mut negative_two = MAX;
    negative_two[0] = 254;
    let cases = [
        (
            ArithmeticOp::Sub,
            small(1),
            small(1),
            ZERO,
            [false; 17],
            false,
            false,
        ),
        (
            ArithmeticOp::Sub,
            ZERO,
            small(1),
            MAX,
            chain(1, 16),
            true,
            false,
        ),
        (
            ArithmeticOp::Sub,
            at(1, 1),
            small(1),
            small(255),
            chain(1, 1),
            false,
            false,
        ),
        (ArithmeticOp::Sub, MAX, MAX, ZERO, [false; 17], false, false),
        (
            ArithmeticOp::Lt,
            small(3),
            small(5),
            negative_two,
            chain(1, 16),
            false,
            true,
        ),
        (
            ArithmeticOp::Lt,
            small(5),
            small(3),
            small(2),
            [false; 17],
            false,
            false,
        ),
        (
            ArithmeticOp::Lt,
            small(7),
            small(7),
            ZERO,
            [false; 17],
            false,
            false,
        ),
        (
            ArithmeticOp::Lt,
            at(15, 1),
            small(255),
            high_remainder,
            chain(1, 15),
            false,
            false,
        ),
        (
            ArithmeticOp::Lt,
            small(255),
            at(15, 1),
            reverse_remainder,
            chain(16, 16),
            false,
            true,
        ),
    ];
    for (op, a, b, r, bo, ovf, lt) in cases {
        let w = materialize_arithmetic(Some(op), Some(&a), Some(&b));
        assert_eq!(w.sub.r, r);
        assert_eq!(w.sub.bo, bo);
        assert!(w.uu);
        assert_eq!((w.ovf, w.lt_out), (ovf, lt));
        assert_eq!(
            (w.sel_sub, w.sel_lt),
            (op == ArithmeticOp::Sub, op == ArithmeticOp::Lt)
        );
        for i in 0..16 {
            assert_eq!(
                u16::from(a[i]) + 256 * u16::from(bo[i + 1]),
                u16::from(b[i]) + u16::from(bo[i]) + u16::from(r[i])
            );
        }
    }
}

#[test]
fn owner_mul_m1_through_m6_full_product_and_all_carries() {
    let mut c3 = [0; 33];
    c3[1] = 254;
    let mut c4 = [0; 33];
    c4[16] = 1;
    let mut c6 = [0u16; 33];
    // Exact owner M-6 formula, not computed using the candidate recurrence.
    for k in 1..=16 {
        c6[k] = 255 * k as u16 - 1;
    }
    for k in 17..=31 {
        c6[k] = 255 * (32 - k) as u16;
    }
    let mut r3 = ZERO;
    r3[0] = 1;
    r3[1] = 254;
    let mut h6 = MAX;
    h6[0] = 254;
    let cases = [
        (ZERO, MAX, ZERO, ZERO, [0; 33], 0, true),
        (small(1), MAX, MAX, ZERO, [0; 33], 0, true),
        (small(255), small(255), r3, ZERO, c3, 0, true),
        (at(15, 128), small(2), ZERO, small(1), c4, 1, false),
        (at(8, 1), at(8, 1), ZERO, small(1), [0; 33], 1, false),
        (MAX, MAX, small(1), h6, c6, 4079, false),
    ];
    for (a, b, r, h, c, sum, zero) in cases {
        let w = materialize_arithmetic(Some(ArithmeticOp::Mul), Some(&a), Some(&b));
        assert_eq!(w.mul.r, r);
        assert_eq!(w.mul.h, h);
        assert_eq!(w.mul.c, c);
        assert_eq!((w.mul.h_sum, w.mul.h_zero), (sum, zero));
        assert_eq!(
            (w.sel_mul, w.uu, w.ovf, w.lt_out),
            (true, true, !zero, false)
        );
    }
    assert_eq!(c6[16], 4079);
}

#[test]
fn owner_type_separation_t1_through_t4_and_p1_numeric_portion() {
    let one = small(1);
    let three = small(3);
    let disabled = materialize_arithmetic(None, Some(&one), Some(&one));
    // T-1/T-2 caller supplies None for the non-UInt fetched node.
    for w in [
        materialize_arithmetic(Some(ArithmeticOp::Add), Some(&one), None),
        materialize_arithmetic(Some(ArithmeticOp::Lt), None, Some(&three)),
    ] {
        assert!(!w.uu && !w.ovf && !w.lt_out);
        assert_eq!(w.add, disabled.add);
        assert_eq!(w.sub, disabled.sub);
        assert_eq!(w.mul, disabled.mul);
    }
    // T-3 Eq and T-4 Cons map to None; no aux/1 Eq or Cons behavior is tested.
    for _ in ["T-3 Eq", "T-4 Cons"] {
        let w = materialize_arithmetic(None, Some(&one), None);
        assert_eq!(w, disabled);
    }
    // P-1 fetch/VR/append are consumer obligations; these are its UInt bytes.
    let w = materialize_arithmetic(Some(ArithmeticOp::Add), Some(&small(7)), Some(&small(5)));
    assert_eq!(w.add.r, small(12));
    assert!(!w.ovf);
}

#[test]
fn owner_fuel_f1_through_f8_probe_decrement_and_publication() {
    let mut f2_post = ZERO;
    f2_post[0] = 255;
    f2_post[1] = 255;
    let mut f3_post = MAX;
    f3_post[15] = 0;
    let mut f4_post = MAX;
    f4_post[0] = 254;
    let cases = [
        (small(1), false, ZERO, chain(0, 0), true),      // F-1
        (at(2, 1), false, f2_post, chain(0, 2), true),   // F-2
        (at(15, 1), false, f3_post, chain(0, 15), true), // F-3
        (MAX, false, f4_post, chain(0, 0), true),        // F-4
        (ZERO, true, ZERO, [false; 17], false),          // F-5a A080
        (ZERO, false, ZERO, [false; 17], false),         // F-5b A084
        (small(5), true, small(5), [false; 17], false),  // F-6 A082
        (ZERO, true, ZERO, [false; 17], false),          // F-7 A082
        (small(3), false, small(2), chain(0, 0), true),  // F-8
    ];
    for (pre, term, post, bw, charged) in cases {
        let w = materialize_fuel(true, term, &pre);
        assert_eq!((w.post, w.bw, w.charged), (post, bw, charged));
        // Literal inputs: z is true exactly at their zero-byte positions;
        // prefix z stays true only until their first nonzero byte.
        let mut expected_z = [true; 16];
        let mut expected_zp = [false; 17];
        expected_zp[0] = true;
        let first_nonzero = pre.iter().position(|&b| b != 0).unwrap_or(16);
        for i in 0..16 {
            expected_z[i] = pre[i] == 0;
            expected_zp[i + 1] = i < first_nonzero;
            assert_eq!(
                u16::from(post[i]) + u16::from(bw[i]),
                u16::from(pre[i]) + 256 * u16::from(bw[i + 1])
            );
        }
        assert_eq!(w.z, expected_z);
        assert_eq!(w.zp, expected_zp);
        assert!(!w.bw[16]);
    }
}
