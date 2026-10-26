//! B1 corpus producer and verifier-free v3 full-batch relation.
//! Witnesses stay in memory. The independent oracle supplies the expected states;
//! the circuit proves every transaction and every intervening header close.
use super::{
    close::{assign_state, close_header, hash, range},
    compose::compose_assigned,
    i1::{
        fp::{self, F, dom},
        oracle::Oracle,
        types::{Expected, Genesis, Header, StateVec, Tx},
    },
    segment::{Segment, compose},
    transition::{TransactionWitness, transition},
    user::{A, USER_FIELDS, user_rules},
};
use crate::util::{f_from_hex, read_u32};
use ff::Field;
use midnight_circuits::instructions::*;
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};
use midnight_zk_stdlib::{Relation, ZkStdLib, ZkStdLibArch};
use std::{fs, io, path::Path};

/// Header transaction counts are key-bound circuit shape, never a witness branch.
#[derive(Clone, Debug)]
pub struct BatchRelation {
    pub header_txs: Vec<u8>,
    pub pre_commitments: u32,
}

#[derive(Clone, Debug)]
pub struct BatchWitness {
    pub headers: Vec<Header>,
    pub after_headers: Vec<StateVec>,
    pub wallets: Vec<Tx>,
    /// Complete dense initial commitment prefix. Its root and count are constrained,
    /// enabling the v3 duplicate-commitment rule without an unauthenticated host set.
    pub pre_commitments: Vec<F>,
}

fn invalid(msg: impl std::fmt::Display) -> io::Error {
    io::Error::other(msg.to_string())
}
fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> io::Result<T> {
    serde_json::from_slice(&fs::read(path)?).map_err(io::Error::other)
}

/// Replay from genesis using the B2 oracle, compare every expected state, then
/// compare the complete 17-lane statement to the Profile A envelope.
pub fn load_batch(root: &Path, batch_no: u64) -> io::Result<(BatchRelation, BatchWitness, Vec<F>)> {
    if !(1..=6).contains(&batch_no) {
        return Err(invalid("S0 batch must be 1..=6"));
    }
    let genesis: Genesis = read_json(&root.join("i1/genesis.json"))?;
    let mut oracle = Oracle::from_genesis(&genesis).map_err(invalid)?;
    let mut commitments = genesis.notes.iter().map(|n| n.cm).collect::<Vec<_>>();
    let mut pre_commitments = Vec::new();
    let mut headers = Vec::new();
    let mut after_headers = Vec::new();
    let first = (batch_no - 1) * 50 + 1;
    for h in 1..=batch_no * 50 {
        if h == first {
            pre_commitments = commitments.clone();
        }
        let header: Header = read_json(&root.join(format!("i1/blocks/{h:06}.json")))?;
        let expected: Expected = read_json(&root.join(format!("i1/expected/{h:06}.json")))?;
        let actual = oracle.apply_header(&header).map_err(invalid)?;
        if actual != expected {
            return Err(invalid(format!("B2 expected state mismatch at header {h}")));
        }
        for tx in &header.txs {
            commitments.extend(
                tx.public
                    .commitments
                    .iter()
                    .filter(|c| c.enabled)
                    .map(|c| c.value),
            );
        }
        if h >= first {
            headers.push(header);
            after_headers.push(actual.state_after);
        }
    }
    let filler = headers
        .iter()
        .flat_map(|h| &h.txs)
        .find(|t| !t.public.kind.is_deposit())
        .ok_or_else(|| invalid("batch has no wallet filler"))?
        .clone();
    let wallets = headers
        .iter()
        .flat_map(|h| &h.txs)
        .map(|t| {
            if t.public.kind.is_deposit() {
                filler.clone()
            } else {
                t.clone()
            }
        })
        .collect();
    let relation = BatchRelation {
        header_txs: headers.iter().map(|h| h.txs.len() as u8).collect(),
        pre_commitments: pre_commitments.len().try_into().map_err(invalid)?,
    };
    relation.validate()?;
    let witness = BatchWitness {
        headers,
        after_headers,
        wallets,
        pre_commitments,
    };
    let envelope: serde_json::Value =
        read_json(&root.join(format!("profile-a/batches/batch-{batch_no:02}.json")))?;
    let expected = envelope["a2_statement"]
        .as_array()
        .ok_or_else(|| invalid("missing a2 statement"))?
        .iter()
        .map(|s| {
            f_from_hex(
                s.as_str()
                    .ok_or_else(|| invalid("statement lane is not text"))?,
            )
        })
        .collect::<io::Result<Vec<_>>>()?;
    if relation.instance(&witness)? != expected {
        return Err(invalid("producer differs from exact Profile A statement"));
    }
    Ok((relation, witness, expected))
}

