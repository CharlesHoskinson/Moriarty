# Current atomic fixture

Simulation-only capture of the current bounded-atomic loan genesis, accrue, and settle. This is SP01.7 fixture evidence. It does not close native migration or RP01-MC03. Language and native files stay unchanged. The accepted atomic baseline remains `c30aaf5` with retained 96-test evidence.

## Commands

Run from the repository root on Node 24. This directory has no package.json and no extra dependency. Node executes the existing TypeScript sources directly.

```
node --test experiments/moriarty-atomic-fixture/fixture.test.mjs
node experiments/moriarty-atomic-fixture/generate.mjs > fixture.json
node experiments/moriarty-atomic-fixture/verify.mjs fixture.json
```

`generate.mjs` prints one JSON document plus a newline and no other text. Importing `generate.mjs` or `verify.mjs` has no side effects. `buildFixture()` is synchronous. `verifyFixture(fixture)` returns true or throws. Do not hand-author `fixture.json`. Parent regenerates twice for byte identity and recomputes hash preimages separately.

## Capture

The generator uses `createSimulator` on original `loan.mori` and `bounds.json`. Setup matches `tests/semantics.test.mjs`: network `simulation`, deployment `test`, instance `instance`, principals borrower, lender, pool, provider, trader, clock observation with external policy, `now` 1, zero evidence digest, simulation-only empty signatures, all checks true, authenticated principal borrower. Authority is Outcome intent with max UInt128 gross cap, five permitted recipients, nonce `nonce0` then `nonce1`, validity `[0, 2000000000)`. This is synthetic test authorization. It is not native fixed authorization and not real signing or custody.

Actions are only `accrue(actor borrower)` then `settle(actor borrower, USD_TEST_ASSET, 533972602 USD_micro)`. The fixture keeps full EvaluationInputs and Simulation results. It does not call production `evaluate` with a fake backend.

Root fields are `schemaVersion` (`moriarty-current-atomic-fixture/1`), `scope` (`simulation-only`), `sourcePins`, `program`, `manifest`, `genesis`, `states`, `steps`, and `commitments`. There are no timestamps, absolute paths, or generated random values. `sourcePins` hash original loan source, bounds, and every `src/*.ts` file.

## Commitments

Each commitment is `{label, domain, preimage, canonicalHex, digest}`. `canonicalHex` is UTF-8 canonical JSON of the preimage as lowercase hex. `digest` is SHA256(UTF8(domain) || NUL || canonical bytes).

Committed objects: PROGRAM over the manifest, GENESIS over `genesis.body`, CLAIMS over `requiredClaims` in canonical manifest order, STATE over the three state bodies, and per step ACTION, OBSERVATIONS, AUTHORITY, SIGNED statement using `authority.domain` `MORIARTY-OUTCOME-bounded-atomic/1`, TRACE over CompleteBody, and PROOF-CONTEXT. STATE, TRACE, and PROOF-CONTEXT preimages omit the digest they define. ProofContext includes `traceHash` of the candidate. Effects live inside TRACE CompleteBody. The current schema has no standalone effects digest and no specification digest. `proofDigest` applies to raw proof bytes and is absent here.

## Verifier

`verify.mjs` does not import `buildFixture` and does not use `evaluate.hash`, `codec.hashDomain`, or `codec.canonicalEncode` to recompute digests. It uses a small independent canonical JSON encoder and `node:crypto`. It rereads pinned local source bytes, compiles the loan, reconstructs genesis and the fixed test inputs, and may replay the two simulation steps. Replay checks complete candidate and context records. It is not the independent economic oracle.

The verifier builds the exact eighteen commitments from the supplied fixture records. Labels are PROGRAM, GENESIS, CLAIMS, STATE-initial, STATE-accrued, STATE-settled, then ACTION, OBSERVATIONS, AUTHORITY, SIGNED, TRACE, and PROOF-CONTEXT for accrue and then settle. Each entry must appear once, in that generator order, with that label, domain, complete preimage, canonicalHex, and digest. This deterministic fixture rejects reordered commitments. No entry may be unused or reused for another step.

Independent economics come from `evidence/moriarty-completion-program-2026-09-07/SP01/loan-swap-subset-01/traces.json` and from `5000000000 * 8 * 31 / (100 * 365)` floor interest 33972602, principal 500000000, payment 533972602. Terminal remaining is 0 with residual notional 4500000000 Outstanding and Settled due tombstones. The verifier rejects fake Accepted status and real-proof claims. It validates this fixed fixture only.

## Limits

Native statement export, Compact encoding correspondence, PCD, Preview loan and swap settlement, and residual servicing of notional 4500000000 stay open.
