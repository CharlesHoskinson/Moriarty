# secp256k1EcdsaRecover

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / secp256k1EcdsaRecover

# Function: secp256k1EcdsaRecover()

```
function secp256k1EcdsaRecover(

   msgHash, 

   sig, 

   recoveryId): Secp256k1Point;
```

Recover the secp256k1 public key from an ECDSA signature and a message hash.

## Recovery ID[​](#recovery-id "Direct link to Recovery ID")

* bit 0 (`recoveryId & 1`) is the parity of `R.y`: 0 for even, 1 for odd.

* bit 1 (`recoveryId >= 2`) says whether the reduction wrapped, i.e. whether `R.x` is `r` (0, 1) or `r + n` (2, 3).

* 0: `R = (r, y)` with `y` even — the common case.

* 1: `R = (r, y)` with `y` odd — the other common case.

* 2: `R = (r + n, y)` with `y` even.

* 3: `R = (r + n, y)` with `y` odd.

## Parameters[​](#parameters "Direct link to Parameters")

### msgHash[​](#msghash "Direct link to msgHash")

`Uint8Array`

### sig[​](#sig "Direct link to sig")

#### r[​](#r "Direct link to r")

`bigint`

#### s[​](#s "Direct link to s")

`bigint`

### recoveryId[​](#recoveryid "Direct link to recoveryId")

`number`

## Returns[​](#returns "Direct link to Returns")

[`Secp256k1Point`](/api-reference/compact-runtime/interfaces/Secp256k1Point.md)