impl BatchRelation {
    pub fn validate(&self) -> io::Result<()> {
        if self.header_txs.len() != 50
            || self.header_txs.iter().any(|&n| n > 16)
            || self.header_txs.iter().map(|&n| n as usize).sum::<usize>() != 16
            || !(1..=1024).contains(&self.pre_commitments)
        {
            return Err(invalid(
                "S0 relation requires 50 headers, 16 transactions and 1..=1024 initial commitments",
            ));
        }
        Ok(())
    }
    fn shape_ok(&self, w: &BatchWitness) -> bool {
        w.headers.len() == 50
            && w.after_headers.len() == 50
            && w.wallets.len() == 16
            && w.pre_commitments.len() == self.pre_commitments as usize
            && w.headers
                .iter()
                .zip(&self.header_txs)
                .all(|(h, &n)| h.txs.len() == n as usize)
    }
    pub fn instance(&self, w: &BatchWitness) -> io::Result<Vec<F>> {
        self.validate()?;
        if !self.shape_ok(w) {
            return Err(invalid("batch witness shape"));
        }
        let mut segs: Vec<Segment> = Vec::new();
        let mut pending: Option<Segment> = None;
        let mut state = w.headers[0].state_before.clone();
        let initial = state.digest();
        for (hi, h) in w.headers.iter().enumerate() {
            for t in &h.txs {
                let pre = if let Some(mut previous) = pending.take() {
                    previous.fields[2] = state.digest();
                    segs.push(previous);
                    state.digest()
                } else {
                    initial
                };
                let p = &t.public;
                pending = Some(Segment {
                    fields: [
                        F::ZERO,
                        pre,
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
                    ],
                });
                state = t.state_after.clone();
            }
            state = w.after_headers[hi].clone();
        }
        let mut last = pending.ok_or_else(|| invalid("no transaction"))?;
        last.fields[2] = state.digest();
        segs.push(last);
        let level = segs
            .chunks(4)
            .map(compose)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| invalid(format!("{e:?}")))?;
        let root = compose(&level).map_err(|e| invalid(format!("{e:?}")))?;
        Ok([
            root.fields[1..].to_vec(),
            vec![F::from(w.headers[0].height), F::from(w.headers[49].height)],
        ]
        .concat())
    }
}

