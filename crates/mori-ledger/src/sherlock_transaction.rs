//! Scoped native `midnight-ledger` transaction and deployment adapter for the
//! Sherlock A102 Literal relation (`LiteralLayout`: capacity-parameterized
//! statement with two context limbs, `ProgramBlock`, `pre` and `post` `StateBlock`s,
//! and `rule_tag`/`charge` envelope lanes; native execution unobserved).
//!
//! This module bridges [`crate::sherlock_literal`]'s [`ProvenIntent`] and
//! [`ContractOperation`] artifacts to the pinned `midnight-ledger`
//! (`04c9c5d9bcebb8d4427d8589fb54d58a55599c14`) [`ContractDeploy`], [`Intent`],
//! and [`Transaction`] structures with canonical tagged serialization and
//! source-derived downstream extraction.
//!
//! Local transaction assembly or byte serialization is NOT chain settlement.
//! Historical C8 and runtime state-cost note: `midnight-ledger`'s
//! `StandardTransaction::sequencing_check` (`ledger/src/verify.rs:1154`) rejects
//! any `ContractCall` whose `guaranteed_transcript` and `fallible_transcript`
//! are both `None` (`SequencingCheckError::CallHasEmptyTranscripts`), and
//! `LedgerState::apply_actions` -> `QueryContext::run_transcript` ->
//! `QueryContext::query` (`onchain-runtime/src/context.rs`, `QueryContext::query`
//! and `QueryContext::run_transcript`) rejects zero-gas empty-program
//! transcripts with `OnchainProgramError::OutOfGas` whenever RC-map accounting
//! costs in `ChargedState::update` are non-zero. `sherlock_literal` therefore
//! binds calls to `Some(empty_verified_transcript(address, pre_call_state, cost_model))`
//! in `guaranteed_transcript` (exact non-zero `RunningCost` from `QueryContext::query`
//! verified by `QueryContext::run_transcript`, `Effects::default()`, empty `program`,
//! `Transcript::VERSION` `2.3`) and `fallible_transcript: None`, while refusing
//! `(None, None)` and zero-gas transcripts fail-closed.

use midnight_base_crypto::{
    cost_model::RunningCost, hash::HashOutput, rng::SplittableRng, time::Timestamp,
};
use midnight_coin_structure::contract::ContractAddress;
use midnight_ledger::{
    error::{MalformedTransaction, SequencingCheckError},
    structure::{
        ContractAction, ContractDeploy, GUARANTEED_SEGMENT, Intent, LedgerState, ProofMarker,
        Segment, Signature, StandardTransaction, Transaction, VerifiedTransaction,
    },
    verify::{ProofVerificationMode, WellFormedStrictness},
};
use midnight_onchain_runtime::{context::QueryContext, cost_model::CostModel};
use midnight_onchain_state::state::{
    ChargedState, ContractMaintenanceAuthority, ContractOperation, ContractState, EntryPointBuf,
    StateValue,
};
use midnight_storage::{
    arena::Sp,
    db::InMemoryDB,
    storage::{Array as StorageArray, HashMap},
};
use midnight_transient_crypto::commitment::{Pedersen, PedersenRandomness, PureGeneratorPedersen};
use rand::{CryptoRng, Rng, SeedableRng, rngs::StdRng};

use crate::{
    Error, Result, codec,
    prepare::RawIntent,
    proving::ProvenIntent,
    sherlock_literal::{
        self, SherlockCallOpening, SherlockLedgerKeys, SherlockLocalCallVerification,
    },
};

const LAYER: &str = "sherlock-transaction";

fn refusal(code: &'static str) -> Error {
    Error::new(LAYER, code)
}

/// Unsealed intent parameterized with real ledger [`Signature`] and [`ProofMarker`].
pub type UnsealedSignedIntent = Intent<Signature, ProofMarker, PedersenRandomness, InMemoryDB>;

/// Sealed intent carrying a [`PureGeneratorPedersen`] Schnorr binding proof.
pub type SealedSignedIntent = Intent<Signature, ProofMarker, PureGeneratorPedersen, InMemoryDB>;

/// Unsealed standard or claim transaction prior to [`Transaction::seal`].
pub type UnsealedTransaction = Transaction<Signature, ProofMarker, PedersenRandomness, InMemoryDB>;

/// Final sealed transaction carrying [`PureGeneratorPedersen`] intent bindings.
pub type SealedTransaction = Transaction<Signature, ProofMarker, PureGeneratorPedersen, InMemoryDB>;

/// Explicit caller-supplied deployment inputs required to construct a [`ContractDeploy`].
///
/// No default maintenance authority or implicit nonce is fabricated.
#[derive(Clone, Debug)]
pub struct SherlockDeploySpec {
    pub entry_point: String,
    pub initial_data: StateValue<InMemoryDB>,
    pub maintenance_authority: ContractMaintenanceAuthority,
    pub deploy_nonce: HashOutput,
}

