# Zhou et al. (2023), SoK: Decentralized Finance (DeFi) Attacks

Source: [SRC-0102 full PDF](../../.raw/captured/863c0ee080bac271b9ff1e3a0f1e036759cf71baf9c4b1d3dd38b963cbde4979.pdf). IEEE Symposium on Security and Privacy 2023; supplied DOI 10.1109/SP46215.2023.10179435 appears in the PDF's publisher side banner. The typeset first-page footer also prints 10.1109/SP46215.2023.00180: preserve this discrepancy in provenance; this analysis does not resolve it externally. Authors: Liyi Zhou, Xihan Xiong, Jens Ernstberger, Stefanos Chaliasos, Zhipeng Wang, Ye Wang, Kaihua Qin, Roger Wattenhofer, Dawn Song, Arthur Gervais.

Coverage: all 18 PDF pages read, including Appendix A/B and references. PDF page 1 is printed page 2444; every locator below uses **1-based PDF pages**. Visually inspected Figure 2 (PDF p. 2), Tables I–II (p. 4), and the full-page rotated Table III (p. 6). No experiment, audit, exploit, tracing result, or statistical estimate was reproduced. Historical empirical figures are source reports about the sampled period, not current prevalence estimates.

## Source taxonomy and separate axes

This paper's main contribution is a five-layer system model plus a separate adversary model and incident taxonomy. It does not propose an operation grammar or a compiler. Figure 2 has four vertically stacked components and an AUX service column beside them; treating AUX as merely a fifth level above PRO loses the diagram's cross-layer role. [§II-A, PDF pp. 2–4, Fig. 2]

| System layer | Objects/mechanisms in the source | Representative cause families in Table III |
|---|---|---|
| Network (NET) | TCP/IP/DNS/BGP, peer discovery/churn, message propagation, private front-running services | Transparency, improper discovery/churn, congestion, exposed Internet services |
| Consensus (CON) | Leader election, consensus, sequencers, rewards/fees/MEV | Blockchain protocol flaws, unstable incentives, unfair sequencing |
| Smart contract (SC) | Transactions, state, state-transition function, executable contracts, block transition | State-transition design mistakes; unsafe calls; coding mistakes; access-control mistakes |
| Protocol design (PRO) | Token standards and semantic variants; financial protocols; protocol incentives; atomic composition | Transaction-order dependency; replayable design; block-state dependency; permissionless interaction; unsafe dependency; unfair/unsafe interaction |
| Auxiliary (AUX) | Front ends, developers, privileged operators, wallets, off-chain oracles | Faulty web development/operation; off-chain oracle manipulation; greedy operators; faulty blockchain service providers |

Sources: §II-A, PDF pp. 2–4; Table III, PDF p. 6. Table III also includes residual “other” categories at every layer. The source marks some SC elements (block-state transition and L2 incentives) as nongeneric examples rather than universal properties. [§II-A(iii), PDF p. 3]

The *incident* is a sequence of actions causing unexpected financial loss to users, liquidity providers, speculators, or operators. Attacks involve an adversary attempting to disable, delay, or alter expected transitions; accidents need not involve a proactive adversary. This is an outcome-anchored definition, distinct from discovering an unexploited vulnerability. The adversary is assumed rational relative to monetary U1 or nonmonetary U2 utility, including reputation/accomplishment and white-hat loss minimization. [§II-B(i–ii), PDF p. 4]

Knowledge has three named classes: K1 public data and public analysis; K2 sequencer access to private pending transactions, order policy, and early block state; K3 insider information such as oracle updates, external prices, or wallet credentials. These are different information sets, not a simple total hierarchy. Table I associates capabilities with the knowledge sufficient for them, and §II-B(iv) stresses that the same capability can come from different knowledge classes: public competition can front-run, while a sequencer directly chooses order. [Tables I–II, PDF p. 4; §II-B(iii–iv), PDF p. 5]

Table I's capabilities are: network-provider control, message manipulation/censorship/delay, private FaaS submission; chain forking, mempool censorship, sequencing/front-/back-running; off-chain simulation; mixing, flash borrowing, cross-protocol composition, single-transaction atomic execution, ABI-mimicking contracts; external-oracle manipulation and wallet compromise. The auxiliary capability notation is C_3RD in that table. These are powers available to an actor, not automatically bugs or exploits. In particular flash loans and composition are ordinary mechanisms with legitimate uses. [Table I, PDF p. 4; §II opening, PDF p. 2]

## Cause/type hierarchy worth retaining

