# Successor syntax profile `moriarty-successor-syntax/0`

This directory is a provisional syntax profile. It gives developers a bounded parser, a canonical formatter, and a read-only CLI that can print an AST or rewrite source. It does not type programs, evaluate them, conserve obligations, emit K, or admit a financial action.

The example `examples/partial-payment.mori` is syntax-only illustrative debt reduction. The identifier `payInterest` and the `debt` constructor are names in a grammar. The parser does not implement a payment. The source is not an authenticated cash transfer and it has no executable financial semantics. Later typed authority, transfer semantics, and K have to supply that relation. A source hash of this file is not a semantic hash.

## What is in this slice

- [lexical.md](lexical.md) records encoding, tokens, comments, and source spans.
- [grammar.ebnf](grammar.ebnf) is the ISO/IEC 14977 grammar for this profile.
- [syntax-profile.json](syntax-profile.json) publishes the fixed bounds and the stable error codes.
- `src/successor/frontend.ts` exports `parseSuccessorSource`.
- `src/successor/format.ts` exports `formatSuccessorSource`.
- `src/successor/syntax-cli.ts` is the `check-syntax` and `format` CLI.

The accepted atomic frontend under `src/frontend.ts` is unchanged. This profile does not fall back to `moriarty-bounded-atomic/1`.

## CLI

The commands read a file and write to stdout. They do not write the input file. They do not execute the program. They do not contact a network. Run them from the working directory `experiments/moriarty-language`.

```
node src/successor/syntax-cli.ts check-syntax spec/successor/examples/partial-payment.mori
node src/successor/syntax-cli.ts format spec/successor/examples/partial-payment.mori
```

`check-syntax` is not the full semantic check named by SP02.3.

## Bounds

The parser mirrors `syntax-profile.json` limits as fixed constants. It does not read the JSON at runtime. Callers cannot raise them. The implementation checks source size, token count, and nesting before it builds an oversized tree. Canonical formatting that would exceed 65536 UTF-8 bytes fails with `FORMAT_BOUND`.

## Unsupported in this profile

The grammar rejects these declarations. They still belong to later owners.

- obligation, request, and composition declarations
- function declarations, import, macros, and extension escape hatches
- loops and source recursion
- observation, settlement, policy, status, reserve, and effect-schema declarations from the atomic profile
- implicit profile fallback

`floor_div` and `ceil_div` parse as ordinary calls. This slice does not implement their rounding policy.

## Missing RP01 freeze

RP01 remains unfrozen. This profile does not promote a successor semantic contract, broaden accepted atomic scope, or close SP02. Static judgments, Core elaboration, K, and financial coverage stay later work.

## Expression source component

The separate [expression-source/1 candidate](expression-source.md) now connects
real `.mori` expressions to the reviewed40-constructor Core evaluator through a
trusted-schema API. It adds explicit numeric type metadata, signed values,
records/options/collections, index access and direct record-valued emissions.
Its exact implementation audits are pending. The included demo checks and
updates ordinary state while retaining financial state; emissions are typed
descriptors with no financial execution. Original syntax-only and funded entry
points keep their exact profiles. Full agreement declarations and SP02 remain open.
