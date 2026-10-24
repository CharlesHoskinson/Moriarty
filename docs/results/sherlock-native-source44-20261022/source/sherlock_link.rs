//! recl9: the Sherlock linked parent and its terminal decider configuration.
//!
//! The parent verifies in circuit a Moriarty A102 (Eval Literal -> Return) child and the
//! next A80 (Return -> StepDone) child of one Machine run, each under its own key fixed as
//! constants. Children cross as bytes only (processed MidnightVK, proof, 32-byte LE lane
//! hex) through the C3 `SherlockChild::from_bytes` boundary against explicit pins; no
//! Moriarty Rust type is linked and `link.json` is cross-checked, never authority.
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

use std::{collections::BTreeMap, fs, io, ops::Range, path::Path, time::{Duration, Instant}};

use ff::PrimeField;
use group::{prime::PrimeCurveAffine, Curve, Group};
use midnight_circuits::{
    hash::poseidon::{PoseidonChip, PoseidonState},
    instructions::{hash::HashCPU, *},
    types::AssignedNative,
    verifier::{fixed_bases, Accumulator},
};
use midnight_curves::{Bls12, G2Affine};
use midnight_proofs::{
    circuit::{Layouter, Value},
    dev::cost_model::CircuitModel,
    plonk::Error,
    poly::{
        commitment::Params,
        kzg::params::ParamsKZG,
    },
};
use midnight_zk_stdlib::{cost_model, prove, setup_pk, setup_vk, verify, MidnightVK, Relation, ZkStdLib, ZkStdLibArch};
use num_bigint::BigUint;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use sha2::{Digest, Sha256};

use crate::{
    decider::{bls_arch, CommBinding, DeciderRelation},
    inner::{acc_pi_len, acc_pis, prepare_own_offcircuit, reconstruct_acc, verify_offcircuit, vk_name, C, S},
    sherlock_child::{verify_sherlock_child_own_offcircuit, SherlockChild, MAX_CHILD_INSTANCE, MAX_CHILD_PROOF_BYTES},
    template::Template,
    util::{f_from_hex, f_to_hex, read_bytes, read_f, read_u32, sha256_hex, vk_bytes, vk_from_bytes, write_bytes, write_f, F},
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
    "keys_id", "a102_ctx_lo", "a102_ctx_hi", "a80_ctx_lo", "a80_ctx_hi", "program_c", "pre_root", "h_pre",
    "mid_root", "post_root", "h_post", "bind_a102", "bind_a80", "n_steps", "work", "trace_c",
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
    [F::from_u128(u128::from_le_bytes(lo)), F::from_u128(u128::from_le_bytes(hi))]
}

/// Context limbs from an independently pinned 64-character lowercase context hash.
pub fn context_limbs(hash: &str) -> io::Result<[F; 2]> {
    if !is_hex64(hash) {
        return Err(bad("context hash must be 64 lowercase hex characters"));
    }
    let b: [u8; 32] = hex::decode(hash).map_err(io::Error::other)?.try_into().map_err(|_| bad("context hash length"))?;
    Ok(limbs(&b))
}

fn sha_limbs(b: &[u8]) -> [F; 2] {
    let d: [u8; 32] = Sha256::digest(b).into();
    limbs(&d)
}

fn is_hex64(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
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
        return Err(bad("link shape outside P >= 1, S >= 1, P + 2S + 5 <= 16383"));
    }
    Ok(LinkOffsets {
        binding: 0, context: 1, program: 3, pre: 3 + p, post: 3 + p + s,
        rule_tag: 3 + p + 2 * s, charge: 4 + p + 2 * s, len: len as usize,
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
pub fn check_link_lanes(o: &LinkOffsets, a102_ctx: &[F; 2], a80_ctx: &[F; 2], a: &[F], b: &[F]) -> Result<(), String> {
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
    let env = |i: &[F], (t, c): (u64, u64)| i[o.rule_tag] == F::from(t) && i[o.charge] == F::from(c);
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
        dom(DOM_TRACE), keys_id, a[c], a[d], b[c], b[d], program_c, pre, h_pre, a[x], a[t], a[g],
        mid, h_mid, b[x], b[t], b[g], post, h_post, n_steps, work,
    ]);
    vec![keys_id, a[c], a[d], b[c], b[d], program_c, pre, h_pre, mid, post, h_post, a[x], b[x], n_steps, work, trace]
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
        return Err(bad("genesis lanes are not an A102 instance under its pinned context"));
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
            a102_instance: a102.instance.clone(), a102_proof: a102.proof.clone(),
            a80_instance: a80.instance.clone(), a80_proof: a80.proof.clone(), h_pre,
        }
    }
}

