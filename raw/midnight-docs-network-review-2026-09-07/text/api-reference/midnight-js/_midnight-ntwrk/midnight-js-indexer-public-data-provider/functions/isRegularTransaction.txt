# isRegularTransaction

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-indexer-public-data-provider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider.md) / isRegularTransaction

# Function: isRegularTransaction()

> **isRegularTransaction**(`tx`): `tx is Transaction & { block: Block; contractActions: readonly ContractAction[]; dustLedgerEvents: readonly DustLedgerEvent[]; endIndex: number; fees: TransactionFees; hash: string; id: number; identifiers: readonly string[]; merkleTreeRoot: string; protocolVersion: number; raw: string; startIndex: number; transactionResult: TransactionResult; unshieldedCreatedOutputs: readonly UnshieldedUtxo[]; unshieldedSpentOutputs: readonly UnshieldedUtxo[]; zswapLedgerEvents: readonly ZswapLedgerEvent[] } & { hash: string; identifiers: string[] }`

## Parameters[​](#parameters "Direct link to Parameters")

### tx[​](#tx "Direct link to tx")

`any`

## Returns[​](#returns "Direct link to Returns")

`tx is Transaction & { block: Block; contractActions: readonly ContractAction[]; dustLedgerEvents: readonly DustLedgerEvent[]; endIndex: number; fees: TransactionFees; hash: string; id: number; identifiers: readonly string[]; merkleTreeRoot: string; protocolVersion: number; raw: string; startIndex: number; transactionResult: TransactionResult; unshieldedCreatedOutputs: readonly UnshieldedUtxo[]; unshieldedSpentOutputs: readonly UnshieldedUtxo[]; zswapLedgerEvents: readonly ZswapLedgerEvent[] } & { hash: string; identifiers: string[] }`
