//! Tier H (K4H): hostile assignments against the FIXED setup-mode matrices of the two adopted
//! relations, statement receipts against independently frozen gold, and the K5 setup digest
//! binding. No key, proof, pairing or randomness. Source only: never compiled or run; every
//! expected outcome is pre-registered in `gold/K4H-GOLD.json` and read from there.
//!
//! How a case is decided. `FixedMatrices::of` synthesizes the relation in setup mode and must
//! reproduce the stage-C `r1csSha256`. A case builds a full assignment `z = [1, x1, x2, w...]`
//! and the fixed rows decide it:
//! - semantic cases (IA, RA, OA) change claimed values; the compiler rebuilds the statement and
//!   its digest consistently and a prove-mode synthesis supplies `z`;
//! - raw cases (IB, RB) edit entries of the honest `z` directly (public inputs, Booleans, limb
//!   bits, duty canonicality, context and body bits, BLAKE2b intermediates, digest bits);
//! - sweeps perturb every witness of the honest `z` once.
//! No host evaluator verdict is consulted. Run single-threaded with `--show-output` (not
//! `--nocapture`, which would split libtest's `test <name> ... ok` line); each case prints one
//! `K4H-CASE` line into the captured output section.
//!
//! Each test builds its own fixtures (no shared statics), trading time for no `Sync` demand.
mod common;

use ark_bls12_381::Fr;
use ark_ff::{One, Zero};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystem, OptimizationGoal, SynthesisMode};
use common::*;
use mori_core::{UInt, Value};
use mori_k2_driver::claim::{self, PreparedClaim};
use mori_k2_driver::error::DriverError;
use mori_k2_driver::fixed::{self, FixedMatrices};
use mori_k2_driver::input::{ContextInput, ProveRequest, REQUEST_FORMAT};
use mori_k2_driver::relation::{self, SelectedRelation};
use mori_k2_driver::{TransitionInput, compile_selected, inspect, setup};
use mori_r1cs::{ContextV2, DutyValue, EffectShape, EffectValues, Tamper, TransitionValues};
use serde_json::Value as Json;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

const GOLD: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/gold/K4H-GOLD.json"));

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Rel {
    Partial,
    Allocate,
    Open,
}

impl Rel {
    fn key(self) -> &'static str {
        match self {
            Rel::Partial => "invoice/partial",
            Rel::Allocate => "reserve/allocate",
            Rel::Open => "invoice/open",
        }
    }
}

fn gold() -> Json {
    json(GOLD)
}

fn rel_gold(g: &Json, rel: Rel) -> Json {
    g["relations"][rel.key()].clone()
}

/// Everything one relation's cases need.
struct Fx {
    rel: Rel,
    gold: Json,
    sel: SelectedRelation,
    fixed: FixedMatrices,
    request: ProveRequest,
    vk: [u8; 32],
    honest: PreparedClaim,
    /// Honest full assignment `[1, x1, x2, w...]`.
    z: Vec<Fr>,
}

/// [R1-AUTH transcription] invoice/open request from the R1 P01 bridge scenario and context.
fn open_request() -> ProveRequest {
    let case = r1_case("P01");
    let c = &case["context"];
    let s = |k: &str| c[k].as_str().unwrap().to_string();
    ProveRequest {
        format: REQUEST_FORMAT.into(),
        selector: selector("open"),
        transition: TransitionInput::CoreReference { scenario: serde_json::from_value(case["bridgeScenario"].clone()).expect("P01 scenario") },
        context: ContextInput {
            claimant_vkh: s("claimantVkh"),
            network_id: c["networkId"].as_u64().unwrap() as u8,
            settle_hash: s("settleHash"),
            thread_id: s("threadId"),
            consumed_txid: s("consumedTxid"),
            consumed_ix: c["consumedIx"].as_u64().unwrap() as u16,
            seq: c["seq"].as_u64().unwrap(),
            catalogue_digest: s("catalogueDigest"),
            intents_digest: s("intentsDigest"),
            vk_digest: None,
        },
    }
}

fn fixture(rel: Rel) -> Fx {
    let g = gold();
    let rg = rel_gold(&g, rel);
    let (source, request) = match rel {
        Rel::Partial => (INVOICE, request(REQ_INVOICE)),
        Rel::Allocate => (RESERVE, request(REQ_RESERVE)),
        Rel::Open => (INVOICE, open_request()),
    };
    let sel = compile_selected(source, &request.selector).unwrap_or_else(|e| panic!("{}: compile: {e}", rel.key()));
    let fixed = FixedMatrices::of(&sel).unwrap_or_else(|e| panic!("{}: fixed matrices: {e}", rel.key()));
    let vk = arr32(rg["declaredVkDigest"].as_str().unwrap());
    let honest = claim::prepare_claim(&sel, &request.transition, &request.context, vk).unwrap_or_else(|e| panic!("{}: honest claim: {e}", rel.key()));
    let pa = fixed::prove_assignment(&sel, &honest, Tamper::none(), false).unwrap_or_else(|e| panic!("{}: honest assignment: {e}", rel.key()));
    assert_eq!(pa.overrides_used, 0);
    assert_eq!(pa.full_assignment.len(), fixed.width());
    if let Some(kinds) = rg["effects"].as_array() {
        let want: Vec<&str> = kinds.iter().map(|k| k.as_str().unwrap()).collect();
        assert_eq!(effect_kinds(&sel), want, "{}: effect order", rel.key());
    }
    Fx { rel, gold: rg, sel, fixed, request, vk, honest, z: pa.full_assignment }
}

fn effect_kinds(sel: &SelectedRelation) -> Vec<&'static str> {
    sel.relation()
        .plan()
        .effects
        .iter()
        .map(|e| match e {
            EffectShape::State { .. } => "State",
            EffectShape::Transfer { .. } => "Transfer",
            EffectShape::Fee { .. } => "Fee",
            EffectShape::Liability { .. } => "Liability",
            EffectShape::Emit { .. } => "Emit",
        })
        .collect()
}

// ## Plan indices by name (never by assumed position)

fn input_ix(f: &Fx, name: &str) -> usize {
    f.sel.relation().plan().inputs.iter().position(|i| i.name == name).unwrap_or_else(|| panic!("input {name}"))
}

