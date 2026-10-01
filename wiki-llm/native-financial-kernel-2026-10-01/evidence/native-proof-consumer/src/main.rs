use midnight_zkir::IrSource;
use rand::rngs::OsRng;
use serde::Deserialize;
use serialize::{tagged_deserialize, tagged_serialize, Deserializable, Serializable, Tagged};
use sha2::{Digest, Sha256};
use std::{error::Error, fs::{self, OpenOptions}, io::{self, Cursor, Read, Write}, path::{Path, PathBuf}};
use transient_crypto::{curve::Fr, proofs::{KeyLocation, ParamsProver, ParamsProverProvider, ParamsVerifier, Proof, ProofPreimage, ProvingKeyMaterial, Resolver, VerifierKey, Zkir}};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const SRS_SHA: &str = "4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74";
const LOCATION: &str = "local-public-fixture/pay/c20c6e733500823c478eea885bebab5d62ba9958cfdd1ec8cf2ae89b32f9d7ae";
const MAX_FILE: u64 = 2 * 1024 * 1024 * 1024;
#[derive(Deserialize)] #[serde(deny_unknown_fields)]
struct Artifact { path: PathBuf, sha256: String }
#[derive(Deserialize)] #[serde(deny_unknown_fields)]
struct ProveConfig { ir: Artifact, pk: Artifact, vk: Artifact, preimage: Artifact, srs: Artifact }
#[derive(Deserialize)] #[serde(deny_unknown_fields)]
struct VerifyConfig { vk: Artifact, srs: Artifact, proof: Artifact, statement: Artifact }
fn sha(bytes: &[u8]) -> String { format!("{:x}", Sha256::digest(bytes)) }
fn read_pinned(path: &Path, expected: &str) -> Result<Vec<u8>> {
    if expected.len()!=64 || !expected.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c)) {return Err("invalid supervisor SHA256".into());}
    let file=fs::File::open(path)?; let mut bytes=Vec::new(); file.take(MAX_FILE+1).read_to_end(&mut bytes)?;
    if bytes.len() as u64>MAX_FILE {return Err("artifact exceeds source read ceiling".into());}
    if sha(&bytes)!=expected {return Err(format!("host artifact identity mismatch: {}",path.display()).into());} Ok(bytes)
}
fn artifact(a: &Artifact) -> Result<Vec<u8>> {read_pinned(&a.path,&a.sha256)}
fn decode<T: Deserializable+Tagged>(bytes: &[u8]) -> Result<T> {
    let mut cursor=Cursor::new(bytes);let value=tagged_deserialize(&mut cursor)?;
    if cursor.position()!=bytes.len() as u64 {return Err("outer tagged decoder trailing bytes".into());}Ok(value)
}
fn encoded<T: Serializable+Tagged>(value:&T)->Result<Vec<u8>> {let mut bytes=Vec::new();tagged_serialize(value,&mut bytes)?;Ok(bytes)}
fn exclusive(path:&Path,bytes:&[u8])->Result<()> {let mut f=OpenOptions::new().write(true).create_new(true).open(path)?;f.write_all(bytes)?;f.sync_all()?;Ok(())}
struct LocalParams { bytes: Vec<u8> }
impl ParamsProverProvider for LocalParams {
    async fn get_params(&self,k:u8)->io::Result<ParamsProver> {
        if k!=17 {return Err(io::Error::other("local provider rejects k other than17"));}
        let mut cursor=Cursor::new(self.bytes.as_slice());let p=ParamsProver::read(&mut cursor)?;
        if cursor.position()!=self.bytes.len() as u64 {return Err(io::Error::other("parameter trailing bytes"));}Ok(p)
    }
}
struct LocalResolver { material: ProvingKeyMaterial }
impl Resolver for LocalResolver {
    async fn resolve_key(&self,key:KeyLocation)->io::Result<Option<ProvingKeyMaterial>> {
        Ok(if key.0.as_ref()==LOCATION {Some(self.material.clone())}else{None})
    }
}
fn params_verifier(bytes:&[u8])->Result<ParamsVerifier> {
    let mut cursor=Cursor::new(bytes);let p=ParamsVerifier::read(&mut cursor)?;
    if cursor.position()!=bytes.len() as u64 {return Err("verifier parameter trailing bytes".into());}Ok(p)
}
async fn prove(config:ProveConfig,out:&Path)->Result<()> {
    if config.srs.sha256!=SRS_SHA {return Err("host pinned k17 SRS identity mismatch".into());}
    let ir_bytes=artifact(&config.ir)?;let ir=IrSource::load(&ir_bytes[..])?;
    if ir.k()!=17 {return Err("source model is not approved k17".into());}
    let pk_bytes=artifact(&config.pk)?;let vk_bytes=artifact(&config.vk)?;let preimage:ProofPreimage=decode(&artifact(&config.preimage)?)?;
    if preimage.key_location.0.as_ref()!=LOCATION || preimage.binding_input!=0u8.into() {return Err("host local fixture key/binding mismatch; not relation refusal".into());}
    let parameters=artifact(&config.srs)?;
    let resolver=LocalResolver{material:ProvingKeyMaterial{prover_key:pk_bytes,verifier_key:vk_bytes,ir_source:encoded(&ir)?}};
    let material=resolver.resolve_key(preimage.key_location.clone()).await?.ok_or("local key unresolved")?;
    let mut ic=Cursor::new(material.ir_source.as_slice());let loaded_ir=IrSource::load_ir_from_tagged(&mut ic)?;
    if ic.position()!=material.ir_source.len() as u64 {return Err("tagged IR trailing bytes".into());}
    let mut pc=Cursor::new(material.prover_key.as_slice());let pk=IrSource::load_prover_key_from_tagged(&mut pc)?;
    if pc.position()!=material.prover_key.len() as u64 {return Err("tagged PK trailing bytes".into());}
    let vk:VerifierKey=decode(&material.verifier_key)?;let pv=params_verifier(&parameters)?;let pp=LocalParams{bytes:parameters};
    // Reserve a new empty output directory before consuming the sole proof attempt.
    fs::create_dir(out)?;
    // Exactly one production proof call. No setup/OnDemand fetch/mock/check substitute.
    let (proof,pis,skips)=loaded_ir.prove(OsRng,&pp,pk,&preimage).await?;
    vk.verify(&pv,&proof,pis.iter().copied())?;
    // No proof-success artifact is published before native self-verification.
    let statement=encoded(&pis)?;let skip_bytes=serde_json::to_vec_pretty(&skips)?;
    exclusive(&out.join("proof.raw"),&proof.0)?;exclusive(&out.join("statement.tagged"),&statement)?;exclusive(&out.join("pi-skips.json"),&skip_bytes)?;
    let receipt=serde_json::json!({"scope":"exact retained manual kernel local binding0; no ledger/Preview","native_self_verified":true,"proof_bytes":proof.0.len(),"proof_sha256":sha(&proof.0),"statement_sha256":sha(&statement),"skips_sha256":sha(&skip_bytes),"ir_sha256":config.ir.sha256,"pk_sha256":config.pk.sha256,"vk_sha256":config.vk.sha256,"srs_sha256":config.srs.sha256,"preimage_sha256":config.preimage.sha256});
    exclusive(&out.join("receipt.json"),&serde_json::to_vec_pretty(&receipt)?)?;
    println!("NATIVE_SELF_VERIFY_OK; artifacts require supervisor freeze and separate verify process");Ok(())
}
fn refused(name:&str,result:std::result::Result<(),transient_crypto::proofs::VerifyingError>)->Result<()> {
    match result {Err(error)=>{println!("NATIVE_REFUSAL {name}: {error:?}");Ok(())},Ok(())=>Err(format!("native control accepted: {name}").into())}
}
fn verify(config:VerifyConfig)->Result<()> {
    if config.srs.sha256!=SRS_SHA {return Err("host pinned k17 SRS identity mismatch".into());}
    let vk_bytes=artifact(&config.vk)?;let statement=artifact(&config.statement)?;let proof_bytes=artifact(&config.proof)?;
    if proof_bytes.is_empty(){return Err("empty proof".into());}
    let vk:VerifierKey=decode(&vk_bytes)?;let pis:Vec<Fr>=decode(&statement)?;if encoded(&pis)?!=statement{return Err("noncanonical statement encoding".into());}
    let pv=params_verifier(&artifact(&config.srs)?)?;let proof=Proof(proof_bytes.clone());
    vk.verify(&pv,&proof,pis.iter().copied())?;println!("INDEPENDENT_NATIVE_GOOD_VERIFY_OK");
    for i in 0..pis.len(){let mut changed=pis.clone();changed[i].0+=Fr::from(1u8).0;refused(&format!("public field {i} +1"),vk.verify(&pv,&proof,changed.into_iter()))?;}
    let mut short=pis.clone();short.pop();refused("removed public field",vk.verify(&pv,&proof,short.into_iter()))?;
    let mut long=pis.clone();long.push(Fr::from(1u8));refused("appended public field",vk.verify(&pv,&proof,long.into_iter()))?;
    let mut corrupt=proof_bytes.clone();corrupt[0]^=1;refused("proof bit corruption",vk.verify(&pv,&Proof(corrupt),pis.iter().copied()))?;
    refused("proof half truncation",vk.verify(&pv,&Proof(proof_bytes[..proof_bytes.len()/2].to_vec()),pis.iter().copied()))?;
    let mut suffix=proof_bytes;suffix.push(0);refused("raw proof suffix EOF",vk.verify(&pv,&Proof(suffix),pis.iter().copied()))?;
    let mut outer=statement;outer.push(0);if decode::<Vec<Fr>>(&outer).is_ok(){return Err("outer statement decoder accepted suffix".into());}println!("DECODER_REFUSAL statement suffix");
    let mut outer=vk_bytes;outer.push(0);if decode::<VerifierKey>(&outer).is_ok(){return Err("outer VK decoder accepted suffix".into());}println!("DECODER_REFUSAL VK suffix");
    vk.verify(&pv,&proof,pis.into_iter())?;println!("INDEPENDENT_NATIVE_ORIGINAL_GOOD_AGAIN_OK");Ok(())
}
fn main()->Result<()> {
    let args:Vec<_>=std::env::args().skip(1).collect();
    if args.len()<3{return Err("usage: prove CONFIG EXPECTED_CONFIG_SHA NEW_OUTPUT_DIR | verify CONFIG EXPECTED_CONFIG_SHA".into());}
    // CONFIG identity comes from supervisor CLI, not self-authored output receipt.
    let config=read_pinned(Path::new(&args[1]),&args[2])?;
    match args[0].as_str(){"prove" if args.len()==4=>tokio::runtime::Builder::new_current_thread().build()?.block_on(prove(serde_json::from_slice(&config)?,Path::new(&args[3]))),"verify" if args.len()==3=>verify(serde_json::from_slice(&config)?),_=>Err("invalid mode/arguments".into())}
}
