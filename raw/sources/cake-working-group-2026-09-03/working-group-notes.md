# 🍰 CAKE working group  

- Standards -hopefully we can agree on cross-chain message passing. 

:::info
💡 Notes from the working group breakout discussion after lunch. 
:::

## IBC and Light client discussion 

- The things we've seen - hard to implement, light clients have not caught on, ethereum light client is specifically  complicated
- ZK proof for the light client - I as someone who is pairing don't need to understand the light client, only the proof - decouple particulars of getting assurances - implementation and integration separation 
- Probably get better security guarantees, internet was a bunch of network protocols that you can move across transparently
- Can you steel man the case not to use IBC (from the POV of the Ethereum)? - you have to trust your source chain, what happens in cases like Terra where suddenly the market collapses (death spiral) and you can take over the chain. 
- In the endgame, everything probably becomes a ZK rollup of everything else, the overhead of having to integrate with other chains won't work 
- For all these consensus detail reasons, you would hope to maybe do better. The validator set is small - you maybe want more validators, so your light client becomes hard to verify, you can accept a ZKP of consensus, agree of the state of chain. Getting a ZKP of the light client. 
	- Getting the light client is applicable across all cosmos chains - ethereum light client / tezos light client, you verify a proof of consensus
- In the ZK light client you can include the proof  "this state was arrived at, this state was arbitrarily the previous state". 
	- Is there a difference in verification cost?  There should not be a difference (proving cost is expensive)
- Doing it across EVM & SVM 
	- independent of the relaying - this chain, on-chain, verifies the light client on-chain, without ZK, a Solana verifier in the EVM would be infeasible. Fixed cost proofs are good. 
		- You can have a consensus proof but not yet state proofs (e.g., zkEVM)
- All of these different chains have different consensus models with finality
	- the general approach in IBC - you have a client, my chain understands your chain with connections and channels. The client has a finality condition. 
	- At this point you can have a threshold without rollbacks across it
	- The sum of the latencies for cross-chain interop
		- you want a piece of standard to be parameterized over finalized, make it legible to other chains. 
		
## Yet another trilemma

1. Throughput
2. Interop 
3. Stateful confirmations
		
- If you are a solver (which are required in cross-chain models) there are many degrees of trust assumptions that solvers have - Latency issues around the verifications and communications in general
	- Optimistically allow things through and slash afterwords is not full interop but still works
	- If you prioritize inclusion guarantees rather than execution guarantees then you lose some stateful pre-confirmations
		- you can go very slow and sacrifice throughput but you are not going to get all three properties together
- Perhaps we are simply rephrasing CAP theorem
- A big part of choosing a standard around this is where in the trilemma we want to focus on is  if you can do something like do an option on an asset - thats the most powerful way we can price the risk, if I have an option I can price the risk and consolidate it
	- I question re-org risk - along the spectrum of this type of risk - relays can lie on where you were included over index n - your swap went through but in reality it didn't. 
	- IBC relays can't lie - this is just some type of solve
- You can make options but options are expensive. It may be cheaper to keep the risk and run high speed. 
	- Low card crusader - you look at these types of attacks in a cross-chain setting which are very doable. How does mev-boost prevent this is latency ?  time fundamentally hinders ability to scale
- Other thing you see alot architecturally in systems is - as long as I'm sending money around you can have a balance of money transfer. 
- One of the models seen from other teams is throughput measured in inventory for validation rather than TPS, whoever is validating something can only validate a certain amount of inventory - those models are tricky though - what is the canonical token per chain? You still run into  throughput bottlenecks but it works as long as you price transactions
	- one way you get back to a priced transaction is you have no idea that this NFT is $1000 of value transfer, then we run into interesting oracle attacks on bridges and solvers
	- Is it fair to set a minimum threshold for cross-chain communication value transfer that won't scale well? 
	- There are a lot of different directions in which the tradeoffs can be made, which of the trade-offs is worth tolerating? We have a lot of room to pick this kind of thing. 
- Argument against the models which requires solvers to handle bridging risk is that you will gate-keep about 95% of solvers - It's already a PIA (pain in the ass) to do infra on one Chain - you are looking at maybe 3-4 teams right now. 
	- Is there room in the market for bridging and solving? I think bridging should be automated after the fact, the solver should not do the bridging

### Balance Abstraction

- Balance abstraction - we don't pay MEV out to the validators at any particular point in time - validators themselves will optimistically loan users fund, then settle after the fact once user gets confirmation 
	- T + 2 settlement - the goal is the settlement delay happens afterwords rather than before 
	- There are different ways to move things around - delay on reusing funds is the easiest to do - balance abstraction uses MEV as liquidity
	- How does this work if solver is not doing bridging ? 
		- solvers are not cross chain - one solver per chain , validator stands in as a proxy for the user
		- Fore example, Same validator on both sides, polygon validators have one foot on each chain 
	- This setup also allows validators to be proxies for market makers; validators bundle operations so solvers dont take on bridge risk - validators do, but they have collateral on both chains bridge risk == finality risk ( think polygon validator on both chains)

### Staked ETH
- Economic consequences all eth is staked?  ETH is no longer money its just locked
	- disagree but thats fine
	- More validators == more overhead - finality becomes harder, higher node requirements

## Light Clients - Back to IBC and Ethereum discussion

- IBC - still an honest majority assumption, for example If you only have ZK proof of consensus you can have rogue validators implement an inflation bug
- With proof on consensus you have honest majority, you can check balances
	- There is some state commitment but in terms of cross-chain, the light client gets you the state root hash and you have proofs against that hash, how much am i trusting the people who produced this hash?
		- you want to validate the proofs on chain
		- execution proofs more assurance - SANRKs
- If you have a  proof of consensus it  enables some optimistic models 
	- Cosmos you only get a new block when everyone agrees
	- ZK for proving consensus and state root - make it so operators are running multiple chains - can verify themselves by trusting themselves
		- The receiving chain should not trust the operator
		- No additional trust assumption if the validator sets are the same
		- Similar insight when you look at synchronous composability b/w rollups
	- IBC is based on consensus but maybe you can add execution proofs in principle
		- Verifying light client is much smaller, its all light clients, and it won't change until ZK magic to make it performant
- Ethereum light client 
	- SNARK the light client and make it cheaper
	- Can make sync committees bigger, can even imagine entire set that is doing this
	- Endgame is you have a SNARK for the entire protocol

