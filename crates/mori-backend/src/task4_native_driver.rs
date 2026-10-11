//! Parameter-free recipes and ignored Task4 native entry. No production API.
//! All scalars below are PUBLIC laboratory constants, never wallet material.
use super::*;
use midnight_curves::{Fr as Scalar, JubjubSubgroup, pairing::group::Group};
use midnight_transient_crypto::hash::transient_hash;
use midnight_zkir_v3::{
    Instruction as I,
    ir::{Identifier, Operand},
    ir_types::{IrType, IrValue},
    ir_vm::Preprocessed,
};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    sync::{Arc, Mutex},
};

const FIXTURES: &str = "/home/charl/Moriarty/.moriarty-dev/runtime/native-liabilities-20261011/task-3-signed-sol/measurements-final";
const RUNTIME: &str =
    "/home/charl/Moriarty/.moriarty-dev/runtime/native-liabilities-20261011/task-4-signed-native";
const IR_SHA: &str = "b009f47c019f756f8d73c1f643433058c79715301249d1d396f42581b7a51eff";
const SLOTS: [&str; 7] = [
    "params_load",
    "params_get",
    "keygen",
    "keygen_vk",
    "prove",
    "prove_unchecked",
    "verify",
];
const CASES: [&str; 10] = [
    "positive",
    "raw-honest",
    "response-plus-one",
    "wrong-private-key",
    "tight-gross",
    "tight-fee",
    "tight-net",
    "tight-duty",
    "alternative",
    "reused-proof",
];
const AS_BYTES: u64 = 2 * 1024 * 1024 * 1024;

