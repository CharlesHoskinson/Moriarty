Independent read-only MIL/4 S1B Stage-guard review. Review only exact embedded file bytes; do not use tools, skills, delegation, external pages or workspace files. Requested reviewers are Grok 4.7 xhigh and GPT-6.1 Sol high, independently. Prior postreview packet SHA256 0e70d765305e7400becb48df9485ed33df6b8affc052de4ad405b699a4629f2e earned narrow LOCAL adequacy from both, but GPT found Quint extra-footprint/work-domain gaps and Grok found TS H7-H9 fault vectors and field-map/status drift. This candidate repairs those points. Check full exact candidate, test strength, first-code precedence, financial effects and field mapping. Give separate verdicts: adequate finite LOCAL S1B comparison, and whether W-D0-W-D4/Sprint1 can close. Stipulated tuples do not authenticate signature, snapshot, successor, proof or ledger admission. Guarded SP01.6 remains blocked.

## Manifest

```json
[
  {
    "path": "deliverables/mil4-k-quint-sprint1-2026-09-29/S1B-HEAD-FAILURE-EXPERIMENT.md",
    "bytes": 4511,
    "sha256": "7d4df07c8355ea079bcec312e91a44192ba64da66fb918c5f0efb1771d8f117d"
  },
  {
    "path": "deliverables/mil4-k-quint-sprint1-2026-09-29/S1B-FINITE-RESULT.md",
    "bytes": 5880,
    "sha256": "3dbebbc7909554c7605d637f5aeeadacae78b66ee4ad4d63d24a2dfbf30e5e39"
  },
  {
    "path": "deliverables/mil4-k-quint-sprint1-2026-09-29/S1-FINITE-CORRESPONDENCE.md",
    "bytes": 6578,
    "sha256": "6928184a4f10e2485c78862be0e5cf2214c7593a56a9499f447652d8484c465a"
  },
  {
    "path": "experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md",
    "bytes": 11648,
    "sha256": "725662ae7572a93cbc8f0d1631ca364a56d1c3dff3b9b02cae92485cd5a899ec"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/s0-implementation-contract.md",
    "bytes": 6518,
    "sha256": "a7f564e3c0a72f549b09849fdd7d3de5e0b0ff1b74ec34261bd9a229acf9a788"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/projection.md",
    "bytes": 8549,
    "sha256": "923fbf40948134c8b13f4fbbb0f6ca98131c7fb62dc25e8053a853c2ce581bf4"
  },
  {
    "path": "experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts",
    "bytes": 20746,
    "sha256": "f3d2a05d07b051d2097fd7fb00ed1cd013eba91e9da7a2591b6f69707d4a02e7"
  },
  {
    "path": "experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts",
    "bytes": 17834,
    "sha256": "e0b6c203eb29045ef6e4357033f6cb519fc1783a9c8e1fc3087227c7795d6aa3"
  },
  {
    "path": "experiments/moriarty-language/src/successor/mil4-s0-source-v6.ts",
    "bytes": 2640,
    "sha256": "1b3bd3b01ed2694ea967f62fea6808843a198045d870270b1d44c121081f66d8"
  },
  {
    "path": "experiments/moriarty-language/tests/mil4-s0-source-v6.test.mjs",
    "bytes": 25947,
    "sha256": "6cdce3bf80bed4f2ce6b32596169e98fe7ee7cdad618406506fdeab17b687971"
  },
  {
    "path": "experiments/moriarty-language/formal/k/mil4/s0.k",
    "bytes": 17632,
    "sha256": "e76c168bc6826e13308f19a34f229e5b07b64196a41892a6ac3122890d4c0cc0"
  },
  {
    "path": "experiments/moriarty-language/formal/k/mil4/corpus/s1b_run.py",
    "bytes": 4900,
    "sha256": "3649ecfee714b1c94ee1cbf8fcffafc7b745368dac77d15cafe8aa3e9f8702a2"
  },
  {
    "path": "experiments/moriarty-language/formal/k/mil4/corpus/s1b-verification.json",
    "bytes": 1915,
    "sha256": "71dd8b6b682c33ff4cf452dbb11456bc78a9c735bd7035fc809d6d9c8c47bad5"
  },
  {
    "path": "deliverables/mil4-k-quint-sprint1-2026-09-29/audits/s1b-postreview-k-observations.json",
    "bytes": 33789,
    "sha256": "f7e3f53d81595abdc6e6b434277d6ea8139c5aa913ab98d32f0336cacaa94242"
  },
  {
    "path": "deliverables/mil4-k-quint-sprint1-2026-09-29/audits/s1b-postreview-k-regression-observations.json",
    "bytes": 16792,
    "sha256": "43bc438d0301c483561fc12fc72ce845b8d4bcb96283469a86165eeb0ab7b6d8"
  },
  {
    "path": "experiments/moriarty-language/formal/quint/mil4/s0.qnt",
    "bytes": 21207,
    "sha256": "9e7953a890325cd5d1294b19f9f00e9b5d8cc68aeffeb1dd693ce5a86467ec53"
  },
  {
    "path": "experiments/moriarty-language/formal/quint/mil4/corpus/s0_witnesses.qnt",
    "bytes": 12615,
    "sha256": "358aa6e5c3d7e19c5ca8750e03597fcf17d21d855b085a18f0c6c456807c7ce1"
  },
  {
    "path": "experiments/moriarty-language/formal/quint/mil4/corpus/s0_divergence_witnesses.qnt",
    "bytes": 10972,
    "sha256": "a46335f66e003af41f4200448a8790f85175a4b78a5942bbefa5240aecb6f885"
  },
  {
    "path": "experiments/moriarty-language/formal/quint/mil4/corpus/s0_diagnostic_witnesses.qnt",
    "bytes": 4081,
    "sha256": "badb83cb42fdf5e0e5801762602ffbe5e8301a9b0139242aece1829172974304"
  },
  {
    "path": "experiments/moriarty-language/formal/quint/mil4/corpus/s0_common_witnesses.qnt",
    "bytes": 8604,
    "sha256": "b5148618f46de24e205c5437e3fd695e5957b169f15fc83484b004a30069119e"
  },
  {
    "path": "experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt",
    "bytes": 26578,
    "sha256": "007b66094455b21467ee27fb744b55706e5a91c7a745523253ecb82cdf297591"
  },
  {
    "path": "experiments/moriarty-language/formal/quint/mil4/corpus/S1B-RESULTS.md",
    "bytes": 7295,
    "sha256": "2eb298dcb40cf650a95d11c140191f06d2c6d0808d5271943e5358bfc95db680"
  },
  {
    "path": "experiments/moriarty-language/formal/quint/mil4/README.md",
    "bytes": 5214,
    "sha256": "87e7ed03d686bfbec14451f99dcb25152c7945f7c80570a93b739c4f27565a83"
  },
  {
    "path": "experiments/moriarty-language/formal/quint/mil4/corpus/s1b-stage-guard-command-results.json",
    "bytes": 8858,
    "sha256": "ed69d4edb9a5c293ac24a1968f12a421f7f84414e837c809a720d5625e1c2a1c"
  },
  {
    "path": "experiments/moriarty-language/formal/quint/mil4/corpus/s1b-stage-guard-pre-repair-command-results.json",
    "bytes": 24640,
    "sha256": "8af96f3b19a53cbcbe770f2a70fbbc978fd0af6b5fab6782638a0e95c969ea49"
  },
  {
    "path": "deliverables/mil4-k-quint-sprint1-2026-09-29/audits/s1b-stage-guard-ts-verification.json",
    "bytes": 837,
    "sha256": "7d1676b3ccec04d8a9989f08ab1690b829ef8c25492860e8a34f00a5e41942c7"
  }
]
```

## deliverables/mil4-k-quint-sprint1-2026-09-29/S1B-HEAD-FAILURE-EXPERIMENT.md

```text
# MIL/4 S0 next finite experiment: head, failure and premise observations

**Status:** initial expectation matrix and executed finite experiment. The [result](S1B-FINITE-RESULT.md) records later controls and observations. W-D3 and Sprint 1 remain open. The executions do not establish a verified head extension, signature, native proof or ledger admission.

## Comparison boundary

Use one exact transfer pre-state, signed intent, round and complete effect vector from T-10-1. Name the predecessor `h0`, the stipulated expected successor `h1`, and an alternate token `h9`. K and TypeScript use opaque head tokens; Quint may use distinct abstract IDs, but it must compare identity and must not infer validity from integer addition. The executed stipulated external tuple is `(intent, pre-state, round, expected successor, requested outcome)`. It is an assumption in this experiment, not an authentication result.

Keep the six-judgment order `Stage → Intent → Effect → Authority → History → Failure`. Within History, check stale predecessor, then consumed replay, then successor mismatch or self-successor. Reject with one abstract diagnostic-work unit and no published post-state or effects. A rejected attempt leaves financial state, allowance, replay, head and work counters unchanged.

| Case | Modification to otherwise valid T-10-1 | Expected first observation |
| --- | --- | --- |
| H1 | Post-head and `AdvanceHead` line are `h1`; terminal outcome and empty retained vectors | Local success under the stipulated tuple |
| H2 | Post-head and complete effect line are `h9`; tuple still expects `h1` | History / `S0_HISTORY_SUCCESSOR` |
| H3 | Post-head and complete effect line are `h0` | History / `S0_HISTORY_SUCCESSOR` |
| H4 | Post-head is `h1`; effect line says `h9` | Effect / `S0_EFFECT_MISMATCH` |
| H5 | Signed predecessor is stale and successor is wrong | History / `S0_HISTORY_STALE` |
| H6 | Composite replay key was consumed and successor is wrong | History / `S0_HISTORY_REPLAY` |
| F1 | Requested phase is failure, with all earlier judgments valid | Failure / `S0_FAILURE_UNSUPPORTED` |
| F2 | Terminal success retains one effect | Failure / `S0_FAILURE_UNSUPPORTED` |
| F3 | Terminal success retains one duty | Failure / `S0_FAILURE_UNSUPPORTED` |
| P1 | Required premise is unavailable | Stage / `S0_STAGE_PREMISE` |
| P2 | Stipulated tuple changes the intent, pre-state or round, one at a time | Stage / `S0_STAGE_PREMISE` |
| P3 | A required typed cell is malformed and the premise is unavailable | Stage / `S0_STAGE_UNSUPPORTED` |

The first S1B audit found that the initial cases did not isolate all stated precedence claims. The post-audit corpus adds stale+replay, effect mismatch+replay, effect mismatch+wrong successor, and effect mismatch+requested failure. It also adds an explicit self-successor case where the tuple itself expects the self head, an outcome-mismatch tuple case, and a Quint case where a locally committed head lacks a fresh authenticated snapshot premise. These cases are in the [post-audit result](S1B-FINITE-RESULT.md); the original matrix above remains the initial independent expectation set.

The finite relation compares complete observations, including first code, diagnostic work, state preservation and the exact accepted effects. A separate qualified consumer would have to establish that `h1` actually extends the authenticated pre-state. The TypeScript preparer may expose a comparison entry point that accepts a stipulated tuple, but its ordinary result remains `PreparedUnqualified`.

## Original implementation and review sequence

1. Preserve the current frozen packet and its reviewer receipts. Fix any findings from both independent reviews before freezing the next candidate.
2. Add submitted successor and typed requested outcome to the abstract comparison inputs. The outcome contains phase, retained effects and retained duties. Make unsupported forms reach Failure after the earlier judgments pass.
3. Execute H1–H6, F1–F3 and P1–P3 in K, Quint and the local TypeScript preparer. Require literal complete outputs; a judgment-only witness does not support code agreement.
4. Publish a field-by-field finite map and a new packet hash. Obtain independent Grok 4.7 xhigh and GPT-6.1 Sol high reviews on those same bytes. Keep W-D1/W-D2 and external qualification open.

This experiment can establish only local behavior under stipulated premises. It cannot establish the premise or turn a caller token into a valid successor.

```

## deliverables/mil4-k-quint-sprint1-2026-09-29/S1B-FINITE-RESULT.md

```text
# MIL/4 S0 head, failure and premise experiment

**Status:** revised finite local experiment, pending fresh independent review of the Stage guard and combined-fault repairs. The first S1B packet was rejected as an aligned local comparison. The second packet received narrow local adequacy votes from GPT-6.1 Sol high and Grok 4.7 xhigh, which also found the gaps now repaired. Their raw reviews remain in `audits/`. W-D0–W-D4 and Sprint 1 remain open. Every external tuple below is stipulated for comparison; no model authenticates its truth.

## Executed slice

The [S1B experiment design](S1B-HEAD-FAILURE-EXPERIMENT.md) uses a common T-10-1 transfer pre-state with balances 100/0/0, owner allowance 11/0, one work unit, round zero, gross cap 11, fee cap 1 and net floor 0. K uses opaque heads `h0`, `h1`, `h9`. Quint uses distinct abstract head IDs `0`, `1`, `9`; TypeScript uses the opaque text heads. The local map identifies those tokens, K domain `D` with TypeScript `Midnight`, and K/Quint `O/R/F` with TypeScript `Owner/Recipient/Fee` for these fixtures only. Nonces are mapped by case name. Successor equality is a comparison to a supplied expected token. It is not proof of head extension.

| Observation | K | Quint | TypeScript direct Core |
| --- | --- | --- | --- |
| H1 exact stipulated successor | Complete accepted term | Complete accepted state and effects | `PreparedUnqualified` with candidate head `h1` |
| H2 alternate complete successor | History / `S0_HISTORY_SUCCESSOR` | Same | Same |
| H3 self successor, including self stipulated as expected | History / `S0_HISTORY_SUCCESSOR` | Same | Same |
| H4 wrong `AdvanceHead` line | Effect / `S0_EFFECT_MISMATCH` | Same | Same |
| H5 stale predecessor plus wrong successor | History / `S0_HISTORY_STALE` | Same | Same |
| H6 consumed replay plus wrong successor | History / `S0_HISTORY_REPLAY` | Same | Same |
| F1 requested failure; F2 retained effect; F3 retained duty | Failure / `S0_FAILURE_UNSUPPORTED` | Same | Same |
| P1 unavailable tuple; P2 wrong intent/state/round; P3 malformed cell plus unavailable tuple | Stage / `S0_STAGE_PREMISE` for P1/P2; Stage / `S0_STAGE_UNSUPPORTED` for P3 | Same | Same |
| H7 stale and consumed replay together, with the expected successor unchanged | History / `S0_HISTORY_STALE` | Same | Same |
| H8/H9 wrong `AdvanceHead` line with replay or wrong expected successor | Effect / `S0_EFFECT_MISMATCH` | Same | Same |
| F4 effect mismatch with requested failure | Effect / `S0_EFFECT_MISMATCH` | Same | Same |
| P2-outcome: tuple outcome differs from submitted outcome | Stage / `S0_STAGE_PREMISE` | Same | Same |

The TypeScript direct-Core comparison accepts a supplied `S0LocalStipulation` only as an experiment assumption. Its success still has status `PreparedUnqualified` and lists the four missing external premises. K's `<external>` cell is a stipulated typed tuple and remains unchanged during each run. Quint's typed stipulated tuple and environmental constants are model inputs, not evidence of a wallet, proof or ledger action. All three make malformed typed cell shape precede an unavailable premise. H7 pins stale-before-replay; H5/H6 pin stale/replay before successor mismatch; H8/H9 and the earlier stale control pin effect mismatch before those History failures. These observations are finite and do not prove the order for every possible input.

K added a typed outcome `(phase, retainedEffects, retainedDuties)` and a premise that binds that outcome with the intent, complete pre-state, round and expected successor. Quint added the corresponding typed comparison request, binds its outcome in the stipulated tuple and eliminated arithmetic head advancement. Its current-snapshot premise is explicit even after a local commit; a newly committed head absent from the stipulated authenticated-head set yields Stage / `S0_STAGE_PREMISE`. TypeScript added a typed requested outcome and an optional local tuple check. Source/6 still admits only `success_only` with empty retained forms; its nonempty variants reject at formation, outside these direct typed-Core cases.

## Exact observed checks

- K 7.1.337 LLVM compile exit 0; original strict corpus **37/37**, expanded S1B H/F/P **20/20**, and common positives **3/3**. Each new K result compares the complete output and unchanged external tuple. Evidence is in `formal/k/mil4/corpus/s1b-verification.json` and its referenced receipts.
- Quint 0.32.0: six files typechecked; **83/83** TypeScript-backend witnesses passed with seed `0x5`: the retained 59 and 24 new Stage guards and work-bound controls. The prior model failed all 22 new negative controls. Negative witnesses assert first code and pre-observation state with blocked submission; accepted witnesses assert complete state and effects. Commands, hashes and red/green outcomes are in `formal/quint/mil4/corpus/s1b-stage-guard-command-results.json` and `s1b-stage-guard-pre-repair-command-results.json`.
- TypeScript: `npm run typecheck` exit 0; Source/6 targeted suite **28/28**; package suite **938/938** after H7–H9 were aligned to the K/Quint fault vectors. The direct-Core test asserts complete rejection objects for H/F/P and complete H1 effects/post-state/required-premise list. Current output and hashes are in `audits/s1b-stage-guard-npm-test.log` and `audits/s1b-stage-guard-ts-verification.json`.

These are separately constructed finite tests. There is no executable cross-model projection or one canonical source lowered through K, Quint and TypeScript. K and Quint abstract away agreement ID, asset scale, source/Core hashes and signed `/3` bytes. The local tuple comparison does not establish signature validity, snapshot authenticity, native qualification, ledger compare-and-consume or actual successor validity. No `quint verify` model check or public transaction ran. The guarded SP01.6 delivery slot remains blocked.

```

## deliverables/mil4-k-quint-sprint1-2026-09-29/S1-FINITE-CORRESPONDENCE.md

```text
# MIL/4 S0 finite local comparison

**Status:** finite prototype evidence. The first repaired packet received independent GPT-6.1 Sol high and Grok 4.7 xhigh reviews: both accepted its three common positives as a narrow local prototype and rejected W-D0–W-D4/Sprint 1 closure. A later S1B head/failure/premise packet was rejected as an aligned local comparison; [post-audit S1B repairs](S1B-FINITE-RESULT.md) have new bytes and need fresh review. These runs do not prove universal refinement, signature validity, native qualification or ledger admission.

## Three common positive fixtures

K's `corpus/common_run.py`, Quint's `corpus/s0_common_witnesses.qnt`, and the Source/6 test file each execute T-10-1, R-30 and R-near-bound with the same numeric pre-state, work budget 1, round 0, signed caps and complete expected effects. Each fixes expected output as literals rather than deriving it from the model under test. K's strict parser compares the **complete** accepted pre-state, ordered effects, post-state, terminal phase, empty duty and remaining work. Quint asserts all model state maps, including absence of receiver/creditor allowance rows. TypeScript asserts the complete prepared effect vector and candidate post-state.

| Fixture | Exact common numeric pre-state | Expected complete financial post-state |
| --- | --- | --- |
| T-10-1 | Owner 100, recipient 0, fee recipient 0, owner allowance 11/0, work 1/0; value 10, fee 1, gross cap 11, fee cap 1, net floor 0 | Ordered debit 11, credits 10 and 1; balances 89/10/1; allowance 0/11; work 0/1; one replay and head advance; no obligation or receiver allowances |
| R-30 | Payer 100, creditor 0, allowance 100/0, work 1/0; obligation principal 1000, accrued 10, outstanding 1010; payment 30 | Ordered debit and bound-creditor credit 30, SetObligation 980/0/980 Outstanding; balances 70/30; allowance 70/30; work 0/1; one replay and head advance; no creditor allowance |
| R-near-bound | Payer 1, creditor U−1, allowance 1/(U−1), work 1/0; obligation (S−1)/1/S; payment 1, where U=2^128−1 and S=2^127−1 | Creditor and allowance spent reach U; payer 0; principal/outstanding S−1, accrued 0; work 0/1; one replay and head advance |

The local mapping is explicit: K domain `D` corresponds to Source/6 `Midnight` and Quint `D`; K `O/R/F` corresponds to Source/6 `Owner/Recipient/Fee` for transfer. K repayment `P/C/L` corresponds to Source/6 `Payer/Creditor/Loan` and Quint `O/C/L`. The older, non-common Quint repayment witness uses `loan`; it is not part of this map. K/Source heads `h0→h1` correspond to Quint `0→1`. Source/6 nonce `n1` corresponds to the K and Quint fixture nonce named by the case. The replay key is compared after this identifier map as `(domain,signer,nonce)`. K and Quint effect lines omit the selected asset on some constructors; this comparison supplies the single signed asset `A` as context. These are local fixture bijections, not a `/3` signed-byte mapping.

This is a field-by-field **manual** normalization of separately asserted literal outputs. There is no general executable `α_local` adapter or ledger-committed `α` proof. The common fixtures do establish the specified complete local financial pre/effect/post observations under the map above; they do not establish exact Source/6 metadata binding in K or Quint.

## Hostile observations and boundaries

The repaired K regression corpus checks 37 complete outputs, including transfer cell identity/asset/allowance Stage shape, obligation debtor/creditor Stage shape, validity/cap-plus-alias Intent order, numeric-range-plus-vector Effect order, stale/replay History order, and the original single-fault controls. The first frozen Quint corpus had 35 witnesses and the TypeScript corpus had 25 targeted checks. Several original Quint witnesses asserted judgment only. The current S1B Quint witnesses now assert exact codes, and the current TypeScript targeted suite has 28 tests. The named overlap has finite first-code evidence; no universal diagnostic theorem follows. Core rejections report one **abstract** diagnostic-work unit and no published post/effects; Source/6 formation is outside the Core judgment sequence.

H-nominal remains a Source/6 admission rejection, outside the K and Quint stage models. Source/6 rejects transfer endpoint aliases and self-creditor repayment during formation; direct typed Core/K/Quint malformed-stage experiments have a separately stated judgment. The current Source/6 wrapper returns only `PreparedUnqualified` on success. S1B K and Quint receive stipulated tuples binding intent, state, round, expected successor and requested outcome. The TypeScript direct-Core path can compare such a tuple but still returns `PreparedUnqualified`. None of these stipulations verifies the corresponding external fact.

Important correspondence leaves remain open: K and Quint do not carry all Source/6 agreement, selected-program, source-hash, asset-scale and predecessor bindings; the provisional `/3` codec has no consumer that binds its digest to signature, proof and complete effects; no model proves the stipulated successor extends an authenticated head. S1B aligns named local head/failure/premise codes, but no general executable projection exists. The Quint Rust evaluator cannot represent its UInt128 maximum literal, so the fixed witnesses run on the TypeScript backend. No `quint verify` model check or native/ledger execution has run.

## Reproduction and audit state

- TypeScript: targeted Source/6 suite **28/28**, `npm run typecheck` exit 0, package suite **938/938** after the post-audit S1B test alignment. The retained log and hashes are under `audits/s1b-postreview-*`.
- K 7.1.337 LLVM: clean compile, original strict corpus **37/37**, expanded S1B H/F/P **20/20**, and round-zero common positives **3/3**. Exact commands, requests, outputs and exits are under `formal/k/mil4/corpus/`.
- Quint 0.32.0: all six current `.qnt` files typecheck; **59/59** fixed witnesses pass with `--backend typescript --seed 0x5`. Exact commands and hashes are under `formal/quint/mil4/corpus/`.
- The two earlier frozen packet reviews and their dissent remain in `audits/`. Both new K/Quint/TypeScript source and evidence bytes require a fresh full-candidate review before broadening the S1B local claim.

A local financial correspondence claim must stay confined to the three common positives and the stated finite S1B overlap. W-D1/W-D2 signature/wire binding, W-D3 external authentication and complete code policy, W-D4 narrowing, and the guarded SP01.6 delivery remain open.

```

## experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md

