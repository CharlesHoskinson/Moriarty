[Skip to content](#resource-model)

java-tron

Resource Model



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
  + Resource Model

    [Resource Model](./)


    Table of contents
    - [Introduction to the Resource Model](#introduction-to-the-resource-model)
    - [TRON Power (TP)](#tron-power-tp)
    - [Bandwidth](#bandwidth)

      * [Obtaining Bandwidth](#obtaining-bandwidth)
      * [How Bandwidth is Consumed](#how-bandwidth-is-consumed)
      * [Automatic Bandwidth Recovery](#automatic-bandwidth-recovery)
      * [Querying Bandwidth Balance](#querying-bandwidth-balance)
    - [Energy](#energy)

      * [Acquiring and Consuming Energy](#acquiring-and-consuming-energy)
      * [How to Set fee\_limit (Essential for Users)](#set-fee-limit)
      * [Energy Consumption Mechanism](#energy-consumption-mechanism)
    - [Dynamic Energy Model](#dynamic-energy-model)

      * [How It Works](#how-it-works)
    - [Staking on TRON](#staking-on-tron)

      * [How to Stake for System Resources](#how-to-stake-for-system-resources)
      * [How to Delegate Resources](#how-to-delegate-resources)
      * [How to Unstake](#how-to-unstake)
      * [How to Cancel All Unstaking Requests](#how-to-cancel-all-unstaking-requests)
      * [Resource Reclamation Upon Undelegation](#resource-reclamation-upon-undelegation)
      * [Relevant API Endpoints](#relevant-api-endpoints)
  + [Smart Contract](../../contracts/contract/)
  + [System Contract](../system-contracts/)
  + [Account Permission Management](../multi-signatures/)
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

* [Introduction to the Resource Model](#introduction-to-the-resource-model)
* [TRON Power (TP)](#tron-power-tp)
* [Bandwidth](#bandwidth)

  + [Obtaining Bandwidth](#obtaining-bandwidth)
  + [How Bandwidth is Consumed](#how-bandwidth-is-consumed)
  + [Automatic Bandwidth Recovery](#automatic-bandwidth-recovery)
  + [Querying Bandwidth Balance](#querying-bandwidth-balance)
* [Energy](#energy)

  + [Acquiring and Consuming Energy](#acquiring-and-consuming-energy)
  + [How to Set fee\_limit (Essential for Users)](#set-fee-limit)
  + [Energy Consumption Mechanism](#energy-consumption-mechanism)
* [Dynamic Energy Model](#dynamic-energy-model)

  + [How It Works](#how-it-works)
* [Staking on TRON](#staking-on-tron)

  + [How to Stake for System Resources](#how-to-stake-for-system-resources)
  + [How to Delegate Resources](#how-to-delegate-resources)
  + [How to Unstake](#how-to-unstake)
  + [How to Cancel All Unstaking Requests](#how-to-cancel-all-unstaking-requests)
  + [Resource Reclamation Upon Undelegation](#resource-reclamation-upon-undelegation)
  + [Relevant API Endpoints](#relevant-api-endpoints)

Resource Model[¶](#resource-model "Permanent link")
===================================================

Introduction to the Resource Model[¶](#introduction-to-the-resource-model "Permanent link")
-------------------------------------------------------------------------------------------

The TRON network's core system resources consist of three components: TRON Power (TP), Bandwidth, and Energy. Their definitions and functions are as follows:

* **TRON Power (TP):** Used exclusively for voting for Super Representatives (SRs) and Super Representative Partners. It serves as the credential for users to participate in network governance and is obtained by staking TRX.
* **Bandwidth:** Measures the byte size of a transaction on the blockchain. Every type of transaction, from a simple transfer to a contract interaction, must consume Bandwidth.
* **Energy:** Measures the computational resources consumed by the TRON Virtual Machine (TVM) to execute a smart contract. It can be understood as a "CPU processing fee." Energy is only consumed when deploying or triggering a smart contract.

TRON Power (TP)[¶](#tron-power-tp "Permanent link")
---------------------------------------------------

Before voting for Super Representatives, an account must first acquire TRON Power (TP).

* **How to Obtain:** When you stake TRX for either Bandwidth or Energy, you simultaneously receive an equivalent amount of TRON Power. This is the only way to get TP. For staking instructions, refer to the [Staking on TRON](#staking-on-tron) section.
* **Conversion Ratio:** The staking-to-TP ratio is 1:1. Staking 1 TRX grants you 1 TP.
* **Accumulation:** You can stake TRX in multiple, separate transactions. The TP acquired from all stakes is automatically added to your account's total TP pool.
* **Querying:** You can check your account's total and used TP at any time using the `wallet/getaccountresource` API endpoint.

Bandwidth[¶](#bandwidth "Permanent link")
-----------------------------------------

Transactions are transmitted and stored on the network as byte arrays. The Bandwidth consumed by a transaction is calculated as `Transaction Size (bytes) * Bandwidth Rate`. The current Bandwidth rate is 1.

For example, a transaction with a size of 200 bytes will consume 200 Bandwidth.

> **Note:** Because the total staked funds in the network and an individual account's staked funds can change at any time, the amount of Bandwidth an account possesses is not a fixed value.

### Obtaining Bandwidth[¶](#obtaining-bandwidth "Permanent link")

There are three ways to obtain Bandwidth:

* **Staking TRX:** Users share a fixed total Bandwidth pool in proportion to the amount of TRX they have staked for Bandwidth.

  ```
  Bandwidth Share = (TRX Staked for Bandwidth / Total TRX Staked for Bandwidth Network-Wide) * Total Bandwidth Limit
  ```

  **Total Bandwidth** is a network parameter that can be modified through a committee proposal ([#62](https://tronscan.io/#/sr/committee)) and is currently set to 43,200,000,000.

  Staking for Bandwidth (`wallet-cli` example):

  ```
  freezeBalanceV2 frozen_balance [ResourceCode:0 BANDWIDTH,1 ENERGY]
  ```
* **Delegation from Others:**
  Another account can delegate their Bandwidth to you.

  Delegating Bandwidth (`wallet-cli` example):

  ```
  delegateResource [OwnerAddress] balance ResourceCode(0 BANDWIDTH,1 ENERGY), ReceiverAddress [lock]
  ```
* **Daily Free Allowance:** Every account receives a fixed daily allowance of free Bandwidth, which can be modified through a committee proposal [#61](https://tronscan.io/#/sr/committee) and is currently set to 600.

### How Bandwidth is Consumed[¶](#how-bandwidth-is-consumed "Permanent link")

All transactions, except for query operations, consume Bandwidth. When you initiate a transaction, the system deducts the Bandwidth fee according to a strict priority order based on the transaction type.

**Scenario 1: Standard Transactions**

The system attempts to pay the Bandwidth fee in the following order:

1. **Staked Bandwidth:** Consumes the Bandwidth obtained by the transaction initiator from staking TRX.
2. **Free Bandwidth:** If staked Bandwidth is insufficient, consumes the initiator's 600 daily free allowance.
3. **TRX Burning:** If both staked and free Bandwidth are insufficient, burns the initiator's TRX to cover the fee.
   * *Burn Fee = Transaction Size (bytes) × 1,000 sun*

**Scenario 2: New Account Creation Transactions**

Transactions that create a new account follow a special rule and do not use the daily free allowance:

1. **Staked Bandwidth:** First, attempts to consume the Bandwidth obtained by the transaction initiator from staking TRX.
2. **TRX Burning:** If staked Bandwidth is insufficient, directly burns 0.1 TRX to complete the account creation.

**Scenario 3: TRC-10 Token Transfers**

TRC-10 token transfers have a unique consumption logic that introduces the "token issuer" as a potential fee payer:

1. **Issuer's Bandwidth (Highest Priority):** The system first attempts to consume the Bandwidth prepaid by the token issuer. This requires all three of the following conditions to be met (only if all three checks pass will the issuer's Bandwidth be deducted; otherwise, the cost falls to the transaction initiator):
   * The token issuer has a sufficient total free Bandwidth allowance.
     + Query Method: [/wallet/getassetissuebyaccount](../../api/http/asset/getassetissuebyaccount/)
     + Formula: `public_free_asset_net_limit - public_free_asset_net_usage`
     + Description: The remaining quota the token issuer can pay for this token's transfers.
   * The transaction initiator has a sufficient Bandwidth allowance for that specific token.
     + Query Method: [/wallet/getaccountnet](../../api/http/account/getaccountnet/)
     + Formula: `assetNetLimit['assetID'] - assetNetUsed['assetID']`
     + Description: The free Bandwidth quota provided by the issuer that the token holder can still use.
   * The token issuer has sufficient staked Bandwidth.
     + Query Method: [/wallet/getaccountnet](../../api/http/account/getaccountnet/)
     + Formula: `NetLimit - NetUsed`
     + Description: The amount of available Bandwidth the issuer has obtained through staking.
2. **Initiator's Staked Bandwidth:** Attempts to consume the initiator's staked Bandwidth.
3. **Initiator's Free Bandwidth:** If staked Bandwidth is insufficient, consumes the initiator's free Bandwidth allowance.
4. **TRX Burning:** If all the above resources are insufficient, burns the initiator's TRX to pay the fee.
   * *Burn Fee = Transaction Size (bytes) × 1,000 sun*

### Automatic Bandwidth Recovery[¶](#automatic-bandwidth-recovery "Permanent link")

An account's consumed free Bandwidth and staked Bandwidth will gradually recover over a 24-hour period.

### Querying Bandwidth Balance[¶](#querying-bandwidth-balance "Permanent link")

You can query an account's current resource status by calling the `wallet/getaccountresource` HTTP endpoint. In the returned JSON data, use the following formulas to calculate the remaining Bandwidth:

```
Remaining Free Bandwidth = freeNetLimit - freeNetUsed
Remaining Staked Bandwidth = NetLimit - NetUsed
```

*Note: If any of these parameters are absent from the API response, their value is `0`.*

Energy[¶](#energy "Permanent link")
-----------------------------------

Energy is the unit of measurement for the computational resources consumed by the TRON Virtual Machine (TVM) when executing the instructions of a smart contract. This section provides a comprehensive overview of Energy focusing on the following three aspects:

* [The acquisition, consumption, and recovery of Energy](#acquiring-and-consuming-energy)
* [How to set the key parameter, `fee_limit`](#set-fee-limit)
* [The TRON network's overall consumption mechanism](#energy-consumption-mechanism)

### Acquiring and Consuming Energy[¶](#acquiring-and-consuming-energy "Permanent link")

Energy can be acquired in two primary ways:

* By Staking TRX: Users can obtain Energy by staking the TRX they hold.
* By Receiving Delegation: Users can receive Energy delegated to them from other accounts.

#### Staking for Energy (`wallet-cli` example)[¶](#staking-for-energy-wallet-cli-example "Permanent link")

```
freezeBalanceV2 frozen_balance [ResourceCode:0 BANDWIDTH,1 ENERGY]
```

#### Delegating Energy (`wallet-cli` example)[¶](#delegating-energy-wallet-cli-example "Permanent link")

```
delegateResource [OwnerAddress] balance ResourceCode(0 BANDWIDTH,1 ENERGY), ReceiverAddress [lock]
```

#### Querying Energy Balance[¶](#querying-energy-balance "Permanent link")

You can query an account's current Energy status using the `wallet/getaccountresource` HTTP endpoint. In the returned JSON data, calculate the remaining Energy using the following formula:

```
Remaining Energy = EnergyLimit - EnergyUsed
```

*Note: If these parameters are absent from the API response, their value is `0`.*

#### Calculating your Energy Share[¶](#calculating-your-energy-share "Permanent link")

The amount of Energy you receive is a dynamic value calculated in real-time based on your stake relative to the total network stake for Energy:

```
Your Energy Share = (TRX Staked for Energy / Total TRX Staked for Energy Network-Wide) * Total Energy Limit
```

Total Energy Limit is a network parameter set by the committee ([#19](https://tronscan.io/#/sr/committee)), currently at 180,000,000,000, and can be modified via proposals.

***Calculation Example***

Because your share is tied to the network's total stake, your available Energy will fluctuate as other users stake or unstake.

```
Assume only two users, A and B, have staked 2 TRX each.

Their respective Energy shares are:
A: 90,000,000,000
B: 90,000,000,000

When a third user, C, stakes 1 TRX, the shares are adjusted:
A: 72,000,000,000
B: 72,000,000,000
C: 36,000,000,000
```

#### Energy Consumption[¶](#energy-consumption "Permanent link")

***Payment Priority***

When a contract transaction consumes Energy, the system deducts the cost in the following order:

1. **Staked Energy:** First, the system consumes the Energy obtained by the transaction initiator from staking TRX.
2. **TRX Burning:** If staked Energy is insufficient to cover all instructions, the system automatically burns the initiator's TRX to cover the difference. The current price is 0.0001 TRX per unit of Energy.

***Fee Deduction for Exceptions***

Contract execution can be interrupted for various reasons, with different rules for Energy deduction:

* **Normal Interruption (`REVERT`):** If a contract exits due to a `REVERT` instruction, only the Energy for the instructions executed up to that point is deducted.
* **Unexpected Interruption (bug or Timeout):** If a contract crashes due to a code bug, timeout, or another unexpected error, the system will deduct all available Energy for the transaction as a penalty. Users can limit this penalty by setting the `fee_limit` parameter for the transaction.

#### Energy Recovery[¶](#energy-recovery "Permanent link")

Consumed Energy resources gradually recover over a 24-hour period.

### How to Set `fee_limit` (Essential for Users)[¶](#set-fee-limit "Permanent link")

Note

In this section, "developer" refers to the person who develops and deploys the contract, while "caller" refers to the user or contract that invokes it.
Since the Energy consumed by a contract can be converted to TRX (or sun), this section uses "Energy" and "TRX" interchangeably to refer to the resource cost. The terms are distinguished only when referring to specific numerical units.

`fee_limit` is a critical safety parameter when calling a smart contract. A properly set `fee_limit` ensures that a transaction can execute successfully while preventing excessive TRX consumption if the contract requires unexpectedly high Energy.

Before setting `fee_limit`, understand these concepts:

1. A valid `fee_limit` is an integer ranging from 0 to 15,000,000,000 sun (equivalent to 15,000 TRX). The `fee_limit` upper bound is a network parameter that can be modified by committee proposals ([#47](https://tronscan.io/#/sr/committee)), and its current value is 15,000 TRX.
2. Contracts of varying complexity consume different amounts of Energy. The same contract generally consumes a similar amount of Energy per execution[1](#fn:1), but with the dynamic energy model, popular contracts may require more Energy at different times. For details, see the [Dynamic Energy Model](#dynamic-energy-model) section. During execution, Energy is deducted instruction by instruction. If the cost exceeds the `fee_limit`, the execution fails, and the consumed Energy is not refunded.
3. The `fee_limit` currently specifies the maximum amount of TRX the **caller** is willing to pay to execute a contract[2](#fn:2). Note that the total Energy consumed by the contract's execution can be a combination of what the caller pays and what the developer covers for the contract.
4. If a contract execution times out or crashes due to a bug, all the Energy allowed for the current transaction will be consumed. This total energy pool is the sum of the following components: `Total Consumed Energy = Energy from Caller's Stake + Energy from Developer's Share + Energy from Burned TRX`. The "Energy from Burned TRX" component is capped by `fee_limit`.
5. Through the [Energy sharing mechanism](https://developers.tron.network/docs/energy-consumption-mechanism#tron-energy-sharing-mechanism), a developer may cover a percentage of the Energy cost (e.g., 90%). However, if the developer's account has insufficient Energy, the remaining cost falls entirely to the caller. Within the `fee_limit`, if the caller's Energy is also insufficient, an equivalent amount of TRX will be burned.

**Example:**
Here's how to estimate the `fee_limit` for executing a contract `C`:

* Assume contract C consumed 18,000 Energy during its last successful execution. By calling the [estimateenergy](../../api/http/smart-contract/estimateenergy/) API to get a pre-execution estimate, let's assume the upper limit of Energy consumption for this transaction is approximately 20,000 Energy.
* When burning TRX, since the unit price of Energy is currently 100 sun, 10 TRX can be exchanged for a fixed 100,000 Energy units.
* Assume the developer has committed to covering 90% of the Energy cost and has sufficient Energy.

The `fee_limit` estimation method is as follows:

* **Step 1: Calculate the Total Transaction Fee**: Calculate the total potential cost of the transaction by multiplying the estimated maximum Energy consumption by the current Energy price: `20,000 Energy * 100 sun = 2,000,000 sun (equivalent to 2 TRX)`.
* **Step 2: Determine the User's Share**: Calculate the portion of the fee the user is responsible for. Given the developer commits to covering 90%, the user's share is 10%: `2,000,000 sun * 10% = 200,000 sun`.
* **Step 3: Set the Final `fee_limit`**: The recommended `fee_limit` for the user to set is 200,000 sun.

### Energy Consumption Mechanism[¶](#energy-consumption-mechanism "Permanent link")

**Basic Energy Consumption Rules**

When executing smart contract transactions, the system calculates and deducts the Energy required for each instruction sequentially. The consumption of Energy in an account follows these priority principles:

1. Available Energy (obtained through staking or renting) in the account is first used.
2. If that part of Energy is insufficient, the remaining part will be covered by burning TRX from the account at a fixed rate (0.0001 TRX per Energy unit).

**Contract Energy Sharing Mechanism**

For smart contract calls, to reduce the caller's costs, TRON allows contract deployers to bear a portion of the Energy consumption. For specific details, please refer to the Contract Energy Sharing Mechanism section.

Energy Deduction Rules:

* Portion borne by the contract deployer:
  + Directly deducted from the available Energy in the deployer's account. TRX in the deployer's account will not be burned.
* Portion borne by the contract caller:
  + Available Energy in the caller’s account is consumed first.
  + If insufficient, the remaining part will be covered by burning TRX from the caller's account at a fixed rate.

Dynamic Energy Model[¶](#dynamic-energy-model "Permanent link")
---------------------------------------------------------------

The Dynamic Energy Model is a resource-balancing mechanism on the TRON network. It dynamically adjusts the Energy consumption of each contract based on its resource usage, promoting a more equitable distribution of Energy and preventing network resources from being excessively concentrated on a few popular contracts. For more details, see the [Introduction to Dynamic Energy Model](https://medium.com/tronnetwork/introduction-to-dynamic-energy-model-31917419b61a).

### How It Works[¶](#how-it-works "Permanent link")

If a contract consumes an excessive amount of Energy within a maintenance period (currently 6 hours), transactions calling that same contract will incur additional Energy costs in the next period. When the contract's resource usage returns to a reasonable level, the Energy cost for calling it will gradually return to normal.

Each contract has an `energy_factor`, which is a multiplier for its base Energy consumption. The initial value is `0`.

* An `energy_factor` of `0` means the contract is using resources reasonably, and calls will incur no extra Energy cost.
* An `energy_factor` greater than `0` indicates it is a popular contract, and calls will consume additional Energy. You can query a contract's `energy_factor` via the `getcontractinfo` API endpoint.

The final Energy consumption for a contract call is calculated as:

```
Contract Transaction Energy Consumption = Base Energy Consumption Generated by the Contract Call Transaction * (1 + energy_factor)
```

The dynamic energy model introduces three network parameters that control the `energy_factor`:

* `threshold`: The threshold for a contract's base Energy consumption. If a contract exceeds this threshold in a maintenance period, its Energy cost will increase in the next period.
* `increase_factor`: The rate at which `energy_factor` increases when the threshold is exceeded.
* `max_factor`: The maximum possible value for `energy_factor`.

There is also a `decrease_factor` used to lower the `energy_factor`:

* `decrease_factor`: Set to **one-fourth** of the `increase_factor`. When a contract's base Energy consumption falls below the threshold, its `energy_factor` is reduced by this rate.

**`energy_factor` Adjustment Formulas:**

* When the base energy consumption of a contract exceeds the `threshold` within a maintenance period, its `energy_factor` will increase during the next maintenance period, but will not exceed `max_factor`. The formula for this calculation is:

  ```
  energy_factor = min((1 + energy_factor) * (1 + increase_factor) - 1, max_factor)
  ```
* When the base energy consumption of a contract falls to or below the `threshold` within a maintenance period, its `energy_factor` will decrease during the next maintenance period, but not below a minimum of `0`. The formula for this calculation is:

  ```
  energy_factor = max((1 + energy_factor) * (1 - decrease_factor) - 1, 0)
  ```

The dynamic energy model is active on Mainnet with the following parameters:

* `threshold`: 5,000,000,000
* `increase_factor`: 0.2
* `max_factor`: 3.4

Since the Energy cost for popular contracts can vary between maintenance periods, it is crucial to set an appropriate `fee_limit` for transactions.

Staking on TRON[¶](#staking-on-tron "Permanent link")
-----------------------------------------------------

### How to Stake for System Resources[¶](#how-to-stake-for-system-resources "Permanent link")

On the TRON network, staking TRX is the unified mechanism for obtaining the three core resources: Energy, Bandwidth, and TRON Power (TP).

#### How to Stake[¶](#how-to-stake "Permanent link")

* **HTTP API:** Call the `wallet/freezebalancev2` endpoint.
* **Smart Contract:** Use the [Stake 2.0 Solidity API](https://developers.tron.network/docs/stake-20-solidity-api) within a contract.

When you unstake, the corresponding resources (Energy/Bandwidth) and TP are released and reclaimed simultaneously.

### How to Delegate Resources[¶](#how-to-delegate-resources "Permanent link")

After an account obtains Energy or Bandwidth through staking, it can choose to delegate these resources to other TRON accounts. This allows accounts with a surplus of resources to help those with insufficient resources complete transactions.

**Delegation Rules and Key Restrictions**

Before delegating, you must understand the following key rules:

* **Delegable Resources:** Only Energy and Bandwidth can be delegated. TP cannot.
* **Source of Resources:** Only available resources obtained through Stake 2.0 are eligible for delegation.
* **Recipient:** The recipient must be an activated external account, not a contract address.

**Time Lock Option**

When delegating, you can choose to enable a time lock, which affects when you can reclaim the resources.

* **With Time Lock:** The resources are locked for a period. You must wait for this period to end before you can undelegate.
  **Important:** If you delegate to the same address again during this period, the pending period resets.
* **Without Time Lock:** You can undelegate at any time and reclaim the resources immediately.

**Related API Endpoints**

* `delegateresource`: Delegate resources.
* `undelegateresource`: Undelegate (reclaim) resources.
* `getcandelegatedmaxsize`: Query the maximum amount of a resource you can delegate.

### How to Unstake[¶](#how-to-unstake "Permanent link")

After staking TRX, you can initiate an unstake operation at any time using the `unfreezebalancev2` API. However, this process has a time delay and follows specific rules.

After staking TRX, you can initiate an unstake operation at any time using the `unfreezebalancev2` API. After initiating an unstake, your TRX enters a 14-day pending period. This pending period is TRON network parameter [#70](https://tronscan.io/#/sr/committee) and can be changed in the future through network governance. After the 14-day period has ended, you can withdraw the funds to your account balance using the `withdrawexpireunfreeze` API.

Important

* **Delegated Resources Cannot Be Unstaked:** You cannot unstake TRX corresponding to resources that are currently delegated. You must first reclaim the resources using `undelegateresource` before you can unstake that portion of TRX.
* **Resource Reclamation:** Unstaking will cause the resources (Energy or Bandwidth) and TRON Power (TP) corresponding to the staked TRX to be synchronously reclaimed by the system. As a result, you will lose the respective Energy or Bandwidth and an equivalent amount of TP.
* **Concurrent Operation Limit:** You can have a maximum of 32 unstake operations in the 14-day pending period at any one time. Use the `getavailableunfreezecount` endpoint to check your remaining unstake capacity.

**Automatic Effects of Unstaking**: Calling `unfreezebalancev2` not only initiates a new unstaking process, but it also automatically withdraws any TRX that has already completed its 14-day pending period.

**How to Verify Withdrawn Amount**: To find out exactly how much unstaked TRX was automatically withdrawn during a specific unstaking operation, you can query the details of that unstake transaction using the `gettransactioninfobyid` API and look for the field `withdraw_expire_amount`, which shows the amount of matured unstaked TRX that was automatically withdrawn in this transaction.

#### Reclaiming TRON Power[¶](#reclaiming-tron-power "Permanent link")

Under Stake 2.0, unstaking TRX simultaneously reclaims an equivalent amount of TRON Power (TP). If the amount of TP to be reclaimed exceeds your account's idle (unvoted) TP, the system will proportionally revoke your cast votes.

***TP Reclamation Priority***: When the system reclaims TP, it follows the following two-step process:

1. **Reclaim Idle TP First:** The system first reclaims all of your account's TP that is not currently being used for voting.
2. **Cancel Votes as Needed:** If the idle TP is insufficient to meet the reclamation demand, the system will begin to revoke your cast votes to reclaim the remaining required TP.

***Rules for Calculating Vote Revocation***

The revocation operation is not random. Instead, votes are revoked proportionally and fairly from every Super Representative (SR) and Super Representative Partner you have voted for.

* *Formula:*

  ```
  Votes Revoked from a Given SR = Total Votes to Revoke * (Votes Cast for that SR or SR Partner / Total Votes Cast by the Account)
  ```
* *Example:*

  Assume User `A`'s initial account state is:

  + Total Staked: 2,000 TRX
  + Total TP: 2,000 TP
  + Votes Cast: 1,000 TP (600 for SR1, 400 for SR2)
  + Unused TP: 1,000 TP

  Now, User `A` unstakes 1,500 TRX.

  *System Process:*

  1. Reclamation Demand: The system needs to reclaim 1,500 TP.
  2. Reclaim Unused TP: First, it reclaims all 1,000 idle TP.
  3. Calculate Shortfall: A remaining `1,500 - 1,000 = 500 TP` must be reclaimed by revoking votes.
  4. Proportional revocation:
     + Votes revoked for SR1: `500 * (600 / 1000) = 300` votes.
     + Votes revoked for SR2: `500 * (400 / 1000) = 200` votes.
  5. Final State: User A successfully unstakes 1,500 TRX. Their voting state is updated to: 300 votes for SR1 and 200 votes for SR2.

Note

Stake 1.0 vs. Stake 2.0

Although Stake 2.0 is the current standard, TRX staked via the legacy Stake 1.0 system is still valid and can be redeemed using its corresponding `wallet/unfreezebalance` API. Unstaking TRX from Stake 1.0 will revoke **all** of the account's votes.

### How to Cancel All Unstaking Requests[¶](#how-to-cancel-all-unstaking-requests "Permanent link")

If you initiate an unstake but change your mind, Stake 2.0 provides an efficient "cancel" feature. You can use the `cancelallunfreezev2` API to immediately cancel all pending unstake requests, bypassing the 14-day pending period, and get your resources back immediately.

**Please note**: this endpoint cancels all of your account's unstaking requests that are currently in the 14-day pending period.

* **TRX Status:** The canceled TRX is immediately re-staked.
* **Resource Type:** The re-staked funds will acquire the same resource type (Energy or Bandwidth) as the original stake.

**Additional Effect - Automatic Withdrawal**: This operation will also automatically withdraw any unstaked TRX that has already completed its 14-day pending period and is awaiting withdrawal.

**How to Verify the Result**

You can query the transaction details using `gettransactioninfobyid` and check the following fields:

* `cancel_unfreezeV2_amount`: The total amount of TRX that was successfully canceled and re-staked.
* `withdraw_expire_amount`: The total amount of matured unstaked TRX that was automatically withdrawn to your account balance.

### Resource Reclamation Upon Undelegation[¶](#resource-reclamation-upon-undelegation "Permanent link")

**Note**: In the TRON network, the amount of resources an account receives depends on the proportion of its TRX staked relative to the total TRX staked across the entire network. When a delegator delegates resources to a recipient, it essentially lends a portion of its own staked TRX to the recipient. In resource quota calculations, the system does not distinguish between the sources of TRX. TRX staked by the account itself and TRX delegated from other accounts are aggregated and treated uniformly as the weighting basis for resource allocation. So, unless otherwise specified, the term “staked amount” in this chapter refers to the total staked TRX of an account, including both self-staked TRX and TRX delegated from other accounts, without distinguishing their sources.

When a Delegator initiates resource delegation, a certain amount of staked TRX is lent to the Recipient, granting them the right to use the corresponding resources. When the Delegator cancels the resource delegation, the system not only reclaims the corresponding staked TRX, but also reclaims a proportional amount of the Recipient’s unrecovered resources.

#### Reclamation Logic for Unrecovered Resources[¶](#reclamation-logic-for-unrecovered-resources "Permanent link")

##### 1. Calculation Formula[¶](#1-calculation-formula "Permanent link")

The system proportionally reclaims the Recipient's unrecovered resources based on the amount of TRX being undelegated according to the following formula:

```
Reclaimed unrecovered resources = (Canceled delegated TRX amount / Recipient’s total staked TRX amount for that resource) * Recipient’s unrecovered resource amount
```

**Note**: The reclaimed unrecovered resources **must not exceed** the maximum resource capacity corresponding to the undelegated TRX amount, calculated in real time based on the total network staking.

* **Canceled delegated TRX amount**: The amount of staked TRX reclaimed in the cancel delegation transaction.
* **Recipient's Total Staked TRX Amount for that resource**: The total staked TRX used by the recipient to obtain a specific resource (Energy or Bandwidth), including self-staked TRX (Stake 1.0 and Stake 2.0) and delegated TRX from others. This can be queried via the [wallet/getaccount](../../api/http/account/getaccount/) API.
* **Recipient's Unrecovered Resource Amount**: The amount of resources that have already been consumed and are currently in the recovery period on the recipient’s account. This can be queried via [wallet/getaccount](../../api/http/account/getaccount/) or [wallet/getaccountresource](../../api/http/account/getaccountresource/).

##### 2. Account State Changes[¶](#2-account-state-changes "Permanent link")

After resource undelegation, the effective resource-related states of both accounts are changed as follows.
The following expressions describe the changes at a logical level. The term “staked amount” represents a computed value rather than a single on-chain field.

**Delegator**

```
Staked amount = Original staked amount + Canceled delegated TRX amount

Unrecovered resource amount = Original unrecovered resource amount + Reclaimed unrecovered resource amount
```

**Recipient**

```
Staked amount = Original staked amount - Canceled delegated TRX amount

Unrecovered resource amount = Original unrecovered resource amount - Reclaimed unrecovered resource amount
```

**Note on Recovery:** Resources recover linearly over a **24-hour period**. If an account uses resources again or reclaims delegated resources during this period, the system performs a weighted merger of the existing recovery progress and the new recovery cycle.

#### Example[¶](#example "Permanent link")

Assume the current network resource conversion ratio is **1 TRX Staked = 0.2 Energy**, and the total network staking amount remains unchanged. User X delegated **200 TRX** worth of Energy to User Y. Before the delegation is canceled, the account states are:

| Account | Role | Total Staked TRX (for Energy) | Total Energy | Unrecovered Energy |
| --- | --- | --- | --- | --- |
| X | Delegator | 1000 TRX | 200 Energy | 75 Energy |
| Y | Recipient | 500 TRX | 100 Energy | 50 Energy |

When User X cancels the Energy delegation of **200 TRX** to User Y:

```
Reclaimed unrecovered Energy amount = (200 / 500) * 50 = 20 Energy
```

After reclamation, the account states become:

| Account | Role | Total Staked TRX (for Energy) | Total Energy | Unrecovered Energy |
| --- | --- | --- | --- | --- |
| **X** | Delegator | 1200 TRX (1000+200) | 240 Energy | 95 Energy (75+20) |
| **Y** | Recipient | 300 TRX (500-200) | 60 Energy | 30 Energy (50-20) |

### Relevant API Endpoints[¶](#relevant-api-endpoints "Permanent link")

| API Endpoint | Description |
| --- | --- |
| `wallet/freezebalancev2` | Stake TRX. |
| `wallet/unfreezebalancev2` | Unstake TRX. |
| `wallet/delegateresource` | Delegate resources. |
| `wallet/undelegateresource` | Undelegate resources. |
| `wallet/withdrawexpireunfreeze` | Withdraw unstaked TRX that has passed the pending period. |
| `wallet/getavailableunfreezecount` | Check the remaining number of unstake operations allowed. |
| `wallet/getcanwithdrawunfreezeamount` | Check the amount of withdrawable unstaked TRX. |
| `wallet/getcandelegatedmaxsize` | Check the maximum amount of delegable resources. |
| `wallet/getdelegatedresourcev2` | Check resources delegated from one address to another. |
| `wallet/getdelegatedresourceaccountindexv2` | Check an account's delegation and received delegation status. |
| `wallet/getaccount` | Check account stake, resources, unstake, and voting status. |
| `wallet/getaccountresource` | Check resource totals, usage, and available amounts. |
| `wallet/cancelallunfreezev2` | Cancel all pending unstake requests. |

---

1. If a developer is unsure about a contract's stability, they should not set the user's cost share to 0%. Otherwise, if the execution is deemed malicious, all of the developer's Energy will be deducted. [↩](#fnref:1 "Jump back to footnote 1 in the text")
2. Therefore, it is recommended that developers set the user's share of the cost to be between 10% and 100%. [↩](#fnref:2 "Jump back to footnote 2 in the text")

June 13, 2026

---

[Previous

Account Model](../account/)
[Next

Smart Contract](../../contracts/contract/)