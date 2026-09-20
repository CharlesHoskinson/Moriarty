# Kernel explanation technical review

Reviewer: fresh GPT-6 Astra agent, scoped read-only review. Date: 2026-09-19 workspace time.

Result: two bounded prose corrections requested. No architecture expansion or repeated implementation-status caveats are requested. The final implementation section provides the appropriate status boundary.

## Findings

1. **Scope atomic rejection to the fixture; preserve phase effects in the general definition.** `site/kernel.html:286` defines transition validity with “a rejected event moves nothing.” This wording existed in the prior page, but is presented as a general acceptance judgment and conflicts with the product contract's explicit distinction between local atomic rejection and Midnight fallible-phase failure, which can retain guaranteed-phase effects and fees. The expanded explanation should not carry this ambiguity forward. Suggested general wording: “The next state and its financial effects follow the program’s rules; each outcome, including failure, retains exactly the effects and duties its phase policy specifies.” The explorer's refusal behavior can remain described as leaving its account unchanged. This is a semantic qualification, not an implementation disclaimer.

2. **Limit the foreign-transfer claim to the foreign transfer.** `site/kernel.html:583–585` adds “The most Midnight can do ... protects the agreement’s history and nothing else.” Neither the prior page nor the design contract establishes that exhaustive restriction. Midnight still enforces local state and consumption rules; refusing an invalid stage can prevent subsequent local effects. The actual boundary is that local refusal cannot reverse a transfer already authorized at a signature-only foreign destination. Suggested replacement: “Midnight can refuse to accept that transfer as a stage of the agreement, but that refusal cannot reverse the foreign transfer.” No new recovery mechanism is implied or requested.

## Preserved meaning

The rewrite retains permissionless authoring and optional federation, ZKIRv3 targeting, owner consent, the distinct acceptance judgments and compiler correspondence obligation. Asset/domain identity and the explicit pre-funded escrow boundary remain. The 11 A gross cap, 1 A fee cap, 20 B goal, first-fill amounts, pending reservation and all three terminal transcript rows match the claim inventory and prior version.

The separation between authenticated external effects and agreement acceptance remains intact, including once-only reconciliation after restoring a predicate. The explanation retains unknown custody location, nonexecution not following from a timeout, recovery prerequisites, persistent duties and absence of global rollback. It retains separate ZK, threshold and hardware assumptions; the t−f bound; common binding versus enforcement; correlated dependencies; and the foreign signature-only bypass. The full mechanism assumptions remain in the inspector data and static fallback table.

OWS delegation and x402 payment/result/delivery distinctions remain, with stable logical request identity and separately authorized new charges. Bounded stages/fan-in, finite growing history, episode/lifetime budgets, exclusive consumption and witness handoff distinctions remain. DeFiFormal retains its reference-only and no-transferred-theorem meaning. The final section identifies kernel integration, recursion, private handoff and source-to-ledger correspondence as unfinished. Changed visible strings in KernelExplorer and EvidenceInspector introduce no material change in meaning.

## Scope and limitations

Read the product contract, relevant consolidated-design provisions, full current HTML, prior published HTML/diff, changed TSX strings, mechanism data and claim inventory. Startup read AGENTS.md and the repository development skill; status reported no pending transactions and unrelated campaign admission gaps. No website or source edits were made. This review does not independently verify cryptographic constructions, deployed behavior, runtime/browser tests, external standards, or claimed implementation results. Present-tense design language was assessed as design exposition under the user's instruction, not treated as a deployment claim.

The fixture-specific fee-exhaustion paragraph is understood as referring to its fixed 0.5 A fee per fill, not as a theorem forbidding every possible zero-fee route under another agreement.

## Reviewed identity

Baseline HEAD: `88fab364fc9c42d174571f5c508b218c4d0e5c92`

- `site/kernel.html`: `e6107e8bf07faa6f191b810400dd2986c88f97c9f0a3a99b76dc6b643aea0c32`
- `site/src/kernel/KernelExplorer.tsx`: `c074a95223c9812482848008b664d9f8023ab52c610aed6cadf1d4303c502c51`
- `site/src/kernel/EvidenceInspector.tsx`: `6c1de42536b86fa8c2ec5fd052dd01c376f20b714b8029457a02bcb6d380fc77`
