//! Independent normal literals, source-only/uncompiled. Derived from frozen UIREQ030 inputs.
//! Frozen public declaration SHA256: 61b764e2aff77c751975372e2a2d94b5c10c42261c412fda02c0a95746ebaa61
//! No expected values are computed from actual producer output. Root owns registration/run.
use super::{
    CarryBit,
    BorrowBit,
    MulCarry32,
    AddCarry32,
    SubBorrow32,
    MulWide32,
    MulAccumulate32,
    add_carry_32,
    sub_borrow_32,
    mul_wide_32,
    mul_accumulate_32,
    checked_u128_add_32,
    checked_u128_sub_32,
    checked_u128_mul_32,
    MulCarry64,
    AddCarry64,
    SubBorrow64,
    MulWide64,
    MulAccumulate64,
    add_carry_64,
    sub_borrow_64,
    mul_wide_64,
    mul_accumulate_64,
    checked_u128_add_64,
    checked_u128_sub_64,
    checked_u128_mul_64
};

#[test]
fn independent_d26_word32() {
    assert_eq!(add_carry_32(4294967293u32, 8u32, CarryBit(false)), AddCarry32 { low: 5u32, carry_out: CarryBit(true) }, "component 0");
    assert_eq!(add_carry_32(0u32, 0u32, CarryBit(true)), AddCarry32 { low: 1u32, carry_out: CarryBit(false) }, "component 1");
    assert_eq!(add_carry_32(0u32, 0u32, CarryBit(false)), AddCarry32 { low: 0u32, carry_out: CarryBit(false) }, "component 2");
    assert_eq!(add_carry_32(0u32, 0u32, CarryBit(false)), AddCarry32 { low: 0u32, carry_out: CarryBit(false) }, "component 3");
    let actual = checked_u128_add_32(4294967293u128, 8u128);
    assert_eq!(actual.full.low, [5u32, 1u32, 0u32, 0u32]);
    assert_eq!(actual.full.carry_out, CarryBit(false));
    assert_eq!(actual.checked, Ok(4294967301u128));
}

#[test]
fn independent_d26_word64() {
    assert_eq!(add_carry_64(4294967293u64, 8u64, CarryBit(false)), AddCarry64 { low: 4294967301u64, carry_out: CarryBit(false) }, "component 0");
    assert_eq!(add_carry_64(0u64, 0u64, CarryBit(false)), AddCarry64 { low: 0u64, carry_out: CarryBit(false) }, "component 1");
    let actual = checked_u128_add_64(4294967293u128, 8u128);
    assert_eq!(actual.full.low, [4294967301u64, 0u64]);
    assert_eq!(actual.full.carry_out, CarryBit(false));
    assert_eq!(actual.checked, Ok(4294967301u128));
}

#[test]
fn independent_d27_word32() {
    assert_eq!(sub_borrow_32(9u32, 7u32, BorrowBit(false)), SubBorrow32 { low: 2u32, borrow_out: BorrowBit(false) }, "component 0");
    assert_eq!(sub_borrow_32(0u32, 0u32, BorrowBit(false)), SubBorrow32 { low: 0u32, borrow_out: BorrowBit(false) }, "component 1");
    assert_eq!(sub_borrow_32(1u32, 0u32, BorrowBit(false)), SubBorrow32 { low: 1u32, borrow_out: BorrowBit(false) }, "component 2");
    assert_eq!(sub_borrow_32(0u32, 0u32, BorrowBit(false)), SubBorrow32 { low: 0u32, borrow_out: BorrowBit(false) }, "component 3");
    let actual = checked_u128_sub_32(18446744073709551625u128, 7u128);
    assert_eq!(actual.full.low, [2u32, 0u32, 1u32, 0u32]);
    assert_eq!(actual.full.borrow_out, BorrowBit(false));
    assert_eq!(actual.checked, Ok(18446744073709551618u128));
}