fn state_ix(f: &Fx, name: &str) -> usize {
    f.sel.relation().plan().state_index(name).unwrap_or_else(|| panic!("state {name}"))
}

fn cell_ix(f: &Fx, asset: &str, account: &str) -> usize {
    f.sel.relation().plan().cells.iter().position(|c| c.asset_name == asset && c.account == account).unwrap_or_else(|| panic!("cell {asset}.{account}"))
}

fn duty_ix(f: &Fx, name: &str) -> usize {
    f.sel.relation().plan().duty_index(name).unwrap_or_else(|| panic!("duty {name}"))
}

// ## Value edits

fn u(n: u128) -> Value {
    Value::UInt(UInt(n))
}

fn b(x: bool) -> Value {
    Value::Bool(x)
}

fn absent() -> DutyValue {
    DutyValue { present: false, amount: UInt(0) }
}

fn present_zero() -> DutyValue {
    DutyValue { present: true, amount: UInt(0) }
}

fn with(f: &Fx, edit: impl FnOnce(&mut TransitionValues)) -> TransitionValues {
    let mut v = f.honest.values.clone();
    edit(&mut v);
    v
}

fn amount(e: &mut EffectValues) -> &mut UInt {
    match e {
        EffectValues::Amount(a) => a,
        other => panic!("not an amount effect: {other:?}"),
    }
}

fn duty_before(e: &mut EffectValues) -> &mut UInt {
    match e {
        EffectValues::Duty { before, .. } => before,
        other => panic!("not a duty effect: {other:?}"),
    }
}

fn duty_after(e: &mut EffectValues) -> &mut UInt {
    match e {
        EffectValues::Duty { after, .. } => after,
        other => panic!("not a duty effect: {other:?}"),
    }
}

fn state_after(e: &mut EffectValues) -> &mut Value {
    match e {
        EffectValues::State { after, .. } => after,
        other => panic!("not a state effect: {other:?}"),
    }
}

/// A Core-proposed claim of the fixture's request with some scenario inputs replaced.
fn core_claim(f: &Fx, inputs: &[(&str, Value)]) -> PreparedClaim {
    let mut req = f.request.clone();
    let TransitionInput::CoreReference { scenario } = &mut req.transition else { panic!("CoreReference") };
    for (name, value) in inputs {
        scenario.inputs.insert((*name).into(), value.clone());
    }
    claim::prepare_claim(&f.sel, &req.transition, &req.context, f.vk).unwrap_or_else(|e| panic!("Core proposal refused: {e}"))
}

// ## Assignment edits

fn edit(f: &Fx, changes: &[(usize, Fr)]) -> Vec<Fr> {
    let mut z = f.z.clone();
    for &(i, v) in changes {
        z[i] = v;
    }
    z
}

fn flipped(f: &Fx, i: usize) -> Vec<Fr> {
    assert!(f.z[i].is_zero() || f.z[i].is_one(), "{}: entry {i} is not a bit in the honest assignment", f.rel.key());
    edit(f, &[(i, Fr::one() - f.z[i])])
}

fn bit(x: u128, w: u32) -> Fr {
    if (x >> w) & 1 == 1 { Fr::one() } else { Fr::zero() }
}

fn sentinel(k: usize) -> Fr {
    Fr::from(1u128 << 120) * Fr::from(1u128 << 120) + Fr::from(k as u64)
}

/// Assignment index of each labelled witness: one tampered synthesis gives every label a
/// distinct sentinel; the only entries that differ from the honest `z` are those witnesses.
fn locate(f: &Fx, labels: &[String]) -> BTreeMap<String, usize> {
    let order: Vec<String> = labels.iter().cloned().collect::<BTreeSet<String>>().into_iter().collect();
    let mut tamper = Tamper::none();
    for (k, l) in order.iter().enumerate() {
        tamper = tamper.set(l.clone(), sentinel(k));
    }
    let pa = fixed::prove_assignment(&f.sel, &f.honest, tamper, false).expect("tampered synthesis");
    assert_eq!(pa.overrides_used, order.len(), "{}: every label names an allocated witness", f.rel.key());
    let mut out = BTreeMap::new();
    for (i, (a, h)) in pa.full_assignment.iter().zip(&f.z).enumerate() {
        if a != h {
            let k = (0..order.len()).find(|&k| *a == sentinel(k)).unwrap_or_else(|| panic!("entry {i} changed without a label"));
            assert!(out.insert(order[k].clone(), i).is_none(), "label {} located twice", order[k]);
        }
    }
    assert_eq!(out.len(), order.len(), "{}: every label located", f.rel.key());
    out
}

// ## Structural rows of the fixed matrices

/// The row `x_k - sum(2^w d_w) = 0` binding public input `k` to digest-output bits, and those
/// bits as (index, weight).
fn public_row(f: &Fx, k: usize) -> (usize, Vec<(usize, u32)>) {
    let m = f.fixed.matrices();
    let one = Fr::one();
    let rows: Vec<usize> = (0..m.num_constraints).filter(|&r| m.a[r].iter().any(|(c, j)| *j == k && *c == one)).collect();
    assert_eq!(rows.len(), 1, "{}: one public-input row for instance {k}", f.rel.key());
    let r = rows[0];
    assert_eq!(m.b[r], vec![(one, 0usize)], "public-input row B is the constant one");
    assert!(m.c[r].is_empty(), "public-input row C is empty");
    let pow: Vec<Fr> = (0..128u32).map(|w| Fr::from(1u128 << w)).collect();
    let mut bits = Vec::new();
    for (c, j) in &m.a[r] {
        if *j == k {
            continue;
        }
        let w = (0..128usize).find(|&w| *c == -pow[w]).unwrap_or_else(|| panic!("row {r}: coefficient is not -2^w"));
        bits.push((*j, w as u32));
    }
    let weights: BTreeSet<u32> = bits.iter().map(|x| x.1).collect();
    assert_eq!((bits.len(), weights.len()), (128, 128), "128 digest bits of distinct weight");
    (r, bits)
}

