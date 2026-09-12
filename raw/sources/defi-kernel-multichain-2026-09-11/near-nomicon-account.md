1. [Introduction](../index.html)
2. - Protocol Specification
   - [Data Structures](../DataStructures/index.html)
   - 1. [Access Keys](../DataStructures/AccessKey.html)
     2. [Accounts](../DataStructures/Account.html)
     3. [Block and Block Header](../DataStructures/Block.html)
     4. [Data Types](../DataStructures/DataTypes.html)
     5. [Merkle Proofs](../DataStructures/MerkleProof.html)
     6. [Transaction](../DataStructures/Transaction.html)
   - [Chain Specification](../ChainSpec/index.html)
   - 1. [Block Processing](../ChainSpec/BlockProcessing.html)
     2. [Consensus](../ChainSpec/Consensus.html)
     3. [Light Client](../ChainSpec/LightClient.html)
     4. [Selecting Chunk and Block Producers](../ChainSpec/SelectingBlockProducers.html)
     5. [Transactions in the Blockchain Layer](../ChainSpec/Transactions.html)
     6. [Upgradability](../ChainSpec/Upgradability.html)
     7. [Epochs and Staking](../ChainSpec/EpochAndStaking/index.html)
     8. 1. [Epoch](../ChainSpec/EpochAndStaking/Epoch.html)
        2. [EpochManager](../ChainSpec/EpochAndStaking/EpochManager.html)
        3. [Staking and slashing](../ChainSpec/EpochAndStaking/Staking.html)
   - [Network Specification](../NetworkSpec/NetworkSpec.html)
   - 1. [Messages](../NetworkSpec/Messages.html)
     2. [Routing Table Exchange Algorithm](../NetworkSpec/RoutingTableExchangeAlgorithm.html)
   - [Runtime Specification](../RuntimeSpec/index.html)
   - 1. [Account Receipt Storage](../RuntimeSpec/AccountStorage.html)
     2. [Actions](../RuntimeSpec/Actions.html)
     3. [Applying chunk](../RuntimeSpec/ApplyingChunk.html)
     4. [Components](../RuntimeSpec/Components/Components.html)
     5. 1. [Runtime Crate](../RuntimeSpec/Components/RuntimeCrate.html)
        2. [Bindings Specification](../RuntimeSpec/Components/BindingsSpec/BindingsSpec.html)
        3. 1. [Context API](../RuntimeSpec/Components/BindingsSpec/ContextAPI.html)
           2. [Economics API](../RuntimeSpec/Components/BindingsSpec/EconomicsAPI.html)
           3. [Math API](../RuntimeSpec/Components/BindingsSpec/MathAPI.html)
           4. [Miscellaneous API](../RuntimeSpec/Components/BindingsSpec/MiscellaneousAPI.html)
           5. [Promises API](../RuntimeSpec/Components/BindingsSpec/PromisesAPI.html)
           6. [Registers API](../RuntimeSpec/Components/BindingsSpec/RegistersAPI.html)
           7. [Trie API](../RuntimeSpec/Components/BindingsSpec/TrieAPI.html)
     6. [Fees](../RuntimeSpec/Fees/Fees.html)
     7. [Function Call](../RuntimeSpec/FunctionCall.html)
     8. [Contract Preparation](../RuntimeSpec/Preparation.html)
     9. [Receipts](../RuntimeSpec/Receipts.html)
     10. [Refunds](../RuntimeSpec/Refunds.html)
     11. [Runtime](../RuntimeSpec/Runtime.html)
     12. [Transactions](../RuntimeSpec/Transactions.html)
     13. [Scenarios](../RuntimeSpec/Scenarios/Scenarios.html)
     14. 1. [Financial Transaction](../RuntimeSpec/Scenarios/FinancialTransaction.html)
         2. [Cross-ContractCall](../RuntimeSpec/Scenarios/CrossContractCall.html)
   - [Economics](../Economics/Economics.html)
   - [GenesisConfig](../GenesisConfig/GenesisConfig.html)
   - 1. [ExtCostsConfig](../GenesisConfig/ExtCostsConfig.html)
     2. [VMConfig](../GenesisConfig/VMConfig.html)
     3. [StateRecord](../GenesisConfig/StateRecord.html)
     4. [RuntimeConfig](../GenesisConfig/RuntimeConfig.html)
     5. [RuntimeFeeConfig](../GenesisConfig/RuntimeFeeConfig.html)
     6. 1. [AccessKeyCreationConfig](../GenesisConfig/RuntimeFeeConfig/AccessKeyCreationConfig.html)
        2. [ActionCreationConfig](../GenesisConfig/RuntimeFeeConfig/ActionCreationConfig.html)
        3. [DataReceiptCreationConfig](../GenesisConfig/RuntimeFeeConfig/DataReceiptCreationConfig.html)
        4. [Fee](../GenesisConfig/RuntimeFeeConfig/Fee.html)
        5. [Fraction](../GenesisConfig/RuntimeFeeConfig/Fraction.html)
        6. [StorageUsageConfig](../GenesisConfig/RuntimeFeeConfig/StorageUsageConfig.html)
   - - Architecture
     - [Overview](../architecture/index.html)
     - [How neard works](../architecture/how/index.html)
     - 1. [How Sync Works](../architecture/how/sync.html)
       2. [Garbage Collection](../architecture/how/gc.html)
       3. [How Epoch Works](../architecture/how/epoch.html)
       4. [Transaction Routing](../architecture/how/tx_routing.html)
       5. [Transactions And Receipts](../architecture/how/tx_receipts.html)
       6. [Cross shard transactions - deep dive](../architecture/how/cross-shard.html)
       7. [Gas](../architecture/how/gas.html)
       8. [Receipt Congestion](../architecture/how/receipt-congestion.html)
       9. [Meta transactions](../architecture/how/meta-tx.html)
       10. [Serialization: Borsh, Json, ProtoBuf](../architecture/how/serialization.html)
       11. [Proofs](../architecture/how/proofs.html)
       12. [Resharding V2](../architecture/how/resharding_v2.html)
       13. [Optimistic block](../architecture/how/optimistic_block.html)
     - [How neard will work](../architecture/next/index.html)
     - 1. [Catchup and state sync improvements](../architecture/next/catchup_and_state_sync.html)
       2. [Malicious producers and phase 2](../architecture/next/malicious_chunk_producer_and_phase2.html)
     - [Storage](../architecture/storage.html)
     - 1. [Storage Request Flow](../architecture/storage/flow.html)
       2. [Trie Storage](../architecture/storage/trie_storage.html)
       3. [Database Format](../architecture/storage/database.html)
       4. [Flat Storage](../architecture/storage/flat_storage.html)
     - [Network](../architecture/network.html)
     - [Gas Cost Parameters](../architecture/gas/index.html)
     - 1. [Parameter Definitions](../architecture/gas/parameter_definition.html)
       2. [Gas Profile](../architecture/gas/gas_profile.html)
       3. [Runtime Parameter Estimator](../architecture/gas/estimator.html)
     - - Practices
       - [Overview](../practices/index.html)
       - [Rust 🦀](../practices/rust.html)
       - [Workflows](../practices/workflows/index.html)
       - 1. [Run a Node](../practices/workflows/run_a_node.html)
         2. [Deploy a Contract](../practices/workflows/deploy_a_contract.html)
         3. [Run Gas Estimations](../practices/workflows/gas_estimations.html)
         4. [Localnet on many machines](../practices/workflows/localnet_on_many_machines.html)
         5. [IO tracing](../practices/workflows/io_trace.html)
         6. [Profiling](../practices/workflows/profiling.html)
         7. [Benchmarks](../practices/workflows/benchmarks.html)
         8. 1. [Synthetic Workloads](../practices/workflows/benchmarking_synthetic_workloads.html)
            2. [Native Transfer Chunk Application](../practices/workflows/benchmarking_chunk_application_on_native_transfers.html)
         9. [Working with OpenTelemetry Traces](../practices/workflows/otel_traces.html)
         10. [Futex contention](../practices/workflows/futex_contention.html)
       - [Code Style](../practices/style.html)
       - [Documentation](../practices/docs.html)
       - [Tracking Issues](../practices/tracking_issues.html)
       - [Security Vulnerabilities](../practices/security_vulnerabilities.html)
       - [Fast Builds](../practices/fast_builds.html)
       - [Testing](../practices/testing/index.html)
       - 1. [Python Tests](../practices/testing/python_tests.html)
         2. [Testing Utils](../practices/testing/test_utils.html)
         3. [Test Coverage](../practices/testing/coverage.html)
       - [Protocol Upgrade](../practices/protocol_upgrade.html)
       - - Advanced configuration
         - [Networking](../advanced_configuration/networking.html)
         - - Custom test networks
           - [Forknet scenarios](../../pytest/tests/mocknet/docs/forknet_scenario.html)
           - [Mirror tool](../../pytest/tests/mocknet/docs/mirror.html)
           - - Misc
             - [Overview](../misc/index.html)
             - [State Sync Dump](../misc/state_sync_dump.html)
             - [Archival node - recovery of missing data](../misc/archival_data_recovery.html)
             - [Grafana MCP Setup for Claude Code](../grafana-mcp-setup.html)
             - [Zulip MCP Setup for Claude Code](../zulip-mcp-setup.html)

