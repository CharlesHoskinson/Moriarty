# Required independent compiler oracle interface

This is a proposed test domain and fixture interface for review, not an adopted compiler result. The two predicates call the public unsigned `mori_backend::compile` entry and explicitly reopen every emitted action with public `midnight_zkir_v3::IrSource::load`. The compiler already performs its own IR serialization/reopen internally. The proposal does not call a host evaluator, IR builder, VM, proof or ledger interface.

## Exact shared inputs

Use the existing files directly; do not replace, simplify, reformat or copy them into another program:

| Shared path under `crates/fixtures/financial-pair/` | SHA256 |
| --- | --- |
| `installment-invoice.mori` | `2f07d17b3aa9634819f65acbc64945af29588b1258d196fe30f55b8a377292c3` |
| `conditional-reserve.mori` | `7cb7c96bae56f344467deb5ac03fe8157fce356527eae608f68dbeb10ac71fa1` |

Each source compiles as a whole: invoice has register/collect; reserve has register/allocate. The test does not select just the financially interesting action. The three frozen host Scenarios, complete outcomes and 39 prefixes retain their separate Core test consumer and original seven-payload hashes. The original v2 freeze digest remains `7e5b09d844861f71ebaf04f56f011ec520d9defb2f0ae87b1ee6e626cea77f5e`; this proposal neither rewrites nor re-freezes it.

## Missing executable fixture files

All four paths below are required by `include_str!` and are currently absent. None is supplied with this proposal. The test must remain unadopted until an independent owner supplies and freezes complete fixtures and the domain is reviewed. No ignored test, empty expected file, partial comparison or candidate-driven regeneration substitutes for them.

| Required path under `crates/fixtures/compiler-financial-pair/` | Meaning |
| --- | --- |
| `invoice/target.json` | Complete exact invoice test Target input |
| `invoice/artifact.json` | Complete independent expected invoice Artifact |
| `reserve/target.json` | Complete exact reserve test Target input |
| `reserve/artifact.json` | Complete independent expected reserve Artifact |

A fixture owner should also provide its immutable inventory, derivation and provenance README in that shared directory. These are custody metadata, not extra compiler inputs. The author of this test has inspected the compiler and is not the independent oracle owner. Independent provenance must name actual sources and derivation; do not assert blinding after source exposure.

## Proposed exact Target inputs

Both test Target files would contain the following complete members, with only anchor differing. `signed_policy` is omitted, not null. These are proposed ordinary test-domain inputs copied from the existing `Target::ledger9` input convention, not selected deployments, generated IDs, measured costs or a resource decision. They require independent domain review before adoption.

```json
{
  "profile": "ledger-9",
  "network": "local",
  "anchor": "financial-invoice-compiler-test",
  "ledger_revision": "04c9c5d9bcebb8d4427d8589fb54d58a55599c14",
  "zkir_version": "3.0.0-rc.2",
  "max_instructions": 100000,
  "max_artifact_bytes": 16777216
}
```

For reserve, the anchor would be exactly `financial-reserve-compiler-test`. No other member changes. These literal bounds match the existing constructor defaults; no assertion about adequacy or native feasibility follows. The full expected Artifact must repeat the exact selected Target and unsigned mode.

StaticBindings/Deployment choice is **none: not consumed by this backend API**. This does not choose missing roles, asset bindings, payer, addresses or deployment identity for the separate `mori-ledger` compile-call path. That path is outside these predicates and needs its own inputs and expectations. Neither anchor is a Midnight deployment or a claim about Preview/Fenrir.

## Complete expected Artifact and IR

`artifact.json` is one ordinary JSON object in the public Artifact serialization shape. The comparison has no projection, field filtering, placeholder replacement, sorting of arrays, digest masking or tolerance for omitted members. Include every format/ABI/relation/mode/program-hash/semantics/Target member; every action in source order; every circuit member; every Resources field; and every Gate entry/status. Keep explicit null conventions and empty arrays exactly as specified independently.

