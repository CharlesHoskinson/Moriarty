# Local mori-recursion parent.rs
> Source: file:///home/charl/code/Moriarty-perf/crates/mori-recursion/src/parent.rs
> Collected: 2026-10-26
> Published: Unknown

// Adapted from projectfenrir recursion/stagenet-strawman/recl9.
// Uses the pinned stock 0.8.3 graph; terminal validity and gadget soundness remain separate.
//! The Sherlock linked two-child parent relation.
//!
//! The parent verifies in circuit a Moriarty A102 (Eval Literal -> Return) child and the
//! next A80 (Return -> StepDone) child of one Machine run, each under its own key fixed as
//! constants. Children enter as processed MidnightVK bytes, proof bytes and complete public
//! lanes through `SherlockChild::from_bytes`, against independently reconstructed key pins.
//!
//! In circuit the complete A102 instance is assigned; the A80 instance is its own binding,
//! context, post and envelope cells with the A102 program and post cells in its program and
//! pre positions, so the shared ProgramBlock and A102.post == A80.pre hold by construction.
//! Both context pairs and the envelopes (3, 1) then (0, 0) are pinned. Both proofs are
//! prepared and accumulated, both keys' fixed bases resolved and the result collapsed; the
//! pairing is deferred to the stock `DeciderRelation`, which subgroup-checks and pairs it.
//! Program, pre, mid and post lanes are committed with pinned-domain Poseidon. Children carry
//! no height lane: h_pre is a range-checked parent input, h_mid = h_pre + 1 and h_post =
//! h_pre + 2 are constrained, and trace_c commits the ordered steps with their heights. Only
//! the anchor binds (pre_root, h_pre) to its current anchored state. 16 statement fields then
//! the 12-field accumulator are public. Binding lanes are exposed, not recomputed. Init
//! equations, projectA, StepInv, F3-F5, G4 and the 39-row dispatch stay open; Done is
//! terminal, never a canonical boundary.

use std::{
    collections::BTreeMap,
    fs, io,
    ops::Range,
    path::Path,
    time::{Duration, Instant},
};

use ff::PrimeField;
use group::{Curve, Group, prime::PrimeCurveAffine};
use midnight_circuits::{
    hash::poseidon::{PoseidonChip, PoseidonState},
    instructions::{hash::HashCPU, *},
    types::AssignedNative,
    verifier::{Accumulator, fixed_bases},
};
use midnight_curves::{Bls12, G2Affine};
use midnight_proofs::{
    circuit::{Layouter, Value},
    dev::cost_model::CircuitModel,
    plonk::Error,
    poly::{commitment::Params, kzg::params::ParamsKZG},
};
use midnight_zk_stdlib::{
    MidnightVK, Relation, ZkStdLib, ZkStdLibArch, cost_model, prove, setup_pk, setup_vk, verify,
};
use num_bigint::BigUint;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use sha2::{Digest, Sha256};

use crate::{
    bls_arch,
    child::{
        MAX_CHILD_INSTANCE, MAX_CHILD_PROOF_BYTES, SherlockChild, verify_sherlock_child_offcircuit,
    },
    decider::{CommBinding, DeciderRelation},
    inner::{
        C, S, acc_pi_len, acc_pis, prepare_own_offcircuit, reconstruct_acc, verify_offcircuit,
        vk_name,
    },
    template::Template,
    util::{
        F, f_from_hex, f_to_hex, read_bytes, read_f, read_u32, sha256_hex, vk_bytes, vk_from_bytes,
        write_bytes, write_f,
    },
};

const MAGIC: &[u8; 8] = b"RECL9H01";

type A = AssignedNative<F>;
type Pos = PoseidonState<F>;

/// Statement fields before the 12-field collapsed accumulator.
pub const LINK_STATE_FIELDS: usize = 16;
/// The terminal public statement (template slots) stays below 1024 fields.
pub const MAX_TERMINAL_PUBLIC: usize = 1023;
pub const LINK_FORMAT: &str = "moriarty-sherlock-link/2";
pub const LINK_FIELD_ENCODING: &str = "bls12-381-scalar/32-byte-little-endian-lowercase-hex";
/// Wire (rule_tag, charge): A102 line 102, then A80 line 80.
pub const A102_ENVELOPE: (u64, u64) = (3, 1);
pub const A80_ENVELOPE: (u64, u64) = (0, 0);
pub const DOM_KEYS: &str = "SHK_KEYS";
pub const DOM_PROG: &str = "SHK_PROG";
pub const DOM_STATE: &str = "SHK_STATE";
pub const DOM_TRACE: &str = "SHK_TRACE";
pub const DOM_STMT: &str = "SHK_STMT";
pub const FIELD_NAMES: [&str; LINK_STATE_FIELDS] = [
    "keys_id",
    "a102_ctx_lo",
    "a102_ctx_hi",
    "a80_ctx_lo",
    "a80_ctx_hi",
    "program_c",
    "pre_root",
    "h_pre",
    "mid_root",
    "post_root",
    "h_post",
    "bind_a102",
    "bind_a80",
    "n_steps",
    "work",
    "trace_c",
];

fn bad(m: impl Into<String>) -> io::Error {
    io::Error::other(m.into())
}

/// ASCII domain tag packed little-endian into a u128 field constant.
pub fn dom(tag: &str) -> F {
    assert!(tag.len() <= 16, "domain tag longer than 16 bytes");
    let mut b = [0u8; 16];
    b[..tag.len()].copy_from_slice(tag.as_bytes());
    F::from_u128(u128::from_le_bytes(b))
}