/// Explicit caller-supplied transaction envelope parameters.
#[derive(Clone, Debug)]
pub struct SherlockTransactionSpec {
    pub network_id: String,
    pub segment: Segment,
    pub ttl: Timestamp,
    pub binding_commitment: PedersenRandomness,
    pub seal_seed: [u8; 32],
}

/// Explicit fee/funding policy for assembling a [`SealedTransaction`].
///
/// Never fabricates zero-fee chain acceptance.
#[derive(Clone, Copy, Debug)]
pub enum SherlockFeePolicy<'a> {
    /// Explicitly assemble an unfunded candidate transaction without fee inputs.
    /// Suitable for local serialization and structural extraction only; full
    /// ledger balancing will reject unfunded transactions.
    ExplicitUnfundedCandidate,
    /// Merge an externally constructed, signed, unsealed fee candidate transaction
    /// (in disjoint non-zero segments) via [`Transaction::merge`] before sealing.
    /// Merging an external candidate does NOT establish fee sufficiency or chain funding.
    MergeUnsealedFeeTransaction(&'a UnsealedTransaction),
}

/// Prepared contract deployment artifact and its deterministic [`ContractAddress`].
#[derive(Clone, Debug)]
pub struct SherlockPreparedDeploy {
    pub contract_address: ContractAddress,
    pub deploy: ContractDeploy<InMemoryDB>,
    pub deploy_bytes: Vec<u8>,
    pub initial_state_bytes: Vec<u8>,
    pub operation_bytes: Vec<u8>,
}

/// Prepared sealed deployment transaction and canonical tagged encodings.
#[derive(Clone, Debug)]
pub struct SherlockPreparedDeployTransaction {
    pub prepared_deploy: SherlockPreparedDeploy,
    pub segment: Segment,
    /// True only when an external unsealed fee-candidate transaction was merged;
    /// does NOT claim that fees are sufficient or paid on chain.
    pub external_fee_candidate_merged: bool,
    pub unsealed_transaction: UnsealedTransaction,
    pub transaction: SealedTransaction,
    pub transaction_bytes: Vec<u8>,
}

/// Prepared sealed Sherlock call transaction and canonical tagged encodings.
#[derive(Clone, Debug)]
pub struct SherlockPreparedCallTransaction {
    pub segment: Segment,
    /// True only when an external unsealed fee-candidate transaction was merged;
    /// does NOT claim that fees are sufficient or paid on chain.
    pub external_fee_candidate_merged: bool,
    pub local_verification: SherlockLocalCallVerification,
    pub unsealed_transaction: UnsealedTransaction,
    pub transaction: SealedTransaction,
    pub transaction_bytes: Vec<u8>,
}

/// Result of extracting `(ProvenIntent, ContractOperation)` from native ledger
/// artifacts and re-verifying the call via [`sherlock_literal::verify_call`].
///
/// `contract_address` carries `Some(call.address)` extracted from the verified
/// single [`ContractAction::Call`] in `proven_intent` (bound to `expected_raw`,
/// and matched against the caller's optional address filter when `Some`); it is
/// local intent/transaction metadata, not authenticated chain-state readback.
#[derive(Clone, Debug)]
pub struct SherlockExtractedCallVerification {
    pub contract_address: Option<ContractAddress>,
    pub segment: Segment,
    pub operation: ContractOperation,
    pub proven_intent: ProvenIntent,
    pub verification: SherlockLocalCallVerification,
}

fn validate_segment(segment: Segment) -> Result<()> {
    if segment == GUARANTEED_SEGMENT {
        return Err(refusal("guaranteed-segment-forbidden"));
    }
    Ok(())
}

fn validate_network_id(network_id: &str) -> Result<()> {
    if network_id.trim().is_empty() {
        return Err(refusal("empty-network-id"));
    }
    Ok(())
}

fn validate_entry_point(entry_point: &str) -> Result<EntryPointBuf> {
    if entry_point.is_empty() {
        return Err(refusal("empty-entry-point"));
    }
    Ok(EntryPointBuf(entry_point.as_bytes().to_vec()))
}

fn validate_maintenance_authority(authority: &ContractMaintenanceAuthority) -> Result<()> {
    if authority.counter != 0 {
        return Err(refusal("non-zero-authority-counter"));
    }
    let committee_len = authority.committee.len();
    let threshold = authority.threshold as usize;
    if threshold > committee_len || (committee_len > 0 && threshold == 0) {
        return Err(refusal("invalid-authority-threshold"));
    }
    Ok(())
}

fn verify_operation_against_keys(
    keys: &SherlockLedgerKeys,
    operation: &ContractOperation,
) -> Result<()> {
    let expected = keys.operation()?;
    if &expected != operation {
        return Err(refusal("operation-key-mismatch"));
    }
    Ok(())
}

