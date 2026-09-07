# MC01 implementation handoff after source-profile freeze

Status: preparation only. The reviewed profile's final schemas override names suggested here.

## Objective

Implement a real bounded source language with parse, check, elaborate, evaluate and initial Compact mapping.
Use the complete reviewed loan and swap source files as author input. Do not dispatch by financial package name.
Preserve the old R2/R2b developer mock as an independent financial comparison and historical signing interface.

## Files and interfaces

Implement the exact MC01 owned files in openspec/changes/mc01-bounded-language/design.md.
Use experiments/moriarty-language/package.json, src/{ast,parser,typecheck,elaborate,evaluate,codec,lower-compact,errors}.ts and tests/{frontend,semantics}.test.mjs.
Use the frozen source profile for all record fields, version literals and canonical preimages.
The mandatory public flow is parse(source), check(ast,profile), elaborate(typed), evaluate(program,state,action,authority,observations).
Return source-located diagnostics or precisely typed results. Unsupported behavior rejects explicitly.
Provide a CLI or developer flow for source -> check -> simulate -> inspect before authorizing any ledger operation.

## Existing-source observations

Legacy core.ts infers canonical numeric-looking literal strings as UInt128. Reuse of that parser would lose text("123") semantics.
Legacy Core types are UInt128/String only and literal strings carry no nominal units. Keep elaborated nominal typing and program binding explicit.
Legacy identifiers permit punctuation, whereas source identifiers and canonical keys use the stricter frozen alphabet.
Legacy CoreState has no obligation records or episode/agreement status. Legacy evaluated results cannot substitute for the new result envelope.
Legacy safeJsonValue and claims.ts canonicalization have different depth limits. New codecs must apply the profile's joint per-object limits.
Legacy packages.ts provides independent first-period loan and constant-product financial traces. Preserve every state field/effect in comparison.
Legacy outcome.ts checks gross debits and net goals. Keep fees and refunds separate and preserve rejection of unauthorized intermediates.
Historical MORIARTY-SIGN-v1 and MORIARTY-OUTCOME-SIGN-v1 bytes must not silently change.

## Meaningful verification before implementation

First add failing tests against missing frontend functions. Preserve their real failures.
Positive source cases: both complete examples, numeric-looking Text, expression-only Bool/Quantity locals, dimensional A*B/A -> B, UTF-8 source spans and canonical roundtrips.
Negative source cases: loops/functions/recursion, unknown fields, duplicate/reserved names, invalid Unicode, incorrect units, bad arity, unbound/forward locals, Bool/Quantity state or effects, duplicate/missing policy targets, stale rounding targets and missing status rules.
Arithmetic cases: underflow, overflow before division despite an in-range quotient, zero divisor, floor remainder, no compiler reassociation, and nominal unit mismatch.
Wire cases: unknown/duplicate keys, JSON numeric tokens, leading zero, -0 exponent, noncanonical escapes/order, tampered profile/program/bounds hashes and simultaneous byte/depth/node violations.
Evaluation cases: atomic rejection with unchanged input, horizon expiry, zero remaining allowance, revision overflow, no lifecycle reset, actor/principal mismatch, observation mismatch and unsupported Pending.
Financial cases: full loan accrual and settlement, separate PR/IP obligations, remaining 4500000000 notional, duplicate/unknown/excess settlements and quantum divisibility; swap19743 with reserves1010000/1980257 and exact closure balances/effects.
Authority cases: exact-plan and outcome modes, gross debit including fees, net receipt after fees, and unapproved intermediate recipient/call. Do not claim durable replay protection before MC04.
Compare complete normalized traces with the legacy evaluator while preserving new obligation/status semantics explicitly.

## Initial Compact mapping

Lower supported typed operations with source maps and representability checks, then compile the generated loan/swap specialization using retained pinned tooling.
Reject unsupported forms and sizes with diagnostics. No host-evaluated final constant may replace action/state-dependent arithmetic in the generated contract.
Compiler success is not native PCD or compiler-to-ledger correspondence. MC04 and MC05 own those actual acceptance predicates.

## Acceptance

Run package build/tests, existing developer mock regressions after integration, independent trace comparisons, generated Compact compilation and exact Fable/fresh GPT-6 result audits.
Keep source-profile approval distinct from whole-MC01 implementation approval and all later packages.
