//! Host immediate-operand conversion for sherlock-c6-phys/1 lanes.
//!
//! Generic lanes become `Operand::Immediate` values only. No circuit
//! instruction, native relation, constraint, proof, context or key is emitted.
//! Every lane is a u128, below the 255-bit Fq modulus, and Fq CAPACITY 254 >=
//! 128, so `Fr::from(u128)` is injective and no range guard applies.
//! Legacy-A line/tag/charge constants are not provided: no checked common-A
//! domain projection exists, so none is exported for relabelling.

use midnight_transient_crypto::curve::Fr;
use midnight_zkir_v3::ir::Operand;
use mori_core::sherlock::helper_machine::c6_phys::{C6PushObserved, PushSelection};

pub const C6_A135_FAMILY: &str = "c6.kMachinePushAccept.phys1";
pub const C6_A140_FAMILY: &str = "c6.kMachinePushRefuse.phys1";

pub fn c6_push_family(selection: PushSelection) -> &'static str {
    match selection {
        PushSelection::Accept => C6_A135_FAMILY,
        PushSelection::Refuse => C6_A140_FAMILY,
    }
}

/// Generic lane conversion. Carries no claim about what the lanes encode.
pub fn c6_lanes_to_immediates(lanes: &[u128]) -> Vec<Operand> {
    lanes.iter().map(|&lane| Operand::Immediate(Fr::from(lane))).collect()
}

/// Operands of one recorder-checked Push step. This value is output data and
/// carries no evidence of its own; only its `C6PushObserved` input is checked.
#[derive(Clone, Debug, PartialEq)]
pub struct C6PushOperands {
    pub selection: PushSelection,
    pub before: Vec<Operand>,
    pub after: Vec<Operand>,
}

pub fn emit_c6_push_observed_operands(observed: &C6PushObserved) -> C6PushOperands {
    C6PushOperands {
        selection: observed.selection(),
        before: c6_lanes_to_immediates(observed.before_lanes()),
        after: c6_lanes_to_immediates(observed.after_lanes()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mori_core::sherlock::helper_machine::c6_phys::C6_PHYS_MAGIC;

    #[test]
    fn immediates_are_integer_injective_for_every_u128_lane() {
        let lanes = [0, 1, C6_PHYS_MAGIC, u128::MAX - 1, u128::MAX];
        let back: Vec<u128> = c6_lanes_to_immediates(&lanes)
            .iter()
            .map(|operand| match operand {
                Operand::Immediate(fr) => u128::try_from(*fr).unwrap(),
                Operand::Variable(_) => panic!("immediate expected"),
            })
            .collect();
        assert_eq!(back, lanes);
        assert_eq!(c6_push_family(PushSelection::Accept), C6_A135_FAMILY);
        assert_eq!(c6_push_family(PushSelection::Refuse), C6_A140_FAMILY);
    }
}