/// The pinned Poseidon, the decider's off-circuit `stmt` hash.
pub fn poseidon(xs: &[F]) -> F {
    <PoseidonChip<F> as HashCPU<F, F>>::hash(xs)
}

fn tagged(tag: &str, lanes: &[F]) -> F {
    let d = dom(tag);
    poseidon(&[std::slice::from_ref(&d), lanes].concat())
}

/// Root of a complete state image (root lanes, then every table row, as in the child).
pub fn state_root(lanes: &[F]) -> F {
    tagged(DOM_STATE, lanes)
}

/// Two little-endian u128 limbs of 32 bytes, as Moriarty's `limbs()`.
fn limbs(b: &[u8; 32]) -> [F; 2] {
    let lo: [u8; 16] = b[..16].try_into().expect("16 bytes");
    let hi: [u8; 16] = b[16..].try_into().expect("16 bytes");
    [
        F::from_u128(u128::from_le_bytes(lo)),
        F::from_u128(u128::from_le_bytes(hi)),
    ]
}

/// Context limbs from an independently pinned 64-character lowercase context hash.
pub fn context_limbs(hash: &str) -> io::Result<[F; 2]> {
    if !is_hex64(hash) {
        return Err(bad("context hash must be 64 lowercase hex characters"));
    }
    let b: [u8; 32] = hex::decode(hash)
        .map_err(io::Error::other)?
        .try_into()
        .map_err(|_| bad("context hash length"))?;
    Ok(limbs(&b))
}

fn sha_limbs(b: &[u8]) -> [F; 2] {
    let d: [u8; 32] = Sha256::digest(b).into();
    limbs(&d)
}

fn is_hex64(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
}

/// One lane in LINK_FIELD_ENCODING: exactly 64 lowercase hex characters, no prefix, of a
/// canonical little-endian scalar. `util::f_from_hex` also accepts `0x` and uppercase.
pub fn f_from_canonical_hex(h: &str) -> io::Result<F> {
    if !is_hex64(h) {
        return Err(bad("field hex must be 64 lowercase hex characters"));
    }
    let x = f_from_hex(h)?;
    if f_to_hex(&x) != h {
        return Err(bad("field hex is not the canonical encoding"));
    }
    Ok(x)
}

/// Lane offsets inside each complete child instance (binding first); len = P + 2S + 5.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkOffsets {
    pub binding: usize,
    pub context: usize,
    pub program: usize,
    pub pre: usize,
    pub post: usize,
    pub rule_tag: usize,
    pub charge: usize,
    pub len: usize,
}

pub fn link_offsets(program_lanes: u32, state_lanes: u32) -> io::Result<LinkOffsets> {
    let (p, s) = (program_lanes as usize, state_lanes as usize);
    let len = program_lanes as u64 + 2 * state_lanes as u64 + 5;
    if p == 0 || s == 0 || len > MAX_CHILD_INSTANCE as u64 {
        return Err(bad(
            "link shape outside P >= 1, S >= 1, P + 2S + 5 <= 16383",
        ));
    }
    Ok(LinkOffsets {
        binding: 0,
        context: 1,
        program: 3,
        pre: 3 + p,
        post: 3 + p + s,
        rule_tag: 3 + p + 2 * s,
        charge: 4 + p + 2 * s,
        len: len as usize,
    })
}

/// The one fixed-base name `fixed_bases` emits unprefixed for every key.
pub const SHARED_BASE: &str = "-G";

/// One fixed-base map for two keys: `-G` must be present in both with exactly `neg_g`
/// (the negated generator) and is kept once; every other name must belong to exactly one
/// key, so no commitment is skipped or replaced. Generic over the base for host tests.
pub fn merge_fixed_bases<V: PartialEq>(
    an: &str,
    a: BTreeMap<String, V>,
    bn: &str,
    b: BTreeMap<String, V>,
    neg_g: &V,
) -> Result<BTreeMap<String, V>, String> {
    if an == bn {
        return Err("both keys share one fixed-base prefix".into());
    }
    if a.get(SHARED_BASE) != Some(neg_g) || b.get(SHARED_BASE) != Some(neg_g) {
        return Err("a key's -G base is missing or not the negated generator".into());
    }
    let mut out = a;
    for (k, v) in b {
        if k != SHARED_BASE && out.insert(k, v).is_some() {
            return Err("a fixed-base name other than -G is shared by both keys".into());
        }
    }
    Ok(out)
}

/// Host refusals mirroring the in-circuit lane constraints: the shape, both pinned contexts,
/// one ProgramBlock, A102.post == A80.pre and the envelopes (3, 1) then (0, 0).
pub fn check_link_lanes(
    o: &LinkOffsets,
    a102_ctx: &[F; 2],
    a80_ctx: &[F; 2],
    a: &[F],
    b: &[F],
) -> Result<(), String> {
    if a.len() != o.len || b.len() != o.len {
        return Err("child instance length differs from the pinned shape".into());
    }
    if a[o.context..o.program] != a102_ctx[..] || b[o.context..o.program] != a80_ctx[..] {
        return Err("a child context differs from its pin".into());
    }
    if a[o.program..o.pre] != b[o.program..o.pre] {
        return Err("A102 and A80 ProgramBlock lanes differ".into());
    }
    if a[o.post..o.rule_tag] != b[o.pre..o.post] {
        return Err("A102 post lanes differ from A80 pre lanes".into());
    }
    let env =
        |i: &[F], (t, c): (u64, u64)| i[o.rule_tag] == F::from(t) && i[o.charge] == F::from(c);
    if !env(a, A102_ENVELOPE) || !env(b, A80_ENVELOPE) {
        return Err("envelopes differ from A102 (3, 1) then A80 (0, 0)".into());
    }
    Ok(())
}

