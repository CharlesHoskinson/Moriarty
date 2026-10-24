//! PROPOSED C6R2 range parent P_k (first handwritten C6R2 circuit change).
//! NEW_PROPOSED_NOT_ADOPTED_UNEXECUTED: source only, uncompiled and unrun. No key,
//! proof, VK, SRS or digest exists for it and none is chosen here. It is not
//! exported and not wired into any stage, W or D. Two scoped reviews agree on this
//! interface; the full16 design has zero whole-design votes. A fresh nonauthor
//! source audit is required; the C6R1 scoped approval does not carry over.
//!
//! P_k fixes its level k, the range-root level m and ONE child key VK_{k-1} for
//! both sides. P_1 children are leaves, ChildShape{20, Leaf}: the instance is
//! [binding_input] ++ 19 semantic lanes, and E1 pins the free binding_input to 0.
//! P_k (k >= 2) children are P_{k-1}, ChildShape{19, Recursive}: 19 + 12 carry
//! lanes. C6R2_STMT_LANES = 19 is the SEMANTIC count (the codec field); the
//! FRAMING count ChildShape.statement_lanes is frame + 19. Both complete child
//! instances are assigned once, both proofs are verified on THOSE cells, and E0-E8
//! run on the same cells. The 19 parent lanes, then the 12 combined-accumulator
//! lanes, are public. The pairing is deferred to a terminal W and the mandatory D.
//! E7 holds only at level m: R.TX_HI - L.TX_LO == 4 and exactly one of
//! R.POST_AT_TX_ENTRY, R.HAS_FINAL. The parent treats TX_* and POST_AT_TX_ENTRY as
//! one abstract predicate contract. The leaf predicate A(S) (the before-state of
//! BHelperBindEntry at Enter{Some(apply_tx)}), its uniqueness and every other leaf,
//! W, D, program, financial or full16 property are NOT established here.
//!
//! m is in 1..=253 (Reviewer B), so h = m + 2 and every level fit in u8. The level
//! bound is also compared in u16 (Reviewer A). K_k binds VK_{k-1} through
//! transcript_repr, which hashes VERSION, k, the fixed and permutation commitments
//! and Debug(domain.pinned(), cs.pinned()) (midnight-proofs 0.8.2 from_parts).

use std::io;

use ff::{Field, PrimeField};
use midnight_circuits::{instructions::*, types::AssignedNative};
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};
use midnight_zk_stdlib::{MidnightVK, Relation, ZkStdLib, ZkStdLibArch};

use crate::{
    c6_binary_verify::{prepare_binary_deferred, verify_binary_incircuit, ChildKind, ChildShape, FixedChild},
    compose::range,
    decider::bls_arch,
    inner::{acc_pi_len, acc_pis},
    sherlock_link::{dom, poseidon},
    util::{read_bytes, read_f, read_u32, vk_bytes, vk_from_bytes, write_bytes, write_f, F},
};

type A = AssignedNative<F>;

/// PROPOSED serialization magic: c6r2 parent relation, format 1.
const MAGIC: &[u8; 8] = b"C6R2P001";
/// S1: child-key byte bound, as in the existing recl9 relation readers.
const MAX_CHILD_VK_BYTES: usize = 1 << 20;
/// S2: child degree bound (sherlock_link automatic_fitting_model probe 9..=25).
pub const C6R2_MAX_CHILD_K: u8 = 25;
/// midnight-proofs 0.8.2 VerifyingKey VERSION byte.
const INNER_VK_VERSION: u8 = 0x03;
/// h = m + 2 must fit in u8.
pub const C6R2_MAX_RANGE_ROOT: u8 = 253;

pub const C6R2_STMT_LANES: usize = 19;
pub const C6R2_LEAF_FRAME_LANES: usize = 1;
/// NEW_PROPOSED tags, packed by sherlock_link::dom (at most 16 bytes).
pub const C6R2_SCHEMA_TAG: &str = "C6R2.STMT";
pub const C6R2_KEYSET_TAG: &str = "C6R2.KEYSET";
pub const C6R2_ROLE_LEAF: u8 = 1;
pub const C6R2_ROLE_PARENT: u8 = 2;

