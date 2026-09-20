[Frontier Research](/)/[🎂

Introducing the CAKE framework](/the-cake-framework)

🎂

Introducing the CAKE framework
==============================

*Chain Abstraction is a piece of CAKE* 🍰

✍🏼

by [Ankit Chiplunkar](https://twitter.com/ankitchiplunkar) & [Stephane Gosselin](https://twitter.com/thegostep) for Frontier Research

🗓️

15th Feb 2024

TL;DR
-----

* The default crypto UX today is for users to always know which network they are interacting with. However, users of the internet don't have to know which cloud provider they are interacting with. Bringing this approach to blockchains is what we call Chain Abstraction.
* This article introduces the CAKE framework i.e. Chain Abstraction Key Elements. It is composed of four layers: Applications, Permissions, Solving, and Settlement, which collectively facilitate seamless cross-chain operations for users.
* Achieving Chain Abstraction requires the use of a complex set of technologies to provide reliable, cost efficient, secure, fast, and private execution.
* We define the cross-chain tradeoff space in chain abstraction as a trilema and propose six designs, which each offering unique advantages.
* In order to successfully make the leap to a chain abstraction future, it is imperative we as an industry define and adopt a common standard for messaging between the layers of the CAKE. A great standard is the icing on the cake. 🎂

Introduction
============

In 2020, the Ethereum network transitioned to a [rollup centric roadmap](https://ethereum-magicians.org/t/a-rollup-centric-ethereum-roadmap/4698) for scaling. Four years since that decision more than 50 rollups (L2s) are already live in production. While rollups provide much needed horizontal scaling for the EVM blockspace, it has [totally ruined the user experience](https://twitter.com/kylesamani/status/1736838682009854213?s=12).

Users should neither care, nor know, which rollup they are interacting with. Crypto users knowing which rollup (Optimism or Base) they are interacting with is equivalent to web2 users knowing which cloud provider (AWS or GCP) they are interacting with. Chain Abstraction is a vision where chain information is abstracted away from the user. The user only connects their wallet to a dApp and signs for the intended operation, the details of making sure that the user has correct balance on the target chain and and then executing the intended operation happens behind the scenes.

Over the course of this article, we will observe that Chain Abstraction is a truly multi-disciplinary problem. Involving interactions with the Application Layer, Permission Layer, Solver Layer and Settlement Layer. We introduce the Chain Abstraction Key Elements (CAKE 🎂) framework and then delve deeper into the design trade-offs of chain abstraction systems.

Introducing the CAKE Framework
==============================

![Blockchain development is a piece of CAKE! 🍰](https://images.spr.so/cdn-cgi/imagedelivery/j42No7y-dcokJuNgXeA0ig/d6534ef6-766f-4199-af31-de9ee7cb76cd/Untitled-2024-02-14-1754/w=1920,quality=90,fit=scale-down)

Blockchain development is a piece of CAKE! 🍰

In a chain abstracted world a user goes to a dApps website, connects their wallet, signs the intended operation and waits for eventual settlement. All the complexity of acquiring the required assets to the target chain and the final settlement gets abstracted away from the user, happening in infrastructure layers of the CAKE. There are three infrastructure layers of the CAKE:

1. **Permission layer:** The user connects their wallet to a dApp and requests the quote for a user intent. An **intent** is what the user expects (i.e. output) at the end of a transaction and not the eventual path the transaction takes. It can be transferring USDT to a Tron address or depositing USDC into a yield generating strategy on Arbitrum. The wallet should be able to both know the users assets (i.e. read state) and execute transactions (i.e. update state) on target chains.
2. **Solver layer:** The solver layer estimates fees and execution speed based on the user's initial balance and intent. This process, referred to as **solving**, is crucial in a cross-chain setting where transactions become asynchronous and sub-transactions may fail during execution. The introduction of asynchronicity creates a cross-chain trilemma involving fees, execution speed, and execution guarantee.
3. **Settlement layer:** After the user approves the transaction with their private key, the settlement layer ensures its execution. It involves two steps: bridging the user's assets onto the target chain and then executing the transaction. If the protocol uses sophisticated solvers for certain operations, they can bring their own liquidity and execute the operation on behalf of the user without the need for bridging.

Achieving Chain Abstraction means combining the above three infrastructure layers into a unified product. A key insight while combining these layers is the difference between transferring information vs transferring value. Transferring information between chains should be lossless and thus needs reliance on the most secure pathways. Suppose a user is trying to vote **Yes** on a governance vote from one chain to another, they don’t want their vote to convert to a Maybe. On the other hand transferring value can be lossy based on users preference. A sophisticated third-party can be leveraged to give the user faster, cheaper or guaranteed transfer of value. Note, 95% of ethereum blockspace (weighted by fees paid to validators) is consumed for transferring value.

Key Design Decisions
====================

The above three layers, introduce key design decisions which need to be taken by a CAF. They are related to who controls power over execution of the intent, what information should be revealed to solvers and what are the settlement pathways available to solvers. Let's look at each of them in detail.

![Chain Abstraction design decisions](https://images.spr.so/cdn-cgi/imagedelivery/j42No7y-dcokJuNgXeA0ig/3b456355-1f5a-48cb-87cd-0fcdff8a566a/design_decisions/w=1920,quality=90,fit=scale-down)

Chain Abstraction design decisions

Permission Layer
----------------

The permission layer holds the private key for the user and signs messages on their behalf, which are then executed on-chain as transactions. A CAF needs to support signing schemes and transaction payloads for all the target chains it wants to support. For example, a wallet supporting the ECDSA signing scheme and EVM transaction standard will be limited to Ethereum, its L2s, and its side-chains (e.g., Metamask wallet). On the other hand, a wallet supporting both EVM and SVM (Solana VM) will be able to support both ecosystems (e.g., Phantom wallet). It's important to note that the same mnemonic can be used to generate wallets on both EVM and SVM chains.

A single multi-chain transaction consists of several sub-transactions that need to be executed in the correct order. These sub-transactions must be executed on multiple chains, each with its own time-varying fees and nonce. How the coordination and settlement of these sub-transactions takes place is a crucial design decision for permission layer.

1. **EOA wallets** are wallet software that run on users' machines and hold their private keys. They can be browser-based extensions (such as Metamask and Phantom), mobile apps (such as Coinbase Wallet), or dedicated hardware (such as Ledger). EOA wallets require the user to individually sign each sub-transaction, which currently requires multiple clicks. They also require the user to hold fee balances on the target chain, which introduces significant friction in the process. However, the friction of multiple clicks can be abstracted away from the user by allowing them to sign multiple sub-transactions with a single click.
2. In **Account Abstraction (AA) wallets** the user still has access to their private key but they separate the signer of the transaction payload with the executor of the transaction. Enabling sophisticated parties to atomically bundle and execute users transactions (Avocado, Pimlico). AA wallets still require the user to individually sign each sub transaction (currently via multiple clicks) but don’t require holding of fee balances on each chain.
3. **Policy-based agents** hold the private key of the user in a separate execution environment and generate signed messages on their behalf based on user policies. Telegram bots, Near Account aggregator or SUAVE TEEs are policy-based wallets while Entropy or Capsule are policy-based wallet extensions. The user just needs to sign a single approval and subsequent signing of sub-transactions and fee management can be performed in-flight by these agents.

Solver Layer
------------

Once a user posts their intent the solving layer involves returning a **fee** and **confirmation time** to the user. This problem is closely related to designing an Order Flow Auction and has been written in detail [here](/the-orderflow-auction-design-space). A CAF can either leverage in-protocol paths to execute a users intent or leverage sophisticated third-parties aka solvers to provide improved UX to the user by compromising on some security guarantees. The next two design decisions arise when we bring solvers into a CAF framework, and are related to information.

![The two types of EV (EV_signal and EV_ordering) and their significance.](https://images.spr.so/cdn-cgi/imagedelivery/j42No7y-dcokJuNgXeA0ig/ffc44712-4bf4-4636-8d7a-e250bb5ecbc1/ev_types/w=1920,quality=90,fit=scale-down)

The two types of EV (EV\_signal and EV\_ordering) and their significance.

An intent consists of two types of extractable values (EV): **EV\_ordering** and **EV\_signal**. EV\_ordering is a value specific to the blockchain, typically extracted by entities that execute user orders like block-builders or validators. On the other hand, EV\_signal represents the value accessible to any entity that observes the order before it is officially recorded on the blockchain.

Different user intents have varying distributions between EV\_ordering and EV\_signal. For example, an intent to swap coins on a DEX usually has high EV\_ordering but low EV\_signal. Conversely, an incoming hack transaction will have a higher component of EV\_signal since front-running it will return significantly more value than executing it. It's important to note that EV\_signal can sometimes be negative, such as in the case of trades from Market Makers, where entities executing these orders can experience losses due to a market makers better understanding of future market conditions.

When someone has the ability to observe a user's intent ahead of time, they can engage in front-running, which leads to value leakage. Additionally, the potential for EV\_signal to be negative creates a competitive environment among solvers, causing them to submit lower bids and resulting in further value leakage (aka adverse selection). Ultimately, leakage impacts the user by either increasing fees or providing less favourable prices. Note, low fees or price improvement are two sides of the same coin and will be used interchangeably during the remainder of the article.

### Information Sharing

There are 3 methods to sharing information with solvers:

1. **Public mempool**: The user's intent is broadcasted publicly, either into a public mempool or a DA layer. The first solver who can fulfill the request executes the order and becomes the winner. This system is highly extractive, as users reveal both the EV\_ordering and EV\_signal from their order. Examples of this type of auction include Ethereum's public mempool and various blockchain bridges. In the case of bridges, users must place their assets in escrow before transferring them to the target chain as a precaution against grief attacks. However, this process unintentionally exposes their intents publically.
2. **Partial sharing**: A CAF may choose to limit the amount of value it reveals to bidders by limiting the information disclosed. However, this approach results in a direct loss of price optimality and can lead to other issues, such as bid spamming.

In an auction with partial information on incoming orders, a solver may try to estimate the missing information by either: 1) making a bid that depends on obscured data, or 2) spamming bids. Both of these approaches raise concerns about latency races, which can lead to centralization and scalability problems due to excessive simulation resource consumption.

3. **Private mempool**: Recent developments in MPC and TEEs open up the possibility to achieve completely private mempools. No information is leaked outside the Execution Environment so solvers encode their preferences, which are matched with every intent. Although private mempool captures EV\_ordering, it cannot fully capture the value in EV\_signal. Imagine what will happen if a hack transaction is sent to the mempool. The first person to see this order can front-run the potential sale and capture EV\_signal. In a private mempool the information is released only after a block is confirmed, and hence whoever can see the transaction can capture the EV\_signal. One can imagine solvers spinning up attestation nodes to catch EV\_signal from fresh blocks minted by a TEE, turning EV\_signal capture into a latency race.

### Solver List

The CAF also needs to decide how many and which bidders are allowed to participate in the auction. Broadly, the options are the following:

* **Open access**: Barriers to entry for the ability to participate are as low as possible. This is similar to a public mempool and leaks both EV\_signal and EV\_ordering.
* **Gated access**: There is some gatekeeping on the ability to execute an order, either through a whitelist, a reputation system, a fee, or a seat auction. The gatekeeping mechanism needs to ensure that solvers in the system don’t capture EV\_signal. Examples are 1inch Auction, Cowswap Auctions and Uniswap X auctions. The competition to win orders captures EV\_ordering for the user whereas the gating mechanism can capture the EV\_signal for the order generator (Wallet, dApps).
* **Exclusive access**: Exclusive access is a special case of the seated solver auction where only one solver is selected each time period. Since no information is leaked to other solvers there is no adverse selection and front-running discount. The orderflow originator captures the expected value of EV\_signal and EV\_ordering, since there is no competition the user can only get execution and no price-improvement. Some examples of these auctions are the Robinhood and DFlow auctions.

Settlement Layer
----------------

Once a wallet signs a set of transactions, they need to be executed on the blockchain. Cross-chain transactions convert the settlement process from atomic to asynchronous. While the initial transactions are being executed and confirmed, the state on the target chain can change, potentially leading to transaction failure. This subsection will study the trade-offs between the cost of security, confirmation time, and execution guarantee.

It is important to note that executing the intended transaction on the target chain depends on the transaction inclusion mechanics of the target chain. Including the ability to censor a transaction and the fee mechanism of the target chain, among other factors. We believe that the choice of the target chain is a decision for the dApp and will consider it beyond the scope of this article.

### Cross-Chain Oracle

Two blockchains with distinct states and consensus mechanisms require an intermediary, such as an Oracle, to facilitate the transfer of information between them. Oracles serve as relays for information between chains. This includes verifying situations such as a user locking funds in an escrow account for a lock and mint bridge, or confirming a user's token balance on the origin chain for participation in governance voting on the target chain.

Oracles transfer information among chains at the speed of the slowest chain. This is necessary to manage reorg-risk, as the Oracle needs to wait for consensus on the origin chain. Let's consider a scenario where a user wants to bridge USDC from the origin chain to the target chain. To do this, the user locks their funds in an escrow. However, if the Oracle doesn't wait for enough confirmations and proceeds to mint tokens for the user on the target chain, a problem can occur. In the event of a reorg, if the user overwrites their escrow transaction, the Oracle would have double spend.

There are two types of oracles:

1. **Out-of-Protocol Oracle** requires third-party validators separate from the ones running consensus to transfer information between chains. The need of extra validators increases the cost of running the Oracle. LayerZero, Wormhole, ChainLink and Axelar network are examples of Out-of-Protocol Oracles.
2. An **In-Protocol Oracle** is deeply integrated into the consensus algorithm of an ecosystem, and uses the validator set running the consensus to transfer information. Cosmos has IBC for chains running the Cosmos SDK, Polygon ecosystem is working on AggLayer, while Optimism is working on the Superchain. Each oracle uses dedicated blockspace to transfer information among chains of the same ecosystem.
3. **Shared Sequencers** are out-of-protocol entities which have transaction ordering rights in-protocol, i.e. they can provide bundling of transactions across chains. Although still in development shared sequencers don’t have to wait for certain block confirmations to reduce reorg risk. To truly provide cross-chain atomicity shared sequencers need to be able to execute subsequent transactions conditioned on success of earlier transactions turning them into a chain of chains.

### Bridging Tokens

In a multi-chain world, user token and fee balances are spread across all the networks. Before every cross-chain operation the user needs to bridge funds from the origin chain to the target chain. Currently there are [34 active](https://l2beat.com/bridges/summary) bridges with a combined TVL of $7.7B and bridging [volume of $8.6B](https://defillama.com/bridges) in the last 30 days.

Bridging tokens is a case of value transfer. This creates an opportunity to utilize specialized third parties who excel in capital management and are willing to assume reorg risk, reducing the cost and time required for user transactions.

There are 2 types of bridges:

1. **Lock and Mint bridge:** A lock and mint bridge verifies token deposits on the origin chain and mints tokens on the target chain. While small capital is needed to start such a bridge, significant investment is necessary for secure transfer of locking information between chains. [Security breaches](https://rekt.news/) in these bridges have resulted in loss of billions of dollars for token holders.

Note, Lock and mint bridges create new tokens on the target chain, which may be wrapped versions of the required tokens. For example, USDC.e instead of USDC or axlUSD instead of USDC.

2. **Liquidity bridges:** Liquidity bridges utilize liquidity pools on origin and target chains, along with an algorithm to determine conversion rates between the origin and target tokens. While these bridges have higher initial costs, they require lower security guarantees. In the event of a security breach, only the funds in the liquidity pools are at risk.

Over time, liquidity bridges have become more capital efficient. Solver networks (Uniswap X) are more efficient than lending protocols (Across), which in turn are more efficient than AMM bridges (Hop protocol).

In both types of bridges there is a liquidity cost which needs to be paid by the user. In Lock and Mint bridges the liquidity cost is while swapping from the wrapped token to the desired token (USDC.e to USDC) on the target chain, whereas in Liquidity Bridges the liquidity cost is while swapping from the token on the origin chain to the token on the target chain.

Cross-Chain Trilemma
--------------------

The above 5 design decisions give rise to the cross-chain trilemma. A CAF has to choose 2 properties between Execution Guarantee, Low Fees and Execution Speed.

![Cross-chain trilemma. A cross-chain infrastructure can only have 2 of the above 3 properties.](https://images.spr.so/cdn-cgi/imagedelivery/j42No7y-dcokJuNgXeA0ig/d5d263c1-b329-4de4-b4e7-3c5df770cf34/cross-chain-trilemma/w=1920,quality=90,fit=scale-down)

Cross-chain trilemma. A cross-chain infrastructure can only have 2 of the above 3 properties.

1. **In-protocol paths** are designated paths for transferring information across chains. These systems account for reorg risk sacrificing execution speed but reduce costs by eliminating the need of an additional validator set or liquidity costs.
2. **Solver aggregation** collects quotes from multiple solvers to identify the cheapest and fastest path for fulfilling a user's intent. However, due to adverse selection and front-running, solvers may sometimes fail to satisfy the intent, resulting in reduced execution.
3. **Execution competition** selects a winning solver by either arranging a race between solvers to execute an intent or choosing a single solver exclusively. Both approaches lead to high fees for the user as solvers compete for execution rather than price improvement.

The Six Pieces Of CAKE
======================

To write this article we studied more than 20 different designs from teams both explicitly and implicitly working on Chain Abstraction. In this section we discuss six independent CA implementations which we believe have inherent efficiencies and product market fit. These designs have the potential to compose with each other if built right.

One key takeaway from this exercise is that we need a common standard for expressing cross-chain intents. Each of the teams are working on their own methods and protocols for encoding user intents. Unifying towards a standard will improve user understanding of the message they are signing, make it easier for solvers and oracles to understand these intents and simplify the integration with wallets.

|  |  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- |
|  | **Token Anointed Bridges** | **Ecosystem aligned bridge** | **Solver price competition** | **Wallet controlled messaging** | **Solver speed competition** | **Exclusive batch auctions** |
| **Purpose** | Cheap Cross-chain transfers | Cross-chain message call | Cheap Cross-chain swaps | Cross-chain message call | Fast Cross-chain transfers | Cross-chain message call |
| **Examples** | CCTP, CCIP, xERC20 | AggLayer, Superchain, IBC | Bungee, Jumper, Uniswap X | Alfred, Avocado, Near Account | Across, Orbiter | NA |
| **Wallet** | Any | Any | Depends on implementation | AA or Policy-based | Any | Any |
| **Information shared** | Public | Public | Depends on implementation | Depends on implementation | All or None | None |
| **Solver list** | Depends on implementation | Depends on implementation | Gated access | Depends on implementation | Depends on implementation | Exclusive |
| **Oracle** | In-protocol | In-protocol | Out-of-protocol | Out-of-protocol | Out-of-protocol | Out-of-protocol |
| **Token Bridging** | Burn and mint | Lock and mint | Depends on solver | Depends on solver | Liquidity bridge | Depends on implementation |

### Token Anointed Bridges

There is a special case of lock and mint bridge which does not pay the liquidity cost also called a burn and mint bridge (eg. USDC CCTP). The token team anoints a canonical token address on each chain while the bridge has the authority to mint the token i.e. the token which the user needs.

![Implementation of an ERC20 transfer function. The tokens are burnt from the senders balance and are minted into the recipients balance.](https://images.spr.so/cdn-cgi/imagedelivery/j42No7y-dcokJuNgXeA0ig/bae9eccd-9912-43ef-9b44-40658c2ee03e/Screenshot_2024-02-07_at_09.28.08/w=1920,quality=90,fit=scale-down)

Implementation of an ERC20 transfer function. The tokens are burnt from the senders balance and are minted into the recipients balance.

If you squint hard enough, a burn and mint bridge is similar to a cross-chain transfer at the speed of enough block-confirmations. [xERC20](https://www.xerc20.com/) is one such standard to anoint canonical tokens and their authorized bridges on target chains. A token anointed bridge is an example of an in-protocol path i.e. it compromises on speed for execution guarantee and low fees, e.g. CCTP takes 20 minutes to execute a transfer.

### Ecosystem Aligned Bridge

An **ecosystem-aligned bridge** enables the transfer of arbitrary messages between chains within the same ecosystem. It falls under the category of in-protocol paths, prioritizing execution guarantee and low fees over speed. Examples include Cosmos IBC, Polygon AggLayer, and Optimism Superchain.

Three years ago, the Cosmos ecosystem faced similar challenges to what the Ethereum is facing today. Liquidity was fragmented across chains, each chain had its own fee token, and managing multi-chain accounts was cumbersome. The Cosmos ecosystem addressed these issues by implementing in-protocol message passing bridges through IBC, resulting in seamless multi-chain accounts and cross-chain transfers.

The cosmos ecosystem comprises of independent chains having sovereign security and fast finality, making the in-protocol path for cross-chain messaging very fast. On the flip-side the rollup ecosystem depends on expiry of the challenge period (Optimistic Rollups) or commiting zk-proofs (Validity Rollups) for finality. In-protocol paths for message passing across ecosystems will be slow due to these finality constraints.

### Solver Price Competition

A **Solver price competition** involves sharing order information with all solvers. Solvers aim to incorporate the expected value (EV) generated by the intention of the order and provide it to users. The selection of the winning solver in the system is based on maximizing user price improvement. However, this design carries the risk of non-execution and requires additional mechanisms to ensure the reliable inclusion of orders. Examples of such mechanisms include Uniswap X, Bungee, and Jumper.

### Wallet Coordinated Messages

**Wallet coordinated messaging** utilize capabilities provided by AA or policy-based wallets to offer a cross-chain experience that is compatible with any intent type. It serves as the ultimate CA aggregator, redirecting user intents among various CA designs to address specific intents. Examples include Avocado wallet, Near Account Aggregator, and Metamask Portfolio.

Note, over the last decade, the crypto-ecosystem has learnt that the relationship between a user and their wallet is very sticky. I personally feel a mortal dread whenever I think about migrating my mnemonic from Metamask to another wallet. This is also the reason why even after 2.5 years and backing from Vitalik Buterin himself EIP-4337 has gained [minimal adoption](https://dune.com/niftytable/account-abstraction). Although newer versions of wallet protocols might provide the user with better price (account abstraction) or improved ease of use (policy-based wallets), migrating the user from their current wallets is an uphill task.

### Solver Speed Competition

The Solver speed competition allows users to express their intentions for specific cross-chain transitions for high execution guarantees. It does not assist users with minimizing fees, but instead offers a reliable channel for including complex transactions. The first solver to execute the intent based on block-builder fees or inclusion speed wins the intent.

The design aims to achieve a high inclusion rate by maximizing the EV captured by solvers. However, it comes at the cost of centralization, as it relies on sophisticated capital management on the Ethereum mainnet or low-latency execution on L2s.

### **Exclusive Batch Auctions**

An **exclusive batch auction** holds an auction for the exclusive rights to execute all the order flow in a time window to a single solver. Since other solvers cannot see the orders, they place the bid based on the predicted market volatility and their average execution quality. Exclusive batch auctions depend on a **backstop price** in order to assure good user prices and therefore can’t be used for price improvement. Sending all the order flow to a single bidder eliminates information leakage and improves execution guarantees.

Conclusion
==========

Chain Abstraction Frameworks (CAFs) promise to give users seamless cross-chain interaction. In this article we studied designs in-production and in-development by several teams who are explicitly or implicitly trying to solve for Chain Abstraction. We believe this will be the year of CAFs and expect significant competition happening between different designs and their implementations in the next 6-12 months.

|  |  |  |
| --- | --- | --- |
|  | **Value Transfer** | **Information Transfer** |
| **In-protocol paths** | Token-anointed bridge | Ecosystem aligned bridge |
| **Solver aggregation** | Solver price competition | Wallet coordinated messaging |
| **Execution competition** | Solver speed competition | Exclusive batch auctions |

Cross-chain value transfers will be routed through a combination of token-anointed bridges for low fees and Solver Speed or Price Competitions for speed and execution. While information transfers will be routed through a combination of ecosystem aligned message bridges which will aim to minimize the cost to users, and to wallet controlled platforms who will maximize speed. Final implementations will cluster around these six distinct designs as they each serve independent needs and benefit from efficiencies existing in different corners of the tradeoff matrix.

One key takeaway from this exercise is that we need a common standard for expressing cross-chain intents. Several teams are working on their individual protocols for encoding user intents causing duplicated work. Unifying towards a standard will improve user understanding of the message they are signing, make it easier for solvers and oracles to work with intents and simplify the integration with wallets.

🙏🏼

Thanks to [Nathan Worsley](https://twitter.com/NathanWorsley_), [Vaibhav Chellani](https://twitter.com/vaibhavchellani), [Hart Lambur](https://twitter.com/hal2001), [Illia Polosukhin](https://twitter.com/ilblackdragon), [Zaki Manian](https://twitter.com/zmanian), [Drew Van der Werff](https://twitter.com/DrewVdW), [Sam Hart](https://twitter.com/hxrts), [Christopher Goes](https://twitter.com/cwgoes), [Arjun Chand](https://twitter.com/arjunnchand), [Anatolii Padenko](https://twitter.com/0xAnatolii) and several others for the feedback, pushback, comments, and review.