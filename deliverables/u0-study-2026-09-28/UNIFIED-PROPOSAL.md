# U0 unified proposal: freeze the contract, demonstrate one stage, qualify the v3 target

**Date:** 2026-09-28
**Status:** specified-only. This is a proposal, not an accepted change. Nothing in it closes a U0 predicate. Under the repository rule, work is accepted by evidence, not by review, and advisor agreement does not complete a capability.

**Sources.** Nine independent read-only reviews, three per lens, one from each model family. Each proposal is in this directory.

| Lens | Claude Opus 5.5 | GPT-6 Sol (xhigh) | Grok 4.7 (xhigh) |
|---|---|---|---|
| L1 semantics | [opus55-L1](opus55-L1.md) | [gpt6sol-L1](gpt6sol-L1.md) | [grok47-L1](grok47-L1.md) |
| L2 target and enforcement | [opus55-L2](opus55-L2.md) | [gpt6sol-L2](gpt6sol-L2.md) | [grok47-L2](grok47-L2.md) |
| L3 numeric profile and exit path | [opus55-L3](opus55-L3.md) | [gpt6sol-L3](gpt6sol-L3.md) | [grok47-L3](grok47-L3.md) |

**Owner correction applied after the reviews.** ZKIRv3 is the target. Its formal specification is merged into the `zkir-v3` branch of `midnightntwrk/midnight-zkir`, through [PR #17](https://github.com/midnightntwrk/midnight-zkir/pull/17). That PR adds an Agda mechanization covering 13 types and 34 instructions, with P5 faithfulness and statement-soundness theorems. It merged on 2026-09-23 as commit `0303e6d`, with head `ebb662c`. The ZKIR v3 integration is the next work. Opus L2 had proposed amending U2 to "the released ZKIR major", which is v2. That option is withdrawn. The v2 path (compactc 0.31.1, ledger-v8 8.1.0) is historical evidence only, not a target.

---

## 1. What U0 must deliver

All nine reviewers agree that U0 is a **contract milestone with one small reference demonstration**. It is not a native-enforcement milestone. U1 and U2 need three things from it:

1. **A frozen, typed stage relation.** Every leaf has a stated origin: source, Core, toolchain, ledger, signed intent, or evidence envelope. Every leaf also has a disposition and an owner milestone. The six judgments are written as executable clauses, not prose.
2. **A named first slice, demonstrated at reference level.** One small signed financial stage. Its relation is produced by running code, and its judgments are evaluated in TypeScript and K over valid and hostile inputs.
3. **A qualified target record.** This names the ZKIRv3 integration target by commit. It keeps the historical v2 path and the native-recursion (IVC) path as separate, named records. It states which open premise blocks which milestone.

## 2. Consensus findings

These points were reached independently. The counts show how many of the nine reviewers made each point.

| # | Finding | Reviewers |
|---|---|---|
| F1 | **The gate should split.** "Contract frozen" should be separate from "capability demonstrated". Requiring 84 of 84 natively enforced leaves, or 24 demonstrated backend rows, would pull U2–U4 work into U0. | 9/9 |
| F2 | **The Aeon `repay` finding is a contract gap, not an evaluator defect.** The written source guards (`loan-lifecycle.mori:119-121`) do not carry the safety argument. The Core evaluator and the K semantics do carry it: `financial-lifecycle.ts:1568` (`EXCEEDS_OUTSTANDING`), `:1459` (`INSUFFICIENT_BALANCE`), `:1426`/`:1433` (`ALLOCATION_COMPONENT`), and `lifecycle-kernel.k:52`. The laws belong in the U0 contract. Refinement types, SMT discharge, linear types and the `/6` grammar do not. | 9/9 (L2 reviewers on the enforcement side only) |
| F3 | **"66 absent embeddings" is the wrong measure.** It counts 12 evidence-envelope leaves (`judgments.*.status/enforcementRef`) and about 12 toolchain or ledger leaves as missing language. Source/Core "embedding" is currently name matching, not computation. | L1 3/3, L3 2/3 |
| F4 | **The schema admits financially invalid instances.** It allows negative amounts, one-account gross lines, liabilities without principal, accrued and outstanding, authority as opaque strings, and a judgment status of `unchecked` in an accepted stage. The aeon −1 state is schema-valid. | L1 3/3 |
| F5 | **The six judgments are prose.** None states conservation, a cap inequality, non-negativity, liability roll-forward or replay freshness. | L1 3/3, L3 3/3 |
| F6 | **K "0 covered" is structural.** 17 of the 23 rows are roadmap-wide UNI requirements owned by U3–U7. "Covered" has no objective definition. The 2026-09-17 K-versus-TS lifecycle run is the strongest semantic evidence available and should be built on. | L1 3/3 |
| F7 | **The target pins are not one tuple.** They combine the historical released path with native-recursion development objects, chosen by tie-break rather than by joint execution. They record no IR major/minor, have no per-circuit key manifest, and leave the Blake2b-versus-Poseidon transcript split unresolved. | L2 3/3 |
| F8 | **The enforcement map is honest but has no priority.** All 84 leaves read `NOT_ENFORCED`. There is no locus, tier or owner, and no signed-intent authentication decision, only an absence finding. | L2 3/3 |
| F9 | **Premises and backend rows lack blocking data.** No premise says what it blocks. MNR01–MNR08 have `owner: null`. ZR01 is labelled U0/U1, but no U0 task exercises it. | L2 3/3 |
| F10 | **The numeric "reserve" rule D2 is under-specified.** `prorata-principal-share` conserves value (`dA = n − dP`) and is misclassified as a reserve gap. Ceil accrual adds to a liability. Sub-quantum remainders cannot be posted to an integer reserve. "Unit dimensions" are implemented in the type checker but recorded only as widths. | L3 3/3 |
| F11 | **The first slice is unnamed.** ROADMAP.md:38 says "connect the smallest useful signed financial stage" but never names it, so U1's "every primitive needed by the initial slice" has no denominator. | L3 3/3, L1 3/3 |

## 3. The unified U0 gate

The current `EXIT-GATE.md` is replaced by three separately reported states:

| State | Meaning | Requires | Unblocks |
|---|---|---|---|
| **U0-F** contract frozen | Relation, judgments, numeric profile, dispositions and premises are versioned and hash-bound. | Workstreams S1–S3, S6, N1–N2, T1–T2, T5–T7, G1 | U1 and U2 design work |
| **U0-S** slice demonstrated (reference) | Slice S0 is projected by executed code. Its judgments are evaluated in TypeScript and K, which agree on golden and hostile corpora. The repay counterexample is replayed and rejected. | S4–S5, N3–N4 | U1 host-level certification of S0 primitives |
| **U0-T** target qualified | A tuple receipt exists on the ZKIRv3 integration target, with ZR01 negative controls. The identity cohort is carried in public inputs. | T3–T4 | U1 native certificates, U2 native work |

The gate checker prints each state independently. **"U0 done" means U0-F and U0-S both hold, and U0-T either holds or is recorded as `blocked on upstream v3 integration` with a named dependency.**

This resolves the one sequencing disagreement. Opus L2 wanted no U1 work before U0-T. The others allowed host-level work first. U1's **host** certificates may start after U0-F plus U0-S. U1's **native** certificates may not start before U0-T. Feasibility measured against an unpinned target is not evidence.

## 4. The first slice: S0

Six different candidate slices were proposed. They overlap heavily and differ only in how much to include:

- Opus L1: a transfer plus a repay.
- Sol L1: a transfer with a fee, then a repay.
- Grok L1: R0, a transfer that funds a repay.
- Opus L3: a payment or swap with no division.
- Sol L3: a principal-only repay at exact 1:1.
- Grok L3: N0, exact, with AccrualFirst.

**S0 is the intersection that every reviewer's constraints admit:**

- One signer, one domain, one settlement asset, one admitted obligation.
- **Program A:** a funded **repay** of an existing obligation. It uses AccrualFirst allocation and exact settlement conversion (mantissa 1, scale 0, `rounding: none`).
- **Program B**, the structurally contrasting program U2 needs: a plain **transfer** to a named recipient with a **literal** fee line. A literal fee needs no rounding. A computed fee is outside S0.
- Atomic rejection with an empty retained-effects set. Empty supply changes, observations and disclosures, each bound as an explicit empty set that rejects any non-empty value. Unit policies: `delegation=none`, `recovery=none`, `outcome=terminal`.
- **Excluded:** accrual, ProRata, price or division, reserve posting, partial phases, non-ledger-induced history.
- **Arithmetic used:** checked add and subtract on u128, construction of checked amounts, comparison, and the AccrualFirst `min`/subtract. That is no open numeric gap. U1's first certified primitive is `checked-add-u128`.
- **Limits** are integer comparisons. When U3 needs ratios (11 A / 1 A / 20 B), it writes them as cross-multiplied UInt256 inequalities, not divisions. S0 is a strict prefix of the U3 discriminator.

The S0 laws, merged from Opus L1 (E1–F1), Grok L3 (L1–L8) and Grok L1 (R0), are:

| Id | Law |
|---|---|
| L1 | `outstanding = principal + accrued` |
| L2 | principal, accrued and outstanding are all ≥ 0 |
| L3 | `0 < nominal ≤ outstanding` |
| L4 | settlement ≤ payer balance, and settlement ≤ remaining allowance |
| L5 | every counter and every credit fits its unsigned width (checked add, or a supply bound) |
| L6 | settlement = nominal (exact conversion) |
| L7 | AccrualFirst: `dA = min(nominal, accrued)`, `dP = nominal − dA`, and each part fits its component |
| E1 | per-asset conservation: Σ gross = Σ supply changes = 0 |
| E2 | net is derived from gross and fees |
| I1–I5 | gross debit ≤ `grossDebitCap`; fees ≤ `feeCap`; recipient net ≥ `minNetOutcome`; recipients and assets ⊆ the signed sets; stage time falls within the validity window |
| A1, A2 | authority remaining = previous remaining − consumed, and ≥ 0; replay freshness |
| S1, H1 | source and Core hashes are bound; predecessor commitment = commitment(pre-state) |
| F1 | on rejection: post-state, allowances and replay IDs are unchanged, and retained fees ≤ `feeCap` |

## 5. Work plan

Size: S = small, M = medium, L = large. Every task writes to a new versioned directory, `deliverables/u0-semantic-contract-v2/`. The 2026-09-23 files stay unchanged because their hashes are bound.

### Semantics (S)

- **S1. Leaf dispositions (S).** One file, `leaf-dispositions.json`, covering all 84 leaves. For each leaf it records:
  - `layer`: source, core, toolchain, ledger, signed-intent or evidence-envelope;
  - separate `sourceStatus` and `coreStatus`;
  - the S0 disposition: dynamic, fixed, empty-with-rejection, or deferred with an owner;
  - the enforcement `requiredLocus` and `tier` (see T5).

  Checker: every leaf appears exactly once, and the judgment `schemaFields` partition the 72 non-evidence leaves. *Source: all L1; Opus L2 T4; Grok L2 T5.*
- **S2. Stage-relation schema v2 (M).** Changes:
  - Amounts are canonical non-negative integers, bounded by width, with an asset and unit reference.
  - Gross lines are `{asset, from, to, amount}`.
  - Liabilities carry `principal`, `accrued` and `outstanding`, with `accrual[]` and `discharge[]`.
  - Authority is per-asset vectors plus a replay-ID set.
  - Policies are tagged unions.
  - `judgments.*` moves to an evidence envelope. In an accepted stage, `unchecked` fails closed.

  Exit: a negative corpus fails the **schema**. It includes principal −1, a one-account gross line, an unbound unit, and a required judgment left `unchecked`. *Source: all L1.*
- **S3. Executable judgments v2 (M).** Each judgment is a list of clauses with `holdsWhen`, `violatedWhen` and `uncheckedWhen`, plus an owner for anything left unchecked. Clauses are written in a small fixed predicate language (linear integer arithmetic and set membership over relation paths), using the §4 laws. Each clause links to the numeric-profile primitives it uses. Checker: every clause parses and every path resolves. *Source: all L1; Grok L3 T4.*
- **S4. Core-to-relation projection (M/L).** Add `experiments/moriarty-language/src/successor/stage-relation.ts`, a total function that calls `prepareFinancialLifecycle` rather than reimplementing it. Golden fixtures cover programs A and B. Hostile fixtures cover:
  - nominal above outstanding;
  - borrower overdraft;
  - fee above its cap;
  - a replayed intent ID;
  - a wrong recipient or domain;
  - a stale validity window;
  - an obligation whose components don't sum;
  - a missing signer.

  Exit: every S0 dynamic leaf is emitted by executed code, and "present" is redefined as "emitted by the projection under a cited law". *Source: all L1.*
- **S5. TypeScript and K differential on S0 (L).** Evaluate each clause in K, reusing `lifecycle-data.k`, and in a TypeScript twin, over the S4 corpora. A row is `covered` only with an executed golden-and-negative trace and full agreement, and only at S0 scope. The trace must stay inside UInt128 and include a near-bound case, because `lcRepay` uses unbounded `Int` (Grok L1). Every UNI row keeps its entry with `ownerMilestone` and `scopeQualifier`, and none is relabelled covered. *Source: all L1.*
- **S6. Signed-intent abstract syntax (M).** A specification only: a Core `SignedIntent` type and a source `intent` EBNF fragment. Its profile ID must be reserved against Aeon's `/6`. *Source: Opus L1 T8.*

### Numeric (N)

- **N1. Numeric decision v2: owner decision (S).** Adds to `docs/decisions/u0-numeric-profile-decision-v2.md`; the hash-bound v1 is left unchanged.
  - **D2a remainder classes** (Grok L3):
    - `none`, for exact results;
    - `conserved-split`, for ProRata, where no posting is allowed;
    - `charged-increment`, for ceil on a liability: the extra unit is the debt, with no separate posting;
    - `sub-unit-residual`: not value, not posted, and not rounded again;
    - `protocol-reserve`, only for a whole atomic unit of an identified asset that would otherwise leave the conservation equation.
  - **D5**: liability components are non-negative, and opening + accrual − discharge = closing.
  - **Fee-cap treatment**: does a charged-minus-credited spread count against `feeCap` and `grossDebitCap`? Opus L3 recommends yes. This must be decided before U3.
- **N2. Profile v2 (M).**
  - Add a `dimensions[]` table checked against `numericFits`, `unitDomain` and `arithmeticType`. Admitted price scale is 0..18. The checker's 77 is only a UInt256 bound.
  - Reclassify `prorata-principal-share` as `conserved-split`/conforms.
  - Add rows for `exact-conversion-none` (`financial-lifecycle.ts:1378-1382`) and `accrual-first-split` (`:1405-1407`), which the loan example actually uses.
  - Record that one field, `Conversion.rounding`, is required to be both floor (origination) and ceil (repayment) at `convertNominal` (`:1376`).
  - Add a signedness column that flags SInt128 liability fields against D5.
  - Fix the checker so that declaring a reserve can't close gaps on its own (see G1).

  *Source: all L3.*
- **N3. Executed numeric vectors (M).** `vectors.json` holds independently computed expected values, including boundary cases. `observed.json` holds what the TypeScript evaluator actually produced. This turns recorded gaps into demonstrated gaps and closes nothing. *Source: Opus L3 T3; Sol L3 T1.*
- **N4. Repay precondition lift and evaluator replay (S/M).**
  - L1–L7 become judgment clauses (S3).
  - Keep a replay of the Aeon witness through the callable evaluator. Use both the archived model and the smaller witness (principal 2, accrued 0, nominal 3, borrower balance 2). The expected result is `EXCEEDS_OUTSTANDING`.
  - Keep the z3 transcripts labelled `source-gap`.
  - Correct the Aeon README wording. The archived model sets principal to −1. Borrower balance at −1 is a different goal, reached by a smaller witness (Grok L3).

  *Source: all L3; Sol L1 T3.*

### Target (T)

- **T1. Target record v2 with separate profiles (S).**
  - **Profile V3, the target:** midnight-zkir `zkir-v3` at `0303e6d` or later, the Agda mechanization as its formal reference, and the compiler, ledger and proof-server integration commits as they land.
  - **Profile H, historical:** compactc 0.31.1, proof-server `8.1.0@sha256:801bbc03…`, `ledger-v8@8.1.0`, ZKIR v2. This is evidence only.
  - **Profile IVC, native recursion:** midnight-zk `695351f1` with the k=17 SRS. Status `blocked`.
  - Each profile records: IR major/minor read from the output bytes, key format versions, a per-circuit key manifest, SRS degree, transcript name, and the verifying binary's `proof-verifying` flag.
  - A presence receipt rehashes each artifact locally, or records it as absent. Grok L2 noted that the proof-server image and the `midnight` checkout are already absent.

  *Source: all L2; the owner's v3 correction.*
- **T2. Make a verified tuple representable (M).** Today the checkers make success impossible:
  - The schema sets the tuple flag with `const: false`.
  - The checker bans words such as "verified" and "compatible".
  - The OK and "capabilities open" lines are hard-coded.
  - G6 requires the flag to stay false.

  Replace this with per-profile states `unverified`, `blocked` or `reverified`. `reverified` is legal only when a receipt exists and its hashes match. *Source: Grok L2 T2, single reviewer. The cited lines were confirmed during synthesis (§10).*
- **T3. Tuple receipt on V3 with ZR01 negative controls (L).**
  - Compile a minimal circuit, then an S0 kernel, with the V3-integrated compiler. Read the IR header from the bytes and prove.
  - Verify in a separately installed verifier, then on a local ledger with proof verification enabled.
  - Negative controls, each of which must reject after its positive control succeeds: rewrite the IR header, substitute another circuit's VK, use an SRS with a different k, tamper one public input, flip one proof byte.
  - Never submit to Preview.
  - **Dependency:** an upstream v3 compiler/ledger integration you can install. Until one exists, U0-T is `blocked` and names that dependency.
  - **Owner decision:** whether to run the same harness now on profile H. Grok L2 argues it validates the negative-control machinery and closes CLM-0958 ("proved, not verified"; k=14 was the only Moriarty prove). It would be labelled harness validation and would carry no target claim.

  *Source: all L2.*
- **T4. Proof/ledger seam and the IVC record (M).** Keep a decision recording that a contract-call proof is not an IVC certificate, and how the Blake2b-versus-Poseidon transcript split is bridged or blocked. Test it with a real proof, including malformed and trailing bytes. Do not rerun k=17 synthesis or raise k. *Source: Sol L2 T2; Grok L2 T4.*
- **T5. Enforcement locus and tiers (M).** The loci are circuit, ledger-intrinsic, contract-state, host-advisory and trust-premise. The tiers, merged from Opus L2 and Grok L2:
  - **Identity cohort, demonstrable in U0-T:** `circuitIdentity.{circuitId,compilerPin,verifierKeyId,zkirVersion}` and `profiles.numericProfile`, carried in the T3 public inputs. A substituted VK or compiler pin must reject.
  - **U2-slice cohort, about 22 leaves:** the S0 `signedIntent.*` fields, `effects.{gross,net,fees}`, `domain.*`, `programIdentity.{programId,entryPoint}`, `authority.replayState`, `failurePolicy.*`.
  - **Everything else:** owners U3, U4 or U6.
  - The 12 `judgments.*` leaves are `not-a-constraint`.

  Status stays `NOT_ENFORCED` until a negative control passes. A new status, `LEDGER_INTRINSIC_DEMONSTRATED`, is legal only with a receipt reference. It also requires showing that the value the ledger checked is the Moriarty field. For example, verifying against the VK at an address binds the circuit only if that address is bound to `programIdentity`.
- **T6. Signed-intent boundary decision, TP07 (S): owner decision.**
  - **Requirement (Sol L2):** a signature is acceptable only if its verified message is the same proved statement that produces the accepted effects.
  - **Proposed decision:** the canonical intent digest is a public input of the stage circuit and is authenticated **in-circuit**. ZKIR v3's type surface (Jubjub, secp256k1, secp256r1, Curve25519 in PR #17) makes in-circuit verification expressible. That answers Opus L2's "unverified primitives" concern at the IR level. U1 owns measuring its cost.
  - **Fallback (Grok L2):** a circuit commitment to the intent bytes, with a ledger-checked signature over that commitment. It is admissible only if a negative control shows the binding holds.
  - Unshielded spend signatures and `evaluate.ts:290-295` remain non-binding.
- **T7. Premise blocking graph and backend ownership (S).**
  - Add `blocks[]`, `owner`, `closeCondition` and `reviewBy` to trust premises:
    - TP01 and TP09 block native certification in U1 and U2.
    - TP07 blocks U2.
    - TP03 blocks U3, or U2 only if a program claims an observation.
    - TP06 and TP08 block U4.
    - TP02 blocks nothing before U4 and gets a tripwire: `reviewBy 2026-12-15`, when U4 is re-planned if no public recursion-capable release exists.
  - Open premises must never be relabelled `accepted-assumption`.
  - Backend matrix owners: MNR01 and MNR05 go to U0/U1, next to ZR01. MNR02, 03, 07 and 08 go to U4. MNR04 splits between U2 and U4.

  *Source: all L2.*

### Gate (G)

- **G1. Rewrite `check_u0_exit_gate.py` and `EXIT-GATE.md` (M).**
  - Report U0-F, U0-S and U0-T as three separate results.
  - Remove the G6 requirement that the tuple flag stay false, and the close rule that requires every leaf enforced.
  - Replace the numeric close rule. Today it closes when the gap count is 0 and a reserve is "present" (`:228`), so declaring an unused reserve type would close all six gaps (Grok L3). The new rule: S0 frozen, dimensions present, every rounding row classified, and the probe classified as a source gap.
  - Drop the constant "Evidence RECORDED: yes" column.
  - Tests must show that a structural pass cannot print "target qualified" when proof bytes, verifier configuration or a negative control are missing. *Source: Grok L2 T7, Grok L3 §4, Sol L2 T5.*

## 6. Sequencing

1. **Phase A, decisions (owner, can start now):**
   - N1: remainder classes, liability domain, fee-cap treatment
   - T6: intent boundary
   - approving S0 as the first slice
   - the T3 harness-on-H option
   - the Aeon disposition (§8)
2. **Phase B, contract freeze (parallel, mostly S/M):** S1, S2, S3, S6, N2, T1, T2, T5, T7, G1, ending at **U0-F**.
3. **Phase C, reference demonstration:** S4, then S5; N3 and N4 in parallel; ending at **U0-S**. U1 host certification of the S0 primitives starts here.
4. **Phase D, target:** T3 and T4, when upstream v3 integration can be installed. That gives **U0-T**, and then U1 native certificates and U2 native work.

Keep one delivery item in progress at a time, as the develop workflow asks. The critical path is S2 → S3 → S4 → S5. N1 and T6 are the decisions that most often block downstream work.

## 7. Disagreements and how they were resolved

| Topic | Positions | Resolution |
|---|---|---|
| U2 target if v3 is not released | Opus L2 offered "amend U2 to the released major". Sol L2 and Grok L2 assumed v3. | **Resolved by owner:** v3 is the target. U0-T is `blocked` until the integration lands. There is no downgrade. |
| Where to prove the harness | Grok L2: on H now (compactc 0.31.1, ledger-v8). Opus L2 and Sol L2: on the target. | Target receipt on V3. Harness on H is an optional owner decision, labelled non-target (§5 T3). |
| May U1 start before U0-T? | Opus L2: no. Sol L3 and Grok L3: host work may. | Host certificates after U0-F plus U0-S. Native certificates only after U0-T. |
| `host-only` status for kernel checks | Opus L3: record the repay preconditions as host-only. Grok L2: never mark `signedIntent.*` host-only, because the host checks a different object. | `host-only` is allowed only for lifecycle-law leaves where the S4 projection's hostile fixtures show the cited kernel code checks that exact projected field. It is never allowed for `signedIntent.*`. |
| Fees in the first slice | Grok L1: empty fees. Opus L1 and Sol L1: an explicit fee. | Program B carries a **literal** fee line: it exercises `feeCap` with no rounding. |
| How many numeric gaps | EXIT-GATE says 6. Opus L3 and Sol L3 say 5, with ProRata misclassified. Grok L3 says 6 reclassified into classes, plus two uninventoried rows and a one-field conflict. | Adopt Grok L3's classes (N1/N2). The count follows the classification. None of it blocks S0. |
| Where the Aeon laws live | All nine: not `/6` in U0. Grok L3 cites the delta's SP01.3 bullet. Opus L3 calls the delta stale because it targets superseded sprint structure. | Laws go into U0 through S3 and N4. The package is re-scoped (§8). |
| Signature mechanism | Opus L2: in-circuit (proof of key ownership, or signature verification). Grok L2: commitment plus ledger signature, with no Ed25519 in circuit. | In-circuit is primary, now expressible in v3. Grok L2's route is a fallback, allowed only if a negative control shows the binding (T6). |

## 8. Aeon package disposition

The uncommitted package at `openspec/changes/aeon-refinement-integration/` and `.aeon-staging/` is recommended for re-scoping, not adoption in U0:

- **Take into U0:** laws L1–L7, the classified z3 transcripts, and the evaluator replay (N4).
- **Keep for U0–U2:** its advisory obligation analyser. This is the roadmap's "obligation and trust reports" line (ROADMAP.md:46).
- **Defer until after U2:** the `/6` grammar, SMT discharge, linear and affine obligations, and refinement-directed lowering. They are gated on a supported encoding and on counterexamples replayed through the evaluator.
- **Reject:** `ROADMAP-DELTA.md` §4–§5 edits to the SP01–SP12 sprint files. Those are provenance aliases now (ROADMAP.md:5).
- **Correct:** "drives principal and borrower balance to −1" becomes "the written source guards omit load-bearing preconditions that the kernel enforces".
- **Owner decides:** whether to commit it under re-scoped text.

## 9. Things not to do

- Do not grow the name-matching embedding table, or add 60+ fields to source/5, to raise the counts.
- Do not treat a checker exit of 0, a cited identifier, or a matching fixture as coverage or enforcement.
- Do not set any tuple to `reverified` from hashes found in documents. Only a receipt with negative controls counts.
- Do not pair zkir-v3 commits with the ledger-8 proof server, or call the v2 path ZKIRv3.
- Do not declare a reserve type, or implement reserve posting, before N1. A declaration would falsely close every rounding gap today.
- Do not edit decision v1, `spec/examples/loan.mori` (its bytes are bound to Preview evidence), `financial-lifecycle.ts` (decision line 22), or the 2026-09-23 receipts.
- Do not submit to Preview, raise k above 17, rerun R3 synthesis, or let the March 2027 assumption gate U0–U3.

## 10. Single-reviewer findings: verify before acting

These were raised by one reviewer only. They are specific and cite sources, but no second reviewer confirmed them:

- **Grok L2:** the U0 checkers make a verified tuple unrepresentable (T2). **Checked during synthesis:** `check_u0_target_pins.py:40` allows only the statuses `historical` and `absent`. `:62-64` bans "verified", "compatible" and "current" in prose. `:147-149` always prints "NOT established". `:243-244` fails if the flag is not false. `check_u0_exit_gate.py:193-194` (G6) fails if the flag is not false. `:228` is the numeric close rule, gap count 0 and reserve `present`. Grok L3's claim that a reserve declaration alone would clear the gaps depends on `conformance_reasons` in `check_u0_numeric_profile.py`, which was not checked here.
- **Grok L2:** the only Moriarty prove used k=14 and was not verified (CLM-0958). The k=17 SRS comes from an unapproved IVC preparation. The pinned `accrue` key never produced a transaction proof.
- **Grok L3:** `Conversion.rounding` is shared by origination (floor) and repayment (ceil) at `convertNominal`. `exact-conversion-none` and AccrualFirst are missing from the 17 rows. Declaring a reserve would close every gap.
- **Grok L1:** K `lcRepay` uses unbounded `Int`, so correspondence needs a stated UInt128 bound.
- **Opus L3:** sub-quantum remainders cannot be posted. The rounding spread against `feeCap` could make U3's "at most 1 A fee" tight.
- **Opus L2:** the enforcement search roots exclude the ledger itself, so ledger-intrinsic checks were never considered. That is a candidate locus only.

## 11. Evidence limits of this study

- **Evaluator rejection not executed.** No reviewer ran the callable evaluator against the repay counterexample. Opus L3's Node run failed because the host `/tmp` was full (ENOSPC). The evaluator's rejection is read from code (`financial-lifecycle.ts`, `repayment.ts`) and test sources. It is not demonstrated, and N4 exists to demonstrate it.
- **z3 re-runs.** Opus L3 and Grok L3 independently re-ran the Aeon z3 probes and reproduced the recorded results for the written source guards.
- **Checkers and tests.** Every reviewer re-ran the U0 checkers, which exited 0 with the counts in EXIT-GATE.md. Opus L2 saw 7 `test_u0_exit_gate.py` failures, all ENOSPC while copying `.git` into `/tmp`, so that suite's status is **unknown** for this run.
- **No guarded status.** The guarded `develop` CLI `status` was not run: Sol L3 found its SQLite store unwritable in a read-only sandbox, and this session's Bash was down because `/tmp` was full.
- **Upstream state.** PR #17 facts come from its GitHub page as fetched on 2026-09-28. Whether a compiler, ledger and proof server emit and accept v3 end to end was not checked.
