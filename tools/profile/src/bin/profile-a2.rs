//! Explicit-run A2 harness. Building this binary never starts a proof.
use midnight_proofs::{profile_result, profile_span};
use mori_backend::{
    proving::{MAX_K, ProofOptions},
    sherlock_child::{CHILD_TRANSCRIPT, ChildKey},
    sherlock_link::{
        LinkedChildren, ReturnChildKey, check_link_lanes, check_linked_requests,
        produce_linked_requests, verify_linked_children, write_link_dir,
    },
    sherlock_machine::{
        literal_bridge::LiteralSourceIdentity, literal_host::LiteralCapacities,
        literal_witness::core_program_capacities,
    },
};
use mori_core::sherlock::{Expr, K_SOURCE_SHA256, Program, Value, layout_a::prepare_a};
use mori_recursion_profile::{expected_rejection, prove_child, sidecar::Sidecar};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{error::Error, path::PathBuf, sync::Arc};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn run(
    options: &ProofOptions,
    output: &std::path::Path,
    evidence: &mut serde_json::Value,
) -> Result<()> {
    let capacities = LiteralCapacities {
        program: [2, 8, 2, 2, 2, 2, 1, 1, 1, 1, 4, 4, 5, 2, 2],
        state: [1; 6],
        stack_limit: 1,
    };
    let program = Arc::new(Program {
        functions: Vec::new(),
        entry: Arc::new(Expr::Literal(Value::UInt(7))),
    });
    let core_capacities =
        core_program_capacities(&capacities).map_err(|_| "A2 capacities rejected")?;
    let prepared =
        prepare_a(program.clone(), core_capacities).map_err(|_| "A2 fixture preparation failed")?;
    let identity = LiteralSourceIdentity {
        k_source_sha256: K_SOURCE_SHA256.into(),
        reference_source_sha256: sha(include_bytes!(
            "../../../../crates/mori-core/src/sherlock/reference.rs"
        )),
        program_source_sha256: sha(&prepared
            .program_bytes()
            .map_err(|_| "A2 public program encoding failed")?),
    };
    let (requests, shape) = produce_linked_requests(program, &capacities, 2, 1, &identity)?;
    if requests.a102.values.post != requests.a80.values.pre {
        return Err("A102 -> A80 link mismatch".into());
    }
    check_linked_requests(&requests)?;

    // The same sequence as prove_linked_children, with caller boundaries around
    // each key/proof. setup_pk inside key.prove is timed by vendor keygen.
    let a102 = {
        let key = profile_result!(
            KeyLoad,
            Exclusive,
            ChildKey::reconstruct(&requests.a102, options)
        )?;
        prove_child(&key)?
    };
    let a80 = {
        let key = profile_result!(
            KeyLoad,
            Exclusive,
            ReturnChildKey::reconstruct(&requests.a80, options)
        )?;
        profile_result!(Prove, Diagnostic, key.prove())?
    };
    check_link_lanes(&a102.instance, &a80.instance, &shape)?;
    let children = LinkedChildren { a102, a80 };
    if children.a102.claim.circuit_k != 13 || children.a80.claim.circuit_k != 12 {
        return Err("A2 expected A102 k13 and A80 k12".into());
    }
    evidence["proofs"] = json!([
        {"shape":"A102","k":13,"vk_sha256":sha(&children.a102.vk_bytes),
         "srs_sha256":mori_backend::parameters::PINNED_HASHES[13],"transcript":CHILD_TRANSCRIPT,
         "proof_sha256":sha(&children.a102.proof),"proof_bytes":children.a102.proof.len(),
         "public_statement_manifest_sha256":sha(&serde_json::to_vec(&children.a102.instance_hex())?)},
        {"shape":"A80","k":12,"vk_sha256":sha(&children.a80.vk_bytes),
         "srs_sha256":mori_backend::parameters::PINNED_HASHES[12],"transcript":CHILD_TRANSCRIPT,
         "proof_sha256":sha(&children.a80.proof),"proof_bytes":children.a80.proof.len(),
         "public_statement_manifest_sha256":sha(&serde_json::to_vec(&children.a80.instance_hex())?)}
    ]);
    evidence["verification"]["stock_verifier"] = json!("fail");
    let verified = profile_result!(
        Verify,
        Exclusive,
        verify_linked_children(&requests, &children, options)
    )?;
    if verified.shape() != shape {
        return Err("verified link shape mismatch".into());
    }
    evidence["verification"]["stock_verifier"] = json!("pass");

    // Independently reconstructed keys send modified *statements*, as well as
    // modified proof bytes, to stock verification. A host equality check alone
    // would not establish malformed-statement rejection by the verifier.
    evidence["verification"]["negative_controls"] = json!("fail");
    let a102_key = profile_result!(
        KeyLoad,
        Exclusive,
        ChildKey::reconstruct(&requests.a102, options)
    )?;
    let a80_key = profile_result!(
        KeyLoad,
        Exclusive,
        ReturnChildKey::reconstruct(&requests.a80, options)
    )?;
    profile_result!(
        Verify,
        Exclusive,
        a102_key.verify(&children.a102.instance, &children.a102.proof)
    )?;
    profile_result!(
        Verify,
        Exclusive,
        a80_key.verify(&children.a80.instance, &children.a80.proof)
    )?;
    for is_a102 in [true, false] {
        let (instance, proof) = if is_a102 {
            (&children.a102.instance, &children.a102.proof)
        } else {
            (&children.a80.instance, &children.a80.proof)
        };
        let mut bad_proof = proof.clone();
        *bad_proof.first_mut().ok_or("empty child proof")? ^= 1;
        let mut bad_statement = instance.clone();
        let last = bad_statement.last_mut().ok_or("empty child statement")?;
        *last = *last + 1u64.into();
        let rejects = profile_span!(
            Verify,
            Exclusive,
            expected_rejection(|| {
                if is_a102 {
                    a102_key.verify(instance, &bad_proof).is_err()
                        && a102_key.verify(&bad_statement, proof).is_err()
                } else {
                    a80_key.verify(instance, &bad_proof).is_err()
                        && a80_key.verify(&bad_statement, proof).is_err()
                }
            })
        );
        if !rejects {
            evidence["verification"]["negative_controls"] = json!("fail");
            return Err("stock verifier accepted a malformed control".into());
        }
        // Preserve the factory's linked malformed-proof control too.
        let mut bad = children.clone();
        if is_a102 {
            bad.a102.proof = bad_proof;
        } else {
            bad.a80.proof = bad_proof;
        }
        if !profile_span!(
            Verify,
            Exclusive,
            expected_rejection(|| verify_linked_children(&requests, &bad, options).is_err())
        ) {
            return Err("linked verifier accepted malformed proof".into());
        }
    }
    let mut wrong_key = children.clone();
    wrong_key.a80.vk_bytes = children.a102.vk_bytes.clone();
    if !profile_span!(
        Verify,
        Exclusive,
        expected_rejection(|| verify_linked_children(&requests, &wrong_key, options).is_err())
    ) {
        return Err("linked verifier accepted wrong key".into());
    }
    evidence["verification"]["negative_controls"] = json!("pass");
    evidence["verification"]["official_srs_authenticated"] = json!(true);
    let manifest = write_link_dir(output, &requests, &children, &verified)?;
    if manifest.children.len() != 2 {
        return Err("wrong child artifact count".into());
    }
    evidence["public_artifacts"] = json!(output);
    Ok(())
}

