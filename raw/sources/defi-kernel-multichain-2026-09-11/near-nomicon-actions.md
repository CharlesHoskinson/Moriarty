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

[Actions](#actions)
===================

There are a several action types in Near:

```
```
#![allow(unused)]
fn main() {
pub enum Action {
    CreateAccount(CreateAccountAction),
    DeployContract(DeployContractAction),
    FunctionCall(FunctionCallAction),
    Transfer(TransferAction),
    Stake(StakeAction),
    AddKey(AddKeyAction),
    DeleteKey(DeleteKeyAction),
    DeleteAccount(DeleteAccountAction),
    Delegate(SignedDelegateAction),
    DeployGlobalContract(DeployGlobalContractAction),
    UseGlobalContract(UseGlobalContractAction),
    DeterministicStateInit(DeterministicStateInitAction),
}
}
```
```

Each transaction consists a list of actions to be performed on the `receiver_id` side. Since transactions are first
converted to receipts when they are processed, we will mostly concern ourselves with actions in the context of receipt
processing.

For the following actions, `predecessor_id` and `receiver_id` are required to be equal:

* `DeployContract`
* `Stake`
* `AddKey`
* `DeleteKey`
* `DeleteAccount`

NOTE: if the first action in the action list is `CreateAccount`, `predecessor_id` becomes `receiver_id`
for the rest of the actions until `DeleteAccount`. This gives permission by another account to act on the newly created account.

[CreateAccountAction](#createaccountaction)
-------------------------------------------

```
```
#![allow(unused)]
fn main() {
pub struct CreateAccountAction {}
}
```
```

If `receiver_id` has length == 64, this account id is considered to be `hex(public_key)`, meaning creation of account only succeeds if followed up with `AddKey(public_key)` action.

**Outcome**:

* creates an account with `id` = `receiver_id`
* sets Account `storage_usage` to `account_cost` (genesis config)

### [Errors](#errors)

**Execution Error**:

* If the action tries to create a top level account whose length is no greater than 32 characters, and `predecessor_id` is not
  `registrar_account_id`, which is defined by the protocol, the following error will be returned

```
```
#![allow(unused)]
fn main() {
/// A top-level account ID can only be created by registrar.
CreateAccountOnlyByRegistrar {
    account_id: AccountId,
    registrar_account_id: AccountId,
    predecessor_id: AccountId,
}
}
```
```

* If the action tries to create an account that is neither a top-level account or a subaccount of `predecessor_id`,
  the following error will be returned

```
```
#![allow(unused)]
fn main() {
/// A newly created account must be under a namespace of the creator account
CreateAccountNotAllowed { account_id: AccountId, predecessor_id: AccountId },
}
```
```

[DeployContractAction](#deploycontractaction)
---------------------------------------------

```
```
#![allow(unused)]
fn main() {
pub struct DeployContractAction {
    pub code: Vec<u8>
}
}
```
```

**Outcome**:

* sets the contract code for account

### [Errors](#errors-1)

**Validation Error**:

* if the length of `code` exceeds `max_contract_size`, which is a genesis parameter, the following error will be returned:

```
```
#![allow(unused)]
fn main() {
/// The size of the contract code exceeded the limit in a DeployContract action.
ContractSizeExceeded { size: u64, limit: u64 },
}
```
```

**Execution Error**:

* If state or storage is corrupted, it may return `StorageError`.

[FunctionCallAction](#functioncallaction)
-----------------------------------------

```
```
#![allow(unused)]
fn main() {
pub struct FunctionCallAction {
    /// Name of exported Wasm function
    pub method_name: String,
    /// Serialized arguments
    pub args: Vec<u8>,
    /// Prepaid gas (gas_limit) for a function call
    pub gas: Gas,
    /// Amount of tokens to transfer to a receiver_id
    pub deposit: Balance,
}
}
```
```

Calls a method of a particular contract. See [details](./FunctionCall.html).

[TransferAction](#transferaction)
---------------------------------

```
```
#![allow(unused)]
fn main() {
pub struct TransferAction {
    /// Amount of tokens to transfer to a receiver_id
    pub deposit: Balance,
}
}
```
```

**Outcome**:

* transfers amount specified in `deposit` from `predecessor_id` to a `receiver_id` account

### [Errors](#errors-2)

**Execution Error**:

* If the deposit amount plus the existing amount on the receiver account exceeds `u128::MAX`,
  a `StorageInconsistentState("Account balance integer overflow")` error will be returned.

[StakeAction](#stakeaction)
---------------------------

```
```
#![allow(unused)]
fn main() {
pub struct StakeAction {
    // Amount of tokens to stake
    pub stake: Balance,
    // This public key is a public key of the validator node
    pub public_key: PublicKey,
}
}
```
```

**Outcome**:

* A validator proposal that contains the staking public key and the staking amount is generated and will be included
  in the next block.

### [Errors](#errors-3)

**Validation Error**:

* If the `public_key` is not an ristretto compatible ed25519 key, the following error will be returned:

```
```
#![allow(unused)]
fn main() {
/// An attempt to stake with a public key that is not convertible to ristretto.
UnsuitableStakingKey { public_key: PublicKey },
}
```
```

**Execution Error**:

* If an account has not staked but it tries to unstake, the following error will be returned:

```
```
#![allow(unused)]
fn main() {
/// Account is not yet staked, but tries to unstake
TriesToUnstake { account_id: AccountId },
}
```
```

* If an account tries to stake more than the amount of tokens it has, the following error will be returned:

```
```
#![allow(unused)]
fn main() {
/// The account doesn't have enough balance to increase the stake.
TriesToStake {
    account_id: AccountId,
    stake: Balance,
    locked: Balance,
    balance: Balance,
}
}
```
```

* If the staked amount is below the minimum stake threshold, the following error will be returned:

```
```
#![allow(unused)]
fn main() {
InsufficientStake {
    account_id: AccountId,
    stake: Balance,
    minimum_stake: Balance,
}
}
```
```

The minimum stake is determined by `last_epoch_seat_price / minimum_stake_divisor` where `last_epoch_seat_price` is the
seat price determined at the end of last epoch and `minimum_stake_divisor` is a genesis config parameter and its current
value is 10.

[AddKeyAction](#addkeyaction)
-----------------------------

```
```
#![allow(unused)]
fn main() {
pub struct AddKeyAction {
    pub public_key: PublicKey,
    pub access_key: AccessKey,
}
}
```
```

**Outcome**:

* Adds a new [AccessKey](/DataStructures/AccessKey.html) to the receiver's account and associates it with a `public_key` provided.

### [Errors](#errors-4)

**Validation Error**:

If the access key is of type `FunctionCallPermission`, the following errors can happen

* If `receiver_id` in `access_key` is not a valid account id, the following error will be returned

```
```
#![allow(unused)]
fn main() {
/// Invalid account ID.
InvalidAccountId { account_id: AccountId },
}
```
```

* If the length of some method name exceed `max_length_method_name`, which is a genesis parameter (current value is 256),
  the following error will be returned

```
```
#![allow(unused)]
fn main() {
/// The length of some method name exceeded the limit in a Add Key action.
AddKeyMethodNameLengthExceeded { length: u64, limit: u64 },
}
```
```

* If the sum of length of method names (with 1 extra character for every method name) exceeds `max_number_bytes_method_names`, which is a genesis parameter (current value is 2000),
  the following error will be returned

```
```
#![allow(unused)]
fn main() {
/// The total number of bytes of the method names exceeded the limit in a Add Key action.
AddKeyMethodNamesNumberOfBytesExceeded { total_number_of_bytes: u64, limit: u64 }
}
```
```

**Execution Error**:

* If an account tries to add an access key with a given public key, but an existing access key with this public key already exists, the following error will be returned

```
```
#![allow(unused)]
fn main() {
/// The public key is already used for an existing access key
AddKeyAlreadyExists { account_id: AccountId, public_key: PublicKey }
}
```
```

* If state or storage is corrupted, a `StorageError` will be returned.

[DeleteKeyAction](#deletekeyaction)
-----------------------------------

```
```
#![allow(unused)]
fn main() {
pub struct DeleteKeyAction {
    pub public_key: PublicKey,
}
}
```
```

**Outcome**:

* Deletes the [AccessKey](/DataStructures/AccessKey.html) associated with `public_key`.

### [Errors](#errors-5)

**Execution Error**:

* When an account tries to delete an access key that doesn't exist, the following error is returned

```
```
#![allow(unused)]
fn main() {
/// Account tries to remove an access key that doesn't exist
DeleteKeyDoesNotExist { account_id: AccountId, public_key: PublicKey }
}
```
```

* `StorageError` is returned if state or storage is corrupted.

[DeleteAccountAction](#deleteaccountaction)
-------------------------------------------

```
```
#![allow(unused)]
fn main() {
pub struct DeleteAccountAction {
    /// The remaining account balance will be transferred to the AccountId below
    pub beneficiary_id: AccountId,
}
}
```
```

**Outcomes**:

* The account, as well as all the data stored under the account, is deleted and the tokens are transferred to `beneficiary_id`.

### [Errors](#errors-6)

**Validation Error**:

* If `beneficiary_id` is not a valid account id, the following error will be returned

```
```
#![allow(unused)]
fn main() {
/// Invalid account ID.
InvalidAccountId { account_id: AccountId },
}
```
```

* If this action is not the last action in the action list of a receipt, the following error will be returned

```
```
#![allow(unused)]
fn main() {
/// The delete action must be a final action in transaction
DeleteActionMustBeFinal
}
```
```

* If the account still has locked balance due to staking, the following error will be returned

```
```
#![allow(unused)]
fn main() {
/// Account is staking and can not be deleted
DeleteAccountStaking { account_id: AccountId }
}
```
```

**Execution Error**:

* If state or storage is corrupted, a `StorageError` is returned.

[Delegate Actions](#delegate-actions)
-------------------------------------

Introduced with [NEP-366](https://github.com/near/NEPs/blob/master/neps/nep-0366.md) to enable meta transactions.

In summary, a delegate action is an indirect submission of a transaction.
It allows a relayer to do the payment (gas and token costs) for a transaction authored by a user.

```
```
#![allow(unused)]
fn main() {
/// The struct contained in transactions and receipts, inside `Action::Delegate(_)``.
struct SignedDelegateAction {
    /// The actual action, see below.
    pub delegate_action: DelegateAction,
    /// NEP-483 proposal compliant signature
    pub signature: Signature,
}
}
```
```

Note that the signature follows a scheme which is proposed to be standardized in [NEP-483](https://github.com/near/NEPs/pull/483).

```
```
#![allow(unused)]
fn main() {
/// The struct a user creates and signs to create a meta transaction.
struct DelegateAction {
    /// Signer of the delegated actions
    pub sender_id: AccountId,
    /// Receiver of the delegated actions.
    pub receiver_id: AccountId,
    /// List of actions to be executed.
    ///
    /// With the meta transactions MVP defined in NEP-366, nested
    /// DelegateActions are not allowed. A separate type is used to enforce it.
    pub actions: Vec<NonDelegateAction>,
    /// Nonce to ensure that the same delegate action is not sent twice by a
    /// relayer and should match for given account's `public_key`.
    /// After this action is processed it will increment.
    pub nonce: Nonce,
    /// The maximal height of the block in the blockchain below which the given DelegateAction is valid.
    pub max_block_height: BlockHeight,
    /// Public key used to sign this delegated action.
    pub public_key: PublicKey,
}
}
```
```

### [Outcomes](#outcomes)

* All actions inside `delegate_action.actions` are submitted with the `delegate_action.sender_id` as the predecessor, `delegate_action.receiver_id` as the receiver, and the relayer (predecessor of `DelegateAction`) as the signer.
* All gas and balance costs for submitting `delegate_action.actions` are subtracted from the relayer.

### [Errors](#errors-7)

**Validation Error**:

* If the list of Transaction actions contains several `DelegateAction`

```
```
#![allow(unused)]
fn main() {
/// There should be the only one DelegateAction
DelegateActionMustBeOnlyOne
}
```
```

**Execution Error**:

* If the Sender's account doesn't exist

```
```
#![allow(unused)]
fn main() {
/// Happens when TX receiver_id doesn't exist
AccountDoesNotExist
}
```
```

* If the `signature` does not match the data and the `public_key` of the given key, then the following error will be returned

```
```
#![allow(unused)]
fn main() {
/// Signature does not match the provided actions and given signer public key.
DelegateActionInvalidSignature
}
```
```

* If the `sender_id` doesn't match the `tx.receiver_id`

```
```
#![allow(unused)]
fn main() {
/// Receiver of the transaction doesn't match Sender of the delegate action
DelegateActionSenderDoesNotMatchTxReceiver
}
```
```

* If the current block is equal or greater than `max_block_height`

```
```
#![allow(unused)]
fn main() {
/// Delegate action has expired
DelegateActionExpired
}
```
```

* If the `public_key` does not exist for Sender account

```
```
#![allow(unused)]
fn main() {
/// The given public key doesn't exist for Sender account
DelegateActionAccessKeyError
}
```
```

* If the `nonce` does match the `public_key` for the `sender_id`

```
```
#![allow(unused)]
fn main() {
/// Nonce must be greater sender[public_key].nonce
DelegateActionInvalidNonce
}
```
```

* If `nonce` is too large

```
```
#![allow(unused)]
fn main() {
/// DelegateAction nonce is larger than the upper bound given by the block height (block_height * 1e6)
DelegateActionNonceTooLarge
}
```
```

[DeployGlobalContractAction](#deployglobalcontractaction)
---------------------------------------------------------

```
```
#![allow(unused)]
fn main() {
pub struct DeployGlobalContractAction {
    /// WebAssembly binary
    pub code: Vec<u8>,
    /// How this global contract should be referenced
    pub deploy_mode: GlobalContractDeployMode,
}

pub enum GlobalContractDeployMode {
    /// Contract is deployed under its code hash.
    /// Users will be able reference it by that hash.
    /// This effectively makes the contract immutable.
    CodeHash,
    /// Contract is deployed under the owner account id.
    /// Users will be able reference it by that account id.
    /// This allows the owner to update the contract for all its users.
    AccountId,
}
}
```
```

**Outcome**:

* First, the provided code is made available as global contract on the current shard.
* The same code propagates globally, shard by shard.
* Eventually, all accounts on all shards can reference the submitted code by the corresponding global contract identifier.

### [Errors](#errors-8)

**Validation Error**:

* `ContractSizeExceeded` if the provided WebAssembly code is larger than `max_contract_size` (4MiB).

**Execution Error**:

* `LackBalanceForState` if the account does not hold enough NEAR to cover the added storage.

[UseGlobalContractAction](#useglobalcontractaction)
---------------------------------------------------

```
```
#![allow(unused)]
fn main() {
pub struct UseGlobalContractAction {
    /// References a deployed global contract to use in the receiver account.
    pub contract_identifier: GlobalContractIdentifier,
}
}
```
```

**Outcome**:

### [Errors](#errors-9)

**Validation Error**:

* `InvalidAccountId` if the provided account id does not follow the [AccountId specification](../DataStructures/Account.html).

**Execution Error**:

* `GlobalContractDoesNotExist` if the referenced global contract does not exist on the shard of the receiver. (It may
  take a while for it to propagate to all shards.)

[DeterministicStateInitAction](#deterministicstateinitaction)
-------------------------------------------------------------

```
```
#![allow(unused)]
fn main() {
pub struct DeterministicStateInitAction {
    /// The data required to initialize the account.
    pub state_init: DeterministicAccountStateInit,
    /// A NEAR balance to cover storage requirements. Extra balance is refunded.
    pub deposit: Balance,
}
}
```
```

**Outcome**:

* if the account was already created before:
  + do nothing
* if the account was not created before:
  + creates an account with deterministic account id
  + sets the contract code to the specified global contract
  + stores the initial data into the contract storage

### [Errors](#errors-10)

**Validation Error**:

* `InvalidDeterministicStateInitReceiver` if the receiver id is not derived from the provided `DeterministicAccountStateInit`
* `DeterministicStateInitKeyLengthExceeded` if any data key is longer than `max_length_storage_key` (4MiB)
* `DeterministicStateInitValueLengthExceeded` if any data value is longer than `max_length_storage_value` (4MiB)

**Execution Error**:

* `GlobalContractDoesNotExist` if the referenced global contract does not exist on the shard of the receiver. (It may
  take a while for it to propagate to all shards.)