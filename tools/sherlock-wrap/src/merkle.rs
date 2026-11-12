//! I2: Merkle paths of the `L_Fr` trees. Leaf hash at `u_leaf` (Midnight fixed-length
//! sponge over `w` native Fr values, rate 2), node hash at `u_node`, the `tree_compress` of
//! the `2^k_whir` leaf digests of one query (pairwise, in order), and the path from the query
//! digest to the root with the direction bits taken from the sampled index bits (the same
//! cells; stock `merkle_verify`: bit 0 means the current node is the left input).

use midnight_proofs::plonk::Error;

use crate::{
    alg::Alg,
    native::{RATE, cap_fixed},
};

/// Leaf hash of `vals` (stock `FrMerkleHasher::hash_slice`).
pub fn hash_slice<A: Alg>(a: &mut A, u: u128, vals: &[A::V]) -> Result<A::V, Error> {
    assert!(!vals.is_empty());
    let zero = a.cst(<crate::native::F as ff::Field>::ZERO)?;
    let cap = a.cst(cap_fixed(u, vals.len() as u128))?;
    let mut s = [zero.clone(), zero, cap];
    for (i, block) in vals.chunks(RATE).enumerate() {
        for (j, x) in block.iter().enumerate() {
            // The first block lands on zero cells: no addition row.
            s[j] = if i == 0 { x.clone() } else { a.add(&s[j], x)? };
        }
        s = a.permute(&s)?;
    }
    Ok(s[0].clone())
}

/// Node hash (stock `FrMerkleHasher::compress`).
pub fn compress<A: Alg>(a: &mut A, u: u128, left: &A::V, right: &A::V) -> Result<A::V, Error> {
    let cap = a.cst(cap_fixed(u, 2))?;
    Ok(a.permute(&[left.clone(), right.clone(), cap])?[0].clone())
}

/// Stock `tree_compress`: pairwise compression of a power-of-two list, in order.
pub fn tree_compress<A: Alg>(a: &mut A, u: u128, mut h: Vec<A::V>) -> Result<A::V, Error> {
    assert!(h.len().is_power_of_two());
    while h.len() > 1 {
        let mut next = Vec::with_capacity(h.len() / 2);
        for p in h.chunks_exact(2) {
            next.push(compress(a, u, &p[0], &p[1])?);
        }
        h = next;
    }
    Ok(h.pop().unwrap())
}

/// Stock `merkle_verify`: from `leaf` with `siblings`; the direction at level `j` is `bits[j]`
/// (bit 0: current is left); asserts the result equals `root`.
pub fn merkle_verify<A: Alg>(
    a: &mut A,
    u: u128,
    root: &A::V,
    bits: &[A::B],
    leaf: &A::V,
    siblings: &[A::V],
) -> Result<(), Error> {
    assert_eq!(bits.len(), siblings.len());
    let mut cur = leaf.clone();
    for (b, sib) in bits.iter().zip(siblings) {
        let left = a.select(b, sib, &cur)?;
        let right = a.select(b, &cur, sib)?;
        cur = compress(a, u, &left, &right)?;
    }
    a.assert_eq(&cur, root, "merkle root")
}