fn reserve(counts: &mut [u32; 7], limits: &[u32; 7], slot: usize) -> Result<(), String> {
    if counts[slot] >= limits[slot] {
        return Err(format!("unplanned {} call", SLOTS[slot]));
    }
    counts[slot] += 1;
    Ok(())
}
fn limits(case: &str) -> [u32; 7] {
    match case {
        "positive" => [3, 4, 1, 2, 1, 0, 3],
        "alternative" => [1, 1, 0, 1, 0, 0, 1],
        "reused-proof" => [1, 1, 0, 1, 0, 0, 6],
        c if CASES[1..8].contains(&c) => [1, 2, 1, 0, 0, 1, 1],
        _ => panic!("unplanned case"),
    }
}
struct Budget {
    case: String,
    counts: [u32; 7],
    limits: [u32; 7],
    events: std::fs::File,
}
static BUDGET: Mutex<Option<Budget>> = Mutex::new(None);
/// Every hook charges immediately before the operation; inactive ordinary tests
/// never touch a parameter or result file through this hook.
pub(crate) fn charge(name: &str) {
    let mut guard = BUDGET.lock().unwrap();
    if let Some(b) = guard.as_mut() {
        let slot = SLOTS.iter().position(|s| *s == name).expect("known call");
        if let Err(reason) = reserve(&mut b.counts, &b.limits, slot) {
            writeln!(
                b.events,
                "{}",
                serde_json::json!({"case":b.case,"refused_before_call":name,"counts":b.counts})
            )
            .unwrap();
            b.events.flush().unwrap();
            drop(guard);
            panic!("{reason}");
        }
        let event = serde_json::json!({"case":b.case,"before_call":name,"counts":b.counts});
        writeln!(b.events, "{event}").unwrap();
        b.events.flush().unwrap();
        println!("TASK4_EVENT {event}");
    }
}
fn sha(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}
fn write_new(path: &Path, bytes: &[u8]) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap();
    file.write_all(bytes).unwrap();
}
fn json_bytes(value: &impl Serialize) -> Vec<u8> {
    serde_json::to_vec_pretty(value).unwrap()
}
struct Fixture {
    source: String,
    scenario: Scenario,
    target: Target,
    authority: ExpectedAuthority,
    signed: SignedIntent,
    input_hashes: serde_json::Value,
}
fn fixture() -> Fixture {
    let names = [
        "mori",
        "scenario.json",
        "target.json",
        "authority.json",
        "payload.json",
        "signed.json",
        "prepared.json",
        "artifact.json",
        "layout.json",
        "zkir",
    ];
    let hashes = [
        "a446c4dc826c1706354dea6b1125cfb30019e0f16a90085ead9be9b1c8d5eb2d",
        "aa5f0a545194dbc85fcb7bc7fdc9de352d03eaa36c5463566303941814d614a7",
        "16001ddce422915abb7d049136d3a2001b9bbbc90e162751c77d6c28d6fe984e",
        "e63e61283d372ee1ab41b3b2dce55beb0da8ab1ff82b416abaadd2555bb55afa",
        "628c659fd7e19255e58ad99ae5208ee68a09b5a502da25914d19b59114189146",
        "86130b08f8d85bc10d54c56c791db3f3b9284d6123f538164e2865c071dbfc44",
        "a5b0a9df9d30025aad3fc705beca99d40031b4401488e10d4869a257882943f6",
        "81ccb2ea7a08b186686a9f8e2f5f8729b68b7f0e6763a56dfd54e50be363758a",
        "68db1985c7fccbde6374b9bc4dc214e9ead76494caf4019b10318506a7b449a1",
        IR_SHA,
    ];
    let embedded: [&[u8]; 10] = [
        include_bytes!("../tests/support/task4-native/independent-payment.mori"),
        include_bytes!("../tests/support/task4-native/independent-payment.scenario.json"),
        include_bytes!("../tests/support/task4-native/independent-payment.target.json"),
        include_bytes!("../tests/support/task4-native/independent-payment.authority.json"),
        include_bytes!("../tests/support/task4-native/independent-payment.payload.json"),
        include_bytes!("../tests/support/task4-native/independent-payment.signed.json"),
        include_bytes!("../tests/support/task4-native/independent-payment.prepared.json"),
        include_bytes!("../tests/support/task4-native/independent-payment.artifact.json"),
        include_bytes!("../tests/support/task4-native/independent-payment.layout.json"),
        include_bytes!("../tests/support/task4-native/independent-payment.zkir"),
    ];
    let mut data = HashMap::new();
    let mut pins = serde_json::Map::new();
    for ((name, expected), embedded) in names.iter().zip(hashes).zip(embedded) {
        let path = Path::new(FIXTURES).join(format!("independent-payment.{name}"));
        let bytes = embedded.to_vec();
        assert_eq!(sha(&bytes), expected, "fixture {name}");
        pins.insert(
            name.to_string(),
            serde_json::json!({"path":path,"sha256":expected,"bytes":bytes.len()}),
        );
        data.insert(*name, String::from_utf8(bytes).unwrap());
    }
    Fixture {
        source: data["mori"].clone(),
        scenario: serde_json::from_str(&data["scenario.json"]).unwrap(),
        target: Target::from_json(&data["target.json"]).unwrap(),
        authority: ExpectedAuthority::from_json(&data["authority.json"]).unwrap(),
        signed: SignedIntent::from_json(&data["signed.json"]).unwrap(),
        input_hashes: pins.into(),
    }
}
fn options() -> ProofOptions {
    ProofOptions {
        params_dir: "/home/charl/.cache/midnight/zk-params".into(),
        max_k: 13,
    }
}
fn prepared(
    f: &Fixture,
    at: u128,
    signed: &SignedIntent,
    authority: &ExpectedAuthority,
) -> Prepared {
    let p = prepare(
        &f.source,
        "settle",
        &f.scenario,
        &f.target,
        &options(),
        Mode::Prove((authority, UInt(at), signed)),
    )
    .unwrap();
    let strict_bytes = include_bytes!("../tests/support/task4-native/independent-payment.zkir");
    assert_eq!(p.ir, IrSource::load(&strict_bytes[..]).unwrap());
    assert_eq!(p.claim.circuit_k, 13);
    assert_eq!(p.values.len(), 47);
    p
}
fn sign(secret: u64, nonce: u64, message: Fr) -> String {
    let g = JubjubSubgroup::generator();
    let (x, k) = (Scalar::from(secret), Scalar::from(nonce));
    let (rx, ry) = schnorr::coordinates(g * k);
    let (px, py) = schnorr::coordinates(g * x);
    let h = transient_hash(&[Fr::from(schnorr::FENS_DOMAIN), rx, ry, px, py, message]);
    let mut wide = [0u8; 64];
    wide[..32].copy_from_slice(&h.as_le_bytes());
    let s = k + Scalar::from_bytes_wide(&wide) * x;
    let mut bytes = rx.as_le_bytes();
    bytes.extend(ry.as_le_bytes());
    bytes.extend(s.to_bytes());
    hex(&bytes)
}
fn key_hex(secret: u64) -> String {
    let (x, y) = schnorr::coordinates(JubjubSubgroup::generator() * Scalar::from(secret));
    let mut bytes = x.as_le_bytes();
    bytes.extend(y.as_le_bytes());
    hex(&bytes)
}
fn raw(secret: u64, sig: &str) -> Vec<Fr> {
    let s = schnorr::decode_signature(sig).unwrap();
    let (px, py) = schnorr::coordinates(JubjubSubgroup::generator() * Scalar::from(secret));
    let (rx, ry) = schnorr::coordinates(s.r);
    vec![px, py, rx, ry, s.response]
}
fn rebind(p: &mut Prepared) {
    p.claim.public_values = p.values.iter().map(|v| schnorr::native_hex(*v)).collect();
    let hash = digest(&(BINDING_VERSION, &p.claim)).unwrap();
    let low = u128::from_le_bytes(hash[..16].try_into().unwrap());
    let high = u128::from_le_bytes(hash[16..].try_into().unwrap());
    p.binding = Fr::from(low) + (Fr::from(u128::MAX) + Fr::from(1u64)) * Fr::from(high);
}
fn preimage(p: &Prepared) -> ProofPreimage {
    ProofPreimage {
        inputs: vec![],
        private_transcript: p.private.clone(),
        public_transcript_inputs: p.values.clone(),
        public_transcript_outputs: p.values.clone(),
        binding_input: p.binding,
        communications_commitment: None,
        key_location: KeyLocation("moriarty-task4-fixed-public-QA/3".into()),
    }
}
/// Only named checks are neutralized, preserving every instruction index,
/// output identifier, typed input, financial slot and public Impact.
fn recipe_ir(strict: &IrSource, sites: &[usize]) -> IrSource {
    let mut copy = strict.clone();
    let mut instructions = (*copy.instructions).clone();
    for &site in sites {
        instructions[site] = match &instructions[site] {
            I::Assert { .. } => I::Assert {
                cond: crate::immediate(1),
            },
            I::ConstrainEq { a, .. } => I::ConstrainEq {
                a: a.clone(),
                b: a.clone(),
            },
            other => panic!("unapproved preprocessing replacement {site}: {other:?}"),
        };
    }
    copy.instructions = Arc::new(instructions);
    copy
}
fn ir_disk(ir: &IrSource) -> Vec<u8> {
    let mut v = serde_json::to_value(ir).unwrap();
    v["version"] = serde_json::json!({"major":3,"minor":0});
    serde_json::to_vec(&v).unwrap()
}
fn native(v: IrValue) -> Fr {
    match v {
        IrValue::Native(n) => n,
        _ => panic!("expected Native"),
    }
}
fn point(v: IrValue) -> JubjubSubgroup {
    match v {
        IrValue::JubjubPoint(n) => n,
        _ => panic!("expected JubjubPoint"),
    }
}
fn scalar(v: IrValue) -> Scalar {
    match v {
        IrValue::JubjubScalar(n) => n,
        _ => panic!("expected JubjubScalar"),
    }
}
fn resolve(memory: &HashMap<Identifier, IrValue>, operand: &Operand) -> IrValue {
    match operand {
        Operand::Variable(id) => memory.get(id).expect("retained memory identifier").clone(),
        Operand::Immediate(n) => IrValue::Native(*n),
    }
}
fn is_bool(n: Fr) -> bool {
    assert!(n == Fr::from(0u64) || n == Fr::from(1u64));
    n == Fr::from(1u64)
}
fn within_bits(n: Fr, bits: u32) {
    assert!(bits < 255);
    let bytes = n.as_le_bytes();
    for bit in bits..256 {
        assert_eq!(bytes[bit as usize / 8] & (1 << (bit % 8)), 0);
    }
}
/// Exact-vocabulary off-circuit evaluation, checked first by the pinned upstream
/// recipe.check. It supplies *all* memory, never fabricated booleans/results.
/// In-circuit code recomputes results and checks retained memory alignment.
fn preprocess_recipe(ir: &IrSource, image: &ProofPreimage) -> Preprocessed {
    assert!(!ir.do_communications_commitment && ir.inputs.is_empty());
    let skips = ir
        .check(image)
        .expect("ordinary recipe must be internally consistent");
    assert_eq!(skips, vec![None; 47]);
    let mut memory = HashMap::new();
    let mut public = 0;
    let mut private = 0;
    let mut pis = vec![image.binding_input.0];
    let mut impacts = 0;
    for (index, ins) in ir.instructions.iter().enumerate() {
        let get = |o| resolve(&memory, o);
        let output: Option<(Identifier, IrValue)> = match ins {
            I::PublicInput {
                guard: None,
                val_t: IrType::Native,
                output,
            } => {
                let n = image.public_transcript_outputs[public];
                public += 1;
                Some((output.clone(), IrValue::Native(n)))
            }
            I::PrivateInput {
                guard: None,
                val_t,
                output,
            } => {
                let val = match val_t {
                    IrType::Native => {
                        let n = image.private_transcript[private];
                        private += 1;
                        IrValue::Native(n)
                    }
                    IrType::JubjubPoint => {
                        let mut b = image.private_transcript[private].as_le_bytes();
                        b.extend(image.private_transcript[private + 1].as_le_bytes());
                        private += 2;
                        IrValue::JubjubPoint(schnorr::decode_key(&hex(&b)).unwrap())
                    }
                    _ => panic!("unexpected private type"),
                };
                Some((output.clone(), val))
            }
            I::Add { a, b, output } => {
                let val = match (get(a), get(b)) {
                    (IrValue::Native(a), IrValue::Native(b)) => IrValue::Native(a + b),
                    (IrValue::JubjubPoint(a), IrValue::JubjubPoint(b)) => {
                        IrValue::JubjubPoint(a + b)
                    }
                    _ => panic!("unexpected add types"),
                };
                Some((output.clone(), val))
            }
            I::Mul { a, b, output } => Some((
                output.clone(),
                IrValue::Native(native(get(a)) * native(get(b))),
            )),
            I::Neg { a, output } => Some((output.clone(), IrValue::Native(-native(get(a))))),
            I::Not { a, output } => Some((
                output.clone(),
                IrValue::Native(Fr::from(!is_bool(native(get(a))))),
            )),
            I::TestEq { a, b, output } => {
                Some((output.clone(), IrValue::Native(Fr::from(get(a) == get(b)))))
            }
            I::LessThan { a, b, bits, output } => {
                let a = native(get(a));
                let b = native(get(b));
                within_bits(a, *bits);
                within_bits(b, *bits);
                Some((output.clone(), IrValue::Native(Fr::from(a < b))))
            }
            I::JubjubScalarFromNative { native: n, output } => {
                let n = native(get(n));
                let mut wide = [0u8; 64];
                wide[..32].copy_from_slice(&n.as_le_bytes());
                Some((
                    output.clone(),
                    IrValue::JubjubScalar(Scalar::from_bytes_wide(&wide)),
                ))
            }
            I::TransientHash { inputs, output } => Some((
                output.clone(),
                IrValue::Native(transient_hash(
                    &inputs.iter().map(|i| native(get(i))).collect::<Vec<_>>(),
                )),
            )),
            I::EcMulGenerator { scalar: s, output } => Some((
                output.clone(),
                IrValue::JubjubPoint(JubjubSubgroup::generator() * scalar(get(s))),
            )),
            I::EcMul {
                a,
                scalar: s,
                output,
            } => Some((
                output.clone(),
                IrValue::JubjubPoint(point(get(a)) * scalar(get(s))),
            )),
            I::IntoCoordinates { point: p, outputs } => {
                let (x, y) = schnorr::coordinates(point(get(p)));
                memory.insert(outputs.0.clone(), IrValue::Native(x));
                memory.insert(outputs.1.clone(), IrValue::Native(y));
                None
            }
            I::Encode { input, outputs } => {
                assert_eq!(outputs.len(), 1);
                let n = Fr::from_le_bytes(&scalar(get(input)).to_bytes()).unwrap();
                Some((outputs[0].clone(), IrValue::Native(n)))
            }
            I::ConstrainBits { val, bits } => {
                within_bits(native(get(val)), *bits);
                None
            }
            I::ConstrainToBoolean { val } => {
                is_bool(native(get(val)));
                None
            }
            I::ConstrainEq { a, b } => {
                assert_eq!(get(a), get(b), "recipe equality at {index}");
                None
            }
            I::Assert { cond } => {
                assert!(is_bool(native(get(cond))), "recipe assertion at {index}");
                None
            }
            I::Impact { guard, inputs } => {
                assert!(is_bool(native(get(guard))));
                assert_eq!(inputs.len(), 1);
                let n = native(get(&inputs[0]));
                assert_eq!(n, image.public_transcript_inputs[impacts]);
                impacts += 1;
                pis.push(n.0);
                None
            }
            I::Output { vals } => {
                assert_eq!(vals.len(), ir.outputs.len());
                for val in vals {
                    native(get(val));
                }
                None
            }
            other => panic!("instruction outside frozen recipe vocabulary at {index}: {other:?}"),
        };
        if let Some((id, value)) = output {
            assert!(
                memory.insert(id, value).is_none(),
                "duplicate memory identifier"
            );
        }
    }
    assert_eq!((public, private, impacts), (47, 5, 47));
    Preprocessed {
        memory,
        pis,
        pi_skips: skips,
        binding_input: image.binding_input.0,
        comm_comm: None,
    }
}
struct Recipe {
    prepared: Prepared,
    sites: Vec<usize>,
    attachment: SignedIntent,
    authority: ExpectedAuthority,
    at: u128,
}
fn recipe(case: &str, f: &Fixture) -> Recipe {
    let mut authority = f.authority.clone();
    let mut attachment = f.signed.clone();
    let mut p = prepared(f, 15, &attachment, &authority);
    let mut sites = vec![];
    match case {
        "response-plus-one" => {
            p.private[4] = p.private[4] + Fr::from(1u64);
            attachment
                .signature_hex
                .replace_range(128..192, &schnorr::native_hex(p.private[4]));
            schnorr::decode_signature(&attachment.signature_hex).unwrap();
            sites = vec![222];
        }
        "wrong-private-key" => {
            let sig = sign(
                11,
                31,
                schnorr::decode_native_hex(&p.claim.authorization.as_ref().unwrap().message_hex)
                    .unwrap(),
            );
            schnorr::verify_attachment(&key_hex(11), &sig, p.values[36]).unwrap();
            p.private = raw(11, &sig);
            attachment.signature_hex = sig;
            sites = vec![202, 203];
        }
        "tight-gross" | "tight-fee" | "tight-net" | "tight-duty" => {
            let i = match case {
                "tight-gross" => 0,
                "tight-fee" => 1,
                "tight-net" => 2,
                _ => 3,
            };
            let cap = &mut authority.limits[0];
            match i {
                0 => cap.gross_cap = UInt(71),
                1 => cap.fee_cap = UInt(1),
                2 => cap.net_floor = UInt(69),
                _ => cap.liability_cap = UInt(34),
            };
            let r = signed_relation::task4_reconstruct_before_caps(
                &f.source,
                "settle",
                &f.scenario,
                &f.target,
                &authority,
                UInt(15),
            )
            .unwrap();
            assert_eq!(p.ir, IrSource::load(r.circuit.zkir.as_bytes()).unwrap());
            let s = r.signed.as_ref().unwrap();
            attachment = SignedIntent {
                payload: s.payload.clone(),
                signature_hex: sign(7, 31, s.message),
            };
            schnorr::verify_attachment(
                &authority.public_key_hex,
                &attachment.signature_hex,
                s.message,
            )
            .unwrap();
            p.values = r.public_values();
            p.private = raw(7, &attachment.signature_hex);
            p.claim.authorization = Some(ClaimAuthorization {
                payload_hash: hex(&s.payload_hash),
                message_hex: schnorr::native_hex(s.message),
                public_key_hex: authority.public_key_hex.clone(),
                at: UInt(15),
            });
            rebind(&mut p);
            sites = vec![243 + i * 3];
        }
        "alternative" => {
            attachment.signature_hex = sign(7, 31, p.values[36]);
            p = prepared(f, 15, &attachment, &authority);
            assert_ne!(attachment.signature_hex, f.signed.signature_hex);
        }
        "positive" | "raw-honest" | "reused-proof" => {}
        _ => panic!("unknown case"),
    }
    Recipe {
        prepared: p,
        sites,
        attachment,
        authority,
        at: 15,
    }
}
fn memory_values(preproc: &Preprocessed) -> std::collections::BTreeMap<String, serde_json::Value> {
    preproc.memory.iter().map(|(id,v)| {
        let value=match v {
            IrValue::Native(n)=>serde_json::json!({"type":"Native","value":schnorr::native_hex(*n)}),
            IrValue::JubjubScalar(n)=>serde_json::json!({"type":"JubjubScalar","value":hex(&n.to_bytes())}),
            IrValue::JubjubPoint(p)=>{let (x,y)=schnorr::coordinates(*p);serde_json::json!({"type":"JubjubPoint","coordinates":[schnorr::native_hex(x),schnorr::native_hex(y)]})},
            _=>panic!("memory type outside frozen vocabulary"),
        };(id.0.clone(),value)
    }).collect()
}
fn summary(case: &str, r: &Recipe, f: &Fixture) -> serde_json::Value {
    let copy = recipe_ir(&r.prepared.ir, &r.sites);
    let image = preimage(&r.prepared);
    let preproc = preprocess_recipe(&copy, &image);
    assert_eq!(
        preproc.pis,
        statement(&r.prepared)
            .iter()
            .map(|n| n.0)
            .collect::<Vec<_>>()
    );
    let sites:Vec<_>=r.sites.iter().map(|&i|serde_json::json!({"index":i,"original":r.prepared.ir.instructions[i],"replacement":copy.instructions[i]})).collect();
    serde_json::json!({"profile":"moriarty-task4-recipe/1","case":case,"qualification":"ordinary-only-not-native-evidence","inputs":f.input_hashes,"strict_ir_sha256":IR_SHA,"recipe_ir_sha256":sha(&ir_disk(&copy)),"recipe_ir":serde_json::from_slice::<serde_json::Value>(&ir_disk(&copy)).unwrap(),"neutralized_checks":sites,"claim":r.prepared.claim,"claim_sha256":hex(&digest(&r.prepared.claim).unwrap()),"statement_sha256":sha(&json_bytes(&statement(&r.prepared).iter().map(|n|schnorr::native_hex(*n)).collect::<Vec<_>>())),"statement":statement(&r.prepared).iter().map(|n|schnorr::native_hex(*n)).collect::<Vec<_>>(),"at":r.at,"raw_private":r.prepared.private.iter().map(|n|schnorr::native_hex(*n)).collect::<Vec<_>>(),"attachment":r.attachment,"authority":r.authority,"memory":memory_values(&preproc),"memory_sha256":sha(&json_bytes(&memory_values(&preproc))),"raw_private_sha256":sha(&json_bytes(&r.prepared.private.iter().map(|n|schnorr::native_hex(*n)).collect::<Vec<_>>())),"memory_identifiers":preproc.memory.len(),"limits":limits(case),"call_categories":SLOTS,"address_space_bytes":AS_BYTES,"deadline_seconds":60,"kill_grace_seconds":2})
}

