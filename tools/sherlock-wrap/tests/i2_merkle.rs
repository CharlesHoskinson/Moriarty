//! I2: the WHIR Merkle openings of a real `L_Fr`-config proof on both back ends.
use ff::Field;
use midnight_proofs::{circuit::Value, dev::MockProver};
use midnight_zk_stdlib::{MidnightCircuit, cost_model};
use sherlock_wrap::{
    alg::Native,
    merkle_test::{MerkleRelation, Opening, openings},
    native::F,
    wrapin::{WrapIn, fixture},
};

fn native_failures(rel: &MerkleRelation, ops: &[Opening]) -> usize {
    let mut a = Native::default();
    rel.run(&mut a, &Value::known(ops.to_vec())).unwrap();
    a.failures.len()
}

#[test]
#[ignore = "needs the exporter fixtures"]
fn i2_merkle_native_all_openings() {
    for name in ["mix-small", "mix-lfr"] {
        let p = fixture(name);
        if !p.exists() {
            continue;
        }
        let w = WrapIn::load(&p);
        let ops = openings(&w);
        let rel = MerkleRelation::new(&w, &ops);
        assert_eq!(native_failures(&rel, &ops), 0);
        let mut a = Native::default();
        rel.run(&mut a, &Value::known(ops.clone())).unwrap();
        let widths: Vec<usize> = ops.iter().map(|o| o.rows[0].len()).collect();
        println!("E5I2 {name}: {} openings accepted natively, {} permutations, widths {:?}..", ops.len(), a.perms, &widths[..widths.len().min(4)]);
        // Negative controls (native verdict).
        let mut bad = ops.clone();
        bad[0].siblings[1] += F::ONE;
        assert!(native_failures(&rel, &bad) > 0, "changed sibling");
        let mut bad = ops.clone();
        bad[0].rows[3][0] += F::ONE;
        assert!(native_failures(&rel, &bad) > 0, "changed opened value");
        let mut bad = ops.clone();
        bad[0].sample += F::ONE; // flips index bit 0
        assert!(native_failures(&rel, &bad) > 0, "changed index bit");
    }
}

#[test]
#[ignore = "needs the exporter fixtures"]
fn i2_merkle_mockprover() {
    let w = WrapIn::load(&fixture("mix-small"));
    let all = openings(&w);
    // the first opening of each distinct shape
    let mut ops: Vec<Opening> = vec![];
    for o in &all {
        if !ops.iter().any(|p| p.rows.len() == o.rows.len() && p.rows[0].len() == o.rows[0].len() && p.siblings.len() == o.siblings.len()) {
            ops.push(o.clone());
        }
    }
    let rel = MerkleRelation::new(&w, &ops);
    let k = 16;
    let m = cost_model(&rel, Some(k));
    println!("E5I2 mock: {} openings, rows {}", ops.len(), m.rows);
    let run = |v: Vec<Opening>| {
        let c = MidnightCircuit::new(&rel, Value::known(()), Value::known(v), Some(k));
        MockProver::<F>::run(&c, vec![vec![], vec![]]).unwrap().verify().is_ok()
    };
    assert!(run(ops.clone()));
    let mut bad = ops.clone();
    bad[0].siblings[0] += F::ONE;
    assert!(!run(bad));
    let mut bad = ops.clone();
    bad[0].sample += F::ONE;
    assert!(!run(bad));
    println!("E5I2 mock: honest accepted; changed sibling and changed index bit refused");
}
