# U0 proposal: Lens L1, Semantics (Claude Opus 5.5 reviewer)

## 1. Goal restatement

U1 and U2 need a stage relation that can be checked, not just an inventory of field names. U0 must hand them three things:

- **A frozen, typed stage relation.** For each leaf it must say which layer supplies it: source, Core, toolchain, ledger, signed-intent envelope or evidence envelope.
- **Six judgments written as executable predicates.** They cover stage, intent, effect, authority, history and failure. Each is a conjunction of named clauses over one relation instance and its pre-state. U1 certifies the arithmetic those clauses use. U2 maps each clause to a native constraint.
- **A demonstrated reference embedding for the smallest signed financial stage (MSS).** This is a Core-to-relation projection plus a reference evaluator (K and TS) that accepts the valid golden instances and rejects the hostile ones. Every other leaf gets a named owner milestone.

Without the executable judgments, U2's "scope-qualified contract/refinement/transition/history enforcement" (ROADMAP.md:23) has nothing objective to enforce against.

## 2. Findings (L1)

### 2.1 The embedding table is a name inventory, not an embedding

- `source-core-embeddings.json` has 84 rows. Each row records one `realisation` class for a *pair* of source and Core symbols. `classificationRule` counts a leaf as "present" if a declaration in either profile realises it. The only present row, `profiles.semanticProfile`, is source-only (`ProfileDecl.value`), and its note says "Core has no semantic-profile field". So even the one "present" leaf is not embedded in Core.
- Every partial row cites Core only (`sourceSymbol: null`). Source/5 does expose financial readers (`financialRead` in `financial-agreement-source-v5-grammar.ebnf:124-130`: `balance`, `outstanding`, `allowance_spent`, ...), but the method only counts production names. Source realisation through expressions is therefore not recorded at all.
- Several partials are really derivable projections, not missing semantics:
  - `liabilities.opening[]` is the pre-state `LifecycleState.obligations`, and `liabilities.closing[]` is `PreparedLifecycle.post` obligations.
  - `effects.gross[]` is the `TransferAction` list.

  A total projection function would turn about 13 partials into demonstrated present leaves without any language change. The table does not say this, because it asks "does a declaration with this meaning exist?" rather than "can the leaf be computed from the Core result?"

### 2.2 The 66 absent leaves are mis-scoped

The 84 leaves split into two groups:

- **12 evidence-envelope leaves** (`judgments.*.status`, `judgments.*.enforcementRef`). No source or Core language should ever declare these. The checker nonetheless counts them as absent language embeddings.
- **72 semantic leaves.** I verified by script that these are partitioned exactly and disjointly by `judgments.json`: stage 17, intent 14, effect 29, authority 5, history 4, failure 3.

Of the 72, about 12 are toolchain- or deployment-bound and cannot be a Core responsibility:

- `circuitIdentity.*` (4)
- `domain.chainId`, `domain.domainId`
- `programIdentity.sourceRef`, `programIdentity.coreRef`, `programIdentity.programId`, `programIdentity.entryPoint`
- `schemaVersion`
- `profiles.numericProfile`

Content hashes for `sourceRef` and `coreRef` are trivially computable today. The real language gap is smaller and sharper:

| Gap | Leaves |
| --- | --- |
| Signed intent | 12 |
| Disclosures | 2 |
| Lifecycle ids | 3 |
| Observations | 5 |
| Fee, net and supply effects | 9 |
| History commitments | 3 absent + 1 partial |
| Failure policy | 3 |
| Outcome | 2 |
| Resources | 2 |

"66 absent" overstates the gap in one way and hides its shape in another.

### 2.3 The judgments are recorded prose

