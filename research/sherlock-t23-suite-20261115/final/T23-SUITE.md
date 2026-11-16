# T23 suite table (final run on the T25 keys)

Built 2026-11-16T14:43:40-0700 from the box returns /home/hoskinson/.cache/t23/final/ret-huginn, /home/hoskinson/.cache/t23/final/ret-odin, /home/hoskinson/.cache/t23/final/ret-loki; ledger check /home/hoskinson/.cache/t23/final/ledger; Preview settles 10.

Row count: 208 rows = 62 refused-as-native + 1 n/a + 3 BLOCKED (chip set) + 142 provable rows over 145 call files (equal to the plan).

| result | calls |
|---|---:|
| pass | 142 |
| refused-as-native | 62 |
| mismatch | 0 |
| blocked | 3 |
| prove-failed | 0 |
| n/a | 1 |
| total | 208 |

Compared after a proof: 142 rows; against the native Preview transcript 95, against the native ref only 47 (no passing native Preview transcript).

Blocked, by reason:

| reason | calls |
|---|---:|
| BLOCKED: secp256k1 chip set deferred (T18 (d) LEAD PICK) | 2 |
| BLOCKED: keccak256 tier 3 chip set deferred (T18 (d) LEAD PICK) | 1 |

| program | call | box | bucket | keys | ledger 9.1.0.1 | compare | Preview settle | result | reason |
|---|---|---|---|---|---|---|---|---|---|
| `a01-adt-counter` | `test_all` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `a01-adt-counter` | `test_big_increment` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `a02-set-field` | `test` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `a03-map-field-field` | `test` | loki | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `a04-map-bool-map-counter` | `test` | loki | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `a05-list-field` | `test` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | acceptRun block 140183, 3 refused | pass | W = on-chain words |
| `a06-merkletree-field` | `test-default-path` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `a07-historic-merkletree` | `test-default-path` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `a08-kernel` | `01-test` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d01-example-bboard` | `01-alice-post` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d01-example-bboard` | `02-bob-post-occupied` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d01-example-bboard` | `03-bob-takedown-not-owner` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d01-example-bboard` | `04-alice-takedown` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d01-example-bboard` | `05-alice-takedown-empty` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d01-example-bboard` | `06-bob-post` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d01-example-bboard` | `07-alice-takedown-not-owner` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d01-example-bboard` | `08-bob-takedown` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d02-battleship` | `01-alice-accept-self` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d02-battleship` | `02-bob-accept` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d02-battleship` | `03-bob-shoot-out-of-turn` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d02-battleship` | `04-alice-shoot-5` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d02-battleship` | `05-bob-claims-miss-on-hit` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d02-battleship` | `06-bob-check-hit` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d02-battleship` | `07-bob-shoot-4` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d02-battleship` | `08-alice-check-miss` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d02-battleship` | `09-alice-shoot-6` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d02-battleship` | `10-bob-check-miss` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d02-battleship` | `11-bob-shoot-3` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d02-battleship` | `12-alice-check-hit` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d02-battleship` | `13-alice-shoot-9` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d02-battleship` | `14-bob-check-hit-alice-wins` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d02-battleship` | `15-bob-shoot-after-win` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d03-micro-dao` | `01-buy-in-zero` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d03-micro-dao` | `02-alice-buy-in-empty-pot` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d03-micro-dao` | `03-set-topic-merge-pot` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d03-micro-dao` | `04-bob-buy-in-merge-pot` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d03-micro-dao` | `05-dao-voting-token` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = native ref words (on-chain words equal native after address substitution) |
| `d03-micro-dao` | `06-carol-buy-in-two` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d03-micro-dao` | `07-alice-commit-yes` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d03-micro-dao` | `08-alice-double-commit` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d03-micro-dao` | `09-bob-commit-no` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d03-micro-dao` | `10-carol-commit-yes` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d03-micro-dao` | `11-alice-advance-unauthorized` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d03-micro-dao` | `12-advance-to-reveal` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d03-micro-dao` | `13-alice-reveal-yes` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d03-micro-dao` | `14-alice-double-reveal` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d03-micro-dao` | `15-carol-reveal-flipped` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d03-micro-dao` | `16-bob-reveal-no` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d03-micro-dao` | `17-carol-reveal-yes` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d03-micro-dao` | `18-advance-to-final` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d03-micro-dao` | `19-advance-final-blocked-by-cashout` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d03-micro-dao` | `20-beneficiary-cash-out` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d03-micro-dao` | `21-advance-in-setup` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d03-micro-dao` | `22-round2-set-topic-empty-pot` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d03-micro-dao` | `23-round2-advance-to-reveal` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d03-micro-dao` | `24-round2-advance-to-final` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d03-micro-dao` | `25-round2-cash-out-no-majority` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d03-micro-dao` | `26-round2-advance-final-reset` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d04-coracle` | `01-red-bad-deposit` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d04-coracle` | `02-red-start` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d04-coracle` | `03-blue-start` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d04-coracle` | `04-third-start` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d04-coracle` | `05-stranger-guess` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d04-coracle` | `06-red-guess-out-of-turn` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d04-coracle` | `07-blue-guess-7` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d04-coracle` | `08-red-guess-5` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d04-coracle` | `09-blue-concede` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d04-coracle` | `10-blue-withdraw-lost` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d04-coracle` | `11-red-withdraw` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `d06-nft` | `01-ownerOf-before-mint` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d06-nft` | `02-mint-1-to-self-new-holder` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d06-nft` | `03-mint-2-to-self-existing-holder` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d06-nft` | `04-mint-3-to-bob` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d06-nft` | `05-mint-by-non-admin` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d06-nft` | `06-mint-duplicate-id` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d06-nft` | `07-balanceOf-self` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d06-nft` | `08-balanceOf-unknown` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d06-nft` | `09-ownerOf-1` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d06-nft` | `10-approve-bob-for-1` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d06-nft` | `11-approve-on-bobs-token` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d06-nft` | `12-getApproved-1` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d06-nft` | `13-setApprovalForAll-bob` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d06-nft` | `14-isApprovedForAll-self-bob` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d06-nft` | `15-isApprovedForAll-unset` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d06-nft` | `16-transfer-1-to-bob` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | acceptRun block 140308 | pass | W = on-chain words |
| `d06-nft` | `17-transfer-bobs-token` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `d06-nft` | `18-transferFrom-2-to-carol` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d06-nft` | `19-burn-3` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d06-nft` | `20-rotate-admin` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `d06-nft` | `21-old-admin-burn` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `h01-election` | `add-voter-not-authority` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `h01-election` | `advance-not-authority` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `h01-election` | `advance-zero-key` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `h01-election` | `set-topic-not-authority` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `h01-election` | `vote-commit-in-setup` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `h01-election` | `vote-reveal-in-setup` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `h02-external-hashes` | `nativeMethods` | - | - | - | - | - | - | n/a | pure circuit: no ZKIR, no proof, no transaction |
| `h03-election-ctor` | `add-voter-31-twice` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `h03-election-ctor` | `add-voter-31` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | acceptRun block 140430 | pass | W = on-chain words |
| `h03-election-ctor` | `add-voter-32` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `h03-election-ctor` | `add-voter-33` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `h03-election-ctor` | `add-voter-after-setup` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `h03-election-ctor` | `advance-not-authority` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `h03-election-ctor` | `advance-to-commit` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `h03-election-ctor` | `advance-to-final` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `h03-election-ctor` | `advance-to-reveal` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `h03-election-ctor` | `set-topic-not-authority` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `h03-election-ctor` | `set-topic` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `h03-election-ctor` | `vote-commit-31-again` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `h03-election-ctor` | `vote-commit-31-yes` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `h03-election-ctor` | `vote-commit-32-no` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `h03-election-ctor` | `vote-commit-33-yes` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `h03-election-ctor` | `vote-commit-before-commit-phase` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `h03-election-ctor` | `vote-commit-not-eligible` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `h03-election-ctor` | `vote-reveal-31-again` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `h03-election-ctor` | `vote-reveal-31-yes` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `h03-election-ctor` | `vote-reveal-32-no` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `h03-election-ctor` | `vote-reveal-33-claims-no` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `h03-election-ctor` | `vote-reveal-33-yes` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `j02-schnorr` | `verify-identity-pk` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `j02-schnorr` | `verify-valid` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | acceptRun block 141386 | pass | W = on-chain words |
| `j02-schnorr` | `verify-wrong-msg` | loki | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `s01-natives` | `conv` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `s01-natives` | `ec` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `s01-natives` | `pc` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `s01-natives` | `ph` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `s01-natives` | `tc` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `s01-natives` | `th` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `s01-natives` | `th3` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `s02-foreign` | `ecdsa-bad` | - | - | - | - | - | - | blocked | BLOCKED: secp256k1 chip set deferred (T18 (d) LEAD PICK) |
| `s02-foreign` | `ecdsa-valid` | - | - | - | - | - | - | blocked | BLOCKED: secp256k1 chip set deferred (T18 (d) LEAD PICK) |
| `s02-foreign` | `keccak` | - | - | - | - | - | - | blocked | BLOCKED: keccak256 tier 3 chip set deferred (T18 (d) LEAD PICK) |
| `s03-language` | `loop` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | acceptRun block 140963 | pass | W = on-chain words |
| `s03-language` | `record-high` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `s03-language` | `record-low` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `s03-language` | `record-too-large` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `s03-language` | `wide-ge` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `s03-language` | `wide-lt` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `t01-tiny` | `clear-old-owner` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `t01-tiny` | `clear-owner` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `t01-tiny` | `clear-wrong-key` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `t01-tiny` | `get-set-again` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `t01-tiny` | `get-set` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `t01-tiny` | `get-unset` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `t01-tiny` | `set-other` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `t01-tiny` | `set-while-set` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `t02-counter-example` | `increment` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | acceptRun block 136144, 3 refused | pass | W = on-chain words |
| `t03-counter-compiler` | `increment` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `t04-welcome` | `add-organizer-k2` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `t04-welcome` | `add-participant-alice` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `t04-welcome` | `add-participant-by-k2` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `t04-welcome` | `add-participant-no-key` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `t04-welcome` | `add-participant-not-organizer` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `t04-welcome` | `check-in-alice` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `t04-welcome` | `check-in-bob` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `t04-welcome` | `check-in-dave` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `t04-welcome` | `check-in-mallory` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `w02-bboard-testcenter` | `01-alice-post` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `w02-bboard-testcenter` | `02-bob-post-occupied` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `w02-bboard-testcenter` | `03-bob-takedown-not-owner` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `w02-bboard-testcenter` | `04-alice-takedown` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `w02-bboard-testcenter` | `05-alice-takedown-empty` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `w02-bboard-testcenter` | `06-bob-post` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `w02-bboard-testcenter` | `07-alice-takedown-not-owner` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `w02-bboard-testcenter` | `08-bob-takedown` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `x01-composable-outer` | `add-cross-contract` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | acceptRun block 140557; acceptRun block 140688 | pass | W = native ref words (on-chain words equal native after address substitution); W = on-chain words |
| `x01-composable-outer` | `add-via-arg-cross-contract` | huginn+loki | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = native ref words (on-chain words equal native after address substitution); W = on-chain words |
| `x01-composable-outer` | `add-via-arg-unbound-callee` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `x01-composable-outer` | `add-via-const-cross-contract` | huginn+loki | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = native ref words (on-chain words equal native after address substitution); W = on-chain words |
| `x01-composable-outer` | `echo-stored-after-set` | loki | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = native ref words (on-chain words equal native after address substitution) |
| `x01-composable-outer` | `echo-stored-initial` | loki | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `x01-composable-outer` | `set-inner-b` | loki | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = native ref words (on-chain words equal native after address substitution) |
| `x02-composable-inner` | `add` | huginn | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `z01-zerocash` | `mint-c1` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `z01-zerocash` | `mint-c2` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `z01-zerocash` | `spend-c1-again` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `z01-zerocash` | `spend-c1` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `z01-zerocash` | `spend-c2-wrong-path` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `z01-zerocash` | `spend-c3` | huginn | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `z02-unshielded-tokens` | `getUnshieldedBalanceGtTest` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `z02-unshielded-tokens` | `getUnshieldedBalanceLtTest` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `z02-unshielded-tokens` | `getUnshieldedBalanceTest` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `z02-unshielded-tokens` | `mintShieldedToContractTest` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `z02-unshielded-tokens` | `mintShieldedToSelfTest` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `z02-unshielded-tokens` | `mintShieldedToUserTest` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `z02-unshielded-tokens` | `mintUnshieldedToContractTest` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `z02-unshielded-tokens` | `mintUnshieldedToSelfTest` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | acceptRun block 140825 | pass | W = native ref words (on-chain words equal native after address substitution) |
| `z02-unshielded-tokens` | `mintUnshieldedToUserTest` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = native ref words (on-chain words equal native after address substitution) |
| `z02-unshielded-tokens` | `receiveShieldedTest` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `z02-unshielded-tokens` | `receiveUnshieldedTest` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `z02-unshielded-tokens` | `sendImmediateShieldedToContractTest` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `z02-unshielded-tokens` | `sendImmediateShieldedToSelfTest` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `z02-unshielded-tokens` | `sendImmediateShieldedToUserTest` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `z02-unshielded-tokens` | `sendShieldedToContractTest` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `z02-unshielded-tokens` | `sendShieldedToSelfTest` | odin | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `z02-unshielded-tokens` | `sendShieldedToUserTest` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `z02-unshielded-tokens` | `sendUnshieldedToContractTest` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `z02-unshielded-tokens` | `sendUnshieldedToSelfTest` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = native ref words (on-chain words equal native after address substitution) |
| `z02-unshielded-tokens` | `sendUnshieldedToUserTest` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
| `z03-block-time` | `testBlockTimeGt` | odin | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | acceptRun block 141480 | pass | W = on-chain words |
| `z03-block-time` | `testBlockTimeGte` | loki | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `z03-block-time` | `testBlockTimeLt` | loki | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `z03-block-time` | `testBlockTimeLte` | loki | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `z04-set-qualified-coin` | `qualified-roundtrip-again` | loki | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `z04-set-qualified-coin` | `qualified-roundtrip` | loki | Plain | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal | - | pass | W = on-chain words |
| `z04-set-qualified-coin` | `shielded-coin-not-received` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `z04-set-qualified-coin` | `shielded-coin-only-first-received` | - | - | - | - | - | - | refused-as-native | DERIVED, not run: native refuses before proving (no tape) |
| `z04-set-qualified-coin` | `shielded-coin-received` | loki | Sha | T25SUITE EQUAL, wrap.vk 6faac46f | Ok | equal (native ref only) | - | pass | native ref only (no passing native Preview transcript): W and comm equal the native ref |
