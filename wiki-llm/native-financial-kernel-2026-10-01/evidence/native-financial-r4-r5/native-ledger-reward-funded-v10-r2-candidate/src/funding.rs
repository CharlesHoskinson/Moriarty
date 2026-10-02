// Public local development fixture trajectory, not authenticated chain genesis.
use crate::*;
use coin_structure::coin::Nonce;
use ledger::error::{MalformedTransaction,SystemTransactionError,TransactionInvalid};
const DISTRIBUTION_NONCE:Nonce=Nonce(HashOutput([0x52;32]));
const CLAIM_NONCE:Nonce=Nonce(HashOutput([0x53;32]));
const SIGNATURE_RNG_SEED:u64=0x46554e44494e4731;
pub(super) fn invariant(s:&LedgerState<DB>,label:&str)->Result<()> {
    s.check_night_balance_invariant().map_err(|e|format!("{label}: native supply invariant: {e:?}"))?;Ok(())
}
fn ctx(s:&LedgerState<DB>,f:&Fixture)->TransactionContext<DB> {
    TransactionContext{ref_state:s.clone(),block_context:BlockContext{tblock:Timestamp::from_secs(f.night_creation_seconds),tblock_err:0,parent_block_hash:HashOutput([0;32]),last_block_time:Timestamp::from_secs(f.night_creation_seconds)},whitelist:None}
}
fn allocate(s:&LedgerState<DB>,f:&Fixture)->Result<(LedgerState<DB>,LedgerState<DB>)> {
    invariant(s,"initial")?;
    if s.reserve_pool!=MAX_SUPPLY || s.locked_pool!=0 || s.block_reward_pool!=0 || s.unclaimed_block_rewards.size()!=0 || s.utxo.utxos.size()!=0 {return Err("funding requires exact pristine reserve allocation origin".into());}
    if f.night_value<s.parameters.min_claimable_rewards() {return Err("fixture reward below native minimum".into());}
    let (reserve,_)=s.apply_system_tx(&SystemTransaction::DistributeReserve{amount:f.night_value},Timestamp::from_secs(f.night_creation_seconds)).map_err(|e|format!("native reserve allocation: {e:?}"))?;
    invariant(&reserve,"reserve allocated")?;
    if reserve.reserve_pool!=MAX_SUPPLY-f.night_value || reserve.block_reward_pool!=f.night_value || reserve.unclaimed_block_rewards.size()!=0 || reserve.utxo.utxos.size()!=0 {return Err("reserve allocation accounting mismatch".into());}
    let instruction=OutputInstructionUnshielded{amount:f.night_value,target_address:UserAddress::from(fixture_key()?.verifying_key()),nonce:DISTRIBUTION_NONCE};
    let (allocated,_)=reserve.apply_system_tx(&SystemTransaction::DistributeNight(ClaimKind::Reward,vec![instruction]),Timestamp::from_secs(f.night_creation_seconds)).map_err(|e|format!("native reward distribution: {e:?}"))?;
    invariant(&allocated,"distributed")?;
    let owner=UserAddress::from(fixture_key()?.verifying_key());
    if allocated.reserve_pool!=reserve.reserve_pool || allocated.block_reward_pool!=0 || allocated.unclaimed_block_rewards.size()!=1 || allocated.unclaimed_block_rewards.get(&owner).copied()!=Some(f.night_value) || allocated.utxo.utxos.size()!=0 {return Err("reward distribution accounting mismatch".into());}
    Ok((reserve,allocated))
}
fn claim(f:&Fixture,value:u128,nonce:Nonce,signer:&SigningKey)->Result<SignedTransaction> {
    let unsigned:ClaimRewardsTransaction<(),DB>=ClaimRewardsTransaction{network_id:f.network.clone(),value,owner:fixture_key()?.verifying_key(),nonce,signature:(),kind:ClaimKind::Reward};
    // Separate deterministic public RNG; financial transaction RNG is unchanged.
    let mut rng=StdRng::seed_from_u64(SIGNATURE_RNG_SEED);
    Ok(Transaction::ClaimRewards(unsigned.add_signature(signer.sign(&mut rng,&unsigned.data_to_sign()))))
}
fn apply_claim(s:&LedgerState<DB>,f:&Fixture,tx:&SignedTransaction)->Result<(LedgerState<DB>,TransactionResult<DB>)> {
    let verified=tx.well_formed(s,WellFormedStrictness::default(),Timestamp::from_secs(f.night_creation_seconds))?;
    Ok(s.apply(&verified,&ctx(s,f)))
}
pub(super) fn fund(s:&LedgerState<DB>,f:&Fixture)->Result<LedgerState<DB>> {
    let (_,allocated)=allocate(s,f)?;
    let tx=claim(f,f.night_value,CLAIM_NONCE,&fixture_key()?)?;
    let (funded,result)=apply_claim(&allocated,f,&tx)?;
    if !matches!(&result,TransactionResult::Success(_)){return Err(format!("signed reward funding not full native Success: {result:?}").into());}
    invariant(&funded,"signed reward claimed")?;
    if funded.reserve_pool!=MAX_SUPPLY-f.night_value || funded.block_reward_pool!=0 || funded.unclaimed_block_rewards.size()!=0 || funded.utxo.utxos.size()!=1 {return Err("signed reward claim accounting mismatch".into());}
    let _=spend(&funded,f,&fixture_key()?)?;Ok(funded)
}
pub(super) fn spend(s:&LedgerState<DB>,f:&Fixture,key:&SigningKey)->Result<UtxoSpend> {
    let owner=UserAddress::from(key.verifying_key());let mut found=None;
    let expected_hash=OutputInstructionUnshielded{amount:f.night_value,target_address:owner,nonce:CLAIM_NONCE}.mk_intent_hash(NIGHT);
    for kv in s.utxo.utxos.iter(){let u=kv.0.deref();if u.type_==NIGHT {
        if found.is_some() || u.owner!=owner || u.value!=f.night_value || u.intent_hash!=expected_hash || u.output_no!=0 || kv.1.ctime!=Timestamp::from_secs(f.night_creation_seconds) {return Err("actual reward-funded NIGHT identity/time/value mismatch".into());}
        found=Some(UtxoSpend{value:u.value,owner:key.verifying_key(),type_:u.type_,intent_hash:u.intent_hash,output_no:u.output_no});
    }}
    found.ok_or_else(||"actual reward-funded NIGHT absent".into())
}
fn unchanged(before:&LedgerState<DB>,after:&LedgerState<DB>,label:&str)->Result<()> {
    if encoded(before)?!=encoded(after)?{return Err(format!("{label}: exact original state changed").into());}invariant(after,label)
}
fn refused_apply(s:&LedgerState<DB>,f:&Fixture,tx:&SignedTransaction,label:&str,predicate:fn(&TransactionInvalid<DB>)->bool)->Result<String> {
    let before=encoded(s)?;let (after,result)=apply_claim(s,f,tx)?;
    match &result {TransactionResult::Failure(e) if predicate(e)=>{},_=>return Err(format!("{label}: unexpected native result {result:?}").into())}
    unchanged(s,&after,label)?;if encoded(s)?!=before{return Err("original fixture mutated".into());}Ok(format!("{result:?}"))
}
pub(super) fn preflight(c:PrepareConfig,out:&Path)->Result<()> {
    // No IR load, k row model, VK/SRS/provider or proof call in this mode.
    if c.fixture.trust!="TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY" || c.fixture.night_creation_seconds>c.fixture.block_seconds {return Err("fixture trust/time invalid".into());}
    fs::create_dir(out)?;
    let initial:ContractState<DB>=decode(&artifact(&c.initial_contract)?)?;
    let pristine=LedgerState::<DB>::new(c.fixture.network.clone());let before=encoded(&pristine)?;
    invariant(&pristine,"pristine")?;
    let (_,allocated)=allocate(&pristine,&c.fixture)?;let allocated_before=encoded(&allocated)?;
    let key=fixture_key()?;let good=claim(&c.fixture,c.fixture.night_value,CLAIM_NONCE,&key)?;
    let wrong=SigningKey::Schnorr(base_crypto::schnorr::SigningKey::from_bytes(&[0x43;32])?);
    let wrong_tx=claim(&c.fixture,c.fixture.night_value,CLAIM_NONCE,&wrong)?;
    if !matches!(wrong_tx.well_formed(&allocated,WellFormedStrictness::default(),Timestamp::from_secs(c.fixture.night_creation_seconds)),Err(MalformedTransaction::IntentSignatureVerificationFailure)){return Err("wrong native signature not refused".into());}
    if encoded(&allocated)?!=allocated_before{return Err("wrong-signature refusal changed original snapshot".into());}
    let Transaction::ClaimRewards(original)=&good else{return Err("not reward claim".into())};
    let mut owner=original.erase_signatures();owner.owner=wrong.verifying_key();let mut rng=StdRng::seed_from_u64(SIGNATURE_RNG_SEED);
    let bad_owner:SignedTransaction=Transaction::ClaimRewards(owner.add_signature(key.sign(&mut rng,&owner.data_to_sign())));
    if !matches!(bad_owner.well_formed(&allocated,WellFormedStrictness::default(),Timestamp::from_secs(c.fixture.night_creation_seconds)),Err(MalformedTransaction::IntentSignatureVerificationFailure)){return Err("wrong owner binding not refused".into());}
    let mut network=original.erase_signatures();network.network_id="wrong-network".into();let mut rng=StdRng::seed_from_u64(SIGNATURE_RNG_SEED);
    let bad_network:SignedTransaction=Transaction::ClaimRewards(network.add_signature(key.sign(&mut rng,&network.data_to_sign())));
    if !matches!(bad_network.well_formed(&allocated,WellFormedStrictness::default(),Timestamp::from_secs(c.fixture.night_creation_seconds)),Err(MalformedTransaction::InvalidNetworkId{..})){return Err("wrong network not refused".into());}
    if encoded(&allocated)?!=allocated_before{return Err("owner/network refusals changed original snapshot".into());}
    // Different owner with its own valid signature: WF succeeds, allocation fails.
    let mut unfunded_owner=original.erase_signatures();unfunded_owner.owner=wrong.verifying_key();let mut rng=StdRng::seed_from_u64(SIGNATURE_RNG_SEED);
    let unfunded_tx:SignedTransaction=Transaction::ClaimRewards(unfunded_owner.add_signature(wrong.sign(&mut rng,&unfunded_owner.data_to_sign())));
    let unfunded=refused_apply(&allocated,&c.fixture,&unfunded_tx,"valid signed unfunded owner",|e|matches!(e,TransactionInvalid::InsufficientClaimable{claimable:0,..}))?;
    let replay=refused_apply(&allocated,&c.fixture,&claim(&c.fixture,c.fixture.night_value,DISTRIBUTION_NONCE,&key)?,"distribution nonce",|e|matches!(e,TransactionInvalid::ReplayProtectionViolation(_)))?;
    let insufficient=refused_apply(&allocated,&c.fixture,&claim(&c.fixture,c.fixture.night_value.checked_add(1).ok_or("claim overflow")?,CLAIM_NONCE,&key)?,"insufficient claim",|e|matches!(e,TransactionInvalid::InsufficientClaimable{..}))?;
    let minimum=allocated.parameters.min_claimable_rewards();if minimum==0{return Err("native reward minimum unexpectedly zero".into());}
    let small=refused_apply(&allocated,&c.fixture,&claim(&c.fixture,minimum-1,CLAIM_NONCE,&key)?,"below minimum",|e|matches!(e,TransactionInvalid::RewardTooSmall{..}))?;
    let reserve_bad=pristine.apply_system_tx(&SystemTransaction::DistributeReserve{amount:MAX_SUPPLY.checked_add(1).ok_or("supply overflow")?},Timestamp::from_secs(c.fixture.night_creation_seconds));
    if !matches!(reserve_bad,Err(SystemTransactionError::IllegalReserveDistribution{..})){return Err("insufficient reserve not refused".into());}
    let pool_bad=pristine.apply_system_tx(&SystemTransaction::DistributeNight(ClaimKind::Reward,vec![OutputInstructionUnshielded{amount:c.fixture.night_value,target_address:UserAddress::from(key.verifying_key()),nonce:DISTRIBUTION_NONCE}]),Timestamp::from_secs(c.fixture.night_creation_seconds));
    if !matches!(pool_bad,Err(SystemTransactionError::IllegalPayout{..})){return Err("insufficient reward pool not refused".into());}
    if encoded(&pristine)?!=before || encoded(&allocated)?!=allocated_before{return Err("native failed controls mutated originals".into());}
    let (funded,result)=apply_claim(&allocated,&c.fixture,&good)?;if !matches!(result,TransactionResult::Success(_)){return Err("pristine valid claim failed".into());}invariant(&funded,"funded")?;
    let duplicate=refused_apply(&funded,&c.fixture,&good,"duplicate claimed allocation",|e|matches!(e,TransactionInvalid::InsufficientClaimable{..}))?;
    let good_again=fund(&pristine,&c.fixture)?;if encoded(&funded)?!=encoded(&good_again)?{return Err("deterministic pristine good-again differed".into());}
    let full=genesis(&c.fixture,initial,None)?;invariant(&full,"complete fixture genesis")?;
    let spend=spend(&full,&c.fixture,&key)?;let meta=full.utxo.utxos.get(&Utxo::from(spend.clone())).ok_or("produced funding metadata absent")?;
    exclusive(&out.join("funded-genesis.tagged"),&encoded(&full)?)?;
    record(&full,&c.fixture,out)?;
    exclusive(&out.join("funding-preflight.json"),&serde_json::to_vec_pretty(&serde_json::json!({"scope":"actual native local privileged reserve/reward funding and signed sealed claim only; no financial proof/Preview","result":"Success","native_default_well_formed":true,"native_claim_applied":true,"supply_invariants":true,"wrong_signature":"IntentSignatureVerificationFailure","wrong_owner":"IntentSignatureVerificationFailure","wrong_network":"InvalidNetworkId","same_distribution_nonce":replay,"valid_signed_unfunded_owner":unfunded,"insufficient_claim":insufficient,"below_minimum":small,"duplicate_claim":duplicate,"reserve_insufficiency":"IllegalReserveDistribution","pool_insufficiency":"IllegalPayout","failed_originals_unchanged":true,"pristine_good_again_equal":true,"funding_spend":format!("{spend:?}"),"actual_ctime":meta.ctime.to_secs(),"genesis_sha256":sha(&encoded(&full)?),"authority":null,"proof_produced":false,"financial_ledger_accepted":false}))?)?;
    println!("LOCAL_NATIVE_REWARD_FUNDING_PREFLIGHT_ONLY; no financial proof/Preview");Ok(())
}

