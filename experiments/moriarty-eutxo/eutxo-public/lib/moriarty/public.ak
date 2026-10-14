//// Public atomic experiment helpers. Not a production or private-claim verifier.
use aiken/interval.{Finite, IntervalBound}
use cardano/transaction.{ValidityRange}

pub const max_u128: Int = 340282366920938463463374607431768211455

pub fn in_u128(n: Int) -> Bool {
  n >= 0 && n <= max_u128
}

pub fn checked_add(a: Int, b: Int) -> Int {
  expect a >= 0 && a <= max_u128
  expect b >= 0 && b <= max_u128
  expect max_u128 - a >= b
  a + b
}

pub fn checked_sub(a: Int, b: Int) -> Int {
  expect a >= 0 && a <= max_u128
  expect b >= 0 && b <= max_u128
  expect a >= b
  a - b
}

pub fn checked_mul(a: Int, b: Int) -> Int {
  expect a >= 0 && a <= max_u128
  expect b >= 0 && b <= max_u128
  if a == 0 || b == 0 {
    0
  } else {
    expect a <= max_u128 / b
    a * b
  }
}

pub fn checked_div(numerator: Int, denominator: Int) -> Int {
  expect numerator >= 0 && numerator <= max_u128
  expect denominator > 0 && denominator <= max_u128
  numerator / denominator
}

/// TIME_PREDICATE_v1
///
/// Bind `obs.now` to the inclusive last POSIX millisecond of the transaction
/// validity interval. Cardano/Plutus V3 validity is a POSIX-millisecond
/// `Interval`. When the upper bound is exclusive (the usual ledger encoding of
/// `[lower, upper)`), last-valid is `upper - 1`. Both bounds must be finite.
/// Unbounded, empty, or inverted intervals reject. This is interval backing,
/// not wall-clock authentication.
pub fn last_valid_posix(range: ValidityRange) -> Int {
  expect Finite(lower) = range.lower_bound.bound_type
  expect Finite(upper) = range.upper_bound.bound_type
  expect range.lower_bound.is_inclusive
  let last =
    if range.upper_bound.is_inclusive {
      upper
    } else {
      upper - 1
    }
  expect last >= lower
  expect last >= 0
  expect last <= max_u128
  last
}
