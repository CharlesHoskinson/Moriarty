# Designer 02: escrow, time, oracle and roles

Vote **B: source-derived bounded backend, real Aiken/UPLC validation, and an isolated Cardano local ledger**. This is a recommendation, not backend acceptance. A is useful for an initial discriminator but cannot establish consumed state identity, ledger interval, authenticated references, actual signers, custody payout or fee/collateral behavior. C adds private proof/certificate and availability work before the public transaction relation is measured; retain those obligations as an explicit later tier. B must compile source/Core, preserve complete effects and reject unsupported primitives. A generic adapter with per-pattern switches or builder assertions would not satisfy B.

## Scope and executed evidence

Four representative patterns, 26 source cases and seven native limitation cases. `cases.json` contains exact canonical ordinary inputs, financial input filenames, independently derived financial poststates and effects for intended positives, and independently derived negatives. `observations.json` contains read-only results from the actual versioned source factories, with full output. **22 cases match the intended source outcomes; four additional cases reproduce documented profile limitations**. No UPLC compilation, ledger submission, privacy proof or private handoff was performed. `observations-initial.json` preserves the initial attempts, including a reserved intrinsic name subsequently corrected from `price` to `observedPrice`.

The independent economic oracle is: locked Cash100 belongs to owner Borrower while held in custody Escrow; release sends100 to beneficiary Lender; refund/cancel sends100 back to Borrower. One custody transfer exhausts Escrow's100 allowance, records its transfer ID, increments revision0→1, and changes locked1→0. The cancellation variant additionally retains a separate debt40 from Borrower to Lender, without Repay. That debt is not waived by a refund or closed cash lock. All unrelated fields survive. Constructor work is derived by counting each actually traversed Core constructor plus one per Transfer kernel operation, rather than copied from evaluation. There are no conditional/short-circuit expressions in the positive fixtures.

## Actual inspected APIs (profiles 0–6)

Here “0” denotes existing atomic source, not a nonexistent successor `/0` factory. Paths below are relative to `experiments/moriarty-language` in the isolated checkout.

| Profile | Inspected entry and representability |
|---|---|
| Atomic `moriarty-bounded-atomic/1` | `src/frontend.ts` exposes parse/check/elaborate/compile; `src/evaluate.ts:320` exposes createSimulator(source,bounds), makeGenesis, initialState, simulate. It has observation/principal bindings and a fail-closed accepted evaluate function at line335. Client checks are discarded by accepted evaluation; a trusted deployment backend is absent. Existing loan/swap samples have source guards, typed effects, remaining work and obligation/status handling. |
| successor `/1` | `createFinancialAgreementSourceV1().evaluate(source,ordinaryJSON,financialJSON)`; one action; financial Core `/1`. |
| `/2` | same factory pattern with actionName; multiple actions; Core `/1`. |
| `/3` | actionName plus same primitive-string JSON inputs; kernel-backed financial reads; Core `/2`. |
| `/4` | financial postcondition reads; Core `/3`. |
| `/5` | `createFinancialAgreementSourceV5().evaluate(source,actionName,ordinaryCanonicalJSON,financialPreStateJSON)`; Originate/Accrue/Repay/Transfer, lifecycle state `/1`; Core `/4`; third funded custody party is usable. |
| `/6` | same evaluate shape; lifecycle state `/2`, Fee, two-party authority/outcome caps; Core `/5`. Authority commitments and party labels bind data but do not verify consent/signatures. |

The agreement compiler sets `observations: {}` (`src/successor/financial-agreement-source-compiler.ts:612`), and the actual `/5` grammar contains no observation declaration. Time, feed and actor fields in these successor fixtures are **untrusted Args**, deliberately not host truth flags. Arithmetic guards check the supplied data only. A backend must extract/authenticate actual transaction fields and bind them to those Args before attributing native significance to a source result.

## Pattern 1: escrow beneficiary release, owner timeout refund and cancellation

`escrow-custody-v5.mori` supplies actual expressible release/refund/cancel on three financial parties. Cases E11/E12/E13 demonstrate source-funded refund, cancellation with residual debt40, and beneficiary release. Existing `/5` suffices; no language syntax is required. E01–E08 additionally exercise recipient/actor and interval guards under `/6`.

