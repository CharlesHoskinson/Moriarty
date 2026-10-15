# Moriarty K-A Quint security review: candidate e729ce2e

**Reviewer:** claude-opus-5-5, high effort. This was a fresh context. I authored none of the candidate. I made no tool calls, executions, builds, edits, network requests or delegations. I read only the supplied packet.

**Evidence class:** this is a review of supplied source and supplied observations. I did not recompute any hash, decompress any gzip member, or rerun Quint. Hash and output fidelity rests on the root's separate verification.

## Verdicts

| Area | Verdict |
|---|---|
| SOURCE | **APPROVE** |
| SPEC | **APPROVE** |
| QUALITY | **APPROVE** (non-blocking nits below) |
| RESULT_EVIDENCE | **APPROVE** (non-blocking binding gaps below) |

**Concrete blockers:** none.

**What approval covers:** only the finite K-A Quint model source `c62c12dd…` and test source `985069bf…`, plus the recorded normal observations: RED on the `ef41…` stub, then 98/98 GREEN.

**What approval does not cover:**
- a B-profile vote or schema freeze
- any security property or hostile/negative coverage
- malformed-program admission
- K runtime execution
- cross-layer correspondence
- Clean/handwritten constraints
- native or stagenet results
- roadmap exits

## What I checked by inspection

### 1. Rule-line fidelity

I counted every line of the pinned `main.k` in the packet. All 32 rule lines in `kProfileMachine.qnt` match their K rule start lines:

- Lifecycle: 74, 80, 82, 84
- Eval forms: 102, 105, 108, 111, 114, 117, 120, 123, 126, 129, 131
- Push: 135, 140
- Return frames: 145, 149, 153, 157, 161, 165, 169, 173, 177, 181, 185, 190, 195
- Call/match/primitive: 200, 202, 204, 206, 208, 212, 214, 217, 220, 223, 225

This includes the lines the tests never exercise: 105, 111, 129, 140, 169, 177, 190, 202, 206, 212 and 225.

### 2. Transition semantics compared with K, rule by rule

- **Push:** the model's frame-count check equals K's `<depth>` check. Depth tracks frame count under every K stack rewrite: replacements keep it, pops reduce it, pushes increase it.
- **Let and Left:** these replace the top frame without changing depth.
- **Right, Unary, If, Match, final Args, Return and Scope:** these pop and restore environments exactly as K does.
- **Failure rules keep state unchanged:**
  - failed push (140)
  - failed If type check (169)
  - failed return type check (190)
  
  Each retains the original frames and environment, as K does.
- **Invoke → Enter → Bound → Push(ReturnF(T, Old), …, New):** matches K, including fresh parameter bindings and saving the caller's environment.
- **Cases → CaseResult:** ordered arms and NoMatch continuation match K.

### 3. Free-publication precedence

`advance` checks in this order:
1. `outcome != Pending` → no-op
2. Return with empty frames → publish Done (80)
3. Return with frames at fuel 0 → Exhausted (84)
4. Error → publish Rejected (82) at any fuel
5. Otherwise, fuel 0 → Exhausted (84)

This matches K's `uiTerminal` partition and `F >Int 0` guards, and it matches `reference.rs::step`.

### 4. First-match function lookup and source order

- `findFunction` is a left fold that keeps the first `FoundFunction`. This is equivalent to K's `uiFind` and Rust's `iter().find`.
- Nothing is hard-coded to function 11. Calls carry names, and every dispatch re-reads the immutable function table.
- The only constant program in the model is the default `init`, which is harmless.

### 5. Binding and pattern helpers

- **`bindParameters`:** the length check plus an all-must-pass foldr over type and duplicate-name checks is result-equivalent to K's `uiBind`/`uiBindOne`. Order does not change the outcome.
- **PCons/PBranch:** both reject equal names and wrong value kinds.
- **PBranch:** on well-formed prefix trees, `treeExtent` produces the correct ordered split into two subtrees.

### 6. Numeric domains

- `MAX_UINT128` equals 2^128 − 1, which matches K's bound `N <Int 2^128`.
- Add, Sub and Mul use mathematical integers followed by `checkedUInt`, which returns error 10 out of range.
- Type mismatches return error 11.
- `Eq` is structural with no type error, which matches K's `==K`.
- Prefix-token encoding is injective on well-formed trees, so token-list equality is the same as tree equality.

### 7. The value domain is closed under transitions

This is the key soundness point for the declared domain.

- Literals pass through `validValue` (rule 102) or fail (rule 105, error 2).
- `checkedUInt`, LeafOp, ConsOp and BranchOp all produce well-formed values from well-formed inputs.
- PCons, PLeaf and PBranch extract well-formed sub-values.

So every value reachable from `initProgram` on a well-formed closed table stays inside the typed K projection. The structural shortcuts are therefore never reached on out-of-domain values: PLeaf only checks the head token, and PBranch relies on `treeExtent`.

### 8. Test independence and transcription

- **Program tables:** I compared the N01–N06 tables against `normal-programs.json`. N04 shares ExprId 5 between two identical `uiVar(0)` occurrences, which the declared projection permits.
- **Rule sequences:** I compared all six exact-trace rule sequences against `normal-prefixes.json`. All match.
- **Full states:** I compared these complete states field by field, and all match:
  - N02 states 1–7
  - N04 states 4–17
  - N05 states 4, 13 and 19
  - N06 states 7 and 14
- **Prefix rows:** I compared these rows against the oracle reconstruction rule (state[f] with fuel f/0, then publication), and all match:
  - N01: fuel 0–2
  - N04: fuel 10 and 12
  - N05: fuel 4 and 26
  - N02 and N05 overfuel rows
