[![](https://hyperliquid.gitbook.io/hyperliquid-docs/~gitbook/image?url=https%3A%2F%2F2356094849-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252FyUdp569E6w18GdfqlGvJ%252Ficon%252FsIAjqhKKIUysM08ahKPh%252FHL-logoSwitchDISliStat.png%3Falt%3Dmedia%26token%3Da81fa25c-0510-4d97-87ff-3fb8944935b1&width=32&dpr=3&quality=100&sign=587b41e3b6e75de5fb66d649a4ba3c1a&sv=3)![](https://hyperliquid.gitbook.io/hyperliquid-docs/~gitbook/image?url=https%3A%2F%2F2356094849-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252FyUdp569E6w18GdfqlGvJ%252Ficon%252FsIAjqhKKIUysM08ahKPh%252FHL-logoSwitchDISliStat.png%3Falt%3Dmedia%26token%3Da81fa25c-0510-4d97-87ff-3fb8944935b1&width=32&dpr=3&quality=100&sign=587b41e3b6e75de5fb66d649a4ba3c1a&sv=3)

Hyperliquid Docs](/hyperliquid-docs)

`⌘Ctrl``k`

* [Hyperliquid Docs](/hyperliquid-docs)
* [Builder Tools](/hyperliquid-docs/builder-tools)
* [Support](/hyperliquid-docs/support)

For the complete documentation index, see [llms.txt](https://hyperliquid.gitbook.io/hyperliquid-docs/llms.txt). This page is also available as [Markdown](https://hyperliquid.gitbook.io/hyperliquid-docs/trading/order-types.md).

Copy

On this page

1. [Hyperliquid Docs](/hyperliquid-docs)
2. [Trading](/hyperliquid-docs/trading)

Order types
===========

### Order types:

* Market: An order that executes immediately at the current market price
* Limit: An order that executes at the selected limit price or better
* Chase: A Post Only (ALO) limit order that automatically re-prices to track the best bid or ask until it is filled or terminated. The order rests one tick above the best bid (for buys) or one tick below the best ask (for sells), or at the best bid/ask when the spread is a single tick. Chase orders run in the browser tab where they are created, and up to 5 can be active at once
* Stop Market: A market order that is activated when the price reaches the selected trigger price. For long orders, the trigger price needs to be higher than the mid price. For short orders, the trigger price needs to be lower than the mid price
* Stop Limit: A limit order that is activated when the price reaches the selected trigger price
* Take Market: A market order that is activated when the price reaches the selected trigger price. For long orders, the trigger price needs to be lower than the mid price. For short orders, the trigger price needs to be higher than the mid price
* Take Limit: A limit order that is activated when the price reaches the selected trigger price
* Scale: Multiple limit orders in a set price range
* TWAP: A large order divided into smaller suborders and executed at regular intervals, based on order size and running time inputs. Intervals are a minimum of 30 seconds. TWAP suborders have a maximum slippage of 3%

  + Randomize: If enabled, the size of each sub-order will be adjusted randomly up to ±20% of the original trade size
  + Trigger Price: The TWAP order will be activated when the mark price reaches the trigger price set
  + Max/Min Price: The TWAP order will be terminated when the mark price reaches the stop price set

### TWAP details:

During execution, a TWAP order attempts to meet an execution target which is defined as the elapsed time divided by the total time times the total size. Suborders are sent at a fixed interval, calculated from the total size and running time inputs. Larger orders over shorter durations are split into suborders sent as often as every 30 seconds; smaller orders over longer durations are split into suborders spaced further apart. Running time can be set from 5 minutes to 7 days, with a $100 minimum total order size.
Example:

* A $10,000 order over 1 hour is split into ~121 suborders of ~$83, sent every 30 seconds
* A $10,000 order over 4 days is split into ~1,000 suborders of ~$10, sent roughly every 6 minutes

A suborder is constrained to have a max slippage of 3%. When suborders do not fully fill because of market conditions (e.g., wide spread, low liquidity, etc.), the TWAP may fall behind its execution target. In this case, the TWAP will try to catch up to this execution target during later suborders. These later suborders will be larger but subject to the constraint of 3 times the normal suborder size (defined as total TWAP size divided by number of suborders). It is possible that if too many suborders did not fill then the TWAP order may not fully catch up to the total size by the end. Like normal market orders, TWAP suborders do not fill during the post-only period of a network upgrade.

### Order options:

* Reduce Only: An order that reduces a current position as opposed to opening a new position in the opposite direction
* Good Til Cancel (GTC): An order that rests on the order book until it is filled or canceled
* Post Only (ALO): An order that is added to the order book but doesn’t execute immediately. It is only executed as a resting order
* Immediate or Cancel (IOC): An order that will be canceled if it is not immediately filled
* Take Profit: An order that triggers when the Take Profit (TP) price is reached.
* Stop Loss: An order that triggers when the Stop Loss (SL) price is reached
* TP and SL orders are often used by traders to set targets and protect profits or minimize losses on positions. TP and SL are automatically market orders. You can set a limit price and configure the amount of the position to have a TP or SL

[PreviousOrder book](/hyperliquid-docs/trading/order-book)[NextTake profit and stop loss orders (TP/SL)](/hyperliquid-docs/trading/take-profit-and-stop-loss-orders-tp-sl)

Last updated 1 month ago

* [Order types:](#order-types)
* [TWAP details:](#twap-details)
* [Order options:](#order-options)