fn main() -> Result<()> {
    let mut sidecar = None;
    let mut params_dir = std::env::var_os("MORIARTY_TEST_PARAMS_DIR").map(PathBuf::from);
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let value = args.next().ok_or("expected option value")?;
        match arg.as_str() {
            "--sidecar" => sidecar = Some(PathBuf::from(value)),
            "--params-dir" => params_dir = Some(PathBuf::from(value)),
            "--workload" if value == "A2" => {},
            "--cache-mode" if value == "process_cold" => {},
            "--cache-mode" => return Err("A2 resident_pk is unsupported: public child APIs rebuild the PK".into()),
            _ => return Err("usage: profile-a2 --sidecar PATH [--params-dir PATH] [--workload A2] [--cache-mode process_cold]".into()),
        }
    }
    let path = sidecar.ok_or("--sidecar is required")?;
    let params_dir = params_dir
        .or_else(|| {
            std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".cache/midnight/zk-params"))
        })
        .ok_or("explicit pinned local SRS directory required")?;
    let options = ProofOptions {
        params_dir,
        max_k: MAX_K,
    };
    let output = path.with_extension("children");
    let mut evidence = json!({"id":"A2","cache_mode":"process_cold","ordered_child_ids":["A102","A80"],
        "verification":{"stock_verifier":"not_run","negative_controls":"not_run","official_srs_authenticated":false},
        "proofs":[],"binary_sha256":sha(&std::fs::read(std::env::current_exe()?)?),
        "cargo_lock_sha256":sha(include_bytes!("../../Cargo.lock")),
        "patch_sha256":sha(include_bytes!("../../patches/midnight-proofs-0.8.3-profile.patch"))});
    let writer = Sidecar::start(&path)?;
    let result = profile_result!(Command, Diagnostic, run(&options, &output, &mut evidence));
    let written = writer.finish(evidence, result.is_ok());
    if let Err(error) = &written {
        eprintln!("profile-a2: sidecar write failed: {error}");
    }
    result?;
    written?;
    println!(
        "A2 stock verification and malformed-proof/statement controls passed; sidecar={}",
        path.display()
    );
    Ok(())
}
