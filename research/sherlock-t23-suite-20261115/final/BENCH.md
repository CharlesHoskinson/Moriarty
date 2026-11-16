# T23 final run on the T25 keys

- derived_tx: 290

**End to end: no call has every stage measured (149 calls).** R_SL front (exec, app, leaf) median 1.1 s over 149 calls; Preview submit not measured in this run.

Times are wall seconds. A dash is a stage without a measurement (reason in the notes); a call has a total only when every stage is measured. Keys: cold = the process generated or loaded its keys in that run, warm = keys resident from a previous call.

## Per call

| program | call | box | R_SL exec + witness | R_SL app proof | leaf proof | N1 | N2 | N3 | L | L_Fr | k=21 wrap | ledger-check 9.1.0.1 | Preview submit -> inclusion | total |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `a01-adt-counter` | `test_all.test_all.0` | huginn | 0.00 | 0.52 | 0.49 | 0.44 | 0.40 | 0.39 | 2.73 | 25.7 | 47.5 | 0.10 | — [1] | — |
| `a01-adt-counter` | `test_big_increment.test_big_increment.0` | huginn | 0.00 | 0.52 | 0.48 | 0.43 | 0.41 | 0.39 | 2.81 | 20.6 | 47.9 | 0.07 | — [1] | — |
| `a02-set-field` | `test.test.0` | huginn | 0.00 | 0.51 | 0.48 | 0.43 | 0.39 | 0.41 | 2.74 | 21.7 | 47.5 | 0.07 | — [1] | — |
| `a03-map-field-field` | `test.test.0` | loki | 0.00 | 0.35 | 0.66 | 0.49 | 0.47 | 0.46 | 2.22 | 34.1 | 79.9 | 0.11 | — [1] | — |
| `a04-map-bool-map-counter` | `test.test.0` | loki | 0.00 | 0.33 | 0.67 | 0.49 | 0.45 | 0.46 | 2.17 | 39.4 | 79.7 | 0.10 | — [1] | — |
| `a05-list-field` | `test.test.0` | odin | 0.00 | 0.65 | 0.73 | 0.59 | 0.48 | 0.52 | 1.48 | 27.4 | 58.4 | 0.07 | 4.24 | — |
| `a08-kernel` | `test.01-test.0` | huginn | 0.00 | 0.52 | 0.51 | 0.46 | 0.40 | 0.40 | 2.76 | 20.7 | 47.5 | 0.06 | — [1] | — |
| `d01-example-bboard` | `post.01-alice-post.0` | huginn | 0.00 | 0.55 | 0.51 | 0.46 | 0.41 | 0.41 | 2.74 | 20.6 | 47.3 | 0.10 | — [1] | — |
| `d01-example-bboard` | `post.06-bob-post.0` | huginn | 0.00 | 0.56 | 0.49 | 0.46 | 0.41 | 0.40 | 2.80 | 24.0 | 47.3 | 0.11 | — [1] | — |
| `d01-example-bboard` | `takeDown.04-alice-takedown.0` | huginn | 0.00 | 0.56 | 0.49 | 0.46 | 0.40 | 0.41 | 2.74 | 22.9 | 47.4 | 0.09 | — [1] | — |
| `d01-example-bboard` | `takeDown.08-bob-takedown.0` | huginn | 0.00 | 0.55 | 0.49 | 0.47 | 0.40 | 0.41 | 2.78 | 22.5 | 47.3 | 0.08 | — [1] | — |
| `d02-battleship` | `acceptGame.02-bob-accept.0` | odin | 0.00 | 0.75 | 0.70 | 0.62 | 0.49 | 0.47 | 1.50 | 25.0 | 58.1 | 0.09 | — [1] | — |
| `d02-battleship` | `checkBoard1.08-alice-check-miss.0` | odin | 0.00 | 0.64 | 0.67 | 0.60 | 0.49 | 0.47 | 1.49 | 26.2 | 58.1 | 0.10 | — [1] | — |
| `d02-battleship` | `checkBoard1.12-alice-check-hit.0` | odin | 0.00 | 0.66 | 0.68 | 0.59 | 0.47 | 0.47 | 1.49 | 25.4 | 58.0 | 0.10 | — [1] | — |
| `d02-battleship` | `checkBoard2.06-bob-check-hit.0` | odin | 0.00 | 0.68 | 0.73 | 0.65 | 0.47 | 0.47 | 1.48 | 27.0 | 58.3 | 0.10 | — [1] | — |
| `d02-battleship` | `checkBoard2.10-bob-check-miss.0` | odin | 0.00 | 0.64 | 0.69 | 0.57 | 0.47 | 0.47 | 1.46 | 25.0 | 58.0 | 0.06 | — [1] | — |
| `d02-battleship` | `checkBoard2.14-bob-check-hit-alice-wins.0` | odin | 0.00 | 0.66 | 0.67 | 0.58 | 0.48 | 0.47 | 1.50 | 27.6 | 58.0 | 0.06 | — [1] | — |
| `d02-battleship` | `player1Shoot.04-alice-shoot-5.0` | huginn | 0.00 | 0.56 | 0.51 | 0.47 | 0.42 | 0.42 | 2.80 | 28.6 | 49.4 | 0.09 | — [1] | — |
| `d02-battleship` | `player1Shoot.04-alice-shoot-5.0` | loki | 0.00 | 0.36 | 0.66 | 0.49 | 0.50 | 0.51 | 2.12 | 32.1 | 77.0 | 0.09 | — [1] | — |
| `d02-battleship` | `player1Shoot.04-alice-shoot-5.0` | odin | 0.00 | 0.67 | 0.67 | 0.59 | 0.49 | 0.46 | 1.49 | 29.1 | 58.5 | 0.09 | — [1] | — |
| `d02-battleship` | `player1Shoot.09-alice-shoot-6.0` | odin | 0.00 | 0.66 | 0.70 | 0.56 | 0.48 | 0.44 | 1.48 | 26.7 | 58.1 | 0.07 | — [1] | — |
| `d02-battleship` | `player1Shoot.13-alice-shoot-9.0` | odin | 0.00 | 0.72 | 0.71 | 0.58 | 0.46 | 0.45 | 1.51 | 26.1 | 58.0 | 0.11 | — [1] | — |
| `d02-battleship` | `player2Shoot.07-bob-shoot-4.0` | odin | 0.00 | 0.71 | 0.67 | 0.60 | 0.49 | 0.46 | 1.45 | 26.3 | 58.0 | 0.06 | — [1] | — |
| `d02-battleship` | `player2Shoot.11-bob-shoot-3.0` | odin | 0.00 | 0.68 | 0.68 | 0.57 | 0.47 | 0.47 | 1.49 | 29.0 | 58.3 | 0.07 | — [1] | — |
| `d03-micro-dao` | `advance.12-advance-to-reveal.0` | huginn | 0.00 | 0.55 | 0.50 | 0.45 | 0.41 | 0.40 | 2.79 | 22.0 | 49.5 | 0.07 | — [1] | — |
| `d03-micro-dao` | `advance.18-advance-to-final.0` | huginn | 0.00 | 0.56 | 0.50 | 0.45 | 0.42 | 0.40 | 2.73 | 22.0 | 49.6 | 0.10 | — [1] | — |
| `d03-micro-dao` | `advance.23-round2-advance-to-reveal.0` | huginn | 0.00 | 0.56 | 0.51 | 0.49 | 0.41 | 0.40 | 2.78 | 23.0 | 49.4 | 0.06 | — [1] | — |
| `d03-micro-dao` | `advance.24-round2-advance-to-final.0` | huginn | 0.00 | 0.56 | 0.49 | 0.46 | 0.40 | 0.40 | 2.75 | 22.6 | 49.8 | 0.11 | — [1] | — |
| `d03-micro-dao` | `advance.26-round2-advance-final-reset.0` | huginn | 0.00 | 0.56 | 0.51 | 0.46 | 0.41 | 0.41 | 2.78 | 24.5 | 49.9 | 0.09 | — [1] | — |
| `d03-micro-dao` | `buy_in.02-alice-buy-in-empty-pot.0` | huginn | 0.01 | 0.61 | 0.51 | 0.46 | 0.40 | 0.39 | 2.71 | 21.6 | 49.7 | 0.09 | — [1] | — |
| `d03-micro-dao` | `buy_in.04-bob-buy-in-merge-pot.0` | huginn | 0.01 | 0.60 | 0.50 | 0.43 | 0.40 | 0.39 | 2.73 | 23.0 | 49.5 | 0.06 | — [1] | — |
| `d03-micro-dao` | `buy_in.06-carol-buy-in-two.0` | huginn | 0.01 | 0.62 | 0.48 | 0.45 | 0.41 | 0.39 | 2.77 | 22.3 | 49.5 | 0.10 | — [1] | — |
| `d03-micro-dao` | `cash_out.20-beneficiary-cash-out.0` | huginn | 0.00 | 0.57 | 0.51 | 0.46 | 0.40 | 0.41 | 2.75 | 20.5 | 49.8 | 0.09 | — [1] | — |
| `d03-micro-dao` | `dao_voting_token.05-dao-voting-token.0` | huginn | 0.00 | 0.55 | 0.50 | 0.45 | 0.42 | 0.40 | 2.76 | 22.5 | 49.5 | 0.07 | — [1] | — |
| `d03-micro-dao` | `set_topic.03-set-topic-merge-pot.0` | huginn | 0.00 | 0.61 | 0.51 | 0.44 | 0.40 | 0.39 | 2.76 | 22.7 | 49.5 | 0.10 | — [1] | — |
| `d03-micro-dao` | `set_topic.22-round2-set-topic-empty-pot.0` | huginn | 0.00 | 0.61 | 0.50 | 0.45 | 0.40 | 0.40 | 2.80 | 20.8 | 49.7 | 0.06 | — [1] | — |
| `d03-micro-dao` | `vote_commit.07-alice-commit-yes.0` | huginn | 0.00 | 0.58 | 0.51 | 0.45 | 0.41 | 0.41 | 2.75 | 24.4 | 49.2 | 0.07 | — [1] | — |
| `d03-micro-dao` | `vote_commit.09-bob-commit-no.0` | huginn | 0.00 | 0.57 | 0.50 | 0.45 | 0.41 | 0.40 | 2.75 | 30.6 | 49.3 | 0.06 | — [1] | — |
| `d03-micro-dao` | `vote_commit.10-carol-commit-yes.0` | huginn | 0.00 | 0.59 | 0.51 | 0.46 | 0.40 | 0.39 | 2.75 | 22.9 | 49.5 | 0.07 | — [1] | — |
| `d03-micro-dao` | `vote_reveal.13-alice-reveal-yes.0` | huginn | 0.01 | 0.63 | 0.51 | 0.45 | 0.39 | 0.39 | 2.79 | 21.1 | 49.5 | 0.07 | — [1] | — |
| `d03-micro-dao` | `vote_reveal.16-bob-reveal-no.0` | huginn | 0.01 | 0.63 | 0.51 | 0.46 | 0.41 | 0.41 | 2.84 | 22.6 | 49.3 | 0.06 | — [1] | — |
| `d03-micro-dao` | `vote_reveal.17-carol-reveal-yes.0` | huginn | 0.01 | 0.64 | 0.51 | 0.44 | 0.39 | 0.39 | 2.76 | 19.6 | 49.1 | 0.07 | — [1] | — |
| `d04-coracle` | `concede.09-blue-concede.0` | loki | 0.01 | 0.43 | 0.68 | 0.53 | 0.46 | 0.46 | 2.15 | 29.1 | 79.9 | 0.06 | — [1] | — |
| `d04-coracle` | `guess.07-blue-guess-7.0` | loki | 0.00 | 0.39 | 0.67 | 0.49 | 0.46 | 0.47 | 2.12 | 34.9 | 79.4 | 0.08 | — [1] | — |
| `d04-coracle` | `guess.08-red-guess-5.0` | loki | 0.00 | 0.39 | 0.65 | 0.50 | 0.52 | 0.52 | 2.13 | 37.6 | 79.7 | 0.07 | — [1] | — |
| `d04-coracle` | `start.02-red-start.0` | loki | 0.01 | 0.41 | 0.66 | 0.48 | 0.51 | 0.52 | 2.13 | 35.0 | 79.7 | 0.07 | — [1] | — |
| `d04-coracle` | `start.03-blue-start.0` | loki | 0.01 | 0.41 | 0.66 | 0.51 | 0.45 | 0.45 | 2.17 | 30.1 | 79.7 | 0.10 | — [1] | — |
| `d04-coracle` | `withdraw.11-red-withdraw.0` | loki | 0.01 | 0.42 | 0.68 | 0.53 | 0.49 | 0.45 | 2.16 | 30.0 | 79.4 | 0.07 | — [1] | — |
| `d06-nft` | `approve.10-approve-bob-for-1.0` | huginn | 0.00 | 0.52 | 0.51 | 0.44 | 0.40 | 0.41 | 2.76 | 23.5 | 49.3 | 0.10 | — [1] | — |
| `d06-nft` | `balanceOf.07-balanceOf-self.0` | huginn | 0.00 | 0.52 | 0.48 | 0.47 | 0.41 | 0.41 | 2.75 | 22.4 | 49.0 | 0.06 | — [1] | — |
| `d06-nft` | `balanceOf.08-balanceOf-unknown.0` | huginn | 0.00 | 0.53 | 0.49 | 0.45 | 0.42 | 0.41 | 2.79 | 20.8 | 49.4 | 0.06 | — [1] | — |
| `d06-nft` | `burnAdmin.19-burn-3.0` | huginn | 0.00 | 0.55 | 0.51 | 0.47 | 0.40 | 0.39 | 2.84 | 24.7 | 49.2 | 0.07 | — [1] | — |
| `d06-nft` | `getApproved.12-getApproved-1.0` | huginn | 0.00 | 0.51 | 0.51 | 0.45 | 0.39 | 0.40 | 2.82 | 23.2 | 49.3 | 0.06 | — [1] | — |
| `d06-nft` | `isApprovedForAll.14-isApprovedForAll-self-bob.0` | huginn | 0.00 | 0.52 | 0.49 | 0.46 | 0.41 | 0.40 | 2.74 | 21.9 | 49.1 | 0.07 | — [1] | — |
| `d06-nft` | `isApprovedForAll.15-isApprovedForAll-unset.0` | huginn | 0.00 | 0.52 | 0.50 | 0.47 | 0.45 | 0.40 | 2.88 | 23.8 | 49.1 | 0.11 | — [1] | — |
| `d06-nft` | `mintAdmin.02-mint-1-to-self-new-holder.0` | huginn | 0.00 | 0.56 | 0.49 | 0.48 | 0.41 | 0.38 | 2.83 | 28.7 | 49.1 | 0.10 | — [1] | — |
| `d06-nft` | `mintAdmin.03-mint-2-to-self-existing-holder.0` | huginn | 0.00 | 0.58 | 0.50 | 0.44 | 0.39 | 0.39 | 2.74 | 20.8 | 49.1 | 0.10 | — [1] | — |
| `d06-nft` | `mintAdmin.04-mint-3-to-bob.0` | huginn | 0.00 | 0.55 | 0.49 | 0.46 | 0.41 | 0.41 | 2.77 | 20.3 | 49.1 | 0.06 | — [1] | — |
| `d06-nft` | `ownerOf.09-ownerOf-1.0` | huginn | 0.00 | 0.52 | 0.49 | 0.45 | 0.42 | 0.41 | 2.77 | 22.4 | 48.3 | 0.06 | — [1] | — |
| `d06-nft` | `rotateAdmin.20-rotate-admin.0` | huginn | 0.00 | 0.54 | 0.51 | 0.44 | 0.40 | 0.39 | 2.75 | 24.2 | 48.4 | 0.11 | — [1] | — |
| `d06-nft` | `setApprovalForAll.13-setApprovalForAll-bob.0` | huginn | 0.00 | 0.52 | 0.49 | 0.47 | 0.41 | 0.42 | 2.76 | 23.5 | 48.3 | 0.09 | — [1] | — |
| `d06-nft` | `transfer.16-transfer-1-to-bob.0` | odin | 0.00 | 0.61 | 0.67 | 0.56 | 0.46 | 0.47 | 1.47 | 29.2 | 58.8 | 0.08 | 6.19 | — |
| `d06-nft` | `transferFrom.18-transferFrom-2-to-carol.0` | huginn | 0.00 | 0.55 | 0.49 | 0.44 | 0.39 | 0.38 | 2.95 | 19.1 | 48.5 | 0.11 | — [1] | — |
| `h03-election-ctor` | `add_voter.add-voter-31.0` | odin | 0.00 | 0.59 | 0.67 | 0.59 | 0.46 | 0.46 | 1.50 | 27.3 | 58.1 | 0.07 | 7.22 | — |
| `h03-election-ctor` | `add_voter.add-voter-32.0` | loki | 0.00 | 0.37 | 0.66 | 0.49 | 0.47 | 0.46 | 2.11 | 34.7 | 79.8 | 0.08 | — [1] | — |
| `h03-election-ctor` | `add_voter.add-voter-33.0` | loki | 0.00 | 0.36 | 0.68 | 0.49 | 0.46 | 0.45 | 2.17 | 26.6 | 79.0 | 0.07 | — [1] | — |
| `h03-election-ctor` | `advance.advance-to-commit.0` | loki | 0.00 | 0.39 | 0.71 | 0.52 | 0.46 | 0.45 | 2.19 | 31.7 | 79.9 | 0.10 | — [1] | — |
| `h03-election-ctor` | `advance.advance-to-final.0` | loki | 0.00 | 0.40 | 0.74 | 0.53 | 0.45 | 0.45 | 2.15 | 27.3 | 79.9 | 0.10 | — [1] | — |
| `h03-election-ctor` | `advance.advance-to-reveal.0` | loki | 0.00 | 0.39 | 0.67 | 0.50 | 0.47 | 0.45 | 2.15 | 33.2 | 79.7 | 0.11 | — [1] | — |
| `h03-election-ctor` | `set_topic.set-topic.0` | loki | 0.00 | 0.39 | 0.77 | 0.50 | 0.46 | 0.45 | 2.19 | 33.0 | 79.9 | 0.10 | — [1] | — |
| `h03-election-ctor` | `vote$commit.vote-commit-31-yes.0` | loki | 0.01 | 0.44 | 0.67 | 0.49 | 0.46 | 0.46 | 2.17 | 30.0 | 79.7 | 0.06 | — [1] | — |
| `h03-election-ctor` | `vote$commit.vote-commit-32-no.0` | loki | 0.01 | 0.44 | 0.67 | 0.48 | 0.45 | 0.46 | 2.18 | 26.1 | 80.3 | 0.07 | — [1] | — |
| `h03-election-ctor` | `vote$commit.vote-commit-33-yes.0` | loki | 0.01 | 0.43 | 0.67 | 0.49 | 0.47 | 0.45 | 2.23 | 28.8 | 79.7 | 0.10 | — [1] | — |
| `h03-election-ctor` | `vote$reveal.vote-reveal-31-yes.0` | loki | 0.01 | 0.44 | 0.66 | 0.52 | 0.54 | 0.50 | 2.19 | 28.7 | 80.1 | 0.06 | — [1] | — |
| `h03-election-ctor` | `vote$reveal.vote-reveal-32-no.0` | loki | 0.01 | 0.44 | 0.67 | 0.49 | 0.46 | 0.45 | 2.17 | 34.3 | 79.5 | 0.06 | — [1] | — |
| `h03-election-ctor` | `vote$reveal.vote-reveal-33-yes.0` | loki | 0.01 | 0.45 | 0.66 | 0.50 | 0.46 | 0.49 | 2.15 | 27.4 | 79.5 | 0.11 | — [1] | — |
| `j02-schnorr` | `verifySchnorrN3.verify-valid.0` | odin | 0.09 | 0.79 | 0.67 | 0.51 | 0.48 | 0.47 | 1.51 | 28.9 | 57.7 | 0.07 | 4.08 | — |
| `j02-schnorr` | `verifySchnorrN3.verify-wrong-msg.0` | loki | 0.09 | 0.49 | 0.66 | 0.49 | 0.48 | 0.47 | 2.22 | 32.9 | 79.8 | 0.09 | — [1] | — |
| `s01-natives` | `conv.conv.0` | huginn | 0.00 | 0.50 | 0.49 | 0.45 | 0.40 | 0.40 | 2.78 | 24.8 | 48.1 | 0.06 | — [1] | — |
| `s01-natives` | `ec.ec.0` | huginn | 0.07 | 0.59 | 0.50 | 0.44 | 0.40 | 0.40 | 2.79 | 22.7 | 48.1 | 0.09 | — [1] | — |
| `s01-natives` | `pc.pc.0` | huginn | 0.00 | 0.55 | 0.51 | 0.46 | 0.41 | 0.41 | 2.74 | 23.4 | 48.1 | 0.09 | — [1] | — |
| `s01-natives` | `ph.ph.0` | huginn | 0.00 | 0.53 | 0.49 | 0.46 | 0.41 | 0.41 | 2.80 | 24.5 | 47.7 | 0.12 | — [1] | — |
| `s01-natives` | `tc.tc.0` | huginn | 0.00 | 0.52 | 0.50 | 0.44 | 0.39 | 0.39 | 2.75 | 21.8 | 47.9 | 0.10 | — [1] | — |
| `s01-natives` | `th.th.0` | huginn | 0.00 | 0.51 | 0.48 | 0.48 | 0.42 | 0.40 | 2.77 | 22.1 | 48.1 | 0.10 | — [1] | — |
| `s01-natives` | `th3.th3.0` | huginn | 0.00 | 0.52 | 0.50 | 0.43 | 0.41 | 0.38 | 2.76 | 24.1 | 48.4 | 0.10 | — [1] | — |
| `s03-language` | `loop.loop.0` | odin | 0.00 | 0.56 | 0.74 | 0.60 | 0.46 | 0.47 | 1.48 | 25.7 | 58.4 | 0.07 | 6.13 | — |
| `s03-language` | `record.record-high.0` | odin | 0.00 | 0.64 | 0.69 | 0.60 | 0.46 | 0.46 | 1.47 | 25.4 | 57.7 | 0.08 | — [1] | — |
| `s03-language` | `record.record-low.0` | odin | 0.00 | 0.63 | 0.67 | 0.57 | 0.48 | 0.46 | 1.48 | 28.7 | 58.2 | 0.07 | — [1] | — |
| `s03-language` | `wide.wide-ge.0` | odin | 0.00 | 0.64 | 0.69 | 0.60 | 0.47 | 0.45 | 1.47 | 31.1 | 57.5 | 0.09 | — [1] | — |
| `s03-language` | `wide.wide-lt.0` | odin | 0.00 | 0.64 | 0.71 | 0.59 | 0.49 | 0.47 | 1.51 | 26.5 | 58.0 | 0.09 | — [1] | — |
| `t01-tiny` | `clear.clear-owner.0` | odin | 0.00 | 0.68 | 0.70 | 0.59 | 0.46 | 0.47 | 1.47 | 27.0 | 58.9 | 0.07 | — [1] | — |
| `t01-tiny` | `get.get-set-again.0` | odin | 0.00 | 0.69 | 0.74 | 0.59 | 0.46 | 0.45 | 1.47 | 27.5 | 58.3 | 0.10 | — [1] | — |
| `t01-tiny` | `get.get-set.0` | odin | 0.00 | 0.59 | 0.71 | 0.57 | 0.46 | 0.45 | 1.46 | 30.5 | 58.4 | 0.07 | — [1] | — |
| `t01-tiny` | `get.get-unset.0` | odin | 0.00 | 0.63 | 0.68 | 0.60 | 0.48 | 0.47 | 1.52 | 28.6 | 58.0 | 0.07 | — [1] | — |
| `t01-tiny` | `set.set-other.0` | odin | 0.00 | 0.66 | 0.71 | 0.56 | 0.50 | 0.46 | 1.48 | 24.8 | 58.3 | 0.07 | — [1] | — |
| `t02-counter-example` | `increment.increment.0` | huginn | 0.00 | 0.53 | 0.49 | 0.46 | 0.40 | 0.39 | 2.80 | 21.9 | 49.6 | 0.06 | 3.20 | — |
| `t02-counter-example` | `increment.increment.0` | loki | 0.00 | 0.42 | 0.67 | 0.50 | 0.46 | 0.46 | 2.19 | 34.9 | 86.0 | 0.06 | 3.20 | — |
| `t02-counter-example` | `increment.increment.0` | odin | 0.00 | 0.63 | 0.70 | 0.57 | 0.49 | 0.46 | 1.47 | 26.2 | 58.2 | 0.06 | 3.20 | — |
| `t03-counter-compiler` | `increment.increment.0` | odin | 0.00 | 0.69 | 0.71 | 0.60 | 0.47 | 0.49 | 1.48 | 26.5 | 58.2 | 0.10 | — [1] | — |
| `t04-welcome` | `add_organizer.add-organizer-k2.0` | huginn | 0.00 | 0.57 | 0.48 | 0.45 | 0.40 | 0.39 | 2.82 | 19.8 | 48.0 | 0.07 | — [1] | — |
| `t04-welcome` | `add_participant.add-participant-alice.0` | huginn | 0.00 | 0.57 | 0.50 | 0.45 | 0.43 | 0.40 | 2.75 | 20.3 | 47.4 | 0.06 | — [1] | — |
| `t04-welcome` | `add_participant.add-participant-by-k2.0` | huginn | 0.00 | 0.54 | 0.50 | 0.46 | 0.39 | 0.40 | 2.79 | 24.6 | 47.7 | 0.08 | — [1] | — |
| `t04-welcome` | `check_in.check-in-alice.0` | huginn | 0.00 | 0.53 | 0.50 | 0.46 | 0.40 | 0.40 | 2.88 | 30.1 | 47.5 | 0.07 | — [1] | — |
| `t04-welcome` | `check_in.check-in-bob.0` | huginn | 0.00 | 0.52 | 0.50 | 0.46 | 0.41 | 0.41 | 2.85 | 23.2 | 47.1 | 0.06 | — [1] | — |
| `t04-welcome` | `check_in.check-in-dave.0` | huginn | 0.00 | 0.52 | 0.50 | 0.45 | 0.42 | 0.41 | 2.77 | 24.0 | 47.3 | 0.10 | — [1] | — |
| `w02-bboard-testcenter` | `post.01-alice-post.0` | loki | 0.00 | 0.37 | 0.67 | 0.49 | 0.45 | 0.45 | 2.17 | 26.8 | 79.9 | 0.07 | — [1] | — |
| `w02-bboard-testcenter` | `post.06-bob-post.0` | loki | 0.00 | 0.36 | 0.69 | 0.51 | 0.45 | 0.46 | 2.15 | 30.3 | 79.6 | 0.10 | — [1] | — |
| `w02-bboard-testcenter` | `take_down.04-alice-takedown.0` | loki | 0.00 | 0.38 | 0.71 | 0.50 | 0.46 | 0.46 | 2.18 | 30.8 | 79.8 | 0.07 | — [1] | — |
| `w02-bboard-testcenter` | `take_down.08-bob-takedown.0` | loki | 0.00 | 0.41 | 0.71 | 0.49 | 0.45 | 0.45 | 2.19 | 32.7 | 79.8 | 0.07 | — [1] | — |
| `x01-composable-outer` | `add.add-cross-contract.1` | odin | 0.00 | 0.62 | 0.65 | 0.55 | 0.49 | 0.50 | 1.49 | 28.1 | 58.5 | 0.08 | 2.69 | — |
| `x01-composable-outer` | `addViaArg.add-via-arg-cross-contract.1` | loki | 0.00 | 0.33 | 0.65 | 0.49 | 0.45 | 0.45 | 2.14 | 26.3 | 79.8 | 0.07 | — [1] | — |
| `x01-composable-outer` | `addViaConst.add-via-const-cross-contract.1` | loki | 0.00 | 0.38 | 0.66 | 0.49 | 0.47 | 0.47 | 2.17 | 27.1 | 79.8 | 0.08 | — [1] | — |
| `x01-composable-outer` | `echoStored.echo-stored-after-set.0` | loki | 0.00 | 0.34 | 0.66 | 0.49 | 0.45 | 0.46 | 2.18 | 28.9 | 79.9 | 0.06 | — [1] | — |
| `x01-composable-outer` | `echoStored.echo-stored-initial.0` | loki | 0.00 | 0.33 | 0.66 | 0.56 | 0.50 | 0.45 | 2.19 | 29.1 | 79.8 | 0.10 | — [1] | — |
| `x01-composable-outer` | `setInner.set-inner-b.0` | loki | 0.00 | 0.32 | 0.68 | 0.49 | 0.45 | 0.45 | 2.18 | 37.3 | 79.7 | 0.10 | — [1] | — |
| `x02-composable-inner` | `add.add-cross-contract.0` | odin | 0.00 | 0.54 | 0.70 | 0.57 | 0.48 | 0.49 | 1.49 | 28.5 | 58.3 | 0.07 | 5.12 | — |
| `x02-composable-inner` | `add.add-via-arg-cross-contract.0` | huginn | 0.00 | 0.53 | 0.48 | 0.44 | 0.41 | 0.39 | 2.84 | 21.0 | 47.5 | 0.11 | — [1] | — |
| `x02-composable-inner` | `add.add-via-const-cross-contract.0` | huginn | 0.00 | 0.51 | 0.50 | 0.47 | 0.40 | 0.42 | 2.88 | 18.9 | 47.3 | 0.06 | — [1] | — |
| `x02-composable-inner` | `add.add.0` | huginn | 0.00 | 0.50 | 0.50 | 0.43 | 0.40 | 0.40 | 2.73 | 19.8 | 47.4 | 0.07 | — [1] | — |
| `z01-zerocash` | `spend.spend-c1.0` | huginn | 0.02 | 0.74 | 0.49 | 0.45 | 0.41 | 0.39 | 2.76 | 20.0 | 47.5 | 0.11 | — [1] | — |
| `z01-zerocash` | `spend.spend-c3.0` | huginn | 0.02 | 0.73 | 0.54 | 0.45 | 0.42 | 0.41 | 2.75 | 22.3 | 47.5 | 0.07 | — [1] | — |
| `z01-zerocash` | `zerocash_mint.mint-c1.0` | huginn | 0.00 | 0.55 | 0.50 | 0.45 | 0.41 | 0.40 | 2.75 | 21.8 | 47.5 | 0.07 | — [1] | — |
| `z01-zerocash` | `zerocash_mint.mint-c2.0` | huginn | 0.00 | 0.55 | 0.51 | 0.46 | 0.40 | 0.41 | 2.81 | 21.3 | 47.2 | 0.10 | — [1] | — |
| `z02-unshielded-tokens` | `getUnshieldedBalanceGtTest.getUnshieldedBalanceGtTest.0` | odin | 0.00 | 0.56 | 0.68 | 0.60 | 0.47 | 0.49 | 1.48 | 30.1 | 58.1 | 0.10 | — [1] | — |
| `z02-unshielded-tokens` | `getUnshieldedBalanceLtTest.getUnshieldedBalanceLtTest.0` | odin | 0.00 | 0.59 | 0.70 | 0.57 | 0.48 | 0.48 | 1.51 | 28.6 | 58.4 | 0.07 | — [1] | — |
| `z02-unshielded-tokens` | `getUnshieldedBalanceTest.getUnshieldedBalanceTest.0` | odin | 0.00 | 0.64 | 0.71 | 0.58 | 0.49 | 0.48 | 1.47 | 28.5 | 58.4 | 0.07 | — [1] | — |
| `z02-unshielded-tokens` | `mintShieldedToContractTest.mintShieldedToContractTest.0` | odin | 0.00 | 0.71 | 0.69 | 0.56 | 0.48 | 0.47 | 1.48 | 26.2 | 58.4 | 0.09 | — [1] | — |
| `z02-unshielded-tokens` | `mintShieldedToSelfTest.mintShieldedToSelfTest.0` | odin | 0.00 | 0.70 | 0.70 | 0.56 | 0.48 | 0.45 | 1.46 | 29.0 | 58.3 | 0.11 | — [1] | — |
| `z02-unshielded-tokens` | `mintShieldedToUserTest.mintShieldedToUserTest.0` | odin | 0.00 | 0.75 | 0.72 | 0.56 | 0.48 | 0.49 | 1.49 | 25.3 | 58.4 | 0.06 | — [1] | — |
| `z02-unshielded-tokens` | `mintUnshieldedToContractTest.mintUnshieldedToContractTest.0` | odin | 0.00 | 0.72 | 0.70 | 0.57 | 0.48 | 0.47 | 1.46 | 29.6 | 58.1 | 0.07 | — [1] | — |
| `z02-unshielded-tokens` | `mintUnshieldedToSelfTest.mintUnshieldedToSelfTest.0` | odin | 0.00 | 0.67 | 0.70 | 0.56 | 0.50 | 0.49 | 1.48 | 25.6 | 58.5 | 0.07 | 3.98 | — |
| `z02-unshielded-tokens` | `mintUnshieldedToUserTest.mintUnshieldedToUserTest.0` | odin | 0.00 | 0.65 | 0.69 | 0.58 | 0.46 | 0.50 | 1.47 | 27.7 | 58.1 | 0.06 | — [1] | — |
| `z02-unshielded-tokens` | `receiveShieldedTest.receiveShieldedTest.0` | odin | 0.00 | 0.71 | 0.66 | 0.56 | 0.48 | 0.47 | 1.51 | 27.6 | 58.8 | 0.11 | — [1] | — |
| `z02-unshielded-tokens` | `receiveUnshieldedTest.receiveUnshieldedTest.0` | odin | 0.00 | 0.65 | 0.67 | 0.56 | 0.45 | 0.49 | 1.48 | 25.6 | 57.8 | 0.10 | — [1] | — |
| `z02-unshielded-tokens` | `sendImmediateShieldedToContractTest.sendImmediateShieldedToContractTest.0` | odin | 0.01 | 0.79 | 0.69 | 0.59 | 0.47 | 0.49 | 1.54 | 24.8 | 58.3 | 0.06 | — [1] | — |
| `z02-unshielded-tokens` | `sendImmediateShieldedToSelfTest.sendImmediateShieldedToSelfTest.0` | odin | 0.01 | 0.81 | 0.74 | 0.62 | 0.48 | 0.46 | 1.53 | 27.5 | 57.9 | 0.09 | — [1] | — |
| `z02-unshielded-tokens` | `sendImmediateShieldedToUserTest.sendImmediateShieldedToUserTest.0` | odin | 0.01 | 0.85 | 0.72 | 0.56 | 0.48 | 0.47 | 1.52 | 30.3 | 58.2 | 0.07 | — [1] | — |
| `z02-unshielded-tokens` | `sendShieldedToContractTest.sendShieldedToContractTest.0` | odin | 0.01 | 0.81 | 0.71 | 0.56 | 0.47 | 0.45 | 1.53 | 32.1 | 58.4 | 0.06 | — [1] | — |
| `z02-unshielded-tokens` | `sendShieldedToSelfTest.sendShieldedToSelfTest.0` | odin | 0.01 | 0.83 | 0.70 | 0.59 | 0.47 | 0.49 | 1.53 | 29.8 | 57.9 | 0.10 | — [1] | — |
| `z02-unshielded-tokens` | `sendUnshieldedToContractTest.sendUnshieldedToContractTest.0` | odin | 0.00 | 0.66 | 0.70 | 0.60 | 0.47 | 0.48 | 1.47 | 29.5 | 58.1 | 0.07 | — [1] | — |
| `z02-unshielded-tokens` | `sendUnshieldedToSelfTest.sendUnshieldedToSelfTest.0` | odin | 0.00 | 0.65 | 0.68 | 0.58 | 0.48 | 0.48 | 1.50 | 27.0 | 58.1 | 0.07 | — [1] | — |
| `z02-unshielded-tokens` | `sendUnshieldedToUserTest.sendUnshieldedToUserTest.0` | odin | 0.00 | 0.62 | 0.68 | 0.57 | 0.51 | 0.47 | 1.52 | 28.2 | 57.8 | 0.11 | — [1] | — |
| `z03-block-time` | `testBlockTimeGt.testBlockTimeGt.0` | odin | 0.00 | 0.58 | 0.67 | 0.60 | 0.47 | 0.46 | 1.46 | 28.2 | 58.3 | 0.10 | 4.97 | — |
| `z03-block-time` | `testBlockTimeGte.testBlockTimeGte.0` | loki | 0.00 | 0.32 | 0.66 | 0.49 | 0.45 | 0.45 | 2.16 | 29.7 | 79.8 | 0.11 | — [1] | — |
| `z03-block-time` | `testBlockTimeLt.testBlockTimeLt.0` | loki | 0.00 | 0.33 | 0.66 | 0.49 | 0.46 | 0.47 | 2.17 | 27.6 | 79.9 | 0.11 | — [1] | — |
| `z03-block-time` | `testBlockTimeLte.testBlockTimeLte.0` | loki | 0.00 | 0.34 | 0.65 | 0.49 | 0.45 | 0.47 | 2.17 | 24.4 | 80.0 | 0.07 | — [1] | — |
| `z04-set-qualified-coin` | `test_QualifiedShieldedCoinInfo.qualified-roundtrip-again.0` | loki | 0.00 | 0.34 | 0.67 | 0.51 | 0.50 | 0.47 | 2.17 | 43.1 | 80.2 | 0.08 | — [1] | — |
| `z04-set-qualified-coin` | `test_QualifiedShieldedCoinInfo.qualified-roundtrip.0` | loki | 0.00 | 0.34 | 0.68 | 0.49 | 0.47 | 0.46 | 2.17 | 34.5 | 80.0 | 0.11 | — [1] | — |
| `z04-set-qualified-coin` | `test_ShieldedCoinInfo.shielded-coin-received.0` | loki | 0.00 | 0.37 | 0.67 | 0.50 | 0.46 | 0.45 | 2.17 | 32.0 | 79.6 | 0.07 | — [1] | — |

