//// Groth16 / BLS12-381 verifier for ABI v2 claims (P3S1, PROPOSED, uncompiled).
////
//// Reused starting material: the pairing equation, arkworks-0.4 compressed VK/proof
//// layout and the F-I identity refusal of `claim-utxo-abi-v1-combined-repair-candidate-
//// 20261015/source/aiken/lib/pvc/abi1.ak` (`groth16_verify`, `points_non_identity`),
//// with one change: the two public scalars are the big-endian halves of the 32-byte
//// statement digest supplied in the envelope, rather than of a digest computed from
//// envelope fields. The digest is later recomputed by S2 from authenticated facts.
//// F-I profile (two votes, V6/V7): gamma, delta, ic1, ic2 must not be the identity;
//// alpha, beta, ic0 are not checked; this is defence in depth, not VK provenance.

use aiken/builtin
use pvc2/bytes

pub const vk_len: Int = 488

pub const proof_len: Int = 192

/// arkworks 0.4 Vec<G1> length prefix (u64 LE): constant term + two public inputs.
pub const vk_ic_count: ByteArray = #"0300000000000000"

pub const g1_identity_compressed: ByteArray =
  #"c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"

pub const g2_identity_compressed: ByteArray =
  #"c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"

/// Public inputs: x1 = int_be(d[0..16]), x2 = int_be(d[16..32]); both < 2^128 < r.
pub fn scalars_of_digest(d: ByteArray) -> (Int, Int) {
  expect bytes.len(d) == 32
  (
    builtin.bytearray_to_integer(True, builtin.slice_bytearray(0, 16, d)),
    builtin.bytearray_to_integer(True, builtin.slice_bytearray(16, 16, d)),
  )
}

pub fn points_non_identity(
  gamma: G2Element,
  delta: G2Element,
  ic1: G1Element,
  ic2: G1Element,
) -> Bool {
  and {
    builtin.bls12_381_g2_compress(gamma) != g2_identity_compressed,
    builtin.bls12_381_g2_compress(delta) != g2_identity_compressed,
    builtin.bls12_381_g1_compress(ic1) != g1_identity_compressed,
    builtin.bls12_381_g1_compress(ic2) != g1_identity_compressed,
  }
}

/// e(A, B) = e(alpha, beta) * e(ic0 + x1 ic1 + x2 ic2, gamma) * e(C, delta).
pub fn verify_digest(vk: ByteArray, proof: ByteArray, digest: ByteArray) -> Bool {
  expect bytes.len(vk) == vk_len
  expect bytes.len(proof) == proof_len
  expect builtin.slice_bytearray(336, 8, vk) == vk_ic_count
  let (x1, x2) = scalars_of_digest(digest)
  let alpha = builtin.bls12_381_g1_uncompress(builtin.slice_bytearray(0, 48, vk))
  let beta = builtin.bls12_381_g2_uncompress(builtin.slice_bytearray(48, 96, vk))
  let gamma =
    builtin.bls12_381_g2_uncompress(builtin.slice_bytearray(144, 96, vk))
  let delta =
    builtin.bls12_381_g2_uncompress(builtin.slice_bytearray(240, 96, vk))
  let ic0 = builtin.bls12_381_g1_uncompress(builtin.slice_bytearray(344, 48, vk))
  let ic1 = builtin.bls12_381_g1_uncompress(builtin.slice_bytearray(392, 48, vk))
  let ic2 = builtin.bls12_381_g1_uncompress(builtin.slice_bytearray(440, 48, vk))
  let a = builtin.bls12_381_g1_uncompress(builtin.slice_bytearray(0, 48, proof))
  let b = builtin.bls12_381_g2_uncompress(builtin.slice_bytearray(48, 96, proof))
  let c =
    builtin.bls12_381_g1_uncompress(builtin.slice_bytearray(144, 48, proof))
  and {
    points_non_identity(gamma, delta, ic1, ic2),
    builtin.bls12_381_final_verify(
      builtin.bls12_381_miller_loop(a, b),
      builtin.bls12_381_mul_miller_loop_result(
        builtin.bls12_381_mul_miller_loop_result(
          builtin.bls12_381_miller_loop(alpha, beta),
          builtin.bls12_381_miller_loop(
            builtin.bls12_381_g1_add(
              ic0,
              builtin.bls12_381_g1_add(
                builtin.bls12_381_g1_scalar_mul(x1, ic1),
                builtin.bls12_381_g1_scalar_mul(x2, ic2),
              ),
            ),
            gamma,
          ),
        ),
        builtin.bls12_381_miller_loop(c, delta),
      ),
    ),
  }
}
