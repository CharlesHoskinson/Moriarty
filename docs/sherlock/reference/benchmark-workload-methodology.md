# Benchmark workloads and measurement methodology

The versioned [catalog](../../../models/sherlock/benchmarks/workloads/catalog.json) defines seven fixed cases: actual public MapQueue and MirrorArbor source programs, a two-transition LiteralSeven recursion target, and Fenrir-derived public synthetic prefixes of 16/32/64/96 transactions. It extends the full Sherlock implementation and stagenet recursive-rollup objective. It does not close U0–U7, MC/SP/G, formal correspondence or retained EARS requirements. Ordinary tests establish data behavior only. The existing native owner selects eligible operations and resource profiles; the benchmark files grant no compiler/proof/wallet/network authority and start no operations.

## Reproduce the public data

From the repository root:

```sh
python3 models/sherlock/benchmarks/workloads/generate.py generate --out /tmp/sherlock-public-corpus
python3 models/sherlock/benchmarks/workloads/generate.py validate
python3 -m unittest discover -s models/sherlock/benchmarks/workloads/tests -v
```

Generation is bounded: four prefix JSON files and six MapQueue source variants. It writes no keys, proofs or receipts and invokes no subprocess. The fixed seed `sherlock-public-fenrir-derived/1` identifies a transparent deterministic schedule; it is a dataset/version label, not an RNG seed compatible with Fenrir's ChaCha8 generator. Every identifier begins `test:` or `synthetic:` and has no wallet/network meaning. There is no recovery material, live address, signature or private witness in this corpus.

The catalog's `inputs` list carries exact SHA-256/byte identities. `workload_digest_sha256` hashes canonical UTF-8 JSON of that ordered list: sorted keys, compact separators and unescaped Unicode. `expected_effects_sha256` hashes the exact expected-file bytes; `expected_effects_canonical_sha256` hashes its canonical parsed JSON. Neither digest authenticates a proof or ledger observation. Expected documents and generator provenance are frozen separately. Source/compiler/ledger/network/circuit identities in measurements must additionally match the actual current execution. A digest matching stale or counterfeit evidence cannot qualify acceptance.

## Actual Moriarty programs

MapQueue declares `Queue<T>` and `Envelope<A,B>`, recursively increments the input queue `[3,8]` by two and returns `Wrap([5,10],true)`. MirrorArbor declares recursive `Arbor<T>` and `Stamp<A,B>`, exchanges children and negates leaves, then returns `Seal(Fork(Fork(Tip(false),Tip(true)),Tip(false)),41)`. Full independently authored expected trees include generic arguments, constructor names, declaration-ordered field names/types and every leaf. Semantic call/return dependencies are listed in order; they are not an independently checked complete StepEventC trace. Historical source observations of 359/545 consumed work and 360/546 events are deliberately excluded from expected work. Exact independent microstep/work expectations are unavailable.

The catalog gives the existing public `mori simulate` command with explicit source, execution profile, config and target. It consumes real source, compiler/lowering and owner simulation; it does not produce native proofs. The checked-in commands are recipes for an already eligible consumer, not commands executed by this suite. Public binary build and underlying native routes remain separately qualified. The MapQueue scaling axis changes only closed-entry length to 0/1/2/4/8/16, preserving program declarations and algorithm. Generated source is concrete. Its successful compilation and resource fit are unobserved; larger lengths may reach the fixed parser/config limits. The asymmetric fixed five-node MirrorArbor complements linear recursion with binary recursion and heterogeneous generic return fields. Do not extrapolate exact work or native k from either source size.

## Two linked transitions and genuine recursion

The minimal target uses the actual source-compiled LiteralSeven producer: A102 `Eval(Literal(7)) → Return(UInt(7))`, followed by uncharged A80 `Return(UInt(7)) → StepDone(UInt(7))`. Charged reference work is one; transition count is two. The exact existing ignored producer and independent frozen-directory consumer tests and their required environment selections are named in the catalog. These are native surfaces and are not executed by ordinary tests.

Each child must be genuinely proved and separately verified. Every program/context/state lane must link, and parent recursion and terminal decider must cryptographically bind both children, the inner keys, predecessor/result root and height, and ordered transition commitment. A stored digest, host success flag or two independently verified children alone does not satisfy this case. Source correspondence, Init/full StepInv, other dispatch rows, transcript/key compatibility, parent and terminal consumption remain explicit premises/gaps. The full independent native public-instance oracle is unavailable, so `expected_effects_independence` is `unavailable` and no target acceptance can pass this case yet. Native layer IDs are `child-0`, `child-1`, `parent`, `decider`.

## Fenrir-derived neutral public corpus

The public profile preserves the source-supported shape: fifty headers and sixteen ordered transactions per batch; four transactions per leaf and four leaves per full batch. Prefixes 1/2/4/6 select the same single six-batch history. Sixteen slots end at heights 3,6,…,45,50 within each batch, retaining empty-header height advances in neighboring endpoints. These are declared normalized endpoints, not real block timestamps. Cross-batch recursive fan-in/depth is unestablished and must be measured from the selected real adapter.

The schedule includes a synthetic deposit, checkout with change, exact checkout, peer transfer, merchant sweep, synthetic withdrawal and ten alternating peer transfers. Transactions contain amounts, fees, transparent account identifiers, synthetic note-output counts, and complete before/after twelve-field state records. This account/scalar projection is newly authored public data. It is **not** Fenrir's note/witness schema, signing rules, Merkle computation, Poseidon state digest or executable `stage_batch` input. No native-compatible claim follows. The four root fields carry `{value:null,status:"unavailable",reason:...}`; artifact SHA-256 values are labeled corpus-byte commitments and never treated as cryptographic state roots.