```text
# Financial agreement Source/6 and Core/5 S0 contract

**Status:** provisional Sprint 0 specification, 2026-09-29. The grammar is a closed presentation syntax for one S0 stage proposal, and an isolated Source/6 parser and local Core/5 preparer implement this first slice. It is not a signed `/3` encoding or an adopted MIL/4 profile. W-D0–W-D4 remain open. See the [MIL/4 semantic contract](../../formal/mil4/semantics-contract.md) and [S0 implementation contract](../../formal/mil4/s0-implementation-contract.md).

## Formation and version gate

The [EBNF](financial-agreement-source-v6-grammar.ebnf) parses exactly one `profile`, one `agreement`, and one stage proposal. Its header must contain the raw token `"moriarty-financial-agreement-source/6"` before parsing the agreement body; a differently escaped spelling is outside this provisional presentation. The fixed declaration and field order is presentation syntax, not a claim about signed bytes. Unknown, repeated, omitted or out-of-order fields reject at Source/6 formation with `SOURCE6_SHAPE`; an unknown action or effect tag rejects with `SOURCE6_UNKNOWN_TAG`. A different header rejects with `SOURCE6_VERSION`. The closed source grammar also rejects a nonempty failure, observation, disclosure, retained-effect or duty form at formation with `SOURCE6_SHAPE`; `S0_FAILURE_UNSUPPORTED` applies only when a typed Core/5 stage reaches the failure judgment. These source rejections have no Core/5 term or K transition. A local Source/5 parser test confirms that a `/6` header rejects with `PROFILE_MISMATCH`; Core/4-to-Core/5 compatibility has not been implemented or verified here.

The grammar borrows only the token definitions from `lexical.md`; all literal words in the Source/6 EBNF are profile-local reserved words and cannot be identifiers. This does not alter Source/5 keywords. Limits are simultaneous: source UTF-8 bytes ≤65536, tokens ≤8192, AST nodes ≤8192, nesting depth ≤64, identifiers ≤64 ASCII characters, decoded strings ≤1024 UTF-8 bytes, and one stage/effect vector per document. Opaque string fields in this S0 presentation must be nonempty. `scale` is 0..18; each nominal amount (`value`, `fee`, `amount`, caps and floor) is 0..`2^127−1`; each balance, allowance counter and work count is 0..`2^128−1`. S0 uses checked UInt128 intermediates, including `value+fee`, `principal+accrued`, credits and spent counters. The current source/kernel nominal bound does not by itself impose the same bound on every lifecycle state field. This S0 proposal additionally caps principal, accrued and outstanding at `2^127−1` and requires `outstanding=principal+accrued`. W-D4 must review that additional narrowing. Invalid bounds, including an inverted validity interval, reject before K admission with `SOURCE6_RANGE`.

Source/6 formation applies an action-dependent shape check after the closed EBNF parses: transfer requires three balance rows in owner, recipient, fee-recipient order and no obligation; repay requires two rows in payer, bound-creditor order and exactly one obligation. All endpoint identifiers are pairwise distinct for transfer; payer and creditor differ for repay. Duplicate or missing authenticated cells reject `SOURCE6_CELL_SHAPE`; an absent receiver balance is not silently initialized. The allowance owner equals signer. The replay row states whether the selected signed key is unused or consumed; a consumed row rejects at history. The authenticated block is a *claim* until a snapshot-to-head premise establishes each cell and the current head. The strings used for key, nonce, digest and heads are opaque typed identifiers in this syntax, not hash byte definitions.

## Source fields to Core/5

One elaboration proposes `Core5Stage(pre, action, signedScope, submittedEffects, premises)`. The current direct parser preserves agreement ID, asset scale and authenticated predecessor on its AST, but the local Core preparer does not bind those three fields or an exact `/3` digest. It reports the candidate as `PreparedUnqualified`. The Core/5 tags below are proposed typed constructors. No caller Boolean can establish authentication or signature validity.

The local intent object carries the proposed version label `moriarty-intent/3`. That label is a type discriminator only. Source/6 lowering does not encode canonical `/3` bytes or calculate their digest, and no verifier consumes this object as a signed authorization.

| Source/6 field or form | Core/5 field or constructor | Rule |
| --- | --- | --- |
| `profile`, `agreement`, `domain`, `settlement` | Proposed `Version(Source6,Core5)`, `AgreementId`, `DomainId`, `AssetId(scale)` | The parser retains all four. The current local Core preparer does not bind `AgreementId` or scale; these remain named unverified bindings. |
| `selected … source_hash … digest` | Proposed `SelectedProgram(actionId,sourceHash,policyDigest)` | The current Core field named `programId` contains the selected **action ID**, not the agreement ID. Binding all three to a signed statement remains unverified. S0 uses exact action IDs `TransferLiteralFee` and `RepayAccrualFirst`, matching the signed and submitted constructor. Any other action ID rejects `SOURCE6_PROFILE_UNSUPPORTED`. |
| `signer … key`, `nonce`, `pre_head`, `valid` | `SignedScope(signer,keyRef,replayKey,preHead,roundLo,roundHi)` | `replayKey=(domain,signer,nonce)`; the exact digest and verifier are external typed premises. |
| `signed_action` | `SignedAction(TransferLiteralFee | RepayAccrualFirst)` | Fix owner/payer, recipients, fee recipient, obligation ID and quantities under the signature. The submitted action must match this signed action exactly. |
| `gross_cap`, `fee_cap`, `net_floor` | `Bounds(grossCap,feeCap,netFloor)` | Bind all three to the signed scope; authority uses gross debit. |
| `failure success_only` and six explicit empty/none fields | `FailurePolicy(SuccessOnly)`, empty observation, disclosure, retained effect and duty, no delegation or recovery | The current parser rejects a nonempty source variant at formation with `SOURCE6_SHAPE`. A separately constructed typed Core/5 requested outcome reaches `S0_FAILURE_UNSUPPORTED` at Failure. The Source/6 parser cannot express that typed hostile input. No accepted fee-bearing failure exists. |
| `authenticated head`, `predecessor`, `round` | `PreHead`, `Predecessor`, `CurrentRound` | Must be authenticated against the same snapshot; current head comparison is atomic with replay consumption. |
| `balance`, `allowance`, `obligation`, `replay`, `work_remaining`, `work_spent` | `BalanceCell`, `AllowanceCell`, `ObligationCell`, `ReplayCell`, `WorkCell` | These are read cells. Obligation binds debtor, creditor, asset, principal, accrued, outstanding and status. Both work counters come from the authenticated snapshot; lowering never resets spent work. |
| `transfer` | `TransferLiteralFee(owner,recipient,feeRecipient,value,fee)` | Owner=signer. Prepare debit gross, recipient credit value, and fee credit only when fee>0. |
| `repay` | `RepayAccrualFirst(obligationId,payer,amount,IdentityConversion)` | Payer=debtor=signer. Read bound creditor and asset from obligation; do not accept caller supplied substitutes. |
| `effects` | `PreparedEffects` comparison candidate | Order and values must equal the internally derived vector below. The supplied vector does not define the effect. In this provisional text profile, `use_replay` supplies the signed nonce; Core derives the domain/signer/nonce replay key. |
| `post_head` | `PostHead` candidate | Must be a valid authenticated successor under the external head-extension premise. |

For transfer, prepare `Debit(owner,v+f)`, `Credit(recipient,v)`, optional `Credit(feeRecipient,f)` when `f>0`, `UseAllowance(owner,v+f)`, `UseReplay(key)`, `AdvanceHead(pre,post)` in exactly that order. The optional line is present iff `f>0`; a zero-valued fee line rejects. Require `v>0`, `f≤feeCap`, `v+f≤grossCap`, `v≥netFloor`, sufficient owner balance and remaining allowance, and no overflow in any receiver or spent counter. Debit and credits conserve the same nominal asset. Preserve gross effects even though balance changes could be netted. The local wrapper checks Stage first, then requires the transfer in `submit` to equal `signed_action`; it withholds any candidate prepared during that local check when they differ.

For repay, require one Outstanding obligation, `0<n≤outstanding`, matching asset, identity conversion `(mantissa=1,scale=0,rounding=none)`, `feeCap=0`, `netFloor=0`, sufficient payer balance and allowance, and no overflow in the creditor or spent counter. Let `da=min(n,accrued)` and `dp=n−da`. Prepare `Debit(payer,n)`, `Credit(boundCreditor,n)`, `SetObligation(p−dp,a−da,p+a−n,status')`, `UseAllowance(payer,n)`, `UseReplay(key)`, `AdvanceHead(pre,post)`. `status'` is Settled iff outstanding becomes zero. Every unlisted authenticated cell is unchanged. A debt reduction without the creditor credit rejects. The local wrapper checks Stage first, then requires the repayment in `submit` to equal `signed_action`; it withholds any candidate prepared during that local check when they differ.

## Admission and observation

Core/5 applies `stage → intent → effect → authority → history → failure`. `stage` checks version, selected program, typed cells and authentication premises. `intent` checks exact signed scope, validity, endpoints, alias policy and all signed nominal caps and floors. `effect` computes and compares the complete ordered vector and post cells. `authority` checks signer, allowance and work budget. `history` compares the current head and unused replay key and verifies successor binding. `failure` accepts only terminal success with empty retained effects and duties. The first failing judgment returns `(judgment,code,diagnosticWork)` without a published post-state or effects. Within-judgment code spelling and precedence remain W-D3 choices; provisional codes are in `s0-implementation-contract.md`.

The result shape is `Core5Observation(pre,action,signedScope,preparedEffects,post,remainingDuty,remainingWork,replay,preHead,postHead,phase,judgment,code)`. Accepted S0 success contains complete effects, consumption, writes, one post-head and empty duty. Atomic rejection contains the first judgment/code and diagnostic work, with `post`, published effects and post-head absent. Snapshot authentication, exact signature verification, head extension and ledger compare-and-consume are typed external premises; failure or absence rejects. Source/6 grammar acceptance alone does not imply admission.

## Boundary with earlier and later forms

Source/5 `profile`, declarations, `action`, expression and `emit` forms have no automatic injection into this S0 stage. A future migration must map every legacy field, selected Core/4 program, authenticated state, signed digest and complete effect obligation, then prove the old and new observations equivalent on the stated domain. `Repay` in Core/4 consumes a separate Transfer; Core/5's `RepayAccrualFirst` is one funded stage. All old Source/5 and Core/4 behavior remains historical until such a mapping is demonstrated.

The eight MIL/4 first families and all later profiles reject from this S0 grammar with `SOURCE6_PROFILE_UNSUPPORTED` or `SOURCE6_UNKNOWN_TAG` at formation. A later full Source/6 grammar must add typed productions and Core/5 constructors per family. General Φ₁, uncertified Ω, division, rounding, mint, reserve, foreign evidence, accepted failures and recovery are outside S0. This document does not turn those forms into generic records or strings.

```

## experiments/moriarty-language/formal/mil4/s0-implementation-contract.md

```text
# S0 implementation contract, provisional

**Status:** executable prototype contract. It fixes choices for isolated K and Quint work, but does not adopt MIL/4, close W-D0–W-D4, or qualify a native/ledger path. The [semantic contract](semantics-contract.md), [projection](projection.md) and [independent cases](s0-discriminators.md) control the intended behavior.

## Version and input

Use a new `Source/6` and `Core/5` identity. S0's signed intent envelope remains a separate proposed `/3` encoding. A stage carries one selected program, domain, signer, settlement asset, signed nonce, signed pre-head, validity round interval, fixed recipient and fee recipient, gross and fee caps, net floor, and a complete submitted effect vector. Repayment also carries an authenticated obligation with debtor, creditor, settlement asset, principal, accrued, outstanding and status. A K input may use symbolic terms rather than JSON, but it must retain these fields as typed values or explicit premises.

The signer is the debited owner/payer in S0. Delegation is absent. The replay key is `(domain, signer, signed nonce)`. A second intent may repay the same obligation with a distinct nonce and current head. Snapshot-to-head authentication, signature verification of the exact intent digest, and atomic ledger compare-and-consume are named premises. An unavailable premise rejects. A caller Boolean does not establish a premise.

## Provisional Source/6 lowering

The Source/6 parser preserves the agreement ID, selected action ID, settlement scale, authenticated predecessor, signed action, submitted action, and explicit empty failure/observation/duty fields in a typed stage wrapper. The selected `digest` is the policy digest; the `/3` signed-intent digest is not defined by Source/6 and must not be fabricated during lowering. The wrapper checks local Stage shape before reporting a changed `signed_action`/`submit` pair at Intent; it withholds any candidate produced during that local check. `work_remaining` and `work_spent` are both authenticated cells; neither counter may be synthesized as zero. The Source/6 `use_replay` string names the signed nonce in this provisional presentation. Core/5 derives the composite replay key from domain, signer and nonce and compares that complete line. This textual convention remains a W-D3 candidate, not final signed bytes.

## Canonical effects and arithmetic

Transfer requires `v>0`, `f≥0`, `f≤feeCap`, `v+f≤grossCap`, `v≥netFloor`, and balances/allowance sufficient for gross `v+f`. The candidate first profile requires owner, recipient and fee recipient to be pairwise distinct. This explicit narrow-profile rejection avoids ambiguous alias writes until a later alias policy is selected. Ordered effects are `Debit(owner,v+f)`, `Credit(recipient,v)`, then `Credit(feeRecipient,f)` if `f>0`; omit the zero-fee line. Preserve gross debit before deriving net cell deltas. Consume allowance remaining and increase spent by `v+f`.

Repayment requires `0<n≤outstanding`, `outstanding=principal+accrued`, status Outstanding, payer=debtor=signer, creditor from the authenticated obligation, matching settlement asset, and identity conversion. Set `da=min(n,accrued)`, `dp=n−da`, `accrued'=accrued−da`, `principal'=principal−dp`, `outstanding'=principal'+accrued'`. Ordered effects are `Debit(payer,n)`, `Credit(boundCreditor,n)`, `SetObligation(...)`, `UseAllowance(n)`, `UseReplay(key)`, `AdvanceHead(pre,post)`. Transfer has the corresponding allowance, replay and head lines. No impairment substitutes for payment.

Use checked UInt128 for balances, allowances and each intermediate sum. Apply the current source/kernel `2^127−1` bound to S0 nominal amounts and liability caps; do not mislabel every lifecycle state field as signed-width. Subtraction underflow and credit overflow reject. S0 has no division, rounding, reserve, mint or accepted retained-effect failure.

## Judgment order and result

Apply `stage → intent → effect → authority → history → failure` in that order. A candidate Core rejection uses `(firstJudgment, stableCode, diagnosticWork=1)` and publishes no post-state or effects. The one diagnostic unit is an abstract observation, not runtime gas; Source/6 formation errors are outside this count. Exact wire code spelling remains provisional under W-D3. `stage` binds the authenticated balance, allowance and obligation cells required by the signed action, including debtor and creditor identity for repayment. `intent` checks fixed signed scope, validity, positive nominal action and caps/floor before diagnosing a supported direct-Core endpoint alias. `effect` checks numeric range and obligation arithmetic before comparing the complete ordered submitted vector with preparation. `authority` checks signer, allowance and work budget. `history` checks pre-head and replay key. `failure` accepts only terminal success with empty retained effects and duties.

| Condition | First judgment | Provisional code |
| --- | --- | --- |
| Unsupported source/Core/profile or malformed typed state | stage | `S0_STAGE_UNSUPPORTED` |
| Required typed premise unavailable or does not bind the exact intent, state, round and requested outcome | stage | `S0_STAGE_PREMISE` in K and Quint; TypeScript reports it only in the optional local-stipulation comparison and remains unqualified on success |
| Changed signed endpoint, nonce, bounds or invalid validity round | intent | `S0_INTENT_SCOPE` |
| Endpoint alias in this first profile | intent | `S0_INTENT_ALIAS` |
| Submitted vector or derived post-state differs from preparation | effect | `S0_EFFECT_MISMATCH` |
| Insufficient balance, numeric overflow or invalid obligation arithmetic | effect | `S0_EFFECT_RANGE` |
| Authenticated obligation debtor differs from signed signer/payer | stage | `S0_STAGE_UNSUPPORTED` |
| Submitted payer differs from the signed payer | intent | `S0_INTENT_SCOPE` |
| Allowance or work budget exceeded after typed payer binding | authority | `S0_AUTH_SCOPE` |
| Stale pre-head, consumed replay key, or wrong stipulated successor | history | `S0_HISTORY_STALE`, `S0_HISTORY_REPLAY`, or `S0_HISTORY_SUCCESSOR` |
| Nonempty retained effect/duty or unselected failure branch | failure | `S0_FAILURE_UNSUPPORTED` |

This table is an implementation discriminator, not an accepted canonical diagnostic schedule. Compile and typecheck results may establish syntax only. Semantic traces, proofs, native qualification and ledger readback remain separate evidence.

```

## experiments/moriarty-language/formal/mil4/projection.md

```text
# K to Quint projection candidate

**Status:** revised design map after nine-seat Sprint 0 review and finite S0 witness execution. K and Quint have executed fixed local transfer, repayment and hostile cases. No ledger-committed K stage or complete K-to-Quint projection has run. The projection's committed domain remains empty.

## Domain and action grain

Define `α` on **ledger-committed K stages** whose authenticated pre-state, prepared effects and atomic head/replay consumption match one signed statement. K-local preparation alone is outside that domain. The intended initial relation is `α(initK) ⊆ initQ`. For each committed stage, require `α(preK)=preQ`, a matching enabled Quint commit action, `α(postK)=postQ`, and equality of the complete ordered effect and consumption records. A finite comparison is trace-correspondence evidence for that corpus, not a universal refinement proof.

The executed S0 corpus uses a narrower local observation relation `α_local`. Three separately written common positive fixtures align numeric balance, obligation, allowance, work and round inputs. Their literal complete pre/effect/post outputs agree after an explicit identifier map; the reviewed packet uses a manual map and has no executable general adapter. The older non-common K, Quint and TypeScript fixtures still use different initial allowance, work and round values. Hostile overlap compares first judgment and code in the current witnesses. Source formation failures, unverified signed-wire fields and external authentication premises are outside this local relation. The finite comparison and its limits are recorded in `deliverables/mil4-k-quint-sprint1-2026-09-29/S1-FINITE-CORRESPONDENCE.md`.

For these single-asset fixtures the local map supplies asset `A` to Quint money and allowance lines, whose constructors omit it; K debit, credit and allowance lines carry `A`, while TypeScript debit and credit carry `A` but `UseAllowance` omits it. Quint's obligation value carries `A`; K and TypeScript `SetObligation` lines omit it. Replay is a K typed key, Quint triple and TypeScript serialized triple. Round is adjacent to K state, inside TypeScript state and also in its stipulated tuple, and outside Quint's financial snapshot. These differences must be checked explicitly by any executable adapter. A requested pending phase is also outside the common S0 failure domain: Quint currently classifies it at Failure, TypeScript at Stage, and K has no such phase constructor.

Keep `sign(intent)` and `submit(intentId, fill, claimedPreHead, submittedEffects)` as distinct abstract actions. Signing stores immutable version, program, signer, payer/owner, debtor/creditor, recipients, fee beneficiary, caps, validity interval, failure branch and evidence choice. Submission checks that the fill narrows the signed scope, the claimed pre-head is current, the intent has not been consumed, and time is within the signed interval. One successful submission is an atomic K stage and head update. A stale competing proposal remains representable as a disabled commit with a semantic rejection observation. Do not create a no-op successor solely to model rejection.

## State and observation mapping

| K field or result | Quint image | Required distinction |
| --- | --- | --- |
| Version, selected Core program, signed template and resolved fill | Immutable intent table and selected fill | Keep every fixed endpoint, gross/fee/net bound, failure branch and policy digest. Compare supplied fields with signed fields. |
| Authenticated pre/post cells and derived footprint | Typed maps keyed by domain, asset, account, obligation, episode or claim | Keep each read/write value. Preserve nominal identities and absent-versus-present cells. |
| Complete ordered effect vector | Ordered typed lines **and** derived cell deltas | Keep line direction and gross debit before netting aliases. Track charged fees and their beneficiary. Compare zero-line policy explicitly. |
| Balances, custody, supply and aggregate cells | Per-domain asset/account balances, episode custody, supply and aggregate state | Enforce local conservation after alias resolution. Cross-domain backing is a separate qualified premise. |
| Obligation | Principal, accrued, outstanding, status, settlement asset, authenticated debtor and creditor | Only credit to that obligation's bound creditor can discharge debt. A caller cannot substitute a creditor. |
| Signer, allowance, grant, budget and work | Owner-keyed remaining **and spent** counters, grant epoch, gross use, remaining work and closure reserve | Tie the debited payer to the signed signer or an explicit grant. Refund does not restore gross use. |
| Predecessor/head, replay, receipt and nullifier | Opaque current head and consumed ID sets | Replay ID is the signed intent nonce/digest scoped by domain and signer. Separate legitimate partial repayments use distinct IDs. K authenticates commitments. |
| Validity, observation round and policy epoch | Finite abstract round or epoch | A fresh but nonselected older observation is distinct from the selected current round. Time alone never proves foreign nonreceipt. |
| Local rejection | `(judgment, stableCode, noCommit)` observation | Preserve the first failing semantic judgment and code for the comparison corpus. No state/effect successor exists. Lexical failures remain outside Quint's action model. |
| Accepted signed failure, pending or unknown | Separate committed phase observation with retained effects, custody, work and duties | These are not local rejections. Keep their authenticated successor state and continuing duty. |

The comparison adapter may compute a no-commit result for a disabled action. It must retain the K judgment and code; a generic disabled action is insufficient for rejection-precedence comparison. The S0 failure policy permits atomic local rejection and terminal success only. Later signed phase failures, escrow pending and bridge unknown have distinct transitions and observations.

## S0 model shape

Use one owner/signer and explicit signed recipient and fee recipient for transfer. The commit action debits gross `v+f`, credits `v` and `f`, consumes allowance remaining by `v+f`, increases allowance spent by `v+f`, consumes the signed replay ID and advances the head. Keep all three effect lines before alias resolution. The exact self-transfer and zero-fee canonical policies remain open W-D3 leaves; the model must reject or represent them explicitly.

Repayment reads an obligation that binds debtor, creditor, settlement asset, principal, accrued, outstanding and status. It debits the signed payer and credits that bound creditor by `n`, consumes gross allowance and replay, and applies AccrualFirst with identity conversion. The model checks `0<n≤outstanding`, `outstanding=principal+accrued`, the first-slice signed nominal bound, UInt128 balances and counters, and exact post status. Debt cannot fall if the matched creditor credit is absent.

Quint integers do not overflow. Guard every balance and counter at `2^128−1`, and every first-slice nominal amount and liability cap at `2^127−1`. The current lifecycle state fields are UInt128, while its source/kernel nominal admission applies the signed bound. Record those different domains rather than saying all obligation fields have the same wire cap.

## Limits and work order

The present K surge's `M4T/1` escrow state is not S0 transfer or repayment state. `M4D` family arithmetic and `M4E` effect shapes are disconnected from its six-judgment admission. `m4AdmitFamily` rejects every first family. Keep `α` undefined for those projections. A Quint model of their arithmetic is an independent design experiment, not K admission.

Extract `common.qnt` after the signed scope, replay namespace, result observations and state types are selected. The isolated `s0.qnt` is a typechecked, fixed-witness prototype. Three common positive fixtures compare complete literal financial observations; the hostile corpus covers gross debit, creditor binding, recipient substitution, stale head, replay and overflow on its stated overlapping domain. The separate head, failure and premise experiment has executed finite witnesses under stipulated tuples and remains provisional. A complete general comparison still needs one canonical input lowered through all artifacts and an executable general projection. No `quint verify` model check has run. Signature truth, native proof validity, oracle provenance and foreign finality remain named external premises.

```

## experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts

```text
/** Provisional, closed Source/6 S0 presentation parser. No authentication occurs here. */
import {
  MIL4_S0_CORE, MIL4_S0_INTENT, MIL4_S0_SOURCE,
  type S0Effect, type S0Intent, type S0State,
} from './mil4-s0-core-v5.ts';

const U128 = (1n << 128n) - 1n;
const S128 = (1n << 127n) - 1n;
const encoder = new TextEncoder();
const RESERVED = new Set((
  'profile agreement unit party asset const state action requires let next emit ensures true false not and or domain settlement scale selected source_hash digest intent signer key nonce pre_head post_head valid gross_cap fee_cap net_floor failure success_only signed_action observations empty disclosures retained_effects retained_duties delegation none recovery authenticated head predecessor round balance allowance remaining spent obligation debtor creditor principal accrued outstanding settled status replay unused consumed work_remaining work_spent submit transfer from to fee_to value fee repay payer amount conversion identity effects debit credit set_obligation use_allowance use_replay advance_head'
).split(' '));

type TokenKind = 'word' | 'integer' | 'string' | 'punctuation' | 'eof';
interface Token { kind: TokenKind; text: string; value: string; start: number; end: number }
export class Source6Error extends Error {
  readonly code: string;
  readonly offset: number;
  constructor(code: string, offset: number, message: string) {
    super(message);
    this.name = 'Source6Error';
    this.code = code;
    this.offset = offset;
  }
}
function fail(code: string, offset: number, message: string): never {
  throw new Source6Error(code, offset, message);
}
function isSurrogate(value: number): boolean { return value >= 0xd800 && value <= 0xdfff; }
function scalarString(value: string, offset: number): void {
  for (let i = 0; i < value.length; i++) {
    const c = value.charCodeAt(i);
    if (c >= 0xd800 && c <= 0xdbff && i + 1 < value.length) {
      const low = value.charCodeAt(i + 1);
      if (low >= 0xdc00 && low <= 0xdfff) { i++; continue; }
    }
    if (isSurrogate(c)) fail('INVALID_SURROGATE', offset + encoder.encode(value.slice(0, i)).length, 'Lone UTF-16 surrogate');
  }
}
function lexical(source: string): Token[] {
  scalarString(source, 0);
  if (encoder.encode(source).length > 65536) fail('SOURCE_BOUND', 0, 'Source exceeds 65536 UTF-8 bytes');
  const tokens: Token[] = [];
  let i = 0;
  let byte = 0;
  const advance = (end: number): void => { byte += encoder.encode(source.slice(i, end)).length; i = end; };
  const emit = (kind: TokenKind, end: number, value = source.slice(i, end)): void => {
    if (tokens.length >= 8191) fail('TOKEN_BOUND', byte, 'Too many tokens');
    const start = byte;
    const raw = source.slice(i, end);
    advance(end);
    tokens.push({ kind, text: raw, value, start, end: byte });
  };
  while (i < source.length) {
    const c = source[i];
    if (/[ \t\r\n]/.test(c)) { advance(i + 1); continue; }
    if (source.startsWith('//', i)) {
      const next = source.indexOf('\n', i + 2);
      advance(next < 0 ? source.length : next); continue;
    }
    if (source.startsWith('/*', i)) {
      const next = source.indexOf('*/', i + 2);
      if (next < 0) fail('UNTERMINATED_COMMENT', byte, 'Unclosed block comment');
      advance(next + 2); continue;
    }
    if (c === '"') {
      let end = i + 1;
      let escaped = false;
      for (; end < source.length; end++) {
        const x = source[end];
        if (x === '"' && !escaped) { end++; break; }
        if (x === '\\' && !escaped) escaped = true;
        else escaped = false;
      }
      const raw = source.slice(i, end);
      if (!raw.endsWith('"') || raw.length < 2) fail('INVALID_STRING', byte, 'Unclosed string');
      let value: string;
      try { value = JSON.parse(raw) as string; }
      catch { fail('INVALID_STRING', byte, 'Invalid JSON string'); }
      scalarString(value, byte);
      if (encoder.encode(value).length > 1024) fail('STRING_BOUND', byte, 'Decoded string exceeds 1024 UTF-8 bytes');
      emit('string', end, value); continue;
    }
    if (/[0-9]/.test(c)) {
      let end = i + 1;
      while (end < source.length && /[0-9]/.test(source[end])) end++;
      const raw = source.slice(i, end);
      if (raw.length > 78) fail('INTEGER_BOUND', byte, 'Integer exceeds 78 digits');
      if (!/^(0|[1-9][0-9]*)$/.test(raw)) fail('INVALID_INTEGER', byte, 'Noncanonical integer');
      emit('integer', end); continue;
    }
    if (/[A-Za-z]/.test(c)) {
      let end = i + 1;
      while (end < source.length && /[A-Za-z0-9_]/.test(source[end])) end++;
      const next = source.codePointAt(end);
      if (next !== undefined && /[\p{L}\p{N}\p{Pc}\p{Mn}\p{Mc}]/u.test(String.fromCodePoint(next)))
        fail('NON_ASCII_IDENTIFIER', byte, 'Non-ASCII identifier');
      if (end - i > 64) fail('IDENTIFIER_BOUND', byte, 'Identifier exceeds 64 characters');
      emit('word', end); continue;
    }
    const point = source.codePointAt(i)!;
    if (/[\p{L}\p{N}\p{Pc}\p{Mn}\p{Mc}]/u.test(String.fromCodePoint(point)))
      fail('NON_ASCII_IDENTIFIER', byte, 'Non-ASCII identifier');
    if (source.startsWith('..', i)) { emit('punctuation', i + 2); continue; }
    if ('{};'.includes(c)) { emit('punctuation', i + 1); continue; }
    fail('UNEXPECTED_CHAR', byte, 'Unexpected source character');
  }
  tokens.push({ kind: 'eof', text: '', value: '', start: byte, end: byte });
  return tokens;
}

export type Source6Action =
  | { kind: 'Transfer'; from: string; to: string; feeTo: string; value: string; fee: string }
  | { kind: 'Repay'; obligation: string; payer: string; amount: string; conversion: 'identity' };
export interface Source6Obligation {
  id: string; debtor: string; creditor: string; asset: string;
  principal: string; accrued: string; outstanding: string; status: 'Outstanding';
}
export interface Source6Ast {
  profile: typeof MIL4_S0_SOURCE; programId: string; domain: string;
  settlement: { asset: string; scale: string };
  selected: { actionId: string; sourceHash: string; policyDigest: string };
  intent: {
    signer: string; keyRef: string; nonce: string; preHead: string;
    notBefore: string; notAfter: string; grossCap: string; feeCap: string; netFloor: string;
    signedAction: Source6Action; failure: 'success_only';
    observations: 'empty'; disclosures: 'empty'; retainedEffects: 'empty'; retainedDuties: 'empty';
    delegation: 'none'; recovery: 'none';
  };
  authenticated: {
    head: string; predecessor: string; round: string;
    balances: { account: string; amount: string }[];
    allowance: { owner: string; remaining: string; spent: string };
    obligation?: Source6Obligation; replay: 'unused' | 'consumed';
    workRemaining: string; workSpent: string;
  };
  submitted: { action: Source6Action; effects: S0Effect[]; postHead: string };
}

class Parser {
  private index = 0;
  private nodes = 0;
  private depth = 0;
  private readonly tokens: Token[];
  constructor(tokens: Token[]) { this.tokens = tokens; }
  private get here(): Token { return this.tokens[this.index]; }
  private node(): void {
    if (++this.nodes > 8192) fail('AST_BOUND', this.here.start, 'Too many AST nodes');
  }
  private enter(): void { if (++this.depth > 64) fail('DEPTH_BOUND', this.here.start, 'Nesting exceeds 64'); }
  private leave(): void { this.depth--; }
  private take(word: string, code = 'SOURCE6_SHAPE'): void {
    if (this.here.text !== word) fail(code, this.here.start, `Expected ${word}`);
    this.index++;
  }
  private effectTag(word: string): void {
    if (this.here.text !== word && this.here.kind === 'word'
        && !new Set(['debit', 'credit', 'set_obligation', 'use_allowance', 'use_replay', 'advance_head']).has(this.here.text))
      fail('SOURCE6_UNKNOWN_TAG', this.here.start, 'Unknown effect tag');
    this.take(word);
  }
  private id(): string {
    const token = this.here;
    if (token.kind !== 'word' || RESERVED.has(token.text)) fail('SOURCE6_SHAPE', token.start, 'Expected identifier');
    this.index++; return token.value;
  }
  private string(): string {
    const token = this.here;
    if (token.kind !== 'string') fail('SOURCE6_SHAPE', token.start, 'Expected string');
    if (token.value.length === 0) fail('SOURCE6_SHAPE', token.start, 'Empty opaque string');
    this.index++; return token.value;
  }
  private uint(max: bigint = U128): string {
    const token = this.here;
    if (token.kind !== 'integer') fail('SOURCE6_SHAPE', token.start, 'Expected integer');
    this.index++;
    if (BigInt(token.value) > max) fail('SOURCE6_RANGE', token.start, 'Integer exceeds nominal bound');
    return token.value;
  }
  private action(): Source6Action {
    this.node();
    if (this.here.text === 'transfer') {
      this.take('transfer'); this.take('from'); const from = this.id();
      this.take('to'); const to = this.id(); this.take('fee_to'); const feeTo = this.id();
      this.take('value'); const value = this.uint(S128); this.take('fee'); const fee = this.uint(S128);
      this.take(';'); return { kind: 'Transfer', from, to, feeTo, value, fee };
    }
    if (this.here.text === 'repay') {
      this.take('repay'); this.take('obligation'); const obligation = this.id();
      this.take('payer'); const payer = this.id(); this.take('amount'); const amount = this.uint(S128);
      this.take('conversion'); this.take('identity'); this.take(';');
      return { kind: 'Repay', obligation, payer, amount, conversion: 'identity' };
    }
    fail('SOURCE6_UNKNOWN_TAG', this.here.start, 'Unknown action');
  }
  private effects(action: Source6Action, asset: string): S0Effect[] {
    this.node(); this.enter(); this.take('effects'); this.take('{');
    const debit = (): S0Effect => { this.effectTag('debit'); const account = this.id(); const amount = this.uint(); this.take(';'); return { kind: 'Debit', account, asset, amount }; };
    const credit = (): S0Effect => { this.effectTag('credit'); const account = this.id(); const amount = this.uint(); this.take(';'); return { kind: 'Credit', account, asset, amount }; };
    const result: S0Effect[] = [debit(), credit()];
    if (action.kind === 'Transfer' && this.here.text === 'credit') result.push(credit());
    if (action.kind === 'Repay') {
      this.effectTag('set_obligation'); const id = this.id(); this.take('principal'); const principal = this.uint(S128);
      this.take('accrued'); const accrued = this.uint(S128); this.take('outstanding'); const outstanding = this.uint(S128);
      this.take('status');
      if (this.here.text !== 'outstanding' && this.here.text !== 'settled')
        fail('SOURCE6_SHAPE', this.here.start, 'Expected obligation status');
      const status = this.here.text === 'settled' ? 'Settled' : 'Outstanding'; this.index++; this.take(';');
      result.push({ kind: 'SetObligation', id, principal, accrued, outstanding, status });
    }
    this.effectTag('use_allowance'); const owner = this.id(); const amount = this.uint(); this.take(';');
    result.push({ kind: 'UseAllowance', owner, amount });
    this.effectTag('use_replay'); const key = this.string(); this.take(';'); result.push({ kind: 'UseReplay', key });
    this.effectTag('advance_head'); const predecessor = this.string(); const successor = this.string(); this.take(';');
    result.push({ kind: 'AdvanceHead', predecessor, successor }); this.take('}'); this.leave();
    return result;
  }
  parse(): Source6Ast {
    this.node(); this.take('profile', 'SOURCE6_VERSION');
    const profileToken = this.here;
    if (profileToken.kind !== 'string' || profileToken.text !== '"moriarty-financial-agreement-source/6"')
      fail('SOURCE6_VERSION', profileToken.start, 'Unsupported source profile');
    this.index++;
    this.take(';'); this.take('agreement'); const programId = this.id(); this.take('{'); this.enter();
    this.take('domain'); const domain = this.id(); this.take(';');
    this.take('settlement'); const asset = this.id(); this.take('scale'); const scale = this.uint(18n); this.take(';');
    this.take('selected'); const actionId = this.id(); this.take('source_hash'); const sourceHash = this.string();
    this.take('digest'); const policyDigest = this.string(); this.take(';');
    this.take('intent'); this.take('{'); this.enter(); this.node();
    this.take('signer'); const signer = this.id(); this.take('key'); const keyRef = this.string(); this.take(';');
    this.take('nonce'); const nonce = this.string(); this.take(';');
    this.take('pre_head'); const preHead = this.string(); this.take(';');
    this.take('valid'); const notBefore = this.uint(); this.take('..'); const notAfter = this.uint(); this.take(';');
    if (BigInt(notBefore) > BigInt(notAfter))
      fail('SOURCE6_RANGE', this.here.start, 'Validity lower bound exceeds upper bound');
    this.take('gross_cap'); const grossCap = this.uint(S128); this.take(';');
    this.take('fee_cap'); const feeCap = this.uint(S128); this.take(';');
    this.take('net_floor'); const netFloor = this.uint(S128); this.take(';');
    this.take('failure'); this.take('success_only'); this.take(';'); this.take('signed_action');
    const signedAction = this.action();
    for (const field of ['observations', 'disclosures', 'retained_effects', 'retained_duties']) {
      this.take(field); this.take('empty'); this.take(';');
    }
    this.take('delegation'); this.take('none'); this.take(';');
    this.take('recovery'); this.take('none'); this.take(';'); this.take('}'); this.leave();
    this.take('authenticated'); this.take('{'); this.enter(); this.node();
    this.take('head'); const head = this.string(); this.take(';');
    this.take('predecessor'); const predecessor = this.string(); this.take(';');
    this.take('round'); const round = this.uint(); this.take(';');
    const balances: { account: string; amount: string }[] = [];
    const balanceCount = signedAction.kind === 'Transfer' ? 3 : 2;
    for (let i = 0; i < balanceCount; i++) {
      this.take('balance', 'SOURCE6_CELL_SHAPE');
      balances.push({ account: this.id(), amount: this.uint() }); this.take(';');
    }
    if (this.here.text === 'balance')
      fail('SOURCE6_CELL_SHAPE', this.here.start, 'Too many balance rows');
    this.take('allowance'); const owner = this.id(); this.take('remaining'); const remaining = this.uint();
    this.take('spent'); const spent = this.uint(); this.take(';');
    let obligation: Source6Obligation | undefined;
    if (signedAction.kind === 'Repay') {
      this.take('obligation', 'SOURCE6_CELL_SHAPE'); const id = this.id(); this.take('{'); this.enter(); this.node();
      this.take('debtor'); const debtor = this.id(); this.take(';');
      this.take('creditor'); const creditor = this.id(); this.take(';');
      this.take('asset'); const obligationAsset = this.id(); this.take(';');
      this.take('principal'); const principal = this.uint(S128); this.take(';');
      this.take('accrued'); const accrued = this.uint(S128); this.take(';');
      this.take('outstanding'); const outstanding = this.uint(S128); this.take(';');
      this.take('status'); this.take('outstanding'); this.take(';'); this.take('}'); this.leave();
      obligation = { id, debtor, creditor, asset: obligationAsset, principal, accrued, outstanding, status: 'Outstanding' };
    }
    if (this.here.text === 'obligation')
      fail('SOURCE6_CELL_SHAPE', this.here.start, 'Unexpected obligation row');
    this.take('replay');
    if (this.here.text !== 'unused' && this.here.text !== 'consumed')
      fail('SOURCE6_SHAPE', this.here.start, 'Expected replay status');
    const replay = this.here.text as 'unused' | 'consumed'; this.index++; this.take(';');
    this.take('work_remaining'); const workRemaining = this.uint(); this.take(';');
    this.take('work_spent'); const workSpent = this.uint(); this.take(';'); this.take('}'); this.leave();
    this.take('submit'); const action = this.action(); const effects = this.effects(action, asset);
    this.take('post_head'); const postHead = this.string(); this.take(';'); this.take('}'); this.leave();
    if (this.tokens[this.index].kind !== 'eof') fail('SOURCE6_SHAPE', this.here.start, 'Trailing source');
    const expectedActionId = signedAction.kind === 'Transfer' ? 'TransferLiteralFee' : 'RepayAccrualFirst';
    if (actionId !== expectedActionId)
      fail('SOURCE6_PROFILE_UNSUPPORTED', this.here.start, 'Selected action is outside the S0 profile');
    const accounts = balances.map((row) => row.account);
    const expectedAccounts = signedAction.kind === 'Transfer'
      ? [signedAction.from, signedAction.to, signedAction.feeTo]
      : [signedAction.payer, obligation?.creditor];
    if (accounts.some((id, i) => id !== expectedAccounts[i]) || new Set(accounts).size !== accounts.length
        || owner !== signer || (signedAction.kind === 'Transfer' &&
          (signedAction.from !== signer || new Set([signedAction.from, signedAction.to, signedAction.feeTo]).size !== 3))
        || (signedAction.kind === 'Repay' && (!obligation || obligation.id !== signedAction.obligation
          || signedAction.payer !== signer || obligation.debtor !== signer || obligation.asset !== asset
          || obligation.creditor === signer)))
      fail('SOURCE6_CELL_SHAPE', this.here.start, 'Authenticated cells do not match action');
    if (BigInt(workRemaining) + BigInt(workSpent) > U128
        || BigInt(remaining) + BigInt(spent) > U128)
      fail('SOURCE6_RANGE', this.here.start, 'Counter total exceeds UInt128');
    if (obligation && BigInt(obligation.principal) + BigInt(obligation.accrued) !== BigInt(obligation.outstanding))
      fail('SOURCE6_RANGE', this.here.start, 'Obligation outstanding must equal principal plus accrued');
    return {
      profile: MIL4_S0_SOURCE, programId, domain, settlement: { asset, scale },
      selected: { actionId, sourceHash, policyDigest },
      intent: { signer, keyRef, nonce, preHead, notBefore, notAfter, grossCap, feeCap, netFloor,
        signedAction, failure: 'success_only', observations: 'empty', disclosures: 'empty',
        retainedEffects: 'empty', retainedDuties: 'empty', delegation: 'none', recovery: 'none' },
      authenticated: { head, predecessor, round, balances, allowance: { owner, remaining, spent },
        obligation, replay, workRemaining, workSpent },
      submitted: { action, effects, postHead },
    };
  }
}

/** Parse one exact Source/6 S0 document. Throws Source6Error on formation failure. */
export function parseSource6(source: string): Source6Ast { return new Parser(lexical(source)).parse(); }

export interface Source6Lowered {
  ast: Source6Ast; state: S0State; intent: S0Intent;
  submittedEffects: S0Effect[]; proposedPostHead: string;
}

/** Lower claims for local Core/5 preparation; no signed digest or external premise is manufactured. */
export function lowerSource6(ast: Source6Ast): Source6Lowered {
  const { authenticated: auth, intent: signed, submitted, settlement, selected } = ast;
  const replayKey = JSON.stringify([ast.domain, signed.signer, signed.nonce]);
  const state: S0State = {
    core: MIL4_S0_CORE, domain: ast.domain, asset: settlement.asset, head: auth.head,
    round: auth.round, workRemaining: auth.workRemaining, workSpent: auth.workSpent,
    balances: auth.balances.map((row) => ({ ...row })), allowances: [{ ...auth.allowance }],
    obligations: auth.obligation ? [{ ...auth.obligation }] : [],
    consumedReplay: auth.replay === 'consumed' ? [replayKey] : [],
  };
  const base = {
    version: MIL4_S0_INTENT, core: MIL4_S0_CORE, sourceProfile: MIL4_S0_SOURCE,
    programId: selected.actionId, sourceHash: selected.sourceHash, policyDigest: selected.policyDigest,
    keyRef: signed.keyRef, domain: ast.domain, asset: settlement.asset,
    signer: signed.signer, nonce: signed.nonce, preHead: signed.preHead,
    notBefore: signed.notBefore, notAfter: signed.notAfter,
    grossCap: signed.grossCap, feeCap: signed.feeCap, netFloor: signed.netFloor,
  };
  const action = signed.signedAction;
  const intent: S0Intent = action.kind === 'Transfer'
    ? { ...base, kind: 'Transfer', recipient: action.to, feeRecipient: action.feeTo,
        amount: action.value, fee: action.fee }
    : { ...base, kind: 'Repay', obligationId: action.obligation, amount: action.amount };
  // Source `use_replay` names a nonce. Core/5 compares its domain/signer/nonce tuple.
  const submittedEffects = submitted.effects.map((effect): S0Effect =>
    effect.kind === 'UseReplay'
      ? { kind: 'UseReplay', key: JSON.stringify([ast.domain, signed.signer, effect.key]) }
      : { ...effect });
  return { ast, state, intent, submittedEffects, proposedPostHead: submitted.postHead };
}

export function parseAndLowerSource6(source: string): Source6Lowered {
  return lowerSource6(parseSource6(source));
}

```

## experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts

```text
/** Provisional MIL/4 S0 preparation. This module never returns ledger admission. */

export const MIL4_S0_CORE = 'moriarty-core/5' as const;
export const MIL4_S0_INTENT = 'moriarty-intent/3' as const;
export const MIL4_S0_SOURCE = 'moriarty-financial-agreement-source/6' as const;
const U128 = (1n << 128n) - 1n;
const S128 = (1n << 127n) - 1n;
const IDENTIFIER = /^[A-Za-z][A-Za-z0-9._-]{0,63}$/;

export interface S0Balance { account: string; amount: string }
export interface S0Allowance { owner: string; remaining: string; spent: string }
export interface S0Obligation {
  id: string; debtor: string; creditor: string; asset: string;
  principal: string; accrued: string; outstanding: string;
  status: 'Outstanding' | 'Settled';
}
export interface S0State {
  core: typeof MIL4_S0_CORE; domain: string; asset: string;
  head: string; round: string; workRemaining: string; workSpent: string;
  balances: S0Balance[]; allowances: S0Allowance[];
  obligations: S0Obligation[]; consumedReplay: string[];
}
interface S0IntentBase {
  version: typeof MIL4_S0_INTENT; core: typeof MIL4_S0_CORE;
  sourceProfile: typeof MIL4_S0_SOURCE; programId: string;
  sourceHash: string; policyDigest: string; signedDigest?: string; keyRef: string;
  domain: string; asset: string; signer: string; nonce: string;
  preHead: string; notBefore: string; notAfter: string;
  grossCap: string; feeCap: string; netFloor: string;
}
export interface S0TransferIntent extends S0IntentBase {
  kind: 'Transfer'; recipient: string; feeRecipient: string;
  amount: string; fee: string;
}
export interface S0RepayIntent extends S0IntentBase {
  kind: 'Repay'; obligationId: string; amount: string;
}
export type S0Intent = S0TransferIntent | S0RepayIntent;

export type S0Effect =
  | { kind: 'Debit'; account: string; asset: string; amount: string }
  | { kind: 'Credit'; account: string; asset: string; amount: string }
  | { kind: 'SetObligation'; id: string; principal: string; accrued: string; outstanding: string; status: 'Outstanding' | 'Settled' }
  | { kind: 'UseAllowance'; owner: string; amount: string }
  | { kind: 'UseReplay'; key: string }
  | { kind: 'AdvanceHead'; predecessor: string; successor: string };

export type S0Judgment = 'stage' | 'intent' | 'effect' | 'authority' | 'history' | 'failure';
export interface S0Rejected {
  status: 'Rejected'; judgment: S0Judgment; code: string;
  diagnosticWork: 1; publishedPost: null; publishedEffects: null;
}
export interface S0PreparedUnqualified {
  status: 'PreparedUnqualified'; core: typeof MIL4_S0_CORE;
  preHead: string; effects: S0Effect[]; candidatePost: S0State;
  requiredPremises: readonly ['canonical-intent-signature', 'snapshot-to-head', 'head-extension', 'atomic-ledger-compare-and-consume'];
}
export type S0Result = S0Rejected | S0PreparedUnqualified;
export interface S0RequestedOutcome {
  phase: 'TerminalSuccess' | 'RequestedFailure';
  retainedEffects: unknown[];
  retainedDuties: unknown[];
}
/** An experiment assumption. Possession of this tuple does not authenticate it. */
export interface S0LocalStipulation {
  intent: S0Intent; state: S0State; round: string; expectedSuccessor: string;
  requestedOutcome: S0RequestedOutcome;
}
const TERMINAL_SUCCESS: S0RequestedOutcome = {
  phase: 'TerminalSuccess', retainedEffects: [], retainedDuties: [],
};

function reject(judgment: S0Judgment, code: string): S0Rejected {
  return { status: 'Rejected', judgment, code, diagnosticWork: 1, publishedPost: null, publishedEffects: null };
}
function id(value: unknown): value is string {
  return typeof value === 'string' && IDENTIFIER.test(value);
}
function opaque(value: unknown): value is string {
  return typeof value === 'string' && value.length > 0 && value.length <= 1024
    && !/[\u0000-\u001f\u007f]/.test(value);
}
function uint(value: unknown, max: bigint = U128): bigint | null {
  if (typeof value !== 'string' || !/^(0|[1-9][0-9]*)$/.test(value)) return null;
  const parsed = BigInt(value);
  return parsed <= max ? parsed : null;
}
function distinct<T>(values: T[]): boolean { return new Set(values).size === values.length; }
function stableValue(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(stableValue);
  if (value !== null && typeof value === 'object') {
    const record = value as Record<string, unknown>;
    return Object.fromEntries(Object.keys(record).sort().map((key) => [key, stableValue(record[key])]));
  }
  return value;
}
function sameValue(left: unknown, right: unknown): boolean {
  return JSON.stringify(stableValue(left)) === JSON.stringify(stableValue(right));
}
function replayId(value: unknown): value is string {
  if (typeof value !== 'string') return false;
  try {
    const parts: unknown = JSON.parse(value);
    return Array.isArray(parts) && parts.length === 3
      && id(parts[0]) && id(parts[1]) && opaque(parts[2])
      && JSON.stringify(parts) === value;
  } catch { return false; }
}
function boundedAdd(a: bigint, b: bigint): bigint | null {
  const sum = a + b;
  return sum <= U128 ? sum : null;
}
function sameEffects(expected: S0Effect[], supplied: unknown): boolean {
  if (!Array.isArray(supplied) || supplied.length !== expected.length) return false;
  return expected.every((line, index) => {
    const got = supplied[index];
    if (got === null || typeof got !== 'object' || Array.isArray(got)) return false;
    const a = line as unknown as Record<string, unknown>;
    const b = got as Record<string, unknown>;
    return Object.keys(a).length === Object.keys(b).length
      && Object.keys(a).every((key) => JSON.stringify(a[key]) === JSON.stringify(b[key]));
  });
}

/**
 * Prepare a complete local candidate against supplied state.
 * The state, signature and ledger head are not authenticated by this function.
 * A localStipulation is a finite-comparison assumption supplied by the caller;
 * matching it does not authenticate any external fact or qualify the result.
 */
export function prepareMil4S0(
  state: S0State,
  intent: S0Intent,
  submittedEffects: unknown,
  proposedPostHead: string,
  requestedOutcome: S0RequestedOutcome = TERMINAL_SUCCESS,
  localStipulation?: S0LocalStipulation | null,
): S0Result {
  if (state?.core !== MIL4_S0_CORE || intent?.core !== MIL4_S0_CORE
      || intent?.version !== MIL4_S0_INTENT || intent?.sourceProfile !== MIL4_S0_SOURCE
      || (intent?.kind !== 'Transfer' && intent?.kind !== 'Repay')
      || (intent?.kind === 'Transfer' && intent?.programId !== 'TransferLiteralFee')
      || (intent?.kind === 'Repay' && intent?.programId !== 'RepayAccrualFirst')
      || !id(state.domain) || !id(state.asset)
      || !opaque(state.head) || !id(intent.domain) || !id(intent.asset)
      || !Array.isArray(state.balances) || !Array.isArray(state.allowances)
      || !Array.isArray(state.obligations) || !Array.isArray(state.consumedReplay)
      || uint(state.round) === null || uint(state.workRemaining) === null
      || uint(state.workSpent) === null
      || !requestedOutcome || typeof requestedOutcome !== 'object'
      || !['TerminalSuccess', 'RequestedFailure'].includes(requestedOutcome.phase)
      || !Array.isArray(requestedOutcome.retainedEffects)
      || !Array.isArray(requestedOutcome.retainedDuties)
      || boundedAdd(BigInt(state.workRemaining), BigInt(state.workSpent)) === null) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (state.balances.some((v) => !v || typeof v !== 'object')
      || state.allowances.some((v) => !v || typeof v !== 'object')
      || state.obligations.some((v) => !v || typeof v !== 'object')
      || !distinct(state.balances.map((v) => v.account))
      || !distinct(state.allowances.map((v) => v.owner))
      || !distinct(state.obligations.map((v) => v.id))
      || !distinct(state.consumedReplay)
      || state.balances.some((v) => !id(v.account) || uint(v.amount) === null)
      || state.allowances.some((v) => !id(v.owner) || uint(v.remaining) === null || uint(v.spent) === null
        || boundedAdd(BigInt(v.remaining), BigInt(v.spent)) === null)
      || state.obligations.some((v) => !id(v.id) || !id(v.debtor) || !id(v.creditor) || !id(v.asset)
        || uint(v.principal, S128) === null || uint(v.accrued, S128) === null
        || uint(v.outstanding, S128) === null
        || !['Outstanding', 'Settled'].includes(v.status)
        || BigInt(v.principal) + BigInt(v.accrued) !== BigInt(v.outstanding)
        || (v.status === 'Settled') !== (v.outstanding === '0'))
      || state.consumedReplay.some((v) => !replayId(v))) return reject('stage', 'S0_STAGE_UNSUPPORTED');

  if (intent.kind === 'Transfer' && id(intent.signer) && id(intent.recipient) && id(intent.feeRecipient)
      && (!state.balances.some((v) => v.account === intent.signer)
        || !state.balances.some((v) => v.account === intent.recipient)
        || !state.balances.some((v) => v.account === intent.feeRecipient)
        || !state.allowances.some((v) => v.owner === intent.signer))) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (intent.kind === 'Transfer' && id(intent.signer) && id(intent.recipient)
      && id(intent.feeRecipient)
      && distinct([intent.signer, intent.recipient, intent.feeRecipient])
      && (state.balances.length !== 3 || state.obligations.length !== 0
        || state.balances[0].account !== intent.signer
        || state.balances[1].account !== intent.recipient
        || state.balances[2].account !== intent.feeRecipient
        || state.allowances.length !== 1
        || state.allowances[0].owner !== intent.signer)) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (intent.kind === 'Repay' && id(intent.signer) && id(intent.obligationId)) {
    const debt = state.obligations.find((v) => v.id === intent.obligationId);
    if (!debt || debt.asset !== state.asset || debt.debtor !== intent.signer
        || debt.creditor === intent.signer
        || debt.status !== 'Outstanding'
        || !state.balances.some((v) => v.account === intent.signer)
        || !state.balances.some((v) => v.account === debt.creditor)
        || !state.allowances.some((v) => v.owner === intent.signer)
        || state.balances.length !== 2 || state.obligations.length !== 1
        || state.allowances.length !== 1
        || state.balances[0].account !== intent.signer
        || state.balances[1].account !== debt.creditor
        || state.allowances[0].owner !== intent.signer) {
      return reject('stage', 'S0_STAGE_UNSUPPORTED');
    }
  }
  if (localStipulation !== undefined && (localStipulation === null
      || !sameValue(localStipulation.intent, intent)
      || !sameValue(localStipulation.state, state)
      || localStipulation.round !== state.round
      || !opaque(localStipulation.expectedSuccessor)
      || !sameValue(localStipulation.requestedOutcome, requestedOutcome))) {
    return reject('stage', 'S0_STAGE_PREMISE');
  }

  const notBefore = uint(intent.notBefore);
  const notAfter = uint(intent.notAfter);
  const grossCap = uint(intent.grossCap, S128);
  const feeCap = uint(intent.feeCap, S128);
  const netFloor = uint(intent.netFloor, S128);
  if (!id(intent.programId) || !opaque(intent.sourceHash) || !opaque(intent.policyDigest)
      || (intent.signedDigest !== undefined && !opaque(intent.signedDigest)) || !opaque(intent.keyRef)
      || !id(intent.signer) || !opaque(intent.nonce) || !opaque(intent.preHead)
      || intent.domain !== state.domain || intent.asset !== state.asset
      || notBefore === null || notAfter === null || notBefore > notAfter
      || grossCap === null || feeCap === null || netFloor === null
      || BigInt(state.round) < notBefore || BigInt(state.round) > notAfter) {
    return reject('intent', 'S0_INTENT_SCOPE');
  }

  const replayKey = JSON.stringify([state.domain, intent.signer, intent.nonce]);
  const balances = state.balances.map((v) => ({ ...v }));
  const allowances = state.allowances.map((v) => ({ ...v }));
  const obligations = state.obligations.map((v) => ({ ...v }));
  const ownerBalance = balances.find((v) => v.account === intent.signer);
  const ownerAllowance = allowances.find((v) => v.owner === intent.signer);
  let gross: bigint;
  let effects: S0Effect[];

  if (intent.kind === 'Transfer') {
    const v = uint(intent.amount, S128);
    const fee = uint(intent.fee, S128);
    if (!id(intent.recipient) || !id(intent.feeRecipient) || v === null || fee === null || v === 0n) {
      return reject('intent', 'S0_INTENT_SCOPE');
    }
    gross = v + fee;
    if (fee > feeCap || gross > grossCap || v < netFloor) return reject('intent', 'S0_INTENT_SCOPE');
    if (!distinct([intent.signer, intent.recipient, intent.feeRecipient])) {
      return reject('intent', 'S0_INTENT_ALIAS');
    }
    const recipient = balances.find((row) => row.account === intent.recipient);
    const feeRecipient = balances.find((row) => row.account === intent.feeRecipient);
    if (!ownerBalance || !recipient || !feeRecipient) return reject('stage', 'S0_STAGE_UNSUPPORTED');
    if (BigInt(ownerBalance.amount) < gross
        || boundedAdd(BigInt(recipient.amount), v) === null
        || (fee > 0n && boundedAdd(BigInt(feeRecipient!.amount), fee) === null)) {
      return reject('effect', 'S0_EFFECT_RANGE');
    }
    ownerBalance.amount = (BigInt(ownerBalance.amount) - gross).toString();
    recipient.amount = (BigInt(recipient.amount) + v).toString();
    if (fee > 0n) feeRecipient!.amount = (BigInt(feeRecipient!.amount) + fee).toString();
    effects = [
      { kind: 'Debit', account: intent.signer, asset: state.asset, amount: gross.toString() },
      { kind: 'Credit', account: intent.recipient, asset: state.asset, amount: v.toString() },
      ...(fee > 0n ? [{ kind: 'Credit' as const, account: intent.feeRecipient, asset: state.asset, amount: fee.toString() }] : []),
    ];
  } else if (intent.kind === 'Repay') {
    const n = uint(intent.amount, S128);
    if (!id(intent.obligationId) || n === null || n === 0n) return reject('intent', 'S0_INTENT_SCOPE');
    const obligation = obligations.find((row) => row.id === intent.obligationId);
    if (!obligation || obligation.asset !== state.asset || obligation.debtor !== intent.signer
        || obligation.status !== 'Outstanding') return reject('stage', 'S0_STAGE_UNSUPPORTED');
    const p = BigInt(obligation.principal);
    const a = BigInt(obligation.accrued);
    gross = n;
    if (gross > grossCap || feeCap !== 0n || netFloor !== 0n) return reject('intent', 'S0_INTENT_SCOPE');
    if (n > BigInt(obligation.outstanding)) return reject('effect', 'S0_EFFECT_RANGE');
    const creditor = balances.find((row) => row.account === obligation.creditor);
    if (!ownerBalance || !creditor) return reject('stage', 'S0_STAGE_UNSUPPORTED');
    if (BigInt(ownerBalance.amount) < n || boundedAdd(BigInt(creditor.amount), n) === null) {
      return reject('effect', 'S0_EFFECT_RANGE');
    }
    const da = n < a ? n : a;
    const dp = n - da;
    const afterP = p - dp;
    const afterA = a - da;
    const afterOutstanding = afterP + afterA;
    ownerBalance.amount = (BigInt(ownerBalance.amount) - n).toString();
    creditor.amount = (BigInt(creditor.amount) + n).toString();
    obligation.principal = afterP.toString();
    obligation.accrued = afterA.toString();
    obligation.outstanding = afterOutstanding.toString();
    obligation.status = afterOutstanding === 0n ? 'Settled' : 'Outstanding';
    effects = [
      { kind: 'Debit', account: intent.signer, asset: state.asset, amount: n.toString() },
      { kind: 'Credit', account: obligation.creditor, asset: state.asset, amount: n.toString() },
      { kind: 'SetObligation', id: obligation.id, principal: obligation.principal,
        accrued: obligation.accrued, outstanding: obligation.outstanding, status: obligation.status },
    ];
  } else return reject('stage', 'S0_STAGE_UNSUPPORTED');

  effects.push(
    { kind: 'UseAllowance', owner: intent.signer, amount: gross.toString() },
    { kind: 'UseReplay', key: replayKey },
    { kind: 'AdvanceHead', predecessor: intent.preHead, successor: proposedPostHead },
  );
  if (!sameEffects(effects, submittedEffects)) {
    return reject('effect', 'S0_EFFECT_MISMATCH');
  }
  if (!ownerAllowance || BigInt(ownerAllowance.remaining) < gross
      || boundedAdd(BigInt(ownerAllowance.spent), gross) === null
      || BigInt(state.workRemaining) < 1n || boundedAdd(BigInt(state.workSpent), 1n) === null) {
    return reject('authority', 'S0_AUTH_SCOPE');
  }
  ownerAllowance.remaining = (BigInt(ownerAllowance.remaining) - gross).toString();
  ownerAllowance.spent = (BigInt(ownerAllowance.spent) + gross).toString();
  if (intent.preHead !== state.head) return reject('history', 'S0_HISTORY_STALE');
  if (state.consumedReplay.includes(replayKey)) return reject('history', 'S0_HISTORY_REPLAY');
  if (!opaque(proposedPostHead) || proposedPostHead === state.head) {
    return reject('history', 'S0_HISTORY_SUCCESSOR');
  }
  if (localStipulation !== undefined
      && proposedPostHead !== localStipulation!.expectedSuccessor) {
    return reject('history', 'S0_HISTORY_SUCCESSOR');
  }
  if (requestedOutcome.phase !== 'TerminalSuccess'
      || requestedOutcome.retainedEffects.length !== 0
      || requestedOutcome.retainedDuties.length !== 0) {
    return reject('failure', 'S0_FAILURE_UNSUPPORTED');
  }

  return {
    status: 'PreparedUnqualified', core: MIL4_S0_CORE, preHead: state.head, effects,
    candidatePost: {
      ...state, balances, allowances, obligations,
      consumedReplay: [...state.consumedReplay, replayKey],
      head: proposedPostHead,
      workRemaining: (BigInt(state.workRemaining) - 1n).toString(),
      workSpent: (BigInt(state.workSpent) + 1n).toString(),
    },
    requiredPremises: ['canonical-intent-signature', 'snapshot-to-head', 'head-extension', 'atomic-ledger-compare-and-consume'],
  };
}

```

## experiments/moriarty-language/src/successor/mil4-s0-source-v6.ts

```text
/** Provisional Source/6 to Core/5 local preparation. This module cannot admit a ledger stage. */
import { prepareMil4S0, type S0PreparedUnqualified, type S0Rejected } from './mil4-s0-core-v5.ts';
import {
  parseAndLowerSource6, Source6Error,
  type Source6Ast, type Source6Lowered,
} from './financial-agreement-source-v6-frontend.ts';

export type Source6S0Outcome =
  | { status: 'SourceRejected'; code: string; offset: number; publishedPost: null; publishedEffects: null }
  | { status: 'CoreRejected'; ast: Source6Ast; rejection: S0Rejected }
  | {
      status: 'PreparedUnqualified'; ast: Source6Ast; candidate: S0PreparedUnqualified;
      unverifiedBindings: readonly ['agreement-id', 'selected-program', 'asset-scale', 'authenticated-predecessor'];
    };

function sameAction(a: Source6Ast['intent']['signedAction'], b: Source6Ast['submitted']['action']): boolean {
  if (a.kind !== b.kind) return false;
  if (a.kind === 'Transfer' && b.kind === 'Transfer') {
    return a.from === b.from && a.to === b.to && a.feeTo === b.feeTo
      && a.value === b.value && a.fee === b.fee;
  }
  return a.kind === 'Repay' && b.kind === 'Repay'
    && a.obligation === b.obligation && a.payer === b.payer && a.amount === b.amount
    && a.conversion === b.conversion;
}

/** Parse and prepare one S0 stage without asserting source authentication or ledger acceptance. */
export function prepareSource6S0Unqualified(source: string): Source6S0Outcome {
  let lowered: Source6Lowered;
  try {
    lowered = parseAndLowerSource6(source);
  } catch (error) {
    if (!(error instanceof Source6Error)) throw error;
    return {
      status: 'SourceRejected', code: error.code, offset: error.offset,
      publishedPost: null, publishedEffects: null,
    };
  }
  const { ast, state, intent, submittedEffects, proposedPostHead } = lowered;
  const result = prepareMil4S0(state, intent, submittedEffects, proposedPostHead);
  if (result.status === 'Rejected' && result.judgment === 'stage') {
    return { status: 'CoreRejected', ast, rejection: result };
  }
  if (!sameAction(ast.intent.signedAction, ast.submitted.action)) {
    return {
      status: 'CoreRejected', ast,
      rejection: {
        status: 'Rejected', judgment: 'intent', code: 'S0_INTENT_SCOPE',
        diagnosticWork: 1, publishedPost: null, publishedEffects: null,
      },
    };
  }
  return result.status === 'Rejected'
    ? { status: 'CoreRejected', ast, rejection: result }
    : {
        status: 'PreparedUnqualified', ast, candidate: result,
        unverifiedBindings: ['agreement-id', 'selected-program', 'asset-scale', 'authenticated-predecessor'],
      };
}

```

## experiments/moriarty-language/tests/mil4-s0-source-v6.test.mjs

```text
import test from 'node:test';
import assert from 'node:assert/strict';
import { prepareSource6S0Unqualified } from '../src/successor/mil4-s0-source-v6.ts';
import { parseAndLowerSource6 } from '../src/successor/financial-agreement-source-v6-frontend.ts';
import { prepareMil4S0 } from '../src/successor/mil4-s0-core-v5.ts';
import { parseFinancialAgreementSourceV5 } from '../src/successor/financial-agreement-source-v5-frontend.ts';

const U = (1n << 128n) - 1n;
const S = (1n << 127n) - 1n;

function transfer(overrides = {}) {
  const p = {
    signedRecipient: 'Recipient', submittedRecipient: 'Recipient',
    value: '10', fee: '1', grossCap: '11', feeCap: '1', netFloor: '10',
    ownerBalance: '100', recipientBalance: '0', feeBalance: '0',
    allowance: '100', spent: '0', workRemaining: '10', workSpent: '0',
    preHead: 'h0', head: 'h0', postHead: 'h1', replay: 'unused',
    includeFeeCredit: true, effectRecipient: 'Recipient',
    ...overrides,
  };
  const gross = (BigInt(p.value) + BigInt(p.fee)).toString();
  return `profile "moriarty-financial-agreement-source/6";
agreement Agreement1 {
  domain Midnight;
  settlement A scale 0;
  selected TransferLiteralFee source_hash "src1" digest "policy1";
  intent {
    signer Owner key "key1";
    nonce "n1";
    pre_head "${p.preHead}";
    valid 0..10;
    gross_cap ${p.grossCap};
    fee_cap ${p.feeCap};
    net_floor ${p.netFloor};
    failure success_only;
    signed_action transfer from Owner to ${p.signedRecipient} fee_to Fee value ${p.value} fee ${p.fee};
    observations empty;
    disclosures empty;
    retained_effects empty;
    retained_duties empty;
    delegation none;
    recovery none;
  }
  authenticated {
    head "${p.head}";
    predecessor "genesis";
    round 1;
    balance Owner ${p.ownerBalance};
    balance Recipient ${p.recipientBalance};
    balance Fee ${p.feeBalance};
    allowance Owner remaining ${p.allowance} spent ${p.spent};
    replay ${p.replay};
    work_remaining ${p.workRemaining};
    work_spent ${p.workSpent};
  }
  submit transfer from Owner to ${p.submittedRecipient} fee_to Fee value ${p.value} fee ${p.fee};
  effects {
    debit Owner ${gross};
    credit ${p.effectRecipient} ${p.value};
    ${p.includeFeeCredit ? `credit Fee ${p.fee};` : ''}
    use_allowance Owner ${gross};
    use_replay "n1";
    advance_head "${p.preHead}" "${p.postHead}";
  }
  post_head "${p.postHead}";
}`;
}
function commonTransfer(overrides = {}) {
  return transfer({ allowance: '11', workRemaining: '1', netFloor: '0', ...overrides })
    .replace('round 1;', 'round 0;');
}

function repay(overrides = {}) {
  const p = {
    amount: '30', principal: '1000', accrued: '10', outstanding: '1010',
    postPrincipal: '980', postAccrued: '0', postOutstanding: '980', postStatus: 'outstanding',
    payerBalance: '2000', creditorBalance: '0', allowance: '2000', spent: '0',
    grossCap: '30', workRemaining: '10', preHead: 'h0', head: 'h0', postHead: 'h1',
    replay: 'unused', creditAccount: 'Creditor', ...overrides,
  };
  return `profile "moriarty-financial-agreement-source/6";
agreement Agreement1 {
  domain Midnight;
  settlement A scale 0;
  selected RepayAccrualFirst source_hash "src1" digest "policy1";
  intent {
    signer Payer key "key1";
    nonce "n1";
    pre_head "${p.preHead}";
    valid 0..10;
    gross_cap ${p.grossCap};
    fee_cap 0;
    net_floor 0;
    failure success_only;
    signed_action repay obligation Loan payer Payer amount ${p.amount} conversion identity;
    observations empty;
    disclosures empty;
    retained_effects empty;
    retained_duties empty;
    delegation none;
    recovery none;
  }
  authenticated {
    head "${p.head}";
    predecessor "genesis";
    round 1;
    balance Payer ${p.payerBalance};
    balance Creditor ${p.creditorBalance};
    allowance Payer remaining ${p.allowance} spent ${p.spent};
    obligation Loan {
      debtor Payer;
      creditor Creditor;
      asset A;
      principal ${p.principal};
      accrued ${p.accrued};
      outstanding ${p.outstanding};
      status outstanding;
    }
    replay ${p.replay};
    work_remaining ${p.workRemaining};
    work_spent 0;
  }
  submit repay obligation Loan payer Payer amount ${p.amount} conversion identity;
  effects {
    debit Payer ${p.amount};
    credit ${p.creditAccount} ${p.amount};
    set_obligation Loan principal ${p.postPrincipal} accrued ${p.postAccrued} outstanding ${p.postOutstanding} status ${p.postStatus};
    use_allowance Payer ${p.amount};
    use_replay "n1";
    advance_head "${p.preHead}" "${p.postHead}";
  }
  post_head "${p.postHead}";
}`;
}

function prepared(source) {
  const result = prepareSource6S0Unqualified(source);
  assert.equal(result.status, 'PreparedUnqualified', JSON.stringify(result));
  return result.candidate;
}
function rejected(source, status, judgment, code) {
  const result = prepareSource6S0Unqualified(source);
  assert.equal(result.status, status, JSON.stringify(result));
  assert.equal(status === 'SourceRejected' ? result.code : result.rejection.code, code);
  if (judgment) {
    assert.equal(result.rejection.judgment, judgment);
    assert.equal(result.rejection.diagnosticWork, 1);
    assert.equal(result.rejection.publishedPost, null);
    assert.equal(result.rejection.publishedEffects, null);
  }
  return result;
}

test('T-10-1 preserves gross debit, fee credit, allowance, replay, head, and work', () => {
  const result = prepared(transfer());
  assert.deepEqual(result.effects.map((e) => e.kind),
    ['Debit', 'Credit', 'Credit', 'UseAllowance', 'UseReplay', 'AdvanceHead']);
  assert.deepEqual(result.effects.slice(0, 3).map((e) => e.amount), ['11', '10', '1']);
  assert.deepEqual(result.candidatePost.balances.map((v) => v.amount), ['89', '10', '1']);
  assert.deepEqual(result.candidatePost.allowances[0], { owner: 'Owner', remaining: '89', spent: '11' });
  assert.equal(result.candidatePost.head, 'h1');
  assert.equal(result.candidatePost.workRemaining, '9');
  assert.equal(result.candidatePost.consumedReplay.length, 1);
});

test('zero fee omits the fee line', () => {
  const result = prepared(transfer({ fee: '0', grossCap: '10', feeCap: '0', includeFeeCredit: false }));
  assert.equal(result.effects.length, 5);
  assert.equal(result.candidatePost.balances[2].amount, '0');
});

test('R-30 pays the bound creditor and accrued amount first', () => {
  const result = prepared(repay());
  assert.deepEqual(result.candidatePost.balances.map((v) => v.amount), ['1970', '30']);
  assert.deepEqual(result.candidatePost.obligations[0], {
    id: 'Loan', debtor: 'Payer', creditor: 'Creditor', asset: 'A',
    principal: '980', accrued: '0', outstanding: '980', status: 'Outstanding',
  });
  assert.deepEqual(result.effects.map((e) => e.kind),
    ['Debit', 'Credit', 'SetObligation', 'UseAllowance', 'UseReplay', 'AdvanceHead']);
});

test('full repay settles the obligation', () => {
  const result = prepared(repay({ amount: '1010', grossCap: '1010', postPrincipal: '0',
    postAccrued: '0', postOutstanding: '0', postStatus: 'settled' }));
  assert.equal(result.candidatePost.obligations[0].status, 'Settled');
  assert.equal(result.candidatePost.obligations[0].outstanding, '0');
});

test('R-near-bound reaches UInt128 receiver and spent limits without overflow', () => {
  const result = prepared(repay({ amount: '1', grossCap: '1', principal: (S - 1n).toString(),
    accrued: '1', outstanding: S.toString(), postPrincipal: (S - 1n).toString(),
    postAccrued: '0', postOutstanding: (S - 1n).toString(), payerBalance: '1',
    creditorBalance: (U - 1n).toString(), allowance: '1', spent: (U - 1n).toString() }));
  assert.equal(result.candidatePost.balances[1].amount, U.toString());
  assert.equal(result.candidatePost.allowances[0].spent, U.toString());
  assert.equal(result.candidatePost.obligations[0].principal, (S - 1n).toString());
});

test('a distinct signed nonce permits a later partial repayment', () => {
  const next = repay({ amount: '10', grossCap: '10', principal: '980', accrued: '0',
    outstanding: '980', postPrincipal: '970', postAccrued: '0', postOutstanding: '970',
    preHead: 'h1', head: 'h1', postHead: 'h2' }).replaceAll('"n1"', '"n2"');
  const result = prepared(next);
  assert.equal(result.candidatePost.obligations[0].outstanding, '970');
  assert.equal(result.candidatePost.head, 'h2');
});

test('recipient substitution is an Intent rejection', () => {
  rejected(transfer({ submittedRecipient: 'Other' }), 'CoreRejected', 'intent', 'S0_INTENT_SCOPE');
});
test('fee cap excess is an Intent rejection', () => {
  rejected(transfer({ fee: '2', feeCap: '1', grossCap: '12' }),
    'CoreRejected', 'intent', 'S0_INTENT_SCOPE');
});
test('missing fee credit is an Effect rejection', () => {
  rejected(transfer({ includeFeeCredit: false }), 'CoreRejected', 'effect', 'S0_EFFECT_MISMATCH');
});
test('an explicit zero-valued fee credit is an Effect rejection', () => {
  rejected(transfer({ fee: '0', grossCap: '10', feeCap: '0', includeFeeCredit: true }),
    'CoreRejected', 'effect', 'S0_EFFECT_MISMATCH');
});
test('insufficient allowance and work reject at Authority', () => {
  rejected(transfer({ allowance: '10' }), 'CoreRejected', 'authority', 'S0_AUTH_SCOPE');
  rejected(transfer({ workRemaining: '0' }), 'CoreRejected', 'authority', 'S0_AUTH_SCOPE');
});
test('stale head and consumed replay reject at History', () => {
  rejected(transfer({ preHead: 'old' }), 'CoreRejected', 'history', 'S0_HISTORY_STALE');
  rejected(transfer({ replay: 'consumed' }), 'CoreRejected', 'history', 'S0_HISTORY_REPLAY');
});
test('effect mismatch precedes stale history', () => {
  rejected(transfer({ includeFeeCredit: false, preHead: 'old' }),
    'CoreRejected', 'effect', 'S0_EFFECT_MISMATCH');
});
test('receiver overflow rejects at Effect', () => {
  rejected(transfer({ recipientBalance: U.toString() }),
    'CoreRejected', 'effect', 'S0_EFFECT_RANGE');
});
test('effect range precedes an incorrect submitted vector', () => {
  rejected(transfer({ recipientBalance: U.toString(), includeFeeCredit: false }),
    'CoreRejected', 'effect', 'S0_EFFECT_RANGE');
  rejected(repay({ amount: '31', grossCap: '31', principal: '20', accrued: '10',
    outstanding: '30', creditAccount: 'Other' }),
    'CoreRejected', 'effect', 'S0_EFFECT_RANGE');
});
test('signed scope and caps precede a direct Core transfer alias', () => {
  const lowered = parseAndLowerSource6(transfer());
  const alias = { ...lowered.intent, recipient: lowered.intent.signer };
  const validAlias = prepareMil4S0(lowered.state, alias, lowered.submittedEffects, lowered.proposedPostHead);
  assert.equal(validAlias.status, 'Rejected');
  assert.deepEqual([validAlias.judgment, validAlias.code], ['intent', 'S0_INTENT_ALIAS']);
  const badCapAlias = prepareMil4S0(lowered.state, { ...alias, feeCap: '0' },
    lowered.submittedEffects, lowered.proposedPostHead);
  assert.equal(badCapAlias.status, 'Rejected');
  assert.deepEqual([badCapAlias.judgment, badCapAlias.code], ['intent', 'S0_INTENT_SCOPE']);
  const badRoundAlias = prepareMil4S0(lowered.state, { ...alias, notAfter: '0' },
    lowered.submittedEffects, lowered.proposedPostHead);
  assert.equal(badRoundAlias.status, 'Rejected');
  assert.deepEqual([badRoundAlias.judgment, badRoundAlias.code], ['intent', 'S0_INTENT_SCOPE']);
});
test('direct Core malformed repayment bindings reject at Stage', () => {
  const lowered = parseAndLowerSource6(repay());
  const selfCredit = { ...lowered.state,
    obligations: [{ ...lowered.state.obligations[0], creditor: lowered.intent.signer }] };
  const alias = prepareMil4S0(selfCredit, lowered.intent, lowered.submittedEffects, lowered.proposedPostHead);
  assert.equal(alias.status, 'Rejected');
  assert.deepEqual([alias.judgment, alias.code], ['stage', 'S0_STAGE_UNSUPPORTED']);
  const wrongDebtor = { ...lowered.state,
    obligations: [{ ...lowered.state.obligations[0], debtor: 'Other' }] };
  const debtor = prepareMil4S0(wrongDebtor, lowered.intent, lowered.submittedEffects, lowered.proposedPostHead);
  assert.equal(debtor.status, 'Rejected');
  assert.deepEqual([debtor.judgment, debtor.code], ['stage', 'S0_STAGE_UNSUPPORTED']);
  rejected(repay().replace('creditor Creditor;', 'creditor Payer;'),
    'SourceRejected', null, 'SOURCE6_CELL_SHAPE');
});
test('source nominal bound rejects before Core preparation', () => {
  rejected(transfer({ value: (S + 1n).toString(), grossCap: S.toString() }),
    'SourceRejected', null, 'SOURCE6_RANGE');
});
test('repayment above outstanding rejects at Effect', () => {
  rejected(repay({ amount: '31', grossCap: '31', principal: '20', accrued: '10', outstanding: '30' }),
    'CoreRejected', 'effect', 'S0_EFFECT_RANGE');
});
test('Source/6 formation rejects invalid validity, cell shape, and trailing input', () => {
  rejected(transfer().replace('valid 0..10;', 'valid 10..0;'), 'SourceRejected', null, 'SOURCE6_RANGE');
  rejected(transfer().replace('balance Fee 0;', ''), 'SourceRejected', null, 'SOURCE6_CELL_SHAPE');
  rejected(`${transfer()} trailing`, 'SourceRejected', null, 'SOURCE6_SHAPE');
  rejected(transfer().replace('agreement Agreement1 {', 'agreement post_head {'),
    'SourceRejected', null, 'SOURCE6_SHAPE');
});
test('Source/5 entry rejects a Source/6 profile before accepting the body', () => {
  assert.throws(() => parseFinancialAgreementSourceV5(transfer()),
    (error) => error && error.code === 'PROFILE_MISMATCH');
});

test('Core comparison rejects missing repayment credit and wrong bound creditor', () => {
  const lowered = parseAndLowerSource6(repay());
  const missing = lowered.submittedEffects.filter((effect) => effect.kind !== 'Credit');
  const missingResult = prepareMil4S0(lowered.state, lowered.intent, missing, lowered.proposedPostHead);
  assert.equal(missingResult.status, 'Rejected');
  assert.equal(missingResult.judgment, 'effect');
  assert.equal(missingResult.code, 'S0_EFFECT_MISMATCH');
  rejected(repay({ creditAccount: 'Other' }), 'CoreRejected', 'effect', 'S0_EFFECT_MISMATCH');
});

test('common round-zero T-10-1 has exact effects and complete post-state', () => {
  const result = prepared(commonTransfer());
  const replayKey = '["Midnight","Owner","n1"]';
  assert.deepEqual(result.effects, [
    { kind: 'Debit', account: 'Owner', asset: 'A', amount: '11' },
    { kind: 'Credit', account: 'Recipient', asset: 'A', amount: '10' },
    { kind: 'Credit', account: 'Fee', asset: 'A', amount: '1' },
    { kind: 'UseAllowance', owner: 'Owner', amount: '11' },
    { kind: 'UseReplay', key: replayKey },
    { kind: 'AdvanceHead', predecessor: 'h0', successor: 'h1' },
  ]);
  assert.deepEqual(result.candidatePost, {
    core: 'moriarty-core/5', domain: 'Midnight', asset: 'A', head: 'h1', round: '0',
    workRemaining: '0', workSpent: '1',
    balances: [{ account: 'Owner', amount: '89' }, { account: 'Recipient', amount: '10' },
      { account: 'Fee', amount: '1' }],
    allowances: [{ owner: 'Owner', remaining: '0', spent: '11' }],
    obligations: [], consumedReplay: [replayKey],
  });
});

test('common round-zero R-30 has exact effects and complete post-state', () => {
  const result = prepared(repay({ payerBalance: '100', allowance: '100', workRemaining: '1' })
    .replace('round 1;', 'round 0;'));
  const replayKey = '["Midnight","Payer","n1"]';
  assert.deepEqual(result.effects, [
    { kind: 'Debit', account: 'Payer', asset: 'A', amount: '30' },
    { kind: 'Credit', account: 'Creditor', asset: 'A', amount: '30' },
    { kind: 'SetObligation', id: 'Loan', principal: '980', accrued: '0', outstanding: '980', status: 'Outstanding' },
    { kind: 'UseAllowance', owner: 'Payer', amount: '30' },
    { kind: 'UseReplay', key: replayKey },
    { kind: 'AdvanceHead', predecessor: 'h0', successor: 'h1' },
  ]);
  assert.deepEqual(result.candidatePost, {
    core: 'moriarty-core/5', domain: 'Midnight', asset: 'A', head: 'h1', round: '0',
    workRemaining: '0', workSpent: '1',
    balances: [{ account: 'Payer', amount: '70' }, { account: 'Creditor', amount: '30' }],
    allowances: [{ owner: 'Payer', remaining: '70', spent: '30' }],
    obligations: [{ id: 'Loan', debtor: 'Payer', creditor: 'Creditor', asset: 'A',
      principal: '980', accrued: '0', outstanding: '980', status: 'Outstanding' }],
    consumedReplay: [replayKey],
  });
});

test('common round-zero near-bound repayment has exact effects and complete post-state', () => {
  const result = prepared(repay({ amount: '1', grossCap: '1', principal: (S - 1n).toString(),
    accrued: '1', outstanding: S.toString(), postPrincipal: (S - 1n).toString(),
    postAccrued: '0', postOutstanding: (S - 1n).toString(), payerBalance: '1',
    creditorBalance: (U - 1n).toString(), allowance: '1', spent: (U - 1n).toString(),
    workRemaining: '1' }).replace('round 1;', 'round 0;'));
  const replayKey = '["Midnight","Payer","n1"]';
  assert.deepEqual(result.effects, [
    { kind: 'Debit', account: 'Payer', asset: 'A', amount: '1' },
    { kind: 'Credit', account: 'Creditor', asset: 'A', amount: '1' },
    { kind: 'SetObligation', id: 'Loan', principal: (S - 1n).toString(),
      accrued: '0', outstanding: (S - 1n).toString(), status: 'Outstanding' },
    { kind: 'UseAllowance', owner: 'Payer', amount: '1' },
    { kind: 'UseReplay', key: replayKey },
    { kind: 'AdvanceHead', predecessor: 'h0', successor: 'h1' },
  ]);
  assert.deepEqual(result.candidatePost, {
    core: 'moriarty-core/5', domain: 'Midnight', asset: 'A', head: 'h1', round: '0',
    workRemaining: '0', workSpent: '1',
    balances: [{ account: 'Payer', amount: '0' }, { account: 'Creditor', amount: U.toString() }],
    allowances: [{ owner: 'Payer', remaining: '0', spent: U.toString() }],
    obligations: [{ id: 'Loan', debtor: 'Payer', creditor: 'Creditor', asset: 'A',
      principal: (S - 1n).toString(), accrued: '0', outstanding: (S - 1n).toString(),
      status: 'Outstanding' }],
    consumedReplay: [replayKey],
  });
});

function localTuple(lowered, expectedSuccessor = 'h1', requestedOutcome = {
  phase: 'TerminalSuccess', retainedEffects: [], retainedDuties: [],
}) {
  return { intent: lowered.intent, state: lowered.state, round: lowered.state.round,
    expectedSuccessor, requestedOutcome };
}
function coreReject(result, judgment, code) {
  assert.deepEqual(result, { status: 'Rejected', judgment, code,
    diagnosticWork: 1, publishedPost: null, publishedEffects: null });
}
test('direct Core rejects an unknown action at Stage before a bad cap', () => {
  const x = parseAndLowerSource6(transfer());
  const unknown = { ...x.intent, kind: 'Alien', feeCap: 'bad' };
  coreReject(prepareMil4S0(x.state, unknown, x.submittedEffects, x.proposedPostHead),
    'stage', 'S0_STAGE_UNSUPPORTED');
});
test('direct Core enforces closed authenticated cell order and footprint', () => {
  const t = parseAndLowerSource6(transfer());
  const reversed = { ...t.state, balances: [t.state.balances[1], t.state.balances[0], t.state.balances[2]] };
  coreReject(prepareMil4S0(reversed, t.intent, t.submittedEffects, t.proposedPostHead),
    'stage', 'S0_STAGE_UNSUPPORTED');
  const extra = { ...t.state, obligations: [{ id: 'Other', debtor: 'Owner', creditor: 'Recipient',
    asset: 'A', principal: '1', accrued: '0', outstanding: '1', status: 'Outstanding' }] };
  coreReject(prepareMil4S0(extra, t.intent, t.submittedEffects, t.proposedPostHead),
    'stage', 'S0_STAGE_UNSUPPORTED');
  const r = parseAndLowerSource6(repay());
  const repayExtra = { ...r.state, balances: [...r.state.balances, { account: 'Other', amount: '0' }] };
  coreReject(prepareMil4S0(repayExtra, r.intent, r.submittedEffects, r.proposedPostHead),
    'stage', 'S0_STAGE_UNSUPPORTED');
});
test('S1B exact successor and typed failure observations stay local and unqualified', () => {
  const t = parseAndLowerSource6(commonTransfer());
  const accepted = prepareMil4S0(t.state, t.intent, t.submittedEffects,
    t.proposedPostHead, localTuple(t).requestedOutcome, localTuple(t));
  assert.equal(accepted.status, 'PreparedUnqualified');
  assert.deepEqual(accepted.effects, [
    { kind: 'Debit', account: 'Owner', asset: 'A', amount: '11' },
    { kind: 'Credit', account: 'Recipient', asset: 'A', amount: '10' },
    { kind: 'Credit', account: 'Fee', asset: 'A', amount: '1' },
    { kind: 'UseAllowance', owner: 'Owner', amount: '11' },
    { kind: 'UseReplay', key: '["Midnight","Owner","n1"]' },
    { kind: 'AdvanceHead', predecessor: 'h0', successor: 'h1' },
  ]);
  assert.deepEqual(accepted.candidatePost, {
    core: 'moriarty-core/5', domain: 'Midnight', asset: 'A', head: 'h1', round: '0',
    workRemaining: '0', workSpent: '1',
    balances: [{ account: 'Owner', amount: '89' }, { account: 'Recipient', amount: '10' },
      { account: 'Fee', amount: '1' }],
    allowances: [{ owner: 'Owner', remaining: '0', spent: '11' }],
    obligations: [], consumedReplay: ['["Midnight","Owner","n1"]'],
  });
  assert.deepEqual(accepted.requiredPremises,
    ['canonical-intent-signature', 'snapshot-to-head', 'head-extension', 'atomic-ledger-compare-and-consume']);
  const alternate = parseAndLowerSource6(commonTransfer({ postHead: 'h9' }));
  const nonArithmetic = prepareMil4S0(alternate.state, alternate.intent, alternate.submittedEffects,
    alternate.proposedPostHead, localTuple(alternate, 'h9').requestedOutcome,
    localTuple(alternate, 'h9'));
  assert.equal(nonArithmetic.status, 'PreparedUnqualified');
  assert.equal(nonArithmetic.candidatePost.head, 'h9');
  assert.deepEqual(nonArithmetic.effects.at(-1),
    { kind: 'AdvanceHead', predecessor: 'h0', successor: 'h9' });
  coreReject(prepareMil4S0(alternate.state, alternate.intent, alternate.submittedEffects,
    alternate.proposedPostHead, localTuple(alternate).requestedOutcome, localTuple(alternate)),
  'history', 'S0_HISTORY_SUCCESSOR');
  const self = parseAndLowerSource6(commonTransfer({ postHead: 'h0' }));
  coreReject(prepareMil4S0(self.state, self.intent, self.submittedEffects,
    self.proposedPostHead, localTuple(self).requestedOutcome, localTuple(self)),
  'history', 'S0_HISTORY_SUCCESSOR');
  coreReject(prepareMil4S0(self.state, self.intent, self.submittedEffects,
    self.proposedPostHead, localTuple(self, 'h0').requestedOutcome, localTuple(self, 'h0')),
  'history', 'S0_HISTORY_SUCCESSOR');
  const wrongHeadLine = t.submittedEffects.map((line) => line.kind === 'AdvanceHead'
    ? { ...line, successor: 'h9' } : line);
  coreReject(prepareMil4S0(t.state, t.intent, wrongHeadLine, 'h1',
    localTuple(t).requestedOutcome, localTuple(t)), 'effect', 'S0_EFFECT_MISMATCH');
  const stale = parseAndLowerSource6(commonTransfer({ preHead: 'h9', postHead: 'h9' }));
  coreReject(prepareMil4S0(stale.state, stale.intent, stale.submittedEffects,
    stale.proposedPostHead, localTuple(stale).requestedOutcome, localTuple(stale)),
  'history', 'S0_HISTORY_STALE');
  const replayed = { ...alternate.state, consumedReplay: [JSON.stringify(['Midnight', 'Owner', 'n1'])] };
  const replayTuple = { ...localTuple(alternate), state: replayed };
  coreReject(prepareMil4S0(replayed, alternate.intent, alternate.submittedEffects,
    alternate.proposedPostHead, replayTuple.requestedOutcome, replayTuple),
  'history', 'S0_HISTORY_REPLAY');
  const staleOnly = parseAndLowerSource6(commonTransfer({ preHead: 'h9' }));
  const staleReplayed = { ...staleOnly.state, consumedReplay: [JSON.stringify(['Midnight', 'Owner', 'n1'])] };
  const staleReplayTuple = { ...localTuple(staleOnly), state: staleReplayed };
  coreReject(prepareMil4S0(staleReplayed, staleOnly.intent, staleOnly.submittedEffects,
    staleOnly.proposedPostHead, staleReplayTuple.requestedOutcome, staleReplayTuple),
  'history', 'S0_HISTORY_STALE');
  const replayedExpected = { ...t.state, consumedReplay: [JSON.stringify(['Midnight', 'Owner', 'n1'])] };
  const replayedExpectedTuple = { ...localTuple(t), state: replayedExpected };
  coreReject(prepareMil4S0(replayedExpected, t.intent, wrongHeadLine,
    t.proposedPostHead, replayedExpectedTuple.requestedOutcome, replayedExpectedTuple),
  'effect', 'S0_EFFECT_MISMATCH');
  coreReject(prepareMil4S0(t.state, t.intent, wrongHeadLine,
    t.proposedPostHead, localTuple(t, 'h9').requestedOutcome, localTuple(t, 'h9')),
  'effect', 'S0_EFFECT_MISMATCH');
  for (const outcome of [
    { phase: 'RequestedFailure', retainedEffects: [], retainedDuties: [] },
    { phase: 'TerminalSuccess', retainedEffects: [{ kind: 'Retain' }], retainedDuties: [] },
    { phase: 'TerminalSuccess', retainedEffects: [], retainedDuties: [{ kind: 'Owe' }] },
  ]) {
    coreReject(prepareMil4S0(t.state, t.intent, t.submittedEffects,
      t.proposedPostHead, outcome, localTuple(t, 'h1', outcome)),
    'failure', 'S0_FAILURE_UNSUPPORTED');
  }
  const failure = { phase: 'RequestedFailure', retainedEffects: [], retainedDuties: [] };
  coreReject(prepareMil4S0(t.state, t.intent, wrongHeadLine,
    t.proposedPostHead, failure, localTuple(t, 'h1', failure)),
  'effect', 'S0_EFFECT_MISMATCH');
  coreReject(prepareMil4S0(t.state, t.intent, t.submittedEffects,
    t.proposedPostHead, localTuple(t).requestedOutcome, null),
  'stage', 'S0_STAGE_PREMISE');
  for (const mismatch of [
    { ...localTuple(t), intent: { ...t.intent, nonce: 'n2' } },
    { ...localTuple(t), state: { ...t.state,
      balances: [{ ...t.state.balances[0], amount: '99' }, ...t.state.balances.slice(1)] } },
    { ...localTuple(t), round: '2' },
    { ...localTuple(t), requestedOutcome: { phase: 'RequestedFailure',
      retainedEffects: [], retainedDuties: [] } },
  ]) coreReject(prepareMil4S0(t.state, t.intent, t.submittedEffects,
    t.proposedPostHead, localTuple(t).requestedOutcome, mismatch),
  'stage', 'S0_STAGE_PREMISE');
  const malformed = { ...t.state, balances: t.state.balances.slice(1) };
  coreReject(prepareMil4S0(malformed, t.intent, t.submittedEffects,
    t.proposedPostHead, localTuple(t).requestedOutcome, null),
  'stage', 'S0_STAGE_UNSUPPORTED');
});

```

## experiments/moriarty-language/formal/k/mil4/s0.k

```text
// Provisional Source/6 -> Core/5 S0 stage. External verification is abstract.
module MIL4-S0-SYNTAX
  imports INT-SYNTAX
  imports STRING-SYNTAX
  // Opaque head identity. A trusted premise must authenticate the submitted successor.
  syntax Head ::= head(String) [symbol(head)]
  syntax ReplayKey ::= replayKey(String, String, String) [symbol(replayKey)]
  syntax ReplaySet ::= noReplays() [symbol(noReplays)] | used(ReplayKey, ReplaySet) [symbol(used)]
  syntax Balance ::= noBalance() [symbol(noBalance)] | balance(String, String, Int) [symbol(balance)]
  syntax Allowance ::= allowance(String, String, Int, Int) [symbol(allowance)]
  syntax Obligation ::= noObligation() [symbol(noObligation)]
                      | obligation(String, String, String, String, Int, Int, Int, String) [symbol(obligation)]
  syntax State ::= state(Balance, Balance, Balance, Allowance, Obligation, Head, ReplaySet, Int, Int) [symbol(state)]
  syntax Action ::= transfer(Int, Int) [symbol(transfer)] | repay(String, String, Int, Int, Int, String) [symbol(repay)]
  // The intent digest is an opaque identifier; its exact signed bytes remain W-D1/W-D2.
  syntax Intent ::= intent(String, String, String, String, String, String, Head, Int, Int,
                           String, String, Int, Int, Int, Action, String) [symbol(intent)]
  syntax Fill ::= fill(String, String, String, String, Head, String, String,
                       Int, Int, Int, Action, Head) [symbol(fill)]
  syntax Effect ::= debit(String, String, Int) [symbol(debit)]
                  | credit(String, String, Int) [symbol(credit)]
                  | setObligation(String, Int, Int, Int, String) [symbol(setObligation)]
                  | useAllowance(String, String, Int) [symbol(useAllowance)]
                  | useReplay(ReplayKey) [symbol(useReplay)]
                  | advanceHead(Head, Head) [symbol(advanceHead)]
  syntax Effects ::= noEffects() [symbol(noEffects)] | effect(Effect, Effects) [symbol(effect)]
  syntax Phase ::= terminalSuccess() [symbol(terminalSuccess)] | requestedFailure(String) [symbol(requestedFailure)]
  syntax Duty ::= noDuty() [symbol(noDuty)] | retainedDuty(String) [symbol(retainedDuty)]
  syntax Duties ::= noDuties() [symbol(noDuties)]
                    | duty(String, Duties) [symbol(duty)]
  // Requested outcome is typed and bound by the stipulated premise. It is not
  // evidence of a signature or supported failure semantics.
  syntax Outcome ::= outcome(Phase, Effects, Duties) [symbol(outcome)]
  syntax Premise ::= unavailable() [symbol(unavailable)]
                   | authenticated(Intent, State, Int, Head, Outcome) [symbol(authenticated)]
  syntax Family ::= ammCP1() [symbol(ammCP1)] | loanFixed1() [symbol(loanFixed1)]
                  | cdp1() [symbol(cdp1)] | optCapped1() [symbol(optCapped1)]
                  | obs1() [symbol(obs1)] | gov1() [symbol(gov1)]
                  | bridgePair1() [symbol(bridgePair1)] | vault1() [symbol(vault1)]
  syntax Request ::= submit(Intent, Fill, State, Effects, Outcome, Int) [symbol(submit)]
                   | submitFamily(Family) [symbol(submitFamily)]
                   | unsupported(String) [symbol(unsupported)]
  syntax Result ::= pending() [symbol(pending)]
                  | rejected(String, String, Int) [symbol(rejected)]
                  | accepted(State, Effects, State, Phase, Duty, Int) [symbol(accepted)]
endmodule

module MIL4-S0
  imports MIL4-S0-SYNTAX
  imports INT
  imports BOOL
  imports STRING
  imports K-EQUAL
  configuration <s0> <k> $PGM:Request </k> <out> pending() </out>
                     <external> unavailable() </external> </s0>

  syntax Int ::= maxU() [function] | maxNominal() [function]
  rule maxU() => 340282366920938463463374607431768211455
  rule maxNominal() => 170141183460469231731687303715884105727
  syntax Bool ::= uint(Int) [function] | nominal(Int) [function]
  rule uint(N) => N >=Int 0 andBool N <=Int maxU()
  rule nominal(N) => N >=Int 0 andBool N <=Int maxNominal()
  syntax Bool ::= hasReplay(ReplayKey, ReplaySet) [function, total]
  rule hasReplay(_, noReplays()) => false
  rule hasReplay(K, used(K, _)) => true
  rule hasReplay(K, used(J, R)) => hasReplay(K, R) requires notBool K ==K J
  syntax String ::= debtStatus(Int) [function]
  rule debtStatus(0) => "Settled"
  rule debtStatus(N) => "Outstanding" requires N >Int 0
  syntax Int ::= accruedPaid(Int, Int) [function]
  rule accruedPaid(N, A) => minInt(N, A)

  syntax Bool ::= goodState(State) [function, total]
  rule goodState(state(balance(O,A,OB),balance(R,A,RB),balance(F,A,FB),
                       allowance(O,A,AR,AS),_,_,_,WR,WS))
    => O =/=String R andBool O =/=String F andBool R =/=String F
       andBool uint(OB) andBool uint(RB) andBool uint(FB)
       andBool uint(AR) andBool uint(AS) andBool uint(WR) andBool uint(WS)
       andBool AR +Int AS <=Int maxU() andBool WR +Int WS <=Int maxU()
  rule goodState(state(balance(O,A,OB),balance(R,A,RB),noBalance(),
                       allowance(_,A,AR,AS),_,_,_,WR,WS))
    => O =/=String R andBool uint(OB) andBool uint(RB)
       andBool uint(AR) andBool uint(AS) andBool uint(WR) andBool uint(WS)
       andBool AR +Int AS <=Int maxU() andBool WR +Int WS <=Int maxU()
  rule goodState(_) => false [owise]
  syntax Bool ::= goodDebt(Obligation) [function, total]
  rule goodDebt(noObligation()) => true
  rule goodDebt(obligation(_,D,C,_,P,A,O,S))
    => D =/=String C andBool nominal(P) andBool nominal(A) andBool nominal(O)
       andBool P +Int A ==Int O andBool P +Int A <=Int maxU()
       andBool ((O >Int 0 andBool S ==String "Outstanding")
                orBool (O ==Int 0 andBool S ==String "Settled"))
  rule goodDebt(_) => false [owise]
  syntax Bool ::= matchingScope(Intent, Fill) [function, total]
  rule matchingScope(intent(_,_,D,S,A,N,H,_,_,R,F,G,FC,NF,transfer(V,Fee),_),
                     fill(D,S,A,N,H,R,F,G,FC,NF,transfer(V,Fee),_)) => true
  // Repay has no recipient or fee endpoint in Source/6; empty strings are absent fields.
  rule matchingScope(intent(_,_,D,S,A,N,H,_,_,"","",G,FC,NF,repay(ID,Payer,V,M,Scale,Rnd),_),
                     fill(D,S,A,N,H,"","",G,FC,NF,repay(ID,Payer,V,M,Scale,Rnd),_)) => true
  rule matchingScope(_,_) => false [owise]
  syntax Bool ::= actionIntentOK(Action, Int, Int, Int) [function, total]
  rule actionIntentOK(transfer(V,F),G,FC,NF)
    => V >Int 0 andBool nominal(V) andBool nominal(F) andBool nominal(G)
       andBool nominal(FC) andBool nominal(NF) andBool F <=Int FC
       andBool V +Int F <=Int G andBool V >=Int NF
  rule actionIntentOK(repay(_,_,N,1,0,"none"),G,FC,NF)
    => N >Int 0 andBool nominal(N) andBool nominal(G) andBool FC ==Int 0
       andBool NF ==Int 0 andBool N <=Int G
  rule actionIntentOK(_,_,_,_) => false [owise]

  syntax Effects ::= transferEffects(Intent, Int, Int, Head) [function]
                   | repayEffects(Intent, Obligation, Int, Head) [function]
  rule transferEffects(intent(_,_,D,S,A,N,H,_,_,R,_,_,_,_,_,_),V,0,Post)
    => effect(debit(S,A,V),effect(credit(R,A,V),
       effect(useAllowance(S,A,V),effect(useReplay(replayKey(D,S,N)),
       effect(advanceHead(H,Post),noEffects())))))
  rule transferEffects(intent(_,_,D,S,A,N,H,_,_,R,F,_,_,_,_,_),V,FEE,Post)
    => effect(debit(S,A,V +Int FEE),effect(credit(R,A,V),effect(credit(F,A,FEE),
       effect(useAllowance(S,A,V +Int FEE),effect(useReplay(replayKey(D,S,N)),
       effect(advanceHead(H,Post),noEffects()))))))
    requires FEE >Int 0
  rule repayEffects(intent(_,_,D,S,A,N,H,_,_,_,_,_,_,_,repay(_,Payer,_,_,_,_),_),
                    obligation(ID,_,C,_,P,AC,_,_),PAY,Post)
    => effect(debit(Payer,A,PAY),effect(credit(C,A,PAY),
       effect(setObligation(ID,P -Int (PAY -Int accruedPaid(PAY,AC)),
                            AC -Int accruedPaid(PAY,AC),P +Int AC -Int PAY,
                            debtStatus(P +Int AC -Int PAY)),
       effect(useAllowance(S,A,PAY),effect(useReplay(replayKey(D,S,N)),
       effect(advanceHead(H,Post),noEffects()))))))

  syntax KItem ::= s0Stage(Request) | s0Intent(Request) | s0Effect(Request)
                 | s0Authority(Request) | s0History(Request) | s0Failure(Request)
                 | s0Commit(Request) | s0Stop(String, String, Int)
  rule <k> unsupported(_) => s0Stop("stage","S0_STAGE_UNSUPPORTED",1) </k>
  rule <k> submitFamily(_) => s0Stop("stage","FAMILY_STAGE_ADAPTER_ABSENT",1) </k>
  rule <k> submit(I,F,S,E,Requested,Round) => s0Stage(submit(I,F,S,E,Requested,Round)) </k>
  rule <k> s0Stop(J,C,W) => .K </k> <out> pending() => rejected(J,C,W) </out>

  rule <k> s0Stage(submit(I,_,S,_,Requested,Round) #as R) => s0Intent(R) </k>
    <external> X </external>
    requires goodState(S) andBool goodDebt(debtOf(S)) andBool selected(I)
             andBool stageShape(I,S) andBool premiseOK(X,I,S,Round,Requested)
  rule <k> s0Stage(submit(I,_,S,_,Requested,Round)) => s0Stop("stage","S0_STAGE_PREMISE",1) </k>
    <external> X </external>
    requires goodState(S) andBool goodDebt(debtOf(S)) andBool selected(I)
             andBool stageShape(I,S) andBool notBool premiseOK(X,I,S,Round,Requested)
  rule <k> s0Stage(_) => s0Stop("stage","S0_STAGE_UNSUPPORTED",1) </k> [owise]
  syntax Obligation ::= debtOf(State) [function]
  rule debtOf(state(_,_,_,_,O,_,_,_,_)) => O
  syntax Bool ::= selected(Intent) [function, total]
  rule selected(intent("Source/6","Core/5",_,_,_,_,_,_,_,_,_,_,_,_,transfer(_,_),_)) => true
  rule selected(intent("Source/6","Core/5",_,_,_,_,_,_,_,_,_,_,_,_,repay(_,_,_,_,_,_),_)) => true
  rule selected(_) => false [owise]
  syntax Bool ::= balancePresent(String,String,Balance) [function, total]
  rule balancePresent(O,A,balance(O,A,_)) => true
  rule balancePresent(_,_,_) => false [owise]
  syntax Bool ::= stageShape(Intent,State) [function, total]
  // Distinct authenticated rows may support signed endpoint aliases. Presence
  // belongs to Stage; a valid direct alias is diagnosed later at Intent.
  rule stageShape(intent(_,_,_,S,A,_,_,_,_,R,F,_,_,_,transfer(_,_),_) #as I,
                  state(balance(S,A,_) #as B0,B1,B2,allowance(S,A,_,_),
                        noObligation(),_,_,_,_))
    => (balancePresent(R,A,B0) orBool balancePresent(R,A,B1) orBool balancePresent(R,A,B2))
       andBool (balancePresent(F,A,B0) orBool balancePresent(F,A,B1) orBool balancePresent(F,A,B2))
       andBool (notBool distinct(I) orBool (balancePresent(R,A,B1) andBool balancePresent(F,A,B2)))
  rule stageShape(intent(_,_,_,S,A,_,_,_,_,_,_,_,_,_,repay(ID,S,_,_,_,_),_),
                  state(balance(S,A,_),balance(C,A,_),noBalance(),allowance(S,A,_,_),
                        obligation(ID,S,C,A,_,_,_,"Outstanding"),_,_,_,_))
    => S =/=String C
  rule stageShape(_,_) => false [owise]

  rule <k> s0Intent(submit(I,F,_,_,_,Round) #as R) => s0Effect(R) </k>
    requires matchingScope(I,F) andBool inRound(I,Round) andBool distinct(I)
             andBool intentAmountOK(I)
  rule <k> s0Intent(submit(I,F,_,_,_,_)) => s0Stop("intent","S0_INTENT_SCOPE",1) </k>
    requires notBool matchingScope(I,F)
  rule <k> s0Intent(submit(I,F,_,_,_,Round)) => s0Stop("intent","S0_INTENT_SCOPE",1) </k>
    requires matchingScope(I,F) andBool (notBool inRound(I,Round) orBool notBool intentAmountOK(I))
  rule <k> s0Intent(submit(I,F,_,_,_,Round)) => s0Stop("intent","S0_INTENT_ALIAS",1) </k>
    requires matchingScope(I,F) andBool inRound(I,Round) andBool intentAmountOK(I)
             andBool notBool distinct(I)
  rule <k> s0Intent(_) => s0Stop("intent","S0_INTENT_SCOPE",1) </k> [owise]
  syntax Bool ::= inRound(Intent,Int) [function, total] | distinct(Intent) [function, total]
                | intentAmountOK(Intent) [function, total]
  rule inRound(intent(_,_,_,_,_,_,_,Lo,Hi,_,_,_,_,_,_,_),R)
    => uint(Lo) andBool uint(Hi) andBool Lo <=Int R andBool R <=Int Hi
  rule distinct(intent(_,_,_,S,_,_,_,_,_,R,F,_,_,_,transfer(_,_),_))
    => S =/=String R andBool S =/=String F andBool R =/=String F
  rule distinct(intent(_,_,_,_,_,_,_,_,_,_,_,_,_,_,repay(_,_,_,_,_,_),_)) => true
  rule intentAmountOK(intent(_,_,_,_,_,_,_,_,_,_,_,G,FC,NF,A,_))
    => actionIntentOK(A,G,FC,NF)

  rule <k> s0Effect(submit(I,F,S,E,_,_) #as R) => s0Authority(R) </k>
    requires effectRange(I,S) andBool E ==K expected(I,F,S)
  rule <k> s0Effect(submit(I,_,S,_,_,_)) => s0Stop("effect","S0_EFFECT_RANGE",1) </k>
    requires notBool effectRange(I,S)
  rule <k> s0Effect(_) => s0Stop("effect","S0_EFFECT_MISMATCH",1) </k> [owise]
  syntax Effects ::= expected(Intent,Fill,State) [function]
  rule expected(intent(_,_,_,_,_,_,_,_,_,_,_,_,_,_,transfer(V,F),_) #as I,Fill,_)
    => transferEffects(I,V,F,postOf(Fill))
  rule expected(intent(_,_,_,_,_,_,_,_,_,_,_,_,_,_,repay(_,_,N,_,_,_),_) #as I,Fill,S)
    => repayEffects(I,debtOf(S),N,postOf(Fill))
  syntax Head ::= postOf(Fill) [function]
  rule postOf(fill(_,_,_,_,_,_,_,_,_,_,_,Post)) => Post
  syntax Bool ::= effectRange(Intent,State) [function, total]
  rule effectRange(intent(_,_,_,S,A,_,_,_,_,R,F,_,_,_,transfer(V,FEE),_),
                   state(balance(S,A,OB),balance(R,A,RB),balance(F,A,FB),_,_,_,_,_,_))
    => V +Int FEE <=Int maxU() andBool OB >=Int V +Int FEE
       andBool RB +Int V <=Int maxU() andBool FB +Int FEE <=Int maxU()
  rule effectRange(intent(_,_,_,_,A,_,_,_,_,_,_,_,_,_,repay(ID,Payer,N,1,0,"none"),_),
                   state(balance(Payer,A,OB),balance(C,A,CB),noBalance(),_,
                         obligation(ID,_,C,A,P,AC,O,"Outstanding"),_,_,_,_))
    => N <=Int O andBool OB >=Int N andBool CB +Int N <=Int maxU()
       andBool P +Int AC ==Int O
  rule effectRange(_,_) => false [owise]

  rule <k> s0Authority(submit(I,_,S,_,_,_) #as R) => s0History(R) </k>
    requires authorityOK(I,S)
  rule <k> s0Authority(_) => s0Stop("authority","S0_AUTH_SCOPE",1) </k> [owise]
  syntax Bool ::= authorityOK(Intent,State) [function, total] | premiseOK(Premise,Intent,State,Int,Outcome) [function, total]
  rule authorityOK(intent(_,_,_,S,A,_,_,_,_,_,_,_,_,_,transfer(V,F),_),
                   state(balance(S,A,_),_,_,allowance(S,A,AR,AS),_,_,_,WR,WS))
    => AR >=Int V +Int F andBool AS +Int V +Int F <=Int maxU()
       andBool WR >=Int 1 andBool WS +Int 1 <=Int maxU()
  rule authorityOK(intent(_,_,_,S,A,_,_,_,_,_,_,_,_,_,repay(_,S,N,_,_,_),_),
                   state(balance(S,A,_),_,_,allowance(S,A,AR,AS),
                         obligation(_,S,_,A,_,_,_,_),_,_,WR,WS))
    => AR >=Int N andBool AS +Int N <=Int maxU()
       andBool WR >=Int 1 andBool WS +Int 1 <=Int maxU()
  rule authorityOK(_,_) => false [owise]
  rule premiseOK(authenticated(I,Pre,Round,_,Requested),I,Pre,Round,Requested) => true
  rule premiseOK(_,_,_,_,_) => false [owise]

  rule <k> s0History(submit(I,F,S,_,_,_) #as R) => s0Failure(R) </k>
    <external> X </external>
    requires historyOK(I,S) andBool successorOK(X,I,F)
  rule <k> s0History(submit(I,_,S,_,_,_)) => s0Stop("history","S0_HISTORY_STALE",1) </k>
    requires notBool headOK(I,S)
  rule <k> s0History(submit(I,_,S,_,_,_)) => s0Stop("history","S0_HISTORY_REPLAY",1) </k>
    requires headOK(I,S) andBool notBool historyOK(I,S)
  rule <k> s0History(_) => s0Stop("history","S0_HISTORY_SUCCESSOR",1) </k> [owise]
  syntax Bool ::= headOK(Intent,State) [function, total] | historyOK(Intent,State) [function, total]
                 | successorOK(Premise,Intent,Fill) [function, total]
  rule headOK(intent(_,_,_,_,_,_,H,_,_,_,_,_,_,_,_,_),state(_,_,_,_,_,H,_,_,_)) => true
  rule headOK(_,_) => false [owise]
  rule historyOK(intent(_,_,D,S,_,N,_,_,_,_,_,_,_,_,_,_) #as I,
                 state(_,_,_,_,_,_,Rs,_,_) #as St)
    => headOK(I,St) andBool notBool hasReplay(replayKey(D,S,N),Rs)
  rule successorOK(authenticated(I,_,_,Post,_),I,Fill)
    => Post ==K postOf(Fill) andBool notBool Post ==K signedHead(I)
  rule successorOK(_,_,_) => false [owise]
  syntax Head ::= signedHead(Intent) [function]
  rule signedHead(intent(_,_,_,_,_,_,H,_,_,_,_,_,_,_,_,_)) => H

  rule <k> s0Failure(submit(_,_,_,_,outcome(terminalSuccess(),noEffects(),noDuties()),_) #as R) => s0Commit(R) </k>
  rule <k> s0Failure(_) => s0Stop("failure","S0_FAILURE_UNSUPPORTED",1) </k> [owise]

  rule <k> s0Commit(submit(intent(_,_,D,S,A,N,H,_,_,R,F,_,_,_,transfer(V,FEE),_),Fill,state(balance(S,A,OB),balance(R,A,RB),balance(F,A,FB),
                                     allowance(S,A,AR,AS),O,H,Rs,WR,WS) #as Pre,E,outcome(terminalSuccess(),noEffects(),noDuties()),_))
       => .K </k>
       <out> pending() => accepted(Pre,E,
         state(balance(S,A,OB -Int V -Int FEE),balance(R,A,RB +Int V),
               balance(F,A,FB +Int FEE),allowance(S,A,AR -Int V -Int FEE,AS +Int V +Int FEE),
               O,postOf(Fill),used(replayKey(D,S,N),Rs),WR -Int 1,WS +Int 1),
         terminalSuccess(),noDuty(),WR -Int 1) </out>
  rule <k> s0Commit(submit(intent(_,_,D,S,A,N,H,_,_,_,_,_,_,_,repay(ID,S,PAY,1,0,"none"),_),Fill,state(balance(S,A,OB),balance(C,A,CB),noBalance(),
                                     allowance(S,A,AR,AS),obligation(ID,S,C,A,P,AC,O,"Outstanding"),
                                     H,Rs,WR,WS) #as Pre,E,outcome(terminalSuccess(),noEffects(),noDuties()),_))
       => .K </k>
       <out> pending() => accepted(Pre,E,
         state(balance(S,A,OB -Int PAY),balance(C,A,CB +Int PAY),noBalance(),
               allowance(S,A,AR -Int PAY,AS +Int PAY),
               obligation(ID,S,C,A,P -Int (PAY -Int accruedPaid(PAY,AC)),
                          AC -Int accruedPaid(PAY,AC),O -Int PAY,debtStatus(O -Int PAY)),
               postOf(Fill),used(replayKey(D,S,N),Rs),WR -Int 1,WS +Int 1),
         terminalSuccess(),noDuty(),WR -Int 1) </out>
endmodule

```

## experiments/moriarty-language/formal/k/mil4/corpus/s1b_run.py

```text
#!/usr/bin/env python3
"""Strict H/F/P experiment under explicit stipulated tuples, never authentication.

Every case uses T-10-1's nonce/intent and round zero unless a named mutation
changes it. Expected accepted output is the independent literal T-10-1 term.
"""
import json
from pathlib import Path

from run import call, effects_term, q, transfer_case
from run_checked import EXPECTED_ACCEPTED, check_case

HERE = Path(__file__).resolve().parent


def case(name, **changes):
    result = transfer_case('T-10-1', round=0, **changes)
    result['name'] = name
    return result


def unavailable(name, **changes):
    result = case(name, **changes)
    result['premise'] = 'unavailable()'
    return result


def mismatched_premise(name, field):
    result = case(name)
    intent, pre, round = result['intent'], result['preState'], result['round']
    outcome = result['outcome']
    if field == 'intent':
        intent = intent.replace(q('digest-T-10-1'), q('digest-other'))
    elif field == 'preState':
        pre = pre.replace('balance("O","A",100)', 'balance("O","A",99)')
    elif field == 'round':
        round = 1
    elif field == 'outcome':
        outcome = call('outcome', call('requestedFailure', q('failure')), 'noEffects()', 'noDuties()')
    else:
        raise ValueError(field)
    result['premise'] = call('authenticated', intent, pre, round, 'head("h1")', outcome)
    return result


def s1b_cases():
    failed = call('outcome', call('requestedFailure', q('failure')), 'noEffects()', 'noDuties()')
    retained_effect = call('outcome', 'terminalSuccess()',
                           effects_term([call('credit', q('R'), q('A'), 1)]), 'noDuties()')
    retained_duty = call('outcome', 'terminalSuccess()', 'noEffects()',
                         call('duty', q('duty'), 'noDuties()'))
    consumed = call('used', call('replayKey', q('D'), q('O'), q('T-10-1')), 'noReplays()')
    return [
        (case('H1'), 'accepted'),
        (case('H2', post_head='h9'), ('history', 'S0_HISTORY_SUCCESSOR')),
        (case('H3', post_head='h0'), ('history', 'S0_HISTORY_SUCCESSOR')),
        (case('H3-self-premise', post_head='h0', premise_post='h0'), ('history', 'S0_HISTORY_SUCCESSOR')),
        (case('H4', effect_head='h9'), ('effect', 'S0_EFFECT_MISMATCH')),
        (case('H5', head='h9', state_head='h0', post_head='h9'), ('history', 'S0_HISTORY_STALE')),
        (case('H6', replay=consumed, post_head='h9'), ('history', 'S0_HISTORY_REPLAY')),
        (case('F1', outcome=failed), ('failure', 'S0_FAILURE_UNSUPPORTED')),
        (case('F2', outcome=retained_effect), ('failure', 'S0_FAILURE_UNSUPPORTED')),
        (case('F3', outcome=retained_duty), ('failure', 'S0_FAILURE_UNSUPPORTED')),
        (unavailable('P1'), ('stage', 'S0_STAGE_PREMISE')),
        (mismatched_premise('P2-intent', 'intent'), ('stage', 'S0_STAGE_PREMISE')),
        (mismatched_premise('P2-preState', 'preState'), ('stage', 'S0_STAGE_PREMISE')),
        (mismatched_premise('P2-round', 'round'), ('stage', 'S0_STAGE_PREMISE')),
        (unavailable('P3', owner_cell=None), ('stage', 'S0_STAGE_UNSUPPORTED')),
        # Extra outcome binding control beyond the 14 required H/F/P instances.
        (mismatched_premise('P2-outcome', 'outcome'), ('stage', 'S0_STAGE_PREMISE')),
        # Combined controls pin the first observation, not just source rule order.
        (case('H7-stale-replay', head='h9', state_head='h0', replay=consumed),
         ('history', 'S0_HISTORY_STALE')),
        (case('H8-effect-replay', effect_head='h9', replay=consumed),
         ('effect', 'S0_EFFECT_MISMATCH')),
        (case('H9-effect-successor', post_head='h9', effect_head='h1'),
         ('effect', 'S0_EFFECT_MISMATCH')),
        (case('F4-effect-failure', effect_head='h9', outcome=failed),
         ('effect', 'S0_EFFECT_MISMATCH')),
    ]


def main():
    cases = s1b_cases()
    required = {'H1', 'H2', 'H3', 'H4', 'H5', 'H6', 'F1', 'F2', 'F3',
                'P1', 'P2-intent', 'P2-preState', 'P2-round', 'P3', 'P2-outcome', 'H3-self-premise',
                'H7-stale-replay', 'H8-effect-replay', 'H9-effect-successor', 'F4-effect-failure'}
    names = [entry['name'] for entry, _ in cases]
    if len(names) != len(required) or set(names) != required:
        raise ValueError('missing, extra, or duplicate experiment case')
    results = []
    for entry, verdict in cases:
        expected = (EXPECTED_ACCEPTED['T-10-1'] if verdict == 'accepted'
                    else call('rejected', q(verdict[0]), q(verdict[1]), 1))
        result = check_case(entry, expected)
        results.append(result)
        print(f'{entry["name"]}: {"MATCH" if result["matched"] else "MISMATCH"}', flush=True)
    (HERE / 's1b-results.json').write_text(json.dumps(results, indent=2) + '\n')
    return 0 if all(entry['matched'] for entry in results) else 1


if __name__ == '__main__':
    raise SystemExit(main())

```

## experiments/moriarty-language/formal/k/mil4/corpus/s1b-verification.json

```text
{
  "scope": "Finite local K controls under a stipulated tuple; no authentication, native proof, ledger admission, or universal correspondence. W-D3 remains open.",
  "compiledSourcesUnchanged": true,
  "compileExitCode": 0,
  "originalCompleteObservationsPreserved": 37,
  "originalExpectedCount": 37,
  "preAuditS1BProgramsAndCompleteObservationsPreserved": 16,
  "preAuditS1BExpectedCount": 16,
  "combinedFaultInputs": {
    "H7-stale-replay": {
      "stale": true,
      "replay": true,
      "effectMismatch": false,
      "wrongSuccessor": false,
      "failureRequested": false,
      "outcomeBound": true
    },
    "H8-effect-replay": {
      "stale": false,
      "replay": true,
      "effectMismatch": true,
      "wrongSuccessor": false,
      "failureRequested": false,
      "outcomeBound": true
    },
    "H9-effect-successor": {
      "stale": false,
      "replay": false,
      "effectMismatch": true,
      "wrongSuccessor": true,
      "failureRequested": false,
      "outcomeBound": true
    },
    "F4-effect-failure": {
      "stale": false,
      "replay": false,
      "effectMismatch": true,
      "wrongSuccessor": false,
      "failureRequested": true,
      "outcomeBound": true
    }
  },
  "runs": {
    "s1b-regression-results.json": {
      "expectedCount": 37,
      "actualCount": 37,
      "matched": 37,
      "externalPreserved": 37,
      "sha256": "9ba861ec1ab85979b2ef7278687914496b845ae90c433ed9afaffc18eb668c24"
    },
    "s1b-results.json": {
      "expectedCount": 20,
      "actualCount": 20,
      "matched": 20,
      "externalPreserved": 20,
      "sha256": "0c5e8ede144a0e7aa157dec8981278c868cf1c62a3e3fb84b0f9a10d5c6dc34e"
    },
    "s1b-common-results.json": {
      "expectedCount": 3,
      "actualCount": 3,
      "matched": 3,
      "externalPreserved": 3,
      "sha256": "c69c709a23dc90c947963cf6f740540a195f58b29f69f90d11f2b8c68d0b95bd"
    }
  }
}

```

## deliverables/mil4-k-quint-sprint1-2026-09-29/audits/s1b-postreview-k-observations.json

```text
[
  {
    "case": "H1",
    "matched": true,
    "expectedOut": "accepted(\n      state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),\n            allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),\n      effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),\n        effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),\n          effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),\n            effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),\n      state(balance(\"O\",\"A\",89),balance(\"R\",\"A\",10),balance(\"F\",\"A\",1),\n            allowance(\"O\",\"A\",0,11),noObligation(),head(\"h1\"),\n            used(replayKey(\"D\",\"O\",\"T-10-1\"),noReplays()),0,1),\n      terminalSuccess(),noDuty(),0)",
    "observedOut": "accepted ( state ( balance ( \"O\" , \"A\" , 100 ) , balance ( \"R\" , \"A\" , 0 ) , balance ( \"F\" , \"A\" , 0 ) , allowance ( \"O\" , \"A\" , 11 , 0 ) , noObligation ( ) , head ( \"h0\" ) , noReplays ( ) , 1 , 0 ) , effect ( debit ( \"O\" , \"A\" , 11 ) , effect ( credit ( \"R\" , \"A\" , 10 ) , effect ( credit ( \"F\" , \"A\" , 1 ) , effect ( useAllowance ( \"O\" , \"A\" , 11 ) , effect ( useReplay ( replayKey ( \"D\" , \"O\" , \"T-10-1\" ) ) , effect ( advanceHead ( head ( \"h0\" ) , head ( \"h1\" ) ) , noEffects ( ) ) ) ) ) ) ) , state ( balance ( \"O\" , \"A\" , 89 ) , balance ( \"R\" , \"A\" , 10 ) , balance ( \"F\" , \"A\" , 1 ) , allowance ( \"O\" , \"A\" , 0 , 11 ) , noObligation ( ) , head ( \"h1\" ) , used ( replayKey ( \"D\" , \"O\" , \"T-10-1\" ) , noReplays ( ) ) , 0 , 1 ) , terminalSuccess ( ) , noDuty ( ) , 0 )",
    "externalPreserved": true,
    "preState": "state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h1\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),0,head(\"h1\"),outcome(terminalSuccess(),noEffects(),noDuties()))",
    "stderr": ""
  },
  {
    "case": "H2",
    "matched": true,
    "expectedOut": "rejected(\"history\",\"S0_HISTORY_SUCCESSOR\",1)",
    "observedOut": "rejected ( \"history\" , \"S0_HISTORY_SUCCESSOR\" , 1 )",
    "externalPreserved": true,
    "preState": "state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h9\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h0\"),head(\"h9\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),0,head(\"h1\"),outcome(terminalSuccess(),noEffects(),noDuties()))",
    "stderr": ""
  },
  {
    "case": "H3",
    "matched": true,
    "expectedOut": "rejected(\"history\",\"S0_HISTORY_SUCCESSOR\",1)",
    "observedOut": "rejected ( \"history\" , \"S0_HISTORY_SUCCESSOR\" , 1 )",
    "externalPreserved": true,
    "preState": "state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h0\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h0\"),head(\"h0\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),0,head(\"h1\"),outcome(terminalSuccess(),noEffects(),noDuties()))",
    "stderr": ""
  },
  {
    "case": "H3-self-premise",
    "matched": true,
    "expectedOut": "rejected(\"history\",\"S0_HISTORY_SUCCESSOR\",1)",
    "observedOut": "rejected ( \"history\" , \"S0_HISTORY_SUCCESSOR\" , 1 )",
    "externalPreserved": true,
    "preState": "state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h0\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h0\"),head(\"h0\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),0,head(\"h0\"),outcome(terminalSuccess(),noEffects(),noDuties()))",
    "stderr": ""
  },
  {
    "case": "H4",
    "matched": true,
    "expectedOut": "rejected(\"effect\",\"S0_EFFECT_MISMATCH\",1)",
    "observedOut": "rejected ( \"effect\" , \"S0_EFFECT_MISMATCH\" , 1 )",
    "externalPreserved": true,
    "preState": "state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h1\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h0\"),head(\"h9\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),0,head(\"h1\"),outcome(terminalSuccess(),noEffects(),noDuties()))",
    "stderr": ""
  },
  {
    "case": "H5",
    "matched": true,
    "expectedOut": "rejected(\"history\",\"S0_HISTORY_STALE\",1)",
    "observedOut": "rejected ( \"history\" , \"S0_HISTORY_STALE\" , 1 )",
    "externalPreserved": true,
    "preState": "state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h9\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h9\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h9\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h9\"),head(\"h9\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h9\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),0,head(\"h1\"),outcome(terminalSuccess(),noEffects(),noDuties()))",
    "stderr": ""
  },
  {
    "case": "H6",
    "matched": true,
    "expectedOut": "rejected(\"history\",\"S0_HISTORY_REPLAY\",1)",
    "observedOut": "rejected ( \"history\" , \"S0_HISTORY_REPLAY\" , 1 )",
    "externalPreserved": true,
    "preState": "state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),used(replayKey(\"D\",\"O\",\"T-10-1\"),noReplays()),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h9\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),used(replayKey(\"D\",\"O\",\"T-10-1\"),noReplays()),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h0\"),head(\"h9\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),used(replayKey(\"D\",\"O\",\"T-10-1\"),noReplays()),1,0),0,head(\"h1\"),outcome(terminalSuccess(),noEffects(),noDuties()))",
    "stderr": ""
  },
  {
    "case": "F1",
    "matched": true,
    "expectedOut": "rejected(\"failure\",\"S0_FAILURE_UNSUPPORTED\",1)",
    "observedOut": "rejected ( \"failure\" , \"S0_FAILURE_UNSUPPORTED\" , 1 )",
    "externalPreserved": true,
    "preState": "state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(requestedFailure(\"failure\"),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h1\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),outcome(requestedFailure(\"failure\"),noEffects(),noDuties()),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),0,head(\"h1\"),outcome(requestedFailure(\"failure\"),noEffects(),noDuties()))",
    "stderr": ""
  },
  {
    "case": "F2",
    "matched": true,
    "expectedOut": "rejected(\"failure\",\"S0_FAILURE_UNSUPPORTED\",1)",
    "observedOut": "rejected ( \"failure\" , \"S0_FAILURE_UNSUPPORTED\" , 1 )",
    "externalPreserved": true,
    "preState": "state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),effect(credit(\"R\",\"A\",1),noEffects()),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h1\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),outcome(terminalSuccess(),effect(credit(\"R\",\"A\",1),noEffects()),noDuties()),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),0,head(\"h1\"),outcome(terminalSuccess(),effect(credit(\"R\",\"A\",1),noEffects()),noDuties()))",
    "stderr": ""
  },
  {
    "case": "F3",
    "matched": true,
    "expectedOut": "rejected(\"failure\",\"S0_FAILURE_UNSUPPORTED\",1)",
    "observedOut": "rejected ( \"failure\" , \"S0_FAILURE_UNSUPPORTED\" , 1 )",
    "externalPreserved": true,
    "preState": "state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),noEffects(),duty(\"duty\",noDuties()))",
    "parseError": null,
    "exitCode": 0,
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h1\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),duty(\"duty\",noDuties())),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),0,head(\"h1\"),outcome(terminalSuccess(),noEffects(),duty(\"duty\",noDuties())))",
    "stderr": ""
  },
  {
    "case": "P1",
    "matched": true,
    "expectedOut": "rejected(\"stage\",\"S0_STAGE_PREMISE\",1)",
    "observedOut": "rejected ( \"stage\" , \"S0_STAGE_PREMISE\" , 1 )",
    "externalPreserved": true,
    "preState": "state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h1\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0)",
    "premise": "unavailable()",
    "stderr": ""
  },
  {
    "case": "P2-intent",
    "matched": true,
    "expectedOut": "rejected(\"stage\",\"S0_STAGE_PREMISE\",1)",
    "observedOut": "rejected ( \"stage\" , \"S0_STAGE_PREMISE\" , 1 )",
    "externalPreserved": true,
    "preState": "state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h1\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-other\"),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),0,head(\"h1\"),outcome(terminalSuccess(),noEffects(),noDuties()))",
    "stderr": ""
  },
  {
    "case": "P2-preState",
    "matched": true,
    "expectedOut": "rejected(\"stage\",\"S0_STAGE_PREMISE\",1)",
    "observedOut": "rejected ( \"stage\" , \"S0_STAGE_PREMISE\" , 1 )",
    "externalPreserved": true,
    "preState": "state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h1\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),state(balance(\"O\",\"A\",99),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),0,head(\"h1\"),outcome(terminalSuccess(),noEffects(),noDuties()))",
    "stderr": ""
  },
  {
    "case": "P2-round",
    "matched": true,
    "expectedOut": "rejected(\"stage\",\"S0_STAGE_PREMISE\",1)",
    "observedOut": "rejected ( \"stage\" , \"S0_STAGE_PREMISE\" , 1 )",
    "externalPreserved": true,
    "preState": "state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h1\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),1,head(\"h1\"),outcome(terminalSuccess(),noEffects(),noDuties()))",
    "stderr": ""
  },
  {
    "case": "P3",
    "matched": true,
    "expectedOut": "rejected(\"stage\",\"S0_STAGE_UNSUPPORTED\",1)",
    "observedOut": "rejected ( \"stage\" , \"S0_STAGE_UNSUPPORTED\" , 1 )",
    "externalPreserved": true,
    "preState": "state(noBalance(),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h1\")),state(noBalance(),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0)",
    "premise": "unavailable()",
    "stderr": ""
  },
  {
    "case": "P2-outcome",
    "matched": true,
    "expectedOut": "rejected(\"stage\",\"S0_STAGE_PREMISE\",1)",
    "observedOut": "rejected ( \"stage\" , \"S0_STAGE_PREMISE\" , 1 )",
    "externalPreserved": true,
    "preState": "state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h1\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),0,head(\"h1\"),outcome(requestedFailure(\"failure\"),noEffects(),noDuties()))",
    "stderr": ""
  },
  {
    "case": "H7-stale-replay",
    "matched": true,
    "expectedOut": "rejected(\"history\",\"S0_HISTORY_STALE\",1)",
    "observedOut": "rejected ( \"history\" , \"S0_HISTORY_STALE\" , 1 )",
    "externalPreserved": true,
    "preState": "state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),used(replayKey(\"D\",\"O\",\"T-10-1\"),noReplays()),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h9\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h9\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h1\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),used(replayKey(\"D\",\"O\",\"T-10-1\"),noReplays()),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h9\"),head(\"h1\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h9\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),used(replayKey(\"D\",\"O\",\"T-10-1\"),noReplays()),1,0),0,head(\"h1\"),outcome(terminalSuccess(),noEffects(),noDuties()))",
    "stderr": ""
  },
  {
    "case": "H8-effect-replay",
    "matched": true,
    "expectedOut": "rejected(\"effect\",\"S0_EFFECT_MISMATCH\",1)",
    "observedOut": "rejected ( \"effect\" , \"S0_EFFECT_MISMATCH\" , 1 )",
    "externalPreserved": true,
    "preState": "state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),used(replayKey(\"D\",\"O\",\"T-10-1\"),noReplays()),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h1\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),used(replayKey(\"D\",\"O\",\"T-10-1\"),noReplays()),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h0\"),head(\"h9\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),used(replayKey(\"D\",\"O\",\"T-10-1\"),noReplays()),1,0),0,head(\"h1\"),outcome(terminalSuccess(),noEffects(),noDuties()))",
    "stderr": ""
  },
  {
    "case": "H9-effect-successor",
    "matched": true,
    "expectedOut": "rejected(\"effect\",\"S0_EFFECT_MISMATCH\",1)",
    "observedOut": "rejected ( \"effect\" , \"S0_EFFECT_MISMATCH\" , 1 )",
    "externalPreserved": true,
    "preState": "state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(terminalSuccess(),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h9\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),outcome(terminalSuccess(),noEffects(),noDuties()),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),0,head(\"h1\"),outcome(terminalSuccess(),noEffects(),noDuties()))",
    "stderr": ""
  },
  {
    "case": "F4-effect-failure",
    "matched": true,
    "expectedOut": "rejected(\"effect\",\"S0_EFFECT_MISMATCH\",1)",
    "observedOut": "rejected ( \"effect\" , \"S0_EFFECT_MISMATCH\" , 1 )",
    "externalPreserved": true,
    "preState": "state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0)",
    "round": 0,
    "requestedOutcome": "outcome(requestedFailure(\"failure\"),noEffects(),noDuties())",
    "parseError": null,
    "exitCode": 0,
    "request": "submit(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),fill(\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),\"R\",\"F\",11,1,0,transfer(10,1),head(\"h1\")),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),effect(advanceHead(head(\"h0\"),head(\"h9\")),noEffects())))))),outcome(requestedFailure(\"failure\"),noEffects(),noDuties()),0)",
    "premise": "authenticated(intent(\"Source/6\",\"Core/5\",\"D\",\"O\",\"A\",\"T-10-1\",head(\"h0\"),0,10,\"R\",\"F\",11,1,0,transfer(10,1),\"digest-T-10-1\"),state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),0,head(\"h1\"),outcome(requestedFailure(\"failure\"),noEffects(),noDuties()))",
    "stderr": ""
  }
]

```

## deliverables/mil4-k-quint-sprint1-2026-09-29/audits/s1b-postreview-k-regression-observations.json

```text
[
  {
    "case": "T-10-1",
    "matched": true,
    "expectedOut": "accepted(\n      state(balance(\"O\",\"A\",100),balance(\"R\",\"A\",0),balance(\"F\",\"A\",0),\n            allowance(\"O\",\"A\",11,0),noObligation(),head(\"h0\"),noReplays(),1,0),\n      effect(debit(\"O\",\"A\",11),effect(credit(\"R\",\"A\",10),\n        effect(credit(\"F\",\"A\",1),effect(useAllowance(\"O\",\"A\",11),\n          effect(useReplay(replayKey(\"D\",\"O\",\"T-10-1\")),\n            effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),\n      state(balance(\"O\",\"A\",89),balance(\"R\",\"A\",10),balance(\"F\",\"A\",1),\n            allowance(\"O\",\"A\",0,11),noObligation(),head(\"h1\"),\n            used(replayKey(\"D\",\"O\",\"T-10-1\"),noReplays()),0,1),\n      terminalSuccess(),noDuty(),0)",
    "observedOut": "accepted ( state ( balance ( \"O\" , \"A\" , 100 ) , balance ( \"R\" , \"A\" , 0 ) , balance ( \"F\" , \"A\" , 0 ) , allowance ( \"O\" , \"A\" , 11 , 0 ) , noObligation ( ) , head ( \"h0\" ) , noReplays ( ) , 1 , 0 ) , effect ( debit ( \"O\" , \"A\" , 11 ) , effect ( credit ( \"R\" , \"A\" , 10 ) , effect ( credit ( \"F\" , \"A\" , 1 ) , effect ( useAllowance ( \"O\" , \"A\" , 11 ) , effect ( useReplay ( replayKey ( \"D\" , \"O\" , \"T-10-1\" ) ) , effect ( advanceHead ( head ( \"h0\" ) , head ( \"h1\" ) ) , noEffects ( ) ) ) ) ) ) ) , state ( balance ( \"O\" , \"A\" , 89 ) , balance ( \"R\" , \"A\" , 10 ) , balance ( \"F\" , \"A\" , 1 ) , allowance ( \"O\" , \"A\" , 0 , 11 ) , noObligation ( ) , head ( \"h1\" ) , used ( replayKey ( \"D\" , \"O\" , \"T-10-1\" ) , noReplays ( ) ) , 0 , 1 ) , terminalSuccess ( ) , noDuty ( ) , 0 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "R-30",
    "matched": true,
    "expectedOut": "accepted(\n      state(balance(\"P\",\"A\",100),balance(\"C\",\"A\",0),noBalance(),\n            allowance(\"P\",\"A\",100,0),\n            obligation(\"L\",\"P\",\"C\",\"A\",1000,10,1010,\"Outstanding\"),\n            head(\"h0\"),noReplays(),1,0),\n      effect(debit(\"P\",\"A\",30),effect(credit(\"C\",\"A\",30),\n        effect(setObligation(\"L\",980,0,980,\"Outstanding\"),\n          effect(useAllowance(\"P\",\"A\",30),\n            effect(useReplay(replayKey(\"D\",\"P\",\"R-30\")),\n              effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),\n      state(balance(\"P\",\"A\",70),balance(\"C\",\"A\",30),noBalance(),\n            allowance(\"P\",\"A\",70,30),\n            obligation(\"L\",\"P\",\"C\",\"A\",980,0,980,\"Outstanding\"),\n            head(\"h1\"),used(replayKey(\"D\",\"P\",\"R-30\"),noReplays()),0,1),\n      terminalSuccess(),noDuty(),0)",
    "observedOut": "accepted ( state ( balance ( \"P\" , \"A\" , 100 ) , balance ( \"C\" , \"A\" , 0 ) , noBalance ( ) , allowance ( \"P\" , \"A\" , 100 , 0 ) , obligation ( \"L\" , \"P\" , \"C\" , \"A\" , 1000 , 10 , 1010 , \"Outstanding\" ) , head ( \"h0\" ) , noReplays ( ) , 1 , 0 ) , effect ( debit ( \"P\" , \"A\" , 30 ) , effect ( credit ( \"C\" , \"A\" , 30 ) , effect ( setObligation ( \"L\" , 980 , 0 , 980 , \"Outstanding\" ) , effect ( useAllowance ( \"P\" , \"A\" , 30 ) , effect ( useReplay ( replayKey ( \"D\" , \"P\" , \"R-30\" ) ) , effect ( advanceHead ( head ( \"h0\" ) , head ( \"h1\" ) ) , noEffects ( ) ) ) ) ) ) ) , state ( balance ( \"P\" , \"A\" , 70 ) , balance ( \"C\" , \"A\" , 30 ) , noBalance ( ) , allowance ( \"P\" , \"A\" , 70 , 30 ) , obligation ( \"L\" , \"P\" , \"C\" , \"A\" , 980 , 0 , 980 , \"Outstanding\" ) , head ( \"h1\" ) , used ( replayKey ( \"D\" , \"P\" , \"R-30\" ) , noReplays ( ) ) , 0 , 1 ) , terminalSuccess ( ) , noDuty ( ) , 0 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "R-near-bound",
    "matched": true,
    "expectedOut": "accepted(\n      state(balance(\"P\",\"A\",1),\n            balance(\"C\",\"A\",340282366920938463463374607431768211454),\n            noBalance(),\n            allowance(\"P\",\"A\",1,340282366920938463463374607431768211454),\n            obligation(\"L\",\"P\",\"C\",\"A\",170141183460469231731687303715884105726,\n                       1,170141183460469231731687303715884105727,\"Outstanding\"),\n            head(\"h0\"),noReplays(),1,0),\n      effect(debit(\"P\",\"A\",1),effect(credit(\"C\",\"A\",1),\n        effect(setObligation(\"L\",170141183460469231731687303715884105726,\n                             0,170141183460469231731687303715884105726,\n                             \"Outstanding\"),\n          effect(useAllowance(\"P\",\"A\",1),\n            effect(useReplay(replayKey(\"D\",\"P\",\"R-near-bound\")),\n              effect(advanceHead(head(\"h0\"),head(\"h1\")),noEffects())))))),\n      state(balance(\"P\",\"A\",0),\n            balance(\"C\",\"A\",340282366920938463463374607431768211455),\n            noBalance(),\n            allowance(\"P\",\"A\",0,340282366920938463463374607431768211455),\n            obligation(\"L\",\"P\",\"C\",\"A\",170141183460469231731687303715884105726,\n                       0,170141183460469231731687303715884105726,\"Outstanding\"),\n            head(\"h1\"),used(replayKey(\"D\",\"P\",\"R-near-bound\"),noReplays()),0,1),\n      terminalSuccess(),noDuty(),0)",
    "observedOut": "accepted ( state ( balance ( \"P\" , \"A\" , 1 ) , balance ( \"C\" , \"A\" , 340282366920938463463374607431768211454 ) , noBalance ( ) , allowance ( \"P\" , \"A\" , 1 , 340282366920938463463374607431768211454 ) , obligation ( \"L\" , \"P\" , \"C\" , \"A\" , 170141183460469231731687303715884105726 , 1 , 170141183460469231731687303715884105727 , \"Outstanding\" ) , head ( \"h0\" ) , noReplays ( ) , 1 , 0 ) , effect ( debit ( \"P\" , \"A\" , 1 ) , effect ( credit ( \"C\" , \"A\" , 1 ) , effect ( setObligation ( \"L\" , 170141183460469231731687303715884105726 , 0 , 170141183460469231731687303715884105726 , \"Outstanding\" ) , effect ( useAllowance ( \"P\" , \"A\" , 1 ) , effect ( useReplay ( replayKey ( \"D\" , \"P\" , \"R-near-bound\" ) ) , effect ( advanceHead ( head ( \"h0\" ) , head ( \"h1\" ) ) , noEffects ( ) ) ) ) ) ) ) , state ( balance ( \"P\" , \"A\" , 0 ) , balance ( \"C\" , \"A\" , 340282366920938463463374607431768211455 ) , noBalance ( ) , allowance ( \"P\" , \"A\" , 0 , 340282366920938463463374607431768211455 ) , obligation ( \"L\" , \"P\" , \"C\" , \"A\" , 170141183460469231731687303715884105726 , 0 , 170141183460469231731687303715884105726 , \"Outstanding\" ) , head ( \"h1\" ) , used ( replayKey ( \"D\" , \"P\" , \"R-near-bound\" ) , noReplays ( ) ) , 0 , 1 ) , terminalSuccess ( ) , noDuty ( ) , 0 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-recipient",
    "matched": true,
    "expectedOut": "rejected(\"intent\",\"S0_INTENT_SCOPE\",1)",
    "observedOut": "rejected ( \"intent\" , \"S0_INTENT_SCOPE\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-fee-cap",
    "matched": true,
    "expectedOut": "rejected(\"intent\",\"S0_INTENT_SCOPE\",1)",
    "observedOut": "rejected ( \"intent\" , \"S0_INTENT_SCOPE\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-missing-fee",
    "matched": true,
    "expectedOut": "rejected(\"effect\",\"S0_EFFECT_MISMATCH\",1)",
    "observedOut": "rejected ( \"effect\" , \"S0_EFFECT_MISMATCH\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-missing-credit",
    "matched": true,
    "expectedOut": "rejected(\"effect\",\"S0_EFFECT_MISMATCH\",1)",
    "observedOut": "rejected ( \"effect\" , \"S0_EFFECT_MISMATCH\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-wrong-creditor",
    "matched": true,
    "expectedOut": "rejected(\"effect\",\"S0_EFFECT_MISMATCH\",1)",
    "observedOut": "rejected ( \"effect\" , \"S0_EFFECT_MISMATCH\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-allowance",
    "matched": true,
    "expectedOut": "rejected(\"authority\",\"S0_AUTH_SCOPE\",1)",
    "observedOut": "rejected ( \"authority\" , \"S0_AUTH_SCOPE\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-stale-head",
    "matched": true,
    "expectedOut": "rejected(\"history\",\"S0_HISTORY_STALE\",1)",
    "observedOut": "rejected ( \"history\" , \"S0_HISTORY_STALE\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-replay",
    "matched": true,
    "expectedOut": "rejected(\"history\",\"S0_HISTORY_REPLAY\",1)",
    "observedOut": "rejected ( \"history\" , \"S0_HISTORY_REPLAY\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-overflow",
    "matched": true,
    "expectedOut": "rejected(\"effect\",\"S0_EFFECT_RANGE\",1)",
    "observedOut": "rejected ( \"effect\" , \"S0_EFFECT_RANGE\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-overpay",
    "matched": true,
    "expectedOut": "rejected(\"effect\",\"S0_EFFECT_RANGE\",1)",
    "observedOut": "rejected ( \"effect\" , \"S0_EFFECT_RANGE\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-both",
    "matched": true,
    "expectedOut": "rejected(\"effect\",\"S0_EFFECT_MISMATCH\",1)",
    "observedOut": "rejected ( \"effect\" , \"S0_EFFECT_MISMATCH\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-missing-owner-cell",
    "matched": true,
    "expectedOut": "rejected(\"stage\",\"S0_STAGE_UNSUPPORTED\",1)",
    "observedOut": "rejected ( \"stage\" , \"S0_STAGE_UNSUPPORTED\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-missing-recipient-cell",
    "matched": true,
    "expectedOut": "rejected(\"stage\",\"S0_STAGE_UNSUPPORTED\",1)",
    "observedOut": "rejected ( \"stage\" , \"S0_STAGE_UNSUPPORTED\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-missing-fee-cell",
    "matched": true,
    "expectedOut": "rejected(\"stage\",\"S0_STAGE_UNSUPPORTED\",1)",
    "observedOut": "rejected ( \"stage\" , \"S0_STAGE_UNSUPPORTED\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-wrong-owner-cell",
    "matched": true,
    "expectedOut": "rejected(\"stage\",\"S0_STAGE_UNSUPPORTED\",1)",
    "observedOut": "rejected ( \"stage\" , \"S0_STAGE_UNSUPPORTED\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-wrong-recipient-cell",
    "matched": true,
    "expectedOut": "rejected(\"stage\",\"S0_STAGE_UNSUPPORTED\",1)",
    "observedOut": "rejected ( \"stage\" , \"S0_STAGE_UNSUPPORTED\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-wrong-fee-cell",
    "matched": true,
    "expectedOut": "rejected(\"stage\",\"S0_STAGE_UNSUPPORTED\",1)",
    "observedOut": "rejected ( \"stage\" , \"S0_STAGE_UNSUPPORTED\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-swapped-transfer-cells",
    "matched": true,
    "expectedOut": "rejected(\"stage\",\"S0_STAGE_UNSUPPORTED\",1)",
    "observedOut": "rejected ( \"stage\" , \"S0_STAGE_UNSUPPORTED\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-wrong-cell-asset",
    "matched": true,
    "expectedOut": "rejected(\"stage\",\"S0_STAGE_UNSUPPORTED\",1)",
    "observedOut": "rejected ( \"stage\" , \"S0_STAGE_UNSUPPORTED\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-wrong-allowance-owner",
    "matched": true,
    "expectedOut": "rejected(\"stage\",\"S0_STAGE_UNSUPPORTED\",1)",
    "observedOut": "rejected ( \"stage\" , \"S0_STAGE_UNSUPPORTED\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-wrong-allowance-asset",
    "matched": true,
    "expectedOut": "rejected(\"stage\",\"S0_STAGE_UNSUPPORTED\",1)",
    "observedOut": "rejected ( \"stage\" , \"S0_STAGE_UNSUPPORTED\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-debtor-mismatch",
    "matched": true,
    "expectedOut": "rejected(\"stage\",\"S0_STAGE_UNSUPPORTED\",1)",
    "observedOut": "rejected ( \"stage\" , \"S0_STAGE_UNSUPPORTED\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-payer-signer-mismatch",
    "matched": true,
    "expectedOut": "rejected(\"stage\",\"S0_STAGE_UNSUPPORTED\",1)",
    "observedOut": "rejected ( \"stage\" , \"S0_STAGE_UNSUPPORTED\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-repay-allowance-owner",
    "matched": true,
    "expectedOut": "rejected(\"stage\",\"S0_STAGE_UNSUPPORTED\",1)",
    "observedOut": "rejected ( \"stage\" , \"S0_STAGE_UNSUPPORTED\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-alias-owner-recipient",
    "matched": true,
    "expectedOut": "rejected(\"intent\",\"S0_INTENT_ALIAS\",1)",
    "observedOut": "rejected ( \"intent\" , \"S0_INTENT_ALIAS\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-alias-owner-fee",
    "matched": true,
    "expectedOut": "rejected(\"intent\",\"S0_INTENT_ALIAS\",1)",
    "observedOut": "rejected ( \"intent\" , \"S0_INTENT_ALIAS\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-alias-recipient-fee",
    "matched": true,
    "expectedOut": "rejected(\"intent\",\"S0_INTENT_ALIAS\",1)",
    "observedOut": "rejected ( \"intent\" , \"S0_INTENT_ALIAS\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-invalid-round-alias",
    "matched": true,
    "expectedOut": "rejected(\"intent\",\"S0_INTENT_SCOPE\",1)",
    "observedOut": "rejected ( \"intent\" , \"S0_INTENT_SCOPE\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-cap-alias",
    "matched": true,
    "expectedOut": "rejected(\"intent\",\"S0_INTENT_SCOPE\",1)",
    "observedOut": "rejected ( \"intent\" , \"S0_INTENT_SCOPE\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-overflow-missing-fee",
    "matched": true,
    "expectedOut": "rejected(\"effect\",\"S0_EFFECT_RANGE\",1)",
    "observedOut": "rejected ( \"effect\" , \"S0_EFFECT_RANGE\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-overpay-missing-credit",
    "matched": true,
    "expectedOut": "rejected(\"effect\",\"S0_EFFECT_RANGE\",1)",
    "observedOut": "rejected ( \"effect\" , \"S0_EFFECT_RANGE\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-nonidentity-multiplier",
    "matched": true,
    "expectedOut": "rejected(\"intent\",\"S0_INTENT_SCOPE\",1)",
    "observedOut": "rejected ( \"intent\" , \"S0_INTENT_SCOPE\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-nonidentity-scale",
    "matched": true,
    "expectedOut": "rejected(\"intent\",\"S0_INTENT_SCOPE\",1)",
    "observedOut": "rejected ( \"intent\" , \"S0_INTENT_SCOPE\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  },
  {
    "case": "H-nonidentity-rounding",
    "matched": true,
    "expectedOut": "rejected(\"intent\",\"S0_INTENT_SCOPE\",1)",
    "observedOut": "rejected ( \"intent\" , \"S0_INTENT_SCOPE\" , 1 )",
    "externalPreserved": true,
    "parseError": null,
    "exitCode": 0,
    "stderr": ""
  }
]

```

## experiments/moriarty-language/formal/quint/mil4/s0.qnt

```text
// Provisional MIL/4 S0 transition model. See README.md for its evidence limits.
module s0 {
  type ReplayKey = (str, str, str)
  type Program = TransferProgram | RepayProgram
  type DebtStatus = Outstanding | Settled
  type Judgment = Accepted | Stage | Intent | Effect | Authority | History | Failure
  type Decision = { accepted: bool, judgment: Judgment, code: str,
                    diagnosticWork: int }

  type SignedIntent = {
    sourceVersion: str,
    coreVersion: str,
    profile: str,
    program: Program,
    domain: str,
    asset: str,
    digest: str,
    policyDigest: str,
    evidenceChoice: str,
    signer: str,
    payer: str,
    recipient: str,
    feeRecipient: str,
    obligationId: str,
    debtor: str,
    creditor: str,
    nonce: str,
    preHead: int,
    validFrom: int,
    validThrough: int,
    grossCap: int,
    feeCap: int,
    netFloor: int,
    actionAmount: int,
    actionFee: int,
    conversionMantissa: int,
    conversionScale: int,
    roundingNone: bool,
    terminalOnly: bool,
  }

  type Obligation = {
    debtor: str,
    creditor: str,
    asset: str,
    principal: int,
    accrued: int,
    outstanding: int,
    status: DebtStatus,
  }

  type EffectLine =
    | Debit({ account: str, amount: int })
    | Credit({ account: str, amount: int })
    | SetObligation({ id: str, value: Obligation })
    | UseAllowance({ owner: str, amount: int })
    | UseReplay(ReplayKey)
    | AdvanceHead({ before: int, after: int })

  type RequestedPhase = TerminalSuccess | RequestedFailure | RequestedPending
  type RetainedDuty = { id: str, owner: str, amount: int }
  type RequestedOutcome = { phase: RequestedPhase,
    retainedEffects: List[EffectLine], retainedDuties: List[RetainedDuty] }
  type FinancialSnapshot = {
    balances: str -> int, allowanceRemaining: str -> int,
    allowanceSpent: str -> int, workRemaining: int, workSpent: int,
    obligations: str -> Obligation, head: int, consumed: Set[ReplayKey],
  }
  type StipulatedTuple = { available: bool, intent: SignedIntent,
    preState: FinancialSnapshot, round: int, expectedSuccessor: int,
    requestedOutcome: RequestedOutcome }
  type ComparisonRequest = { submittedSuccessor: int,
    outcome: RequestedOutcome, premise: StipulatedTuple }
  pure val terminalOutcome: RequestedOutcome = {
    phase: TerminalSuccess, retainedEffects: List(), retainedDuties: List(),
  }

  // These are environmental premises, not checks implemented by Quint. The
  // selected external interfaces must establish their truth before admission.
  const signatureVerified: Set[str]
  const authenticatedSnapshots: Set[int]
  const nativeQualified: Set[str]
  const ledgerAtomicReady: bool
  const executingDomain: str
  const settlementAsset: str
  const initialWorkBudget: int

  pure val UINT128_MAX = 340282366920938463463374607431768211455
  pure val NOMINAL_MAX = 170141183460469231731687303715884105727

  var signed: str -> SignedIntent
  var balances: str -> int
  var allowanceRemaining: str -> int
  var allowanceSpent: str -> int
  var workRemaining: int
  var workSpent: int
  var obligations: str -> Obligation
  var currentHead: int
  var round: int
  var consumed: Set[ReplayKey]
  var lastEffects: List[EffectLine]
  var committedCount: int

  val financialSnapshot: FinancialSnapshot = {
    balances: balances, allowanceRemaining: allowanceRemaining,
    allowanceSpent: allowanceSpent, workRemaining: workRemaining,
    workSpent: workSpent, obligations: obligations, head: currentHead,
    consumed: consumed,
  }
  // Snapshot authentication remains an explicit external premise at every head.
  // A local commit never adds a head to this stipulated set.
  val currentSnapshotAuthenticated = authenticatedSnapshots.contains(currentHead)

  pure def replayKey(i: SignedIntent): ReplayKey = (i.domain, i.signer, i.nonce)
  pure def ok: Decision = { accepted: true, judgment: Accepted, code: "",
                            diagnosticWork: 0 }
  // Rejection reports one abstract diagnostic unit; it does not debit work.
  pure def reject(j: Judgment, c: str): Decision =
    { accepted: false, judgment: j, code: c, diagnosticWork: 1 }
  pure def withinUInt(n: int): bool = n >= 0 and n <= UINT128_MAX
  pure def withinNominal(n: int): bool = n >= 0 and n <= NOMINAL_MAX
  pure def minInt(a: int, b: int): int = if (a < b) a else b

  // Stage validates the complete work pair before premise or Authority checks.
  val stageWorkWellFormed = withinUInt(workRemaining) and withinUInt(workSpent)
    and workRemaining + workSpent <= UINT128_MAX

  pure def transferLines(i: SignedIntent, v: int, f: int, successor: int): List[EffectLine] = {
    val gross = v + f
    val money = if (f == 0)
      List(Debit({ account: i.payer, amount: gross }),
           Credit({ account: i.recipient, amount: v }))
    else
      List(Debit({ account: i.payer, amount: gross }),
           Credit({ account: i.recipient, amount: v }),
           Credit({ account: i.feeRecipient, amount: f }))
    money.concat(List(
      UseAllowance({ owner: i.payer, amount: gross }),
      UseReplay(replayKey(i)),
      AdvanceHead({ before: i.preHead, after: successor })
    ))
  }

  pure def repaid(o: Obligation, n: int): Obligation = {
    val da = minInt(n, o.accrued)
    val dp = n - da
    val p = o.principal - dp
    val a = o.accrued - da
    { ...o, principal: p, accrued: a, outstanding: p + a,
      status: if (p + a == 0) Settled else Outstanding }
  }

  pure def repayLines(i: SignedIntent, o: Obligation, n: int, successor: int): List[EffectLine] =
    List(
      Debit({ account: i.payer, amount: n }),
      Credit({ account: o.creditor, amount: n }),
      SetObligation({ id: i.obligationId, value: repaid(o, n) }),
      UseAllowance({ owner: i.payer, amount: n }),
      UseReplay(replayKey(i)),
      AdvanceHead({ before: i.preHead, after: successor })
    )

  // A rejected proposal is inspected through this value; it is never a stage.
  // Code spelling and within-judgment precedence are provisional W-D3 leaves.
  // Signed scope and caps precede alias diagnosis. Numeric effect guards in
  // each observation precede submitted-vector comparison.
  def beforeEffect(id: str, supplied: SignedIntent,
                     claimedHead: int, amount: int, fee: int,
                     recipient: str, debtor: str, isRepay: bool, request: ComparisonRequest): Decision = {
    if (not(signed.keys().contains(id)) or
        supplied.sourceVersion != "Source/6" or
        supplied.coreVersion != "Core/5" or
        supplied.profile != "S0" or
        supplied.domain != executingDomain or
        supplied.asset != settlementAsset)
      reject(Stage, "S0_STAGE_UNSUPPORTED")
    else if (not(signatureVerified.contains(signed.get(id).digest)) or
             not(currentSnapshotAuthenticated) or
             not(nativeQualified.contains(signed.get(id).digest)) or
             not(ledgerAtomicReady) or
             not(request.premise.available) or
             request.premise.intent != signed.get(id) or
             request.premise.preState != financialSnapshot or
             request.premise.round != round or
             request.premise.requestedOutcome != request.outcome)
      reject(Stage, "S0_STAGE_PREMISE")
    else if (signed.get(id) != supplied or
             amount != supplied.actionAmount or
             fee != supplied.actionFee or
             amount <= 0 or
             not(withinNominal(supplied.actionAmount)) or
             not(withinNominal(supplied.actionFee)) or
             not(withinNominal(supplied.grossCap)) or
             not(withinNominal(supplied.feeCap)) or
             not(withinNominal(supplied.netFloor)) or
             not(withinUInt(supplied.validFrom)) or
             not(withinUInt(supplied.validThrough)) or
             round < supplied.validFrom or round > supplied.validThrough or
             supplied.validFrom > supplied.validThrough or
             claimedHead != supplied.preHead or
             (not(isRepay) and
              (fee > supplied.feeCap or
               amount + fee > supplied.grossCap or
               amount < supplied.netFloor)) or
             (isRepay and (amount > supplied.grossCap or
                           supplied.feeCap != 0 or
                           supplied.netFloor != 0 or
                           supplied.recipient != "" or
                           supplied.feeRecipient != "" or
                           supplied.debtor != debtor or
                           supplied.creditor != recipient or
                           supplied.conversionMantissa != 1 or
                           supplied.conversionScale != 0 or
                           not(supplied.roundingNone))))
      reject(Intent, "S0_INTENT_SCOPE")
    else if (supplied.payer == recipient or
             (not(isRepay) and
              (supplied.payer == supplied.feeRecipient or
               supplied.recipient == supplied.feeRecipient)))
      reject(Intent, "S0_INTENT_ALIAS")
    else if (not(withinNominal(amount)) or
             not(withinNominal(amount + fee)) or fee < 0)
      reject(Effect, "S0_EFFECT_RANGE")
    else ok
  }

  def afterEffect(supplied: SignedIntent, claimedHead: int,
                  amount: int, fee: int, request: ComparisonRequest): Decision = {
    if (supplied.signer != supplied.payer or
             allowanceRemaining.get(supplied.payer) < amount + fee or
             allowanceSpent.get(supplied.payer) + amount + fee > UINT128_MAX or
             workRemaining < 1 or workSpent + 1 > UINT128_MAX)
      reject(Authority, "S0_AUTH_SCOPE")
    else if (claimedHead != currentHead)
      reject(History, "S0_HISTORY_STALE")
    else if (consumed.contains(replayKey(supplied)))
      reject(History, "S0_HISTORY_REPLAY")
    else if (request.submittedSuccessor == claimedHead or
             request.submittedSuccessor != request.premise.expectedSuccessor)
      reject(History, "S0_HISTORY_SUCCESSOR")
    else if (not(supplied.terminalOnly) or
             request.outcome.phase != TerminalSuccess or
             request.outcome.retainedEffects != List() or
             request.outcome.retainedDuties != List())
      reject(Failure, "S0_FAILURE_UNSUPPORTED")
    else ok
  }

  def transferObservation(id: str, supplied: SignedIntent,
                          claimedHead: int, v: int, f: int,
                          lines: List[EffectLine], request: ComparisonRequest): Decision = {
    if (not(signed.keys().contains(id)))
      reject(Stage, "S0_STAGE_UNSUPPORTED")
    else {
      val bound = signed.get(id)
      if (bound.program != TransferProgram or
          not(stageWorkWellFormed) or
          obligations.keys() != Set() or
          allowanceRemaining.keys() != Set(bound.payer) or
          allowanceSpent.keys() != Set(bound.payer) or
          // Preserve Intent diagnosis for signed endpoint aliases. Distinct
          // endpoints select the exact three-balance S0 footprint.
          (bound.payer != bound.recipient and bound.payer != bound.feeRecipient and
           bound.recipient != bound.feeRecipient and
           balances.keys() != Set(bound.payer, bound.recipient, bound.feeRecipient)) or
          not(balances.keys().contains(bound.payer)) or
          not(balances.keys().contains(bound.recipient)) or
          not(balances.keys().contains(bound.feeRecipient)) or
          not(withinUInt(balances.get(bound.recipient))) or
          not(withinUInt(balances.get(bound.feeRecipient))) or
          not(allowanceRemaining.keys().contains(bound.payer)) or
          not(allowanceSpent.keys().contains(bound.payer)) or
          not(withinUInt(balances.get(bound.payer))) or
          not(withinUInt(allowanceRemaining.get(bound.payer))) or
          not(withinUInt(allowanceSpent.get(bound.payer))) or
          allowanceRemaining.get(bound.payer) + allowanceSpent.get(bound.payer) > UINT128_MAX)
        reject(Stage, "S0_STAGE_UNSUPPORTED")
      else {
        val base = beforeEffect(id, supplied, claimedHead, v, f,
                                supplied.recipient, "", false, request)
      if (not(base.accepted)) base
      else if (balances.get(supplied.payer) < v + f or
               balances.get(supplied.recipient) + v > UINT128_MAX or
               balances.get(supplied.feeRecipient) + f > UINT128_MAX or
               not(withinUInt(balances.get(supplied.payer))) or
               not(withinUInt(balances.get(supplied.recipient))) or
               not(withinUInt(balances.get(supplied.feeRecipient))))
        reject(Effect, "S0_EFFECT_RANGE")
      else if (lines != transferLines(supplied, v, f, request.submittedSuccessor))
        reject(Effect, "S0_EFFECT_MISMATCH")
      else afterEffect(supplied, claimedHead, v, f, request)
      }
    }
  }

  def repayObservation(id: str, supplied: SignedIntent,
                       claimedHead: int, n: int,
                       lines: List[EffectLine], request: ComparisonRequest): Decision = {
    if (not(signed.keys().contains(id)))
      reject(Stage, "S0_STAGE_UNSUPPORTED")
    else {
      val bound = signed.get(id)
      if (bound.program != RepayProgram or
          not(stageWorkWellFormed) or
          obligations.keys() != Set(bound.obligationId) or
          allowanceRemaining.keys() != Set(bound.payer) or
          allowanceSpent.keys() != Set(bound.payer) or
          not(obligations.keys().contains(bound.obligationId)) or
          not(balances.keys().contains(bound.payer)) or
          not(allowanceRemaining.keys().contains(bound.payer)) or
          not(allowanceSpent.keys().contains(bound.payer)) or
          not(withinUInt(balances.get(bound.payer))) or
          not(withinUInt(allowanceRemaining.get(bound.payer))) or
          not(withinUInt(allowanceSpent.get(bound.payer))) or
          allowanceRemaining.get(bound.payer) + allowanceSpent.get(bound.payer) > UINT128_MAX)
        reject(Stage, "S0_STAGE_UNSUPPORTED")
      else {
        val o = obligations.get(bound.obligationId)
        // Use the immutable bound intent here: a supplied payer substitution
        // belongs to Intent after the existing typed state has passed Stage.
        if (balances.keys() != Set(bound.payer, o.creditor) or
            not(balances.keys().contains(o.creditor)) or
            not(withinUInt(balances.get(o.creditor))) or
            not(withinNominal(o.principal)) or
            not(withinNominal(o.accrued)) or
            not(withinNominal(o.outstanding)) or
            o.outstanding != o.principal + o.accrued or
            o.status != Outstanding or o.asset != settlementAsset or
            o.debtor != bound.signer or o.debtor != bound.payer or
            o.creditor == bound.payer)
          reject(Stage, "S0_STAGE_UNSUPPORTED")
        else {
        val base = beforeEffect(id, supplied, claimedHead, n, 0,
                                  o.creditor, o.debtor, true, request)
        if (not(base.accepted)) base
        else if (n > o.outstanding or
                 balances.get(supplied.payer) < n or
                 balances.get(o.creditor) + n > UINT128_MAX or
                 not(withinUInt(balances.get(supplied.payer))) or
                 not(withinUInt(balances.get(o.creditor))) or
                 not(withinUInt(repaid(o, n).outstanding)))
          reject(Effect, "S0_EFFECT_RANGE")
        else if (lines != repayLines(supplied, o, n, request.submittedSuccessor))
          reject(Effect, "S0_EFFECT_MISMATCH")
        else afterEffect(supplied, claimedHead, n, 0, request)
        }
      }
    }
  }

  action init: bool = all {
    withinUInt(initialWorkBudget),
    signed' = Map(),
    balances' = Map(),
    allowanceRemaining' = Map(),
    allowanceSpent' = Map(),
    workRemaining' = initialWorkBudget,
    workSpent' = 0,
    obligations' = Map(),
    currentHead' = 0,
    round' = 0,
    consumed' = Set(),
    lastEffects' = List(),
    committedCount' = 0,
  }

  // Environment setup represents authenticated cells; it is not a ledger proof.
  action seedAccount(account: str, balance: int,
                     remaining: int, spent: int): bool = all {
    committedCount == 0,
    not(balances.keys().contains(account)),
    withinUInt(balance) and withinUInt(remaining) and withinUInt(spent),
    remaining + spent <= UINT128_MAX,
    balances' = balances.put(account, balance),
    allowanceRemaining' = allowanceRemaining.put(account, remaining),
    allowanceSpent' = allowanceSpent.put(account, spent),
    workRemaining' = workRemaining, workSpent' = workSpent,
    signed' = signed, obligations' = obligations,
    currentHead' = currentHead, round' = round, consumed' = consumed,
    lastEffects' = lastEffects, committedCount' = committedCount,
  }

  // A receiving balance does not imply authority to spend from that account.
  action seedBalanceOnly(account: str, balance: int): bool = all {
    committedCount == 0,
    not(balances.keys().contains(account)),
    withinUInt(balance),
    balances' = balances.put(account, balance),
    allowanceRemaining' = allowanceRemaining,
    allowanceSpent' = allowanceSpent,
    workRemaining' = workRemaining, workSpent' = workSpent,
    signed' = signed, obligations' = obligations,
    currentHead' = currentHead, round' = round, consumed' = consumed,
    lastEffects' = lastEffects, committedCount' = committedCount,
  }

  action seedObligation(id: str, o: Obligation): bool = all {
    committedCount == 0,
    not(obligations.keys().contains(id)),
    o.asset == settlementAsset,
    withinUInt(o.principal) and withinUInt(o.accrued) and
      withinUInt(o.outstanding),
    o.outstanding == o.principal + o.accrued,
    obligations' = obligations.put(id, o),
    signed' = signed, balances' = balances,
    allowanceRemaining' = allowanceRemaining,
    allowanceSpent' = allowanceSpent,
    workRemaining' = workRemaining, workSpent' = workSpent,
    currentHead' = currentHead, round' = round, consumed' = consumed,
    lastEffects' = lastEffects, committedCount' = committedCount,
  }

  action advanceRound: bool = all {
    round' = round + 1,
    signed' = signed, balances' = balances,
    allowanceRemaining' = allowanceRemaining,
    allowanceSpent' = allowanceSpent,
    workRemaining' = workRemaining, workSpent' = workSpent,
    obligations' = obligations, currentHead' = currentHead,
    consumed' = consumed, lastEffects' = lastEffects,
    committedCount' = committedCount,
  }

  action sign(i: SignedIntent): bool = all {
    not(signed.keys().contains(i.digest)),
    i.domain == executingDomain and i.asset == settlementAsset,
    i.sourceVersion == "Source/6" and i.coreVersion == "Core/5",
    i.profile == "S0",
    signed' = signed.put(i.digest, i),
    balances' = balances, allowanceRemaining' = allowanceRemaining,
    allowanceSpent' = allowanceSpent, obligations' = obligations,
    workRemaining' = workRemaining, workSpent' = workSpent,
    currentHead' = currentHead, round' = round, consumed' = consumed,
    lastEffects' = lastEffects, committedCount' = committedCount,
  }

  action submitTransfer(id: str, supplied: SignedIntent, claimedHead: int,
                        v: int, f: int, lines: List[EffectLine], request: ComparisonRequest): bool = all {
    transferObservation(id, supplied, claimedHead, v, f, lines, request).accepted,
    balances' = balances.put(supplied.payer,
                             balances.get(supplied.payer) - v - f)
      .put(supplied.recipient, balances.get(supplied.recipient) + v)
      .put(supplied.feeRecipient, balances.get(supplied.feeRecipient) + f),
    allowanceRemaining' = allowanceRemaining.put(supplied.payer,
      allowanceRemaining.get(supplied.payer) - v - f),
    allowanceSpent' = allowanceSpent.put(supplied.payer,
      allowanceSpent.get(supplied.payer) + v + f),
    workRemaining' = workRemaining - 1,
    workSpent' = workSpent + 1,
    consumed' = consumed.union(Set(replayKey(supplied))),
    currentHead' = request.submittedSuccessor,
    lastEffects' = lines,
    committedCount' = committedCount + 1,
    signed' = signed, obligations' = obligations, round' = round,
  }

  action submitRepay(id: str, supplied: SignedIntent, claimedHead: int,
                     n: int, lines: List[EffectLine], request: ComparisonRequest): bool = all {
    repayObservation(id, supplied, claimedHead, n, lines, request).accepted,
    balances' = balances.put(supplied.payer,
                             balances.get(supplied.payer) - n)
      .put(obligations.get(supplied.obligationId).creditor,
           balances.get(obligations.get(supplied.obligationId).creditor) + n),
    obligations' = obligations.put(supplied.obligationId,
      repaid(obligations.get(supplied.obligationId), n)),
    allowanceRemaining' = allowanceRemaining.put(supplied.payer,
      allowanceRemaining.get(supplied.payer) - n),
    allowanceSpent' = allowanceSpent.put(supplied.payer,
      allowanceSpent.get(supplied.payer) + n),
    workRemaining' = workRemaining - 1,
    workSpent' = workSpent + 1,
    consumed' = consumed.union(Set(replayKey(supplied))),
    currentHead' = request.submittedSuccessor,
    lastEffects' = lines,
    committedCount' = committedCount + 1,
    signed' = signed, round' = round,
  }
}

```

## experiments/moriarty-language/formal/quint/mil4/corpus/s0_witnesses.qnt

```text
// Finite MIL/4 S0 semantic witnesses. External premises are stipulated here.
module s0_witnesses {
  import s0(
    signatureVerified = Set("t", "r", "t2"),
    authenticatedSnapshots = Set(0, 1),
    nativeQualified = Set("t", "r", "t2"),
    ledgerAtomicReady = true,
    executingDomain = "D",
    settlementAsset = "A",
    initialWorkBudget = 3
  ).* from "../s0"

  // This helper stipulates comparison input only. It establishes no premise.
  def fixtureRequest(id: str, supplied: SignedIntent, successor: int): ComparisonRequest = {
    submittedSuccessor: successor, outcome: terminalOutcome,
    premise: { available: true,
      intent: if (signed.keys().contains(id)) signed.get(id) else supplied,
      preState: financialSnapshot, round: round, expectedSuccessor: successor, requestedOutcome: terminalOutcome },
  }

  pure val t: SignedIntent = {
    sourceVersion: "Source/6", coreVersion: "Core/5", profile: "S0",
    program: TransferProgram, domain: "D", asset: "A", digest: "t",
    policyDigest: "p", evidenceChoice: "e", signer: "O", payer: "O",
    recipient: "R", feeRecipient: "F", obligationId: "",
    debtor: "", creditor: "", nonce: "nt", preHead: 0,
    validFrom: 0, validThrough: 2, grossCap: 11, feeCap: 1,
    netFloor: 10, actionAmount: 10, actionFee: 1,
    conversionMantissa: 1, conversionScale: 0, roundingNone: true,
    terminalOnly: true,
  }
  pure val r: SignedIntent = {
    ...t, program: RepayProgram, digest: "r", nonce: "nr",
    recipient: "", feeRecipient: "", obligationId: "loan",
    debtor: "O", creditor: "C", grossCap: 30, feeCap: 0,
    netFloor: 0, actionAmount: 30, actionFee: 0,
  }
  pure val o: Obligation = {
    debtor: "O", creditor: "C", asset: "A", principal: 1000,
    accrued: 10, outstanding: 1010, status: Outstanding,
  }
  pure val u = UINT128_MAX
  pure val s = NOMINAL_MAX
  pure val rNear: SignedIntent = {
    ...r, actionAmount: 1, grossCap: 1,
  }
  pure val oNear: Obligation = {
    ...o, principal: s - 1, accrued: 1, outstanding: s,
  }
  pure val t2: SignedIntent = {
    ...t, digest: "t2", preHead: 1,
  }
  pure val rOverpay: SignedIntent = {
    ...r, actionAmount: 1011, grossCap: 1011,
  }
  pure val tFeeCap: SignedIntent = {
    ...t, actionFee: 2, grossCap: 12,
  }
  pure val tGrossCap: SignedIntent = {
    ...t, grossCap: 10,
  }
  pure val missingFee: List[EffectLine] = List(
    Debit({ account: "O", amount: 11 }),
    Credit({ account: "R", amount: 10 }),
    UseAllowance({ owner: "O", amount: 11 }),
    UseReplay(replayKey(t)),
    AdvanceHead({ before: 0, after: 1 })
  )
  pure val missingCreditor: List[EffectLine] = List(
    Debit({ account: "O", amount: 30 }),
    SetObligation({ id: "loan", value: repaid(o, 30) }),
    UseAllowance({ owner: "O", amount: 30 }),
    UseReplay(replayKey(r)),
    AdvanceHead({ before: 0, after: 1 })
  )
  pure val wrongCreditor: List[EffectLine] = List(
    Debit({ account: "O", amount: 30 }),
    Credit({ account: "X", amount: 30 }),
    SetObligation({ id: "loan", value: repaid(o, 30) }),
    UseAllowance({ owner: "O", amount: 30 }),
    UseReplay(replayKey(r)),
    AdvanceHead({ before: 0, after: 1 })
  )

  action transferReady: bool =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedBalanceOnly("R", 0))
      .then(seedBalanceOnly("F", 0))
      .then(sign(t))

  action repayReady: bool =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedBalanceOnly("C", 0))
      .then(seedObligation("loan", o))
      .then(sign(r))

  run transferTenOneTest = transferReady
    .expect(transferObservation("t", t, 0, 10, 1, transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)) == { accepted: true, judgment: Accepted, code: "", diagnosticWork: 0 })
    .then(submitTransfer("t", t, 0, 10, 1, transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)))
    .expect(all {
      balances.get("O") == 89,
      balances.get("R") == 10,
      balances.get("F") == 1,
      allowanceRemaining.get("O") == 89,
      allowanceSpent.get("O") == 11,
      workRemaining == 2 and workSpent == 1,
      consumed == Set(replayKey(t)),
      currentHead == 1 and committedCount == 1,
      lastEffects == transferLines(t, 10, 1, 1),
    })

  run repayThirtyTest = repayReady
    .expect(repayObservation("r", r, 0, 30, repayLines(r, o, 30, 1), fixtureRequest("r", r, 1)) == { accepted: true, judgment: Accepted, code: "", diagnosticWork: 0 })
    .then(submitRepay("r", r, 0, 30, repayLines(r, o, 30, 1), fixtureRequest("r", r, 1)))
    .expect(all {
      balances.get("O") == 70 and balances.get("C") == 30,
      obligations.get("loan") == repaid(o, 30),
      obligations.get("loan").principal == 980,
      obligations.get("loan").accrued == 0,
      obligations.get("loan").outstanding == 980,
      obligations.get("loan").status == Outstanding,
      allowanceRemaining.get("O") == 70,
      allowanceSpent.get("O") == 30,
      consumed == Set(replayKey(r)),
      currentHead == 1 and committedCount == 1,
      lastEffects == repayLines(r, o, 30, 1),
    })

  run repayNearBoundTest =
    init.then(seedAccount("O", 1, 1, u - 1))
      .then(seedBalanceOnly("C", u - 1))
      .then(seedObligation("loan", oNear))
      .then(sign(rNear))
      .expect(repayObservation("r", rNear, 0, 1, repayLines(rNear, oNear, 1, 1), fixtureRequest("r", rNear, 1)) == { accepted: true, judgment: Accepted, code: "", diagnosticWork: 0 })
      .then(submitRepay("r", rNear, 0, 1, repayLines(rNear, oNear, 1, 1), fixtureRequest("r", rNear, 1)))
      .expect(all {
        balances.get("C") == u,
        allowanceSpent.get("O") == u,
        obligations.get("loan").principal == s - 1,
        obligations.get("loan").accrued == 0,
        obligations.get("loan").outstanding == s - 1,
        committedCount == 1,
      })

  run recipientSubstitutionTest = transferReady
    .expect(all {
      not(transferObservation("t", {...t, recipient: "X"}, 0, 10, 1,
        transferLines({...t, recipient: "X"}, 10, 1, 1), fixtureRequest("t", {...t, recipient: "X"}, 1)).accepted),
      transferObservation("t", {...t, recipient: "X"}, 0, 10, 1,
        transferLines({...t, recipient: "X"}, 10, 1, 1), fixtureRequest("t", {...t, recipient: "X"}, 1)).judgment == Intent,
      transferObservation("t", {...t, recipient: "X"}, 0, 10, 1,
        transferLines({...t, recipient: "X"}, 10, 1, 1), fixtureRequest("t", {...t, recipient: "X"}, 1)).code == "S0_INTENT_SCOPE",
      transferObservation("t", {...t, recipient: "X"}, 0, 10, 1, transferLines({...t, recipient: "X"}, 10, 1, 1), fixtureRequest("t", {...t, recipient: "X"}, 1)).diagnosticWork == 1,
    })
    .expect(committedCount == 0 and lastEffects == List())
    .then(submitTransfer("t", {...t, recipient: "X"}, 0, 10, 1,
      transferLines({...t, recipient: "X"}, 10, 1, 1), fixtureRequest("t", {...t, recipient: "X"}, 1)).fail())

  run feeCapTest =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedBalanceOnly("R", 0))
      .then(seedBalanceOnly("F", 0))
      .then(sign(tFeeCap))
    .expect(transferObservation("t", tFeeCap, 0, 10, 2,
      transferLines(tFeeCap, 10, 2, 1), fixtureRequest("t", tFeeCap, 1)) == { accepted: false, judgment: Intent, code: "S0_INTENT_SCOPE", diagnosticWork: 1 })
    .expect(committedCount == 0 and lastEffects == List())
    .then(submitTransfer("t", tFeeCap, 0, 10, 2,
      transferLines(tFeeCap, 10, 2, 1), fixtureRequest("t", tFeeCap, 1)).fail())

  run grossCapTest =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedBalanceOnly("R", 0))
      .then(seedBalanceOnly("F", 0))
      .then(sign(tGrossCap))
      .expect(transferObservation("t", tGrossCap, 0, 10, 1,
        transferLines(tGrossCap, 10, 1, 1), fixtureRequest("t", tGrossCap, 1)) == { accepted: false, judgment: Intent, code: "S0_INTENT_SCOPE", diagnosticWork: 1 })
      .then(submitTransfer("t", tGrossCap, 0, 10, 1,
        transferLines(tGrossCap, 10, 1, 1), fixtureRequest("t", tGrossCap, 1)).fail())

  run missingFeeTest = transferReady
    .expect(transferObservation("t", t, 0, 10, 1, missingFee, fixtureRequest("t", t, 1)) == { accepted: false, judgment: Effect, code: "S0_EFFECT_MISMATCH", diagnosticWork: 1 })
    .expect(committedCount == 0 and lastEffects == List())
    .then(submitTransfer("t", t, 0, 10, 1, missingFee, fixtureRequest("t", t, 1)).fail())

  run missingCreditorTest = repayReady
    .expect(repayObservation("r", r, 0, 30, missingCreditor, fixtureRequest("r", r, 1)) == { accepted: false, judgment: Effect, code: "S0_EFFECT_MISMATCH", diagnosticWork: 1 })
    .expect(committedCount == 0 and obligations.get("loan") == o and
      lastEffects == List())
    .then(submitRepay("r", r, 0, 30, missingCreditor, fixtureRequest("r", r, 1)).fail())

  run wrongCreditorTest = repayReady
    .expect(repayObservation("r", r, 0, 30, wrongCreditor, fixtureRequest("r", r, 1)) == { accepted: false, judgment: Effect, code: "S0_EFFECT_MISMATCH", diagnosticWork: 1 })
    .expect(committedCount == 0 and obligations.get("loan") == o and
      lastEffects == List())
    .then(submitRepay("r", r, 0, 30, wrongCreditor, fixtureRequest("r", r, 1)).fail())

  run allowanceTest =
    init.then(seedAccount("O", 100, 10, 0))
      .then(seedBalanceOnly("R", 0))
      .then(seedBalanceOnly("F", 0))
      .then(sign(t))
      .expect(transferObservation("t", t, 0, 10, 1,
        transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)) == { accepted: false, judgment: Authority, code: "S0_AUTH_SCOPE", diagnosticWork: 1 })
      .expect(committedCount == 0 and lastEffects == List())
      .then(submitTransfer("t", t, 0, 10, 1,
        transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)).fail())

  run staleHeadTest = transferReady
    .then(sign({...t2, nonce: "fresh"}))
    .then(submitTransfer("t", t, 0, 10, 1, transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)))
    .expect(transferObservation("t2", {...t2, nonce: "fresh"}, 1, 10, 1,
      transferLines({...t2, nonce: "fresh"}, 10, 1, 2), fixtureRequest("t2", {...t2, nonce: "fresh"}, 2)) == { accepted: true, judgment: Accepted, code: "", diagnosticWork: 0 })
    .expect(transferObservation("t", t, 0, 10, 1,
      transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)) == { accepted: false, judgment: History, code: "S0_HISTORY_STALE", diagnosticWork: 1 })
    .expect(committedCount == 1 and currentHead == 1)
    .then(submitTransfer("t", t, 0, 10, 1,
      transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)).fail())

  run replayTest = transferReady
    .then(sign(t2))
    .then(submitTransfer("t", t, 0, 10, 1, transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)))
    .expect(transferObservation("t2", t2, 1, 10, 1,
      transferLines(t2, 10, 1, 2), fixtureRequest("t2", t2, 2)) == { accepted: false, judgment: History, code: "S0_HISTORY_REPLAY", diagnosticWork: 1 })
    .expect(committedCount == 1 and currentHead == 1)
    .then(submitTransfer("t2", t2, 1, 10, 1,
      transferLines(t2, 10, 1, 2), fixtureRequest("t2", t2, 2)).fail())

  run overflowTest =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedBalanceOnly("R", u))
      .then(seedBalanceOnly("F", 0))
      .then(sign(t))
      .expect(transferObservation("t", t, 0, 10, 1,
        transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)) == { accepted: false, judgment: Effect, code: "S0_EFFECT_RANGE", diagnosticWork: 1 })
      .expect(committedCount == 0 and lastEffects == List())
      .then(submitTransfer("t", t, 0, 10, 1,
        transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)).fail())

  run overpayTest =
    init.then(seedAccount("O", 1011, 1011, 0))
      .then(seedBalanceOnly("C", 0))
      .then(seedObligation("loan", o))
      .then(sign(rOverpay))
      .expect(repayObservation("r", rOverpay, 0, 1011,
        repayLines(rOverpay, o, 1011, 1), fixtureRequest("r", rOverpay, 1)) == { accepted: false, judgment: Effect, code: "S0_EFFECT_RANGE", diagnosticWork: 1 })
      .expect(committedCount == 0 and obligations.get("loan") == o)
      .then(submitRepay("r", rOverpay, 0, 1011,
        repayLines(rOverpay, o, 1011, 1), fixtureRequest("r", rOverpay, 1)).fail())

  run missingFeeAndStaleTest = transferReady
    .then(sign({...t2, nonce: "fresh"}))
    .then(submitTransfer("t", t, 0, 10, 1, transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)))
    .expect(transferObservation("t", t, 0, 10, 1, missingFee, fixtureRequest("t", t, 1)) == { accepted: false, judgment: Effect, code: "S0_EFFECT_MISMATCH", diagnosticWork: 1 })
    .expect(committedCount == 1 and currentHead == 1)
    .then(submitTransfer("t", t, 0, 10, 1, missingFee, fixtureRequest("t", t, 1)).fail())
}

