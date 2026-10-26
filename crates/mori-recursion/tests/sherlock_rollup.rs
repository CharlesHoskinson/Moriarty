//! v3 header closes must match the independent B2 corpus, including empty headers.
use ff::Field;
use midnight_proofs::{circuit::Value, dev::MockProver};
use midnight_zk_stdlib::{MidnightCircuit, optimal_k};
use mori_recursion::rollup::{
    close::{CloseRelation, CloseWitness},
    i1::types::{Expected, Header},
};
use std::{fs, path::PathBuf};

#[derive(Clone, Debug)]
struct TransactionGuard;
impl midnight_zk_stdlib::Relation for TransactionGuard {
    type Instance = Vec<mori_recursion::util::F>;
    type Witness = mori_recursion::rollup::transition::TransactionWitness;
    type Error = midnight_proofs::plonk::Error;
    fn format_instance(i: &Self::Instance) -> Result<Self::Instance, Self::Error> {
        Ok(i.clone())
    }
    fn used_chips(&self) -> midnight_zk_stdlib::ZkStdLibArch {
        midnight_zk_stdlib::ZkStdLibArch {
            poseidon: true,
            jubjub: true,
            nr_pow2range_cols: 4,
            ..Default::default()
        }
    }
    fn circuit(
        &self,
        s: &midnight_zk_stdlib::ZkStdLib,
        l: &mut impl midnight_proofs::circuit::Layouter<mori_recursion::util::F>,
        _: Value<Self::Instance>,
        w: Value<Self::Witness>,
    ) -> Result<(), Self::Error> {
        use midnight_circuits::instructions::*;
        let (seg, _, _, _, _) = mori_recursion::rollup::transition::transition(s, l, w)?;
        for c in seg {
            s.constrain_as_public_input(l, &c)?;
        }
        Ok(())
    }
    fn write_relation<W: std::io::Write>(&self, _: &mut W) -> std::io::Result<()> {
        Ok(())
    }
    fn read_relation<R: std::io::Read>(_: &mut R) -> std::io::Result<Self> {
        Ok(Self)
    }
}

#[derive(Clone, Debug)]
struct WalletLinkGuard;
impl midnight_zk_stdlib::Relation for WalletLinkGuard {
    type Instance = Vec<mori_recursion::util::F>;
    type Witness = (
        mori_recursion::rollup::i1::types::Tx,
        mori_recursion::rollup::transition::TransactionWitness,
    );
    type Error = midnight_proofs::plonk::Error;
    fn format_instance(i: &Self::Instance) -> Result<Self::Instance, Self::Error> {
        Ok(i.clone())
    }
    fn used_chips(&self) -> midnight_zk_stdlib::ZkStdLibArch {
        midnight_zk_stdlib::Relation::used_chips(&TransactionGuard)
    }
    fn circuit(
        &self,
        s: &midnight_zk_stdlib::ZkStdLib,
        l: &mut impl midnight_proofs::circuit::Layouter<mori_recursion::util::F>,
        _: Value<Self::Instance>,
        w: Value<Self::Witness>,
    ) -> Result<(), Self::Error> {
        use midnight_circuits::instructions::*;
        let wallet =
            mori_recursion::rollup::user::user_rules(s, l, w.as_ref().map(|w| w.0.clone()))?;
        let (seg, pi, deposit, _, _) =
            mori_recursion::rollup::transition::transition(s, l, w.map(|w| w.1))?;
        let spend = s.not(l, &deposit)?;
        for (a, b) in wallet.iter().zip(&pi) {
            s.cond_assert_equal(l, &spend, a, b)?;
        }
        for c in seg {
            s.constrain_as_public_input(l, &c)?;
        }
        Ok(())
    }
    fn write_relation<W: std::io::Write>(&self, _: &mut W) -> std::io::Result<()> {
        Ok(())
    }
    fn read_relation<R: std::io::Read>(_: &mut R) -> std::io::Result<Self> {
        Ok(Self)
    }
}

