use aiken/interval.{IntervalBoundType}
use aiken/primitive/string
use cardano/transaction.{OutputReference, Transaction}

validator parity_probe {
  spend(_d: Option<Data>, _r: Data, _o: OutputReference, self: Transaction) {
    expect IntervalBoundType.Finite(ms) = self.validity_range.lower_bound.bound_type
    let inclusive = if self.validity_range.lower_bound.is_inclusive { ms } else { ms + 1 }
    fail string.concat(@"MORIARTY_LOWER:", string.from_int(inclusive))
  }
}