- Each `judgments.json` definition is a single sentence, for example: "The stage judgment holds when the bound profiles, program, circuit, domain, and lifecycle identity satisfy the program contract."
- `check_u0_stage_schema.py` C9 (lines 361–379) checks only the key order and that each `schemaFields` entry is a leaf. It does not check that the fields are disjoint and cover every leaf (that holds today only by accident), and it has no semantic content at all.
- No judgment states a conservation law, a cap inequality, a non-negativity range, a liability roll-forward or a replay-freshness condition.
- Mapping to the design doc: MORIARTY-CONSOLIDATED-DESIGN.md:48 names four judgments. The six-key split (authority separated out of effect, failure made explicit) is sound and consistent with the design, and I support keeping it.

### 2.4 The schema admits financially invalid instances

`stage-relation.schema.json` is structural only:

- Every amount uses `"pattern": "^-?[0-9]+$"`, so negatives are allowed. No sign or direction convention is stated, and `effects.*[].account` is a single field.
- There is no reference from amounts to units or to the numeric profile.
- `authority.consumed` and `authority.remaining` are untyped strings, not per-asset amounts.
- `validity`, `replayPolicy`, `consentPolicy`, `delegationPolicy`, `recoveryPolicy`, `phasePolicy` and `retainedEffects` are opaque strings.

As a result, the aeon counterexample state (principal and borrower balance at −1) can be serialised as a schema-valid stage relation. The schema should make the aeon class of violation *unrepresentable or judgment-violating*.

### 2.5 The aeon counterexample is a source-contract gap, not a demonstrated Core defect

The Core and K kernels reject the case:

- `lifecycle-kernel.k:52` checks `EXCEEDS_OUTSTANDING` and `INSUFFICIENT_UNALLOCATED`.
- `financial-lifecycle.ts:1461` rejects with `INSUFFICIENT_BALANCE`, and `:1569` with `EXCEEDS_OUTSTANDING`.
- Aeon's own `evidence/verification-discharge/vc-repay-int.smt2:35-36` marks these guards as "[ADDED] kernel" assumptions.

The finding is real and important, but its correct reading for U0 is this: the safety law exists only as kernel rejection codes. It is neither a named judgment clause nor source-expressible, so the enforcement map cannot attribute it to a layer. This is recorded, not demonstrated at the executed Core level; I did not execute the SMT files.

### 2.6 K reconciliation's "0 covered" is structural

- `k-reconciliation.json` has 23 rows: 17 are roadmap-wide `UNI-*` rows and 6 are judgments.
- Most UNI rows are owned by later milestones by construction:
  - UNI-007 conditional settlement (U3)
  - UNI-009 history (U4)
  - UNI-011 federation (U5)
  - UNI-014 libraries (U6)
  - UNI-016 release (U7)

  They cannot be covered in U0.
- The schema's `covered` enum has no objective definition (`k-reconciliation.schema.json:81-87`). The file's limitation text says it plainly: "Coverage is a reviewed claim".
- What is sound: `deliverables/k-lifecycle-execution-2026-09-17/RESULT.md` shows 104 lifecycle cases where K matches TS Core. This is the strongest *demonstrated* semantic artefact in the L1 scope. The judgment layer should be built on it, not on new infrastructure.

### 2.7 What is sound

- The 84-leaf schema is closed (`additionalProperties: false`) and hash-pinned (`stageSchemaSha256`).
- Embedding notes are careful and honest about absence.
- Checkers and tests pass. I ran `check_u0_stage_schema`, `check_u0_k_reconciliation` and `check_u0_exit_gate`, all exit 0, and 32 tests passed. As FOOTGUNS requires, these runs prove structure only.

## 3. Proposed work

All outputs go to a new versioned directory, `deliverables/u0-semantic-contract-v2/`, so the 2026-09-23 record stays immutable. I call the smallest signed financial stage **MSS**: one signer, one or two assets, gross debit to named recipients, an explicit fee line, a single phase, no minting, and either a genesis stage or one ledger-induced predecessor. As its second, contrasting program it uses a repay against an existing obligation. This fits U2's "structurally contrasting program".