Retained counterexamples: E06/E08 intended `/6` owner self-transfers reject `SELF_TRANSFER` (`financial-lifecycle-v2.ts:1457`); E09/E10 no-effect unlocks reject `EMPTY_BATCH`. These are expected capability failures, not successful refund coverage. A self Fee is not a substitute for Transfer. For an actual `/6` custody mapping, a third-party debit would violate its two-role cap schema (`financial-outcome-intent.ts:43–50`); this is distinct from nominal balances or transaction signer checks.

Native predicate: spend exactly the authenticated escrow head carrying one instance token and100 Cash; the selected source action determines exact owner/beneficiary payment output with pinned address credentials and Cash policy/name. If debt40 remains, retain exactly one authenticated continuation with obligations, allowance/work/history, revision1 and locked0. Closing the escrow lock cannot delete that state. If all agreement duties are absent, terminal token disposition must be declared separately; do not infer termination from locked0. Repeated heads, missing/duplicate successors, redirected payout and replay reject. Native fee/minADA/change sponsorship is an explicitly authorized separate funding component; do not count fees as business payout or mask gross inputs with change.

## Pattern 2: finite validity interval and timeout boundary

`interval-boundary.mori` has `lower <= upper`, `lower >=1000`, `upper <1100`. The abstract integer interval is inclusive. `[1099,1099]` succeeds; `[1100,1100]`, reversed `[1050,1049]`, or wide `[1000,1200]` rejects. Refund starts at `[1100,1100]` and is bounded to upper≤1200. A range straddling1100 cannot select release or refund merely by choosing a convenient timestamp.

Native predicate: interval Args must equal the pinned ledger context's finite bounds after an explicit slot-to-time conversion. Cardano slot upper bounds are exclusive; convert to the last valid slot/time before using this inclusive fixture convention. Publish network genesis/system-start/era parameters and exact endpoint openness; there is no host wall-clock check. Empty/unbounded ranges reject. A forged narrow Args window paired with an actual wide ledger range must reject (N01), even though source-only evaluation accepts the Args.

## Pattern 3: authenticated reference observation and whole-window freshness

`oracle-observation.mori` requires feed USD_Cash, publisher ApprovedFeed, price≥10, publish time t≤lower, and upper≤t+20. `[1010,1020]` with t1000 is accepted at the freshness boundary. upper1021, future t1011, wrong feed/publisher and price9 reject. Overflow of t+20 must reject by checked arithmetic rather than wrap.

Native predicate: resolve the actual reference input; check the deployment/script credential, unique oracle policy/token, datum codec, feed/channel and publisher authorization or pinned verification withdrawal. Extract timestamp and observed price from that datum and bind them to source Args. Check the whole actual validity range. A correctly authenticated feed is a statement from that publisher, not market truth. The reference remains unconsumed and its value cannot fund Cash payout. The escrow/state head must be a spending input (N02); reference-only state is not a transition. Attacker data relabeled ApprovedFeed must reject (N03). Feed update/spend races must be tracked as ledger conflicts rather than semantic oracle freshness successes.

## Pattern 4: joint signers and bearer roles

`joint-role.mori` expresses two distinct textual roles and exact recipient, with positive/negative label cases. It does not authenticate signatures. Native key-role admission requires both approved credential hashes in actual required signer hashes with corresponding ledger-valid witnesses. “Witness is present” alone is insufficient if absent from the V3 context required-signer set (N05).

Bearer variant is specified-only: bind role asset policy/name to this instance and consume the role-token UTXO in the same transaction; referencing it alone rejects (N04). Preserve the token at an authorized output or declare burn. A transferred bearer role can authorize its current holder, independent of obsolete address label, only after this role mapping is specified. Current source `/6` has two parties and no role-token spend primitive; a third arbitrator/multisig role is unsupported. Do not imitate this with supplied `true` flags or actor strings.

## Smallest required feature/gap

For the public suite, **no new arithmetic/source syntax is needed**: `/5` already handles distinct custody accounts and residual financial duties; `/6` handles supplied time/freshness predicates. The missing capability is a versioned source/Core-to-transaction relation that:

1. Resolves spend and reference inputs separately; authenticates head/genesis/script/program identities and exact successor/terminal disposition.
2. Binds finite actual validity bounds, authenticated oracle values and actual signer/consumed role credentials to source arguments using source-declared metadata/manifest, never pattern-specific host acceptance switches.
3. Enforces full source effects, retained obligations/history/work and whole-input gross/native fee policy against decoded transaction fields.