[1] not settled: DERIVED (LEAD PICK SETTLE-REP)

## Per program

| program | calls | compactc | Mycroft -> R_SL | measured call stages (sum) | stages measured | total |
|---|---:|---:|---:|---:|---:|---:|
| `a01-adt-counter` | 2 | — | — | 151.9 | 20 of 22 | — |
| `a02-set-field` | 1 | — | — | 74.3 | 10 of 11 | — |
| `a03-map-field-field` | 1 | — | — | 118.7 | 10 of 11 | — |
| `a04-map-bool-map-counter` | 1 | — | — | 123.8 | 10 of 11 | — |
| `a05-list-field` | 1 | — | — | 94.6 | 11 of 11 | — |
| `a08-kernel` | 1 | — | — | 73.3 | 10 of 11 | — |
| `d01-example-bboard` | 4 | — | — | 299.9 | 40 of 44 | — |
| `d02-battleship` | 13 | — | — | 1,179.2 | 130 of 143 | — |
| `d03-micro-dao` | 18 | — | — | 1,393.3 | 180 of 198 | — |
| `d04-coracle` | 6 | — | — | 703.1 | 60 of 66 | — |
| `d06-nft` | 15 | — | — | 1,175.8 | 151 of 165 | — |
| `h03-election-ctor` | 13 | — | — | 1,473.1 | 131 of 143 | — |
| `j02-schnorr` | 2 | — | — | 213.0 | 21 of 22 | — |
| `s01-natives` | 7 | — | — | 535.9 | 70 of 77 | — |
| `s03-language` | 5 | — | — | 455.4 | 51 of 55 | — |
| `t01-tiny` | 5 | — | — | 452.2 | 50 of 55 | — |
| `t02-counter-example` | 3 | — | — | 300.7 | 33 of 33 | — |
| `t03-counter-compiler` | 1 | — | — | 89.2 | 10 of 11 | — |
| `t04-welcome` | 6 | — | — | 458.1 | 60 of 66 | — |
| `w02-bboard-testcenter` | 4 | — | — | 458.6 | 40 of 44 | — |
| `x01-composable-outer` | 6 | — | — | 664.9 | 61 of 66 | — |
| `x02-composable-inner` | 4 | — | — | 313.5 | 41 of 44 | — |
| `z01-zerocash` | 4 | — | — | 296.1 | 40 of 44 | — |
| `z02-unshielded-tokens` | 19 | — | — | 1,728.7 | 191 of 209 | — |
| `z03-block-time` | 4 | — | — | 431.3 | 41 of 44 | — |
| `z04-set-qualified-coin` | 3 | — | — | 363.5 | 30 of 33 | — |

