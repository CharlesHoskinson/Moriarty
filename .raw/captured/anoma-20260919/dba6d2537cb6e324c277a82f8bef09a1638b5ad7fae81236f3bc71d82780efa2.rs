//! Witness library for the GenericCall resource kind.
//!
//! A GenericCall resource can only be ephemeral. It encodes one or more EVM
//! calls that will be forwarded by the protocol adapter to the
//! `GenericCallForwarder` contract.

use alloy_primitives::{Address, Bytes, U256};
use alloy_sol_types::{SolValue, sol};
pub use anoma_rm_risc0::resource_logic::LogicCircuit;
use anoma_rm_risc0::{
    Digest,
    error::ArmError,
    logic_instance::{AppData, ExpirableBlob, LogicInstance},
    nullifier_key::NullifierKey,
    resource::Resource,
    utils::{bytes_to_words, hash_bytes},
};
use anoma_rm_risc0_gadgets::evm::ForwarderCalldata;
use serde::{Deserialize, Serialize};

pub enum DeletionCriterion {
    Immediately = 0,
    Never = 1,
}

sol! {
    struct Call {
        address to;
        uint256 value;
        bytes data;
    }
}

/// A single EVM call to forward.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenericCall {
    /// Address of the EVM contract to call.
    pub to: Vec<u8>,
    /// Native token value (wei) to send with the call.
    pub value: u128,
    /// ABI-encoded function selector and calldata.
    pub data: Vec<u8>,
}

/// ABI-encodes `calls` as `Call[]` matching `abi.decode(input, (Call[]))` in
/// the `GenericCallForwarder` contract.
pub fn encode_generic_call_forwarder_input(calls: &[GenericCall]) -> Result<Vec<u8>, ArmError> {
    let sol_calls: Vec<Call> = calls
        .iter()
        .map(|c| {
            let to: Address =
                c.to.as_slice()
                    .try_into()
                    .map_err(|_| ArmError::ProveFailed("Invalid call `to` bytes".to_string()))?;
            Ok(Call {
                to,
                value: U256::from(c.value),
                data: Bytes::from(c.data.clone()),
            })
        })
        .collect::<Result<_, ArmError>>()?;

    Ok(sol_calls.abi_encode())
}

/// Computes `label_ref = hash(forwarder_addr)`.
pub fn calculate_label_ref(forwarder_addr: &[u8]) -> Digest {
    hash_bytes(forwarder_addr)
}

/// Computes `value_ref = hash(abi_encode(calls))`.
pub fn calculate_value_ref(encoded_calls: &[u8]) -> Digest {
    hash_bytes(encoded_calls)
}

/// Witness for a single GenericCall resource (consumed or created ephemeral).
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct GenericCallWitness {
    pub resource: Resource,
    pub is_consumed: bool,
    pub action_tree_root: Digest,
    /// Required when `is_consumed == true`.
    pub nf_key: Option<NullifierKey>,
    /// Address of the `GenericCallForwarder` contract.
    pub forwarder_addr: Vec<u8>,
    /// The EVM calls to execute.
    pub calls: Vec<GenericCall>,
}

impl LogicCircuit for GenericCallWitness {
    fn constrain(&self) -> Result<LogicInstance, ArmError> {
        if !self.resource.is_ephemeral {
            return Err(ArmError::ProveFailed(
                "GenericCall resource must be ephemeral".to_string(),
            ));
        }

        let expected_label_ref = calculate_label_ref(&self.forwarder_addr);
        if self.resource.label_ref != expected_label_ref {
            return Err(ArmError::ProveFailed(
                "Invalid resource label_ref".to_string(),
            ));
        }

        let encoded_calls = encode_generic_call_forwarder_input(&self.calls)?;

        let expected_value_ref = calculate_value_ref(&encoded_calls);
        if self.resource.value_ref != expected_value_ref {
            return Err(ArmError::ProveFailed(
                "Invalid resource value_ref".to_string(),
            ));
        }

        let tag = if self.is_consumed {
            let nf_key = self
                .nf_key
                .as_ref()
                .ok_or(ArmError::MissingField("Nullifier key"))?;
            self.resource.nullifier(nf_key)?
        } else {
            self.resource.commitment()
        };

        let forwarder_calldata =
            ForwarderCalldata::from_bytes(&self.forwarder_addr, encoded_calls, vec![]);
        let external_blob = ExpirableBlob {
            blob: bytes_to_words(&forwarder_calldata.encode()),
            deletion_criterion: DeletionCriterion::Immediately as u32,
        };

        let app_data = AppData {
            resource_payload: vec![],
            discovery_payload: vec![],
            external_payload: vec![external_blob],
            application_payload: vec![],
        };

        Ok(LogicInstance {
            tag,
            is_consumed: self.is_consumed,
            root: self.action_tree_root,
            app_data,
        })
    }
}
