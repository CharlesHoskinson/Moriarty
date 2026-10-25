//! Native ledger validation. Unlike the distributed JS WASM, proof-verifying is enabled.
use midnight_base_crypto::{hash::HashOutput, time::Timestamp};
use midnight_ledger::{
    semantics::{TransactionContext, TransactionResult},
    structure::{ContractAction, LedgerState, ProofMarker, ProofVersioned, Signature, Transaction},
    verify::WellFormedStrictness,
};
use midnight_onchain_runtime::context::BlockContext;
use midnight_serialize::{Deserializable, Serializable, Tagged};
use midnight_storage::db::InMemoryDB;
use midnight_transient_crypto::commitment::PureGeneratorPedersen;
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    io::{self, Cursor, Read},
};

type Tx = Transaction<Signature, ProofMarker, PureGeneratorPedersen, InMemoryDB>;
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    state: String,
    transaction: String,
    block_seconds: u64,
}
fn decode<T: Deserializable + Tagged>(raw: &[u8]) -> Result<T, Box<dyn Error>> {
    let mut reader = Cursor::new(raw);
    let value = midnight_serialize::tagged_deserialize(&mut reader)?;
    if reader.position() != raw.len() as u64 {
        return Err("trailing bytes".into());
    }
    Ok(value)
}
fn encode<T: Serializable + Tagged>(value: &T) -> Vec<u8> {
    let mut raw = Vec::new();
    midnight_serialize::tagged_serialize(value, &mut raw).unwrap();
    raw
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut raw = Vec::new();
    io::stdin()
        .take(32 * 1024 * 1024 + 1)
        .read_to_end(&mut raw)?;
    if raw.len() > 32 * 1024 * 1024 {
        return Err("input exceeds 32 MiB".into());
    }
    let input: Input = serde_json::from_slice(&raw)?;
    let state_raw = hex::decode(input.state)?;
    let tx_raw = hex::decode(input.transaction)?;
    let state: LedgerState<InMemoryDB> = decode(&state_raw)?;
    let tx: Tx = decode(&tx_raw)?;
    if encode(&state) != state_raw || encode(&tx) != tx_raw {
        return Err("noncanonical input".into());
    }
    let Transaction::Standard(standard) = &tx else {
        return Err("expected standard call transaction".into());
    };
    let mut calls = Vec::new();
    for entry in standard.intents.iter() {
        let intent = &entry.1;
        for index in 0..intent.actions.len() {
            let ContractAction::Call(call) = intent.actions.get(index).ok_or("missing action")?
            else {
                return Err("non-call contract action".into());
            };
            let (version, proof) = match &call.proof {
                ProofVersioned::V2(p) => ("V2", p),
                ProofVersioned::V3(p) => ("V3", p),
                _ => return Err("unsupported proof version".into()),
            };
            calls.push(serde_json::json!({"address":hex::encode(call.address.0.0),
                "entryPoint":String::from_utf8_lossy(&call.entry_point.0), "proofVersion":version,
                "proofSHA256":hex::encode(Sha256::digest(&proof.0)),
                "publicInputs":call.public_inputs(intent.binding_commitment.clone().into()).iter()
                    .map(|f| hex::encode(f.as_le_bytes())).collect::<Vec<_>>()}));
        }
    }
    if calls.len() != 1 {
        return Err("expected exactly one contract call".into());
    }
    let tblock = Timestamp::from_secs(input.block_seconds);
    let mut strict = WellFormedStrictness::default();
    strict.enforce_balancing = false;
    strict.verify_native_proofs = false;
    let conditional = tx.well_formed(&state, strict, tblock);
    let (accepted, reason, applied, post_sha256) = match conditional {
        Ok(verified) => {
            let (post, result) = state.apply(
                &verified,
                &TransactionContext {
                    ref_state: state.clone(),
                    block_context: BlockContext {
                        tblock,
                        tblock_err: 30,
                        parent_block_hash: HashOutput([0; 32]),
                        last_block_time: Timestamp::from_secs(
                            input.block_seconds.saturating_sub(6),
                        ),
                    },
                    whitelist: None,
                },
            );
            (
                true,
                None,
                matches!(result, TransactionResult::Success(_)),
                Some(hex::encode(Sha256::digest(encode(&post)))),
            )
        }
        Err(error) => (false, Some(error.to_string()), false, None),
    };
    let all_on = tx.well_formed(&state, WellFormedStrictness::default(), tblock);
    println!(
        "{}",
        serde_json::json!({"schema":"sherlock-native-ledger-validation/1",
        "proofVerification":"native/proof-verifying", "contractCalls":calls, "transactionSHA256":hex::encode(Sha256::digest(&tx_raw)),
        "contractProofsSignaturesLimitsAccepted":accepted, "contractProfileFailure":reason,
        "applied":applied, "postStateSHA256":post_sha256,
        "allOn":all_on.is_ok(), "allOnFailure":all_on.err().map(|e| e.to_string())})
    );
    Ok(())
}
