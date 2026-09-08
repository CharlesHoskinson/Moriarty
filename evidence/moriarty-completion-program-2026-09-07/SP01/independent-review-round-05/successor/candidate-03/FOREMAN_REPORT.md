# SP01.3 successor generator completion

Status: proposed design reconstruction. Specified-only. Not a semantic freeze.
Author: Grok 4.6 high. Worktree: `/home/charl/Moriarty/.worktrees/sp01-successor-contract-grok`.
No subagents. No network. No install. No compiler. No proof. No git.

Interrupted `interrupted-generator-correction/` and prior RED receipts stay
as recorded. This pass did not delete them.

## Owned files

Eight files. Limit 768 KiB.

## Commands and exits

1. `python3 generate-signing-examples.py --output /tmp/sp01-gen-out-1.json`
   Exit 1. Stderr: `ProofContext typecheck: 'moriarty-succ-core/0' was expected path=['specVersion']`.
   Validator stayed Draft 2020-12. Schema const was not disabled.
2. Generator `ProofContext.specVersion` set to `moriarty-succ-core/0`.
   Document `schemaVersion` stays `moriarty-succ-sign/0`.
3. `python3 generate-signing-examples.py --output /tmp/sp01-gen-a.json` exit 0.
   `python3 generate-signing-examples.py --output /tmp/sp01-gen-b.json` exit 0.
   Byte-identical. SHA-256 `56c34ef5104970550bffcd0bb3743d5a62495a25f8783a1851f1a8a5350ececc`.
   Owned `signing-examples.json` written from that output only after those two runs.
4. `python3 signing-contract.test.py` exit 0. `PASS 338` `FAIL 0` `GREEN`.
   Tests do not import generator helpers. They read the owned examples file.

Prior interrupted pass recorded meaningful RED 144 failures. That receipt is
kept. An earlier wrong test-path failure is not that RED.

## Corrections

- R1: Rate/Price inhabit StoredValue. StateBody.fields registry. Shares.holder.
  Message.payloadHash. CoreOp table with Arity, Parameters, Result, State effect.
- R2: true-end patterns, domain widths, four claim kinds kept.
- R3: display values equal parsed leaves. Schema `x-display` required. No
  signed-field fallback.
- R4: inner DAG from typed bodies. PolicyBody domain. Genesis bootstrap zeros.
  Stale accrue/outcome inner hashes rebuilt.
- R5: raw floor 41095890. Cap 6000000000. lastAccrualEnd non-overlap.
- R6: trader AssetA/AssetB ledgers. Residual gross 0 after 10000. Debt ledger
  includes cumulativeAccrued.
- R7: post-charge work 7 on state, prepared, successor.
- R8: migrate/cancel/fee/clone contexts recompute wrappers. Specified-only.

Contract words: 5596 (limit 8000). Old atomic pins unchanged.

## Limits

No GPT-6 review in this pass. No freeze. No RP01, SP01, BNF, K, native, wallet,
or network acceptance. Native hash unresolved. Context evaluator did not run.
Signature verification did not run. Results remain specified-only.
