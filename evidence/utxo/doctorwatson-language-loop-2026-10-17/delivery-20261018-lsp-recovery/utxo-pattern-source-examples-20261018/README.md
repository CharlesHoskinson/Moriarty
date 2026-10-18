# Prefunded bilateral exchange: ordinary source example

This source-only proposal adds one small developer example for `moriarty/1` and `moriarty-core/1` at exact main `74a909d69639df8823a527a4d608a7516ccd69a2`. Its proposed normal call exchanges A10 for B25, pays a separate A1 processor fee, increments revision 4 to 5, and preserves unrelated balances and two residual duties. Parsing, validation, simulation, metering, compilation and all target operations are UNRUN.

The proposed example is expressible using the inspected current Core/1 parser and interpreter primitives. That is a source-based inference, pending the actual consumer check and independent review. It introduces no language extension.

## Files and actual consumer

- `prefunded-bilateral-exchange.mori`: full developer program source.
- `exchange.normal.scenario.json`: complete typed input for the `exchange` action.
- `PROPOSED-NORMAL-OUTCOME.json`: independently authored complete pre/post snapshots, six ordered effects and all eleven normal statement prefixes. It is an expectation artifact, not captured output or guessed expression-index IR.
- `TWENTY-FAMILY-SCOPE.json`: all twenty original family identities and historical matrix fields, with every family acceptance kept open.
- `SOURCE-MANIFEST.json` and `main-inputs/`: exact revision and byte-pinned public input sources.

The intended ordinary consumer is `mori_core::parse`, followed by `mori_core::simulate(program, "exchange", scenario)` from `crates/mori-core/src/runtime.rs`. An optional separate call to `simulate_metered` under `UInt(34)` has a proposed logical cost of 34. None of these APIs was invoked. The source literal `local` is an asset-identity label; it selects no node, wallet or Cardano network.

## Why this example

Existing `examples/core/batch-clear/batch.mori` already expresses two-asset batch transfers and fees. `enforcement-report/source.mori` shows two assets and residual delivery duties; `escrow-recovery/escrow.mori` contains a larger staged financial story. The new deliverable is a concise, dedicated one-action prefunded bilateral exchange with an explicit separate fee payer and a complete trace retaining unrelated duties. The checked filename inventory establishes that the proposed dedicated example path is missing; it does not establish global absence of exchange behavior.

Three bounded candidates were considered: copy the larger staged escrow example, add a constant-product arithmetic example, or write this fixed-account exchange. Copying escrow adds little teaching value. Exact constant-product quotient computation is not a Core/1 primitive because division is unsupported; this proposal does not substitute a hardcoded quotient for that missing capability. The fixed-account exchange uses existing primitives directly and gives a useful independent normal expectation.

## Complete proposed normal outcome

The input supplies `quotedA=10`, `quotedB=25`, `serviceFee=1`, `phase=0`, `revision=4` and `marker=77`. All four guards therefore evaluate to true. These are ordinary caller-supplied facts.

1. A10 moves from `escrow-a` 10 to `trader` 3, producing balances 0 and 13.
2. B25 moves from `escrow-b` 25 to `provider` 4, producing balances 0 and 29.
3. A1 moves as a distinct `Fee` from `fee-payer` 5 to `treasury` 2, producing 4 and 3. The A10 business receipt remains intact.
4. `phase` changes 0 to 1, then `revision` changes 4 to 5. `marker=77` is untouched.
5. The final `ensures` is true. The last effect emits `exchanged` with UInt10.

The complete balance table follows. Asset identities include exact network/issuer/reference/scale fields in JSON; A and B share only the network and issuer labels.

| Asset | Account | Before | After |
|---|---|---:|---:|
| A | escrow-a | 10 | 0 |
| A | fee-payer | 5 | 4 |
| A | observer | 9 | 9 |
| A | trader | 3 | 13 |
| A | treasury | 2 | 3 |
| B | escrow-b | 25 | 0 |
| B | observer | 11 | 11 |
| B | provider | 4 | 29 |
| B | trader | 8 | 8 |

A totals 29 before and after. B totals 48 before and after. `service-B` remains B2 owed by provider to auditor. `warranty-A` remains A7 owed by provider to trader. No statement creates or discharges a duty. Phase1 means this exchange action completed locally; it does not assert debt-free termination or completion of the agreement's other duties. Existing Core/1 permits external duties in a supplied scenario; these rows are not authenticated history.

Core/1 normalizes balances by exact asset then account and duties by name. This expectation follows those rules. Object-key order is immaterial, while array order follows the selected Core/1 normalization. This differs from legacy /5-/6 evaluator row-order contracts and supplies no automatic profile adapter. The JSON contains every field of the proposed pre/post snapshots and all normal effects; no unsupported legacy allowance, nonce, operation-history, transport or lifecycle-work fields are invented.

## Work and observed status

The eleven statement costs are proposed as 4,4,4,4,2,2,2,2,4,4,2 under `moriarty-reference-logical-cost/1`. Each is one statement charge plus its evaluated expression nodes, totaling 34. This is an independent source calculation, not a metering observation, proving estimate, time bound, ledger ExUnits or fee measurement. Unmetered `simulate` returns no `workRemaining` counter.

## Original families and remaining boundaries

The example illustrates primitives related to `bilateral atomic escrow swap` and the separate fee aspect of `F04-fee-gross-net-policy`. It neither reproduces the original /5-/6 SB01 envelope nor supplies gross-input intent enforcement. EU-FIN-009 and its original positive/failure scenarios remain open. EU-FIN-001/003/018/019/020/024/026 and EU-CON-002/004/005/006 retain their full independent-evidence, field/profile and layer requirements. All twenty original families remain listed and open; the example is not a twenty-family implementation or native result.

Account labels such as escrow-a do not prove custody. This source makes no role/signature, authorization, nonce, replay, oracle, property-certificate, cryptographic or other security claim. It supplies no authenticated input outrefs, real asset policies, complete body, Cardano claim issuance/consumption, VI/VS composition, native transaction fit, submission, inclusion, finality or ledger readback. Local interpreter semantics cannot establish atomic settlement across real assets. General recursion, private handoff, multi-parent PCD, mandatory claim discharge and Midnight acceptance remain separate open obligations.

## Next actual check and integration

After root's full source audit, run only the separately authorized ordinary Core consumer on these exact bytes and compare complete pre/post/effects and every prefix against the frozen expectation. Resolve any parser/normalization/result mismatch at its real layer before considering integration. Logical metering, if separately selected, compares the independently derived 34-unit expectation. No test or runtime command is prescribed or executed by this bundle.

Proposed repository placement is `examples/core/bilateral-exchange/`; root owns integration and fresh applicable review. This task wrote only its newly assigned output directory. Native authority remains zero and the broader Preprod goal remains paused.

## Read limits

All repository evidence is read from exact revision 74a909, not mutable main. Full original financial requirements and the family matrix were read; Core README, schema and selected parser/runtime/meter spans supply the expressibility inference. Some broad combined reads were truncated, including the large FeePool example; no semantic audit of every example is claimed. Full public inputs are copied for the integrator. No SDK/compiler/Node/model/test/proof/wallet/network/provider dispatch or shared Git/worktree write occurred. The development skill startup status was read as historical context. No historical absolute pointer was followed.