/// [h_pre, h_mid, h_post]: one height per Machine step, all below 2^64. Heights are not
/// child lanes; h_pre is bound only by the anchor.
pub fn link_heights(h_pre: u64) -> Result<[u64; 3], String> {
    let h_post = h_pre.checked_add(2).ok_or("h_pre + 2 exceeds 64 bits")?;
    Ok([h_pre, h_pre + 1, h_post])
}

/// The 16 statement fields in FIELD_NAMES order from checked lanes and heights. trace_c is
/// TRACE_ORDER: SHK_TRACE, keys_id, both contexts, program_c, (pre_root, h_pre), A102
/// (binding, rule_tag, charge), (mid_root, h_mid), A80 (binding, rule_tag, charge),
/// (post_root, h_post), n_steps, work; every input is a statement field or a constant.
pub fn link_statement(keys_id: F, o: &LinkOffsets, a: &[F], b: &[F], h: [u64; 3]) -> Vec<F> {
    let program_c = tagged(DOM_PROG, &a[o.program..o.pre]);
    let pre = state_root(&a[o.pre..o.post]);
    let mid = state_root(&a[o.post..o.rule_tag]);
    let post = state_root(&b[o.post..o.rule_tag]);
    let (h_pre, h_mid, h_post) = (F::from(h[0]), F::from(h[1]), F::from(h[2]));
    let (n_steps, work) = (F::from(2u64), a[o.charge] + b[o.charge]);
    let (c, d, x, t, g) = (o.context, o.context + 1, o.binding, o.rule_tag, o.charge);
    let trace = poseidon(&[
        dom(DOM_TRACE),
        keys_id,
        a[c],
        a[d],
        b[c],
        b[d],
        program_c,
        pre,
        h_pre,
        a[x],
        a[t],
        a[g],
        mid,
        h_mid,
        b[x],
        b[t],
        b[g],
        post,
        h_post,
        n_steps,
        work,
    ]);
    vec![
        keys_id, a[c], a[d], b[c], b[d], program_c, pre, h_pre, mid, post, h_post, a[x], b[x],
        n_steps, work, trace,
    ]
}

/// The genesis candidate root over an A102 instance's pre lanes. The instance must have the
/// pinned shape, the pinned A102 context and the (3, 1) envelope.
pub fn genesis_root_lanes(o: &LinkOffsets, a102_ctx: &[F; 2], i: &[F]) -> io::Result<F> {
    let (t, c) = A102_ENVELOPE;
    let a102 = i.len() == o.len
        && i[o.context..o.program] == a102_ctx[..]
        && i[o.rule_tag] == F::from(t)
        && i[o.charge] == F::from(c);
    if !a102 {
        return Err(bad(
            "genesis lanes are not an A102 instance under its pinned context",
        ));
    }
    Ok(state_root(&i[o.pre..o.post]))
}

/// Explicit authority: independently reviewed key digests, context hashes and shape.
/// Nothing here is taken from a producer manifest.
#[derive(Clone, Debug)]
pub struct LinkPins {
    pub a102_vk_sha256: String,
    pub a80_vk_sha256: String,
    pub a102_context_hash: String,
    pub a80_context_hash: String,
    pub program_lanes: u32,
    pub state_lanes: u32,
}

impl LinkPins {
    /// Shape, key-digest and context pins are well formed and pairwise distinct.
    pub fn validate(&self) -> io::Result<()> {
        link_offsets(self.program_lanes, self.state_lanes)?;
        if !is_hex64(&self.a102_vk_sha256) || !is_hex64(&self.a80_vk_sha256) {
            return Err(bad("key digest pins must be 64 lowercase hex characters"));
        }
        if self.a102_vk_sha256 == self.a80_vk_sha256 {
            return Err(bad("both children pin one key digest"));
        }
        if context_limbs(&self.a102_context_hash)? == context_limbs(&self.a80_context_hash)? {
            return Err(bad("A102 and A80 context pins coincide"));
        }
        Ok(())
    }
}

/// Two-child parent (the decider's W): both child keys fixed, shape and contexts pinned.
#[derive(Clone, Debug)]
pub struct SherlockLinkRelation {
    pub a102_vk: MidnightVK,
    pub a80_vk: MidnightVK,
    pub program_lanes: u32,
    pub state_lanes: u32,
    pub a102_context: [F; 2],
    pub a80_context: [F; 2],
}

#[derive(Clone, Debug)]
pub struct SherlockLinkWitness {
    pub a102_instance: Vec<F>,
    pub a102_proof: Vec<u8>,
    pub a80_instance: Vec<F>,
    pub a80_proof: Vec<u8>,
    /// The predecessor anchored height.
    pub h_pre: u64,
}

impl SherlockLinkWitness {
    pub fn from_children(a102: &SherlockChild, a80: &SherlockChild, h_pre: u64) -> Self {
        Self {
            a102_instance: a102.instance.clone(),
            a102_proof: a102.proof.clone(),
            a80_instance: a80.instance.clone(),
            a80_proof: a80.proof.clone(),
            h_pre,
        }
    }
}