/// Construct a real [`ContractDeploy`] and deterministic [`ContractAddress`] from
/// [`SherlockLedgerKeys`], [`ContractOperation`], and an explicit [`SherlockDeploySpec`].
///
/// Qualification: `ContractDeploy::well_formed` (`ledger/src/verify.rs:1755`) is
/// `pub(crate)` in `midnight-ledger` and takes `&impl StateReference<D>`, so standalone
/// ledger well-formedness is only reachable via [`Transaction::well_formed`]
/// ([`check_ledger_well_formed`]) against an explicit [`LedgerState`].
pub fn build_contract_deploy(
    keys: &SherlockLedgerKeys,
    operation: &ContractOperation,
    spec: &SherlockDeploySpec,
) -> Result<SherlockPreparedDeploy> {
    verify_operation_against_keys(keys, operation)?;
    let ep = validate_entry_point(&spec.entry_point)?;
    validate_maintenance_authority(&spec.maintenance_authority)?;

    let operations = HashMap::new().insert(ep, operation.clone());
    let initial_state = ContractState::new(
        spec.initial_data.clone(),
        operations,
        spec.maintenance_authority.clone(),
    );
    if initial_state.data != ChargedState::new(spec.initial_data.clone()) {
        return Err(refusal("unnormalized-initial-state"));
    }

    let deploy = ContractDeploy {
        initial_state: initial_state.clone(),
        nonce: spec.deploy_nonce,
    };

    let contract_address = deploy.address();
    let deploy_bytes = codec::encode(&deploy)?;
    let initial_state_bytes = codec::encode(&initial_state)?;
    let operation_bytes = codec::encode(operation)?;

    Ok(SherlockPreparedDeploy {
        contract_address,
        deploy,
        deploy_bytes,
        initial_state_bytes,
        operation_bytes,
    })
}

/// Assemble and seal a deployment [`SealedTransaction`] from [`SherlockLedgerKeys`],
/// [`ContractOperation`], [`SherlockDeploySpec`], and [`SherlockTransactionSpec`].
pub fn build_deploy_transaction(
    keys: &SherlockLedgerKeys,
    operation: &ContractOperation,
    deploy_spec: &SherlockDeploySpec,
    tx_spec: &SherlockTransactionSpec,
    fee_policy: SherlockFeePolicy<'_>,
) -> Result<SherlockPreparedDeployTransaction> {
    validate_network_id(&tx_spec.network_id)?;
    validate_segment(tx_spec.segment)?;

    let prepared_deploy = build_contract_deploy(keys, operation, deploy_spec)?;
    let action: ContractAction<ProofMarker, InMemoryDB> =
        ContractAction::Deploy(Sp::new(prepared_deploy.deploy.clone()));
    let actions = StorageArray::new().push(action);

    let deploy_intent: UnsealedSignedIntent = Intent {
        guaranteed_unshielded_offer: None,
        fallible_unshielded_offer: None,
        actions,
        dust_actions: None,
        ttl: tx_spec.ttl,
        binding_commitment: tx_spec.binding_commitment,
    };

    let (unsealed_transaction, external_fee_candidate_merged) = assemble_unsealed_transaction(
        &tx_spec.network_id,
        tx_spec.segment,
        deploy_intent,
        fee_policy,
    )?;
    let rng = StdRng::from_seed(tx_spec.seal_seed);
    let transaction = seal_and_verify_transaction(&unsealed_transaction, rng)?;
    let transaction_bytes = codec::encode(&transaction)?;

    Ok(SherlockPreparedDeployTransaction {
        prepared_deploy,
        segment: tx_spec.segment,
        external_fee_candidate_merged,
        unsealed_transaction,
        transaction,
        transaction_bytes,
    })
}

/// Lift a signature-free [`ProvenIntent`] (`S = ()`) into an [`UnsealedSignedIntent`]
/// (`S = Signature`) without fabricating any signature.
///
/// Refuses any intent that carries unshielded offers or dust actions, since those
/// sub-structures embed `S`-typed signature fields.
pub fn lift_proven_call_intent(proven: &ProvenIntent) -> Result<UnsealedSignedIntent> {
    if proven.guaranteed_unshielded_offer.is_some()
        || proven.fallible_unshielded_offer.is_some()
        || proven.dust_actions.is_some()
    {
        return Err(refusal("unexpected-signed-intent-offers"));
    }
    Ok(Intent {
        guaranteed_unshielded_offer: None,
        fallible_unshielded_offer: None,
        actions: proven.actions.clone(),
        dust_actions: None,
        ttl: proven.ttl,
        binding_commitment: proven.binding_commitment,
    })
}

