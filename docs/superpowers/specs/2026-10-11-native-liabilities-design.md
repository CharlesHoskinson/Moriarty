# Native persistent liability lowering

Prepared next delivery slice under the approved entire Moriarty roadmap. PR25 merged at60b3657a5a840c07cf82b7d59b3e6ed3a8b67bb1, tree d8d5b4459d7ce921b1ff52700377532f96441bf7. Sol6.1HIGH and Grok4.6HIGH independently approved this next design. This specification is not implementation, measured feasibility or certification.

The missing behavior is compilation/proving of the existing liability and discharge statements and preservation of a nonempty liability predecessor. Keep the existing source grammar and exact reference semantics. Derive a finite typed registry across all source actions, sorted by unique name, with immutable exact asset/debtor/creditor identity. Reject conflicting identities, unresolved discharge names and scenario debts outside this supported registry at backend admission. These restrictions leave broader interpreter behavior intact.

Each action carries every registered liability, including untouched duties, as public opening/closing Boolean presence and UInt128 amount. Absence requires zero amount; present zero remains distinct and consumes capacity. Accrual uses checked addition and sets presence true; discharge asserts presence, uses checked subtraction and retains presence at zero. Source-ordered effect positions constrain before/after amount and presence. Existing tokens, state and events remain separate; discharge implies no payment or consent. A fixed compiler-profile ceiling of16 registry entries is an engineering boundary, checked before registry/circuit allocation, not a measured proving/fraud/replay certificate.

Use artifact `moriarty-zkir-artifact/2`, ABI `moriarty-public-abi/2`, relation `moriarty-zkir-relation/2`, proof `moriarty-native-transition-proof/2` and binding `moriarty-native-transition-binding/2`. Commit ABI/relation, the fixed compiler-profile ceiling and full typed registry in context/claim. The compiler constant is `MAX_LIABILITY_REGISTRY_ENTRIES = 16`; do not add a Target field or pad the circuit to16 slots. Artifact2 requires liability_frame and strict unknown-field refusal. Publish the compatibility boundary: existing source semantics remain unchanged, while previous artifacts/proof bundles need recompilation/reproving for the new relation profile and may never be silently reinterpreted. The new trusted compiler descriptors and actual proof public-value reconstruction must agree on every position, operation, immutable identity and complete step snapshot. Keep offline SRS provenance, explicit measured-k caps, bounded JSON/export and exclusive proof publication.

Independent tests cover absence versus present-zero, fresh creation/accrual/partial/full/zero discharge, untouched debts in non-liability actions, creation and repayment in distinct actions, sequential intermediate overflow/underflow, exact-identity mismatch, duplicates, unresolved debts, changed effect/frame presence and amounts, cap overflow and existing liability-free behavior. Upstream IR parsing and preimage checks are evidence distinct from native proofs. Add Quint deterministic witnesses and seeded invariant sampling with explicit cryptographic/authentication premises. No new native execution is authorized by the previous payment allocation; measure proposed circuits and obtain separate bounded votes before genuine proof work.

The result is a local native duty-transition precursor for U2/U3/M13. Authenticated predecessor completeness, debtor consent, signed monetary caps, persistent ledger consumption, pending/recovery/races, reserved recovery work, private/recursive history, live target deployment and full roadmap acceptance remain open.

Independent design votes are retained in .moriarty-dev/runtime/native-proofs-20261011/{sol-next-liability-design-vote.md,next-liability-design-grok.json}; the public decision and expectations are included below as provenance, not source/result approval.

# Sol next-liability design vote — 2026-10-11

**Vote: APPROVE** the specified static-registry liability slice, including the objective16-slot compiler ceiling and explicit versioned artifact/proof compatibility boundary below. Reviewer: **GPT-6.1 Sol, requested HIGH effort**, inherited explicit identity. Vote time: **2026-10-11 10:49:41 UTC**. This is a consequential design vote, not production-source approval, proof/resource authorization, a new implementation start, or actual-result acceptance.

Confirmed exact inspected snapshot: commit **`c22aea0574c7ee0b544a711a43f6f6a68f2eaf73`**, tree **`d8d5b4459d7ce921b1ff52700377532f96441bf7`**; worktree `/home/charl/Moriarty/.worktrees/native-proofs-20261011` clean. Required skill/AGENTS remain loaded and status refreshed; historical SP01 binding/accounting/history gaps remain, no pending transactions. The detailed independent expectations remain unchanged at `next-liability-expectations.md`, SHA-256 `286ecbc057d730e705df14dcf1cb173f01126ec387dde04141a0b84fc846639e`.

## Approved relation and admitted subset