At SC, unsafe external calls include direct untrusted calls, reentrancy, and delegatecall/call injection. Coding errors include mishandled exceptions, locked/frozen assets, overflow/underflow, missing logic/sanity checks, short address, casts, costly/unbounded operations, and arithmetic mistakes. Access-control errors include inconsistent authorization and visibility/unrestricted actions. The labels mix mechanism, defect, and result: “locked asset” is observable harm while “arithmetic mistake” is a cause. Preserve the source labels while giving each a separate normalized role in any derived model. [Table III, PDF p. 6; final sentence is analyst inference]

At PRO, Table III organizes the following:

- Transaction-order dependency → front-running, back-running, sandwiching, other order dependencies.
- Replayable design → transaction/strategy replay.
- Block-state dependency → randomness and other block-state dependencies.
- Permissionless interaction → token and non-token contract camouflage.
- Unsafe dependency → on-chain oracle manipulation, governance attacks, token standard incompatibility, liquidity borrowing/purchasing/minting/deposit, phantom-function calls, other protocol dependencies.
- Unfair/unsafe interaction → slippage protection, liquidity provision, unsafe/infinite token approval, other interactions.

All locators: Table III, PDF p. 6. These headings should not be flattened into a set of “DeFi primitives”: some are unsafe conditions, others attack tactics, and some are ordinary actions in the context of a faulty assumption. [Analyst inference]

At AUX, oracle failure is split into malicious updater, malicious source, and external-market manipulation. Greedy-operator categories cover backdoors/honeypots, insider actions, phishing, authority misuse/broken promises; faulty operations include compromised credentials and deployment mistakes. This keeps code behavior, external service integrity, and human authority in view. [Table III, PDF p. 6; §II-A(v), PDF p. 4]

## Composability, oracles, governance, and MEV

The paper explicitly identifies a semantic contract problem: a backward-compatible token interface can violate existing security assumptions; ERC-777 hooks illustrate this. Its 19 permissionless-interaction incidents combine incompatible token standards and camouflaged token/non-token contracts. The victims checked ABI shape without constraining implementation semantics. The paper proposes whitelisting as an alternative and says efficient general on-chain implementation verification is unavailable within the discussed SC design. [§II-A(iv), PDF p. 3; §VI insight 4 and footnote 25, PDF p. 12]

On-chain oracle manipulation belongs to PRO unsafe dependencies and is called a composability attack; the source reports 28 such incidents (15%). Off-chain oracle manipulation is separately categorized at AUX. It identifies DeFiRanger and DeFiPoser as then-known tools with different limits: observed-transaction detection versus costly manual protocol modeling. Path explosion from composition complicates general defenses. [Table III, PDF p. 6; §VI challenges/insights 2–4, PDF p. 12]

Governance appears in three distinct roles: legitimate protocol incentives, malicious governance as PRO unsafe dependency, and privileged human/operator controls in AUX. Emergency-pause authority is also AUX operational power even when code implements it. Do not equate token voting with all governance or classify every governance action as an attack. [§II-A(iv–v), PDF pp. 3–4; Table III, PDF p. 6; analyst normalization]

MEV is a CON incentive component; sequencer ordering is a CON capability, order-sensitive protocol behavior is a PRO cause, and private FaaS transport is NET. The same strategy can traverse all three. FaaS changes who can see a pending attack: §V-C says only actors with K2 can react before confirmation to private adversarial transactions. A monitor that only watches a public mempool has a materially smaller observation model. [§II-A(i–ii), PDF pp. 2–3; Table I, p. 4; §V-C, pp. 10–11]

## Evidence and limitations

The reported corpus is 77 academic papers, 30 audit reports, and 181 incidents over 30 April 2018–30 April 2022 on Ethereum/BSC. Incidents can carry multiple causes/types and involve multiple layers. Academic selection is scoped to eight conferences plus citation following and exclusions; audit coverage means explicit mention of a risk or passing check, not proof of all auditing work performed. The paper explicitly acknowledges manual-label error, limited chains/public disclosure, and inherited sampling bias. [§III, PDF pp. 5, 7]

Useful historical observations: 42%/40%/30% of incidents involve SC/PRO/AUX; the proportions overlap. Table IX says 15/29 tools relate to PRO (52%), while the average PRO incident-type coverage of each tool is 6%. §VI challenge 2 prose blurs these measures into “cover 52% ... on average”; use Table IX's own defined columns and example, not that paraphrase. Table III also shows missing logic/sanity checks in 44 cases (24%), so the abstract's “two most frequent incident types” description of oracle/permissionless figures must not be generalized across all rows. [§IV-A, PDF p. 7; Table III, p. 6; Table IX and §VI, p. 12]

