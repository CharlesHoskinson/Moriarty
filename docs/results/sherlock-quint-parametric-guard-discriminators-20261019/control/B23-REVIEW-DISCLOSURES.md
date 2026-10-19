# B23 additional review disclosures — no source edit

The immutable B22 disclosures and full B23 opinion remain authoritative historical bodies. The optional D7 source diff has NOT been adopted. Source hash remains08cf937257c66e86bfe4175e730800a96079bec038e25bfd346e9fad13793ebc. The following reviewer passages are verbatim and must accompany any result publication.

### New disclosure D7 (non-blocking; must accompany any result publication)

**D7. `gReturnSpaceTwo` is overdetermined.**

- In `pWideMiddle`, the return index is 2, and in the space ≠ 0 branch the bound is `activeData` = 2. So `2 < 2` already rejects.
- Deleting `(returnSpace == 0 or returnSpace == 1)` would still leave the run passing.
- B22 gap 4's "space = 2" item is therefore **not** discriminated by B23. Report it as remaining.

**Optional exact bounded diff** (not required for this approval). Adopting it creates a new source hash and requires a fresh review:

```diff
-  pure val gReturnSpaceTwo: PImage = pChangeImageCell(pWideMiddle,5,0,2,2)
+  pure val gReturnSpaceTwo: PImage = pChangeImageCell(pDataReturnBefore,5,0,2,2)
```

With this change, index 1 < activeData 2, so only the space-domain conjunct rejects. The header remains allowed, and both existing assertions still hold.

---


## Remaining finite coverage limitations (retain with D1–D7)

**B22 gaps 1–6, as now covered by B23:**

- **Gap 1, scalar domain:** closed only as finite samples. Still untested:
  - non-zero ctorId on scalar kinds
  - non-zero fieldStart/fieldLength
  - Bool with typeId ≠ 1
  - fixed active-count mutants (`root[11..13]`)
  - activeCF > 1
  - KF > 1
- **Gap 2, padding:** only Program Expressions (literal input) and State Data (literal and publication inputs) are sampled. Still untested:
  - Program padding at publication input
  - padding in other tables
  - input Control padding beyond the gate tests
- **Gap 3, envelopes:** four samples. Closed for those values only.
- **Gap 4, preconditions:**
  - outcome lanes 43/44 untested on the literal side; outcome lanes 42/44 untested on publication
  - the frame-zero conjunct is redundant under D1 at depth 0
  - **space = 2 remains open (D7)**
  - A102 fuel > 0 and at-active-length index cases rely on B22 samples
- **Gap 5, start/layout/join:**
  - layout cases still untested: K = 0, Q < 4, QF < 5, a negative state capacity, an upper bound on stackLimit
  - complete-start cases still untested: environment present/index ≠ 0, individual root[1..4]
  - the nonempty-arena conjunct split is not isolated
  - the join samples only one discontinuous cell (stackLimit)
- **Gap 6, formula sharing:** removed only for the false-Bool pair. All other expectations still rely on `pHand*` / source-exposed same-author fixtures.

**What this review does not establish:**
- actual PASS
- universal mutation adequacy
- full uiValid (constructors/DAG), per-lane ranges, F4/prepareA, StepInv/O1–O4, dispatch, error precedence, OBL.G4 bytes, projectA
- authenticated context
- Rust or cross-layer correspondence (UNISSUED)
- native, cryptographic or recursive proof
- ledger or stagenet behaviour
- general admission
- EARS/SCON closure

**Qualified as reported, not verified here:**
- the parent-reported `34a8593b` / PR294 merge-pending status
- the WW byte-identity readback of the seven Rust files at the recorded time

Root alone executes, selects and publishes.