/// The row `v * v = v`.
fn booleanity_row(f: &Fx, v: usize) -> usize {
    let m = f.fixed.matrices();
    let single = |row: &Vec<(Fr, usize)>| row.len() == 1 && row[0] == (Fr::one(), v);
    (0..m.num_constraints).find(|&r| single(&m.a[r]) && single(&m.b[r]) && single(&m.c[r])).unwrap_or_else(|| panic!("no booleanity row for {v}"))
}

/// The row `(1 - present) * amount = 0`.
fn canonicality_row(f: &Fx, present: usize) -> usize {
    let m = f.fixed.matrices();
    let one = Fr::one();
    (0..m.num_constraints)
        .find(|&r| m.c[r].is_empty() && m.a[r].len() == 2 && m.a[r].contains(&(one, 0)) && m.a[r].contains(&(-one, present)))
        .unwrap_or_else(|| panic!("no canonicality row for {present}"))
}

// ## Case runner

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Outcome {
    Sat,
    Unsat,
    HostRefused,
}

impl Outcome {
    fn parse(s: &str) -> Self {
        match s {
            "SAT" => Self::Sat,
            "UNSAT" => Self::Unsat,
            "HOST_REFUSED" => Self::HostRefused,
            other => panic!("unknown expected outcome {other}"),
        }
    }
    fn name(self) -> &'static str {
        match self {
            Self::Sat => "SAT",
            Self::Unsat => "UNSAT",
            Self::HostRefused => "HOST_REFUSED",
        }
    }
}

struct Run<'a> {
    f: &'a Fx,
    expected: BTreeMap<String, Outcome>,
    seen: BTreeSet<String>,
    failures: Vec<String>,
}

impl<'a> Run<'a> {
    fn new(f: &'a Fx) -> Self {
        let g = gold();
        let expected = g["hostile"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|h| h["relation"] == f.rel.key())
            .map(|h| (h["id"].as_str().unwrap().to_string(), Outcome::parse(h["expected"].as_str().unwrap())))
            .collect();
        Self { f, expected, seen: BTreeSet::new(), failures: Vec::new() }
    }

    fn want(&mut self, id: &str) -> Outcome {
        assert!(self.seen.insert(id.to_string()), "case {id} run twice");
        *self.expected.get(id).unwrap_or_else(|| panic!("case {id} is not pre-registered in the gold"))
    }

    /// A claimed transition in a context; `keep_public` keeps the honest public inputs.
    fn sem(&mut self, id: &str, values: TransitionValues, ctx: ContextV2, keep_public: bool) {
        let want = self.want(id);
        let f = self.f;
        let got = match claim::claim_from_values(&f.sel, values, ctx) {
            Err(DriverError::Proposal(e)) => {
                println!("K4H-CASE {} {id} expected={} observed=HOST_REFUSED reason={e}", f.rel.key(), want.name());
                Outcome::HostRefused
            }
            Err(e) => panic!("{id}: unexpected refusal {e}"),
            Ok(c) => {
                let pa = fixed::prove_assignment(&f.sel, &c, Tamper::none(), true).unwrap_or_else(|e| panic!("{id}: synthesis {e}"));
                assert_eq!(pa.overrides_used, 0, "{id}");
                let same = pa.matrix_digest_hex.as_deref() == Some(f.fixed.digest_hex());
                let mut z = pa.full_assignment;
                if keep_public {
                    z[1] = f.z[1];
                    z[2] = f.z[2];
                }
                let ev = f.fixed.evaluate(&z).unwrap_or_else(|e| panic!("{id}: {e}"));
                let o = if ev.satisfied() { Outcome::Sat } else { Outcome::Unsat };
                println!(
                    "K4H-CASE {} {id} expected={} observed={} violated={} firstViolated={:?} proveMatricesEqualFixed={same}",
                    f.rel.key(),
                    want.name(),
                    o.name(),
                    ev.violated,
                    ev.first_violated
                );
                if !same {
                    self.failures.push(format!("{id}: prove-mode matrices of a dishonest claim differ from the fixed matrices"));
                }
                o
            }
        };
        if got != want {
            self.failures.push(format!("{id}: expected {} observed {}", want.name(), got.name()));
        }
    }

    /// A raw assignment; `rows` lists rows whose satisfied/violated state is also predicted.
    fn raw(&mut self, id: &str, z: Vec<Fr>, rows: &[(usize, bool)]) {
        let want = self.want(id);
        let f = self.f;
        let ev = f.fixed.evaluate(&z).unwrap_or_else(|e| panic!("{id}: {e}"));
        let got = if ev.satisfied() { Outcome::Sat } else { Outcome::Unsat };
        let row_misses: Vec<usize> = rows.iter().filter(|(r, sat)| f.fixed.row_satisfied(*r, &z) != *sat).map(|(r, _)| *r).collect();
        println!(
            "K4H-CASE {} {id} expected={} observed={} violated={} firstViolated={:?} rowPredictionsMissed={row_misses:?}",
            f.rel.key(),
            want.name(),
            got.name(),
            ev.violated,
            ev.first_violated
        );
        if got != want {
            self.failures.push(format!("{id}: expected {} observed {}", want.name(), got.name()));
        }
        if !row_misses.is_empty() {
            self.failures.push(format!("{id}: row predictions missed at {row_misses:?}"));
        }
    }

    fn finish(self, prefix: &str) {
        let registered: BTreeSet<String> = self.expected.keys().filter(|k| k.starts_with(prefix)).cloned().collect();
        let run: BTreeSet<String> = self.seen.iter().filter(|k| k.starts_with(prefix)).cloned().collect();
        assert_eq!(run, registered, "{prefix}: the cases run must be exactly the pre-registered ones");
        assert!(self.failures.is_empty(), "{prefix} failures:\n{}", self.failures.join("\n"));
    }
}

// ## Financial check over decoded slots (gold-driven)