/// Assemble and seal a [`SealedTransaction`] from a verified Sherlock [`ProvenIntent`].
///
/// Runs [`sherlock_literal::verify_call`] against the caller-supplied pre-call
/// [`ChargedState`] and runtime [`CostModel`] prior to transaction assembly so
/// that any mismatched operation, raw intent, transcript gas, binding commitment,
/// communication commitment, or ZK proof is refused fail-closed before sealing.
pub fn build_call_transaction(
    keys: &SherlockLedgerKeys,
    operation: &ContractOperation,
    pre_call_state: &ChargedState<InMemoryDB>,
    cost_model: &CostModel,
    expected_raw: &RawIntent,
    proven: &ProvenIntent,
    opening: &SherlockCallOpening,
    tx_spec: &SherlockTransactionSpec,
    fee_policy: SherlockFeePolicy<'_>,
) -> Result<SherlockPreparedCallTransaction> {
    validate_network_id(&tx_spec.network_id)?;
    validate_segment(tx_spec.segment)?;

    if tx_spec.binding_commitment != expected_raw.binding_commitment
        || proven.binding_commitment != expected_raw.binding_commitment
    {
        return Err(refusal("binding-commitment-mismatch"));
    }
    if tx_spec.ttl != expected_raw.ttl || proven.ttl != expected_raw.ttl {
        return Err(refusal("intent-ttl-mismatch"));
    }

    let local_verification = sherlock_literal::verify_call(
        keys,
        operation,
        pre_call_state,
        cost_model,
        expected_raw,
        proven,
        opening,
        tx_spec.segment,
    )?;
    let signed_intent = lift_proven_call_intent(proven)?;

    let (unsealed_transaction, external_fee_candidate_merged) = assemble_unsealed_transaction(
        &tx_spec.network_id,
        tx_spec.segment,
        signed_intent,
        fee_policy,
    )?;
    let rng = StdRng::from_seed(tx_spec.seal_seed);
    let transaction = seal_and_verify_transaction(&unsealed_transaction, rng)?;
    let transaction_bytes = codec::encode(&transaction)?;

    Ok(SherlockPreparedCallTransaction {
        segment: tx_spec.segment,
        external_fee_candidate_merged,
        local_verification,
        unsealed_transaction,
        transaction,
        transaction_bytes,
    })
}

fn assemble_unsealed_transaction(
    network_id: &str,
    segment: Segment,
    intent: UnsealedSignedIntent,
    fee_policy: SherlockFeePolicy<'_>,
) -> Result<(UnsealedTransaction, bool)> {
    let intents = HashMap::new().insert(segment, intent.clone());
    let base_tx = Transaction::Standard(StandardTransaction {
        network_id: network_id.to_owned(),
        intents,
        guaranteed_coins: None,
        fallible_coins: HashMap::new(),
        binding_randomness: intent.binding_commitment,
    });

    match fee_policy {
        SherlockFeePolicy::ExplicitUnfundedCandidate => Ok((base_tx, false)),
        SherlockFeePolicy::MergeUnsealedFeeTransaction(fee_tx) => {
            let Transaction::Standard(fee_stx) = fee_tx else {
                return Err(refusal("unsupported-claim-rewards-transaction"));
            };
            if fee_stx.intents.get(&GUARANTEED_SEGMENT).is_some() {
                return Err(refusal("guaranteed-segment-forbidden"));
            }
            if fee_stx.intents.get(&segment).is_some() {
                return Err(refusal("fee-segment-collision"));
            }
            let merged = base_tx
                .merge(fee_tx)
                .map_err(|_| refusal("fee-transaction-merge-failed"))?;
            Ok((merged, true))
        }
    }
}

fn seal_and_verify_transaction<R: Rng + CryptoRng + SplittableRng>(
    unsealed: &UnsealedTransaction,
    rng: R,
) -> Result<SealedTransaction> {
    let sealed = unsealed.seal(rng);
    verify_sealed_transaction_bindings(&sealed)?;
    Ok(sealed)
}

/// Verify Schnorr binding proofs on every segment of a [`SealedTransaction`] and,
/// when no Zswap coin offers are present, verify transaction-level Pedersen binding
/// balance across all intents.
pub fn verify_sealed_transaction_bindings(tx: &SealedTransaction) -> Result<()> {
    let Transaction::Standard(stx) = tx else {
        return Err(refusal("unsupported-claim-rewards-transaction"));
    };
    validate_network_id(&stx.network_id)?;
    if stx.intents.is_empty() {
        return Err(refusal("empty-transaction-intents"));
    }

    let mut segments: Vec<(Segment, SealedSignedIntent)> = stx
        .intents
        .iter()
        .map(|entry| {
            let (seg, intent) = &*entry;
            (**seg, (**intent).clone())
        })
        .collect();
    segments.sort_by_key(|(seg, _)| *seg);

    let mut total_intent_commitment = Pedersen::default();
    for (seg, intent) in &segments {
        validate_segment(*seg)?;
        let challenge_pre = intent.challenge_pre_for(*seg);
        if !intent.binding_commitment.valid(&challenge_pre) {
            return Err(refusal("invalid-sealed-binding-proof"));
        }
        total_intent_commitment =
            total_intent_commitment + Pedersen::from(intent.binding_commitment.clone());
    }

    if stx.guaranteed_coins.is_none() && stx.fallible_coins.is_empty() {
        let expected_total = Pedersen::from(stx.binding_randomness);
        if total_intent_commitment != expected_total {
            return Err(refusal("transaction-binding-randomness-mismatch"));
        }
    }
    Ok(())
}

