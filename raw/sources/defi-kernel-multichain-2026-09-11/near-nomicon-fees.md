1. [Introduction](../../index.html)
2. - Protocol Specification
   - [Data Structures](../../DataStructures/index.html)
   - 1. [Access Keys](../../DataStructures/AccessKey.html)
     2. [Accounts](../../DataStructures/Account.html)
     3. [Block and Block Header](../../DataStructures/Block.html)
     4. [Data Types](../../DataStructures/DataTypes.html)
     5. [Merkle Proofs](../../DataStructures/MerkleProof.html)
     6. [Transaction](../../DataStructures/Transaction.html)
   - [Chain Specification](../../ChainSpec/index.html)
   - 1. [Block Processing](../../ChainSpec/BlockProcessing.html)
     2. [Consensus](../../ChainSpec/Consensus.html)
     3. [Light Client](../../ChainSpec/LightClient.html)
     4. [Selecting Chunk and Block Producers](../../ChainSpec/SelectingBlockProducers.html)
     5. [Transactions in the Blockchain Layer](../../ChainSpec/Transactions.html)
     6. [Upgradability](../../ChainSpec/Upgradability.html)
     7. [Epochs and Staking](../../ChainSpec/EpochAndStaking/index.html)
     8. 1. [Epoch](../../ChainSpec/EpochAndStaking/Epoch.html)
        2. [EpochManager](../../ChainSpec/EpochAndStaking/EpochManager.html)
        3. [Staking and slashing](../../ChainSpec/EpochAndStaking/Staking.html)
   - [Network Specification](../../NetworkSpec/NetworkSpec.html)
   - 1. [Messages](../../NetworkSpec/Messages.html)
     2. [Routing Table Exchange Algorithm](../../NetworkSpec/RoutingTableExchangeAlgorithm.html)
   - [Runtime Specification](../../RuntimeSpec/index.html)
   - 1. [Account Receipt Storage](../../RuntimeSpec/AccountStorage.html)
     2. [Actions](../../RuntimeSpec/Actions.html)
     3. [Applying chunk](../../RuntimeSpec/ApplyingChunk.html)
     4. [Components](../../RuntimeSpec/Components/Components.html)
     5. 1. [Runtime Crate](../../RuntimeSpec/Components/RuntimeCrate.html)
        2. [Bindings Specification](../../RuntimeSpec/Components/BindingsSpec/BindingsSpec.html)
        3. 1. [Context API](../../RuntimeSpec/Components/BindingsSpec/ContextAPI.html)
           2. [Economics API](../../RuntimeSpec/Components/BindingsSpec/EconomicsAPI.html)
           3. [Math API](../../RuntimeSpec/Components/BindingsSpec/MathAPI.html)
           4. [Miscellaneous API](../../RuntimeSpec/Components/BindingsSpec/MiscellaneousAPI.html)
           5. [Promises API](../../RuntimeSpec/Components/BindingsSpec/PromisesAPI.html)
           6. [Registers API](../../RuntimeSpec/Components/BindingsSpec/RegistersAPI.html)
           7. [Trie API](../../RuntimeSpec/Components/BindingsSpec/TrieAPI.html)
     6. [Fees](../../RuntimeSpec/Fees/Fees.html)
     7. [Function Call](../../RuntimeSpec/FunctionCall.html)
     8. [Contract Preparation](../../RuntimeSpec/Preparation.html)
     9. [Receipts](../../RuntimeSpec/Receipts.html)
     10. [Refunds](../../RuntimeSpec/Refunds.html)
     11. [Runtime](../../RuntimeSpec/Runtime.html)
     12. [Transactions](../../RuntimeSpec/Transactions.html)
     13. [Scenarios](../../RuntimeSpec/Scenarios/Scenarios.html)
     14. 1. [Financial Transaction](../../RuntimeSpec/Scenarios/FinancialTransaction.html)
         2. [Cross-ContractCall](../../RuntimeSpec/Scenarios/CrossContractCall.html)
   - [Economics](../../Economics/Economics.html)
   - [GenesisConfig](../../GenesisConfig/GenesisConfig.html)
   - 1. [ExtCostsConfig](../../GenesisConfig/ExtCostsConfig.html)
     2. [VMConfig](../../GenesisConfig/VMConfig.html)
     3. [StateRecord](../../GenesisConfig/StateRecord.html)
     4. [RuntimeConfig](../../GenesisConfig/RuntimeConfig.html)
     5. [RuntimeFeeConfig](../../GenesisConfig/RuntimeFeeConfig.html)
     6. 1. [AccessKeyCreationConfig](../../GenesisConfig/RuntimeFeeConfig/AccessKeyCreationConfig.html)
        2. [ActionCreationConfig](../../GenesisConfig/RuntimeFeeConfig/ActionCreationConfig.html)
        3. [DataReceiptCreationConfig](../../GenesisConfig/RuntimeFeeConfig/DataReceiptCreationConfig.html)
        4. [Fee](../../GenesisConfig/RuntimeFeeConfig/Fee.html)
        5. [Fraction](../../GenesisConfig/RuntimeFeeConfig/Fraction.html)
        6. [StorageUsageConfig](../../GenesisConfig/RuntimeFeeConfig/StorageUsageConfig.html)
   - - Architecture
     - [Overview](../../architecture/index.html)
     - [How neard works](../../architecture/how/index.html)
     - 1. [How Sync Works](../../architecture/how/sync.html)
       2. [Garbage Collection](../../architecture/how/gc.html)
       3. [How Epoch Works](../../architecture/how/epoch.html)
       4. [Transaction Routing](../../architecture/how/tx_routing.html)
       5. [Transactions And Receipts](../../architecture/how/tx_receipts.html)
       6. [Cross shard transactions - deep dive](../../architecture/how/cross-shard.html)
       7. [Gas](../../architecture/how/gas.html)
       8. [Receipt Congestion](../../architecture/how/receipt-congestion.html)
       9. [Meta transactions](../../architecture/how/meta-tx.html)
       10. [Serialization: Borsh, Json, ProtoBuf](../../architecture/how/serialization.html)
       11. [Proofs](../../architecture/how/proofs.html)
       12. [Resharding V2](../../architecture/how/resharding_v2.html)
       13. [Optimistic block](../../architecture/how/optimistic_block.html)
     - [How neard will work](../../architecture/next/index.html)
     - 1. [Catchup and state sync improvements](../../architecture/next/catchup_and_state_sync.html)
       2. [Malicious producers and phase 2](../../architecture/next/malicious_chunk_producer_and_phase2.html)
     - [Storage](../../architecture/storage.html)
     - 1. [Storage Request Flow](../../architecture/storage/flow.html)
       2. [Trie Storage](../../architecture/storage/trie_storage.html)
       3. [Database Format](../../architecture/storage/database.html)
       4. [Flat Storage](../../architecture/storage/flat_storage.html)
     - [Network](../../architecture/network.html)
     - [Gas Cost Parameters](../../architecture/gas/index.html)
     - 1. [Parameter Definitions](../../architecture/gas/parameter_definition.html)
       2. [Gas Profile](../../architecture/gas/gas_profile.html)
       3. [Runtime Parameter Estimator](../../architecture/gas/estimator.html)
     - - Practices
       - [Overview](../../practices/index.html)
       - [Rust 🦀](../../practices/rust.html)
       - [Workflows](../../practices/workflows/index.html)
       - 1. [Run a Node](../../practices/workflows/run_a_node.html)
         2. [Deploy a Contract](../../practices/workflows/deploy_a_contract.html)
         3. [Run Gas Estimations](../../practices/workflows/gas_estimations.html)
         4. [Localnet on many machines](../../practices/workflows/localnet_on_many_machines.html)
         5. [IO tracing](../../practices/workflows/io_trace.html)
         6. [Profiling](../../practices/workflows/profiling.html)
         7. [Benchmarks](../../practices/workflows/benchmarks.html)
         8. 1. [Synthetic Workloads](../../practices/workflows/benchmarking_synthetic_workloads.html)
            2. [Native Transfer Chunk Application](../../practices/workflows/benchmarking_chunk_application_on_native_transfers.html)
         9. [Working with OpenTelemetry Traces](../../practices/workflows/otel_traces.html)
         10. [Futex contention](../../practices/workflows/futex_contention.html)
       - [Code Style](../../practices/style.html)
       - [Documentation](../../practices/docs.html)
       - [Tracking Issues](../../practices/tracking_issues.html)
       - [Security Vulnerabilities](../../practices/security_vulnerabilities.html)
       - [Fast Builds](../../practices/fast_builds.html)
       - [Testing](../../practices/testing/index.html)
       - 1. [Python Tests](../../practices/testing/python_tests.html)
         2. [Testing Utils](../../practices/testing/test_utils.html)
         3. [Test Coverage](../../practices/testing/coverage.html)
       - [Protocol Upgrade](../../practices/protocol_upgrade.html)
       - - Advanced configuration
         - [Networking](../../advanced_configuration/networking.html)
         - - Custom test networks
           - [Forknet scenarios](../../../pytest/tests/mocknet/docs/forknet_scenario.html)
           - [Mirror tool](../../../pytest/tests/mocknet/docs/mirror.html)
           - - Misc
             - [Overview](../../misc/index.html)
             - [State Sync Dump](../../misc/state_sync_dump.html)
             - [Archival node - recovery of missing data](../../misc/archival_data_recovery.html)
             - [Grafana MCP Setup for Claude Code](../../grafana-mcp-setup.html)
             - [Zulip MCP Setup for Claude Code](../../zulip-mcp-setup.html)