```

## experiments/moriarty-language/formal/quint/mil4/corpus/s0_divergence_witnesses.qnt

```text
// Fixed ordering regressions for the repaired provisional S0 Quint candidate.
module s0_divergence_witnesses {
  import s0(
    signatureVerified = Set("overflow", "alias"),
    authenticatedSnapshots = Set(0),
    nativeQualified = Set("overflow", "alias"),
    ledgerAtomicReady = true,
    executingDomain = "D",
    settlementAsset = "A",
    initialWorkBudget = 1
  ).* from "../s0"

  // This helper stipulates comparison input only. It establishes no premise.
  def fixtureRequest(id: str, supplied: SignedIntent, successor: int): ComparisonRequest = {
    submittedSuccessor: successor, outcome: terminalOutcome,
    premise: { available: true,
      intent: if (signed.keys().contains(id)) signed.get(id) else supplied,
      preState: financialSnapshot, round: round, expectedSuccessor: successor, requestedOutcome: terminalOutcome },
  }

  pure val transfer: SignedIntent = {
    sourceVersion: "Source/6", coreVersion: "Core/5", profile: "S0",
    program: TransferProgram, domain: "D", asset: "A",
    digest: "overflow", policyDigest: "p", evidenceChoice: "e",
    signer: "O", payer: "O", recipient: "R", feeRecipient: "F",
    obligationId: "", debtor: "", creditor: "", nonce: "n1",
    preHead: 0, validFrom: 0, validThrough: 1,
    grossCap: 11, feeCap: 1, netFloor: 10,
    actionAmount: 10, actionFee: 1,
    conversionMantissa: 1, conversionScale: 0,
    roundingNone: true, terminalOnly: true,
  }
  pure val missingFee: List[EffectLine] = List(
    Debit({ account: "O", amount: 11 }),
    Credit({ account: "R", amount: 10 }),
    UseAllowance({ owner: "O", amount: 11 }),
    UseReplay(replayKey(transfer)),
    AdvanceHead({ before: 0, after: 1 })
  )
  pure val repayAlias: SignedIntent = {
    ...transfer, program: RepayProgram, digest: "alias",
    obligationId: "loan", debtor: "O", creditor: "O",
    recipient: "O", actionAmount: 30, actionFee: 0,
    grossCap: 30, feeCap: 0, netFloor: 0,
  }
  pure val selfDebt: Obligation = {
    debtor: "O", creditor: "O", asset: "A",
    principal: 1000, accrued: 10, outstanding: 1010,
    status: Outstanding,
  }