impl SherlockLinkRelation {
    pub fn from_pins(a102_vk: MidnightVK, a80_vk: MidnightVK, pins: &LinkPins) -> io::Result<Self> {
        pins.validate()?;
        if sha256_hex(&vk_bytes(&a102_vk)) != pins.a102_vk_sha256
            || sha256_hex(&vk_bytes(&a80_vk)) != pins.a80_vk_sha256
        {
            return Err(bad("a child key differs from its pinned digest"));
        }
        let rel = Self {
            a102_vk,
            a80_vk,
            program_lanes: pins.program_lanes,
            state_lanes: pins.state_lanes,
            a102_context: context_limbs(&pins.a102_context_hash)?,
            a80_context: context_limbs(&pins.a80_context_hash)?,
        };
        rel.validate()?;
        Ok(rel)
    }

    pub fn a102_name(&self) -> String {
        vk_name("sherlock_a102", &self.a102_vk)
    }

    pub fn a80_name(&self) -> String {
        vk_name("sherlock_a80", &self.a80_vk)
    }

    pub fn offsets(&self) -> io::Result<LinkOffsets> {
        link_offsets(self.program_lanes, self.state_lanes)
    }

    pub fn public_len(&self) -> usize {
        LINK_STATE_FIELDS + acc_pi_len()
    }

    pub fn validate(&self) -> io::Result<()> {
        self.offsets()?;
        if vk_bytes(&self.a102_vk) == vk_bytes(&self.a80_vk) {
            return Err(bad("both children pin one key"));
        }
        let wide = |x: &F| {
            AsRef::<[u8]>::as_ref(&x.to_repr())[16..]
                .iter()
                .any(|b| *b != 0)
        };
        if self.a102_context.iter().chain(&self.a80_context).any(wide) {
            return Err(bad("a context limb exceeds 128 bits"));
        }
        if self.a102_context == self.a80_context {
            return Err(bad("A102 and A80 contexts coincide"));
        }
        Ok(())
    }

    /// keys_id = Poseidon(SHK_KEYS, sha256(A102 vk) limbs, sha256(A80 vk) limbs, n, P, S).
    pub fn keys_id(&self) -> io::Result<F> {
        self.validate()?;
        let o = self.offsets()?;
        let [a0, a1] = sha_limbs(&vk_bytes(&self.a102_vk));
        let [b0, b1] = sha_limbs(&vk_bytes(&self.a80_vk));
        let (p, s) = (
            F::from(self.program_lanes as u64),
            F::from(self.state_lanes as u64),
        );
        Ok(poseidon(&[
            dom(DOM_KEYS),
            a0,
            a1,
            b0,
            b1,
            F::from(o.len as u64),
            p,
            s,
        ]))
    }

    /// Host refusals mirroring the in-circuit constraints, before any native work.
    pub fn check_witness(&self, w: &SherlockLinkWitness) -> Result<LinkOffsets, String> {
        self.validate().map_err(|e| e.to_string())?;
        let o = self.offsets().map_err(|e| e.to_string())?;
        check_link_lanes(
            &o,
            &self.a102_context,
            &self.a80_context,
            &w.a102_instance,
            &w.a80_instance,
        )?;
        link_heights(w.h_pre)?;
        for p in [&w.a102_proof, &w.a80_proof] {
            if p.is_empty() || p.len() > MAX_CHILD_PROOF_BYTES {
                return Err("child proof is empty or oversized".into());
            }
        }
        Ok(o)
    }

    /// The 16 statement fields in FIELD_NAMES order, from every child lane.
    pub fn statement(&self, w: &SherlockLinkWitness) -> Result<Vec<F>, String> {
        let o = self.check_witness(w)?;
        let keys_id = self.keys_id().map_err(|e| e.to_string())?;
        Ok(link_statement(
            keys_id,
            &o,
            &w.a102_instance,
            &w.a80_instance,
            link_heights(w.h_pre)?,
        ))
    }

    /// Both keys' fixed bases as one map with one shared, checked `-G`. Refuses an invalid
    /// relation.
    pub fn fixed_base_values(&self) -> Result<BTreeMap<String, C>, String> {
        self.validate().map_err(|e| e.to_string())?;
        let (an, bn) = (self.a102_name(), self.a80_name());
        let neg_g = -<C as Group>::generator();
        merge_fixed_bases(
            &an,
            fixed_bases::<S>(&an, self.a102_vk.vk()),
            &bn,
            fixed_bases::<S>(&bn, self.a80_vk.vk()),
            &neg_g,
        )
    }

    /// Off circuit in the in-circuit order, after the witness checks: both own accumulators
    /// (A102 then A80), one accumulation, the merged fixed bases resolved, one collapse. No
    /// pairing is checked here.
    pub fn link_acc_offcircuit(&self, w: &SherlockLinkWitness) -> Result<Accumulator<S>, String> {
        self.check_witness(w)?;
        let bases = self.fixed_base_values()?;
        let (an, bn) = (self.a102_name(), self.a80_name());
        let own_a = prepare_own_offcircuit(&self.a102_vk, &an, &w.a102_instance, &w.a102_proof)?;
        let own_b = prepare_own_offcircuit(&self.a80_vk, &bn, &w.a80_instance, &w.a80_proof)?;
        let mut acc = Accumulator::accumulate(&[own_a, own_b]);
        acc.resolve_fixed_bases(&bases);
        acc.collapse();
        Ok(acc)
    }

    /// The 28-field public instance: statement, then the collapsed accumulator.
    pub fn instance(&self, w: &SherlockLinkWitness) -> Result<Vec<F>, String> {
        let st = self.statement(w)?;
        Ok([st, acc_pis(&self.link_acc_offcircuit(w)?)].concat())
    }
}