/// Run `midnight-ledger`'s native [`Transaction::well_formed`] check against a
/// reference [`LedgerState`] and map ledger rejections into fail-closed [`Error`] codes.
///
/// Refuses any [`WellFormedStrictness`] that disables balancing, native/contract proof
/// verification, signature verification, or limits, or that uses a non-`Real`
/// [`ProofVerificationMode`].
///
/// Note: For legacy transcript-free calls (`guaranteed_transcript == None` and
/// `fallible_transcript == None`), `midnight-ledger` (`ledger/src/verify.rs:1154`)
/// returns `SequencingCheckError::CallHasEmptyTranscripts`, which this function
/// surfaces as `sherlock-transaction: ledger-rejects-empty-call-transcripts`.
pub fn check_ledger_well_formed(
    tx: &SealedTransaction,
    ref_state: &LedgerState<InMemoryDB>,
    strictness: WellFormedStrictness,
    block_time: Timestamp,
) -> Result<VerifiedTransaction<InMemoryDB>> {
    if !strictness.enforce_balancing
        || !strictness.verify_native_proofs
        || !strictness.verify_contract_proofs
        || !strictness.verify_signatures
        || !strictness.enforce_limits
        || !matches!(
            strictness.proof_verification_mode,
            ProofVerificationMode::Real
        )
    {
        return Err(refusal("weakened-well-formed-strictness-forbidden"));
    }
    let verified = tx
        .well_formed(ref_state, strictness, block_time)
        .map_err(|err| match err {
            MalformedTransaction::SequencingCheckFailure(
                SequencingCheckError::CallHasEmptyTranscripts { .. },
            ) => refusal("ledger-rejects-empty-call-transcripts"),
            MalformedTransaction::BalanceCheckOverspend { .. }
            | MalformedTransaction::BalanceCheckOutOfBounds { .. }
            | MalformedTransaction::BalanceCheckConversionFailure { .. }
            | MalformedTransaction::FeeCalculation(_) => {
                refusal("ledger-fee-or-balance-unsatisfied")
            }
            MalformedTransaction::ContractNotPresent(..) => refusal("ledger-contract-not-present"),
            MalformedTransaction::InvalidSchnorrProof => refusal("invalid-sealed-binding-proof"),
            MalformedTransaction::PedersenCheckFailure { .. } => {
                refusal("transaction-binding-randomness-mismatch")
            }
            MalformedTransaction::IntentAtGuaranteedSegmentId
            | MalformedTransaction::IllegallyDeclaredGuaranteed => {
                refusal("guaranteed-segment-forbidden")
            }
            _ => refusal("ledger-well-formed-rejected"),
        })?;
    if let Transaction::Standard(stx) = tx {
        let runtime_cost_model = &ref_state.parameters.cost_model.runtime_cost_model;
        let mut segments: Vec<(Segment, SealedSignedIntent)> = stx
            .intents
            .iter()
            .map(|entry| {
                let (seg, intent) = &*entry;
                (**seg, (**intent).clone())
            })
            .collect();
        segments.sort_by_key(|(seg, _)| *seg);
        let mut charged_states: std::collections::BTreeMap<
            ContractAddress,
            ChargedState<InMemoryDB>,
        > = std::collections::BTreeMap::new();
        for guaranteed_phase in [true, false] {
            for (_, intent) in &segments {
                for action in intent.actions.iter() {
                    if let ContractAction::Call(call) = &*action {
                        let transcript = if guaranteed_phase {
                            call.guaranteed_transcript.as_ref()
                        } else {
                            call.fallible_transcript.as_ref()
                        };
                        let Some(transcript) = transcript else {
                            continue;
                        };
                        if transcript.gas == RunningCost::ZERO {
                            return Err(refusal("zero-transcript-gas-forbidden"));
                        }
                        let current_state = match charged_states.get(&call.address) {
                            Some(state) => state.clone(),
                            None => {
                                let Some(cstate) = ref_state.index(call.address) else {
                                    return Err(refusal("ledger-contract-not-present"));
                                };
                                cstate.data
                            }
                        };
                        if !transcript.program.is_empty() {
                            return Err(refusal("unsupported-non-empty-call-transcript"));
                        }
                        let baseline_rcmap_cost = runtime_cost_model.gc_rcmap_constant
                            + runtime_cost_model.update_rcmap_constant
                            + runtime_cost_model.get_writes_constant;
                        if baseline_rcmap_cost
                            == midnight_base_crypto::cost_model::CostDuration::ZERO
                            || runtime_cost_model.gc_rcmap_coeff_keys_removed_size
                                == midnight_base_crypto::cost_model::CostDuration::ZERO
                        {
                            return Err(refusal("zero-runtime-state-cost-model"));
                        }
                        let qcontext = QueryContext::new(current_state, call.address);
                        let results = qcontext
                            .run_transcript(transcript, runtime_cost_model)
                            .map_err(|_| refusal("ledger-transcript-out-of-gas-or-rejected"))?;
                        if results.context.effects != transcript.effects
                            || !results.events.is_empty()
                        {
                            return Err(refusal("ledger-transcript-effects-mismatch"));
                        }
                        charged_states.insert(call.address, results.context.state);
                    }
                }
            }
        }
    }
    Ok(verified)
}