## Evolving CAKE

- CAKE - there are boxes in places not expected (diagram), how do we evolve it - someone says I dont like this cake want to make better -go back update the blog
	- More than one type of CAKE and we should enjoy all of them 
	- Oracles in this settlement layer, just one of those things is weird. 
	- A related thing is many components can have mixed concern - consensus in solver layer (Cake inside of CAKE - recursive cake)
	- The intent is to create a common language - oracle has been abused and disabused multiple times
	- Expect this CAKE to evolve pieces to be ripped out and added - the goal is really to have a Schelling point for people to come together - having a place where people can get criticized is 90% of the value
	- Wallet flavor CAKE that dictates chain abstraction

- One model of CA is users seamlessly access services and assets regardless of the chain
	- In many scenarios there will not be an obvious wallet, why is there a wallet? - putting the wallet too high is baking in a theory with how the user interacts
	- Should identity be the top layer as opposed to applications? - application being the use case. Can have an entire CAKE with Identity at the top 
	- Coinbase launched a thing today we'll forward you money to use Defi which is effectively placing the wallet at the top of the CAKE
	- Users that have private keys which identify where they control their state on chain and then you have interfaces which help users operate on their state, the wallet is usually trying to join these two things together - separating these things is more informative for discussion
        - Do shit with your keys -  Key, custody and control
    - Coinbase super app is both the application layer that aggregates all of the apps and a key management tool which manages your identity, they try to put both of those things into one
	    - the user interface == app
	    - permission layer == key management
	    - These two levels can be exchanges
- Maybe we should push back on app layer being the front end - anyone can fork uniswap frontend and point at Uniswap contracts, the interface for the users / the users that is dictating what happens
     - would expect one element of intents - some amount of redistribution to the user for better fills
## 4337 
- With 4337 and the public mempool everything there is permissionless, anyone who can sandwich it the most will win the MEV auction - when you start looking at user operations in general whether thats EOA or smart account - if you make the operation available for everyone to see whoever can extract the most will win the intent 
	- Whether that comes true at the intent layer is the design of the protocol
	- Fastlane approach is just permissioning the bundler - you can have redundant ones 

### Trust, MEV, privacy Trilemma
- Always worth asking the question whether ZK breaks the trilemma
	- FHE is promising but it takes very long in terms of performance
	- SUAVE fixes this just put an SGX

 
## RAAS
- RAAS providers run optimism arbitrum and various other DAs and it is a PIA to keep track of different formats and standards - as soon as you start to deploy hundreds of networks becomes a mess without standards - how you post to DA, bridge assets standards, between rollups no handling of message passing, this is what we will explore with layer threes to make them interact, Layer threes you have to withdraw not to make a mess. 
- Storage proofs can be a way some cross chain protocol could be an easier way, scalable to sustain the base of people want to have. Time is invested to make this kind of abstraction - also because we apply changes to those maybe we improve sequencing for optimism, do something similar with Arbitrum. Abstract DA to plug into any DA. 
- Having these kind of discussion groups is helpful because it save a lot of nights. - where is the most pain in abstracting out the infra when maintaing 100s of Nodes? to users we have to explore common standards - posting on eigen layer or celestia is diferent - the API spec is there a problem in signing or broadcast? The pain layer is more for us to settle on vairous systems - finding a way where its easy to maintain
	- Effective modularity - can modularize on macro level and you still have this monolithic view - projecting modularity on different accesses

## Pre-confirmations
- People like pre-confirmations - many different ways to take it
	- basedCAKE - based vision of the world which are interesting this and CAKE go together nicely 
	- In the endgame you abstract away as much as can. 
-  Spicy take: pre-confirmations is the ethereum master node
	- outsource to L1 builder, then censorship issues
	- many open questions - this is what we get with L1, can you decentralize this in a meaningful way?
- If there is a way to abstract away the execution for L1 that is huge for application builders - unfortunate for centralization 

---

# 🎂🎂 V2 

- Workshop took Place on March 15, 2024

## tl;dr


- Defining CAKE (Cross-chain Intents)
    - Agreeing on definitions and a common language
    - Mutually exclusive set of definitions for the solver layer
- Defining Intents
    - User specifies the assets they want
    - Declarative message requesting a target state (generalized token as state)
    - Specifying the desired state transition across different execution environments
    - Narrowing the scope to cross-chain limit orders for bootstrapping solver costs
- Permission Layer
    - Different aspects of how users express what they want to do through an application
    - Expression Layer vs. Permission Layer
- Cross-chain Limit Order
    - User's intent to sell an asset on one chain and buy the most of another asset on a different chain
    - Expanding the definition to include call data execution
    - 95% of what people want to do is move money
- Account Abstraction and Intents
    - Using smart wallets on different chains and giving them instructions
    - Signing user ops for different chains or using a different approach
- Defining the Intent Message Format
    - User signs an off-chain message stating what they want to do
    - Descriptive vs. Prescriptive (Declarative message)
    - Specifying the settlement contract address as part of the order ticket
- Counterparty Risk and Settlement
    - User has counterparty risk to the settlement layer until they receive the desired asset on the destination chain
    - Solver has counterparty risk to the settlement risk as well
- Bootstrapping the Solver Network
    - Defining types of messages and chains solvers are willing to handle
    - Nested model of message passing allows for versioning and granularity
    - Proposing a draft standard that is generally positive sum and evolves over time
- Specialized Solvers
    - Solvers can specialize in specific types of intents (e.g., liquidations, sandwich attacks)
    - Standardized message format (e.g., Permit 2 type) reduces work for solvers to listen to various sources of order flow
- Settlement Layer
    - Competing Protocols as settlement layers for cross-chain intents with good trust assumptions and cost-effectiveness
    - Having a form of chain abstraction that looks like a CAKE (Cross-chain Intents)


## Raw notes

Defining CAKE - Agreeing on definitions first and foremost
- want to find a standard for cross-chain intents- solver layer maybe positive sum 
- Chatham rule - come up with a common languages - maybe we can come up with some formal language and this is the definition of this - mutually exclusive set of defining these things in this layer - define words (much of miscommunication is semantics)

- Permission Layer - Different aspects of this are (user says via application what they want to do )
	- 4 ways how the user can say what they want to do 
	- Expression Layer vs. permission layer - User intent/ orderflow - how far does the 
