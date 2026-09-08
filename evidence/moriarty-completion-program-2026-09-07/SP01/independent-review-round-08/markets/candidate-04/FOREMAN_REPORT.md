# SP01 markets fragment (specified-only SR01-SR05 correction)

Group `markets`. Status `specified-only`. This is not a semantic freeze, implementation, proof, runtime, K, native, Preview, network, or full-map acceptance. Candidate-03 is immutable. Pending independent GPT-6 Astra review.

Worktree `/home/charl/Moriarty/.worktrees/sp01-map-markets-grok`. Base `45c42cf95d0e500f8307e99214c8f23ebc701ce6`. Input packet `6641cbbadf6505b2404ce741e4917b81eba9b3105b9de98885bef2451ea03a71`. Review `6bd6b9b9e09dff877360d29322f807c86146a1b8aacc1f809b0f5b1a703b916e`.

The prior candidate-03 report said sixteen step transfers. The six traces contain 16 steps and 13 token transfers.

## Owned files

| path | bytes | sha256 |
| --- | --- | --- |
| experiments/moriarty-language/spec/successor/financial-fragments/markets.json | 476700 | `933f1bf541ce33538c73ddc5d8330331fa2d106bd8524d297163207a447706eb` |

Six rows, six traces, eleven mutations. Three owned files stay under 512KiB. Source pins, owner packages, and closure sprint/task are unchanged. Toy and non-protocol scope remain explicit.

## Patch checker (read-only)

Checker `/home/charl/.local/state/moriarty/sp01-map-fragments-20260908/verify-market-patches.py` sha256 `6918adaf3062574f7c94a6ddaecdda0a2e4988f4b5b0fce74612f49b9b75a7f9`. Semantics are this fragment `typed-patch-inheritance/1`. The checker was not edited. `patch-diagnostic.json` was not overwritten.

Candidate-03 already materializes. SR01-SR05 are identity, footprint, bound, and claim defects, not patch-inheritance diffs. The candidate-03 checker run exits 0.

```
python3 .../verify-market-patches.py .../candidate-03/.../markets.json --output .../markets/context-sr-candidate03-patch-red.json
```

Exit 0. Status `pass`. 0 field differences. Output sha256 `467e71d82651f03f250f578917f613b0776369259adcb8537caa6426c3d602bf`. This matches the retained root GREEN hash.

Worktree GREEN:

```
python3 .../verify-market-patches.py .../sp01-map-markets-grok/.../markets.json --output .../markets/context-sr-worktree-patch-green.json
```

Exit 0. Status `pass`. 0 field differences. Output sha256 `efd0e53ce8e349013b57ede72bb11a3ea702b32967e362dbef284dfd02f8f17a`.

Historical candidate-02 still fails the same unedited checker with 16 field differences. Output sha256 `7e5ec25a9297fafdce772e5371c13708128c164c2c3ea530dd2d846d466b4013` equals root `patch-diagnostic.json`.

Captures stay under `/home/charl/.local/state/moriarty/sp01-map-fragments-20260908/markets/`. They are not only `/tmp`.

## SR01-SR05 (specified-only)

SR01. Mutations 0, 1, and 8 now consume their admitted genesis, admin, and head. Newton 3, margin 7, and boundedCross resolve their own genesis, admin, and environmentRegistry records. Resolved IDs match current membership. Baseline `sharedRecords` genesis is not the sibling identity.

SR02. Mutation 0 rollback restores pool cap 19744. Mutation 1 rollback restores pool cap 4883. Rejection writes work only. Negative assertions show baseline 19743/4882 would change original authority by -1 and cannot pass. Conflicting `samePreStateAs` fields were removed.

SR03. All 16 steps list operand reads. Settlement steps write reserves, outgoing caps, remaining work, history, and registry. Redemption steps 0/1 write debt, collateral, and principal. Vault step 1 writes temporary `unsettledDelta` token1 -1. Alias expansion then dedup makes `vault.unsettledDelta` and `unsettledDelta` one resource. Vault unique reads/writes are 31/21. AMM unique/weighted 31/45 is unchanged. Rejection writes include conservation and reserve flags.

SR04. Shared `boundDomains` name concrete arrays, maps, and the canonical sidecar UTF-8 encoding. Checks cover input, proposed post, and effects, including collection 32, schedule 32, ticks/positions 16, and sidecar 4096. Isolated `postmessages9`, `collection33`, and `sidecar4097` controls live under trace 0. Existing messages/claims/observations/predecessor checks remain. Mutation IDs stay 11.

SR05. BoundedCross instantiates work conservation 32+12=44 and 1024-44=980, gross caps 477=477+0 and 4882=4882+0, plan/effects/frame, and admission/successor predicates on its own records. Crossing 477/4882/fee1, tokensOwed 1, and clamp 909/0 are unchanged. Isolated `crossWorkReset` (980 to 1024 with matching patch and top-level work) and `crossAuthorityPreservation` must fail. A PASS flag is not evidence.

## Cash oracles (unchanged)

Exact output 19743 remainder 162290000. CL input 477 output 4882 fee 1. AMM D 1999 y 850 dy 50. Redemption burn 150 gross 300 fee 3 net 297. Vault 10/9/1. Margin withdraw 75. Thirteen token transfers across sixteen steps match plans and effects. Eleven negatives keep ordinary work on original 1024 with reserve 16 unspent.

## Independent reconstruction

Executable recipe `/home/charl/.local/state/moriarty/sp01-map-fragments-20260908/markets/verify-markets-sr.py` sha256 `09b21875841800835e38417bf10d568e4533c773d213afc36f5f476c35ce0713`. Capture `context-sr-verification.json` sha256 `89f4f5e13e93248ce53bcebcd3250a27ffe7f8ad60f71bf140d008d8f0622ea8`. Two runs: 67 passed, 0 failed. No generator. Structured `$ref` count 304. Unresolved 0.

```
python3 -c "rI,rO,aI=1000000,2000000,10000;n,d=997,1000;adj=aI*n;num=adj*rO;den=rI*d+adj;o=num//den;print(adj,num,den,o,num%den)"
# exit 0  9970000 19940000000000 1009970000 19743 162290000
```

## Limitations

Toy profile only. Source gaps unchanged. No runtime, proof, Uniswap/Curve/Liquity/Balancer/Velocity conformance, K, native, Preview, or sprint completion. Fresh independent GPT-6 Astra review is required.
