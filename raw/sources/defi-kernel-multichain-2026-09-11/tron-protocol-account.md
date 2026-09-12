[Skip to content](#account-model)

java-tron

Account Model



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
  + Account Model

    [Account Model](./)


    Table of contents
    - [Introduction](#introduction)
    - [How to Create an Account](#how-to-create-an-account)
    - [Key Pair Generation Algorithm](#key-pair-generation-algorithm)
    - [TRON Address Generation](#tron-address-generation)

      * [Base58Check Calculation Process](#base58check-calculation-process)
      * [TRON Address Characteristics](#tron-address-characteristics)
    - [Signature Specification](#signature-specification)

      * [Algorithm](#algorithm)
      * [Java Code Sample](#java-code-sample)
    - [Signature Verification](#signature-verification)

      * [Algorithm](#algorithm_1)
      * [Example](#example)
      * [Signature normalization](#signature-normalization)
  + [Resource Model](../resource/)
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

* [Introduction](#introduction)
* [How to Create an Account](#how-to-create-an-account)
* [Key Pair Generation Algorithm](#key-pair-generation-algorithm)
* [TRON Address Generation](#tron-address-generation)

  + [Base58Check Calculation Process](#base58check-calculation-process)
  + [TRON Address Characteristics](#tron-address-characteristics)
* [Signature Specification](#signature-specification)

  + [Algorithm](#algorithm)
  + [Java Code Sample](#java-code-sample)
* [Signature Verification](#signature-verification)

  + [Algorithm](#algorithm_1)
  + [Example](#example)
  + [Signature normalization](#signature-normalization)

Account Model[¶](#account-model "Permanent link")
=================================================

Introduction[¶](#introduction "Permanent link")
-----------------------------------------------

TRON employs an account model for its ledger. All activities on the network, such as transfers, voting, and contract deployment, revolve around accounts.

* **Unique Identifier**: Each account is uniquely identified by its Address, which typically begins with a `T`.
* **Access Control**: Any operation on an account (such as a transfer) requires a signature from the corresponding Private Key.
* **Account Assets and Capabilities**: Each account can own and manage various resources, including:

  + **Assets**: TRX, TRC-10, TRC-20, TRC-721/TRC-1155 NFTs, etc.
  + **Network Resources**: Bandwidth and Energy.
  + **Permissions and Activities**: Initiating transactions, deploying and calling smart contracts, participating in Super Representative elections (voting or becoming a candidate), and more.

How to Create an Account[¶](#how-to-create-an-account "Permanent link")
-----------------------------------------------------------------------

There are two primary ways to create a new TRON account:

**Method 1: Offline Generation and On-Chain Activation**

* Generate Address: Use a wallet application (like [TronLink](https://www.tronlink.org/)) to generate a new key pair (private key and address).
* Activate Account: At this stage, the account exists only conceptually and must be "activated" to be used on the blockchain. Activation is accomplished by sending any amount of TRX or a TRC-10 token from an existing account to this new address. Once the transaction is successful, the new account is officially created on the TRON network.

**Method 2: Creation via System Contract**

* Developers can create an account by calling the `AccountCreateContract` system contract.

**Account Creation Cost:**

* Activating a new account costs 1 TRX, which is burned. This amount is the `getCreateNewAccountFeeInSystemContract` network parameter (currently 1 TRX on mainnet) and can be changed by committee proposal.
* Additionally, if the creator's account has sufficient Bandwidth (either from staking TRX or delegated from others), the creation will only consume Bandwidth. Otherwise, the `getCreateAccountFee` is burned to pay for the Bandwidth. This Bandwidth fee is a network parameter (currently 0.1 TRX on mainnet) and can also be changed by committee proposal. Note that the daily free Bandwidth cannot be used to create an account; only Bandwidth obtained from staking TRX or delegated from others is eligible.

Key Pair Generation Algorithm[¶](#key-pair-generation-algorithm "Permanent link")
---------------------------------------------------------------------------------

TRON's signature algorithm is ECDSA, and the selected curve is SECP256K1. Its private key is a random number, and the public key is a point on the elliptic curve. The generation process is as follows: first, generate a random number `d` as the private key, then calculate `P = d × G` as the public key, where `G` is the base point of the elliptic curve, and the base point is public.

The private key is a 32-byte large number, and the public key consists of two 32-byte large numbers, which are the abscissa and ordinate of the above-mentioned `P` point respectively.

TRON Address Generation[¶](#tron-address-generation "Permanent link")
---------------------------------------------------------------------

1. Take the public key `P` as input, calculate `SHA3` to get the result `H` (`SHA3` uses Keccak256).
2. Take the last 20 bytes of `H`, and prepend the byte `0x41` to get the `address`.
3. Perform a Base58Check calculation on the `address` to get the final address.

### Base58Check Calculation Process[¶](#base58check-calculation-process "Permanent link")

1. Calculate the checksum

   a. Perform a SHA256 hash operation on `address` to get `h1`.  
   b. Perform a SHA256 operation on `h1` again to get `h2`.  
   c. Take the first 4 bytes of `h2` as the checksum `check`.
2. Splice data: Append `check` to `address` to get `address||check`.
3. Base58 encoding: Perform Base58 encoding on `address||check`. The character table for Base58 is: `"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"`, excluding easily confused characters: `0` (Arabic numeral 0), `O` (uppercase letter O), `I` (uppercase letter I), `l` (lowercase letter L).

### TRON Address Characteristics[¶](#tron-address-characteristics "Permanent link")

The principle of Base58 encoding is to convert a large integer with a base of 256 into a representation with a base of 58, and then map it to the character table. Since the first byte of `address||check` is fixed as `0x41`, its decimal value `N` satisfies: `65 × 256²⁴ ≤ N < 66 × 256²⁴`.

* Length is `34`: Because `58³⁴ > 66 × 256²⁴` indicates that a length of `34` is sufficient, and `58³³ < 65 × 256²⁴` indicates that a length of `33` is insufficient, so the length can only be `34`.
* The first character is `T`: First, the length is determined to be `34`, and since `27 × 58³³ > 66 × 256²⁴` indicates that the index of the first character in the Base58 character table must be less than `27`, and `26 × 58³³ < 65 × 256²⁴` indicates that the index must be greater than or equal to `26`, so the first index can only be `26`, and `26` corresponds to `T` in the Base58 character table (`0` corresponds to `'1'`).

Signature Specification[¶](#signature-specification "Permanent link")
---------------------------------------------------------------------

### Algorithm[¶](#algorithm "Permanent link")

1. Take the `raw_data` of the transaction, convert it to `byte[]` format, and record it as `data` (for example, the `byte[]` type in Java).
2. Perform a `sha256` operation on `data` to get the hash value of the transaction, recorded as `hash`.
3. Use the private key `d` corresponding to the address in the transaction contract to sign the `hash`. The signature algorithm is the ECDSA algorithm (using the SECP256K1 curve). The signature result includes three values: `r`, `s`, and `v`:

   * **Calculate the `r` value**: Randomly generate a temporary private key `k`, calculate the temporary public key `K = k × G` (`G`: curve base point), `r = K_x mod n`, that is, the abscissa of `K` modulo `n`, where `n` is the curve order (`n` and `G` satisfy `n × G = O`, and `O` is the zero point of the elliptic curve group on the finite field). `r` is 32 bytes.
   * **Calculate the `s` value**: First calculate the modular inverse of the temporary private key `k` with respect to `n`, `k⁻¹`, that is, `k⁻¹` satisfies `k⁻¹ × k = 1 mod n`, then calculate the `s` value through the transaction's `hash`, the user's private key `d`, and the `r` value: `s = (k⁻¹ × (hash + d × r)) mod n`. `s` is 32 bytes.
   * **Calculate the `v` value**: Calculate the `recoveryId` first. Due to the modulo operation on the `r` value and the symmetry of the elliptic curve, if there is only the `r` value, one `r` can recover up to four `K`s. The value range of `recoveryId` is `0, 1, 2, 3`. The calculation of the `v` value depends on `recoveryId`. Historically, to remain consistent with Ethereum, the `v` value will be `27` added to `recoveryId`, that is, the value range of `v` is `27, 28, 29, 30`. With a determined `v`, the unique `K` can be recovered. `v` is 1 byte.
4. Concatenate the values of `r`, `s`, and `recoveryId` (or `r`, `s`, and `v`) to obtain the complete signature. The concatenation order shall be `r || s || recoveryId` (or `r || s || v`). At present, both the transaction signing in the official client `wallet-cli` and the block signing in `java-tron` (during block production by SRs) adopt the `recoveryId` scheme, i.e., `signature = r || s || recoveryId`.

### Java Code Sample[¶](#java-code-sample "Permanent link")

```
public static Transaction sign(Transaction transaction, ECKey myKey) {
    Transaction.Builder transactionBuilderSigned = transaction.toBuilder();
    byte[] hash = sha256(transaction.getRawData().toByteArray());
    ECDSASignature signature = myKey.sign(hash);
    ByteString bsSign = ByteString.copyFrom(signature.toByteArray());
    transactionBuilderSigned.addSignature(bsSign);
    return transactionBuilderSigned.build();
}
```

Signature Verification[¶](#signature-verification "Permanent link")
-------------------------------------------------------------------

When a FullNode receives a transaction, it uses the transaction hash and signature to recover a public key via the ECDSA recovery mechanism (ecrecover). An address is then derived from this public key. If the derived address matches the originator's address specified in the transaction, the signature is considered valid.

### Algorithm[¶](#algorithm_1 "Permanent link")

1. Recover the public key point `K`: The temporary public key point `K` can be uniquely recovered through `v` and `r` in the signature.
2. Derive the public key `P`:

   * Known signature equation: `s = k⁻¹(hash + d × r) mod n`
   * Multiply both sides by `k`: `s × k = (hash + d × r) mod n`
   * Multiply both sides by the base point `G` of the curve (using `K = k × G` and `P = d × G`): `s × K = hash × G + r × P`
   * Since `s`, `K`, `hash`, `G`, and `r` are all known, `P` can be obtained.
3. Generate TRON address: same as [TRON Address Generation](#tron-address-generation).
4. Address verification: compare whether the generated TRON address is consistent with the address in the transaction contract.

### Example[¶](#example "Permanent link")

Signature verification is performed by the FullNode. For [ECDSA algorithm signature verification](https://github.com/tronprotocol/java-tron/blob/master/crypto/src/main/java/org/tron/common/crypto/ECKey.java), you can refer to java-tron, and the core function is `signatureToAddress`.

### Signature normalization[¶](#signature-normalization "Permanent link")

ECDSA signatures (using the secp256k1 curve) are malleable, meaning that for a signature (r, s), where r, s \in [1, n-1], the pair (r, n - s) is also a valid signature. Since signatures affect transaction ID in both Bitcoin and Ethereum, [BIP-62](https://github.com/bitcoin/bips/blob/master/bip-0062.mediawiki) and [EIP-2](https://eips.ethereum.org/EIPS/eip-2) require signatures to be normalized, i.e., s \leq n/2. However, for the TRON network, the transaction ID does not include signature information, so there is no strict requirement for signature normalization, and signature verification does not need to check whether the signature is normalized. Although there is no strict restriction, both java-tron and wallet-cli currently perform signature normalization.

June 12, 2026

---

[Previous

Super Representative](../sr/)
[Next

Resource Model](../resource/)