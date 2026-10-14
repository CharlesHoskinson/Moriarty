# SP01.3 semantic-replay completion

Status: proposed design reconstruction. Specified-only. Not a semantic freeze.
Author: Grok 4.6 high. Worktree: `/home/charl/Moriarty/.worktrees/sp01-successor-contract-grok`.
No subagents. No network. No install. No compiler. No proof. No git.

Interrupted `interrupted-semantic-replay/` and prior RED receipts stay
as recorded. This pass did not delete them. Frozen candidates stay immutable.

## Owned files

Eight files. Limit 768 KiB. Used 704866 bytes.

## Commands and exits

1. Root diagnostic of the cancelled 48-call generator wrote
   `semantic-replay-diagnostic-generated.json` exit 0, 290553 bytes,
   4 valid, 27 invalid. Tests with only `EXAMPLES_PATH` rebound:
   PASS 882 FAIL 69. That receipt is kept. It is not author completion.
2. `python3 generate-signing-examples.py --output /tmp/sp01-sem-replay-a.json`
   Exit 0. 312975 bytes. SHA-256
   `18ccb7814863b07bc11ce11ce22969a415ab10eb2a12e70fd22ef9b04f5429af`.
3. `python3 generate-signing-examples.py --output /tmp/sp01-sem-replay-b.json`
   Exit 0. Byte-identical to A.
4. Owned `signing-examples.json` written from that output only after those
   two runs.
5. `python3 signing-contract.test.py` exit 0. `PASS 965` `FAIL 0` `GREEN`.
   Tests do not import generator helpers. They read the owned examples file
   and recompute writes, work, wrappers, and first predicates.

## Corrections this pass

- F2: Price mantissa display unit is `AssetB per AssetA at scale 4`.
  Context resolution runs at emit time so schema `scaled-integer` does not
  overwrite the ancestor Price.
- F4: `ledgerExamples.inputAssetFee30Inflow0` is signed net -30.
  `shortfallCancellationDuty` creditor is trader, debtor is pool, AssetB 30,
  named-balance backing. Extra specified-only fixtures
  `inv-input-asset-fee-only` and `inv-unfunded-cancellation` have complete
  contexts. All 27 original invalid ids remain.
- F5/R7: Genesis posts 7. Swap and accrue inherit 7 and post 6 from the
  declared cost certificate. Tests compare actual predecessor and cost.
  They do not hardcode 8 to 7. `+1` refresh still rejects.
- F6: Fee shortfall uses `feeAdmittedStateBody` with fee cap 30 AssetB.
  Cancel `winningFill` consumes the stale consumption id. Clone parent work
  is genesis post 7. Migrate, overdelivery, partial, network, deployment,
  revoked, expired, and duplicate-predecessor fixtures bind reconstructed
  wrappers and admitted prestates. Named first predicates are present.

Contract words and schema bytes are unchanged from the interrupted replay
tree. Decisions and reports now describe this generated artifact.

## Limits

No GPT-6 review in this pass. No freeze. No RP01, SP01, BNF, K, native,
wallet, or network acceptance. Native hash unresolved. No evaluator ran.
Signature verification did not run. Results remain specified-only.
Partial tests and a proposed finite contract are not full BNF, K, semantic
freeze, native, ledger, or sprint acceptance.
