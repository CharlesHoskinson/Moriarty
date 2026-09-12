---
type: "query"
date: "2026-09-02T17:25:31.736787+00:00"
question: "How do Marlowe language types, semantics, Runtime APIs, and TypeScript clients relate?"
contributor: "graphify"
outcome: "useful"
source_nodes: ["Contract", "Input", "Transaction", "Party", "Token", "semantics.ts", "contract/details.ts"]
---

# Q: How do Marlowe language types, semantics, Runtime APIs, and TypeScript clients relate?

## Answer

Expanded from original query via graph vocabulary: [marlowe, contract, semantics, transaction, state, input, validator, runtime, continuation, payout, role, token]. The merged structural graph identifies a hand-maintained V1 TypeScript model and semantics under marlowe-ts-sdk packages/language/core/v1, including Contract, Input, Token, Transaction, semantics.ts, actions, values/observations, state, participants, choices and next/applicable-action modules. The same repository contains REST Runtime client endpoints for contracts, transactions, payouts and withdrawals plus lifecycle wrappers and E2E tests. It also exposes a specification client and codec/guard boundaries, so cross-language semantics and JSON conformance are a material version-skew boundary. The graph is navigation evidence only; Haskell, Isabelle and Agda coverage is sparse and each claim requires direct pinned-file verification.

## Outcome

- Signal: useful

## Source Nodes

- Contract
- Input
- Transaction
- Party
- Token
- semantics.ts
- contract/details.ts