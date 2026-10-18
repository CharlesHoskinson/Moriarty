# Generated spending artifact boundary

Source-static disposition at Moriarty main `9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb`.

**Result:** the existing T1 approved-funding branch already passes its applied generated validator, continuation datum, and action redeemer into the existing transaction builder. The production consumed-financial branch explicitly stops before construction because its policy/script/budget/minimum-ADA bindings are incomplete. No ordinary export-only patch was identified that completes generated-validator plus strand/public-claim consumption without choosing new bindings or changing that boundary. **No candidate source patch is proposed.**

This is a bounded repository observation and proposed owner handoff, not compiler compatibility, validator correctness, security acceptance, executed correspondence, or ledger evidence. No source, requirement, pinned I/J candidate, or W source was changed. Only this output directory was written.

## Source and requirement pins

Repository: `https://github.com/CharlesHoskinson/Moriarty.git`, default branch `main`, inspected commit `9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb`. The startup checkout `/home/charl/Moriarty/.worktrees/utxo` was at `5f9138640529b09dea5611ce41a5eabaac50c68a`; main sources were read with `git show`, without switching it. `SOURCE-PINS.json` records exact Git blobs, byte sizes and SHA-256 values. `source-main/` contains complete copied files for review, although the inspection below is bounded to the cited paths and spans.

Original requirements remain unchanged:

- `openspec/changes/eutxo-compiler-correspondence/specs/eutxo-compiler/spec.md:124–133`: EUC-012 requires isolated generated Aiken, checked blueprint, UPLC, Cardano credential, source digest, dependency manifest, failure isolation and stale-artifact refusal.
- Same file `135–143`: EUC-013 requires actual generated spending execution on reconstructed context and source envelopes, comparison of complete ordinary post, admitted financial post, ordered effects and remaining work, and preservation of source diagnostics on refusal. Its positive includes Lender 107, PriorSpend plus Release1, and revision 5. Its negative excludes helper-only evidence.
- Same file `145–153`: EUC-014 distinguishes actual decoding and generated-validator execution from representation round trips and other evidence layers.
- `openspec/changes/eutxo-compiler-correspondence/design.md:17–23,37–45,90–101`: proposed general compiler interfaces and manifest; current closed revision03 ABI; unresolved exposure of complete effects from generated spending tests. Proposed interfaces are not callable implementations.

`STARTUP-STATUS.json` is the required read-only plugin status, exit 0. It reports historical SP01 dependent dispatch blocked by stale inputs/missing accounting/live state. It supplies no execution authority and does not select this task.

## Actual consumers and bytes

Paths in this section are relative to `source-main/` and pinned to the commit above. They describe the experimental public Cardano path, not an assertion that the main Rust `mori compile` route emits Aiken.

