# T23 suite table (DRY RUN)

Run 2026-11-15T06:36:11-0700 to 2026-11-15T07:03:25-0700; prover `odin`, dispatch off, settle `dry`, Preview transactions sent: 0.

Settle rehearsal (SherlockLedger acceptRun built offline by t10a-ledger-run.mjs dry-run with the T17 stand-in arguments, no network, no wallet): ok; JS ledger accepts and applies: True, runs[claim_pub] = outcome_pub read back: True, replay refused: True, privacy scan: True.

Open T23 dependency: the map from a Mycroft statement (`[bind, comm] ++ anchor ++ W`) to the acceptRun arguments (c_p, input_pub, outcome_pub) is not defined yet; it comes with the T11 R_SL chip set and the wrap public-value layout. Live settle stays off until then.

Mycroft source: mycroft-factory d1e8eed8d plus 12 uncommitted paths (snapshot tree sha256 f9f71aa999eadf13); binary sha256 b9e2e7eabf491c68; native Preview receipts: `/home/hoskinson/Projects/Moriarty-wt/mycroft/mycroft/tests/compact-suite/preview`.

| result | calls |
|---|---:|
| pass | 0 |
| refused-as-native | 62 |
| mismatch | 0 |
| blocked | 145 |
| prove-failed | 0 |
| n/a | 1 |
| total | 208 |

Mismatch: 0 of 0 compared after a proof (the compare stage runs only on a proved outcome).

Blocked, by first blocking stage:

| reason | calls |
|---|---:|
| BLOCKED: R_SL chip set (T11) | 139 |
| BLOCKED: kit jubjub, in the T11 rekey | 3 |
| BLOCKED: kit secp256k1 deferred to a later chip set | 2 |
| BLOCKED: kit tier3 (keccak256) deferred to a later chip set | 1 |

Refused-as-native: 62 calls, DERIVED, not run (the native runtime refuses them before proving). 61 of them were refused on Preview as native on the T18 (a) run; the rest: `d04-coracle/04-third-start` (native Preview failed-before-submit).

Of the blocked calls, 47 also have no passing native Preview transcript (T18 (a) verdict not PASS), so they stay blocked after T11/T12 until the native run includes them.

Precheck without a proof (compiled W and comm against the native Preview transcript): 92 equal, 0 different, 47 without a passing native Preview transcript, of 139 calls with an emitted mycroft-call file.

