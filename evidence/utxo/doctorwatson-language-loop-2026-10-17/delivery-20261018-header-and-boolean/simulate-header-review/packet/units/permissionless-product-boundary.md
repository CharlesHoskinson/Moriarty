# Moriarty: permissionless language of provable intention

Status: target architecture, specified-only where not linked to implementation. Authority: user clarification, 2026-09-19.

Moriarty is a programming language for all DeFi developers on Midnight. Any developer may author, compile, prove, publish and deploy programs in the supported language without approval from Moriarty maintainers, an agent council, Foreman, a hosted solver or a program registry. Objective Midnight validity rules, resource costs and asset-owner authorization still apply.

This document supersedes conflicting product-scope and administrative-deployment requirements in earlier designs. Historical receipts remain evidence of their original scope. It does not declare missing proof or ledger capabilities implemented.

## Consolidated design and delivery

The [consolidated vision and design](MORIARTY-CONSOLIDATED-DESIGN.md), [single U0–U7 roadmap](../ROADMAP.md), and [next ZKIR/recursion requirements](MORIARTY-BACKEND-REQUIREMENTS.md) integrate the research agenda. P/C/K identifiers remain requirement lineage; they are not independent delivery queues. The user supplied a planning assumption on 2026-09-19 that comprehensive Midnight recursion arrives in about six months. Plan full native recursive compliance and private multi-parent composition accordingly; current interface limitations do not remove that scope. The timing is not a verified release claim.

## Product boundary

The product is the language, semantics, compiler, proof interfaces, verifier integration and developer tooling. ACTUS, loans, swaps, vaults and other DeFi applications test expressivity and conformance; they are libraries and examples, not an exhaustive list of programs developers may deploy. Supported constructs compose into new programs. New language primitives require sound semantics and implementation; they do not justify approval of individual programs built from existing constructs.

A managed multichain router, identity provider, adapter catalog or pricing service can be an optional application. None is required to use Moriarty. Application-specific access control remains expressible without becoming a language-wide deployment restriction.
