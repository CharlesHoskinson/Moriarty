[![](https://hyperliquid.gitbook.io/hyperliquid-docs/~gitbook/image?url=https%3A%2F%2F2356094849-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252FyUdp569E6w18GdfqlGvJ%252Ficon%252FsIAjqhKKIUysM08ahKPh%252FHL-logoSwitchDISliStat.png%3Falt%3Dmedia%26token%3Da81fa25c-0510-4d97-87ff-3fb8944935b1&width=32&dpr=3&quality=100&sign=587b41e3b6e75de5fb66d649a4ba3c1a&sv=3)![](https://hyperliquid.gitbook.io/hyperliquid-docs/~gitbook/image?url=https%3A%2F%2F2356094849-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252FyUdp569E6w18GdfqlGvJ%252Ficon%252FsIAjqhKKIUysM08ahKPh%252FHL-logoSwitchDISliStat.png%3Falt%3Dmedia%26token%3Da81fa25c-0510-4d97-87ff-3fb8944935b1&width=32&dpr=3&quality=100&sign=587b41e3b6e75de5fb66d649a4ba3c1a&sv=3)

Hyperliquid Docs](/hyperliquid-docs)

`⌘Ctrl``k`

* [Hyperliquid Docs](/hyperliquid-docs)
* [Builder Tools](/hyperliquid-docs/builder-tools)
* [Support](/hyperliquid-docs/support)

For the complete documentation index, see [llms.txt](https://hyperliquid.gitbook.io/hyperliquid-docs/llms.txt). This page is also available as [Markdown](https://hyperliquid.gitbook.io/hyperliquid-docs/trading/builder-codes.md).

Copy

On this page

1. [Hyperliquid Docs](/hyperliquid-docs)
2. [Trading](/hyperliquid-docs/trading)

Builder codes
=============

*Note: The term "builder" in the context of builder codes does not refer to block builders within consensus, but rather "defi builders" who build applications on Hyperliquid.*

**Builder codes** allow builders to receive a fee on fills that they send on behalf of a user. They are set per-order for maximal flexibility. The user must approve a maximum builder fee for each builder, and can revoke permissions at any time. Builder codes are processed entirely onchain as part of the fee logic.

In order to use builder codes, the end user would first approve a max fee for the builder address via the `ApproveBuilderFee` action. This action must be signed by the user's main wallet, not an agent/API wallet. The builder must have at least 100 USDC in perps account value and must use standard as the account abstraction mode.

Builder codes currently only apply to fees that are collected in the quote or collateral asset. In other words, builder codes do not apply to the buying side of spot trades but apply to both sides of perp trades. Builder fees charged can be at most 0.1% on perps and 1% on spot.

Once the authorization is complete, future order actions sent on behalf of the user may include an optional builder parameter: `{"b": address, "f": number}`. `b` is the address of the builder and `f` is the builder fee to charge in tenths of basis points. I.e. a value of 10 means 1 basis point.

Builders can claim fees from builder codes through the usual referral reward claim process.

Each user can have a maximum of 10 active builder code approvals at a time.

For example code see the Python SDK <https://github.com/hyperliquid-dex/hyperliquid-python-sdk/blob/master/examples/basic_builder_fee.py>

### API for builders

The approved maximum builder fee for a user can be queried via an info request `{"type": "maxBuilderFee", "user": "0x...", "builder": "0x..."}.`

The total builder fees collected for a builder is part of the referral state response from info request `{"type": "referral", "user": "0x..."}`.

The trades that use a particular builder code are uploaded in compressed LZ4 format to `https://stats-data.hyperliquid.xyz/Mainnet/builder_fills/{builder_address}/{YYYYMMDD}.csv.lz4`e.g. `https://stats-data.hyperliquid.xyz/Mainnet/builder_fills/0x123.../20241031.csv.lz4`

Important: Note that these URLs are case sensitive, and require that `builder_address`be entirely lowercase.

[PreviousSub-accounts](/hyperliquid-docs/trading/sub-accounts)[NextPerpetual assets](/hyperliquid-docs/trading/perpetual-assets)

Last updated 4 months ago