// T20 fix 4: L_TX covers every measured entry in one call; chunked calls are a capped fallback.
//
// One logical Compact call of L_e effect words. With the anchor prefix A = 9 lanes, one call carries up to
// L_TX - A = 1,015 words. Above that the plan chunks it into k calls of at most W_CHUNK = 990 words each
// (ATOMIC-CHUNKS: 34 words of chunk glue), k <= K_MAX, or refuses it at plan time with the entry named.
// Each chunk is one wrap proof (175.5 s warm) and one anchor overwrite; the intent TTL must be at least ten
// times the proving time (kit COMPACT FRONTEND), so K_MAX = floor(TTL / (10 * 175.5)) at TTL = 3,600 s.
//
// Entry sizes: 474 is the largest measured call (d04-coracle withdraw, 476 lanes, Mycroft compact-suite);
// 699 is the largest static count (a07-historic-merkletree test, no passing call); the rest are synthetic
// sizes around the bounds.
//
// MUT:
//   "none"           the design (L_TX = 1,024, K_MAX = 2, TTL = 3,600 s)
//   "no_cap"         the plan chunks any size (the audit's k = 5 to 30)
//   "small_ltx"      L_TX = 256 (the Fable a3 assumption): measured entries chunk
//   "cap_truncate"   at the cap the plan keeps K_MAX chunks and drops the rest of the words
//   "cap_four"       K_MAX = 4 at TTL = 3,600 s (the TTL rule breaks)
module chunkCap {
  const MUT: str

  pure val A = 9
  pure val L_TX = if (MUT == "small_ltx") 256 else 1024
  pure val ONE_CALL = L_TX - A
  pure val W_CHUNK = if (MUT == "small_ltx") 222 else 990
  pure val K_MAX = if (MUT == "cap_four") 4 else 2
  pure val TTL = 3600
  pure val PROVE_DS = 1755                 // 175.5 s warm per chunk, in tenths of a second
  pure val MEASURED = Set(474, 443, 328, 287, 10)
  pure val STATIC_MAX = 699
  pure val SIZES = MEASURED.union(Set(STATIC_MAX, 1015, 1016, 1980, 1981, 2970, 5000))

  pure def ceilDiv(a: int, b: int): int = (a + b - 1) / b

  type Plan = { size: int, k: int, covered: int, refused: bool }

  pure def plan(n: int): Plan =
    if (n <= ONE_CALL) { size: n, k: 1, covered: n, refused: false }
    else {
      val k = ceilDiv(n, W_CHUNK)
      if (MUT == "no_cap" or k <= K_MAX) { size: n, k: k, covered: n, refused: false }
      else if (MUT == "cap_truncate") { size: n, k: K_MAX, covered: K_MAX * W_CHUNK, refused: false }
      else { size: n, k: 0, covered: 0, refused: true }
    }

  var plans: Set[Plan]
  var calls: int                           // contract calls (and wrap proofs) over all accepted plans
  var anchorWrites: int

  action init = all { plans' = Set(), calls' = 0, anchorWrites' = 0 }
  action submit(n: int): bool = all {
    val p = plan(n)
    all {
      plans' = plans.union(Set(p)),
      calls' = calls + p.k,
      anchorWrites' = anchorWrites + (if (p.k > 1) p.k else 0),
    }
  }
  action step = { nondet s = SIZES.oneOf() submit(s) }

  // ---- invariants
  // every measured entry and the static maximum run as one call, with no anchor write
  val MeasuredEntriesOneCall = plans.forall(p => (MEASURED.contains(p.size) or p.size == STATIC_MAX) implies p.k == 1)
  // no accepted plan has more than K_MAX chunks (the writes of one logical call are capped)
  val ChunksAtMostCap = plans.forall(p => p.k <= K_MAX)
  // an accepted plan carries every effect word: no silent truncation
  val NoSilentTruncation = plans.forall(p => not(p.refused) implies p.covered >= p.size)
  // the intent TTL covers ten times the proving time of every accepted plan
  val TtlCoversProving = plans.forall(p => 10 * p.k * PROVE_DS <= 10 * TTL)
  // a plan is either accepted with k >= 1 or refused at plan time (never a ledger-side failure)
  val RefusedAtPlanTime = plans.forall(p => p.refused iff p.k == 0)

  val Inv = MeasuredEntriesOneCall and ChunksAtMostCap and NoSilentTruncation and TtlCoversProving and RefusedAtPlanTime

  // ---- non-vacuity: a chunked plan (k = 2) and a refused plan both occur
  val WitChunked = plans.exists(p => p.k == 2)
  val WitRefused = plans.exists(p => p.refused)
  val NotWitChunked = not(WitChunked)
  val NotWitRefused = not(WitRefused)
}