  run overflowAndMissingFeeTest =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedBalanceOnly("R", UINT128_MAX))
      .then(seedBalanceOnly("F", 0))
      .then(sign(transfer))
      .expect(all {
        transferObservation("overflow", transfer, 0, 10, 1, missingFee, fixtureRequest("overflow", transfer, 1)).judgment == Effect,
        transferObservation("overflow", transfer, 0, 10, 1, missingFee, fixtureRequest("overflow", transfer, 1)).code == "S0_EFFECT_RANGE",
        committedCount == 0,
        lastEffects == List(),
      })
      .then(submitTransfer("overflow", transfer, 0, 10, 1, missingFee, fixtureRequest("overflow", transfer, 1)).fail())

  run repaymentAliasTest =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedObligation("loan", selfDebt))
      .then(sign(repayAlias))
      .expect(all {
        repayObservation("alias", repayAlias, 0, 30,
          repayLines(repayAlias, selfDebt, 30, 1), fixtureRequest("alias", repayAlias, 1)).judgment == Stage,
        repayObservation("alias", repayAlias, 0, 30,
          repayLines(repayAlias, selfDebt, 30, 1), fixtureRequest("alias", repayAlias, 1)).code == "S0_STAGE_UNSUPPORTED",
        committedCount == 0,
        obligations.get("loan") == selfDebt,
      })
      .then(submitRepay("alias", repayAlias, 0, 30,
        repayLines(repayAlias, selfDebt, 30, 1), fixtureRequest("alias", repayAlias, 1)).fail())
  pure val debt: Obligation = { ...selfDebt, creditor: "C" }
  pure val repay: SignedIntent = {
    ...repayAlias, creditor: "C", recipient: "", feeRecipient: "",
  }
  pure val badDebtor: Obligation = { ...debt, debtor: "X" }
  pure val signerMismatch: SignedIntent = { ...repay, signer: "X" }
  pure val payerMismatch: SignedIntent = { ...repay, payer: "X" }
  pure val signedDebtorMismatch: SignedIntent = { ...repay, debtor: "X" }
  pure val signedCreditorMismatch: SignedIntent = { ...repay, creditor: "X" }
  pure val nonidentity: SignedIntent = { ...repay, conversionMantissa: 2 }
  pure val scaled: SignedIntent = { ...repay, conversionScale: 1 }
  pure val rounded: SignedIntent = { ...repay, roundingNone: false }
  pure val roundAlias: SignedIntent = {
    ...transfer, recipient: "O", validFrom: 1,
  }
  pure val capAlias: SignedIntent = {
    ...transfer, recipient: "O", grossCap: 10,
  }
  pure val overpay: SignedIntent = {
    ...repay, actionAmount: 1011, grossCap: 1011,
  }
  pure val missingCreditor: List[EffectLine] = List(
    Debit({ account: "O", amount: 1011 }),
    SetObligation({ id: "loan", value: repaid(debt, 1011) }),
    UseAllowance({ owner: "O", amount: 1011 }),
    UseReplay(replayKey(overpay)),
    AdvanceHead({ before: 0, after: 1 })
  )

  action repayReady(i: SignedIntent, o: Obligation): bool =
    init.then(seedAccount("O", 2000, 2000, 0))
      .then(seedBalanceOnly("C", 0))
      .then(seedObligation("loan", o))
      .then(sign(i))

  def rejectedRepay(i: SignedIntent, o: Obligation, j: Judgment,
                    c: str, lines: List[EffectLine]): bool = all {
    not(repayObservation("alias", i, 0, i.actionAmount, lines, fixtureRequest("alias", i, 1)).accepted),
    repayObservation("alias", i, 0, i.actionAmount, lines, fixtureRequest("alias", i, 1)).judgment == j,
    repayObservation("alias", i, 0, i.actionAmount, lines, fixtureRequest("alias", i, 1)).code == c,
    committedCount == 0 and currentHead == 0,
    balances.get("O") == 2000 and balances.get("C") == 0,
    allowanceRemaining.get("O") == 2000 and allowanceSpent.get("O") == 0,
    workRemaining == 1 and workSpent == 0,
    consumed == Set() and lastEffects == List(),
    obligations.get("loan") == o,
  }

  run debtorMismatchTest = repayReady(repay, badDebtor)
    .expect(rejectedRepay(repay, badDebtor, Stage, "S0_STAGE_UNSUPPORTED",
      repayLines(repay, badDebtor, 30, 1)))
    .then(submitRepay("alias", repay, 0, 30,
      repayLines(repay, badDebtor, 30, 1), fixtureRequest("alias", repay, 1)).fail())

  run debtorSignerMismatchTest = repayReady(signerMismatch, debt)
    .expect(rejectedRepay(signerMismatch, debt, Stage, "S0_STAGE_UNSUPPORTED",
      repayLines(signerMismatch, debt, 30, 1)))
    .then(submitRepay("alias", signerMismatch, 0, 30,
      repayLines(signerMismatch, debt, 30, 1), fixtureRequest("alias", signerMismatch, 1)).fail())

  run debtorPayerMismatchTest = repayReady(payerMismatch, debt)
    .expect(rejectedRepay(payerMismatch, debt, Stage, "S0_STAGE_UNSUPPORTED",
      repayLines(payerMismatch, debt, 30, 1)))
    .then(submitRepay("alias", payerMismatch, 0, 30,
      repayLines(payerMismatch, debt, 30, 1), fixtureRequest("alias", payerMismatch, 1)).fail())

  run signedDebtorScopeTest = repayReady(signedDebtorMismatch, debt)
    .expect(rejectedRepay(signedDebtorMismatch, debt, Intent, "S0_INTENT_SCOPE",
      repayLines(signedDebtorMismatch, debt, 30, 1)))
    .then(submitRepay("alias", signedDebtorMismatch, 0, 30,
      repayLines(signedDebtorMismatch, debt, 30, 1), fixtureRequest("alias", signedDebtorMismatch, 1)).fail())

  run signedCreditorScopeTest = repayReady(signedCreditorMismatch, debt)
    .expect(rejectedRepay(signedCreditorMismatch, debt, Intent, "S0_INTENT_SCOPE",
      repayLines(signedCreditorMismatch, debt, 30, 1)))
    .then(submitRepay("alias", signedCreditorMismatch, 0, 30,
      repayLines(signedCreditorMismatch, debt, 30, 1), fixtureRequest("alias", signedCreditorMismatch, 1)).fail())

  // A substituted supplied payer must not change the bound Stage classification.
  run suppliedPayerScopeTest = repayReady(repay, debt)
    .expect(all {
      not(repayObservation("alias", payerMismatch, 0, 30,
        repayLines(payerMismatch, debt, 30, 1), fixtureRequest("alias", payerMismatch, 1)).accepted),
      repayObservation("alias", payerMismatch, 0, 30,
        repayLines(payerMismatch, debt, 30, 1), fixtureRequest("alias", payerMismatch, 1)).judgment == Intent,
      repayObservation("alias", payerMismatch, 0, 30,
        repayLines(payerMismatch, debt, 30, 1), fixtureRequest("alias", payerMismatch, 1)).code == "S0_INTENT_SCOPE",
      obligations.get("loan") == debt and committedCount == 0,
      consumed == Set() and lastEffects == List(),
    })
    .then(submitRepay("alias", payerMismatch, 0, 30,
      repayLines(payerMismatch, debt, 30, 1), fixtureRequest("alias", payerMismatch, 1)).fail())

  run nonidentityConversionTest = repayReady(nonidentity, debt)
    .expect(rejectedRepay(nonidentity, debt, Intent, "S0_INTENT_SCOPE",
      repayLines(nonidentity, debt, 30, 1)))
    .then(submitRepay("alias", nonidentity, 0, 30,
      repayLines(nonidentity, debt, 30, 1), fixtureRequest("alias", nonidentity, 1)).fail())

  run conversionScaleTest = repayReady(scaled, debt)
    .expect(rejectedRepay(scaled, debt, Intent, "S0_INTENT_SCOPE",
      repayLines(scaled, debt, 30, 1)))
    .then(submitRepay("alias", scaled, 0, 30,
      repayLines(scaled, debt, 30, 1), fixtureRequest("alias", scaled, 1)).fail())

  run conversionRoundingTest = repayReady(rounded, debt)
    .expect(rejectedRepay(rounded, debt, Intent, "S0_INTENT_SCOPE",
      repayLines(rounded, debt, 30, 1)))
    .then(submitRepay("alias", rounded, 0, 30,
      repayLines(rounded, debt, 30, 1), fixtureRequest("alias", rounded, 1)).fail())

  action aliasReady(i: SignedIntent): bool =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedBalanceOnly("F", 0)).then(sign(i))

  def rejectedAlias(i: SignedIntent): bool = all {
    not(transferObservation("overflow", i, 0, 10, 1,
      transferLines(i, 10, 1, 1), fixtureRequest("overflow", i, 1)).accepted),
    transferObservation("overflow", i, 0, 10, 1,
      transferLines(i, 10, 1, 1), fixtureRequest("overflow", i, 1)).judgment == Intent,
    transferObservation("overflow", i, 0, 10, 1,
      transferLines(i, 10, 1, 1), fixtureRequest("overflow", i, 1)).code == "S0_INTENT_SCOPE",
    committedCount == 0 and currentHead == 0,
    balances.get("O") == 100 and balances.get("F") == 0,
    allowanceRemaining.get("O") == 100 and allowanceSpent.get("O") == 0,
    workRemaining == 1 and workSpent == 0,
    consumed == Set() and lastEffects == List(),
  }

  run invalidRoundAndAliasTest = aliasReady(roundAlias)
    .expect(rejectedAlias(roundAlias))
    .then(submitTransfer("overflow", roundAlias, 0, 10, 1,
      transferLines(roundAlias, 10, 1, 1), fixtureRequest("overflow", roundAlias, 1)).fail())

  run capAndAliasTest = aliasReady(capAlias)
    .expect(rejectedAlias(capAlias))
    .then(submitTransfer("overflow", capAlias, 0, 10, 1,
      transferLines(capAlias, 10, 1, 1), fixtureRequest("overflow", capAlias, 1)).fail())

  run overpayAndMissingCreditorTest = repayReady(overpay, debt)
    .expect(rejectedRepay(overpay, debt, Effect, "S0_EFFECT_RANGE",
      missingCreditor))
    .then(submitRepay("alias", overpay, 0, 1011, missingCreditor, fixtureRequest("alias", overpay, 1)).fail())
}