- **Counts:** 3 + 9 + 11 + 19 + 29 + 21 = 92 prefix tests, plus 6 exact traces, gives 98. The `--match '^n0[1-6].*Test$'` filter selects exactly those 98, and the GREEN stdout lists 98.
- **Limit:** this is a manual spot-check, not a mechanical comparison of every field in all 98 tests.

### 9. RED to GREEN

- The RED stub `ef41…` failed N01's second `expect` with actual `Return(UInt(0))` against literal `Return(UInt(7))`. Initialization succeeded first (`initProgram … => true`).
- The test bytes `985069…` are the same in the RED receipt, the typecheck-green receipt and the GREEN receipt.

### 10. Sandbox

All receipts show the same sandbox:
- `bwrap --ro-bind / /`, with only a scratch directory writable
- `--unshare-net`
- 4 GB memory ceiling, swap 0, 256 tasks, 60 s timeout

GREEN stderr is the SHA256 of empty input (`e3b0…`), consistent with the claimed empty stderr.

## Findings and limits (none blocking)

**F1 (security coverage, disclosed).** No error path is exercised: codes 2–11, including the frame-cap path (Error 4, rule 140) that `Outcome::is_resource_limit` classifies as a resource limit. These cases are also untested:
- stack limit actually reached (maximum observed depth is 3)
- UInt128 overflow
- multi-argument calls
- duplicate function names (first-match)
- mutual recursion
- PBranch

I checked these by source inspection only. Hostile and negative cases stay with the security owner as separate future work. A passing normal trace does not satisfy them.

**F2 (out-of-domain behavior must not become an oracle).**

| Input | Model behavior | K counterpart |
|---|---|---|
| Missing ExprId (`expressions.get` in `evaluate`) | Quint runtime evaluation error | None |
| `returnValue` / `frames.head()` with no frames | Quint runtime error (unreachable from init) | None |
| ExprId cycles | Fuel-bounded per step | None |
| Empty or trailing-token tree literals | Defined but invented Error(2) | Outside the typed K tree projection |
| `EmptyControl` with outcome `Pending` | Would exhaust | K's `.K` is stuck (unreachable) |
| Negative `remaining` (`advance` checks `== 0`, not `<= 0`) | Would keep stepping | K is stuck (unreachable; `initProgram` requires fuel ≥ 0) |

The README declares all of this out of domain, and that is acceptable for this candidate. Any future differential or generated-program harness must:
- admit only closed, acyclic, well-formed tables;
- classify a Quint runtime error as a harness failure, never as a pass or a rejection.

**F3 (profile-name collision risk).** The model declares `PROFILE = "sherlock-k-a/1"`, which is correct for its clock. However, `docs/SHERLOCK-CORRECTNESS-LOOP.md` and the `reference_types.rs` doc comment describe "Quint" as `sherlock-quint-a/1`, where terminal return is charged. Consumers must not mix this model's raw steps with the scalar15 model's. The K-PROFILE-README is explicit about this. A one-line note in the correctness loop doc would be a useful follow-up.

**F4 (stale text).**
- The header comment in `models/sherlock/quint/kProfileMachine.qnt` (line 5) still says "source-only until its own ordinary model checks are captured." This understates the evidence rather than overstating it. Do not edit it in place, because that would change `c62c…` and invalidate the receipts.
- In `AGENTS.md`, the "Temporary security routing — October 15" section and the October 14 paragraph ("temporarily superseded by the … Argon assignment") remain worded as if active. The new top section does state that the substitution has ended. I recommend relabelling those paragraphs as historical.
- `docs/SHERLOCK-CORRECTNESS-LOOP.md` ("Current gaps…" paragraph) still names the AGY Argon assignment as current. It is not in this candidate's membership, so this is a follow-up only.

**F5 (receipt binding gaps).**
- `docs/audits/2026-10-15/sherlock-k-profile-quint/quint-normal-red-trace.json` has no model, test, stdout or stderr hashes. The `stubStep(` call in its stdout ties it to the stub by content only. The hash-bound RED receipt (`quint-normal-red.json`) carries `ef41…`/`985069…`, but its stdout was not reproduced in the packet.
- `quint-root-diagnosis.json` has no source hashes or elapsed time. It was a setup diagnosis only.
- No receipt records a Quint package or `cli.js` digest, or version output. "0.32.0" is asserted in prose only.

I recommend adding these bindings in future receipts. GREEN, which is the result being approved, is fully bound.

**F6 (claim I cannot check).** I cannot verify from the packet the claim that reversing `BoolValue(`→`Bool(` reproduces the earlier test source `c8a7…` byte for byte. The rename is semantically neutral by inspection, and it happened before the behavioral RED run.

**Routing restoration.** `raw/assignments/moriarty-opus-security-restored-2026-10-15.md` and `IDENTITY.json` record an observed, completed Opus response with its returned canonical identity. This satisfies the "observe availability, don't infer it" condition in the Argon record. It preserves Argon's historical identities and changes no native authority, resource limit or acceptance gate. I can't independently verify the receipt. My own run is consistent with Opus being available.

**Documentation scope.** The K-PROFILE-README, the audit README and `SOURCE-BINDING.json` keep the claim scope narrow, consistent with the evidence:
- finite normal observations only
- the scalar15 model unchanged
- no K, Clean, native, stagenet, security or correspondence claims
- reviews still pending

I found no overclaim.

**Dissent preserved.** I have no disagreement with the author on semantics. My only reservations are F1–F6, which I raise as non-blocking.

END_K_QUINT_REVIEW