impl SherlockLinkRelation {
    pub fn from_pins(a102_vk: MidnightVK, a80_vk: MidnightVK, pins: &LinkPins) -> io::Result<Self> {
        pins.validate()?;
        if sha256_hex(&vk_bytes(&a102_vk)) != pins.a102_vk_sha256 || sha256_hex(&vk_bytes(&a80_vk)) != pins.a80_vk_sha256 {
            return Err(bad("a child key differs from its pinned digest"));
        }
        let rel = Self {
            a102_vk, a80_vk, program_lanes: pins.program_lanes, state_lanes: pins.state_lanes,
            a102_context: context_limbs(&pins.a102_context_hash)?, a80_context: context_limbs(&pins.a80_context_hash)?,
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
        let wide = |x: &F| AsRef::<[u8]>::as_ref(&x.to_repr())[16..].iter().any(|b| *b != 0);
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
        let (p, s) = (F::from(self.program_lanes as u64), F::from(self.state_lanes as u64));
        Ok(poseidon(&[dom(DOM_KEYS), a0, a1, b0, b1, F::from(o.len as u64), p, s]))
    }

    /// Host refusals mirroring the in-circuit constraints, before any native work.
    pub fn check_witness(&self, w: &SherlockLinkWitness) -> Result<LinkOffsets, String> {
        self.validate().map_err(|e| e.to_string())?;
        let o = self.offsets().map_err(|e| e.to_string())?;
        check_link_lanes(&o, &self.a102_context, &self.a80_context, &w.a102_instance, &w.a80_instance)?;
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
        Ok(link_statement(keys_id, &o, &w.a102_instance, &w.a80_instance, link_heights(w.h_pre)?))
    }

    /// Both keys' fixed bases as one map with one shared, checked `-G`. Refuses an invalid
    /// relation.
    pub fn fixed_base_values(&self) -> Result<BTreeMap<String, C>, String> {
        self.validate().map_err(|e| e.to_string())?;
        let (an, bn) = (self.a102_name(), self.a80_name());
        let neg_g = -<C as Group>::generator();
        merge_fixed_bases(&an, fixed_bases::<S>(&an, self.a102_vk.vk()), &bn, fixed_bases::<S>(&bn, self.a80_vk.vk()), &neg_g)
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

    /// Parent-only reuse: inputs must be the original own accumulators from the two
    /// full child checks in this invocation, in A102/A80 order and these key namespaces.
    /// Resolve/collapse only AFTER accumulate hashes their original encodings.
    fn fold_verified_own(&self, own: [Accumulator<S>; 2]) -> Result<Accumulator<S>, String> {
        let bases = self.fixed_base_values()?;
        let mut acc = Accumulator::accumulate(&own);
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

fn tagged_incircuit(s: &ZkStdLib, l: &mut impl Layouter<F>, tag: &str, lanes: &[A]) -> Result<A, Error> {
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

    fn circuit(&self, s: &ZkStdLib, l: &mut impl Layouter<F>, _instance: Value<Vec<F>>, w: Value<SherlockLinkWitness>) -> Result<(), Error> {
        let syn = |e: io::Error| Error::Synthesis(e.to_string());
        self.validate().map_err(syn)?;
        let o = self.offsets().map_err(syn)?;
        let n = o.len;
        w.as_ref()
            .map(|w| (w.a102_instance.len(), w.a80_instance.len()))
            .error_if_known_and(|(x, y)| *x != n || *y != n)?;
        // 1. The complete A102 instance; the A80 instance aliases its program and post cells.
        let a: Vec<A> = s.assign_many(l, &w.as_ref().map(|w| w.a102_instance.clone()).transpose_vec(n))?;
        let bv = w.as_ref().map(|w| w.a80_instance.clone());
        let lanes = |r: Range<usize>| bv.as_ref().map(|v| v[r.clone()].to_vec()).transpose_vec(r.len());
        let b_head: Vec<A> = s.assign_many(l, &lanes(0..o.program))?;
        let b_post: Vec<A> = s.assign_many(l, &lanes(o.post..o.rule_tag))?;
        let b_env: Vec<A> = s.assign_many(l, &lanes(o.rule_tag..n))?;
        let b: Vec<A> = [&b_head[..], &a[o.program..o.pre], &a[o.post..o.rule_tag], &b_post[..], &b_env[..]].concat();
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
            (&self.a102_vk, self.a102_name(), &a, w.as_ref().map(|w| w.a102_proof.clone())),
            (&self.a80_vk, self.a80_name(), &b, w.as_ref().map(|w| w.a80_proof.clone())),
        ];
        for (vk, name, inst, proof) in children {
            let pvk = vk.vk();
            let avk = verifier.assign_fixed_vk(l, &name, pvk.get_domain(), pvk.cs(), pvk.transcript_repr())?;
            owns.push(verifier.prepare(l, &avk, &committed, &[inst.as_slice()], proof)?);
        }
        let mut acc = verifier.accumulate(l, &owns)?;
        acc.resolve_fixed_bases(&bases);
        acc.collapse(l, bls, bls.scalar_field_chip())?;
        // 4. Commitments, heights and the order trace over the constrained cells.
        let keys_id: A = s.assign_fixed(l, self.keys_id().map_err(|e| Error::Synthesis(e.to_string()))?)?;
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
            dom_trace, keys_id.clone(), a[c].clone(), a[d].clone(), b[c].clone(), b[d].clone(), program_c.clone(),
            pre.clone(), h_pre.clone(), a[x].clone(), a[t].clone(), a[g].clone(), mid.clone(), h_mid,
            b[x].clone(), b[t].clone(), b[g].clone(), post.clone(), h_post.clone(), n_steps.clone(), work.clone(),
        ];
        let trace_c = s.poseidon(l, &trace_in)?;
        let fields = [
            keys_id, a[c].clone(), a[d].clone(), b[c].clone(), b[d].clone(), program_c, pre, h_pre,
            mid, post, h_post, a[x].clone(), b[x].clone(), n_steps, work, trace_c,
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
        let rel = Self { a102_vk, a80_vk, program_lanes, state_lanes, a102_context, a80_context };
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
    fs::File::open(p)?.take(max as u64 + 1).read_to_end(&mut b)?;
    if b.len() > max {
        return Err(bad(format!("{} exceeds {max} bytes", p.display())));
    }
    Ok(b)
}

fn write_new(dir: &Path, name: &str, bytes: &[u8]) -> io::Result<()> {
    use std::io::Write;
    let mut file = fs::OpenOptions::new().write(true).create_new(true).open(dir.join(name))?;
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
        return Err(bad("SRS g2 is not the generator or its [tau]_2 differs from the pin"));
    }
    Ok(())
}

fn exact_k(params: &ParamsKZG<Bls12>, k: u32, what: &str) -> io::Result<()> {
    if params.max_k() != k {
        return Err(bad(format!("{what} needs SRS 2^{k}; supplied 2^{}", params.max_k())));
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

/// /2 producer text, restated exactly from Moriarty `sherlock_link`, `sherlock_child` and
/// `literal_host`. Equality identifies the manifest representation; it authenticates no K,
/// reference, program, source, key, proof or history.
const LINK_TRANSCRIPT: &str = "midnight-zk-stdlib-2.3.5/PoseidonState<Fq>";
const LINK_PROVENANCE: &str = "host-checked request and instance lane equalities over caller-supplied LinkedRequests; H1 single-Machine two-step observation is producer-asserted only when built by produce_linked_requests and is not authenticated by this manifest or verified in circuit";
const LINK_PIN_AUTHORITY: &str = "vk_sha256 values pin a key only when reproduced by the consumer's own key reconstruction or an independently reviewed pin; this manifest and its digests are not approval";
const GENESIS_ROOT_AUTHORITY: &str = "consumer pinned Poseidon over a102 instance lanes [state_offset, state_offset + state_lanes); Init host subset only";
const LINK_DISCLOSURES: [&str; 5] = [
    "LinkManifest::from_json and validate check only this document's fixed text, encodings, shape arithmetic and internal consistency; they read no sibling file, recompute no file digest and verify no claim, key, proof, instance or source",
    "file sha256 values are over the bytes written at export and identify them only; no digest here authenticates source, K, reference, program, carrier or key provenance",
    "children[0].carrier_source_sha256 is null because the A102 ChildClaim has no carrier field; children[1].carrier_source_sha256 is the A80 claim's own field; per-child open_obligations are the exact sealed ChildVerification lists",
    "genesis locates a102 instance pre lanes after a host subset of the Init equations on the request pre; it is not circuit Init and no Poseidon root is computed",
    "export is per-file create_new plus sync_all after an in-memory sealed verification; the directory is not fsynced, a failed export is not removed, link.json presence is not a durable commit marker, and child files have no reader profile or loader",
];
const LINK_EQUALITIES: [&str; 5] = [
    "children[0] = a102 and children[1] = a80 in fixed child order",
    "a102.context == LiteralLayout context and a80.context == ReturnPublicationLayout context",
    "a102.program == a80.program",
    "a102.post == a80.pre",
    "a102.envelope == (3, 1) and a80.envelope == (0, 0); work == 1",
];
const LINK_OPEN: [&str; 5] = [
    "every link equality here is host tooling; validity needs a reviewed parent that recursively verifies both children and constrains the same lanes and key pins in circuit",
    "binding lanes are SHA-256 claim commitments; neither child nor this module recomputes them in circuit",
    "neither child constrains Machine::new Init equations on the A102 pre; the genesis anchor is producer-asserted H1 data with a host subset check",
    "state roots, keys_id, trace and statement commitments, deferred pairing and subgroup checks, the terminal decider and anchoring are consumer work, unobserved",
    "k, rows, proof sizes, times and memory are unmeasured",
];
const A102_QUALIFICATION: &str = "local-a102-poseidon-child-proof-unqualified";
const A102_CLAIM_FORMAT: &str = "moriarty-sherlock-a102-poseidon-child/1";
const A102_BINDING: &str = "moriarty-sherlock-a102-poseidon-child-binding/1";
const A80_QUALIFICATION: &str = "local-a80-poseidon-child-proof-unqualified";
const A80_CLAIM_FORMAT: &str = "moriarty-sherlock-a80-poseidon-child/1";
const A80_BINDING: &str = "moriarty-sherlock-a80-poseidon-child-binding/1";
/// Moriarty `proving::MAX_K`, restated: the producer's `validate` refuses a child
/// `circuit_k` above it. A bound on unauthenticated metadata, not a degree policy.
const LINK_MAX_K: u8 = 19;
/// `literal_host::OPEN_OBLIGATIONS` then `CHILD_OPEN_OBLIGATIONS`: the a102 entry's list.
const A102_OPEN: [&str; 12] = [
    "canonical ProgramBlock equals prepareA(source) (GAP-1 unimplemented); the F4 least-equal mechanism is unselected and applies only at canonical boundaries",
    "the post is a Step-mode image (CR0=Return holding the Literal's Constant-space ValueRef, all five arenas equal to pre), never a canonical-boundary StateBlock; checkpoint, chunk or external-claim use needs the separately reviewed, unimplemented projectA boundary operation",
    "full StepInv/O1-O4 bounds, acyclicity, suffix slices, variant padding and Constants typing/literal uiValid beyond the listed host subset, which no ledger operation VK enforces",
    "OBL.G4 canonical-byte projection of nonfuel carriers",
    "the other 38 rows of the 39-row public rule_line_tags dictionary; K's invalid-Literal sibling head, which has no wire tag and whose exclusion depends on unimplemented prepareA typing/uiValid (40 post-startup K heads per a supplied public QA fact; K bodies not read here); the exactly-one dispatch partition; post-to-pre continuity",
    "trace, chunk and profile binding across proofs and K source correspondence; the three source digests are caller-asserted and unverified",
    "SRS provenance, keygen correspondence and adversarial soundness review",
    "ledger-9 installation, transaction acceptance, call address/entry-point readback and stagenet settlement; the ledger statement opening is off-chain",
    "Fenrir consumption crosses crate graphs only as processed VK, proof and lane bytes; midnight-proofs 0.8.3 versus 0.8.2 byte compatibility and the absence of truncated-challenges on both graphs are unestablished",
    "the key digest a consumer pins must come from this independent reconstruction, never from exported bytes",
    "a parent's collapsed accumulator is not validity until a reviewed terminal fold and pairing check; no terminal decider consumes this child",
    "child and in-circuit verifier resource admission over the full instance is separate and unestablished",
];
/// `RETURN_OPEN_OBLIGATIONS`; the a80 entry's list continues with LINK_OPEN.
const A80_OPEN: [&str; 4] = [
    "A80 pre and post are Step-mode Constant-space images, never canonical-boundary StateBlocks; standalone ReturnPublicationLayout::statement checks L4 shape, lane bounds and envelope (0, 0) without the private literal_host representation subset (covered in linked requests via A102 program/post equality); projectA, F3 and F5 are unimplemented",
    "the circuit enforces only the line-80 fragment under a forced enable; full StepInv/O1-O4, F4, OBL.G4 and the other 38 dispatch rows remain open",
    "the A80 context binds the caller-asserted LiteralSourceIdentity of the linked A102 request; the three digests are not authenticated",
    "crate-graph byte compatibility, consumer key pins and in-circuit verifier resources are unestablished, as for the A102 child",
];

/// (name, qualification, claim_format, binding, envelope, carrier present, open_obligations)
/// in fixed child order. The envelopes are this crate's pins, as in LINK_EQUALITIES.
type LinkRole = (&'static str, &'static str, &'static str, &'static str, (u64, u64), bool, Vec<&'static str>);

fn link_roles() -> [LinkRole; 2] {
    [
        ("a102", A102_QUALIFICATION, A102_CLAIM_FORMAT, A102_BINDING, A102_ENVELOPE, false, A102_OPEN.to_vec()),
        ("a80", A80_QUALIFICATION, A80_CLAIM_FORMAT, A80_BINDING, A80_ENVELOPE, true, A80_OPEN.iter().chain(&LINK_OPEN).copied().collect()),
    ]
}

type Json = serde_json::Value;

/// `mori_core::json::parse` restated for this crate graph: duplicate decoded keys, depth
/// above 32 and more than 65,536 nodes are refused. The 1 MiB cap is `read_capped`.
struct Strict<'a> {
    depth: usize,
    nodes: &'a std::cell::Cell<usize>,
}

impl<'de> serde::de::DeserializeSeed<'de> for Strict<'_> {
    type Value = Json;
    fn deserialize<D: serde::Deserializer<'de>>(self, d: D) -> Result<Json, D::Error> {
        use serde::de::Error as _;
        self.nodes.set(self.nodes.get() + 1);
        if self.depth > 32 || self.nodes.get() > 65_536 {
            return Err(D::Error::custom("JSON work/depth limit"));
        }
        d.deserialize_any(self)
    }
}

impl<'de> serde::de::Visitor<'de> for Strict<'_> {
    type Value = Json;
    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("bounded JSON")
    }
    fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Json, E> {
        Ok(v.into())
    }
    fn visit_unit<E: serde::de::Error>(self) -> Result<Json, E> {
        Ok(Json::Null)
    }
    fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Json, E> {
        Ok(v.into())
    }
    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Json, E> {
        Ok(v.into())
    }
    fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Json, E> {
        serde_json::Number::from_f64(v).map(Json::Number).ok_or_else(|| E::custom("nonfinite JSON number"))
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Json, E> {
        Ok(v.into())
    }
    fn visit_seq<Q: serde::de::SeqAccess<'de>>(self, mut a: Q) -> Result<Json, Q::Error> {
        let mut out = Vec::new();
        while let Some(v) = a.next_element_seed(Strict { depth: self.depth + 1, nodes: self.nodes })? {
            out.push(v);
        }
        Ok(Json::Array(out))
    }
    fn visit_map<M: serde::de::MapAccess<'de>>(self, mut a: M) -> Result<Json, M::Error> {
        use serde::de::Error as _;
        let mut out = serde_json::Map::new();
        while let Some(k) = a.next_key::<String>()? {
            if out.contains_key(&k) {
                return Err(M::Error::custom("duplicate decoded key"));
            }
            let v = a.next_value_seed(Strict { depth: self.depth + 1, nodes: self.nodes })?;
            out.insert(k, v);
        }
        Ok(Json::Object(out))
    }
}

