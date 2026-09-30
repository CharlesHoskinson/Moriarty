[![](https://hyperliquid.gitbook.io/hyperliquid-docs/~gitbook/image?url=https%3A%2F%2F2356094849-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252FyUdp569E6w18GdfqlGvJ%252Ficon%252FsIAjqhKKIUysM08ahKPh%252FHL-logoSwitchDISliStat.png%3Falt%3Dmedia%26token%3Da81fa25c-0510-4d97-87ff-3fb8944935b1&width=32&dpr=3&quality=100&sign=587b41e3b6e75de5fb66d649a4ba3c1a&sv=3)![](https://hyperliquid.gitbook.io/hyperliquid-docs/~gitbook/image?url=https%3A%2F%2F2356094849-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252FyUdp569E6w18GdfqlGvJ%252Ficon%252FsIAjqhKKIUysM08ahKPh%252FHL-logoSwitchDISliStat.png%3Falt%3Dmedia%26token%3Da81fa25c-0510-4d97-87ff-3fb8944935b1&width=32&dpr=3&quality=100&sign=587b41e3b6e75de5fb66d649a4ba3c1a&sv=3)

Hyperliquid Docs](/hyperliquid-docs)

`⌘Ctrl``k`

* [Hyperliquid Docs](/hyperliquid-docs)
* [Builder Tools](/hyperliquid-docs/builder-tools)
* [Support](/hyperliquid-docs/support)

For the complete documentation index, see [llms.txt](https://hyperliquid.gitbook.io/hyperliquid-docs/llms.txt). This page is also available as [Markdown](https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/overview.md).

Copy

On this page

1. [Hyperliquid Docs](/hyperliquid-docs)
2. [HyperCore](/hyperliquid-docs/hypercore)

Overview
========

### Consensus

Hyperliquid is secured by HyperBFT, a variant of HotStuff consensus. Like most proof-of-stake chains, blocks are produced by validators in proportion to the native token staked to each validator.

### Execution

The Hyperliquid state consists of HyperCore and the general purpose HyperEVM.

HyperCore includes margin and matching engine state. Importantly, HyperCore does not rely on the crutch of off-chain order books. A core design principle is full decentralization with one consistent order of transactions achieved through HyperBFT consensus.

### Latency

Consensus currently uses an optimized consensus algorithm called HyperBFT, which is optimized for end-to-end latency. End-to-end latency is measured as duration between sending request and receiving committed response.

For an order placed from a geographically co-located client, end-to-end latency has a median 0.2 seconds and 99th percentile 0.9 seconds. This performance allows users to port over automated strategies from other crypto venues with minimal changes and gives retail users instant feedback through the UI.

### Throughput

Mainnet currently supports approximately 200k orders/sec. The current bottleneck is execution. The consensus algorithm and networking stack can scale to millions of orders per second once the execution can keep up. There are plans to further optimize the execution logic once the need arises.

[PreviousHyperCore](/hyperliquid-docs/hypercore)[NextUSDC](/hyperliquid-docs/hypercore/usdc)

Last updated 3 months ago

* [Consensus](#consensus)
* [Execution](#execution)
* [Latency](#latency)
* [Throughput](#throughput)