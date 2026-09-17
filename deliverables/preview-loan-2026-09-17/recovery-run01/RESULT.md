# September 17 fixed loan payment on Preview

The existing fixed LAM loan finalized all four stages and paid **533,972,602 test-asset units** to the lender. The [actual integration result](actual-run/integration-result.json) reports `FINANCIAL_COMPLETE` and `financialComparison.status=PASS`; [the process exited 0](service-terminal.txt). **Independent actual-result reviews: APPROVED** by [Astra](astra-result-review01.json) and [Grok](grok-result-review01.json), each independently checking native effects, canonical finality and contract state. Their [source](../recovery-review01/astra-review02.json) [approvals](../recovery-review01/grok-review02.json) preceded the [admitted continuation](admission.json).

Contract: `8034dffa6ce124cf34135799969d34831da5ce44fd287a3bead2fa3ce293a341`.

| Stage | Finalized block | Transaction ID | Native transaction hash |
| --- | --- | --- | --- |
| [deploy](actual-run/stage-deploy.json) | 908628 | `0074727c89072a920c3fe28bc0be689b0da73a7027fbf0ae734b6a62f743bb293a` | `ecd8468a01cf85896b2bc53f08c6621029659d31f127b6dc50c7f9eb71b9d400` |
| [initialize](actual-run/stage-initialize.json) | 908960 | `008bdeb7aa3b73c16456635508e733e05d2ef10b2a88bda65d080cae43c351e5bb` | `28f3d047303dfa9e1b0a895422198b78eb2ef2126da429e38f8414f47b378ab1` |
| [accrue](actual-run/stage-accrue.json) | 908964 | `00b7f8046179a9253098b99d15af33e097326383bd883932b57c566af3713e44c6` | `183781b21e8782ac2729ad61478582b5370ad75f0b88077400e60b61baddff6d` |
| [settle](actual-run/stage-settle.json) | 908968 | `005789261380cdcac5da16c0debb153fae39bd82dd0fc43a8907163acb4fa8968b` | `5088fd94ba903f53cd7e8ad58793b19a2bb36b0e46eb0f87a807fd5cdb2bf5be` |

Canonical block hashes retained in the stage receipts:

- Block 908628: `e3f51b9a3d1518980506e9cb5efbbb8487c8acc9989590628cfb5b7330a4f8a4`.
- Block 908960: `0761be8decf474137fcc6fe90cffdbb3e53e4848cfe3cf21f323cce50ebb5c01`.
- Block 908964: `3edab820a6212d31713bf5316ec8cc8fb0d9b4357b33be17506965f24e2a1d54`.
- Block 908968: `386db79ae34a51f21177bce5d0f1ddb576b978568b631656ae6cf2aa3ddb9a2f`.

Initialization minted **20,000,000,000** units of test asset `23b5e3c033240f04f65f9fcd2054a9d3a39d0f52094976a2b0736a69652a6199` to the borrower. Accrual recorded **500,000,000** principal due and **33,972,602** interest. Settlement consumed the borrower's full 20,000,000,000-unit UTXO, paid **533,972,602** to the lender and returned **19,466,027,398** change. Gross debit remains 20,000,000,000; change does not erase it. Contract holdings of this asset are zero. The due principal and interest are cleared, while residual principal remains **4,500,000,000**; the fixed call budget is exhausted.

Each finalized native transaction debited **300,000,000,000,001 DUST specks**, totaling **1,200,000,000,000,004**. The three new calls reserved **900,000,000,000,003** against their **1,399,999,999,999,998** cap. The original HTTP attempt and same-bytes WebSocket submission retain **600,000,000,000,002** conservative historical reservations, so cumulative charged reservations are **1,500,000,000,000,005** against the **2,000,000,000,000,000** overall ceiling. Indexer paid/estimated strings remain separate from native debit; their encoding relationship is unresolved.

The continuation ran from **13:00:15 to 13:01:48 MDT** on September 17: **93 seconds**, raw exit **0**, peak memory **567,410,688 bytes**, swap **0**. Admission allowed 8 GiB, zero swap, 256 tasks and a 30-minute deadline; dispatch retained 1,747 seconds of runtime. [Outer containment](outer-containment.json) confirms the exited process and absent cgroup after stop. The internal SDK cleanup remains `containmentComplete=false`; it was not rewritten to claim external containment.

The [failed HTTP run](../actual-run01/integration-result.json), [size-dependent HTTP failure control](../actual-run01/http-size-control01.json), [same-bytes deployment finality](../deployment-finality01.json) and prior review snapshots remain immutable. Recovery checked the unchanged constructor state, preserved the original private store and stopped reservation, and made **three new calls with no redeployment**. Public native bytes are retained for [deployment](../actual-run01/public-transactions/ecd8468a01cf85896b2bc53f08c6621029659d31f127b6dc50c7f9eb71b9d400.bin) and the [three continuation transactions](actual-run/public-transactions/). No keys, role secrets or private contract state are included.

This result demonstrates one payment in the existing fixed loan example. Mandatory PCD, the newer four-step language lifecycle, aggregate I2/SP05 acceptance and full ACTUS conformance remain open. Finalized-stage receipts remain `uncertified-I2-observation`; machine acceptance flags remain false pending their separate requirements.
