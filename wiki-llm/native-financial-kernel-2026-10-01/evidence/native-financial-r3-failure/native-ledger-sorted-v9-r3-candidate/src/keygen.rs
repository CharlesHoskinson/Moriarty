//! Dedicated offline key generation; source-only and resource authority remains separate.
use crate::*;
const IR_SHA: &str = "c20c6e733500823c478eea885bebab5d62ba9958cfdd1ec8cf2ae89b32f9d7ae";
const SRS_BYTES: u64 = 25_166_212;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config { ir: Artifact, srs: Artifact }

pub async fn run(c: Config, config_sha: &str, out: &Path) -> Result<()> {
    if c.ir.sha256 != IR_SHA || c.srs.sha256 != SRS_SHA {
        return Err("immutable IR/SRS authority mismatch".into());
    }
    // Reject huge/wrong input before the existing bounded artifact reader allocates.
    if fs::metadata(&c.srs.path)?.len() != SRS_BYTES {
        return Err("exact k17 SRS byte length required".into());
    }
    let ir_bytes = artifact(&c.ir)?;
    let ir = IrSource::load(ir_bytes.as_slice())?;
    if ir.k() != 17 { return Err("only current frozen k17 IR permitted".into()); }
    let params_bytes = artifact(&c.srs)?;
    if params_bytes.len() as u64 != SRS_BYTES { return Err("SRS changed length".into()); }
    let params = provider::Parameters { bytes: params_bytes };
    // The fixed provider has no network/OnDemand path and enforces full decode EOF.
    let decoded_params = params.get_params(17).await?;
    drop(decoded_params);
    fs::create_dir(out)?; // existing outputs fail; no prior keys overwritten
    exclusive(&out.join("keygen-started.json"), &serde_json::to_vec_pretty(&serde_json::json!({
        "scope":"keygen started; no usable keys until success receipt",
        "config_sha256":config_sha,"ir_sha256":IR_SHA,"srs_sha256":SRS_SHA,
        "srs_bytes":SRS_BYTES,"k":17,"keygen_complete":false,
        "proof_produced":false,"ledger_accepted":false
    }))?)?;
    let result = generate(&ir, &params, config_sha, out).await;
    if result.is_err() {
        // A hard resource stop may only leave started/partial artifacts; never a success receipt.
        let _ = exclusive(&out.join("keygen-failed.json"),
            b"{\"keygen_complete\":false,\"partial_keys_usable\":false,\"proof_produced\":false,\"ledger_accepted\":false}");
    }
    result
}
async fn generate(ir: &IrSource, params: &provider::Parameters, config_sha: &str, out: &Path) -> Result<()> {
    let (pk, vk) = ir.keygen(params).await?; // actual public Zkir trait API
    let pk_bytes = encoded(&pk)?;
    let vk_bytes = encoded(&vk)?;
    let tagged_ir = encoded(ir)?;
    // Official tagged loaders plus full EOF; roundtrip bytes must preserve generated identities.
    let mut c = Cursor::new(pk_bytes.as_slice());
    let restored_pk = IrSource::load_prover_key_from_tagged(&mut c)?;
    if c.position() != pk_bytes.len() as u64 || encoded(&restored_pk)? != pk_bytes {
        return Err("generated PK tagged identity/EOF mismatch".into());
    }
    let restored_vk: VerifierKey = decode(&vk_bytes)?;
    if encoded(&restored_vk)? != vk_bytes { return Err("generated VK tagged identity mismatch".into()); }
    let mut c = Cursor::new(tagged_ir.as_slice());
    let restored_ir = IrSource::load_ir_from_tagged(&mut c)?;
    if c.position() != tagged_ir.len() as u64 || restored_ir.k() != 17 || encoded(&restored_ir)? != tagged_ir {
        return Err("generated IR tagged identity/EOF mismatch".into());
    }
    let mut records = Vec::new();
    for (name, bytes) in [("pk.tagged", &pk_bytes), ("vk.tagged", &vk_bytes), ("ir.tagged", &tagged_ir)] {
        exclusive(&out.join(name), bytes)?;
        records.push(serde_json::json!({"name":name,"sha256":sha(bytes),"bytes":bytes.len()}));
    }
    exclusive(&out.join("keygen-success.json"), &serde_json::to_vec_pretty(&serde_json::json!({
        "scope":"actual offline native key generation only; not a financial proof or ledger acceptance",
        "config_sha256":config_sha,"input_ir_sha256":IR_SHA,"srs_sha256":SRS_SHA,
        "srs_bytes":SRS_BYTES,"k":17,"keygen_complete":true,"artifacts":records,
        "source_format":"JSON IR3.1; official tagged PK/VK/IR outputs",
        "tagged_roundtrip_eof_identity":true,"srs_ceremony_independently_audited":false,
        "resolver_registration_key_agreement":"NotChecked; required in future actual finalized proof",
        "proof_produced":false,"well_formed_checked":false,"ledger_applied":false,
        "authority_valid":null,"ledger_accepted":false,
        "resources":"separately authorized execution required; source preparation grants none"
    }))?)?;
    println!("NATIVE_KEYGEN_COMPLETE; no financial proof/WF/application/ledger acceptance");
    Ok(())
}