| Boundary | Actual producer and consumer | Repository observation |
|---|---|---|
| Public generated-custody command | `experiments/moriarty-eutxo/src/cli.mjs:299–331` → `compileCustody` | Accepts source, seed, manifest, designer cases and expected results; stdout exposes hashes, instance, blueprint and generated validator paths. |
| Source/profile | `admit-custody.mjs:11–15,128–181`; `compile-custody.mjs:202–321` | T1 Source/5, contract/4, lifecycle/1; admission, evaluator fixtures, capacity and oracle binding precede generation. General arbitrary ordinary fields and nonempty duties are not admitted by this T1 route. |
| Generated entrypoint | `lower-custody.mjs:258–272,1279–1281,1297–1330` | Generated `financial_head.spend` takes parameters seed/cash/lender/borrower/genesis, then `Option<HeadDatum>`, `Redeemer`, own reference and transaction. `spend_unit` invokes that generated entrypoint. |
| Compile/publication | `compile-custody.mjs:322–350,353–415` | Writes `custody.ak`, runs Aiken check/build in an isolated project, copies `plutus.json`, calls `applyUnitParameters`, returns applied report/blueprint/validator paths. No command was executed here. |
| Unit applied artifacts | `apply-head.mjs:87–217` | Finds mint/spend/else purpose entries, applies five parameters, and can persist `plutus.applied.json`, `cash.script.json`, `apply-report.json`. This report labels cash fail-closed and native pins false. It does not itself publish the approved-funding head script envelope. |
| Approved-funding head script | `native-caller.mjs:1763–1819` → `applyHeadToCashPolicy`; `apply-head.mjs:346–418` | Uses the source blueprint plus compiled cash policy, applies five head parameters, writes existing `head.applied.script.json` as `{type:'PlutusScriptV3', description, cborHex}`, returns `scriptPath`, applied hash and instance. This is the concrete generated-script producer used by the next row. |
| Approved-funding transaction | `native-caller.mjs:1858–1909,1937–1976` → `encodeUnsignedBody` / `encodeSpendBody` | Opening body receives `headApply.scriptPath`; spending body receives the same path, `JSON.stringify(redeemerForSpend(...))`, continuing `datumFromPost(...).plutus`, head/cash asset rows and interval. This forwarding already exists. `runNativeCaller:2943–2958` consumes the returned funding/spend stages. |
| Existing builder | `native-body.mjs:545–559,598–610,622–660` | `encodeSpendBody(spec)` accepts an existing V3 script envelope at `spec.spendScriptPath`, string `redeemerJson`, two output records with continuing inline datum, input references, signer/payout information, interval, fees and budget/protocol inputs. It passes the script/redeemer/datum file paths into `cardano-cli conway transaction build-raw`. This is a process-spawning boundary, not an ordinary callable check to run in this task. |
| Separate consumed-financial transaction | `native-caller.mjs:2408–2444,2634–2697` | Takes `request.detailedData` (13-field constructor), `request.spendScriptPath`, source-derived post/redeemer and unit head/cash policy constants. It does not select the approved-funding `headApply.scriptPath` producer. |
| Explicit production stop | `native-caller.mjs:2891–2943` | Production sibling source identities are copied into `stages.source`, but `productionConsumerIncomplete` produces `PRODUCTION_FINANCIAL_CONSUMER_INCOMPLETE`, reason `current-policy-script-budget-minAda-binding-required`, and prevents the body branch. Adding a path or field cannot by itself satisfy those dependencies. |

Two source limits qualify the connected approved-funding branch. First, `apply-head.mjs:355–356` requires the embedded cash hash to equal `FAIL_CLOSED_CASH` and the unapplied head hash to equal the fixed `UNAPPLIED_HEAD`; connection of this fixture does not establish arbitrary generated-source admission. Second, `native-caller.mjs:1848–1849` derives `holding` from the cash mint transaction, and `1908–1910` parses that same `holding` as `openedRef` before passing it as `head` to the spending builder. The exact reference flow is reported for the security owner; this agent does not decide the correctness of the consumed head, replace it, or design tests for it.

The main Rust CLI is a different route: `crates/mori/src/cli.rs:114–125` describes ledger-9 ZKIRv3 compilation; `crates/mori/src/commands/compile.rs:21–69,74–113` publishes ZKIR circuits and `artifact.json` (plus source projection for the ordinary route). These are not the generated Aiken blueprint/validator or a Cardano spending adapter. No Rust/native compiler was invoked.

## Existing state carrier and public-claim boundary

The T1 persistent state carrier is the **existing** 13-field `HeadDatum`: program/bounds/source/schema/seed hashes; network/deployment; instance; transport revision/remaining; ordinary locked/revision; lifecycle. `native-financial.mjs:190–288` serializes lifecycle balances, allowances, obligations, used IDs and work into that existing positional datum. `redeemerFromArgs:291–309` serializes Release/Refund/Cancel with actor, recipient, transfer ID and lower/upper bounds. `loadSourceTerminals:368–426` evaluates the fixed designer source cases into terminal envelopes; it is not a reader of arbitrary previously exported source-envelope/strand claims.

This observation does **not** equate HeadDatum with the full product's strand semantics or claim that the T1 fields authenticate a general public claim. There is no field in this closed T1 ABI to which this agent can simply forward the fixed-C2 proof claim. The absence of an identifier named `strand` is not treated as a defect.

The fixed-C2 boundary is different: `experiments/cardano-four-piece-settlement/validators/c2.ak:9–26` accepts `C2Mint` Issue/Burn and decodes its spending datum as `EnvV2`. `lib/pvc2/types.ak:113–128,228–243` declares `ThreadDatumV2`/`EnvV2` and the Issue VK/proof fields. The T1 generated spender decodes `HeadDatum` and `Redeemer`. No conversion between these interfaces was located in the bounded inspected consumer paths. No conclusion about cryptographic compatibility follows from this structural observation. The host README expressly retains separate Rust `/1` profiles, unauthenticated supplied context and missing consumer/compiled/ledger correspondence.

