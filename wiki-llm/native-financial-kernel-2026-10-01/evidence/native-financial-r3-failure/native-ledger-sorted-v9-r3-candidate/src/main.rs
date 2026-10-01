mod keygen;
mod artifacts;
mod provider;
mod verify;
use artifacts::*;
use midnight_zkir::IrSource;
use rand::{rngs::{OsRng, StdRng}, SeedableRng};
use serde::Deserialize;
use serialize::{tagged_deserialize, tagged_serialize, Deserializable, Serializable, Tagged};
use sha2::{Digest, Sha256};
use std::{error::Error, fs::{self, OpenOptions}, io::{self, Cursor, Read, Write}, ops::Deref, path::{Path, PathBuf}, sync::{Arc, Mutex}};
use transient_crypto::{curve::Fr, commitment::{PedersenRandomness, PureGeneratorPedersen}, proofs::{KeyLocation, ParamsProver, ParamsProverProvider, ParamsVerifier, Proof, ProofPreimage, ProvingKeyMaterial, Resolver, VerifierKey, Zkir}};
use base_crypto::{fab::AlignedValue, hash::HashOutput, time::Timestamp};
use coin_structure::{contract::ContractAddress, coin::{NIGHT, TokenType, UnshieldedTokenType, UserAddress, PublicAddress}};
use storage::{arena::Sp, db::InMemoryDB, storage::{HashMap, Map}};
use onchain_runtime::{context::{BlockContext, QueryContext, ClaimedUnshieldedSpendsKey}, ops::Op, result_mode::ResultModeVerify, state::{ContractState, ContractOperation, EntryPointBuf}};
use ledger::{construct::{PrePartitionContractCall, PreTranscript, SegmentSpecifier}, dust::{DustActions, DustRegistration, DustPublicKey}, structure::*, semantics::{TransactionContext, TransactionResult}, verify::WellFormedStrictness, events::EventDetails};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
type DB = InMemoryDB;
type SignedTransaction = Transaction<Signature, ProofMarker, PureGeneratorPedersen, DB>;
const SRS_SHA: &str = "4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74";
const LOCATION: &str = "local-public-fixture/pay/c20c6e733500823c478eea885bebab5d62ba9958cfdd1ec8cf2ae89b32f9d7ae";
const MAX_FILE: u64 = 2 * 1024 * 1024 * 1024;
const SEGMENT: u16 = 1;
#[derive(Clone, Deserialize)] #[serde(deny_unknown_fields)]
pub struct Artifact { path: PathBuf, sha256: String }
#[derive(Deserialize)] #[serde(deny_unknown_fields)]
struct ProveConfig {
    ir: Artifact, pk: Artifact, vk: Artifact, srs: Artifact,
    initial_contract: Artifact, expected_contract: Artifact, runtime: Artifact,
    retained_preimage: Artifact, fixture: Fixture,
}
#[derive(Clone, Deserialize)] #[serde(deny_unknown_fields)]
struct Fixture {
    trust: String, network: String, block_seconds: u64, night_creation_seconds: u64,
    night_value: u128, fee_allowance: u128, ttl_seconds: u64,
}
#[derive(Deserialize)] #[serde(deny_unknown_fields, rename_all="camelCase")]
struct RuntimeData {
    input: AlignedValue, output: AlignedValue, public_transcript: Vec<Op<ResultModeVerify, DB>>,
    private_transcript_outputs: Vec<AlignedValue>,
}
fn verifier_parameters(bytes: &[u8]) -> Result<ParamsVerifier> {
    let mut c=Cursor::new(bytes); let p=ParamsVerifier::read(&mut c)?;
    if c.position()!=bytes.len() as u64 {return Err("verifier parameter suffix".into());} Ok(p)
}
fn address()->ContractAddress {ContractAddress(HashOutput([4;32]))}
fn asset()->TokenType {TokenType::Unshielded(UnshieldedTokenType(HashOutput([0xa1;32])))}
fn recipient()->UserAddress {UserAddress(HashOutput([2;32]))}
fn fee_recipient()->UserAddress {UserAddress(HashOutput([3;32]))}
fn block(f:&Fixture)->BlockContext {BlockContext{tblock:Timestamp::from_secs(f.block_seconds),tblock_err:0,parent_block_hash:HashOutput([0;32]),last_block_time:Timestamp::from_secs(f.block_seconds)}}
fn fixture_key()->Result<SigningKey> {
    // Public deterministic development key, never a wallet import.
    Ok(SigningKey::Schnorr(base_crypto::schnorr::SigningKey::from_bytes(&[0x42;32])?))
}
fn funding(f:&Fixture,key:&SigningKey)->UtxoSpend {UtxoSpend{value:f.night_value,owner:key.verifying_key(),type_:NIGHT,intent_hash:IntentHash(HashOutput([0x51;32])),output_no:0}}
fn availability(f:&Fixture, state:&LedgerState<DB>, spend:&UtxoSpend)->Result<u128> {
    let u=Utxo::from(spend.clone());
    let m=state.utxo.utxos.get(&u).ok_or("NIGHT UTXO absent")?;
    if spend.type_!=NIGHT || state.dust.generation.night_indices.contains_key(&spend.initial_nonce()) {return Err("NIGHT not generationless".into());}
    let age=f.block_seconds.saturating_sub(m.ctime.to_secs()) as u128;
    let p=state.parameters.dust;
    Ok(age.saturating_mul(spend.value).saturating_mul(p.generation_decay_rate as u128).min(spend.value.saturating_mul(p.night_dust_ratio as u128)))
}
fn genesis(f:&Fixture,mut contract:ContractState<DB>,vk:Option<VerifierKey>)->Result<LedgerState<DB>> {
    if f.trust!="TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY" || f.night_value==0 || f.night_value>MAX_SUPPLY || f.fee_allowance==0 || f.night_creation_seconds>f.block_seconds {return Err("fixture trust/funding/time invalid".into());}
    if contract.balance.size()!=1 || contract.balance.get(&asset()).map(|x|*x)!=Some(10000) {return Err("exact A1 escrow fixture absent".into());}
    contract.operations=HashMap::new().insert(b"pay"[..].into(),ContractOperation::new(vk,None));
    let mut state=LedgerState::new(f.network.clone());
    let ttl_delta=f.ttl_seconds.checked_sub(f.block_seconds).filter(|delta|*delta>0).ok_or("TTL must be strictly after block time; no seconds overflow")?;
    if i128::from(ttl_delta)>state.parameters.global_ttl.as_seconds() {return Err("TTL exceeds native default global TTL".into());}
    state.contract=state.contract.insert(address(),contract);
    let key=fixture_key()?;let spend=funding(f,&key);
    let mut us=(*state.utxo).clone(); us.utxos=us.utxos.insert(Utxo::from(spend.clone()),UtxoMeta{ctime:Timestamp::from_secs(f.night_creation_seconds)});state.utxo=Sp::new(us);
    if f.fee_allowance>availability(f,&state,&spend)? {return Err("fee allowance exceeds actual genesis NIGHT availability".into());}
    Ok(state)
}
fn offer(mut inputs:Vec<UtxoSpend>,mut outputs:Vec<UtxoOutput>)->UnshieldedOffer<Signature,DB> {
    // Envelope canonicalization before binding/proving/signing; VM effects keep their order.
    // This constructor receives no signatures: never independently reorder signed inputs.
    inputs.sort(); outputs.sort();
    UnshieldedOffer{inputs:inputs.into(),outputs:outputs.into(),signatures:vec![].into()}
}
fn envelope_preflight(tx:&Transaction<Signature,ProofPreimageMarker,PedersenRandomness,DB>,out:&Path)->Result<()> {
    let (_,parent)=single_call(tx)?;
    let erased=parent.erase_proofs().erase_signatures();
    let mut counts=vec![];
    for (label,offer) in [("guaranteed",parent.guaranteed_unshielded_offer.as_ref()),("fallible",parent.fallible_unshielded_offer.as_ref())] {
        if let Some(o)=offer {
            // Official eager envelope checks: ordering, nonzero outputs, duplicate inputs.
            // Returned signature-verification closure is deliberately NOT called pre-signing.
            let _deferred=o.deref().clone().well_formed(SEGMENT,&erased)?;
            let ins=Vec::from(&o.inputs);let outs=Vec::from(&o.outputs);
            if outs.len()>1 {
                let mut wrong=outs.clone();wrong.reverse();
                if !wrong.is_sorted() {
                    let negative=UnshieldedOffer::<Signature,DB>{inputs:ins.clone().into(),outputs:wrong.into(),signatures:vec![].into()};
                    if !matches!(negative.well_formed(SEGMENT,&erased),Err(ledger::error::MalformedTransaction::OutputsNotSorted(_))) {return Err("unsorted envelope did not produce exact native OutputsNotSorted".into());}
                }
            }
            if !outs.is_empty() {
                let mut zero=outs.clone();zero[0].value=0;zero.sort();
                let negative=UnshieldedOffer::<Signature,DB>{inputs:ins.clone().into(),outputs:zero.into(),signatures:vec![].into()};
                if !matches!(negative.well_formed(SEGMENT,&erased),Err(ledger::error::MalformedTransaction::ZeroValueUtxo(_))) {return Err("zero output did not produce exact native ZeroValueUtxo".into());}
            }
            if !ins.is_empty() {
                let mut duplicate=ins.clone();duplicate.push(ins[0].clone());duplicate.sort();
                let negative=UnshieldedOffer::<Signature,DB>{inputs:duplicate.into(),outputs:outs.clone().into(),signatures:vec![].into()};
                if !matches!(negative.well_formed(SEGMENT,&erased),Err(ledger::error::MalformedTransaction::DuplicateInputs(_))) {return Err("duplicate input did not produce exact native DuplicateInputs".into());}
            }
            let _good_again=o.deref().clone().well_formed(SEGMENT,&erased)?;
            counts.push(serde_json::json!({"partition":label,"inputs":ins.len(),"outputs":outs.len(),"scope":"native eager envelope only; deferred signatures/proofs/default transaction WF unperformed"}));
        }
    }
    exclusive(&out.join("envelope-preflight.json"),&serde_json::to_vec_pretty(&serde_json::json!({"scope":"cheap actual native offer-envelope preflight only; not transaction acceptance","partitions":counts,"proof_invocations":0,"well_formed_checked":false,"ledger_applied":false,"ledger_accepted":false}))?)?;
    Ok(())
}
fn financial_outputs(e:&onchain_runtime::context::Effects<DB>)->Result<Vec<UtxoOutput>> {
    // This checks actual VM-derived effects; it never creates or alters those effects.
    if !e.claimed_shielded_receives.is_empty() || !e.claimed_shielded_spends.is_empty() || !e.claimed_nullifiers.is_empty() || !e.shielded_mints.is_empty() || !e.unshielded_mints.is_empty() || !e.unshielded_inputs.is_empty() || !e.claimed_contract_calls.is_empty() {return Err("unexpected native financial effect family".into());}
    if e.unshielded_outputs.is_empty() && e.claimed_unshielded_spends.is_empty() {return Ok(vec![]);}
    if e.unshielded_outputs.size()!=1 || e.unshielded_outputs.get(&asset()).map(|x|*x)!=Some(1010) || e.claimed_unshielded_spends.size()!=2 {return Err("financial effects split/amount mismatch; no success_only substitution".into());}
    for (dest,amount) in [(recipient(),1000u128),(fee_recipient(),10u128)] {
        let k=ClaimedUnshieldedSpendsKey(asset(),PublicAddress::User(dest));
        if e.claimed_unshielded_spends.get(&k).map(|x|*x)!=Some(amount) {return Err("financial destination mismatch".into());}
    }
    Ok(vec![UtxoOutput{value:1000,owner:recipient(),type_:UnshieldedTokenType(HashOutput([0xa1;32]))},UtxoOutput{value:10,owner:fee_recipient(),type_:UnshieldedTokenType(HashOutput([0xa1;32]))}])
}
fn single_call<B: storage::Storable<DB> + Clone,P:ProofKind<DB>>(tx:&Transaction<Signature,P,B,DB>)->Result<(ContractCall<P,DB>,ledger::structure::Intent<Signature,P,B,DB>)> {
    let Transaction::Standard(s)=tx else {return Err("not standard transaction".into())};
    if s.intents.size()!=1 || s.guaranteed_coins.is_some() || !s.fallible_coins.is_empty() {return Err("unexpected intents/shielded offers".into());}
    let i=s.intents.get(&SEGMENT).ok_or("missing segment")?.deref().clone();
    if i.actions.len()!=1 {return Err("not exactly one action".into());}
    let ContractAction::Call(c)=i.actions.get(0).ok_or("call absent")? else {return Err("non-call action".into())};
    Ok((c.deref().clone(),i))
}
#[derive(Deserialize)] #[serde(deny_unknown_fields)]
struct PrepareConfig { ir:Artifact, initial_contract:Artifact, expected_contract:Artifact, runtime:Artifact, retained_preimage:Artifact, fixture:Fixture }
struct Prepared { state:LedgerState<DB>,expected:ContractState<DB>,unproven:Transaction<Signature,ProofPreimageMarker,PedersenRandomness,DB>,ir:IrSource,rng:StdRng,key:SigningKey }
fn require_prepare_operation(contract:&ContractState<DB>,label:&str)->Result<()> {
    if contract.operations.size()!=1 {return Err(format!("{label}: prepare requires exactly one pay operation").into());}
    let op=contract.operations.get(&EntryPointBuf(b"pay".to_vec())).ok_or_else(||format!("{label}: pay operation missing"))?;
    if op.v2.is_some() || op.v3.is_some() || op.ir.is_some() {return Err(format!("{label}: prepare operation must have v2/v3/ir all None").into());}
    Ok(())
}
fn public_stage(name:&str)->Result<()> {
    // Public phase name only: no values, witness or accepted-status flag.
    let mut stderr=io::stderr().lock();
    writeln!(stderr,"NATIVE_PUBLIC_STAGE {name}")?;stderr.flush()?;Ok(())
}
fn construct(c:&PrepareConfig,vk:Option<VerifierKey>,out:&Path)->Result<Prepared> {
    public_stage("ir_parse_begin")?;
    let irbytes=artifact(&c.ir)?;let ir=IrSource::load(irbytes.as_slice())?;
    public_stage("ir_parse_end")?;
    public_stage("k_model_begin")?;
    if ir.k()!=17 {return Err("unapproved IR k".into());}
    public_stage("k_model_end")?;
    let initial:ContractState<DB>=decode(&artifact(&c.initial_contract)?)?;
    let vk_present=vk.is_some();
    if !vk_present {require_prepare_operation(&initial,"incoming initial contract")?;}
    let state=genesis(&c.fixture,initial,vk)?;
    let expected:ContractState<DB>=decode(&artifact(&c.expected_contract)?)?;
    if !vk_present {require_prepare_operation(&expected,"incoming expected storage reference")?;require_prepare_operation(&state.index(address()).ok_or("constructed genesis contract absent")?,"constructed genesis contract")?;}
    let data:RuntimeData=serde_json::from_slice(&artifact(&c.runtime)?)?;
    if data.public_transcript.len()!=293 || data.public_transcript.iter().filter(|o|matches!(o,Op::Popeq{..})).count()!=47 {return Err("retained293Ops/47reads mismatch".into());}
    let key=fixture_key()?;let spend=funding(&c.fixture,&key);let payer=UserAddress::from(key.verifying_key());
    let contract=state.index(address()).ok_or("genesis contract absent")?;
    let mut qc=QueryContext::new(contract.data.clone(),address());
    qc.call_context.own_address=address();qc.call_context.tblock=block(&c.fixture).tblock;qc.call_context.tblock_err=0;qc.call_context.parent_block_hash=HashOutput([0;32]);qc.call_context.last_block_time=block(&c.fixture).last_block_time;qc.call_context.balance=contract.balance.clone();qc.call_context.caller=Some(PublicAddress::User(payer));
    let replay_call_context=qc.call_context.clone();
    public_stage("replay_begin")?;
    let replay=qc.query(&data.public_transcript,None,&state.parameters.cost_model.runtime_cost_model)?;
    public_stage("replay_end")?;
    if replay.context.state.get_ref()!=expected.data.get_ref() {return Err("actual native replay disagrees with exported generated kernel poststate".into());}
    let _=financial_outputs(&replay.context.effects)?;
    let call=PrePartitionContractCall{address:address(),entry_point:EntryPointBuf(b"pay".to_vec()),op:contract.operations.get(&b"pay"[..].into()).ok_or("registered pay absent")?.deref().clone(),pre_transcript:PreTranscript{context:qc,program:data.public_transcript.clone(),comm_comm:None},private_transcript_outputs:data.private_transcript_outputs,input:data.input,output:data.output,communication_commitment_rand:Fr::from(0u8),key_location:KeyLocation(LOCATION.into())};
    let mut rng=StdRng::seed_from_u64(0x4d4f524941525459);
    let empty:StandardTransaction<Signature,ProofPreimageMarker,PedersenRandomness,DB>=StandardTransaction::new(c.fixture.network.clone(),HashMap::new(),None,HashMap::new());
    public_stage("add_calls_begin")?;
    let mut stx=empty.add_calls::<ProofPreimage>(&mut rng,SegmentSpecifier::Specific(SEGMENT),&[call],&state.parameters,Timestamp::from_secs(c.fixture.ttl_seconds),&[],&[],&[])?;
    public_stage("add_calls_end")?;
    let mut intent=stx.intents.get(&SEGMENT).ok_or("partition intent absent")?.deref().clone();
    let provisional=Transaction::Standard(stx.clone());let (call,_)=single_call(&provisional)?;
    let mut guaranteed=vec![UtxoOutput{value:c.fixture.night_value,owner:payer,type_:NIGHT}];
    if let Some(t)=call.guaranteed_transcript.as_ref(){guaranteed.extend(financial_outputs(&t.effects)?);}
    let fallible=if let Some(t)=call.fallible_transcript.as_ref(){financial_outputs(&t.effects)?}else{vec![]};
    if guaranteed.iter().filter(|o|o.type_!=NIGHT).count()+fallible.len()!=2 {return Err("incomplete financial partition".into());}
    intent.guaranteed_unshielded_offer=Some(Sp::new(offer(vec![spend],guaranteed)));
    intent.fallible_unshielded_offer=if fallible.is_empty(){None}else{Some(Sp::new(offer(vec![],fallible)))};
    intent.dust_actions=Some(Sp::new(DustActions{spends:vec![].into(),registrations:vec![DustRegistration{night_key:key.verifying_key(),dust_address:Some(Sp::new(DustPublicKey(Fr::from(0x44u8)))),allow_fee_payment:c.fixture.fee_allowance,signature:None}].into(),ctime:Timestamp::from_secs(c.fixture.block_seconds)}));
    stx=stx.set_intent(SEGMENT,intent);stx.recompute_binding_randomness();
    let unproven=Transaction::Standard(stx);
    public_stage("envelope_begin")?;
    envelope_preflight(&unproven,out)?;
    public_stage("envelope_end")?;
    // Compare actual constructed preimage to the frozen native fixture before proof allocation.
    let (prepared,prepared_parent)=single_call(&unproven)?;
    let actual_context=prepared.clone().context(&block(&c.fixture),&prepared_parent.erase_proofs().erase_signatures(),contract.clone(),&Map::new());
    let ProofPreimageVersioned::V2(p)=&prepared.proof else {return Err("unsupported native preimage version; construction refused".into());};
    let frozen:ProofPreimage=decode(&artifact(&c.retained_preimage)?)?;
    exclusive(&out.join("genesis.tagged"),&encoded(&state)?)?;
    exclusive(&out.join("prepared-unproven.tagged"),&encoded(&unproven)?)?;
    exclusive(&out.join("partition-costs-effects.txt"),format!("guaranteed={:#?}\nfallible={:#?}",prepared.guaranteed_transcript,prepared.fallible_transcript).as_bytes())?;
    exclusive(&out.join("constructed.preimage"),&encoded(p.as_ref())?)?;
    exclusive(&out.join("frozen.preimage"),&encoded(&frozen)?)?;
    let context_fields=serde_json::json!({"own_address":actual_context.own_address==replay_call_context.own_address,"caller":actual_context.caller==replay_call_context.caller,"balance":actual_context.balance==replay_call_context.balance,"tblock":actual_context.tblock==replay_call_context.tblock,"tblock_err":actual_context.tblock_err==replay_call_context.tblock_err,"parent_block_hash":actual_context.parent_block_hash==replay_call_context.parent_block_hash,"com_indices":actual_context.com_indices==replay_call_context.com_indices,"last_block_time":actual_context.last_block_time==replay_call_context.last_block_time});
    let context_ok=actual_context.own_address==replay_call_context.own_address && actual_context.caller==replay_call_context.caller && actual_context.balance==replay_call_context.balance && actual_context.tblock==replay_call_context.tblock && actual_context.tblock_err==replay_call_context.tblock_err && actual_context.parent_block_hash==replay_call_context.parent_block_hash && actual_context.com_indices==replay_call_context.com_indices && actual_context.last_block_time==replay_call_context.last_block_time;
    let equality=**p==frozen;
    exclusive(&out.join("construction-diagnostics.json"),&serde_json::to_vec_pretty(&serde_json::json!({"scope":"construction only; no proof/registered-key acceptance","constructed_preimage_sha256":sha(&encoded(p.as_ref())?),"frozen_preimage_sha256":sha(&encoded(&frozen)?),"preimage_equal":equality,"fields_equal":{"inputs":p.inputs==frozen.inputs,"private_transcript":p.private_transcript==frozen.private_transcript,"public_transcript_inputs":p.public_transcript_inputs==frozen.public_transcript_inputs,"public_transcript_outputs":p.public_transcript_outputs==frozen.public_transcript_outputs,"binding_input":p.binding_input==frozen.binding_input,"communications_commitment":p.communications_commitment==frozen.communications_commitment,"key_location":p.key_location==frozen.key_location},"actual_context_matches":context_ok,"call_context_fields_equal":context_fields,"expected_contract_scope":"storage-only generated poststate reference; exported escrow is not applied ledger balance","native_parameter_sha256":sha(&encoded(state.parameters.deref())?),"ttl_seconds":c.fixture.ttl_seconds,"block_seconds":c.fixture.block_seconds,"default_global_ttl_seconds":state.parameters.global_ttl.as_seconds().to_string(),"runtime_ops":293,"runtime_reads":47,"call_context":format!("{actual_context:#?}")}))?)?;
    if !equality {return Err("ledger constructed preimage differs from frozen pay preimage; retained diagnostics; no adaptation or allocation".into());}
    if !context_ok {return Err("actual native CallContext mismatch; diagnostics retained".into());}
    let preliminary_fee=unproven.fees_with_margin(&state.parameters,2)?;
    exclusive(&out.join("preliminary-fees.json"),&serde_json::to_vec_pretty(&serde_json::json!({"scope":"native unproven pre-proof estimate; not actual consumed fees","method":"Transaction::fees_with_margin(default LedgerParameters,2)","margin":2,"unproven":true,"keyless":!vk_present,"limitation":"No actual final proof/signature/noop serialized size or real VK read cost established in keyless preparation; proving repeats construction with real VK and acceptance checks native consumed fees","native_parameter_sha256":sha(&encoded(state.parameters.deref())?),"estimate_with_margin2":preliminary_fee.to_string(),"allowance":c.fixture.fee_allowance.to_string(),"vk_present":vk_present,"estimate_within_allowance":preliminary_fee<=c.fixture.fee_allowance}))?)?;
    if preliminary_fee>c.fixture.fee_allowance {return Err("preliminary native fee estimate exceeds frozen allowance before allocation".into());}
    // Estimate is preparation only; final fees are evidenced by strict validation and native dust event.
    public_stage("ir_check_begin")?;
    let checked=ir.check(p);
    public_stage("ir_check_end")?;
    exclusive(&out.join("native-check-result.txt"),format!("{checked:#?}").as_bytes())?;
    let native_skips=checked?;
    exclusive(&out.join("native-check-skips.json"),&serde_json::to_vec_pretty(&native_skips)?)?;
    let diagnostic=serde_json::json!({"scope":if vk_present {"registered-VK preflight only; no final proof, strict well_formed or ledger application claimed"} else {"keyless preparation only; no registered VK, final proof, strict well_formed or ledger application claimed"},"preliminary_fee_with_margin2":preliminary_fee.to_string(),"fee_allowance":c.fixture.fee_allowance.to_string(),"generationless_available":availability(&c.fixture,&state,&funding(&c.fixture,&key))?.to_string(),"native_check":"successful actual Zkir::check","preimage_equal":true,"registered_vk_present":vk_present,"proof_invocations":0});
    exclusive(&out.join("preparation.json"),&serde_json::to_vec_pretty(&diagnostic)?)?;
    Ok(Prepared{state,expected,unproven,ir,rng,key})
}
fn prepare(c:PrepareConfig,out:&Path)->Result<()> {
    fs::create_dir(out)?;
    let _=construct(&c,None,out)?;
    println!("NATIVE_PREPARATION_SOURCE_CHECK_OK; no VK/PK/SRS/proof/ledger acceptance");Ok(())
}
fn preflight(c:ProveConfig,out:&Path)->Result<()> {
    let vk:VerifierKey=decode(&artifact(&c.vk)?)?;
    fs::create_dir(out)?;
    let p=PrepareConfig{ir:c.ir,initial_contract:c.initial_contract,expected_contract:c.expected_contract,runtime:c.runtime,retained_preimage:c.retained_preimage,fixture:c.fixture};
    let _=construct(&p,Some(vk),out)?;
    println!("REGISTERED_VK_NATIVE_ENVELOPE_PREFLIGHT_ONLY; no proof/signature/defaultWF/apply");Ok(())
}
async fn prove(c:ProveConfig,out:&Path)->Result<()> {
    if c.srs.sha256!=SRS_SHA {return Err("SRS identity".into());}
    let vkbytes=artifact(&c.vk)?;let vk:VerifierKey=decode(&vkbytes)?;
    fs::create_dir(out)?;
    let preparation=PrepareConfig{ir:c.ir.clone(),initial_contract:c.initial_contract.clone(),expected_contract:c.expected_contract.clone(),runtime:c.runtime.clone(),retained_preimage:c.retained_preimage.clone(),fixture:c.fixture.clone()};
    let Prepared{state,expected,unproven,ir,mut rng,key}=construct(&preparation,Some(vk),out)?;
    let budget=Arc::new(Mutex::new(provider::Budget::default()));
    let pv=provider::Provider{resolver:Arc::new(provider::LocalResolver{material:ProvingKeyMaterial{prover_key:artifact(&c.pk)?,verifier_key:vkbytes.clone(),ir_source:encoded(&ir)?}}),params:Arc::new(provider::Parameters{bytes:artifact(&c.srs)?}),budget:budget.clone()};
    let proven=unproven.prove(pv,&state.parameters.cost_model.runtime_cost_model).await?;
    let (finalcall,parent)=single_call(&proven)?;
    let native=budget.lock().map_err(|_|"budget poisoned")?;
    if native.records.len()!=1 {return Err("not exactly one actual contract proof".into());}
    let rec=&native.records[0];let pis=finalcall.public_inputs(parent.binding_commitment.into());
    if pis!=rec.pis || pis.first()!=Some(&rec.binding) {return Err("native PIs differ from finalized ledger call".into());}
    exclusive(&out.join("proof.raw"),&rec.proof.0)?;exclusive(&out.join("statement.tagged"),&encoded(&pis)?)?;exclusive(&out.join("skips.json"),&serde_json::to_vec_pretty(&rec.skips)?)?;
    drop(native);
    let Transaction::Standard(mut ps)=proven else {return Err("not standard".into())};
    let signed=ps.intents.get(&SEGMENT).ok_or("intent absent")?.deref().clone().sign(&mut rng,SEGMENT,&[key.clone()],&[],&[key])?;
    ps.intents=ps.intents.insert(SEGMENT,signed);
    let tx=Transaction::Standard(ps).seal(rng);
    let (sc,sp)=single_call(&tx)?;
    if sc.public_inputs(sp.binding_commitment.into())!=pis {return Err("sign/seal changed finalized statement".into());}
    exclusive(&out.join("transaction.tagged"),&encoded(&tx)?)?;
    let evidence=accept(&tx,&state,&c.fixture,&expected,out)?;
    exclusive(&out.join("receipt.json"),&serde_json::to_vec_pretty(&evidence)?)?;
    println!("CONDITIONAL_TRUSTED_GENESIS_NATIVE_LEDGER_SUCCESS; independent verification and result reviews still required");Ok(())
}
fn accept(tx:&SignedTransaction,state:&LedgerState<DB>,f:&Fixture,expected:&ContractState<DB>,out:&Path)->Result<serde_json::Value> {
    let (call,intent)=single_call(tx)?;
    if call.address!=address() || call.entry_point!=EntryPointBuf(b"pay".to_vec()) {return Err("call identity mismatch".into());}
    let da=intent.dust_actions.as_ref().ok_or("dust registration missing")?;
    if !da.spends.is_empty() || da.registrations.len()!=1 {return Err("auxiliary proof family prohibited".into());}
    let reg=da.registrations.get(0).ok_or("registration absent")?;
    if reg.dust_address.is_none() || reg.allow_fee_payment!=f.fee_allowance {return Err("registration mismatch".into());}
    let d=availability(f,state,&funding(f,&fixture_key()?))?;
    let verified=tx.well_formed(state,WellFormedStrictness::default(),block(f).tblock)?;
    let (after,result)=state.apply(&verified,&TransactionContext{ref_state:state.clone(),block_context:block(f),whitelist:None});
    // Preserve partial/guaranteed failure evidence before refusing a full-success claim.
    exclusive(&out.join("application-result.txt"),format!("{result:#?}").as_bytes())?;
    exclusive(&out.join("poststate.tagged"),&encoded(&after)?)?;
    if !matches!(&result,TransactionResult::Success(_)) {return Err("not full native Success; inspect retained guaranteed/fallible fee effects".into());}
    let cs=after.index(address()).ok_or("contract absent")?;
    if cs.balance.get(&asset()).map(|x|*x)!=Some(8990) || cs.data.get_ref()!=expected.data.get_ref() {return Err("native final storage/escrow mismatch".into());}
    if after.utxo.utxos.contains_key(&Utxo::from(funding(f,&fixture_key()?))) {return Err("funding input survived".into());}
    let mut a1=vec![];let mut night=vec![];
    for kv in after.utxo.utxos.iter(){let u=kv.0.deref();if u.type_==UnshieldedTokenType(HashOutput([0xa1;32])){a1.push((u.owner,u.value));}else if u.type_==NIGHT{night.push((u.owner,u.value));}else{return Err("unexpected output asset".into());}}
    a1.sort();let mut expected_a1=vec![(recipient(),1000u128),(fee_recipient(),10u128)];expected_a1.sort();
    if a1!=expected_a1 || night!=vec![(UserAddress::from(fixture_key()?.verifying_key()),f.night_value)] {return Err("complete native UTXO conservation mismatch".into());}
    let dust_values:Vec<u128>=result.events().iter().filter_map(|e|match &e.content{EventDetails::DustInitialUtxo{output,..}=>Some(output.initial_value),_=>None}).collect();
    if dust_values.len()!=1 || dust_values[0]>d {return Err("native dust accounting event mismatch".into());}
    // Exactly one same-owner NIGHT output makes native distribution ratio exactly10000.
    // Consequently D - emitted initial_value is actual consumed protocol fee, without private fees getter.
    let consumed=d-dust_values[0];if consumed==0 || consumed>f.fee_allowance {return Err("protocol fees not fully evidenced within allowance".into());}
    if tx.balance(Some(consumed))?.values().any(|v|*v<0) {return Err("observed fee balancing mismatch".into());}
    let (replayed,replay_result)=after.apply(&verified,&TransactionContext{ref_state:after.clone(),block_context:block(f),whitelist:None});
    if !matches!(replay_result,TransactionResult::Failure(_)) || replayed.state_hash()!=after.state_hash() {return Err("identical transaction replay changed accepted state".into());}
    exclusive(&out.join("replay-refusal.txt"),format!("{replay_result:#?}").as_bytes())?;
    Ok(serde_json::json!({"scope":"conditional strict ledger9 from explicit trusted genesis; no authenticated mint/deploy/funding or Preview","proof_count":1,"strictness":"native default Real; proof-verifying enabled","runtime_ops":293,"runtime_reads":47,"prestate_sha256":sha(&encoded(state)?),"poststate_sha256":sha(&encoded(&after)?),"statement_sha256":sha(&encoded(&call.public_inputs(intent.binding_commitment.into()))?),"generationless_available":d.to_string(),"allow_fee_payment":f.fee_allowance.to_string(),"actual_native_fee_consumed":consumed.to_string(),"dust_remainder":dust_values[0].to_string(),"result":"Success","events":format!("{:#?}",result.events())}))
}
fn main()->Result<()> {
    let args:Vec<_>=std::env::args().skip(1).collect();
    if args.len()<3 {return Err("keygen CONFIG PIN_SHA NEW_OUTPUT | prepare CONFIG PIN_SHA NEW_OUTPUT | preflight CONFIG PIN_SHA NEW_OUTPUT | prove CONFIG PIN_SHA NEW_OUTPUT | verify CONFIG PIN_SHA NEW_OUTPUT".into());}
    let bytes=read_pinned(Path::new(&args[1]),&args[2])?;
    if args.len()!=4 {return Err("exactly four arguments required".into());}
    match args[0].as_str(){"preflight"=>preflight(serde_json::from_slice(&bytes)?,Path::new(&args[3])),"keygen"=>tokio::runtime::Builder::new_current_thread().build()?.block_on(keygen::run(serde_json::from_slice(&bytes)?,&args[2],Path::new(&args[3]))),"prepare"=>prepare(serde_json::from_slice(&bytes)?,Path::new(&args[3])),"prove"=>tokio::runtime::Builder::new_current_thread().build()?.block_on(prove(serde_json::from_slice(&bytes)?,Path::new(&args[3]))),"verify"=>verify::run(serde_json::from_slice(&bytes)?,Path::new(&args[3])),_=>Err("unknown mode".into())}
}
