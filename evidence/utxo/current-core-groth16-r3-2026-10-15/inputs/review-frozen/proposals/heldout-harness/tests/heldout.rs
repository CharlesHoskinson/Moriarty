//! PROPOSED, UNCOMPILED reviewer-authored held-out harness (claude-opus-5-5, non-author).
//!
//! Expected values come only from the reviewer's frozen HELDOUT-VECTORS.json
//! (freeze aggregate 7f6a4d1e…b53c, frozen before any candidate test, fixture or
//! source was read) and the post-freeze HOSTILE-CLAIMS-ADDENDUM.json. Nothing here
//! derives an expectation from the candidate or from Core.
//!
//! Two differentials per vector:
//! 1. actual Core (`mori_core::simulate[_metered]`) against the reviewer oracle values;
//! 2. the compiled relation against the same values (accepted => satisfiable;
//!    rejected => the permissive claim is unsatisfiable or not encodable).
//! Every outcome is printed; the test fails on the first unexpected result after
//! printing all rows, so one run yields the complete table.
//! Deliberately no glob imports (see review finding F-01).
use ark_bls12_381::Fr;
use ark_relations::r1cs::ConstraintSystem;
use mori_core::{Effect, Program, Scenario, Snapshot, UInt};
use mori_r1cs::{
    AccountingBinding, CapBinding, CellRef, CompileError, CompileOptions, CompiledRelation, CompilerBuild, Digest32,
    FrameCells, StatementAdapter, Tamper, TransitionValues,
};
use serde_json::Value as Json;
use std::collections::BTreeMap;

const VECTORS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../heldout/HELDOUT-VECTORS.json"));
const HOSTILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../HOSTILE-CLAIMS-ADDENDUM.json"));
const H1: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../heldout/programs/H1-settle.mori"));
const H2: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../heldout/programs/H2-accrual.mori"));
const H4: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../heldout/programs/H4-emit.mori"));
const H5: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../heldout/programs/H5-strict.mori"));
const H6: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../heldout/programs/H6-bounds.mori"));
const H7: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../heldout/programs/H7-staged.mori"));
const H3_JSON: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../heldout/programs/H3-dag.program.json"));
/// POST-FREEZE addendum 2 (absent-duty discharge control).
const H8: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../heldout-addendum-2/programs/H8-discharge.mori"));
const ADDENDUM2: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../heldout-addendum-2/HELDOUT-ADDENDUM-2.json"));

/// Frozen candidate identities (BUILD-MANIFEST.json and CORE-CLOSURE-PINS closure_sha256).
const BUILD_MANIFEST_SHA256: &str = "2c943b511bb6cb912889728ed2c5de90d4257939ded53b1d626b998edd3beb21";
const CORE_CLOSURE_SHA256: &str = "f47318a26a4df0477e89962a9de0e9b48f12d01d0e9b4a8f5e74603beaa235ed";

fn json(text: &str) -> Json {
    serde_json::from_str(text).expect("frozen reviewer JSON parses")
}

fn build() -> CompilerBuild {
    CompilerBuild {
        compiler_source_manifest_sha256: Digest32::from_hex(BUILD_MANIFEST_SHA256).expect("hex"),
        core_closure_sha256: Digest32::from_hex(CORE_CLOSURE_SHA256).expect("hex"),
    }
}

fn opts(action: &str, cap: bool, frame: FrameCells) -> CompileOptions {
    CompileOptions {
        action: action.to_string(),
        cap: if cap { CapBinding::PublicCap } else { CapBinding::NotBound },
        frame,
        accounting: AccountingBinding::NotBound,
        compiler: build(),
    }
}

fn h3_program() -> Program {
    serde_json::from_str(H3_JSON).expect("H3 decodes through the v1 wire")
}