fn check_financial(rg: &Json, slots: &[inspect::SlotValue]) -> Vec<String> {
    let val: BTreeMap<&str, &Json> = slots.iter().map(|s| (s.label.as_str(), &s.value)).collect();
    let num = |label: &Json| -> i128 { val[label.as_str().unwrap()].as_str().unwrap().parse::<i128>().unwrap() };
    let flag = |label: &Json| -> bool { val[label.as_str().unwrap()].as_bool().unwrap() };
    let dec = |j: &Json| -> i128 { j.as_str().unwrap().parse::<i128>().unwrap() };
    let fin = &rg["financial"];
    let mut problems = Vec::new();
    let mut cells: BTreeMap<String, (i128, i128)> = BTreeMap::new();
    for c in fin["cells"].as_array().unwrap() {
        let (pre, post) = (num(&c["pre"]), num(&c["post"]));
        if post - pre != dec(&c["net"]) {
            problems.push(format!("cell {}: net {} != gold {}", c["cell"], post - pre, c["net"]));
        }
        cells.insert(c["cell"].as_str().unwrap().into(), (pre, post));
    }
    for d in fin["debits"].as_array().unwrap() {
        let sum = |key: &str| -> i128 { d[key].as_array().unwrap().iter().map(&num).sum() };
        let (t, fe) = (sum("transferSlots"), sum("feeSlots"));
        if t + fe != dec(&d["gross"]) || num(&d["grossSlot"]) != t + fe {
            problems.push(format!("debit {}: gross slot {} effects {} gold {}", d["cell"], num(&d["grossSlot"]), t + fe, d["gross"]));
        }
        if fe != dec(&d["fees"]) || num(&d["feesSlot"]) != fe {
            problems.push(format!("debit {}: fees slot {} effects {} gold {}", d["cell"], num(&d["feesSlot"]), fe, d["fees"]));
        }
    }
    for c in fin["conservation"].as_array().unwrap() {
        let members: Vec<&str> = c["cells"].as_array().unwrap().iter().map(|x| x.as_str().unwrap()).collect();
        let pre: i128 = members.iter().map(|m| cells[*m].0).sum();
        let post: i128 = members.iter().map(|m| cells[*m].1).sum();
        if pre != dec(&c["preTotal"]) || post != dec(&c["postTotal"]) {
            problems.push(format!("conservation {}: {pre} -> {post}", c["asset"]));
        }
    }
    for l in fin["liabilities"].as_array().unwrap() {
        let open_present = flag(&l["openingPresentSlot"]);
        let opening = if open_present { num(&l["openingSlot"]) } else { 0 };
        let pairs = |key: &str| -> Vec<(i128, i128)> {
            l[key].as_array().unwrap().iter().map(|p| (num(&p[0]), num(&p[1]))).collect()
        };
        let accrued: i128 = pairs("accrualSlots").iter().map(|(b0, a0)| a0 - b0).sum();
        let discharged: i128 = pairs("dischargeSlots").iter().map(|(b0, a0)| b0 - a0).sum();
        let closing = opening + accrued - discharged;
        let ok = open_present == l["openingPresent"].as_bool().unwrap()
            && opening == dec(&l["opening"])
            && accrued == dec(&l["accrued"])
            && discharged == dec(&l["discharged"])
            && closing == dec(&l["closing"])
            && closing == num(&l["closingSlot"])
            && flag(&l["closingPresentSlot"]) == l["closingPresent"].as_bool().unwrap();
        if !ok {
            problems.push(format!("liability {}: opening {opening} accrued {accrued} discharged {discharged} closing {closing}", l["duty"]));
        }
    }
    problems
}

// ## Tests

#[test]
fn h01_fixed_matrices_equal_stage_c_and_honest_claims_satisfy() {
    for rel in [Rel::Partial, Rel::Allocate] {
        let f = fixture(rel);
        let g = &f.gold;
        let m = f.fixed.matrices();
        assert_eq!(f.fixed.digest_hex(), g["r1csSha256"].as_str().unwrap(), "{}: stage-C r1csSha256", rel.key());
        assert_eq!(f.sel.descriptor().predicate_digest, g["predicateDigest"].as_str().unwrap());
        assert_eq!(f.sel.descriptor().abi2_shape_digest, g["abi2ShapeDigest"].as_str().unwrap());
        assert_eq!(f.sel.descriptor().program_hash, g["programHash"].as_str().unwrap());
        assert_eq!(m.num_constraints as u64, g["constraints"].as_u64().unwrap());
        assert_eq!(m.num_witness_variables as u64, g["witnessVariables"].as_u64().unwrap());
        assert_eq!(m.num_instance_variables as u64, g["instanceVariables"].as_u64().unwrap());
        assert_eq!(f.honest.statement.len() as u64, g["statementLength"].as_u64().unwrap());
        assert_eq!(mori_r1cs::blake2b::block_count(f.honest.statement.len()) as u64, g["blake2bBlocks"].as_u64().unwrap());
        let ev = f.fixed.evaluate(&f.z).unwrap();
        println!("K4H-FIXED {} r1cs={} constraints={} witnesses={} honestViolated={}", rel.key(), f.fixed.digest_hex(), m.num_constraints, m.num_witness_variables, ev.violated);
        assert!(ev.satisfied(), "{}: honest claim violates {} fixed rows", rel.key(), ev.violated);
    }
}

