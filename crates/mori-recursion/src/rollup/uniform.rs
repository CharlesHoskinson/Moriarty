//! One fixed S0 key for every 16-transaction/50-header batch in the six-batch corpus.
//! A complete permutation of 66 edge indices binds operations to one committed
//! state-and-slot trace. The transcript uses the existing Poseidon state assumption.
use super::{
    batch::{BatchRelation, BatchWitness, load_batch},
    close::{assign_state, close_header, hash, range},
    compose::compose_assigned,
    i1::{
        fp::{self, F, dom},
        types::StateVec,
    },
    trace::{bind_event, ordered_indices, unique_indices},
    transition::{TransactionWitness, transition},
    user::{A, USER_FIELDS, user_rules},
};
use ff::Field;
use midnight_circuits::{instructions::*, types::AssignedBit};
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};
use midnight_zk_stdlib::{Relation, ZkStdLib, ZkStdLibArch};
use std::{io, path::Path};

pub const INITIAL_COMMITMENTS: usize = 512;
pub const TRANSACTIONS: usize = 16;
pub const HEADERS: usize = 50;
pub const EVENTS: usize = TRANSACTIONS + HEADERS;
const EVENT_DOMAIN: &str = "SHK_EVENT";

#[derive(Clone, Debug)]
pub struct UniformBatch;
#[derive(Clone, Debug)]
pub struct UniformWitness {
    pub batch: BatchWitness,
    pub states: Vec<StateVec>,
    pub slots: Vec<u64>,
    pub tx_events: Vec<u32>,
    pub close_events: Vec<u32>,
}
fn bad(message: impl std::fmt::Display) -> io::Error {
    io::Error::other(message.to_string())
}
fn event_hash(state: &StateVec, slot: u64) -> F {
    fp::hash(
        &[
            vec![fp::field_from_tag(EVENT_DOMAIN)],
            state.fields().to_vec(),
            vec![F::from(slot)],
        ]
        .concat(),
    )
}
fn assigned_event(
    s: &ZkStdLib,
    l: &mut impl Layouter<F>,
    state: &[A],
    slot: &A,
) -> Result<A, Error> {
    hash(
        s,
        l,
        fp::field_from_tag(EVENT_DOMAIN),
        &[state.to_vec(), vec![slot.clone()]].concat(),
    )
}

impl UniformWitness {
    pub fn from_batch(batch: BatchWitness) -> io::Result<Self> {
        if batch.headers.len() != HEADERS
            || batch.after_headers.len() != HEADERS
            || batch.wallets.len() != TRANSACTIONS
            || batch.headers.iter().map(|h| h.txs.len()).sum::<usize>() != TRANSACTIONS
            || batch.pre_commitments.is_empty()
            || batch.pre_commitments.len() > INITIAL_COMMITMENTS
        {
            return Err(bad("uniform S0 witness shape"));
        }
        let mut states = vec![batch.headers[0].state_before.clone()];
        let mut slots = vec![0];
        let mut tx_events = Vec::new();
        let mut close_events = Vec::new();
        for (hi, h) in batch.headers.iter().enumerate() {
            for (ti, t) in h.txs.iter().enumerate() {
                tx_events.push((states.len() - 1) as u32);
                states.push(t.state_after.clone());
                slots.push((ti + 1) as u64);
            }
            close_events.push((states.len() - 1) as u32);
            states.push(batch.after_headers[hi].clone());
            slots.push(0);
        }
        Ok(Self {
            batch,
            states,
            slots,
            tx_events,
            close_events,
        })
    }
}
impl UniformBatch {
    pub fn load(root: &Path, batch_no: u64) -> io::Result<(Self, UniformWitness, Vec<F>)> {
        let (_, batch, expected) = load_batch(root, batch_no)?;
        Ok((Self, UniformWitness::from_batch(batch)?, expected))
    }
    pub fn instance(&self, w: &UniformWitness) -> io::Result<Vec<F>> {
        let relation = BatchRelation {
            header_txs: w.batch.headers.iter().map(|h| h.txs.len() as u8).collect(),
            pre_commitments: w.batch.pre_commitments.len().try_into().map_err(bad)?,
        };
        relation.instance(&w.batch)
    }
    fn shape(w: &UniformWitness) -> bool {
        w.states.len() == EVENTS + 1
            && w.slots.len() == EVENTS + 1
            && w.tx_events.len() == TRANSACTIONS
            && w.close_events.len() == HEADERS
            && w.batch.headers.len() == HEADERS
            && w.batch.after_headers.len() == HEADERS
            && w.batch.wallets.len() == TRANSACTIONS
            && w.batch.headers.iter().map(|h| h.txs.len()).sum::<usize>() == TRANSACTIONS
            && !w.batch.pre_commitments.is_empty()
            && w.batch.pre_commitments.len() <= INITIAL_COMMITMENTS
    }
}

