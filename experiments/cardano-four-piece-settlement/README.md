# Cardano four-piece settlement experiment

This experimental Aiken package compiled and matched **147/147 frozen expected unit outcomes** in one reviewed local run on 2026-10-16. The [root adoption](../../evidence/utxo/cardano-four-piece-settlement-2026-10-16/root/ACTUAL-ADOPTION.json) adopts the narrow predicate closed by the fresh independent [Opus actual-result review](../../evidence/utxo/cardano-four-piece-settlement-2026-10-16/reviews/actual-opus/REPORT.md). The result covers library predicates through `test_ctx`, compiled script sizes and a tool-derived parameter/hash chain. Compiled applied validator execution, genuine signatures/proofs and ledger settlement remain unproved.

This package sits alongside the separate [Rust experiment](../moriarty-cardano-rust/README.md), published at `515bd5c5`. The first Rust setup source was separately published at `82c20ad4`. Genuine current-Rust proof integration with these validators remains unproven: no current-Rust proof, validated proving-key decode, independent keyed verification or Preprod transaction is established here. This is an experiment toward Cardano support; a full Moriarty UTxO port and end-to-end integration remain open.

## Package and measured result

The frozen M2 package contains 32 source/configuration files plus the captured [aiken.lock](aiken.lock). Historical `PROPOSED`, `unreviewed` and `uncompiled` comments and the description in [aiken.toml](aiken.toml) retain their source-time wording; use the dated adoption above for the current local result.

The [Opus source review](../../evidence/utxo/cardano-four-piece-settlement-2026-10-16/reviews/source-opus/REPORT.md) records sixteen VS **components**, partitioned 6/3/4/3, plus three VI components. These are not sixteen catalogue claim IDs. The four public entry predicates are in [shard4.ak](lib/pvc2/shard4.ak).

| Script | Role | Applied bytes |
| --- | --- | ---: |
| [C2](validators/c2.ak) | Claim issue, burn and release | 2,770 |
| [VI](validators/vi.ak) | Authority shape, intent window and native co-authority; full wrapper also verifies signatures | 12,784 |
| [VS1](validators/vs1.ak) | Structure: shape structure/cells, own mint, top-up, reserve floor, global output distinctness | 6,859 |
| [VS2](validators/vs2.ak) | Accounting: recomputation, bounds, intent effects | 9,940 |
| [VS3](validators/vs3.ak) | Identity: claim script, shape digest, claim, statement digest; binds VI | 10,832 |
| [VS4](validators/vs4.ak) | Custody: prior value, continuation, payouts | 10,825 |
| [S2′](validators/s2p.ak) | Thread mint/spend wrapper; delegates settlement to four pieces | 13,018 |

The four pieces total **38,456 B**. All seven scripts total **67,028 B**, versus **48,413 B** for the prior four-script layout, an increase of **18,615 B**. Each piece is smaller than the prior monolithic VS at 20,503 B. The measured P01 host-CEK cost of each piece is lower; aggregate piece costs are higher for P01 and P07. Decode subtraction only approximates predicate work. See [sizes](../../evidence/utxo/cardano-four-piece-settlement-2026-10-16/run/receipts/sizes.json) and [cost comparison](../../evidence/utxo/cardano-four-piece-settlement-2026-10-16/run/receipts/OPX2-compare.json).

Every applied script is within **14,336 B**, a working per-script target. This target is not a qualified protocol parameter, transaction-size limit, fee or ExUnits budget, or deployability result. Host-CEK costs measure library predicates through decoded unit contexts, not compiled wrappers or ledger ExUnits. None of the 147 tests executes VI signature verification or C2 Groth16 pairing. Script hashes are tool-derived; independent recomputation (M-HASH) remains open.

The selected tests are 125 `shard4_test` cases, including nine `fail` annotations, plus 22 cost benches: 147 unit outcomes, zero failed, zero property tests. A passing `fail` case records expected refusal, not an ignored failure or an isolated reason for refusal. The retained 472 parent tests were not rerun in this measurement. `allCommandsExit0` is true; `allPredictionsHeld` is false because VS1 at 6,859 B misses the informative 7,000–10,500 B range. The [evidence README](../../evidence/utxo/cardano-four-piece-settlement-2026-10-16/README.md) preserves the original 129/147 run and its 18 failures.

## Local reproduction

Use the normal Aiken CLI from this directory. The captured tool was **aiken v1.1.24+bacbeb3**; the package declares compiler `v1.1.24`, Plutus `v3`, and stdlib `v2.2.0`, with its dependency lock included. Install that Aiken release and make the declared dependency available through normal Aiken tooling, then run:

```sh
aiken check --skip-tests --trace-filter all --trace-level silent
aiken build
aiken check --trace-filter all --trace-level silent --match-tests pvc2/shard4_test --match-tests pvc2/shard4_cost_test
```

These source-inspected reproduction steps have not been executed as a new clean-clone validation for this publication. The captured measurement applied parameters after building; `aiken build` alone produces the unapplied blueprint. Archived operator scripts record the spent experiment and contain hard-coded paths; they are evidence, not portable tooling or retry instructions. Internal root authorization records and home-directory paths are not package build dependencies. Maintainer experiment review and resource controls never become public developer compile/prove/deploy permissions.

## Acceptance boundary

The adopted Opus review carries **FV-01**: preserve the exact parameter DAG `C2 → VI(c2) → VS3(c2, vi) → S2′(c2, vi, vs1, vs2, vs3, vs4)`. VS1, VS2 and VS4 have no parameters. **FV-02** records that their shared hashes restrict atomic settlement across distinct S2′ deployments in one transaction.

The first unproved boundary is actual execution of the compiled, applied B9 wrappers on ledger-shaped contexts with genuine hashes (X1/PX04, M-EU), alongside independent hash recomputation. FX1 genuine signatures, actual proofs, a concrete Conway transaction with measured size/fees/ExUnits, registration and real Preprod follow. K6 concrete-operator work is source preparation; this publication grants no operation.

All 20 UTxO families; ACTUS 277 fixtures/18 types/32 taxonomy items/DS01–07; DeFi 72/DA01–24, held-outs, intents and regressions; general financial obligations; real recursion/PCD; private handoff; bounded multi-parent composition; universal certificates; and real Preprod remain open. Midnight U0–U7, M1–M17 and every MC/SP/G exit retain their separate acceptance requirements. No compatibility percentage is claimed. The synthetic or caller-declared `.ak` unit contexts authenticate no real chain. See the [full inventory and provenance](../../evidence/utxo/cardano-four-piece-settlement-2026-10-16/INVENTORY.md).