fn changed_claims(f: &Fixture) -> Vec<(String, Prepared, SignedIntent, ExpectedAuthority, u128)> {
    let mut out = vec![];
    // Exact direct raw financial mutation: closing bob +1, all other financial
    // positions fixed. Recompute MOI1 and Claim binding; this is deliberately
    // an invalid financial execution, not an invented host-accepted report.
    let mut p = prepared(f, 15, &f.signed, &f.authority);
    let position = p
        .claim
        .financial_bindings
        .iter()
        .position(|b| b.role == "post_balance" && b.name == "[\"QA\",\"bob\"]")
        .expect("bob post balance");
    p.values[position] = p.values[position] + Fr::from(1u64);
    let bob = p
        .execution
        .post
        .balances
        .iter_mut()
        .find(|b| b.account == "bob")
        .unwrap();
    bob.amount = UInt(bob.amount.0.checked_add(1).unwrap());
    p.claim.execution_hash = hex(&digest(&p.execution).unwrap());
    let recipe = &p.ir;
    // MOI1 inputs are evaluated from the original financial/public input IDs;
    // the full generic recipe evaluator is not used for this invalid transition.
    let mut memory = HashMap::new();
    let mut public = 0;
    for ins in recipe.instructions.iter() {
        if let I::PublicInput { output, .. } = ins {
            memory.insert(output.clone(), IrValue::Native(p.values[public]));
            public += 1;
        }
        if let I::TransientHash { inputs, .. } = ins {
            assert_eq!(inputs.len(), 83);
            p.values[36] = transient_hash(
                &inputs
                    .iter()
                    .map(|o| native(resolve(&memory, o)))
                    .collect::<Vec<_>>(),
            );
            break;
        }
    }
    p.claim.authorization.as_mut().unwrap().message_hex = schnorr::native_hex(p.values[36]);
    rebind(&mut p);
    let sig = SignedIntent {
        payload: f.signed.payload.clone(),
        signature_hex: sign(7, 31, p.values[36]),
    };
    schnorr::verify_attachment(
        &f.authority.public_key_hex,
        &sig.signature_hex,
        p.values[36],
    )
    .unwrap();
    out.push((
        "financial-post-bob-plus-one".into(),
        p,
        sig,
        f.authority.clone(),
        15,
    ));
    // Nonce changes recompute the payload SHA, MOI1 and a valid attachment.
    for (id, secret, at) in [
        ("message-nonce-two", 7u64, 15u128),
        ("public-key-eleven", 11, 15),
        ("current-at-sixteen", 7, 16),
    ] {
        let mut authority = f.authority.clone();
        if id == "message-nonce-two" {
            authority.nonce = UInt(2);
        }
        if secret == 11 {
            authority.public_key_hex = key_hex(11);
        }
        let r = signed_relation::task4_reconstruct_before_caps(
            &f.source,
            "settle",
            &f.scenario,
            &f.target,
            &authority,
            UInt(at),
        )
        .unwrap();
        let s = r.signed.as_ref().unwrap();
        let sig = SignedIntent {
            payload: s.payload.clone(),
            signature_hex: sign(secret, 31, s.message),
        };
        let p = prepared(f, at, &sig, &authority);
        out.push((id.into(), p, sig, authority, at));
    }
    out
}
fn assert_execution(execution: &Execution) {
    let value = serde_json::to_value(execution).unwrap();
    let closing: Vec<_> = value["post"]["balances"]
        .as_array()
        .unwrap()
        .iter()
        .map(|b| {
            (
                b["account"].as_str().unwrap(),
                b["amount"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(closing, [("alice", "28"), ("bob", "80"), ("treasury", "2")]);
    assert_eq!(value["post"]["state"]["paid"]["value"], "70");
    let duties: Vec<_> = value["post"]["liabilities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|b| (b["name"].as_str().unwrap(), b["amount"].as_str().unwrap()))
        .collect();
    assert_eq!(duties, [("payment", "30"), ("reserve", "5"), ("zero", "0")]);
    let effects = &execution.effects;
    assert_eq!(effects.len(), 5);
    match &effects[0] {
        Effect::Transfer {
            from, to, amount, ..
        } => assert_eq!((from.as_str(), to.as_str(), amount.0), ("alice", "bob", 70)),
        _ => panic!("first ordered effect"),
    };
    match &effects[1] {
        Effect::Fee {
            from, to, amount, ..
        } => assert_eq!(
            (from.as_str(), to.as_str(), amount.0),
            ("alice", "treasury", 2)
        ),
        _ => panic!("second ordered effect"),
    };
    match &effects[2] {
        Effect::Liability {
            name,
            before,
            after,
            ..
        } => assert_eq!(
            (name.as_str(), *before, *after),
            ("payment", UInt(50), UInt(30))
        ),
        _ => panic!("third ordered effect"),
    };
    match &effects[3] {
        Effect::State {
            name,
            before,
            after,
        } => assert_eq!(
            (name.as_str(), before, after),
            ("paid", &Value::UInt(UInt(0)), &Value::UInt(UInt(70)))
        ),
        _ => panic!("fourth ordered effect"),
    };
    match &effects[4] {
        Effect::Emit { name, value } => {
            assert_eq!((name.as_str(), value), ("paid", &Value::UInt(UInt(70))))
        }
        _ => panic!("fifth ordered effect"),
    };
}
fn native_verify(
    vk: &midnight_transient_crypto::proofs::VerifierKey,
    proof: &Proof,
    p: &Prepared,
) -> Result<(), String> {
    charge("verify");
    vk.verify(&PARAMS_VERIFIER, proof, statement(p).into_iter())
        .map_err(|e| format!("{e:?}"))
}
fn begin(case: &str) -> PathBuf {
    let binary_sha = sha(&fs::read(std::env::current_exe().unwrap()).unwrap());
    assert_eq!(
        std::env::var("MORIARTY_TASK4_BINARY_SHA").as_deref(),
        Ok(binary_sha.as_str()),
        "exact frozen binary required"
    );
    assert_eq!(
        std::env::var("MORIARTY_TASK4_NATIVE_ADMISSION").as_deref(),
        Ok("root-approved-exact-resource-votes"),
        "both fresh exact resource votes required"
    );
    #[cfg(unix)]
    {
        let mut limit = libc::rlimit {
            rlim_cur: 0,
            rlim_max: 0,
        };
        assert_eq!(unsafe { libc::getrlimit(libc::RLIMIT_AS, &mut limit) }, 0);
        assert!(
            limit.rlim_cur <= AS_BYTES && limit.rlim_cur > 0,
            "external 2GiB RLIMIT_AS required"
        );
    }
    let dir = Path::new(RUNTIME).join("execution").join(case);
    fs::create_dir_all(dir.parent().unwrap()).unwrap();
    fs::create_dir(&dir).expect("one attempt only; preserve prior captures");
    let events = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(dir.join("calls.jsonl"))
        .unwrap();
    *BUDGET.lock().unwrap() = Some(Budget {
        case: case.into(),
        counts: [0; 7],
        limits: limits(case),
        events,
    });
    dir
}
fn saved_positive() -> ProofBundle {
    let bytes = fs::read(Path::new(RUNTIME).join("execution/positive/bundle.json"))
        .expect("positive must finish first");
    ProofBundle::from_json(std::str::from_utf8(&bytes).unwrap()).unwrap()
}
fn require_pass(case: &str) {
    let result: serde_json::Value = serde_json::from_slice(
        &fs::read(
            Path::new(RUNTIME)
                .join("execution")
                .join(case)
                .join("result.json"),
        )
        .expect("prerequisite native case must finish first"),
    )
    .unwrap();
    assert_eq!(
        result["passed"], true,
        "failed prerequisite; stop without retry"
    );
    assert_eq!(
        result["binary_sha256"],
        std::env::var("MORIARTY_TASK4_BINARY_SHA").unwrap(),
        "prerequisite from different binary"
    );
}
fn execute(case: &str, dir: &Path) {
    if case != "positive" {
        require_pass("positive");
    }
    if CASES[2..8].contains(&case) {
        require_pass("raw-honest");
    }
    let f = fixture();
    for pin in f.input_hashes.as_object().unwrap().values() {
        let bytes = fs::read(pin["path"].as_str().unwrap()).unwrap();
        assert_eq!(
            sha(&bytes),
            pin["sha256"].as_str().unwrap(),
            "runtime input changed"
        );
    }
    let r = recipe(case, &f);
    write_new(
        &dir.join("recipe.json"),
        &json_bytes(&summary(case, &r, &f)),
    );
    write_new(
        &dir.join("execution.json"),
        &json_bytes(&r.prepared.execution),
    );
    let p = &r.prepared;
    assert_execution(&p.execution);
    match case {
        "positive" => {
            // Host attachment refusals run against production entrypoints while
            // counters are active, before any real SRS access.
            let bad = recipe("response-plus-one", &f).attachment;
            let mut missing = options();
            missing.params_dir = dir.join("does-not-exist");
            let err = prove_signed(
                &f.source,
                "settle",
                &f.scenario,
                &f.target,
                &f.authority,
                UInt(15),
                &bad,
                &missing,
            )
            .unwrap_err();
            assert_eq!(err.code, "mori::backend::signature");
            let dummy: ProofBundle =
                serde_json::from_value(with_proof(&p.claim, "00").unwrap()).unwrap();
            let err = verify_signed(
                &f.source,
                "settle",
                &f.scenario,
                &f.target,
                &f.authority,
                UInt(15),
                &bad,
                &dummy,
                &missing,
            )
            .unwrap_err();
            assert_eq!(err.code, "mori::backend::signature");
            assert_eq!(BUDGET.lock().unwrap().as_ref().unwrap().counts, [0; 7]);
            write_new(
                &dir.join("host-attachment-refusal.json"),
                &json_bytes(
                    &serde_json::json!({"code":err.code,"message":err.message,"native_counts":vec![0;7]}),
                ),
            );
            let (bundle, produced) = prove_signed(
                &f.source,
                "settle",
                &f.scenario,
                &f.target,
                &f.authority,
                UInt(15),
                &f.signed,
                &options(),
            )
            .unwrap();
            // Save exact proof immediately, even if later consumer checking fails.
            write_new(
                &dir.join("proof.bin"),
                &decode_proof(&bundle.proof_hex).unwrap().0,
            );
            write_new(&dir.join("bundle.json"), &json_bytes(&bundle));
            write_new(&dir.join("producer-receipt.json"), &json_bytes(&produced));
            let consumed = verify_signed(
                &f.source,
                "settle",
                &f.scenario,
                &f.target,
                &f.authority,
                UInt(15),
                &f.signed,
                &bundle,
                &options(),
            )
            .unwrap();
            assert_eq!(json_bytes(&consumed), json_bytes(&produced));
            assert_eq!(json_bytes(&produced.execution), json_bytes(&p.execution));
            write_new(&dir.join("consumer-receipt.json"), &json_bytes(&consumed));
            // Fresh independently reconstructed original IR/statement/VK.
            let fresh = prepare(
                &f.source,
                "settle",
                &f.scenario,
                &f.target,
                &options(),
                Mode::Verify((&f.authority, UInt(15), &f.signed)),
            )
            .unwrap();
            let params = OfflineParameters::load(&options().params_dir, 13).unwrap();
            charge("keygen_vk");
            let vk = futures::executor::block_on(fresh.ir.keygen_vk(&params)).unwrap();
            native_verify(&vk, &decode_proof(&bundle.proof_hex).unwrap(), &fresh).unwrap();
        }
        "alternative" => {
            let bundle = saved_positive();
            let signed = r.attachment.clone();
            assert_ne!(signed.signature_hex, f.signed.signature_hex);
            let fresh = prepared(&f, 15, &signed, &f.authority);
            assert_eq!(statement(&fresh), statement(p));
            write_new(
                &dir.join("alternative-attachment.json"),
                &json_bytes(&signed),
            );
            let receipt = verify_signed(
                &f.source,
                "settle",
                &f.scenario,
                &f.target,
                &f.authority,
                UInt(15),
                &signed,
                &bundle,
                &options(),
            )
            .unwrap();
            write_new(&dir.join("consumer-receipt.json"), &json_bytes(&receipt));
        }
        "reused-proof" => {
            let bundle = saved_positive();
            let proof = decode_proof(&bundle.proof_hex).unwrap();
            let params = OfflineParameters::load(&options().params_dir, 13).unwrap();
            charge("keygen_vk");
            let vk = futures::executor::block_on(p.ir.keygen_vk(&params)).unwrap();
            for (id, changed, signed, authority, at) in changed_claims(&f) {
                write_new(
                    &dir.join(format!("{id}.json")),
                    &json_bytes(
                        &serde_json::json!({"claim":changed.claim,"statement":statement(&changed).iter().map(|n|schnorr::native_hex(*n)).collect::<Vec<_>>(),"attachment":signed,"authority":authority,"at":at,"scope":if id.starts_with("financial") {"direct-raw-invalid-financial-report"}else{"independently-reconstructed-host-valid-statement"}}),
                    ),
                );
                assert_ne!(statement(p), statement(&changed));
                if !id.starts_with("financial") {
                    let before = BUDGET.lock().unwrap().as_ref().unwrap().counts;
                    let host = verify_signed(
                        &f.source,
                        "settle",
                        &f.scenario,
                        &f.target,
                        &authority,
                        UInt(at),
                        &signed,
                        &bundle,
                        &options(),
                    )
                    .unwrap_err();
                    assert_eq!(host.code, "mori::backend::claim");
                    assert_eq!(before, BUDGET.lock().unwrap().as_ref().unwrap().counts);
                    write_new(
                        &dir.join(format!("{id}-host-refusal.json")),
                        &json_bytes(
                            &serde_json::json!({"phase":"host-claim-comparison-before-SRS","code":host.code,"message":host.message}),
                        ),
                    );
                }
                let err = native_verify(&vk, &proof, &changed)
                    .expect_err("unchanged proof accepted changed native statement");
                write_new(
                    &dir.join(format!("{id}-refusal.json")),
                    &json_bytes(&serde_json::json!({"phase":"native-verifier","error":err})),
                );
            }
            let mut append = proof.clone();
            append.0.push(0);
            let mut flip = proof.clone();
            flip.0[0] ^= 1;
            for (id, bad) in [("proof-append", append), ("proof-bitflip-byte0-bit0", flip)] {
                write_new(&dir.join(format!("{id}.bin")), &bad.0);
                let err = native_verify(&vk, &bad, p)
                    .expect_err("native verifier accepted corrupted proof");
                write_new(
                    &dir.join(format!("{id}-refusal.json")),
                    &json_bytes(&serde_json::json!({"phase":"native-verifier","error":err})),
                );
            }
        }
        _ => {
            let recipe = recipe_ir(&p.ir, &r.sites);
            let preproc = preprocess_recipe(&recipe, &preimage(p));
            let params = OfflineParameters::load(&options().params_dir, 13).unwrap();
            charge("keygen");
            let (pk, vk) = futures::executor::block_on(p.ir.keygen(&params)).unwrap();
            charge("prove_unchecked");
            let proof = futures::executor::block_on(p.ir.prove_unchecked(
                rand::rngs::OsRng,
                &params,
                pk,
                preproc,
            ));
            let proof = match proof {
                Ok(proof) => proof,
                Err(err) => {
                    write_new(
                        &dir.join("native-prover-error.json"),
                        &json_bytes(
                            &serde_json::json!({"phase":"strict-original-prove_unchecked","error":format!("{err:?}"),"identified_recipe_sites":r.sites,"qualification":"FAIL-unexpected-prover-error-not-constraint-evidence"}),
                        ),
                    );
                    panic!("unqualified native prover error: {err:?}")
                }
            };
            assert!(!proof.0.is_empty() && proof.0.len() <= MAX_PROOF_BYTES);
            write_new(&dir.join("proof.bin"), &proof.0);
            let verification = native_verify(&vk, &proof, p);
            if case == "raw-honest" {
                verification.unwrap();
            } else {
                let err = verification
                    .expect_err("hostile witness produced accepted strict-original proof");
                write_new(
                    &dir.join("native-verifier-refusal.json"),
                    &json_bytes(
                        &serde_json::json!({"phase":"strict-original-native-verifier","error":err,"identified_recipe_sites":r.sites,"qualification":"produced-proof-rejected"}),
                    ),
                );
            }
        }
    }
}

#[test]
fn task4_recipe_budget_refuses_unplanned_calls() {
    let mut counts = [0; 7];
    assert!(reserve(&mut counts, &[0; 7], 0).is_err());
    assert_eq!(counts, [0; 7]);
    let limit = [1; 7];
    for i in 0..7 {
        reserve(&mut counts, &limit, i).unwrap();
        assert!(reserve(&mut counts, &limit, i).is_err());
    }
    assert_eq!(counts, [1; 7]);
}
#[test]
fn task4_recipe_parameter_free_smoke() {
    let f = fixture();
    let mut summaries = vec![];
    for case in CASES {
        let r = recipe(case, &f);
        let result = summary(case, &r, &f);
        let original = r.prepared.ir.check(&preimage(&r.prepared));
        if r.sites.is_empty() {
            original.unwrap();
        } else {
            assert!(original.is_err());
        }
        summaries.push(result);
    }
    let p = prepared(&f, 15, &f.signed, &f.authority);
    assert_execution(&p.execution);
    let changed = changed_claims(&f);
    assert_eq!(changed.len(), 4);
    for (_, c, _, _, _) in &changed {
        assert_ne!(statement(&p), statement(c));
    }
    assert!(BUDGET.lock().unwrap().is_none());
    if let Ok(path) = std::env::var("MORIARTY_TASK4_RECIPE_OUTPUT") {
        let dir = Path::new(&path);
        fs::create_dir(dir).expect("new recipe capture directory only");
        for value in summaries {
            let case = value["case"].as_str().unwrap();
            write_new(&dir.join(format!("{case}.json")), &json_bytes(&value));
        }
        for (id, p, signed, authority, at) in changed {
            write_new(
                &dir.join(format!("{id}.json")),
                &json_bytes(
                    &serde_json::json!({"claim":p.claim,"statement":statement(&p).iter().map(|n|schnorr::native_hex(*n)).collect::<Vec<_>>(),"signed":signed,"authority":authority,"at":at}),
                ),
            );
        }
        write_new(
            &dir.join("plan.json"),
            &json_bytes(
                &serde_json::json!({"profile":"moriarty-task4-call-plan/1","cases":CASES,"categories":SLOTS,"aggregate_max_calls":[12,20,8,4,1,7,17],"processes":10,"address_space_bytes":AS_BYTES,"per_process_deadline_seconds":60,"kill_grace_seconds":2,"aggregate_deadline_seconds":600,"aggregate_kill_grace_seconds":20,"no_retry":true,"k":13,"rows":4193,"params_snapshot":"task-3-signed-sol/k13-resource-snapshot.json","params_sha256":"d3324910969c4cc54143b8045b649e5c3a4bd5fb7b8f85fe1b770f640ce1c803","params_bytes":1573252,"qualification":"specified-only-resource-votes-still-required"}),
            ),
        );
    }
}
#[test]
#[ignore = "Task4 genuine native experiment: both fresh exact resource votes and external process limits required"]
fn task4_native_resource_gated_entry() {
    let case = std::env::var("MORIARTY_TASK4_CASE").expect("exact approved case");
    assert!(CASES.contains(&case.as_str()));
    let dir = begin(&case);
    // Catch only to persist failure. Panics never qualify a hostile constraint.
    let started = std::time::Instant::now();
    let outcome = std::panic::catch_unwind(|| execute(&case, &dir));
    let panic_message = outcome.as_ref().err().map(|payload| {
        if let Some(s) = payload.downcast_ref::<String>() {
            s.clone()
        } else if let Some(s) = payload.downcast_ref::<&str>() {
            s.to_string()
        } else {
            "non-string panic".into()
        }
    });
    let guard = BUDGET.lock().unwrap();
    let budget = guard.as_ref().unwrap();
    let complete = outcome.is_ok() && budget.counts == budget.limits;
    let result = serde_json::json!({"case":case,"binary_sha256":std::env::var("MORIARTY_TASK4_BINARY_SHA").unwrap(),"elapsed_milliseconds":started.elapsed().as_millis(),"panic":panic_message,"passed":complete,"counts":budget.counts,"planned":budget.limits,"qualification":if complete {"scoped-native-result-pending-independent-review"}else{"FAILED-unqualified-no-retry"}});
    write_new(&dir.join("result.json"), &json_bytes(&result));
    println!("TASK4_RESULT {result}");
    drop(guard);
    assert!(
        complete,
        "failed or incomplete native case; allocation consumed"
    );
}