Derive one stable sorted registry from all existing literal `Liability` statements in every action. Index by exact name, with immutable typed identity `(name, AssetId(network,issuer,reference,scale), debtor, creditor)`. Repeated identical identities share one slot; the same name with any conflicting tuple refuses backend admission. Full-tuple indexing must not admit duplicate names, because the existing reference semantics use name uniqueness. Every Discharge name must resolve to this program-wide registry, including a repayment action whose matching creation is in a different action.

All actions, including ones with no liability statements, expose/constrain opening and closing **Bool presence plus UInt128 amount for every registry slot**. Absent implies amount0; present0 is distinct and persists. Liability statements set presence true and checked-add their delta; Discharge asserts presence, checked-subtracts delta and preserves presence even at0. Sequential before/after amount/presence effects chain in source order, preserving current expression/update order. Unmodified slots remain exactly equal through each action. No implicit payment, mint/burn, deletion, identity reassignment, forgiveness, or default erasure is inferred.

Every source/scenario uses the generic compiler/proof consumer. Unknown scenario names, wrong full tuples, conflicting source identities and unresolved discharge-only names are explicit unsupported/backend-admission refusals. They are not weakened, silently dropped or used to produce scenario-specific circuits. Existing parser and reference interpreter retain their broader accepted grammar and original semantics; the new backend documentation states the narrower admitted subset. The same independently authored program can accept many valid amounts/scenarios without approval or fixture matching.

This design is coherent because static literals already contain identities needed for creation, while Discharge otherwise lacks asset/parties. A fixed source-derived frame resolves that missing identity and constrains untouched registered duties without a new dictionary circuit. A more permissive dynamic table requires additional name matching, uniqueness, allocation and identity constraints and is a later capability. An action-local registry would erase or leave unrelated duties unencoded. A scenario-derived registry would make compiled layout/key selection depend on prover input. Keeping only host preservation would not establish the requested in-circuit frame relation. These alternatives do not meet this slice's intended boundary as directly.

## Approved cap16 and resource meaning

Let **C be the number of distinct registered names over the whole program**. Admit **0 <= C <= 16**, including C0 for existing liability-free programs. Compile actual C slots, not16 padded slots for every program. The cap counts possible identities, not only currently nonzero or present records; zero residuals never free identities. Every state has `sum(present_i) <= C <= 16`.

Check the ceiling before allocating proportional ABI/frame/circuit structures or emitting their instructions. Derive the registry with a bounded map; refuse a17th distinct name before insertion or growing another slot vector. Existing parsed Program allocation and its source/IR bounds remain separately necessary. Duplicate identical source occurrences do not consume extra slots; identity conflicts still refuse.

Sixteen is approved as a small explicit **engineering cap**, sufficient to exercise independent simultaneous and untouched duties without introducing unbounded persistent growth. It is not a theorem about expressivity, a measured k/rows/proof-time ceiling or an authorization to prove a16-slot circuit. Smaller ceilings could reduce cost but would unnecessarily narrow the useful typed-frame exercise; the existing1,024 interpreter-liability bound supplies no basis for a native proof budget. The exact16 value is public/objective compiler capability metadata and part of the revised relation/profile identity, never a developer registry or council condition.

If compile/model-only measurement later shows16 infeasible under the supported target, report that limitation and reconcile a scoped cap/encoding decision. Do not increase native k, start proof runs, or silently lower/change the advertised cap. Existing source, escaped-output, trace, instruction, artifact, statement-count and caller k bounds still apply. The previous one-payment k10 allocation does not authorize liability proving.

The cap bounds identity count only. Per-owner/per-asset monetary closing-liability caps and their checked aggregate remain the separate existing host `intent::check_effects` predicate. This slice does not consume signed intent in the native relation and must not claim cryptographic signed-cap enforcement or debtor consent.

## Approved revision and compatibility boundary

Version the new backend meaning explicitly. A coherent minimum implementation is:

- Artifact/positional ABI format **`moriarty-zkir-artifact/2`**, containing exact registry layout and profile maximum16.
- Relation context domain **`moriarty-zkir-relation/2`**, committing the revised ABI/profile identity and source-derived registry alongside canonical Program, exact target and selected action.
- Native proof bundle/claim format **`moriarty-native-transition-proof/2`** and binding domain **`moriarty-native-transition-binding/2`**, with the exact revised relation/ABI/profile identity included in the canonical claim. Keep source-byte, program, target, scenario, complete execution, action and measured k commitments.

These version strings are proposed concrete spellings, not already implemented formats. Equivalent explicit version commitments are acceptable if the final design documents them before authoring. Merely changing a report label while retaining an ambiguously identical relation context is insufficient.