const MODE: &str = "child mode differs from level: P_1 takes Leaf, P_k (k >= 2) takes Recursive";

/// Semantic lane indices, counted after any leaf frame.
pub mod lane {
    pub const SCHEMA: usize = 0;
    pub const ROLE: usize = 1;
    pub const LEVEL: usize = 2;
    pub const KEYSET: usize = 3;
    pub const PROG: usize = 4;
    pub const ADMIT: usize = 5;
    pub const PRE_MACHINE: usize = 6;
    pub const POST_MACHINE: usize = 7;
    pub const TX_LO: usize = 8;
    pub const TX_HI: usize = 9;
    pub const MICRO_LO: usize = 10;
    pub const MICRO_HI: usize = 11;
    pub const FUEL_PRE: usize = 12;
    pub const FUEL_POST: usize = 13;
    pub const CONSUMED: usize = 14;
    pub const HELPER_WORK: usize = 15;
    pub const POST_AT_TX_ENTRY: usize = 16;
    pub const HAS_FINAL: usize = 17;
    pub const TERMINAL_CLASS: usize = 18;
}
use lane::*;

pub fn c6r2_schema() -> F {
    dom(C6R2_SCHEMA_TAG)
}

/// K_k = Poseidon([dom(C6R2.KEYSET), k, m, transcript_repr(VK_{k-1})]). Computed
/// off-circuit only; the circuit assigns it as a fixed constant.
pub fn c6r2_keyset(level: u8, m: u8, child_vk: &MidnightVK) -> F {
    poseidon(&[dom(C6R2_KEYSET_TAG), F::from(level as u64), F::from(m as u64), child_vk.vk().transcript_repr()])
}

/// 19 parent lanes, then the 12-lane combined accumulator.
pub fn c6r2_public_len() -> usize {
    C6R2_STMT_LANES + acc_pi_len()
}

const SHARED: [(usize, &str); 2] = [(PROG, "prog"), (ADMIT, "admit")];
/// (left out lane, right in lane): L.out == R.in.
const CONTINUITY: [(usize, usize, &str); 4] = [
    (POST_MACHINE, PRE_MACHINE, "machine"),
    (TX_HI, TX_LO, "tx"),
    (MICRO_HI, MICRO_LO, "micro"),
    (FUEL_POST, FUEL_PRE, "fuel"),
];
/// (lo, hi, bits): on both children, both ends and hi - lo are range-checked.
const ORDERED: [(usize, usize, usize, &str); 2] = [(TX_LO, TX_HI, 32, "tx"), (MICRO_LO, MICRO_HI, 128, "micro")];
const U128_LANES: [(usize, &str); 4] =
    [(FUEL_PRE, "fuel_pre"), (FUEL_POST, "fuel_post"), (CONSUMED, "consumed"), (HELPER_WORK, "helper_work")];
const BOOLS: [(usize, &str); 2] = [(POST_AT_TX_ENTRY, "post_at_tx_entry"), (HAS_FINAL, "has_final")];
const SUMS: [(usize, &str); 2] = [(CONSUMED, "consumed"), (HELPER_WORK, "helper_work")];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Out {
    Schema,
    Role,
    Level,
    Keyset,
    L(usize),
    R(usize),
    Sum(usize),
}
/// E8, in lane order.
const OUTPUT: [Out; C6R2_STMT_LANES] = [
    Out::Schema, Out::Role, Out::Level, Out::Keyset,
    Out::L(PROG), Out::L(ADMIT), Out::L(PRE_MACHINE), Out::R(POST_MACHINE),
    Out::L(TX_LO), Out::R(TX_HI), Out::L(MICRO_LO), Out::R(MICRO_HI),
    Out::L(FUEL_PRE), Out::R(FUEL_POST), Out::Sum(CONSUMED), Out::Sum(HELPER_WORK),
    Out::R(POST_AT_TX_ENTRY), Out::R(HAS_FINAL), Out::R(TERMINAL_CLASS),
];

