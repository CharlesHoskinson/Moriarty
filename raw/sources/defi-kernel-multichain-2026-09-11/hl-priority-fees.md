> For the complete documentation index, see [llms.txt](https://hyperliquid.gitbook.io/hyperliquid-docs/llms.txt). Markdown versions of documentation pages are available by appending `.md` to page URLs; this page is available as [Markdown](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/priority-fees.md).

# Priority fees

Advanced feature for latency-sensitive users

### Gossip (read) priority

There are 2 independent Dutch auctions synced to the same 3 minute schedule. The auction indices are optionally interpreted by nodes as an ordering for their peers when sending data. The priority ordering affects both split client blocks and normal client blocks that include responses. The foundation non-validator will opt into respecting the gossip priority auction ordering.&#x20;

The independent slot auctions are not additive, so an IP bidding multiple slots will be prioritized according to its lowest slot. For the auction to affect latency, the onchain IP must exactly match the IP seen by the peer sending data. Each auction's results only affect the duration of the following auction. For nodes that are connected via a chain of nodes to the validating network, note that each network hop may or may not respect the priority auction depending on how the parent node is configured.&#x20;

The lower auction indices are strictly prioritized over higher indices. The prioritization applies to all data in the mempool transactions stream equally. The ordering should be relatively consistent in expectation but is subject to variance from the p2p consensus network. Gossip priority applies only to reading data. For prioritization of sending data, see the section below on order priority. Nodes respecting gossip priority will use the previous auction winners relative to that node's current execution state. Similar to token, spot, and perp deployment auctions, fees come out of spot balance are burned.

The previous auction winners (i.e., current gossip priority ordering) and current auction statuses can be queried using the info request `{ "type": "gossipPriorityAuctionStatus"}`.&#x20;

Gossip priority reduces the need for market makers to hyper-optimize their infrastructure to remain competitive. To participate in the auction, users send action `{"type": "gossipPriorityBid", "slotId": 0, "ip": "1.2.3.4", "maxGas": 100000000}` where gas is wei (i.e. units of `0.00000001` HYPE) of HYPE charged from spot balance. Each auction resets to 10 times the previous winning price of that slot, and have a minimum price of `0.1 HYPE`. Any address may bid on behalf of any IP address.&#x20;

The current empirical effect of gossip priority on mainnet is approximately 25 ms reduction in latency per auction slot.

### Order (write) priority

Order priority reduces the need for market makers to hyper-optimize their order entry and connectivity, and instead focus on the strategy itself. It further protects makers and allows responsiveness to price moves driven by other venues.

Priority grouping is only supported for order actions where both

1. every order is on a non-outcome asset
2. every order is IOC, or every order is a non-reduce-only ALO

Orders can be sent with grouping of the form `{"p": 12345}` where the rate is interpreted as a fraction `p / 100000000.0`. The rate is charged from undelegated staking balance as a fraction of the filled notional in the case of IOC, and resting notional in the case of ALO. The priority gas charged is converted to HYPE using the spot mark price. &#x20;

#### IOC priority

The priority rate has a linear effect on end-to-end latency from 0-8bps, i.e. `p = 80000`. The current empirical effect of order priority fees on mainnet is approximately 45 ms reduction in end-to-end latency per 1 bp of priority fee paid. Even with order priority, all cancels are prioritized before all immediately executable orders. However, higher priority orders are prioritized before lower priority orders as a linear function of priority rate.&#x20;

The max priority rate is 100%. Any priority between 8bps and 100% is treated with identical time preference in the mempool. However, IOC orders with at least 8bps priority fee that would be executed by the block proposer in each 70ms unix time bucket (usually corresponding to 1 block) are sorted in decreasing order of priority fee. In other words, the 0-8bps range effects temporal prioritization, and 8bps-100% range breaks ties among orders in this range received at a similar time. As a concrete example, if the unix milliseconds boundary is `(T, T + 70ms)` , the orders with `(mempool_receive_time, priority)` equal to `(T, 8bps)`, `(T + 30ms, 9bps)`, `(T + 69ms, 10bps)` would be sorted in reverse order of receive time.

Although blocks are discrete, order prioritization is continuous with respect to order write priority fee. The mempool transactions are effectively sorted by `effective_time = arrival_time + f(action, priority_fee)` where `f` is a strictly decreasing function of priority fee, and zero for prioritized actions such as cancels. For a given action type, boundaries between blocks are not relevant for relative prioritization, as the transactions within a block are still sorted according to `effective_time` .

Order priority applies only to sending data. For prioritization of reading data, see the previous section on gossip priority. Similar to gossip priority auctions, order priority fees are burned.

IOC priority fee paid can be read from the `user_fills` file written by the nodes as field `priorityGas` .

#### ALO priority&#x20;

On the timescale of `T = 400ms`, ALO orders are sorted by priority. More precisely, the tail of the queue at each level consisting of orders placed within the past `T` will be sorted in decreasing order of priority rate.&#x20;

As a consequence, any slot in the queue is eligible for priority fee bidding for a window of duration `T`.  Each new order compares against the tail of orders less than `T` old at placement time. In other words, the `T` window is continuous, not bucketed. ALO priority fees are deducted at time of placing the order regardless of whether the order fills.&#x20;

Unlike for IOC orders, ALO priority fees do not effect mempool prioritization. ALO orders are processed FIFO as transactions always, with similar end-to-end latency as cancels. The priority fee implementation affects place in queue across transactions. In other words, ALO orders are prioritized similarly in the mempool regardless of their priority, but queue position can change on a 400ms time scale after orders transactions are executed on the L1.

A concrete example: a new level `L` opens and is empty at time `t`. The first order  `A`  at level `L` is applied onchain (as above, this means executed on the L1) at time `t' > t` with zero priority fee. The next order `B` is applied onchain at time `t' + T - 1ms` with priority `p > 0`. Assuming `A` and `B` are the only orders at level `L`, the ordering at time `t' + T` will have `B` in front of `A`. A third order `C` is applied at time `t' + T + 1ms` with priority `p' > p`. The ordering at time `t' + T + 1ms` is `B > A > C` . Even though `C` has higher priority, `B` has already locked in its place in queue because `A` has been on the book for `T`, and `B` sits before `A`. &#x20;

Gossip priority fees can be read from the `replica_cmds` files recorded by nodes. The USDC value of the priority fee amounts can be computed as `order_price * order_sz * grouping_priority_rate` . The [order\_book\_server](https://github.com/hyperliquid-dex/order_book_server) maintains the correct level ordering by using the `insertBefore` field on `RawBookDiff::New` from the `raw_book_diff` files.
