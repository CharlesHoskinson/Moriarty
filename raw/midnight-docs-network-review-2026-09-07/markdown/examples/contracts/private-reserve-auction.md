# Private Reserve Auction Contract

> For the complete documentation index, see [llms.txt](/llms.txt)

This Compact contract implements an auction with a private reserve that also maintains the privacy of bidders with bid amounts as public information. It offers demonstration of the following features:

* Hiding information on the public ledger
* Reducing user trackability with specific DApp public keys
* Operations on a `Map`

```
pragma language_version 0.23;

import CompactStandardLibrary;



export enum AuctionState {

    CLOSED,

    OPEN

}



export sealed ledger auctionOrganizer: Bytes<32>;

export sealed ledger hiddenPrice: Bytes<32>;

export sealed ledger maxBids: Uint<16>;

export ledger publicPrice: Uint<16>;

export ledger bidders: Map<Bytes<32>, Uint<16>>;// unique bidders, overwriting bids allowed

export ledger bidCount: Counter;// number of bids

export ledger highestBid: Uint<16>;

export ledger auctionState: AuctionState;



witness localSk(): Bytes<32>;



constructor(_minPrice: Uint<16>, maxBidCount: Uint<16>) {

    const _sk = localSk();

    const pubKey = getDappPubKey(_sk);

    auctionOrganizer = disclose(pubKey);



    const hashedPrice = commitWithSk(_minPrice as Bytes<32>, _sk);



    maxBids = disclose(maxBidCount);

    hiddenPrice = hashedPrice;

    highestBid = 0;

    auctionState = AuctionState.OPEN;

}



// bidders can bid more than once

export circuit bid(bidAmount: Uint<16>): [] {

    assert(bidCount < maxBids, "Sorry, bids are full");

    assert(auctionState == AuctionState.OPEN, "The auction has ended");

    

    const _sk = localSk();

    const key = getDappPubKey(_sk);

    const pubKey = disclose(key);

    const publicBid = disclose(bidAmount);



    if(bidders.member(pubKey)){

        assert(bidders.lookup(pubKey) < publicBid, "New bid lower than your previous bid");

    }

    bidders.insert(pubKey, publicBid);

    bidCount.increment(1);

    if(publicBid > highestBid){

        highestBid = publicBid;

    }

    if(bidCount == maxBids){

        auctionState = AuctionState.CLOSED;

    }

}



// allow the organizer to close the auction if maxBids is not reached

export circuit closeAuction(minPrice: Uint<16>): Uint<16> {

    const _sk = localSk();

    const pubKey = getDappPubKey(_sk);

    assert(disclose(pubKey) == auctionOrganizer, "You are not the auction organizer");

    assert(auctionState == AuctionState.OPEN, "The auction has already been closed");



    auctionState = AuctionState.CLOSED;

    return revealWin(minPrice);

}



export circuit revealWin(minPrice: Uint<16>): Uint<16> {

    const _sk = localSk();

    const pubKey = getDappPubKey(_sk);

    assert(auctionOrganizer == disclose(pubKey), "You are not the auction organizer");

    assert(auctionState == AuctionState.CLOSED, "The auction is still open");

    

    const hashedPrice = commitWithSk(minPrice as Bytes<32>, _sk);

    assert(hashedPrice == hiddenPrice, "Attempt to change min price detected, shame on you.");

    

    // all votes are in, public price can be revealed

    publicPrice = disclose(minPrice);



    // if the publicPrice is not met, the highestBid doesn't matter

    if(highestBid < publicPrice){

        return 0;

    } else {

        return highestBid;

    }

}



circuit commitWithSk(_minPrice: Bytes<32>, _sk: Bytes<32>): Bytes<32> {

    const hash = persistentHash<Vector<2, Bytes<32>>>([_minPrice, _sk]);

    return disclose(hash);

}



// hash a "public key" specific to this DApp so that users cannot be tracked

circuit getDappPubKey(_sk: Bytes<32>): Bytes<32> {

    return persistentHash<Vector<2, Bytes<32>>>([pad(32, "silent-auction:pk:"), _sk]);

}
```