fn tagged_incircuit(
    s: &ZkStdLib,
    l: &mut impl Layouter<F>,
    tag: &str,
    lanes: &[A],
) -> Result<A, Error> {
    let d: A = s.assign_fixed(l, dom(tag))?;
    s.poseidon(l, &[std::slice::from_ref(&d), lanes].concat())
}

impl Relation for SherlockLinkRelation {
    type Instance = Vec<F>;
    type Witness = SherlockLinkWitness;
    type Error = Error;

    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }

    fn used_chips(&self) -> ZkStdLibArch {
        bls_arch(4)
    }

    fn circuit(
        &self,
        s: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _instance: Value<Vec<F>>,
        w: Value<SherlockLinkWitness>,
    ) -> Result<(), Error> {
        let syn = |e: io::Error| Error::Synthesis(e.to_string());
        self.validate().map_err(syn)?;
        let o = self.offsets().map_err(syn)?;
        let n = o.len;
        w.as_ref()
            .map(|w| (w.a102_instance.len(), w.a80_instance.len()))
            .error_if_known_and(|(x, y)| *x != n || *y != n)?;
        // 1. The complete A102 instance; the A80 instance aliases its program and post cells.
        let a: Vec<A> = s.assign_many(
            l,
            &w.as_ref().map(|w| w.a102_instance.clone()).transpose_vec(n),
        )?;
        let bv = w.as_ref().map(|w| w.a80_instance.clone());
        let lanes = |r: Range<usize>| {
            bv.as_ref()
                .map(|v| v[r.clone()].to_vec())
                .transpose_vec(r.len())
        };
        let b_head: Vec<A> = s.assign_many(l, &lanes(0..o.program))?;
        let b_post: Vec<A> = s.assign_many(l, &lanes(o.post..o.rule_tag))?;
        let b_env: Vec<A> = s.assign_many(l, &lanes(o.rule_tag..n))?;
        let b: Vec<A> = [
            &b_head[..],
            &a[o.program..o.pre],
            &a[o.post..o.rule_tag],
            &b_post[..],
            &b_env[..],
        ]
        .concat();
        if b.len() != n {
            return Err(Error::Synthesis("A80 instance shape".into()));
        }
        // 2. Pinned contexts and envelopes.
        let ctx = a[o.context..o.program].iter().zip(self.a102_context);
        for (x, c) in ctx.chain(b[o.context..o.program].iter().zip(self.a80_context)) {
            s.assert_equal_to_fixed(l, x, c)?;
        }
        for (i, (t, c)) in [(&a, A102_ENVELOPE), (&b, A80_ENVELOPE)] {
            s.assert_equal_to_fixed(l, &i[o.rule_tag], F::from(t))?;
            s.assert_equal_to_fixed(l, &i[o.charge], F::from(c))?;
        }
        // 3. Both proofs under their fixed keys: one accumulation (A102 then A80), the merged
        //    fixed bases (one shared, checked -G) assigned and resolved once, one collapse.
        //    The pairing is deferred.
        let (verifier, bls) = (s.verifier(), s.bls12_381());
        let mut bases = BTreeMap::new();
        for (key, base) in self.fixed_base_values().map_err(Error::Synthesis)? {
            bases.insert(key, bls.assign_fixed(l, base)?);
        }
        let committed = [bls.assign_fixed(l, C::identity())?];
        let mut owns = Vec::with_capacity(2);
        let children = [
            (
                &self.a102_vk,
                self.a102_name(),
                &a,
                w.as_ref().map(|w| w.a102_proof.clone()),
            ),
            (
                &self.a80_vk,
                self.a80_name(),
                &b,
                w.as_ref().map(|w| w.a80_proof.clone()),
            ),
        ];
        for (vk, name, inst, proof) in children {
            let pvk = vk.vk();
            let avk = verifier.assign_fixed_vk(
                l,
                &name,
                pvk.get_domain(),
                pvk.cs(),
                pvk.transcript_repr(),
            )?;
            owns.push(verifier.prepare(l, &avk, &committed, &[inst.as_slice()], proof)?);
        }
        let mut acc = verifier.accumulate(l, &owns)?;
        acc.resolve_fixed_bases(&bases);
        acc.collapse(l, bls, bls.scalar_field_chip())?;
        // 4. Commitments, heights and the order trace over the constrained cells.
        let keys_id: A = s.assign_fixed(
            l,
            self.keys_id()
                .map_err(|e| Error::Synthesis(e.to_string()))?,
        )?;
        let program_c = tagged_incircuit(s, l, DOM_PROG, &a[o.program..o.pre])?;
        let pre = tagged_incircuit(s, l, DOM_STATE, &a[o.pre..o.post])?;
        let mid = tagged_incircuit(s, l, DOM_STATE, &a[o.post..o.rule_tag])?;
        let post = tagged_incircuit(s, l, DOM_STATE, &b[o.post..o.rule_tag])?;
        let bound = BigUint::from(1u8) << 64usize;
        let h_pre: A = s.assign(l, w.as_ref().map(|w| F::from(w.h_pre)))?;
        s.assert_lower_than_fixed(l, &h_pre, &bound)?;
        let h_mid = s.add_constant(l, &h_pre, F::from(1u64))?;
        let h_post = s.add_constant(l, &h_pre, F::from(2u64))?;
        s.assert_lower_than_fixed(l, &h_post, &bound)?;
        let n_steps: A = s.assign_fixed(l, F::from(2u64))?;
        let work = s.add(l, &a[o.charge], &b[o.charge])?;
        let (c, d, x) = (o.context, o.context + 1, o.binding);
        let dom_trace: A = s.assign_fixed(l, dom(DOM_TRACE))?;
        // TRACE_ORDER, exactly as `link_statement`.
        let (t, g) = (o.rule_tag, o.charge);
        let trace_in = [
            dom_trace,
            keys_id.clone(),
            a[c].clone(),
            a[d].clone(),
            b[c].clone(),
            b[d].clone(),
            program_c.clone(),
            pre.clone(),
            h_pre.clone(),
            a[x].clone(),
            a[t].clone(),
            a[g].clone(),
            mid.clone(),
            h_mid,
            b[x].clone(),
            b[t].clone(),
            b[g].clone(),
            post.clone(),
            h_post.clone(),
            n_steps.clone(),
            work.clone(),
        ];
        let trace_c = s.poseidon(l, &trace_in)?;
        let fields = [
            keys_id,
            a[c].clone(),
            a[d].clone(),
            b[c].clone(),
            b[d].clone(),
            program_c,
            pre,
            h_pre,
            mid,
            post,
            h_post,
            a[x].clone(),
            b[x].clone(),
            n_steps,
            work,
            trace_c,
        ];
        for f in &fields {
            s.constrain_as_public_input(l, f)?;
        }
        s.verifier().constrain_as_public_input(l, &acc)
    }

    fn write_relation<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(MAGIC)?;
        w.write_all(&self.program_lanes.to_le_bytes())?;
        w.write_all(&self.state_lanes.to_le_bytes())?;
        for x in self.a102_context.iter().chain(&self.a80_context) {
            write_f(w, x)?;
        }
        write_bytes(w, &vk_bytes(&self.a102_vk))?;
        write_bytes(w, &vk_bytes(&self.a80_vk))
    }

    fn read_relation<R: io::Read>(r: &mut R) -> io::Result<Self> {
        let mut magic = [0u8; 8];
        r.read_exact(&mut magic)?;
        if &magic != MAGIC {
            return Err(bad("not a recl9 sherlock link relation"));
        }
        let program_lanes = read_u32(r)?;
        let state_lanes = read_u32(r)?;
        let a102_context = [read_f(r)?, read_f(r)?];
        let a80_context = [read_f(r)?, read_f(r)?];
        let a102_vk = vk_from_bytes(&read_bytes(r, 1 << 20)?)?;
        let a80_vk = vk_from_bytes(&read_bytes(r, 1 << 20)?)?;
        let rel = Self {
            a102_vk,
            a80_vk,
            program_lanes,
            state_lanes,
            a102_context,
            a80_context,
        };
        rel.validate()?;
        let mut rest = Vec::new();
        r.read_to_end(&mut rest)?;
        if !rest.is_empty() {
            return Err(bad("trailing bytes after sherlock link relation"));
        }
        Ok(rel)
    }
}

