> For the complete documentation index, see [llms.txt](/llms.txt)

# Tokens on Midnight

Midnight has one native token, NIGHT, and one native resource, DUST. NIGHT is transferable, used for staking and governance, and generates DUST over time. DUST is a shielded, non-transferable resource that the network consumes to pay transaction fees. Both operate across a dual ledger that supports shielded (private) and unshielded (public) state.

This section covers NIGHT, DUST, and the custom tokens you create in Compact.

## NIGHT and DUST[​](#night-and-dust "Direct link to NIGHT and DUST")

* **NIGHT**: the native utility token. You hold NIGHT for staking, governance, and DUST generation. Unit: STAR (1 NIGHT = 10^6 STAR).
* **DUST**: a shielded, non-transferable fee resource (not a token). Your NIGHT balance generates DUST over time up to a cap, and every transaction consumes it. Unit: SPECK (1 DUST = 10^15 SPECK).

For the economic rationale behind this design, see [Dual-component tokenomics](/concepts/dual-component-tokenomics.md). For the technical architecture of DUST (generation, decay, registration), see [DUST architecture](/concepts/dust-architecture.md).

## Shielded and unshielded[​](#shielded-and-unshielded "Direct link to Shielded and unshielded")

Midnight's dual ledger lets tokens exist in two forms:

* **Shielded**: private state where wallet addresses and transaction details stay confidential. DUST is always shielded, as are tokens minted with `mintShieldedToken`.
* **Unshielded**: public state visible on the ledger, similar to transparent blockchains. NIGHT is always unshielded, as are tokens minted with `mintUnshieldedToken`.

A token's privacy comes from its token type, not from the address that holds it. NIGHT is an unshielded token: its balances and transfers are always public, and holding NIGHT at a shielded address does not make it private. There is no mechanism to move a token between shielded and unshielded state, shielded and unshielded tokens are distinct token types, tracked in separate pools. For code examples of shielded and unshielded token transfers in Compact, see [Token transfers](/examples/contracts/token-transfers.md).

## Custom tokens[​](#custom-tokens "Direct link to Custom tokens")

You can create your own fungible and non-fungible tokens in Compact. The standard library provides `mintShieldedToken` (and its unshielded counterpart) to mint new tokens, and `sendShielded` / `receiveShielded` to transfer them. Token standards on Midnight will come through the MIP (Midnight Improvement Proposal) process. In the meantime, the [OpenZeppelin contracts for Compact](/sdks/community/openzeppelin-compact-contracts.md) library provides reusable access control and security primitives modeled on the Solidity originals.

To build and move your own tokens end to end, follow the two-part tutorial: [Create and transfer an unshielded token](/tokens/unshielded-token.md) and [Create and transfer a shielded token](/tokens/shielded-token.md).

## Fees[​](#fees "Direct link to Fees")

Every transaction on Midnight requires DUST. On a fresh wallet, DUST generation depends on how you register your NIGHT. Today, most NIGHT reaches Midnight through the cross-chain path (cNIGHT on Cardano), and the registration must finalize on Cardano and then reach a Midnight node, a process that takes about 12 hours. On a local network, DUST generates in about 5 minutes. This delay goes away once native mNIGHT launches. To get started:

* [Fund a wallet](/guides/acquire-tokens.md) (faucet tNIGHT, Lace registration, and the wallet SDK path)

## Further reading[​](#further-reading "Direct link to Further reading")

* [Dual-component tokenomics](/concepts/dual-component-tokenomics.md): the economic model behind NIGHT and DUST
* [DUST architecture](/concepts/dust-architecture.md): generation, decay, registration, and protocol parameters
* [Ledgers](/concepts/ledgers.md): the UTXO-based ledger and how it represents tokens
* [Token transfers example](/examples/contracts/token-transfers.md): shielded and unshielded transfer code in Compact
* [Compact standard library](/compact/standard-library.md): token management functions
