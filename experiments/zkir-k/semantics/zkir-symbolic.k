// Main module for the Haskell backend (symbolic execution, kprove).
//
//   kompile zkir-symbolic.k --backend haskell        (in this directory)
//   kprove ../claims/<file>.k --definition zkir-symbolic-kompiled --spec-module <MODULE>
//
// The module is ZKIR-VM (the base surface, no extension) with the additions
// a proof needs and nothing that changes a concrete run:
//
//   Uninterpreted hashes. The defining rules of poseidonHash, transientCommit,
//   varLenSponge, hashToCurve, sha256Bytes and keccak256Bytes (zkir-hash.k)
//   and of ecMul (zkir-curves.k) carry [concrete], so they evaluate only on
//   fully concrete arguments. On a symbolic argument the application stays as
//   a term, and the only property the prover has of it is functionality: two
//   applications to the same argument are the same term. No collision
//   resistance, injectivity or range fact is assumed. A concrete argument still
//   evaluates to the real digest, as on the LLVM backend.
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
//   The observable result of a run, [[P]](pre) = (status, outputs, public
//   inputs), is read off the <status>, <outputs> and <pi> cells after job(P,
//   pre) has finished; the job rule rewrites the whole <k> cell, so nothing
//   can be sequenced after it (claims/README.md, tools/run_claims.py
//   `spec_compiled_observable`).
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
endmodule
