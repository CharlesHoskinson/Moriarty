java-tron



Initializing search

[tronprotocol/documentation-en](https://github.com/tronprotocol/documentation-en "Go to repository")

java-tron

[tronprotocol/documentation-en](https://github.com/tronprotocol/documentation-en "Go to repository")

* [Home](/documentation-en/.)
* Getting Started With java-tron



  Getting Started With java-tron
  + [Getting Started](/documentation-en/getting_started/getting_started_with_javatron/)
* Using java-tron



  Using java-tron
  + [Deploying](/documentation-en/using_javatron/installing_javatron/)
  + [Node Configuration](/documentation-en/using_javatron/configuration/)
  + [Backup and Restore](/documentation-en/using_javatron/backup_restore/)
  + [Lite FullNode](/documentation-en/using_javatron/litefullnode/)
  + [Private Network](/documentation-en/using_javatron/private_network/)
  + [Event Subscription](/documentation-en/architecture/event/)
  + [Database Configuration](/documentation-en/architecture/database/)
  + [Network Connection](/documentation-en/using_javatron/connecting_to_tron/)
  + [Node Logging](/documentation-en/using_javatron/logging/)
  + [Node Monitoring](/documentation-en/using_javatron/metrics/)
  + [Node Maintenance Tool](/documentation-en/using_javatron/toolkit/)
* [API](/documentation-en/api/)

  API
  + [HTTP API](/documentation-en/api/http/)

    HTTP API
    - Account



      Account
      * [Query an account by address](/documentation-en/api/http/account/getaccount/)
      * [Query an account's balance at a specific block](/documentation-en/api/http/account/getaccountbalance/)
      * [Query an account's bandwidth resources](/documentation-en/api/http/account/getaccountnet/)
      * [Query bandwidth + energy + TronPower](/documentation-en/api/http/account/getaccountresource/)
      * [Create an on-chain account](/documentation-en/api/http/account/createaccount/)
      * [Update an account's name](/documentation-en/api/http/account/updateaccount/)
      * [Configure multi-sig permissions](/documentation-en/api/http/account/accountpermissionupdate/)
      * [Validate an address](/documentation-en/api/http/account/validateaddress/)
    - Block / transaction query



      Block / transaction query
      * [Latest block](/documentation-en/api/http/block-and-tx-query/getnowblock/)
      * [Generic block query](/documentation-en/api/http/block-and-tx-query/getblock/)
      * [Block by height](/documentation-en/api/http/block-and-tx-query/getblockbynum/)
      * [Block by hash](/documentation-en/api/http/block-and-tx-query/getblockbyid/)
      * [Blocks in a range](/documentation-en/api/http/block-and-tx-query/getblockbylimitnext/)
      * [The most recent N blocks](/documentation-en/api/http/block-and-tx-query/getblockbylatestnum/)
      * [Per-account balance changes within a block](/documentation-en/api/http/block-and-tx-query/getblockbalance/)
      * [Transaction count in a block](/documentation-en/api/http/block-and-tx-query/gettransactioncountbyblocknum/)
      * [Transaction by txid](/documentation-en/api/http/block-and-tx-query/gettransactionbyid/)
      * [Transaction receipt by txid](/documentation-en/api/http/block-and-tx-query/gettransactioninfobyid/)
      * [Transaction receipts by block](/documentation-en/api/http/block-and-tx-query/gettransactioninfobyblocknum/)
      * [Pending pool size](/documentation-en/api/http/block-and-tx-query/getpendingsize/)
      * [Single pending transaction](/documentation-en/api/http/block-and-tx-query/gettransactionfrompending/)
      * [All pending transaction IDs](/documentation-en/api/http/block-and-tx-query/gettransactionlistfrompending/)
    - Transaction build / broadcast



      Transaction build / broadcast
      * [Build a TRX transfer transaction](/documentation-en/api/http/tx-build-and-broadcast/createtransaction/)
      * [Current multi-sig weight](/documentation-en/api/http/tx-build-and-broadcast/getsignweight/)
      * [Addresses that have already signed](/documentation-en/api/http/tx-build-and-broadcast/getapprovedlist/)
      * [Broadcast a signed transaction (JSON)](/documentation-en/api/http/tx-build-and-broadcast/broadcasttransaction/)
      * [Broadcast a signed transaction (hex)](/documentation-en/api/http/tx-build-and-broadcast/broadcasthex/)
    - TRC10 asset



      TRC10 asset
      * [Issue a TRC10 token](/documentation-en/api/http/asset/createassetissue/)
      * [Update a TRC10 description](/documentation-en/api/http/asset/updateasset/)
      * [Transfer TRC10](/documentation-en/api/http/asset/transferasset/)
      * [Participate in a TRC10 fundraising](/documentation-en/api/http/asset/participateassetissue/)
      * [Unfreeze TRC10 frozen by the issuer](/documentation-en/api/http/asset/unfreezeasset/)
      * [Look up a TRC10 by id](/documentation-en/api/http/asset/getassetissuebyid/)
      * [Look up a TRC10 by name](/documentation-en/api/http/asset/getassetissuebyname/)
      * [All TRC10s with a given name](/documentation-en/api/http/asset/getassetissuelistbyname/)
      * [TRC10s issued by an account](/documentation-en/api/http/asset/getassetissuebyaccount/)
      * [All TRC10s on the network](/documentation-en/api/http/asset/getassetissuelist/)
      * [Paginated TRC10 list](/documentation-en/api/http/asset/getpaginatedassetissuelist/)
    - Smart contract



      Smart contract
      * [Deploy a contract](/documentation-en/api/http/smart-contract/deploycontract/)
      * [Trigger a contract (write)](/documentation-en/api/http/smart-contract/triggersmartcontract/)
      * [Read-only contract call](/documentation-en/api/http/smart-contract/triggerconstantcontract/)
      * [Estimate energy usage of a call](/documentation-en/api/http/smart-contract/estimateenergy/)
      * [Contract metadata](/documentation-en/api/http/smart-contract/getcontract/)
      * [Full contract runtime info](/documentation-en/api/http/smart-contract/getcontractinfo/)
      * [Clear a contract's ABI](/documentation-en/api/http/smart-contract/clearabi/)
      * [Change the user-energy percentage](/documentation-en/api/http/smart-contract/updatesetting/)
      * [Change the deployer's energy limit](/documentation-en/api/http/smart-contract/updateenergylimit/)
    - Witness / governance



      Witness / governance
      * [Apply to become an SR candidate](/documentation-en/api/http/witness-and-governance/createwitness/)
      * [Update an SR's URL](/documentation-en/api/http/witness-and-governance/updatewitness/)
      * [All SR candidates](/documentation-en/api/http/witness-and-governance/listwitnesses/)
      * [Paginated SR list](/documentation-en/api/http/witness-and-governance/getpaginatednowwitnesslist/)
      * [Vote for SRs](/documentation-en/api/http/witness-and-governance/votewitnessaccount/)
      * [An SR's current brokerage rate](/documentation-en/api/http/witness-and-governance/getBrokerage/)
      * [Update an SR's brokerage](/documentation-en/api/http/witness-and-governance/updateBrokerage/)
      * [Claimable rewards for an account](/documentation-en/api/http/witness-and-governance/getReward/)
      * [Withdraw block production rewards / dividends](/documentation-en/api/http/witness-and-governance/withdrawbalance/)
      * [Create a chain-parameter proposal](/documentation-en/api/http/witness-and-governance/proposalcreate/)
      * [Vote on a proposal as an SR](/documentation-en/api/http/witness-and-governance/proposalapprove/)
      * [Withdraw your own proposal](/documentation-en/api/http/witness-and-governance/proposaldelete/)
      * [List of proposals](/documentation-en/api/http/witness-and-governance/listproposals/)
      * [Proposal by ID](/documentation-en/api/http/witness-and-governance/getproposalbyid/)
      * [Paginated proposal list](/documentation-en/api/http/witness-and-governance/getpaginatedproposallist/)
      * [Current chain parameters](/documentation-en/api/http/witness-and-governance/getchainparameters/)
      * [Next maintenance period time](/documentation-en/api/http/witness-and-governance/getnextmaintenancetime/)
    - Stake 1.0 (unfreeze and query only)



      Stake 1.0 (unfreeze and query only)
      * [Freeze TRX for resources (V1)](/documentation-en/api/http/stake-v1/freezebalance/)
      * [Unfreeze matured resources (V1)](/documentation-en/api/http/stake-v1/unfreezebalance/)
      * [Query delegation records (V1)](/documentation-en/api/http/stake-v1/getdelegatedresource/)
      * [Query delegation counterparty addresses (V1)](/documentation-en/api/http/stake-v1/getdelegatedresourceaccountindex/)
    - Stake 2.0



      Stake 2.0
      * [Freeze TRX for resources](/documentation-en/api/http/stake-v2/freezebalancev2/)
      * [Initiate unfreeze](/documentation-en/api/http/stake-v2/unfreezebalancev2/)
      * [Withdraw matured unfreezes](/documentation-en/api/http/stake-v2/withdrawexpireunfreeze/)
      * [Cancel all unmatured unfreezes](/documentation-en/api/http/stake-v2/cancelallunfreezev2/)
      * [Delegate resources to another account](/documentation-en/api/http/stake-v2/delegateresource/)
      * [Undelegate resources from another account](/documentation-en/api/http/stake-v2/undelegateresource/)
      * [Query delegation records](/documentation-en/api/http/stake-v2/getdelegatedresourcev2/)
      * [Query delegation counterparty addresses](/documentation-en/api/http/stake-v2/getdelegatedresourceaccountindexv2/)
      * [Current maximum delegatable amount](/documentation-en/api/http/stake-v2/getcandelegatedmaxsize/)
      * [Remaining unfreeze count](/documentation-en/api/http/stake-v2/getavailableunfreezecount/)
      * [Withdrawable unfreeze amount at a given time](/documentation-en/api/http/stake-v2/getcanwithdrawunfreezeamount/)
    - Node / pricing / tools



      Node / pricing / tools
      * [Node status](/documentation-en/api/http/node-and-tools/getnodeinfo/)
      * [Known peers](/documentation-en/api/http/node-and-tools/listnodes/)
      * [Historical energy unit prices](/documentation-en/api/http/node-and-tools/getenergyprices/)
      * [Historical bandwidth unit prices](/documentation-en/api/http/node-and-tools/getbandwidthprices/)
      * [Cumulative burned TRX](/documentation-en/api/http/node-and-tools/getburntrx/)
  + [gRPC API](/documentation-en/api/rpc/)

    gRPC API
    - Account



      Account
      * [Query an account by address](/documentation-en/api/rpc/account/GetAccount/)
      * [Query an account's balance at a specific block](/documentation-en/api/rpc/account/GetAccountBalance/)
      * [Query an account's bandwidth resources](/documentation-en/api/rpc/account/GetAccountNet/)
      * [Query bandwidth + energy + TronPower](/documentation-en/api/rpc/account/GetAccountResource/)
      * [Create an on-chain account](/documentation-en/api/rpc/account/CreateAccount2/)
      * [Update an account's name](/documentation-en/api/rpc/account/UpdateAccount2/)
      * [Configure multi-sig permissions](/documentation-en/api/rpc/account/AccountPermissionUpdate/)
    - Block / transaction query



      Block / transaction query
      * [Latest block](/documentation-en/api/rpc/block-and-tx-query/GetNowBlock2/)
      * [Generic block query](/documentation-en/api/rpc/block-and-tx-query/GetBlock/)
      * [Block by height](/documentation-en/api/rpc/block-and-tx-query/GetBlockByNum2/)
      * [Block by hash](/documentation-en/api/rpc/block-and-tx-query/GetBlockById/)
      * [Blocks in a range](/documentation-en/api/rpc/block-and-tx-query/GetBlockByLimitNext2/)
      * [The most recent N blocks](/documentation-en/api/rpc/block-and-tx-query/GetBlockByLatestNum2/)
      * [Per-account balance changes within a block](/documentation-en/api/rpc/block-and-tx-query/GetBlockBalanceTrace/)
      * [Transaction count in a block](/documentation-en/api/rpc/block-and-tx-query/GetTransactionCountByBlockNum/)
      * [Transaction by txid](/documentation-en/api/rpc/block-and-tx-query/GetTransactionById/)
      * [Transaction receipt by txid](/documentation-en/api/rpc/block-and-tx-query/GetTransactionInfoById/)
      * [Transaction receipts by block](/documentation-en/api/rpc/block-and-tx-query/GetTransactionInfoByBlockNum/)
      * [Pending pool size](/documentation-en/api/rpc/block-and-tx-query/GetPendingSize/)
      * [Single pending transaction](/documentation-en/api/rpc/block-and-tx-query/GetTransactionFromPending/)
      * [All pending transaction IDs](/documentation-en/api/rpc/block-and-tx-query/GetTransactionListFromPending/)
    - Transaction build / broadcast



      Transaction build / broadcast
      * [Build a TRX transfer transaction](/documentation-en/api/rpc/tx-build-and-broadcast/CreateTransaction2/)
      * [Current multi-sig weight](/documentation-en/api/rpc/tx-build-and-broadcast/GetTransactionSignWeight/)
      * [Addresses that have already signed](/documentation-en/api/rpc/tx-build-and-broadcast/GetTransactionApprovedList/)
      * [Broadcast a signed transaction](/documentation-en/api/rpc/tx-build-and-broadcast/BroadcastTransaction/)
    - TRC10 asset



      TRC10 asset
      * [Issue a TRC10 token](/documentation-en/api/rpc/asset/CreateAssetIssue2/)
      * [Update a TRC10 description](/documentation-en/api/rpc/asset/UpdateAsset2/)
      * [Transfer TRC10](/documentation-en/api/rpc/asset/TransferAsset2/)
      * [Participate in a TRC10 fundraising](/documentation-en/api/rpc/asset/ParticipateAssetIssue2/)
      * [Unfreeze TRC10 frozen by the issuer](/documentation-en/api/rpc/asset/UnfreezeAsset2/)
      * [Look up a TRC10 by id](/documentation-en/api/rpc/asset/GetAssetIssueById/)
      * [Look up a TRC10 by name](/documentation-en/api/rpc/asset/GetAssetIssueByName/)
      * [All TRC10s with a given name](/documentation-en/api/rpc/asset/GetAssetIssueListByName/)
      * [TRC10s issued by an account](/documentation-en/api/rpc/asset/GetAssetIssueByAccount/)
      * [All TRC10s on the network](/documentation-en/api/rpc/asset/GetAssetIssueList/)
      * [Paginated TRC10 list](/documentation-en/api/rpc/asset/GetPaginatedAssetIssueList/)
    - Smart contract



      Smart contract
      * [Deploy a contract](/documentation-en/api/rpc/smart-contract/DeployContract/)
      * [Trigger a contract (write)](/documentation-en/api/rpc/smart-contract/TriggerContract/)
      * [Read-only contract call](/documentation-en/api/rpc/smart-contract/TriggerConstantContract/)
      * [Estimate energy usage of a call](/documentation-en/api/rpc/smart-contract/EstimateEnergy/)
      * [Contract metadata](/documentation-en/api/rpc/smart-contract/GetContract/)
      * [Full contract runtime info](/documentation-en/api/rpc/smart-contract/GetContractInfo/)
      * [Clear a contract's ABI](/documentation-en/api/rpc/smart-contract/ClearContractABI/)
      * [Change the user-energy percentage](/documentation-en/api/rpc/smart-contract/UpdateSetting/)
      * [Change the deployer's energy limit](/documentation-en/api/rpc/smart-contract/UpdateEnergyLimit/)
    - Witness / governance



      Witness / governance
      * [Apply to become an SR candidate](/documentation-en/api/rpc/witness-and-governance/CreateWitness2/)
      * [Update an SR's URL](/documentation-en/api/rpc/witness-and-governance/UpdateWitness2/)
      * [All SR candidates](/documentation-en/api/rpc/witness-and-governance/ListWitnesses/)
      * [Paginated SR list](/documentation-en/api/rpc/witness-and-governance/GetPaginatedNowWitnessList/)
      * [Vote for SRs](/documentation-en/api/rpc/witness-and-governance/VoteWitnessAccount2/)
      * [An SR's current brokerage rate](/documentation-en/api/rpc/witness-and-governance/GetBrokerageInfo/)
      * [Update an SR's brokerage](/documentation-en/api/rpc/witness-and-governance/UpdateBrokerage/)
      * [Claimable rewards for an account](/documentation-en/api/rpc/witness-and-governance/GetRewardInfo/)
      * [Withdraw block production rewards / dividends](/documentation-en/api/rpc/witness-and-governance/WithdrawBalance2/)
      * [Create a chain-parameter proposal](/documentation-en/api/rpc/witness-and-governance/ProposalCreate/)
      * [Vote on a proposal as an SR](/documentation-en/api/rpc/witness-and-governance/ProposalApprove/)
      * [Withdraw your own proposal](/documentation-en/api/rpc/witness-and-governance/ProposalDelete/)
      * [List of proposals](/documentation-en/api/rpc/witness-and-governance/ListProposals/)
      * [Proposal by ID](/documentation-en/api/rpc/witness-and-governance/GetProposalById/)
      * [Paginated proposal list](/documentation-en/api/rpc/witness-and-governance/GetPaginatedProposalList/)
      * [Current chain parameters](/documentation-en/api/rpc/witness-and-governance/GetChainParameters/)
      * [Next maintenance period time](/documentation-en/api/rpc/witness-and-governance/GetNextMaintenanceTime/)
    - Stake 1.0 (unfreeze and query only)



      Stake 1.0 (unfreeze and query only)
      * [Freeze TRX for resources (V1)](/documentation-en/api/rpc/stake-v1/FreezeBalance2/)
      * [Unfreeze matured resources (V1)](/documentation-en/api/rpc/stake-v1/UnfreezeBalance2/)
      * [Query delegation records (V1)](/documentation-en/api/rpc/stake-v1/GetDelegatedResource/)
      * [Query delegation counterparty addresses (V1)](/documentation-en/api/rpc/stake-v1/GetDelegatedResourceAccountIndex/)
    - Stake 2.0



      Stake 2.0
      * [Freeze TRX for resources](/documentation-en/api/rpc/stake-v2/FreezeBalanceV2/)
      * [Initiate unfreeze](/documentation-en/api/rpc/stake-v2/UnfreezeBalanceV2/)
      * [Withdraw matured unfreezes](/documentation-en/api/rpc/stake-v2/WithdrawExpireUnfreeze/)
      * [Cancel all unmatured unfreezes](/documentation-en/api/rpc/stake-v2/CancelAllUnfreezeV2/)
      * [Delegate resources to another account](/documentation-en/api/rpc/stake-v2/DelegateResource/)
      * [Undelegate resources from another account](/documentation-en/api/rpc/stake-v2/UnDelegateResource/)
      * [Query delegation records](/documentation-en/api/rpc/stake-v2/GetDelegatedResourceV2/)
      * [Query delegation counterparty addresses](/documentation-en/api/rpc/stake-v2/GetDelegatedResourceAccountIndexV2/)
      * [Current maximum delegatable amount](/documentation-en/api/rpc/stake-v2/GetCanDelegatedMaxSize/)
      * [Remaining unfreeze count](/documentation-en/api/rpc/stake-v2/GetAvailableUnfreezeCount/)
      * [Withdrawable unfreeze amount at a given time](/documentation-en/api/rpc/stake-v2/GetCanWithdrawUnfreezeAmount/)
    - Node / pricing / tools



      Node / pricing / tools
      * [Node status](/documentation-en/api/rpc/node-and-tools/GetNodeInfo/)
      * [Known peers](/documentation-en/api/rpc/node-and-tools/ListNodes/)
      * [Historical energy unit prices](/documentation-en/api/rpc/node-and-tools/GetEnergyPrices/)
      * [Historical bandwidth unit prices](/documentation-en/api/rpc/node-and-tools/GetBandwidthPrices/)
      * [Cumulative burned TRX](/documentation-en/api/rpc/node-and-tools/GetBurnTrx/)
  + [jsonRPC API](/documentation-en/api/json-rpc/)

    jsonRPC API
    - Node info / chain identity



      Node info / chain identity
      * [Client version string](/documentation-en/api/json-rpc/node/web3_clientVersion/)
      * [Keccak-256 hash](/documentation-en/api/json-rpc/node/web3_sha3/)
      * [Network ID](/documentation-en/api/json-rpc/node/net_version/)
      * [Whether listening on P2P](/documentation-en/api/json-rpc/node/net_listening/)
      * [Number of peers](/documentation-en/api/json-rpc/node/net_peerCount/)
      * [Chain ID](/documentation-en/api/json-rpc/node/eth_chainId/)
      * [Protocol version](/documentation-en/api/json-rpc/node/eth_protocolVersion/)
      * [Sync status](/documentation-en/api/json-rpc/node/eth_syncing/)
      * [Latest block height](/documentation-en/api/json-rpc/node/eth_blockNumber/)
      * [Current energy unit price](/documentation-en/api/json-rpc/node/eth_gasPrice/)
    - Block / transaction query



      Block / transaction query
      * [Query a block by hash](/documentation-en/api/json-rpc/block-and-tx-query/eth_getBlockByHash/)
      * [Query a block by height / tag](/documentation-en/api/json-rpc/block-and-tx-query/eth_getBlockByNumber/)
      * [Block transaction count (by hash)](/documentation-en/api/json-rpc/block-and-tx-query/eth_getBlockTransactionCountByHash/)
      * [Block transaction count (by height)](/documentation-en/api/json-rpc/block-and-tx-query/eth_getBlockTransactionCountByNumber/)
      * [Query a transaction by txid](/documentation-en/api/json-rpc/block-and-tx-query/eth_getTransactionByHash/)
      * [Query a transaction by block hash + index](/documentation-en/api/json-rpc/block-and-tx-query/eth_getTransactionByBlockHashAndIndex/)
      * [Query a transaction by block height + index](/documentation-en/api/json-rpc/block-and-tx-query/eth_getTransactionByBlockNumberAndIndex/)
      * [Query a receipt by txid](/documentation-en/api/json-rpc/block-and-tx-query/eth_getTransactionReceipt/)
      * [Receipt list for an entire block](/documentation-en/api/json-rpc/block-and-tx-query/eth_getBlockReceipts/)
    - Account state



      Account state
      * [Account TRX balance](/documentation-en/api/json-rpc/account/eth_getBalance/)
      * [Contract storage slot](/documentation-en/api/json-rpc/account/eth_getStorageAt/)
      * [Contract runtime bytecode](/documentation-en/api/json-rpc/account/eth_getCode/)
    - Smart contract calls



      Smart contract calls
      * [Read-only contract call](/documentation-en/api/json-rpc/smart-contract/eth_call/)
      * [Estimate energy consumption](/documentation-en/api/json-rpc/smart-contract/eth_estimateGas/)
    - Logs / filters



      Logs / filters
      * [One-shot log query](/documentation-en/api/json-rpc/filter/eth_getLogs/)
      * [Register a log filter](/documentation-en/api/json-rpc/filter/eth_newFilter/)
      * [Register a new-block filter](/documentation-en/api/json-rpc/filter/eth_newBlockFilter/)
      * [Uninstall a filter](/documentation-en/api/json-rpc/filter/eth_uninstallFilter/)
      * [Pull and drain filter increments](/documentation-en/api/json-rpc/filter/eth_getFilterChanges/)
      * [Pull a log filter's full set](/documentation-en/api/json-rpc/filter/eth_getFilterLogs/)
    - Transaction build



      Transaction build
      * [Build an unsigned transaction](/documentation-en/api/json-rpc/tx-build/buildTransaction/)
    - Compatibility stub methods



      Compatibility stub methods
      * [etherbase address](/documentation-en/api/json-rpc/stub/eth_coinbase/)
      * [Node-managed accounts list](/documentation-en/api/json-rpc/stub/eth_accounts/)
      * [Mining work info](/documentation-en/api/json-rpc/stub/eth_getWork/)
  + [Machine-readable definitions](/documentation-en/api/interface-definitions/)
* Core Protocol



  Core Protocol
  + [DPoS](/documentation-en/mechanism-algorithm/dpos/)
  + [Super Representative](/documentation-en/mechanism-algorithm/sr/)
  + [Account Model](/documentation-en/mechanism-algorithm/account/)
  + [Resource Model](/documentation-en/mechanism-algorithm/resource/)
  + [Smart Contract](/documentation-en/contracts/contract/)
  + [System Contract](/documentation-en/mechanism-algorithm/system-contracts/)
  + [Account Permission Management](/documentation-en/mechanism-algorithm/multi-signatures/)
* For java-tron Developers



  For java-tron Developers
  + [Developer Guide](/documentation-en/developers/java-tron/)
  + [CI Workflows](/documentation-en/developers/workflows/)
  + [Core Modules](/documentation-en/developers/code-structure/)
  + [ChainBase Deep Dive](/documentation-en/developers/chainbase/)
  + [P2P Network Deep Dive](/documentation-en/developers/network/)
  + [TIPs Workflow](/documentation-en/developers/tip-workflow/)
  + [TIPs](/documentation-en/developers/tips/)
  + [Issue Workflow](/documentation-en/developers/issue-workflow/)
  + [Governance Workflow](/documentation-en/developers/governance/)
  + [Configure the IDE](/documentation-en/developers/run-in-idea/)
  + [Development Example](/documentation-en/developers/demo/)
* For Dapp Developers



  For Dapp Developers
  + [Tools](/documentation-en/contracts/tools/)
* Clients



  Clients
  + [wallet-cli](/documentation-en/clients/wallet-cli/)

    wallet-cli
    - [Java CLI](/documentation-en/clients/wallet-cli/java/)

      Java CLI
      * [Guides](/documentation-en/clients/wallet-cli/java/guide/)

        Guides
        + [Getting started](/documentation-en/clients/wallet-cli/java/guide/getting-started/)
        + [Command-line operation flow](/documentation-en/clients/wallet-cli/java/guide/command-flow/)
      * [Concepts](/documentation-en/clients/wallet-cli/java/concepts/)

        Concepts
        + [Resources: bandwidth, energy & shares](/documentation-en/clients/wallet-cli/java/concepts/resources/)
        + [Staking models: Stake 1.0 vs 2.0](/documentation-en/clients/wallet-cli/java/concepts/staking-models/)
        + [Multi-signature concepts](/documentation-en/clients/wallet-cli/java/concepts/multisig/)
      * [Command reference](/documentation-en/clients/wallet-cli/java/commands/)

        Command reference
        + [Wallet management](/documentation-en/clients/wallet-cli/java/commands/wallet/)
        + [Account commands](/documentation-en/clients/wallet-cli/java/commands/account/)
        + [Network commands](/documentation-en/clients/wallet-cli/java/commands/network/)
        + [TRC10 tokens](/documentation-en/clients/wallet-cli/java/commands/transfer-trc10/)
        + [USDT & TRC20 transfers](/documentation-en/clients/wallet-cli/java/commands/usdt/)
        + [Staking (Stake 2.0)](/documentation-en/clients/wallet-cli/java/commands/stake-v2/)
        + [Staking (Stake 1.0, legacy)](/documentation-en/clients/wallet-cli/java/commands/stake-v1-legacy/)
        + [Resource prices & memo fee](/documentation-en/clients/wallet-cli/java/commands/resources/)
        + [Voting, rewards & witnesses](/documentation-en/clients/wallet-cli/java/commands/vote-reward/)
        + [Proposals](/documentation-en/clients/wallet-cli/java/commands/proposals/)
        + [Exchange (Bancor)](/documentation-en/clients/wallet-cli/java/commands/exchange/)
        + [TRON-DEX market](/documentation-en/clients/wallet-cli/java/commands/dex/)
        + [Multi-signature](/documentation-en/clients/wallet-cli/java/commands/multisig/)
        + [Smart contracts](/documentation-en/clients/wallet-cli/java/commands/contract/)
        + [GasFree transfers](/documentation-en/clients/wallet-cli/java/commands/gasfree/)
        + [Chain data & utilities](/documentation-en/clients/wallet-cli/java/commands/chain-data/)
      * [Configuration reference](/documentation-en/clients/wallet-cli/java/reference/config/)
    - [TypeScript / npm CLI](/documentation-en/clients/wallet-cli/typescript/)

      TypeScript / npm CLI
      * [Guides](/documentation-en/clients/wallet-cli/typescript/guide/)

        Guides
        + [Getting Started](/documentation-en/clients/wallet-cli/typescript/guide/getting-started/)
        + [Sending TRX and Tokens](/documentation-en/clients/wallet-cli/typescript/guide/send-tokens/)
        + [Staking and Resources](/documentation-en/clients/wallet-cli/typescript/guide/stake-and-resources/)
        + [Using a Ledger Hardware Wallet](/documentation-en/clients/wallet-cli/typescript/guide/ledger/)
        + [Scripting wallet-cli](/documentation-en/clients/wallet-cli/typescript/guide/scripting/)
      * [Concepts](/documentation-en/clients/wallet-cli/typescript/concepts/)

        Concepts
        + [Networks](/documentation-en/clients/wallet-cli/typescript/concepts/networks/)
        + [Accounts and HD Wallets](/documentation-en/clients/wallet-cli/typescript/concepts/accounts-and-hd/)
        + [Energy and Bandwidth](/documentation-en/clients/wallet-cli/typescript/concepts/energy-bandwidth/)
        + [Security Model](/documentation-en/clients/wallet-cli/typescript/concepts/security/)
      * [Command reference](/documentation-en/clients/wallet-cli/typescript/commands/)

        Command reference
        + [create](/documentation-en/clients/wallet-cli/typescript/commands/create/)
        + [list](/documentation-en/clients/wallet-cli/typescript/commands/list/)
        + [use](/documentation-en/clients/wallet-cli/typescript/commands/use/)
        + [current](/documentation-en/clients/wallet-cli/typescript/commands/current/)
        + [derive](/documentation-en/clients/wallet-cli/typescript/commands/derive/)
        + [rename](/documentation-en/clients/wallet-cli/typescript/commands/rename/)
        + [backup](/documentation-en/clients/wallet-cli/typescript/commands/backup/)
        + [delete](/documentation-en/clients/wallet-cli/typescript/commands/delete/)
        + [change-password](/documentation-en/clients/wallet-cli/typescript/commands/change-password/)
        + [block](/documentation-en/clients/wallet-cli/typescript/commands/block/)
        + [networks](/documentation-en/clients/wallet-cli/typescript/commands/networks/)
        + [config](/documentation-en/clients/wallet-cli/typescript/commands/config/)
        + [account](/documentation-en/clients/wallet-cli/typescript/commands/account/)

          account
          - [account activate](/documentation-en/clients/wallet-cli/typescript/commands/account/activate/)
          - [account balance](/documentation-en/clients/wallet-cli/typescript/commands/account/balance/)
          - [account history](/documentation-en/clients/wallet-cli/typescript/commands/account/history/)
          - [account info](/documentation-en/clients/wallet-cli/typescript/commands/account/info/)
          - [account portfolio](/documentation-en/clients/wallet-cli/typescript/commands/account/portfolio/)
          - [account set](/documentation-en/clients/wallet-cli/typescript/commands/account/set/)
        + [address](/documentation-en/clients/wallet-cli/typescript/commands/address/)

          address
          - [address generate](/documentation-en/clients/wallet-cli/typescript/commands/address/generate/)
        + [asset](/documentation-en/clients/wallet-cli/typescript/commands/asset/)

          asset
          - [asset info](/documentation-en/clients/wallet-cli/typescript/commands/asset/info/)
          - [asset issue](/documentation-en/clients/wallet-cli/typescript/commands/asset/issue/)
          - [asset list](/documentation-en/clients/wallet-cli/typescript/commands/asset/list/)
          - [asset participate](/documentation-en/clients/wallet-cli/typescript/commands/asset/participate/)
          - [asset unfreeze](/documentation-en/clients/wallet-cli/typescript/commands/asset/unfreeze/)
          - [asset update](/documentation-en/clients/wallet-cli/typescript/commands/asset/update/)
        + [chain](/documentation-en/clients/wallet-cli/typescript/commands/chain/)

          chain
          - [chain node](/documentation-en/clients/wallet-cli/typescript/commands/chain/node/)
          - [chain params](/documentation-en/clients/wallet-cli/typescript/commands/chain/params/)
          - [chain prices](/documentation-en/clients/wallet-cli/typescript/commands/chain/prices/)
        + [contact](/documentation-en/clients/wallet-cli/typescript/commands/contact/)

          contact
          - [contact add](/documentation-en/clients/wallet-cli/typescript/commands/contact/add/)
          - [contact list](/documentation-en/clients/wallet-cli/typescript/commands/contact/list/)
          - [contact remove](/documentation-en/clients/wallet-cli/typescript/commands/contact/remove/)
        + [contract](/documentation-en/clients/wallet-cli/typescript/commands/contract/)

          contract
          - [contract call](/documentation-en/clients/wallet-cli/typescript/commands/contract/call/)
          - [contract clear-abi](/documentation-en/clients/wallet-cli/typescript/commands/contract/clear-abi/)
          - [contract create2](/documentation-en/clients/wallet-cli/typescript/commands/contract/create2/)
          - [contract deploy](/documentation-en/clients/wallet-cli/typescript/commands/contract/deploy/)
          - [contract info](/documentation-en/clients/wallet-cli/typescript/commands/contract/info/)
          - [contract send](/documentation-en/clients/wallet-cli/typescript/commands/contract/send/)
          - [contract set-origin-energy-limit](/documentation-en/clients/wallet-cli/typescript/commands/contract/set-origin-energy-limit/)
          - [contract set-user-resource-percent](/documentation-en/clients/wallet-cli/typescript/commands/contract/set-user-resource-percent/)
        + [encoding](/documentation-en/clients/wallet-cli/typescript/commands/encoding/)

          encoding
          - [encoding convert](/documentation-en/clients/wallet-cli/typescript/commands/encoding/convert/)
        + [exchange](/documentation-en/clients/wallet-cli/typescript/commands/exchange/)

          exchange
          - [exchange create](/documentation-en/clients/wallet-cli/typescript/commands/exchange/create/)
          - [exchange inject](/documentation-en/clients/wallet-cli/typescript/commands/exchange/inject/)
          - [exchange list](/documentation-en/clients/wallet-cli/typescript/commands/exchange/list/)
          - [exchange show](/documentation-en/clients/wallet-cli/typescript/commands/exchange/show/)
          - [exchange trade](/documentation-en/clients/wallet-cli/typescript/commands/exchange/trade/)
          - [exchange withdraw](/documentation-en/clients/wallet-cli/typescript/commands/exchange/withdraw/)
        + [gasfree](/documentation-en/clients/wallet-cli/typescript/commands/gasfree/)

          gasfree
          - [gasfree info](/documentation-en/clients/wallet-cli/typescript/commands/gasfree/info/)
          - [gasfree trace](/documentation-en/clients/wallet-cli/typescript/commands/gasfree/trace/)
          - [gasfree transfer](/documentation-en/clients/wallet-cli/typescript/commands/gasfree/transfer/)
        + [import](/documentation-en/clients/wallet-cli/typescript/commands/import/)

          import
          - [import keystore](/documentation-en/clients/wallet-cli/typescript/commands/import/keystore/)
          - [import ledger](/documentation-en/clients/wallet-cli/typescript/commands/import/ledger/)
          - [import mnemonic](/documentation-en/clients/wallet-cli/typescript/commands/import/mnemonic/)
          - [import private-key](/documentation-en/clients/wallet-cli/typescript/commands/import/private-key/)
          - [import watch](/documentation-en/clients/wallet-cli/typescript/commands/import/watch/)
        + [message](/documentation-en/clients/wallet-cli/typescript/commands/message/)

          message
          - [message sign](/documentation-en/clients/wallet-cli/typescript/commands/message/sign/)
        + [permission](/documentation-en/clients/wallet-cli/typescript/commands/permission/)

          permission
          - [permission show](/documentation-en/clients/wallet-cli/typescript/commands/permission/show/)
          - [permission update](/documentation-en/clients/wallet-cli/typescript/commands/permission/update/)
        + [proposal](/documentation-en/clients/wallet-cli/typescript/commands/proposal/)

          proposal
          - [proposal approve](/documentation-en/clients/wallet-cli/typescript/commands/proposal/approve/)
          - [proposal create](/documentation-en/clients/wallet-cli/typescript/commands/proposal/create/)
          - [proposal delete](/documentation-en/clients/wallet-cli/typescript/commands/proposal/delete/)
          - [proposal list](/documentation-en/clients/wallet-cli/typescript/commands/proposal/list/)
          - [proposal show](/documentation-en/clients/wallet-cli/typescript/commands/proposal/show/)
        + [reward](/documentation-en/clients/wallet-cli/typescript/commands/reward/)

          reward
          - [reward balance](/documentation-en/clients/wallet-cli/typescript/commands/reward/balance/)
          - [reward withdraw](/documentation-en/clients/wallet-cli/typescript/commands/reward/withdraw/)
        + [stake](/documentation-en/clients/wallet-cli/typescript/commands/stake/)

          stake
          - [stake cancel-unfreeze](/documentation-en/clients/wallet-cli/typescript/commands/stake/cancel-unfreeze/)
          - [stake delegate](/documentation-en/clients/wallet-cli/typescript/commands/stake/delegate/)
          - [stake delegated](/documentation-en/clients/wallet-cli/typescript/commands/stake/delegated/)
          - [stake freeze](/documentation-en/clients/wallet-cli/typescript/commands/stake/freeze/)
          - [stake info](/documentation-en/clients/wallet-cli/typescript/commands/stake/info/)
          - [stake undelegate](/documentation-en/clients/wallet-cli/typescript/commands/stake/undelegate/)
          - [stake unfreeze](/documentation-en/clients/wallet-cli/typescript/commands/stake/unfreeze/)
          - [stake withdraw](/documentation-en/clients/wallet-cli/typescript/commands/stake/withdraw/)
        + [token](/documentation-en/clients/wallet-cli/typescript/commands/token/)

          token
          - [token add](/documentation-en/clients/wallet-cli/typescript/commands/token/add/)
          - [token balance](/documentation-en/clients/wallet-cli/typescript/commands/token/balance/)
          - [token info](/documentation-en/clients/wallet-cli/typescript/commands/token/info/)
          - [token list](/documentation-en/clients/wallet-cli/typescript/commands/token/list/)
          - [token remove](/documentation-en/clients/wallet-cli/typescript/commands/token/remove/)
        + [tx](/documentation-en/clients/wallet-cli/typescript/commands/tx/)

          tx
          - [tx approvals](/documentation-en/clients/wallet-cli/typescript/commands/tx/approvals/)
          - [tx broadcast](/documentation-en/clients/wallet-cli/typescript/commands/tx/broadcast/)
          - [tx info](/documentation-en/clients/wallet-cli/typescript/commands/tx/info/)
          - [tx multisig](/documentation-en/clients/wallet-cli/typescript/commands/tx/multisig/)
          - [tx send](/documentation-en/clients/wallet-cli/typescript/commands/tx/send/)
          - [tx sign](/documentation-en/clients/wallet-cli/typescript/commands/tx/sign/)
          - [tx status](/documentation-en/clients/wallet-cli/typescript/commands/tx/status/)
        + [typed-data](/documentation-en/clients/wallet-cli/typescript/commands/typed-data/)

          typed-data
          - [typed-data sign](/documentation-en/clients/wallet-cli/typescript/commands/typed-data/sign/)
        + [vote](/documentation-en/clients/wallet-cli/typescript/commands/vote/)

          vote
          - [vote cast](/documentation-en/clients/wallet-cli/typescript/commands/vote/cast/)
          - [vote list](/documentation-en/clients/wallet-cli/typescript/commands/vote/list/)
          - [vote status](/documentation-en/clients/wallet-cli/typescript/commands/vote/status/)
        + [witness](/documentation-en/clients/wallet-cli/typescript/commands/witness/)

          witness
          - [witness create](/documentation-en/clients/wallet-cli/typescript/commands/witness/create/)
          - [witness set-brokerage](/documentation-en/clients/wallet-cli/typescript/commands/witness/set-brokerage/)
          - [witness update](/documentation-en/clients/wallet-cli/typescript/commands/witness/update/)
      * [Machine Interface](/documentation-en/clients/wallet-cli/typescript/machine-interface/)
      * [Troubleshooting](/documentation-en/clients/wallet-cli/typescript/troubleshooting/)
* Releases



  Releases
  + [Deployment Manual for the New Version](/documentation-en/releases/upgrade-instruction/)
  + [Integrity Check](/documentation-en/releases/signature_verification/)
  + [History](/documentation-en/releases/versions/)

    History
    - [v4.8.2 (Pyrrho)](/documentation-en/releases/versions/v4.8.2/)
    - [v4.8.1 (Democritus)](/documentation-en/releases/versions/v4.8.1/)
    - [v4.8.0.1 (Seneca)](/documentation-en/releases/versions/v4.8.0.1/)
    - [v4.8.0 (Kant)](/documentation-en/releases/versions/v4.8.0/)
    - [v4.7.7 (Epicurus)](/documentation-en/releases/versions/v4.7.7/)
    - [v4.7.6 (Anaximander)](/documentation-en/releases/versions/v4.7.6/)
    - [v4.7.5 (Cleobulus)](/documentation-en/releases/versions/v4.7.5/)
    - [v4.7.4 (Bias)](/documentation-en/releases/versions/v4.7.4/)
    - [v4.7.3.1 (Solon)](/documentation-en/releases/versions/v4.7.3.1/)
    - [v4.7.3 (Chilon)](/documentation-en/releases/versions/v4.7.3/)
    - [v4.7.2 (Periander)](/documentation-en/releases/versions/v4.7.2/)
    - [v4.7.1.1 (Pittacus)](/documentation-en/releases/versions/v4.7.1.1/)
    - [v4.7.1 (Sartre)](/documentation-en/releases/versions/v4.7.1/)
    - [v4.7.0.1 (Aristotle)](/documentation-en/releases/versions/v4.7.0.1/)
    - [v4.6.0 (Socrates)](/documentation-en/releases/versions/v4.6.0/)
    - [v4.5.2 (Aurelius)](/documentation-en/releases/versions/v4.5.2/)
    - [v4.5.1 (Tertullian)](/documentation-en/releases/versions/v4.5.1/)
    - [v4.4.6 (David)](/documentation-en/releases/versions/v4.4.6/)
    - [v4.4.5 (Cicero)](/documentation-en/releases/versions/v4.4.5/)
    - [v4.4.4 (Plotinus)](/documentation-en/releases/versions/v4.4.4/)
    - [v4.4.2 (Augustinus)](/documentation-en/releases/versions/v4.4.2/)
    - [v4.4.0 (Rousseau)](/documentation-en/releases/versions/v4.4.0/)
    - [v4.3.0 (Bacon)](/documentation-en/releases/versions/v4.3.0/)
    - [v4.2.2.1 (Epictetus)](/documentation-en/releases/versions/v4.2.2.1/)
    - [v4.2.2 (Lucretius)](/documentation-en/releases/versions/v4.2.2/)
    - [v4.2.0 (Plato)](/documentation-en/releases/versions/v4.2.0/)
    - [v4.1.3 (Thales)](/documentation-en/releases/versions/v4.1.3/)
    - [v4.1.2](/documentation-en/releases/versions/v4.1.2/)
    - [v4.1.1](/documentation-en/releases/versions/v4.1.1/)
    - [v4.0.0](/documentation-en/releases/versions/v4.0.0/)
    - [Odyssey-v3.7](/documentation-en/releases/versions/v3.7/)
    - [Odyssey-v3.6.5](/documentation-en/releases/versions/v3.6.5/)
* Appendix



  Appendix
  + [Glossary](/documentation-en/glossary/)

404 - Not found
===============

Redirecting to [HomePage](/documentation-en/)...