/// UTF-8, the strict seed, then trailing data refused.
fn parse_strict(b: &[u8]) -> io::Result<Json> {
    use serde::de::DeserializeSeed;
    let nodes = std::cell::Cell::new(0);
    let mut de = serde_json::Deserializer::from_str(std::str::from_utf8(b).map_err(io::Error::other)?);
    let v = Strict { depth: 0, nodes: &nodes }.deserialize(&mut de).map_err(io::Error::other)?;
    de.end().map_err(io::Error::other)?;
    Ok(v)
}

#[derive(serde::Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct WireOffsets {
    binding: usize,
    context: usize,
    program: usize,
    pre: usize,
    post: usize,
    rule_tag: usize,
    charge: usize,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct WireGenesis {
    height: u64,
    child: String,
    state_offset: usize,
    state_lanes: usize,
    root_authority: String,
}

/// The /2 `LinkManifest` child wire, private to this reader: the producer's `LinkChild`
/// fields, types and `deny_unknown_fields`, as for the other wire structs. Every key is
/// required. As the producer's `deserialize_with = "Option::deserialize"` with no default,
/// `carrier_source_sha256` must be present: `null` is `None`, which `validate_wire` accepts
/// only for a102; a80 needs a string.
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct WireChild {
    name: String,
    claim_file: String,
    claim_sha256: String,
    vk_file: String,
    vk_sha256: String,
    proof_file: String,
    proof_sha256: String,
    instance_file: String,
    instance_sha256: String,
    qualification: String,
    claim_format: String,
    binding: String,
    instance_lanes: usize,
    proof_bytes: u32,
    context_hash: String,
    zkir_hash: String,
    adapter_source_sha256: String,
    #[serde(deserialize_with = "<Option<String> as serde::Deserialize>::deserialize")]
    carrier_source_sha256: Option<String>,
    circuit_k: u8,
    rule_tag: u32,
    charge: u32,
    open_obligations: Vec<String>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct WireManifest {
    format: String,
    transcript: String,
    field_encoding: String,
    provenance: String,
    program_lanes: usize,
    state_lanes: usize,
    instance_lanes: usize,
    offsets: WireOffsets,
    links: Vec<String>,
    children: Vec<WireChild>,
    genesis: WireGenesis,
    pin_authority: String,
    open_obligations: Vec<String>,
    disclosures: Vec<String>,
}

fn same_text<'a>(actual: &[String], expected: impl IntoIterator<Item = &'a str>) -> bool {
    actual.iter().map(String::as_str).eq(expected)
}