```

## experiments/moriarty-language/formal/quint/mil4/corpus/s0_diagnostic_witnesses.qnt

```text
// Fixed full-decision witnesses. External admission premises are stipulated.
module s0_diagnostic_witnesses {
  import s0(
    signatureVerified = Set("t"),
    authenticatedSnapshots = Set(0, 1),
    nativeQualified = Set("t"),
    ledgerAtomicReady = true,
    executingDomain = "D",
    settlementAsset = "A",
    initialWorkBudget = 3
  ).* from "../s0"

  // This helper stipulates comparison input only. It establishes no premise.
  def fixtureRequest(id: str, supplied: SignedIntent, successor: int): ComparisonRequest = {
    submittedSuccessor: successor, outcome: terminalOutcome,
    premise: { available: true,
      intent: if (signed.keys().contains(id)) signed.get(id) else supplied,
      preState: financialSnapshot, round: round, expectedSuccessor: successor, requestedOutcome: terminalOutcome },
  }

  pure val t: SignedIntent = {
    sourceVersion: "Source/6", coreVersion: "Core/5", profile: "S0",
    program: TransferProgram, domain: "D", asset: "A", digest: "t",
    policyDigest: "p", evidenceChoice: "e", signer: "O", payer: "O",
    recipient: "R", feeRecipient: "F", obligationId: "",
    debtor: "", creditor: "", nonce: "nt", preHead: 0,
    validFrom: 0, validThrough: 2, grossCap: 11, feeCap: 1,
    netFloor: 10, actionAmount: 10, actionFee: 1,
    conversionMantissa: 1, conversionScale: 0, roundingNone: true,
    terminalOnly: true,
  }

