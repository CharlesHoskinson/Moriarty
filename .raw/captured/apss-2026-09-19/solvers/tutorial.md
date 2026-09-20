---
title: Learn to separate a good quote from a valid intent fulfillment
diataxis: tutorial
status: conceptual-exercise
---
# Learn to separate a good quote from a valid intent fulfillment

This paper exercise needs no wallet, funds or deployed Moriarty feature. You will classify three candidate plans against one fixed intent.

## Write the intent

Use two exact assets, `A=(Midnight,issuer-A,asset-A)` and `B=(Midnight,issuer-B,asset-B)`. Alice permits at most 100 units of A to leave her account, including all A-denominated fees. She requires at least 190 units of B credited to her address. No new liability is permitted. The intent has a fixed nonce and expiry. For this exercise, settlement is one atomic successful state transition and all fees use A.

## Evaluate candidates

| Candidate | Swap debit A | Fee A | Net credit B | Recipient |
|---|---:|---:|---:|---|
| Red | 99 | 2 | 200 | Alice |
| Blue | 97 | 3 | 194 | Alice |
| Green | 98 | 2 | 205 | Mallory |

Add swap debit and fee before comparing with the cap. Red spends 101 and fails. Blue spends 100, credits 194 to Alice and meets the stated numeric constraints. Green sends value to the wrong recipient and fails despite its higher quoted amount.

Now change Green's recipient to Alice but give B a different issuer. It still fails: the symbol does not establish asset identity.

## Distinguish what you established

You established that Blue meets this small arithmetic predicate. You did not prove liquidity, signatures, fresh state, absence of hidden effects, correct compilation or finality. A complete Moriarty artifact must discharge those applicable obligations.

Have two independent builders propose the same Blue effects. Their identities should not change the language's objective validity decision. Then change the predecessor state without regenerating bound evidence: it must fail.

## Add an asynchronous boundary

Suppose the B transfer occurs on a different chain. A successful origin transaction no longer proves Alice received B. Add a pending record and ask what authenticates the destination event, what fees can survive failure, and which recovery action is actually available. Do not invent rollback.

You have now separated price, constraint satisfaction, authority and settlement. Continue with the explanation note for the economic and proof boundaries. Inspired by [CAKE](https://frontier.tech/the-cake-framework) and [UniswapX's distinct settlement paths](https://app.uniswap.org/whitepaper-uniswapx.pdf); the numbers and exercise are original and make no claim of runnable Moriarty syntax.