/// The producer's checked `LinkShape::offsets` over the manifest's own lane counts.
fn checked_offsets(p: usize, s: usize, n: usize) -> Option<WireOffsets> {
    let at = |base: usize, k: usize| -> Option<usize> { s.checked_mul(k)?.checked_add(p)?.checked_add(base) };
    let o = WireOffsets { binding: 0, context: 1, program: 3, pre: at(3, 0)?, post: at(3, 1)?, rule_tag: at(3, 2)?, charge: at(4, 2)? };
    (o.charge.checked_add(1) == Some(n)).then_some(o)
}

/// The producer's `LinkManifest::validate`, restated: fixed text, checked offsets, the
/// genesis locator, both children in fixed role order with fixed filenames, digest
/// encodings, envelopes, bounds and per-child obligations. Reads no file. Digests must be
/// 64 lowercase hex characters, this crate's form. `circuit_k` must be in 1..=`LINK_MAX_K`,
/// the producer's bound.
fn validate_wire(m: &WireManifest) -> io::Result<()> {
    if m.format != LINK_FORMAT || m.transcript != LINK_TRANSCRIPT || m.field_encoding != LINK_FIELD_ENCODING
        || m.provenance != LINK_PROVENANCE || m.pin_authority != LINK_PIN_AUTHORITY
        || !same_text(&m.links, LINK_EQUALITIES) || !same_text(&m.open_obligations, LINK_OPEN)
        || !same_text(&m.disclosures, LINK_DISCLOSURES)
    {
        return Err(bad("link.json format, encoding, authority or disclosure text differs"));
    }
    if checked_offsets(m.program_lanes, m.state_lanes, m.instance_lanes).as_ref() != Some(&m.offsets) {
        return Err(bad("link.json offsets differ from its checked lane counts"));
    }
    let g = &m.genesis;
    if g.height != 0 || g.child != "a102" || g.state_offset != m.offsets.pre
        || g.state_lanes != m.state_lanes || g.root_authority != GENESIS_ROOT_AUTHORITY
    {
        return Err(bad("link.json genesis locator differs from the a102 pre lanes"));
    }
    if m.children.len() != 2 {
        return Err(bad("link.json needs exactly the a102 and a80 children"));
    }
    for (c, (name, qualification, claim_format, binding, envelope, carrier, open)) in m.children.iter().zip(link_roles()) {
        let files = [&c.claim_file, &c.vk_file, &c.proof_file, &c.instance_file];
        let digests = [
            &c.claim_sha256, &c.vk_sha256, &c.proof_sha256, &c.instance_sha256,
            &c.context_hash, &c.zkir_hash, &c.adapter_source_sha256,
        ];
        if c.name != name
            || files.into_iter().zip(["claim.json", "vk", "proof", "instance.json"]).any(|(f, k)| *f != format!("{name}.{k}"))
            || c.qualification != qualification || c.claim_format != claim_format || c.binding != binding
            || c.carrier_source_sha256.is_some() != carrier
            || !digests.into_iter().chain(&c.carrier_source_sha256).all(|d| is_hex64(d))
            || c.instance_lanes != m.instance_lanes
            || c.proof_bytes == 0 || c.proof_bytes as usize > MAX_CHILD_PROOF_BYTES
            || c.circuit_k == 0 || c.circuit_k > LINK_MAX_K
            || (u64::from(c.rule_tag), u64::from(c.charge)) != envelope
            || !same_text(&c.open_obligations, open)
        {
            return Err(bad(format!("link.json child {name} disagrees with its fixed role, encoding or bounds")));
        }
    }
    Ok(())
}

