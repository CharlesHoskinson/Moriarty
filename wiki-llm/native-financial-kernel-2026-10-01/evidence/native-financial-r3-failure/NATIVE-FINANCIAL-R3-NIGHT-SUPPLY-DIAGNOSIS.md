# Failed R3 native apply: NIGHT supply diagnosis and cheap reproducer proposal

2026-10-01. Systematic-debugging skill loaded: preserve actual failure, trace producer→invariant, compare official working funding, propose isolated causal check before correction. Only failed-result freeze/this report/proposed Rust diagnostic source written. No native/Cargo/runtime/proof/verify/network/wallet/Compact execution or candidate/key/binary edits.

## Frozen actual failure

NATIVE-FINANCIAL-R3-FAILED-RESULT-FREEZE.json SHA6ed8f63baad2d9c0933e41bf081894b158cbadc95392ebe4f82fbd2d047f91e5 covers45 file identities plus exact current ELF7430…/696902872bytes. Authorizations/reviews/config/source/input/wrapper/attempt/log/receipt/all17outputs and candidate sources are retained. Proof6336bytes, statement2275bytes and signed transaction exist. Actual child terminal1/no resource stop,203.82358197s/269.11CPU/2910072832peakRSS. Application-result is exact Failure(InvariantViolation(NightBalance(24001000000000000))). poststate.tagged is byte-identical5227-byte genesis63f008…. No full-success receipt/replay-refusal or independent verifier. Verification is blocked; consumed proof cannot be rebound/retried.

Pinned caller uses real defaultReal well_formed before apply and records application result/poststate before refusing non-Success. Thus this failure occurs at native application, beyond prior unsorted-WF refusal. Raw proof/file existence is not acceptance or independent verification.

## Source-grounded causal chain

Official source ledger9a8777c4d035fc7f38ae286bcf5f8656668efd9f:

- ledger/src/structure.rs:3361 defines MAX_SUPPLY=24billion*1million=24000000000000000 atomic NIGHT.
- structure.rs:3367 new(network) calls with_genesis_settings(network,INITIAL_PARAMETERS,locked0,reserveMAX_SUPPLY,treasury0). The full supply is initially in **reserve**, not treasury/locked.
- Current unchanged candidate genesis creates LedgerState::new, adds A1 escrow then manually inserts one NIGHT UTXO1e12 with creation timestamp0; reserve remainsMAX_SUPPLY. A1 is excluded from NIGHT annotation.
- semantics.rs:529 check_night_balance_invariant sums NIGHT UTXO annotation+locked+reserve+block_rewards+treasuryNIGHT+unclaimed+bridge+contractNIGHT. It requires exactMAX_SUPPLY.
- Consequently source fixture total is24000000000000000+1000000000000=24001000000000000, exactly the observed native failure. The original genesis is already supply-inconsistent. A balanced spend/return of that same NIGHT principal cannot remove the excess supply.
- semantics.rs:1272 checks invariant during apply_section. apply:1343 returns original self on guaranteed application error. Byte-identical poststate/genesis corroborates this actual guaranteed failure; it does not establish blanket rollback for future fallible errors.

This is causal source evidence, with exact failure arithmetic. No native invariant check on retained genesis was newly executed. The earlier suggestion of treasury decrement was wrong: default treasury is0. Changing fees, disabling invariant, private-field workaround or changing proof success flags would not repair authorized supply provenance.

## Official coherent funding routes inspected

semantics.rs:981 SystemTransaction::DistributeReserve checks amount<=reserve, debits reserve and credits block_reward_pool, then native invariant. DistributeNight(Reward):583 debits block reward pool and credits unclaimed_block_rewards; subsequent actual claim path transfers that allocated value into a spendable UTXO. These are public typed transitions, not newly created supply. Official simple_block_rewards_claim test near2220 starts with full supply in unclaimed rewards/reserve0; its semantic claim transfers the allocation. Its unsigned internal test helper is not evidence of production signature/WF acceptance.

Official bridge_transfer_passes_invariant test:2407 starts with with_genesis_settings(locked amount,reserveMAX-amount,treasury0), then DistributeNight(CardanoBridge). It moves locked allocation into bridge_receiving and treasury bridge fees, and checks invariant. Native claim then draws from bridge allocation. Distribution alone is not a spendable UTXO, and real bridge/claim signatures, fees, timestamps and asset provenance remain obligations. Disabled treasury payouts in semantics are not usable funding APIs.

A supply-consistent conditional genesis can be explicit, but it is not authenticated public genesis/funding/deployment. Any successor must run native invariant at genesis preparation before circuit modeling/proving and preserve exact supply debit provenance. A changed genesis may alter context/binding/material; do not reuse/rebind this retained proof. No broad financial caller correction is authored here.

## Proposed minimal isolated native reproducer

New night-supply-diagnostic.rs is source-only. It consumes exact retained failed genesis path/SHA63f008…, checks bounded8MiB read, strict taggedEOF/canonical byte identity, then calls **actual** LedgerState.check_night_balance_invariant expecting typed NightBalance(MAX+1e12). It checks actual default LedgerState::new invariant good; removes the sole excess NIGHT UTXO only in an isolated clone as causal control and checks native invariant good; rechecks original bad-again and unchanged state. It produces no corrected funded fixture/proof/transaction. No native success boolean can substitute.

Suggested build packaging, after fresh source/resource review: copy unchanged existing Cargo/lock into a distinct diagnostic package directory with the same package identity/dependencies and add one [[bin]] name beta-night-supply-diagnostic/path to this source. Do not generate lock or add dependencies. Root must freeze exact manifest/source/input closure first; this report does not create or authorize that packet. Separate binary means original financial ELF is not overwritten. No costly IR construction/keygen/proof imported or called by the diagnostic.

Proposed one offline existing-target build120wall/240CPU/jobs2/4GiBRSS-AS, namespace/no fetch/locked/offline, existing10GiBtotal/incremental2GiBtarget+1GiBcache/freefloor10GiB. One isolated diagnostic30wall/60CPU/2GiBRSS-AS/output1MiB/perfile8MiB, only exact5227-byte genesis input, exclusive durable attempt/log/receipt/output ownership. No SRS/keys/proof/verify/application/network/signatures. Source/APIs are inspected, uncompiled; cheap native check may reveal type/API issues. No actual command may run until exact source/resource reviews and root phase authorization. Any diagnostic failure remains consumed, not retry permission.

Expected native outcomes are exact typed bad supply, default good, excess-removal good, original bad again. They establish this accounting cause only, never full financial acceptance or authorized funding. Broader source-owner/genesis/registry/time/history/generic compiler correspondence and mandatory Preview settlement remain open.