fn relation(program: &str, action: &str, cap: bool, frame: FrameCells) -> Result<CompiledRelation, CompileError> {
    let source = match program {
        "H1-settle" => H1,
        "H2-accrual" => H2,
        "H4-emit" => H4,
        "H5-strict" => H5,
        "H6-bounds" => H6,
        "H8-discharge" => H8,
        "H3-dag" => {
            let admitted = mori_r1cs::admit_program(h3_program())?;
            return mori_r1cs::compile::<Fr>(admitted, opts(action, cap, frame), StatementAdapter::DirectFieldV0);
        }
        other => panic!("unknown held-out program {other}"),
    };
    mori_r1cs::compile_source::<Fr>(source, opts(action, cap, frame), StatementAdapter::DirectFieldV0)
}

fn core_program(program: &str) -> Program {
    match program {
        "H3-dag" => h3_program(),
        "H1-settle" => mori_core::parse(H1).expect("Core parses H1"),
        "H2-accrual" => mori_core::parse(H2).expect("Core parses H2"),
        "H4-emit" => mori_core::parse(H4).expect("Core parses H4"),
        "H5-strict" => mori_core::parse(H5).expect("Core parses H5"),
        "H6-bounds" => mori_core::parse(H6).expect("Core parses H6"),
        "H8-discharge" => mori_core::parse(H8).expect("Core parses H8"),
        other => panic!("unknown held-out program {other}"),
    }
}

fn cap_of(v: &Json) -> Option<UInt> {
    v["logical_cap"].as_str().map(|s| UInt(s.parse().expect("cap")))
}

fn satisfied(rel: &CompiledRelation, values: TransitionValues, tamper: Tamper<Fr>) -> Result<bool, String> {
    let assignment = rel.assignment::<Fr>(values).map_err(|e| format!("not encodable: {e}"))?;
    let cs = ConstraintSystem::<Fr>::new_ref();
    rel.circuit(Some(assignment), tamper)
        .map_err(|e| format!("circuit: {e}"))?
        .synthesize_into(cs.clone())
        .map_err(|e| format!("synthesis: {e}"))?;
    cs.is_satisfied().map_err(|e| format!("is_satisfied: {e}"))
}

fn values_from(rel: &CompiledRelation, scenario: &Scenario, pre: &Snapshot, post: &Snapshot, effects: &[Effect], cap: Option<UInt>) -> Result<TransitionValues, String> {
    mori_r1cs::witness::from_records(rel.plan(), &scenario.inputs, pre, post, effects, cap).map_err(|e| format!("not encodable: {e}"))
}

struct Table {
    rows: Vec<String>,
    unexpected: usize,
}

impl Table {
    fn row(&mut self, id: &str, ok: bool, detail: String) {
        if !ok {
            self.unexpected += 1;
        }
        self.rows.push(format!("{} {id}: {detail}", if ok { "OK  " } else { "FAIL" }));
    }
    fn finish(self, name: &str) {
        for r in &self.rows {
            println!("{name} {r}");
        }
        assert_eq!(self.unexpected, 0, "{name}: {} unexpected rows (see output)", self.unexpected);
    }
}

