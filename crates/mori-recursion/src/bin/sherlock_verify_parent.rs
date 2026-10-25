//! Source45-compatible, read-only parent verifier for the evaluator-owned rejection suite.
use ff::{Field, PrimeField};
use midnight_circuits::{
    hash::poseidon::PoseidonState,
    types::Instantiable,
    verifier::{Accumulator, BlstrsEmulation, Msm, SelfEmulation},
};
use midnight_curves::{Bls12, Fq};
use midnight_proofs::{poly::kzg::params::ParamsKZG, utils::SerdeFormat};
use midnight_zk_stdlib::{MidnightVK, ZkStdLibArch, batch_verify};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, error::Error, fs, io::Cursor, path::Path};

type S = BlstrsEmulation;
type AP = <S as SelfEmulation>::AssignedPoint;
fn bounded(path: &Path, max: u64) -> Result<Vec<u8>, Box<dyn Error>> {
    if fs::metadata(path)?.len() > max {
        return Err("input exceeds size bound".into());
    }
    Ok(fs::read(path)?)
}
fn run() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.first().is_some_and(|s| s == "sherlock-verify-parent") {
        args.remove(0);
    }
    if args.len() != 8 {
        return Err(
            "usage: sherlock-verify-parent --vk FILE --proof FILE --instance FILE --srs14 FILE"
                .into(),
        );
    }
    let mut paths = BTreeMap::new();
    for pair in args.as_chunks::<2>().0 {
        if !["--vk", "--proof", "--instance", "--srs14"].contains(&pair[0].as_str())
            || paths
                .insert(pair[0].as_str(), Path::new(&pair[1]))
                .is_some()
        {
            return Err("unexpected or repeated argument".into());
        }
    }
    let srs = bounded(paths["--srs14"], 3_146_116)?;
    if srs.len() != 3_146_116
        || hex::encode(Sha256::digest(&srs))
            != "fc253016885ec830e97808c9ec920bb5cab5c21af590380a6cb5eb0538e2b244"
    {
        return Err("SRS14 does not match the official ceremony pin".into());
    }
    let mut cursor = Cursor::new(srs);
    let params = ParamsKZG::<Bls12>::read_custom(&mut cursor, SerdeFormat::RawBytesUnchecked)?;
    if cursor.position() != cursor.get_ref().len() as u64 {
        return Err("SRS suffix".into());
    }
    let raw = bounded(paths["--vk"], 65_536)?;
    let mut prefix = &raw[..];
    let _ = ZkStdLibArch::read(&mut prefix)?;
    if !prefix.first().is_some_and(|k| (1..=25).contains(k)) {
        return Err("VK degree out of range".into());
    }
    let mut reader = &raw[..];
    let vk = MidnightVK::read(&mut reader, SerdeFormat::Processed)?;
    if !reader.is_empty() {
        return Err("trailing bytes after VK".into());
    }
    let mut canonical = Vec::new();
    vk.write(&mut canonical, SerdeFormat::Processed)?;
    if canonical != raw {
        return Err("noncanonical VK".into());
    }
    let lanes: Vec<String> = serde_json::from_slice(&bounded(paths["--instance"], 128 * 1024)?)?;
    if lanes.len() != 28 {
        return Err("parent instance must have 28 fields".into());
    }
    let mut pi = Vec::new();
    for lane in lanes {
        let bytes: [u8; 32] = hex::decode(lane)?
            .try_into()
            .map_err(|_| "field must be 32 bytes")?;
        pi.push(Option::<Fq>::from(Fq::from_repr(bytes)).ok_or("noncanonical scalar")?);
    }
    let proof = bounded(paths["--proof"], 1024 * 1024)?;
    let own = batch_verify::<PoseidonState<Fq>>(
        &params.verifier_params(),
        &[vk],
        &[pi.clone()],
        &[proof],
    )
    .is_ok();
    let side = |f: &[Fq]| {
        if f[5] != Fq::ONE {
            return None;
        }
        let point = <AP as Instantiable<Fq>>::from_public_input(&f[..5])?;
        Some(Msm::<S>::new(&[point], &[Fq::ONE], &BTreeMap::new()))
    };
    let carried = match (side(&pi[16..22]), side(&pi[22..28])) {
        (Some(lhs), Some(rhs)) => {
            Accumulator::<S>::new(lhs, rhs).check(&params.verifier_params(), &BTreeMap::new())
        }
        _ => false,
    };
    println!(
        "{}",
        serde_json::json!({"kind":"sherlock-parent-independent-verify",
        "verifies":own && carried,"stdlib_verifies":own,"pairing_verifies":carried,
        "failure":if !own {Some("PLONK proof rejected")} else if !carried {Some("carried pairing rejected")} else {None}})
    );
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    run()
}