**Exact owner boundary:** a generated-spender consumer of strand/public-claim data needs an authoritative, versioned mapping between its selected source envelope, instance/state/history carrier, generated entrypoint ABI, claim relation/verifier identity and authenticated transaction context. Defining or evaluating that mapping, changing validator/relation semantics, judging any current identity/reference derivation, and designing security assertions or hostile tests belong to Opus (or the explicitly authorized fallback). This proposal does not select such a mapping, append a new payload, treat a fixed C2 verifier as generated-script compatibility, or change the production stop.

## Smallest ordinary export candidate considered

`applyUnitParameters` could mechanically persist the already selected applied spend bytes using the existing V3 script-envelope shape and return a path; the compile-custody CLI could display that path. That would only expose the **unit/fail-closed-cash** applied artifact. It would not connect the approved cash-policy identities, consumed head, production interval, budget, minimum-ADA or public claim. The approved-funding branch already writes and consumes its own corresponding script file. Therefore such a patch would duplicate an artifact export without resolving the requested consumer boundary. It is not proposed as a product increment here.

Likewise, substituting `stages.sibling.applied` or `headApply` into `assembleConsumedFinancial`, removing `productionConsumerIncomplete`, replacing UNIT policies, or changing the positional HeadDatum/EnvV2 layout would choose substantive bindings. No full before/after or patch was authored for those changes. `DISPOSITION.json` records `candidatePatch: null` and the condition that failed: normative production/claim mapping was not established as ordinary plumbing in the inspected source.

## Falsifiable predicates and first blocked dependencies

**Source task predicate (inspected):** at the pinned source, follow the approved-funding generated head script to the exact spending builder and its existing datum/redeemer arguments. The predicate is falsified if any cited call is missing or instead consumes an unrelated fixed-C2 script. The complete source snapshots permit direct checking. The cited calls are present; this is source connectivity only.

**Requested usable-consumer predicate (UNRUN / not established):** a named admitted generated source and complete independently derived source envelope traverse the existing public path; the built body uses that source's exact applied generated spender and existing correctly bound state/action representation; actual generated spending consumes reconstructed context and agrees on complete ordinary post, admitted financial post, ordered effects and work remaining. This is the original EUC-013 predicate, not an artifact-only substitute. Execution and security judgments remain owner work.

**First concrete publication gap:** EUC-012 remains upstream. `compile-custody.mjs:344–350` copies the build blueprint and applies parameters under `if (compiled)` without `checkOk`; `blueprintPath` at `401` is returned when `compiled`, although overall `ok` requires `checkOk`. A failing check plus successful build is therefore a source-visible route to a non-null published blueprint. This is control-flow inspection, not a reproduced run. It conflicts with the stated failure-isolation requirement and the CUSTODY.md description. The separate publication owner should resolve that exact existing behavior; this task does not duplicate their patch.

**First production consumer dependency:** after publication is correctly established, the existing real-CLI production sibling route stops at `native-caller.mjs:2915–2923`. Its named missing current policy/script/budget/minimum-ADA bindings and the absent adopted public-claim/state mapping must be supplied through the applicable owner before this agent can prepare an ordinary consumer patch. Whether any present identity, consumed reference, datum, proof or verifier is correct is left explicitly undecided.

## Verification and handoff state

Executed only source listing/search/reading, Git read operations, file hashing/copying into this directory, and the mandated read-only plugin status. No tests, compiler, SDK/Cargo, Aiken, VM/UPLC, proving/verifying, setup/key generation, wallet, network, node, ledger, deploy, Git mutation, review dispatch or new orchestration was run. Earlier historical evidence was not relabelled as a fresh result.

The artifact manifest check in `ARTIFACT-VERIFICATION.json` verifies only that these snapshots still equal their pinned Git blobs and that the machine-readable disposition is valid JSON. It does not execute product code. Source status: inspected. Candidate implementation: none. Runtime evidence: unrun. EUC-012/013/014 and parent product acceptance: open.

The checkpoint skill was read for the handoff, but its Foreman session runtime was not used: checkout AGENTS.md explicitly prohibits Foreman and its session runtime for current DoctorWatson work. This report and the typed disposition contain the bounded stopping point without starting another mechanism.
