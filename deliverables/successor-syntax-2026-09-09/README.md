# Provisional successor syntax

Moriarty can parse a `moriarty-successor-syntax/0` source file, print its AST and format it canonically. The profile has explicit lexical rules and EBNF, fixed admission bounds, UTF-8 byte spans, and stable error codes. The read-only CLI rejects oversized files, malformed UTF-8 and non-regular inputs.

This is a syntax foundation for SP02. It does not type or execute a financial agreement, conserve obligations, provide K semantics, freeze RP01, or close SP02/MC01. The partial-payment example is illustrative source, not an authenticated cash transfer. No native proof or Midnight transaction was produced in this slice. Accepted atomic files and resource history remain unchanged.

## Use

From `experiments/moriarty-language`:

```sh
node src/successor/syntax-cli.ts check-syntax spec/successor/examples/partial-payment.mori
node src/successor/syntax-cli.ts format spec/successor/examples/partial-payment.mori
npm run build
npm test
```

Node 24 runs the TypeScript source directly. The existing build type-checks with `noEmit`; it does not produce a `dist` directory.

## Verification and review

[Checks](checks.json) record type checking, all 117 tests, 32 independent API controls, nine CLI controls and 500 deterministic formatter round trips. Those are bounded tests, not a proof of grammar equivalence or financial correctness. [The candidate](candidate.json) identifies the exact reviewed files. [GPT-6's independent audit](gpt6-audit.md) records its verdict and additional checks; an approval applies only to those bytes and this syntax scope.

The independent audit found that splitting compact generic `>=` endings could hide tokens from admission. A saved regression accepts the adjacent 8192-token program and rejects the oversized case. Grok fixed the parser accounting. The initial Unicode span error, omitted `ensures` statement accounting, Node-incompatible parameter properties and CLI import mismatch were also corrected and checked. Tests include formatter byte expansion, source/token/depth limits and malformed input rejection.

## Authorship and failures

[Authorship metadata](authoring.json) records actual Grok 4.6 requests at high effort and the observed serving model `grok-4.6-build`. GPT-6 integrated returned source mechanically and performed independent checks; a separate fresh GPT-6 reviewer audited it. The additional token-bound regression is an independent GPT-6 audit test.

The first broad Grok worker timed out after 900 seconds with partial specification/test files. A structured response returned placeholders and was rejected. Two repair calls returned file-inspection chatter and hit turn limits. A split-token proposal threw the wrong error class and was never applied. These attempts remain failures; later successful source and tests do not convert them into successful runs. Direct source-text requests produced the final focused correction. Hidden model reasoning is excluded from publication.

The [refined roadmap](../../ROADMAP.md) retains all twelve sprints and their existing acceptance gates. The host completion goal was observed active on September 9 at 06:56 UTC. That runtime observation is not a guarantee of future host availability or roadmap completion.
