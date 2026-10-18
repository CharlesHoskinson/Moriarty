//! Fresh ephemeral secp256k1 test-key producer (SOURCE ONLY, unexecuted).
//!
//! One invocation produces exactly ONE scheme's artifacts:
//!   moriarty-vi-signer schnorr_bip340
//!   moriarty-vi-signer ecdsa_secp256k1_sha256
//! It creates one fresh key INSIDE this process from exactly one 32-byte
//! read of the authorized entropy device, fills ONLY the declared
//! `public_key_hex` template field of the embedded invoice-partial test
//! template, asks mori-core to prepare the canonical owner intent, signs
//! the exact Core `signing_message` bytes with the high-level k256
//! `Signer` (which applies the single outer SHA-256 that Core's
//! `verify_intent` and the Aiken `SigCheckV2.digest = sha2_256(message)`
//! both expect; no double hash, no guessed auxiliary randomness: the
//! Schnorr `Signer` path uses zero auxiliary bytes per k256 schnorr
//! signing.rs `Signer::try_sign`, and ECDSA uses deterministic RFC6979),
//! then reports the honest `verify_intent` outcome. The secret scalar is
//! never stored, written, printed or formatted; errors are static strings.
//! This helper is NOT a wallet, CLI product surface or custody interface:
//! the key dies with the process and must never be reused as a live
//! catalogue key. No existing wallet, intent key, proving key or private
//! file is read on any path.

use std::io::Read;
use std::process::exit;

use k256::schnorr::signature::Signer as SchnorrSigner;
use k256::ecdsa::signature::Signer as EcdsaSigner;
use mori_core::intent::{
    prepare_intent, signing_message, verify_intent, AssetLimit, Authorization, PreparedIntent,
    Scheme, SignedIntent,
};
use mori_core::{Action, AssetDecl, AssetId, Balance, InputDecl, Liability, Program, Scenario, StateDecl, Type, UInt, Value};
use serde::Serialize;
use std::collections::BTreeMap;

/// The exact authorized entropy device. Phase A AUTH must name this same
/// device; the operator refuses to run the helper under any other record.
const ENTROPY_DEVICE: &str = "/dev/urandom";
/// SHA-256 of fixtures/installment-invoice.mori in INPUT-INDEX.json.
const SOURCE_HASH: &str = "105137af8ac6a1f060ec7b634e3ec4aa913808e94bd1b7cffb736f8105d2cfc9";
const SIGNER_NAME: &str = "escrow";
const SEQ: u128 = 1;
const NOT_BEFORE: u128 = 0;
const NOT_AFTER: u128 = 2_000_000_000_000;
const VERIFY_AT: u128 = 1_700_000_030_000;
/// Declared test-template public-key field. This placeholder is the ONLY
/// template field the fresh helper rewrites, and it is rewritten before
/// canonicalization, so the derived key propagates into the signed intent
/// and, through the public producer output, into the adapter catalogue.
const TEMPLATE_PUBLIC_KEY_HEX_PLACEHOLDER: &str =
    "TEMPLATE_PUBLIC_KEY_HEX_REWRITTEN_WITH_FRESH_KEY_BEFORE_CANONICALIZATION";

fn fail() -> ! {
    // Static sanitized error only: no secret, key, path or Debug output.
    eprintln!("signer_error");
    exit(3)
}

fn cash() -> AssetId {
    AssetId { network: "oracle-unit".into(), issuer: "invoice-issuer".into(), reference: "cash".into(), scale: 0 }
}

fn program() -> Program {
    // Typed Core program for the invoice-partial fixture. This is a signing
    // document input to prepare_intent, not a new program IR and not a
    // compiler: the action name, asset and state names are the current
    // language fixture's own (installment-invoice.mori partial action).
    Program {
        language: "moriarty/1".into(),
        semantics: "moriarty-core/1".into(),
        name: "InstallmentInvoice".into(),
        states: vec![
            StateDecl { name: "opened".into(), ty: Type::Bool, initial: Value::Bool(false) },
            StateDecl { name: "paid".into(), ty: Type::UInt, initial: Value::UInt(UInt(0)) },
            StateDecl { name: "marker".into(), ty: Type::UInt, initial: Value::UInt(UInt(7)) },
        ],
        assets: vec![AssetDecl { name: "CASH".into(), identity: cash() }],
        expressions: Vec::new(),
        actions: vec![Action {
            name: "partial".into(),
            inputs: vec![InputDecl { name: "amount".into(), ty: Type::UInt }],
            statements: Vec::new(),
        }],
    }
}

