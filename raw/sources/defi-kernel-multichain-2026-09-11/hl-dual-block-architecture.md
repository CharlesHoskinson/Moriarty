> For the complete documentation index, see [llms.txt](https://hyperliquid.gitbook.io/hyperliquid-docs/llms.txt). Markdown versions of documentation pages are available by appending `.md` to page URLs; this page is available as [Markdown](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/hyperevm/dual-block-architecture.md).

# Dual-block architecture

The total HyperEVM throughput is split between small blocks produced at a fast rate and large blocks produced at a slower rate.&#x20;

The primary motivation behind the dual-block architecture is to decouple block speed and block size when allocating throughput improvements. Users want faster blocks for lower time to confirmation. Builders want larger blocks to include larger transactions such as more complex contract deployments. Instead of a forced tradeoff, the dual-block system will allow simultaneous improvement along both axes.&#x20;

The HyperEVM "mempool" is still onchain state with respect to the umbrella L1 execution, but is split into two independent mempools that source transactions for the two block types. The two block types are interleaved with a unique increasing sequence of EVM block numbers. The onchain mempool implementation accepts only the next 8 nonces for each address. Transactions older than 1 day old in the mempool are pruned.&#x20;

The initial configuration is set conservatively, and throughput is expected to increase over successive technical upgrades. Fast block duration is set to 1 seconds with a 3M gas limit. Slow block duration is set to 1 minute with a 30M gas limit.&#x20;

More precisely, in the definitions above, *block duration* of `x` means that the first L1 block for each value   of `l1_block_time % x` produces an EVM block.&#x20;

Developers can deploy larger contracts as follows:

1. Submit action `{"type": "evmUserModify", "usingBigBlocks": true}` to direct HyperEVM transactions to big blocks instead of small blocks. Note that this user state flag is set on the HyperCore user level, and must be unset again to target small blocks. Like any action, this requires an existing Core user to send. Like any EOA, the deployer address can be converted to a Core user by receiving a Core asset such as USDC.
2. Optionally use the JSON-RPC method `bigBlockGasPrice` in place of `gasPrice` to estimate base gas fee on the next big block.