/// Extract the installed [`ContractOperation`] for `entry_point` from a [`ContractState`].
///
/// Qualification: extraction does not authenticate the returned [`ContractOperation`]
/// against [`SherlockLedgerKeys`]; key/IR authenticity requires [`sherlock_literal::verify_call`].
pub fn extract_operation_from_contract_state(
    state: &ContractState<InMemoryDB>,
    entry_point: &str,
) -> Result<ContractOperation> {
    let ep = validate_entry_point(entry_point)?;
    let Some(op) = state.operations.get(&ep) else {
        return Err(refusal("missing-contract-entry-point"));
    };
    Ok((*op).clone())
}

/// Extract the deterministic [`ContractAddress`] and installed [`ContractOperation`]
/// for `entry_point` from a [`ContractDeploy`].
///
/// Qualification: `ContractDeploy::well_formed` (`ledger/src/verify.rs:1755`) is
/// `pub(crate)` in `midnight-ledger` and requires a [`StateReference`]; full ledger
/// deploy validation requires [`check_ledger_well_formed`], and operation key/IR
/// authenticity requires [`sherlock_literal::verify_call`].
pub fn extract_operation_from_deploy(
    deploy: &ContractDeploy<InMemoryDB>,
    entry_point: &str,
) -> Result<(ContractAddress, ContractOperation)> {
    let op = extract_operation_from_contract_state(&deploy.initial_state, entry_point)?;
    Ok((deploy.address(), op))
}

/// Extract `(ContractAddress, ContractOperation)` from a single-deploy segment of
/// a [`SealedTransaction`].
///
/// Qualification: verifies transaction Schnorr/Pedersen bindings only; full ledger
/// deploy well-formedness requires [`check_ledger_well_formed`], and operation key/IR
/// authenticity against [`SherlockLedgerKeys`] requires [`sherlock_literal::verify_call`].
pub fn extract_operation_from_deploy_transaction(
    tx: &SealedTransaction,
    segment: Segment,
    entry_point: &str,
) -> Result<(ContractAddress, ContractOperation)> {
    verify_sealed_transaction_bindings(tx)?;
    let Transaction::Standard(stx) = tx else {
        return Err(refusal("unsupported-claim-rewards-transaction"));
    };
    let Some(intent) = stx.intents.get(&segment) else {
        return Err(refusal("missing-deploy-segment"));
    };
    if intent.actions.len() != 1 {
        return Err(refusal("deploy-action-arity-mismatch"));
    }
    let Some(action) = intent.actions.get(0) else {
        return Err(refusal("deploy-action-arity-mismatch"));
    };
    let ContractAction::Deploy(deploy) = &*action else {
        return Err(refusal("expected-deploy-action"));
    };
    extract_operation_from_deploy(deploy, entry_point)
}

/// Decode a canonical tagged deployment [`SealedTransaction`] and extract its
/// [`ContractAddress`] and installed [`ContractOperation`].
///
/// Qualification: decodes and checks transaction bindings only; operation key/IR
/// authenticity against [`SherlockLedgerKeys`] requires [`sherlock_literal::verify_call`].
pub fn extract_operation_from_deploy_transaction_bytes(
    tx_bytes: &[u8],
    segment: Segment,
    entry_point: &str,
) -> Result<(ContractAddress, ContractOperation)> {
    let tx: SealedTransaction = codec::decode_artifact(tx_bytes)?;
    extract_operation_from_deploy_transaction(&tx, segment, entry_point)
}

/// Recover a [`ProvenIntent`] from `segment` of a [`SealedTransaction`] after
/// verifying the transaction's Schnorr seal (`PureGeneratorPedersen::valid`) and
/// verifying that the sealed commitment point equals `Pedersen::from(expected_raw.binding_commitment)`.
pub fn extract_proven_intent_from_transaction(
    tx: &SealedTransaction,
    segment: Segment,
    expected_raw: &RawIntent,
) -> Result<ProvenIntent> {
    validate_segment(segment)?;
    verify_sealed_transaction_bindings(tx)?;

    let Transaction::Standard(stx) = tx else {
        return Err(refusal("unsupported-claim-rewards-transaction"));
    };
    let Some(sealed_intent) = stx.intents.get(&segment) else {
        return Err(refusal("missing-call-segment"));
    };

    let actual_commitment = Pedersen::from(sealed_intent.binding_commitment.clone());
    let expected_commitment = Pedersen::from(expected_raw.binding_commitment);
    if actual_commitment != expected_commitment {
        return Err(refusal("binding-commitment-mismatch"));
    }

    if sealed_intent.guaranteed_unshielded_offer.is_some()
        || sealed_intent.fallible_unshielded_offer.is_some()
        || sealed_intent.dust_actions.is_some()
    {
        return Err(refusal("unexpected-signed-intent-offers"));
    }

    Ok(Intent {
        guaranteed_unshielded_offer: None,
        fallible_unshielded_offer: None,
        actions: sealed_intent.actions.clone(),
        dust_actions: None,
        ttl: sealed_intent.ttl,
        binding_commitment: expected_raw.binding_commitment,
    })
}