fn scenario() -> Scenario {
    // Exact scenario of fixtures/invoice-partial.request.json. Balance and
    // liability array order is the adapter thread-datum order, because the
    // Aiken scenario_json emits datum rows in datum order while Core
    // canonicalizes this array in the order supplied here.
    let mut state = BTreeMap::new();
    state.insert("marker".to_string(), Value::UInt(UInt(7)));
    state.insert("opened".to_string(), Value::Bool(true));
    state.insert("paid".to_string(), Value::UInt(UInt(0)));
    let mut inputs = BTreeMap::new();
    inputs.insert("amount".to_string(), Value::UInt(UInt(35)));
    let bal = |account: &str, amount: u128| Balance { account: account.into(), asset: cash(), amount: UInt(amount) };
    Scenario {
        state,
        inputs,
        balances: vec![bal("escrow", 109), bal("spectator", 41), bal("fees", 0), bal("seller", 0)],
        liabilities: vec![
            Liability { name: "invoice".into(), asset: cash(), debtor: "escrow".into(), creditor: "seller".into(), amount: UInt(90) },
            Liability { name: "reserve".into(), asset: cash(), debtor: "escrow".into(), creditor: "buyer".into(), amount: UInt(14) },
        ],
    }
}

fn authority(scheme: Scheme, public_key_hex: String) -> Authorization {
    // Domain facts are the domain/2 mapping of intent.ak intent_shared:
    // anchor cardano-plutus-v3:<settle hash hex>, program_address
    // cardano-thread:<thread id hex>, target predicate:vk:shape (194
    // bytes), predecessor txid#index. Settle hash is the S2 thread-policy
    // hash e315b315... from chain.json (the covered input's own script
    // hash), never the VI hash and never a fake #b1 credential.
    Authorization {
        network: "oracle-unit".into(),
        anchor: "cardano-plutus-v3:e315b315819a3c7695eb2d33fb468d2186b7be4cfba485c34850f255".into(),
        program_address: "cardano-thread:7a7fafdd81303746cd8af0fc48add098ab6468e5c510042d41be69f33abcf29a".into(),
        target_hash: "0d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd:d19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd:0000000000000000000000000000000000000000000000000000000000000000".into(),
        signer: SIGNER_NAME.into(),
        public_key_hex,
        scheme,
        nonce: UInt(SEQ),
        predecessor: "2222222222222222222222222222222222222222222222222222222222222222#0".into(),
        not_before: UInt(NOT_BEFORE),
        not_after: UInt(NOT_AFTER),
        limits: vec![AssetLimit {
            asset: cash(),
            beneficiary: "escrow".into(),
            gross_cap: UInt(35),
            fee_cap: UInt(0),
            net_floor: UInt(0),
            liability_cap: UInt(104),
        }],
    }
}

#[derive(Serialize)]
struct ProducerOutput {
    format: String,
    scheme: String,
    public_key_hex: String,
    prepared: PreparedIntent,
    signed_intent: SignedIntent,
    signing_message_hex: String,
    core_verify: serde_json::Value,
}