#[test]
fn heldout_vectors_core_and_relation() {
    let doc = json(VECTORS);
    let mut t = Table { rows: Vec::new(), unexpected: 0 };
    for v in doc["vectors"].as_array().expect("vectors") {
        let id = v["id"].as_str().expect("id");
        let program = v["program"].as_str().expect("program");
        let action = v["action"].as_str().expect("action");
        let cap = cap_of(v);
        let scenario: Scenario = serde_json::from_value(v["scenario"].clone()).expect("Scenario");
        let ex = &v["expected"];
        let core = core_program(program);
        // Differential 1: actual Core versus the reviewer oracle.
        let reference = match cap {
            Some(c) => mori_core::simulate_metered(&core, action, &scenario, c).map(|m| m.execution).map_err(|e| e.error),
            None => mori_core::simulate(&core, action, &scenario),
        };
        let rel = match relation(program, action, cap.is_some(), FrameCells::TouchedOnly) {
            Ok(r) => r,
            Err(e) => {
                t.row(id, false, format!("compile refused: {e}"));
                continue;
            }
        };
        if ex["accepted"].as_bool() == Some(true) {
            let pre: Snapshot = serde_json::from_value(ex["pre"].clone()).expect("pre");
            let post: Snapshot = serde_json::from_value(ex["post"].clone()).expect("post");
            let effects: Vec<Effect> = serde_json::from_value(ex["effects"].clone()).expect("effects");
            let work = ex["work_used"].as_u64().expect("work") as u128;
            match &reference {
                Ok(exe) => t.row(id, exe.pre == pre && exe.post == post && exe.effects == effects, "Core reference equals reviewer oracle".into()),
                Err(e) => t.row(id, false, format!("Core refused an expected success: {e}")),
            }
            t.row(id, rel.plan().logical_work == UInt(work), format!("W {} vs reviewer {work}", rel.plan().logical_work.0));
            match values_from(&rel, &scenario, &pre, &post, &effects, cap).and_then(|vals| satisfied(&rel, vals, Tamper::none())) {
                Ok(sat) => t.row(id, sat, format!("relation satisfiable = {sat} (expected true)")),
                Err(e) => t.row(id, false, e),
            }
        } else {
            let code = ex["error"].as_str().expect("code");
            let step = ex["step"].as_u64().map(|s| s as u16);
            match &reference {
                Ok(_) => t.row(id, false, "Core accepted an expected refusal".into()),
                Err(e) => t.row(id, e.code == code && e.step == step, format!("Core refusal {} step {:?} vs reviewer {code} {step:?}", e.code, e.step)),
            }
        }
    }
    t.finish("heldout");
}

#[test]
fn permissive_hostile_claims_are_refused() {
    let doc = json(HOSTILE);
    let vectors = json(VECTORS);
    let by_id: BTreeMap<String, Json> =
        vectors["vectors"].as_array().expect("vectors").iter().map(|v| (v["id"].as_str().unwrap().to_string(), v.clone())).collect();
    let mut t = Table { rows: Vec::new(), unexpected: 0 };
    for c in doc["rejected_vector_claims"].as_array().expect("claims") {
        let id = c["base_vector"].as_str().expect("id");
        let v = &by_id[id];
        let program = v["program"].as_str().unwrap();
        let action = v["action"].as_str().unwrap();
        let cap = cap_of(v);
        let rel = relation(program, action, cap.is_some(), FrameCells::TouchedOnly).expect("held-out program compiles");
        let claim = &c["claim"];
        if claim.is_null() {
            t.row(id, true, "admission refusal; see note (premise rows)".into());
            continue;
        }
        let scenario: Scenario = serde_json::from_value(v["scenario"].clone()).expect("Scenario");
        let parsed = (|| -> Result<TransitionValues, String> {
            let pre: Snapshot = serde_json::from_value(claim["pre"].clone()).map_err(|e| format!("not decodable: {e}"))?;
            let post: Snapshot = serde_json::from_value(claim["post"].clone()).map_err(|e| format!("not decodable: {e}"))?;
            let effects: Vec<Effect> = serde_json::from_value(claim["effects"].clone()).map_err(|e| format!("not decodable: {e}"))?;
            values_from(&rel, &scenario, &pre, &post, &effects, cap)
        })();
        match parsed.and_then(|vals| satisfied(&rel, vals, Tamper::none())) {
            Ok(sat) => t.row(id, !sat, format!("permissive claim satisfiable = {sat} (expected false)")),
            Err(e) => t.row(id, true, format!("refused before proving: {e}")),
        }
    }
    t.finish("hostile");
}