Use one explicit documented boundary: the new verifier accepts the new format and refuses old v1 artifacts/proof bundles; historical v1 proofs remain verifiable only with their exact retained v1 implementation/parameters. No silent v1-to-v2 reinterpretation or claimed proof migration. Valid old **source** programs within the admitted subset still parse/simulate under unchanged `moriarty/1` / `moriarty-core/1` and can recompile/reprove for v2. Local snapshot semantics remain unchanged. Document refusal versus recompilation; preserve frozen v1 implementation/evidence. This is a backend proof compatibility break, not a new source language.

A dual v1/v2 verifier is a coherent alternative only with explicit dispatch to both exact relation implementations and their trust/profile checks. It adds maintenance/security scope and is unnecessary for this smallest next delivery. An unversioned positional ABI expansion is unsafe. Reusing source semantics version1 is correct because liability semantics already exist; claiming that old proof bytes acquire the new framing guarantee is incorrect.

## Acceptance limits and sequencing

Only start this implementation after the sole current native-proof item is merged following its required final pair. This vote authorizes no worktree edits, competing delivery item or crypto run now. Both selected substantive design votes are still needed for the consequential decision; this report is Sol's vote only.

Later exact-candidate audits must check static identity closure, full-action frame constraints, absent/present0 lifecycle, checked arithmetic/sequential failure, generic consumer ABI reconstruction, explicit version refusal, cap boundaries and independently authored contrasting programs from the unchanged expectations. Native evidence requires a separately admitted bounded resource experiment.

Keep these gates open: adversarial source-to-constraint soundness; authenticated actual predecessor and full real-liability completeness; debtor consent/signed owner intent and monetary caps; unique consumption and history/PCD; ledger phase correspondence, financial settlement/finality and Preview deployment; pending/unknown/partial/recovered lifecycle, late-success/cancellation/compensation races, scoped recovery authority and reserved work. The compiler can preserve the complete admitted local registry without proving the caller supplied every real outstanding obligation. This local liability slice advances U2/U3/M13 preparation without closing those broader exits.

No source edits, native calls, resource executions, network, external Fenrir changes or commit made for this vote. Earlier expectations, resource votes and audit findings remain preserved.


I will load the Moriarty develop skill and AGENTS override first, then inspect the Sol recommendation, expectations, and actual IR/runtime/proof paths before voting.Next I’ll load the Moriarty develop skill and AGENTS override, then inspect the Sol recommendation and source paths.I’ll load the develop skill, AGENTS override, and the next-liability expectations packet next.The skill requires `status` at startup. I’ll run that, then locate the expectations packet and the frozen source identity.Status is loaded. I’ll locate the expectations packet and confirm the frozen commit identity next.Found the packet under `.moriarty-dev/runtime`. I’ll read the Sol recommendation, expectations, and supporting source next.Sol’s packet is loaded. I’ll inspect core IR, runtime, parser, backend, and the U2/U3/M13 contract before voting.I’ll now inspect the actual liability/discharge paths in core IR, runtime, parser, and backend.I’ll read the runtime liability semantics, backend refusals, proof reconstruction, and ABI next.Status loaded. Frozen checkout matches `c22aea0574c7ee0b544a711a43f6f6a68f2eaf73` / tree `d8d5b4459d7ce921b1ff52700377532f96441bf7`. This vote does not approve that implementation.

## Decision

**APPROVE** the Sol 6.1 HIGH static typed registry as the next source-to-ZKIR/local-proof slice after the frozen native-proof pair closes.

Requested identity: Grok 4.6 high. Returned identity: Grok 4.6. Read-only design vote only.

## Rationale

The reference evaluator already has the financial meaning. `Program` has no liability declarations. `liability(name, asset, debtor, creditor, amount)` and `discharge(name, amount)` use literals plus a UInt expression. Snapshot identity is `(name, AssetId{network,issuer,reference,scale}, debtor, creditor)` with **name unique**. Accrual uses checked `u128` add and writes a present record even at amount 0. Discharge requires a present name, uses checked sub, and leaves a **present-zero** record. Discharge does not transfer, mint, or erase identity. Core `Effect::Liability` carries before/after amounts and no presence bit, so amount 0→d cannot tell absent-create from present-zero accrual.

`compile` emits a scenario-free relation. `lower_action` currently refuses every `Liability`/`Discharge`. `prepare` refuses nonempty scenario liabilities and any liability effect. That is the missing lowering, not a missing grammar.