* Light (default)
* Rust
* Coal
* Navy
* Ayu

Guide to Nearcore Development
=============================

[Runtime Fees](#runtime-fees)
=============================

Runtime fees are measured in Gas. Gas price will be discussed separately.

When a transaction is converted into a receipt, the signer account is charged for the full cost of the transaction.
This cost consists of extra attached gas, attached deposits and the transaction fee.

The total transaction fee is the sum of the following:

* A fee for creation of the receipt
* A fee for every action

Every [Fee](/GenesisConfig/RuntimeFeeConfig/Fee.html) consists of 3 values measured in gas:

* `send_sir` and `send_not_sir` - the gas burned when the action is being created to be sent to a receiver.
  + `send_sir` is used when `current_account_id == receiver_id` (`current_account_id` is a `signer_id` for a signed transaction).
  + `send_not_sir` is used when `current_account_id != receiver_id`
* `execution` - the gas burned when the action is being executed on the receiver's account.

[Receipt creation cost](#receipt-creation-cost)
-----------------------------------------------

There are 2 types of receipts:

* Action receipts [ActionReceipt](/RuntimeSpec/Receipts.html#actionreceipt)
* Data receipts [DataReceipt](/RuntimeSpec/Receipts.html#datareceipt)

A transaction is converted into an [ActionReceipt](/RuntimeSpec/Receipts.html#actionreceipt).
Data receipts are used for data dependencies and will be discussed separately.

The `Fee` for an action receipt creation is described in the config [`action_receipt_creation_config`](/GenesisConfig/RuntimeFeeConfig.html#action_receipt_creation_config).

Example: when a signed transaction is being converted into a receipt, the gas for `action_receipt_creation_config.send` is being burned immediately,
while the gas for `action_receipt_creation_config.execution` is only charged, but not burned. It'll be burned when
the newly created receipt is executed on the receiver's account.

[Fees for actions](#fees-for-actions)
-------------------------------------

Every [`Action`](/RuntimeSpec/Actions) has a corresponding Fee(s) described in the config [`action_creation_config`](/GenesisConfig/RuntimeFeeConfig/ActionCreationConfig.html).
Similar to a receipt creation costs, the `send` gas is burned when an action is added to a receipt to be sent, and the `execution` gas is only charged, but not burned.

Fees are either a base fee or a fee per byte of some data within the action.

Here is the list of actions and their corresponding fees:

* [CreateAccount](/RuntimeSpec/Actions.html#createaccountaction) uses
  + the base fee [`create_account_cost`](/GenesisConfig/RuntimeFeeConfig/ActionCreationConfig.html#create_account_cost)
* [DeployContract](/RuntimeSpec/Actions.html#deploycontractaction) uses the sum of the following fees:
  + the base fee [`deploy_contract_cost`](/GenesisConfig/RuntimeFeeConfig/ActionCreationConfig.html#deploy_contract_cost)
  + the fee per byte of the contract code to be deployed with the fee [`deploy_contract_cost_per_byte`](/GenesisConfig/RuntimeFeeConfig/ActionCreationConfig.html#deploy_contract_cost_per_byte)
    To compute the number of bytes for a deploy contract action `deploy_contract_action` use `deploy_contract_action.code.len()`
* [FunctionCall](/RuntimeSpec/Actions.html#functioncallaction) uses the sum of the following fees:
  + the base fee [`function_call_cost`](/GenesisConfig/RuntimeFeeConfig/ActionCreationConfig.html#function_call_cost)
  + the fee per byte of method name string and per byte of arguments with the fee [`function_call_cost_per_byte`](/GenesisConfig/RuntimeFeeConfig/ActionCreationConfig.html#function_call_cost_per_byte).
    To compute the number of bytes for a function call action `function_call_action` use `function_call_action.method_name.as_bytes().len() + function_call_action.args.len()`
* [Transfer](/RuntimeSpec/Actions.html#transferaction) uses one of the following fees:
  + if the `receiver_id` is an [Implicit Account ID](/DataStructures/Account#near-implicit-account-id), then a sum of base fees is used:
    - the create account base fee [`create_account_cost`](/GenesisConfig/RuntimeFeeConfig/ActionCreationConfig.html#create_account_cost)
    - the transfer base fee [`transfer_cost`](/GenesisConfig/RuntimeFeeConfig/ActionCreationConfig.html#transfer_cost)
    - the add full access key base fee [`add_key_cost.full_access_cost`](/GenesisConfig/RuntimeFeeConfig/AccessKeyCreationConfig.html#full_access_cost)
  + if the `receiver_id` is NOT an [Implicit Account ID](/DataStructures/Account.html#near-implicit-account-id), then only the base fee is used:
    - the transfer base fee [`transfer_cost`](/GenesisConfig/RuntimeFeeConfig/ActionCreationConfig.html#transfer_cost)
* [Stake](/RuntimeSpec/Actions.html#stakeaction) uses
  + the base fee [`stake_cost`](/GenesisConfig/RuntimeFeeConfig/ActionCreationConfig.html#stake_cost)
* [AddKey](/RuntimeSpec/Actions.html#addkeyaction) uses one of the following fees:
  + if the access key is [`AccessKeyPermission::FullAccess`](/DataStructures/AccessKey) the base fee is used
    - the add full access key base fee [`add_key_cost.full_access_cost`](/GenesisConfig/RuntimeFeeConfig/AccessKeyCreationConfig.html#full_access_cost)
  + if the access key is [`AccessKeyPermission::FunctionCall`](/DataStructures/AccessKey.html#accesskeypermissionfunctioncall) the sum of the fees is used
    - the add function call permission access key base fee [`add_key_cost.function_call_cost`](/GenesisConfig/RuntimeFeeConfig/AccessKeyCreationConfig.html#full_access_cost)
    - the fee per byte of method names with extra byte for every method with the fee [`add_key_cost.function_call_cost_per_byte`](/GenesisConfig/RuntimeFeeConfig/AccessKeyCreationConfig.html#function_call_cost_per_byte)
      To compute the number of bytes for `function_call_permission` use `function_call_permission.method_names.iter().map(|name| name.as_bytes().len() as u64 + 1).sum::<u64>()`
* [DeleteKey](/RuntimeSpec/Actions.html#deletekeyaction) uses
  + the base fee [`delete_key_cost`](/GenesisConfig/RuntimeFeeConfig/ActionCreationConfig.html#delete_key_cost)
* [DeleteAccount](/RuntimeSpec/Actions.html#deleteaccountaction) uses
  + the base fee [`delete_account_cost`](/GenesisConfig/RuntimeFeeConfig/ActionCreationConfig.html#delete_account_cost)
  + action receipt creation fee for creating Transfer to send remaining funds to `beneficiary_id`
  + full transfer fee described in the corresponding item

[Gas tracking](#gas-tracking)
-----------------------------

In `Runtime`, gas is tracked in the following fields of `ActionResult` struct:

* `gas_burnt` - irreversible amount of gas spent on computations.
* `gas_used` - includes burnt gas and gas attached to the new `ActionReceipt`s created during the method execution.
* `gas_burnt_for_function_call` - stores gas burnt during function call execution. Later, contract account gets 30% of it as a reward for a possibility to invoke the function.

Initially runtime charges `gas_used` from the account. Some gas may be refunded later, see [Refunds](../Refunds.html).

At first, we charge fees related to conversion from `SignedTransaction` to `ActionReceipt` and future execution of this receipt:

* costs of all `SignedTransaction`s passed to `Runtime::apply` are computed in `tx_cost` function during validation;
* `total_cost` is deducted from signer, which is a sum of:
  + `gas_to_balance(gas_burnt)` where `gas_burnt` is action receipt send fee + `total_send_fees(transaction.actions)`);
  + `gas_to_balance(gas_remaining)` where `gas_remaining` is action receipt exec fee + `total_prepaid_exec_fees(transaction.actions)` to pay all remaining fees caused by transaction;
  + `total_deposit(transaction.actions)`;
* each transaction is converted to receipt and passed to `Runtime::process_receipt`.

Then each `ActionReceipt` is passed to `Runtime::apply_action_receipt` where gas is tracked as follows:

* `ActionResult` is created with `ActionReceipt` execution fee;
* all actions inside `ActionReceipt` are passed to `Runtime::apply_action`;
* `ActionResult` with charged base execution fees is created there;
* if action execution leads to new `ActionReceipt`s creation, corresponding `action_[action_name]` function adds new fees to the `ActionResult`. E.g. `action_delete_account` also charges the following fees:
  + `gas_burnt`: **send** fee for **new** `ActionReceipt` creation + complex **send** fee for `Transfer` to beneficiary account
  + `gas_used`: `gas_burnt` + **exec** fee for created `ActionReceipt` + complex **exec** fee for `Transfer`
* all computed `ActionResult`s are merged into one, where all gas values are summed up;
* unused gas is refunded in `generate_refund_receipts`, after subtracting the gas refund fee, see [Refunds](../Refunds.html).

Inside `VMLogic`, the fees are tracked in the `GasCounter` struct.
The VM itself is called in the `action_function_call` inside `Runtime`. When all actions are processed, the result is returned as a `VMOutcome`, which is later merged with `ActionResult`.

[Example](#example)
===================

Let's say we have the following transaction:

```
```
#![allow(unused)]
fn main() {
Transaction {
    signer_id: "alice.near",
    public_key: "2onVGYTFwyaGetWckywk92ngBiZeNpBeEjuzSznEdhRE",
    nonce: 23,
    receiver_id: "lockup.alice.near",
    block_hash: "3CwEMonK6MmKgjKePiFYgydbAvxhhqCPHKuDMnUcGGTK",
    actions: [
        Action::CreateAccount(CreateAccountAction {}),
        Action::Transfer(TransferAction {
            deposit: 100000000000000000000000000,
        }),
        Action::DeployContract(DeployContractAction {
            code: vec![/*<...128000 bytes...>*/],
        }),
        Action::FunctionCall(FunctionCallAction {
            method_name: "new",
            args: b"{\"owner_id\": \"alice.near\"}".to_vec(),
            gas: 25000000000000,
            deposit: 0,
        }),
    ],
}
}
```
```

It has `signer_id != receiver_id` so it will use `send_not_sir` for send fees.

It contains 4 actions with 2 actions that requires to compute number of bytes.
We assume `code` in `DeployContractAction` contains `128000` bytes. And `FunctionCallAction` has
`method_name` with length of `3` and `args` length of `26`, so total of `29`.

First let's compute the amount that will be burned immediately for sending a receipt.

```
burnt_gas = \
    config.action_receipt_creation_config.send_not_sir + \
    config.action_creation_config.create_account_cost.send_not_sir + \
    config.action_creation_config.transfer_cost.send_not_sir + \
    config.action_creation_config.deploy_contract_cost.send_not_sir + \
    128000 * config.action_creation_config.deploy_contract_cost_per_byte.send_not_sir + \
    config.action_creation_config.function_call_cost.send_not_sir + \
    29 * config.action_creation_config.function_call_cost_per_byte.send_not_sir
```

Now, by using `burnt_gas`, we can calculate the total transaction fee

```
total_transaction_fee = burnt_gas + \
    config.action_receipt_creation_config.execution + \
    config.action_creation_config.create_account_cost.execution + \
    config.action_creation_config.transfer_cost.execution + \
    config.action_creation_config.deploy_contract_cost.execution + \
    128000 * config.action_creation_config.deploy_contract_cost_per_byte.execution + \
    config.action_creation_config.function_call_cost.execution + \
    29 * config.action_creation_config.function_call_cost_per_byte.execution
```

This `total_transaction_fee` is the amount of gas required to create a new receipt from the transaction.

NOTE: There are extra amounts required to prepay for deposit in `TransferAction` and gas in `FunctionCallAction`, but this is not part of the total transaction fee.