//! Fresh ephemeral secp256k1 test-key producer (SOURCE ONLY, unexecuted).
//!
//! One invocation produces ONE scheme's genuine owner-intent signatures:
//!   moriarty-vi-signer schnorr_bip340          <source.mori> <request.json>
//!   moriarty-vi-signer ecdsa_secp256k1_sha256  <source.mori> <request.json>
//!
//! The invoice-partial authorization facts require TWO signers on-chain:
//! thread.required_signers demands the `from` account of every Transfer
//! and Fee (escrow) and the creditor of every discharge (seller, for the
//! invoice duty). This helper therefore creates TWO fresh ephemeral keys
//! inside this one process, in the fixed order escrow then seller, each
//! from exactly one 32-byte read of the authorized entropy device, with
//! NO retry and NO redraw: an invalid scalar stops the whole invocation.
//! The scheme token is validated BEFORE any entropy is opened, so a
//! malformed argument consumes no key attempt. The program under
//! signature is obtained by reading the exact pinned public .mori source
//! bytes and calling the actual current Core parser (mori_core::parse,
//! which also validates the moriarty/1 profile); NO Program, Action,
//! expression or statement is constructed by hand, and no duplicated
//! transcription of the .mori source is embedded in this file. The
//! scenario is decoded from the exact pinned public request JSON with
//! the Core strict JSON parser and is guarded against the frozen
//! invoice-partial identities before any key exists. Each signer signs
//! the exact Core signing_message bytes of its own prepared intent with
//! the high-level k256 Signer (one outer SHA-256, matching Core
//! verify_intent and the Aiken SigCheckV2 digest = sha2_256(message);
//! no double hash, no guessed auxiliary randomness: Schnorr uses zero
//! auxiliary bytes, ECDSA uses deterministic RFC6979). Both verifier
//! outcomes are computed honestly with verify_intent and reported;
//! a rejection stops with no producer output. Secret scalars are never
//! stored, written, printed or formatted; errors are one static string.
//! Entropy zeroization is best effort only and is NOT an erasure
//! theorem. This helper is not a wallet, CLI product surface or custody
//! interface: both keys die with the process, must never be reused as
//! live catalogue keys, and no existing wallet, intent key, proving
//! key, SRS or private file is read on any path.

use std::io::Read;
use std::process::exit;

use k256::ecdsa::signature::Signer as EcdsaSigner;
use k256::schnorr::signature::Signer as SchnorrSigner;
use mori_core::intent::{
    hash_bytes, prepare_intent, signing_message, verify_intent, AssetLimit, Authorization,
    PreparedIntent, Scheme, SignedIntent,
};
use mori_core::{Scenario, UInt, Value};
use serde::Serialize;

/// The exact authorized entropy device. Phase A AUTH must name this
/// same device; the helper opens it and nothing else for entropy.
const ENTROPY_DEVICE: &str = "/dev/urandom";
/// SHA-256 of the frozen fixtures/installment-invoice.mori source bytes
/// (INPUT-INDEX.json pin). The helper refuses any other source bytes.
const SOURCE_HASH: &str = "105137af8ac6a1f060ec7b634e3ec4aa913808e94bd1b7cffb736f8105d2cfc9";
/// Gold K4H program hash of the parsed invoice program. The adapter
/// independently asserts the same value; a mismatch stops here too,
/// because a different parsed program is a different signing document.
const PROGRAM_HASH: &str = "d3b3c02933020e7da2623d5d83acf83c47c72ceb3953d155141af9527cb900d9";
/// domain/2 target_hash digests (predicate, verifying key, ABI v2 shape)
/// of the frozen invoice/partial action entry (K4H-GOLD).
const TARGET_HASH: &str = "0d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd:d19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd:36310727c784c8a8b1204a9cb2de5eb60193afd5517179c16ebd1ae3a92e2969";
const NETWORK: &str = "oracle-unit";
/// Anchor: cardano-plutus-v3:<settle (thread policy S2P) script hash>
/// from the B9 chain receipts. Never the VI hash, never a fake credential.
const ANCHOR: &str = "cardano-plutus-v3:e315b315819a3c7695eb2d33fb468d2186b7be4cfba485c34850f255";
/// Program address: cardano-thread:<frozen thread id>.
const PROGRAM_ADDRESS: &str =
    "cardano-thread:7a7fafdd81303746cd8af0fc48add098ab6468e5c510042d41be69f33abcf29a";
/// Fabricated local predecessor outref (own input), not a ledger fact.
const PREDECESSOR: &str = "2222222222222222222222222222222222222222222222222222222222222222#0";
const SEQ: u128 = 1;
const NOT_BEFORE: u128 = 0;
const NOT_AFTER: u128 = 2_000_000_000_000;
const VERIFY_AT: u128 = 1_700_000_030_000;
const MAX_SOURCE_BYTES: usize = 1_048_576;
const MAX_REQUEST_BYTES: usize = 1_048_576;