/// Reads at most max + 1 bytes, whatever the file becomes after it is opened.
fn read_capped(p: &Path, max: usize) -> io::Result<Vec<u8>> {
    use std::io::Read;
    let mut b = Vec::new();
    fs::File::open(p)?
        .take(max as u64 + 1)
        .read_to_end(&mut b)?;
    if b.len() > max {
        return Err(bad(format!("{} exceeds {max} bytes", p.display())));
    }
    Ok(b)
}

fn write_new(dir: &Path, name: &str, bytes: &[u8]) -> io::Result<()> {
    use std::io::Write;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(dir.join(name))?;
    file.write_all(bytes)?;
    file.sync_all()
}

/// The SRS must carry the standard g2 and exactly the [tau]_2 that the decider relation pins,
/// so children, parent and decider fold under one pairing. Its G1 powers are authenticated
/// only by the caller's pinned SRS file digest; no G1 getter is used here.
fn check_srs(params: &ParamsKZG<Bls12>, s_g2: &G2Affine) -> io::Result<()> {
    if bool::from(s_g2.is_identity())
        || params.g2().to_affine() != G2Affine::generator()
        || params.s_g2().to_affine() != *s_g2
    {
        return Err(bad(
            "SRS g2 is not the generator or its [tau]_2 differs from the pin",
        ));
    }
    Ok(())
}

fn exact_k(params: &ParamsKZG<Bls12>, k: u32, what: &str) -> io::Result<()> {
    if params.max_k() != k {
        return Err(bad(format!(
            "{what} needs SRS 2^{k}; supplied 2^{}",
            params.max_k()
        )));
    }
    Ok(())
}

fn vk_roundtrip(vk: &MidnightVK) -> io::Result<Vec<u8>> {
    let b = vk_bytes(vk);
    if vk_bytes(&vk_from_bytes(&b)?) != b {
        return Err(bad("key bytes do not round-trip"));
    }
    Ok(b)
}

fn relation_bytes<R: Relation>(rel: &R) -> io::Result<Vec<u8>> {
    let mut b = Vec::new();
    rel.write_relation(&mut b)?;
    let mut again = Vec::new();
    R::read_relation(&mut &b[..])?.write_relation(&mut again)?;
    if again != b {
        return Err(bad("relation bytes do not round-trip"));
    }
    Ok(b)
}

/// One internally computed model borrowed against the exact parent relation.
/// Its private fields cannot be forged from caller-provided k/model data.
/// The immutable borrow remains live until this plan is consumed by keygen.
///
/// ```compile_fail
/// use mori_recursion::parent::{ParentKeygenPlan, SherlockLinkRelation};
/// use midnight_proofs::dev::cost_model::CircuitModel;
/// fn forge(rel: &SherlockLinkRelation, model: CircuitModel) {
///     let _ = ParentKeygenPlan { rel, model, model_elapsed: std::time::Duration::ZERO };
/// }
/// ```
/// ```compile_fail
/// use mori_recursion::parent::ParentKeygenPlan;
/// fn force_degree(plan: &ParentKeygenPlan<'_>) { plan.model().k = 19; }
/// ```
pub struct ParentKeygenPlan<'a> {
    rel: &'a SherlockLinkRelation,
    model: CircuitModel,
    model_elapsed: Duration,
}