For each circuit include action, full ZKIR string, context hash, all positional public bindings (role/name/type/constant/asset), authorization/private bindings, instruction count, complete ordered effect frame, complete balance frame, complete liability frame and replay steps. Every effect descriptor includes all identity/operation/null fields and all public positions. Every balance/duty descriptor includes exact asset identity, parties and every pre/post position. No digest, constant, type, instruction, resource number or context field may be inferred from the actual result under test and inserted into expected data.

Whole `serde_json::Value` equality compares the complete artifact. Since ZKIR is a string member, it also compares that embedded serialization byte-for-byte as UTF-8 content. The test then loads each actual ZKIR string through the real SDK loader and compares the complete typed `IrSource` to the loader's decoding of the independent expected string. Decoding expected text does not compile a program or generate an expected relation. No instruction-subset assertion replaces that complete comparison. The outer artifact's whitespace/member order is not a raw-byte assertion; no CLI writer or process is involved.

There is currently no independent complete financial compiler Artifact/IR oracle. Program/context digests, every instruction and constant, complete public positions/counts and exact resource/serialization fields are not filled in by this author. Independently specified structural metadata alone cannot satisfy a complete Artifact oracle. If independent complete IR/digest/serialization derivation is unavailable, that is a blocker for these predicates; do not weaken them or treat a compiler-generated baseline as independent evidence. Any differently sourced baseline would require its own explicitly reviewed purpose/provenance and would not inherit this claim.

## Smallest complete frame-mapping descriptor

In addition to the four executable inputs, propose two independent mapping documents at `invoice/frame-mapping.json` and `reserve/frame-mapping.json`. They are specification prerequisites for later correspondence work, **not read or verified by this test source**. Keeping them separate avoids inventing an actual compiled financial execution that this API does not return.

Use one record per existing literal host case, with source/action/case/scenario identifiers and references to its complete pre/post/effects literals. Retain host row/effect order. The minimal mapping covers:

| Host observation | Required explicit mapping |
| --- | --- |
| Every state member | Name/type and pre/post public positions, including unchanged marker; reference complete literal values |
| Every balance row | Full asset/account identity and pre/post literal row references; exact balance-frame positions when represented, otherwise explicit `outside_balance_frame` with no fabricated positions |
| Every source-registry duty | Name/asset/debtor/creditor and pre/post presence/amount positions; literal presence/absence and amount references, including residual/unchanged duties |
| Every ordered Core effect | Original statement index and effect index, exact Core identity/kind, matching compiler descriptor kind/operation/positions; no netted or reordered replacement |
| Every input/action | Declared type/order and exact public position; include the uninvoked register action in the structural source-wide descriptor |

Invoice collect represents CASH payer/seller/treasury; observer 23 and reserve 19 are host-preserved rows outside its balance frame. Reserve allocate represents BOND auditor/reserve and CASH reserve/supplier/treasury; observer BOND 7 and CASH 11 are host-preserved rows outside its balance frame. Register has no Transfer/Fee balance endpoints. Duty registries remain source-wide: invoice/warranty and allocation/bond/emergency. These are source observations to check independently, not a supplied oracle or runtime proof of preservation. Complete expected Artifact equality checks all emitted frames, but it does not execute inputs 47/80/50/7, observe a compiled balance amount, prove outside-frame preservation or link the two invoice executions.

## Predicates and remaining acceptance

`installment_invoice_matches_complete_compiler_oracle` and `conditional_reserve_matches_complete_compiler_oracle` are the two callable predicates. Successful future observations would establish only exact complete compiler output equality and SDK reopen equality for these exact source/domain/expectation triples. No test result exists now.

Core full-snapshot/prefix/work comparisons remain separate, including their existing Step.value/raw ExprId/per-prefix measured-work gaps. This source proposal does not close UNI-005/007/018/019/020/021 or EARS/U2, and changes no U0–U7/M1–M17/MC/SP/G or full financial/composition exit. Compiler correctness, native/proof/ledger behavior, durable conditional settlement, recursive/private composition and release qualification require their retained evidence and owners.
