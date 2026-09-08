# SP01.7 RP01-MC03 fixed native-statement crosswalk

Status: pending-review. Candidate hash is null until parent freeze. This note is not RP01-MC03 completion, F2 admission, or Midnight network evidence. `raw/assignments/moriarty-midnight-milestone-testing-2026-09-07.md` still requires later Preview milestones. This task supplies none.

The retained relation is the original R3 two-step first-period loan. `episode.json` SHA-256 `2a2ab1d92256d5778044a8e9580c96d37dada1eaa78fbad64e63cea88db0635a` and `harness/episode.rs` keep the three 11-field rows and eight digest limbs. Original k17 row exhaustion stays failed evidence. The encoding-candidate README remains source-only. No native run occurred here.

## What the fixed relation commits

Host arithmetic in `moriarty_loan_r3.rs` `native_financial` recomputes the table from terms, not from expected-output copy alone. Floor interest is `5000000000 * 8 * 31 / (100 * 365) = 33972602` with remainder `27000/36500`. Principal installment is `500000000`. Settle total and max gross debit are `533972602`. Final cash is borrower `19466027398`, lender `533972602`. Notional `4500000000` stays outstanding. Remaining work is `2,1,0` with revisions `0,1,2`. Closed episode is not agreement discharge.

Encoding is UInt128 as low64 then high64, plus eight SHA-256 digests as four little-endian u64 limbs. That is 22 + 32 = 54 private limbs. Phase is the finite index `0/1/2`, not a hash. Checked encoding publishes only phase as application public input. Canonical VK and accumulator remain additional IVC public data. `circuit_transition` binds ordinary input phase below 2 to complete before-row constants and successor constants. This is a domain-restricted gadget argument. It is not an executed proof and it does not recompute SHA-256.

## What remains to bind to the accepted atomic loan

Current atomic acceptance is pinned as codec `c30aaf5c0d091b1efea7e042d0fcbd539ca6cc51`, evidence `b296ce95afae722177b5bba482d1d195fa334d1e`, candidate `2f1fb86407537720174bbf73cb50426499744986e97ce8a5dde9f792577e0b5e`. Numerical projection of the eleven fields matches `loan.mori`. Unit labels do not. Old denomination `USD` and asset `demo:USD6` are distinct from current `USD_micro` and `USD_TEST_ASSET`. Due identifiers rename `loan:demo:principal|interest` to `lam01:period1:PR|IP`. Accrue drops `event=PR_IP` and adds `obs.now`. Settle argument names become `settlement_asset` and `amount_due`. `closed` becomes `episode_closed`.

Old hashes are SHA-256 of R2 canonical JSON that embeds `kind`/`version`. Current hashes are domain prefix, one NUL, then canonical typed bytes from `bounds.json` `domainRegistry` and `codec.ts` `hashDomain`. The eight retained concrete hashes may be cited from `episode.json` rows. They are not current `programHash`, `stateHash`, `genesisHash`, `authorityDigest`, or related fields. Equality of numbers does not yield equality of hashes. Migration is not waived.

The old intent preimage is `FixedAuthorityExperiment` with `dynamicSignatureVerified: false` and `maxGrossDebit 533972602`. Current authority signs ExactPlan `ActionCall` plus exact writes/effects, or Outcome allowed actions and constraints, together with the common fields in `runtime-types.ts`. Observations are authenticated separately. `actionHash` and `observationsHash` are derived. Neither authority object signs `ObservationSet`. Carrying the fixed hashes does not prove signature, currentness, history, or custody.

Eleven numbers plus old effect lists are not `StateEnvelope`, `CompleteBody`, or `ProofContext`. Terminal status, remaining notional, obligation tombstones, and structural hash fields need a current export that does not yet exist. Contract, intent, transition, and history claims stay distinct. That missing atomic-context export and mapping is an acceptance blocker owned by MC03/F0a.

## Remaining admission work

Parent verifies these files and a fresh GPT-6 reviews them. Parent later writes `subsets.RP01-MC03` in `semantic-challenges.json`. This worker did not edit that file, to avoid loan/swap writer collision.

F2 still needs current atomic acceptance, the reviewed subset, all F1 probes, and MC03 RP03 admission. F0 no-go stays blocked and does not retry k17. Required invalid controls that accept stop the stage. Those tests were not run here.