#[test]
fn h02_inspect_receipts_match_independent_gold() {
    for rel in [Rel::Partial, Rel::Allocate] {
        let f = fixture(rel);
        let g = &f.gold;
        let r = inspect::inspect(&f.sel, &f.request, f.vk).unwrap_or_else(|e| panic!("{}: receipt: {e}", rel.key()));
        let want = g["slots"].as_array().unwrap();
        let got: Vec<(String, &str, Json)> = r.slots.iter().map(|s| (s.label.clone(), s.encoding, s.value.clone())).collect();
        let gold_slots: Vec<(String, &str, Json)> =
            want.iter().map(|s| (s["label"].as_str().unwrap().to_string(), s["encoding"].as_str().unwrap(), s["value"].clone())).collect();
        assert_eq!(got, gold_slots, "{}: decoded statement body differs from the gold", rel.key());
        assert_eq!(r.statement_len as u64, g["statementLength"].as_u64().unwrap());
        assert_eq!(r.r1cs_sha256, g["r1csSha256"].as_str().unwrap());
        assert!(r.fixed_matrix.equals_descriptor && r.fixed_matrix.prove_matrix_digest_equal);
        assert_eq!(r.fixed_matrix.violated, 0, "{}: honest receipt claim violates fixed rows", rel.key());
        // Header and context bytes, read here without the driver's parser.
        let s = unhex(&r.statement_hex);
        let c = &f.request.context;
        assert_eq!(&s[0..16], b"MRG16-PVC-STMT-2");
        assert_eq!(&s[16..48], b"MRG16-PVC-FAM2-G16-BLS-ARK04-HL2");
        assert_eq!(hex(&s[48..80]), g["predicateDigest"].as_str().unwrap());
        assert_eq!(hex(&s[80..112]), g["abi2ShapeDigest"].as_str().unwrap());
        assert_eq!(hex(&s[112..144]), g["declaredVkDigest"].as_str().unwrap());
        assert_eq!(hex(&s[144..172]), c.claimant_vkh);
        assert_eq!(s[172], c.network_id);
        assert_eq!(hex(&s[173..201]), c.settle_hash);
        assert_eq!(hex(&s[201..233]), c.thread_id);
        assert_eq!(hex(&s[233..265]), c.consumed_txid);
        assert_eq!(u16::from_be_bytes([s[265], s[266]]), c.consumed_ix);
        assert_eq!(u64::from_be_bytes(s[267..275].try_into().unwrap()), c.seq);
        assert_eq!(hex(&s[275..307]), c.catalogue_digest);
        assert_eq!(hex(&s[307..339]), c.intents_digest);
        assert_eq!(u32::from_be_bytes(s[339..343].try_into().unwrap()) as u64, g["bodyLen"].as_u64().unwrap());
        let problems = check_financial(g, &r.slots);
        println!(
            "K4H-RECEIPT {} statementLen={} digest={} x1={} x2={} violated={} financialProblems={problems:?}",
            rel.key(),
            r.statement_len,
            r.statement_digest_blake2b256,
            r.x1,
            r.x2,
            r.fixed_matrix.violated
        );
        assert!(problems.is_empty(), "{}: {problems:?}", rel.key());
    }
}

#[test]
fn h03_invoice_partial_semantic_mutations() {
    let f = fixture(Rel::Partial);
    let amt = input_ix(&f, "amount");
    let (marker, opened, paid) = (state_ix(&f, "marker"), state_ix(&f, "opened"), state_ix(&f, "paid"));
    let (escrow, fees, seller) = (cell_ix(&f, "CASH", "escrow"), cell_ix(&f, "CASH", "fees"), cell_ix(&f, "CASH", "seller"));
    let (inv, res) = (duty_ix(&f, "invoice"), duty_ix(&f, "reserve"));
    let ctx = f.honest.context.clone();
    let hi = 1u128 << 64;
    let mut r = Run::new(&f);
    r.sem("IA01", f.honest.values.clone(), ctx.clone(), false);
    r.sem("IA02", with(&f, |v| v.inputs[amt] = u(36)), ctx.clone(), false);
    r.sem("IA03", with(&f, |v| v.inputs[amt] = u(35 + hi)), ctx.clone(), false);
    r.sem("IA04", with(&f, |v| v.pre.states[marker] = u(8)), ctx.clone(), false);
    r.sem("IA05", with(&f, |v| v.pre.states[opened] = b(false)), ctx.clone(), false);
    r.sem("IA06", with(&f, |v| v.pre.states[paid] = u(1)), ctx.clone(), false);
    r.sem("IA07", with(&f, |v| v.pre.cells[escrow] = UInt(110)), ctx.clone(), false);
    r.sem("IA08", with(&f, |v| v.pre.cells[escrow] = UInt(109 + hi)), ctx.clone(), false);
    r.sem("IA09", with(&f, |v| v.pre.cells[fees] = UInt(1)), ctx.clone(), false);
    r.sem("IA10", with(&f, |v| v.pre.cells[seller] = UInt(1)), ctx.clone(), false);
    r.sem("IA11", with(&f, |v| v.pre.duties[inv].amount = UInt(91)), ctx.clone(), false);
    r.sem("IA12", with(&f, |v| v.pre.duties[res].amount = UInt(15)), ctx.clone(), false);
    r.sem("IA13", with(&f, |v| v.pre.duties[res] = absent()), ctx.clone(), false);
    r.sem("IA14", with(&f, |v| v.pre.duties[inv] = absent()), ctx.clone(), false);
    r.sem("IA15", with(&f, |v| v.post.states[paid] = u(36)), ctx.clone(), false);
    r.sem("IA16", with(&f, |v| v.post.states[opened] = b(false)), ctx.clone(), false);
    r.sem("IA17", with(&f, |v| v.post.cells[escrow] = UInt(73)), ctx.clone(), false);
    r.sem("IA18", with(&f, |v| v.post.cells[seller] = UInt(36)), ctx.clone(), false);
    r.sem("IA19", with(&f, |v| v.post.cells[fees] = UInt(3)), ctx.clone(), false);
    r.sem("IA20", with(&f, |v| v.post.duties[inv].amount = UInt(54)), ctx.clone(), false);
    r.sem("IA21", with(&f, |v| v.post.duties[inv] = absent()), ctx.clone(), false);
    r.sem("IA22", with(&f, |v| v.post.duties[res] = absent()), ctx.clone(), false);
    r.sem("IA23", with(&f, |v| *amount(&mut v.effects[0]) = UInt(3)), ctx.clone(), false);
    r.sem("IA24", with(&f, |v| *amount(&mut v.effects[1]) = UInt(36)), ctx.clone(), false);
    r.sem("IA25", with(&f, |v| *duty_after(&mut v.effects[2]) = UInt(54)), ctx.clone(), false);
    r.sem("IA26", with(&f, |v| *state_after(&mut v.effects[3]) = u(36)), ctx.clone(), false);
    r.sem(
        "IA27",
        with(&f, |v| {
            *amount(&mut v.effects[0]) = UInt(3);
            v.post.cells[escrow] = UInt(71);
            v.post.cells[fees] = UInt(3);
        }),
        ctx.clone(),
        false,
    );
    r.sem(
        "IA28",
        with(&f, |v| {
            *amount(&mut v.effects[1]) = UInt(36);
            v.post.cells[escrow] = UInt(71);
            v.post.cells[seller] = UInt(36);
            *duty_after(&mut v.effects[2]) = UInt(54);
            v.post.duties[inv].amount = UInt(54);
            v.post.states[paid] = u(36);
            *state_after(&mut v.effects[3]) = u(36);
        }),
        ctx.clone(),
        false,
    );
    let alt = core_claim(&f, &[("amount", u(36))]);
    r.sem("IA29", alt.values, alt.context, false);
    r.sem(
        "IA30",
        with(&f, |v| {
            v.pre.cells[escrow] = UInt(110);
            v.post.cells[escrow] = UInt(73);
        }),
        ctx.clone(),
        false,
    );
    let mut seq2 = ctx.clone();
    seq2.seq = 2;
    r.sem("IA31", f.honest.values.clone(), seq2.clone(), false);
    r.sem("IA32", f.honest.values.clone(), seq2, true);
    let mut vk2 = ctx.clone();
    vk2.vk_digest = [0x5a; 32];
    r.sem("IA33", f.honest.values.clone(), vk2, false);
    r.sem("IA34", with(&f, |v| *duty_before(&mut v.effects[2]) = UInt(91)), ctx, false);
    r.finish("IA");
}

