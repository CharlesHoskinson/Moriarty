> For the complete documentation index, see [llms.txt](https://hyperliquid.gitbook.io/hyperliquid-docs/llms.txt). Markdown versions of documentation pages are available by appending `.md` to page URLs; this page is available as [Markdown](https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/oracle.md).

# Oracle

The validators are responsible for publishing spot oracle prices for each perp asset every 3 seconds. The oracle prices are used to compute funding rates. They are also a component in the `mark price` which is used for margining, liquidations, and triggering TP/SL orders.

The spot oracle prices are computed by each validator as the weighted median of Binance, OKX, Bybit, Kraken, Kucoin, Gate IO, MEXC, and Hyperliquid spot mid prices for each asset, with weights 3, 2, 2, 1, 1, 1, 1, 1 respectively. Perps on assets which have primary spot liquidity on Hyperliquid (e.g. HYPE) do not include external sources in the oracle until sufficient liquidity is met. Perps on assets that have primary spot liquidity outside of Hyperliquid (e.g. BTC) do not include Hyperliquid spot prices in the oracle.

The final oracle price used by the clearinghouse is the weighted median of each validator's submitted oracle prices, where the validators are weighted by their stake.