## Per program by box (final run)

Prove = the chain process stages R_SL exec to L_Fr (median per call); wrap = E5JSON prove; ledger check = ledger 9.1.0.1 verify on muninn; Preview tx = included transactions (deploy + acceptRun) of the settled calls; blocks = acceptRun inclusion blocks.

| program | box | calls | proved | prove median s | wrap median s | ledger check median s | Preview tx | blocks |
|---|---|---:|---:|---:|---:|---:|---:|---|
| `a01-adt-counter` | huginn | 2 | 2 | 28.2 | 47.7 | 0.08 | — | — |
| `a02-set-field` | huginn | 1 | 1 | 26.7 | 47.5 | 0.07 | — | — |
| `a03-map-field-field` | loki | 1 | 1 | 38.7 | 79.9 | 0.11 | — | — |
| `a04-map-bool-map-counter` | loki | 1 | 1 | 43.9 | 79.7 | 0.10 | — | — |
| `a05-list-field` | odin | 1 | 1 | 31.9 | 58.4 | 0.07 | 2 | 140183 |
| `a08-kernel` | huginn | 1 | 1 | 25.8 | 47.5 | 0.06 | — | — |
| `d01-example-bboard` | huginn | 4 | 4 | 27.7 | 47.3 | 0.09 | — | — |
| `d02-battleship` | huginn | 1 | 1 | 33.8 | 49.4 | 0.09 | — | — |
| `d02-battleship` | loki | 1 | 1 | 36.7 | 77.0 | 0.09 | — | — |
| `d02-battleship` | odin | 11 | 11 | 30.7 | 58.1 | 0.09 | — | — |
| `d03-micro-dao` | huginn | 18 | 18 | 27.6 | 49.5 | 0.07 | — | — |
| `d04-coracle` | loki | 6 | 6 | 37.1 | 79.7 | 0.07 | — | — |
| `d06-nft` | huginn | 14 | 14 | 27.9 | 49.1 | 0.08 | — | — |
| `d06-nft` | odin | 1 | 1 | 33.5 | 58.8 | 0.08 | 2 | 140308 |
| `h03-election-ctor` | loki | 12 | 12 | 34.1 | 79.8 | 0.09 | — | — |
| `h03-election-ctor` | odin | 1 | 1 | 31.5 | 58.1 | 0.07 | 2 | 140430 |
| `j02-schnorr` | loki | 1 | 1 | 37.8 | 79.8 | 0.09 | — | — |
| `j02-schnorr` | odin | 1 | 1 | 33.4 | 57.7 | 0.07 | 2 | 141386 |
| `s01-natives` | huginn | 7 | 7 | 28.5 | 48.1 | 0.10 | — | — |
| `s03-language` | odin | 5 | 5 | 31.0 | 58.0 | 0.08 | 2 | 140963 |
| `t01-tiny` | odin | 5 | 5 | 31.9 | 58.3 | 0.07 | — | — |
| `t02-counter-example` | huginn | 1 | 1 | 26.9 | 49.6 | 0.06 | 2 | 136144 |
| `t02-counter-example` | loki | 1 | 1 | 39.6 | 86.0 | 0.06 | 2 | 136144 |
| `t02-counter-example` | odin | 1 | 1 | 30.6 | 58.2 | 0.06 | 2 | 136144 |
| `t03-counter-compiler` | odin | 1 | 1 | 30.9 | 58.2 | 0.10 | — | — |
| `t04-welcome` | huginn | 6 | 6 | 28.7 | 47.4 | 0.07 | — | — |
| `w02-bboard-testcenter` | loki | 4 | 4 | 35.2 | 79.8 | 0.07 | — | — |
| `x01-composable-outer` | loki | 5 | 5 | 33.5 | 79.8 | 0.08 | — | — |
| `x01-composable-outer` | odin | 1 | 1 | 32.4 | 58.5 | 0.08 | 2 | 140557 |
| `x02-composable-inner` | huginn | 3 | 3 | 24.7 | 47.4 | 0.07 | — | — |
| `x02-composable-inner` | odin | 1 | 1 | 32.8 | 58.3 | 0.07 | 2 | 140688 |
| `z01-zerocash` | huginn | 4 | 4 | 26.6 | 47.5 | 0.09 | — | — |
| `z02-unshielded-tokens` | odin | 19 | 19 | 32.6 | 58.2 | 0.07 | 2 | 140825 |
| `z03-block-time` | loki | 3 | 3 | 32.2 | 79.9 | 0.11 | — | — |
| `z03-block-time` | odin | 1 | 1 | 32.4 | 58.3 | 0.10 | 2 | 141480 |
| `z04-set-qualified-coin` | loki | 3 | 3 | 39.1 | 80.0 | 0.08 | — | — |