/// The parent-law constants that the circuit reads.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct C6R2ParentLaw {
    pub level: u8,
    pub range_root_level: u8,
    pub keyset_k: F,
    pub child_keyset: F,
}

impl C6R2ParentLaw {
    pub fn check(&self) -> Result<(), String> {
        let m = self.range_root_level;
        if self.level == 0 {
            return Err("c6r2 parent level must be >= 1".into());
        }
        if m == 0 || m > C6R2_MAX_RANGE_ROOT {
            return Err("c6r2 range-root level m must be in 1..=253".into());
        }
        if (self.level as u16) > (m as u16) + 2 {
            return Err("c6r2 parent level is above range-root level m + 2".into());
        }
        if self.level == 1 && self.child_keyset != F::ZERO {
            return Err("P_1 children are leaves: child keyset must be present-zero 0".into());
        }
        Ok(())
    }

    pub fn child_frame(&self) -> usize {
        if self.level == 1 { C6R2_LEAF_FRAME_LANES } else { 0 }
    }

    /// Framing count: frame + 19 semantic lanes.
    pub fn child_statement_lanes(&self) -> usize {
        self.child_frame() + C6R2_STMT_LANES
    }

    pub fn child_kind(&self) -> ChildKind {
        if self.level == 1 { ChildKind::Leaf } else { ChildKind::Recursive }
    }

    pub fn child_role(&self) -> u8 {
        if self.level == 1 { C6R2_ROLE_LEAF } else { C6R2_ROLE_PARENT }
    }

    /// Meaningful only after check (level >= 1).
    pub fn child_level(&self) -> u8 {
        self.level.saturating_sub(1)
    }

    pub fn is_range_root(&self) -> bool {
        self.level == self.range_root_level
    }