#[test]
fn independent_d27_word64() {
    assert_eq!(sub_borrow_64(9u64, 7u64, BorrowBit(false)), SubBorrow64 { low: 2u64, borrow_out: BorrowBit(false) }, "component 0");
    assert_eq!(sub_borrow_64(1u64, 0u64, BorrowBit(false)), SubBorrow64 { low: 1u64, borrow_out: BorrowBit(false) }, "component 1");
    let actual = checked_u128_sub_64(18446744073709551625u128, 7u128);
    assert_eq!(actual.full.low, [2u64, 1u64]);
    assert_eq!(actual.full.borrow_out, BorrowBit(false));
    assert_eq!(actual.checked, Ok(18446744073709551618u128));
}

#[test]
fn independent_d28_word32() {
    assert_eq!(mul_wide_32(1u32, 3u32), MulWide32 { low: 3u32, high: 0u32 }, "component 0");
    assert_eq!(mul_wide_32(1u32, 0u32), MulWide32 { low: 0u32, high: 0u32 }, "component 1");
    assert_eq!(mul_wide_32(1u32, 0u32), MulWide32 { low: 0u32, high: 0u32 }, "component 2");
    assert_eq!(mul_wide_32(1u32, 0u32), MulWide32 { low: 0u32, high: 0u32 }, "component 3");
    assert_eq!(mul_wide_32(65536u32, 3u32), MulWide32 { low: 196608u32, high: 0u32 }, "component 4");
    assert_eq!(mul_wide_32(65536u32, 0u32), MulWide32 { low: 0u32, high: 0u32 }, "component 5");
    assert_eq!(mul_wide_32(65536u32, 0u32), MulWide32 { low: 0u32, high: 0u32 }, "component 6");
    assert_eq!(mul_wide_32(65536u32, 0u32), MulWide32 { low: 0u32, high: 0u32 }, "component 7");
    assert_eq!(mul_wide_32(0u32, 3u32), MulWide32 { low: 0u32, high: 0u32 }, "component 8");
    assert_eq!(mul_wide_32(0u32, 0u32), MulWide32 { low: 0u32, high: 0u32 }, "component 9");
    assert_eq!(mul_wide_32(0u32, 0u32), MulWide32 { low: 0u32, high: 0u32 }, "component 10");
    assert_eq!(mul_wide_32(0u32, 0u32), MulWide32 { low: 0u32, high: 0u32 }, "component 11");
    assert_eq!(mul_wide_32(0u32, 3u32), MulWide32 { low: 0u32, high: 0u32 }, "component 12");
    assert_eq!(mul_wide_32(0u32, 0u32), MulWide32 { low: 0u32, high: 0u32 }, "component 13");
    assert_eq!(mul_wide_32(0u32, 0u32), MulWide32 { low: 0u32, high: 0u32 }, "component 14");
    assert_eq!(mul_wide_32(0u32, 0u32), MulWide32 { low: 0u32, high: 0u32 }, "component 15");
    let actual = checked_u128_mul_32(281474976710657u128, 3u128);
    assert_eq!(actual.full.low, [3u32, 196608u32, 0u32, 0u32]);
    assert_eq!(actual.full.high, [0u32, 0u32, 0u32, 0u32]);
    let actual_full_256 = [actual.full.low[0], actual.full.low[1], actual.full.low[2], actual.full.low[3], actual.full.high[0], actual.full.high[1], actual.full.high[2], actual.full.high[3]];
    assert_eq!(actual_full_256, [3u32, 196608u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]);
    assert_eq!(actual.checked, Ok(844424930131971u128));
}

