# Financial ledger effects successor01

Independent R6 comparison result: **CHANGES_REQUIRED**. SP05 remains **BLOCKED**.

Candidate `e10f46574dcb04b07732eebd9f33ec34b1bdaa30af0a483def7ea1deccad0cee` repairs the reproduced unshielded comparison omissions. Root verification passed 33 supplied tests, seven retained effect checks, three native SDK checks and three actual recipe-shape checks. Fresh GPT-6 independently passed the 33 supplied tests and checked 38 native cases.

The complete-effects claim is rejected. Nine altered transaction-level Zswap/network cases still pass. Unshielded input/output/signature mutations, TTL and collisions reject; unchanged and native disjoint-merge controls pass. See [the independent review](gpt6-result-review.md). Author reports are retained as candidate claims, not accepted conclusions.

A separate root regression exercises `createFinancialProviders().fundUnshielded` with native signed synthetic UTXOs and inert local adapters. The allowed control reaches only a throwing local finalization stub. Signed input100 with cap50 and declared gross0 incorrectly reaches the same boundary. Base100 plus balancing200 with cap200 also reaches it. Both expected gross-admission rejections fail. See [the retained root observation](root-gross-admission.json). The fresh review explicitly excluded this accounting portion.

The next existing source pass, `successor-effects-02`, addresses the exact transaction-level and gross-admission counterexamples. It must preserve valid supported transformations and reserve actual signed gross debit and actual submission count. All other original R1-R5/R7-R8 findings, full language/formal deliverables, native proofs and financial Preview acceptance remain open.

All tests here are offline, use pre-proof/pre-binding objects and synthetic UTXOs, and establish no valid finalized transaction or submission exploit. No wallet, native finalization, proof, network call or blockchain transaction was performed.

Original state: `/home/charl/.local/state/moriarty/sp05-ledger-integration-20260908/successor-effects-01`. Independent probes import the sibling frozen candidate. The supplied suite needs the pinned original worktree dependencies. The candidate freeze and review name all source hashes and scope limits.
