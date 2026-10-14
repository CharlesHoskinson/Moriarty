# FOREMAN_REPORT

Worker: Grok 4.6 high author.
Worktree: this SP01 composition worktree.
Task: implement the admitted finite decoder, primitive relation, ordered replay and claim checker (CM01–CM10).
Status: specified-only finite executable checker. Independent GPT-6 result review is still required.
This is not a successor semantic freeze, Moriarty evaluator, K implementation, proof, Preview settlement, ledger acceptance, or sprint completion.

Independent feasibility judgment before code: the reviewed proposal is internally consistent. Pending→Refundable opening is distinct from RefundLock consumption. Canonical Ord serialization is a total order on effects. Generator self-check is not independent expected-state evidence. No material semantic contradiction. Work proceeded under the two-vote delegated decision. No source acceptance is implied.

## Owned files

- `experiments/moriarty-language/spec/successor/financial-fragments/composition.json`
- `experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py`
- `experiments/moriarty-language/spec/successor/financial-fragments/composition-model.test.py`
- `FOREMAN_REPORT.md`
- `FOREMAN_REPORT.json`

Immutable candidate-03 was the generator seed. The generator does not read the mutable output path.

## Interfaces

`decode_candidate`, `apply_primitive`, `replay`, `check_claims`.

Check CLI (exit 0 only if every declared positive, prefix, branch, negative and mapping passes):

```
python3 experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py \
  --check-candidate FILE --old-fixture FILE --old-pins FILE
```

Generator (immutable seed; self-check before write):

```
python3 experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py \
  --seed FILE --old-fixture FILE --old-pins FILE --output FILE
```

Independent tests invoke the check CLI as a subprocess and do not import implementation helpers:

```
python3 experiments/moriarty-language/spec/successor/financial-fragments/composition-model.test.py \
  --candidate FILE --old-fixture FILE --old-pins FILE
```

## Commands actually run

Inputs:

- seed: immutable candidate-03 `composition.json` sha256 `c5801af2a4fd14e5968c4ecc00963e515ba19fdb38c052cf0fb3f94de42a054e`
- old fixture: `/home/charl/.local/state/moriarty/sp01-atomic-fixture-20260908/fixture.json` sha256 `2c44dcd0b364afae95213568aa74dd0e75dbc4264b4164772ba0593efbc1a560`
- old pins: `/home/charl/.local/state/moriarty/sp01-map-fragments-20260908/composition/old-domain-pins.json` sha256 `c1025a13d7ccc8c0dfed808969165ca968c4b5af14f167eb80fc028fb2902db0`

1. Generator to owned `composition.json`. Exit 0. output_sha256 `f912745cf1edb9d376300fd1690b92c6df613b3343027a2c7e98a3d0e43f57ae` output_bytes 770232.
2. Check CLI on owned candidate. Exit 0. report sha256 `7735927f57165ef055e073b17493665a3ea30104e0f51d7ab88e66c0be62bf1f`. checked positives=6 prefixes=100 branches=2 negatives=11 mappings=2. failures=[].
3. Independent tests. Exit 0. SUMMARY total=84 fail=0 pass=84. Four adversarial probes each rejected (createCash, erasePrimitiveEquation, eraseNominalDebt, forgeFailureEvidence). Additional rejects: Alice 101 drift, Lock declared as cash, missing second RR.
4. Two further generations from the same immutable seed to `/tmp/sp01-comp-check/a/composition.json` and `/tmp/sp01-comp-check/b/composition.json`. Both exit 0. Both sha256 `f912745cf1edb9d376300fd1690b92c6df613b3343027a2c7e98a3d0e43f57ae`. Bytes equal to each other and to the owned output.

Internal checker is self-consistency only. Independent black-box oracles in the test module remain the acceptance evidence for cash, NAM projection, first failures and probes.

## Derived cash and NAM (independent oracles)

- NAM19: IED/RR/IPCI/RR; reported principal `5236.461356333502`; duty 5000 exists before capitalization; two rate resets including `0.111679012345679000`.
- sequential: Alice 70, Bob 30.
- parallel: Alice WETH 13, Bob USDC 29.
- shared: Alice 29/16, Bob 8/12/LP 9, PoolP 132/92, Treasury 1.
- atomic: Alice 0/47, Lender 100, PoolQ 1050/953, Treasury 1, Alice gross 101.
- async refund main: Alice 39, Escrow 0, Bob 0, Treasury 1. Lock is annotation-only. Transfer is the sole cash mover.

Footprints (distinct derived accesses): 23, 11, 21, 32, 31, 26. All ≤ 64. No hidden-field omission. Toy maximum 64 unchanged.

## Remaining limits

Owned source after this report pair is recorded in FOREMAN_REPORT.json. Bound is 1048576 bytes.

Full BNF and K remain owed. This finite checker does not substitute for them. Real network, parser, shared language source, proof and ledger settlement remain outside this task. Fresh independent GPT-6 result review is still required. No candidate is accepted by this author report.