/// Decode a canonical tagged [`SealedTransaction`] and recover the target segment's
/// [`ProvenIntent`].
pub fn extract_proven_intent_from_transaction_bytes(
    tx_bytes: &[u8],
    segment: Segment,
    expected_raw: &RawIntent,
) -> Result<ProvenIntent> {
    let tx: SealedTransaction = codec::decode_artifact(tx_bytes)?;
    extract_proven_intent_from_transaction(&tx, segment, expected_raw)
}

/// Extract the [`ProvenIntent`] from `call_tx` at `segment` and verify it against
/// `keys`, `operation`, `pre_call_state`, `cost_model`, `expected_raw`, and `opening`
/// via [`sherlock_literal::verify_call`].
///
/// When `contract_address` is `Some(expected_addr)`, refuses any call whose extracted
/// `call.address` differs from `expected_addr`. Populates
/// [`SherlockExtractedCallVerification::contract_address`] with `Some(call.address)`
/// from the verified single [`ContractAction::Call`] whether or not an optional
/// filter was supplied. Neither `expected_raw` nor the returned address is
/// authenticated chain readback.
pub fn verify_extracted_call(
    keys: &SherlockLedgerKeys,
    operation: &ContractOperation,
    pre_call_state: &ChargedState<InMemoryDB>,
    cost_model: &CostModel,
    expected_raw: &RawIntent,
    call_tx: &SealedTransaction,
    opening: &SherlockCallOpening,
    segment: Segment,
    contract_address: Option<ContractAddress>,
) -> Result<SherlockExtractedCallVerification> {
    let proven_intent = extract_proven_intent_from_transaction(call_tx, segment, expected_raw)?;
    if proven_intent.actions.len() != 1 {
        return Err(refusal("call-action-arity-mismatch"));
    }
    let Some(action) = proven_intent.actions.get(0) else {
        return Err(refusal("call-action-arity-mismatch"));
    };
    let ContractAction::Call(call) = &*action else {
        return Err(refusal("expected-call-action"));
    };
    let extracted_address = call.address;
    if let Some(expected_addr) = contract_address {
        if extracted_address != expected_addr {
            return Err(refusal("call-contract-address-mismatch"));
        }
    }
    let verification = sherlock_literal::verify_call(
        keys,
        operation,
        pre_call_state,
        cost_model,
        expected_raw,
        &proven_intent,
        opening,
        segment,
    )?;

    Ok(SherlockExtractedCallVerification {
        contract_address: Some(extracted_address),
        segment,
        operation: operation.clone(),
        proven_intent,
        verification,
    })
}

/// End-to-end downstream extraction and verification from canonical tagged
/// deployment transaction bytes and canonical tagged call transaction bytes
/// under an explicit pre-call [`ChargedState`] and runtime [`CostModel`].
pub fn verify_extracted_call_from_bytes(
    keys: &SherlockLedgerKeys,
    deploy_tx_bytes: &[u8],
    deploy_segment: Segment,
    entry_point: &str,
    pre_call_state: &ChargedState<InMemoryDB>,
    cost_model: &CostModel,
    expected_raw: &RawIntent,
    call_tx_bytes: &[u8],
    opening: &SherlockCallOpening,
    call_segment: Segment,
) -> Result<SherlockExtractedCallVerification> {
    let (contract_address, operation) = extract_operation_from_deploy_transaction_bytes(
        deploy_tx_bytes,
        deploy_segment,
        entry_point,
    )?;
    let call_tx: SealedTransaction = codec::decode_artifact(call_tx_bytes)?;
    verify_extracted_call(
        keys,
        &operation,
        pre_call_state,
        cost_model,
        expected_raw,
        &call_tx,
        opening,
        call_segment,
        Some(contract_address),
    )
}

fn verify_expected_raw_entry_point_and_envelope(
    keys: &SherlockLedgerKeys,
    contract_address: ContractAddress,
    entry_point: &str,
    pre_call_state: &ChargedState<InMemoryDB>,
    cost_model: &CostModel,
    expected_raw: &RawIntent,
    opening: &SherlockCallOpening,
) -> Result<()> {
    if expected_raw.actions.len() != 1 || expected_raw.calls().count() != 1 {
        return Err(refusal("call-action-arity-mismatch"));
    }
    let Some(raw_call) = expected_raw.calls().next() else {
        return Err(refusal("expected-call-action"));
    };
    let midnight_ledger::structure::ProofPreimageVersioned::V2(preimage) = &raw_call.proof else {
        return Err(refusal("native-preimage-version"));
    };
    if preimage.key_location.0.as_ref() != entry_point {
        return Err(refusal("call-entry-point-mismatch"));
    }
    let canonical_raw = sherlock_literal::raw_call_intent(
        keys,
        contract_address,
        pre_call_state,
        cost_model,
        opening,
        expected_raw.ttl,
        expected_raw.binding_commitment,
    )?;
    if canonical_raw.erase_proofs().erase_signatures()
        != expected_raw.erase_proofs().erase_signatures()
    {
        return Err(refusal("call-entry-point-mismatch"));
    }
    Ok(())
}