fn main() {
    std::panic::set_hook(Box::new(|_| {})); // no backtrace or payload text
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        fail();
    }
    // Exactly one 32-byte entropy draw. No retry: an invalid scalar stops.
    let mut entropy = [0u8; 32];
    match std::fs::File::open(ENTROPY_DEVICE) {
        Ok(mut f) => {
            if f.read_exact(&mut entropy).is_err() {
                fail();
            }
        }
        Err(_) => fail(),
    }
    let scheme_token = args[1].as_str();
    // The placeholder documents the single rewritten field; its value is
    // never canonicalized, signed or emitted.
    let _ = TEMPLATE_PUBLIC_KEY_HEX_PLACEHOLDER;
    let (scheme, public_key_hex, signature_hex) = match scheme_token {
        "schnorr_bip340" => {
            let signing_key = match k256::schnorr::SigningKey::from_bytes(&entropy) {
                Ok(k) => k,
                Err(_) => fail(),
            };
            let public_key_hex = mori_core::intent::hex(signing_key.verifying_key().to_bytes().as_slice());
            let auth = authority(Scheme::SchnorrBip340, public_key_hex.clone());
            let prepared = prepared_or_fail(&auth);
            let message = message_or_fail(&prepared);
            // High-level Signer on the exact Core message: one outer
            // SHA-256 inside k256, zero auxiliary bytes. Signing the
            // digest with Signer would double-hash; sign_prehash on the
            // full message would skip Core's outer hash. Both refused.
            let signature: k256::schnorr::Signature = match SchnorrSigner::try_sign(&signing_key, &message) {
                Ok(s) => s,
                Err(_) => fail(),
            };
            (Scheme::SchnorrBip340, public_key_hex, mori_core::intent::hex(&signature.to_bytes()))
        }
        "ecdsa_secp256k1_sha256" => {
            let signing_key = match k256::ecdsa::SigningKey::from_slice(&entropy) {
                Ok(k) => k,
                Err(_) => fail(),
            };
            let public_key_hex = mori_core::intent::hex(signing_key.verifying_key().to_sec1_bytes().as_ref());
            let auth = authority(Scheme::EcdsaSecp256k1Sha256, public_key_hex.clone());
            let prepared = prepared_or_fail(&auth);
            let message = message_or_fail(&prepared);
            // High-level Signer: SHA-256(msg) then deterministic RFC6979,
            // low-S normalized by the k256 SignPrimitive, exactly the
            // bytes Core verify_intent accepts (it rejects high-S).
            let signature: k256::ecdsa::Signature = match EcdsaSigner::try_sign(&signing_key, &message) {
                Ok(s) => s,
                Err(_) => fail(),
            };
            use k256::ecdsa::signature::SignatureEncoding as _;
            (Scheme::EcdsaSecp256k1Sha256, public_key_hex, mori_core::intent::hex(&signature.to_bytes()))
        }
        _ => fail(),
    };
    entropy.iter_mut().for_each(|b| *b = 0); // best effort only; no erasure theorem claimed
    let auth = authority(scheme, public_key_hex.clone());
    let prepared = prepared_or_fail(&auth);
    let signed_intent = SignedIntent { intent: prepared.intent.clone(), signature_hex };
    // Honest Core verification: the result is reported as observed, never
    // replaced by an unconditional true. A rejection stops with no output.
    let core_verify = match verify_intent(&prepared.intent, &signed_intent, UInt(VERIFY_AT)) {
        Ok(()) => serde_json::json!({"status": "ok", "at": VERIFY_AT.to_string()}),
        Err(e) => {
            eprintln!("signer_error");
            let _ = e;
            exit(3);
        }
    };
    let message_hex = prepared.signing_message_hex.clone();
    let out = ProducerOutput {
        format: "moriarty-vi-signer-producer-output/1".into(),
        scheme: scheme_token.into(),
        public_key_hex,
        prepared,
        signed_intent,
        signing_message_hex: message_hex,
        core_verify,
    };
    match serde_json::to_string(&out) {
        Ok(text) => println!("{text}"),
        Err(_) => fail(),
    }
}

fn prepared_or_fail(auth: &Authorization) -> PreparedIntent {
    match prepare_intent(&program(), SOURCE_HASH, "partial", &scenario(), auth) {
        Ok(p) => p,
        Err(_) => fail(),
    }
}

fn message_or_fail(prepared: &PreparedIntent) -> Vec<u8> {
    match signing_message(&prepared.intent) {
        Ok(m) => m,
        Err(_) => fail(),
    }
}