  pure val missingFee: List[EffectLine] = List(
    Debit({ account: "O", amount: 11 }),
    Credit({ account: "R", amount: 10 }),
    UseAllowance({ owner: "O", amount: 11 }),
    UseReplay(replayKey(t)),
    AdvanceHead({ before: 0, after: 1 })
  )

  action transferReady: bool =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedBalanceOnly("R", 0))
      .then(seedBalanceOnly("F", 0))
      .then(sign(t))

  run stageDiagnosticTest = init
    .expect(all {
      transferObservation("missing", t, 0, 10, 1,
        transferLines(t, 10, 1, 1), fixtureRequest("missing", t, 1)) == {
          accepted: false, judgment: Stage, code: "S0_STAGE_UNSUPPORTED",
          diagnosticWork: 1,
        },
      committedCount == 0 and currentHead == 0,
      workRemaining == 3 and workSpent == 0,
      consumed == Set() and lastEffects == List(),
    })
    .then(submitTransfer("missing", t, 0, 10, 1,
      transferLines(t, 10, 1, 1), fixtureRequest("missing", t, 1)).fail())

  run effectDiagnosticTest = transferReady
    .expect(all {
      transferObservation("t", t, 0, 10, 1, missingFee, fixtureRequest("t", t, 1)) == {
        accepted: false, judgment: Effect, code: "S0_EFFECT_MISMATCH",
        diagnosticWork: 1,
      },
      balances.get("O") == 100 and balances.get("R") == 0 and
        balances.get("F") == 0,
      allowanceRemaining.get("O") == 100 and allowanceSpent.get("O") == 0,
      committedCount == 0 and currentHead == 0,
      workRemaining == 3 and workSpent == 0,
      consumed == Set() and lastEffects == List(),
    })
    .then(submitTransfer("t", t, 0, 10, 1, missingFee, fixtureRequest("t", t, 1)).fail())

  run historyDiagnosticTest = transferReady
    .expect(transferObservation("t", t, 0, 10, 1,
      transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)) == {
        accepted: true, judgment: Accepted, code: "", diagnosticWork: 0,
      })
    .then(submitTransfer("t", t, 0, 10, 1, transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)))
    .expect(all {
      transferObservation("t", t, 0, 10, 1,
        transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)) == {
          accepted: false, judgment: History, code: "S0_HISTORY_STALE",
          diagnosticWork: 1,
        },
      balances.get("O") == 89 and balances.get("R") == 10 and
        balances.get("F") == 1,
      allowanceRemaining.get("O") == 89 and allowanceSpent.get("O") == 11,
      committedCount == 1 and currentHead == 1,
      workRemaining == 2 and workSpent == 1,
      consumed == Set(replayKey(t)) and lastEffects == transferLines(t, 10, 1, 1),
    })
    .then(submitTransfer("t", t, 0, 10, 1,
      transferLines(t, 10, 1, 1), fixtureRequest("t", t, 1)).fail())
}