## Preview settle (SherlockLedger, one ledger per settled call)

| program | call | deploy tx | deploy block | acceptRun tx | acceptRun block | submit to inclusion s | refused (not included) |
|---|---|---|---:|---|---:|---:|---:|
| `a05-list-field` | `test.test.0` | `0xf08422fbe9…` | 139550 | `0xc7f0d1fb7a…` | 140183 | 4.24 | 3 |
| `d06-nft` | `transfer.16-transfer-1-to-bob.0` | `0x95224e255d…` | 140227 | `0x942ff5ba7e…` | 140308 | 6.19 | 0 |
| `h03-election-ctor` | `add_voter.add-voter-31.0` | `0x5315b7aa76…` | 140349 | `0x131123b00f…` | 140430 | 7.22 | 0 |
| `j02-schnorr` | `verifySchnorrN3.verify-valid.0` | `0xb7d9d5dd18…` | 141315 | `0x8cf9bc7fa7…` | 141386 | 4.08 | 0 |
| `s03-language` | `loop.loop.0` | `0xa23cf3c8cc…` | 140869 | `0x480225e77c…` | 140963 | 6.13 | 0 |
| `t02-counter-example` | `increment.increment.0` | `0xcb25aa99ac…` | 135999 | `0xc4c3313206…` | 136144 | 3.20 | 3 |
| `x01-composable-outer` | `add.add-cross-contract.1` | `0x36c45cf997…` | 140471 | `0xc4cda40360…` | 140557 | 2.69 | 0 |
| `x02-composable-inner` | `add.add-cross-contract.0` | `0xe9342960eb…` | 140606 | `0xd4d781107f…` | 140688 | 5.12 | 0 |
| `z02-unshielded-tokens` | `mintUnshieldedToSelfTest.mintUnshieldedToSelfTest.0` | `0xc4066c157d…` | 140735 | `0x0dbb6abb20…` | 140825 | 3.98 | 0 |
| `z03-block-time` | `testBlockTimeGt.testBlockTimeGt.0` | `0xbf4404bbbb…` | 141405 | `0x5a951f8feb…` | 141480 | 4.97 | 0 |

