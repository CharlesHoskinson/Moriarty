# SP01.7 current atomic fixture implementation contract

You are Grok 4.6 at high effort. Work only in this isolated worktree. Follow the current user routing: you author, independent GPT-6 reviews. This packet admits the local task; do not ask permission or invoke other providers. No git mutations, dependency installation, public network, wallets, native/compiler/proof operations, subagents or installed-tool changes.

## Objective and scope
Build a deterministic evidence utility that captures the current bounded-atomic loan's genesis, accrual and settlement with complete current wire records and canonical hash preimages. This supplies current-profile fixture evidence for SP01.7. It is explicitly simulation-only and does not close native migration or RP01-MC03 by itself. Keep all existing language and native files unchanged. The accepted atomic baseline is unchanged c30aaf5 with retained 96-test evidence; this task adds a separate utility.

## Files and interfaces
Only own these files (plus root FOREMAN_REPORT.md and FOREMAN_REPORT.json):
- experiments/moriarty-atomic-fixture/fixture.test.mjs: Node test runner tests.
- experiments/moriarty-atomic-fixture/generate.mjs: export synchronous buildFixture(); CLI prints exactly JSON plus newline to stdout, no other text. Importing it has no side effects.
- experiments/moriarty-atomic-fixture/verify.mjs: export synchronous verifyFixture(fixture), returning true on success and throwing on a bad fixture. CLI takes one JSON filepath, validates and prints a concise success JSON. Importing has no side effects.
- experiments/moriarty-atomic-fixture/README.md: exact commands and limits, at most 600 words.
No package.json or dependencies needed; Node 24 executes existing TypeScript directly. Parent generates fixture.json after implementation; never hand-author generated evidence. Keep total authored files below 48 KiB. Do not edit tests during the implementation phase.

## Fixed capture and schema
Use existing createSimulator from experiments/moriarty-language/src/evaluate.ts and original loan.mori + exact bounds bytes. Reuse the deterministic loan setup/input contract in tests/semantics.test.mjs: domain network simulation/deployment test, instance instance, five actor/principal bindings borrower/lender/pool/provider/trader in that order, clock observation binding with external policy, UInt128 now 1 and zero evidenceDigest, simulation-only signature with empty bytes, checks all true and authenticatedPrincipal borrower. Outcome authority per action, max UInt128 gross cap, all five permitted recipients, nonce nonce0 then nonce1, validity [0,2000000000). This is synthetic test authorization, NOT the old native fixed authorization and NOT real signing or custody.
Only execute accrue(actor borrower), then settle(actor borrower, settlement_asset Text USD_TEST_ASSET, amount_due Amount 533972602 USD_micro). Retain both full EvaluationInputs and Simulation results. Do not call production evaluate with a fake backend.
Root fields exactly: schemaVersion ('moriarty-current-atomic-fixture/1'), scope ('simulation-only'), sourcePins (sorted array of {path,sha256} for original loan source, bounds and every src/*.ts), program (ProgramRef), manifest (bound.manifest), genesis, states (initial/accrued/settled StateEnvelope array), steps (array of {input,result}), commitments (array below). No timestamps, absolute paths or generated random values.
Each commitment is exactly {label,domain,preimage,canonicalHex,digest}. canonicalHex is UTF-8 canonicalEncode(preimage) encoded as lowercase hex. digest is SHA256(UTF8(domain) || NUL || canonical bytes). Include PROGRAM/manifest, GENESIS/genesis.body, CLAIMS/manifest.requiredClaims, STATE for all three states, and per step ACTION/input.action, OBSERVATIONS/input.observations, AUTHORITY/input.authority, SIGNED statement using input.authority.domain and input.authority.statement, TRACE/result.candidate.body, PROOF-CONTEXT/result.context. Labels unique and deterministic; domains exactly current source domains. Check corresponding stored digests, including statementDigest in authorityConsumption. No invented standalone effects/domain/specification digest; document these as embedded or missing in current schema. All seven required claim IDs must retain actual canonical manifest order. Never put stateHash/traceHash/proofContextHash into their own preimage. Preserve every typed field, complete observations, obligations/tombstones, statuses and terminal residual notional.

## Independent verification obligations
Verifier must NOT import or invoke buildFixture, and must NOT use evaluate.hash, codec.hashDomain or codec.canonicalEncode for digest recomputation. Implement a small independent canonical JSON encoder for the admitted string/boolean/array/record domain, and use node:crypto to check every canonicalHex/domain/digest. Check the exact required commitment set and cross-links to the fixture's source objects, not merely self-consistent arbitrary commitments.
Pin original source bytes by rereading only the documented local inputs. Compile source to cross-check program/manifest and reconstruct fixed genesis and expected test inputs. Replaying the two simulation steps is permitted to verify complete candidate/context records, but it is NOT the independent economic oracle.
Independent economic oracle is accepted SP01/loan-swap-subset-01/traces.json: compare initial values, and each step's afterValues, revision, remaining, episodeStatus, agreementStatus, remainingNotional, orderedWrites, orderedEffects, obligationDelta and retainedObligations. Additionally derive interest with BigInt: 5000000000*8*31 divided by 100*365 = 33972602 floor, principal 500000000, payment 533972602. Final remaining is 0 while 4500000000 notional remains Outstanding; dues remain as Settled tombstones. Enforce no fake Accepted status or real-proof claims. This verifier validates this fixed evidence fixture only, not arbitrary incoming production wire objects.

## Tests first and verification
Write tests BEFORE generate.mjs/verify.mjs. Tests can dynamically import with a caught missing module and assert expected exported APIs exist, so the initial red result is a substantive missing-utility assertion. Parent runs the red command; you do not run it yourself.
Test buildFixture determinism; verifyFixture true for valid capture; independently compare fixed economics; reject changed source pin; changed amount or recipient; missing obligation tombstone; terminal residual-debt erasure; changed genesis/authority/predecessor context; missing/duplicate commitment; changed canonical byte or domain/digest; and extra or missing root/step/result fields. Include a self-consistent rehashed mutated object: it still must reject against fixed expected economics or context. Avoid shallow tests that only check an echoed status string.
Commands parent runs:
  node --test experiments/moriarty-atomic-fixture/fixture.test.mjs
  node experiments/moriarty-atomic-fixture/generate.mjs > fixture.json
  node experiments/moriarty-atomic-fixture/verify.mjs fixture.json
Parent also regenerates twice for byte identity and independently recomputes hash preimages with Python. No acceptance claim comes from specified tests alone.

## Reports
Record changed files, tests authored (not executed by you), scope, fixed synthetic authority limitations, and remaining native/export/encoding/proof blockers. No private reasoning or credentials. Keep report concise. Stop after writing your phase's files.

CURRENT PHASE: TEST AUTHOR ONLY. Write fixture.test.mjs and reports. Do not create generate.mjs, verify.mjs or README.md. Do not execute tests.