- We talk about this as cross-chain limit order - not as specific as transaction - 
	- cross-chain limit order - user daniel wants to sell assett a on chain x to buy the most of assett B on chain y - this is not a transaction because we have not specified how this work 
		- has a financial root and mostly specifies, 1000 usdc on optimism want to deposit on aave on base, intent solver could do this for me - arbitrary code - differnece would be a cross chain lending market - more generalized is much harder. Assets on chain A or some set of chains. 
		- UniswapX single chain vs. cross-chain - settlement layer is different escorws on origina chain, solver fills on destination chain after intent verified settled by Solver. 
		- Expand definition to include call data execution - we could make our cross-chain limit order slightly more expansive do limit order and execute code on destination chain. Do transfer and execute code. 
		- The second part is determined by solver not by you. 
- 95% of what people want to do is move money 
	- One to many or Many to many - near wallet controls all the wallets or other wallets control wallets across chains. 
		- Near you have to start off with NEAR - base account - create an optimism account based on this - starting from EVM instead of starting from NEAR and you would need account Abstraction
		- Go to NEAR - you use the near wallet and you can talk to all the blockchains and have to use the NEAR wallet to make this work. On base have MM setup PK setup to sign txs and now execute something on Optimism - could sign a user op - chain ID, 
			- So now to me this is where intents and account abstraction overlap - need to pay gas on the destination chain. Arbitrary code want to execute is user op. Intent is pay gas and user op. 
		- There are multiple ways where the Solver can jsut do everything they will take it from the user's account. The concept of these cross-chain paymasters are the interface for solvers. Might things slightly inefficient - can be done without paymasters as well. Without paymasters the user is paying for gas - 
	- Using account abstraction today - its all very similar ofc. In this paymaster thing its similar
	- At the permission Layer have AA figured, signing user ops for different chain or doing different approach and figure out how to get that executed on another chain - have a smart wallet on that chain and given it instructions on what can do. 
	- Intent - user signs some off-chain message that says what they want to do 
		- we define that looks like a cross-chain limit order 
			- can be interpreted as EVM tx or another chain - tx subject to bottlenecks of chain where txs are not 
		- Defining a message that is EVM compatible seems tractable - across ecosystem - seems hard 
		- Descriptive vs. Prescriptive - Declarative message (Intent is...)
			- Order ticket - I have an order have this here want this there - execute this arbitrary code. think of blocks fill out order ticket and sign it and done - this particular type of order ticket not maximally generalized but intuitively specified
			- Solver needs to know if you have the assetts and where you get them at the end 
				- 1000 usdc on home chain - you do care that it doesn't go on garbage scan chain 
				- Some set of chains - take settlement on 
			- Deeper question - how do you or user care about security on different chains? do they originate on single chain or come from multiple chains. This is token address on single chain. 
				- USDC on optimism vs. base have the same risk - if moving assetts to different chain applciation they want to do probably holds assetts on another chain 
				- Exchange wallet or payment walelt might trade usc and usdc.e on behalf of the user, the app is there making that opinion 
				- app wants programmability to specify - application layer should always be making trust decision 
- Across -user has assett on chain want to sell, solver is going to fill them with assett the user has risk to the settlement layer until they get assett B on the destination chain - there is some risk there - if the solver doesn't show up. There is a form of counterparty risk - the solver has counterparty risk to settlement risk as well. The general way this should be structured - the user counterparty risk - solver risk is days or weeks or hours and are more sophisticated and should price this. 
- Does this have an impact on the design of the message format - the user has to specify the order ticket - specify beyond just the routing - settlement contract address part of the order ticket
- ERC-20 standard - contract is settlement contract - when sending settlement contract believing ERC-20 is written in correct fashion - but when you are sending your message or escrowing funds you are trusting that settlement contract. 


- **User specifies the assets they want**
	- declarative message request some target state - generalize token as just state. I want this state to happen on this target execution environment willing to give this permission can be on any origin chain and specifying the settlement gadget that is doping the execution - 
		- look at this as escrowing then validating state on destination chain happened
		- settlement layer here is the one doing all the messaging- orcale function or verification of an intent - solvers take more counterparty risk - they are the ones at risk of not getting paid back 
	- Permission is kind of like the state offer. 
	- The tough part about the standard is the format of these components - it almost doens't matter in a way? It does but you want people to agree to - here is my desired state transition generalized way of saying i want these assets and execute this code - less sure how you specify this or grock this in an easy way - is  it a state root? 
	- End state yo have these tokens and message you attempted to execute - grockable of what its doing, like the generality of this concept 
	- State transition request around different chain - state and different environments - I want an IF condition a certain amount of state on any one of these execution environment - a state transition request is the most generalized request of doing something on a blcokchain. There is merit in contracting the scope to limit order. 
	- Cross-chain limit Order - there is a need today where everyone wants to define a need or a use case - push in the working group is think about ti from the solvers perspective - what is easy way to bootstrap solver costs 
	- Bootstrapping solver network is a necessary evil not necessarily something people want to do - every settlement mechanisms is going to have trust assumptions which will be where solver spends the most time what is risk reward for doing this integration
	- I want across to be a settlement layer for cross-chain intents - all this stuff has good trust assumptions and its cheap - needs to be a standard at the settlement layer of what does this look like, can and should and will have competition at the settlement layer 
	- I as a solver assume there are different type of intents - define types of messages you are willing to solve and which chains you are willing to solve them on 
	- The beauty of this nested model of message passing - as we get more granularity of specifiying a state transition this could be a versioned field. 
		- We did booststrap a solver network for cross chain bridging - reasonably decent now have flow and volume going to that and its more robust and resilient. More for solvers to do more positive sum
		- We are going to propose something very soon which is a draft - idea is generally positive sum this works and solves problem today and generally going to evolve. Not disjoint solver networks or fight with them. 
	- Certain solver doesn't have to be able to do everything - as a solver you can say im not just solving. Many solvers impolicitly do this today, some solvers specialize in sandwhiching, cross-chain stuff. There osi a standard for a liquidation on Aaave there is a standard for what a sandwhich attack lloks like and other solver sspecializing for that, supersede that with intent standard. More competition which leads to a standardized way for a message format probably a permit 2 type of message - uniswap OFA spits these out, across' front end spits these out, because its in the standard format its zerio work for solver to lsiten to sources of order flow. Settlemtn layer is an interesting problem, we have somehting that works well as a settlmeent layer, Have a form of chain abstraction that looks liek a CAKE. 

