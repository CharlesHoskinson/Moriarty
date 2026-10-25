//! Direct stock V3 proving for a compiled contract call. No daemon or wallet.
//! Private preimages arrive only on stdin; files and receipts contain public data.
use ff::Field;
use midnight_curves::Bls12;
use midnight_proofs::{
    poly::{commitment::Params, kzg::params::ParamsKZG},
    utils::SerdeFormat,
};
use midnight_transient_crypto::{
    curve::Fr,
    proofs::{
        PARAMS_VERIFIER, ParamsProver, ParamsProverProvider, ProofPreimage, VerifierKey, Zkir,
    },
};
use midnight_zkir_v3::IrSource;
use mori_recursion::{
    parent::f_from_canonical_hex,
    util::{f_to_hex, peak_rss_kb, sha256_hex},
};
use std::{
    fs,
    io::{self, Cursor, Read, Write},
    path::Path,
    sync::Arc,
    time::Instant,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn bad(message: &str) -> Box<dyn std::error::Error> {
    io::Error::other(message).into()
}
struct Parameters(ParamsProver);
impl ParamsProverProvider for Parameters {
    async fn get_params(&self, k: u8) -> io::Result<ParamsProver> {
        if self.0.as_ref().max_k() != u32::from(k) {
            return Err(io::Error::other("parameter degree differs"));
        }
        Ok(self.0.clone())
    }
}
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Opening {
    binding_input: String,
    preimage: String,
}
fn decode_opening(input: &[u8], location: &str, overwrite_binding: bool) -> Result<ProofPreimage> {
    let opening: Opening = serde_json::from_slice(input).map_err(|_| bad("invalid pipe input"))?;
    let binding =
        f_from_canonical_hex(&opening.binding_input).map_err(|_| bad("invalid binding field"))?;
    let bytes = hex::decode(opening.preimage).map_err(|_| bad("invalid preimage encoding"))?;
    let mut reader = &bytes[..];
    let mut p: ProofPreimage = midnight_serialize::tagged_deserialize(&mut reader)
        .map_err(|_| bad("invalid preimage codec"))?;
    if !reader.is_empty() || p.key_location.0 != location {
        return Err(bad("preimage location or length differs"));
    }
    // Matches the official proof server's explicit overwriteBindingInput path.
    if overwrite_binding {
        p.binding_input = Fr(binding);
    }
    Ok(p)
}
fn write_new(path: &Path, bytes: &[u8]) -> Result<()> {
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?
        .write_all(bytes)?;
    Ok(())
}
fn run() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 7 || !matches!(args[0].as_str(), "check" | "prove") {
        return Err(bad(
            "usage: compiled_call check|prove IR PK VK OUTPUT SRS KEY_LOCATION",
        ));
    }
    if std::env::var("RAYON_NUM_THREADS").as_deref() != Ok("4") {
        return Err(bad("RAYON_NUM_THREADS must be 4"));
    }
    let mut input = Vec::new();
    io::stdin()
        .take(2 * 1024 * 1024 + 1)
        .read_to_end(&mut input)?;
    if input.len() > 2 * 1024 * 1024 {
        return Err(bad("input exceeds 2 MiB"));
    }
    let preimage = decode_opening(&input, &args[6], args[0] == "prove")?;
    let ir_raw = fs::read(&args[1])?;
    let ir =
        IrSource::load_ir_from_tagged(Cursor::new(&ir_raw)).map_err(|_| bad("invalid V3 IR"))?;
    let skips = ir
        .check(&preimage)
        .map_err(|_| bad("compiled call witness rejected"))?;
    if args[0] == "check" {
        println!(
            "{}",
            serde_json::json!({"schema":"sherlock-compiled-call-check/1","pi_skips":skips})
        );
        return Ok(());
    }
    let started = Instant::now();
    let pk_raw = fs::read(&args[2])?;
    let pk = IrSource::load_prover_key_from_tagged(Cursor::new(&pk_raw))
        .map_err(|_| bad("invalid V3 prover key"))?;
    let k = pk.init().map_err(|_| bad("invalid V3 prover key"))?.k();
    let vk_raw = fs::read(&args[3])?;
    let vk: VerifierKey = midnight_serialize::tagged_deserialize(&mut &vk_raw[..])
        .map_err(|_| bad("invalid V3 verifier key"))?;
    let srs_raw = fs::read(&args[5])?;
    if srs_raw.len() != 201326980
        || sha256_hex(&srs_raw)
            != "1cc62978558fdc1e445cd70cfd9a86ec3c2e2151b6d74811232d37faf9133ff1"
    {
        return Err(bad("SRS differs from pinned ceremony"));
    }
    let mut params =
        ParamsKZG::<Bls12>::read_custom(&mut Cursor::new(srs_raw), SerdeFormat::RawBytesUnchecked)?;
    if params.max_k() != 20 || k > 20 {
        return Err(bad("parameter degree exceeds pinned SRS"));
    }
    params.downsize(u32::from(k));
    let params = Parameters(ParamsProver(Arc::new(params)));
    let fresh = futures::executor::block_on(ir.keygen_vk(&params))
        .map_err(|_| bad("compiled VK reconstruction failed"))?;
    let mut fresh_raw = Vec::new();
    midnight_serialize::tagged_serialize(&fresh, &mut fresh_raw)?;
    if fresh_raw != vk_raw {
        return Err(bad("compiled IR and installed VK differ"));
    }
    let prove_start = Instant::now();
    let (proof, public, proved_skips) =
        futures::executor::block_on(ir.prove(rand::rngs::OsRng, &params, pk, &preimage))
            .map_err(|_| bad("compiled V3 proving failed"))?;
    let prove_seconds = prove_start.elapsed().as_secs_f64();
    if proved_skips != skips {
        return Err(bad("compiled public padding differs"));
    }
    let verify_start = Instant::now();
    vk.verify(&PARAMS_VERIFIER, &proof, public.iter().copied())
        .map_err(|_| bad("stock verification rejected compiled proof"))?;
    let verify_seconds = verify_start.elapsed().as_secs_f64();
    let mut wrong = public.clone();
    if wrong.is_empty() {
        return Err(bad("empty compiled public statement"));
    }
    wrong[0].0 += mori_recursion::util::F::ONE;
    if vk
        .verify(&PARAMS_VERIFIER, &proof, wrong.into_iter())
        .is_ok()
    {
        return Err(bad("changed binding accepted"));
    }
    let mut tagged = Vec::new();
    midnight_serialize::tagged_serialize(&proof, &mut tagged)?;
    let output = Path::new(&args[4]);
    write_new(output, &tagged)?;
    let receipt = serde_json::json!({"schema":"sherlock-compiled-call-proof/1","prover":"midnight-proofs 0.8.3 registry stock",
        "k":k,"ir_sha256":sha256_hex(&ir_raw),"pk_sha256":sha256_hex(&pk_raw),"vk_sha256":sha256_hex(&vk_raw),
        "proof_bytes":proof.0.len(),"proof_sha256":sha256_hex(&proof.0),"public_inputs":public.iter().map(|v|f_to_hex(&v.0)).collect::<Vec<_>>(),
        "prove_seconds":prove_seconds,"local_verify_seconds":verify_seconds,"load_reconstruct_prove_verify_seconds":started.elapsed().as_secs_f64(),
        "peak_rss_bytes":peak_rss_kb()*1024,"negative_changed_binding":"rejected","opening_persisted":false,"submitted":false});
    write_new(
        &output.with_extension("json"),
        &(serde_json::to_string_pretty(&receipt)? + "\n").into_bytes(),
    )?;
    println!(
        "PASS direct stock V3 proof k={k}; changed binding refused; private opening not persisted"
    );
    Ok(())
}
fn main() {
    std::panic::set_hook(Box::new(|_| {
        eprintln!("direct compiled-call worker aborted; private details suppressed");
    }));
    if let Err(error) = run() {
        eprintln!("direct compiled-call worker failed: {error}; private request not logged");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use midnight_transient_crypto::proofs::KeyLocation;
    fn encoded() -> serde_json::Value {
        let p = ProofPreimage {
            inputs: vec![],
            private_transcript: vec![],
            public_transcript_inputs: vec![],
            public_transcript_outputs: vec![],
            binding_input: Fr(mori_recursion::util::F::ZERO),
            communications_commitment: None,
            key_location: KeyLocation("expected".into()),
        };
        let mut raw = Vec::new();
        midnight_serialize::tagged_serialize(&p, &mut raw).unwrap();
        serde_json::json!({"binding_input":f_to_hex(&mori_recursion::util::F::from(9)),"preimage":hex::encode(raw)})
    }
    #[test]
    fn pipe_codec_binds_the_supplied_field_and_exact_location() {
        let raw = serde_json::to_vec(&encoded()).unwrap();
        let p = decode_opening(&raw, "expected", true).unwrap();
        assert_eq!(p.binding_input.0, mori_recursion::util::F::from(9));
        assert_eq!(
            decode_opening(&raw, "expected", false)
                .unwrap()
                .binding_input
                .0,
            mori_recursion::util::F::ZERO
        );
        assert!(decode_opening(&raw, "other", true).is_err());
        let mut raw = encoded();
        raw["preimage"] = (raw["preimage"].as_str().unwrap().to_string() + "00").into();
        assert!(decode_opening(&serde_json::to_vec(&raw).unwrap(), "expected", true).is_err());
        let mut raw = encoded();
        raw["extra"] = true.into();
        assert!(decode_opening(&serde_json::to_vec(&raw).unwrap(), "expected", true).is_err());
        let mut raw = encoded();
        raw["binding_input"] =
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".into();
        assert!(decode_opening(&serde_json::to_vec(&raw).unwrap(), "expected", true).is_err());
    }
}
