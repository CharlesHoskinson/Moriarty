**Verdict: REQUEST_CHANGES.** This covers integrating the proposed design and plan only. B1 and B2 are now resolved in substance. There is one factual blocker, and it is narrow.

**Reviewer:** Opus 5.5 (`claude-opus-5-5`), high effort, fresh and non-author. All security judgments here are mine.
**Candidate:** `08b1d2f26892043f4d15322f9272113c290de853`. The worktree was clean at the start.
**Baseline:** `9c63048a18315d7eb3cf114b983bd68c0d842dc6`. 12 files changed since `aad502a3`.

## What I checked
- **Startup:** I loaded the develop skill, AGENTS and the product contract. `status --json` shows only the historical SP01 state, which doesn't select this work.
- **Checksums:** all 19 `SHA256SUMS` entries match. The plan, CONCEPT-ADOPTION, ROADMAP, the openspec, models and research READMEs, and the papers manifest all match `publication-binding.json`.
- **UISEC-001–012:** `security-requirements.md` (`dcd15273…`) is byte-identical to `R/OPUS-INTERPRETER-SECURITY-DRAFT.md`.
- **Composition of `security-candidate-b.md`:**
  - It is the B1/B2 author text (`7da270f4…`) before its handoff, with A1 replaced by the erratum text (`9aca0403…`).
  - I confirmed this by comparing lines visually, because the sandbox blocked `diff`.
  - Both RESULT.json files return `claude-opus-5-5` with `is_error:false`.
  - The B1/B2 prompt didn't dictate any resolutions.
- **Frozen sources:**
  - In K, frame overflow is a charged failing push that publishes `uiError(4)`, and zero fuel takes precedence (`models/sherlock/k/main.k:84-86,140-143`).
  - In Quint, `refuse` sets `phase=Rejected` with the given reason (`universalMachine.qnt:66-67`). Push refuses with `StackBound` when `depth > MAX_FRAMES` (`:69-71`), inside a tick that has already been charged (`:156-157`).

## Blocker
**R1. The "frozen Quint spelling" erratum cites a function that doesn't exist.** Frozen Quint (`b7aadd99…`, `universalMachine.qnt:69`) defines `push`. Nothing in the repository or runtime `.qnt` files is named `pushFrame`. The wrong name appears in:
- the normative A1 table, `security-candidate-b.md:27`;
- `interfaces.md:122`;
- `model-coverage.md:60`, the evidence record;
- `design.md:29`, which calls this the "actual Quint refusal spelling".

The error started in the root's erratum prompt (`candidate-b-opus-profile-erratum/prompt.txt:1`), which stated `pushFrame` as literal source, and the author copied it. Correcting the spelling was the erratum's whole purpose, so I can't approve a normative claim about frozen evidence that is false (UISEC-012, B009).

The semantic content is correct: the `depth > MAX_FRAMES` cap and the `phase=Rejected ∧ reason=StackBound` classifier both match the source.

**Fix:** an Opus-authored one-word correction in A1 (`push`), carried mechanically into the three other files, then a fresh exact-candidate audit. No semantic change or implementation is needed. With only that change, I expect to approve.

## Prior blockers, assessed myself
- **B1 is resolved.**
  - `security-candidate-b.md:12` now says UISEC binds "except for the profile-specific amendments A1–A4", which matches what the file does.
  - A1 (`:20-36`) gives each profile its own overflow spelling. Overflow is terminal, explicit and charged in each profile.
  - A1 states that no consumer may read a resourceLimit outcome as a semantic, typing or financial rejection, and that UISEC-007 and B008 still hold.
  - A2 (`:39-54`) and `interfaces.md:43-47` define `stepCap` and `admit`. `CapacityNotAdmitted` stays outside `Outcome`.
- **B2 is resolved.**
  - A3 (`:56-62`) says UISEC-003 covers the physical arena between boundaries and also the logical allocation counters.
  - A4 (`:64-90`) limits canonical decoding to boundaries. It defines `StepInv`, states the genesis and preservation obligations (unproved), lists the per-step constraints, and requires the physical post-image to be deterministic.
  - `materializeStep` names `decodeStep` (`interfaces.md:236`).
- **Integration:**
  - Error publication is now zero-cost in UIREQ-028 (`spec.md:199`).
  - `tasks.md:16` is current.
  - The Phase 0, Phase 3 and every-phase Opus gates are quoted verbatim in the plan (`docs/SHERLOCK-IMPLEMENTATION-PLAN.md:103-119,155-157`).
  - The full error, rule, charge, cap and layout tables are honestly left as Phase 0 obligations.

## Non-blocking (Opus-owned follow-ups)
- **N1. Boundary binding.** Per-step statements bind physical digests, but a ChunkStore or checkpoint resume starts from a projected canonical image (`security-candidate-b.md:66,90`; `interfaces.md:240`). Before any chunked or recursive realization claim, `project` must be constrained or decider-checked between a physical post-digest and the next canonical pre-digest. Otherwise the boundary is a point where state can be substituted. This is missing, not contradictory.
- **N2. Where `Exhausted(HelperWork)` sits in the precedence order.** Line 43 says it fires before the charge ("would exceed"), but item 4 at `:52` lists helper caps after the charge. "Reached" versus "exceeded" also needs settling at Phase 0.
- **N3. "Layout-declared reusable cells"** (A3, `:62`) must be listed at Phase 0. That list must never include data rows or fields, and it should state whether environment cells are reused.
- **N4. A2 applies only to B.** For the A profiles, UISEC-006's word "reject" is covered only implicitly by A1. The predicates hold (`main.k:84-86`, `.qnt:153-154`), so this isn't a contradiction, but it should be stated.
- **N5. Stale wording.** `tasks.md:76` still says "partial encode/decode", and `traceability.md:145` still says "Compatibility is OPEN" for UISEC-004.
- **N6. A4 determinism inputs.** A4's determinism premise (`:88`) must name the bound external inputs once financial observations are added.
- **N7. Local paths.** `models/sherlock/observations.json` still contains `/home/charl` paths. This is hygiene, not a secret.

## Evidence status (unchanged and correctly scoped)
- **K:** frontend syntax and Kore text only. No rules were executed.
- **Quint:** 8 tests; 32 traces with seed 1 and at most 32 steps; no `allInv` violation; all seven witnesses (`observations.json:171-186`).
- **B:** no implementation, model, circuit, native, proof, correspondence or performance evidence exists.
- **Native authority:** still 0.
- **Private material:** no Fenrir access, and no private pins or keys appear in public files.

## Prior votes
- The `aad502a3` Opus REQUEST_CHANGES still stands for those bytes.
- `candidate-b-b1b2-opus-review` and `candidate-b-b1b2-muse-review` contain no RESULT, so neither has voted.
- I didn't assess the Muse final review. Earlier approvals don't cover these bytes.

## What this review cannot establish
- No soundness, `StepInv` preservation, codec or projection correctness, refinement, cost, privacy, ledger or native result.
- I ran no builds, tests, K, Quint, Lean or prover commands.
- An approval would cover source integration only. It would not be adoption, a semantic freeze, security, product or release acceptance, or one of the two required decision votes. It would not close any U0–U7, M1–M17, MC, SP, G, ZR, MNR or OSR exit, or any ACTUS, DeFi or composition gate.

**Dissent:** none, apart from R1. The Opus02/Opus05 disagreement over realizations properly remains open.
