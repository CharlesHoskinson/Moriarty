//! Native ledger validation. Unlike the distributed JS WASM, proof-verifying is enabled.
use midnight_base_crypto::{
    cost_model::{FixedPoint, NormalizedCost},
    hash::HashOutput,
    time::Timestamp,
};
use midnight_ledger::{
    semantics::{TransactionContext, TransactionResult},
    structure::{
        ContractAction, LedgerState, ProofMarker, ProofPreimageMarker, ProofVersioned, Signature,
        Transaction,
    },
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
    block_context: Option<StrictBlockContext>,
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
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct StrictBlockContext {
    uncertainty_seconds: u32,
    last_block_seconds: u64,
    parent_block_hash: String,
}

fn strict_calls(
    state: &LedgerState<InMemoryDB>,
    tx: &Tx,
) -> Result<Vec<serde_json::Value>, Box<dyn Error>> {
    let mut calls = Vec::new();
    for (segment, action) in tx.actions() {
        if let ContractAction::Call(call) = action {
            let contract = state.index(call.address).ok_or("missing contract state")?;
            let operation = contract
                .operations
                .get(&call.entry_point)
                .ok_or("missing operation")?;
            let ProofVersioned::V3(proof) = &call.proof else {
                return Err("strict order replay requires V3 contract proofs".into());
            };
            let key = operation.v3.as_ref().ok_or("missing V3 verifier key")?;
            calls.push(serde_json::json!({
                "segment": segment, "address": hex::encode(call.address.0.0),
                "entryPoint": std::str::from_utf8(&call.entry_point.0)?,
                "proofVersion": "V3", "proofSHA256": hex::encode(Sha256::digest(&proof.0)),
                "verifierKeySHA256": hex::encode(Sha256::digest(encode(key))),
            }));
        }
    }
    Ok(calls)
}

fn strict_replay(
    state: &LedgerState<InMemoryDB>,
    tx_raw: &[u8],
    input: &Input,
    block: &StrictBlockContext,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let tblock = Timestamp::from_secs(input.block_seconds);
    let parent_hash: [u8; 32] = hex::decode(&block.parent_block_hash)?
        .try_into()
        .map_err(|_| "expected 32-byte parent block hash")?;
    type RewardEnvelope =
        Transaction<Signature, ProofPreimageMarker, PureGeneratorPedersen, InMemoryDB>;
    let (verified, calls) =
        if midnight_serialize::peek_tag(&mut Cursor::new(tx_raw))? == RewardEnvelope::tag() {
            let reward: RewardEnvelope = decode(tx_raw)?;
            // The SDK wraps proof-free reward claims in this envelope. Contract
            // calls and all other standard transactions must use proven bytes.
            if !matches!(reward, Transaction::ClaimRewards(_)) || encode(&reward) != tx_raw {
                return Err("expected canonical reward claim".into());
            }
            (
                reward.well_formed(state, WellFormedStrictness::default(), tblock)?,
                Vec::new(),
            )
        } else {
            let tx: Tx = decode(tx_raw)?;
            if encode(&tx) != tx_raw {
                return Err("noncanonical transaction".into());
            }
            (
                tx.well_formed(state, WellFormedStrictness::default(), tblock)?,
                strict_calls(state, &tx)?,
            )
        };
    let (post, result) = state.apply(
        &verified,
        &TransactionContext {
            ref_state: state.clone(),
            block_context: BlockContext {
                tblock,
                tblock_err: block.uncertainty_seconds,
                parent_block_hash: HashOutput(parent_hash),
                last_block_time: Timestamp::from_secs(block.last_block_seconds),
            },
            whitelist: None,
        },
    );
    let outcome = match result {
        TransactionResult::Success(_) => "success",
        TransactionResult::PartialSuccess(_, _) => "partialSuccess",
        TransactionResult::Failure(_) => "failure",
    };
    // Mirror LedgerState.postBlockUpdate(date) from the controlled local SDK.
    let finalized =
        post.post_block_update(tblock, NormalizedCost::ZERO, FixedPoint::from_u64_div(1, 2))?;
    Ok(serde_json::json!({
        "schema": "moriarty-strict-ledger-replay/1", "outcome": outcome,
        "blockContext": block, "blockSeconds": input.block_seconds, "contractCalls": calls,
        "preStateSHA256": hex::encode(Sha256::digest(encode(state))),
        "transactionSHA256": hex::encode(Sha256::digest(tx_raw)),
        "postStateSHA256": hex::encode(Sha256::digest(encode(&post))),
        "afterBlockStateSHA256": hex::encode(Sha256::digest(encode(&finalized))),
    }))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let strict = match args.as_slice() {
        [] => false,
        [arg] if arg == "--strict" => true,
        _ => return Err("usage: validate_call [--strict]".into()),
    };

    let mut raw = Vec::new();
    io::stdin()
        .take(32 * 1024 * 1024 + 1)
        .read_to_end(&mut raw)?;
    if raw.len() > 32 * 1024 * 1024 {
        return Err("input exceeds 32 MiB".into());
    }
    let input: Input = serde_json::from_slice(&raw)?;
    let state_raw = hex::decode(&input.state)?;
    let tx_raw = hex::decode(&input.transaction)?;
    let state: LedgerState<InMemoryDB> = decode(&state_raw)?;
    if encode(&state) != state_raw {
        return Err("noncanonical input".into());
    }
    if strict {
        let block = input
            .block_context
            .as_ref()
            .ok_or("--strict requires block_context")?;
        println!("{}", strict_replay(&state, &tx_raw, &input, block)?);
        return Ok(());
    }
    if input.block_context.is_some() {
        return Err("block_context requires --strict".into());
    }
    let tx: Tx = decode(&tx_raw)?;
    if encode(&tx) != tx_raw {
        return Err("noncanonical transaction".into());
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
                "publicInputs":call.public_inputs(intent.binding_commitment.clone().into())
                    .map_err(|e| format!("public inputs: {e:?}"))?.iter()
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
