# Cumulative-prefix observation adapter increment

UNAPPLIED / UNRUN. Exact root-formatted base de1eaf81eec57c7cec48c2ddae57362ff2c7d498, tree 1ddfc13fd8195fa97231a7f8106241d4a37c0e48. Full before/candidate Rust source and one-hunk candidate.patch accompany SOURCE-PINS.json.

Root's old batch exited 101. Both reserve tests reached the third cumulative-prefix assertion at line 72; the complete financial projection and source-kind assertions precede that point. Payout tests were not reached. Original logs, captures and root diagnosis are preserved unchanged under captured-before/ with full original/copy hashes. No old result is rewritten as a pass.

Root cause: the authored adapter returned an array of effect arrays, while the frozen oracle specifies an array of indexed {index,effects} records. The original author checked the prefix count without checking each record's schema. First exact JSON difference is /0: actual [] is an array; expected {"effects":[],"index":0} is an object. This is an observation adapter error, not an identified financial-value or producer error.

The minimal correction replaces Value::Array(prefix.clone()) with json! containing the actual step["index"] and the complete prefix.clone(). It retains every effect, index, independent expected field and existing assertion. It does not enumerate replacement indices, strip the expected wrapper, recalculate financial values, change production semantics or alter any oracle.

Offline validation over the retained old public CLI JSON confirms the full financial projection and source-kind list; all 20 old complete effect arrays already equal expected[i].effects. The corrected indexed records equal all 20 unchanged expected records. DATA-DIAGNOSIS.json and validate-shape-only.py preserve that mechanical comparison. captured-adapter-before.json and captured-adapter-after.json are diagnostic representations of the old capture, not expected goldens or a new runtime result.

All 25 original oracle payload checksums and terminal FREEZE 1b606141dff7ca162bcd132d55d62014187de4f936cfc6528c23c8d22591aefb remain valid. The three current root Git fixtures are byte-identical to those originals; their hashes are recorded in DATA-DIAGNOSIS.json. Expected bytes and original source/failure packets remain unchanged. oracle-reference/ contains exact copies for review.

Patch SHA256 527ba1a5a9065a09343753bd72f929e88fdf52fcb7d036dc1e1cd1d39a18b37d. Before source SHA256 852709b2118040ba2ca5e380ed09500107db0f2da156d8655a5df5da7cb02ae4; after source SHA256 27c502d178d7422b2c53f03e1b294cdb792dfc719aac1e772ca2cf405b514b90. Before Git blob 6a875a5ff1f00a8fa1aea1df31f5c1b32a1b1f67; prospective after blob 69b68a798d61a977517084410a9cffe9434a3a87.

Root owns integration and a newly recorded rerun on the changed tree. A bounded source command remains cargo test -p mori --no-default-features --locked --test core_reserve_spend_refund -- --nocapture, with a new capture directory if supplied. This author ran pure JSON/hash/source validation only: no Rust build/test/formatter, Node, SDK/native, provider or shared/Git mutation. No full Program/ExprId, work, byte-golden, allowance-enforcement, Target/Artifact, security/native or parent acceptance scope is narrowed or closed.