#[test]
fn h04_reserve_allocate_semantic_mutations() {
    let f = fixture(Rel::Allocate);
    let (approved, units, quotient, remainder) = (input_ix(&f, "approved"), input_ix(&f, "units"), input_ix(&f, "quotient"), input_ix(&f, "remainder"));
    let (allocated, marker, rounds) = (state_ix(&f, "allocated"), state_ix(&f, "marker"), state_ix(&f, "rounds"));
    let (a_alpha, a_fees, a_vault) = (cell_ix(&f, "A", "alpha"), cell_ix(&f, "A", "fees"), cell_ix(&f, "A", "vault"));
    let (b_alpha, b_vault) = (cell_ix(&f, "B", "alpha"), cell_ix(&f, "B", "vault"));
    let (ar, rebate) = (duty_ix(&f, "allocation_reserve"), duty_ix(&f, "rebate"));
    let ctx = f.honest.context.clone();
    let hi = 1u128 << 64;
    let mut r = Run::new(&f);
    r.sem("RA01", f.honest.values.clone(), ctx.clone(), false);
    r.sem("RA02", with(&f, |v| v.inputs[approved] = b(false)), ctx.clone(), false);
    r.sem("RA03", with(&f, |v| v.inputs[units] = u(24)), ctx.clone(), false);
    r.sem("RA04", with(&f, |v| v.inputs[remainder] = u(3)), ctx.clone(), false);
    r.sem("RA05", with(&f, |v| v.inputs[quotient] = u(8)), ctx.clone(), false);
    r.sem("RA06", with(&f, |v| v.pre.states[allocated] = b(true)), ctx.clone(), false);
    r.sem("RA07", with(&f, |v| v.pre.states[rounds] = u(1)), ctx.clone(), false);
    r.sem("RA08", with(&f, |v| v.pre.states[marker] = u(78)), ctx.clone(), false);
    r.sem("RA09", with(&f, |v| v.pre.cells[a_vault] = UInt(41)), ctx.clone(), false);
    r.sem("RA10", with(&f, |v| v.pre.cells[b_vault] = UInt(30 + hi)), ctx.clone(), false);
    r.sem("RA11", with(&f, |v| v.pre.cells[a_fees] = UInt(6)), ctx.clone(), false);
    r.sem("RA12", with(&f, |v| v.pre.duties[ar] = present_zero()), ctx.clone(), false);
    r.sem("RA13", with(&f, |v| v.pre.duties[rebate] = present_zero()), ctx.clone(), false);
    r.sem(
        "RA14",
        with(&f, |v| {
            v.pre.duties[ar] = present_zero();
            v.pre.duties[rebate] = present_zero();
        }),
        ctx.clone(),
        false,
    );
    r.sem("RA15", with(&f, |v| v.pre.duties[ar] = DutyValue { present: true, amount: UInt(1) }), ctx.clone(), false);
    r.sem("RA16", with(&f, |v| v.pre.duties[ar] = DutyValue { present: false, amount: UInt(1) }), ctx.clone(), false);
    r.sem("RA17", with(&f, |v| v.post.cells[a_alpha] = UInt(15)), ctx.clone(), false);
    r.sem("RA18", with(&f, |v| v.post.cells[b_alpha] = UInt(5)), ctx.clone(), false);
    r.sem("RA19", with(&f, |v| v.post.cells[a_vault] = UInt(16)), ctx.clone(), false);
    r.sem("RA20", with(&f, |v| v.post.duties[rebate].amount = UInt(7)), ctx.clone(), false);
    r.sem("RA21", with(&f, |v| v.post.duties[ar] = absent()), ctx.clone(), false);
    r.sem("RA22", with(&f, |v| v.post.states[rounds] = u(2)), ctx.clone(), false);
    r.sem("RA23", with(&f, |v| *amount(&mut v.effects[1]) = UInt(15)), ctx.clone(), false);
    r.sem("RA24", with(&f, |v| *amount(&mut v.effects[3]) = UInt(5)), ctx.clone(), false);
    r.sem("RA25", with(&f, |v| *duty_after(&mut v.effects[5]) = UInt(7)), ctx.clone(), false);
    r.sem("RA26", with(&f, |v| *state_after(&mut v.effects[6]) = b(false)), ctx.clone(), false);
    r.sem(
        "RA27",
        with(&f, |v| {
            *amount(&mut v.effects[3]) = UInt(5);
            v.post.cells[b_vault] = UInt(25);
            v.post.cells[b_alpha] = UInt(5);
        }),
        ctx.clone(),
        false,
    );
    let alt = core_claim(&f, &[("quotient", u(8)), ("units", u(26))]);
    r.sem("RA28", alt.values, alt.context, false);
    r.sem(
        "RA29",
        with(&f, |v| {
            *duty_after(&mut v.effects[4]) = UInt(1);
            v.post.duties[ar].amount = UInt(1);
        }),
        ctx.clone(),
        false,
    );
    r.sem(
        "RA30",
        with(&f, |v| {
            v.pre.cells[a_vault] = UInt(41);
            v.post.cells[a_vault] = UInt(18);
        }),
        ctx.clone(),
        false,
    );
    let mut claimant = ctx.clone();
    claimant.claimant_vkh = [0x56; 28];
    r.sem("RA31", f.honest.values.clone(), claimant.clone(), false);
    r.sem("RA32", f.honest.values.clone(), claimant, true);
    r.finish("RA");
}

