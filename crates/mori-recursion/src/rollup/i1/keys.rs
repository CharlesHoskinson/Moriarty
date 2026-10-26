// Adapted from the pinned Fenrir B2 oracle; see ../sources.json.
//! Jubjub keys and Schnorr signatures.
//!
//! Party secret: `master = H(DOM_KEY, seed, party_id)` in the BLS
//! field, `sk = reduce_to_jfr(master)` (re-hashed with a counter
//! in the negligible zero case). `pk = [sk]G` with G the Jubjub
//! prime-order subgroup generator. `nk = H(DOM_NK, sk_as_field)`.
//! Per transaction the spender publishes `rk = pk + [alpha]G` and
//! signs with `sk + alpha`; the challenge is
//! `e = reduce(H(DOM_SIG, R.x, R.y, rk.x, rk.y, msg))` and the
//! equation is `s*G == R + e*rk` (STRAWMAN section 2.4).

use super::fp::{F, JFr, dom, f_u64, hash, jfr_to_f, reduce_to_jfr};
use super::types::{Point, Signature};
use ff::Field;
use group::{Group, cofactor::CofactorGroup};
use midnight_curves::{JubjubAffine, JubjubExtended, JubjubSubgroup};

#[derive(Clone, Debug)]
pub struct PartyKeys {
    pub sk: JFr,
    pub pk: Point,
    pub nk: F,
}

pub fn generator_point() -> Point {
    subgroup_to_point(JubjubSubgroup::generator())
}

pub fn derive_party_keys(seed: u64, party_id: u32) -> PartyKeys {
    let mut counter = 0u64;
    let sk = loop {
        let master = hash(&[
            dom::key(),
            f_u64(seed),
            f_u64(party_id as u64),
            f_u64(counter),
        ]);
        let sk = reduce_to_jfr(master);
        if sk != JFr::ZERO {
            break sk;
        }
        counter += 1;
    };
    PartyKeys {
        pk: public_key_of(sk),
        nk: nk_of(sk),
        sk,
    }
}

/// pk = [sk]G
pub fn public_key_of(sk: JFr) -> Point {
    subgroup_to_point(JubjubSubgroup::generator() * sk)
}

/// nk = H(DOM_NK, sk_as_field)
pub fn nk_of(sk: JFr) -> F {
    hash(&[dom::nk(), jfr_to_f(sk)])
}

pub fn subgroup_to_point(p: JubjubSubgroup) -> Point {
    let affine = JubjubAffine::from(JubjubExtended::from(p));
    Point {
        x: affine.get_u(),
        y: affine.get_v(),
    }
}

/// Check that (x, y) is a prime-order Jubjub point and return it.
/// The compressed round trip checks the curve equation; the
/// cofactor trait checks torsion freeness.
pub fn point_to_subgroup(p: Point) -> Option<JubjubSubgroup> {
    let affine = JubjubAffine::from_raw_unchecked(p.x, p.y);
    let bytes = affine.to_bytes();
    let parsed = JubjubAffine::from_bytes(bytes).into_option()?;
    if parsed.get_u() != p.x || parsed.get_v() != p.y {
        return None;
    }
    let ext: JubjubExtended = parsed.to_extended();
    ext.into_subgroup().into_option()
}

pub fn is_valid_point(p: Point) -> bool {
    if p == Point::identity() {
        return false;
    }
    point_to_subgroup(p).is_some()
}

pub fn challenge(r: Point, rk: Point, msg: F) -> JFr {
    reduce_to_jfr(hash(&[dom::sig(), r.x, r.y, rk.x, rk.y, msg]))
}

/// Sign with an explicit nonce. The generator draws nonces from its
/// seeded RNG; invalid-case construction uses fixed nonces.
pub fn sign_with_nonce(signing_key: JFr, msg: F, nonce: JFr) -> (Point, Signature) {
    assert!(nonce != JFr::ZERO && signing_key != JFr::ZERO);
    let rk_pt = JubjubSubgroup::generator() * signing_key;
    let rk = subgroup_to_point(rk_pt);
    let r = subgroup_to_point(JubjubSubgroup::generator() * nonce);
    let e = challenge(r, rk, msg);
    let s = nonce + e * signing_key;
    (rk, Signature { r_point: r, s })
}

/// Verify s*G == R + e*rk with prime-order, non-identity points.
pub fn verify(rk: Point, msg: F, sig: &Signature) -> bool {
    if sig.s == JFr::ZERO {
        return false;
    }
    let (Some(rk_g), Some(r_g)) = (point_to_subgroup(rk), point_to_subgroup(sig.r_point)) else {
        return false;
    };
    if rk == Point::identity() || sig.r_point == Point::identity() {
        return false;
    }
    let e = challenge(sig.r_point, rk, msg);
    let lhs = JubjubSubgroup::generator() * sig.s;
    let rhs = r_g + rk_g * e;
    lhs == rhs
}

/// rk bound to pk by alpha: rk == pk + [alpha]G.
pub fn rk_binding_ok(pk: Point, alpha: JFr, rk: Point) -> bool {
    let (Some(pk_g), Some(rk_g)) = (point_to_subgroup(pk), point_to_subgroup(rk)) else {
        return false;
    };
    pk_g + JubjubSubgroup::generator() * alpha == rk_g
}
