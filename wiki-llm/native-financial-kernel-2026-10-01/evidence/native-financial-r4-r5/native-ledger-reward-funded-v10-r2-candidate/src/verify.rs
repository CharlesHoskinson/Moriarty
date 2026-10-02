use crate::*;
#[derive(Deserialize)] #[serde(deny_unknown_fields)]
pub struct VerifyConfig {
    vk:Artifact, srs:Artifact, proof:Artifact, statement:Artifact,
    transaction:Artifact, genesis:Artifact, expected_contract:Artifact, funding_history:Artifact, funding_claim:Artifact, fixture:Fixture,
}
fn refused(name:&str,result:std::result::Result<(),transient_crypto::proofs::VerifyingError>)->Result<()> {
    match result {Err(e)=>{println!("NATIVE_CRYPTO_REFUSAL {name}: {e:?}");Ok(())},Ok(())=>Err(format!("mutation accepted: {name}").into())}
}
pub fn run(c:VerifyConfig,out:&Path)->Result<()> {
    if c.srs.sha256!=SRS_SHA || c.fixture.trust!="TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY" {return Err("SRS/trust identity".into());}
    let vkbytes=artifact(&c.vk)?;let vk:VerifierKey=decode(&vkbytes)?;
    let raw=artifact(&c.proof)?;if raw.is_empty(){return Err("empty proof".into());}
    let proof=Proof(raw.clone());let statement=artifact(&c.statement)?;let pis:Vec<Fr>=decode(&statement)?;
    if encoded(&pis)?!=statement{return Err("noncanonical statement".into());}
    let pv=verifier_parameters(&artifact(&c.srs)?)?;
    let txbytes=artifact(&c.transaction)?;let tx:SignedTransaction=decode(&txbytes)?;
    if encoded(&tx)?!=txbytes{return Err("noncanonical final transaction".into());}
    let statebytes=artifact(&c.genesis)?;let state:LedgerState<DB>=decode(&statebytes)?;
    if encoded(&state)?!=statebytes{return Err("noncanonical genesis".into());}
    funding::invariant(&state,"independent verification genesis")?;
    funding::verify_history(&state,&c.fixture,&artifact(&c.funding_history)?,&artifact(&c.funding_claim)?)?;
    let expected:ContractState<DB>=decode(&artifact(&c.expected_contract)?)?;
    let reconstructed=genesis(&c.fixture,state.index(address()).ok_or("missing genesis contract")?,Some(vk.clone()))?;
    if encoded(&reconstructed)?!=statebytes{return Err("genesis differs from declared exact public fixture".into());}
    let (call,parent)=single_call(&tx)?;
    let registered=state.index(address()).ok_or("contract absent")?.operations.get(&b"pay"[..].into()).ok_or("registered operation absent")?.deref().clone();
    if encoded(registered.v3_vk().ok_or("v3VK absent")?)?!=vkbytes || registered.v2.is_some() {return Err("registered VK identity/version mismatch".into());}
    let ProofVersioned::V3(cp)=&call.proof else{return Err("not actual v3 proof".into())};
    if cp.0!=raw || call.public_inputs(parent.binding_commitment.clone().into())!=pis || pis.first()==Some(&Fr::from(0u8)){return Err("finalized call/proof/statement mismatch or binding0".into());}
    vk.verify(&pv,&proof,pis.iter().copied())?;
    vk.verify(&transient_crypto::proofs::PARAMS_VERIFIER,&proof,pis.iter().copied())?;
    for i in 0..pis.len(){let mut p=pis.clone();p[i].0+=Fr::from(1u8).0;refused(&format!("public field {i}+1"),vk.verify(&pv,&proof,p.into_iter()))?;}
    let mut p=pis.clone();p.pop();refused("missing public field",vk.verify(&pv,&proof,p.into_iter()))?;
    let mut p=pis.clone();p.push(Fr::from(1u8));refused("extra public field",vk.verify(&pv,&proof,p.into_iter()))?;
    let mut bad=call.clone();bad.address=ContractAddress(HashOutput([5;32]));refused("actual call address",vk.verify(&pv,&proof,bad.public_inputs(parent.binding_commitment.clone().into()).into_iter()))?;
    let mut bad=call.clone();bad.entry_point=EntryPointBuf(b"wrong-pay".to_vec());refused("actual call entrypoint",vk.verify(&pv,&proof,bad.public_inputs(parent.binding_commitment.clone().into()).into_iter()))?;
    let mut bad=call.clone();bad.communication_commitment.0+=Fr::from(1u8).0;refused("actual call communication",vk.verify(&pv,&proof,bad.public_inputs(parent.binding_commitment.clone().into()).into_iter()))?;
    if let Some(t)=call.guaranteed_transcript.as_ref().or(call.fallible_transcript.as_ref()) {
        let mut changed=t.deref().clone();changed.gas.compute_time+=state.parameters.cost_model.runtime_cost_model.noop_constant;
        let mut bad=call.clone();if bad.guaranteed_transcript.is_some(){bad.guaranteed_transcript=Some(Sp::new(changed));}else{bad.fallible_transcript=Some(Sp::new(changed));}
        refused("actual declared gas",vk.verify(&pv,&proof,bad.public_inputs(parent.binding_commitment.clone().into()).into_iter()))?;
        let mut changed=t.deref().clone();changed.effects.unshielded_outputs=changed.effects.unshielded_outputs.insert(asset(),1011);
        let mut bad=call.clone();if bad.guaranteed_transcript.is_some(){bad.guaranteed_transcript=Some(Sp::new(changed));}else{bad.fallible_transcript=Some(Sp::new(changed));}
        refused("actual declared effects",vk.verify(&pv,&proof,bad.public_inputs(parent.binding_commitment.clone().into()).into_iter()))?;
    }
    let mut bit=raw.clone();bit[0]^=1;refused("proof corruption",vk.verify(&pv,&Proof(bit),pis.iter().copied()))?;
    refused("proof truncation",vk.verify(&pv,&Proof(raw[..raw.len()/2].to_vec()),pis.iter().copied()))?;
    let mut suffix=raw;suffix.push(0);refused("proof suffix EOF",vk.verify(&pv,&Proof(suffix),pis.iter().copied()))?;
    for (label,bytes) in [("VK",vkbytes),("statement",statement),("transaction",txbytes),("genesis",statebytes)] {
        let mut trailing=bytes;trailing.push(0);
        let accepted=match label{"VK"=>decode::<VerifierKey>(&trailing).is_ok(),"statement"=>decode::<Vec<Fr>>(&trailing).is_ok(),"transaction"=>decode::<SignedTransaction>(&trailing).is_ok(),_=>decode::<LedgerState<DB>>(&trailing).is_ok()};
        if accepted{return Err(format!("decoder accepted {label} suffix").into());}println!("STRICT_DECODER_REFUSAL {label} suffix");
    }
    let Transaction::Standard(mut bad)=tx.clone() else{return Err("not standard".into())};
    let mut i=bad.intents.get(&SEGMENT).ok_or("intent absent")?.deref().clone();
    let mut o=i.guaranteed_unshielded_offer.as_ref().ok_or("offer absent")?.deref().clone();o.signatures=vec![].into();i.guaranteed_unshielded_offer=Some(Sp::new(o));bad.intents=bad.intents.insert(SEGMENT,i);
    if Transaction::Standard(bad).well_formed(&state,WellFormedStrictness::default(),block(&c.fixture).tblock).is_ok(){return Err("missing native ownership signature accepted".into());}
    println!("NATIVE_LEDGER_WELL_FORMED_REFUSAL absent ownership signature");
    fs::create_dir(out)?;
    // Fresh pristine state; no request to reuse the already consumed live UTXO/nonce.
    let evidence=accept(&tx,&state,&c.fixture,&expected,out)?;
    vk.verify(&pv,&proof,pis.into_iter())?;
    exclusive(&out.join("independent-ledger-receipt.json"),&serde_json::to_vec_pretty(&evidence)?)?;
    println!("INDEPENDENT_CONDITIONAL_NATIVE_LEDGER_GOOD_AGAIN_OK");Ok(())
}