impl Relation for BatchRelation {
    type Instance = Vec<F>;
    type Witness = BatchWitness;
    type Error = Error;
    fn used_chips(&self) -> ZkStdLibArch {
        ZkStdLibArch {
            poseidon: true,
            jubjub: true,
            nr_pow2range_cols: 4,
            ..Default::default()
        }
    }
    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }
    fn circuit(
        &self,
        s: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _: Value<Vec<F>>,
        w: Value<BatchWitness>,
    ) -> Result<(), Error> {
        self.validate()
            .map_err(|e| Error::Synthesis(e.to_string()))?;
        w.error_if_known_and(|w| !self.shape_ok(w))?;
        let zero: A = s.assign_fixed(l, F::ZERO)?;
        let mut state = assign_state(s, l, w.as_ref().map(|w| w.headers[0].state_before.clone()))?;
        let initial = hash(s, l, dom::state(), &state)?;
        let first: A = s.assign(l, w.as_ref().map(|w| F::from(w.headers[0].height)))?;
        range(s, l, &first, 64)?;
        let pre_height = s.add_constant(l, &first, -F::ONE)?;
        range(s, l, &pre_height, 64)?;
        s.assert_equal(l, &state[8], &pre_height)?;
        let count = s.add_constant(l, &pre_height, F::ONE)?;
        s.assert_equal(l, &state[5], &count)?;
        let batch: A = s.assign(l, w.as_ref().map(|w| F::from(w.headers[0].batch)))?;
        range(s, l, &batch, 64)?;
        let batch_pre = s.add_constant(l, &batch, -F::ONE)?;
        range(s, l, &batch_pre, 64)?;
        let start = s.mul_by_constant(l, &batch_pre, F::from(50))?;
        s.assert_equal(l, &start, &pre_height)?;
        // Authenticate the full initial commitment set against the state's tree.
        let mut cms = (0..self.pre_commitments as usize)
            .map(|i| s.assign(l, w.as_ref().map(|w| w.pre_commitments[i])))
            .collect::<Result<Vec<A>, _>>()?;
        for cm in &cms {
            s.assert_not_equal_to_fixed(l, cm, F::ZERO)?;
        }
        s.assert_equal_to_fixed(l, &state[1], F::from(self.pre_commitments as u64))?;
        let mut leaves = cms.clone();
        leaves.resize(leaves.len().next_power_of_two(), zero.clone());
        let mut empty = zero.clone();
        for _ in 0..fp::CM_DEPTH {
            if leaves.len() == 1 {
                leaves.push(empty.clone());
            }
            leaves = leaves
                .as_chunks::<2>()
                .0
                .iter()
                .map(|p| hash(s, l, dom::node(), p))
                .collect::<Result<Vec<_>, _>>()?;
            empty = hash(s, l, dom::node(), &[empty.clone(), empty])?;
        }
        s.assert_equal(l, &leaves[0], &state[0])?;
        let mut segs = Vec::new();
        let mut pending: Option<Vec<A>> = None;
        let mut wi = 0;
        for (hi, &n_tx) in self.header_txs.iter().enumerate() {
            let height = s.add_constant(l, &first, F::from(hi as u64))?;
            let supplied: A = s.assign(l, w.as_ref().map(|w| F::from(w.headers[hi].height)))?;
            s.assert_equal(l, &height, &supplied)?;
            let hb: A = s.assign(l, w.as_ref().map(|w| F::from(w.headers[hi].batch)))?;
            s.assert_equal(l, &batch, &hb)?;
            let time = s.mul_by_constant(l, &height, F::from(fp::HEADER_SECS))?;
            let time = s.add_constant(l, &time, F::from(fp::T0))?;
            range(s, l, &time, 64)?;
            let supplied: A = s.assign(l, w.as_ref().map(|w| F::from(w.headers[hi].time)))?;
            s.assert_equal(l, &time, &supplied)?;
            let header_pre =
                assign_state(s, l, w.as_ref().map(|w| w.headers[hi].state_before.clone()))?;
            for (a, b) in state.iter().zip(&header_pre) {
                s.assert_equal(l, a, b)?;
            }
            for ti in 0..n_tx as usize {
                let pre = if let Some(mut previous) = pending.take() {
                    previous[2] = hash(s, l, dom::state(), &state)?;
                    segs.push(previous);
                    hash(s, l, dom::state(), &state)?
                } else {
                    initial.clone()
                };
                let tx = w.as_ref().map(|w| w.headers[hi].txs[ti].clone());
                let l0 = user_rules(s, l, w.as_ref().map(|w| w.wallets[wi].clone()))?;
                let tw = tx.as_ref().map(|t| TransactionWitness {
                    pre_height: t.state_before.height,
                    post_height: t.state_after.height,
                    tx: t.clone(),
                });
                let (mut seg, pi, deposit, before, after) = transition(s, l, tw)?;
                let spend = s.not(l, &deposit)?;
                for j in 0..USER_FIELDS {
                    s.cond_assert_equal(l, &spend, &l0[j], &pi[j])?;
                }
                for (a, b) in state.iter().zip(&before) {
                    s.assert_equal(l, a, b)?;
                }
                s.assert_equal(l, &after[8], &height)?;
                let position = s.mul_by_constant(l, &height, F::from(65536))?;
                let position = s.add_constant(l, &position, F::from(ti as u64))?;
                s.assert_equal(l, &seg[3], &position)?;
                let tb: A = s.assign(l, tx.as_ref().map(|t| F::from(t.public.batch)))?;
                s.assert_equal(l, &batch, &tb)?;
                // Every enabled output differs from the authenticated initial set
                // and all earlier outputs, including the other output of this tx.
                for j in 0..2 {
                    let absent = s.is_equal_to_fixed(l, &pi[3 + j], F::ZERO)?;
                    let enabled = s.not(l, &absent)?;
                    for old in &cms {
                        let eq = s.is_equal(l, &pi[3 + j], old)?;
                        let bad = s.and(l, &[enabled.clone(), eq])?;
                        s.assert_false(l, &bad)?;
                    }
                    cms.push(pi[3 + j].clone());
                }
                state = after;
                seg[1] = pre;
                pending = Some(seg);
                wi += 1;
            }
            state = close_header(
                s,
                l,
                &state,
                &height,
                w.as_ref().map(|w| w.headers[hi].rh_append_path.clone()),
            )?;
            let expected = assign_state(s, l, w.as_ref().map(|w| w.after_headers[hi].clone()))?;
            for (a, b) in state.iter().zip(&expected) {
                s.assert_equal(l, a, b)?;
            }
        }
        let mut last = pending.ok_or_else(|| Error::Synthesis("no transactions".into()))?;
        last[2] = hash(s, l, dom::state(), &state)?;
        segs.push(last);
        let level = segs
            .chunks(4)
            .map(|cs| compose_assigned(s, l, cs))
            .collect::<Result<Vec<_>, _>>()?;
        let root = compose_assigned(s, l, &level)?;
        for c in &root[1..] {
            s.constrain_as_public_input(l, c)?;
        }
        s.constrain_as_public_input(l, &first)?;
        s.constrain_as_public_input(l, &state[8])?;
        Ok(())
    }
    fn write_relation<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        self.validate()?;
        w.write_all(b"FSYNB001")?;
        w.write_all(&self.pre_commitments.to_le_bytes())?;
        w.write_all(&self.header_txs)
    }
    fn read_relation<R: io::Read>(r: &mut R) -> io::Result<Self> {
        let mut magic = [0; 8];
        r.read_exact(&mut magic)?;
        if &magic != b"FSYNB001" {
            return Err(invalid("invalid v3 batch relation"));
        }
        let pre_commitments = read_u32(r)?;
        let mut header_txs = vec![0; 50];
        r.read_exact(&mut header_txs)?;
        let mut rest = Vec::new();
        r.read_to_end(&mut rest)?;
        if !rest.is_empty() {
            return Err(invalid("trailing batch relation bytes"));
        }
        let relation = Self {
            pre_commitments,
            header_txs,
        };
        relation.validate()?;
        Ok(relation)
    }
}
