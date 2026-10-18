# Graph report: K8/K9 native consumer and wallet navigation fragment

This navigation-only fragment appends 24 source-attributed nodes and 26 links to the fixed R2 graph. It retains all 1,440 base node payloads and 3,704 ordered link payloads as exact prefixes. Preexisting metadata entries and values are unchanged; `fragment_updates` has one appended item with the complete previous list as an exact prefix. Whole metadata equality is false because of that append.

K8 records the unchanged `pvc2/groth16.verify_digest` local function result: the genuine fixture is accepted and the three named digest/proof tamper inputs are refused. Its earlier outer wrapper remains `FAILED` because the expected failure metadata did not match the recorded helper mapping. A later ordinary telemetry correction closes metadata validation only and preserves the historical wrapper result.

K9 records the unchanged typed `pvc2/claim.issue` local function result: one coherent positive and six negative guard cases. These rows are local test observations, not a transaction or ledger acceptance. Both functions reuse the existing public K6 proof fixture.

A separately adopted fresh wallet is ready for a human to report its public address for Preprod test ADA funding: `addr_test1vzt2n53u2vlr8ggxz8xan6ywrw8m55umfwvrgmzh3wmcmkcfxunjc`. The public adoption limits this to funding-address readiness; funding, ledger observation, signing and settlement are not claimed. No private key material was included in or reviewed from the source set; only operator-reported existence/mode booleans were reviewed.

The capsule keeps Aiken VM resource values separate from ledger ExUnits, transaction fit, and transitive native CPU. A compiled proof-applied wrapper with a genuine hash, authenticated context, transaction economics, qualified costs and real Preprod evidence remain open. Broader UTXO, ACTUS, DeFi, general financial, recursion, private-handoff, bounded multi-parent PCD and universal-certificate obligations also remain open.

Graphify 0.9.91 generated the HTML locally. Primary package metadata recorded 0.9.91 as latest at retrieval time. The visualization groups inherited nodes without a numeric community under a display-only group; JSON payloads are unchanged. HTML may retain its documented CDN dependency. Runtime token accounting is unknown/unavailable. See the accompanying JSON files for source pins, hashes, read coverage and additions.