| program | call | circuit | native | native Preview | compile | precheck | prove | result | reason |
|---|---|---|---|---|---|---|---|---|---|
| `a01-adt-counter` | `test_all` | `test_all` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `a01-adt-counter` | `test_big_increment` | `test_big_increment` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `a02-set-field` | `test` | `test` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `a03-map-field-field` | `test` | `test` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `a04-map-bool-map-counter` | `test` | `test` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `a05-list-field` | `test` | `test` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `a06-merkletree-field` | `test-default-path` | `test` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `a07-historic-merkletree` | `test-default-path` | `test` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `a08-kernel` | `01-test` | `test` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d01-example-bboard` | `01-alice-post` | `post` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d01-example-bboard` | `02-bob-post-occupied` | `post` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d01-example-bboard` | `03-bob-takedown-not-owner` | `takeDown` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d01-example-bboard` | `04-alice-takedown` | `takeDown` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d01-example-bboard` | `05-alice-takedown-empty` | `takeDown` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d01-example-bboard` | `06-bob-post` | `post` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d01-example-bboard` | `07-alice-takedown-not-owner` | `takeDown` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d01-example-bboard` | `08-bob-takedown` | `takeDown` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d02-battleship` | `01-alice-accept-self` | `acceptGame` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d02-battleship` | `02-bob-accept` | `acceptGame` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d02-battleship` | `03-bob-shoot-out-of-turn` | `player2Shoot` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d02-battleship` | `04-alice-shoot-5` | `player1Shoot` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d02-battleship` | `05-bob-claims-miss-on-hit` | `checkBoard2` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d02-battleship` | `06-bob-check-hit` | `checkBoard2` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d02-battleship` | `07-bob-shoot-4` | `player2Shoot` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d02-battleship` | `08-alice-check-miss` | `checkBoard1` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d02-battleship` | `09-alice-shoot-6` | `player1Shoot` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d02-battleship` | `10-bob-check-miss` | `checkBoard2` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d02-battleship` | `11-bob-shoot-3` | `player2Shoot` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d02-battleship` | `12-alice-check-hit` | `checkBoard1` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d02-battleship` | `13-alice-shoot-9` | `player1Shoot` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d02-battleship` | `14-bob-check-hit-alice-wins` | `checkBoard2` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d02-battleship` | `15-bob-shoot-after-win` | `player2Shoot` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d03-micro-dao` | `01-buy-in-zero` | `buy_in` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d03-micro-dao` | `02-alice-buy-in-empty-pot` | `buy_in` | ok | failed-before-submit | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d03-micro-dao` | `03-set-topic-merge-pot` | `set_topic` | ok | failed-before-submit | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d03-micro-dao` | `04-bob-buy-in-merge-pot` | `buy_in` | ok | failed-before-submit | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d03-micro-dao` | `05-dao-voting-token` | `dao_voting_token` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d03-micro-dao` | `06-carol-buy-in-two` | `buy_in` | ok | failed-before-submit | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d03-micro-dao` | `07-alice-commit-yes` | `vote_commit` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d03-micro-dao` | `08-alice-double-commit` | `vote_commit` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d03-micro-dao` | `09-bob-commit-no` | `vote_commit` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d03-micro-dao` | `10-carol-commit-yes` | `vote_commit` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d03-micro-dao` | `11-alice-advance-unauthorized` | `advance` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d03-micro-dao` | `12-advance-to-reveal` | `advance` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d03-micro-dao` | `13-alice-reveal-yes` | `vote_reveal` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d03-micro-dao` | `14-alice-double-reveal` | `vote_reveal` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d03-micro-dao` | `15-carol-reveal-flipped` | `vote_reveal` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d03-micro-dao` | `16-bob-reveal-no` | `vote_reveal` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d03-micro-dao` | `17-carol-reveal-yes` | `vote_reveal` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d03-micro-dao` | `18-advance-to-final` | `advance` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d03-micro-dao` | `19-advance-final-blocked-by-cashout` | `advance` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d03-micro-dao` | `20-beneficiary-cash-out` | `cash_out` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d03-micro-dao` | `21-advance-in-setup` | `advance` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d03-micro-dao` | `22-round2-set-topic-empty-pot` | `set_topic` | ok | failed-before-submit | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d03-micro-dao` | `23-round2-advance-to-reveal` | `advance` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d03-micro-dao` | `24-round2-advance-to-final` | `advance` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d03-micro-dao` | `25-round2-cash-out-no-majority` | `cash_out` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d03-micro-dao` | `26-round2-advance-final-reset` | `advance` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d04-coracle` | `01-red-bad-deposit` | `start` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d04-coracle` | `02-red-start` | `start` | ok | failed-before-submit | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d04-coracle` | `03-blue-start` | `start` | ok | failed-before-submit | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d04-coracle` | `04-third-start` | `start` | rejected | failed-before-submit | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: failed-before-submit |
| `d04-coracle` | `05-stranger-guess` | `guess` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d04-coracle` | `06-red-guess-out-of-turn` | `guess` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d04-coracle` | `07-blue-guess-7` | `guess` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d04-coracle` | `08-red-guess-5` | `guess` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d04-coracle` | `09-blue-concede` | `concede` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d04-coracle` | `10-blue-withdraw-lost` | `withdraw` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d04-coracle` | `11-red-withdraw` | `withdraw` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d06-nft` | `01-ownerOf-before-mint` | `ownerOf` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d06-nft` | `02-mint-1-to-self-new-holder` | `mintAdmin` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d06-nft` | `03-mint-2-to-self-existing-holder` | `mintAdmin` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d06-nft` | `04-mint-3-to-bob` | `mintAdmin` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d06-nft` | `05-mint-by-non-admin` | `mintAdmin` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d06-nft` | `06-mint-duplicate-id` | `mintAdmin` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d06-nft` | `07-balanceOf-self` | `balanceOf` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d06-nft` | `08-balanceOf-unknown` | `balanceOf` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d06-nft` | `09-ownerOf-1` | `ownerOf` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d06-nft` | `10-approve-bob-for-1` | `approve` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d06-nft` | `11-approve-on-bobs-token` | `approve` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d06-nft` | `12-getApproved-1` | `getApproved` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d06-nft` | `13-setApprovalForAll-bob` | `setApprovalForAll` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d06-nft` | `14-isApprovedForAll-self-bob` | `isApprovedForAll` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d06-nft` | `15-isApprovedForAll-unset` | `isApprovedForAll` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d06-nft` | `16-transfer-1-to-bob` | `transfer` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d06-nft` | `17-transfer-bobs-token` | `transfer` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `d06-nft` | `18-transferFrom-2-to-carol` | `transferFrom` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d06-nft` | `19-burn-3` | `burnAdmin` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d06-nft` | `20-rotate-admin` | `rotateAdmin` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `d06-nft` | `21-old-admin-burn` | `burnAdmin` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `h01-election` | `add-voter-not-authority` | `add_voter` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `h01-election` | `advance-not-authority` | `advance` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `h01-election` | `advance-zero-key` | `advance` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `h01-election` | `set-topic-not-authority` | `set_topic` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `h01-election` | `vote-commit-in-setup` | `vote$commit` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `h01-election` | `vote-reveal-in-setup` | `vote$reveal` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `h02-external-hashes` | `nativeMethods` | `nativeMethods` | ok | pure | - | - | - | n/a | pure circuit: no ZKIR, no proof, no transaction |
| `h03-election-ctor` | `add-voter-31-twice` | `add_voter` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `h03-election-ctor` | `add-voter-31` | `add_voter` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `h03-election-ctor` | `add-voter-32` | `add_voter` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `h03-election-ctor` | `add-voter-33` | `add_voter` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `h03-election-ctor` | `add-voter-after-setup` | `add_voter` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `h03-election-ctor` | `advance-not-authority` | `advance` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `h03-election-ctor` | `advance-to-commit` | `advance` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `h03-election-ctor` | `advance-to-final` | `advance` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `h03-election-ctor` | `advance-to-reveal` | `advance` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `h03-election-ctor` | `set-topic-not-authority` | `set_topic` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `h03-election-ctor` | `set-topic` | `set_topic` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `h03-election-ctor` | `vote-commit-31-again` | `vote$commit` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `h03-election-ctor` | `vote-commit-31-yes` | `vote$commit` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `h03-election-ctor` | `vote-commit-32-no` | `vote$commit` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `h03-election-ctor` | `vote-commit-33-yes` | `vote$commit` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `h03-election-ctor` | `vote-commit-before-commit-phase` | `vote$commit` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `h03-election-ctor` | `vote-commit-not-eligible` | `vote$commit` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `h03-election-ctor` | `vote-reveal-31-again` | `vote$reveal` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `h03-election-ctor` | `vote-reveal-31-yes` | `vote$reveal` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `h03-election-ctor` | `vote-reveal-32-no` | `vote$reveal` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `h03-election-ctor` | `vote-reveal-33-claims-no` | `vote$reveal` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `h03-election-ctor` | `vote-reveal-33-yes` | `vote$reveal` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `j02-schnorr` | `verify-identity-pk` | `verifySchnorrN3` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `j02-schnorr` | `verify-valid` | `verifySchnorrN3` | ok | included | kit refused | - | - | blocked | BLOCKED: kit jubjub, in the T11 rekey |
| `j02-schnorr` | `verify-wrong-msg` | `verifySchnorrN3` | ok | included | kit refused | - | - | blocked | BLOCKED: kit jubjub, in the T11 rekey |
| `s01-natives` | `conv` | `conv` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `s01-natives` | `ec` | `ec` | ok | included | kit refused | - | - | blocked | BLOCKED: kit jubjub, in the T11 rekey |
| `s01-natives` | `pc` | `pc` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `s01-natives` | `ph` | `ph` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `s01-natives` | `tc` | `tc` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `s01-natives` | `th` | `th` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `s01-natives` | `th3` | `th3` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `s02-foreign` | `ecdsa-bad` | `ecdsa` | ok | included | kit refused | - | - | blocked | BLOCKED: kit secp256k1 deferred to a later chip set |
| `s02-foreign` | `ecdsa-valid` | `ecdsa` | ok | included | kit refused | - | - | blocked | BLOCKED: kit secp256k1 deferred to a later chip set |
| `s02-foreign` | `keccak` | `keccak` | ok | included | kit refused | - | - | blocked | BLOCKED: kit tier3 (keccak256) deferred to a later chip set |
| `s03-language` | `loop` | `loop` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `s03-language` | `record-high` | `record` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `s03-language` | `record-low` | `record` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `s03-language` | `record-too-large` | `record` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `s03-language` | `wide-ge` | `wide` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `s03-language` | `wide-lt` | `wide` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `t01-tiny` | `clear-old-owner` | `clear` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `t01-tiny` | `clear-owner` | `clear` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `t01-tiny` | `clear-wrong-key` | `clear` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `t01-tiny` | `get-set-again` | `get` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `t01-tiny` | `get-set` | `get` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `t01-tiny` | `get-unset` | `get` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `t01-tiny` | `set-other` | `set` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `t01-tiny` | `set-while-set` | `set` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `t02-counter-example` | `increment` | `increment` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `t03-counter-compiler` | `increment` | `increment` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `t04-welcome` | `add-organizer-k2` | `add_organizer` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `t04-welcome` | `add-participant-alice` | `add_participant` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `t04-welcome` | `add-participant-by-k2` | `add_participant` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `t04-welcome` | `add-participant-no-key` | `add_participant` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `t04-welcome` | `add-participant-not-organizer` | `add_participant` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `t04-welcome` | `check-in-alice` | `check_in` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `t04-welcome` | `check-in-bob` | `check_in` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `t04-welcome` | `check-in-dave` | `check_in` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `t04-welcome` | `check-in-mallory` | `check_in` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `w02-bboard-testcenter` | `01-alice-post` | `post` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `w02-bboard-testcenter` | `02-bob-post-occupied` | `post` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `w02-bboard-testcenter` | `03-bob-takedown-not-owner` | `take_down` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `w02-bboard-testcenter` | `04-alice-takedown` | `take_down` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `w02-bboard-testcenter` | `05-alice-takedown-empty` | `take_down` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `w02-bboard-testcenter` | `06-bob-post` | `post` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `w02-bboard-testcenter` | `07-alice-takedown-not-owner` | `take_down` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `w02-bboard-testcenter` | `08-bob-takedown` | `take_down` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `x01-composable-outer` | `add-cross-contract` | `add` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `x01-composable-outer` | `add-via-arg-cross-contract` | `addViaArg` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `x01-composable-outer` | `add-via-arg-unbound-callee` | `addViaArg` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `x01-composable-outer` | `add-via-const-cross-contract` | `addViaConst` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `x01-composable-outer` | `echo-stored-after-set` | `echoStored` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `x01-composable-outer` | `echo-stored-initial` | `echoStored` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `x01-composable-outer` | `set-inner-b` | `setInner` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `x02-composable-inner` | `add` | `add` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z01-zerocash` | `mint-c1` | `zerocash_mint` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z01-zerocash` | `mint-c2` | `zerocash_mint` | ok | refused-or-failed-after-submit | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z01-zerocash` | `spend-c1-again` | `spend` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `z01-zerocash` | `spend-c1` | `spend` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z01-zerocash` | `spend-c2-wrong-path` | `spend` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `z01-zerocash` | `spend-c3` | `spend` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z02-unshielded-tokens` | `getUnshieldedBalanceGtTest` | `getUnshieldedBalanceGtTest` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z02-unshielded-tokens` | `getUnshieldedBalanceLtTest` | `getUnshieldedBalanceLtTest` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z02-unshielded-tokens` | `getUnshieldedBalanceTest` | `getUnshieldedBalanceTest` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z02-unshielded-tokens` | `mintShieldedToContractTest` | `mintShieldedToContractTest` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z02-unshielded-tokens` | `mintShieldedToSelfTest` | `mintShieldedToSelfTest` | ok | included | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z02-unshielded-tokens` | `mintShieldedToUserTest` | `mintShieldedToUserTest` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z02-unshielded-tokens` | `mintUnshieldedToContractTest` | `mintUnshieldedToContractTest` | ok | refused-or-failed-after-submit | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z02-unshielded-tokens` | `mintUnshieldedToSelfTest` | `mintUnshieldedToSelfTest` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z02-unshielded-tokens` | `mintUnshieldedToUserTest` | `mintUnshieldedToUserTest` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z02-unshielded-tokens` | `receiveShieldedTest` | `receiveShieldedTest` | ok | failed-before-submit | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z02-unshielded-tokens` | `receiveUnshieldedTest` | `receiveUnshieldedTest` | ok | failed-before-submit | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z02-unshielded-tokens` | `sendImmediateShieldedToContractTest` | `sendImmediateShieldedToContractTest` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z02-unshielded-tokens` | `sendImmediateShieldedToSelfTest` | `sendImmediateShieldedToSelfTest` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z02-unshielded-tokens` | `sendImmediateShieldedToUserTest` | `sendImmediateShieldedToUserTest` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z02-unshielded-tokens` | `sendShieldedToContractTest` | `sendShieldedToContractTest` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z02-unshielded-tokens` | `sendShieldedToSelfTest` | `sendShieldedToSelfTest` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z02-unshielded-tokens` | `sendShieldedToUserTest` | `sendShieldedToUserTest` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `z02-unshielded-tokens` | `sendUnshieldedToContractTest` | `sendUnshieldedToContractTest` | ok | refused-or-failed-after-submit | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z02-unshielded-tokens` | `sendUnshieldedToSelfTest` | `sendUnshieldedToSelfTest` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z02-unshielded-tokens` | `sendUnshieldedToUserTest` | `sendUnshieldedToUserTest` | ok | refused-or-failed-after-submit | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z03-block-time` | `testBlockTimeGt` | `testBlockTimeGt` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z03-block-time` | `testBlockTimeGte` | `testBlockTimeGte` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z03-block-time` | `testBlockTimeLt` | `testBlockTimeLt` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z03-block-time` | `testBlockTimeLte` | `testBlockTimeLte` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z04-set-qualified-coin` | `qualified-roundtrip-again` | `test_QualifiedShieldedCoinInfo` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z04-set-qualified-coin` | `qualified-roundtrip` | `test_QualifiedShieldedCoinInfo` | ok | included | W, comm, outputs equal to native ref | equal | blocked | blocked | BLOCKED: R_SL chip set (T11) |
| `z04-set-qualified-coin` | `shielded-coin-not-received` | `test_ShieldedCoinInfo` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `z04-set-qualified-coin` | `shielded-coin-only-first-received` | `test_ShieldedCoinInfo` | rejected | refused-locally | no native preimage | - | - | refused-as-native | DERIVED, not run: native refuses before proving, so the Mycroft host has no tape; native Preview: refused-locally |
| `z04-set-qualified-coin` | `shielded-coin-received` | `test_ShieldedCoinInfo` | ok | refused-locally | W, comm, outputs equal to native ref | no-native | blocked | blocked | BLOCKED: R_SL chip set (T11) |