#[test]
fn h05_invoice_open_addendum_e_t1() {
    let f = fixture(Rel::Open);
    let (inv, res) = (duty_ix(&f, "invoice"), duty_ix(&f, "reserve"));
    let ctx = f.honest.context.clone();
    println!("K4H-OPEN r1cs={} (not adopted; not pinned by stage C)", f.fixed.digest_hex());
    let mut r = Run::new(&f);
    r.sem("OA01", f.honest.values.clone(), ctx.clone(), false);
    r.sem("OA02", with(&f, |v| v.pre.duties[res] = present_zero()), ctx.clone(), false);
    r.sem("OA03", with(&f, |v| v.pre.duties[inv] = present_zero()), ctx, false);
    r.finish("OA");
}

/// Witnesses one raw suite targets, by compiler label.
struct RawTargets {
    bool_label: String,
    carry_lo: String,
    carry_hi: String,
    canon_label: String,
    canon_value: Fr,
    canon_present: String,
}

const HASH_LABELS: [(u32, &str); 8] = [
    (12, "abi2/abi2.ctx#0"),
    (13, "abi2/abi2.ctx#1815"),
    (14, "abi2/abi2.body#0"),
    (15, "abi2/blake2b/b2.xor#0"),
    (16, "abi2/blake2b/b2.xor#50000"),
    (17, "abi2/blake2b/b2.sum#0"),
    (18, "abi2/blake2b/b2.sum#65"),
    (19, "abi2/blake2b/b2.sum#60000"),
];

fn raw_suite(f: &Fx, p: &str, t: RawTargets) {
    let mut labels = vec![t.bool_label.clone(), t.carry_lo.clone(), t.carry_hi.clone(), t.canon_label.clone(), t.canon_present.clone()];
    labels.extend(HASH_LABELS.iter().map(|(_, l)| l.to_string()));
    let ix = locate(f, &labels);
    let at = |l: &str| ix[l];
    let (r1, bits1) = public_row(f, 1);
    let (r2, bits2) = public_row(f, 2);
    let id = |n: u32| format!("{p}{n:02}");
    let mut run = Run::new(f);

    let mut zero = vec![Fr::zero(); f.z.len()];
    zero[0] = Fr::one();
    run.raw(&id(1), zero, &[]);
    run.raw(&id(2), edit(f, &[(1, f.z[1] + Fr::one())]), &[(r1, false)]);
    run.raw(&id(3), edit(f, &[(2, f.z[2] + Fr::one())]), &[(r2, false)]);
    run.raw(&id(4), edit(f, &[(1, f.z[2]), (2, f.z[1])]), &[]);
    run.raw(&id(5), edit(f, &[(1, Fr::from(f.honest.public_inputs[0] ^ (1u128 << 127)))]), &[(r1, false)]);

    let mut foreign_ctx = f.honest.context.clone();
    foreign_ctx.seq += 1;
    let foreign = claim::claim_from_values(&f.sel, f.honest.values.clone(), foreign_ctx).expect("foreign statement");
    assert_eq!(foreign.statement.len(), f.honest.statement.len());
    assert_ne!(foreign.public_inputs, f.honest.public_inputs);
    let [fx1, fx2] = foreign.public_inputs;
    let forged = edit(f, &[(1, Fr::from(fx1)), (2, Fr::from(fx2))]);
    run.raw(&id(6), forged.clone(), &[(r1, false), (r2, false)]);
    let mut forced = forged;
    for &(v, w) in &bits1 {
        forced[v] = bit(fx1, w);
    }
    for &(v, w) in &bits2 {
        forced[v] = bit(fx2, w);
    }
    run.raw(&id(7), forced, &[(r1, true), (r2, true)]);
    let low = bits2.iter().find(|x| x.1 == 0).unwrap().0;
    run.raw(&id(8), flipped(f, low), &[(r2, false)]);

    let bv = at(&t.bool_label);
    run.raw(&id(9), edit(f, &[(bv, Fr::from(2u64))]), &[(booleanity_row(f, bv), false)]);
    let (lo, hi) = (at(&t.carry_lo), at(&t.carry_hi));
    assert!(f.z[lo].is_zero() && f.z[hi].is_zero(), "carry targets are zero bits in the honest claim");
    run.raw(&id(10), edit(f, &[(lo, Fr::from(2u64)), (hi, -Fr::one())]), &[(booleanity_row(f, lo), false), (booleanity_row(f, hi), false)]);
    let (cv, pv) = (at(&t.canon_label), at(&t.canon_present));
    assert_ne!(f.z[cv], t.canon_value, "canonicality edit changes the honest value");
    run.raw(&id(11), edit(f, &[(cv, t.canon_value)]), &[(canonicality_row(f, pv), false)]);
    for (n, l) in HASH_LABELS {
        run.raw(&id(n), flipped(f, at(l)), &[]);
    }
    run.finish(p);
}

#[test]
fn h06_invoice_partial_raw_assignment_mutations() {
    let f = fixture(Rel::Partial);
    let reserve_present = format!("entry/duty{}.present#0", duty_ix(&f, "reserve"));
    let escrow = cell_ix(&f, "CASH", "escrow");
    let t = RawTargets {
        bool_label: format!("entry/state{}#0", state_ix(&f, "opened")),
        carry_lo: format!("entry/cell{escrow}.bit63#0"),
        carry_hi: format!("entry/cell{escrow}.bit64#0"),
        canon_label: reserve_present.clone(),
        canon_value: Fr::zero(),
        canon_present: reserve_present,
    };
    raw_suite(&f, "IB", t);
}

