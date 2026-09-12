[Skip to main content](#content-area)

* [System Status](https://status.near-intents.org/posts/dashboard)
* [Explorer](https://explorer.near-intents.org/)
* [Support](https://t.me/near_intents)
* [Partners](https://partners.near-intents.org/)

[Swaps](/getting-started/what-are-intents)[Market Makers](/integration/market-makers/introduction)[Bridges](/integration/bridging/overview)[Verifier Contract](/integration/verifier-contract/introduction)[Changelog](/changelog/overview)

[NEAR Intents home page![light logo](https://mintcdn.com/defuselabsltd/H7BXykL_JPKw4xWb/images/logo/light.svg?fit=max&auto=format&n=H7BXykL_JPKw4xWb&q=85&s=e31b086fb76c5f6228ea4020afc8db58)![dark logo](https://mintcdn.com/defuselabsltd/H7BXykL_JPKw4xWb/images/logo/dark.svg?fit=max&auto=format&n=H7BXykL_JPKw4xWb&q=85&s=24c0664a9d5a23125fbdbafce3213d1f)](/)

Search...

⌘KAsk Assistant⌘I

* [System Status](https://status.near-intents.org/posts/dashboard)
* [Explorer](https://explorer.near-intents.org/)
* [Support](https://t.me/near_intents)
* [Partners](https://partners.near-intents.org/)
* [Partners](https://partners.near-intents.org/)

Search...

Navigation

Market Makers

Market Makers

[Swaps](/getting-started/what-are-intents)[Market Makers](/integration/market-makers/introduction)[Bridges](/integration/bridging/overview)[Verifier Contract](/integration/verifier-contract/introduction)[Changelog](/changelog/overview)

Market Makers

Market Makers
=============

Copy page

Fulfill cross-chain swap intents as a liquidity provider

Copy page

Market Makers compete to fulfill user [Swap intents](/integration/distribution-channels/introduction). They listen for swap requests on the [Message Bus](/integration/market-makers/message-bus/introduction), evaluate whether they can fill the request, and respond with signed quotes.
![NEAR Intents Market Makers](https://mintcdn.com/defuselabsltd/OBaHx8_FZb8uK__Y/images/diagrams/market-makers-intro-light.png?fit=max&auto=format&n=OBaHx8_FZb8uK__Y&q=85&s=2a907b1f71b967cc47d9accbcb049cf7)
![NEAR Intents Market Makers](https://mintcdn.com/defuselabsltd/OBaHx8_FZb8uK__Y/images/diagrams/market-makers-intro-dark.png?fit=max&auto=format&n=OBaHx8_FZb8uK__Y&q=85&s=b180131b3ec3c921a63edc30828c6ea3)


---

How it works
------------

1

A Quote Request is sent

A user sends a [quote request](/integration/distribution-channels/1click-api/quickstart/making-a-request#request-token) to the [Message Bus](/integration/market-makers/message-bus/introduction), a WebSocket relay that broadcasts the request to all connected solvers.

2

Solvers evaluate and quote

Each solver checks whether they can fulfill the swap. If they can, they compute pricing, and return a signed quote as response.

3

Quotes compete for selection

Multiple solvers can respond to the same request with different prices. The Message Bus collects responses and returns the top quotes to the user application.

4

Selected quote settles on-chain

After the user chooses a quote, the Message Bus matches the requested intent with the selected quote and submits it to the [Verifier contract](/integration/verifier-contract/introduction) where the swap settles on-chain.

The NEAR Intents protocol can operate without the Message Bus. Frontends can use other quoting mechanisms, and market makers can index the NEAR blockchain directly to find intents to fill.

---

Next steps
----------

Quickstart
----------

Set up and run a solver that automatically responds to quote requests

Usage Examples
--------------

Understand the code necessary to make a solver respond to quotes

Confidential Intents
--------------------

Solve for private liquidity with shielded balances

Message Bus
-----------

Learn about the Message Bus architecture and how it works

Was this page helpful?

YesNo

[Quickstart

Become a market maker on NEAR Intents

Next](/integration/market-makers/quickstart)

⌘I

[x](https://x.com/near_intents)[github](https://github.com/defuse-protocol)[telegram](https://t.me/near_intents)

[Powered byThis documentation is built and hosted on Mintlify, a developer documentation platform](https://www.mintlify.com?utm_campaign=poweredBy&utm_medium=referral&utm_source=defuselabsltd)