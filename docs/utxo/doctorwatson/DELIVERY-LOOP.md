# DoctorWatson language delivery loop

Current delivery focus, 2026-10-20: implement the first scoped Cardano Groth16 settlement through the existing `doctorwatson<UTXO>` consumer. Use direct model calls, Git/GitHub and repository-native commands. DoctorWatson remains explicit local QA; this policy adds no scheduler, daemon, Foreman, campaign, hosted CI or approval menu. The [product contract](../../MORIARTY-PRODUCT-CONTRACT.md) and original [roadmap](../../../ROADMAP.md) remain controlling. Cardano evidence does not establish Midnight ZKIRv3 acceptance.

## One transaction acceptance target

Reuse the genuine current-Rust invoice proof recorded in K6 and the already compiled C2 wrapper exercised in K10. Advance that path to one concrete, funded **Preview** transaction, followed by chain readback and the designated security owner's tampered-input rejection checks. Historical proof and wrapper observations are reusable scoped evidence, with their original Class-0/setup/context/publication qualifications; repeating a passed prefix solely to refresh a report is not progress.

The acceptance record must bind the exact proof, public inputs, relation/verifying-key identity and applied wrapper to the proposed transaction. It must retain the target tuple, complete signed transaction identity, funded inputs, collateral, outputs, fees, whole-transaction size and ledger ExUnits. Submission alone is not settlement: observe inclusion, consumed inputs, complete effects and the declared confirmation condition. Record tampered rejection at its actual tested boundary. A C2 claim-mint receipt establishes only that scoped proof-consumption result; financial settlement still needs its intended consumption and complete effects, and the historical fabricated context supplies no authenticated wallet or intent claim. Preview, Preprod, Fenrir and Midnight stagenet retain separate evidence.

Use the existing entrypoint to identify the first missing step:

```bash
./scripts/doctorwatson-utxo.sh cardano-next
./scripts/doctorwatson-utxo.sh cardano-next --json
./scripts/doctorwatson-utxo.sh cardano-next --cardano-observations public/cardano-observations.json --json
```

`--cardano-next` is the equivalent explicit selector. JSON separates historical evidence, current local observations and Preview chain acceptance. A successfully selected action does not mean that action ran. The original no-argument metadata/regression checks and `--next`, `--requirement` and `--gate` selectors remain available.

`--cardano-observations` accepts a public JSON object relative to the selected `--repo`, with optional `preview` and `stages` objects as documented in the [selector](../../../models/utxo/qa/cardano_first_proof.py). The normalized public snapshot goes in `preview`; exact consumer-result reports go in `stages`. The existing safe metadata reader rejects escapes, private paths, links and malformed JSON. This option requires the Cardano selector and supplies reports to its actual readiness and stage consumers; it performs no query, evaluation, signing or submission, and does not authenticate the reports. Missing distinct funding/collateral inputs remain named prerequisites; a historical funding receipt neither supplies current inputs nor calls for a replacement wallet or faucet.

The selector also reads bounded Graphify context through the existing graph-navigation consumer. It preserves graph freshness and partial coverage, supplies an exact seeded query when its identity is bound, and requires reading the actual source. It neither refreshes the graph nor runs the suggested query. Graph labels and workflow-model checks establish no proof, chain result or live continuation state.

This first path does not wait for whole-language, ACTUS, DeFi, parser or frontend cleanup. Their requirements remain open. The original EUC-012 generated-Aiken and EUC-013 generated-spending predicates cannot close through a fixed C2 wrapper. The [assessment](../../../openspec/changes/utxo-doctorwatson/current-evidence.json), [audit](ROADMAP-AUDIT.md) and [roadmap register](audit/roadmap.md) retain their broader gaps and historical failures.

## Three source lanes and one integrator

