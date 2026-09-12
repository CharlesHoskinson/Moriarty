[Skip to content](#account-permission-management)

java-tron

Account Permission Management



Initializing search

[tronprotocol/documentation-en](https://github.com/tronprotocol/documentation-en "Go to repository")

java-tron

[tronprotocol/documentation-en](https://github.com/tronprotocol/documentation-en "Go to repository")

* [Home](../..)
* Getting Started With java-tron



  Getting Started With java-tron
  + [Getting Started](../../getting_started/getting_started_with_javatron/)
* Using java-tron



  Using java-tron
  + [Deploying](../../using_javatron/installing_javatron/)
  + [Node Configuration](../../using_javatron/configuration/)
  + [Backup and Restore](../../using_javatron/backup_restore/)
  + [Lite FullNode](../../using_javatron/litefullnode/)
  + [Private Network](../../using_javatron/private_network/)
  + [Event Subscription](../../architecture/event/)
  + [Database Configuration](../../architecture/database/)
  + [Network Connection](../../using_javatron/connecting_to_tron/)
  + [Node Logging](../../using_javatron/logging/)
  + [Node Monitoring](../../using_javatron/metrics/)
  + [Node Maintenance Tool](../../using_javatron/toolkit/)
* [API](../../api/)

  API
  + [HTTP API](../../api/http/)

    HTTP API
    - Account



      Account
      * [Query an account by address](../../api/http/account/getaccount/)
      * [Query an account's balance at a specific block](../../api/http/account/getaccountbalance/)
      * [Query an account's bandwidth resources](../../api/http/account/getaccountnet/)
      * [Query bandwidth + energy + TronPower](../../api/http/account/getaccountresource/)
      * [Create an on-chain account](../../api/http/account/createaccount/)
      * [Update an account's name](../../api/http/account/updateaccount/)
      * [Configure multi-sig permissions](../../api/http/account/accountpermissionupdate/)
      * [Validate an address](../../api/http/account/validateaddress/)
    - Block / transaction query



      Block / transaction query
      * [Latest block](../../api/http/block-and-tx-query/getnowblock/)
      * [Generic block query](../../api/http/block-and-tx-query/getblock/)
      * [Block by height](../../api/http/block-and-tx-query/getblockbynum/)
      * [Block by hash](../../api/http/block-and-tx-query/getblockbyid/)
      * [Blocks in a range](../../api/http/block-and-tx-query/getblockbylimitnext/)
      * [The most recent N blocks](../../api/http/block-and-tx-query/getblockbylatestnum/)
      * [Per-account balance changes within a block](../../api/http/block-and-tx-query/getblockbalance/)
      * [Transaction count in a block](../../api/http/block-and-tx-query/gettransactioncountbyblocknum/)
      * [Transaction by txid](../../api/http/block-and-tx-query/gettransactionbyid/)
      * [Transaction receipt by txid](../../api/http/block-and-tx-query/gettransactioninfobyid/)
      * [Transaction receipts by block](../../api/http/block-and-tx-query/gettransactioninfobyblocknum/)
      * [Pending pool size](../../api/http/block-and-tx-query/getpendingsize/)
      * [Single pending transaction](../../api/http/block-and-tx-query/gettransactionfrompending/)
      * [All pending transaction IDs](../../api/http/block-and-tx-query/gettransactionlistfrompending/)
    - Transaction build / broadcast



      Transaction build / broadcast
      * [Build a TRX transfer transaction](../../api/http/tx-build-and-broadcast/createtransaction/)
      * [Current multi-sig weight](../../api/http/tx-build-and-broadcast/getsignweight/)
      * [Addresses that have already signed](../../api/http/tx-build-and-broadcast/getapprovedlist/)
      * [Broadcast a signed transaction (JSON)](../../api/http/tx-build-and-broadcast/broadcasttransaction/)
      * [Broadcast a signed transaction (hex)](../../api/http/tx-build-and-broadcast/broadcasthex/)
    - TRC10 asset



      TRC10 asset
      * [Issue a TRC10 token](../../api/http/asset/createassetissue/)
      * [Update a TRC10 description](../../api/http/asset/updateasset/)
      * [Transfer TRC10](../../api/http/asset/transferasset/)
      * [Participate in a TRC10 fundraising](../../api/http/asset/participateassetissue/)
      * [Unfreeze TRC10 frozen by the issuer](../../api/http/asset/unfreezeasset/)
      * [Look up a TRC10 by id](../../api/http/asset/getassetissuebyid/)
      * [Look up a TRC10 by name](../../api/http/asset/getassetissuebyname/)
      * [All TRC10s with a given name](../../api/http/asset/getassetissuelistbyname/)
      * [TRC10s issued by an account](../../api/http/asset/getassetissuebyaccount/)
      * [All TRC10s on the network](../../api/http/asset/getassetissuelist/)
      * [Paginated TRC10 list](../../api/http/asset/getpaginatedassetissuelist/)
    - Smart contract



      Smart contract
      * [Deploy a contract](../../api/http/smart-contract/deploycontract/)
      * [Trigger a contract (write)](../../api/http/smart-contract/triggersmartcontract/)
      * [Read-only contract call](../../api/http/smart-contract/triggerconstantcontract/)
      * [Estimate energy usage of a call](../../api/http/smart-contract/estimateenergy/)
      * [Contract metadata](../../api/http/smart-contract/getcontract/)
      * [Full contract runtime info](../../api/http/smart-contract/getcontractinfo/)
      * [Clear a contract's ABI](../../api/http/smart-contract/clearabi/)
      * [Change the user-energy percentage](../../api/http/smart-contract/updatesetting/)
      * [Change the deployer's energy limit](../../api/http/smart-contract/updateenergylimit/)
    - Witness / governance



      Witness / governance
      * [Apply to become an SR candidate](../../api/http/witness-and-governance/createwitness/)
      * [Update an SR's URL](../../api/http/witness-and-governance/updatewitness/)
      * [All SR candidates](../../api/http/witness-and-governance/listwitnesses/)
      * [Paginated SR list](../../api/http/witness-and-governance/getpaginatednowwitnesslist/)
      * [Vote for SRs](../../api/http/witness-and-governance/votewitnessaccount/)
      * [An SR's current brokerage rate](../../api/http/witness-and-governance/getBrokerage/)
      * [Update an SR's brokerage](../../api/http/witness-and-governance/updateBrokerage/)
      * [Claimable rewards for an account](../../api/http/witness-and-governance/getReward/)
      * [Withdraw block production rewards / dividends](../../api/http/witness-and-governance/withdrawbalance/)
      * [Create a chain-parameter proposal](../../api/http/witness-and-governance/proposalcreate/)
      * [Vote on a proposal as an SR](../../api/http/witness-and-governance/proposalapprove/)
      * [Withdraw your own proposal](../../api/http/witness-and-governance/proposaldelete/)
      * [List of proposals](../../api/http/witness-and-governance/listproposals/)
      * [Proposal by ID](../../api/http/witness-and-governance/getproposalbyid/)
      * [Paginated proposal list](../../api/http/witness-and-governance/getpaginatedproposallist/)
      * [Current chain parameters](../../api/http/witness-and-governance/getchainparameters/)
      * [Next maintenance period time](../../api/http/witness-and-governance/getnextmaintenancetime/)
    - Stake 1.0 (unfreeze and query only)



      Stake 1.0 (unfreeze and query only)
      * [Freeze TRX for resources (V1)](../../api/http/stake-v1/freezebalance/)
      * [Unfreeze matured resources (V1)](../../api/http/stake-v1/unfreezebalance/)
      * [Query delegation records (V1)](../../api/http/stake-v1/getdelegatedresource/)
      * [Query delegation counterparty addresses (V1)](../../api/http/stake-v1/getdelegatedresourceaccountindex/)
    - Stake 2.0



      Stake 2.0
      * [Freeze TRX for resources](../../api/http/stake-v2/freezebalancev2/)
      * [Initiate unfreeze](../../api/http/stake-v2/unfreezebalancev2/)
      * [Withdraw matured unfreezes](../../api/http/stake-v2/withdrawexpireunfreeze/)
      * [Cancel all unmatured unfreezes](../../api/http/stake-v2/cancelallunfreezev2/)
      * [Delegate resources to another account](../../api/http/stake-v2/delegateresource/)
      * [Undelegate resources from another account](../../api/http/stake-v2/undelegateresource/)
      * [Query delegation records](../../api/http/stake-v2/getdelegatedresourcev2/)
      * [Query delegation counterparty addresses](../../api/http/stake-v2/getdelegatedresourceaccountindexv2/)
      * [Current maximum delegatable amount](../../api/http/stake-v2/getcandelegatedmaxsize/)
      * [Remaining unfreeze count](../../api/http/stake-v2/getavailableunfreezecount/)
      * [Withdrawable unfreeze amount at a given time](../../api/http/stake-v2/getcanwithdrawunfreezeamount/)
    - Node / pricing / tools



      Node / pricing / tools
      * [Node status](../../api/http/node-and-tools/getnodeinfo/)
      * [Known peers](../../api/http/node-and-tools/listnodes/)
      * [Historical energy unit prices](../../api/http/node-and-tools/getenergyprices/)
      * [Historical bandwidth unit prices](../../api/http/node-and-tools/getbandwidthprices/)
      * [Cumulative burned TRX](../../api/http/node-and-tools/getburntrx/)
  + [gRPC API](../../api/rpc/)

    gRPC API
    - Account



      Account
      * [Query an account by address](../../api/rpc/account/GetAccount/)
      * [Query an account's balance at a specific block](../../api/rpc/account/GetAccountBalance/)
      * [Query an account's bandwidth resources](../../api/rpc/account/GetAccountNet/)
      * [Query bandwidth + energy + TronPower](../../api/rpc/account/GetAccountResource/)
      * [Create an on-chain account](../../api/rpc/account/CreateAccount2/)
      * [Update an account's name](../../api/rpc/account/UpdateAccount2/)
      * [Configure multi-sig permissions](../../api/rpc/account/AccountPermissionUpdate/)
    - Block / transaction query



      Block / transaction query
      * [Latest block](../../api/rpc/block-and-tx-query/GetNowBlock2/)
      * [Generic block query](../../api/rpc/block-and-tx-query/GetBlock/)
      * [Block by height](../../api/rpc/block-and-tx-query/GetBlockByNum2/)
      * [Block by hash](../../api/rpc/block-and-tx-query/GetBlockById/)
      * [Blocks in a range](../../api/rpc/block-and-tx-query/GetBlockByLimitNext2/)
      * [The most recent N blocks](../../api/rpc/block-and-tx-query/GetBlockByLatestNum2/)
      * [Per-account balance changes within a block](../../api/rpc/block-and-tx-query/GetBlockBalanceTrace/)
      * [Transaction count in a block](../../api/rpc/block-and-tx-query/GetTransactionCountByBlockNum/)
      * [Transaction by txid](../../api/rpc/block-and-tx-query/GetTransactionById/)
      * [Transaction receipt by txid](../../api/rpc/block-and-tx-query/GetTransactionInfoById/)
      * [Transaction receipts by block](../../api/rpc/block-and-tx-query/GetTransactionInfoByBlockNum/)
      * [Pending pool size](../../api/rpc/block-and-tx-query/GetPendingSize/)
      * [Single pending transaction](../../api/rpc/block-and-tx-query/GetTransactionFromPending/)
      * [All pending transaction IDs](../../api/rpc/block-and-tx-query/GetTransactionListFromPending/)
    - Transaction build / broadcast



      Transaction build / broadcast
      * [Build a TRX transfer transaction](../../api/rpc/tx-build-and-broadcast/CreateTransaction2/)
      * [Current multi-sig weight](../../api/rpc/tx-build-and-broadcast/GetTransactionSignWeight/)
      * [Addresses that have already signed](../../api/rpc/tx-build-and-broadcast/GetTransactionApprovedList/)
      * [Broadcast a signed transaction](../../api/rpc/tx-build-and-broadcast/BroadcastTransaction/)
    - TRC10 asset



      TRC10 asset
      * [Issue a TRC10 token](../../api/rpc/asset/CreateAssetIssue2/)
      * [Update a TRC10 description](../../api/rpc/asset/UpdateAsset2/)
      * [Transfer TRC10](../../api/rpc/asset/TransferAsset2/)
      * [Participate in a TRC10 fundraising](../../api/rpc/asset/ParticipateAssetIssue2/)
      * [Unfreeze TRC10 frozen by the issuer](../../api/rpc/asset/UnfreezeAsset2/)
      * [Look up a TRC10 by id](../../api/rpc/asset/GetAssetIssueById/)
      * [Look up a TRC10 by name](../../api/rpc/asset/GetAssetIssueByName/)
      * [All TRC10s with a given name](../../api/rpc/asset/GetAssetIssueListByName/)
      * [TRC10s issued by an account](../../api/rpc/asset/GetAssetIssueByAccount/)
      * [All TRC10s on the network](../../api/rpc/asset/GetAssetIssueList/)
      * [Paginated TRC10 list](../../api/rpc/asset/GetPaginatedAssetIssueList/)
    - Smart contract



      Smart contract
      * [Deploy a contract](../../api/rpc/smart-contract/DeployContract/)
      * [Trigger a contract (write)](../../api/rpc/smart-contract/TriggerContract/)
      * [Read-only contract call](../../api/rpc/smart-contract/TriggerConstantContract/)
      * [Estimate energy usage of a call](../../api/rpc/smart-contract/EstimateEnergy/)
      * [Contract metadata](../../api/rpc/smart-contract/GetContract/)
      * [Full contract runtime info](../../api/rpc/smart-contract/GetContractInfo/)
      * [Clear a contract's ABI](../../api/rpc/smart-contract/ClearContractABI/)
      * [Change the user-energy percentage](../../api/rpc/smart-contract/UpdateSetting/)
      * [Change the deployer's energy limit](../../api/rpc/smart-contract/UpdateEnergyLimit/)
    - Witness / governance



      Witness / governance
      * [Apply to become an SR candidate](../../api/rpc/witness-and-governance/CreateWitness2/)
      * [Update an SR's URL](../../api/rpc/witness-and-governance/UpdateWitness2/)
      * [All SR candidates](../../api/rpc/witness-and-governance/ListWitnesses/)
      * [Paginated SR list](../../api/rpc/witness-and-governance/GetPaginatedNowWitnessList/)
      * [Vote for SRs](../../api/rpc/witness-and-governance/VoteWitnessAccount2/)
      * [An SR's current brokerage rate](../../api/rpc/witness-and-governance/GetBrokerageInfo/)
      * [Update an SR's brokerage](../../api/rpc/witness-and-governance/UpdateBrokerage/)
      * [Claimable rewards for an account](../../api/rpc/witness-and-governance/GetRewardInfo/)
      * [Withdraw block production rewards / dividends](../../api/rpc/witness-and-governance/WithdrawBalance2/)
      * [Create a chain-parameter proposal](../../api/rpc/witness-and-governance/ProposalCreate/)
      * [Vote on a proposal as an SR](../../api/rpc/witness-and-governance/ProposalApprove/)
      * [Withdraw your own proposal](../../api/rpc/witness-and-governance/ProposalDelete/)
      * [List of proposals](../../api/rpc/witness-and-governance/ListProposals/)
      * [Proposal by ID](../../api/rpc/witness-and-governance/GetProposalById/)
      * [Paginated proposal list](../../api/rpc/witness-and-governance/GetPaginatedProposalList/)
      * [Current chain parameters](../../api/rpc/witness-and-governance/GetChainParameters/)
      * [Next maintenance period time](../../api/rpc/witness-and-governance/GetNextMaintenanceTime/)
    - Stake 1.0 (unfreeze and query only)



      Stake 1.0 (unfreeze and query only)
      * [Freeze TRX for resources (V1)](../../api/rpc/stake-v1/FreezeBalance2/)
      * [Unfreeze matured resources (V1)](../../api/rpc/stake-v1/UnfreezeBalance2/)
      * [Query delegation records (V1)](../../api/rpc/stake-v1/GetDelegatedResource/)
      * [Query delegation counterparty addresses (V1)](../../api/rpc/stake-v1/GetDelegatedResourceAccountIndex/)
    - Stake 2.0



      Stake 2.0
      * [Freeze TRX for resources](../../api/rpc/stake-v2/FreezeBalanceV2/)
      * [Initiate unfreeze](../../api/rpc/stake-v2/UnfreezeBalanceV2/)
      * [Withdraw matured unfreezes](../../api/rpc/stake-v2/WithdrawExpireUnfreeze/)
      * [Cancel all unmatured unfreezes](../../api/rpc/stake-v2/CancelAllUnfreezeV2/)
      * [Delegate resources to another account](../../api/rpc/stake-v2/DelegateResource/)
      * [Undelegate resources from another account](../../api/rpc/stake-v2/UnDelegateResource/)
      * [Query delegation records](../../api/rpc/stake-v2/GetDelegatedResourceV2/)
      * [Query delegation counterparty addresses](../../api/rpc/stake-v2/GetDelegatedResourceAccountIndexV2/)
      * [Current maximum delegatable amount](../../api/rpc/stake-v2/GetCanDelegatedMaxSize/)
      * [Remaining unfreeze count](../../api/rpc/stake-v2/GetAvailableUnfreezeCount/)
      * [Withdrawable unfreeze amount at a given time](../../api/rpc/stake-v2/GetCanWithdrawUnfreezeAmount/)
    - Node / pricing / tools



      Node / pricing / tools
      * [Node status](../../api/rpc/node-and-tools/GetNodeInfo/)
      * [Known peers](../../api/rpc/node-and-tools/ListNodes/)
      * [Historical energy unit prices](../../api/rpc/node-and-tools/GetEnergyPrices/)
      * [Historical bandwidth unit prices](../../api/rpc/node-and-tools/GetBandwidthPrices/)
      * [Cumulative burned TRX](../../api/rpc/node-and-tools/GetBurnTrx/)
  + [jsonRPC API](../../api/json-rpc/)

    jsonRPC API
    - Node info / chain identity



      Node info / chain identity
      * [Client version string](../../api/json-rpc/node/web3_clientVersion/)
      * [Keccak-256 hash](../../api/json-rpc/node/web3_sha3/)
      * [Network ID](../../api/json-rpc/node/net_version/)
      * [Whether listening on P2P](../../api/json-rpc/node/net_listening/)
      * [Number of peers](../../api/json-rpc/node/net_peerCount/)
      * [Chain ID](../../api/json-rpc/node/eth_chainId/)
      * [Protocol version](../../api/json-rpc/node/eth_protocolVersion/)
      * [Sync status](../../api/json-rpc/node/eth_syncing/)
      * [Latest block height](../../api/json-rpc/node/eth_blockNumber/)
      * [Current energy unit price](../../api/json-rpc/node/eth_gasPrice/)
    - Block / transaction query



      Block / transaction query
      * [Query a block by hash](../../api/json-rpc/block-and-tx-query/eth_getBlockByHash/)
      * [Query a block by height / tag](../../api/json-rpc/block-and-tx-query/eth_getBlockByNumber/)
      * [Block transaction count (by hash)](../../api/json-rpc/block-and-tx-query/eth_getBlockTransactionCountByHash/)
      * [Block transaction count (by height)](../../api/json-rpc/block-and-tx-query/eth_getBlockTransactionCountByNumber/)
      * [Query a transaction by txid](../../api/json-rpc/block-and-tx-query/eth_getTransactionByHash/)
      * [Query a transaction by block hash + index](../../api/json-rpc/block-and-tx-query/eth_getTransactionByBlockHashAndIndex/)
      * [Query a transaction by block height + index](../../api/json-rpc/block-and-tx-query/eth_getTransactionByBlockNumberAndIndex/)
      * [Query a receipt by txid](../../api/json-rpc/block-and-tx-query/eth_getTransactionReceipt/)
      * [Receipt list for an entire block](../../api/json-rpc/block-and-tx-query/eth_getBlockReceipts/)
    - Account state



      Account state
      * [Account TRX balance](../../api/json-rpc/account/eth_getBalance/)
      * [Contract storage slot](../../api/json-rpc/account/eth_getStorageAt/)
      * [Contract runtime bytecode](../../api/json-rpc/account/eth_getCode/)
    - Smart contract calls



      Smart contract calls
      * [Read-only contract call](../../api/json-rpc/smart-contract/eth_call/)
      * [Estimate energy consumption](../../api/json-rpc/smart-contract/eth_estimateGas/)
    - Logs / filters



      Logs / filters
      * [One-shot log query](../../api/json-rpc/filter/eth_getLogs/)
      * [Register a log filter](../../api/json-rpc/filter/eth_newFilter/)
      * [Register a new-block filter](../../api/json-rpc/filter/eth_newBlockFilter/)
      * [Uninstall a filter](../../api/json-rpc/filter/eth_uninstallFilter/)
      * [Pull and drain filter increments](../../api/json-rpc/filter/eth_getFilterChanges/)
      * [Pull a log filter's full set](../../api/json-rpc/filter/eth_getFilterLogs/)
    - Transaction build



      Transaction build
      * [Build an unsigned transaction](../../api/json-rpc/tx-build/buildTransaction/)
    - Compatibility stub methods



      Compatibility stub methods
      * [etherbase address](../../api/json-rpc/stub/eth_coinbase/)
      * [Node-managed accounts list](../../api/json-rpc/stub/eth_accounts/)
      * [Mining work info](../../api/json-rpc/stub/eth_getWork/)
  + [Machine-readable definitions](../../api/interface-definitions/)
* Core Protocol



  Core Protocol
  + [DPoS](../dpos/)
  + [Super Representative](../sr/)
  + [Account Model](../account/)
  + [Resource Model](../resource/)
  + [Smart Contract](../../contracts/contract/)
  + [System Contract](../system-contracts/)
  + Account Permission Management

    [Account Permission Management](./)


    Table of contents
    - [Function Overview](#function-overview)
    - [Permission Level Concepts](#permission-level-concepts)
    - [Permission Structure Definition](#permission-structure-definition)

      * [1. Account Structure: Account](#1-account-structure-account)
      * [2. Permission Configuration: Permission](#2-permission-configuration-permission)
      * [3. Permission Key Structure: Key](#3-permission-key-structure-key)
      * [4. Permission Update Transaction: AccountPermissionUpdateContract](#4-permission-update-transaction-accountpermissionupdatecontract)
      * [5. Contract Type Enumeration: ContractType](#5-contract-type-enumeration-contracttype)
    - [Explanation of Each Permission Type](#explanation-of-each-permission-type)

      * [owner Permission (Account Master Control)](#owner-permission-account-master-control)
      * [witness Permission (Block Production Permission)](#witness-permission-block-production-permission)
      * [Active Permission (Functional Permission Combination)](#active-permission-functional-permission-combination)
    - [Fees and Constraints](#fees-and-constraints)
    - [Interfaces and Operation Examples](#interfaces-and-operation-examples)

      * [1. Permission Modification Operation Process](#1-permission-modification-operation-process)
      * [2. Operations Value Calculation Example](#2-operations-value-calculation-example)
      * [3. Transaction Execution Process](#3-transaction-execution-process)
    - [Auxiliary Interfaces](#auxiliary-interfaces)

      * [Query Signed Addresses](#query-signed-addresses)
      * [Query Signature Weight](#query-signature-weight)
    - [References](#references)
* For java-tron Developers



  For java-tron Developers
  + [Developer Guide](../../developers/java-tron/)
  + [CI Workflows](../../developers/workflows/)
  + [Core Modules](../../developers/code-structure/)
  + [ChainBase Deep Dive](../../developers/chainbase/)
  + [P2P Network Deep Dive](../../developers/network/)
  + [TIPs Workflow](../../developers/tip-workflow/)
  + [TIPs](../../developers/tips/)
  + [Issue Workflow](../../developers/issue-workflow/)
  + [Governance Workflow](../../developers/governance/)
  + [Configure the IDE](../../developers/run-in-idea/)
  + [Development Example](../../developers/demo/)
* For Dapp Developers



  For Dapp Developers
  + [Tools](../../contracts/tools/)
* Clients



  Clients
  + [wallet-cli](../../clients/wallet-cli/)

    wallet-cli
    - [Java CLI](../../clients/wallet-cli/java/)

      Java CLI
      * [Guides](../../clients/wallet-cli/java/guide/)

        Guides
        + [Getting started](../../clients/wallet-cli/java/guide/getting-started/)
        + [Command-line operation flow](../../clients/wallet-cli/java/guide/command-flow/)
      * [Concepts](../../clients/wallet-cli/java/concepts/)

        Concepts
        + [Resources: bandwidth, energy & shares](../../clients/wallet-cli/java/concepts/resources/)
        + [Staking models: Stake 1.0 vs 2.0](../../clients/wallet-cli/java/concepts/staking-models/)
        + [Multi-signature concepts](../../clients/wallet-cli/java/concepts/multisig/)
      * [Command reference](../../clients/wallet-cli/java/commands/)

        Command reference
        + [Wallet management](../../clients/wallet-cli/java/commands/wallet/)
        + [Account commands](../../clients/wallet-cli/java/commands/account/)
        + [Network commands](../../clients/wallet-cli/java/commands/network/)
        + [TRC10 tokens](../../clients/wallet-cli/java/commands/transfer-trc10/)
        + [USDT & TRC20 transfers](../../clients/wallet-cli/java/commands/usdt/)
        + [Staking (Stake 2.0)](../../clients/wallet-cli/java/commands/stake-v2/)
        + [Staking (Stake 1.0, legacy)](../../clients/wallet-cli/java/commands/stake-v1-legacy/)
        + [Resource prices & memo fee](../../clients/wallet-cli/java/commands/resources/)
        + [Voting, rewards & witnesses](../../clients/wallet-cli/java/commands/vote-reward/)
        + [Proposals](../../clients/wallet-cli/java/commands/proposals/)
        + [Exchange (Bancor)](../../clients/wallet-cli/java/commands/exchange/)
        + [TRON-DEX market](../../clients/wallet-cli/java/commands/dex/)
        + [Multi-signature](../../clients/wallet-cli/java/commands/multisig/)
        + [Smart contracts](../../clients/wallet-cli/java/commands/contract/)
        + [GasFree transfers](../../clients/wallet-cli/java/commands/gasfree/)
        + [Chain data & utilities](../../clients/wallet-cli/java/commands/chain-data/)
      * [Configuration reference](../../clients/wallet-cli/java/reference/config/)
    - [TypeScript / npm CLI](../../clients/wallet-cli/typescript/)

      TypeScript / npm CLI
      * [Guides](../../clients/wallet-cli/typescript/guide/)

        Guides
        + [Getting Started](../../clients/wallet-cli/typescript/guide/getting-started/)
        + [Sending TRX and Tokens](../../clients/wallet-cli/typescript/guide/send-tokens/)
        + [Staking and Resources](../../clients/wallet-cli/typescript/guide/stake-and-resources/)
        + [Using a Ledger Hardware Wallet](../../clients/wallet-cli/typescript/guide/ledger/)
        + [Scripting wallet-cli](../../clients/wallet-cli/typescript/guide/scripting/)
      * [Concepts](../../clients/wallet-cli/typescript/concepts/)

        Concepts
        + [Networks](../../clients/wallet-cli/typescript/concepts/networks/)
        + [Accounts and HD Wallets](../../clients/wallet-cli/typescript/concepts/accounts-and-hd/)
        + [Energy and Bandwidth](../../clients/wallet-cli/typescript/concepts/energy-bandwidth/)
        + [Security Model](../../clients/wallet-cli/typescript/concepts/security/)
      * [Command reference](../../clients/wallet-cli/typescript/commands/)

        Command reference
        + [create](../../clients/wallet-cli/typescript/commands/create/)
        + [list](../../clients/wallet-cli/typescript/commands/list/)
        + [use](../../clients/wallet-cli/typescript/commands/use/)
        + [current](../../clients/wallet-cli/typescript/commands/current/)
        + [derive](../../clients/wallet-cli/typescript/commands/derive/)
        + [rename](../../clients/wallet-cli/typescript/commands/rename/)
        + [backup](../../clients/wallet-cli/typescript/commands/backup/)
        + [delete](../../clients/wallet-cli/typescript/commands/delete/)
        + [change-password](../../clients/wallet-cli/typescript/commands/change-password/)
        + [block](../../clients/wallet-cli/typescript/commands/block/)
        + [networks](../../clients/wallet-cli/typescript/commands/networks/)
        + [config](../../clients/wallet-cli/typescript/commands/config/)
        + [account](../../clients/wallet-cli/typescript/commands/account/)

          account
          - [account activate](../../clients/wallet-cli/typescript/commands/account/activate/)
          - [account balance](../../clients/wallet-cli/typescript/commands/account/balance/)
          - [account history](../../clients/wallet-cli/typescript/commands/account/history/)
          - [account info](../../clients/wallet-cli/typescript/commands/account/info/)
          - [account portfolio](../../clients/wallet-cli/typescript/commands/account/portfolio/)
          - [account set](../../clients/wallet-cli/typescript/commands/account/set/)
        + [address](../../clients/wallet-cli/typescript/commands/address/)

          address
          - [address generate](../../clients/wallet-cli/typescript/commands/address/generate/)
        + [asset](../../clients/wallet-cli/typescript/commands/asset/)

          asset
          - [asset info](../../clients/wallet-cli/typescript/commands/asset/info/)
          - [asset issue](../../clients/wallet-cli/typescript/commands/asset/issue/)
          - [asset list](../../clients/wallet-cli/typescript/commands/asset/list/)
          - [asset participate](../../clients/wallet-cli/typescript/commands/asset/participate/)
          - [asset unfreeze](../../clients/wallet-cli/typescript/commands/asset/unfreeze/)
          - [asset update](../../clients/wallet-cli/typescript/commands/asset/update/)
        + [chain](../../clients/wallet-cli/typescript/commands/chain/)

          chain
          - [chain node](../../clients/wallet-cli/typescript/commands/chain/node/)
          - [chain params](../../clients/wallet-cli/typescript/commands/chain/params/)
          - [chain prices](../../clients/wallet-cli/typescript/commands/chain/prices/)
        + [contact](../../clients/wallet-cli/typescript/commands/contact/)

          contact
          - [contact add](../../clients/wallet-cli/typescript/commands/contact/add/)
          - [contact list](../../clients/wallet-cli/typescript/commands/contact/list/)
          - [contact remove](../../clients/wallet-cli/typescript/commands/contact/remove/)
        + [contract](../../clients/wallet-cli/typescript/commands/contract/)

          contract
          - [contract call](../../clients/wallet-cli/typescript/commands/contract/call/)
          - [contract clear-abi](../../clients/wallet-cli/typescript/commands/contract/clear-abi/)
          - [contract create2](../../clients/wallet-cli/typescript/commands/contract/create2/)
          - [contract deploy](../../clients/wallet-cli/typescript/commands/contract/deploy/)
          - [contract info](../../clients/wallet-cli/typescript/commands/contract/info/)
          - [contract send](../../clients/wallet-cli/typescript/commands/contract/send/)
          - [contract set-origin-energy-limit](../../clients/wallet-cli/typescript/commands/contract/set-origin-energy-limit/)
          - [contract set-user-resource-percent](../../clients/wallet-cli/typescript/commands/contract/set-user-resource-percent/)
        + [encoding](../../clients/wallet-cli/typescript/commands/encoding/)

          encoding
          - [encoding convert](../../clients/wallet-cli/typescript/commands/encoding/convert/)
        + [exchange](../../clients/wallet-cli/typescript/commands/exchange/)

          exchange
          - [exchange create](../../clients/wallet-cli/typescript/commands/exchange/create/)
          - [exchange inject](../../clients/wallet-cli/typescript/commands/exchange/inject/)
          - [exchange list](../../clients/wallet-cli/typescript/commands/exchange/list/)
          - [exchange show](../../clients/wallet-cli/typescript/commands/exchange/show/)
          - [exchange trade](../../clients/wallet-cli/typescript/commands/exchange/trade/)
          - [exchange withdraw](../../clients/wallet-cli/typescript/commands/exchange/withdraw/)
        + [gasfree](../../clients/wallet-cli/typescript/commands/gasfree/)

          gasfree
          - [gasfree info](../../clients/wallet-cli/typescript/commands/gasfree/info/)
          - [gasfree trace](../../clients/wallet-cli/typescript/commands/gasfree/trace/)
          - [gasfree transfer](../../clients/wallet-cli/typescript/commands/gasfree/transfer/)
        + [import](../../clients/wallet-cli/typescript/commands/import/)

          import
          - [import keystore](../../clients/wallet-cli/typescript/commands/import/keystore/)
          - [import ledger](../../clients/wallet-cli/typescript/commands/import/ledger/)
          - [import mnemonic](../../clients/wallet-cli/typescript/commands/import/mnemonic/)
          - [import private-key](../../clients/wallet-cli/typescript/commands/import/private-key/)
          - [import watch](../../clients/wallet-cli/typescript/commands/import/watch/)
        + [message](../../clients/wallet-cli/typescript/commands/message/)

          message
          - [message sign](../../clients/wallet-cli/typescript/commands/message/sign/)
        + [permission](../../clients/wallet-cli/typescript/commands/permission/)

          permission
          - [permission show](../../clients/wallet-cli/typescript/commands/permission/show/)
          - [permission update](../../clients/wallet-cli/typescript/commands/permission/update/)
        + [proposal](../../clients/wallet-cli/typescript/commands/proposal/)

          proposal
          - [proposal approve](../../clients/wallet-cli/typescript/commands/proposal/approve/)
          - [proposal create](../../clients/wallet-cli/typescript/commands/proposal/create/)
          - [proposal delete](../../clients/wallet-cli/typescript/commands/proposal/delete/)
          - [proposal list](../../clients/wallet-cli/typescript/commands/proposal/list/)
          - [proposal show](../../clients/wallet-cli/typescript/commands/proposal/show/)
        + [reward](../../clients/wallet-cli/typescript/commands/reward/)

          reward
          - [reward balance](../../clients/wallet-cli/typescript/commands/reward/balance/)
          - [reward withdraw](../../clients/wallet-cli/typescript/commands/reward/withdraw/)
        + [stake](../../clients/wallet-cli/typescript/commands/stake/)

          stake
          - [stake cancel-unfreeze](../../clients/wallet-cli/typescript/commands/stake/cancel-unfreeze/)
          - [stake delegate](../../clients/wallet-cli/typescript/commands/stake/delegate/)
          - [stake delegated](../../clients/wallet-cli/typescript/commands/stake/delegated/)
          - [stake freeze](../../clients/wallet-cli/typescript/commands/stake/freeze/)
          - [stake info](../../clients/wallet-cli/typescript/commands/stake/info/)
          - [stake undelegate](../../clients/wallet-cli/typescript/commands/stake/undelegate/)
          - [stake unfreeze](../../clients/wallet-cli/typescript/commands/stake/unfreeze/)
          - [stake withdraw](../../clients/wallet-cli/typescript/commands/stake/withdraw/)
        + [token](../../clients/wallet-cli/typescript/commands/token/)

          token
          - [token add](../../clients/wallet-cli/typescript/commands/token/add/)
          - [token balance](../../clients/wallet-cli/typescript/commands/token/balance/)
          - [token info](../../clients/wallet-cli/typescript/commands/token/info/)
          - [token list](../../clients/wallet-cli/typescript/commands/token/list/)
          - [token remove](../../clients/wallet-cli/typescript/commands/token/remove/)
        + [tx](../../clients/wallet-cli/typescript/commands/tx/)

          tx
          - [tx approvals](../../clients/wallet-cli/typescript/commands/tx/approvals/)
          - [tx broadcast](../../clients/wallet-cli/typescript/commands/tx/broadcast/)
          - [tx info](../../clients/wallet-cli/typescript/commands/tx/info/)
          - [tx multisig](../../clients/wallet-cli/typescript/commands/tx/multisig/)
          - [tx send](../../clients/wallet-cli/typescript/commands/tx/send/)
          - [tx sign](../../clients/wallet-cli/typescript/commands/tx/sign/)
          - [tx status](../../clients/wallet-cli/typescript/commands/tx/status/)
        + [typed-data](../../clients/wallet-cli/typescript/commands/typed-data/)

          typed-data
          - [typed-data sign](../../clients/wallet-cli/typescript/commands/typed-data/sign/)
        + [vote](../../clients/wallet-cli/typescript/commands/vote/)

          vote
          - [vote cast](../../clients/wallet-cli/typescript/commands/vote/cast/)
          - [vote list](../../clients/wallet-cli/typescript/commands/vote/list/)
          - [vote status](../../clients/wallet-cli/typescript/commands/vote/status/)
        + [witness](../../clients/wallet-cli/typescript/commands/witness/)

          witness
          - [witness create](../../clients/wallet-cli/typescript/commands/witness/create/)
          - [witness set-brokerage](../../clients/wallet-cli/typescript/commands/witness/set-brokerage/)
          - [witness update](../../clients/wallet-cli/typescript/commands/witness/update/)
      * [Machine Interface](../../clients/wallet-cli/typescript/machine-interface/)
      * [Troubleshooting](../../clients/wallet-cli/typescript/troubleshooting/)
* Releases



  Releases
  + [Deployment Manual for the New Version](../../releases/upgrade-instruction/)
  + [Integrity Check](../../releases/signature_verification/)
  + [History](../../releases/versions/)

    History
    - [v4.8.2 (Pyrrho)](../../releases/versions/v4.8.2/)
    - [v4.8.1 (Democritus)](../../releases/versions/v4.8.1/)
    - [v4.8.0.1 (Seneca)](../../releases/versions/v4.8.0.1/)
    - [v4.8.0 (Kant)](../../releases/versions/v4.8.0/)
    - [v4.7.7 (Epicurus)](../../releases/versions/v4.7.7/)
    - [v4.7.6 (Anaximander)](../../releases/versions/v4.7.6/)
    - [v4.7.5 (Cleobulus)](../../releases/versions/v4.7.5/)
    - [v4.7.4 (Bias)](../../releases/versions/v4.7.4/)
    - [v4.7.3.1 (Solon)](../../releases/versions/v4.7.3.1/)
    - [v4.7.3 (Chilon)](../../releases/versions/v4.7.3/)
    - [v4.7.2 (Periander)](../../releases/versions/v4.7.2/)
    - [v4.7.1.1 (Pittacus)](../../releases/versions/v4.7.1.1/)
    - [v4.7.1 (Sartre)](../../releases/versions/v4.7.1/)
    - [v4.7.0.1 (Aristotle)](../../releases/versions/v4.7.0.1/)
    - [v4.6.0 (Socrates)](../../releases/versions/v4.6.0/)
    - [v4.5.2 (Aurelius)](../../releases/versions/v4.5.2/)
    - [v4.5.1 (Tertullian)](../../releases/versions/v4.5.1/)
    - [v4.4.6 (David)](../../releases/versions/v4.4.6/)
    - [v4.4.5 (Cicero)](../../releases/versions/v4.4.5/)
    - [v4.4.4 (Plotinus)](../../releases/versions/v4.4.4/)
    - [v4.4.2 (Augustinus)](../../releases/versions/v4.4.2/)
    - [v4.4.0 (Rousseau)](../../releases/versions/v4.4.0/)
    - [v4.3.0 (Bacon)](../../releases/versions/v4.3.0/)
    - [v4.2.2.1 (Epictetus)](../../releases/versions/v4.2.2.1/)
    - [v4.2.2 (Lucretius)](../../releases/versions/v4.2.2/)
    - [v4.2.0 (Plato)](../../releases/versions/v4.2.0/)
    - [v4.1.3 (Thales)](../../releases/versions/v4.1.3/)
    - [v4.1.2](../../releases/versions/v4.1.2/)
    - [v4.1.1](../../releases/versions/v4.1.1/)
    - [v4.0.0](../../releases/versions/v4.0.0/)
    - [Odyssey-v3.7](../../releases/versions/v3.7/)
    - [Odyssey-v3.6.5](../../releases/versions/v3.6.5/)
* Appendix



  Appendix
  + [Glossary](../../glossary/)

Table of contents

* [Function Overview](#function-overview)
* [Permission Level Concepts](#permission-level-concepts)
* [Permission Structure Definition](#permission-structure-definition)

  + [1. Account Structure: Account](#1-account-structure-account)
  + [2. Permission Configuration: Permission](#2-permission-configuration-permission)
  + [3. Permission Key Structure: Key](#3-permission-key-structure-key)
  + [4. Permission Update Transaction: AccountPermissionUpdateContract](#4-permission-update-transaction-accountpermissionupdatecontract)
  + [5. Contract Type Enumeration: ContractType](#5-contract-type-enumeration-contracttype)
* [Explanation of Each Permission Type](#explanation-of-each-permission-type)

  + [owner Permission (Account Master Control)](#owner-permission-account-master-control)
  + [witness Permission (Block Production Permission)](#witness-permission-block-production-permission)
  + [Active Permission (Functional Permission Combination)](#active-permission-functional-permission-combination)
* [Fees and Constraints](#fees-and-constraints)
* [Interfaces and Operation Examples](#interfaces-and-operation-examples)

  + [1. Permission Modification Operation Process](#1-permission-modification-operation-process)
  + [2. Operations Value Calculation Example](#2-operations-value-calculation-example)
  + [3. Transaction Execution Process](#3-transaction-execution-process)
* [Auxiliary Interfaces](#auxiliary-interfaces)

  + [Query Signed Addresses](#query-signed-addresses)
  + [Query Signature Weight](#query-signature-weight)
* [References](#references)

Account Permission Management[¶](#account-permission-management "Permanent link")
=================================================================================

The TRON network supports fine-grained control of account permissions. By configuring permissions (owner, witness, active), joint control of accounts, secure delegation, and functional permission separation can be achieved. The following document details the account permission model, contract structure, configuration methods, and common interface calls.

Function Overview[¶](#function-overview "Permanent link")
---------------------------------------------------------

Account permission management allows:

* Setting permission levels for an account.
* Associating each permission with a group of addresses and weights.
* Implementing permission control through a threshold mechanism.
* Flexibly configuring which addresses can execute which contract types.

For detailed specifications, see [TIP-16: Account Permission Management](https://github.com/tronprotocol/tips/blob/master/tip-16.md).

Permission Level Concepts[¶](#permission-level-concepts "Permanent link")
-------------------------------------------------------------------------

TRON supports three types of permission:

| Permission Type | Description |
| --- | --- |
| owner | Highest account permission, controls ownership and permission structure |
| witness | Super Representative permission, used only for block production |
| active | Custom permission, can specify functional permission combinations |

---

Permission Structure Definition[¶](#permission-structure-definition "Permanent link")
-------------------------------------------------------------------------------------

### 1. Account Structure: `Account`[¶](#1-account-structure-account "Permanent link")

```
message Account {
  ...
  Permission owner_permission = 31;
  Permission witness_permission = 32;
  repeated Permission active_permission = 33;
}
```

Explanation:

* `owner_permission`: `owner` permission (only one).
* `witness_permission`: Super Representative permission (only one).
* `active_permission`: `active` permission list, supports up to 8.

### 2. Permission Configuration: `Permission`[¶](#2-permission-configuration-permission "Permanent link")

```
message Permission {
  enum PermissionType {
    Owner = 0;
    Witness = 1;
    Active = 2;
  }
  PermissionType type = 1;
  int32 id = 2;
  string permission_name = 3;
  int64 threshold = 4;
  int32 parent_id = 5;
  bytes operations = 6;
  repeated Key keys = 7;
}
```

Explanation:

* `type`: Permission type (owner/witness/active).
* `id`: Permission ID, automatically assigned by the system.
  + `owner` = 0, `witness` = 1, `active` starts from 2 and increments.
* `permission_name`: Permission name, maximum 32 characters.
* `threshold`: Permission threshold, the operation is authorized only when the cumulative weight of valid signatures meets or exceeds this value.
* `operations`: Used only for Active permissions, specifies executable contract types.
* `keys`: Addresses and weights with this permission (up to 5).

### 3. Permission Key Structure: `Key`[¶](#3-permission-key-structure-key "Permanent link")

```
message Key {
  bytes address = 1;
  int64 weight = 2;
}
```

* `address`: Address with permission.
* `weight`: Weight of this address under this permission.

### 4. Permission Update Transaction: `AccountPermissionUpdateContract`[¶](#4-permission-update-transaction-accountpermissionupdatecontract "Permanent link")

```
message AccountPermissionUpdateContract {
  bytes owner_address = 1;
  Permission owner = 2;
  Permission witness = 3;
  repeated Permission actives = 4;
}
```

* This contract is used to **update all account permission structures at once**, which is an "all-or-nothing" update. Even if modifying a single permission, the full permission set must be resubmitted to prevent accidental loss of access.
* Even if only one permission is modified, all other existing permissions must be fully specified in the contract.

### 5. Contract Type Enumeration: `ContractType`[¶](#5-contract-type-enumeration-contracttype "Permanent link")

Active permissions configure which `ContractType` can be executed through the `operations` field. The full list of `ContractType` enum values together with their Proto Message, Actuator, status, and business behaviors is maintained in [System Contracts — ContractType Overview](../system-contracts/#contracttype-overview). For details on how to calculate the value of `operations` from these enum values, please see [Operations Value Calculation Example](#2-operations-value-calculation-example).

Explanation of Each Permission Type[¶](#explanation-of-each-permission-type "Permanent link")
---------------------------------------------------------------------------------------------

### `owner` Permission (Account Master Control)[¶](#owner-permission-account-master-control "Permanent link")

* Holds full control over the account.
* Can modify any permission structure (including itself).
* Automatically set when creating an account, default threshold is 1, includes the account's own address.
* By default, transactions without a specified `Permission_id` use the Owner permission.

### `witness` Permission (Block Production Permission)[¶](#witness-permission-block-production-permission "Permanent link")

* Available only for Super Representative, Super Representative Partner, and Super Representative Candidate accounts.
* Controls block-producing nodes, does not have permissions for fund transfers or other operations.
* Can delegate block production permission to other addresses to enhance account security.
* Must contain exactly one key.

#### Super Representative Node Configuration Example:[¶](#super-representative-node-configuration-example "Permanent link")

```
# config.conf
//localWitnessAccountAddress = TMK5c1jd...m6FXFXEz  # TRON Address
localwitness = [
  xxx  # Private key of TMK5c1jd...m6FXFXEz
]
```

If the `witness` permission is modified, then:

```
localWitnessAccountAddress = TSMC4YzU...PBebBk2E
localwitness = [
  yyy  # Private key of TSMC4YzU...PBebBk2E
]
```

> **Note**: Only one private key is allowed in `localwitness`.

### Active Permission (Functional Permission Combination)[¶](#active-permission-functional-permission-combination "Permanent link")

* Can combine contract permissions, assigning sub-permissions to different roles.
* Supports up to 8 `active` permission configurations.
* Permission IDs start from 2 and increment.
* When creating an account by default, one `active` permission is generated, with a default threshold of 1, containing only the account's own address.

Fees and Constraints[¶](#fees-and-constraints "Permanent link")
---------------------------------------------------------------

| Operation | Fee Standard |
| --- | --- |
| Modify Account Permission | 100 TRX |
| Transaction (2 or more signatures) | Additional 1 TRX |

The above fees can be adjusted through proposals.

Interfaces and Operation Examples[¶](#interfaces-and-operation-examples "Permanent link")
-----------------------------------------------------------------------------------------

### 1. Permission Modification Operation Process[¶](#1-permission-modification-operation-process "Permanent link")

1. Use `getaccount` to query the current account permission structure.
2. Construct the new permission configuration.
3. Call `AccountPermissionUpdateContract`.
4. Sign and broadcast the transaction.

**Note**: When a block contains transactions that modify account permissions, no further transactions from this account will be included in this block to ensure that such account permission modify transactions actually take effect starting from the next block.

#### Example Request:[¶](#example-request "Permanent link")

```
POST http://{{host}}:{{port}}/wallet/accountpermissionupdate

{
  "owner_address": "41ffa946...",
  "owner": {
    "type": 0,
    "id": 0,
    "permission_name": "owner",
    "threshold": 2,
    "keys": [...]
  },
  "witness": {
    "type": 1,
    "id": 1,
    "permission_name": "witness",
    "threshold": 1,
    "keys": [...]
  },
  "actives": [
    {
      "type": 2,
      "id": 2,
      "permission_name": "active0",
      "threshold": 3,
      "operations": "7fff1fc0037e...",
      "keys": [...]
    }
  ]
}
```

### 2. Operations Value Calculation Example[¶](#2-operations-value-calculation-example "Permanent link")

`operations` field is a 32-byte hexadecimal string (little-endian) where each bit represents a specific `ContractType` , defining the functional scope of an Active permission.
The following Java example generates permissions for contracts (ID=0-45):

```
Integer[] contractId = {0, 1, 2, ..., 45};
byte[] operations = new byte[32];
for (int id : contractId) {
  operations[id / 8] |= (1 << id % 8);
}
System.out.println(ByteArray.toHexString(operations));
```

> **Note**: The `contractId` above only illustrates the bit-setting logic. `ContractType` IDs are not contiguous (7, 21-29, 34-40, etc. are gaps), and a bit can only be set for a contract type already present in the on-chain `AVAILABLE_CONTRACT_TYPE` bitmap; otherwise the transaction is rejected during validation. `AVAILABLE_CONTRACT_TYPE` roughly corresponds to the `#` column in the [ContractType Overview](../system-contracts/#contracttype-overview) table, except it does not include ShieldedTransferContract(51).

### 3. Transaction Execution Process[¶](#3-transaction-execution-process "Permanent link")

1. Create the transaction.
2. Set `Permission_id` (default is 0, i.e., `owner` permission).
3. User A signs and forwards to B.
4. User B signs and forwards to C.
5. (Under Asynchronous Multi-Sig Orchestration,transactions can be partially signed and passed between authorized parties until the threshold is met.)
6. The last user signs and broadcasts.
7. The node verifies if the total signature weight ≥ `threshold`; if yes, accepts the transaction.

> Example code reference: [wallet-cli Example](https://github.com/tronprotocol/wallet-cli/blob/develop/java/src/main/java/org/tron/common/utils/TransactionUtils.java)

Auxiliary Interfaces[¶](#auxiliary-interfaces "Permanent link")
---------------------------------------------------------------

### Query Signed Addresses[¶](#query-signed-addresses "Permanent link")

```
POST /wallet/getapprovedlist

rpc GetTransactionApprovedList(Transaction) returns (TransactionApprovedList) {}
```

### Query Signature Weight[¶](#query-signature-weight "Permanent link")

```
POST /wallet/getsignweight

rpc GetTransactionSignWeight(Transaction) returns (TransactionSignWeight) {}
```

References[¶](#references "Permanent link")
-------------------------------------------

* [TIP-16 Permission Management Proposal](https://github.com/tronprotocol/tips/blob/master/tip-16.md)
* [Tron.proto Contract Type Definitions](https://github.com/tronprotocol/java-tron/blob/master/protocol/src/main/protos/core/Tron.proto)

July 31, 2026

---

[Previous

System Contract](../system-contracts/)
[Next

Developer Guide](../../developers/java-tron/)