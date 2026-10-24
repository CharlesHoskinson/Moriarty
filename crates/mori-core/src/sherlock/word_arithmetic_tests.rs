//! Ordinary author normal test SOURCE. SOURCE_ONLY / UNRUN at preparation.
//! Separate independent oracle values are neither imported nor read here.
use super::*;

#[test]
fn named_word32_components_retain_bits_and_wide_words() {
    assert_eq!(
        add_carry_32(23, 19, CarryBit(true)),
        AddCarry32 {
            low: 43,
            carry_out: CarryBit(false)
        }
    );
    assert_eq!(
        add_carry_32(1 << 31, 1 << 31, CarryBit(true)),
        AddCarry32 {
            low: 1,
            carry_out: CarryBit(true)
        }
    );
    assert_eq!(
        sub_borrow_32(43, 19, BorrowBit(true)),
        SubBorrow32 {
            low: 23,
            borrow_out: BorrowBit(false)
        }
    );
    assert_eq!(
        sub_borrow_32(4, 9, BorrowBit(true)),
        SubBorrow32 {
            low: u32::MAX - 5,
            borrow_out: BorrowBit(true)
        }
    );
    assert_eq!(mul_wide_32(17, 29), MulWide32 { low: 493, high: 0 });
    assert_eq!(mul_wide_32(1 << 31, 4), MulWide32 { low: 0, high: 2 });
    assert_eq!(
        mul_accumulate_32(1 << 31, 4, 9, MulCarry32(17)),
        MulAccumulate32 {
            low: 26,
            carry_out: MulCarry32(2)
        }
    );
}

#[test]
fn named_word64_components_retain_bits_and_wide_words() {
    assert_eq!(
        add_carry_64(23, 19, CarryBit(true)),
        AddCarry64 {
            low: 43,
            carry_out: CarryBit(false)
        }
    );
    assert_eq!(
        add_carry_64(1 << 63, 1 << 63, CarryBit(true)),
        AddCarry64 {
            low: 1,
            carry_out: CarryBit(true)
        }
    );
    assert_eq!(
        sub_borrow_64(43, 19, BorrowBit(true)),
        SubBorrow64 {
            low: 23,
            borrow_out: BorrowBit(false)
        }
    );
    assert_eq!(
        sub_borrow_64(4, 9, BorrowBit(true)),
        SubBorrow64 {
            low: u64::MAX - 5,
            borrow_out: BorrowBit(true)
        }
    );
    assert_eq!(mul_wide_64(17, 29), MulWide64 { low: 493, high: 0 });
    assert_eq!(mul_wide_64(1 << 63, 4), MulWide64 { low: 0, high: 2 });
    assert_eq!(
        mul_accumulate_64(1 << 63, 4, 9, MulCarry64(17)),
        MulAccumulate64 {
            low: 26,
            carry_out: MulCarry64(2)
        }
    );
}

fn join32(limbs: [u32; 4]) -> u128 {
    limbs
        .iter()
        .enumerate()
        .map(|(i, &x)| u128::from(x) << (32 * i))
        .sum()
}

fn join64(limbs: [u64; 2]) -> u128 {
    u128::from(limbs[0]) | (u128::from(limbs[1]) << 64)
}

#[test]
fn both_checked_add_consumers_preserve_normal_cross_limb_results() {
    for (a, b) in [
        (0u128, 17u128),
        ((1 << 48) + 31, (1 << 80) + 73),
        ((1 << 96) + 11, (1 << 32) + 5),
    ] {
        let expected = a + b;
        let r32 = checked_u128_add_32(a, b);
        let r64 = checked_u128_add_64(a, b);
        assert_eq!(r32.checked, Ok(expected));
        assert_eq!(r64.checked, Ok(expected));
        assert_eq!(join32(r32.full.low), expected);
        assert_eq!(join64(r64.full.low), expected);
        assert_eq!(r32.full.carry_out, CarryBit(false));
        assert_eq!(r64.full.carry_out, CarryBit(false));
    }
}

#[test]
fn both_checked_sub_consumers_preserve_normal_cross_limb_results() {
    for (a, b) in [
        (17u128, 0u128),
        ((1 << 80) + 31, (1 << 48) + 73),
        ((1 << 96) + 11, (1 << 32) + 5),
    ] {
        let expected = a - b;
        let r32 = checked_u128_sub_32(a, b);
        let r64 = checked_u128_sub_64(a, b);
        assert_eq!(r32.checked, Ok(expected));
        assert_eq!(r64.checked, Ok(expected));
        assert_eq!(join32(r32.full.low), expected);
        assert_eq!(join64(r64.full.low), expected);
        assert_eq!(r32.full.borrow_out, BorrowBit(false));
        assert_eq!(r64.full.borrow_out, BorrowBit(false));
    }
}

#[test]
fn both_checked_mul_consumers_preserve_every_normal_product_limb() {
    for (a, b) in [
        (0u128, 17u128),
        ((1 << 40) + 31, (1 << 48) + 73),
        ((1 << 60) + 11, (1 << 56) + 5),
    ] {
        let expected = a * b;
        let r32 = checked_u128_mul_32(a, b);
        let r64 = checked_u128_mul_64(a, b);
        assert_eq!(r32.checked, Ok(expected));
        assert_eq!(r64.checked, Ok(expected));
        assert_eq!(join32(r32.full.low), expected);
        assert_eq!(join64(r64.full.low), expected);
        assert_eq!(r32.full.high, [0; 4]);
        assert_eq!(r64.full.high, [0; 2]);
    }
}