| Lane | Source result | Ownership boundary |
| --- | --- | --- |
| Proof handoff | Bind the reusable genuine invoice proof and compiled wrapper into the exact downstream public handoff; identify missing inputs explicitly | Handoff module and its scoped ordinary fixtures; security interpretation remains with Opus |
| Preview readiness | Prepare the concrete transaction/readback contract and report missing funding, parameters or target evidence | Preview-readiness module and its scoped ordinary fixtures; preserve wallet identity and transaction admission |
| Consumer selection | Select the earliest missing step and expose its consumer, required evidence and acceptance scope through DoctorWatson | Selector, bounded Graphify context and public CLI, with agreed interfaces |

Assign disjoint paths and bounded tasks before writing. One integrator owns shared interfaces, the composed tree, affected checks and integration. Dependencies constrain consumption and acceptance while independent source preparation continues. The lanes describe implementation ownership; they create no native concurrency or execution authority.

Nonsecurity authors use GPT-6.1 Sol at high effort. Fresh non-author reviews follow the current applicable routing; security authoring, hostile-test design and judgments belong to Opus under the checkout's current availability/fallback rules. Review changed source and actual results with exact applicable requirements and independently frozen expectations. Preserve reviewer identity, effort, terminal status, dissent and limits. Missing or failed providers do not approve. Keep review context bounded to the changed consumer, necessary dependencies and actual outputs; do not repeatedly send multi-megabyte historical packets or rerun already passed prefixes without a changed dependency or unresolved defect.

## Observe, review and continue

For each increment, name one falsifiable consumer predicate and inspect its actual transitive effects. Implement the smallest useful source change, run the affected authorized local checks, and review the exact resulting bytes and outcomes. Report source preparation, observed local behavior and chain acceptance separately. A helper without a consumer is preparation; a test count or metadata-consistency pass cannot establish ledger behavior. New source or consumed dependencies invalidate affected reviews and observations.

Financial expectations remain independent of the candidate: preserve complete affected and protected state, ordered effects, durable identities, frame, gross debit, all fees, net receipts, reserves, liabilities and residual duties. Keep logical work, circuit cost, DUST/service fees, recovery reservation and wall time distinct. Do not replace an oracle with implementation snapshots to remove a failure.

Preserve the existing concrete execution admission, aggregate accounting, job/process limits and native resource boundaries. C4, exhausted R3 k17, historical terminal attempts and their counters remain intact; this priority neither resets caps nor revives failed attempts. Exact proof/setup/pairing, SDK/VM/key, wallet/provider and ledger operations use their applicable recorded admission. Authorized source edits and inspected ordinary QA continue independently when one of those operations is blocked. Do not turn an operation-specific hold into a blanket barrier on Cardano source implementation or apply a Midnight-only constraint to every target.

After two failed cycles of the same defect class, retain the first failing input/field, reduce a reproducer and change approach. After two process-only cycles or thirty minutes of administration, return to eligible implementation or consumer diagnosis. Renaming tasks or changing agents does not reset these counters. Record the actual delta, first remaining missing predicate and next feasible action in existing evidence locations; do not fabricate a transaction, acceptance receipt or closure.

## Wider exits remain open

Retain every original requirement and scenario: 119 UTXO clauses, 11 first-contract requirements, 18 Midnight PCD clauses, all twenty UTXO families and all 69 U0–U7/M1–M17/MC01–08/SP01–12/G01–24 identities. Preserve lifecycle/recovery, recursion, private handoff, multi-parent PCD, universal property certificates, fee/federation policy and independent-developer release exits.

ACTUS retains 276 contract cases plus AD0 analysis as the 277 aggregate, 18 executable types, 32 taxonomy dispositions, DS01–07 and the ANN file-count discrepancy. DeFi retains all 72 original rows, DA01–24, supplements, three held-outs, eight intent cases and eight regression classes. Sequential, disjoint parallel, shared-state interleaving, atomic synchronization and asynchronous composition each require their own evidence. K, finite Quint, Lean/Clean, circuit/native and target correspondence remain separate obligations.

One scoped Preview settlement is a useful child delivery. It cannot close these wider exits, establish general language support or complete the language goal.