#[test]
fn independent_d28_word64() {
    assert_eq!(mul_wide_64(281474976710657u64, 3u64), MulWide64 { low: 844424930131971u64, high: 0u64 }, "component 0");
    assert_eq!(mul_wide_64(281474976710657u64, 0u64), MulWide64 { low: 0u64, high: 0u64 }, "component 1");
    assert_eq!(mul_wide_64(0u64, 3u64), MulWide64 { low: 0u64, high: 0u64 }, "component 2");
    assert_eq!(mul_wide_64(0u64, 0u64), MulWide64 { low: 0u64, high: 0u64 }, "component 3");
    let actual = checked_u128_mul_64(281474976710657u128, 3u128);
    assert_eq!(actual.full.low, [844424930131971u64, 0u64]);
    assert_eq!(actual.full.high, [0u64, 0u64]);
    let actual_full_256 = [actual.full.low[0], actual.full.low[1], actual.full.high[0], actual.full.high[1]];
    assert_eq!(actual_full_256, [844424930131971u64, 0u64, 0u64, 0u64]);
    assert_eq!(actual.checked, Ok(844424930131971u128));
}

#[test]
fn independent_n01_full_normal_square_word32() {
    assert_eq!(mul_wide_32(4294967295u32, 4294967295u32), MulWide32 { low: 1u32, high: 4294967294u32 }, "component 0");
    assert_eq!(mul_wide_32(4294967295u32, 4294967295u32), MulWide32 { low: 1u32, high: 4294967294u32 }, "component 1");
    assert_eq!(mul_wide_32(4294967295u32, 0u32), MulWide32 { low: 0u32, high: 0u32 }, "component 2");
    assert_eq!(mul_wide_32(4294967295u32, 0u32), MulWide32 { low: 0u32, high: 0u32 }, "component 3");
    assert_eq!(mul_wide_32(4294967295u32, 4294967295u32), MulWide32 { low: 1u32, high: 4294967294u32 }, "component 4");
    assert_eq!(mul_wide_32(4294967295u32, 4294967295u32), MulWide32 { low: 1u32, high: 4294967294u32 }, "component 5");
    assert_eq!(mul_wide_32(4294967295u32, 0u32), MulWide32 { low: 0u32, high: 0u32 }, "component 6");
    assert_eq!(mul_wide_32(4294967295u32, 0u32), MulWide32 { low: 0u32, high: 0u32 }, "component 7");
    assert_eq!(mul_wide_32(0u32, 4294967295u32), MulWide32 { low: 0u32, high: 0u32 }, "component 8");
    assert_eq!(mul_wide_32(0u32, 4294967295u32), MulWide32 { low: 0u32, high: 0u32 }, "component 9");
    assert_eq!(mul_wide_32(0u32, 0u32), MulWide32 { low: 0u32, high: 0u32 }, "component 10");
    assert_eq!(mul_wide_32(0u32, 0u32), MulWide32 { low: 0u32, high: 0u32 }, "component 11");
    assert_eq!(mul_wide_32(0u32, 4294967295u32), MulWide32 { low: 0u32, high: 0u32 }, "component 12");
    assert_eq!(mul_wide_32(0u32, 4294967295u32), MulWide32 { low: 0u32, high: 0u32 }, "component 13");
    assert_eq!(mul_wide_32(0u32, 0u32), MulWide32 { low: 0u32, high: 0u32 }, "component 14");
    assert_eq!(mul_wide_32(0u32, 0u32), MulWide32 { low: 0u32, high: 0u32 }, "component 15");
    let actual = checked_u128_mul_32(18446744073709551615u128, 18446744073709551615u128);
    assert_eq!(actual.full.low, [1u32, 0u32, 4294967294u32, 4294967295u32]);
    assert_eq!(actual.full.high, [0u32, 0u32, 0u32, 0u32]);
    let actual_full_256 = [actual.full.low[0], actual.full.low[1], actual.full.low[2], actual.full.low[3], actual.full.high[0], actual.full.high[1], actual.full.high[2], actual.full.high[3]];
    assert_eq!(actual_full_256, [1u32, 0u32, 4294967294u32, 4294967295u32, 0u32, 0u32, 0u32, 0u32]);
    assert_eq!(actual.checked, Ok(340282366920938463426481119284349108225u128));
}

