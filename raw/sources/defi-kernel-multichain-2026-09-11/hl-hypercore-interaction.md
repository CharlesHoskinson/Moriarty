[![](https://hyperliquid.gitbook.io/hyperliquid-docs/~gitbook/image?url=https%3A%2F%2F2356094849-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252FyUdp569E6w18GdfqlGvJ%252Ficon%252FsIAjqhKKIUysM08ahKPh%252FHL-logoSwitchDISliStat.png%3Falt%3Dmedia%26token%3Da81fa25c-0510-4d97-87ff-3fb8944935b1&width=32&dpr=3&quality=100&sign=587b41e3b6e75de5fb66d649a4ba3c1a&sv=3)![](https://hyperliquid.gitbook.io/hyperliquid-docs/~gitbook/image?url=https%3A%2F%2F2356094849-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252FyUdp569E6w18GdfqlGvJ%252Ficon%252FsIAjqhKKIUysM08ahKPh%252FHL-logoSwitchDISliStat.png%3Falt%3Dmedia%26token%3Da81fa25c-0510-4d97-87ff-3fb8944935b1&width=32&dpr=3&quality=100&sign=587b41e3b6e75de5fb66d649a4ba3c1a&sv=3)

Hyperliquid Docs](/hyperliquid-docs)

`⌘Ctrl``k`

* [Hyperliquid Docs](/hyperliquid-docs)
* [Builder Tools](/hyperliquid-docs/builder-tools)
* [Support](/hyperliquid-docs/support)

For the complete documentation index, see [llms.txt](https://hyperliquid.gitbook.io/hyperliquid-docs/llms.txt). This page is also available as [Markdown](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/hyperevm/interacting-with-hypercore.md).

Copy

On this page

1. [Hyperliquid Docs](/hyperliquid-docs)
2. [For developers](/hyperliquid-docs/for-developers)
3. [HyperEVM](/hyperliquid-docs/for-developers/hyperevm)

Interacting with HyperCore
==========================

### Read precompiles

The testnet EVM provides read precompiles that allows querying HyperCore information. The precompile addresses start at 0x0000000000000000000000000000000000000800 and have methods for querying information such as perps positions, spot balances, vault equity, staking delegations, oracle prices, and the L1 block number.

The values are guaranteed to match the latest HyperCore state at the time the EVM block is constructed.

Attached is a Solidity file `L1Read.sol` describing the read precompiles. As an example, this call queries the third perp oracle price on testnet:

Copy

```
cast call 0x0000000000000000000000000000000000000807 0x0000000000000000000000000000000000000000000000000000000000000003 --rpc-url https://rpc.hyperliquid-testnet.xyz/evm
```

To convert to floating point numbers, divide the returned price by `10^(6 - szDecimals)`for perps and `10^(8 - base asset szDecimals)` for spot.

Precompiles called on invalid inputs such as invalid assets or vault address will return an error and consume all gas passed into the precompile call frame. Precompiles have a gas cost of `2000 + 65 * (input_len + output_len)`.

### CoreWriter contract

A system contract is available at 0x3333333333333333333333333333333333333333 for sending transactions from the HyperEVM to HyperCore. It burns ~25,000 gas before emitting a log to be processed by HyperCore as an action. In practice the gas usage for a basic call will be ~47000. A solidity file `CoreWriter.sol` for the write system contract is attached.

#### Action encoding details

* Byte 1: Encoding version

  + Currently, only version `1` is supported, but enables future upgrades while maintaining backward compatibility.
* Bytes 2-4: Action ID

  + These three bytes, when decoded as a big-endian unsigned integer, represent the unique identifier for the action.
* Remaining bytes: Action encoding

  + The rest of the bytes constitue the action-specific data. It is always the raw ABI encoding of a sequence of Solidity types

To prevent any potential latency advantages for using HyperEVM to bypass the L1 mempool, order actions and vault transfers sent from CoreWriter are delayed onchain for a few seconds. This has no noticeable effect on UX because the end user has to wait for at least one small block confirmation. These onchain-delayed actions appear twice in the L1 explorer: first as an enqueuing and second as a HyperCore execution.

Action ID

Action

Fields

Solidity Type

Notes

1

Limit order

(asset, isBuy, limitPx, sz, reduceOnly, encodedTif, cloid)

(uint32, bool, uint64, uint64, bool, uint8, uint128)

Tif encoding: `1` for `Alo` , `2` for `Gtc` , `3` for `Ioc` . Cloid encoding: 0 means no cloid, otherwise uses the number as the cloid. limitPx and sz should be sent as 10^8 \* the human readable value

2

Vault transfer

(vault, isDeposit, usd)

(address, bool, uint64)

3

Token delegate

(validator, wei, isUndelegate)

(address, uint64, bool)

4

Staking deposit

wei

uint64

5

Staking withdraw

wei

uint64

6

Spot send

(destination, token, wei)

(address, uint64, uint64)

7

USD class transfer

(ntl, toPerp)

(uint64, bool)

8

Finalize EVM Contract

(token, encodedFinalizeEvmContractVariant, createNonce)

(uint64, uint8, uint64)

encodedFinalizeEvmContractVariant `1` for `Create`, `2` for `FirstStorageSlot` , `3` for `CustomStorageSlot` . If `Create` variant, then `createNonce` input argument is used.

9

Add API wallet

(API wallet address, API wallet name)

(address, string)

If the API wallet name is empty then this becomes the main API wallet / agent

10

Cancel order by oid

(asset, oid)

(uint32, uint64)

11

Cancel order by cloid

(asset, cloid)

(uint32, uint128)

12

Approve builder fee

(maxFeeRate, builder address)

(uint64, address)

maxFeeRate is in decibps. To approve a builder fee of 0.01% maxFreeRate should be 10.

13

Send asset

(destination, subAccount, source\_dex, destination\_dex, token, wei)

(address, address, uint32, uint32, uint64, uint64)

If subAccount is not the zero address, then transfer from subAccount. Specify uint32::MAX for the source\_dex or destination\_dex for spot.

15

Borrow lend operation

(encodedOperation, token, wei)

(uint8, uint64, uint64)

encodedOperation `0` for `Supply`, `1` for `Withdraw` . If `wei` is 0 then maximally apply the operation, e.g. withdraw full balance from reserve.

16

Set abstraction

(user, abstraction)

(address, uint8)

abstraction `1` for `disabled`, `2` for `unifiedAccount` , `3` for `portfolioMargin` . `user` can either be the master user or a sub-account.

17

Outcome operation

(encodedOperation, question, outcome, wei)

(uint8, uint32, uint32, uint64)

encodedOperation `0` for `SplitOutcome`, `1` for `MergeOutcome`, `2` for `MergeQuestion`, `3` for `NegateOutcome`. Unused fields must be set to 0: `SplitOutcome` and `MergeOutcome` ignore question; `MergeQuestion` ignores outcome. negate uses both. Payloads with non-zero unused fields are dropped. `wei` must be a multiple of size. For the merge operations 0 means merge the maximal amount.

Below is an example contract that would send an action on behalf of its own contract address on HyperCore, which also demonstrates one way to construct the encoded action in Solidity.

Happy building. Any feedback is appreciated.

[L1Read.sol](https://2356094849-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2FyUdp569E6w18GdfqlGvJ%2Fuploads%2FLXZLGWjNoZCV4IpfEQUJ%2FL1Read.sol?alt=media&token=f19b1226-95bc-4ad4-9bbc-a55665676b92)

12KB

Download[Open](https://2356094849-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2FyUdp569E6w18GdfqlGvJ%2Fuploads%2FLXZLGWjNoZCV4IpfEQUJ%2FL1Read.sol?alt=media&token=f19b1226-95bc-4ad4-9bbc-a55665676b92)

[CoreWriter.sol](https://2356094849-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2FyUdp569E6w18GdfqlGvJ%2Fuploads%2F5eRfywPQ5JlFkb0pGjJf%2FCoreWriter.sol?alt=media&token=0993d3f8-ae92-4c59-81e3-12d58bb9ec96)

298B

Download[Open](https://2356094849-files.gitbook.io/~/files/v0/b/gitbook-x-prod.appspot.com/o/spaces%2FyUdp569E6w18GdfqlGvJ%2Fuploads%2F5eRfywPQ5JlFkb0pGjJf%2FCoreWriter.sol?alt=media&token=0993d3f8-ae92-4c59-81e3-12d58bb9ec96)

[PreviousRaw HyperEVM block data](/hyperliquid-docs/for-developers/hyperevm/raw-hyperevm-block-data)[NextHyperCore <> HyperEVM transfers](/hyperliquid-docs/for-developers/hyperevm/hypercore-less-than-greater-than-hyperevm-transfers)

Last updated 22 days ago

* [Read precompiles](#read-precompiles)
* [CoreWriter contract](#corewriter-contract)

Copy

```
contract CoreWriterCaller {
    function sendUsdClassTransfer(uint64 ntl, bool toPerp) external {
        bytes memory encodedAction = abi.encode(ntl, toPerp);
        bytes memory data = new bytes(4 + encodedAction.length);
        data[0] = 0x01;
        data[1] = 0x00;
        data[2] = 0x00;
        data[3] = 0x07;
        for (uint256 i = 0; i < encodedAction.length; i++) {
            data[4 + i] = encodedAction[i];
        }
        CoreWriter(0x3333333333333333333333333333333333333333).sendRawAction(data);
    }
}
```