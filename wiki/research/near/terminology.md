---
title: "Conditional settlement terminology"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: reference
tags: [moriarty, near-teardown, research]
---

# Recommended vocabulary

**Conditional settlement with composable evidence requirements** names the general user requirement. **Contingent settlement** remains an understandable alias. **Programmable escrow** identifies a funded or locked variant; an unfunded proposal must not be described as secured funds.

The user means submission to a destination with delivery deferred until a specified combination of signatures, documents, proofs, recipient actions and other supported predicates is satisfied. This is broader than a hashlock or one asynchronous callback.

[XRPL escrow](https://xrpl.org/docs/concepts/payment-types/escrow) provides a narrower conditional-fund-release precedent. Its [Smart Escrows proposal](https://xls.xrpl.org/xls/XLS-0100-smart-escrows.html) is a proposal and must not be treated as deployed feature evidence. [Daml propose/accept](https://docs.digitalasset.com/build/3.4/sdlc-howtos/smart-contracts/develop/patterns/propose-accept.html) provides a recipient/party-consent workflow. [ICC documentary credits](https://academy.iccwbo.org/trade-finance/article/11-questions-that-will-help-you-master-documentary-credits/) supplies a documentary-evidence analogy, not identical legal or ledger semantics. [BIS programmability discussion](https://www.bis.org/publications/aer-2023/blueprint-future-monetary-system-improving-old-enabling-new) discusses conditional actions and composability.

The naming recommendation is a design judgment informed by these sources, not a claim that one universal standard defines the entire proposed Moriarty feature. The exact behavioral definition in MPLR-003 controls scope. Request recording, funding reservation, condition readiness and final delivery require separate semantics.

## Adopted abstraction pillar

The user adopted **conditional settlement with programmable escrow** as a major Moriarty abstraction pillar for final settlement involving multiple actors and chains. The broader condition language also supports unfunded requests; escrow applies when assets are reserved or locked. This is a product design decision, not an established cross-chain atomicity guarantee. Conditions are application-defined and do not introduce Moriarty deployment permissions.