/// Read `<dir>/{a102,a80}.{vk,proof,instance.json}` by fixed names through the C3 boundary
/// against the explicit pins. Then refuse any pair whose lanes break the pinned contexts,
/// the envelopes, the shared ProgramBlock or A102.post == A80.pre. Authority comes from the
/// pins (key digests, contexts, shape) and, later, from proof verification of these bytes.
/// `link.json` is parsed strictly as the producer's /2 `LinkManifest`, validated intrinsically
/// (`validate_wire`) and must agree with the pins. Each proof's length and SHA-256 and each
/// instance file's SHA-256 must equal its entry: exact-byte identification only.
/// `<name>.claim.json` is a required name and digest and is never read. No producer
/// assertion authenticates a proof, a lane, a key, a source or history.
pub fn read_link_children(dir: &Path, pins: &LinkPins) -> io::Result<(SherlockChild, SherlockChild)> {
    pins.validate()?;
    let o = link_offsets(pins.program_lanes, pins.state_lanes)?;
    let m: WireManifest =
        serde_json::from_value(parse_strict(&read_capped(&dir.join("link.json"), 1 << 20)?)?).map_err(io::Error::other)?;
    validate_wire(&m)?;
    let offs = WireOffsets {
        binding: o.binding, context: o.context, program: o.program, pre: o.pre,
        post: o.post, rule_tag: o.rule_tag, charge: o.charge,
    };
    if m.program_lanes != pins.program_lanes as usize
        || m.state_lanes != pins.state_lanes as usize
        || m.instance_lanes != o.len
        || m.offsets != offs
    {
        return Err(bad("link.json disagrees with the pinned shape"));
    }
    let mut out = Vec::with_capacity(2);
    for (d, name, vk_pin, ctx_pin) in [
        (&m.children[0], "a102", &pins.a102_vk_sha256, &pins.a102_context_hash),
        (&m.children[1], "a80", &pins.a80_vk_sha256, &pins.a80_context_hash),
    ] {
        if d.vk_sha256 != *vk_pin || d.context_hash != *ctx_pin {
            return Err(bad(format!("link.json child {name} disagrees with the pins")));
        }
        let vk = read_capped(&dir.join(format!("{name}.vk")), 1 << 20)?;
        let proof = read_capped(&dir.join(format!("{name}.proof")), MAX_CHILD_PROOF_BYTES)?;
        let raw = read_capped(&dir.join(format!("{name}.instance.json")), 80 * o.len + 64)?;
        // Exact-byte identification of the two files read here; it authenticates nothing.
        if proof.len() != d.proof_bytes as usize
            || sha256_hex(&proof) != d.proof_sha256
            || sha256_hex(&raw) != d.instance_sha256
        {
            return Err(bad(format!("{name} proof or instance bytes differ from link.json")));
        }
        let hexes: Vec<String> = serde_json::from_slice(&raw).map_err(io::Error::other)?;
        let instance = hexes.iter().map(|h| f_from_canonical_hex(h)).collect::<io::Result<Vec<F>>>()?;
        out.push(SherlockChild::from_bytes(&vk, vk_pin, o.len, instance, proof)?);
    }
    let a80 = out.pop().expect("two children");
    let a102 = out.pop().expect("two children");
    let (ca, cb) = (context_limbs(&pins.a102_context_hash)?, context_limbs(&pins.a80_context_hash)?);
    check_link_lanes(&o, &ca, &cb, &a102.instance, &a80.instance).map_err(io::Error::other)?;
    Ok((a102, a80))
}

