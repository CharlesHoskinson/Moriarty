// Main module for the Haskell backend (symbolic execution, kprove).
//
//   kompile zkir-symbolic.k --backend haskell        (in this directory)
//   kprove ../claims/<file>.k --definition zkir-symbolic-kompiled --spec-module <MODULE>
//
// The module is ZKIR-VM (the base surface, no extension) with the additions
// a proof needs and nothing that changes a concrete run:
//
//   Uninterpreted hashes. The defining rules of poseidonHash, varLenSponge,
//   hashToCurve, sha256Bytes and keccak256Bytes and of their helpers
//   (zkir-hash.k) and of ecMul with its accumulator (zkir-curves.k) carry
//   [concrete], so they evaluate only on fully concrete arguments. On a
//   symbolic argument the application stays as a term; what the prover has of
//   it is its sort, its totality, functionality (two applications to the same
//   argument are the same term) and, for poseidonHash, the range lemma below
//   (a digest is a field element). No collision resistance, injectivity or
//   preimage fact is assumed. transientCommit carries no [concrete]: it is
//   the alias transientCommit(Vs, R) = poseidonHash(ListItem(R) Vs) and
//   unfolds once, so the witness-side commitment check and the commitment
//   gate meet on the same term. A concrete argument still evaluates to the
//   real digest, as on the LLVM backend.
//
//   Reduction lemmas. modInt over a positive modulus is idempotent and a
//   congruence for +Int, *Int and negation; these let a claim state a field
//   result in reduced form (for instance ((A +Int B) *Int C) modInt #r for a
//   mul of a sum) and have it meet the term the semantics builds
//   ((((A +Int B) modInt #r) *Int C) modInt #r). The reduction of an
//   already-reduced value, X modInt P => X when 0 <= X < P, is the lemma of
//   the K prelude (INT-SYMBOLIC in domains.md) and is not repeated. fadd, fmul,
//   fneg with a concrete modulus need no lemma: their defining rules
//   (zkir-field.k) unfold on symbolic operands because their only condition,
//   P >Int 1, is decided by the modulus.
//
//   Range lemmas. A Euclidean remainder by a positive modulus lies in
//   [0, modulus), and a Poseidon digest lies in [0, #r): both are needed by
//   wellTyped (ZKIR-CONSTRAINTS), which demands 0 <= X < #r of every native
//   register, on a register holding a reduced sum or a hash of symbolic
//   inputs. The digest lemma is a fact of the codomain of a total function
//   into the field, not a property of the hash.
//
//   The observable result of a run, [[P]](pre) = obs(status, encoded
//   outputs, public inputs, skips), is written to the <observable> cell by
//   #observable at the end of job(P, pre) (zkir-vm.k `observable`), so a
//   claim states it by pinning that one cell (claims/README.md,
//   tools/run_claims.py `spec_compiled_observable`).
requires "zkir-vm.k"

module ZKIR-SYMBOLIC
  imports ZKIR-VM

  // --- modInt reduction lemmas (P > 0) ----------------------------------------
  rule (X modInt P) modInt P => X modInt P
    requires P >Int 0 [simplification, preserves-definedness]
  rule ((X modInt P) +Int Y) modInt P => (X +Int Y) modInt P
    requires P >Int 0 [simplification, preserves-definedness]
  rule (Y +Int (X modInt P)) modInt P => (Y +Int X) modInt P
    requires P >Int 0 [simplification, preserves-definedness]
  rule ((X modInt P) *Int Y) modInt P => (X *Int Y) modInt P
    requires P >Int 0 [simplification, preserves-definedness]
  rule (Y *Int (X modInt P)) modInt P => (Y *Int X) modInt P
    requires P >Int 0 [simplification, preserves-definedness]
  rule (0 -Int (X modInt P)) modInt P => (0 -Int X) modInt P
    requires P >Int 0 [simplification, preserves-definedness]

  // --- range lemmas (P > 0) ----------------------------------------------------
  rule 0 <=Int (_ modInt P) => true requires P >Int 0 [simplification, preserves-definedness]
  rule (_ modInt P) <Int P => true requires P >Int 0 [simplification, preserves-definedness]
  rule 0 <=Int poseidonHash(_) => true [simplification]
  rule poseidonHash(_) <Int 52435875175126190479447740508185965837690552500527637822603658699938581184513 => true [simplification]   // #r as a literal: a lemma head takes no function symbol
endmodule