**T1. Binding-layer reclassification** (S; no dependencies)
- **Output:** `leaf-binding-layers.json`. For each of the 84 leaves:
  - `layer` ∈ {source, core, toolchain, deployment/ledger, signed-intent-envelope, evidence-envelope}
  - separate `sourceStatus` and `coreStatus` fields
- **Checker:** extend `check_u0_stage_schema.py`:
  - C10: the file covers every leaf exactly once.
  - C11: the judgment `schemaFields` are a disjoint partition of all non-evidence leaves (72 today).
- **Exit:** the checker passes, and the language-gap count is reported per layer.

**T2. MSS profile of the relation** (M; depends on T1)
- **Output:** `stage-relation-profile-mss-1.json`. For each leaf it gives one of:
  - `dynamic`: computed per stage
  - `fixed`: a profile constant, e.g. `delegationPolicy="none"`, `recoveryPolicy="none-declared"`, `outcome.kind="terminal"`
  - `empty-with-rejection`: e.g. `supplyChanges=[]`, where any supply change is rejected
  - `deferred`: with `ownerMilestone` ∈ {U1, U2, U3, U4, U5} and a reason

  Critical-path dynamic leaves:
  - `signedIntent`: intentId, signer, assetIdentities, recipients, grossDebitCap, feeCap, minNetOutcome, validity, replayPolicy
  - `effects.gross`, `effects.fees`, `effects.net`
  - `liabilities.opening` and `liabilities.closing` (repay program)
  - `authority.consumed`, `authority.remaining`, `authority.replayState`
  - `lifecycleIds.*`
  - `programIdentity.*` and `profiles.*`
  - `predecessorCommitments` (one ledger-induced predecessor, labelled scoped per ROADMAP.md:34)
  - `obligationCommitments` (repay)
  - `failurePolicy.phasePolicy="atomic"` with `retainedFees` bound to the actual Midnight fee-on-failure behaviour, which is an L-pins/trust premise to confirm

  Deferred leaves:
  - `resources.*` to U1: feasibility and cost certificate
  - `observations[]`: MSS avoids accrual, so the list is empty with rejection; the accruing loan goes to U2's second slice
  - `disclosures[]`: fixed "public-effects-only"
  - `circuitIdentity.*` to U2: pinned by the toolchain once a compatible tuple exists
- **Checker:** a new `check_u0_mss_profile.py`. It fails if any leaf has no disposition, or if a deferred leaf has no owner.
- **Exit:** 84 of 84 leaves are dispositioned.

**T3. Stage-relation schema v2** (M; depends on T2; coordinate with the numeric lens)
- **Output:** `stage-relation.schema.json` as `moriarty-stage-relation/2`, plus a `MIGRATION.md` row table from v1 to v2. Changes:
  - Amounts become non-negative integer strings bounded by the numeric-profile width, with an explicit `direction` (debit/credit) or a from/to pair.
  - Each amount carries `asset` and `unit` refs that resolve against `numeric-profile.json`.
  - `authority.consumed` and `authority.remaining` become per-asset amount vectors.
  - Policies become tagged unions (`kind` plus typed params), e.g. `validity: {notBefore, notAfter, clock}` and `replayPolicy: {kind:"nonce-set"|"sequence", key}`.
  - `judgments.*` moves to a separate evidence envelope, outside the semantic relation.
- **Checker:** `check_u0_stage_schema.py` validates v2 and also a negative corpus: an instance with principal −1 or an unbound unit must fail the *schema*.
- **Exit:** every negative schema instance is rejected, and every golden instance from T5 is accepted.

