// -*- mode: quint; -*-
// E6 Call-Vector Soundness & Completeness Model — module `e6_callvec`
// Re-exports the parametric `LaneSpec` and concrete reduced-width instance from `./callvec`.

module e6_callvec {
  import LaneSpec(
    LIMBS = 3,
    LIMB_BASE = 8,
    CHUNK_BASE = 16,
    R_MOD = 353,
    P_BB = 13,
    PACK_N = 2
  ).* from "./callvec"
}
