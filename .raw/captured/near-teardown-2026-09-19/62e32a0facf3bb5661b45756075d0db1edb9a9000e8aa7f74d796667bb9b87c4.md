# Introduction

The Verifier contract is a smart contract on the Near blockchain that is used as the atomic mediator of transferring assets among users in the intents ecosystem. A brief explanation of the terms mentioned follows.

The [.](./ "mention") section is written for those who would like to interact with the `Verifier` smart contract directly, or would like to create payloads for the [Message Bus](../bus/). After all, the Message Bus acts as a matching system that brings together quotes from market makers with requests for transactions.

Note: The former name of the smart contract is "Defuse". It is still the name used for the smart contract at the time of writing this documentation in a few places. It is planned to change this in the future.

### What payload are we referring to, or what is the workflow?

Let's start with the assumption that any participant in the `Verifier` smart contract has the tokens in question [deposited](deposits-and-withdrawals/) into the smart contract.

In the intents protocol, a single user, be it a market maker, end-user or otherwise, expresses their "intent" to do a transaction. The transaction can be to transfer coins within the smart contract against receive other coins, transfer to another user, etc. These intents can be submitted directly to the smart contract, or can be bundled together by a 3rd party, like the Message Bus, to have them all executed atomically.&#x20;

Let's discuss a few use-cases of the `Verifier` smart contract.

#### Example 1: Simple transfer

Alice, the owner of `alice.near`, wants to transfer a fungible token from their own account to the user `bob.near`. In this case, Alice creates an Intent object of type `Transfer`, expressing the desired destination, `bob.near`, [signs it](signing-intents.md), and calls the function `execute_intents` in the Verifier smart contract. This will transfer ownership of the tokens in question from `alice.near` to `bob.near`.

#### Example 2: Token swap

Alice and Bob, agree to swap their `USDT` and `USDC`, 1000 `USDT` from Alice for 1000 `USDC` from Bob. It can be through chat, or a 3rd party tool, like the Message Bus. However, it is obvious that such a transaction requires trust if done without a impartial mediator. Alice could receive the `USDC` from Bob, and never send him the `USDT` she promised.&#x20;

A solution to this problem goes as follows using the `Verifier` smart contract:

1. Alice and Bob deposit their `USDT` and `USDC` to the `Verifier` smart contract. The `Verifier` maintains full ownership of these assets to their depositors, giving them the freedom to withdraw them in case of any party backing out.
2. Alice creates and signs an intent of type `TokenDiff`, expressing that she is willing to lose 1000 `USDT` if anyone would provide 1000 `USDC` in return.&#x20;
3. Bob creates and signs an intent of type `TokenDiff`, expressing that he is willing to lose 1000 `USDC` if anyone would provide 1000 `USDT` in return.
4. Both intents from Alice and Bob are bundled in a list. Alice or Bob can then call the function `execute_intents` in the `Verifier` smart contract. The smart contract will fulfill the swap, since both Intents are achievable within that transaction.&#x20;
5. Alice now has 1000 `USDC` in her account in the `Verifier` contract, and Bob has 1000 `USDT` in his account.
6. Alice and Bob have the right to withdraw their `USDC/USDT` and allocate the funds according to their preferences.

Notice that this example is how peer to peer trading is achieved using Near Intents. After all, all that is needed to be able to trade is to have a mediator that bundles swap requests, i.e., `TokenDiff` intents. This is what the Message Bus typically does. In fact, any 3rd party can do this.

#### Example 3: Withdrawal

From the previous example, if Alice wants to withdraw her USDC tokens, she creates and [signs](signing-intents.md) an Intent of type `FtWithdraw`, where she specifies the receiving address and the amount she wants to withdraw. She, then, calls the function `execute_intents` , and the `Verifier` smart contract will verify her ownership of the USDC tokens, and send them to the address she specified.

Withdrawals can also be done by directly calling the `Verifier` smart contract. More information in the [deposits-and-withdrawals](deposits-and-withdrawals/ "mention")