#[test]
fn h07_reserve_allocate_raw_assignment_mutations() {
    let f = fixture(Rel::Allocate);
    let ar = duty_ix(&f, "allocation_reserve");
    let vault = cell_ix(&f, "A", "vault");
    let t = RawTargets {
        bool_label: format!("entry/input{}#0", input_ix(&f, "approved")),
        carry_lo: format!("entry/cell{vault}.bit63#0"),
        carry_hi: format!("entry/cell{vault}.bit64#0"),
        canon_label: format!("entry/duty{ar}.amount.bit0#0"),
        canon_value: Fr::one(),
        canon_present: format!("entry/duty{ar}.present#0"),
    };
    raw_suite(&f, "RB", t);
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Sweep {
    checked: usize,
    skipped: usize,
    undetected: Vec<usize>,
    zero_multiplier_only: usize,
    unused: usize,
}

/// Perturbs every witness of the honest assignment once and re-decides only the fixed rows that
/// contain it (`flip`: 0 <-> 1 for bit-valued entries; otherwise `+1`).
fn sweep(f: &Fx, flip: bool) -> Sweep {
    let m = f.fixed.matrices();
    let n = m.num_constraints;
    let width = f.fixed.width();
    let z = &f.z;
    let mut az = Vec::with_capacity(n);
    let mut bz = Vec::with_capacity(n);
    for r in 0..n {
        let (a, b_, c) = f.fixed.row_values(r, z);
        assert!(a * b_ == c, "honest assignment violates row {r}");
        az.push(a);
        bz.push(b_);
    }
    let sides = [&m.a, &m.b, &m.c];
    let mut count = vec![0usize; width];
    for r in 0..n {
        for side in sides {
            for (_, j) in &side[r] {
                count[*j] += 1;
            }
        }
    }
    let mut start = vec![0usize; width + 1];
    for j in 0..width {
        start[j + 1] = start[j] + count[j];
    }
    let mut fill = start.clone();
    let mut occ = vec![(0u32, 0u8, Fr::zero()); start[width]];
    for r in 0..n {
        for (s, side) in sides.iter().enumerate() {
            for (k, j) in &side[r] {
                occ[fill[*j]] = (r as u32, s as u8, *k);
                fill[*j] += 1;
            }
        }
    }
    let mut out = Sweep::default();
    for v in m.num_instance_variables..width {
        let delta = if flip {
            if z[v].is_zero() {
                Fr::one()
            } else if z[v].is_one() {
                -Fr::one()
            } else {
                out.skipped += 1;
                continue;
            }
        } else {
            Fr::one()
        };
        out.checked += 1;
        let list = &occ[start[v]..start[v + 1]];
        if list.is_empty() {
            out.unused += 1;
            out.undetected.push(v);
            continue;
        }
        let (mut detected, mut zero_only, mut i) = (false, true, 0);
        while i < list.len() {
            let r = list[i].0;
            let (mut da, mut db, mut dc) = (Fr::zero(), Fr::zero(), Fr::zero());
            while i < list.len() && list[i].0 == r {
                let (_, s, k) = list[i];
                let d = k * delta;
                match s {
                    0 => da += d,
                    1 => db += d,
                    _ => dc += d,
                }
                i += 1;
            }
            let ru = r as usize;
            if !(az[ru] * db + da * bz[ru] + da * db - dc).is_zero() {
                detected = true;
                break;
            }
            let b_only = da.is_zero() && dc.is_zero() && az[ru].is_zero();
            let a_only = db.is_zero() && dc.is_zero() && bz[ru].is_zero();
            if !(b_only || a_only) {
                zero_only = false;
            }
        }
        if !detected {
            out.undetected.push(v);
            if zero_only {
                out.zero_multiplier_only += 1;
            }
        }
    }
    out
}

fn sweeps(rel: Rel) {
    let f = fixture(rel);
    let g = gold();
    let want = &g["sweeps"][rel.key()];
    let mut failures = Vec::new();
    for (mode, flip) in [("increment", false), ("flip", true)] {
        let s = sweep(&f, flip);
        println!(
            "K4H-SWEEP {} {mode} checked={} skipped={} undetected={} zeroMultiplierOnly={} unused={} undetectedIndices={:?}",
            rel.key(),
            s.checked,
            s.skipped,
            s.undetected.len(),
            s.zero_multiplier_only,
            s.unused,
            s.undetected.iter().take(32).collect::<Vec<_>>()
        );
        let w = &want[mode];
        let observed = (s.undetected.len() as u64, s.zero_multiplier_only as u64, s.unused as u64);
        let expected = (w["undetected"].as_u64().unwrap(), w["zeroMultiplierOnly"].as_u64().unwrap(), w["unused"].as_u64().unwrap());
        if observed != expected {
            failures.push(format!("{mode}: observed {observed:?} expected {expected:?}"));
        }
    }
    assert!(failures.is_empty(), "{}: {failures:?}", rel.key());
}

#[test]
fn h08_invoice_partial_single_variable_sweeps() {
    sweeps(Rel::Partial);
}

#[test]
fn h09_reserve_allocate_single_variable_sweeps() {
    sweeps(Rel::Allocate);
}

#[test]
fn h10_k5_setup_digest_binding_without_keys() {
    let sel = compile_selected(INVOICE, &request(REQ_INVOICE).selector).expect("compile");
    let digest = sel.descriptor().r1cs_sha256.clone();
    let setup_cs = || {
        let cs = ConstraintSystem::<Fr>::new_ref();
        cs.set_optimization_goal(OptimizationGoal::Constraints);
        cs.set_mode(SynthesisMode::Setup);
        cs
    };
    // As ark-groth16's generator does: synthesize, then finalize again.
    let observed = RefCell::new(None);
    let circuit = setup::DigestBound::new(sel.relation().circuit_v2::<Fr>(None, None, Tamper::none()).unwrap(), digest.clone(), &observed);
    let cs = setup_cs();
    circuit.generate_constraints(cs.clone()).expect("the descriptor digest is accepted");
    cs.finalize();
    let again = relation::matrix_digest(&cs.to_matrices().expect("matrices")).hex();
    println!("K4H-DIGEST-BOUND observed={:?} afterSecondFinalize={again}", observed.borrow());
    assert_eq!(observed.borrow().as_deref(), Some(digest.as_str()));
    assert_eq!(again, digest, "a second finalize leaves the matrices unchanged");
    // A different expected digest refuses before any key could exist.
    let observed = RefCell::new(None);
    let circuit = setup::DigestBound::new(sel.relation().circuit_v2::<Fr>(None, None, Tamper::none()).unwrap(), "00".repeat(32), &observed);
    assert!(circuit.generate_constraints(setup_cs()).is_err(), "a foreign digest is refused");
    assert_eq!(observed.borrow().as_deref(), Some(digest.as_str()), "the refusal names the matrices actually seen");
}
