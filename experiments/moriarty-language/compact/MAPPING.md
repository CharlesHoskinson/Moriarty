# Restricted source-to-Compact kernel mapping

This experiment compiles both complete source examples into dynamic Compact kernels. The mapper parses and checks the original source, then traverses its typed Core. It does not select behavior by agreement/package name and does not embed a host-computed final state. The generated functions take every numeric state field, action argument, observation, remaining allowance, revision and arithmetic hint as inputs. They return every numeric state field, updated lifecycle counters and every source-order emitted operand.

Run with the retained Compact 0.31.1 compiler, language 0.23.0 and runtime 0.16.0:

```sh
node compact/materialize-mapping.mjs
python3 compact/verify-mapping.py --runtime-node-modules /path/to/node_modules --output /path/to/evidence
```

`generated/{loan,swap}/kernel.compact` contains source-derived pure transitions. Each arithmetic node calls the checked UInt128 helpers, including intermediate multiplication before division. An explicit hints struct contains six bounded multiplication limbs or bounded quotient/remainder plus those limbs. The advisory host helper calculates candidate hints. Generated circuits independently constrain the hints against dynamic operands. Mutating any supplied limb, quotient or remainder is covered by a rejection test.

Text uses a collision-free Uint32 index into the exact program-bound table sorted by UTF8 bytes. Every dynamic Text action argument must have a top-level equality guard against a Text literal or constant. An unknown external string cannot silently become a hash or another identity. Persistent Text, Text observations and `And`/`Or` are explicitly unsupported. Numeric-looking Text stays a Text-table entry. Other source-supported forms may be rejected by these explicit mapper restrictions; the source language is not reduced to this mapper subset.

Amounts and local quantities use Uint128 arithmetic in Compact. `metadata.json` retains every Core node's original type, unit vector and source reference, each generated expression line, argument/state/effect field mappings and the sorted Text table. `bound-program.json` retains the full policy, settlement, status and obligation-effect declarations. `programHash`, source hash, bounds hash and generated-source hash bind these companion artifacts. Policy FloorDiv provenance is checked deterministically during lowering, including source-order state writes and reads.

`harness.compact` is a test-only wrapper that writes a full kernel output into a public snapshot cell. This forces exported circuit/ZKIR generation and permits runtime atomic-rejection checks. A snapshot is not an asset transfer, custody record or financial settlement. Compilation always uses `--skip-zk`; no proving/verifying keys, proofs, network calls or wallet operations occur.

The six suites compare all four loan/swap transitions with source simulation, including every numeric state field and every typed effect operand. They also cover all arithmetic hints, overflow before division, minimum output, actor ID, input-state order, lifetime reserve/reset/expiry, unsupported Text/short-circuit forms, and source/unit metadata. Both snapshot harnesses compile ZKIR and reject malicious hints without changing the original runtime state.

This is a restricted Core execution kernel, not the complete stage-9–13 acceptance relation. It does not implement the semantic obligation ledger/tombstones, effect settlement quantum conversion, outcome authority, signature/oracle/genesis authentication, mandatory claims, PCD, durable state/nonce consumption or real ledger asset movement. The full source evaluator retains those candidate semantics. MC02–MC05 must bind compiled numeric inputs/results and companion metadata into the actual settlement/proof/history protocol. Runtime agreement on these finite cases is not full compiler-to-ledger correspondence or ACTUS/DeFi conformance. Independent audits and MC01 acceptance remain open.