**T4. Formal judgments v2** (M; depends on T3)
- **Output:** `judgments.json` v2. Each judgment is a list of named clauses. Each clause has an id, a formula in a small fixed predicate language (linear integer arithmetic plus set membership over relation paths), its fields, and a `numericPrimitiveRefs` link into the numeric profile for U1. Minimum clauses:
  - **E1 conservation:** per asset, Σ gross = Σ supplyChanges (0 in MSS).
  - **E2 net derivation:** net = gross aggregated per account + fees.
  - **E3 range:** 0 ≤ every amount < 2^w.
  - **E4 liability roll-forward:** closing = opening − discharged + accrued, and closing ≥ 0.
  - **I1** Σ signer gross debits ≤ grossDebitCap.
  - **I2** Σ fees ≤ feeCap.
  - **I3** each recipient's net ≥ minNetOutcome.
  - **I4** recipients and assets ⊆ the signed sets.
  - **I5** validity window contains the stage time.
  - **A1** remaining_post = remaining_pre − consumed ≥ 0.
  - **A2 replay freshness:** intentId ∉ replayState_pre and intentId ∈ replayState_post.
  - **S1** coreRef = H(Core bytes) and sourceRef = H(source bytes), with the correspondence named.
  - **H1** predecessor commitment = commitment(pre-state).
  - **F1** on reject, retained effects are ⊆ the permitted phase effects and retained fees ≤ feeCap.
- **Checker:** C9 is extended to parse every clause and resolve its paths.
- **Exit:** every non-deferred MSS leaf appears in at least one clause, and no clause references an undefined path.

**T5. Executable Core-to-relation projection** (M/L; depends on T3 and T4)
- **Output:**
  - `experiments/moriarty-language/src/successor/stage-relation.ts`, a total function `(program, preState, inputs, signedIntent, KernelResult) → StageRelation`
  - golden instances under `deliverables/u0-semantic-contract-v2/golden/`, covering an MSS transfer, a repay, and a reject case
- **Test:** `tests/test_u0_projection.py`. It runs the projection on the fixtures and validates the output against schema v2.
- **Exit:** leaves marked `dynamic` in T2 are produced by executed code rather than cited declarations. Embedding status for those leaves is recomputed as "demonstrated-by-projection".

**T6. Reference judgment evaluators and K coverage** (L; depends on T4 and T5)
- **Output:**
  - `formal/k/stage-relation.k`, which reuses the EJSON machinery from `lifecycle-data.k`: one K function per clause and one per judgment
  - a TS twin, `stage-judgments.ts`
  - a hostile corpus of mutated golden instances: at least one mutation per clause, including principal −1, borrower overdraft, a fee over the cap, a replayed intentId, a wrong recipient and a stale validity window
- **Test:** `tests/test_u0_judgments_differential.py`.
- **Exit:**
  - K and TS agree on 100% of cases.
  - Every golden case is `held`.
  - Every mutant is `violated` on the intended clause.
  - Clause coverage: each clause is killed by at least one mutant.

  Only then does a judgment row in the K reconciliation become `covered`, scoped to MSS.

**T7. K reconciliation rescope** (S; depends on T6 for status)
- **Output:** `k-reconciliation.json` v2 with `u0Scope` (bool), `ownerMilestone` and `scopeQualifier` columns, and an objective `covered` rule: K clause definitions + executed golden/negative corpus + a TS differential match.
- **Checker:** `check_u0_k_reconciliation.py` rejects a `covered` row that has no executed-corpus evidence path.
- **Exit:** 6 of 6 judgment rows are covered at MSS scope, and the UNI rows carry owners. UNI-002 ("one semantic relation") may be marked `partial (MSS)`.

**T8. Signed-intent abstract syntax** (M; depends on T3; can run in parallel with T5)
- **Output:** a Core-level `SignedIntent` type and an EBNF fragment for a source `intent` block, specification only.
- **Coordination:** reserve a profile id with the aeon package, which also claims "/6".
- **Checker:** a grammar-parse test on two example intents.
- **Exit:** the Core type round-trips to `signedIntent` in schema v2. The compiler and native authentication belong to U2, owned by the U2 slice lead.

## 4. Exit redefinition

Yes. Split the gate into three states:

