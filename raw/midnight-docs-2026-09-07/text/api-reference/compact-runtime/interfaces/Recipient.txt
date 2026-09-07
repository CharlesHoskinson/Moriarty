# Recipient

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / Recipient

# Interface: Recipient

The recipient of a coin produced by a circuit.

## Properties[​](#properties "Direct link to Properties")

### is\_left[​](#is_left "Direct link to is_left")

```
readonly is_left: boolean;
```

Whether the recipient is a user or a contract.

***

### left[​](#left "Direct link to left")

```
readonly left: string;
```

The recipient's public key, if the recipient is a user.

***

### right[​](#right "Direct link to right")

```
readonly right: string;
```

The recipient's contract address, if the recipient is a contract.
