# REPORT: independent source review of the RR-03 owner-constraints candidate

## Verdict: SOURCE_ACCEPTABLE_FOR_LOCAL_PREPARATION (qualified)

The materialized candidate implements the voted owner-constraints profile in Rust and the
Cardano domain `/3` in Aiken. I found no fail-open defect. Unsigned premises are not trusted, and
no financial, authority, frame or residual-duty obligation is relaxed. `/2` behaviour is
preserved in source. No source finding blocks local preparation runs (lock preparation, `cargo
test`, `aiken check`).

The verdict is qualified because two parts of the predicate are not fully met:

- **Voted text.** One S2 behaviour, DV-3, falls outside the literal voted R7 text. It can only
  cause refusals, never admissions (IR-01). The root must record it.
- **Complete expectations.** Every one of the 82 frozen rows is mapped. Some outcome fields are
  not asserted (IR-02, IR-03), and every Cardano refusal that rests on signatures is unobserved
  (IR-04). These fields cannot count as observed in an acceptance review until a test-only
  supplement or E-3 covers them.

This is not adoption and not acceptance. RR-03 stays High and open.

| Axis | Result |
|---|---|
| Implementation fidelity | Faithful to MR-3 + MR3-R + A1–A3 + MR3-R5 + A4 + A5, except DV-3 (IR-01, refuse-only) |
| Security (trust, authority, arithmetic) | No fail-open path found; A4 cap implemented as an exact checked sum (OS-01 confirmed) |
| `/2` preservation | Preserved at source level; the 146 historical CEK results must be re-observed on the new `thread.ak` |
| Frozen oracle (82 rows) | All mapped. Core layer: 70 EXACT, 9 PARTIAL_MINOR, 2 PARTIAL, 1 INSTANCE (`ORACLE-COVERAGE.json`) |
| Cardano layer | Gate-level only for positives; signature-dependent refusals unobserved (IR-04) |
| Compile/API | No predictable failure found; uncertain points have prepared fallbacks (IR-07) |
| Execution | No authority. E-1 needs a reviewed lock first (IR-05); see `EXECUTION-ELIGIBILITY.json` |

**Blocking findings for local preparation:** none.
**Blocking for adoption or acceptance:** IR-01, IR-02, IR-03, IR-04, IR-06.
**Next action:** The root should:
1. Record decisions on DV-3 (the MR3-R7c text or a catalogue-completeness precondition) and on
   the SettleV3 ABI choice (RR03-ABI-1).
2. Prepare the reviewed offline lock and the A-1/R-1 isolation conditions, then run E-1 and E-2.
3. In parallel, decide whether to commission the test-only supplement in
   `proposals/MINIMAL-REPAIRS.md` §1. It would need a fresh audit of the changed files only.
4. Hand the actual results to a fresh E-4 review that grades with `ORACLE-COVERAGE.json`.

## 1. Identity, scope, startup

- **Reviewer.** `claude-opus-5-5` (runtime-stated). Requested effort is high; I cannot observe the
  actual effort. This is a fresh context. I am not V12, V13 or V14. Every author and voter in
  this lineage is the same model family, so independence is context-level only.
- **Tools.** Read, Write and Edit only. Nothing was listed, searched, hashed, parsed, compiled,
  executed, signed or proved. I opened no provider, stderr, prompt, wrapper, credential, key or
  wallet file and no live W0/W1 or R1 source-effect draft. Writes went only to this folder.
- **Startup.** I read the installed `develop` skill and `execution-focus.md`. From the
  full-integration checkout I read `AGENTS.md`, `MORIARTY-PRODUCT-CONTRACT.md` and
  `SECURITY-ROUTING.md`. The root refreshed the guarded CLI; old SP01 is unrelated. All security
  judgments here are Opus 5.5 judgments. The language stays permissionless: the A5 key map is the
  program instance's account authority, not an admission list.
- **Author identity.** `TERMINAL-RECOVERY.json` records:
  - requested `claude-opus-5-5`/high;
  - returned canonical `claude-opus-5-5`, firstParty;
  - provider terminal `success`, `is_error` false.

  The original CLI's numeric exit code is UNKNOWN; I do not infer exit 0. This qualifies identity
  only and approves nothing.
- **Candidate.** The root-materialized folder (40 files) is authoritative. I read all 39 non-fixture
  files in full. Of `fixtures_r1.ak` I read only lines 1–30; the rest is treated as frozen opaque
  data known by the root's hash and the historical A1 result. Its decoder (`test_ctx.ak`) and every
  use (`settle_r1_test.ak`) were read in full. I do not claim to have read its full body.

## 2. Votes (MR3-R5, A4, A5) and their qualifications

**The FOR votes.** V14's three FOR votes quote V13's texts word for word and give reasons that I
re-derived from source. These are substantive votes, not rubber stamps (`DESIGN-VOTES.md` §1).

**The qualifications:**

- **A4 versus V13 C5's "(saturating)".** "At most" is exact, and so is the checked sum the
  candidate uses. A saturating sum fails open at a cap of `2^128 − 1`. This is not a semantic
  change to voted text. I concur with OS-01.
