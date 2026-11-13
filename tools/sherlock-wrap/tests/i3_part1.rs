//! I3: preamble, LogUp PoW, LogUp GKR and the zerocheck rounds on both back ends.
use ff::Field;
use midnight_proofs::{circuit::Value, dev::MockProver};
use midnight_zk_stdlib::{MidnightCircuit, cost_model};
use sherlock_wrap::{
    alg::Native,
    native::F,
    transcript::Transcript,
    verifier::{ProofData, Shape, part1},
    verifier_test::Part1Relation,
    wrapin::{Ev, WrapIn, fixture},
};

fn stock_samples(w: &WrapIn) -> Vec<F> {
    w.log().into_iter().filter_map(|e| if let Ev::Sample(x) = e { Some(x) } else { None }).collect()
}

/// Native run: (failures, samples).
fn native(s: &Shape, p: &ProofData) -> (Vec<String>, Vec<F>) {
    let mut a = Native::default();
    let mut t = Transcript::new(&mut a, s.u_transcript).unwrap();
    part1(&mut a, &mut t, s, &Value::known(p)).unwrap();
    let mut out = vec![];
    for x in &t.samples {
        x.map(|v| out.push(v));
    }
    (a.failures, out)
}

#[test]
#[ignore = "needs the exporter fixtures"]
fn i3_part1_native_equals_stock() {
    for name in ["mix-small", "mix-lfr"] {
        let w = WrapIn::load(&fixture(name));
        let s = Shape::from_wrapin(&w);
        let p = ProofData::from_wrapin(&w);
        let stock = stock_samples(&w);
        let (fails, smp) = native(&s, &p);
        assert!(fails.is_empty(), "{name}: {fails:?}");
        assert_eq!(&stock[..smp.len()], &smp[..], "{name}: samples");
        println!("E5I3 {name}: {} traces, n_logup {}, {} samples equal to the stock record, 0 failed checks",
            s.trace_to_air.len(), s.n_logup(), smp.len());
        // Negative controls.
        let mut bad = p.clone();
        bad.layer_claims[2][0] += F::ONE;
        assert!(!native(&s, &bad).0.is_empty(), "changed GKR layer claim");
        let mut bad = p.clone();
        bad.gkr_sumcheck[3][1][2] += F::ONE;
        assert!(!native(&s, &bad).0.is_empty(), "changed GKR round polynomial");
        let mut bad = p.clone();
        bad.univariate_round_coeffs[1] += F::ONE;
        let (f, _) = native(&s, &bad);
        // a changed coefficient not on the step_by(2^l_skip) positions only changes later
        // samples; part 1 has no later check of it (I4's final claim does), so test coefficient 0
        let mut bad0 = p.clone();
        bad0.univariate_round_coeffs[0] += F::ONE;
        assert!(!native(&s, &bad0).0.is_empty(), "changed univariate coefficient 0");
        let mut bad = p.clone();
        bad.numerator_term_per_air[0] += F::ONE;
        assert!(!native(&s, &bad).0.is_empty(), "changed numerator term");
        let mut bad = p.clone();
        bad.logup_pow_witness += F::ONE;
        let pow_refused = !native(&s, &bad).0.is_empty();
        println!("E5I3 {name}: refused layer claim, GKR round poly, univariate coeff 0, numerator term; coeff 1 failures {} (checked in I4); changed LogUp PoW witness refused: {pow_refused} (PoW bits {})",
            f.len(), s.logup_pow_bits);
        if s.logup_pow_bits > 0 {
            assert!(pow_refused);
        }
    }
}

#[test]
#[ignore = "needs the exporter fixtures"]
fn i3_part1_mockprover() {
    let w = WrapIn::load(&fixture("mix-small"));
    let s = Shape::from_wrapin(&w);
    let p = ProofData::from_wrapin(&w);
    let (_, smp) = native(&s, &p);
    let rel = Part1Relation { shape: s.clone(), n_samples: smp.len(), part2: false };
    let k = 16;
    let m = cost_model(&rel, Some(k));
    println!("E5I3 mock mix-small: rows {}", m.rows);
    let run = |p: ProofData| {
        let c = MidnightCircuit::new(&rel, Value::known(()), Value::known((p, smp.clone())), Some(k));
        MockProver::<F>::run(&c, vec![vec![], vec![]]).unwrap().verify().is_ok()
    };
    assert!(run(p.clone()));
    let mut bad = p.clone();
    bad.layer_claims[2][0] += F::ONE;
    assert!(!run(bad));
    println!("E5I3 mock: honest accepted; changed layer claim refused");
}
