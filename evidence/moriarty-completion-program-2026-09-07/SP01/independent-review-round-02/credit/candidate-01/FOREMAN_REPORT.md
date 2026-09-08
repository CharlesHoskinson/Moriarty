# FOREMAN_REPORT credit quantity-repair packaging

Status is specified-only. Independent GPT-6 Astra review is pending.

This pass completes packaging only. This pass is not financial validation. This pass is not runtime, proof, or source conformance.

## Timeout provenance

The design worker wrote pretty-printed credit.json at 187564 bytes. Process exit code 124. Elapsed 892.1841544149793s of charged 900s. Frozen copy is interrupted-credit.json. Manifest status is timeout-incomplete-over-budget-not-accepted. SHA-256 before correction is 97578f05c3cf33bad94c4827cdf59b1d1148194a214f2ad40e57673d0fadb77f.

The completion worker also exited 124. Elapsed 292.21378911699867s of charged 300s. Git status SHA-256 did not change. The live fragment still equalled the frozen file.

## Correction

The pass swapped amount and asset on the 40 listed quantity paths. Amount is the original numeric asset value. Asset is the original symbolic amount value. Every other parsed field is identical to the frozen interrupted input.

- listed swaps: 40
- other parsed fields identical: True
- json.dumps compact bytes: 113542
- expected dumps bytes: 113542
- dumps match: True
- file bytes with final newline: 113543

## Hashes and sizes

- interrupted-credit.json SHA-256: 97578f05c3cf33bad94c4827cdf59b1d1148194a214f2ad40e57673d0fadb77f (187564 bytes)
- credit.json after SHA-256: fc7cc675c0b0a76e8e393e9d899da46814c6c276077e29827495de3d35692de6 (113543 bytes)
- input-packet.json SHA-256: b6a72b079f4b64f9eae2ca89d048b5d1e67d65b6f2c8d57c54cdc73d18b66dcf
- serialization: Python json.dumps separators comma/colon, ensure_ascii=False, plus final newline

## Identifier checks

- 5 row ids match packet: True ['accepted-refinance', 'pending-redemption', 'refinance', 'pending-redemption', 'bad-debt']
- 5 qualified row ids match: True ['heldouts:accepted-refinance', 'heldouts:pending-redemption', 'intent-cases:refinance', 'intent-cases:pending-redemption', 'DeFi-regressions:bad-debt']
- 5 trace ids match packet: True ['heldouts:accepted-refinance:trace', 'heldouts:pending-redemption:trace', 'intent-cases:refinance:trace', 'intent-cases:pending-redemption:trace', 'DeFi-regressions:bad-debt:trace']
- original mutation ids present: True
- packet mutation ids: ['heldouts:accepted-refinance:mut:discharge-without-settlement', 'heldouts:pending-redemption:mut:erase-claim-on-illiquidity', 'intent-cases:refinance:mut:temp-uncollateralized', 'intent-cases:pending-redemption:mut:skip-to-settled', 'DeFi-regressions:bad-debt:mut:erase-or-omit-socialized-loss']
- fragment mutation ids: ['heldouts:accepted-refinance:mut:discharge-without-settlement', 'heldouts:accepted-refinance:mut:fee-untransferred', 'intent-cases:refinance:mut:temp-uncollateralized', 'intent-cases:refinance:mut:new-debt-without-authority', 'heldouts:pending-redemption:mut:erase-claim-on-illiquidity', 'heldouts:pending-redemption:mut:replay-claim', 'intent-cases:pending-redemption:mut:skip-to-settled', 'intent-cases:pending-redemption:mut:drop-exact-floor', 'DeFi-regressions:bad-debt:mut:erase-or-omit-socialized-loss', 'DeFi-regressions:bad-debt:mut:discharge-residual-without-auth']

## Quantity checks

All 40 pre/post account quantities are canonical integer strings with named assets. Non-account quantity canonical failures: 0.

## Independent cash replay

The pass recomputed transfers and explicit burns for refinance, redemption, and badDebt. The pass did not run verify-credit.py. The pass did not evaluate untrusted command strings. The pass did not evaluate the 35 verification expressions.

- refinance: replay-matches-recorded-post. transfers 4. burns 0. checks 4. failures 0.
- redemption: fail. transfers 2. burns 2. checks 5. failures 1.
  - failure: step2/n0/share-burn key missing ('FundShares', 'supply', 'shares', '1')
- badDebt: replay-matches-recorded-post. transfers 2. burns 0. checks 4. failures 0.

## Local refs

JSON pointers resolved: 124. JSON pointers unresolved: 0. Id refs resolved: 60. Id refs unresolved: 0. Non-local source pointer labels ignored: 14.

- no unresolved local json-pointer or id ref recorded

## Gaps

- oracle-cash-redemption status=fail failures=step2/n0/share-burn key missing ('FundShares', 'supply', 'shares', '1')
- fragment-remainingGap gap:aave-morpho-primary
- fragment-remainingGap gap:erc7540-primary
- fragment-remainingGap gap:morpho-bad-debt-primary
- fragment-remainingGap gap:runtime
- fragment-remainingGap gap:review
- independent-root-verifier-not-run /home/charl/.local/state/moriarty/sp01-map-fragments-20260908/verify-credit.py
- pending-independent-gpt6-review

## Owned files

- experiments/moriarty-language/spec/successor/financial-fragments/credit.json
- FOREMAN_REPORT.md
- FOREMAN_REPORT.json

No git write. No network. No proof. No build. Combined owned output must be <= 131072 bytes.
