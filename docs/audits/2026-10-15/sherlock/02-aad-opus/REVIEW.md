I'm not approving this candidate. Two security requirements contradict the new reconciliation text, and both can be fixed by editing the documents alone.

# Opus 5.5 security audit: Sherlock Candidate B design-and-plan integration

**Reviewer:** Opus 5.5 (`claude-opus-5-5`), high effort. I'm a fresh non-author and wrote none of the security text.
**Candidate:** `aad502a3ec8d60100a779a4e55942e5db5f24107` on `feat/universal-interpreter-spec-20261015`; the worktree is clean.
**Baseline:** `9c63048a18315d7eb3cf114b983bd68c0d842dc6`. Delta from `d1d11108`: 9 package files.
**Verdict: REQUEST_CHANGES**, scoped to integrating the proposed design and plan.

## What I checked

- **Skill and status:** I loaded the checked-in develop skill and AGENTS. The read-only `status --json` run reported historical SP01 state only, which doesn't select this work.
- **Checksums:** `SHA256SUMS` passes for all 19 package files. The plan, CONCEPT-ADOPTION, ROADMAP, `openspec/README`, the provenance file and the UISEC file all match their `publication-binding.json` hashes.
- **UISEC-001–012:** `security-requirements.md` (`dcd15273…`) is byte-identical to the runtime `OPUS-INTERPRETER-SECURITY-DRAFT.md`.
- **Provenance:**
  - The d1 `security-candidate-b.md` hashes to `123f1624…`. That equals `TIMED-OUT-PARTIAL-SOURCE.md`, so the partial artifact really was removed.
  - The current file (`e481ef01…`) has lines 3–32 identical to `candidate-b-opus-override/RESULT.md` (`c8f52820…`). The rest is identical to `ORIGINAL-RESTORED.md` (`f191706e…`). The composition claim holds.
  - `RESULT.json` shows the canonical model `claude-opus-5-5` and `is_error: false`.
- **Limitation on the reconciliation:** its prompt (`candidate-b-opus-override/prompt.txt`) dictated each disposition and told the author "no further scope needed". The author therefore transcribed the root's resolutions without checking them against UISEC-003/004. The authorship provenance is honest, but the text had no substantive security analysis before this review.

## Blockers (normative contradictions, not missing implementation)

### B1. UISEC-004 contradicts the error4 disposition, but is declared "unchanged and binds"

- `security-requirements.md:52` says frame overflow "SHALL yield an explicit `Exhausted` outcome".
- The reconciliation says frame-cap overflow "remains `Rejected(error4)`" for both A-K and B (`security-candidate-b.md:8,10`). `interfaces.md:55-56,102,104,110`, `design.md:27,29,33` and `spec.md:87` agree. The `Outcome.Exhausted` kinds deliberately leave out `Frames`.
- The precedence clause only overrides wording in C1–C4, B001, B002, B006 and B007 (`security-candidate-b.md:5`), not UISEC-004.
- Line 34 then says UISEC-001–012 are "unchanged and bind[s]", while line 5 calls the same document "frozen as historical requirements".
- Result: every named profile violates a requirement the package says still binds. Disposing of C2 only rewrites the record of the conflict, not UISEC-004 itself.
- The same class of problem is weaker in UISEC-006 (`:61`). It says "SHALL reject" on budget overflow, while B publishes `Exhausted(Fuel)`.

**Required fix:** an Opus-owned clause that maps the UISEC outcome words to concrete `Outcome` constructors for each profile. Either amend UISEC-004's overflow arm explicitly (error4: charged, terminal, explicit, never success, never silent truncation), or choose `Exhausted(Frames)` for B. Then make line 34 accurate, for example "binds except as dispositioned above".

I'd accept keeping error4. What matters for security is that overflow stays explicit, charged and terminal; the constructor name matters less. But a resource limit classified as `Rejected` must not be read downstream as a semantic or financial rejection. Requirements UISEC-007 and B008 already keep duties alive under every outcome.

### B2. The canonical-image rule contradicts the immutable heap (UISEC-003) and the bounded-step contract

- UISEC-B002 (`security-candidate-b.md:52-53`) says `decode` accepts only canonical images, and that every live slot is reachable with no unreachable bytes.
- `materializeStep` (`interfaces.md:186`) decodes every step's output image. So each step must produce a canonical image, which means removing nodes that became unreachable (for example when a frame pop drops the last reference).
- That conflicts with three other requirements:
  - UISEC-003 (`security-requirements.md:48-49`): "`H` restricted to `[0,nextLoc)` is unchanged by every step".
  - The bounded fixed-step contract: "No whole-state `decode` … inside a B step" (`interfaces.md:92`).
  - The reconciliation lets a canonical projection "compact" storage (`security-candidate-b.md:28`; `interfaces.md:148`) but never says when compaction happens. If it happens every step, it is unbounded garbage collection.
- The reconciliation says C3 is disposed of, but it doesn't settle whether UISEC-003 applies to the arena in each step or to the logical allocation history.

**Required fix:** specify where canonicality applies.
- Canonical-only decode applies at codec, checkpoint, `ChunkStore` and statement boundaries.
- Step images in between satisfy a stated inductive invariant: typed nodes, child below parent, zeroed inactive fields, and unreachable slots never observable or reusable without a full overwrite.
- The invariant must hold from an authenticated initial image, because no per-step revalidation is allowed.
- `materializeStep` must name which decoder it uses.
- UISEC-003 must name whether it governs the physical arena or the logical allocation counters.