```

## experiments/moriarty-language/formal/quint/mil4/corpus/s0_common_witnesses.qnt

```text
// Three common-state positive witnesses with independent literal expectations.
// K h0/h1 map to 0/1 and repayment account P maps to O in this instance.
module s0_common_witnesses {
  import s0(
    signatureVerified = Set("digest-T-10-1", "digest-R-30", "digest-R-near-bound"),
    authenticatedSnapshots = Set(0),
    nativeQualified = Set("digest-T-10-1", "digest-R-30", "digest-R-near-bound"),
    ledgerAtomicReady = true,
    executingDomain = "D",
    settlementAsset = "A",
    initialWorkBudget = 1
  ).* from "../s0"

  // This helper stipulates comparison input only. It establishes no premise.
  def fixtureRequest(id: str, supplied: SignedIntent, successor: int): ComparisonRequest = {
    submittedSuccessor: successor, outcome: terminalOutcome,
    premise: { available: true,
      intent: if (signed.keys().contains(id)) signed.get(id) else supplied,
      preState: financialSnapshot, round: round, expectedSuccessor: successor, requestedOutcome: terminalOutcome },
  }

  pure val t: SignedIntent = {
    sourceVersion: "Source/6", coreVersion: "Core/5", profile: "S0",
    program: TransferProgram, domain: "D", asset: "A",
    digest: "digest-T-10-1", policyDigest: "p", evidenceChoice: "e",
    signer: "O", payer: "O", recipient: "R", feeRecipient: "F",
    obligationId: "", debtor: "", creditor: "", nonce: "T-10-1",
    preHead: 0, validFrom: 0, validThrough: 10,
    grossCap: 11, feeCap: 1, netFloor: 0, actionAmount: 10, actionFee: 1,
    conversionMantissa: 1, conversionScale: 0, roundingNone: true,
    terminalOnly: true,
  }
  pure val r: SignedIntent = {
    ...t, program: RepayProgram, digest: "digest-R-30", nonce: "R-30",
    recipient: "", feeRecipient: "", obligationId: "L",
    debtor: "O", creditor: "C", grossCap: 30, feeCap: 0,
    actionAmount: 30, actionFee: 0,
  }
  pure val near: SignedIntent = {
    ...r, digest: "digest-R-near-bound", nonce: "R-near-bound",
    grossCap: 1, actionAmount: 1,
  }
  pure val debt: Obligation = {
    debtor: "O", creditor: "C", asset: "A", principal: 1000,
    accrued: 10, outstanding: 1010, status: Outstanding,
  }
  pure val debtAfterThirty: Obligation = {
    debtor: "O", creditor: "C", asset: "A", principal: 980,
    accrued: 0, outstanding: 980, status: Outstanding,
  }
  pure val debtNear: Obligation = {
    debtor: "O", creditor: "C", asset: "A",
    principal: 170141183460469231731687303715884105726,
    accrued: 1, outstanding: 170141183460469231731687303715884105727,
    status: Outstanding,
  }
  pure val debtAfterOne: Obligation = {
    debtor: "O", creditor: "C", asset: "A",
    principal: 170141183460469231731687303715884105726,
    accrued: 0, outstanding: 170141183460469231731687303715884105726,
    status: Outstanding,
  }

  // Expected lines and debt values are literal; no model preparation helper
  // computes the oracle supplied to submit or checked in the successor.
  pure val transferExpected: List[EffectLine] = List(
    Debit({ account: "O", amount: 11 }),
    Credit({ account: "R", amount: 10 }),
    Credit({ account: "F", amount: 1 }),
    UseAllowance({ owner: "O", amount: 11 }),
    UseReplay(("D", "O", "T-10-1")),
    AdvanceHead({ before: 0, after: 1 })
  )
  pure val thirtyExpected: List[EffectLine] = List(
    Debit({ account: "O", amount: 30 }),
    Credit({ account: "C", amount: 30 }),
    SetObligation({ id: "L", value: debtAfterThirty }),
    UseAllowance({ owner: "O", amount: 30 }),
    UseReplay(("D", "O", "R-30")),
    AdvanceHead({ before: 0, after: 1 })
  )
  pure val oneExpected: List[EffectLine] = List(
    Debit({ account: "O", amount: 1 }),
    Credit({ account: "C", amount: 1 }),
    SetObligation({ id: "L", value: debtAfterOne }),
    UseAllowance({ owner: "O", amount: 1 }),
    UseReplay(("D", "O", "R-near-bound")),
    AdvanceHead({ before: 0, after: 1 })
  )

  run transferCommonTest =
    init.then(seedAccount("O", 100, 11, 0))
      .then(seedBalanceOnly("R", 0))
      .then(seedBalanceOnly("F", 0))
      .then(sign(t))
      .expect(all {
        signed == Map("digest-T-10-1" -> t),
        balances == Map("O" -> 100, "R" -> 0, "F" -> 0),
        allowanceRemaining == Map("O" -> 11),
        allowanceSpent == Map("O" -> 0),
        obligations == Map(),
        consumed == Set() and lastEffects == List(),
        currentHead == 0 and round == 0 and committedCount == 0,
        workRemaining == 1 and workSpent == 0,
        transferObservation("digest-T-10-1", t, 0, 10, 1,
          transferExpected, fixtureRequest("digest-T-10-1", t, 1)) == {
            accepted: true, judgment: Accepted, code: "", diagnosticWork: 0,
          },
      })
      .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, transferExpected, fixtureRequest("digest-T-10-1", t, 1)))
      .expect(all {
        signed == Map("digest-T-10-1" -> t),
        balances == Map("O" -> 89, "R" -> 10, "F" -> 1),
        allowanceRemaining == Map("O" -> 0),
        allowanceSpent == Map("O" -> 11),
        not(allowanceRemaining.keys().contains("R")) and
          not(allowanceRemaining.keys().contains("F")),
        not(allowanceSpent.keys().contains("R")) and
          not(allowanceSpent.keys().contains("F")),
        obligations == Map(),
        consumed == Set(("D", "O", "T-10-1")),
        lastEffects == transferExpected,
        currentHead == 1 and round == 0 and committedCount == 1,
        workRemaining == 0 and workSpent == 1,
      })

  run repayThirtyCommonTest =
    init.then(seedAccount("O", 100, 100, 0))
      .then(seedBalanceOnly("C", 0))
      .then(seedObligation("L", debt)).then(sign(r))
      .expect(all {
        signed == Map("digest-R-30" -> r),
        balances == Map("O" -> 100, "C" -> 0),
        allowanceRemaining == Map("O" -> 100),
        allowanceSpent == Map("O" -> 0),
        obligations == Map("L" -> debt),
        consumed == Set() and lastEffects == List(),
        currentHead == 0 and round == 0 and committedCount == 0,
        workRemaining == 1 and workSpent == 0,
        repayObservation("digest-R-30", r, 0, 30, thirtyExpected, fixtureRequest("digest-R-30", r, 1)) == {
          accepted: true, judgment: Accepted, code: "", diagnosticWork: 0,
        },
      })
      .then(submitRepay("digest-R-30", r, 0, 30, thirtyExpected, fixtureRequest("digest-R-30", r, 1)))
      .expect(all {
        signed == Map("digest-R-30" -> r),
        balances == Map("O" -> 70, "C" -> 30),
        allowanceRemaining == Map("O" -> 70),
        allowanceSpent == Map("O" -> 30),
        not(allowanceRemaining.keys().contains("C")) and
          not(allowanceSpent.keys().contains("C")),
        obligations == Map("L" -> debtAfterThirty),
        consumed == Set(("D", "O", "R-30")),
        lastEffects == thirtyExpected,
        currentHead == 1 and round == 0 and committedCount == 1,
        workRemaining == 0 and workSpent == 1,
      })

  run repayNearBoundCommonTest =
    init.then(seedAccount("O", 1, 1, 340282366920938463463374607431768211454))
      .then(seedBalanceOnly("C", 340282366920938463463374607431768211454))
      .then(seedObligation("L", debtNear)).then(sign(near))
      .expect(all {
        signed == Map("digest-R-near-bound" -> near),
        balances == Map("O" -> 1,
          "C" -> 340282366920938463463374607431768211454),
        allowanceRemaining == Map("O" -> 1),
        allowanceSpent == Map("O" -> 340282366920938463463374607431768211454),
        obligations == Map("L" -> debtNear),
        consumed == Set() and lastEffects == List(),
        currentHead == 0 and round == 0 and committedCount == 0,
        workRemaining == 1 and workSpent == 0,
        repayObservation("digest-R-near-bound", near, 0, 1, oneExpected, fixtureRequest("digest-R-near-bound", near, 1)) == {
          accepted: true, judgment: Accepted, code: "", diagnosticWork: 0,
        },
      })
      .then(submitRepay("digest-R-near-bound", near, 0, 1, oneExpected, fixtureRequest("digest-R-near-bound", near, 1)))
      .expect(all {
        signed == Map("digest-R-near-bound" -> near),
        balances == Map("O" -> 0,
          "C" -> 340282366920938463463374607431768211455),
        allowanceRemaining == Map("O" -> 0),
        allowanceSpent == Map("O" -> 340282366920938463463374607431768211455),
        not(allowanceRemaining.keys().contains("C")) and
          not(allowanceSpent.keys().contains("C")),
        obligations == Map("L" -> debtAfterOne),
        consumed == Set(("D", "O", "R-near-bound")),
        lastEffects == oneExpected,
        currentHead == 1 and round == 0 and committedCount == 1,
        workRemaining == 0 and workSpent == 1,
      })
}

```

## experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt

```text
// S1B finite local observations. Every tuple is stipulated, never authenticated here.
module s0_s1b_witnesses {
  import s0(
    signatureVerified = Set("digest-T-10-1", "digest-R-30"),
    authenticatedSnapshots = Set(0),
    nativeQualified = Set("digest-T-10-1", "digest-R-30"),
    ledgerAtomicReady = true, executingDomain = "D", settlementAsset = "A",
    initialWorkBudget = 1
  ).* from "../s0"

  pure val t: SignedIntent = {
    sourceVersion: "Source/6", coreVersion: "Core/5", profile: "S0",
    program: TransferProgram, domain: "D", asset: "A",
    digest: "digest-T-10-1", policyDigest: "p", evidenceChoice: "e",
    signer: "O", payer: "O", recipient: "R", feeRecipient: "F",
    obligationId: "", debtor: "", creditor: "", nonce: "T-10-1",
    preHead: 0, validFrom: 0, validThrough: 10,
    grossCap: 11, feeCap: 1, netFloor: 0, actionAmount: 10, actionFee: 1,
    conversionMantissa: 1, conversionScale: 0, roundingNone: true,
    terminalOnly: true,
  }

  pure val pre: FinancialSnapshot = {
    balances: Map("O" -> 100, "R" -> 0, "F" -> 0),
    allowanceRemaining: Map("O" -> 11), allowanceSpent: Map("O" -> 0),
    obligations: Map(), head: 0, consumed: Set(),
    workRemaining: 1, workSpent: 0,
  }
  pure val terminal: RequestedOutcome = {
    phase: TerminalSuccess, retainedEffects: List(), retainedDuties: List(),
  }
  pure val tuple: StipulatedTuple = {
    available: true, intent: t, preState: pre, round: 0, expectedSuccessor: 1,
    requestedOutcome: terminal,
  }
  pure val request: ComparisonRequest = {
    submittedSuccessor: 1, outcome: terminal, premise: tuple,
  }
  pure def effects(before: int, after: int): List[EffectLine] = List(
    Debit({ account: "O", amount: 11 }),
    Credit({ account: "R", amount: 10 }),
    Credit({ account: "F", amount: 1 }),
    UseAllowance({ owner: "O", amount: 11 }),
    UseReplay(("D", "O", "T-10-1")),
    AdvanceHead({ before: before, after: after })
  )
  pure val expected: List[EffectLine] = effects(0, 1)

  action ready(i: SignedIntent): bool =
    init.then(seedAccount("O", 100, 11, 0))
      .then(seedBalanceOnly("R", 0)).then(seedBalanceOnly("F", 0))
      .then(sign(i))

  // Malformed-cell and consumed-replay fixtures bypass seed guards deliberately.
  action replacePre(p: FinancialSnapshot): bool = all {
    balances' = p.balances, allowanceRemaining' = p.allowanceRemaining,
    allowanceSpent' = p.allowanceSpent, obligations' = p.obligations,
    currentHead' = p.head, consumed' = p.consumed,
    workRemaining' = p.workRemaining, workSpent' = p.workSpent,
    signed' = signed, round' = round, lastEffects' = lastEffects,
    committedCount' = committedCount,
  }

  def rejected(i: SignedIntent, p: FinancialSnapshot, req: ComparisonRequest,
               lines: List[EffectLine], j: Judgment, c: str): bool = all {
    transferObservation("digest-T-10-1", i, i.preHead, 10, 1, lines, req) == {
      accepted: false, judgment: j, code: c, diagnosticWork: 1,
    },
    financialSnapshot == p,
    signed == Map("digest-T-10-1" -> i),
    round == 0 and committedCount == 0 and lastEffects == List(),
  }

