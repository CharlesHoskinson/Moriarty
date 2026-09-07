# EncodedZswapLocalState

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / EncodedZswapLocalState

# Interface: EncodedZswapLocalState

Tracks the coins consumed and produced throughout circuit execution.

## Properties[​](#properties "Direct link to Properties")

### coinPublicKey[​](#coinpublickey "Direct link to coinPublicKey")

```
coinPublicKey: EncodedCoinPublicKey;
```

The Zswap coin public key of the user executing the circuit.

***

### currentIndex[​](#currentindex "Direct link to currentIndex")

```
currentIndex: bigint;
```

The Merkle tree index of the next coin produced.

***

### inputs[​](#inputs "Direct link to inputs")

```
inputs: EncodedQualifiedShieldedCoinInfo[];
```

The coins consumed as inputs to the circuit.

***

### outputs[​](#outputs "Direct link to outputs")

```
outputs: {

  coinInfo: EncodedShieldedCoinInfo;

  recipient: EncodedRecipient;

}[];
```

The coins produced as outputs from the circuit.

#### coinInfo[​](#coininfo "Direct link to coinInfo")

```
coinInfo: EncodedShieldedCoinInfo;
```

#### recipient[​](#recipient "Direct link to recipient")

```
recipient: EncodedRecipient;
```