## Notes from Presentations 

## CAKE 


Why Chain abstraction
- if there were no standards in web2 world sending pdf file from one person to another you would have to search for an aggregator and then pay for delivery - fast and slow timings depending on what you are willing to pay
- Clean drag drop and send interface for users
- Thankfully we have http, smtp, OSI, TCP/IP
- Powerful economic incentives  = activity on many chains, but it is not scalable if we want to bring next set of applications and users 
- Chain abstraction - user connects wallet to dapp, user click what they want to do and it just happens 

Where to put standards?
- Common Language - what are we trying to do? 
- Settlement and solver layers there is a large overlap in the group
- How do these different layers talk to each other and interface (application<>permission layer - identity, permission<>solver layer -value transfer, solver layer<>settlement layer - information transfer)
	- how does value accrue to the settlement layer? 

Trade-off space
- users generalize trust the interface they use so up to the interface to figure out what they trust
- Some minimum set of standards 

--- 

## Ethereum Sequencing

Two bottom layers of the cake - settlement and solving, dramatically reduce complexity with shared settlement and shared solving - standard just used Ethereum as shared settlement and shared solver. 
- complexity blows up with multiple settlement layers you are going to abstract over

Lets say we have two settlement layers - basic thing we want is a bridge. Any L1 can suffer a 51% attack or update through governance or have micro re-orgs (Social governance can always change the rules of L1)
- need a governance token to do maintenance to update the light clients on both sides. Now we have three social communities. Now lets assume we have k bridges, k+2 social communities, K+2 tokens
	- Fragmentation of economic security 
		- amount of economic resource that consensus participants put forward to secure the blockchain, total amount dollars 31M eth * price of eth - important because the amount of economic security highly correlated with dollar cost amount to attack the chain. Censorship and Liveness attacks.
	- Base case is we see one settlement layer gobble up everything, ethereum is the begins to gobble everything up
- Solana on Ethereum - SOL is governance token - they want to do emergency maintenance forks

The other big problem is around shared solving. What are you solving over. You have multiple providers of blockspace, if you have on producer or supplier of blockspace - it becomes easier to get get composability
- Multiple proposers now - arbitrum, optimism - lets simplify and have one single counter party
- One of the big advantageous is you can get synchronous composition - in a world where you get super transactions made up of smaller transactions 
- Endgame you have one master shared sequencer - can craft super transactions that touch multiple execution zones for the whole bundle. 

Shower thought: how do you deal with reversion? In asynchronous world with k domains - as soon as one of k domains messes up you have to revert k minus one. Reversion is expensive process, need to do DA and SNARK proving even tx reverts. One provider - ultra cheap simulation, try tx and revert within one data center. 
- Cost - prove computation you have 1000x overhead - CPU consumes 1 jul, prover will consume 1000 jules, similar thing with DA 
- Consume 1 byte of info doing it on my server vs. All to All is much cheaper 
	- if many txs will be reverting better off with this simulation stage which filters out the reversions. You can think of block building as being this highly sophisticated data and computation compression service
- Claim - from efficiency standpoint better to have builders do pruning on one node rather than gossiping to the whole world. 
	- once you have synchrony can have shared liquidity and complex applications
	- each application you interact with today is a tiny lego - expect as we have more money legos want to start building super structures, the brittleness of synchrony will compound as super structure grows. 

