> For the complete documentation index, see [llms.txt](/llms.txt)

# RPC interface

The Remote Procedure Call (RPC) layer lets external clients, such as dApps, wallets, explorers, and services, interact with a running Midnight node over HTTP/HTTPS or WebSocket connections.

In Midnight, RPCs follow the JSON-RPC standard and expose methods for:

* Submitting transactions, which trigger state transitions
* Querying on-chain contract state
* Fetching auxiliary data, such as off-chain values or metadata

These methods form the interface for building tools and applications on top of Midnight.

### Core RPC methods[​](#core-rpc-methods "Direct link to Core RPC methods")

Midnight nodes expose a set of custom RPC methods focused on ledger state, contract state, and system information. Key methods include:

```
#[method(name = "midnight_jsonContractState")]

fn get_json_state(

    &self,

    contract_address: String,

    at: Option<BlockHash>,

) -> Result<String, StateRpcError>;
```

Returns the JSON-encoded state of a given smart contract.

```
#[method(name = "midnight_contractState")]

fn get_state(

    &self,

    contract_address: String,

    at: Option<BlockHash>,

) -> Result<String, StateRpcError>;
```

Returns the raw (binary-encoded) contract state at a specific block (or the latest block if omitted).

```
#[method(name = "midnight_unclaimedAmount")]

fn get_unclaimed_amount(

    &self,

    beneficiary: String,

    at: Option<BlockHash>,

) -> Result<u128, StateRpcError>;
```

Fetches the amount of unclaimed tokens or rewards for a beneficiary address.

```
#[method(name = "midnight_zswapChainState")]

fn get_zswap_chain_state(

    &self,

    contract_address: String,

    at: Option<BlockHash>,

) -> Result<String, StateRpcError>;
```

Returns the current ZSwap chain state for a contract.

```
#[method(name = "midnight_apiVersions")]

fn get_supported_api_versions(&self) -> RpcResult<Vec<u32>>;
```

Lists all supported RPC API versions, which is useful for tooling compatibility checks.

```
#[method(name = "midnight_ledgerVersion")]

fn get_ledger_version(&self, at: Option<BlockHash>) -> Result<String, BlockRpcError>;
```

Returns the ledger version at the specified block.

### Polkadot SDK RPC support[​](#polkadot-sdk-rpc-support "Direct link to Polkadot SDK RPC support")

Midnight also supports default Polkadot SDK RPC methods, including:

* `system_health`
* `chain_getBlock`
* `state_getStorage`
* `rpc_methods`, which returns a list of all supported RPC endpoints.

You can call `rpc_methods` at any time to retrieve the full list of methods exposed by your node.

note

Some RPC methods exposed by Midnight might not appear in the [Polkadot JS web app](https://polkadot.js.org/). For a general reference, see the [Polkadot SDK RPC method documentation](https://polkadot.js.org/docs/substrate/rpc/). Midnight implementations overlap with this list, but support can differ by node version and configuration.

### Partnerchain RPCs[​](#partnerchain-rpcs "Direct link to Partnerchain RPCs")

Midnight exposes partnerchain-specific RPC methods, especially for block producers and sidechain integration. These methods query consensus signals, relay finality information, and coordinate between chains.

### Security note for block producers[​](#security-note-for-block-producers "Direct link to Security note for block producers")

warning

Not all RPC methods are safe to expose on public or production nodes.

If you run a block-producing node, be cautious about which RPC endpoints you enable. Some methods can leak sensitive data or increase performance risk.

To minimize risk:

* Use the `--rpc-methods Safe` flag to limit exposed methods to a safer subset.
* Avoid using `--rpc-external` unless absolutely necessary, as it exposes RPC interfaces to external networks.

Always review your node’s RPC configuration to ensure it aligns with your threat model and operational role (for example, validator vs. observer).