The source reports a rescue window between adversarial deployment and first malicious transition and an incident window between first and last harmful transitions. It reports 103 (56%) non-atomic attacks with a rescue opportunity. This does not establish a reliable available defense: deployment can be bundled with execution, windows vary, private transport hides observations, and detection/response takes time. Emergency-pause support and prompt execution differ: 87/183 victims support it, 51 pause within 48 hours, one within an hour. Table VI's denominator wording is internally inconsistent (“out of 87” versus percentages of 51). [§IV-D, §V-A, Fig. 6/Table VI, PDF p. 9; Fig. 7, p. 10; analyst inference on guarantee boundary]

Bytecode comparison reports 31 vulnerable and 23 adversarial exact-match contracts potentially detectable by comparison with known examples, not an independently demonstrated deployed preventive system. Similarity is limited by compiler/optimization changes and obfuscation. The audit association (4.09% audited versus 15.49% unaudited exploited in the additional matched sample) is observational and described as a rough approximation. SEM does not find strong evidence preventive defense reduces harm in the selected incident sample; this is not a proof audits are useless or a causal randomized intervention result. These analyses answer different questions and have selection/confounding limits. [§IV-C/E, PDF pp. 8–9; §V-B and Table VII, p. 10; causal caution is analyst critique]

Money tracing identifies funding flows and possible account links, with relayer handling, explorer-label incompleteness, and explicitly weaker identification for indirect withdrawals. Linked addresses are not demonstrated real-world identities. Do not import trace counts as verified attribution. [§V-D, Algorithm 1/Table VIII/Fig. 8, PDF p. 11; Appendix A/Table X, p. 14]

The paper cites Werner et al. as reference [14] (arXiv:2101.08778, 2021 citation form) and distinguishes its own network/consensus coverage and harm measures from Werner's technical/economic split. This is explicit intellectual dependence, not independent corroboration for all Werner claims. [§VII, PDF p. 13; references [14], PDF p. 14]

## Language design implications — analyst inference, not paper claims

| Candidate language feature | Source-grounded motivation | Boundary of defensible guarantee |
|---|---|---|
| Resource/amount types, unit/scale distinctions, explicit rounding, checked arithmetic | Table III SC arithmetic and coding causes, PDF p. 6 | Compiler can reject specified misuse; financial valuation or conservation requires a declared invariant and verified implementation |
| Typed capabilities and authority scopes | Table I powers, Table III access control, AUX operators, PDF pp. 4, 6 | Static authority discipline and runtime authorization can constrain declared actions; stolen keys and governance collusion remain external threats |
| Semantic adapter contracts and callback effects | ERC-777 warning and ABI mismatch, PDF pp. 3, 12 | ABI typing alone proves shape; adapter behavior needs implementation-specific evidence, upgrade/version binding, and runtime checks where required |
| Explicit oracle observations with source, time/block, unit, tolerance, validity policy | On-chain versus off-chain oracle classification, PDF pp. 6, 12 | Compiler can track provenance and prohibit inadmissible data use; runtime can check freshness/bounds; truthful external prices remain assumptions |
| Atomic execution scopes plus scheduler/environment assumptions | SC atomicity, FaaS bundle distinction, ordering powers, PDF pp. 2–4 | A compiler cannot create chain-wide atomicity or ordering rights; bundle delivery/finality and cross-ledger execution need their own evidence |
| Protocol-specific accounting/postconditions and bounded composition verification | Unsafe dependencies/path explosion, PDF pp. 6, 12 | A checked finite composition establishes its stated property under modeled contracts/state; arbitrary open-world composition is not covered |
| Monitor/pause/recovery policies outside ordinary financial action types | Rescue/incident windows and pause delays, PDF pp. 9–12 | Runtime detectors need observable signals and actors able to respond; code compilation does not guarantee timely detection or recovery |

Recommended normalization for a design crosswalk is a record with separate fields for service family, affected layer, root cause, actor/knowledge/capability, ordered action trace, property violated, observed outcome, and defense/assumption boundary. This is an analyst synthesis of the paper's axes, not the source's own formal grammar. No claim here establishes Moriarty, Midnight, a compiler, or any implementation as secure.


---

Library identity: SRC-0102. Authority: primary descriptive research. Version/date: 2023. Reviewed 2026-09-08. Source transcription confidence high; language inference provisional, S2, not implemented or reproduced. [Source record](sources.json) · [Design comparison](DESIGN-IMPLICATIONS.md) · [Library](README.md).