/// Stage names and monotonic durations only; never keys, proofs or instances.
fn parent_keygen_phase(stage: &str, elapsed: Duration) {
    eprintln!(
        "{}",
        serde_json::json!({"kind": "sherlock-parent-keygen-phase",
        "stage": stage, "elapsed_seconds": elapsed.as_secs_f64()})
    );
}

impl<'a> ParentKeygenPlan<'a> {
    pub fn new(rel: &'a SherlockLinkRelation) -> io::Result<Self> {
        rel.validate()?;
        Ok(Self::model_validated(rel))
    }

    // Only this module can construct a plan; this always computes the model.
    fn model_validated(rel: &'a SherlockLinkRelation) -> Self {
        parent_keygen_phase("cost-model-start", Duration::ZERO);
        let started = Instant::now();
        let model = cost_model(rel, None);
        let model_elapsed = started.elapsed();
        parent_keygen_phase("cost-model-complete", model_elapsed);
        Self {
            rel,
            model,
            model_elapsed,
        }
    }

    pub fn model(&self) -> &CircuitModel {
        &self.model
    }
    pub fn model_elapsed(&self) -> Duration {
        self.model_elapsed
    }

    pub fn keygen(
        self,
        params: &ParamsKZG<Bls12>,
        s_g2: &G2Affine,
        key_dir: &Path,
    ) -> io::Result<MidnightVK> {
        let started = Instant::now();
        parent_keygen_phase("srs-degree-check-start", Duration::ZERO);
        check_srs(params, s_g2)?;
        let k = self.model.k;
        exact_k(params, k, "parent")?;
        parent_keygen_phase("srs-degree-check-complete", started.elapsed());
        parent_keygen_phase("setup-vk-start", Duration::ZERO);
        let started = Instant::now();
        let vk = setup_vk(params, self.rel);
        parent_keygen_phase("setup-vk-complete", started.elapsed());
        if vk.k() as u32 != k {
            return Err(bad("parent key degree differs from the cost model"));
        }
        parent_keygen_phase("roundtrips-start", Duration::ZERO);
        let started = Instant::now();
        let vkb = vk_roundtrip(&vk)?;
        let relb = relation_bytes(self.rel)?;
        parent_keygen_phase("roundtrips-complete", started.elapsed());
        parent_keygen_phase("write-new-start", Duration::ZERO);
        let started = Instant::now();
        fs::create_dir(key_dir)?;
        write_new(key_dir, "link.vk", &vkb)?;
        write_new(key_dir, "link.relation", &relb)?;
        parent_keygen_phase("write-new-complete", started.elapsed());
        Ok(vk)
    }
}

/// Native: standalone parent keygen still checks relation/SRS before modeling.
/// The computed model is internal, and both exact-degree and output round trips
/// are identical to the CLI's borrowed-plan path.
pub fn keygen_link_parent(
    params: &ParamsKZG<Bls12>,
    s_g2: &G2Affine,
    rel: &SherlockLinkRelation,
    key_dir: &Path,
) -> io::Result<MidnightVK> {
    rel.validate()?;
    check_srs(params, s_g2)?;
    ParentKeygenPlan::model_validated(rel).keygen(params, s_g2, key_dir)
}

/// The parent key from a key_dir, against its pinned digest and the expected relation bytes.
pub fn load_link_parent_key(
    key_dir: &Path,
    vk_sha256: &str,
    expect: &SherlockLinkRelation,
) -> io::Result<MidnightVK> {
    let relb = read_capped(&key_dir.join("link.relation"), 3 << 20)?;
    // Byte identity with the expected relation's checked round trip, before any parse of the
    // unpinned bytes; equal bytes were already parsed and validated by `relation_bytes`.
    if relb != relation_bytes(expect)? {
        return Err(bad("parent key dir pins another relation"));
    }
    let vkb = read_capped(&key_dir.join("link.vk"), 1 << 20)?;
    if sha256_hex(&vkb) != vk_sha256 {
        return Err(bad("parent key differs from its pinned digest"));
    }
    let vk = vk_from_bytes(&vkb)?;
    if vk_bytes(&vk) != vkb {
        return Err(bad("parent key does not round-trip"));
    }
    Ok(vk)
}

#[derive(Clone, Debug)]
pub struct LinkParentProof {
    pub vk: MidnightVK,
    pub instance: Vec<F>,
    pub proof: Vec<u8>,
}

