# Bounded repayment K experiment — September 9, 2026

All 16 bounded K traces pass with complete K/source/independent-result agreement.
The successful run took 34.763 seconds and peaked at 418.6 MiB with zero swap.
[Execution result](execution-result.json), [complete comparison](complete-result-comparison.json)
and [raw observations](attempt-03/observations.json) retain the evidence.

The root README includes all 36 productions of the current successor EBNF. Its
drift test compares the whole fenced grammar with the canonical file. Nine
[official K documentation captures](../../docs/research/2026-09-09-bounded-k-references.md)
were acquired with Scrapling 0.4.15 and ingested with provenance.

## Scope

The K definition executes a bounded Transfer followed by Repay projection. It
accepts two initial balance rows, one allowance, one obligation, identity
conversion and AccrualFirst or PrincipalFirst allocation. The codec validates
input shape and lexical bounds; K computes changed financial values. This is a
local preparation experiment, not full successor Core semantics, a proof of
correspondence, or Midnight financial settlement. SP03 remains open.

## Retained attempts

1. [Attempt 01](attempt-01/compile.command.json) consumed one compiler invocation.
   The installed compiler rejected `-O0` as a second positional argument in
   0.416 seconds. The earlier help command's success did not establish support.
   No definition or trace was produced. Raw stderr and original candidate,
   resource and admission receipts remain under `attempt-01/`.
2. [Attempt 02](attempt-02/compile.command.json) compiled the definition in
   4.334 seconds and ran the first fixture in 1.919 seconds. Both K commands
   exited zero. The harness rejected the result because it expected KAST v3
   while the installed tool emitted v4. Raw output is retained in
   [trace-01.stdout](attempt-02/trace-01.stdout). The first output contains the
   expected 70 residual principal after payment of 30, but this attempt did
   not establish complete decoded agreement or complete the 16-case suite.

The second service peaked at 681.2 MiB with zero swap, according to its
[supervisor output](attempt-02/execution-02.stderr). Its actual cgroup enforced
4 GiB memory and zero swap. All 174 compiled artifacts remained byte-identical
after the first krun; see [the check](attempt-02-artifact-stability.json).

3. [Attempt 03](attempt-03/execution-supervisor-03.json) used the reviewed strict
   KAST v4 decoder. Its compile and all 16 krun commands exited zero, and all
   complete results matched independent expectations and actual source results.
   Six positive cases and ten financial failures passed. All 174 compiled
   artifact hashes remained unchanged after the suite. The six codec tests
   include exact replay of the prior actual output and malformed v4 schemas.

Historical totals are three compiler invocations and 17 krun invocations. No
failed attempt was deleted or refunded. Each run used a separate reviewed
prospective allocation with the same limits.

## Verification already completed

- README grammar and source examples: 236 language tests passed; separate GPT-6
  grammar audit and Fable review passed. See `readme-grammar-report.md` and
  `readme-gpt6-audit.md`.
- Six independent positive financial observations plus ten failure cases pass
  through actual `.mori` parsing and preparation. The complete source results
  are in `source-observations.json`; `check-source-cases.mjs` reproduces them.
- Five initial codec tests passed before actual K execution. They did not
  establish compatibility with the installed K output format.
- Scoped preexecution GPT-6 and exact Fable 5.1 reviews, corrections and resource
  votes are retained individually. Preexecution approval is not result approval.

## Remaining acceptance

This evidence freezes the observed run before independent GPT-6 and exact
Fable 5.1 result review. Subsequent verdicts and publication disposition are
recorded in `acceptance.json`. Finite trace
agreement does not close formal correspondence, untested financial branches,
full SP03, native recursive proofs, or public financial ledger acceptance.
No Midnight transaction was submitted by this local experiment.

The [wiki lessons](../../wiki/k-framework/k-best-practices.md) retain the missed
installed-interface assumptions and recommend an early bounded executable smoke.