## Areas I examined and found consistent as proposed design

- **Profiles and helper costs:**
  - A-K, A-Quint and B are cleanly separated, and nothing transfers between them without a differential predicate.
  - In B, each legacy completion and each helper event costs 1; startup and publication cost 0.
  - The counters are distinct: `helperWork`, `helperVisits` and `helperTasks`, with `consumedWork` including helper work. Caching never removes a charge (`interfaces.md:104,110`).
  - Failure precedence is publication, then fuel, then frame cap, then data/env/helper caps. That is deterministic enough for a design. Tying it into UISEC-B001 (check before work) is an implementation obligation.
- **NoMatch:** it maps to `Rejected(error9)` and adds no `Outcome` variant. It agrees with UISEC-002's explicit-reject arm and is consistent across `interfaces.md:257`, `design.md:65` and the reconciliation.
- **Typed ADT first-match:** it takes the minimum source arm, evaluates the scrutinee once, binds fresh and uses occurrence paths rather than addresses. Both `workMap` and B007 keep the charges. I hand-checked the D-cases: first arm gives 4, the structural sum gives 9, and repeated `Pay4` gives 8.
- **Exact u128:** values must be exactly in `[0,2^128)`. The full 256-bit product is kept, error10 applies, and field residues are not treated as u128. Division and rounding stay outside scope until specified, and UISEC-005 still binds when they are added.
- **Witness vs constraint:** B005 and `interfaces.md:228` keep `cacheHit`/`rowGenerated` as diagnostics. Acceptance depends only on emitted constraint components, and JSON is not a witness plan.
- **Static provenance vs dynamic runtime:** immutable occurrence IDs may be static. Ordinals, control, fuel, frames and outcomes stay dynamic. No unrolling is allowed, and the size-limit fallback is generic dispatch, not exhaustion (`interfaces.md:243`).
- **Codec:** deterministic roots, child-before-parent ordering and zeroed inactive fields are stated as an obligation only. That is acceptable apart from the B2 timing gap.
- **Financial Core/2:**
  - Successful-prefix await, residual duties, ordered effects, gross debit and fees are all retained.
  - A chunk is not a commit, and Commit/Yield/Await stays open.
  - The worked arithmetic is correct: reserve 100 → 68 → 57, debit 32 → 43, liability 40 → 10 → 0, receipts 40.
- **Secret/private exclusion:** I found no private Fenrir paths, pins or keys. The public Clean pin is upstream `1a6c2842…`.
- **Retained parent requirements:** U/M/MC/SP/G/ZR/MNR/OSR stay open. No VerifyProof or ledger10 prerequisite is added, and the backend winner is left unsettled.

## Implementation obligations correctly left open (not blockers)

These can stay open and be honestly labelled before the Phase 0 freeze:
- the full error-code table;
- charge and allocation rows for each rule and each failure transition;
- the order among data/env/helper caps within a step;
- how `maxHelperWork` relates to `maxReferenceWork`;
- root enumeration and the projection algorithm;
- codec proofs;
- exact helper rule tables;
- the backend and realization winner;
- all hostile tests and theorems, which belong to me or a later Opus owner.

## Non-blocking findings

1. `tasks.md:16` still reads "error4 versus proposed exhaustion classification", which is stale after the reconciliation.
2. `spec.md:199` (UIREQ-028) lists startup and terminal publication as zero-cost but leaves out error publication.
3. The phase predicates in the implementation plan accept normal cases only. For example, Phase 3 `bounded_decode_matches` doesn't depend on the B002 hostile scenarios (garbage in an inactive slot, a forged unreachable node). I recommend that Opus-authored B001–B009 hostile cases gate each phase's B-specific claims, not just Phase 8.
4. `models/sherlock/observations.json:30-36` and later rows publish local `/home/charl/...` tool paths. These are host-path hygiene, not secrets.
5. The design should state explicitly that typing and child-below-parent must be enforced by per-step constraints from an authenticated genesis image. With no per-step decode, this inductive invariant is the core defence against forged images in a universal circuit. It is part of the B2 fix.

## What this review cannot establish

- **No soundness, completeness or correspondence.** There is no K/Lean/Clean refinement, circuit, native or ledger correctness, and none of the cost, privacy or measurement claims are checked.
- **No unselected decisions settled.** This review does not settle the backend winner, the exact helper or error tables, or the Commit/Yield/Await opcodes.
- **Nothing was executed.** I ran no builds, tests, K/Quint, Lean, prover or ledger commands, and I accessed no private Fenrir material.
- **Old model evidence doesn't apply.** The K frontend and Quint sample evidence covers A only and doesn't validate B.
- **Not an adoption vote.** An APPROVE after the fixes would cover integrating the source only. It would not be adoption, a semantic freeze, product, security or release acceptance, and it would not stand in for the separate Muse audit or the two-vote decision rule. The earlier Muse d1 APPROVE does not cover these bytes.

## Path to APPROVE

An Opus-owned amendment that resolves B1 and B2 and corrects `tasks.md:16`. That needs changes to the documents only, followed by a fresh exact-candidate re-audit. No implementation is required first.

**Dissent:** none recorded beyond the blockers above. The Opus02/Opus05 preference split on realizations remains properly unresolved.
