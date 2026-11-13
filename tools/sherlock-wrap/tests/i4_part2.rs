//! I4: `batch_constraints.rs` steps 7 to 9 (column openings, DAG evaluation, final claim).
use ff::Field;
use midnight_proofs::{circuit::Value, dev::MockProver};
use midnight_zk_stdlib::{MidnightCircuit, cost_model};
use sherlock_wrap::{
    alg::Native,
    native::F,
    transcript::Transcript,
    verifier::{ProofData, Shape, part1, part2},
    verifier_test::Part1Relation,
    wrapin::{Ev, WrapIn, fixture},
};

fn stock_samples(w: &WrapIn) -> Vec<F> {
    w.log().into_iter().filter_map(|e| if let Ev::Sample(x) = e { Some(x) } else { None }).collect()
}

/// Native part 1 + part 2 + one more sample: (failures, samples).
fn native(s: &Shape, p: &ProofData) -> (Vec<String>, Vec<F>) {
    let mut a = Native::default();
    let mut t = Transcript::new(&mut a, s.u_transcript).unwrap();
    let pv = Value::known(p);
    let p1 = part1(&mut a, &mut t, s, &pv).unwrap();
    part2(&mut a, &mut t, s, &pv, &p1).unwrap();
    t.sample(&mut a).unwrap();
    let mut out = vec![];
    for x in &t.samples {
        x.map(|v| out.push(v));
    }
    (a.failures, out)
}

#[test]
#[ignore = "needs the exporter fixtures"]
fn i4_part2_native_equals_stock() {
    for name in ["mix-small", "mix-lfr"] {
        let w = WrapIn::load(&fixture(name));
        let s = Shape::from_wrapin(&w);
        let p = ProofData::from_wrapin(&w);
        // the key-derived opening shape equals the proof's
        let lens: Vec<Vec<usize>> = p.column_openings.iter().map(|t| t.iter().map(|x| x.len()).collect()).collect();
        assert_eq!(lens, s.opening_lens, "{name}: opening shape");
        let stock = stock_samples(&w);
        let (fails, smp) = native(&s, &p);
        assert!(fails.is_empty(), "{name}: {fails:?}");
        assert_eq!(&stock[..smp.len()], &smp[..], "{name}: samples");
        let nodes: usize = s.trace_to_air.iter().map(|&i| s.airs[i].nodes.len()).sum();
        let ints: usize = s.trace_to_air.iter().map(|&i| s.airs[i].interactions.len()).sum();
        println!("E5I4 {name}: final batch claim holds; {} samples equal (incl. first stacked sample); {} DAG nodes, {} interactions, 0 failed checks",
            smp.len(), nodes, ints);
        // Negative controls: every opening part, the off-grid univariate coefficient, a round poly.
        let mut refused = 0;
        let mut tried = 0;
        for ti in 0..p.column_openings.len() {
            for pi in 0..p.column_openings[ti].len() {
                let mut bad = p.clone();
                bad.column_openings[ti][pi][0] += F::ONE;
                tried += 1;
                if !native(&s, &bad).0.is_empty() {
                    refused += 1;
                }
            }
        }
        assert_eq!(refused, tried, "{name}: changed openings");
        let mut bad = p.clone();
        bad.univariate_round_coeffs[1] += F::ONE;
        assert!(!native(&s, &bad).0.is_empty(), "changed univariate coefficient 1");
        if !p.sumcheck_round_polys.is_empty() {
            let mut bad = p.clone();
            bad.sumcheck_round_polys[0][1] += F::ONE;
            assert!(!native(&s, &bad).0.is_empty(), "changed round poly");
        }
        let mut bad = p.clone();
        bad.public_values.iter_mut().find(|v| !v.is_empty()).map(|v| v[0] += F::ONE);
        let pv_refused = !native(&s, &bad).0.is_empty() || native(&s, &bad).1 != smp;
        println!("E5I4 {name}: refused {refused}/{tried} changed opening parts, univariate coeff 1, round poly; changed public value changes the run: {pv_refused}");
    }
}

#[test]
#[ignore = "needs the exporter fixtures"]
fn i4_part2_mockprover() {
    let w = WrapIn::load(&fixture("mix-small"));
    let s = Shape::from_wrapin(&w);
    let p = ProofData::from_wrapin(&w);
    let (_, smp) = native(&s, &p);
    let rel = Part1Relation { shape: s.clone(), n_samples: smp.len(), part2: true, stack: None, whir: None };
    let k = 17;
    let m = cost_model(&rel, Some(k));
    println!("E5I4 mock mix-small: rows {}", m.rows);
    let run = |p: ProofData| {
        let c = MidnightCircuit::new(&rel, Value::known(()), Value::known((p, smp.clone())), Some(k));
        MockProver::<F>::run(&c, vec![vec![], vec![]]).unwrap().verify().is_ok()
    };
    assert!(run(p.clone()));
    let mut bad = p.clone();
    bad.column_openings[0][0][0] += F::ONE;
    assert!(!run(bad));
    println!("E5I4 mock: honest accepted; changed column opening refused");
}