#[test]
fn targeted_mutations() {
    let vectors = json(VECTORS);
    let get = |id: &str| -> Json { vectors["vectors"].as_array().unwrap().iter().find(|v| v["id"] == id).cloned().expect("vector") };
    let parts = |v: &Json| -> (Scenario, Snapshot, Snapshot, Vec<Effect>) {
        let ex = &v["expected"];
        (
            serde_json::from_value(v["scenario"].clone()).unwrap(),
            serde_json::from_value(ex["pre"].clone()).unwrap(),
            serde_json::from_value(ex["post"].clone()).unwrap(),
            serde_json::from_value(ex["effects"].clone()).unwrap(),
        )
    };
    let mut t = Table { rows: Vec::new(), unexpected: 0 };

    // M-01 / M-02 / M-08: delete a present-zero post duty row.
    for (mid, vid, program, action, duty) in [
        ("M-01", "H1-P1", "H1-settle", "settle", "warranty"),
        ("M-02", "H1-P3", "H1-settle", "close", "warranty"),
        ("M-08", "H2-P3", "H2-accrual", "repay", "loan"),
    ] {
        let rel = relation(program, action, false, FrameCells::TouchedOnly).unwrap();
        let (s, pre, mut post, eff) = parts(&get(vid));
        post.liabilities.retain(|l| l.name != duty);
        let r = values_from(&rel, &s, &pre, &post, &eff, None).and_then(|v| satisfied(&rel, v, Tamper::none()));
        t.row(mid, matches!(r, Ok(false)), format!("{r:?} (expected Ok(false))"));
    }

    // M-03: swap Transfer and Fee amounts.
    {
        let rel = relation("H1-settle", "settle", false, FrameCells::TouchedOnly).unwrap();
        let (s, pre, post, mut eff) = parts(&get("H1-P1"));
        let a0 = match &eff[0] { Effect::Transfer { amount, .. } => *amount, _ => panic!("effect 0 kind") };
        let a1 = match &eff[1] { Effect::Fee { amount, .. } => *amount, _ => panic!("effect 1 kind") };
        if let Effect::Transfer { amount, .. } = &mut eff[0] { *amount = a1; }
        if let Effect::Fee { amount, .. } = &mut eff[1] { *amount = a0; }
        let r = values_from(&rel, &s, &pre, &post, &eff, None).and_then(|v| satisfied(&rel, v, Tamper::none()));
        t.row("M-03", matches!(r, Ok(false)), format!("{r:?} (expected Ok(false))"));
    }

    // M-04: change the fee recipient (a relation constant).
    {
        let rel = relation("H1-settle", "settle", false, FrameCells::TouchedOnly).unwrap();
        let (s, pre, post, mut eff) = parts(&get("H1-P1"));
        if let Effect::Fee { to, .. } = &mut eff[1] { *to = "seller".into(); }
        let r = values_from(&rel, &s, &pre, &post, &eff, None);
        t.row("M-04", r.is_err(), format!("encodable = {} (expected not encodable)", r.is_ok()));
    }

    // M-05: framed auditor cell must be preserved.
    {
        let frame = FrameCells::Include(vec![CellRef { asset: "USDX".into(), account: "auditor".into() }]);
        let rel = relation("H1-settle", "settle", false, frame).unwrap();
        let (s, pre, post, eff) = parts(&get("H1-P1"));
        let honest = values_from(&rel, &s, &pre, &post, &eff, None).and_then(|v| satisfied(&rel, v, Tamper::none()));
        t.row("M-05-honest", matches!(honest, Ok(true)), format!("{honest:?}"));
        let mut vals = values_from(&rel, &s, &pre, &post, &eff, None).unwrap();
        let cell = rel.plan().cells.iter().position(|c| c.account == "auditor").unwrap();
        vals.post.cells[cell] = UInt(8);
        let r = satisfied(&rel, vals, Tamper::none());
        t.row("M-05", matches!(r, Ok(false)), format!("{r:?} (expected Ok(false))"));
    }

    // M-09: one shared wire for DAG node 2 across statements.
    {
        let rel = relation("H3-dag", "bump", false, FrameCells::TouchedOnly).unwrap();
        let (s, pre, mut post, mut eff) = parts(&get("H3-P1"));
        post.state.insert("s".into(), mori_core::Value::UInt(UInt(5)));
        if let Effect::State { after, .. } = &mut eff[0] { *after = mori_core::Value::UInt(UInt(5)); }
        if let Effect::State { before, after, .. } = &mut eff[1] { *before = mori_core::Value::UInt(UInt(5)); *after = mori_core::Value::UInt(UInt(5)); }
        let r = values_from(&rel, &s, &pre, &post, &eff, None).and_then(|v| satisfied(&rel, v, Tamper::none()));
        t.row("M-09", matches!(r, Ok(false)), format!("{r:?} (expected Ok(false))"));
    }

    // M-12: cap below W.
    {
        let rel = relation("H1-settle", "settle", true, FrameCells::TouchedOnly).unwrap();
        let (s, pre, post, eff) = parts(&get("W-1"));
        let r = values_from(&rel, &s, &pre, &post, &eff, Some(UInt(35))).and_then(|v| satisfied(&rel, v, Tamper::none()));
        t.row("M-12", matches!(r, Ok(false)), format!("{r:?} (expected Ok(false))"));
    }

    // M-14 (finding F-04, documented, expected SATISFIABLE today): the operator row is
    // missing in the Core scenario of H1-R7, but the relation has no balance presence bit.
    {
        let rel = relation("H1-settle", "settle", false, FrameCells::TouchedOnly).unwrap();
        let v = get("H1-R7");
        let mut s: Scenario = serde_json::from_value(v["scenario"].clone()).unwrap();
        let usdx = rel.plan().cells.iter().find(|c| c.account == "buyer").unwrap().asset.clone();
        s.balances.push(mori_core::Balance { account: "operator".into(), asset: usdx, amount: UInt(0) });
        let core = core_program("H1-settle");
        let r = mori_core::simulate(&core, "settle", &s)
            .map_err(|e| e.to_string())
            .and_then(|exe| values_from(&rel, &s, &exe.pre, &exe.post, &exe.effects, None))
            .and_then(|vals| satisfied(&rel, vals, Tamper::none()));
        t.row("M-14", matches!(r, Ok(true)), format!("{r:?} (expected Ok(true): documents F-04, row presence is an unauthenticated premise)"));
    }
    t.finish("mutations");
}

