> For the complete documentation index, see [llms.txt](https://hyperliquid.gitbook.io/hyperliquid-docs/llms.txt). Markdown versions of documentation pages are available by appending `.md` to page URLs; this page is available as [Markdown](https://hyperliquid.gitbook.io/hyperliquid-docs/trading/contract-specifications.md).

# Contract specifications

### Crypto Perps

Hyperliquid perpetuals are derivatives products without expiration date. Instead, they rely on funding payments to ensure convergence to the underlying spot price over time. See [Funding](/hyperliquid-docs/trading/funding.md) for more information.&#x20;

Hyperliquid has one main style of margining for perpetual contracts: USDC margining, USDT denominated linear contracts. That is, the oracle price is denominated in USDT, but the collateral is USDC. This allows for the best combination of liquidity and accessibility. Note that no conversions with the USDC/USDT exchange rate are applied, so these contracts are technically quanto contracts where USDT pnl is denominated in USDC.

When the spot asset's primary source of liquidity is USDC denominated, the oracle price is denominated in USDC. Currently, the only USDC-denominated perpetual contracts are PURR-USD and HYPE-USD, where the most liquid spot oracle source is Hyperliquid spot.

Hyperliquid's contract specifications are simpler than most platforms. There are few contract-specific details and no address-specific restrictions.

|                             |                                                                                                                                            |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| Instrument type             | Linear perpetual                                                                                                                           |
| Contract                    | 1 unit of underlying spot asset                                                                                                            |
| Underlying asset / ticker   | Hyperliquid oracle index of underlying spot asset                                                                                          |
| Initial margin fraction     | 1 / (leverage set by user)                                                                                                                 |
| Maintenance margin fraction | Half of maximum initial margin fraction                                                                                                    |
| Mark price                  | See [here](/hyperliquid-docs/trading/robust-price-indices.md)                                                                              |
| Delivery / expiration       | N/A (funding payments every hour)                                                                                                          |
| Position limit              | N/A                                                                                                                                        |
| Account type                | Per-wallet cross or isolated margin                                                                                                        |
| Funding impact notional     | <p>20000 USDC for BTC and ETH</p><p>6000 USDC for all other assets </p>                                                                    |
| Maximum market order value  | $30,000,000 for max leverage >= 25, $5,000,000 for max leverage in \[20, 25), $2,000,000 for max leverage in \[10, 20), otherwise $500,000 |
| Maximum limit order value   | 10 \* maximum market order value                                                                                                           |

### Recurring outcomes

Recurring outcomes are automatically deployed and settled by the protocol on a fixed cadence. The specification for the current instance of the recurring outcome is found in the `description` field of `outcomeMeta` .

#### Binary&#x20;

```
class:priceBinary|underlying:BTC|expiry:20260503-0600|targetPrice:78213|period:1d"
```

The target price is computed using a linear interpolation between the mark price updates immediately before and immediately after the settlement timestamp. Precisely, the contract settles to YES if and only if `markPrice0 + (settlementTime - t0) / (t1 - t0) * (markPrice1 - markPrice0) ≥ targetPrice` where `t0` and `t1` are the timestamps of mark price updates immediately before and after `settlementTime`

#### Multi-price&#x20;

```
"class:priceBucket|underlying:BTC|expiry:20260507-0745|priceThresholds:81538,81783|period:15m"
```

There are three price buckets depending on the target price at the time of settlement: `< P1`, `[P1, P2)` and `≥ P2.` Exactly one of the 3 outcomes settles to 1, and the others settle to 0. The target price computation uses the same interpolation as binary markets described above.

### Uniqueness

There is guaranteed to be at most one recurring series for each `(seriesType, underlying, period)` combination.