/// Genesis root candidate for the anchor constructor: the pinned state root over the A102
/// pre lanes. The child must carry the pinned A102 key, context and (3, 1) envelope. An A80
/// child, whose pre is the mid root, is refused. Host only: no Init equation is checked, so
/// the value is input to independent review and never anchor authority by itself.
pub fn genesis_root(rel: &SherlockLinkRelation, a102: &SherlockChild) -> io::Result<F> {
    rel.validate()?;
    if vk_bytes(&a102.vk) != vk_bytes(&rel.a102_vk) {
        return Err(bad("genesis child is not the pinned A102 key"));
    }
    genesis_root_lanes(&rel.offsets()?, &rel.a102_context, &a102.instance)
}

// BEGIN private parent fitting-degree helpers (also extracted for SDK-free tests).
// Full-parent modeling assumes there are no cost_measure subregion markers.
// This chooses an automatic valid fixed point, not the SDK's global minimum.
// Follow valid unseen degrees in either direction without assuming monotonicity.
// Repeated/invalid observations fall back unchanged; SDK panics propagate.
fn automatic_fitting_model<M>(
    mut probe_model: impl FnMut(u32) -> M,
    degree: impl Fn(&M) -> u32,
    fallback: impl FnOnce() -> M,
) -> M {
    let mut probe = 9;
    let mut visited = 0u32;
    for _ in 9..=25 {
        let bit = 1u32 << (probe - 9);
        if visited & bit != 0 {
            return fallback();
        }
        visited |= bit;
        let model = probe_model(probe);
        let required = degree(&model);
        if required == probe {
            return model;
        }
        if !(9..=25).contains(&required) {
            return fallback();
        }
        probe = required;
    }
    fallback()
}