/// Native: full off-circuit verification of both children, exact-degree SRS and key,
/// the folded accumulator's pairing, prove under the Poseidon transcript, then the stdlib
/// verify and the decider's view (prepare, fold the carried accumulator, one pairing).
/// Cheap refusals (relation, witness lanes, heights, proof sizes, SRS g2 and [tau]_2, key
/// bytes) run before the degree probe and any verification. The verifier parameters are
/// derived from `params` itself.
pub fn prove_link_parent(
    params: &ParamsKZG<Bls12>,
    s_g2: &G2Affine,
    rel: &SherlockLinkRelation,
    vk: &MidnightVK,
    w: &SherlockLinkWitness,
) -> Result<LinkParentProof, String> {
    rel.check_witness(w)?;
    check_srs(params, s_g2).map_err(|e| e.to_string())?;
    vk_roundtrip(vk).map_err(|e| e.to_string())?;
    let vp = &params.verifier_params();
    let k = vk.k() as u32;
    if params.max_k() != k || cost_model(rel, None).k != k {
        return Err(format!(
            "parent needs key and SRS at its cost-model degree; key 2^{k}, SRS 2^{}",
            params.max_k()
        ));
    }
    for (cvk, inst, proof) in [
        (&rel.a102_vk, &w.a102_instance, &w.a102_proof),
        (&rel.a80_vk, &w.a80_instance, &w.a80_proof),
    ] {
        let child = SherlockChild {
            name: vk_name("sherlock", cvk),
            vk: cvk.clone(),
            instance: inst.clone(),
            proof: proof.clone(),
        };
        verify_sherlock_child_offcircuit(vp, &child)?;
    }
    let instance = rel.instance(w)?;
    let acc =
        reconstruct_acc(&instance[LINK_STATE_FIELDS..]).ok_or("parent accumulator malformed")?;
    if !acc.check(vp, &BTreeMap::new()) {
        return Err("folded child accumulator fails its pairing".into());
    }
    let pk = setup_pk(rel, vk);
    let proof = prove::<SherlockLinkRelation, Pos>(
        params,
        &pk,
        rel,
        &instance,
        w.clone(),
        &mut ChaCha20Rng::from_entropy(),
    )
    .map_err(|e| format!("prove parent: {e:?}"))?;
    drop(pk);
    verify::<SherlockLinkRelation, Pos>(vp, vk, &instance, None, &proof)
        .map_err(|e| format!("parent stdlib verify: {e:?}"))?;
    verify_offcircuit(vp, vk, &vk_name("w", vk), &instance, &proof, true)?;
    Ok(LinkParentProof {
        vk: vk.clone(),
        instance,
        proof,
    })
}

/// The w_dir consumed unchanged by `straw-recl9 prove <w_dir> <key_dir> <out> <K>`:
/// w.vk (processed), w.proof, w.instance.json (32-byte LE hex array) and SOURCE.
pub fn write_w_dir(dir: &Path, parent: &LinkParentProof, source: &str) -> io::Result<()> {
    if parent.instance.len() != LINK_STATE_FIELDS + acc_pi_len() {
        return Err(bad("parent instance is not the 28-field layout"));
    }
    fs::create_dir(dir)?;
    write_new(dir, "w.vk", &vk_bytes(&parent.vk))?;
    write_new(dir, "w.proof", &parent.proof)?;
    let fields: Vec<String> = parent.instance.iter().map(f_to_hex).collect();
    write_new(
        dir,
        "w.instance.json",
        &serde_json::to_vec_pretty(&fields).map_err(io::Error::other)?,
    )?;
    write_new(dir, "SOURCE", source.as_bytes())
}

/// The stock decider over the parent: W instance 16 statement fields plus 12 accumulator
/// fields, SHK_STMT, every statement field in order. DeciderRelation's 1024 stmt_fields
/// read cap is unchanged; the terminal public statement is the captured template's slot
/// vector, refused above 1023 slots.
pub fn sherlock_decider_relation(
    parent_vk: MidnightVK,
    s_g2: G2Affine,
    template: Template,
) -> io::Result<DeciderRelation> {
    if template.slots.len() > MAX_TERMINAL_PUBLIC {
        return Err(bad("terminal public statement must stay below 1024 fields"));
    }
    let rel = DeciderRelation {
        nr_cols: 4,
        w_vk: parent_vk,
        n_w_instance: (LINK_STATE_FIELDS + acc_pi_len()) as u32,
        s_g2,
        dom_stmt: dom(DOM_STMT),
        stmt_fields: (0..LINK_STATE_FIELDS as u32).collect(),
        template,
        comm_binding: CommBinding::BoundPoseidonStmtV1,
    };
    rel.validate()?;
    Ok(rel)
}

/// Native: decider keygen at exactly its cost-model degree, standard g2 and matching
/// [tau]_2, key and relation round trips, then decider.vk and decider.relation in a new
/// key_dir for the unchanged per-call prove route.
pub fn keygen_sherlock_decider(
    params: &ParamsKZG<Bls12>,
    rel: &DeciderRelation,
    key_dir: &Path,
) -> io::Result<MidnightVK> {
    let shaped = rel.n_w_instance as usize == LINK_STATE_FIELDS + acc_pi_len()
        && rel
            .stmt_fields
            .iter()
            .copied()
            .eq(0..LINK_STATE_FIELDS as u32)
        && rel.dom_stmt == dom(DOM_STMT)
        && rel.template.slots.len() <= MAX_TERMINAL_PUBLIC
        && rel.comm_binding == CommBinding::BoundPoseidonStmtV1;
    if !shaped {
        return Err(bad(
            "decider relation is not the Sherlock parent configuration",
        ));
    }
    rel.validate()?;
    check_srs(params, &rel.s_g2)?;
    let k = cost_model(rel, None).k;
    exact_k(params, k, "decider")?;
    let vk = setup_vk(params, rel);
    if vk.k() as u32 != k {
        return Err(bad("decider key degree differs from the cost model"));
    }
    let vkb = vk_roundtrip(&vk)?;
    let relb = relation_bytes(rel)?;
    fs::create_dir(key_dir)?;
    write_new(key_dir, "decider.vk", &vkb)?;
    write_new(key_dir, "decider.relation", &relb)?;
    Ok(vk)
}
