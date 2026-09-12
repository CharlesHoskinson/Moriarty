> For the complete documentation index, see [llms.txt](https://hyperliquid.gitbook.io/hyperliquid-docs/llms.txt). Markdown versions of documentation pages are available by appending `.md` to page URLs; this page is available as [Markdown](https://hyperliquid.gitbook.io/hyperliquid-docs/hypercore/staking.md).

# Staking

### Basics

HYPE staking on Hyperliquid happens within HyperCore. Just as USDC can be transferred between perps and spot accounts, HYPE can be transferred between spot and staking accounts.&#x20;

Within the staking account, HYPE may be staked to any number of validators. Here and in other docs, *delegate* and *stake* are used interchangeably, as Hyperliquid only supports delegated proof-of-stake.&#x20;

Each validator has a self-delegation requirement of 10k HYPE to become active. The self-delegation requirement is locked for one year. Any time that the self-delegation for a validator drops below 10k HYPE, the validator enters undelegate-only mode (i.e., where all future delegations to this validator are disabled), so the validator's total stake can only decrease going forward.&#x20;

Once active, validators produce blocks and receive rewards proportional to their total delegated stake. Validators may charge a commission to their delegators. This commission cannot be increased unless the new commission is less than or equal to 1%. This prevents scenarios where a validator attracts a large amount of stake and then raises the commission significantly to take advantage of unaware stakers.

Delegations to a particular validator have a lockup duration of 1 day. After this lockup, delegations may be partially or fully undelegated at any time. Undelegated balances instantly reflect in staking account balance.&#x20;

Transfers from spot account to staking account are instant. However, transfers from staking account to spot account have a 7 day unstaking queue. Most other proof-of-stake chains have a similar mechanism, which ensures that large-scale consensus attacks are penalized by slashing or social layer mechanisms. There is currently no automatic slashing implemented. Each address may have at most 5 pending withdrawals in the unstaking queue.&#x20;

As an example, if you initiate a staking to spot transfer of 100 HYPE at 08:00:00 UTC on March 11 and a transfer of 50 HYPE at 09:00:00 UTC on March 12, the 100 HYPE transfer will be finalized at 08:00:01 UTC on March 18 and the 50 HYPE transfer will be finalized at 09:00:01 UTC on March 19.&#x20;

The staking reward rate formula is inspired by Ethereum, where the reward rate is inversely proportional to the square root of total HYPE staked. At 400M total HYPE staked, the reward rate is approximately 2.37% per year. Staking rewards come from the future emissions reserve.

Rewards are accrued every minute and distributed to stakers every day. Rewards are redelegated automatically to the staked validator, i.e., compounded. Rewards are based on the minimum balance that a delegator has staked during each staking epoch (100k rounds, as explained below).

### Technical Details

The notion of a *quorum* is essential to modern proof-of-stake consensus algorithms such as HyperBFT. A quorum is any set of validators that has more than ⅔ of the total stake in the network. The operating requirement of consensus is that a quorum of stake is honest (non-Byzantine). Therefore it is an essential responsibility of every staker to only delegate to trusted validators.&#x20;

HyperBFT consensus proceeds in *rounds*, which is a fundamental discrete bundle of transactions along with signatures from a quorum of validators. Each *round* may be *committed* after certain conditions are met, after which it is sent to the execution state for processing. A key property of the consensus algorithm is that all honest nodes agree on the ordered list of committed rounds.

Rounds may result in a new execution state block. Execution blocks are indexed by a separate increasing counter called *height*. Height only increments on consensus rounds with at least one transaction.

The validator set evolves in epochs of 100k rounds, which is approximately 90 minutes on mainnet. The validators and consensus stakes are static for each staking epoch.

Validators may vote to jail peers that do not respond with adequate latency or frequency to the consensus messages of the voter. Upon receiving a quorum of jail votes, a validator becomes *jailed* and no longer participates in consensus. A jailed validator does not produce rewards for its delegators. A validator may unjail itself by diagnosing and fixing the causes, subject to onchain unjailing rate limits. Note that jailing is not the same as slashing, which is reserved for provably malicious behavior such as double-signing blocks at the same round.&#x20;