// SRS equality precedes modeling. A fixed point is accepted directly; a lower
// modeled degree retains the original SDK automatic-degree equality policy.
// An undersized key is refused without a second model. No model is mutated.
fn parent_degree_matches(
    key_degree: u32,
    srs_degree: u32,
    model_at_degree: impl FnOnce(u32) -> u32,
    legacy_model_degree: impl FnOnce() -> u32,
) -> bool {
    if srs_degree != key_degree {
        return false;
    }
    let required = model_at_degree(key_degree);
    if required > key_degree {
        return false;
    }
    required == key_degree || legacy_model_degree() == key_degree
}
// END private parent fitting-degree helpers.

/// One internally computed model borrowed against the exact parent relation.
/// Automatic distinct-degree probes select a valid fitting degree, not the SDK minimum.
/// Fallback retains the SDK strategy verbatim and can require additional work.
/// Its private fields cannot be forged from caller-provided k/model data.
/// The immutable borrow remains live until this plan is consumed by keygen.
///
/// ```compile_fail
/// use straw_recl9::sherlock_link::{ParentKeygenPlan, SherlockLinkRelation};
/// use midnight_proofs::dev::cost_model::CircuitModel;
/// fn forge(rel: &SherlockLinkRelation, model: CircuitModel) {
///     let _ = ParentKeygenPlan { rel, model, model_elapsed: std::time::Duration::ZERO };
/// }
/// ```
/// ```compile_fail
/// use straw_recl9::sherlock_link::ParentKeygenPlan;
/// fn force_degree(plan: &ParentKeygenPlan<'_>) { plan.model().k = 19; }
/// ```
pub struct ParentKeygenPlan<'a> {
    rel: &'a SherlockLinkRelation,
    model: CircuitModel,
    model_elapsed: Duration,
}