- **U0-F, contract frozen.** T1–T4 and T7 are complete, with schema v2 and judgments v2 hash-pinned.
- **U0-D, reference semantics demonstrated for MSS.** T5 and T6 pass. K and TS judgments execute over golden and hostile corpora, and the dynamic MSS leaves are produced by the executed projection.
- **Native capability.** This is not a U0 condition.

"Enforcement map 0/84" and "no compatible tuple" should stop being *U0* open reasons. They become owned inputs: native enforcement of each clause belongs to U2, and arithmetic clause certificates to U1.

Minimum that must be demonstrated in U0: U0-D. Deferred items, with owners:

| Item | Owner |
| --- | --- |
| `resources.*` | U1 |
| `circuitIdentity.*` binding and native clause enforcement | U2 |
| Observations with issuer and finality, and the accruing loan | U2 second slice |
| Partial-phase failure beyond atomic | U3 |
| Non-ledger-induced history | U4 |
| UNI-011/012/013 | U5 |

## 5. Risks and things NOT to do

- **Do not grow the name-inventory table.** Adding more cited declarations raises "partial" counts without demonstrating anything. Replace citation with execution (T5).
- **Do not add 60+ fields to source/5 to turn absent into present.** Most leaves are toolchain, ledger or envelope data, and the MSS fixes many of them as profile constants.
- **Do not let the reviewed-claim `covered` label survive.** It is a FOOTGUNS pattern: agreement is not coverage.
- **Do not adopt the aeon refinement package inside U0.** Take its invariants as judgment clauses (T4) and its SMT files as a cross-check. Refinement typing, VC discharge and gadget elimination belong after U2 has a native path, and aeon's `/6` profile must not collide with T8.
- **Do not touch `spec/examples/loan.mori`.** Its bytes are bound to the only banked Preview evidence, as aeon's proposal notes.
- **Risk:** Midnight's fee-on-failure semantics, which F1 needs, may not be observable before U1. Mark F1's `retainedFees` clause as premised and link it to trust-premises.json.
- **Risk:** K performance on corpus size. Keep the hostile corpus at roughly one to three mutants per clause.

## 6. Disagreements

1. **Headline counts.** The "1/17/66" headline mixes evidence-envelope and toolchain leaves with language gaps. Report per layer instead (T1).
2. **K scope.** A 23-row K reconciliation with 17 roadmap-wide UNI rows cannot be a U0 metric. It should measure the six judgments at MSS scope.
3. **Judgments.** `judgments.json` "recorded" definitions are not judgments. A U0 gate that lists "stage, intent, effect, authority, history and failure judgments" (ROADMAP.md:21) should require executable clauses.
4. **Schema.** The schema's signed, unit-free amounts and opaque policy strings contradict MORIARTY-CONSOLIDATED-DESIGN.md ("Assets ... have exact identities and units"; "Track gross debit and fee limits separately").
5. **"Present" rule.** A source-only declaration currently counts as "present". It should require both source and Core, or state per-layer status.
6. **Aeon's framing.** Aeon says the written guards are unsound. I agree for source guards, but executed Core and K reject the case (lifecycle-kernel.k:52; financial-lifecycle.ts:1461, 1569). The U0 remedy is to name the law as a judgment clause, not to treat Core as broken.

## 7. Top three recommendations

1. **Make the judgments executable (T4 + T6).** Clause-level predicates, evaluated in K and TS, differentially tested against golden and hostile corpora, including the aeon −1 case. This is the single artefact U1 certifies against and U2 enforces.
2. **Replace citation-based embeddings with an executed Core-to-relation projection for the MSS (T2 + T5).** Pair it with per-leaf dispositions (dynamic, fixed, empty-with-rejection, or deferred with an owner), so that "embedded" means "computed by running code".
3. **Split the gate into U0-F (frozen) and U0-D (reference-demonstrated), and rescope K (T7).** Move native enforcement and pins out of U0's open reasons into owned U1/U2 inputs, and give `covered` an objective, executed definition.