Independent expected effects were authored from these per-batch equations before generator tests:

| Quantity | Per batch |
| --- | ---: |
| Alice balance change | +76 |
| Bob balance change | −10 |
| Merchant balance change | 0 |
| Processor balance change | +6 |
| Synthetic deposits / withdrawals / fees | 100 / 13 / 15 |
| Synthetic note outputs / spends | 28 / 15 |
| Header advances / transactions | 50 / 16 |

Genesis balances are 10000 each for Alice/Bob and zero for merchant/processor; synthetic commitment count starts at two, nullifier count at one. Final supply changes by +72 per batch. The independently frozen files contain all scalar endpoints, complete account balances, gross deposits/withdrawals/fees and ordered transaction IDs. The scalar replay checks every transaction's full scalar predecessor/result, order, supply conservation and batch continuity. The four tree roots and full native effects remain unavailable, so native acceptance remains incomplete despite an independent scalar oracle.

An alternate source-compatible profile uses the existing owner-built `straw-history generate` to create a **new** synthetic six-batch v2 history with an explicit transparent seed, and `recl9 stage HISTORY_DIR BATCH_NO BATCH_DIR` for each selected batch. Exact positional staging syntax was source-inspected. Before any eligible operation, its owner must pin the current private generator/stager, SDK and outputs. The suite's `inspect-owner-history --history HISTORY_DIR` command only reads metadata/shape and emits counts, never payload. It requires v2 per-header root-history append witnesses, exactly sixteen transactions per batch and all twelve state fields. Existing private fixtures/source are not distributed. Source-oracle self-replay is correlated and cannot supply the missing independent cryptographic-root oracle. Build/generation, SDK compatibility, independent full-state verification, and the Sherlock financial/native adapter remain deferred.

The eight invalid transaction and three invalid batch source predicates remain in each manifest: duplicate/previous nullifier, wrong-key/wrong-message signature, unpaid fee/extra output, expired/unknown root, wrong predecessor, replay and poisoned batch. Their intended rejection layers and actual native verdicts must be retained. Ordinary scalar-tamper tests are not observations of those native cryptographic rejection predicates.

## Measurements and comparability

Freeze source/inputs, independent expected effects, expected work and ordered predicates before the owning implementation's execution. Preserve actual raw failures and censored observations. Separate frontend, lowering and native compile phases and cold/warm cache conditions. If existing commands expose only combined subprocess duration, keep it combined/qualified and mark unavailable phase metrics; do not relabel it pure frontend, lowering or algorithm-kernel time. Record wall/CPU/peakRSS, process/thread/hardware/toolchain/build profile and exact operation receipt for each actual phase.

For each real child/leaf, parent and decider record actual k and domain capacity separately from exposed used/lookup rows and columns; `2^k` is capacity, never a row-usage measurement. Key generation, witness, prove and local cryptographic verify have separate wall/CPU/RSS values and proof/key/instance/artifact byte counts. Used rows/columns are optional only when unavailable in the selected exposed interface, with explicit reasons. Required timing/k/byte metrics remain missing if unobserved, preventing profile completion. Actual native k/time are unknown; no positive estimate is an expected result.

Record deployment/call serialized transaction bytes, fee/DUST, proof-ready, submission, acknowledgement, inclusion and complete settlement readback with immutable network/ledger/contract/circuit identities and complete before/after effects. Local verification, RPC queue/inclusion and readback latency remain distinct. Network observation does not measure validator CPU. The metrics interface preserves unavailable/censored/failure rather than zero filling. Hardware, software, workload/profile, cache state, evidence class and resource choice must match before combining repetitions or comparing regressions.

No latency, throughput or regression SLA was supplied. Start with one selected genuine end-to-end deployed sample, then choose repetitions and scale stages explicitly within existing operation limits. Do not automatically rerun setup/keygen or unchanged passing prefixes. Publish raw observations, successes/failures/censoring counts and baseline distributions. A single observation is a baseline sample, not a reliable tail estimate. Regression tolerances, if later chosen, must identify their comparable profile and statistic before execution. Historical Fenrir k19/64KB constraints do not apply to Midnight without verified current applicability. The nominal host serialization cap is likewise not a transaction limit. Larger scale/repetitions remain separately incomplete after a smaller sample succeeds.

## Primary sources consulted online

Retrieved 2026-10-21 UTC. Only public terms were sent to search; no private repository path or payload was sent.

- [Succinct Labs zkvm-perf](https://github.com/succinctlabs/zkvm-perf): explicit program/revision/trial parameters and separation of setup downloads from timing. Borrowed the concepts of immutable workload/software identities and declared cold/warm setup. Its hosted workflows are not adopted.
- [RISC Zero performance datasheet, April 2023](https://dev.risczero.com/datasheet.pdf): separates execution size, prover time, RAM and proof bytes and explains padded execution domains. Borrowed capacity-versus-work distinction; no RISC Zero timings/cycles are treated as Sherlock predictions or current cross-product comparisons.
- [Gabriel Scherer, How to benchmark: the Measure-Explain-Test-Improve loop](https://cambium.inria.fr/~scherer/drafts/how-to-benchmark-2026.pdf): benchmarks require testing and falsifiable explanations. Borrowed independent correctness/economic checks and explicit scope before performance interpretation.

Local source provenance is recorded in `SOURCE-PROVENANCE.json`; online sources inform methodology and do not prove native compatibility or Sherlock results.