If `/6` authority is mandatory for escrow, its bounded role authority must additionally admit a distinct custody actor/asset cap and declared credential/custody mapping; use a new profile and preserve `/6` semantics. Existing `/5` is sufficient for initial public financial semantics, but does not supply `/6` authority guarantees. An observation declaration may improve ergonomics later; adding it alone does not authenticate ledger/oracle truth.

## Acceptance tiers

| Tier | Actual completion predicate | Current evidence |
|---|---|---|
| Source evaluation | Exact public source-derived evaluator poststate/effects/deltas match independently frozen expectations; required rejection has no published post/effects. | 22 matches and4 documented profile limits. |
| Compiled UPLC | Real pinned compiler produces script; actual UPLC/context evaluator accepts positives and rejects mutations with execution budgets; source/Core/context correspondence is checked, no fixture-specific adapter branch. | Unperformed. |
| Local real ledger | Isolated real Cardano node validates serialized transactions; chain state independently decoded; actual custody payouts, preserved debt, authentic input/reference identities, races and required signatures match; retain tx CBOR and IDs. | Unperformed. |
| Privacy/proof correspondence | Proof relation binds source/program/key, predecessor identity, authority, oracle policy/data, successor, complete effects/debt/history; actual target verifier and invalid-proof controls pass; private openings permit successor execution. | Unperformed; B does not claim this tier. |

## Source and graph locators

Graph query used `knowledge-focus.json`: node “Validity interval and oracle freshness binding” at `09-authority-intents/REPORT.md:P4 time, oracle and failed transaction semantics`; linked EXTRACTED edge to “Reference-input read without consumption” at `02-standards/REPORT.md:CIP-31 comparison`; P4 experiment rationale at `EXPERIMENTS.md:P4`. This is navigation evidence, not exhaustive review of the878MB extraction.

Underlying retained dossier `/home/charl/moriarty-utxo-research-2026-10-12`:

- `EXPERIMENTS.md:P0/P4` specifies state consumption, exact identity and whole validity window; P1/P2 require complete economic projection and retained duties.
- `03-marlowe/REPORT.md:25,34,40,65` identifies explicit time conversion, consumed state builder, timeout refund requiring a transaction, and endpoint/ambiguous-range controls. Pinned Marlowe builder: commit826b5fc5cf6ee77d40936d403301b5a65ec85ab9, `marlowe-transactions/src/Language/Marlowe/Runtime/Transaction/BuildConstraints.hs:529–627`.
- `02-standards/REPORT.md:27,56` locates CIP-31 and actual reference-input body field18; reference value does not balance the transaction and is not consumed. CIP source pin b4a593c960f2751fef2ddc8df28bec7b22c68eb5, `CIP-0031/README.md`.
- `09-authority-intents/REPORT.md:17,29,35,41,55`: actual required-signer distinction, spent bearer role vs reference, authenticated oracle vs timestamp freshness, and exact failing controls. cardano-ledger commit56ddee12ec768b85d14c62da049a33fc05114323, `eras/conway/impl/src/Cardano/Ledger/Conway/TxInfo.hs:497–543`; Plutus commit0849ed6a3385d17ca7dacc075b29f15999d490e9, `plutus-ledger-api/src/PlutusLedgerApi/V3/Contexts.hs:436–456,573`.
- `09-authority-intents/pyth-official.extract.txt`: Step1 and warnings; signature/state verification does not enforce freshness.

These are inspected retained sources. No new network evidence or current adoption assertion was acquired for this design.

## Assertion verifier amendment

`node verify.mjs` directly calls the actual /5 and /6 factories against frozen full expected envelopes. It does not call the exploratory feasibility runner, regenerate expectations, or read observed results. Every positive compares the entire accepted envelope, including all financial fields, debt, authority, history and work. Every ordinary negative compares its complete rejection envelope, including exact guard code, source span, node path and work debit. `expected-rejections.json` was authored from independently specified first failing requirements and static source/Core spans/counts using `freeze-rejection-expectations.mjs`, which never evaluates source. Exactly the four named profile limitations are allowed, and their complete `SELF_TRANSFER`/`EMPTY_BATCH` envelopes must match. Extra/missing cases, unknown profiles and additional declared limitations fail. Fresh run:22 exact ordinary envelopes and4 exact limits, exit0. A separate temporary expectation mutation payment100→99 produced exit1; retained in `verify-counterexample.stdout.json`. Historical feasibility stdout and observations were preserved.
