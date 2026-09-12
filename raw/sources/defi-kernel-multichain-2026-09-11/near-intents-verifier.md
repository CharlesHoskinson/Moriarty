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

Verifier Contract

Verifier Contract

[Swaps](/getting-started/what-are-intents)[Market Makers](/integration/market-makers/introduction)[Bridges](/integration/bridging/overview)[Verifier Contract](/integration/verifier-contract/introduction)[Changelog](/changelog/overview)

Verifier Contract

Verifier Contract
=================

Copy page

The on-chain settlement layer for NEAR Intents

Copy page

![NEAR Intents Verifier Contract](https://mintcdn.com/defuselabsltd/OBaHx8_FZb8uK__Y/images/diagrams/verifier-contract-intro-light.png?fit=max&auto=format&n=OBaHx8_FZb8uK__Y&q=85&s=1e1a1cef101bddee33e469aa5bd16f62)
![NEAR Intents Verifier Contract](https://mintcdn.com/defuselabsltd/OBaHx8_FZb8uK__Y/images/diagrams/verifier-contract-intro-dark.png?fit=max&auto=format&n=OBaHx8_FZb8uK__Y&q=85&s=fd39c26f6eb1287c10bd7e2d7fa40e20)
The Verifier Smart Contract (`intents.near`) is the on-chain settlement layer for NEAR Intents. Deposited tokens are credited to accounts in the contract’s ledger. Swaps and transfers update those records; tokens leave the contract only on withdrawal through [token bridges](/integration/bridging/overview). Intents Technology does not custody these balances — control stays with the keys that can sign for each account.
The typical flow is:

1. **Deposit** tokens to `intents.near`. The contract credits your account in its on-chain ledger. *(See [Deposits](/integration/verifier-contract/deposits-and-withdrawals/deposits))*
2. **Swap or transfer** by submitting intents. Users express their “intent” to perform a transaction (e.g., exchange USDT for USDC, or transfer tokens to another user). Intents can be submitted directly or bundled together by a third party like the Message Bus for atomic execution. *(See [Intent Types](/integration/verifier-contract/intent-types-and-execution))*
3. **Withdraw** tokens to your NEAR account or external chain. For external chain asset transfer, the Verifier settles the withdrawal through the appropriate token bridge, delivering tokens to the destination address. *(See [Withdrawals](/integration/verifier-contract/deposits-and-withdrawals/withdrawals))*

Who is this documentation for?
------------------------------

Most applications integrating token swaps do not need to interact with the Verifier contract directly. The [1Click Swap API](/integration/distribution-channels/1click-api/about-1click-api) handles intent creation, market maker coordination, and execution.
This section is for developers who need lower-level control and want to:

* Interact with the Verifier smart contract directly
* Create payloads for the [Message Bus](/integration/market-makers/message-bus/introduction) (a matching system that brings together quotes from market makers with transaction requests)

Prerequisites
-------------

Before working directly with the Verifier contract, you should have:

* A [NEAR account](https://docs.near.org/tutorials/protocol/create-account) (named account like `yourname.near` or implicit account)
* Tokens to deposit (fungible tokens like USDC, USDT, or wrapped NEAR)
* Basic understanding of [NEAR transactions](https://docs.near.org/concepts/protocol/transactions) and [cross-contract calls](https://docs.near.org/smart-contracts/anatomy/crosscontract)
* Familiarity with JSON and digital signatures

If you’re new to NEAR, start with the [NEAR documentation](https://docs.near.org/) to understand accounts, transactions, and the basics of smart contract interaction.

Deployment
----------

The Verifier smart contract is deployed at [`intents.near`](https://nearblocks.io/address/intents.near).

There is no testnet deployment. Use small amounts for testing purposes.

Source code
-----------

The contract is open source. You can find it on GitHub:

NEAR Intents Repository
-----------------------

View the source code for the Verifier smart contract

The former name of the smart contract is “Defuse”. You may still see this name in some places in the codebase. It is planned to be updated in the future.

Next steps
----------

Account Abstraction
-------------------

Learn about account identification and key management

Deposits & Withdrawals
----------------------

Learn how to deposit and withdraw tokens

Intent Types
------------

Explore available intent types and how to structure them

Signing Intents
---------------

Understand how to sign intents for different wallet types

Was this page helpful?

YesNo

[Account Abstraction

How the Verifier contract identifies users and manages account keys

Next](/integration/verifier-contract/account-abstraction)

⌘I

[x](https://x.com/near_intents)[github](https://github.com/defuse-protocol)[telegram](https://t.me/near_intents)

[Powered byThis documentation is built and hosted on Mintlify, a developer documentation platform](https://www.mintlify.com?utm_campaign=poweredBy&utm_medium=referral&utm_source=defuselabsltd)