    pub fn child_shape(&self) -> ChildShape {
        ChildShape { statement_lanes: self.child_statement_lanes(), kind: self.child_kind() }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

/// Host-mirror refusal, in E0..E7 order. A refusal earns no proof credit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C6R2ParentRefusal {
    Parameters,
    Shape,
    Frame(Side),
    Schema(Side),
    Role(Side),
    Level(Side),
    Keyset(Side),
    Range(Side, &'static str),
    Order(Side, &'static str),
    NotBool(Side, &'static str),
    Shared(&'static str),
    Discontinuity(&'static str),
    FinalNotRightmost,
    TerminalBeforeEnd,
    Overflow(&'static str),
    RangeRootCount,
    RangeRootBoundary,
}

/// The integer value of x if x < 2^bits (bits a multiple of 8, at most 128).
fn le_width(x: &F, bits: usize) -> Option<u128> {
    let repr = x.to_repr();
    let b: &[u8] = repr.as_ref();
    let n = bits / 8;
    if n > 16 || b[n..].iter().any(|&v| v != 0) {
        return None;
    }
    let mut w = [0u8; 16];
    w[..n].copy_from_slice(&b[..n]);
    Some(u128::from_le_bytes(w))
}

/// E8 projection without any check. Inputs are the framed child statements.
pub(crate) fn project(law: &C6R2ParentLaw, left_full: &[F], right_full: &[F]) -> [F; C6R2_STMT_LANES] {
    let f = law.child_frame();
    let (l, r) = (&left_full[f..], &right_full[f..]);
    std::array::from_fn(|i| match OUTPUT[i] {
        Out::Schema => c6r2_schema(),
        Out::Role => F::from(C6R2_ROLE_PARENT as u64),
        Out::Level => F::from(law.level as u64),
        Out::Keyset => law.keyset_k,
        Out::L(j) => l[j],
        Out::R(j) => r[j],
        Out::Sum(j) => l[j] + r[j],
    })
}

/// Host mirror of constrain_c6r2_parent_lanes, checked in the same E0..E7 order.
/// It runs before proving and is never accepted in place of the circuit.
pub fn c6r2_parent_law(
    law: &C6R2ParentLaw,
    left_full: &[F],
    right_full: &[F],
) -> Result<[F; C6R2_STMT_LANES], C6R2ParentRefusal> {
    use C6R2ParentRefusal as E;
    law.check().map_err(|_| E::Parameters)?;
    let n = law.child_statement_lanes();
    if left_full.len() != n || right_full.len() != n {
        return Err(E::Shape);
    }
    let f = law.child_frame();
    if f == C6R2_LEAF_FRAME_LANES {
        for (side, c) in [(Side::Left, left_full), (Side::Right, right_full)] {
            if c[0] != F::ZERO {
                return Err(E::Frame(side));
            }
        }
    }
    let (l, r) = (&left_full[f..], &right_full[f..]);
    for (side, c) in [(Side::Left, l), (Side::Right, r)] {
        if c[SCHEMA] != c6r2_schema() {
            return Err(E::Schema(side));
        }
        if c[ROLE] != F::from(law.child_role() as u64) {
            return Err(E::Role(side));
        }
        if c[LEVEL] != F::from(law.child_level() as u64) {
            return Err(E::Level(side));
        }
        if c[KEYSET] != law.child_keyset {
            return Err(E::Keyset(side));
        }
        for &(lo, hi, bits, name) in &ORDERED {
            match (le_width(&c[lo], bits), le_width(&c[hi], bits)) {
                (Some(a), Some(b)) if a <= b => {}
                (Some(_), Some(_)) => return Err(E::Order(side, name)),
                _ => return Err(E::Range(side, name)),
            }
        }
        for &(i, name) in &U128_LANES {
            if le_width(&c[i], 128).is_none() {
                return Err(E::Range(side, name));
            }
        }
        for &(i, name) in &BOOLS {
            if c[i] != F::ZERO && c[i] != F::ONE {
                return Err(E::NotBool(side, name));
            }
        }
    }
    for &(i, name) in &SHARED {
        if l[i] != r[i] {
            return Err(E::Shared(name));
        }
    }
    for &(out, inn, name) in &CONTINUITY {
        if l[out] != r[inn] {
            return Err(E::Discontinuity(name));
        }
    }
    if l[HAS_FINAL] != F::ZERO {
        return Err(E::FinalNotRightmost);
    }
    if l[TERMINAL_CLASS] != F::ZERO {
        return Err(E::TerminalBeforeEnd);
    }
    for &(i, name) in &SUMS {
        let (a, b) = (le_width(&l[i], 128), le_width(&r[i], 128));
        if a.zip(b).and_then(|(a, b)| a.checked_add(b)).is_none() {
            return Err(E::Overflow(name));
        }
    }
    if law.is_range_root() {
        if r[TX_HI] - l[TX_LO] != F::from(4u64) {
            return Err(E::RangeRootCount);
        }
        if r[POST_AT_TX_ENTRY] + r[HAS_FINAL] != F::ONE {
            return Err(E::RangeRootBoundary);
        }
    }
    Ok(project(law, left_full, right_full))
}

fn assert_bool(s: &ZkStdLib, l: &mut impl Layouter<F>, x: &A) -> Result<(), Error> {
    let zero = s.is_equal_to_fixed(l, x, F::ZERO)?;
    let one = s.is_equal_to_fixed(l, x, F::ONE)?;
    let ok = s.or(l, &[zero, one])?;
    s.assert_true(l, &ok)
}

/// E0-E8 on the framed statement cells of each child. Callers MUST pass the same
/// cells that were verified as child instance lanes, without the carry tail.
/// Returns the 19 parent cells in lane order: fixed constants, aliases of the
/// input cells, and the two range-checked sums.
pub fn constrain_c6r2_parent_lanes(
    s: &ZkStdLib,
    l: &mut impl Layouter<F>,
    law: &C6R2ParentLaw,
    left_full: &[A],
    right_full: &[A],
) -> Result<Vec<A>, Error> {
    law.check().map_err(Error::Synthesis)?;
    // E0: complete framed statements.
    let n = law.child_statement_lanes();
    if left_full.len() != n || right_full.len() != n {
        return Err(Error::Synthesis("c6r2 parent law needs two complete frame + 19 lane statements".into()));
    }
    let f = law.child_frame();
    // E1: P_1 pins each leaf's free binding_input lane to 0.
    if f == C6R2_LEAF_FRAME_LANES {
        let l = &mut l.namespace(|| "c6r2 E1 leaf frame");
        for c in [left_full, right_full] {
            s.assert_equal_to_fixed(l, &c[0], F::ZERO)?;
        }
    }
    let (left, right) = (&left_full[f..], &right_full[f..]);
    for c in [left, right] {
        {
            // E2: each child's schema, role, level and keyset are relation constants.
            let l = &mut l.namespace(|| "c6r2 E2 child constants");
            s.assert_equal_to_fixed(l, &c[SCHEMA], c6r2_schema())?;
            s.assert_equal_to_fixed(l, &c[ROLE], F::from(law.child_role() as u64))?;
            s.assert_equal_to_fixed(l, &c[LEVEL], F::from(law.child_level() as u64))?;
            s.assert_equal_to_fixed(l, &c[KEYSET], law.child_keyset)?;
        }
        // E3: both ends and hi - lo are range-checked, so the difference is an
        // integer difference, never a wrapped field difference.
        let l = &mut l.namespace(|| "c6r2 E3 child ranges, order and bools");
        for &(lo, hi, bits, _) in &ORDERED {
            range(s, l, &c[lo], bits)?;
            range(s, l, &c[hi], bits)?;
            let d = s.sub(l, &c[hi], &c[lo])?;
            range(s, l, &d, bits)?;
        }
        for &(i, _) in &U128_LANES {
            range(s, l, &c[i], 128)?;
        }
        for &(i, _) in &BOOLS {
            assert_bool(s, l, &c[i])?;
        }
    }
    {
        // E4.
        let l = &mut l.namespace(|| "c6r2 E4 shared and continuity");
        for &(i, _) in &SHARED {
            s.assert_equal(l, &left[i], &right[i])?;
        }
        for &(out, inn, _) in &CONTINUITY {
            s.assert_equal(l, &left[out], &right[inn])?;
        }
    }
    {
        // E5: only the rightmost leaf may be final or carry a class.
        let l = &mut l.namespace(|| "c6r2 E5 placement");
        s.assert_equal_to_fixed(l, &left[HAS_FINAL], F::ZERO)?;
        s.assert_equal_to_fixed(l, &left[TERMINAL_CLASS], F::ZERO)?;
    }
    // E6: the sum of two range-checked u128 values is exact (< 2^129, far below p).
    // The post-sum range check makes u128 overflow unsatisfiable.
    let mut sums: Vec<A> = Vec::with_capacity(SUMS.len());
    {
        let l = &mut l.namespace(|| "c6r2 E6 sums");
        for &(i, _) in &SUMS {
            let x = s.add(l, &left[i], &right[i])?;
            range(s, l, &x, 128)?;
            sums.push(x);
        }
    }
    if law.is_range_root() {
        // E7: E3 made both TX ends u32 and both flags bool, so these field
        // equalities mean an integer count of 4 and exactly one boundary flag.
        let l = &mut l.namespace(|| "c6r2 E7 range root");
        let d = s.sub(l, &right[TX_HI], &left[TX_LO])?;
        s.assert_equal_to_fixed(l, &d, F::from(4u64))?;
        let e = s.add(l, &right[POST_AT_TX_ENTRY], &right[HAS_FINAL])?;
        s.assert_equal_to_fixed(l, &e, F::ONE)?;
    }
    // E8: fixed constants, aliases and sums. No semantic cell is witnessed here.
    let l = &mut l.namespace(|| "c6r2 E8 output");
    let schema: A = s.assign_fixed(l, c6r2_schema())?;
    let role: A = s.assign_fixed(l, F::from(C6R2_ROLE_PARENT as u64))?;
    let level: A = s.assign_fixed(l, F::from(law.level as u64))?;
    let keyset: A = s.assign_fixed(l, law.keyset_k)?;
    Ok(OUTPUT
        .iter()
        .map(|o| match *o {
            Out::Schema => schema.clone(),
            Out::Role => role.clone(),
            Out::Level => level.clone(),
            Out::Keyset => keyset.clone(),
            Out::L(j) => left[j].clone(),
            Out::R(j) => right[j].clone(),
            Out::Sum(j) => sums[j - CONSUMED].clone(),
        })
        .collect())
}

/// Complete witness: both complete child instances and proofs, in left/right order.
#[derive(Clone, Debug)]
pub struct C6R2ParentWitness {
    pub left_instance: Vec<F>,
    pub left_proof: Vec<u8>,
    pub right_instance: Vec<F>,
    pub right_proof: Vec<u8>,
}

/// P_k. Every field is a relation constant serialized by write_relation. keyset_k
/// is not a free choice: validate requires it to equal c6r2_keyset(level, m, child_vk).
#[derive(Clone, Debug)]
pub struct C6R2ParentRelation {
    pub level: u8,
    pub range_root_level: u8,
    pub child_vk: MidnightVK,
    pub child_kind: ChildKind,
    pub keyset_k: F,
    pub child_keyset: F,
}

fn bad(m: impl Into<String>) -> io::Error {
    io::Error::other(m.into())
}

fn kind_byte(k: ChildKind) -> u8 {
    match k {
        ChildKind::Leaf => 0,
        ChildKind::Recursive => 1,
    }
}

fn kind_from_byte(b: u8) -> io::Result<ChildKind> {
    match b {
        0 => Ok(ChildKind::Leaf),
        1 => Ok(ChildKind::Recursive),
        _ => Err(bad("unknown c6r2 child kind byte")),
    }
}

/// S1-S3 and the inner header, on the MidnightVK::write frame
/// arch || k:u8 || nb_public_inputs:u32 LE || VERSION || k || ...
/// The decoder runs this BEFORE any key is deserialized. validate reruns it on
/// vk_bytes(child_vk), so new and write_relation are held to the same checks.
pub(crate) fn check_child_vk_frame(raw: &[u8], expect_lanes: usize) -> io::Result<()> {
    if raw.len() > MAX_CHILD_VK_BYTES {
        return Err(bad("child key exceeds the 1 << 20 byte bound"));
    }
    let mut rest = raw;
    ZkStdLibArch::read(&mut rest)?;
    if rest.len() < 7 {
        return Err(bad("child key frame truncated before degree, public-input count and inner header"));
    }
    let k = rest[0];
    if !(2..=C6R2_MAX_CHILD_K).contains(&k) {
        return Err(bad("child key degree byte outside 2..=25"));
    }
    if u32::from_le_bytes([rest[1], rest[2], rest[3], rest[4]]) as usize != expect_lanes {
        return Err(bad("child key public-input count differs from the fixed child instance length"));
    }
    if rest[5] != INNER_VK_VERSION {
        return Err(bad("child key inner version byte is not 0x03"));
    }
    if rest[6] != k {
        return Err(bad("child key inner degree differs from the outer degree byte"));
    }
    Ok(())
}

/// Bounded, exact child-key decoding: frame checks first, then the key, which must
/// round-trip byte for byte.
fn decode_child_vk(raw: &[u8], shape: ChildShape) -> io::Result<MidnightVK> {
    check_child_vk_frame(raw, shape.instance_lanes().map_err(bad)?)?;
    let vk = vk_from_bytes(raw)?;
    if vk_bytes(&vk) != raw {
        return Err(bad("child key does not round-trip in this graph"));
    }
    Ok(vk)
}

impl C6R2ParentRelation {
    /// Derives the child mode and K_k. child_keyset (K_{k-1}, or 0 for P_1) must
    /// come from the child relation's own keygen; it is not invented here.
    pub fn new(level: u8, range_root_level: u8, child_vk: MidnightVK, child_keyset: F) -> io::Result<Self> {
        let law = C6R2ParentLaw { level, range_root_level, keyset_k: F::ZERO, child_keyset };
        law.check().map_err(bad)?;
        let keyset_k = c6r2_keyset(level, range_root_level, &child_vk);
        let rel = Self { level, range_root_level, child_kind: law.child_kind(), keyset_k, child_vk, child_keyset };
        rel.validate()?;
        Ok(rel)
    }

    pub fn law(&self) -> C6R2ParentLaw {
        C6R2ParentLaw {
            level: self.level,
            range_root_level: self.range_root_level,
            keyset_k: self.keyset_k,
            child_keyset: self.child_keyset,
        }
    }

    pub fn child_shape(&self) -> ChildShape {
        self.law().child_shape()
    }

    pub fn validate(&self) -> io::Result<()> {
        let law = self.law();
        law.check().map_err(bad)?;
        if self.child_kind != law.child_kind() {
            return Err(bad(MODE));
        }
        let lanes = self.child_shape().instance_lanes().map_err(bad)?;
        // S1, S2, S3 and the inner header, exactly as on decode.
        check_child_vk_frame(&vk_bytes(&self.child_vk), lanes)?;
        let cs = self.child_vk.vk().cs();
        // verify_child_incircuit passes one committed and one plain instance column.
        if cs.num_instance_columns() != 2 {
            return Err(bad("child key must define one committed and one plain instance column"));
        }
        // S4: the verifier gadget assumes a single phase.
        if cs.phases().count() != 1 {
            return Err(bad("child key must have exactly one phase"));
        }
        if self.keyset_k != c6r2_keyset(self.level, self.range_root_level, &self.child_vk) {
            return Err(bad("keyset_k is not K_k derived from (level, m, fixed child key)"));
        }
        Ok(())
    }

    /// Both sides use the ONE fixed level key and the fixed shape.
    pub fn fixed_children(&self) -> Result<[FixedChild<'_>; 2], String> {
        let shape = self.child_shape();
        Ok([FixedChild::new(&self.child_vk, shape)?, FixedChild::new(&self.child_vk, shape)?])
    }

    pub fn public_len(&self) -> usize {
        c6r2_public_len()
    }

    /// Host mirror of the 19 parent lanes. The carry tails are checked by
    /// prepare_binary_deferred.
    pub fn statement(&self, left: &[F], right: &[F]) -> Result<[F; C6R2_STMT_LANES], String> {
        self.validate().map_err(|e| e.to_string())?;
        let shape = self.child_shape();
        shape.check(left.len())?;
        shape.check(right.len())?;
        let n = self.law().child_statement_lanes();
        c6r2_parent_law(&self.law(), &left[..n], &right[..n]).map_err(|e| format!("c6r2 parent law: {e:?}"))
    }

    /// The 31-field public instance in in-circuit order. The pairing is NOT
    /// checked here; it is deferred to the terminal W and D.
    pub fn instance(&self, w: &C6R2ParentWitness) -> Result<Vec<F>, String> {
        let stmt = self.statement(&w.left_instance, &w.right_instance)?;
        let children = self.fixed_children()?;
        let acc = prepare_binary_deferred(
            &children,
            [&w.left_instance[..], &w.right_instance[..]],
            [&w.left_proof[..], &w.right_proof[..]],
        )?;
        Ok([stmt.to_vec(), acc_pis(&acc)].concat())
    }
}

impl Relation for C6R2ParentRelation {
    type Instance = Vec<F>;
    type Witness = C6R2ParentWitness;
    type Error = Error;

    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }

    /// Same architecture as the existing recursive recl9 relations.
    fn used_chips(&self) -> ZkStdLibArch {
        bls_arch(4)
    }

    fn circuit(
        &self,
        s: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _instance: Value<Vec<F>>,
        w: Value<C6R2ParentWitness>,
    ) -> Result<(), Error> {
        self.validate().map_err(|e| Error::Synthesis(e.to_string()))?;
        let law = self.law();
        let n = self.child_shape().instance_lanes().map_err(Error::Synthesis)?;
        w.as_ref()
            .map(|w| (w.left_instance.len(), w.right_instance.len()))
            .error_if_known_and(|lens| lens.0 != n || lens.1 != n)?;
        // The COMPLETE child instances, assigned once.
        let left: Vec<A> = s.assign_many(l, &w.as_ref().map(|w| w.left_instance.clone()).transpose_vec(n))?;
        let right: Vec<A> = s.assign_many(l, &w.as_ref().map(|w| w.right_instance.clone()).transpose_vec(n))?;
        // Fixed level key; both proofs are verified on THOSE cells. A carried tail
        // is tied to its reassigned collapsed accumulator inside the kernel.
        let children = self.fixed_children().map_err(Error::Synthesis)?;
        let proofs = [w.as_ref().map(|w| w.left_proof.clone()), w.as_ref().map(|w| w.right_proof.clone())];
        let acc = verify_binary_incircuit(s, l, &children, [&left[..], &right[..]], proofs)?;
        // E0-E8 on the same cells: frame included, carry tail excluded.
        let stmt = law.child_statement_lanes();
        let out = constrain_c6r2_parent_lanes(s, l, &law, &left[..stmt], &right[..stmt])?;
        for x in &out {
            s.constrain_as_public_input(l, x)?;
        }
        s.verifier().constrain_as_public_input(l, &acc)
    }

    fn write_relation<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        self.validate()?;
        w.write_all(MAGIC)?;
        w.write_all(&[self.level, self.range_root_level, kind_byte(self.child_kind)])?;
        w.write_all(&(C6R2_STMT_LANES as u32).to_le_bytes())?;
        w.write_all(&[self.law().child_frame() as u8])?;
        write_f(w, &self.keyset_k)?;
        write_f(w, &self.child_keyset)?;
        write_bytes(w, &vk_bytes(&self.child_vk))
    }

    fn read_relation<R: io::Read>(r: &mut R) -> io::Result<Self> {
        let mut magic = [0u8; 8];
        r.read_exact(&mut magic)?;
        if &magic != MAGIC {
            return Err(bad("not a c6r2 parent relation (PROPOSED C6R2P001)"));
        }
        let mut hdr = [0u8; 3];
        r.read_exact(&mut hdr)?;
        let (level, range_root_level, child_kind) = (hdr[0], hdr[1], kind_from_byte(hdr[2])?);
        if read_u32(r)? as usize != C6R2_STMT_LANES {
            return Err(bad("c6r2 semantic lane count differs from 19"));
        }
        let mut frame = [0u8; 1];
        r.read_exact(&mut frame)?;
        let keyset_k = read_f(r)?;
        let child_keyset = read_f(r)?;
        // Every parameter refusal comes before any key byte is read or decoded.
        let law = C6R2ParentLaw { level, range_root_level, keyset_k, child_keyset };
        law.check().map_err(bad)?;
        if law.child_kind() != child_kind {
            return Err(bad(MODE));
        }
        if frame[0] as usize != law.child_frame() {
            return Err(bad("frame byte differs from the child frame lane count"));
        }
        let child_vk = decode_child_vk(&read_bytes(r, MAX_CHILD_VK_BYTES)?, law.child_shape())?;
        let rel = C6R2ParentRelation { level, range_root_level, child_vk, child_kind, keyset_k, child_keyset };
        rel.validate()?;
        let mut rest = Vec::new();
        r.read_to_end(&mut rest)?;
        if !rest.is_empty() {
            return Err(bad("trailing bytes after c6r2 parent relation"));
        }
        Ok(rel)
    }
}

#[cfg(test)]
#[path = "c6r2_parent_tests.rs"]
mod tests;