* Light (default)
* Rust
* Coal
* Navy
* Ayu

Guide to Nearcore Development
=============================

[Accounts](#accounts)
=====================

[Account ID](#account-id)
-------------------------

NEAR Protocol has an account names system. Account ID is similar to a username. Account IDs have to follow the rules.

### [Account ID Rules](#account-id-rules)

* minimum length is 2
* maximum length is 64
* **Account ID** consists of **Account ID parts** separated by `.`
* **Account ID part** consists of lowercase alphanumeric symbols separated by either `_` or `-`.
* **Account ID** that is 64 characters long and consists of lowercase hex characters is a specific **NEAR-implicit account ID**.
* **Account ID** that is `0x` followed by 40 lowercase hex characters is a specific **ETH-implicit account ID**.
* **Account ID** that is `0s` followed by 40 lowercase hex characters is a specific **NEAR-deterministic account ID**.

Account names are similar to a domain names.
Top level account (TLA) like `near`, `com`, `eth` can only be created by `registrar` account (see next section for more details).
Only `near` can create `alice.near`. And only `alice.near` can create `app.alice.near` and so on.
Note, `near` can NOT create `app.alice.near` directly.

Additionally, there is an [implicit account creation path](#implicit-account-creation).

Regex for a full account ID, without checking for length:

```
^(([a-z\d]+[\-_])*[a-z\d]+\.)*([a-z\d]+[\-_])*[a-z\d]+$
```

### [Top Level Accounts](#top-level-accounts)

| Name | Value |
| --- | --- |
| REGISTRAR\_ACCOUNT\_ID | `registrar` |

Top level account names (TLAs) are very valuable as they provide root of trust and discoverability for companies, applications and users.
To allow for fair access to them, the top level account names are going to be auctioned off.

Specifically, only `REGISTRAR_ACCOUNT_ID` account can create new top level accounts (other than [implicit accounts](#implicit-accounts)). `REGISTRAR_ACCOUNT_ID` implements standard Account Naming (link TODO) interface to allow create new accounts.

*Note: we are not going to deploy `registrar` auction at launch, instead allow to deploy it by Foundation after initial launch. The link to details of the auction will be added here in the next spec release post MainNet.*

### [Examples](#examples)

Valid accounts:

```
ok
bowen
ek-2
ek.near
com
google.com
bowen.google.com
near
illia.cheap-accounts.near
max_99.near
100
near2019
over.9000
a.bro
// Valid, but can't be created, because "a" is too short
bro.a
```

Invalid accounts:

```
not ok           // Whitespace characters are not allowed
a                // Too short
100-             // Suffix separator
bo__wen          // Two separators in a row
_illia           // Prefix separator
.near            // Prefix dot separator
near.            // Suffix dot separator
a..near          // Two dot separators in a row
$$$              // Non alphanumeric characters are not allowed
WAT              // Non lowercase characters are not allowed
me@google.com    // @ is not allowed (it was allowed in the past)
system           // cannot use the system account, see the section on System account below
// TOO LONG:
abcdefghijklmnopqrstuvwxyz.abcdefghijklmnopqrstuvwxyz.abcdefghijklmnopqrstuvwxyz
```

[System account](#system-account)
---------------------------------

`system` is a special account that is only used to identify refund receipts. For refund receipts, we set the predecessor\_id to be `system` to indicate that it is a refund receipt. Users cannot create or access the `system` account. In fact, this account does not exist as part of the state.

[Implicit accounts](#implicit-accounts)
---------------------------------------

Implicit accounts work similarly to Bitcoin/Ethereum accounts.
You can reserve an account ID before it's created by generating a corresponding (public, private) key pair locally.
The public key maps to the account ID. The corresponding secret key allows you to use the account once it's created on chain.

### [NEAR-implicit account ID](#near-implicit-account-id)

The account ID is a lowercase hex representation of the public key.
An ED25519 public key is 32 bytes long and maps to a 64-character account ID.

Example: a public key in base58 `BGCCDDHfysuuVnaNVtEhhqeT4k9Muyem3Kpgq2U1m9HX` will map to the account ID `98793cd91a3f870fb126f66285808c7e094afcfc4eda8a970f6648cdf0dbd6de`.

### [ETH-implicit account ID](#eth-implicit-account-id)

The account ID is derived from a Secp256K1 public key using the following formula: `'0x' + keccak256(public_key)[12:32].hex()`.

Example: a public key in base58 `2KFsZcvNUMBfmTp5DTMmguyeQyontXZ2CirPsb21GgPG3KMhwrkRuNiFCdMyRU3R4KbopMpSMXTFQfLoMkrg4HsT` will map to the account ID `0x87b435f1fcb4519306f9b755e274107cc78ac4e3`.

### [Implicit account creation](#implicit-account-creation)

An account with a NEAR-implicit or ETH-implicit account ID can only be created by sending a transaction/receipt with a single `Transfer` action to the implicit account ID receiver ([deterministic accounts](#deterministic-account-creation) have their own rules):

* The account will be created with the account ID.
* The account balance will have a transfer balance deposited to it.
* If this is NEAR-implicit account, it will have a new full access key with the ED25519-curve public key of `decode_hex(account_id)` and nonce `(block_height - 1) * MULTIPLIER` (to address an issues discussed [here](https://gov.near.org/t/issue-with-access-key-nonce/749)).
* If this is ETH-implicit account, it will have the [Wallet Contract](#wallet-contract) deployed, which can only be used by the owner of the Secp256K1 private key where `'0x' + keccak256(public_key)[12:32].hex()` matches the account ID.

Implicit account can not be created using `CreateAccount` action to avoid being able to hijack the account without having the corresponding private key.

Once a NEAR-implicit account is created it acts as a regular account until it's deleted.

An ETH-implicit account can only be used by calling the methods of the [Wallet Contract](#wallet-contract). It cannot be deleted, nor can a full access key be added.
The primary purpose of ETH-implicit accounts is to enable seamless integration of existing Ethereum tools (such as wallets) with the NEAR blockchain.

### [Wallet Contract](#wallet-contract)

The Wallet Contract (see [NEP-518](https://github.com/near/NEPs/issues/518) for more details) functions as a user account and is designed to receive, validate, and execute Ethereum-compatible transactions on the NEAR blockchain.

Without going into details, an Ethereum-compatible wallet user sends a transaction to an RPC endpoint, which wraps it and passes it to the Wallet Contract (on the target account) as an `rlp_execute(target: AccountId, tx_bytes_b64: Vec<u8>)` contract call.
Then, the contract parses `tx_bytes_b64` and verifies it is signed with the private key matching the target [ETH-implicit account ID](#eth-implicit-account-id) on which the contract is hosted.

Under the hood, the transaction encodes a NEAR-native action. Currently supported actions are:

* Transfer (from ETH-implicit account).
* Function call (call another contract).
* Add `AccessKey` with `FunctionCallPermission`. This allows adding a relayer's public key to an ETH-implicit account, enabling the relayer to pay the gas fee for transactions from this account. Still, each transaction has to be signed by the owner of the account (corresponding Secp256K1 private key).
* Delete `AccessKey`.

[Deterministic accounts](#deterministic-accounts)
-------------------------------------------------

Deterministic accounts are an advanced kind of implicit account.
A normal implicit account has a fixed access key that is implicitly associated with it.
Deterministic accounts have a fixed code and fixed initial state associated with it.

### [State-initializing data of deterministic accounts](#state-initializing-data-of-deterministic-accounts)

The initial state of a deterministic account is fully defined by an instance of `DeterministicAccountStateInit`.

Key-value pairs in `data` must be at most `max_length_storage_key` and `max_length_storage_value` bytes long,
respectively. These parameters are currently both set to 4Mib (4194304 bytes).

```
```
#![allow(unused)]
fn main() {
pub enum DeterministicAccountStateInit {
    V1(DeterministicAccountStateInitV1),
}

pub struct DeterministicAccountStateInitV1 {
    pub code: GlobalContractIdentifier,
    pub data: BTreeMap<Vec<u8>, Vec<u8>>,
}

pub enum GlobalContractIdentifier {
    CodeHash(CryptoHash) = 0,
    AccountId(AccountId) = 1,
}
}
```
```

### [Deterministic account ID](#deterministic-account-id)

The account ID is derived from a `DeterministicAccountStateInit` instance.

To derive the deterministic account id, borsh-encode the `DeterministicAccountStateInit` enum instance into raw
bytes. Then use the following formula: `'0s' + keccak256(bytes)[12:32].hex()`.

### [Deterministic account creation](#deterministic-account-creation)

Deterministic accounts cannot be created with a `CreateAccount` nor with a `Transfer` action.
A special action, `DeterministicStateInitAction`, is required.
This action validates that the account id matches its initial state.

As with all account types, you have to attach a large enough deposit to cover for the account storage. If the code and
initial state is small enough to fit below the zero-balance limit, no balance is required. The
`DeterministicStateInitAction` has a field to include a balance specifically to cover storage. Anything above the
required amount to cover storage requirements is refunded.

A `Transfer` can also deposit a balance before the `DeterministicStateInitAction`. The account will not be usable,
however, until the `DeterministicStateInitAction` is also executed.

After successful initialization, the account will have all key-value pairs in `data: BTreeMap<Vec<u8>, Vec<u8>>` in its
contract storage. The contract code will be set to that of the global contract defined with
`code: GlobalContractIdentifier`.

[Account](#account)
-------------------

Data for a single account is collocated in one shard. The account data consists of the following:

* Balance
* Locked balance (for staking)
* Code of the contract
* Key-value storage of the contract. Stored in a ordered trie
* [Access Keys](AccessKey.html)
* [Postponed ActionReceipts](../RuntimeSpec/Receipts.html#postponed-actionreceipt)
* [Received DataReceipts](../RuntimeSpec/Receipts.html#received-datareceipt)

#### [Balances](#balances)

Total account balance consists of unlocked balance and locked balance.

Unlocked balance is tokens that the account can use for transaction fees, transfers staking and other operations.

Locked balance is the tokens that are currently in use for staking to be a validator or to become a validator.
Locked balance may become unlocked at the beginning of an epoch. See [Staking](../ChainSpec/EpochAndStaking/Staking.html) for details.

#### [Contracts](#contracts)

A contract (AKA smart contract) is a program in WebAssembly that belongs to a specific account.
When account is created, it doesn't have a contract (except ETH-implicit accounts).
A contract has to be explicitly deployed, either by the account owner, or during the account creation.
A contract can be executed by anyone who calls a method on your account. A contract has access to the storage on your account.

#### [Storage](#storage)

Every account has its own storage. It's a persistent key-value trie. Keys are ordered in lexicographical order.
The storage can only be modified by the contract on the account.
Current implementation on Runtime only allows your account's contract to read from the storage, but this might change in the future and other accounts's contracts will be able to read from your storage.

NOTE: Accounts must maintain a minimum amount of value at a rate of 1 NEAR per 100kb of total storage in order to remain responsive.
This includes the storage of the account itself, contract code, contract storage, and all access keys.
Any account with less than this minimum amount will not be able to maintain a responsive contract and will, instead, return an error related to this mismatch in storage vs. minimum account balance.
See [Storage Staking](https://docs.near.org/concepts/storage/storage-staking) in the docs.

#### [Access Keys](#access-keys)

An access key grants an access to a account. Each access key on the account is identified by a unique public key.
This public key is used to validate signature of transactions.
Each access key contains a unique nonce to differentiate or order transactions signed with this access key.

An access key has a permission associated with it. The permission can be one of two types:

* `FullAccess` permission. It grants full access to the account.
* `FunctionCall` permission. It grants access to only issued function call transactions.

See [Access Keys](AccessKey.html) for more details.