#[test]
fn wallet_rules_and_conditional_transaction_binding_reject_mutations() {
    use mori_recursion::rollup::{
        transition::TransactionWitness,
        user::{UserInstance, UserRelation},
    };
    let header: Header =
        serde_json::from_slice(&fs::read(corpus().join("i1/blocks/000004.json")).unwrap()).unwrap();
    let base = header.txs[0].clone();
    let k = optimal_k(&UserRelation);
    let run_wallet = |t: &mori_recursion::rollup::i1::types::Tx| {
        let pi = UserInstance::from_public(&t.public);
        let c = MidnightCircuit::new(
            &UserRelation,
            Value::known(pi.clone()),
            Value::known(t.clone()),
            Some(k),
        );
        MockProver::run(&c, vec![vec![], pi.fields.to_vec()])
            .unwrap()
            .verify()
            .is_ok()
    };
    assert!(run_wallet(&base));
    let mut cases = Vec::new();
    let mut t = base.clone();
    t.witness.spender_secret_key_jfr = Some("01".to_string() + &"00".repeat(31));
    cases.push(t);
    let mut t = base.clone();
    t.witness.spender_nk += mori_recursion::util::F::ONE;
    cases.push(t);
    let mut t = base.clone();
    t.witness.inputs[0].membership_path[0] += mori_recursion::util::F::ONE;
    cases.push(t);
    let mut t = base.clone();
    t.witness.outputs[0].note.value += 1;
    t.public.commitments[0].value = t.witness.outputs[0].note.commitment();
    cases.push(t);
    for (i, t) in cases.iter().enumerate() {
        assert!(!run_wallet(t), "wallet mutation {i}");
    }
    let r = WalletLinkGuard;
    let k = optimal_k(&r);
    let run_link = |wallet, tx: mori_recursion::rollup::i1::types::Tx| {
        let pi = transaction_instance(&tx);
        let w = TransactionWitness {
            pre_height: tx.state_before.height,
            post_height: tx.state_after.height,
            tx,
        };
        let c = MidnightCircuit::new(
            &r,
            Value::known(pi.clone()),
            Value::known((wallet, w)),
            Some(k),
        );
        MockProver::run(&c, vec![vec![], pi])
            .unwrap()
            .verify()
            .is_ok()
    };
    assert!(run_link(base.clone(), base.clone()));
    let other: Header =
        serde_json::from_slice(&fs::read(corpus().join("i1/blocks/000005.json")).unwrap()).unwrap();
    assert!(run_wallet(&other.txs[0]));
    assert!(
        !run_link(other.txs[0].clone(), base.clone()),
        "individually valid wallet from another transaction"
    );
    let deposit: Header =
        serde_json::from_slice(&fs::read(corpus().join("i1/blocks/000042.json")).unwrap()).unwrap();
    assert!(
        run_link(base, deposit.txs[0].clone()),
        "deposit uses a valid unlinked filler wallet"
    );
}

fn transaction_instance(t: &mori_recursion::rollup::i1::types::Tx) -> Vec<mori_recursion::util::F> {
    use mori_recursion::util::F;
    let p = &t.public;
    vec![
        F::ZERO,
        t.pre_digest,
        t.post_digest,
        F::from(p.pos()),
        F::from(p.pos()),
        p.record_hash,
        F::from(t.state_before.dep_consumed),
        F::from(t.state_after.dep_consumed),
        t.witness.dep_acc_before,
        t.witness.dep_acc_after,
        F::from(p.fee),
        F::from(if p.kind.is_withdrawal() {
            p.pub_amount
        } else {
            0
        }),
        F::ZERO,
        F::from(if p.kind.is_deposit() { p.pub_amount } else { 0 }),
        F::ZERO,
        F::ZERO,
    ]
}

fn accepts_tx(t: mori_recursion::rollup::i1::types::Tx) -> bool {
    use mori_recursion::rollup::transition::TransactionWitness;
    let instance = transaction_instance(&t);
    let w = TransactionWitness {
        pre_height: t.state_before.height,
        post_height: t.state_after.height,
        tx: t,
    };
    let r = TransactionGuard;
    let k = optimal_k(&r);
    eprintln!("v3 transaction guard k={k}");
    let c = MidnightCircuit::new(&r, Value::known(instance.clone()), Value::known(w), Some(k));
    MockProver::run(&c, vec![vec![], instance])
        .unwrap()
        .verify()
        .is_ok()
}

#[test]
fn transaction_matches_b2_v3_states() {
    for h in 1..=50 {
        let header: Header = serde_json::from_slice(
            &fs::read(corpus().join(format!("i1/blocks/{h:06}.json"))).unwrap(),
        )
        .unwrap();
        for tx in header.txs {
            assert!(accepts_tx(tx), "honest transaction in {h}");
        }
    }
}

#[test]
fn transaction_rejects_signature_and_v3_state_mutations() {
    use mori_recursion::util::F;
    let header: Header =
        serde_json::from_slice(&fs::read(corpus().join("i1/blocks/000004.json")).unwrap()).unwrap();
    let base = header.txs[0].clone();
    let mut cases = Vec::new();
    let mut t = base.clone();
    t.public.signature.as_mut().unwrap().s += mori_recursion::rollup::i1::fp::JFr::ONE;
    cases.push(t);
    let mut t = base.clone();
    t.witness.rh_path[0] += F::ONE;
    cases.push(t);
    let mut t = base.clone();
    t.state_after.rh_count += 1;
    cases.push(t);
    let mut t = base.clone();
    t.state_after.rh_root += F::ONE;
    cases.push(t);
    let mut t = base.clone();
    t.state_after.fee_total += 1;
    cases.push(t);
    let mut t = base;
    t.witness.anchor_era = 1;
    cases.push(t);
    for (i, mut t) in cases.into_iter().enumerate() {
        t.pre_digest = t.state_before.digest();
        t.post_digest = t.state_after.digest();
        assert!(!accepts_tx(t), "accepted v3 transaction mutation {i}");
    }
}

