# Sherlock stagenet deployment

Sherlock's terminal verification/settlement contract was deployed to Midnight ledger-9 stagenet on 2026-10-23. The transaction finalized at block **993565**. The selected runtime checked canonical inclusion, exact submitted bytes, complete contract-state/effect readback, fee bounds, wallet close and owned-process cleanup. Two independent actual-result reviews supported the deployment adoption.

| Public identity | Value |
| --- | --- |
| Contract | `80a0291cc688bc627b5dfef1a18d8e02ec5ed2f15343e60f18ee61e58a955371` |
| Transaction identifier | `0004799b402e5a83823d40bc6502078ee375ba6f653091c8520d9eabcb45baa283` |
| Transaction hash | `84566edeff7a1ea58b2e61cbd1ad8213cc02af1cf5700b4e62fc2d34b46592e2` |
| Finalized block hash | `2ea9a8230f7b0b3eb1d3e4b4fc92a2b62adea46aedf0315410b3a2b6543eb2f8` |
| Actual fee | `993524550460293` SPECK |

The SDK deployment pipeline completed in **82.6504 s**. The Root capture took **147.4486 s**; a conservative complete-process interval, including final cleanup and exit, was **149.0304 s**. These are nested timing scopes and must not be added. The pre-controller-exit service-cgroup sample recorded a peak counter of **1,349,775,360 bytes**, cumulative CPU usage of **59.408454 s**, and zero OOM events. These counters cover the service through that sample; they are not a final aggregate peak or isolated proof measurements.

The fee provider produced one DUST-spend proof and requested SRS parameters of degree 13. That is a proof count and requested parameter degree, not a payment of one DUST or a measured contract-circuit k. The accompanying [sample](../../deliverables/sherlock-stagenet-2026-10-23/samples/sdk06-deployment-baseline.json) and [public-safe receipt](../../deliverables/sherlock-stagenet-2026-10-23/receipts/sdk06-deployment-baseline.json) keep unmeasured values explicit.

This deployment has zero physical contract calls. It does not yet demonstrate the genuine recursive terminal-D proof call, altered-proof/replay refusals, the synthetic 16-transaction Fenrir rollup, or the finished universal interpreter. Those remain required. Deployment/readback closes one item in the first experiment: **5/10** accepted; the synthetic-rollup experiment remains **1/9**, and the full interpreter remains **0/6**. These are acceptance checklists, not implementation percentages.

The SDK correction compares serialized **TransactionCostModel** values while preserving owned-resource cleanup. The installed **CostModel** has no serialization method. The [exact policy and ownership helpers](../../tools/sherlock-sdk/producer-policy.mjs) are published with the [13 focused tests](../../tools/sherlock-sdk/model-api.test.mjs). Those source bytes passed all 13 tests and a separate real installed-SDK public-default smoke before this deployment. These support files do not add a public Moriarty CLI command.

Run the focused tests locally with Node 24.21.0, the version used for the recorded checks:

```sh
node --test tools/sherlock-sdk/model-api.test.mjs
```

Public provenance: source freeze `3f34e40ed63d888541cf894fb892cdb476e08ba6422fde4003d7b9fece2f3d09`; configuration `9e467975878fec787c8da507407ab75d86c779b913bf7fffcee82e16387c85c2`; Root deployment adoption `b1c5a10f7531e16beb64b4310ab7d05493874acd01815a05b732a76b108df14b`; independent actual reviews `a0938a07daadef3e655f1fbb37cc4cab799c2b5d3f4724b4e73b23ea858595d4` and `9d3801c164844f85ec3fbd8bd3699a60416b3badfcb0692c56f230d66c655a4d`. Hashes preserve local lineage; they do not authenticate the producer by themselves. Wallet material, signed transaction bodies, proof/key/SRS payloads and private runtime files are excluded.
