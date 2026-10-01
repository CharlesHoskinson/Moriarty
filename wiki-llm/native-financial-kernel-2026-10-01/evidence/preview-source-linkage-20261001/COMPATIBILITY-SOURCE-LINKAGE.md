# Official Preview version source linkage

2026-10-01. Independent bounded public source inspection; no candidate execution or acceptance. Prior same-block RPC/indexer evidence and PREVIEW-COMPATIBILITY-20261001.md remain immutable. Develop/status refreshed and wiki/index queried first. No other audit reports were read.

## Concrete source mapping

Source fact: official GitHub REST matching-refs returns ledger-8.1.2 at commit `36d7442172136758e33009f75ff4aa56616cff40` in midnightntwrk/midnight-ledger. Its Cargo.toml:4 declares workspace package.version8.1.2. Files were fetched by full commit, not a moving branch.

Source fact: official midnightntwrk/midnight-node tag runtime-1.0.300 resolves to `c3895bd2bd930b03ad53a4c8de0d98a490b8bfcb`. At that exact commit runtime/src/lib.rs:284,287 declares spec_version001_000_300 and transaction_version3. Root Cargo.toml:79-82 declares midnight-ledger =8.1.2, midnight-onchain-runtime =3.1.1 and zswap =8.1.2 for the ledger8 aliases. ledger/src/lib.rs:55-77 builds ledger_8 from those aliases and exports ledger_8 as latest; lines95-96 export its active bridge/types. This source version tuple matches the live sampled ledger=8.1.2/spec1000300/transaction3 tuple in the prior report. This is an explicit release-tag/source match, not a binary attestation of the live node.

GitHub release lookup for runtime-1.0.300 returned404. The first node releases listing timed out and was not retried. No release asset/checksum was acquired. The existing tag, source and exact dependency tuple suffice for a conditional source capability determination below.

## Actual supported and unsupported source predicates

| Surface | Official ledger8.1.2 source predicate | Frozen candidate requirement | Consequence at this source |
| --- | --- | --- | --- |
| Proof encoding | ledger/src/structure.rs:289-290 enum has only ProofVersioned::V2; deserialize:309-328 accepts discriminator1 and rejects other values | Candidate ledger-v9 source has ProofVersioned::V3 with discriminator2 | V3 encoding is rejected by the published8.1.2 proof deserializer |
| Contract operation version | structure.rs:2521-2568 enum has only V3, encoding discriminator2; unknown discriminator rejected | Candidate V3proof maps to ContractOperationVersionV4, discriminator3 | V4 operation encoding is unsupported by the published8.1.2 deserializer |
| IR loading | zkir/src/ir.rs:390-406 IrSource::load accepts only major2/minor0 and returns Unhandled version otherwise | Exact pay.zkir header is major3/minor1 | This release's ZKIR source loader rejects IR3.1 |
| Native runtime dependency | ledger/Cargo.toml:23 selects midnight-onchain-runtime3.1.1; transient-crypto2.1.1 at26; optional test-utility zkir_v2 package2.1.1 at31 | Candidate ledger-v9 uses later proof/runtime API and generated JS requires Compact runtime0.20.0 | Distinct APIs require compatibility evidence; package versions alone are not proof of a particular transcript refusal |

The ZKIR loader finding is a compiler/prover source predicate, not an assertion that chain consensus loads JSON IR. Consensus compatibility here is constrained by actual proof and contract operation serialization support. RPC API2 and Substrate transactionVersion3 have no inferred correspondence to ZKIR major2/3. Likewise ContractOperationVersionV3 in ledger8.1.2 is NOT V3proof support: that source represents a V2proof and the older verifier key through its V3 operation.

The current candidate's source requirements are frozen in previous report: pinned midnight-ledger-v9 commit9a8777c4d035fc7f38ae286bcf5f8656668efd9f, IR3.1, generated Compact runtime0.20.0, real V3 proof/V4 operation path. This task changed none of those inputs. Future successful local preparation/proof cannot substitute a chain format match.

## What remains missing

Inference: the matching official runtime/ledger source tuple substantially explains the live compatibility blocker. The published8.1.2 code cannot decode the candidate V3proof/V4operation formats; consequently the candidate is incompatible if the sampled Preview execution follows this published runtime/ledger tuple. This goes beyond a package-number mismatch but remains conditional on the node/source linkage.

Missing: runtime WASM/code-hash attestation connecting the live finalized state to the exact published source/build, any official release artifact hash usable for that comparison, and a chain validation result for the candidate. No huge runtime/body/binary download occurred. A runtime hash alone without an independently published matching artifact would not close source linkage, so no additional runtime storage query was spent here. No wallet/proof/transaction was requested to test rejection. This report does not assert an observed network rejection, authenticate custody/registry/funding/time, or establish financial acceptance.

Actionable boundary: retain current native experiment as ledger-v9 conditional trusted-genesis evidence. A future Preview delivery needs a reviewed compatible encoding/profile decision or evidence that Preview has upgraded to compatible V3proof/V4operation support. Published ledger8.1.2's IR2.0 loader and V2proof/V3operation source define a concrete potential compatibility target; this task does not authorize adopting it, recompiling Compact, refactoring, or migrating. Four Compact attempts remain exhausted.

## Acquisition and provenance

All new acquisitions used public GitHub structured REST GET against official midnightntwrk repositories, no credentials/auth,20second timeout,2MiB per-response bound, no retries, no native/compiled artifact execution. URL, UTC retrieval timestamp, HTTP/error status, response byte count and SHA256 are recorded in immutable *.receipt.json. GitHub contents responses retain base64 source; decoded *.source files are separately pinned in MANIFEST.json. These receipts establish retrieval and exact bytes, not correctness or live node deployment provenance.

Caps:64 captures/32MiB total, one bounded source pass. Acquisitions used17 captures and 698776bytes total staging at report time. Missing-path404 and initial timeout are preserved. No Scrapling article acquisition was needed: all remote work was structured official repository access. Candidate/handoff/caller/resource bytes, canonical wiki/source ledger, wallet/state and prior live receipts were untouched. No cargo/node/nativecheck/SRS/keys/proof/well_formed/apply/network transaction or fifth Compact compile ran.