#[test]
fn absent_duty_discharge_control() {
    // POST-FREEZE addendum 2: the in-circuit presence assertion is the only refusal.
    let doc = json(ADDENDUM2);
    let mut t = Table { rows: Vec::new(), unexpected: 0 };
    let rel = relation("H8-discharge", "pay", false, FrameCells::TouchedOnly).expect("H8 compiles");
    let core = core_program("H8-discharge");
    for v in doc["vectors"].as_array().expect("vectors") {
        let id = v["id"].as_str().unwrap();
        let scenario: Scenario = serde_json::from_value(v["scenario"].clone()).expect("Scenario");
        let ex = &v["expected"];
        let reference = mori_core::simulate(&core, "pay", &scenario);
        if ex["accepted"].as_bool() == Some(true) {
            let pre: Snapshot = serde_json::from_value(ex["pre"].clone()).unwrap();
            let post: Snapshot = serde_json::from_value(ex["post"].clone()).unwrap();
            let effects: Vec<Effect> = serde_json::from_value(ex["effects"].clone()).unwrap();
            t.row(id, reference.as_ref().is_ok_and(|e| e.post == post && e.effects == effects), "Core reference equals reviewer oracle".into());
            let r = values_from(&rel, &scenario, &pre, &post, &effects, None).and_then(|vals| satisfied(&rel, vals, Tamper::none()));
            t.row(id, matches!(r, Ok(true)), format!("{r:?} (expected Ok(true))"));
        } else {
            t.row(id, reference.as_ref().is_err_and(|e| e.code == ex["error"].as_str().unwrap()), "Core refuses as the reviewer oracle".into());
            let claim = &v["hostile_claim"];
            let pre = Snapshot { state: BTreeMap::new(), balances: Vec::new(), liabilities: Vec::new() };
            let post: Snapshot = serde_json::from_value(claim["post"].clone()).unwrap();
            let effects: Vec<Effect> = serde_json::from_value(claim["effects"].clone()).unwrap();
            let r = values_from(&rel, &scenario, &pre, &post, &effects, None).and_then(|vals| satisfied(&rel, vals, Tamper::none()));
            t.row(id, matches!(r, Ok(false)), format!("{r:?} (expected Ok(false))"));
        }
    }
    t.finish("addendum2");
}

