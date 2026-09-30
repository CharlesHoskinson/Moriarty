[![](https://hyperliquid.gitbook.io/hyperliquid-docs/~gitbook/image?url=https%3A%2F%2F2356094849-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252FyUdp569E6w18GdfqlGvJ%252Ficon%252FsIAjqhKKIUysM08ahKPh%252FHL-logoSwitchDISliStat.png%3Falt%3Dmedia%26token%3Da81fa25c-0510-4d97-87ff-3fb8944935b1&width=32&dpr=3&quality=100&sign=587b41e3b6e75de5fb66d649a4ba3c1a&sv=3)![](https://hyperliquid.gitbook.io/hyperliquid-docs/~gitbook/image?url=https%3A%2F%2F2356094849-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252FyUdp569E6w18GdfqlGvJ%252Ficon%252FsIAjqhKKIUysM08ahKPh%252FHL-logoSwitchDISliStat.png%3Falt%3Dmedia%26token%3Da81fa25c-0510-4d97-87ff-3fb8944935b1&width=32&dpr=3&quality=100&sign=587b41e3b6e75de5fb66d649a4ba3c1a&sv=3)

Hyperliquid Docs](/hyperliquid-docs)

`⌘Ctrl``k`

* [Hyperliquid Docs](/hyperliquid-docs)
* [Builder Tools](/hyperliquid-docs/builder-tools)
* [Support](/hyperliquid-docs/support)

For the complete documentation index, see [llms.txt](https://hyperliquid.gitbook.io/hyperliquid-docs/llms.txt). This page is also available as [Markdown](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/hyperevm/interaction-timings.md).

Copy

On this page

1. [Hyperliquid Docs](/hyperliquid-docs)
2. [For developers](/hyperliquid-docs/for-developers)
3. [HyperEVM](/hyperliquid-docs/for-developers/hyperevm)

Interaction timings
===================

Transfer Timing
---------------

Transfers from HyperCore to HyperEVM are queued on the L1 until the next HyperEVM block. Transfers from HyperEVM to HyperCore happen in the same L1 block as the HyperEVM block, immediately after the HyperEVM block is built.

Timing within a HyperEVM block
------------------------------

On an L1 block that produces a HyperEVM block:

1. L1 block is built
2. EVM block is built
3. EVM -> Core transfers are processed
4. CoreWriter actions are processed

Note that the account performing the CoreWriter action must exist on HyperCore before the EVM block is built. An EVM -> Core transfer to initialize the account in the same block will still result in the CoreWriter action being rejected.

[PreviousHyperCore <> HyperEVM transfers](/hyperliquid-docs/for-developers/hyperevm/hypercore-less-than-greater-than-hyperevm-transfers)[NextWrapped HYPE](/hyperliquid-docs/for-developers/hyperevm/wrapped-hype)

Last updated 1 year ago

* [Transfer Timing](#transfer-timing)
* [Timing within a HyperEVM block](#timing-within-a-hyperevm-block)