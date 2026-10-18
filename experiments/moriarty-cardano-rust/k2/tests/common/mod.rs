//! Shared test helpers. Source only: none of these tests has been compiled or run.
//!
//! Expectation grades used by the tests (full table in `EXPECTATIONS.json`):
//! - [R1-AUTH]: `RUST-ABI2-R1-EXPECTATIONS.json`, the R1 author's fixtures. Their Core effects
//!   are the frozen non-author Sol `/1` vectors; bodies, shapes and statements are same-lineage
//!   author derivations. Agreement is cross-implementation, not an independent oracle.
//! - [HELDOUT]: `HELDOUT-VECTORS.json` H1-P1, frozen by a non-author reviewer before reading the
//!   compiler. The SettlementHashV2 context used with it is synthetic.
//! - [ADDENDUM-AUTH]: program hashes from `EXPECTED-PACKET/ADDENDUM-1.json` (author-derived from
//!   the oracle typed Programs).
//! - [CONSUMER]: byte constants copied from the Aiken C2 consumer `groth16.ak`.
//! - [K2-AUTH]: this author's predictions (refusal kinds, structural properties).
//! - [SPECIMEN]: structurally valid bytes built in the test (generator points, labelled
//!   digests). A specimen is never a setup output and no proof verifies under it.
//! No expected value is computed by the driver under test.
#![allow(dead_code)]

use mori_k2_driver::input::{ContextInput, ProveRequest, RelationSelector, TransitionInput};
use serde_json::Value as Json;

macro_rules! r1cs {
    ($p:literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../r1cs/", $p))
    };
}

macro_rules! own {
    ($p:literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $p))
    };
}

pub const INVOICE: &str = r1cs!("fixtures/reused/installment-invoice.mori");
pub const RESERVE: &str = r1cs!("fixtures/reused/conditional-reserve.mori");
pub const H1: &str = r1cs!("fixtures/heldout/H1-settle.mori");
pub const DELIVERY2: &str = r1cs!("fixtures/staged/delivery.mori");
pub const PAYROLL: &str = r1cs!("EXPECTED-PACKET/fresh/payroll-ledger.mori");
pub const R1_EXPECT: &str = r1cs!("expectations/RUST-ABI2-R1-EXPECTATIONS.json");
pub const HELDOUT: &str = r1cs!("fixtures/heldout/HELDOUT-VECTORS.json");
pub const ADDENDUM: &str = r1cs!("EXPECTED-PACKET/ADDENDUM-1.json");

pub const REQ_INVOICE: &str = own!("requests/invoice-partial.request.json");
pub const REQ_RESERVE: &str = own!("requests/reserve-allocate.request.json");
pub const SEL_INVOICE: &str = own!("requests/invoice-partial.selector.json");
pub const SEL_RESERVE: &str = own!("requests/reserve-allocate.selector.json");

/// [CONSUMER] `groth16.ak:20` `vk_ic_count`.
pub const CONSUMER_VK_IC_COUNT_HEX: &str = "0300000000000000";
/// [CONSUMER] `groth16.ak:15,17`.
pub const CONSUMER_VK_LEN: usize = 488;
pub const CONSUMER_PROOF_LEN: usize = 192;

pub fn json(text: &str) -> Json {
    serde_json::from_str(text).expect("frozen JSON parses")
}

/// [R1-AUTH] one fixture case.
pub fn r1_case(id: &str) -> Json {
    json(R1_EXPECT)["cases"][id].clone()
}

/// Test-local hex decoder, independent of the driver's.
pub fn unhex(s: &str) -> Vec<u8> {
    assert!(s.len() % 2 == 0, "even hex");
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex")).collect()
}

pub fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

pub fn arr32(s: &str) -> [u8; 32] {
    unhex(s).try_into().expect("32 bytes")
}

pub fn request(text: &str) -> ProveRequest {
    mori_k2_driver::input::parse_request(text.as_bytes()).expect("example request parses")
}

pub fn selector(action: &str) -> RelationSelector {
    RelationSelector {
        format: "moriarty-k2-relation-selector/1".into(),
        language: "moriarty/1".into(),
        semantics: "moriarty-core/1".into(),
        action: action.into(),
        adapter: "SettlementHashV2".into(),
        cap: "NotBound".into(),
        accounting: "GrossDebitAndFees".into(),
        frame: "TouchedOnly".into(),
        expected_source_sha256: None,
        expected_program_hash: None,
    }
}

/// [SPECIMEN] a labelled synthetic context (held-out tests only).
pub fn synthetic_context() -> ContextInput {
    ContextInput {
        claimant_vkh: "55".repeat(28),
        network_id: 0,
        settle_hash: "a2".repeat(28),
        thread_id: "33".repeat(32),
        consumed_txid: "44".repeat(32),
        consumed_ix: 0,
        seq: 0,
        catalogue_digest: "66".repeat(32),
        intents_digest: "77".repeat(32),
        vk_digest: None,
    }
}

/// [HELDOUT] vector by id.
pub fn heldout(id: &str) -> Json {
    json(HELDOUT)["vectors"].as_array().expect("vectors").iter().find(|v| v["id"] == id).cloned().unwrap_or_else(|| panic!("no vector {id}"))
}

/// [HELDOUT] H1-P1 as explicit records (scenario inputs; expected pre, post and effects).
pub fn h1_records() -> TransitionInput {
    let v = heldout("H1-P1");
    TransitionInput::Records {
        inputs: serde_json::from_value(v["scenario"]["inputs"].clone()).expect("inputs"),
        pre: serde_json::from_value(v["expected"]["pre"].clone()).expect("pre"),
        post: serde_json::from_value(v["expected"]["post"].clone()).expect("post"),
        effects: serde_json::from_value(v["expected"]["effects"].clone()).expect("effects"),
    }
}

/// [HELDOUT, HOSTILE] H1-P1 with the seller's USDX post balance raised by one.
pub fn h1_records_dishonest() -> TransitionInput {
    let TransitionInput::Records { inputs, pre, mut post, effects } = h1_records() else {
        panic!("records")
    };
    let row = post
        .balances
        .iter_mut()
        .find(|b| b.account == "seller" && b.asset.reference == "usdx")
        .expect("seller USDX post row");
    row.amount = mori_core::UInt(row.amount.0 + 1);
    TransitionInput::Records { inputs, pre, post, effects }
}