Preview tx MEASURED: 20 included (10 settled calls), 6 refused submissions (no fee, not included).
Preview tx DERIVED for the full suite at one ledger per call (acceptRun inserts one claim per tx): 290 included tx for 145 proved calls.

## Suite statistics per stage

| stage | subject | n | median | p95 | min | max |
|---|---|---:|---:|---:|---:|---:|
| R_SL exec + witness | R_SL calls | 149 | 0.00 | 0.01 | 0.00 | 0.09 |
| R_SL app proof | R_SL calls | 149 | 0.55 | 0.75 | 0.32 | 0.85 |
| leaf proof | R_SL calls | 149 | 0.66 | 0.72 | 0.48 | 0.77 |
| N1 | R_SL calls | 149 | 0.49 | 0.60 | 0.43 | 0.65 |
| N2 | R_SL calls | 149 | 0.45 | 0.50 | 0.39 | 0.54 |
| N3 | R_SL calls | 149 | 0.45 | 0.49 | 0.38 | 0.52 |
| L | R_SL calls | 149 | 2.18 | 2.84 | 1.45 | 2.95 |
| L_Fr | R_SL calls | 149 | 26.3 | 34.7 | 18.9 | 43.1 |
| k=21 wrap | R_SL calls | 149 | 58.0 | 79.9 | 47.1 | 86.0 |
| ledger-check 9.1.0.1 | R_SL calls | 145 | 0.07 | 0.11 | 0.06 | 0.12 |
| Preview submit -> inclusion | R_SL calls | 10 | 4.60 | 7.22 | 2.69 | 7.22 |
