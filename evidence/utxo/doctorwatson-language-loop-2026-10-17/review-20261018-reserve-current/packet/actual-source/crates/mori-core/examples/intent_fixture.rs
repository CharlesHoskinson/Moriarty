//! Public deterministic QA key only. Never use this fixture for wallet authority.
use k256::schnorr::{SigningKey, signature::Signer};
use mori_core::{Scenario, UInt, intent::*};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 3 {
        return Err("usage: intent_fixture SOURCE SCENARIO ACTION".into());
    }
    let source = std::fs::read_to_string(&args[0])?;
    let program = mori_core::parse(&source)?;
    let scenario: Scenario =
        serde_json::from_value(mori_core::json::parse(&std::fs::read_to_string(&args[1])?)?)?;
    let key = SigningKey::from_bytes(&[3; 32])?;
    let authority = Authorization {
        network: "local".into(),
        anchor: "local-unqualified".into(),
        program_address: "qa-counter".into(),
        target_hash: "qa-ledger-9-profile".into(),
        signer: "alice".into(),
        public_key_hex: hex(&key.verifying_key().to_bytes()),
        scheme: Scheme::SchnorrBip340,
        nonce: UInt(7),
        predecessor: "qa-head-1".into(),
        not_before: UInt(10),
        not_after: UInt(20),
        limits: vec![],
    };
    let prepared = prepare_intent(
        &program,
        &hash_bytes(source.as_bytes()),
        &args[2],
        &scenario,
        &authority,
    )?;
    let sig: k256::schnorr::Signature = key.sign(&signing_message(&prepared.intent)?);
    let signed = SignedIntent {
        intent: prepared.intent,
        signature_hex: hex(sig.to_bytes().as_ref()),
    };
    println!(
        "{}",
        serde_json::to_string_pretty(
            &serde_json::json!({"authority":authority,"signed":signed,"qualification":"public-test-key-only"})
        )?
    );
    Ok(())
}