A program-wide registry `R` derived from every `Liability` literal preserves compile-without-scenario. Discharge-only names, extra scenario identities, and conflicting source tuples cannot enter that relation without a dynamic table or a new declaration form. Both of those are larger than this U2/U3/M13 precursor. The interpreter already accepts those broader programs; the compiled subset refuses them and leaves evaluator semantics unchanged.

Putting every `R` slot on every action is the right frame for this slice. Discharge carries only a name. Sibling actions already share one program digest. Host-only preservation of untouched duties would split one action that discharges `L1` while `L2` stays in `R`. Copy-constraints on untouched slots are the cheap alternative to a dictionary circuit. This still does not authenticate a real predecessor.

Cap **16** is an engineering admission ceiling on `|R|`. Circuit width is `|R|`, not a padded 16. It is not a k, replay, or ledger budget.

## Exact constraints

Bind these before authoring.

**Registry**
- Derive `R` from every `Liability` statement in every action. Index by exact `name`. Store immutable `(AssetId, debtor, creditor)` after resolving the source asset alias.
- Names are unique. Two names with the same parties are two slots. Two statements with the same name and different AssetId/debtor/creditor refuse at **compile**.
- Sort `R` by name, matching runtime snapshot order.
- `|R| > 16` refuses **before** slot vectors, ZKIR text, or parameter load. Use one compiler-profile constant `MAX_LIABILITY_REGISTRY_ENTRIES = 16` recorded on the artifact. Do not add a `Target` JSON field. Keep existing target instruction/artifact, interpreter 1,024/65,536, and native k/proof-byte caps.
- Present-zero records occupy their slot. Discharge does not free a name for reuse. `C = |R|`. `live_count = sum(present_i) <= C`.

**Admission split**
- Compile refuses: conflicting source identity, `discharge` of a name absent from `R`, `|R| > 16`.
- Prove/reconstruct refuses: scenario name not in `R`; scenario name present with AssetId/debtor/creditor ≠ `R`; absent-with-nonzero amount.
- `mori-core` parse/validate/simulate/replay stay on `moriarty/1` and `moriarty-core/1`. No evaluator change to close the subset.

**Circuit / ABI**
- Every action, including actions with no liability statements, publishes for each `i in R`: opening `(present_i: Bool, amount_i: UInt128)` and closing `(present'_i, amount'_i)`.
- Empty `R`: zero liability public slots. Liability-free source still compiles. New `liability_frame: []` is schema, not extra public values.
- Constrain each presence with `ConstrainToBoolean`. Constrain `(1-present)*amount = 0` at opening and closing. Range-constrain every UInt to 128 bits with the existing add/sub/mul lowering. No modular amount arithmetic.
- Accrual: `a_next = checked_add(a,d)`, `p_next = true` even when `d = 0`.
- Discharge: `Assert p`, `a_next = checked_sub(a,d)`, `p_next = p` even when `a_next = 0`. Absent discharge 0 fails.
- Untouched slots: `p' = p`, `a' = a` in-circuit.
- Ordered per-step constraints follow source order. Final opening+accrual−discharge=closing is a derived check and cannot rescue transient overflow/underflow.
- Per-effect public positions are mandatory: kind `create_or_accrue` or `discharge` from the **statement**, name, before/after **presence** and before/after **amount**. Core `Effect::Liability` stays amount-only. Presence 0→1 versus 1→1 is what distinguishes absent-create from present-zero accrual.
- Add `CircuitArtifact.liability_frame` with `name`, `AssetId`, `debtor`, `creditor`, and the four opening/closing positions. Do not overload `balance_frame` or transfer `from`/`to` as the identity source.
- Reconstruction fills each slot from canonical `pre`/`post`/step snapshots: name present in the vector ⇒ `present=true` plus exact tuple match; omitted ⇒ `present=false`, `amount=0`. Do not infer presence from `amount > 0`. Extra snapshot names refuse. Keep original scenario, full execution, source-byte hash, program/target/action binding, trusted VK reconstruction, EOF, and bounded proof hex.

**Revision / compatibility**
- New identities, all bound in context: `moriarty-zkir-artifact/2`, `moriarty-zkir-relation/2`, `moriarty-native-transition-proof/2`, `moriarty-native-transition-binding/2`. Put the ABI revision in the hashed context tuple.
- Artifact/2 requires `liability_frame` and `deny_unknown_fields`. Old artifact/1 JSON is a different profile.
- Liability-free **source** behavior remains: same programs compile, simulate, and admit local proofs under the new compiler.
- Old c22 proof bytes, VKs, and `/1` artifacts are a different revision. The new compiler must fail them closed. Do not keep `/1` relation tags for empty `R` as a silent dual ABI.
- `mori prove` / `mori verify` / `mori compile` stay generic descriptor consumers. No host success flag, native escape, fixture name dispatch, or loan-specific lowering.