fn history(state:&LedgerState<DB>,f:&Fixture)->Result<(Vec<LedgerState<DB>>,SignedTransaction)> {
    invariant(state,"funding history final genesis")?;
    let mut initial=LedgerState::<DB>::new(f.network.clone());initial.contract=state.contract.clone();
    invariant(&initial,"funding history initial")?;
    let (reserve,distributed)=allocate(&initial,f)?;
    let tx=claim(f,f.night_value,CLAIM_NONCE,&fixture_key()?)?;
    let (funded,result)=apply_claim(&distributed,f,&tx)?;
    if !matches!(result,TransactionResult::Success(_)){return Err("history signed native claim failed".into());}
    invariant(&funded,"history signed claim")?;
    let _=spend(&funded,f,&fixture_key()?)?;
    if encoded(&funded)?!=encoded(state)?{return Err("funding predecessor reconstruction differs from actual genesis".into());}
    Ok((vec![initial,reserve,distributed,funded],tx))
}
pub(super) fn record(state:&LedgerState<DB>,f:&Fixture,out:&Path)->Result<()> {
    let (states,tx)=history(state,f)?;
    exclusive(&out.join("funding-history.tagged"),&encoded(&states)?)?;
    exclusive(&out.join("funding-claim.tagged"),&encoded(&tx)?)?;
    let hashes=states.iter().map(|s|encoded(s).map(|b|sha(&b))).collect::<Result<Vec<_>>>()?;
    exclusive(&out.join("funding-history.json"),&serde_json::to_vec_pretty(&serde_json::json!({"scope":"deterministic actual local privileged funding predecessors; not chain authorization","state_order":["default-with-manual-A1-contract","reserve-to-reward-pool","reward-pool-to-unclaimed","signed-sealed-claim-to-utxo"],"state_sha256":hashes,"signed_claim_sha256":sha(&encoded(&tx)?),"distribution_nonce":format!("{DISTRIBUTION_NONCE:?}"),"claim_nonce":format!("{CLAIM_NONCE:?}"),"funding_ctime":f.night_creation_seconds,"authority":null}))?)?;Ok(())
}
pub(super) fn verify_history(state:&LedgerState<DB>,f:&Fixture,history_bytes:&[u8],claim_bytes:&[u8])->Result<()> {
    let supplied:Vec<LedgerState<DB>>=decode(history_bytes)?;let claim:SignedTransaction=decode(claim_bytes)?;
    if encoded(&supplied)?!=history_bytes || encoded(&claim)?!=claim_bytes{return Err("noncanonical funding history/claim".into());}
    let (expected,expected_claim)=history(state,f)?;
    if encoded(&expected)?!=history_bytes || encoded(&expected_claim)?!=claim_bytes{return Err("supervisor-pinned funding predecessor history differs from independent native reconstruction".into());}
    for s in &supplied {invariant(s,"independent predecessor history")?;}Ok(())
}