/// Extract the installed [`ContractOperation`] for `entry_point` from a caller-supplied
/// [`ContractState<InMemoryDB>`] and verify canonical tagged call transaction bytes
/// against that same state's [`ContractState::data`] (`&contract_state.data`),
/// explicit expected [`ContractAddress`], [`SherlockLedgerKeys`], runtime [`CostModel`],
/// `expected_raw`, and `opening`.
///
/// Does not accept a separate [`ChargedState`] argument and never calls
/// [`ChargedState::new`] or reconstructs/normalizes `contract_state.data`.
///
/// Trust qualification: caller-supplied `contract_state`, `contract_address`,
/// `cost_model`, state/source freshness, and chain association remain external
/// assumptions. This helper neither fetches nor authenticates chain state, nor
/// establishes full transaction balancing, [`LedgerState`] application, network
/// inclusion/finality/settlement, full Sherlock correctness, or native acceptance.
/// State identity is NOT established by transcript gas equality, and existing
/// companion-transcript restrictions remain scoped.
pub fn verify_extracted_call_from_contract_state(
    keys: &SherlockLedgerKeys,
    contract_state: &ContractState<InMemoryDB>,
    contract_address: ContractAddress,
    entry_point: &str,
    cost_model: &CostModel,
    expected_raw: &RawIntent,
    call_tx_bytes: &[u8],
    opening: &SherlockCallOpening,
    call_segment: Segment,
) -> Result<SherlockExtractedCallVerification> {
    let operation = extract_operation_from_contract_state(contract_state, entry_point)?;
    let call_tx: SealedTransaction = codec::decode_artifact(call_tx_bytes)?;
    let extracted = verify_extracted_call(
        keys,
        &operation,
        &contract_state.data,
        cost_model,
        expected_raw,
        &call_tx,
        opening,
        call_segment,
        Some(contract_address),
    )?;
    verify_operation_against_keys(keys, &extracted.operation)?;
    verify_expected_raw_entry_point_and_envelope(
        keys,
        contract_address,
        entry_point,
        &contract_state.data,
        cost_model,
        expected_raw,
        opening,
    )?;
    Ok(extracted)
}

/// Decode a canonical tagged [`ContractState<InMemoryDB>`] (`contract-state[v8]` at
/// this pinned ledger-v9 source revision, with embedded `charged-state[v1]`)
/// via [`codec::decode_artifact`] and verify canonical tagged call transaction
/// bytes against that decoded state via [`verify_extracted_call_from_contract_state`].
///
/// Serialization tags identify per-type binary encodings, not ledger major versions,
/// and Rust tagged decoding does not establish TypeScript, indexer, or provider
/// envelope interoperability. All external trust boundary qualifications of
/// [`verify_extracted_call_from_contract_state`] apply.
pub fn verify_extracted_call_from_contract_state_bytes(
    keys: &SherlockLedgerKeys,
    contract_state_bytes: &[u8],
    contract_address: ContractAddress,
    entry_point: &str,
    cost_model: &CostModel,
    expected_raw: &RawIntent,
    call_tx_bytes: &[u8],
    opening: &SherlockCallOpening,
    call_segment: Segment,
) -> Result<SherlockExtractedCallVerification> {
    let contract_state: ContractState<InMemoryDB> = codec::decode_artifact(contract_state_bytes)?;
    verify_extracted_call_from_contract_state(
        keys,
        &contract_state,
        contract_address,
        entry_point,
        cost_model,
        expected_raw,
        call_tx_bytes,
        opening,
        call_segment,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refuses_guaranteed_segment_zero() {
        let err = validate_segment(GUARANTEED_SEGMENT).unwrap_err();
        assert_eq!(err.code, "guaranteed-segment-forbidden");
    }

    #[test]
    fn refuses_empty_network_id_and_entry_point() {
        assert_eq!(
            validate_network_id("   ").unwrap_err().code,
            "empty-network-id"
        );
        assert_eq!(
            validate_entry_point("").unwrap_err().code,
            "empty-entry-point"
        );
    }

    #[test]
    fn refuses_invalid_maintenance_authority() {
        let non_zero_counter = ContractMaintenanceAuthority {
            committee: Vec::new(),
            threshold: 0,
            counter: 1,
        };
        assert_eq!(
            validate_maintenance_authority(&non_zero_counter)
                .unwrap_err()
                .code,
            "non-zero-authority-counter"
        );

        let impossible_threshold = ContractMaintenanceAuthority {
            committee: Vec::new(),
            threshold: 1,
            counter: 0,
        };
        assert_eq!(
            validate_maintenance_authority(&impossible_threshold)
                .unwrap_err()
                .code,
            "invalid-authority-threshold"
        );
    }
}