- **R7 q1.** The voted text says "the only permitted difference". q1 reads this as limiting
  omissions only, which changes its scope.
  - DV-2 was already listed by V13 itself.
  - DV-3 is new and was voted by nobody. It is refuse-only, but it is semantics outside the voted
    text, so the root must record it (IR-01).
  - My clarifying text MR3-R7c is in `DESIGN-VOTES.md` §4 with one position. Alternatively, the
    root can state catalogue completeness as an S2 precondition.
- **SettleV3 appended (OS-02).** I agree with this choice and proposed text RR03-ABI-1. It differs
  from V12's IMPLEMENTATION-SCOPE wording, which was scope text, not voted text.

## 3. Security and correspondence (Core)

**Consumer trust (A5):**

- `verify_plan` takes `StepFacts`.
- For a `/3` record it rebuilds the expected intent from the facts, program, source hash and
  commitment. Only signer, targets, limits, recipients, bounds and window come from the record.
- For a `/2` record it binds the five facts, the caller target and the account key.
- An absent key is refused.
- Nothing supplies required signers, premises or ok flags.
- **Gap (IR-06, doc only):** only `StepFacts` is named as consumer-authenticated. `program`,
  `source_hash`, `action`, `target`, `scenario`, `commitment`, `at` and `cap` are equally
  unauthenticated by Core. Host simulation never authenticates ledger state.

**Order and cost:**

- *Pre-execution, cost 0:*
  - preparation;
  - record order (strict, at least one record);
  - signature and window;
  - step 1 with the caller target;
  - R6 committed rows existing in the normalized scenario;
  - step 3 (A1).
- *Execution:* unchanged `simulate_metered`.
- *Post-execution, at the execution cost:*
  - R3 coverage;
  - for each `/3` record: R6 against `Execution.pre`, R2 premises (gross debit including
    self-transfers), step 4 as one `check_effects` call, then 4b, 5, 5b, 5c and 6;
  - for each `/2` record: unchanged `check_effects`.

**Copied private bodies** (`CORRESPONDENCE.md` §1):

- `unhex`, `check_key` and the signature block match `intent.rs:92-127, 214-247`.
- The accounting loop matches `intent.rs:286-337`.
- Only error codes differ.
- The normalization copy is partial and non-authoritative.

**Arithmetic (re-derived):**

- R5 floors use a saturating sum, which is exact.
- The A4 cap uses a checked sum, where overflow means exceeded; this is exact.
- OV-2 refuses at `2^128 + 3`.
- OV-2b admits at exactly `2^128 − 1`.
- SX-03 admits at `2^128 + 55` saturated to `2^128 − 1`.
- The neutral A2/A4 values keep MR-3 behaviour.
- Fees and self-transfers enter gross as `check_effects` counts them.
- 5b is evaluated only after step 4 passes for every limit.
- Debtor and creditor sums match the voted texts.
- Every new field is typed u128 in Rust and range-checked in Aiken.

## 4. Cardano S2

**Bindings:**

- S2 rebuilds the `/3` message from the catalogue, the ledger and the spent datum. `pre_hash`
  always comes from the datum and never from the redeemer.
- Step components are separate Booleans. `position` and `exposure` are guarded by step 4.
- The domain is bound as before: profile prefix with domain `/3`, the anchor's own hash, thread,
  predecessor, `seq` and the catalogue key.
- The statement, ShapeV2, C2, claim and VK are unchanged. These files are hash-equal R1 copies;
  `intents_digest` is a free context field.

**What the source tests show:**

- `partial(35)` and `finish()` are admitted at the gate with byte-identical `/3` messages under
  one authorization.
- `partial(20)` is refused with exactly `["intent_effects"]`, the frozen value.
- The frozen custody values (153M → 116M/63M/62M/119M/111M) are asserted.
- Global output allocation, continuation, residual duties, multi-thread checks and RR-16 are
  unchanged.

**What they do not show (IR-04, extends OS-09):**

- No `/3` test reaches full `thread.settle`.
- RP-1..RP-6, RP-8, RP-9, PF-3, PF-4, KY-1 (SX-07), PG-1/2 and TG-9 at Cardano rest on
  signature verification. So do the MX-1 finish refusal and the MX-2 partial refusal.
- The message-binding test varies `seq`, predecessor, thread, anchor and `pre_hash`, but not the
  network.
- Forged-digest vectors are not proofs.

**`/2` preservation:**

- The R1 `Settle` path is the R1 computation verbatim, plus eight components that are True for
  `/2` records.
- Every R1 test asserts `false_parts`, digests or messages, so no R1 expectation changes.
- Expected E-2 membership is 178 tests (146 + 20 + 12).
- Cost and size grow for `/2` steps too (IR-09). R1 S2 was already 30,764 B against an observed
  `maxTxSize` of 16,384 B. No Preprod fit is claimed.
- Overlap with live W0/W1 work is future integration, not a merge here.

