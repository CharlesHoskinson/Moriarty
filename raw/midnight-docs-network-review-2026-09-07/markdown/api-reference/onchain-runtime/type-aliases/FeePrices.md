# FeePrices

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / FeePrices

# Type Alias: FeePrices

```
type FeePrices: {

  blockUsageFactor: number;

  computeFactor: number;

  overallPrice: number;

  readFactor: number;

  writeFactor: number;

};
```

The fee prices for transaction

## Type declaration[​](#type-declaration "Direct link to Type declaration")

### blockUsageFactor[​](#blockusagefactor "Direct link to blockUsageFactor")

```
blockUsageFactor: number;
```

The price factor of block usage.

### computeFactor[​](#computefactor "Direct link to computeFactor")

```
computeFactor: number;
```

The price factor of time spent in single-threaded compute.

### overallPrice[​](#overallprice "Direct link to overallPrice")

```
overallPrice: number;
```

The overall price of a full block in an average cost dimension.

### readFactor[​](#readfactor "Direct link to readFactor")

```
readFactor: number;
```

The price factor of time spent reading from disk.

### writeFactor[​](#writefactor "Direct link to writeFactor")

```
writeFactor: number;
```

The price factor of time spent writing to disk.
