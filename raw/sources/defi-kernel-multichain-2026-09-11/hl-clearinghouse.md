[![](https://hyperliquid.gitbook.io/hyperliquid-docs/~gitbook/image?url=https%3A%2F%2F2356094849-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252FyUdp569E6w18GdfqlGvJ%252Ficon%252FsIAjqhKKIUysM08ahKPh%252FHL-logoSwitchDISliStat.png%3Falt%3Dmedia%26token%3Da81fa25c-0510-4d97-87ff-3fb8944935b1&width=32&dpr=3&quality=100&sign=587b41e3b6e75de5fb66d649a4ba3c1a&sv=3)![](https://hyperliquid.gitbook.io/hyperliquid-docs/~gitbook/image?url=https%3A%2F%2F2356094849-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252FyUdp569E6w18GdfqlGvJ%252Ficon%252FsIAjqhKKIUysM08ahKPh%252FHL-logoSwitchDISliStat.png%3Falt%3Dmedia%26token%3Da81fa25c-0510-4d97-87ff-3fb8944935b1&width=32&dpr=3&quality=100&sign=587b41e3b6e75de5fb66d649a4ba3c1a&sv=3)

Hyperliquid Docs](/hyperliquid-docs)

`⌘Ctrl``k`

* [Hyperliquid Docs](/hyperliquid-docs)
* [Builder Tools](/hyperliquid-docs/builder-tools)
* [Support](/hyperliquid-docs/support)

For the complete documentation index, see [llms.txt](https://hyperliquid.gitbook.io/hyperliquid-docs/llms.txt). This page is also available as [Markdown](https://hyperliquid.gitbook.io/hyperliquid-docs/trading/margining.md).

Copy

On this page

1. [Hyperliquid Docs](/hyperliquid-docs)
2. [Trading](/hyperliquid-docs/trading)

Margining
=========

Margin computations follow similar formulas to major centralized derivatives exchanges.

### Margin Mode

When opening a position, a margin mode is selected. *Cross margin* is the default, which allows for maximal capital efficiency by sharing collateral between all other cross margin positions. *Isolated margin* is also supported, which allows an asset's collateral to be constrained to that asset. Liquidations in that asset do not affect other isolated positions or cross positions. Similarly, cross liquidations or other isolated liquidations do not affect the original isolated position.

Some assets are *strict isolated*, which functions the same as isolated margin with the additional constraint that margin cannot be removed. Margin is proportionally removed as the position is closed.

### HIP-3 Margin Modes

When users have perp positions across multiple DEXs, cross margin behaves different depending on the user's account abstraction. For unified account and portfolio margin, the user's cross margin positions in DEXs with the same collateral all share margin. For standard abstraction, cross margin only applies to the assets within the same DEX. See [here](/hyperliquid-docs/trading/account-abstraction-modes) for more details.

HIP-3 DEXs also support "no cross" margin mode, which allows isolated margin with margin removal enabled, but does not allow cross margin.

### Initial Margin and Leverage

Leverage can be set by a user to any integer between 1 and the max leverage. Max leverage depends on the asset.

The margin required to open a position is `position_size * mark_price / leverage`. The initial margin is used by the position and cannot be withdrawn for cross margin positions. Isolated positions support adding and removing margin after opening the position. Unrealized pnl for cross margin positions will automatically be available as initial margin for new positions, while isolated positions will apply unrealized pnl as additional margin for the open position.
The leverage of an existing position can be increased without closing the position. Leverage is only checked upon opening a position. Afterwards, the user is responsible for monitoring the leverage usage to avoid liquidation. Possible actions to take on positions with negative unrealized pnl include partially or fully closing the position, adding margin (if isolated), and depositing USDC (if cross).

### Unrealized PNL and transfer margin requirements

Unrealized pnl can be withdrawn from isolated positions or cross account, but only if the remaining margin is at least 10% of the total notional position value of all open positions. The margin remaining must also meet the initial margin requirement, i.e. `transfer_margin_required = max(initial_margin_required, 0.1 * total_position_value)`

Here, "transferring" includes any action that removes margin from a position, other than trading. Examples include withdrawals, transfer to spot wallet, and isolated margin transfers.

### Maintenance Margin and Liquidations

Cross positions are liquidated when the account value (including unrealized pnl) is less than the *maintenance margin* times the total open notional position. The maintenance margin is currently set to half of the initial margin at max leverage.

Isolated positions are liquidated by the same maintenance margin logic, but the only inputs to the computation are the isolated margin and the notional value of the isolated position.

[PreviousContract specifications](/hyperliquid-docs/trading/contract-specifications)[NextAccount abstraction modes](/hyperliquid-docs/trading/account-abstraction-modes)

Last updated 6 months ago

* [Margin Mode](#margin-mode)
* [HIP-3 Margin Modes](#hip-3-margin-modes)
* [Initial Margin and Leverage](#initial-margin-and-leverage)
* [Unrealized PNL and transfer margin requirements](#unrealized-pnl-and-transfer-margin-requirements)
* [Maintenance Margin and Liquidations](#maintenance-margin-and-liquidations)