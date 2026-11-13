//! I5: WHIR, and with it the whole `L_Fr` verifier over `Alg`.
use ff::Field;
use midnight_proofs::{circuit::Value, dev::MockProver};
use midnight_zk_stdlib::{MidnightCircuit, cost_model};
use sherlock_wrap::{
    alg::Native,
    native::F,
    stacked::{StackShape, part3},
    transcript::Transcript,
    verifier::{ProofData, Shape, commit_cells, part1, part2},
    verifier_test::Part1Relation,
    whir::{WhirShape, part4},
    wrapin::{Ev, WrapIn, fixture},
};

fn stock_samples(w: &WrapIn) -> Vec<F> {
    w.log().into_iter().filter_map(|e| if let Ev::Sample(x) = e { Some(x) } else { None }).collect()
}

struct Fx {
    s: Shape,
    ss: StackShape,
    ws: WhirShape,
    p: ProofData,
    stock: Vec<F>,
}

fn load(name: &str) -> Fx {
    let w = WrapIn::load(&fixture(name));
    let s = Shape::from_wrapin(&w);
    let ss = StackShape::from_wrapin(&w, &s);
    let ws = WhirShape::from_wrapin(&w, &ss.widths);
    Fx { s, ss, ws, p: ProofData::from_wrapin(&w), stock: stock_samples(&w) }
}

/// The whole verifier natively: (failures, samples, permutations).
fn native(f: &Fx, p: &ProofData) -> (Vec<String>, Vec<F>, usize) {
    let mut a = Native::default();
    let mut t = Transcript::new(&mut a, f.s.u_transcript).unwrap();
    let pv = Value::known(p);
    let p1 = part1(&mut a, &mut t, &f.s, &pv).unwrap();
    let op = part2(&mut a, &mut t, &f.s, &pv, &p1).unwrap();
    let (u, q) = part3(&mut a, &mut t, &f.s, &f.ss, &Value::known(&p.stack), &p1.rs, &op).unwrap();
    let commits = commit_cells(&mut a, &f.s, &p1).unwrap();
    part4(&mut a, &mut t, &f.ws, &Value::known(&p.whir), &q, &commits, &u, f.s.l_skip).unwrap();
    let mut out = vec![];
    for x in &t.samples {
        x.map(|v| out.push(v));
    }
    (a.failures, out, a.perms)
}

#[test]
#[ignore = "needs the exporter fixtures"]
fn i5_whole_verifier_native_equals_stock() {
    for name in ["mix-small", "mix-lfr"] {
        let f = load(name);
        let p = &f.p;
        let (fails, smp, perms) = native(&f, p);
        assert!(fails.is_empty(), "{name}: {fails:?}");
        assert_eq!(smp, f.stock, "{name}: every sample of the stock run, in order");
        println!("E5I5 {name}: whole verifier accepts; {} of {} stock samples equal; {perms} permutations; queries {:?}, widths {:?}",
            smp.len(), f.stock.len(), f.ws.num_queries, f.ws.widths);
        type M = Box<dyn Fn(&mut ProofData)>;
        let negs: Vec<(&str, M)> = vec![
            ("initial row value", Box::new(|b: &mut ProofData| b.whir.initial_rows[0][5][3][0] += F::ONE)),
            ("initial row value, last commitment", Box::new(|b: &mut ProofData| { let c = b.whir.initial_rows.len() - 1; b.whir.initial_rows[c][0][0][0] += F::ONE })),
            ("initial sibling", Box::new(|b: &mut ProofData| b.whir.initial_proofs[0][7][2] += F::ONE)),
            ("codeword value", Box::new(|b: &mut ProofData| b.whir.codeword_values[0][1][4] += F::ONE)),
            ("codeword sibling", Box::new(|b: &mut ProofData| b.whir.codeword_proofs[0][1][0] += F::ONE)),
            ("codeword commitment", Box::new(|b: &mut ProofData| b.whir.codeword_commits[0] += F::ONE)),
            ("OOD value", Box::new(|b: &mut ProofData| b.whir.ood_values[0] += F::ONE)),
            ("WHIR sumcheck value", Box::new(|b: &mut ProofData| b.whir.sumcheck_polys[3][1] += F::ONE)),
            ("final poly coefficient", Box::new(|b: &mut ProofData| b.whir.final_poly[1] += F::ONE)),
            ("stacking opening", Box::new(|b: &mut ProofData| b.stack.stacking_openings[0][0] += F::ONE)),
            ("common main commitment", Box::new(|b: &mut ProofData| b.common_main_commit += F::ONE)),
        ];
        for (what, m) in &negs {
            let mut bad = p.clone();
            m(&mut bad);
            assert!(!native(&f, &bad).0.is_empty(), "{name}: changed {what} accepted");
        }
        if f.ws.query_pow_bits > 0 {
            let mut bad = p.clone();
            bad.whir.query_pow[0] += F::ONE;
            assert!(!native(&f, &bad).0.is_empty(), "{name}: query PoW");
        }
        println!("E5I5 {name}: refused all {} changes ({}); query PoW bits {}",
            negs.len(), negs.iter().map(|x| x.0).collect::<Vec<_>>().join(", "), f.ws.query_pow_bits);
    }
}

fn relation(f: &Fx, n: usize) -> Part1Relation {
    Part1Relation { shape: f.s.clone(), n_samples: n, part2: true, stack: Some(f.ss.clone()), whir: Some(f.ws.clone()) }
}

#[test]
#[ignore = "needs the exporter fixtures"]
fn i5_whole_verifier_mockprover() {
    let f = load("mix-small");
    let (_, smp, _) = native(&f, &f.p);
    let rel = relation(&f, smp.len());
    let k = 19;
    let m = cost_model(&rel, Some(k));
    println!("E5I5 mock mix-small: rows {} at k = {k}", m.rows);
    let run = |p: ProofData| {
        let c = MidnightCircuit::new(&rel, Value::known(()), Value::known((p, smp.clone())), Some(k));
        MockProver::<F>::run(&c, vec![vec![], vec![]]).unwrap().verify().is_ok()
    };
    assert!(run(f.p.clone()));
    let mut bad = f.p.clone();
    bad.whir.initial_rows[0][5][3][0] += F::ONE;
    assert!(!run(bad));
    println!("E5I5 mock: honest accepted; changed opened row refused");
}

#[test]
#[ignore = "needs the exporter fixtures"]
fn i5_rows_at_lfr_params() {
    let f = load("mix-lfr");
    let (_, smp, perms) = native(&f, &f.p);
    let rel = relation(&f, smp.len());
    let m = cost_model(&rel, Some(21));
    println!("E5I5 cost model mix-lfr (L_Fr parameters, mixture widths {:?}): rows {} of the k = 21 gate 1782573, {perms} permutations",
        f.ws.widths, m.rows);
}
