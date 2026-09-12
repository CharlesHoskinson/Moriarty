# Receiving: what Part I section 4 is missing

Scope: Part I section 4 of `docs/superpowers/specs/2026-09-11-defi-kernel-sdk-interface-design.md`.
Section 4 today makes five claims: one receiving preference for everything arriving from anywhere;
inbound value on any chain is routed to that preference; unroutable value is held and reported as an
explicit claim; better-than-quoted results are assigned in the intent; locked positions are reported
with a release condition and never shown as spendable.

Two structural observations before the gaps.

**Section 4 is an outbound section written backwards.** Every other Part I section is about authority
the account holder grants. Receiving is the one surface where a stranger acts on the account holder
without any authority at all, and section 4 answers that with an automatic action ("is routed to that
preference"). Automatic action on an unauthenticated stranger's input is the definition of an attack
surface, and it is the source of most of what follows.

**Section 4 already contains one internal contradiction.** "Value that cannot be routed to your
preference is held ... It is never silently left on a chain for you to discover" promises movement,
and movement costs money that section 1 bills to the account holder ("Execution costs on every chain
are sourced by the kernel and billed to you in your settlement asset"). A sender the account holder
never chose therefore decides what the account holder spends. The resolution is to make *held* mean
*recorded and attributed where it already sits*, so nothing is silent and nothing is paid for.

---

## GAPS

### G1. The receiving policy says where value lands but never what is accepted, so auto-routing is weaponizable

**The gap.** A receiving preference is a destination only. There is no acceptance half: nothing the
account holder can state that distinguishes value they were expecting from value a stranger pushed at
them. Combined with "value arriving on any chain, from any venue, from any counterparty, is routed to
that preference", an unauthenticated third party controls what the kernel does and therefore what the
account holder pays.

**The concrete failures.** Four distinct ones, all real.

*Cost griefing.* Pushing worthless assets is nearly free and industrialised at scale: over 65.4
million address-poisoning transactions were flagged on-chain from January 2025, and bulk spam
distribution to public addresses costs a fraction of a cent per recipient on cheap networks
([Blockaid](https://blockaid.io/blog/address-poisoning-the-growing-threat-draining-millions-from-crypto-users),
[Sol Incinerator](https://sol-incinerator.com/blog/how-to-burn-scam-and-spam-tokens-on-solana)).
Routing each one costs the recipient a real conversion and a real execution cost, and under this
design's own economics even *pricing* one costs money: Part II section 13 states "a quote round is
itself a priced job charged to its requester, so soliciting quotes has a cost and quote spam has a
payer." A kernel that auto-routes makes the account holder the payer for an attacker's volume. The
economics are established independently: "Sweeping a $3 stablecoin deposit where the fee is $2
destroys value. Thresholds exist for this reason"
([TronDealer](https://www.trondealer.com/en/guides/integration)); a UTXO worth under about $10 is
uneconomic to move at all ([Spark](https://www.spark.money/research/bitcoin-utxo-management-strategies)).

*Hostile asset logic executed inside the settlement path.* Routing an inbound asset means calling that
asset's own code. Receive hooks are a proven reentrancy vector with a loss record — roughly $300k from
the imBTC/Uniswap V1 pool, about $25M at Lendf.Me, about $18.8M at Cream
([Ackee](https://ackee.xyz/blog/reentrancy-attack-in-erc-777/),
[Immunefi](https://medium.com/immunefi/the-potential-impact-of-erc-777-tokens-on-defi-protocols-51cdb07be733)).
Assets that take a cut on transfer break the assumption that the amount sent equals the amount
received and make balance-delta accounting revert or misreport
([AuditBase](https://www.auditbase.com/detectors/fee-on-transfer-solidity)). Honeypot assets accept
transfers in and revert on the way out. Spam assets are specifically bait for the act of disposing of
them: "If a user attempts to sell or transfer one of these assets, they inadvertently interact with a
malicious smart contract" ([Trezor](https://trezor.io/support/troubleshooting/coins-tokens/dusting-attacks-airdrop-scam-tokens)).
Auto-routing is precisely the interaction every wallet vendor tells users never to perform.

*Taint concentration into the one destination the design guarantees.* This is the worst of the four,
because guarantee four ("One identity ... receives everywhere") and section 4's single destination
mean every inbound source is consolidated into one account. Sanctioned-source dusting is a documented
grief: the day after the Tornado Cash designation an attacker pushed 0.1 ETH from the sanctioned
contract to hundreds of doxxed wallets and to exchange wallets
([Decrypt](https://decrypt.co/107090/tornado-cash-dusts-public-wallets-jimmy-fallon-brian-armstrong-steve-aoki-logan-paul)),
and Kraken reported its users receiving dust from a sanctioned wallet
([Bitcoin Magazine](https://bitcoinmagazine.com/news/kraken-says-users-were-dust-attacked)). Screening
is assessed on received provenance several hops out, and "If many of those addresses are themselves
linked to illicit activity, then one can conclude that the initial address carries more risk"
([Chainalysis](https://www.chainalysis.com/blog/cryptocurrency-risk-blockchain-analysis-indirect-exposure/)).
The consequence is not theoretical discomfort: issuers of the settlement assets this kernel will
actually use can blacklist an address so that it "can neither send nor receive", and over $4bn has
been frozen that way, 384 addresses and about $515M in one 30-day window
([Koru Legal](https://korulegal.com/en/blog/usdt-usdc-freeze-stablecoin-blacklist-clean-coin/),
[Eagle Virtual](https://eaglevirtual.com/learn/usdc-usdt-blacklist-freeze-functions)). An attacker
spends dust; the account holder's single settlement destination stops working. Auto-consolidation
converts a $2 attack into a total loss of availability.

*Privacy inversion.* The universal defence against dusting is not to move or co-spend the dust: "The
most critical step is inaction ... As long as the dust remains unspent and isolated in the wallet, the
attacker cannot link it to other funds"
([Lightspark](https://www.lightspark.com/glossary/dust-attack),
[Wikipedia](https://en.wikipedia.org/wiki/Dusting_attack)). Auto-routing everything to one
destination performs the linkage the attacker paid for, on every chain, automatically. Part II already
concedes "Derivation separates control, not visibility", so the interface is actively worsening a
property it has already declined to guarantee.

There is a fifth, cheaper consequence worth one line: routing an inbound asset into the settlement
asset is a disposal, and per-account basis tracking is now mandatory in at least one major
jurisdiction, so auto-routing manufactures taxable events and moves basis between accounts on an
attacker's schedule ([IRS Rev. Proc. 2024-28 summary, The Network Firm](https://www.thenetworkfirm.com/blog/2025-new-crypto-tax-rules)).

**Why prevailing practice proves the need.** Every wallet and custodian solves this by *not*
treating inbound value as automatically real. Detection is allowlist-driven — tokens are auto-detected
only if they "belong to lists of established tokens", which "provides some protection against
malicious token airdrops" ([MetaMask](https://support.metamask.io/stay-safe/safety-in-web3/token-safety-practices/)) —
and hiding is explicitly a display act that changes no balance
([MetaMask](https://support.metamask.io/manage-crypto/portfolio/how-to-hide-an-asset-token-or-nft-in-metamask-portfolio/)).
Nobody auto-converts unsolicited inbound value. This design would be the first to do it, with the
account holder's budget.

### G2. Nothing acting for the account holder is forbidden from changing where value lands

**The gap.** Section 2 delegates through capabilities bounded by "what it may do and what it may
spend". Section 3's caps are defined on value leaving accounts. Section 4 never says who may set the
receiving preference. A capability that redirects the destination therefore drains the account holder
while spending nothing and breaching no cap.

**The concrete failure.** An application is granted a modest capability to move value for the account
holder. It calls `setReceivingPreference` once with its own destination. Every subsequent proceed,
refund, surplus and claim from every intent — including intents the application had nothing to do with
— lands with the application. Bounded loss is untouched because the bound is on outflow. Revocation is
forward-looking, so the destination stays changed until someone notices. This is an unbounded loss
inside a system whose first guarantee is bounded loss, and it exists only because section 4 is silent
about authority.

**Evidence it is needed.** This is the receiving-side analogue of the approval-drainer class that
section 3 was written to eliminate, and the drainer pattern reaches victims exactly through
application interaction ([Trezor](https://trezor.io/support/troubleshooting/coins-tokens/dusting-attacks-airdrop-scam-tokens)).
The document already accepts that an unassigned surplus defaults to "whichever operator touches it
last" is a defect (Part IV); an unprotected destination is the same defect with a larger blast radius.

### G3. There is no record of what the account holder is owed, so "proceeds you expected" is undefined

**The gap.** Section 4 distinguishes "proceeds you expected" from "an asset you did not ask to hold"
and supplies no mechanism for the distinction. Inbound value that is not the tail of one of the
account holder's own intents — a customer payment, an invoice settlement, a payroll stream, a promised
transfer from a counterparty — has no representation at all.

**The concrete failure.** A developer builds a storefront. A customer pays. The kernel cannot tell
that payment from an airdrop, so either everything inbound is trusted (G1) or nothing is. The
developer then reimplements matching, expiry and underpayment handling outside the kernel, using the
raw chain data the kernel exists to hide, and loses every guarantee in Part I while doing it.

**Evidence it is needed.** Payment requests are a first-class artifact everywhere, and the field lists
show exactly what receiving needs and what existing standards lack. EIP-681 carries target, chain id,
value, optional function and gas, and explicitly no expiry, no reference or invoice id and no required
asset — and even the amount is "only a suggestion ... which the user is free to change"
([EIP-681](https://eips.ethereum.org/EIPS/eip-681)). ERC-7930 and ERC-7828 exist because a bare
address is ambiguous about which chain it means
([ERC-7930](https://eips.ethereum.org/EIPS/eip-7930)). x402 binds a payment to a specific request
through headers and returns a settlement response the server verifies before releasing the resource
([coinbase/x402](https://github.com/coinbase/x402), [x402 whitepaper](https://www.x402.org/x402-whitepaper.pdf)).
Invoicing systems already name the outcomes this design has no words for: a rate-locked window with a
default 15-minute expiry, a "paid partially" state, configurable underpayment tolerance, and automatic
handling of overpayment ([BTCPay](https://docs.btcpayserver.org/Invoices/),
[BitPay](https://support.bitpay.com/hc/en-us/articles/203411533-Why-is-my-invoice-underpaid-overpaid)).
Without an expectation record the kernel cannot say met, short, late or excess, and every one of those
is a routine commercial event.

### G4. Inbound value carries no grade and no finality, so it is reported as real too early

**The gap.** The grade model in section 1 attaches marks to the outcomes of the account holder's own
intents. Value arriving unbidden has no intent, so it has no strength mark, no finality mark and no
stated reversal condition. Section 4 offers nothing.

**The concrete failure.** Received value is exactly where reversal risk lives. An application credits
a user on first observation; the block is reorganised; the deposit is gone and the credited value has
already left. This is the canonical exchange loss — "if the original deposit later disappears because
of a reorg, the exchange may have already allowed value to leave its platform despite no longer
possessing the corresponding funds", against which the rule is to "wait for finality before doing
anything you cannot undo"
([Trail of Bits](https://blog.trailofbits.com/2023/08/23/the-engineers-guide-to-blockchain-finality/),
[Dwellir](https://www.dwellir.com/blog/what-is-blockchain-finality)). The same shape recurs whenever
inbound value arrives behind an attestation or a dispute window, which this design's own `Contingent`
mark already describes. Honest outcomes is already a standing guarantee; section 4 simply fails to
extend it to the direction where the guarantee is easiest to break.

A second failure rides along: the credited amount is not the sent amount. Assets that take a cut on
transfer deliver less than announced ([AuditBase](https://www.auditbase.com/detectors/fee-on-transfer-solidity)),
and share-based accounting leaves residue — transferring a full balance typically moves only s−1
shares, "leaving the contract holding 1 share of dust", enough to prevent a position ever closing
([ChainSecurity](https://www.chainsecurity.com/blog/the-hitchhikers-guide-to-rebasing-tokens)). A
receipt that repeats what the sender claimed is a false report.

### G5. There is no way to return value to a sender, and no safe way to address a return

**The gap.** Section 6's `Refunded` covers the account holder's own consideration coming back. Nothing
covers sending value back out to whoever sent it: a mistaken transfer, a customer refund, a payment
the account holder declines to accept.

**The concrete failure.** Two. A developer building commerce on this kernel cannot refund a customer at
all, which disqualifies it for payments work. And an account holder who wants to return a misdirected
transfer must construct an outbound intent to an address they read out of their own transaction
history — which is the exact action address poisoning is engineered to exploit. The losses are
enormous and current: about $50M sent to a poisoned address 26 minutes after the attacker planted it,
4,556 ETH (about $12.4M) lost in January 2026, roughly 316,000 confirmed successful attacks since
January 2025 ([Blockaid](https://blockaid.io/blog/address-poisoning-the-growing-threat-draining-millions-from-crypto-users),
[The Block](https://www.theblock.co/post/383423/crypto-trader-loses-50-million-in-address-poisoning-attack-offers-1-million-bounty-for-return),
[Chainalysis](https://www.chainalysis.com/blog/address-poisoning-scam/)).

**Evidence it is needed, and what the fix must be.** Refunds are not reversals: "a crypto refund is
not a reversal of the original transaction but a new outbound payment", requiring a refund address
collected out of band ([CoinGate](https://coingate.com/blog/post/how-crypto-refunds-work)). Circle
built a whole Refund Protocol because non-custodial stablecoin payments have no return path, and its
central design choice is that the payer's refund destination is recorded *at payment time* — "The
smart contract records the recipient address, value, and refund address" — with an arbiter who can
refund but "does *not* have the ability to send funds to an address of their choosing"
([Circle](https://www.circle.com/blog/refund-protocol-non-custodial-dispute-resolution-for-stablecoin-payments)).
The lesson for this kernel is sharper than the protocol: if a return is addressed to the *receipt*
rather than to a string the account holder supplies, address poisoning has no purchase on the return
path at all. That is a guarantee no wallet currently offers, it costs one sentence, and it follows
directly from the kernel already holding the provenance.

### G6. Value that accrues, and value that expires unless claimed, are both missing

**The gap.** Section 4's only non-arrival case is locked positions with a release condition. It has no
category for value that grows in place without any transfer, and none for value that ceases to exist
unless an action is taken by a deadline.

**The concrete failure.** Two directions, both losses.

*Value silently lost to a deadline.* Claim windows expire and the value reverts to the issuer. About
69 million tokens, roughly $57–59M, went unclaimed past one six-month window and were transferred to a
treasury; another programme's final window closed with unclaimed value returned to its DAO; estimates
put 5–20% of distributed value unclaimed as a matter of course
([The Block](https://www.theblock.co/post/252727/arbitrum-dao-unclaimed-airdrop),
[CCN](https://www.ccn.com/news/missed-58m-claimed-arbitrum-treasury-new-deadline/),
[Airdrop Alert](https://airdropalert.com/blogs/what-happens-to-unclaimed-airdrop-tokens/)). Streamed
value has the same shape: the recipient "can send a transaction to withdraw the tokens", accrual and
realisation are separate acts, and a sender can end a stream and recover what has not yet streamed
([Sablier](https://blog.sablier.com/overview-token-streaming-models)). An interface that reports only
what has arrived tells the account holder nothing until the value is already gone.

*The mirror failure.* Claiming can cost more than the claim is worth, which is the dust threshold
again. A kernel that claims everything burns the budget; a kernel that claims nothing loses the
deadline. Neither is acceptable without the account holder being told the cost and the deadline
together.

*And a reporting failure.* Value that grows in place changes balance with no transfer at all
([ChainSecurity](https://www.chainsecurity.com/blog/the-hitchhikers-guide-to-rebasing-tokens),
[Fortress](https://fortress-accounting.com/rebasing-token-accounting/)). Reporting it as spendable
balance is wrong in both directions, since the same mechanism can revise a balance downward.

### G7. "Held and reported as an explicit claim" does not say enough to be a claim

**The gap.** An open claim, as specified, states that something is owed. It does not state where it
sits, who controls it in the meantime, what taking it would cost, whether it expires, or what the
account holder may do other than take it.

**The concrete failure.** The claim list becomes a permanent list of things the account holder cannot
evaluate and cannot clear. Every custodian that has run this surface has ended up with exactly the
policies section 4 omits, and they are unpleasant ones: unsupported deposits are lost unless a
recovery path exists, with a percentage recovery fee on top of network cost, and some assets are
simply unrecoverable ([Coinbase policy](https://help.coinbase.com/en/coinbase/trading-and-funding/sending-or-receiving-cryptocurrency/unsupported-crypto-deposits),
[Kraken](https://support.kraken.com/articles/crypto-assets-deposit-recovery)); a deposit missing its
routing reference sits unassigned until support reassigns it, and if the amount net of the retrieval
fee falls below a minimum, it cannot be retrieved at all
([Binance](https://www.binance.com/en/support/faq/how-to-retrieve-crypto-deposit-with-wrong-or-missing-tag-memo-40b87335db904481888ef406b105442b),
[Cobo](https://help-center.cobo.com/en/articles/11811471-how-to-recover-deposits-with-incorrect-or-missing-memo-tag)).
A claim that cannot state its own cost cannot be acted on rationally. A claim that cannot be abandoned
never goes away — and abandonment is what users actually want for spam, which is why dedicated tools
exist to discard unwanted inbound assets in bulk
([Sol Incinerator](https://sol-incinerator.com/blog/how-to-burn-scam-and-spam-tokens-on-solana)).

### G8. Applications cannot observe receiving, and the statement excludes it

**The gap.** The application interface offers `claims` and `claim` — a pull of exceptions — with no
enumeration of ordinary receipts, no stable receipt identity, and no statement of value received.
Part I defines `statement` as "the cost statement in one asset", and Part II maps it to charges and
meter receipts only. Nothing inbound appears in it.

**The concrete failure.** An application credits a customer twice on a redelivered notification, or
misses a payment entirely, and has no way to reconcile because it cannot enumerate what arrived.
Prevailing practice is unambiguous about what is required: at-least-once delivery is the industry
norm, consumers must deduplicate on an event id or transaction hash, and webhooks must be paired with
a polling reconciliation layer because some events will be missed and some will arrive twice; reorg
handling means corrected data is re-delivered
([Spark](https://www.spark.money/tools/crypto-webhook-notification-comparison),
[Cryptum](https://www.cryptumpay.com/blog-posts/crypto-payment-webhooks),
[dev.to](https://dev.to/kevins1988/crypto-payment-webhooks-the-part-most-developers-get-wrong-2bj0)).
Separately, the account holder must account for received value somewhere the kernel cannot see, and
account-by-account basis tracking is now mandatory with no retroactive reallocation permitted
([Rev. Proc. 2024-28 summaries](https://www.thenetworkfirm.com/blog/2025-new-crypto-tax-rules),
[CoinTracking](https://cointracking.info/crypto-taxes-us/crypto-irs-revenue-procedure-2024-28)). A
statement that omits receipts is not a statement.

### G9. The kernel implies it can control what it receives, and it cannot

**The gap.** Section 7 lists what the kernel will not do and says nothing about what it cannot do on
the inbound side. Section 4's confident "everything arriving from anywhere" invites the reading that
the kernel filters inbound value.

**The concrete failure.** An account holder believes an acceptance policy blocks hostile inbound
value. It cannot: pushes are unrefusable in the general case, because the dominant asset standard has
no receive hook and "Smart contracts cannot reject token transfers because of the lack of standardized
token receive hook" ([dev.to on EIP-223](https://dev.to/dsotec/eip-223-fixing-ethereums-token-loss-problem-31ei),
[status-im issue](https://github.com/status-im/status-mobile/issues/11136)), and "it isn't possible to
decline an incoming transaction on the blockchain"
([gfinityesports on the Tornado dusting](https://www.gfinityesports.com/cryptocurrency/tornado-cash-dusting-attack/)).
The account holder then relies on a filter that does not exist and is surprised when a sanctioned-source
dust transfer is sitting against their identity. The honest claim is narrow and still valuable: the
kernel controls what it *touches*, not what exists.

The same honesty is owed on screening. Provenance assessment is probabilistic, hop-based and revises
over time ([Chainalysis](https://www.chainalysis.com/blog/cryptocurrency-risk-blockchain-analysis-indirect-exposure/)),
a clean report today is no protection against an issuer freeze tomorrow
([Koru Legal](https://korulegal.com/en/blog/usdt-usdc-freeze-stablecoin-blacklist-clean-coin/)), and
the obligations in this area attach to obliged institutions rather than to a non-custodial kernel, with
beneficiary-side duties discharged against their own customer
([FATF best practices, June 2025](https://www.fatf-gafi.org/content/dam/fatf-gafi/recommendations/Best-Practices-Travel-Rule-Supervision.pdf),
[Sumsub](https://sumsub.com/blog/what-is-the-fatf-travel-rule/)). A kernel that presents a screening
result as clearance is making a promise it has no standing to make.

### G10. Asset identity is guaranteed in general and unguaranteed exactly where it is attacked

**The gap.** Section 1 and section 7 already refuse to treat two assets as the same because they share
a name. Section 4 then says inbound value "is routed to that preference" without saying how an inbound
asset is identified as the preferred one.

**The concrete failure.** An attacker mints an asset whose name matches the account holder's settlement
asset and pushes it. A name-matching router reports settlement asset received. The honest-user version
of the same confusion is routine and costly: a bridged variant is not the issued asset, "Exchanges,
payment processors, and many DeFi protocols only accept native USDC for deposits", and sending the
variant to a venue leaves funds "lost or stuck in support limbo for weeks", while burn-and-mint
transfer yields the canonical asset rather than a wrapped one
([Eco on CCTP](https://eco.com/support/en/articles/14998923-cctp-cross-chain-usdc-complete-guide-2026),
[stablecoininsider](https://stablecoininsider.org/how-to-recover-usdc-sent-to-the-wrong-network/)).
One sentence in section 4 closes this, and it only restates an existing guarantee in the place it is
actually tested.

---

## ADDITIONS

Six additions, written to be pasted into Part I section 4 (with one line for section 5's call table).
They name no chain, no venue and no asset. Each covers several gaps deliberately; the intent is that
section 4 roughly doubles in length and answers ten gaps, not that it acquires ten bullet lists.

Note on naming: **receiving preference becomes receiving policy**, matching the spending policy in
section 3, because it now has two halves rather than one field. The calls become
`setReceivingPolicy` / `receivingPolicy`.

### A1 — Acceptance comes before routing

*Replaces the opening of section 4 and its second paragraph. Covers G1, and with A5 resolves the
existing contradiction about held value.*

> One receiving policy, written once, binding everywhere. It has two halves: what you accept, and
> where accepted value lands.
>
> You state what you accept — value you are expecting, value from counterparties you name, assets you
> name — and everything else is unsolicited. Accepted value is routed to your destination in your
> settlement asset without your involvement, on whatever chain it arrives, from whatever venue, so
> you never maintain a destination per chain. Unsolicited value is recorded where it sits, attributed
> to you, reported to you, and otherwise untouched: not priced, not converted, not moved, and never
> merged with your settlement destination or with value from another source, until you decide it.
>
> **Nothing arriving unbidden spends your money.** Pricing, routing, converting and consolidating all
> cost something, and a stranger who could trigger them would be choosing your expenditure. The
> kernel therefore performs none of them for value you did not ask for, and performs none of them for
> value whose delivery would cost more than the value is worth. It reports the shortfall and leaves
> the decision with you.
>
> **The kernel never runs an unrecognized asset's own logic inside your settlement path.** An asset
> whose transfer, conversion or redemption behaviour it cannot evaluate in advance is left as it
> arrived and reported as unroutable. It is never carried through a route on the strength of what it
> claims about itself.

**Without it:** an attacker who spends dust makes the account holder pay routing and quoting costs at
the attacker's chosen volume, drags hostile asset code into the settlement path, links every
receiving address the identity owns, and consolidates sanctioned-source value into the one destination
every proceed lands in — where an issuer freeze ends the account holder's ability to send or receive
at all.

### A2 — Nothing acting for you can change where your value lands

*Add to section 4. Covers G2.*

> Your receiving policy is set by your identity and by nothing acting for it. No capability, however
> broad, and no application, however trusted, can change where your value lands or what you accept. A
> capability may read what you have received, and may direct value it is owed to itself where the
> intent said so before you signed — and that is the whole of its reach over receiving. A redirected
> destination would take everything you receive without anything leaving your accounts and without
> exceeding any cap you set, so it is not a permission the kernel offers.

**Without it:** a delegated application redirects the destination once and collects every future
receipt. The loss is unbounded and breaches no spending cap, because every cap in section 3 is defined
on value leaving.

### A3 — What you are owed is stated before it arrives

*Add to section 4. Covers G3, and gives "proceeds you expected" a definition.*

> You can state what is owed to you before it arrives: the amount, the asset, the counterparty where
> you know it, your own reference, and the time by which it is due. That is an expectation, and it is
> what makes an arrival expected. Value matching an expectation is applied to it and routed. Value
> matching no expectation is unsolicited, whatever it is and whoever sent it.
>
> A request you hand to a payer carries the asset, the amount, your reference and the deadline, and
> names no chain. Which network a payer uses to satisfy it is their routing decision, and you are
> told which one they used after the fact, not asked to choose beforehand.
>
> An expectation ends in exactly one of: met, met in part, met in excess, met late, or unmet at its
> deadline. Each is reported as itself. An amount short of what was due is never reported as met, an
> expectation is never quietly extended past its deadline, and value beyond what was due is reported
> as excess rather than absorbed. These are the states of an expectation and they do not widen the
> outcomes an intent can reach.

**Without it:** no application can tell a customer payment from an airdrop, so it must either trust
everything inbound or rebuild matching, expiry, underpayment and overpayment handling on raw chain
data outside every guarantee in Part I.

### A4 — A receipt is graded, measured, and identified

*Add to section 4. Covers G4 and G10.*

> Every receipt carries the same two marks as every outcome. A receipt marked contingent names the
> condition that could still undo it and when that condition closes, and it is reported and not
> spendable until then. Value is never reported as received because a sender announced it, only
> because the kernel observed it, and never as final because time passed.
>
> A receipt states the amount actually credited, measured after arrival. It never repeats the amount a
> sender said they sent.
>
> A receipt names its asset by what issued it, never by the name the asset carries. An inbound asset
> bearing your settlement asset's name is not your settlement asset until the kernel has established
> that it is, and a name alone never establishes it.

**Without it:** an application credits a user on first sight and the value is reorganised away or a
dispute window reverses it; a fee-taking or share-rounded asset is credited at the wrong amount and
a position can never be closed; and an attacker's imitation of the settlement asset is reported as
the settlement asset arriving.

### A5 — A claim states its cost, its custody and its exits

*Replaces the existing "held and reported as an explicit claim" bullet. Covers G5 and G7.*

> An open claim states what is owed, where it sits until you act, what taking it requires, what taking
> it would cost you in your settlement asset, and whether it expires. A claim that cannot state its
> cost and its custody is not reported as a claim, because you could not decide about it.
>
> You have four exits from a claim, and the interface offers all four: take it to your destination;
> take it once to a different destination without changing your policy; return it to the origin of the
> receipt; or abandon it. Abandonment is recorded, is permanent, and is stated to be permanent before
> you choose it.
>
> **A return is addressed to the origin of the receipt, never to an address you supply.** The kernel
> already knows where value came from, so it never asks you to retype, paste or pick out an address
> in order to send value back. A return is a new outbound movement under your policy and your caps,
> and it is never presented as a reversal of what arrived.

**Without it:** the claim list becomes an unclearable inventory of things whose cost and location the
account holder cannot see; a developer cannot refund a customer at all; and an account holder
returning a mistaken payment must copy an address out of a transaction history, which is the precise
action address-poisoning attacks are built to capture.

### A6 — Holdings are reported in three states, and deadlines are never silent

*Replaces the existing locked-positions bullet. Covers G6.*

> What you hold is reported in three states that are never merged: spendable, accruing, and claimable.
>
> Accruing value is value growing in place that no transfer has delivered. It is reported with what it
> has accrued and it is not spendable. A mechanism that can revise it downward is reported as one.
>
> Claimable value exists only if an action is taken, and the report names the action, its cost in your
> settlement asset, and the deadline after which the value ceases to be yours. The kernel does not let
> such a deadline pass without telling you it is closing, and does not spend your budget to take
> something worth less than it costs to take. Authority to realize a claim lives inside an intent with
> its own cap and expiry, like every other authority; there is no standing permission over your
> positions.
>
> Positions that are locked or subject to a release condition are reported with that condition and are
> never shown as spendable balance.

**Without it:** value the account holder owns reverts to whoever issued it because a window closed
unannounced, streamed value sits unrealized until a sender withdraws it, and balances that change with
no transfer are reported as spendable and then revised.

### A7 — Applications can observe receiving without touching it

*Add to section 4 and amend the section 5 table. Covers G8.*

> Receipts are records. Each has a stable identity, each is enumerable from any point you choose, and
> reporting the same receipt twice reports the same record — so an application that credits a payment
> twice is a bug the interface did not create.
>
> Your statement covers value received as well as value spent: each receipt with its time, its
> measured amount, its asset, its counterparty where known, the expectation it satisfied, and its
> marks. It is exportable, because you will have to account for what you received somewhere the kernel
> cannot see.

*Section 5 table, Receiving row:*

> | Receiving | `setReceivingPolicy`, `receivingPolicy`, `expect`, `expectations`, `receipts`,
> `claims`, `claim`, `returnReceipt`, `abandonClaim` | Say what you accept and where it lands; state
> what you are owed; read what arrived; take, return or abandon what is open |

**Without it:** an application double-credits on a redelivered notification or silently misses a
payment, with no enumeration to reconcile against; and the account holder cannot produce a record of
received value for accounting that is now required to be kept per account with no retroactive fix.

---

## REFUSE AND SAY SO

Sentences for section 7. Seven, in the existing voice.

> - **It will not route, convert, consolidate or price value you did not ask for.** Doing any of those
>   costs money, and a stranger able to trigger them would be choosing what you spend.
> - **It will not spend more to deliver something than the something is worth.** It reports the
>   shortfall and leaves the choice with you.
> - **It will not run an unrecognized asset's own logic inside your settlement path** to deliver that
>   asset to you.
> - **It will not claim it can refuse an inbound transfer.** Networks let strangers push value at you
>   and no one can decline it. What the kernel controls is what it touches: it can leave value where it
>   is, keep it apart from everything else you hold, and tell you it is there. That is the whole of
>   what is possible, and it is stated rather than implied.
> - **It will not let anything acting for you change where your value lands.**
> - **It will not treat an inbound asset as your settlement asset because it carries that name.**
> - **It will not present what it knows about where value came from as a clearance.** Provenance
>   assessment is evidence of the same imported kind as any other attested fact, it changes as more is
>   learned, and no report of it protects you from whoever issued the asset deciding otherwise later.
> - **It will not let a claim deadline pass without telling you**, and will not take a claim on your
>   behalf without authority that names its cost.

---

## DO NOT ADD

**A spam classifier, sender reputation or a trust score.** Tempting because wallets ship them. Wrong
here because the kernel would be guaranteeing an inference. Every false negative becomes a breach of
a Part I guarantee, and the classifier is a permanent adversarial maintenance burden. Default-deny by
expectation gets the same protection from a fact the kernel already has, and vendor practice is the
same shape: allowlists of established assets, not judgments about bad ones.

**Screening as a gate that blocks inbound value.** Cannot be done — pushes are unrefusable — and
should not be promised. A gate also makes the kernel a censorship point with no standing to be one;
the obligations here attach to obliged institutions, not to a non-custodial kernel. Segregate,
report provenance as imported evidence, refuse the word "clearance".

**Automatic disposal of unwanted inbound value to reclaim storage deposits.** Chain-specific, and it
requires exactly the interaction A1 forbids: touching an attacker-authored asset in order to get rid
of it, which is the bait. Abandonment stays a record of a decision, not an on-chain act.

**Refund arbitration with a third-party arbiter.** The nearest existing design needs an arbiter able
to refund from the recipient's balance. That is standing authority over the account holder's funds and
contradicts guarantee three outright. Returns stay voluntary, addressed to the receipt, and executed
under the account holder's own policy.

**Payer identity, verification or travel-rule data collection.** The kernel cannot verify who a payer
is and is not the obliged party. It can carry a reference the payer supplies and name the origin of a
receipt. Anything more is an identity claim it cannot support.

**A tax lot engine, basis allocation or gain computation.** Jurisdictional, moving, and already
changed once in the period this design covers. Export complete records; compute nothing.

**Streaming as a first-class primitive.** Per-second balances, solvency buffers and liquidation are a
venue mechanism, and importing them into Part I imports a mechanism the account holder was promised
never to see. The accruing state covers what the account holder needs: value growing in place, not
yet delivered, not spendable.

**A destination per chain, per asset or per counterparty.** It defeats the premise of one receiving
policy and recreates the per-chain bookkeeping guarantee four abolishes. One-off redirection of a
single claim covers the legitimate need.

**Automatic valuation of unsolicited assets, even just to sort the list.** Valuing requires soliciting
priced quotes, and priced quotes are the griefing vector. An unsolicited receipt is reported by what it
is and who sent it, not by what it might be worth.

**Notification transport — webhooks, subscriptions, delivery guarantees — in Part I.** The guarantee
that matters is that receipts have stable identity, are enumerable from a chosen point, and never
duplicate. Given that, any transport is reconcilable. Delivery mechanics belong below the line.

---

## SOURCES

**Hostile inbound**
- https://blockaid.io/blog/address-poisoning-the-growing-threat-draining-millions-from-crypto-users — scale of address poisoning: 65.4M flagged transactions since Jan 2025, ~316k confirmed successes.
- https://www.theblock.co/post/383423/crypto-trader-loses-50-million-in-address-poisoning-attack-offers-1-million-bounty-for-return — ~$50M sent to a poisoned address 26 minutes after planting.
- https://www.chainalysis.com/blog/address-poisoning-scam/ — attack anatomy: look-alike generation, automated dusting, history manipulation.
- https://www.ledger.com/academy/topics/security/what-are-address-poisoning-attacks-in-crypto-and-how-to-avoid-them — zero-value transfer variant; history as the attack surface.
- https://en.wikipedia.org/wiki/Dusting_attack and https://www.lightspark.com/glossary/dust-attack — dusting as deanonymization; the defence is inaction and non-co-spending.
- https://trezor.io/support/troubleshooting/coins-tokens/dusting-attacks-airdrop-scam-tokens — vendor guidance: assume unsolicited assets are scams; interacting is the loss event.
- https://sol-incinerator.com/blog/how-to-burn-scam-and-spam-tokens-on-solana — bulk spam distribution economics; spam as bait; disposal tooling and reclaimable deposits.
- https://support.metamask.io/stay-safe/safety-in-web3/token-safety-practices/ — detection is allowlist-driven as airdrop-scam protection.
- https://support.metamask.io/manage-crypto/portfolio/how-to-hide-an-asset-token-or-nft-in-metamask-portfolio/ — hiding is display-only and changes no balance.

**Malicious or non-standard asset logic**
- https://ackee.xyz/blog/reentrancy-attack-in-erc-777/ and https://medium.com/immunefi/the-potential-impact-of-erc-777-tokens-on-defi-protocols-51cdb07be733 — receive-hook reentrancy loss record (~$300k imBTC/Uniswap V1, ~$25M Lendf.Me, ~$18.8M Cream); malicious receivers that revert to break distribution.
- https://www.auditbase.com/detectors/fee-on-transfer-solidity — amount sent ≠ amount received; balance-delta accounting breaks.
- https://www.chainsecurity.com/blog/the-hitchhikers-guide-to-rebasing-tokens — share rounding leaves 1-share dust that blocks position closure; accumulated rounding reverts transfers.
- https://fortress-accounting.com/rebasing-token-accounting/ — balances change with no transfer; reporting consequences.
- https://dev.to/dsotec/eip-223-fixing-ethereums-token-loss-problem-31ei and https://github.com/status-im/status-mobile/issues/11136 — no standard receive hook, so pushes cannot be rejected and tokens sent to unprepared contracts are lost.

**Payment requests, invoicing, machine payments**
- https://eips.ethereum.org/EIPS/eip-681 — request fields; no expiry, no reference, no required asset; amounts are suggestions.
- https://eips.ethereum.org/EIPS/eip-7930 and https://eips.ethereum.org/EIPS/eip-7828 — a bare address is chain-ambiguous; chain-qualified and human-readable forms.
- https://github.com/coinbase/x402 and https://www.x402.org/x402-whitepaper.pdf — payment bound to a specific request; server verifies before serving; settlement response as receipt.
- https://docs.btcpayserver.org/Invoices/ — rate-locked request with a default 15-minute expiry; partially-paid state; underpayment tolerance.
- https://support.bitpay.com/hc/en-us/articles/203411533-Why-is-my-invoice-underpaid-overpaid — underpaid and overpaid as first-class states with automatic handling.

**Cross-chain and identity of received assets**
- https://eco.com/support/en/articles/14998923-cctp-cross-chain-usdc-complete-guide-2026 — burn-and-mint yields the canonical asset, not a wrapped one.
- https://stablecoininsider.org/how-to-recover-usdc-sent-to-the-wrong-network/ — bridged variants rejected by venues; right ticker, wrong chain as the most common error.
- https://developers.circle.com/cctp/references/technical-guide — destination-side post-mint callbacks and their integrator-side implementation burden.
- https://help.coinbase.com/en/coinbase/trading-and-funding/sending-or-receiving-cryptocurrency/unsupported-crypto-deposits and https://support.kraken.com/articles/crypto-assets-deposit-recovery — unsupported inbound is lost absent a recovery path; percentage recovery fee; some assets unrecoverable.
- https://www.binance.com/en/support/faq/how-to-retrieve-crypto-deposit-with-wrong-or-missing-tag-memo-40b87335db904481888ef406b105442b and https://help-center.cobo.com/en/articles/11811471-how-to-recover-deposits-with-incorrect-or-missing-memo-tag — missing routing reference leaves value unassigned; retrieval fee can exceed the amount.

**Finality on the receiving side**
- https://blog.trailofbits.com/2023/08/23/the-engineers-guide-to-blockchain-finality/ — deposit-credit-then-reorg as the canonical loss.
- https://www.dwellir.com/blog/what-is-blockchain-finality — credit after finality; confirmation depths in practice.

**Accrual, streaming, claim deadlines, dust economics**
- https://blog.sablier.com/overview-token-streaming-models — accrual and withdrawal are separate acts; senders can cancel and recover unstreamed value.
- https://www.theaccountantquits.com/articles/stream-payments-using-superfluid — continuously updating balances; off-chain liquidators maintain solvency.
- https://www.theblock.co/post/252727/arbitrum-dao-unclaimed-airdrop and https://www.ccn.com/news/missed-58m-claimed-arbitrum-treasury-new-deadline/ — ~69M tokens (~$57–59M) unclaimed past a six-month window, transferred to treasury.
- https://airdropalert.com/blogs/what-happens-to-unclaimed-airdrop-tokens/ — 5–20% typically unclaimed; windows of 30 days to six months; revert or burn.
- https://www.trondealer.com/en/guides/integration — a token-only receiving address has nothing to pay a fee with; sweeping a $3 deposit for a $2 fee destroys value.
- https://www.spark.money/research/bitcoin-utxo-management-strategies — dust thresholds; sub-$10 holdings uneconomic to move individually.

**Returns and refunds**
- https://coingate.com/blog/post/how-crypto-refunds-work — a refund is a new outbound payment, not a reversal; refund destination must be collected.
- https://www.circle.com/blog/refund-protocol-non-custodial-dispute-resolution-for-stablecoin-payments — refund address recorded at payment time; arbiter can refund but cannot direct funds to an address of its choosing.

**Application reconciliation and accounting**
- https://www.spark.money/tools/crypto-webhook-notification-comparison and https://www.cryptumpay.com/blog-posts/crypto-payment-webhooks — at-least-once delivery, idempotency on event id or transaction hash, polling reconciliation layer, reorg re-delivery.
- https://dev.to/kevins1988/crypto-payment-webhooks-the-part-most-developers-get-wrong-2bj0 — accept, queue, process asynchronously; duplicates are normal.
- https://www.thenetworkfirm.com/blog/2025-new-crypto-tax-rules and https://cointracking.info/crypto-taxes-us/crypto-irs-revenue-procedure-2024-28 — per-account basis tracking mandatory from 2025; no retroactive reallocation.
- https://coinledger.io/blog/airdrop-taxes — unsolicited receipts and dominion-and-control; phantom liability risk.

**Screening, freezes, taint**
- https://decrypt.co/107090/tornado-cash-dusts-public-wallets-jimmy-fallon-brian-armstrong-steve-aoki-logan-paul and https://www.gfinityesports.com/cryptocurrency/tornado-cash-dusting-attack/ — sanctioned-source dusting as a deliberate grief; an inbound transfer cannot be declined.
- https://bitcoinmagazine.com/news/kraken-says-users-were-dust-attacked — exchange users dusted from a sanctioned wallet.
- https://www.chainalysis.com/blog/cryptocurrency-risk-blockchain-analysis-indirect-exposure/ — hop-based indirect exposure; received provenance raises an address's own risk.
- https://korulegal.com/en/blog/usdt-usdc-freeze-stablecoin-blacklist-clean-coin/ and https://eaglevirtual.com/learn/usdc-usdt-blacklist-freeze-functions — issuer blacklisting stops sending and receiving; >$4bn frozen; 384 addresses / ~$515M in one 30-day window.
- https://leodex.io/learn/exchange-issues/why-exchanges-freeze-accounts — deposits scored on inbound provenance; whole accounts quarantined.
- https://www.fatf-gafi.org/content/dam/fatf-gafi/recommendations/Best-Practices-Travel-Rule-Supervision.pdf and https://sumsub.com/blog/what-is-the-fatf-travel-rule/ — beneficiary-side obligations attach to obliged institutions and are discharged against their own customer.
