//// K8 genuine-proof consumer tests (source-only, uncompiled, no results claimed).
////
//// These tests import the UNCHANGED pvc2/groth16 verifier and the GENERATED
//// pvc2/k8_fixture module (produced by tools/generate_k8_fixture.py from the
//// frozen PUBLIC K6/K7 exports). They do not import or modify any production
//// verifier or validator. Expectations were fixed before execution:
////   positive  verify_digest(vk, proof, statement_digest)      == True
////   N1        verify_digest(vk, proof, n1_statement_digest)   == False
////             (first possible refusal is the pairing equation itself:
////             lengths and the IC prefix are unchanged, points are the
////             positive points, so no length/prefix guard fires first)
////   N2        verify_digest(vk, proof, n2_statement_digest)   == False
////             (same actual pairing path as N1, with the re-derived
////             context digest; seq 1 -> 2 in the public export)
////   N3        verify_digest(vk, n3_proof, statement_digest)   == False
////             (A and C are both valid positive G1 points swapped, so
////             uncompress succeeds and the pairing equation must refuse)
//// Negative cases use explicit !verify_digest(...) == True assertions on the
//// Bool result, never bare `fail` annotations, so a False is observed rather
//// than any evaluation error being counted as a pass. The single `fail` test
//// below is the optional malformed-length case; verify_digest's own length
//// expects are the check under test there and nothing else depends on it.
//// The C2 claim.issue consumer is intentionally not stubbed here: see the
//// README section naming the precise C2 adapter gap.

use aiken/builtin
use pvc2/bytes
use pvc2/groth16
use pvc2/k8_fixture

test k8_positive_verify_digest() {
  groth16.verify_digest(k8_fixture.vk, k8_fixture.proof, k8_fixture.statement_digest)
}

test k8_n1_body_rederived_digest_rejected() {
  !groth16.verify_digest(k8_fixture.vk, k8_fixture.proof, k8_fixture.n1_statement_digest)
}

test k8_n2_context_rederived_digest_rejected() {
  !groth16.verify_digest(k8_fixture.vk, k8_fixture.proof, k8_fixture.n2_statement_digest)
}

test k8_n3_proof_ac_swapped_rejected() {
  !groth16.verify_digest(k8_fixture.vk, k8_fixture.n3_proof, k8_fixture.statement_digest)
}

test k8_scalars_are_be128_halves_of_statement_digest() {
  groth16.scalars_of_digest(k8_fixture.statement_digest) == (k8_fixture.x1, k8_fixture.x2)
}

test k8_n1_scalars_are_be128_halves_of_n1_digest() {
  groth16.scalars_of_digest(k8_fixture.n1_statement_digest) == (
    k8_fixture.n1_x1,
    k8_fixture.n1_x2,
  )
}

test k8_n2_scalars_are_be128_halves_of_n2_digest() {
  groth16.scalars_of_digest(k8_fixture.n2_statement_digest) == (
    k8_fixture.n2_x1,
    k8_fixture.n2_x2,
  )
}

test k8_byte_profile_matches_groth16_layout() {
  and {
    bytes.len(k8_fixture.vk) == groth16.vk_len,
    bytes.len(k8_fixture.proof) == groth16.proof_len,
    bytes.len(k8_fixture.n3_proof) == groth16.proof_len,
    bytes.len(k8_fixture.statement_digest) == 32,
    bytes.len(k8_fixture.n1_statement_digest) == 32,
    bytes.len(k8_fixture.n2_statement_digest) == 32,
    bytes.len(k8_fixture.statement) == 721,
    builtin.slice_bytearray(336, 8, k8_fixture.vk) == groth16.vk_ic_count,
    bytes.len(builtin.slice_bytearray(0, 48, k8_fixture.vk)) == 48,
    bytes.len(builtin.slice_bytearray(48, 96, k8_fixture.vk)) == 96,
    bytes.len(builtin.slice_bytearray(144, 96, k8_fixture.vk)) == 96,
    bytes.len(builtin.slice_bytearray(240, 96, k8_fixture.vk)) == 96,
    bytes.len(builtin.slice_bytearray(344, 48, k8_fixture.vk)) == 48,
    bytes.len(builtin.slice_bytearray(392, 48, k8_fixture.vk)) == 48,
    bytes.len(builtin.slice_bytearray(440, 48, k8_fixture.vk)) == 48,
    builtin.slice_bytearray(0, 48, k8_fixture.proof) == builtin.slice_bytearray(
      144,
      48,
      k8_fixture.n3_proof,
    ),
    builtin.slice_bytearray(48, 96, k8_fixture.proof) == builtin.slice_bytearray(
      48,
      96,
      k8_fixture.n3_proof,
    ),
    builtin.slice_bytearray(144, 48, k8_fixture.proof) == builtin.slice_bytearray(
      0,
      48,
      k8_fixture.n3_proof,
    ),
  }
}

test k8_negative_cases_are_distinct_from_positive() {
  and {
    k8_fixture.n1_statement_digest != k8_fixture.statement_digest,
    k8_fixture.n2_statement_digest != k8_fixture.statement_digest,
    k8_fixture.n1_statement_digest != k8_fixture.n2_statement_digest,
    k8_fixture.n3_proof != k8_fixture.proof,
    builtin.blake2b_256(k8_fixture.vk) == k8_fixture.vk_digest,
    builtin.blake2b_256(k8_fixture.statement) == k8_fixture.statement_digest,
  }
}

test k8_short_vk_is_refused() fail {
  groth16.verify_digest(#"00", k8_fixture.proof, k8_fixture.statement_digest)
}