impl Relation for UniformBatch {
    type Instance = Vec<F>;
    type Witness = UniformWitness;
    type Error = Error;
    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }
    fn used_chips(&self) -> ZkStdLibArch {
        ZkStdLibArch {
            poseidon: true,
            jubjub: true,
            nr_pow2range_cols: 4,
            ..Default::default()
        }
    }
    fn circuit(
        &self,
        s: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _: Value<Vec<F>>,
        w: Value<UniformWitness>,
    ) -> Result<(), Error> {
        w.error_if_known_and(|w| !Self::shape(w))?;
        let zero: A = s.assign_fixed(l, F::ZERO)?;
        let pre = assign_state(s, l, w.as_ref().map(|w| w.states[0].clone()))?;
        let post = assign_state(s, l, w.as_ref().map(|w| w.states[EVENTS].clone()))?;
        let pre_root = hash(s, l, dom::state(), &pre)?;
        let post_root = hash(s, l, dom::state(), &post)?;
        let first = s.add_constant(l, &pre[8], F::ONE)?;
        range(s, l, &first, 64)?;
        let last = s.add_constant(l, &first, F::from(49))?;
        s.assert_equal(l, &last, &post[8])?;
        s.assert_equal(l, &pre[5], &first)?;
        let final_count = s.add_constant(l, &last, F::ONE)?;
        s.assert_equal(l, &final_count, &post[5])?;
        let batch: A = s.assign(l, w.as_ref().map(|w| F::from(w.batch.headers[0].batch)))?;
        let previous = s.add_constant(l, &batch, -F::ONE)?;
        range(s, l, &previous, 64)?;
        let previous_height = s.mul_by_constant(l, &previous, F::from(50))?;
        s.assert_equal(l, &previous_height, &pre[8])?;
        // Each private trace lane is a commitment to all 12 state fields and
        // the next slot. Both endpoints open to the public state digests.
        let trace = (0..=EVENTS)
            .map(|i| s.assign(l, w.as_ref().map(|w| event_hash(&w.states[i], w.slots[i]))))
            .collect::<Result<Vec<A>, _>>()?;
        let initial_event = assigned_event(s, l, &pre, &zero)?;
        s.assert_equal(l, &trace[0], &initial_event)?;
        let final_event = assigned_event(s, l, &post, &zero)?;
        s.assert_equal(l, &trace[EVENTS], &final_event)?;
        // Fixed-capacity, authenticated dense prefix. Prefix flags sum to
        // cm_count and cannot turn on again after the first zero.
        let mut cms = Vec::with_capacity(INITIAL_COMMITMENTS + 2 * TRANSACTIONS);
        let mut count = zero.clone();
        let mut previous_enabled: AssignedBit<F> = s.assign_fixed(l, true)?;
        for i in 0..INITIAL_COMMITMENTS {
            let enabled: AssignedBit<F> =
                s.assign(l, w.as_ref().map(|w| i < w.batch.pre_commitments.len()))?;
            if i == 0 {
                s.assert_true(l, &enabled)?;
            }
            let previous_off = s.not(l, &previous_enabled)?;
            let resumed = s.and(l, &[previous_off, enabled.clone()])?;
            s.assert_false(l, &resumed)?;
            previous_enabled = enabled.clone();
            let cm: A = s.assign(
                l,
                w.as_ref()
                    .map(|w| w.batch.pre_commitments.get(i).copied().unwrap_or(F::ZERO)),
            )?;
            let off = s.not(l, &enabled)?;
            s.cond_assert_equal(l, &off, &cm, &zero)?;
            let empty = s.is_equal_to_fixed(l, &cm, F::ZERO)?;
            let invalid = s.and(l, &[enabled.clone(), empty])?;
            s.assert_false(l, &invalid)?;
            let bit: A = s.convert(l, &enabled)?;
            count = s.add(l, &count, &bit)?;
            cms.push(cm);
        }
        s.assert_equal(l, &count, &pre[1])?;
        let mut leaves = cms.clone();
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
        s.assert_equal(l, &leaves[0], &pre[0])?;
        let mut segs = Vec::new();
        let mut raw_pre = Vec::new();
        let mut tx_indices = Vec::new();
        for i in 0..TRANSACTIONS {
            let tx = w.as_ref().map(|w| {
                w.batch
                    .headers
                    .iter()
                    .flat_map(|h| &h.txs)
                    .nth(i)
                    .unwrap()
                    .clone()
            });
            let wallet = user_rules(s, l, w.as_ref().map(|w| w.batch.wallets[i].clone()))?;
            let tw = tx.as_ref().map(|t| TransactionWitness {
                tx: t.clone(),
                pre_height: t.state_before.height,
                post_height: t.state_after.height,
            });
            let (seg, public, deposit, before, after) = transition(s, l, tw)?;
            let declared_batch: A = s.assign(l, tx.as_ref().map(|t| F::from(t.public.batch)))?;
            s.assert_equal(l, &declared_batch, &batch)?;
            let spend = s.not(l, &deposit)?;
            for j in 0..USER_FIELDS {
                s.cond_assert_equal(l, &spend, &wallet[j], &public[j])?;
            }
            let start = s.sub(l, &after[8], &first)?;
            range(s, l, &start, 6)?;
            let end = s.sub(l, &last, &after[8])?;
            range(s, l, &end, 6)?;
            let slot: A = s.assign(l, tx.as_ref().map(|t| F::from(t.public.slot)))?;
            range(s, l, &slot, 5)?;
            let position = s.mul_by_constant(l, &after[8], F::from(65536))?;
            let position = s.add(l, &position, &slot)?;
            s.assert_equal(l, &seg[3], &position)?;
            let next = s.add_constant(l, &slot, F::ONE)?;
            let a = assigned_event(s, l, &before, &slot)?;
            let b = assigned_event(s, l, &after, &next)?;
            let index: A = s.assign(l, w.as_ref().map(|w| F::from(w.tx_events[i] as u64)))?;
            bind_event(s, l, &trace, &index, &a, &b)?;
            tx_indices.push(index);
            for j in 0..2 {
                let absent = s.is_equal_to_fixed(l, &public[3 + j], F::ZERO)?;
                let enabled = s.not(l, &absent)?;
                for old in &cms {
                    let equal = s.is_equal(l, &public[3 + j], old)?;
                    let duplicate = s.and(l, &[enabled.clone(), equal])?;
                    s.assert_false(l, &duplicate)?;
                }
                cms.push(public[3 + j].clone());
            }
            raw_pre.push(seg[1].clone());
            segs.push(seg);
        }
        let mut close_indices = Vec::new();
        let mut previous_closed = pre.clone();
        for hi in 0..HEADERS {
            let height = s.add_constant(l, &first, F::from(hi as u64))?;
            let header = w.as_ref().map(|w| w.batch.headers[hi].clone());
            let declared_height: A = s.assign(l, header.as_ref().map(|h| F::from(h.height)))?;
            s.assert_equal(l, &height, &declared_height)?;
            let declared_batch: A = s.assign(l, header.as_ref().map(|h| F::from(h.batch)))?;
            s.assert_equal(l, &declared_batch, &batch)?;
            let declared_before =
                assign_state(s, l, header.as_ref().map(|h| h.state_before.clone()))?;
            for (a, b) in declared_before.iter().zip(&previous_closed) {
                s.assert_equal(l, a, b)?;
            }
            let time = s.mul_by_constant(l, &height, F::from(fp::HEADER_SECS))?;
            let time = s.add_constant(l, &time, F::from(fp::T0))?;
            range(s, l, &time, 64)?;
            let declared_time: A = s.assign(l, header.as_ref().map(|h| F::from(h.time)))?;
            s.assert_equal(l, &time, &declared_time)?;
            let before = assign_state(
                s,
                l,
                header.as_ref().map(|h| {
                    h.txs
                        .last()
                        .map(|t| t.state_after.clone())
                        .unwrap_or_else(|| h.state_before.clone())
                }),
            )?;
            s.assert_equal(l, &before[5], &height)?;
            let after = close_header(
                s,
                l,
                &before,
                &height,
                header.as_ref().map(|h| h.rh_append_path.clone()),
            )?;
            let expected =
                assign_state(s, l, w.as_ref().map(|w| w.batch.after_headers[hi].clone()))?;
            for (a, b) in after.iter().zip(&expected) {
                s.assert_equal(l, a, b)?;
            }
            previous_closed = after.clone();
            let slot: A = s.assign(l, header.as_ref().map(|h| F::from(h.txs.len() as u64)))?;
            range(s, l, &slot, 5)?;
            let a = assigned_event(s, l, &before, &slot)?;
            let b = assigned_event(s, l, &after, &zero)?;
            let index: A = s.assign(l, w.as_ref().map(|w| F::from(w.close_events[hi] as u64)))?;
            bind_event(s, l, &trace, &index, &a, &b)?;
            close_indices.push(index);
        }
        ordered_indices(s, l, &tx_indices)?;
        ordered_indices(s, l, &close_indices)?;
        unique_indices(s, l, &[tx_indices, close_indices].concat())?;
        for (i, seg) in segs.iter_mut().enumerate() {
            seg[1] = if i == 0 {
                pre_root.clone()
            } else {
                raw_pre[i].clone()
            };
            seg[2] = if i + 1 == TRANSACTIONS {
                post_root.clone()
            } else {
                raw_pre[i + 1].clone()
            };
        }
        let level = segs
            .chunks(4)
            .map(|cs| compose_assigned(s, l, cs))
            .collect::<Result<Vec<_>, _>>()?;
        let root = compose_assigned(s, l, &level)?;
        for cell in &root[1..] {
            s.constrain_as_public_input(l, cell)?;
        }
        s.constrain_as_public_input(l, &first)?;
        s.constrain_as_public_input(l, &last)?;
        Ok(())
    }
    fn write_relation<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"FSYNB002")
    }
    fn read_relation<R: io::Read>(r: &mut R) -> io::Result<Self> {
        let mut magic = [0; 8];
        r.read_exact(&mut magic)?;
        if &magic != b"FSYNB002" {
            return Err(bad("invalid uniform S0 relation"));
        }
        Ok(Self)
    }
}