#[test]
fn static_refusals() {
    let mut t = Table { rows: Vec::new(), unexpected: 0 };
    // S-1: moriarty/2 source.
    let r = mori_r1cs::compile_source::<Fr>(H7, opts("go", false, FrameCells::TouchedOnly), StatementAdapter::DirectFieldV0);
    t.row("S-1", matches!(r, Err(CompileError::StagedNotLowered(_)) | Err(CompileError::Core(_))), format!("{:?}", r.as_ref().err().map(|e| e.to_string())));
    // S-2: in-memory /1 program carrying Await.
    let mut p = h3_program();
    p.actions[1].statements.push(mori_core::Statement::Await { label: "x".into(), binders: Vec::new(), guard: 5 });
    t.row("S-2", matches!(mori_r1cs::admit_program(p), Err(CompileError::Core(_))), "Await under /1 refused by Core".into());
    // S-3: semantics label mismatch.
    let mut p = h3_program();
    p.semantics = "moriarty-core/2".into();
    t.row("S-3", matches!(mori_r1cs::admit_program(p), Err(CompileError::Core(_))), "profile refused".into());
    // S-5: depth 65 chain.
    let mut p = h3_program();
    let base = p.expressions.len() as u16;
    p.expressions.push(mori_core::Expr::Literal { value: mori_core::Value::UInt(UInt(1)) });
    for i in 0..65u16 {
        p.expressions.push(mori_core::Expr::Binary { op: mori_core::BinaryOp::Add, lhs: base + i, rhs: 1 });
    }
    t.row("S-5", matches!(mori_r1cs::admit_program(p), Err(CompileError::Core(_))), "depth > 64 refused".into());
    // S-6: self reference.
    let mut p = h3_program();
    p.expressions[2] = mori_core::Expr::Binary { op: mori_core::BinaryOp::Add, lhs: 2, rhs: 1 };
    t.row("S-6", matches!(mori_r1cs::admit_program(p), Err(CompileError::Core(_))), "cycle refused".into());
    // S-7: dead nodes admitted; every action compiles.
    for action in ["open", "bump", "pay", "poly_u", "poly_b"] {
        let r = relation("H3-dag", action, false, FrameCells::TouchedOnly);
        t.row(&format!("S-7/{action}"), r.is_ok(), format!("{:?}", r.as_ref().err().map(|e| e.to_string())));
    }
    // S-8: asset scale 39.
    let bad = H1.replace("scale: 6", "scale: 39");
    let r = mori_r1cs::compile_source::<Fr>(&bad, opts("settle", false, FrameCells::TouchedOnly), StatementAdapter::DirectFieldV0);
    t.row("S-8", matches!(r, Err(CompileError::Core(_))), "scale 39 refused".into());
    // H4: Emit lowered by the compiler, refused whole-program by the initial ledger target.
    let r = relation("H4-emit", "tick", false, FrameCells::TouchedOnly);
    t.row("S-4/compile", r.is_ok(), "Emit program compiles under the test adapter".into());
    let target = mori_r1cs::initial_ledger_target_admits(&core_program("H4-emit"));
    t.row("S-4/target", target.is_err(), "initial ledger target refuses whole-program Emit".into());
    t.finish("static");
}
