//! I4b: the stacked reduction (`stacked_reduction.rs`).
use ff::Field;
use midnight_proofs::{circuit::Value, dev::MockProver};
use midnight_zk_stdlib::{MidnightCircuit, cost_model};
use sherlock_wrap::{
    alg::Native,
    native::F,
    stacked::{StackShape, part3},
    transcript::Transcript,
    verifier::{ProofData, Shape, part1, part2},
    verifier_test::Part1Relation,
    wrapin::{Ev, WrapIn, fixture},
};

fn stock_samples(w: &WrapIn) -> Vec<F> {
    w.log().into_iter().filter_map(|e| if let Ev::Sample(x) = e { Some(x) } else { None }).collect()
}

/// Native parts 1 to 3: (failures, samples).
fn native(s: &Shape, ss: &StackShape, p: &ProofData) -> (Vec<String>, Vec<F>) {
    let mut a = Native::default();
    let mut t = Transcript::new(&mut a, s.u_transcript).unwrap();
    let pv = Value::known(p);
    let p1 = part1(&mut a, &mut t, s, &pv).unwrap();
    let op = part2(&mut a, &mut t, s, &pv, &p1).unwrap();
    part3(&mut a, &mut t, s, ss, &Value::known(&p.stack), &p1.rs, &op).unwrap();
    let mut out = vec![];
    for x in &t.samples {
        x.map(|v| out.push(v));
    }
    (a.failures, out)
}

#[test]
#[ignore = "needs the exporter fixtures"]
fn i4b_stacked_native_equals_stock() {
    for name in ["mix-small", "mix-lfr"] {
        let w = WrapIn::load(&fixture(name));
        let s = Shape::from_wrapin(&w);
        let ss = StackShape::from_wrapin(&w, &s);
        let p = ProofData::from_wrapin(&w);
        assert_eq!(p.stack.univariate_round_coeffs.len(), ss.n_uni, "{name}: univariate count");
        let wl: Vec<usize> = p.stack.stacking_openings.iter().map(|v| v.len()).collect();
        assert_eq!(wl, ss.widths, "{name}: stacked widths");
        let stock = stock_samples(&w);
        let (fails, smp) = native(&s, &ss, &p);
        assert!(fails.is_empty(), "{name}: {fails:?}");
        assert_eq!(&stock[..smp.len()], &smp[..], "{name}: samples");
        let cols: usize = ss.layouts.iter().map(|l| l.len()).sum();
        println!("E5I4b {name}: stacked final sum holds; {} samples equal; {} column claims over {} commitments, widths {:?}, n_stack {}",
            smp.len(), cols, ss.layouts.len(), ss.widths, ss.n_stack);
        let mut tried = 0;
        let mut refused = 0;
        for ci in 0..p.stack.stacking_openings.len() {
            for k in [0, p.stack.stacking_openings[ci].len() - 1] {
                let mut bad = p.clone();
                bad.stack.stacking_openings[ci][k] += F::ONE;
                tried += 1;
                refused += (!native(&s, &ss, &bad).0.is_empty()) as usize;
            }
        }
        assert_eq!(refused, tried, "{name}: changed stacking openings");
        for (what, f) in [
            ("univariate coeff 0", Box::new(|b: &mut ProofData| b.stack.univariate_round_coeffs[0] += F::ONE) as Box<dyn Fn(&mut ProofData)>),
            ("univariate coeff 1", Box::new(|b: &mut ProofData| b.stack.univariate_round_coeffs[1] += F::ONE)),
            ("round poly", Box::new(|b: &mut ProofData| b.stack.sumcheck_round_polys[2][1] += F::ONE)),
        ] {
            let mut bad = p.clone();
            f(&mut bad);
            assert!(!native(&s, &ss, &bad).0.is_empty(), "{name}: {what}");
        }
        println!("E5I4b {name}: refused {refused}/{tried} changed stacking openings, univariate coeffs 0 and 1, a round poly value");
    }
}

#[test]
#[ignore = "needs the exporter fixtures"]
fn i4b_stacked_mockprover() {
    let w = WrapIn::load(&fixture("mix-small"));
    let s = Shape::from_wrapin(&w);
    let ss = StackShape::from_wrapin(&w, &s);
    let p = ProofData::from_wrapin(&w);
    let (_, smp) = native(&s, &ss, &p);
    let rel = Part1Relation { shape: s.clone(), n_samples: smp.len(), part2: true, stack: Some(ss), whir: None };
    let k = 17;
    let m = cost_model(&rel, Some(k));
    println!("E5I4b mock mix-small: rows {}", m.rows);
    let run = |p: ProofData| {
        let c = MidnightCircuit::new(&rel, Value::known(()), Value::known((p, smp.clone())), Some(k));
        MockProver::<F>::run(&c, vec![vec![], vec![]]).unwrap().verify().is_ok()
    };
    assert!(run(p.clone()));
    let mut bad = p.clone();
    bad.stack.stacking_openings[0][0] += F::ONE;
    assert!(!run(bad));
    println!("E5I4b mock: honest accepted; changed stacking opening refused");
}