/// Stage names and monotonic durations only; never keys, proofs or instances.
fn parent_keygen_phase(stage: &str, elapsed: Duration) {
    eprintln!("{}", serde_json::json!({"kind": "sherlock-parent-keygen-phase",
        "stage": stage, "elapsed_seconds": elapsed.as_secs_f64()}));
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
        let model = automatic_fitting_model(
            |probe| cost_model(rel, Some(probe)),
            |model| model.k,
            || cost_model(rel, None),
        );
        let model_elapsed = started.elapsed();
        parent_keygen_phase("cost-model-complete", model_elapsed);
        Self { rel, model, model_elapsed }
    }

    pub fn model(&self) -> &CircuitModel { &self.model }
    pub fn model_elapsed(&self) -> Duration { self.model_elapsed }

    pub fn keygen(self, params: &ParamsKZG<Bls12>, s_g2: &G2Affine, key_dir: &Path) -> io::Result<MidnightVK> {
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
pub fn keygen_link_parent(params: &ParamsKZG<Bls12>, s_g2: &G2Affine, rel: &SherlockLinkRelation, key_dir: &Path) -> io::Result<MidnightVK> {
    rel.validate()?;
    check_srs(params, s_g2)?;
    ParentKeygenPlan::model_validated(rel).keygen(params, s_g2, key_dir)
}

/// The parent key from a key_dir, against its pinned digest and the expected relation bytes.
pub fn load_link_parent_key(key_dir: &Path, vk_sha256: &str, expect: &SherlockLinkRelation) -> io::Result<MidnightVK> {
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

#[derive(Clone, Copy)]
enum ParentProofPhase {
    Model, ChildA102Verify, ChildA80Verify, AccumulatorFold, FoldedPairing,
    SetupPk, Prove, StdlibVerify, PairingVerify, Write,
}

fn parent_proof_phase_record(phase: ParentProofPhase, complete: bool, elapsed: Duration) -> serde_json::Value {
    let stage = match phase {
        ParentProofPhase::Model => "model",
        ParentProofPhase::ChildA102Verify => "child_a102_verify",
        ParentProofPhase::ChildA80Verify => "child_a80_verify",
        ParentProofPhase::AccumulatorFold => "accumulator_fold",
        ParentProofPhase::FoldedPairing => "folded_pairing",
        ParentProofPhase::SetupPk => "setup_pk",
        ParentProofPhase::Prove => "prove",
        ParentProofPhase::StdlibVerify => "stdlib_verify",
        ParentProofPhase::PairingVerify => "pairing_verify",
        ParentProofPhase::Write => "write",
    };
    serde_json::json!({"kind": "sherlock-parent-proof-phase", "stage": stage,
        "boundary": if complete { "complete" } else { "start" }, "elapsed_seconds": elapsed.as_secs_f64()})
}

fn parent_proof_phase(phase: ParentProofPhase, complete: bool) {
    use std::io::Write;
    static BEGAN: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
    let elapsed = BEGAN.get_or_init(Instant::now).elapsed();
    // Diagnostics carry fixed labels and time only. A closed stderr is not a proof refusal.
    let _ = writeln!(io::stderr().lock(), "{}", parent_proof_phase_record(phase, complete, elapsed));
}

/// Native: full off-circuit verification of both children, exact-degree SRS and key,
/// the folded accumulator's pairing, prove under the Poseidon transcript, then the stdlib
/// verify and the decider's view (prepare, fold the carried accumulator, one pairing).
/// Cheap refusals (relation, witness lanes, heights, proof sizes, SRS g2 and [tau]_2, key
/// bytes) run before the degree probe and any verification. The verifier parameters are
/// derived from `params` itself. The key degree must be an explicit-model fixed
/// point, or a fitting lower-model degree accepted by the original SDK automatic
/// comparison. The CLI pins the key digest and relation before this call.
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
    parent_proof_phase(ParentProofPhase::Model, false);
    if !parent_degree_matches(
        k,
        params.max_k(),
        |degree| cost_model(rel, Some(degree)).k,
        || cost_model(rel, None).k,
    ) {
        return Err(format!("parent needs matching SRS and a fixed-point or legacy cost-model key degree; key 2^{k}, SRS 2^{}", params.max_k()));
    }
    parent_proof_phase(ParentProofPhase::Model, true);
    let mut own = Vec::with_capacity(2);
    for (cvk, inst, proof, name, phase) in [
        (&rel.a102_vk, &w.a102_instance, &w.a102_proof, rel.a102_name(), ParentProofPhase::ChildA102Verify),
        (&rel.a80_vk, &w.a80_instance, &w.a80_proof, rel.a80_name(), ParentProofPhase::ChildA80Verify),
    ] {
        parent_proof_phase(phase, false);
        let child = SherlockChild { name, vk: cvk.clone(), instance: inst.clone(), proof: proof.clone() };
        own.push(verify_sherlock_child_own_offcircuit(vp, &child)?);
        parent_proof_phase(phase, true);
    }
    parent_proof_phase(ParentProofPhase::AccumulatorFold, false);
    let statement = rel.statement(w)?;
    let own_b = own.pop().expect("two verified children");
    let own_a = own.pop().expect("two verified children");
    let instance = [statement, acc_pis(&rel.fold_verified_own([own_a, own_b])?)].concat();
    parent_proof_phase(ParentProofPhase::AccumulatorFold, true);
    parent_proof_phase(ParentProofPhase::FoldedPairing, false);
    let acc = reconstruct_acc(&instance[LINK_STATE_FIELDS..]).ok_or("parent accumulator malformed")?;
    if !acc.check(vp, &BTreeMap::new()) {
        return Err("folded child accumulator fails its pairing".into());
    }
    parent_proof_phase(ParentProofPhase::FoldedPairing, true);
    parent_proof_phase(ParentProofPhase::SetupPk, false);
    let pk = setup_pk(rel, vk);
    parent_proof_phase(ParentProofPhase::SetupPk, true);
    parent_proof_phase(ParentProofPhase::Prove, false);
    let proof = prove::<SherlockLinkRelation, Pos>(params, &pk, rel, &instance, w.clone(), &mut ChaCha20Rng::from_entropy())
        .map_err(|e| format!("prove parent: {e:?}"))?;
    drop(pk);
    parent_proof_phase(ParentProofPhase::Prove, true);
    parent_proof_phase(ParentProofPhase::StdlibVerify, false);
    verify::<SherlockLinkRelation, Pos>(vp, vk, &instance, None, &proof).map_err(|e| format!("parent stdlib verify: {e:?}"))?;
    parent_proof_phase(ParentProofPhase::StdlibVerify, true);
    parent_proof_phase(ParentProofPhase::PairingVerify, false);
    verify_offcircuit(vp, vk, &vk_name("w", vk), &instance, &proof, true)?;
    parent_proof_phase(ParentProofPhase::PairingVerify, true);
    Ok(LinkParentProof { vk: vk.clone(), instance, proof })
}

/// The w_dir consumed unchanged by `straw-recl9 prove <w_dir> <key_dir> <out> <K>`:
/// w.vk (processed), w.proof, w.instance.json (32-byte LE hex array) and SOURCE.
pub fn write_w_dir(dir: &Path, parent: &LinkParentProof, source: &str) -> io::Result<()> {
    parent_proof_phase(ParentProofPhase::Write, false);
    if parent.instance.len() != LINK_STATE_FIELDS + acc_pi_len() {
        return Err(bad("parent instance is not the 28-field layout"));
    }
    fs::create_dir(dir)?;
    write_new(dir, "w.vk", &vk_bytes(&parent.vk))?;
    write_new(dir, "w.proof", &parent.proof)?;
    let fields: Vec<String> = parent.instance.iter().map(f_to_hex).collect();
    write_new(dir, "w.instance.json", &serde_json::to_vec_pretty(&fields).map_err(io::Error::other)?)?;
    write_new(dir, "SOURCE", source.as_bytes())?;
    parent_proof_phase(ParentProofPhase::Write, true);
    Ok(())
}

/// The stock decider over the parent: W instance 16 statement fields plus 12 accumulator
/// fields, SHK_STMT, every statement field in order. DeciderRelation's 1024 stmt_fields
/// read cap is unchanged; the terminal public statement is the captured template's slot
/// vector, refused above 1023 slots.
pub fn sherlock_decider_relation(parent_vk: MidnightVK, s_g2: G2Affine, template: Template) -> io::Result<DeciderRelation> {
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
pub fn keygen_sherlock_decider(params: &ParamsKZG<Bls12>, rel: &DeciderRelation, key_dir: &Path) -> io::Result<MidnightVK> {
    let shaped = rel.n_w_instance as usize == LINK_STATE_FIELDS + acc_pi_len()
        && rel.stmt_fields.iter().copied().eq(0..LINK_STATE_FIELDS as u32)
        && rel.dom_stmt == dom(DOM_STMT)
        && rel.template.slots.len() <= MAX_TERMINAL_PUBLIC
        && rel.comm_binding == CommBinding::BoundPoseidonStmtV1;
    if !shaped {
        return Err(bad("decider relation is not the Sherlock parent configuration"));
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

#[cfg(test)]
#[path = "sherlock_link_tests.rs"]
mod tests;
