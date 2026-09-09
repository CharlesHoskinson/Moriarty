## Amendment to Numeric K Result Audit (`sp03-numeric-01`)

Scope: the supplied audit text, the 64 extracted token vectors, and their listed `sha256`/timing values. No filesystem, no `krun`, no rehashing. This amendment corrects the audit; it does not re-open unreviewed material.

### 1. Constructor tally — corrected, non-material

Correct counts are **22 `prepared` (arity 16) + 6 `preparedTransfer` (arity 9) + 36 `rejected` (arity 3) = 64**. The audit's §1 opening line ("28 prepared … 4 preparedTransfer … 32 rejected") was wrong; its inline self-correction fixed only the Prepared split and left **"all 32 rejected outputs" in the Rejections bullet uncorrected — that reads 36**. The cohort tallies were already right and reconcile: new 22 = 13 Prepared-family + 9 rejected; retained 42 = 15 + 27; totals 28 Prepared-family / 36 rejected.

These are arity/label tallies only. They establish nothing about semantics — equal counts across cohorts are not evidence of equivalent behaviour, and I do not treat them as such.

### 2. `statusOf` claim — over-scoped, now bounded

The audit asserted the status token is `"Settled"` exactly when the outstanding scalar is `0` "in all 28 Prepared rows." **Only the 22 arity-16 Repay outputs carry a status token at all.** Restricted to those 22, the biconditional holds on the extracted vectors: `"Settled"` appears in traces 15, 22, 26, and exactly those three carry `(0,0,0)` in the principal/accrued/outstanding slots; every other Repay row has a non-zero outstanding and `"Outstanding"`.

The six arity-9 `preparedTransfer` vectors (20, 21, 55, 56, 57, 58) contain **no status token and no debt scalars**. Consequently the audit's §2 claim that trace‑21 "preserves P0/A0/O0 `Settled` under Transfer-only" is **not supported by the raw output**: the constructor has no such fields to observe. The correct statement is that the Transfer-only constructor omits debt entirely — debt is carried through unchanged — which is a property of the codec/kernel contract as attested, not something I read off the token vector. I withdraw the trace‑21 wording and mark that property attested rather than verified.

### 3. First-trace aggregate — corrected

`firstTraceAggregateSeconds` is **18.000994734000415**, not 12.355; the audit apparently carried over the compile figure (12.329168…). The corrected value is consistent with compile 12.329 plus a first `krun` ≥ 2.273. Monotonic increase to 328.6530731849998, agreement with the supervisor's 328.769 s, and headroom under the 1512 s `RuntimeMaxUSec` are all unaffected.

### 4. Effect on the verdict

All three slips are errors in the audit's own prose and arithmetic-of-description, not defects in the recorded execution:

- The digest bindings (64 distinct, each matching its case), rejection codes/indices, `receiver = -1` append triple (39, 42, 56 with appended amounts 17/17/19), zero-append closure on all `receiver ≥ 0` rows, and the staging discriminators (traces 09, 12, and the scale‑1/scale‑0 pair) are unchanged by these corrections.
- The one substantive change is a **narrowing**: the Transfer-only debt-preservation claim moves from "verified at token level" to "attested by contract," and the `statusOf` invariant is bounded to 22 rows. Neither was a load-bearing gate.

**Verdict: PASS** for the same result candidate (`38e6bf46…f5f8b`), scope = supplied text plus this correction. Section 6 of the original audit stands: no correspondence, semantic-freeze, proof, or ledger claim, and the source/CLI checker runs and 187-artifact hashes remain attested, not observed.
