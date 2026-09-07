# FeePrices

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / FeePrices

# Type Alias: FeePrices

```
type FeePrices = {

  blockUsageFactor: number;

  computeFactor: number;

  overallPrice: number;

  readFactor: number;

  writeFactor: number;

};
```

The fee prices for transaction

## Properties[​](#properties "Direct link to Properties")

### blockUsageFactor[​](#blockusagefactor "Direct link to blockUsageFactor")

```
blockUsageFactor: number;
```

The price factor of block usage.

***

### computeFactor[​](#computefactor "Direct link to computeFactor")

```
computeFactor: number;
```

The price factor of time spent in single-threaded compute.

***

### overallPrice[​](#overallprice "Direct link to overallPrice")

```
overallPrice: number;
```

The overall price of a full block in an average cost dimension.

***

### readFactor[​](#readfactor "Direct link to readFactor")

```
readFactor: number;
```

The price factor of time spent reading from disk.

***

### writeFactor[​](#writefactor "Direct link to writeFactor")

```
writeFactor: number;
```

The price factor of time spent writing to disk.