#[test]
fn producer_replays_b2_and_reconstructs_exact_profile_a_statement() {
    use mori_recursion::rollup::batch::load_batch;
    for b in 1..=6 {
        let (r, w, expected) = load_batch(&corpus(), b).unwrap();
        assert_eq!(r.header_txs.len(), 50);
        assert_eq!(r.header_txs.iter().map(|&n| n as usize).sum::<usize>(), 16);
        assert_eq!(r.instance(&w).unwrap(), expected);
    }
}

#[test]
fn batch_layout_fits_the_sherlock_capacity() {
    let (r, _, _) = mori_recursion::rollup::batch::load_batch(&corpus(), 1).unwrap();
    let model = midnight_zk_stdlib::cost_model(&r, None);
    eprintln!("v3 batch layout {}", serde_json::to_string(&model).unwrap());
    assert!(model.k <= 20, "batch must split before proving");
}

#[test]
#[ignore = "whole batch MockProver; run through DoctorWatson"]
fn batch_constraints_match_profile_a_and_reject_mutations() {
    use mori_recursion::rollup::batch::load_batch;
    let (r, w, instance) = load_batch(&corpus(), 1).unwrap();
    let k = optimal_k(&r);
    eprintln!("v3 batch k={k}");
    assert!(k <= 20, "v3 batch requires splitting");
    let run = |w| {
        let c = MidnightCircuit::new(&r, Value::known(instance.clone()), Value::known(w), Some(k));
        MockProver::run(&c, vec![vec![], instance.clone()])
            .unwrap()
            .verify()
            .is_ok()
    };
    assert!(run(w.clone()));
    let mut changed = w.clone();
    changed.headers[0].rh_append_path[0] += mori_recursion::util::F::ONE;
    assert!(!run(changed), "empty header close path accepted");
    let mut changed = w.clone();
    changed.headers[3].txs[0]
        .public
        .signature
        .as_mut()
        .unwrap()
        .s += mori_recursion::rollup::i1::fp::JFr::ONE;
    assert!(!run(changed), "signature mutation accepted");
    let mut changed = w;
    changed.pre_commitments[0] += mori_recursion::util::F::ONE;
    assert!(!run(changed), "unbound commitment set accepted");
}

fn corpus() -> PathBuf {
    std::env::var_os("FRTF_V3_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../../projectfenrir-main/recursion/rollup-test-format/v3")
        })
}

fn close(height: u64) -> (CloseWitness, Expected) {
    let root = corpus();
    let header: Header = serde_json::from_slice(
        &fs::read(root.join(format!("i1/blocks/{height:06}.json"))).unwrap(),
    )
    .unwrap();
    let expected: Expected = serde_json::from_slice(
        &fs::read(root.join(format!("i1/expected/{height:06}.json"))).unwrap(),
    )
    .unwrap();
    let before = header
        .txs
        .last()
        .map(|t| t.state_after.clone())
        .unwrap_or(header.state_before);
    (
        CloseWitness {
            before,
            after: expected.state_after.clone(),
            height,
            path: header.rh_append_path,
        },
        expected,
    )
}

fn accepts(w: CloseWitness) -> bool {
    let relation = CloseRelation;
    let instance = w.instance();
    let circuit = MidnightCircuit::new(
        &relation,
        Value::known(instance.clone()),
        Value::known(w),
        Some(optimal_k(&relation)),
    );
    MockProver::run(&circuit, vec![vec![], instance])
        .unwrap()
        .verify()
        .is_ok()
}

#[test]
fn header_close_matches_b2_including_empty_headers() {
    for h in 1..=50 {
        let (w, expected) = close(h);
        assert_eq!(w.after.digest(), expected.post_digest);
        assert!(accepts(w), "honest close {h}");
    }
}

#[test]
fn header_close_rejects_rehashed_mutations() {
    for h in [1, 4, 8, 50] {
        let (base, _) = close(h);
        let mut cases = Vec::new();
        let mut w = base.clone();
        w.path[3] += mori_recursion::util::F::ONE;
        cases.push(w);
        let mut w = base.clone();
        w.after.rh_count += 1;
        cases.push(w);
        let mut w = base.clone();
        w.after.cm_root += mori_recursion::util::F::ONE;
        cases.push(w);
        let mut w = base.clone();
        w.after.rh_root += mori_recursion::util::F::ONE;
        cases.push(w);
        let mut w = base.clone();
        w.after.fee_total += 1;
        cases.push(w);
        let mut w = base.clone();
        w.after.height += 1;
        cases.push(w);
        let mut w = base;
        w.height += 1;
        cases.push(w);
        for (i, w) in cases.into_iter().enumerate() {
            assert!(!accepts(w), "accepted close mutation {h}/{i}");
        }
    }
}