**Non-claims**
- No automatic transfer, consent, forgiveness, deletion, or signed `liability_cap` in-circuit. `intent::check_effects` stays host-only until a later signed native acceptance relation.
- Qualification remains `local-native-transition-proof`. Unqualified `toolchains/ledger-9.json` is not ledger, history, or predecessor authentication.
- A caller can still prove a locally empty opening for registered names. Rejecting reuse of an old scenario proof does not prove real-world duty completeness.

**Native execution**
- Do not start implementation while the frozen slice still awaits its final source/result pair.
- First delivery is compile + reconstruction + honest preimage/`check` correspondence plus the tables below.
- Actual native prove/verify of liability circuits needs **new** k/byte/time measurements and **two** resource votes. The previous one-proof k10 payment allocation does not authorize this slice.

## Positive and hostile controls

Adopt Sol’s tables as the acceptance contract for this slice, with these bindings:

- Absent create 7, present-zero create, full discharge to **present 0**, ordered 10→14→11→13→8, create/discharge/create on one name, shared `R` across create and repay actions, mixed transfer/fee/state/events, UInt128 boundaries, reversed scenario serialization (canonical ABI; raw scenario hash follows array order), `|R|` at 16, and two independently authored obligation programs on the generic compiler path.
- Duplicate names, conflicting source tuples, unresolved discharge-only names, extra scenario identities, absent discharge 0, `present=2`, absent-with-nonzero, untouched slot omission, reordered add/discharge hiding underflow, max+1, payment-for-discharge substitution, relabelled metadata, bundle-selected VK, host `liability_cap` evasion labeled host-only, cap+1 before allocation, and late predicate failure remaining atomic.
- Keep separate evidence columns: source/host refusal, honest preimage `check`, targeted adversarial constraints, genuine native verify, ledger acceptance. `check` success is not native soundness.

## Remaining acceptance gates

1. Frozen c22 native-proof slice: independent final source and actual-result pair. This vote is not that approval.
2. After that pair: implement this registry on the same `compile`/`prove`/`verify` path, with both selected fresh audits on the frozen candidate.
3. Local compile/reconstruction/`check` must close the tables above, including in-circuit presence and untouched slots.
4. Distinct native measurements at `|R| ∈ {1,2,16}` and two resource votes before liability prove/verify. Failure to synthesize at 16 revises the ceiling; it does not silently drop slots.
5. Authenticated predecessor, complete stored frame, unique consumption, phase effects, signed caps/consent, continuations, timeout/default, cancellation/compensation, and coordinator pending/partial/unknown/settled/recovered (U3/M13) stay open.
6. U0–U7 and M1–M17 stay active. This slice is a persistent local typed-duty precursor, not U2 generality, not MC05, not Fenrir, and not a second implementation.

## Counterargument

A touched-only frame matching `balance_frame` would keep sibling transfer actions cheaper. It would also leave untouched duties to host reconstruction, which is already how balances work, and it cannot close completeness. The discharge name-only statement and the duty-omission rule make the full-`R` frame the smaller correct circuit, not the dynamic table.

New `liability(...)` declarations at program scope would make identity explicit for discharge-only programs. That is a `moriarty/1` grammar/version change and extra author burden. Reuse of existing literals is the smaller useful subset.

Cap 16 is unmeasured. Keep it as a named admission ceiling. Do not pad. Do not put it on `Target`. Revisit only with compile/model numbers, then a resource vote.

## Evidence inspected

- Skill/AGENTS status: capability `SP01.6 loan-swap-subset`; operational-history/accounting gaps; no pending transactions.
- Frozen HEAD `c22aea0574c7ee0b544a711a43f6f6a68f2eaf73`, tree `d8d5b4459d7ce921b1ff52700377532f96441bf7`.
- Sol packet `/.moriarty-dev/runtime/native-proofs-20261011/next-liability-expectations.md`.
- `crates/mori-core/src/{ir,parser,runtime,intent}.rs`, `tests/language.rs`.
- `crates/mori-backend/src/{lib,proving}.rs`, `tests/{compile,proving}.rs`, `README.md`.
- CLI `compile`/`prove`/`verify`; `examples/core/native-payment/payment.mori`.
- `docs/MORIARTY-PRODUCT-CONTRACT.md`, `ROADMAP.md` U2/U3, M13 in the Fenrir delivery map, `docs/NATIVE-CORE.md`.

No source edits, no native prove/verify/SRS/keygen, no Fenrir/network work, and no inference of c22 source approval from this vote.