  run H1Test = ready(t)
    .expect(all {
      financialSnapshot == pre,
      transferObservation("digest-T-10-1", t, 0, 10, 1, expected, request) == {
        accepted: true, judgment: Accepted, code: "", diagnosticWork: 0,
      },
    })
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, request))
    .expect(all {
      financialSnapshot == {
        balances: Map("O" -> 89, "R" -> 10, "F" -> 1),
        allowanceRemaining: Map("O" -> 0), allowanceSpent: Map("O" -> 11),
        obligations: Map(), head: 1, consumed: Set(("D", "O", "T-10-1")),
        workRemaining: 0, workSpent: 1,
      },
      signed == Map("digest-T-10-1" -> t),
      round == 0 and committedCount == 1 and lastEffects == expected,
    })

  pure val wrongHead = { ...request, submittedSuccessor: 9 }
  pure val selfHead = { ...request, submittedSuccessor: 0 }
  run H2Test = ready(t)
    .expect(rejected(t, pre, wrongHead, effects(0, 9), History, "S0_HISTORY_SUCCESSOR"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      effects(0, 9), wrongHead).fail())

  run H3Test = ready(t)
    .expect(rejected(t, pre, selfHead, effects(0, 0), History, "S0_HISTORY_SUCCESSOR"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      effects(0, 0), selfHead).fail())

  run H4Test = ready(t)
    .expect(rejected(t, pre, request, effects(0, 9), Effect, "S0_EFFECT_MISMATCH"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      effects(0, 9), request).fail())

  pure val stale = { ...t, preHead: 9 }
  pure val staleRequest = {
    ...wrongHead, premise: { ...tuple, intent: stale },
  }
  run H5Test = ready(stale)
    .expect(rejected(stale, pre, staleRequest, effects(9, 9), History, "S0_HISTORY_STALE"))
    .then(submitTransfer("digest-T-10-1", stale, stale.preHead, 10, 1,
      effects(9, 9), staleRequest).fail())

  pure val replayPre = { ...pre, consumed: Set(("D", "O", "T-10-1")) }
  pure val replayRequest = {
    ...wrongHead, premise: { ...tuple, preState: replayPre },
  }
  run H6Test = ready(t)
    .then(replacePre(replayPre))
    .expect(rejected(t, replayPre, replayRequest, effects(0, 9), History, "S0_HISTORY_REPLAY"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      effects(0, 9), replayRequest).fail())

  pure val failureOutcome = { ...terminal, phase: RequestedFailure }
  pure val retainedEffectOutcome = {
    ...terminal, retainedEffects: List(Credit({ account: "R", amount: 10 })),
  }
  pure val retainedDutyOutcome = {
    ...terminal, retainedDuties: List({ id: "d", owner: "O", amount: 1 }),
  }
  pure val failureRequest = {
    ...request, outcome: failureOutcome,
    premise: { ...tuple, requestedOutcome: failureOutcome },
  }
  pure val retainedEffectRequest = {
    ...request, outcome: retainedEffectOutcome,
    premise: { ...tuple, requestedOutcome: retainedEffectOutcome },
  }
  pure val retainedDutyRequest = {
    ...request, outcome: retainedDutyOutcome,
    premise: { ...tuple, requestedOutcome: retainedDutyOutcome },
  }
  run F1Test = ready(t)
    .expect(rejected(t, pre, failureRequest, expected, Failure, "S0_FAILURE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      expected, failureRequest).fail())

  run F2Test = ready(t)
    .expect(rejected(t, pre, retainedEffectRequest, expected, Failure, "S0_FAILURE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      expected, retainedEffectRequest).fail())

  run F3Test = ready(t)
    .expect(rejected(t, pre, retainedDutyRequest, expected, Failure, "S0_FAILURE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      expected, retainedDutyRequest).fail())

  pure val unavailableRequest = {
    ...request, premise: { ...tuple, available: false },
  }
  pure val changedIntentRequest = {
    ...request, premise: { ...tuple, intent: { ...t, nonce: "different" } },
  }
  pure val changedPreRequest = {
    ...request, premise: { ...tuple, preState: { ...pre, balances: Map("O" -> 99, "R" -> 0, "F" -> 0) } },
  }
  pure val changedRoundRequest = {
    ...request, premise: { ...tuple, round: 1 },
  }
  pure val malformedPre = { ...pre, balances: Map("R" -> 0, "F" -> 0) }
  pure val malformedRequest = {
    ...unavailableRequest, premise: {
      ...unavailableRequest.premise, preState: malformedPre },
  }
  run P1Test = ready(t)
    .expect(rejected(t, pre, unavailableRequest, expected, Stage, "S0_STAGE_PREMISE"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      expected, unavailableRequest).fail())

  run P2IntentTest = ready(t)
    .expect(rejected(t, pre, changedIntentRequest, expected, Stage, "S0_STAGE_PREMISE"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      expected, changedIntentRequest).fail())

  run P2PreStateTest = ready(t)
    .expect(rejected(t, pre, changedPreRequest, expected, Stage, "S0_STAGE_PREMISE"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      expected, changedPreRequest).fail())

  run P2RoundTest = ready(t)
    .expect(rejected(t, pre, changedRoundRequest, expected, Stage, "S0_STAGE_PREMISE"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      expected, changedRoundRequest).fail())

  run P3Test = ready(t)
    .then(replacePre(malformedPre))
    .expect(rejected(t, malformedPre, malformedRequest, expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      expected, malformedRequest).fail())

  pure val abstractNineRequest = {
    ...wrongHead, premise: { ...tuple, expectedSuccessor: 9 },
  }
  run nonArithmeticSuccessorTest = ready(t)
    .expect(transferObservation("digest-T-10-1", t, 0, 10, 1,
      effects(0, 9), abstractNineRequest) == {
        accepted: true, judgment: Accepted, code: "", diagnosticWork: 0,
      })
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1,
      effects(0, 9), abstractNineRequest))
    .expect(financialSnapshot == {
      balances: Map("O" -> 89, "R" -> 10, "F" -> 1),
      allowanceRemaining: Map("O" -> 0), allowanceSpent: Map("O" -> 11),
      obligations: Map(), head: 9, consumed: Set(("D", "O", "T-10-1")),
      workRemaining: 0, workSpent: 1,
    } and lastEffects == effects(0, 9) and committedCount == 1 and
      signed == Map("digest-T-10-1" -> t) and round == 0)

  run failureAfterEffectTest = ready(t)
    .expect(rejected(t, pre, failureRequest, effects(0, 9), Effect, "S0_EFFECT_MISMATCH"))
    .then(submitTransfer("digest-T-10-1", t, t.preHead, 10, 1,
      effects(0, 9), failureRequest).fail())

  pure val r: SignedIntent = {
    ...t, program: RepayProgram, digest: "digest-R-30", nonce: "R-30",
    recipient: "", feeRecipient: "", obligationId: "L",
    debtor: "O", creditor: "C", grossCap: 30, feeCap: 0,
    actionAmount: 30, actionFee: 0,
  }
  pure val debt: Obligation = {
    debtor: "O", creditor: "C", asset: "A", principal: 1000,
    accrued: 10, outstanding: 1010, status: Outstanding,
  }
  pure val debtAfter: Obligation = { ...debt, principal: 980, accrued: 0, outstanding: 980 }
  pure val repayPre: FinancialSnapshot = {
    ...pre, balances: Map("O" -> 100, "C" -> 0),
    allowanceRemaining: Map("O" -> 100), obligations: Map("L" -> debt),
  }
  pure val repayExpected: List[EffectLine] = List(
    Debit({ account: "O", amount: 30 }), Credit({ account: "C", amount: 30 }),
    SetObligation({ id: "L", value: debtAfter }),
    UseAllowance({ owner: "O", amount: 30 }),
    UseReplay(("D", "O", "R-30")), AdvanceHead({ before: 0, after: 1 })
  )
  pure val recipientRepay = { ...r, recipient: "C" }
  pure val feeRecipientRepay = { ...r, feeRecipient: "F" }
  pure def repayRequest(i: SignedIntent): ComparisonRequest = {
    ...request, premise: { ...tuple, intent: i, preState: repayPre },
  }
  action repayReady(i: SignedIntent): bool =
    init.then(seedAccount("O", 100, 100, 0)).then(seedBalanceOnly("C", 0))
      .then(seedObligation("L", debt)).then(sign(i))
  def rejectedEndpoint(i: SignedIntent): bool = all {
    repayObservation("digest-R-30", i, 0, 30, repayExpected, repayRequest(i)) == {
      accepted: false, judgment: Intent, code: "S0_INTENT_SCOPE", diagnosticWork: 1,
    },
    financialSnapshot == repayPre,
    signed == Map("digest-R-30" -> i),
    round == 0 and lastEffects == List() and committedCount == 0,
  }
  run repayRecipientAbsentTest = repayReady(recipientRepay)
    .expect(rejectedEndpoint(recipientRepay))
    .then(submitRepay("digest-R-30", recipientRepay, 0, 30,
      repayExpected, repayRequest(recipientRepay)).fail())
  run repayFeeRecipientAbsentTest = repayReady(feeRecipientRepay)
    .expect(rejectedEndpoint(feeRecipientRepay))
    .then(submitRepay("digest-R-30", feeRecipientRepay, 0, 30,
      repayExpected, repayRequest(feeRecipientRepay)).fail())

  pure val changedOutcomeRequest = {
    ...request, premise: { ...tuple, requestedOutcome: failureOutcome },
  }
  run P2OutcomeTest = ready(t)
    .expect(rejected(t, pre, changedOutcomeRequest, expected, Stage, "S0_STAGE_PREMISE"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1,
      expected, changedOutcomeRequest).fail())

  pure val selfStipulatedRequest = {
    ...selfHead, premise: { ...tuple, expectedSuccessor: 0 },
  }
  run H3SelfPremiseTest = ready(t)
    .expect(rejected(t, pre, selfStipulatedRequest, effects(0, 0),
      History, "S0_HISTORY_SUCCESSOR"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1,
      effects(0, 0), selfStipulatedRequest).fail())

  pure val staleReplayRequest = {
    ...request, premise: { ...tuple, intent: stale, preState: replayPre },
  }
  run staleAndReplayTest = ready(stale)
    .then(replacePre(replayPre))
    .expect(rejected(stale, replayPre, staleReplayRequest, effects(9, 1),
      History, "S0_HISTORY_STALE"))
    .then(submitTransfer("digest-T-10-1", stale, 9, 10, 1,
      effects(9, 1), staleReplayRequest).fail())

  pure val replayOnlyRequest = {
    ...request, premise: { ...tuple, preState: replayPre },
  }
  run effectAndReplayTest = ready(t)
    .then(replacePre(replayPre))
    .expect(rejected(t, replayPre, replayOnlyRequest, effects(0, 9),
      Effect, "S0_EFFECT_MISMATCH"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1,
      effects(0, 9), replayOnlyRequest).fail())

  run effectAndWrongSuccessorTest = ready(t)
    .expect(rejected(t, pre, wrongHead, expected, Effect, "S0_EFFECT_MISMATCH"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1,
      expected, wrongHead).fail())

  pure val post: FinancialSnapshot = {
    balances: Map("O" -> 89, "R" -> 10, "F" -> 1),
    allowanceRemaining: Map("O" -> 0), allowanceSpent: Map("O" -> 11),
    obligations: Map(), head: 1, consumed: Set(("D", "O", "T-10-1")),
    workRemaining: 0, workSpent: 1,
  }
  pure val postRequest = {
    ...request, premise: { ...tuple, preState: post },
  }
  // Only head 0 is stipulated as authenticated in this module. Local success
  // leaves head 1 unavailable, even though a commit exists and tuple matches.
  run committedHeadPremiseUnavailableTest = ready(t)
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, request))
    .expect(all {
      transferObservation("digest-T-10-1", t, 0, 10, 1,
        expected, postRequest) == {
          accepted: false, judgment: Stage, code: "S0_STAGE_PREMISE", diagnosticWork: 1,
        },
      financialSnapshot == post,
      signed == Map("digest-T-10-1" -> t),
      round == 0 and committedCount == 1 and lastEffects == expected,
      not(currentSnapshotAuthenticated),
    })
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1,
      expected, postRequest).fail())

  // Postreview Stage controls: complete matching tuples isolate local shape.
  pure def stageRequest(i: SignedIntent, p: FinancialSnapshot): ComparisonRequest = {
    ...request, premise: { ...tuple, intent: i, preState: p },
  }
  def repayStageRejected(p: FinancialSnapshot, req: ComparisonRequest): bool = all {
    repayObservation("digest-R-30", r, 0, 30, repayExpected, req) == {
      accepted: false, judgment: Stage, code: "S0_STAGE_UNSUPPORTED", diagnosticWork: 1,
    },
    financialSnapshot == p,
    signed == Map("digest-R-30" -> r),
    round == 0 and committedCount == 0 and lastEffects == List(),
  }

  pure val transferExtraBalancePre: FinancialSnapshot = { ...pre, balances: pre.balances.put("X", 0) }
  run transferStageExtraBalanceTest = ready(t)
    .then(replacePre(transferExtraBalancePre))
    .expect(rejected(t, transferExtraBalancePre, stageRequest(t, transferExtraBalancePre), expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferExtraBalancePre)).fail())

  pure val transferExtraRemainingPre: FinancialSnapshot = { ...pre, allowanceRemaining: pre.allowanceRemaining.put("X", 0) }
  run transferStageExtraRemainingTest = ready(t)
    .then(replacePre(transferExtraRemainingPre))
    .expect(rejected(t, transferExtraRemainingPre, stageRequest(t, transferExtraRemainingPre), expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferExtraRemainingPre)).fail())

  pure val transferExtraSpentPre: FinancialSnapshot = { ...pre, allowanceSpent: pre.allowanceSpent.put("X", 0) }
  run transferStageExtraSpentTest = ready(t)
    .then(replacePre(transferExtraSpentPre))
    .expect(rejected(t, transferExtraSpentPre, stageRequest(t, transferExtraSpentPre), expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferExtraSpentPre)).fail())

  pure val transferExtraObligationPre: FinancialSnapshot = { ...pre, obligations: pre.obligations.put("X", debt) }
  run transferStageExtraObligationTest = ready(t)
    .then(replacePre(transferExtraObligationPre))
    .expect(rejected(t, transferExtraObligationPre, stageRequest(t, transferExtraObligationPre), expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferExtraObligationPre)).fail())

  pure val transferNegativeWorkRemainingPre: FinancialSnapshot = { ...pre, workRemaining: -1 }
  run transferStageNegativeWorkRemainingTest = ready(t)
    .then(replacePre(transferNegativeWorkRemainingPre))
    .expect(rejected(t, transferNegativeWorkRemainingPre, stageRequest(t, transferNegativeWorkRemainingPre), expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferNegativeWorkRemainingPre)).fail())

  pure val transferNegativeWorkSpentPre: FinancialSnapshot = { ...pre, workSpent: -1 }
  run transferStageNegativeWorkSpentTest = ready(t)
    .then(replacePre(transferNegativeWorkSpentPre))
    .expect(rejected(t, transferNegativeWorkSpentPre, stageRequest(t, transferNegativeWorkSpentPre), expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferNegativeWorkSpentPre)).fail())

  pure val transferOverBoundWorkRemainingPre: FinancialSnapshot = { ...pre, workRemaining: UINT128_MAX + 1 }
  run transferStageOverBoundWorkRemainingTest = ready(t)
    .then(replacePre(transferOverBoundWorkRemainingPre))
    .expect(rejected(t, transferOverBoundWorkRemainingPre, stageRequest(t, transferOverBoundWorkRemainingPre), expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferOverBoundWorkRemainingPre)).fail())

  pure val transferOverBoundWorkSpentPre: FinancialSnapshot = { ...pre, workSpent: UINT128_MAX + 1 }
  run transferStageOverBoundWorkSpentTest = ready(t)
    .then(replacePre(transferOverBoundWorkSpentPre))
    .expect(rejected(t, transferOverBoundWorkSpentPre, stageRequest(t, transferOverBoundWorkSpentPre), expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferOverBoundWorkSpentPre)).fail())

  pure val transferWorkAggregateOverflowPre: FinancialSnapshot = { ...pre, workRemaining: 2, workSpent: UINT128_MAX - 1 }
  run transferStageWorkAggregateOverflowTest = ready(t)
    .then(replacePre(transferWorkAggregateOverflowPre))
    .expect(rejected(t, transferWorkAggregateOverflowPre, stageRequest(t, transferWorkAggregateOverflowPre), expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferWorkAggregateOverflowPre)).fail())

  pure val transferAllowanceAggregateOverflowPre: FinancialSnapshot = { ...pre, allowanceRemaining: Map("O" -> 100), allowanceSpent: Map("O" -> UINT128_MAX - 1) }
  run transferStageAllowanceAggregateOverflowTest = ready(t)
    .then(replacePre(transferAllowanceAggregateOverflowPre))
    .expect(rejected(t, transferAllowanceAggregateOverflowPre, stageRequest(t, transferAllowanceAggregateOverflowPre), expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferAllowanceAggregateOverflowPre)).fail())

  run transferStageWorkBeforePremiseTest = ready(t)
    .then(replacePre(transferNegativeWorkSpentPre))
    .expect(rejected(t, transferNegativeWorkSpentPre, { ...stageRequest(t, transferNegativeWorkSpentPre), premise: { ...tuple, available: false } }, expected, Stage, "S0_STAGE_UNSUPPORTED"))
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, { ...stageRequest(t, transferNegativeWorkSpentPre), premise: { ...tuple, available: false } }).fail())

  pure val transferWorkBoundaryPre: FinancialSnapshot = { ...pre, workRemaining: 1, workSpent: UINT128_MAX - 1 }
  run transferStageWorkBoundaryTest = ready(t)
    .then(replacePre(transferWorkBoundaryPre))
    .expect(transferObservation("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferWorkBoundaryPre)) == ok)
    .then(submitTransfer("digest-T-10-1", t, 0, 10, 1, expected, stageRequest(t, transferWorkBoundaryPre)))
    .expect(workRemaining == 0 and workSpent == UINT128_MAX and committedCount == 1)

  pure val repayExtraBalancePre: FinancialSnapshot = { ...repayPre, balances: repayPre.balances.put("X", 0) }
  run repayStageExtraBalanceTest = repayReady(r)
    .then(replacePre(repayExtraBalancePre))
    .expect(repayStageRejected(repayExtraBalancePre, stageRequest(r, repayExtraBalancePre)))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayExtraBalancePre)).fail())

  pure val repayExtraRemainingPre: FinancialSnapshot = { ...repayPre, allowanceRemaining: repayPre.allowanceRemaining.put("X", 0) }
  run repayStageExtraRemainingTest = repayReady(r)
    .then(replacePre(repayExtraRemainingPre))
    .expect(repayStageRejected(repayExtraRemainingPre, stageRequest(r, repayExtraRemainingPre)))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayExtraRemainingPre)).fail())

  pure val repayExtraSpentPre: FinancialSnapshot = { ...repayPre, allowanceSpent: repayPre.allowanceSpent.put("X", 0) }
  run repayStageExtraSpentTest = repayReady(r)
    .then(replacePre(repayExtraSpentPre))
    .expect(repayStageRejected(repayExtraSpentPre, stageRequest(r, repayExtraSpentPre)))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayExtraSpentPre)).fail())

  pure val repayExtraObligationPre: FinancialSnapshot = { ...repayPre, obligations: repayPre.obligations.put("X", debt) }
  run repayStageExtraObligationTest = repayReady(r)
    .then(replacePre(repayExtraObligationPre))
    .expect(repayStageRejected(repayExtraObligationPre, stageRequest(r, repayExtraObligationPre)))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayExtraObligationPre)).fail())

  pure val repayNegativeWorkRemainingPre: FinancialSnapshot = { ...repayPre, workRemaining: -1 }
  run repayStageNegativeWorkRemainingTest = repayReady(r)
    .then(replacePre(repayNegativeWorkRemainingPre))
    .expect(repayStageRejected(repayNegativeWorkRemainingPre, stageRequest(r, repayNegativeWorkRemainingPre)))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayNegativeWorkRemainingPre)).fail())

  pure val repayNegativeWorkSpentPre: FinancialSnapshot = { ...repayPre, workSpent: -1 }
  run repayStageNegativeWorkSpentTest = repayReady(r)
    .then(replacePre(repayNegativeWorkSpentPre))
    .expect(repayStageRejected(repayNegativeWorkSpentPre, stageRequest(r, repayNegativeWorkSpentPre)))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayNegativeWorkSpentPre)).fail())

  pure val repayOverBoundWorkRemainingPre: FinancialSnapshot = { ...repayPre, workRemaining: UINT128_MAX + 1 }
  run repayStageOverBoundWorkRemainingTest = repayReady(r)
    .then(replacePre(repayOverBoundWorkRemainingPre))
    .expect(repayStageRejected(repayOverBoundWorkRemainingPre, stageRequest(r, repayOverBoundWorkRemainingPre)))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayOverBoundWorkRemainingPre)).fail())

  pure val repayOverBoundWorkSpentPre: FinancialSnapshot = { ...repayPre, workSpent: UINT128_MAX + 1 }
  run repayStageOverBoundWorkSpentTest = repayReady(r)
    .then(replacePre(repayOverBoundWorkSpentPre))
    .expect(repayStageRejected(repayOverBoundWorkSpentPre, stageRequest(r, repayOverBoundWorkSpentPre)))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayOverBoundWorkSpentPre)).fail())

  pure val repayWorkAggregateOverflowPre: FinancialSnapshot = { ...repayPre, workRemaining: 2, workSpent: UINT128_MAX - 1 }
  run repayStageWorkAggregateOverflowTest = repayReady(r)
    .then(replacePre(repayWorkAggregateOverflowPre))
    .expect(repayStageRejected(repayWorkAggregateOverflowPre, stageRequest(r, repayWorkAggregateOverflowPre)))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayWorkAggregateOverflowPre)).fail())

  pure val repayAllowanceAggregateOverflowPre: FinancialSnapshot = { ...repayPre, allowanceRemaining: Map("O" -> 100), allowanceSpent: Map("O" -> UINT128_MAX - 1) }
  run repayStageAllowanceAggregateOverflowTest = repayReady(r)
    .then(replacePre(repayAllowanceAggregateOverflowPre))
    .expect(repayStageRejected(repayAllowanceAggregateOverflowPre, stageRequest(r, repayAllowanceAggregateOverflowPre)))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayAllowanceAggregateOverflowPre)).fail())

  run repayStageWorkBeforePremiseTest = repayReady(r)
    .then(replacePre(repayNegativeWorkSpentPre))
    .expect(repayStageRejected(repayNegativeWorkSpentPre, { ...stageRequest(r, repayNegativeWorkSpentPre), premise: { ...tuple, available: false } }))
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, { ...stageRequest(r, repayNegativeWorkSpentPre), premise: { ...tuple, available: false } }).fail())

  pure val repayWorkBoundaryPre: FinancialSnapshot = { ...repayPre, workRemaining: 1, workSpent: UINT128_MAX - 1 }
  run repayStageWorkBoundaryTest = repayReady(r)
    .then(replacePre(repayWorkBoundaryPre))
    .expect(repayObservation("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayWorkBoundaryPre)) == ok)
    .then(submitRepay("digest-R-30", r, 0, 30, repayExpected, stageRequest(r, repayWorkBoundaryPre)))
    .expect(workRemaining == 0 and workSpent == UINT128_MAX and committedCount == 1)
}

```

## experiments/moriarty-language/formal/quint/mil4/corpus/S1B-RESULTS.md

```text
# S1B finite Quint results

Quint 0.32.0, TypeScript backend, seed `0x5`: 83/83 finite witnesses passed
(59 existing witnesses and 24 Stage guard controls).
Six `.qnt` files typechecked. Exact argv, exit codes, stdout, stderr and
candidate hashes are retained in
[s1b-stage-guard-command-results.json](s1b-stage-guard-command-results.json).
That new receipt supersedes the prior S1B candidate for the local Quint result;
[s1b-command-results.json](s1b-command-results.json) remains unchanged.
The previous S1B receipt is preserved byte-for-byte as
[s1b-pre-audit-command-results.json](s1b-pre-audit-command-results.json).
Earlier receipts describe their recorded bytes.

| Corpus | Passed |
| --- | ---: |
| Original `s0_witnesses` | 15 |
| Ordering `s0_divergence_witnesses` | 14 |
| Common-state `s0_common_witnesses` | 3 |
| Diagnostic `s0_diagnostic_witnesses` | 3 |
| S1B `s0_s1b_witnesses` | 48 |
| Total | 83 |

The retained original witnesses now supply explicit successor IDs and typed
comparison requests. Original repayment fixtures use absent recipient and
fee-recipient fields. Original rejection witnesses pin exact diagnostic
codes. Original and diagnostic history imports explicitly stipulate heads
`0` and `1` for postcommit stale/replay checks. `committedCount` contributes
nothing to snapshot authentication. A separate S1B witness stipulates only
head `0`, commits head `1`, and rejects its next submission at Stage because
that current snapshot premise is unavailable.

| S1B witness | Complete first decision |
| --- | --- |
| H1 | Accepted, empty code, diagnostic work 0 |
| H2, H3, H3SelfPremise | History, `S0_HISTORY_SUCCESSOR`, diagnostic work 1 |
| H4 | Effect, `S0_EFFECT_MISMATCH`, diagnostic work 1 |
| H5 | History, `S0_HISTORY_STALE`, diagnostic work 1 |
| H6 | History, `S0_HISTORY_REPLAY`, diagnostic work 1 |
| F1, F2, F3 | Failure, `S0_FAILURE_UNSUPPORTED`, diagnostic work 1 |
| P1 | Stage, `S0_STAGE_PREMISE`, diagnostic work 1 |
| P2Intent, P2PreState, P2Round, P2Outcome | Stage, `S0_STAGE_PREMISE`, diagnostic work 1 |
| P3 | Stage, `S0_STAGE_UNSUPPORTED`, diagnostic work 1 |
| nonArithmeticSuccessor | Accepted stipulated `0 → 9`, empty code, diagnostic work 0 |
| failureAfterEffect, effectAndReplay, effectAndWrongSuccessor | Effect, `S0_EFFECT_MISMATCH`, diagnostic work 1 |
| staleAndReplay | History, `S0_HISTORY_STALE`, diagnostic work 1 |
| committedHeadPremiseUnavailable | Stage, `S0_STAGE_PREMISE`, diagnostic work 1 |
| repayRecipientAbsent, repayFeeRecipientAbsent | Intent, `S0_INTENT_SCOPE`, diagnostic work 1 |

Every S1B negative witness checks literal complete financial pre-state,
signed-intent map, round, commit count and published effect record, then
checks that submission is disabled. The postcommit unavailable-head witness
retains the previous accepted effects; all other rejection fixtures have an
empty record. `fail()` ends the witness trace; a
disabled action has no committed successor, so these are pre-observation
and blocked-submission checks rather than a sampled post-rejection state.
H1 and the separately stipulated `0 → 9` witness check literal complete
post-state and exact effects. The common H/F/P baseline has round 0, net floor
0, balances O100/R0/F0, owner allowance11/spent0 and work1/spent0. H4 submits
head1 with an effect head9; H5 has predecessor9, current head0 and submitted
head9; H6 has consumed replay and submitted head9. P2PreState changes only the
premise owner balance from 100 to 99. P3 omits the required owner balance cell
and marks the tuple unavailable. These are Quint observations on named
comparison fixtures; this file does not execute a K or TypeScript comparison.
The non-arithmetic, absent-endpoint and unavailable-head witnesses are
supplementary Quint controls.

The tuple binds stored intent, complete financial snapshot, round,
expected successor and requested outcome. A supplied intent substitution still reaches Intent when
the stipulated tuple correctly names the stored intent. Successor equality
is checked after stale predecessor and consumed replay; the effect vector
must first agree with the submitted successor. Requested phase and retained
vectors are typed. Stage checks equality of the submitted outcome and
premise outcome; Failure checks whether that matching outcome is supported.
F1–F3 bind their requested outcomes in the tuple; P2Outcome changes the tuple
outcome only. Combined controls pin stale before replay and effect mismatch
before replay or successor mismatch. H3SelfPremise rejects a self-successor
even when the tuple names that same successor.

These executions establish local behavior under supplied premises. They do
not authenticate signatures or snapshots, establish head extension, qualify
native proofs, execute a ledger, prove exhaustive invariants or implement an
executable K/Quint projection. W-D0–W-D4 and Sprint 1 remain open.

## Postreview Stage guard repair

Repository observation: distinct Transfer endpoints require exactly their three
balance keys, empty obligations and singleton payer remaining/spent allowance
maps. Repay requires exactly payer and stored creditor balance keys, the singleton
bound obligation and singleton payer allowance maps. Signed Transfer aliases
retain the existing Intent diagnosis; they cannot reach commit. State checks use
the stored intent so supplied intent substitutions retain their original ordering.
Both paths now require each work component to be UInt128 and their sum at most
`UINT128_MAX`; the owner allowance aggregate receives the same Stage bound.
These local failures precede premise comparison and return the complete decision
`{ accepted: false, judgment: Stage, code: "S0_STAGE_UNSUPPORTED", diagnosticWork: 1 }`.

Experiment observation: each path has ten isolated negative controls: an extra
balance, remaining-allowance key, spent-allowance key or obligation; negative or
above-bound remaining/spent work; work aggregate overflow; and owner allowance
aggregate overflow. Each binds a matching complete pre-state tuple, checks the
complete decision and all unchanged observable state, and confirms submission
is disabled. Two further controls combine malformed work with an unavailable
tuple to pin unsupported state before missing premise. Two positive controls
admit work `(1, UINT128_MAX - 1)` and commit it to `(0, UINT128_MAX)`.

The prior model failed all 22 new negative controls and passed both new boundary
controls; the exact commands, prior model hash and output are preserved in
[s1b-stage-guard-pre-repair-command-results.json](s1b-stage-guard-pre-repair-command-results.json).
The repaired model passes all 83 witnesses. Original and diagnostic receiving
fixtures now seed only balances, removing unrelated zero allowance rows.
The repayment ordering fixture removes unrelated account X; its stored payer
mismatch still rejects at Stage. The 59 existing witness names and intended first
judgments are retained. The prior Grok review covers its recorded candidate
bytes; this repair requires a fresh review of the new candidate.

External signatures, snapshots, successor extension, native qualification and
ledger atomicity remain stipulated. No exhaustive verification or executable
cross-layer projection was run. W-D0–W-D4 and Sprint 1 remain open.

```

## experiments/moriarty-language/formal/quint/mil4/README.md

```text
# Provisional MIL/4 S0 Quint model

`s0.qnt` models local Source/6 and Core/5 S0 transfer and funded
AccrualFirst repayment. It is a finite design experiment. It does not establish
a semantic freeze, native proof, K correspondence or ledger admission.

The model separates `sign` from `submitTransfer` and `submitRepay`. The stored
signed record fixes domain, asset, program, policy digest, signer, payer,
endpoints, obligation parties, exact amount and fee, nonce, predecessor,
validity, caps, conversion and terminal policy. Submitted intent and amounts
must match it. Repayment requires absent recipient and fee-recipient fields,
zero fee cap, zero net floor and identity conversion. Its creditor is the
party in the authenticated obligation. Transfer endpoints must be distinct;
a zero fee omits its credit line.

Every submission carries a typed `ComparisonRequest`. It names the submitted
successor, requested phase, retained effects and retained duties. Its
`StipulatedTuple` names an available premise, the exact stored intent, the
complete financial pre-state, round, expected successor and requested outcome. The financial
pre-state contains balances, allowance rows, work counters, obligations,
head and consumed replay keys. The request and tuple are supplied assumptions.
Comparing them does not authenticate their contents.

The observations follow `Stage → Intent → Effect → Authority → History →
Failure`. Stage checks required state cells and premise availability and
binding, including equality of the premise outcome and requested outcome.
Unavailable or mismatched premises return `S0_STAGE_PREMISE`;
a malformed required cell takes `S0_STAGE_UNSUPPORTED` first. Effect compares
the complete ordered financial vector against the submitted successor.
History then checks stale predecessor, consumed replay, successor mismatch
and self-successor in that order. The latter two return
`S0_HISTORY_SUCCESSOR`. Head IDs are abstract identities: the model uses no
arithmetic successor rule. A tuple can stipulate `0 → 9`; acceptance checks
identity and never establishes that token's external validity.

Failure accepts only terminal success with no retained effects or duties and
the stored terminal-only policy. Other typed outcomes return
`S0_FAILURE_UNSUPPORTED` after all earlier judgments pass. No committed
failure, pending episode, retained duty, grant, foreign evidence, fee reserve
or alias resolution is implemented.

`transferObservation` and `repayObservation` inspect decisions without
changing state. A rejected attempt returns one abstract diagnostic-work unit
and disables its submit action. It publishes no new state or effects and
consumes no financial authority, replay key or execution work. A successful
submit consumes one abstract work unit and stores the exact submitted vector,
including all money lines, allowance use, replay use, head advance and the
repayment obligation write. This cost is a provisional experiment schedule.

`signatureVerified`, `authenticatedSnapshots`, `nativeQualified` and
`ledgerAtomicReady` remain explicit external premises. The model establishes
none of their truth. Every current head must appear in the stipulated
`authenticatedSnapshots` set. A local commit never authenticates its new
head. Existing postcommit History fixtures explicitly stipulate both `0` and
`1`; a new witness stipulates only `0`, commits head `1`, and observes
`S0_STAGE_PREMISE` on its next submission.
`seedAccount`, `seedBalanceOnly` and `seedObligation` stipulate initial cells.
Quint integers are unbounded; guards separate UInt128 state cells from the
`2^127−1` nominal bound. Source formation and its earlier range rejection
are outside this model.

## Executed finite witnesses

Quint 0.32.0, TypeScript backend, seed `0x5`: all 59 witnesses passed.
The retained 35 comprise 15 original, 14 ordering regressions, 3 complete
common-state positives and 3 diagnostic witnesses. Original rejection
witnesses now pin their exact codes. The 24 S1B witnesses preserve the
original 18 and add outcome binding, a self-successor also stipulated as
expected, stale with replay, effect mismatch with replay, effect mismatch
with wrong successor and a postcommit unavailable snapshot premise. F1–F3
stipulate the submitted outcome and reach Failure; P2Outcome supplies an
unequal premise outcome and reaches Stage. S1B compares literal complete financial states and
complete decisions, including diagnostic work; its expected effects are
literal vectors. The H/F/P baseline is the common T-10-1 numeric fixture:
round 0, owner allowance 11/0, work 1/0, net floor 0 and balances 100/0/0.
P2PreState changes the premise owner balance to 99; P3 omits the owner cell.
This receipt records Quint execution only, not a cross-artifact comparison.

Every model and witness file typechecked. Exact commands, outputs and code
hashes are in `corpus/s1b-command-results.json`. The earlier receipts are
historical results for their recorded hashes; the previous S1B receipt is
preserved as `corpus/s1b-pre-audit-command-results.json`. No exhaustive simulation,
`quint verify`, native qualification or ledger execution was performed.
W-D0–W-D4 and Sprint 1 remain open.

```

## experiments/moriarty-language/formal/quint/mil4/corpus/s1b-stage-guard-command-results.json

```text
{
  "recordedAt": "2026-09-30T07:19:45.742301+00:00",
  "scope": "Postreview Stage guard repair; finite local witnesses only. External premises remain stipulated; no native qualification, ledger admission, exhaustive proof or Sprint 1 closure.",
  "candidateSha256": {
    "experiments/moriarty-language/formal/quint/mil4/s0.qnt": "9e7953a890325cd5d1294b19f9f00e9b5d8cc68aeffeb1dd693ce5a86467ec53",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s0_common_witnesses.qnt": "b5148618f46de24e205c5437e3fd695e5957b169f15fc83484b004a30069119e",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s0_diagnostic_witnesses.qnt": "badb83cb42fdf5e0e5801762602ffbe5e8301a9b0139242aece1829172974304",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s0_divergence_witnesses.qnt": "a46335f66e003af41f4200448a8790f85175a4b78a5942bbefa5240aecb6f885",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt": "007b66094455b21467ee27fb744b55706e5a91c7a745523253ecb82cdf297591",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s0_witnesses.qnt": "358aa6e5c3d7e19c5ca8750e03597fcf17d21d855b085a18f0c6c456807c7ce1"
  },
  "priorReceipt": "s1b-command-results.json",
  "reproductionReceipt": "s1b-stage-guard-pre-repair-command-results.json",
  "commands": [
    {
      "argv": [
        "quint",
        "--version"
      ],
      "exitCode": 0,
      "stdout": "0.32.0\n",
      "stderr": ""
    },
    {
      "argv": [
        "quint",
        "typecheck",
        "experiments/moriarty-language/formal/quint/mil4/s0.qnt"
      ],
      "exitCode": 0,
      "stdout": "",
      "stderr": ""
    },
    {
      "argv": [
        "quint",
        "typecheck",
        "experiments/moriarty-language/formal/quint/mil4/corpus/s0_common_witnesses.qnt"
      ],
      "exitCode": 0,
      "stdout": "",
      "stderr": ""
    },
    {
      "argv": [
        "quint",
        "typecheck",
        "experiments/moriarty-language/formal/quint/mil4/corpus/s0_diagnostic_witnesses.qnt"
      ],
      "exitCode": 0,
      "stdout": "",
      "stderr": ""
    },
    {
      "argv": [
        "quint",
        "typecheck",
        "experiments/moriarty-language/formal/quint/mil4/corpus/s0_divergence_witnesses.qnt"
      ],
      "exitCode": 0,
      "stdout": "",
      "stderr": ""
    },
    {
      "argv": [
        "quint",
        "typecheck",
        "experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt"
      ],
      "exitCode": 0,
      "stdout": "",
      "stderr": ""
    },
    {
      "argv": [
        "quint",
        "typecheck",
        "experiments/moriarty-language/formal/quint/mil4/corpus/s0_witnesses.qnt"
      ],
      "exitCode": 0,
      "stdout": "",
      "stderr": ""
    },
    {
      "argv": [
        "quint",
        "test",
        "experiments/moriarty-language/formal/quint/mil4/corpus/s0_common_witnesses.qnt",
        "--main",
        "s0_common_witnesses",
        "--match",
        ".*Test",
        "--seed",
        "0x5",
        "--backend",
        "typescript"
      ],
      "exitCode": 0,
      "stdout": "\n  s0_common_witnesses\n    ok transferCommonTest passed 1 test(s)\n    ok repayThirtyCommonTest passed 1 test(s)\n    ok repayNearBoundCommonTest passed 1 test(s)\n\n  3 passing (132ms)\n",
      "stderr": ""
    },
    {
      "argv": [
        "quint",
        "test",
        "experiments/moriarty-language/formal/quint/mil4/corpus/s0_diagnostic_witnesses.qnt",
        "--main",
        "s0_diagnostic_witnesses",
        "--match",
        ".*Test",
        "--seed",
        "0x5",
        "--backend",
        "typescript"
      ],
      "exitCode": 0,
      "stdout": "\n  s0_diagnostic_witnesses\n    ok stageDiagnosticTest passed 1 test(s)\n    ok effectDiagnosticTest passed 1 test(s)\n    ok historyDiagnosticTest passed 1 test(s)\n\n  3 passing (68ms)\n",
      "stderr": ""
    },
    {
      "argv": [
        "quint",
        "test",
        "experiments/moriarty-language/formal/quint/mil4/corpus/s0_divergence_witnesses.qnt",
        "--main",
        "s0_divergence_witnesses",
        "--match",
        ".*Test",
        "--seed",
        "0x5",
        "--backend",
        "typescript"
      ],
      "exitCode": 0,
      "stdout": "\n  s0_divergence_witnesses\n    ok overflowAndMissingFeeTest passed 1 test(s)\n    ok repaymentAliasTest passed 1 test(s)\n    ok debtorMismatchTest passed 1 test(s)\n    ok debtorSignerMismatchTest passed 1 test(s)\n    ok debtorPayerMismatchTest passed 1 test(s)\n    ok signedDebtorScopeTest passed 1 test(s)\n    ok signedCreditorScopeTest passed 1 test(s)\n    ok suppliedPayerScopeTest passed 1 test(s)\n    ok nonidentityConversionTest passed 1 test(s)\n    ok conversionScaleTest passed 1 test(s)\n    ok conversionRoundingTest passed 1 test(s)\n    ok invalidRoundAndAliasTest passed 1 test(s)\n    ok capAndAliasTest passed 1 test(s)\n    ok overpayAndMissingCreditorTest passed 1 test(s)\n\n  14 passing (212ms)\n",
      "stderr": ""
    },
    {
      "argv": [
        "quint",
        "test",
        "experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt",
        "--main",
        "s0_s1b_witnesses",
        "--match",
        ".*Test",
        "--seed",
        "0x5",
        "--backend",
        "typescript"
      ],
      "exitCode": 0,
      "stdout": "\n  s0_s1b_witnesses\n    ok H1Test passed 1 test(s)\n    ok H2Test passed 1 test(s)\n    ok H3Test passed 1 test(s)\n    ok H4Test passed 1 test(s)\n    ok H5Test passed 1 test(s)\n    ok H6Test passed 1 test(s)\n    ok F1Test passed 1 test(s)\n    ok F2Test passed 1 test(s)\n    ok F3Test passed 1 test(s)\n    ok P1Test passed 1 test(s)\n    ok P2IntentTest passed 1 test(s)\n    ok P2PreStateTest passed 1 test(s)\n    ok P2RoundTest passed 1 test(s)\n    ok P3Test passed 1 test(s)\n    ok nonArithmeticSuccessorTest passed 1 test(s)\n    ok failureAfterEffectTest passed 1 test(s)\n    ok repayRecipientAbsentTest passed 1 test(s)\n    ok repayFeeRecipientAbsentTest passed 1 test(s)\n    ok P2OutcomeTest passed 1 test(s)\n    ok H3SelfPremiseTest passed 1 test(s)\n    ok staleAndReplayTest passed 1 test(s)\n    ok effectAndReplayTest passed 1 test(s)\n    ok effectAndWrongSuccessorTest passed 1 test(s)\n    ok committedHeadPremiseUnavailableTest passed 1 test(s)\n    ok transferStageExtraBalanceTest passed 1 test(s)\n    ok transferStageExtraRemainingTest passed 1 test(s)\n    ok transferStageExtraSpentTest passed 1 test(s)\n    ok transferStageExtraObligationTest passed 1 test(s)\n    ok transferStageNegativeWorkRemainingTest passed 1 test(s)\n    ok transferStageNegativeWorkSpentTest passed 1 test(s)\n    ok transferStageOverBoundWorkRemainingTest passed 1 test(s)\n    ok transferStageOverBoundWorkSpentTest passed 1 test(s)\n    ok transferStageWorkAggregateOverflowTest passed 1 test(s)\n    ok transferStageAllowanceAggregateOverflowTest passed 1 test(s)\n    ok transferStageWorkBeforePremiseTest passed 1 test(s)\n    ok transferStageWorkBoundaryTest passed 1 test(s)\n    ok repayStageExtraBalanceTest passed 1 test(s)\n    ok repayStageExtraRemainingTest passed 1 test(s)\n    ok repayStageExtraSpentTest passed 1 test(s)\n    ok repayStageExtraObligationTest passed 1 test(s)\n    ok repayStageNegativeWorkRemainingTest passed 1 test(s)\n    ok repayStageNegativeWorkSpentTest passed 1 test(s)\n    ok repayStageOverBoundWorkRemainingTest passed 1 test(s)\n    ok repayStageOverBoundWorkSpentTest passed 1 test(s)\n    ok repayStageWorkAggregateOverflowTest passed 1 test(s)\n    ok repayStageAllowanceAggregateOverflowTest passed 1 test(s)\n    ok repayStageWorkBeforePremiseTest passed 1 test(s)\n    ok repayStageWorkBoundaryTest passed 1 test(s)\n\n  48 passing (142ms)\n",
      "stderr": ""
    },
    {
      "argv": [
        "quint",
        "test",
        "experiments/moriarty-language/formal/quint/mil4/corpus/s0_witnesses.qnt",
        "--main",
        "s0_witnesses",
        "--match",
        ".*Test",
        "--seed",
        "0x5",
        "--backend",
        "typescript"
      ],
      "exitCode": 0,
      "stdout": "\n  s0_witnesses\n    ok transferTenOneTest passed 1 test(s)\n    ok repayThirtyTest passed 1 test(s)\n    ok repayNearBoundTest passed 1 test(s)\n    ok recipientSubstitutionTest passed 1 test(s)\n    ok feeCapTest passed 1 test(s)\n    ok grossCapTest passed 1 test(s)\n    ok missingFeeTest passed 1 test(s)\n    ok missingCreditorTest passed 1 test(s)\n    ok wrongCreditorTest passed 1 test(s)\n    ok allowanceTest passed 1 test(s)\n    ok staleHeadTest passed 1 test(s)\n    ok replayTest passed 1 test(s)\n    ok overflowTest passed 1 test(s)\n    ok overpayTest passed 1 test(s)\n    ok missingFeeAndStaleTest passed 1 test(s)\n\n  15 passing (116ms)\n",
      "stderr": ""
    }
  ]
}

```

## experiments/moriarty-language/formal/quint/mil4/corpus/s1b-stage-guard-pre-repair-command-results.json

```text
{
  "recordedAt": "2026-09-30T07:18:38.515775+00:00",
  "scope": "Pre-repair reproduction of Stage footprint and work-range guards; premises remain stipulated.",
  "candidateSha256": {
    "experiments/moriarty-language/formal/quint/mil4/s0.qnt": "4dd7be0ab5f69afff89fce5108e124f843338ddd9b31b891ea26fdc492dd576c",
    "experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt": "007b66094455b21467ee27fb744b55706e5a91c7a745523253ecb82cdf297591"
  },
  "commands": [
    {
      "argv": [
        "quint",
        "typecheck",
        "experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt"
      ],
      "exitCode": 0,
      "stdout": "",
      "stderr": ""
    },
    {
      "argv": [
        "quint",
        "test",
        "experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt",
        "--main",
        "s0_s1b_witnesses",
        "--match",
        ".*Stage.*Test",
        "--seed",
        "0x5",
        "--backend",
        "typescript"
      ],
      "exitCode": 1,
      "stdout": "\n  s0_s1b_witnesses\n    1) transferStageExtraBalanceTest failed after 1 test(s)\n    2) transferStageExtraRemainingTest failed after 1 test(s)\n    3) transferStageExtraSpentTest failed after 1 test(s)\n    4) transferStageExtraObligationTest failed after 1 test(s)\n    5) transferStageNegativeWorkRemainingTest failed after 1 test(s)\n    6) transferStageNegativeWorkSpentTest failed after 1 test(s)\n    7) transferStageOverBoundWorkRemainingTest failed after 1 test(s)\n    8) transferStageOverBoundWorkSpentTest failed after 1 test(s)\n    9) transferStageWorkAggregateOverflowTest failed after 1 test(s)\n    10) transferStageAllowanceAggregateOverflowTest failed after 1 test(s)\n    11) transferStageWorkBeforePremiseTest failed after 1 test(s)\n    ok transferStageWorkBoundaryTest passed 1 test(s)\n    12) repayStageExtraBalanceTest failed after 1 test(s)\n    13) repayStageExtraRemainingTest failed after 1 test(s)\n    14) repayStageExtraSpentTest failed after 1 test(s)\n    15) repayStageExtraObligationTest failed after 1 test(s)\n    16) repayStageNegativeWorkRemainingTest failed after 1 test(s)\n    17) repayStageNegativeWorkSpentTest failed after 1 test(s)\n    18) repayStageOverBoundWorkRemainingTest failed after 1 test(s)\n    19) repayStageOverBoundWorkSpentTest failed after 1 test(s)\n    20) repayStageWorkAggregateOverflowTest failed after 1 test(s)\n    21) repayStageAllowanceAggregateOverflowTest failed after 1 test(s)\n    22) repayStageWorkBeforePremiseTest failed after 1 test(s)\n    ok repayStageWorkBoundaryTest passed 1 test(s)\n\n  2 passing (96ms)\n  22 failed\n\n  1) transferStageExtraBalanceTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:357:39\n        357:   run transferStageExtraBalanceTest = ready(t)\n                                                   ^^^^^^^^\n        358:     .then(replacePre(transferExtraBalancePre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        359:     .expect(rejected(t, transferExtraBalancePre, stageRequest(t, transferExtraBalancePre), expected, Stage, \"S0_STAGE_UNSUPPORTED\"))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=transferStageExtraBalanceTest to repeat.\n  2) transferStageExtraRemainingTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:363:41\n        363:   run transferStageExtraRemainingTest = ready(t)\n                                                     ^^^^^^^^\n        364:     .then(replacePre(transferExtraRemainingPre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        365:     .expect(rejected(t, transferExtraRemainingPre, stageRequest(t, transferExtraRemainingPre), expected, Stage, \"S0_STAGE_UNSUPPORTED\"))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=transferStageExtraRemainingTest to repeat.\n  3) transferStageExtraSpentTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:369:37\n        369:   run transferStageExtraSpentTest = ready(t)\n                                                 ^^^^^^^^\n        370:     .then(replacePre(transferExtraSpentPre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        371:     .expect(rejected(t, transferExtraSpentPre, stageRequest(t, transferExtraSpentPre), expected, Stage, \"S0_STAGE_UNSUPPORTED\"))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=transferStageExtraSpentTest to repeat.\n  4) transferStageExtraObligationTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:375:42\n        375:   run transferStageExtraObligationTest = ready(t)\n                                                      ^^^^^^^^\n        376:     .then(replacePre(transferExtraObligationPre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        377:     .expect(rejected(t, transferExtraObligationPre, stageRequest(t, transferExtraObligationPre), expected, Stage, \"S0_STAGE_UNSUPPORTED\"))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=transferStageExtraObligationTest to repeat.\n  5) transferStageNegativeWorkRemainingTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:381:48\n        381:   run transferStageNegativeWorkRemainingTest = ready(t)\n                                                            ^^^^^^^^\n        382:     .then(replacePre(transferNegativeWorkRemainingPre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        383:     .expect(rejected(t, transferNegativeWorkRemainingPre, stageRequest(t, transferNegativeWorkRemainingPre), expected, Stage, \"S0_STAGE_UNSUPPORTED\"))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=transferStageNegativeWorkRemainingTest to repeat.\n  6) transferStageNegativeWorkSpentTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:387:44\n        387:   run transferStageNegativeWorkSpentTest = ready(t)\n                                                        ^^^^^^^^\n        388:     .then(replacePre(transferNegativeWorkSpentPre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        389:     .expect(rejected(t, transferNegativeWorkSpentPre, stageRequest(t, transferNegativeWorkSpentPre), expected, Stage, \"S0_STAGE_UNSUPPORTED\"))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=transferStageNegativeWorkSpentTest to repeat.\n  7) transferStageOverBoundWorkRemainingTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:393:49\n        393:   run transferStageOverBoundWorkRemainingTest = ready(t)\n                                                             ^^^^^^^^\n        394:     .then(replacePre(transferOverBoundWorkRemainingPre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        395:     .expect(rejected(t, transferOverBoundWorkRemainingPre, stageRequest(t, transferOverBoundWorkRemainingPre), expected, Stage, \"S0_STAGE_UNSUPPORTED\"))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=transferStageOverBoundWorkRemainingTest to repeat.\n  8) transferStageOverBoundWorkSpentTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:399:45\n        399:   run transferStageOverBoundWorkSpentTest = ready(t)\n                                                         ^^^^^^^^\n        400:     .then(replacePre(transferOverBoundWorkSpentPre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        401:     .expect(rejected(t, transferOverBoundWorkSpentPre, stageRequest(t, transferOverBoundWorkSpentPre), expected, Stage, \"S0_STAGE_UNSUPPORTED\"))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=transferStageOverBoundWorkSpentTest to repeat.\n  9) transferStageWorkAggregateOverflowTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:405:48\n        405:   run transferStageWorkAggregateOverflowTest = ready(t)\n                                                            ^^^^^^^^\n        406:     .then(replacePre(transferWorkAggregateOverflowPre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        407:     .expect(rejected(t, transferWorkAggregateOverflowPre, stageRequest(t, transferWorkAggregateOverflowPre), expected, Stage, \"S0_STAGE_UNSUPPORTED\"))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=transferStageWorkAggregateOverflowTest to repeat.\n  10) transferStageAllowanceAggregateOverflowTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:411:53\n        411:   run transferStageAllowanceAggregateOverflowTest = ready(t)\n                                                                 ^^^^^^^^\n        412:     .then(replacePre(transferAllowanceAggregateOverflowPre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        413:     .expect(rejected(t, transferAllowanceAggregateOverflowPre, stageRequest(t, transferAllowanceAggregateOverflowPre), expected, Stage, \"S0_STAGE_UNSUPPORTED\"))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=transferStageAllowanceAggregateOverflowTest to repeat.\n  11) transferStageWorkBeforePremiseTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:416:44\n        416:   run transferStageWorkBeforePremiseTest = ready(t)\n                                                        ^^^^^^^^\n        417:     .then(replacePre(transferNegativeWorkSpentPre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        418:     .expect(rejected(t, transferNegativeWorkSpentPre, { ...stageRequest(t, transferNegativeWorkSpentPre), premise: { ...tuple, available: false } }, expected, Stage, \"S0_STAGE_UNSUPPORTED\"))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=transferStageWorkBeforePremiseTest to repeat.\n  12) repayStageExtraBalanceTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:429:36\n        429:   run repayStageExtraBalanceTest = repayReady(r)\n                                                ^^^^^^^^^^^^^\n        430:     .then(replacePre(repayExtraBalancePre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        431:     .expect(repayStageRejected(repayExtraBalancePre, stageRequest(r, repayExtraBalancePre)))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=repayStageExtraBalanceTest to repeat.\n  13) repayStageExtraRemainingTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:435:38\n        435:   run repayStageExtraRemainingTest = repayReady(r)\n                                                  ^^^^^^^^^^^^^\n        436:     .then(replacePre(repayExtraRemainingPre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        437:     .expect(repayStageRejected(repayExtraRemainingPre, stageRequest(r, repayExtraRemainingPre)))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=repayStageExtraRemainingTest to repeat.\n  14) repayStageExtraSpentTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:441:34\n        441:   run repayStageExtraSpentTest = repayReady(r)\n                                              ^^^^^^^^^^^^^\n        442:     .then(replacePre(repayExtraSpentPre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        443:     .expect(repayStageRejected(repayExtraSpentPre, stageRequest(r, repayExtraSpentPre)))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=repayStageExtraSpentTest to repeat.\n  15) repayStageExtraObligationTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:447:39\n        447:   run repayStageExtraObligationTest = repayReady(r)\n                                                   ^^^^^^^^^^^^^\n        448:     .then(replacePre(repayExtraObligationPre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        449:     .expect(repayStageRejected(repayExtraObligationPre, stageRequest(r, repayExtraObligationPre)))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=repayStageExtraObligationTest to repeat.\n  16) repayStageNegativeWorkRemainingTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:453:45\n        453:   run repayStageNegativeWorkRemainingTest = repayReady(r)\n                                                         ^^^^^^^^^^^^^\n        454:     .then(replacePre(repayNegativeWorkRemainingPre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        455:     .expect(repayStageRejected(repayNegativeWorkRemainingPre, stageRequest(r, repayNegativeWorkRemainingPre)))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=repayStageNegativeWorkRemainingTest to repeat.\n  17) repayStageNegativeWorkSpentTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:459:41\n        459:   run repayStageNegativeWorkSpentTest = repayReady(r)\n                                                     ^^^^^^^^^^^^^\n        460:     .then(replacePre(repayNegativeWorkSpentPre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        461:     .expect(repayStageRejected(repayNegativeWorkSpentPre, stageRequest(r, repayNegativeWorkSpentPre)))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=repayStageNegativeWorkSpentTest to repeat.\n  18) repayStageOverBoundWorkRemainingTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:465:46\n        465:   run repayStageOverBoundWorkRemainingTest = repayReady(r)\n                                                          ^^^^^^^^^^^^^\n        466:     .then(replacePre(repayOverBoundWorkRemainingPre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        467:     .expect(repayStageRejected(repayOverBoundWorkRemainingPre, stageRequest(r, repayOverBoundWorkRemainingPre)))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=repayStageOverBoundWorkRemainingTest to repeat.\n  19) repayStageOverBoundWorkSpentTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:471:42\n        471:   run repayStageOverBoundWorkSpentTest = repayReady(r)\n                                                      ^^^^^^^^^^^^^\n        472:     .then(replacePre(repayOverBoundWorkSpentPre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        473:     .expect(repayStageRejected(repayOverBoundWorkSpentPre, stageRequest(r, repayOverBoundWorkSpentPre)))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=repayStageOverBoundWorkSpentTest to repeat.\n  20) repayStageWorkAggregateOverflowTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:477:45\n        477:   run repayStageWorkAggregateOverflowTest = repayReady(r)\n                                                         ^^^^^^^^^^^^^\n        478:     .then(replacePre(repayWorkAggregateOverflowPre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        479:     .expect(repayStageRejected(repayWorkAggregateOverflowPre, stageRequest(r, repayWorkAggregateOverflowPre)))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=repayStageWorkAggregateOverflowTest to repeat.\n  21) repayStageAllowanceAggregateOverflowTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:483:50\n        483:   run repayStageAllowanceAggregateOverflowTest = repayReady(r)\n                                                              ^^^^^^^^^^^^^\n        484:     .then(replacePre(repayAllowanceAggregateOverflowPre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        485:     .expect(repayStageRejected(repayAllowanceAggregateOverflowPre, stageRequest(r, repayAllowanceAggregateOverflowPre)))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=repayStageAllowanceAggregateOverflowTest to repeat.\n  22) repayStageWorkBeforePremiseTest:\n       Error [QNT508]: Expect condition does not hold true\n        at /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt:488:41\n        488:   run repayStageWorkBeforePremiseTest = repayReady(r)\n                                                     ^^^^^^^^^^^^^\n        489:     .then(replacePre(repayNegativeWorkSpentPre))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n        490:     .expect(repayStageRejected(repayNegativeWorkSpentPre, { ...stageRequest(r, repayNegativeWorkSpentPre), premise: { ...tuple, available: false } }))\n             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n    Use --seed=0x5 --match=repayStageWorkBeforePremiseTest to repeat.\n\n\n  Use --verbosity=3 to show executions.\n  Further debug with: quint test --verbosity=3 experiments/moriarty-language/formal/quint/mil4/corpus/s0_s1b_witnesses.qnt\n",
      "stderr": "error: Tests failed\n"
    }
  ]
}

```

## deliverables/mil4-k-quint-sprint1-2026-09-29/audits/s1b-stage-guard-ts-verification.json

```text
{
  "status": "pass",
  "typecheckExitCode": 0,
  "targetedTests": {
    "pass": 28,
    "fail": 0
  },
  "packageTests": {
    "pass": 938,
    "fail": 0
  },
  "files": {
    "experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts": {
      "bytes": 17834,
      "sha256": "e0b6c203eb29045ef6e4357033f6cb519fc1783a9c8e1fc3087227c7795d6aa3"
    },
    "experiments/moriarty-language/tests/mil4-s0-source-v6.test.mjs": {
      "bytes": 25947,
      "sha256": "6cdce3bf80bed4f2ce6b32596169e98fe7ee7cdad618406506fdeab17b687971"
    },
    "deliverables/mil4-k-quint-sprint1-2026-09-29/audits/s1b-stage-guard-npm-test.log": {
      "bytes": 77970,
      "sha256": "d8615aa557ad5d04e33fc4e7ef9643e08b83d8254119e9009d38c9ebc3291daf"
    }
  },
  "scope": "local unqualified TypeScript comparison; no native or ledger qualification"
}

```

