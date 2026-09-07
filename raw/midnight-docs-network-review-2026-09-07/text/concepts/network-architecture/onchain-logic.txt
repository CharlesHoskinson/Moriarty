> For the complete documentation index, see [llms.txt](/llms.txt)

# Onchain logic and state

The Midnight node follows the standard Polkadot SDK model for defining and executing on-chain logic. All core logic (except native pre-compiles) is compiled into *WebAssembly (WASM)*, which forms the node’s *runtime* — the state transition function executed consistently across all participating nodes.

This runtime comprises modular components known as *FRAME pallets*. Each pallet encapsulates a specific domain of logic and can include:

* Storage definitions (maps, multi-key maps, lists, values)
* Events
* Dispatchable functions (transactions)
* Offchain workers
* Hooks
* Host-exposed functions
* RPC methods

This modular design allows for a flexible and extensible on-chain execution environment, similar in spirit to a smart contract framework but compiled ahead of time and governed at the chain level.

Midnight’s runtime integrates both upstream pallets from *Polkadot SDK* and components developed by *Cardano Partner-Chains*. Examples include:

* `pallet-aura`: Handles block production using the AURA protocol.
* `pallet-grandpa`: Manages finality through the GRANDPA protocol.
* `pallet-partnerchains-session` and `pallet_session_validator_management`: Coordinate session rotation and validator management in the partnerchain context.

The core pallet powering Midnight’s privacy-preserving transaction logic is `pallet-midnight`. This internally maintained pallet encapsulates the state machine for the [Midnight Ledger](/api-reference/ledger.md).

`pallet-midnight` processes specialized transactions from the Midnight Ledger, including *ZSwap asset transfers* and *contract operations*. Instead of traditional signature-based dispatch, it validates the cryptographic *proofs* embedded in each transaction. Native libraries are used to verify these proofs and execute the corresponding state transitions.

After execution, the new state is committed on-chain. As with other Polkadot SDK chains, Midnight stores canonical ledger state in a *Patricia-Merkle trie* backed by a key-value database. A *commitment to the full Midnight Ledger state* is also persisted, providing a tamper-proof and verifiable snapshot after each block.