- Thesis- which shared sequencer should we use? 
	- Ethereum - no new security assumption, most credibly neutral as well, (e.g. ENS wants to L2 and don't want to socially piss off rollups), need to have the root of trust somewhere. 
		- ENS said you know what should probably launch as a based rollup 
		- Aztec recently strated considering based sequencing wave that has some amount of momentum 
		- In order to get rollups to adopt this need to fix major problem which is pre-confirmations. 

### Decentralized pre-confs in general
- user communicates with sequencer at current slot here is my tx please confirm it, and it is a promise given by the sequencer, similar to centralized sequencer
- what we want is trustless stake collateral- can just use financial collateral with slashing, different than centralized reputation like we see today with optimism, arbitrum, etc
- Slashing
	- Safety faults - made a preconf for something and sequenced something als
	- Liveness- made a preconf and then happened to be off line
- From POV of PBS and mev-boost need to do a slight change - current flow is uni-directional builder->relay->proposer, need a back channel flow where user establish constraints on builders, bi-directional flow on information, relays can help enforce this

Centralized - fixed sequencer at every slot, decentralized - rotating
Slashing - reputational, financial
MEV policy-based, market-based

What happens if the pre-conf is not correct? 
- entity giving the preconf is the sequencer who has monoply power, preconfer is taking low risk only liveness, b/w few seconds they create pre-conf and the block

Liveness / Safety, could also happen there is a re-org which is not your fault? 
- pre-conf gadget is two things - a pair of a finality gadget with consensus on who the next sequencers will be
- pre-confs always conditional on some sort of finalized stuff, only give preconf on things that are under my control 
- As a user, don't know what the tip of the chain is? Pre-conf in both cases, can even buy insurance against this rare event. 

### Based Sequencing
- In order to provide preconfs need collateral, need L1 proposers to opt into preconfs with collateral. 32 slots in beacon chain look ahead, some subset will opt in with collateral, only the subset that opts in are the sequencers. The decentralzied sequencer mechanism is made out of  asubset of L1 prposers deemed to be the sequencer. 
	- Variable weird block times which is fine - give some work to the non-preconfirmers (includers)
	- once user agress to preconf with next sequencer this preconf can be shared publicly and settled immediately by the includer. Take promises then settle them immediately in the next slot. 
- Is there potential for girefing? In slot n+ 30 an includer includes something that invalidates a promise.
	- the shared sequencer is a subset of L1 proposer, rollups that use shared sequencer will not allow entity before to change their state. Includers can act as an inclusion list. Forced include transactions sequencer by slot n+ 1
- Couldn't some of the transactions be ordered by inclduers on the base chain? Couldn't that also be revived to touch state on L1? 
	- What this sequencer gives you is synchronous composability b/w L2s, for L1 to L2 is a bit more subtle only guaranteed synchronous composability in the green slots (preconf slots) - excpect if this catches on all slots will be green 
	- Presumably everyone rational will be running MEV-boost++ 
- The shared sequencer posts a blob, are includers posting blobs, taking same messages, calculating root and posting that, but they are not defining the sequence. Watching sequencer output taking that and putting on chain? They also create an inclusion list. 
- If you compare to PBS terminology - the sequencer is like the proposer, includer is dumb node. Includer is what is supposed to be the beacon includer of execution tickets. 
	- It does feel like there are griefing vecotrs here also from a practical perspective doubt of includers should post them or sell them and howndo you compensate includes. 
		- Includers still get tx fee - open up the design space. 
- Includers bc cheap than sequencer? Want faster settlement, allow the non-sequencing L1 proposers to do settlement.
- What are the L1 gas costs vs optimisim - one benefit is you can have optimal gas costs. Two reasons, you cna now do data compression over multiple rollups, The more data you have the better the compression. If you are compressing optimism and arbitrum simultaneously. Blobs are chunky 125KB, If you are a rollup you may consume 1.5 blobs, what shared sequencer can do is take data for all rollups compress it and optimally pack it. 
	- Optimism today every 4 minutues is pushing all data into blobs and doing the sequencing and proving the ordering on chian? in this setup you do this every block? - there will be some amount of overhead, it can still happen every two or three slots - whenever its economically meanignful to settle 
	- If optimism never has enough data this has an advantage where you can push blobs every slot - this is very friendly for the long tail dont need to wait to consume a whole blob - can have settlement every slot. 
	- Eventually there will be full usage of cpaacity of blobs - consume as much as fast as produced - most optimal way forward, waiting 100 slots won't make sense because you are consuming all of the data all of the time any wastage.
	- How does it impact customization of sequencing fees? There is a bit of a design space even if you are not a shared sequencer to restrict it in some way, I only want my blocks to settle in even blocks or odd blocks. You can try and have your own Policy
	- Arbitrum thinks you can have decentralized sequencing and policy based ordering. 
Is there an incentive for rollups to join a shared sequencer
- execution fees (base fees at layer one and two MEV)
- On L1 ration is 80 20 - 800 eth of MEV and 20 execution fees
	- expect MEV to go down by an order of magnitude - things like sandwhichs will go away, and encryptred mempools, expect cex-dex arbitrage - thanks to next generation dexes - arb opportunity is auctioned off and rebated bacl to LPs, MEV always stays at user level
	- Expect the ratio of inclome will be 99% execution fees and 1% 
		- join shared sequencer increase execution fees - say someone wantas to buy a large amount of token optimal way to do this is route through different execution environment. 
		- If you have asynchrony you cna't do this - 1inch, matcha, you have 1inch copies not 1 inch whcih works for everything if I want to buy amount of token foing to pauy execution fees there
		- From perspective of rollup beneficial to join serquencing zone
		- By default if i join shared sequencer MEV goes to shared sequencer - possible loss of MEV is what you gain from execution fees. - Try to identify where MEV came from and rebate back to sequencer. Beleive they have a mechanism if I generate 100 ETH per day I get at least 100 ETH. If you agree there is negligable loss of MEV or no MEV - agree that joining a sequencer increases fees. 
	- What is best served by shared sequencer vs. Solver? 
		- want shared sequencer to be maximally simply and robust - something like a relay 
		- Need big fat type and high uptime
		- Want to see all of the algorithm capital order flow to be segregated away from shared sequencer. 
		- One way to accelerate this is encrypted mempools - from POV of the sequencer just receiving white noise and the most natural thing to do is setlte FCFS, as soon as you have more info have ability to be sophisticated. 
- Want as at a minimum is an encrypted mempool - private mempool, hoping we don't need to use trust, if we use trust lifting ourseves to trusted operators, how do we gap trust SGX, Threshold, delay encryption - pure cryptography 
- Could you talk about similator about pricing preconfs and providing gaurantees of execution inside of an OFA?
	- there is a similarity. Lets assumethat I'm the sequencer for a slot a fairly long slot like 12s. Here I receive a preconf and want to respond within 100ms, you can be tempted to think weve jsut created 100 ms blocktimes. We have reduced the latency - but from an MEV standpoint the slot time is 12s. As the sequencer I have the incentive to just sit on this transaction wait until the very end and make an informed decision. The mev slot time doesn't change still ong, in order to price tips properly we need to be able to estimate a total expectation of MEV if i were to wait until the end of the slot. Very important preconf tips are priced correctly. USer and proposer we have a helper called a gateway. The gateway is trusted to price the tips properly. If tips priced too low, then there is less money. The user if charged too much they won't be happy either and won't be willing to pay for precomf and will get angry at the gateway. The gateway needs to price this thing very precisely. In OFAs there are similar problem , you have a window of time flow coming in here solvers or searchers that have to respond, you have settlement happening here, not an OFA expert but there is some uncertainty about the future. Even If i win the ofa auction not the blcokc-builder am the searcher, need to price future uncertainty, MEV gain and MEV loss, will be really cool if someone solves this problem
		- multiple actors in OFA vs. Single signal in the first one, in both cases there are uncertainty on future flow vs. who the winning block builder will be, and also who the wiling solver will be. Here the competition is lacking and there will be a monopoly - here its a two party auction and you get correct pricing, on the other side u have user, proposer and searchers, this three way auction dont know how to solve yet.  

## Tradefi experience how it relates to chain abstraction CAKE
(From Orchestration to Abstraction)
- In general regulated payment systems an interest
- Form advent of people figuring how to trade for prophet - atomic swaps continuous to discrete signals to financial evolution of complex human interactions - driven by perennial quest for convenience - intrinsic motivations propel us to simplification 
	- prone to natural forces that lead to entrenchment 
- The web of payment systems we use today - payers, payees, network operators, gateways, regulators, clearing houses, central banks....
	- authentication - pay with digital sig
	- authorization - custodian checks funs
	- Messaging - known as switching details of payload communicates
	- settlement - locally institutions use clearing houses - standards swift messaging
- Also how card payments work - fees are completely arbitrary and vary across geographies
	- Visa wanted to boost profit - invented a route that send data there and back and charged on both sides
	- Unsuited to digital world - card payments are IRL MEV - 50B USD per year, 
	- Additional security comes with extra payment for merchants (tax)
	- Receiving payments online was not as simple - purchase something overseas funds take anywhere from two to eight weeks to hit merchants bank account 
- How does real time MEV emerge? - card fraud
    - How is MEV? - its not same as MEV but is extractive force that should be helping system self-improve but doesn't do this
- cross- border model relies on network of corresponding banks
- Leads to astronomical settlement costs and massive times to make this happen
- fully reliant on periodic swift messaging - truly open global financial abstraction 
- Local instant payment schemes adopted canonically across nations - load up an EOA which holds funds on trust
- Others saw arb opportunity - automate online payments b/w banks - train bots to make transfers on your behalf -"screen-scraping"
- EU regulators stepped in - PSD2 - everyone banks exposed interfaces with a set of APIs - open bank in Europe 50 countries adopted this own instant payment scheme ICO 22 standard
	- market driven or regulation driven frameworks
- Australia took an industry driven approach - open banking is control - new payment scheme that is instant
- Trust or governance frameworks are systems of control - underscores absence of global banking standards 
- Global payments ecosystem exhibits super strong paradox - AMEX, VISA, MC, ALLY pay adopt technical standards together - QR codes for example - illusion of cohesion is managed by these standards - only alternative is walled gardens which manage their own liquidity pools across jurisdictions - extensions of local payment schemes
- If the core is rotted on relationships and messaging systems the illusion can only take us so far
- Limitations - reliance on permissioned infrastructures, rent-seeking behaviors 
	- assumes a level of trust these systems have attempted to justify 
	- oversight creates additional cost and also dependencies leading to monopolies 
	- consider the dual edges of standardization** - want to avoid cartel like behavior and ensure its adaptable
- DeFi space - transparency, decentralization, overcome fragmentation (alt framework that potentially helps solve problems)
- reduce complexity - unified abstraction create more time to solve next problems
- Craft standards that are flexible, democratic and simplify complexity - remove dependencies, hurdles imposed by nation states, resource requirements (DID, bandwidth)
    - blockchains are our best shots at shaping this 
- How can we harness abstraction and emergence in financial innovation?
    - x-chain comms and tx execution - abstract it away overcoming front and back-end inertia - we can definitely achieve efficiency gains and make the systems accessible to wider audiences
	- adaptive and sophisticated behavior patterns across blockchains
- Simplicity + modularity allows innovation to flow
- CAKE mirrors dialectic process - opposing forces propel the system forward - people buy into it , true importance of network effect (standards) - quantization transform signals into high frequency discrete digital transactions which are harmonized
- New levels of abstraction - consider crucial role of cryptography (information flow control)
- Data integrity will work to reduce the noise - mitigate the risk of information leakage and enhance trust amongst participants (crypto + formal verification)
- ability to process txs with high fidelity is essential to protect system from manipulation
- How can chain abstraction and CAKE apply to financial complexities of the mempool? 
    - Mapping out the tradfi system can pinpoint operations for consolidation - mitigate congestion - abstracting and segmenting tx into smaller and discrete units - optimize for more specialized processing
    - Enhancing clarity of financial tx submission - increase system confidence (enhanced interop is critical - both systems can learn from each other)
    - Standardized adapters - designed to serve as lynchpin in interop  (data formats) - humans can engage in financial systems without understanding technical intricacies
- Protocol transaction layer - messages, settlement commands are accurately translated and convertible across ecosystems
	- challenge: identity verification, authentication methods that are safe - we can solve problems more quickly 
	- compliance and openness - create an interesting tension - adaptive framework can suggest frameworks like unified dashboards bringing transparency - tradfi regulation has limited innovation (self-regulation by community is one approach - we should embrace this)
	- regulators will learn through engaging in open conversation

### Autonomous Agents
- AI into financial infra - enhance capacity for abstractions - predictive capabilities will help
- Orchestration of future finance removes boundaries

What do you think we can do to avoid replicating mistakes of payments industry? 
- standardization is a double edge sword - self- adapting standards, always being open to change as needed. No regulatory burdens impacting us we can be dynamic - put some framework for structure
- EMVco - formed by visa and MC - is this how you expect things to evolve here some player develops standards and everyone adopts them?
	- Depends - Defi encourages competition - perhaps its just strongest survive
	- Standards with 4337 - solved a lot of issues and enshrined a certain way of doing things which shows many cracks - standards minimized perhaps (lesson we can learn from tradfi and learn from our own experience - enshrine or not good for standardization but hard to adapt to changing trends)
	- 3074 vs. 4337 - hardfork vs. non-hardfork - what learnings, lessons, and mistakes can we draw from here.
- When we discuss standards we assume it will be EVM is standardized - there is something to be said for problems being solved being universal - is there a way to design an even more general AA standard - in favor of minimal standards

----

## Cross-chain without Bridges

Cross chain defi is increasingly important - getting incredibly tough as well 

- looking at 2 beat roughly there are about 80 rollups that are live - something we are thinking about and trying to solve, Near is asynchronous - every shard produces chunks and aggregates blocks 
- When you look at bridge volume - the bridge volume is almost 2-4x up since november - the volumw is always increasing - folks want to borrow or lend assets from different chains 
- Bridges are not just enough - need to do more to solve these problems
	- Binance, Orbit bridge recently, $3.2B stolen in DeFi hacks
- Lack of chain support - a bunch of bridges from Near - built in 2020 and its slow which becomes a problem for near DeFi. If you look at ecosystem there is one highway but nothing to connect them all, Wormhole has been all over the place
- Many bridges but bridges don't support all of the blockchains 

Inconsistent UX
- Bridging speeds, usually minutes to hours
- Lack of assets liquidity 
- Painful gas token management

Programmable MPC is a crucial abstractions
- Support all chains including some non-smart contract ones
- Instant support for new chains
- Standardized user and developer experience 

Programmable MPC - there are several folks in the space working on this  
- account-based what NEAR is going for (ping MPC to get wallet back)
- Deposit based approach - most other folks are taking (validators support ethereum - run light client think about it as a multi-sig, a credit account on each chain)

Chain signaturs
- Threshold signatures controlled by Near accounts and smart contracts
- Simple API you send some payload and key derivation, and key type, ECDSA currently, ED25119 later
- MPC nodes hold the accounts - when you send that signature, validators sign payload via MPC
	- can build gas free layer on top if user doesn't want to pay gas 
	- put multi-chain relayers between address and MPC node

Use Cases
1. Bridgeless cross-chain DeFi
2. DeFi on non-smart contract chains
3. Multi-chain Account Abstraction
4. Trust-less multichain deployments
5. Privacy-focused apps

MPC is a multi-sig? Yes you have to do compute if you increase the nodes the compute overhead increases - considered trade-off between SGX and MPC. 

- Native swaps between chains 
- Cross-chain lending order-book 
- Re-staking for any asset on any-chain
- Buy and sell Ordinals in Marketplace

Ordinals marketplace example - buyer and seller and marketplace contract that lives on near - buyer deposits USDC on NEAR side, what happens is when created seller can send to BTC account - Buyer accepts the listing and at the end the seller gets 10 USDC on NEAR - the owner changes from seller to buyer. So asset lives on BTC and NEAR. 

Do you have slide on account based between deposit based - MPC side one problem is the collusion is one of the trade-offs, what happens if the nodes collude - trying to figure out slashing for collusion 
- Account based MPC nodes - account is retrieved by MPC - can have an account based system inside of SGX - seems concern is related to MPC vs account based system 
- Is there significant latency added to MPC - 12 seconds to sign with only 5 nodes. 

MPC on deposit side - enforcement of threshold signatures schemes (FROST for example) which defines how many validators you can run. Thorchain approach is more costly and complex. What cryptography you use to allow nodes to operate on the network. MPC is a requirement if you are supporting bitcoin. 
- Any chain that supports SC you could recover the regular cryptographic scheme - pre-processes tx this was signed by this person public function any relayer can support on any chain
- Deposit perspective - the architecture - CEX do trading logic and settlement - how to replicated in decentralized way - settlement logic the bast way to allow to run deposits on each chain. We have our app-chain when a tx comes through it is monitored by the Network if the deposit goes from Ethereum to Bitcoin - 
- On each chain there is a pool users deposit to - in account based everyone as account you deposit into 

How does the assets movement happen on separate chains - asset stays on base chains. 

How does it work in case when I have ETH on Ethereum and want it on whatever else - would be marketplace contract - pooled or per address? it is per address - seller creates Eth account, USDC on NEAR, deposit Eth, list ordinal for 10 USDC, accept listing and finally. Bitcoin account is fully controlled by Bitcoin Nodes. Seller revokes control. Seller when they create Bitcoin account. 
- even for transfers you need to find a counterparty 
- Seems like Deposit is better because i can force exit depending on how proofs are generated. Challenge of exiting state to arbitrary chain. Need some sort of economic security on the MPC. What about re-orgs and finality - is USDC settlement only possible on NEAR. Finality is dependent on NEAR finality. 

Can you explain through the lense of counterparty risk? MPC node - its threshold so you can collude the network if you have an adversary. Trust if near validators are opting in. You might also have marketplace smart contract risk. 

Are validators managed reputation based - Block producers are about 100k NEAR - for validator to join validator set there are spots open. If you are chunk producer you need to pledge more.

Is the account ownership just being transferred to the buyer - the MPC network managed this for the network? Te seller creates the account through the MPC - MPC network changes the owner at the end. Would there have to be custom permission here? Programmable MPC has permissions to change the owner. 

---- 
## Aori - Future proof DEX design 

How Defi builders could look at things Top down such that they can be able to make new iterations and save costs

My on-chain Order-book 
- bottom up approach - start learning solidity, hardhat, play around with tools making DEX your own order-book - take different limit orders and match them against each other - with this protocol in mind can create a solidity smart contract 
- Very inefficient with gas involved - a dependency created by operating completely onchain 
- Consider moving orderbook off chain with onchain settlement - still might be the case that Ethereum global fee market may not work for settlement
- Version three deploy onto Layer 2  - Arbitrum, make use of blockspace there. Its the case in which we notice this issue of gas - have to bridge over to arbitrum and need gas to trade does cause friction - may move to something more solver based. Solvers paying gas for transactions. Users sign different signatures that are intent objects that represent different limit orders. 
- Gotten a lot of news asking us when you going to go live. Is there a different consensus or sequencing mechanism which might mess up how limit orders are processed. 
- Where to deploy settlement contracts are opcodes available on newer chains. Creates a lot of headache. Can you guys also do cross-chain stuff as well - what about aggregation across all chains. Throw some ZK in there in terms of proof based approach. All of this has led us along this incremental path where we very mcuh have had to make lots of different nuanced choices. Execute on chain how users are affected, inefficiencies. Migrate them over to new systems migrating version 2s over to version three. Still much traction on V2 even though V3 might be seen as much customizability. The other side in terms of more top down approach when it comes to dapp developer very much the case where you have to think about all of these sort of things. EVM has been the most standardized and available on these different blockchains - SVM (move, SUAVE) - what a dapp developer has to ask is how wil lI work with these different systems? Accomodate these new environments to be able to handle transactions
- For smart contract wallets, Gnosis safes make use of EIP 1271 standard. For Solana its ED25119 its also all sorts of different standards a developer would have to think about 
- On the solver side of things many of the solvers want to make use of their own private liquidity (money market or AMM), make the use of flash loans - Its very much the case in which market maker or LP can say how to get liquidity in the pool, they make opinionated design choices
- Accounting is a mess - EIP 1271 helps clean these up but different systems for griefing as well you have off-chain component which may check for things weather an address is completely clean - funds might be in another account which is not the signer of a signature
- Local or temporary solutions are attempting to solve for these - sig verification for both smart contract wallets, utilize essentially brute force different signature verification methods to see if one ids valid or not. ertain signature schemes are very different 
- Intent details - signer and a funder - good example of this is in account abstraction - you can have someone else pay gas when you abstract away the gas via paymaster then you are able to have separate funding accounts - multiple accounts to pull liquidity from 

### In Modularity We trust 
- Different solutions we've seen different data in an attempt to standardize specific implementation details - when you think about top left or top down - take inspiration from good friends at scratch - can be seen as programming language or programming tool - simple user interface where you can have simple blocks which are very high level
	- Alleviate these details - does the web browser need to make sure the sound is checked or user hasn't muted this specific tab - different sort of blocks we can think about in DeFi (transfer - a step inside the protocol, but the specific implementation is differed for later; transferring from a vault for example)
- Standards that people come with bottom up are static social contracts of confirming to static implementation of how this resource, token or account is brought in 
- Conceptualizing protocols inside high level blocks
Simple OB contract or AMM - not be able to handle smart contract wallets - can we hot swap in different functionalities - native EC recovers on EVM case. Be able to handle perhaps the news of account abstraction scheme of 3074, think about it in a lighter way such that we are not dependent on specific implementation details - think about it at a high level where the main protocol itself does not need to chain

- Abstract away addresses inside of settlement contracts - SVM addresses and EVM addresses, module adapter what interprets address 
- Decentralized validation networks you can pick yourself or others can pick for you - you choose what risks you want to take
	- Uniswap V4 allows for more opinionated swaps, Across Bus and being able to have cross-chain composability after your message is processed then some sort of SC made on your behalf
	- Modular approach in Cosmos ecosystem 

### Creating a modular single-chain CLOB
- validator Maker's limit order - > validate taker's limit order make sure its sufficient
- transfer taker's assets to the taker and vica versa 
- Simple protocol but it might be how CLOBs become structures 
- In terms of how to implement this - Can be left for later - code up these different modules (adaptors or gadgets that can pull in logic and do the necessary work for you)
	- verify the Maker's limit order (ECDSA sig scheme to make sure intent matches signature)
	- Transferring taker's assets (allow for people to choose module that allows to pull assets from different account s or flash loan in)
- Achieve infrastructure abstraction - every new functionality that can potentially support a new VM, protocol , or environment that comes up - Code a s a different module 
	- Can do this optimistically - 3rd party agent potentially a solver do this job for you (verify sig or generate ZKP off-chain)
- These modules can be fitted in as we like and can have either security or economic security guarantees of user's choosing
- In terms of offering dapp to user this isn't necessarily the best - cause a lot of friction when it comes to UX - take different modules user's want to use and have opinion of them 
	- who is the mot gas efficient (hyperlane, axelar, Layer0?)
- Simple as CAKE choose modules you like and offer clean experience to user. 

Flash order books 
- raised seed round off of this - essentially a tool an idea where you can aggregate order book fit in all of these different pieces of pulling in liquidity from frequent batch auctions - essentially have solvers doing the execution for you for gas abstraction. 
- Rivals a FBA but can also be filled on top of it. 

How to protect intent solvers from griefing - assuming the account implementation are permissionless - could behave differently on chain - passes validation but on chain assett never moves?
- 2 ways to go about griefing - do it onchain by charging people if they grief or do it off chain and accept the fact that griefing will happen, either permission or control 
- Comes down to how you want to protect users and dapp developers - you yourself the bundler having a means to essentially throw away spam orders - user sends you a lot of spam requests (opion) - charging model there of gas vs. Off-chain execution 

Can you scope the problem space of solver griefing?
- relates to can have different users send different requests which may supprot these different sort of standards  - smart contract sig verification - you have to do this lookup simulation on chain to verify the signature works
- could be the case where someone makes a malicious contract where sig verification contract is valid but runs you up to 100M gas which would be very costly in compute
- Nodes might do this but NOdes might defer to solvers to do themselves in an optimistic case - this simulation is all done from nothing and charges everyone in the systems

The case of validation being malicious - validation you have to do cost validation, need an oracle that comes in a storage proof - we havn't reached that sort of level where proofs are viable
- I think about not validation function its the execution which is malicious. Receive the assett, not going to transfer the assett - still means you paid for the gas - griefing in execution and not validation
- Solver cryptographically commit to a transaction and have a staking slashing condition - of solver did something else within expiry date could commit proof
- Its the user being adversarial to the Solver - does this always come down to last look? Very much an open problem - some systems expect user to not be adversarial - some systems encodes in adversarial assumption 
- Feels like equillibrium will end up being protection for the solvers and reputation based protection of solvers from users. Even completely eliminating potential griefing of solvers - don't want to assume honesty assumptions of the user. 

# Cake WG v3 ⊙

:::info
💡 Notes from the working group breakout discussion after lunch - *not* comprehensive
:::

## tl;dr 
- token representations (best solution is xERC20)
- revert rates (OFA trilema —> need to reduce risk for solvers, oneb helps from upstream, precommits help from downstream)
- speed (Onebalance —> eliminates need for source chain tx)

### Discourse 



### User Agency in Cross chain 
- I control my assets -> I want this state transition or that one 
	- should not be hard for user
- Funds locked in OneBalance -> tension here b/w locks and equivocation
- if we black box the system the lock bounds the system
- Assets should be liquid <-> fungibility b/w tokens
    - fungibility b/w non-canonical representations
    - challenges: different fee markets, slippage, gas, high revert rates
- One solution: xERC-20 whitelist bridges and rate limit them
    - in general gas estimation is an unsolvable problem
    - what type of tokens are we talking about? 
	    - blue chip -> long-tail-> LP shares
    - ideally token liquidity in one place

### All bridging technologies are immature with hard to maintain integrations
- arbitrary messaging bridges -> all bridges relay on some form of multi-sig? 
	- MPC, optimistic approach, ZK - relayers/oracle
	- all these bridges have some kind of trust assumption
	- can we have a module that uses zk light clients? -> trust modules are unclear
- one idea: think of a diamond proxy across multiple chains, treat state asynchronously across chains
- create ultra-secure connection between L1 chains
- users are not willing to compromise or care about trust assumptions until its too late
	- users are not putting pressure on anyone to adopt xERC-20s
	- how do we promote security and maximize security at the same time? 

### Cross-chain reverts
- gas estimations are not reliable
- one solution: dynamically check 

### Whiteboard

![image](https://hackmd.io/_uploads/Hyxpb9E4A.png)


- gas futures could help but unlikely to be implemented anytime soon
- users only care about predictability, latency and cost
- finality risk

Gas abstraction -> solver risk/execution risk

### How should JSON-RPC API be exposed to capabilities that wallets have? 
- e.g. Ethereum tx (this is how you pay for gas) -> what does this new API look like
- what permissions and how are they enforced? 
- expose to the app developers, purely frontend to wallet communication
- credible account model: super app-specific rollup
- it is extremely difficult working with wallets /dapps if adding  more thing to JSON-RPC API
	- existing methods would have to overload
- Do dapps need to be aware of credible accounts? 