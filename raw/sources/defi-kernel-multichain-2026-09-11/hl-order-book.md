> For the complete documentation index, see [llms.txt](https://hyperliquid.gitbook.io/hyperliquid-docs/llms.txt). Markdown versions of documentation pages are available by appending `.md` to page URLs; this page is available as [Markdown](https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/order-book.md).

# Order book

HyperCore state includes an order book for each asset. The order book works similarly to that of centralized exchanges. Orders are added where price is an integer multiple of the tick size, and size is an integer multiple of lot size. Orders are matched in price-time priority.

Operations on order books for perp assets take a reference to the clearinghouse, as all positions and margin checks are handled there. Margin checks happen on the opening of a new order, and again for the resting side at the matching of each order. This ensures that the margining system is consistent despite oracle price fluctuations after the resting order is placed.

One unique aspect of the Hyperliquid L1 is that the mempool and consensus logic are semantically aware of transactions that interact with HyperCore order books. Within a block, actions are sorted within each consensus batch (there are usually 1-2 consensus batches of actions in a block) as

1. Actions that do not send GTC or IOC orders to any book
2. Cancels
3. Actions that send at least one GTC or IOC

Within each category, actions are sorted in the order they were proposed by the block proposer. Modifies are categorized according to the new order they place.    &#x20;
