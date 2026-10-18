#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RunIntC { Negative, Nat(u128), NatAboveU128 }
    // Negative fuel/stack: genesis-invalid R2. NatAboveU128 fuel: CapacityNotAdmitted{MaxReferenceWork}; stack: {Frames}. Never bool+u128.