## 5. Oracle coverage (details in `ORACLE-COVERAGE.json`)

- **Golden values.** Every frozen expected value in the tests is copied from
  `EXPECTATIONS-RR03.json`. None is produced by the evaluator under review.
- **Layers kept separate.**
  - V13's SX-01..SX-19 are graded as their own layer.
  - OC-1 and OC-2 are applied as separate assertions; the frozen verdicts are untouched.
  - Author rows (OV-2, AS-1..7, DV-3, CO-6, step units) are labelled AUTHOR. I re-derived them and
    agree.
- **Untallied columns** (IB-1/IB-5/LM-2/LM-9 MR3, the SX "no facts" and non-conforming columns)
  are not tested and are not counted as passed.
- **Concrete gaps (IR-02).** These frozen fields are not asserted:

  | Row | Unasserted fields |
  |---|---|
  | P-A89 | Sol-full variant; reserve duty |
  | P-A32 | effect list; state; Sol-full variant; reserve |
  | P-A, P-B | Sol-full per-account debit columns; closeBlocked |
  | H71-R (Cardano) | forged custody and duty values |
  | H1-B | PTS row |
  | H1-C, H1-D | buyer-passing |
  | PC-2 | which premise rows mismatch |
  | N-C, N-31 | first-failing signer identity (unobservable from the error) |

  Cardano coverage is also missing for MX-2, RS-2, RS-3, IB-3, IB-4, LM-10, PF-4, PF-5, RP-4 and
  RP-9, and RP-7 at S2 covers one side only.
- **Lineage corrections (IR-03).**
  - IB-5 at Cardano is DV-1-divergent, not a reproduction.
  - TG-6 at Cardano holds only under DV-3.
  - RP-1's reason code is `binding`, not `signature`; RP-1 is not an OC-1 row.
  - Several costs are author-derived.

## 6. Syntax, API and Cargo

**Syntax and API (IR-07):**

- No predictable compile failure was found.
- The author-flagged points (`list.repeat`, `transaction.placeholder`, `let` record destructuring,
  k256 signing and high-S construction, serde enum tagging, crate path) read as consistent with the
  pinned APIs or with idioms R1 already used.
- Fallbacks are in `proposals/MINIMAL-REPAIRS.md` §2.
- `aiken build` type-checks test-support modules too, so a fixture API failure would also block S2.

**Cargo:**

- The candidate adds no registry package beyond the Core closure (serde, serde_json, k256; same
  features; no dev-dependencies).
- Reuse of the known reviewed registry set is plausible but unverified here.
- `serde_json =1.0.151` must match the lock exactly (IR-05).
- The proposal's `--locked` cannot run without a lock, and the toolchain must be set explicitly.

## 7. Execution eligibility (no authority)

- **E-0:** done by the root.
- **E-L, E-1, E-2, E-2b:** eligible in principle under the stated isolation, lock, toolchain and
  argv conditions.
- **E-1 key handling:** keys are derived in memory from public labels. These are public test keys;
  no wallet or existing key is used.
- **E-3** (signed vectors) is not eligible under this review. It needs a reviewed, scoped
  test-signing plan within the user's existing prototype authorization; no new user question is
  inferred.
- **E-4** (independent actual-result review) is required after any run.
- Native/SDK authority stays zero.

## 8. Retained open obligations

These remain open, and nothing here closes any of them:

- **RR-03 and its design scope:** RR-03 itself (High); O-D-2 (signer display); O-D-5 (staged
  partial completion and recovery); substitution predicates; multi-step plans.
- **Cardano and resources:** RR-07 (size and cost); RR-16; PB-01..PB-12.
- **Families and coverage:**
  - all twenty pattern families;
  - the original 42-gap inventory;
  - ACTUS 277/18/32/DS01–07;
  - DeFi 72/DA01–24 with the held-outs, intents and regressions;
  - general financial obligations.
- **Composition:** recursion/PCD; private handoff; multi-parent composition.
- **Exits:** every U0–U7, M1–M17, MC01–MC08, SP01–SP12 and G01–G24 exit; Preprod.

Two further limits:

- Signed bounds are not a full financial law.
- `discharge == payment` still rests on proof soundness and setup honesty.

This review makes no claim of general program soundness or ledger acceptance.

## 9. Limitations

- Core was read from the currentness checkout. Equality with the frozen-70 pins is unverified
  (IR-12).
- I did not read the reviewed lock or the 38 stdlib files, so API names are judged from knowledge
  of the pinned versions, not checked.
- All arithmetic is by hand.
- Same-family, context-level independence only.

## 10. Files

`REPORT.md`, `FINDINGS.json`, `ORACLE-COVERAGE.json`, `CORRESPONDENCE.md`, `DESIGN-VOTES.md`,
`READ-COVERAGE.json`, `EXECUTION-ELIGIBILITY.json`, `proposals/MINIMAL-REPAIRS.md` and
`OWN-OUTPUT-ALLOWLIST.json` (written last). The root hashes them after the terminal state; this
review is separate from any adoption.
