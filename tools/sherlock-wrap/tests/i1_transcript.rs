//! I1: the stock `L_Fr` transcript replayed natively and in circuit.
//! Needs the fixtures of the exporter (`E5_FIXTURES`); run with `--include-ignored`.
use ff::Field;
use midnight_proofs::{circuit::Value, dev::MockProver};
use midnight_zk_stdlib::{MidnightCircuit, cost_model};
use sherlock_wrap::{
    native::{F, NativeTranscript},
    replay::{ReplayRelation, log_values},
    wrapin::{Ev, WrapIn, fixture},
};

fn native_replay(w: &WrapIn) -> usize {
    let mut t = NativeTranscript::new(w.u_transcript());
    let mut n = 0;
    for e in w.log() {
        match e {
            Ev::Obs(x) | Ev::Commit(x) => t.observe(x),
            Ev::Sample(x) => {
                assert_eq!(t.sample(), x);
                n += 1;
            }
        }
    }
    n
}

#[test]
#[ignore = "needs the exporter fixtures"]
fn i1_native_replay_equals_stock() {
    for name in ["mix-small", "mix-lfr"] {
        let p = fixture(name);
        if !p.exists() {
            continue;
        }
        let w = WrapIn::load(&p);
        let n = native_replay(&w);
        println!("E5I1 {name}: {} events, {n} samples equal", w.log().len());
    }
}

#[test]
#[ignore = "needs the exporter fixtures"]
fn i1_circuit_replay_equals_stock() {
    let w = WrapIn::load(&fixture("mix-small"));
    let log = w.log();
    let rel = ReplayRelation::new(&log, w.u_transcript());
    let k = 17;
    let m = cost_model(&rel, Some(k));
    let perms = {
        let mut t = NativeTranscript::new(w.u_transcript());
        let mut p = 0usize;
        for e in &log {
            let before = t.state;
            match e {
                Ev::Obs(x) | Ev::Commit(x) => t.observe(*x),
                Ev::Sample(_) => {
                    t.sample();
                }
            }
            // a permutation changed the capacity cell
            if t.state[2] != before[2] {
                p += 1;
            }
        }
        p
    };
    println!("E5I1 replay rows {} for {} events, {perms} permutations", m.rows, log.len());
    let vals = log_values(&log);
    let circ = MidnightCircuit::new(&rel, Value::known(()), Value::known(vals.clone()), Some(k));
    assert!(MockProver::<F>::run(&circ, vec![vec![], vec![]]).unwrap().verify().is_ok());
    // A changed observed value (the first observe after the preamble) is refused.
    let i = log.iter().position(|e| matches!(e, Ev::Obs(_))).unwrap();
    let mut bad = vals.clone();
    bad[i] += F::ONE;
    let circ = MidnightCircuit::new(&rel, Value::known(()), Value::known(bad), Some(k));
    assert!(MockProver::<F>::run(&circ, vec![vec![], vec![]]).unwrap().verify().is_err());
    println!("E5I1 MockProver: honest accepted, changed observe refused");
}