fn fail() -> ! {
    // Static sanitized error only: no secret, key, path or Debug output.
    eprintln!("signer_error");
    exit(3)
}

fn read_capped(path: &str, cap: usize) -> Vec<u8> {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(_) => fail(),
    };
    if bytes.is_empty() || bytes.len() > cap {
        fail();
    }
    bytes
}

fn draw_entropy() -> [u8; 32] {
    // Exactly one 32-byte read per key. No retry, no redraw, ever.
    let mut buf = [0u8; 32];
    match std::fs::File::open(ENTROPY_DEVICE) {
        Ok(mut f) => {
            if f.read_exact(&mut buf).is_err() {
                fail();
            }
        }
        Err(_) => fail(),
    }
    buf
}

#[derive(Serialize)]
struct IntentOutput {
    signer: String,
    public_key_hex: String,
    prepared: PreparedIntent,
    signed_intent: SignedIntent,
    signing_message_hex: String,
    core_verify: serde_json::Value,
}

#[derive(Serialize)]
struct ProducerOutput {
    format: String,
    scheme: String,
    source_hash: String,
    program_hash: String,
    intents: Vec<IntentOutput>,
}

fn main() {
    std::panic::set_hook(Box::new(|_| {})); // no backtrace or payload text
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        fail();
    }
    // (1) Scheme validity is checked BEFORE any file or entropy access,
    // so a malformed argument consumes no key attempt.
    let scheme = match args[1].as_str() {
        "schnorr_bip340" => Scheme::SchnorrBip340,
        "ecdsa_secp256k1_sha256" => Scheme::EcdsaSecp256k1Sha256,
        _ => fail(),
    };
    let scheme_token = args[1].clone();

    // (2) Exact frozen .mori source bytes, hashed and parsed by Core.
    let source_bytes = read_capped(&args[2], MAX_SOURCE_BYTES);
    if hash_bytes(&source_bytes) != SOURCE_HASH {
        fail();
    }
    let source_text = match String::from_utf8(source_bytes) {
        Ok(t) => t,
        Err(_) => fail(),
    };
    // Actual current Core parsing: mori_core::parse lexes, parses AND
    // validates the moriarty/1 profile. No hand-written program IR.
    let program = match mori_core::parse(&source_text) {
        Ok(p) => p,
        Err(_) => fail(),
    };
    if program.name != "InstallmentInvoice"
        || program.language != "moriarty/1"
        || program.semantics != "moriarty-core/1"
        || !program.actions.iter().any(|a| a.name == "partial")
    {
        fail();
    }
    let cash = match program.assets.iter().find(|a| a.name == "CASH") {
        Some(a) => a.identity.clone(),
        None => fail(),
    };
    if cash.network != NETWORK || cash.issuer != "invoice-issuer" || cash.reference != "cash" || cash.scale != 0 {
        fail();
    }

    // (3) Exact frozen request; scenario decoded strictly, preserving
    // the request's balance/liability array order and its explicit
    // zero balances (fees 0, seller 0 are present rows, not absences).
    let request_bytes = read_capped(&args[3], MAX_REQUEST_BYTES);
    let request_text = match String::from_utf8(request_bytes) {
        Ok(t) => t,
        Err(_) => fail(),
    };
    // Core strict JSON: duplicate keys, trailing data and non-finite
    // numbers are rejected before any schema decoding.
    let request = match mori_core::json::parse(&request_text) {
        Ok(v) => v,
        Err(_) => fail(),
    };
    let sel = match request.get("selector") {
        Some(v) => v,
        None => fail(),
    };
    if request.get("format").and_then(|v| v.as_str()) != Some("moriarty-k2-prove-request/1")
        || sel.get("action").and_then(|v| v.as_str()) != Some("partial")
        || sel.get("language").and_then(|v| v.as_str()) != Some("moriarty/1")
        || sel.get("semantics").and_then(|v| v.as_str()) != Some("moriarty-core/1")
        || request
            .get("context")
            .and_then(|c| c.get("seq"))
            .and_then(|v| v.as_u64())
            != Some(1)
    {
        fail();
    }
    let scenario_value = match request
        .get("transition")
        .and_then(|t| t.get("scenario"))
    {
        Some(v) => v.clone(),
        None => fail(),
    };
    let scenario: Scenario = match serde_json::from_value(scenario_value) {
        Ok(s) => s,
        Err(_) => fail(),
    };
    // Frozen scenario identities: array order and present-vs-absent
    // distinctions are signing facts, so drift stops before keys exist.
    let balance_facts: Vec<(&str, u128)> = scenario
        .balances
        .iter()
        .map(|b| (b.account.as_str(), b.amount.0))
        .collect();
    if balance_facts != [("escrow", 109), ("spectator", 41), ("fees", 0), ("seller", 0)]
        || scenario.balances.iter().any(|b| b.asset != cash)
    {
        fail();
    }
    let liability_facts: Vec<(&str, &str, &str, u128)> = scenario
        .liabilities
        .iter()
        .map(|l| (l.name.as_str(), l.debtor.as_str(), l.creditor.as_str(), l.amount.0))
        .collect();
    if liability_facts != [("invoice", "escrow", "seller", 90), ("reserve", "escrow", "buyer", 14)]
        || scenario.liabilities.iter().any(|l| l.asset != cash)
    {
        fail();
    }
    if scenario.inputs.get("amount") != Some(&Value::UInt(UInt(35)))
        || scenario.state.get("marker") != Some(&Value::UInt(UInt(7)))
        || scenario.state.get("opened") != Some(&Value::Bool(true))
        || scenario.state.get("paid") != Some(&Value::UInt(UInt(0)))
    {
        fail();
    }

    // (4) Fresh ephemeral keys, in the fixed signer order escrow, seller.
    // Each key is exactly one 32-byte draw; an invalid scalar fails the
    // whole invocation with no redraw and no signing attempt.
    let mut entropy_escrow = draw_entropy();
    let mut entropy_seller = draw_entropy();
    let (escrow_key_hex, escrow_sign): (String, Box<dyn Fn(&[u8]) -> Option<String>>) =
        match scheme {
            Scheme::SchnorrBip340 => {
                let escrow_key = match k256::schnorr::SigningKey::from_bytes(&entropy_escrow) {
                    Ok(k) => k,
                    Err(_) => fail(),
                };
                let seller_key = match k256::schnorr::SigningKey::from_bytes(&entropy_seller) {
                    Ok(k) => k,
                    Err(_) => fail(),
                };
                let escrow_hex =
                    mori_core::intent::hex(escrow_key.verifying_key().to_bytes().as_slice());
                // Keys must stay alive for both signatures; move them into
                // the closure pair by re-deriving inside the sign step.
                let _ = (escrow_key, seller_key);
                (escrow_hex, Box::new(|_| None))
            }
            Scheme::EcdsaSecp256k1Sha256 => {
                let escrow_key = match k256::ecdsa::SigningKey::from_slice(&entropy_escrow) {
                    Ok(k) => k,
                    Err(_) => fail(),
                };
                let escrow_hex = mori_core::intent::hex(
                    escrow_key.verifying_key().to_sec1_bytes().as_ref(),
                );
                let _ = escrow_key;
                (escrow_hex, Box::new(|_| None))
            }
        };
    // The placeholder closure above is never used: signing is performed
    // by the scheme blocks below, which re-derive the two keys from the
    // two entropy buffers exactly once (deterministic from those 32-byte
    // draws). This comment, not code, documents why buffers stay alive
    // until both signatures exist.
    let _ = escrow_sign;
    let _ = escrow_key_hex;

    let auth_for = |signer: &str, public_key_hex: String, limits: Vec<AssetLimit>| Authorization {
        network: NETWORK.into(),
        anchor: ANCHOR.into(),
        program_address: PROGRAM_ADDRESS.into(),
        target_hash: TARGET_HASH.into(),
        signer: signer.into(),
        public_key_hex,
        scheme,
        nonce: UInt(SEQ),
        predecessor: PREDECESSOR.into(),
        not_before: UInt(NOT_BEFORE),
        not_after: UInt(NOT_AFTER),
        limits,
    };
    // Gold-true limits for the frozen partial relation: the escrow signer
    // authorizes gross debit 37 (fee 2 plus transfer 35) and fees 2
    // toward beneficiary seller, with closing liabilities invoice 55
    // plus reserve 14 = 69. The seller, as discharge creditor of the
    // invoice duty, authorizes no debit of its own.
    let escrow_limits = vec![AssetLimit {
        asset: cash.clone(),
        beneficiary: "seller".into(),
        gross_cap: UInt(37),
        fee_cap: UInt(2),
        net_floor: UInt(0),
        liability_cap: UInt(69),
    }];
    let seller_limits = vec![AssetLimit {
        asset: cash.clone(),
        beneficiary: "seller".into(),
        gross_cap: UInt(0),
        fee_cap: UInt(0),
        net_floor: UInt(0),
        liability_cap: UInt(0),
    }];

    let mut intents: Vec<IntentOutput> = Vec::new();
    match scheme {
        Scheme::SchnorrBip340 => {
            let keys = [
                match k256::schnorr::SigningKey::from_bytes(&entropy_escrow) {
                    Ok(k) => k,
                    Err(_) => fail(),
                },
                match k256::schnorr::SigningKey::from_bytes(&entropy_seller) {
                    Ok(k) => k,
                    Err(_) => fail(),
                },
            ];
            let signers = ["escrow", "seller"];
            let limits = [escrow_limits, seller_limits];
            for (i, key) in keys.iter().enumerate() {
                let public_key_hex =
                    mori_core::intent::hex(key.verifying_key().to_bytes().as_slice());
                let auth = auth_for(signers[i], public_key_hex.clone(), limits[i].clone());
                let prepared = match prepare_intent(&program, SOURCE_HASH, "partial", &scenario, &auth) {
                    Ok(p) => p,
                    Err(_) => fail(),
                };
                if prepared.intent.program_hash != PROGRAM_HASH {
                    fail();
                }
                let message = match signing_message(&prepared.intent) {
                    Ok(m) => m,
                    Err(_) => fail(),
                };
                // High-level Signer on the exact Core message: exactly one
                // outer SHA-256 inside k256, zero auxiliary bytes.
                let signature: k256::schnorr::Signature = match SchnorrSigner::try_sign(key, &message) {
                    Ok(s) => s,
                    Err(_) => fail(),
                };
                let signed_intent = SignedIntent {
                    intent: prepared.intent.clone(),
                    signature_hex: mori_core::intent::hex(&signature.to_bytes()),
                };
                let core_verify = match verify_intent(&prepared.intent, &signed_intent, UInt(VERIFY_AT)) {
                    Ok(()) => serde_json::json!({"status": "ok", "at": VERIFY_AT.to_string()}),
                    Err(_) => fail(),
                };
                intents.push(IntentOutput {
                    signer: signers[i].into(),
                    public_key_hex,
                    signing_message_hex: prepared.signing_message_hex.clone(),
                    prepared,
                    signed_intent,
                    core_verify,
                });
            }
        }
        Scheme::EcdsaSecp256k1Sha256 => {
            let keys = [
                match k256::ecdsa::SigningKey::from_slice(&entropy_escrow) {
                    Ok(k) => k,
                    Err(_) => fail(),
                },
                match k256::ecdsa::SigningKey::from_slice(&entropy_seller) {
                    Ok(k) => k,
                    Err(_) => fail(),
                },
            ];
            let signers = ["escrow", "seller"];
            let limits = [escrow_limits, seller_limits];
            for (i, key) in keys.iter().enumerate() {
                let public_key_hex =
                    mori_core::intent::hex(key.verifying_key().to_sec1_bytes().as_ref());
                let auth = auth_for(signers[i], public_key_hex.clone(), limits[i].clone());
                let prepared = match prepare_intent(&program, SOURCE_HASH, "partial", &scenario, &auth) {
                    Ok(p) => p,
                    Err(_) => fail(),
                };
                if prepared.intent.program_hash != PROGRAM_HASH {
                    fail();
                }
                let message = match signing_message(&prepared.intent) {
                    Ok(m) => m,
                    Err(_) => fail(),
                };
                // High-level Signer: SHA-256(message), then deterministic
                // RFC6979, low-S normalized by k256; exactly the bytes
                // Core verify_intent accepts (it rejects high-S).
                use k256::ecdsa::signature::SignatureEncoding as _;
                let signature: k256::ecdsa::Signature = match EcdsaSigner::try_sign(key, &message) {
                    Ok(s) => s,
                    Err(_) => fail(),
                };
                let signed_intent = SignedIntent {
                    intent: prepared.intent.clone(),
                    signature_hex: mori_core::intent::hex(&signature.to_bytes()),
                };
                let core_verify = match verify_intent(&prepared.intent, &signed_intent, UInt(VERIFY_AT)) {
                    Ok(()) => serde_json::json!({"status": "ok", "at": VERIFY_AT.to_string()}),
                    Err(_) => fail(),
                };
                intents.push(IntentOutput {
                    signer: signers[i].into(),
                    public_key_hex,
                    signing_message_hex: prepared.signing_message_hex.clone(),
                    prepared,
                    signed_intent,
                    core_verify,
                });
            }
        }
    }
    // Best-effort erasure of the draw buffers only; signing-key internals
    // are dropped with the process. No erasure theorem is claimed.
    entropy_escrow.iter_mut().for_each(|b| *b = 0);
    entropy_seller.iter_mut().for_each(|b| *b = 0);
    if intents.len() != 2 || intents[0].signer != "escrow" || intents[1].signer != "seller" {
        fail();
    }
    let out = ProducerOutput {
        format: "moriarty-vi-signer-producer-output/2".into(),
        scheme: scheme_token,
        source_hash: SOURCE_HASH.into(),
        program_hash: PROGRAM_HASH.into(),
        intents,
    };
    match serde_json::to_string(&out) {
        Ok(text) => println!("{text}"),
        Err(_) => fail(),
    }
}