#[test]
fn independent_n01_full_normal_square_word64() {
    assert_eq!(mul_wide_64(18446744073709551615u64, 18446744073709551615u64), MulWide64 { low: 1u64, high: 18446744073709551614u64 }, "component 0");
    assert_eq!(mul_wide_64(18446744073709551615u64, 0u64), MulWide64 { low: 0u64, high: 0u64 }, "component 1");
    assert_eq!(mul_wide_64(0u64, 18446744073709551615u64), MulWide64 { low: 0u64, high: 0u64 }, "component 2");
    assert_eq!(mul_wide_64(0u64, 0u64), MulWide64 { low: 0u64, high: 0u64 }, "component 3");
    let actual = checked_u128_mul_64(18446744073709551615u128, 18446744073709551615u128);
    assert_eq!(actual.full.low, [1u64, 18446744073709551614u64]);
    assert_eq!(actual.full.high, [0u64, 0u64]);
    let actual_full_256 = [actual.full.low[0], actual.full.low[1], actual.full.high[0], actual.full.high[1]];
    assert_eq!(actual_full_256, [1u64, 18446744073709551614u64, 0u64, 0u64]);
    assert_eq!(actual.checked, Ok(340282366920938463426481119284349108225u128));
}

#[test]
fn independent_n02_normal_cross_word_borrow_word32() {
    assert_eq!(sub_borrow_32(1u32, 2u32, BorrowBit(false)), SubBorrow32 { low: 4294967295u32, borrow_out: BorrowBit(true) }, "component 0");
    assert_eq!(sub_borrow_32(0u32, 0u32, BorrowBit(true)), SubBorrow32 { low: 4294967295u32, borrow_out: BorrowBit(true) }, "component 1");
    assert_eq!(sub_borrow_32(1u32, 0u32, BorrowBit(true)), SubBorrow32 { low: 0u32, borrow_out: BorrowBit(false) }, "component 2");
    assert_eq!(sub_borrow_32(0u32, 0u32, BorrowBit(false)), SubBorrow32 { low: 0u32, borrow_out: BorrowBit(false) }, "component 3");
    let actual = checked_u128_sub_32(18446744073709551617u128, 2u128);
    assert_eq!(actual.full.low, [4294967295u32, 4294967295u32, 0u32, 0u32]);
    assert_eq!(actual.full.borrow_out, BorrowBit(false));
    assert_eq!(actual.checked, Ok(18446744073709551615u128));
}

#[test]
fn independent_n02_normal_cross_word_borrow_word64() {
    assert_eq!(sub_borrow_64(1u64, 2u64, BorrowBit(false)), SubBorrow64 { low: 18446744073709551615u64, borrow_out: BorrowBit(true) }, "component 0");
    assert_eq!(sub_borrow_64(1u64, 0u64, BorrowBit(true)), SubBorrow64 { low: 0u64, borrow_out: BorrowBit(false) }, "component 1");
    let actual = checked_u128_sub_64(18446744073709551617u128, 2u128);
    assert_eq!(actual.full.low, [18446744073709551615u64, 0u64]);
    assert_eq!(actual.full.borrow_out, BorrowBit(false));
    assert_eq!(actual.checked, Ok(18446744073709551615u128));
}

#[test]
fn independent_mac_word32() {
    let actual = mul_accumulate_32(4294967295u32, 4294967295u32, 2u32, MulCarry32(3u32));
    assert_eq!(actual, MulAccumulate32 { low: 6u32, carry_out: MulCarry32(4294967294u32) });
}

#[test]
fn independent_mac_word64() {
    let actual = mul_accumulate_64(18446744073709551615u64, 18446744073709551615u64, 2u64, MulCarry64(3u64));
    assert_eq!(actual, MulAccumulate64 { low: 6u64, carry_out: MulCarry64(18446744073709551614u64) });
}
