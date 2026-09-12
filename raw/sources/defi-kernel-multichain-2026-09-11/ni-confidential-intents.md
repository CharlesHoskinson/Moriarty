> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Confidential Intents

> Solve for private liquidity on the confidential relay

Confidential Intents extend the NEAR Intents protocol with privacy-preserving swaps. Market makers can provide liquidity on a private chain where balances and swap details are shielded from public view.

***

## How it works

Confidential Intents use a two-chain architecture:

1. **NEAR (Public)** — The public blockchain. `intents.near` is the smart-contract ledger that credits deposited balances.
2. **FAR (Private)** — A private NEAR fork where the `intents.far` contract executes swaps with shielded balances

FAR is a permissioned instance of the NEAR protocol operated by a small validator set with no public networking. Balances and swap details on FAR are not visible to the public — the RPC is private and reachable only through authenticated channels.

**Shield (public → private):** Tokens are transferred to the Treasury account on NEAR. The Private PoA Bridge processes this deposit and mints equivalent IMT tokens on FAR.

**Unshield (private → public):** IMT tokens are burned on FAR. The PoA Bridge processes the burn request and releases the backing tokens from the Treasury on NEAR.

<Info>
  The bridge operates on a request-driven model — it processes deposits and burns via RPC requests rather than actively detecting on-chain events.
</Info>

Market makers quote against their private IMT liquidity on FAR. The swap settles entirely on FAR via a `token_diff` intent, invisible to public indexers.

### Asset identifiers

Confidential assets are represented as **IMT (Intents Multi-Token)** tokens on FAR. Each IMT is backed 1:1 by the corresponding tokens locked in the Treasury smart-contract account on the public chain.

The token ID format prefixes the public asset ID with a system account:

```
imt:<ACCOUNT_ID>:nep141:<token>
```

For example, wrapped NEAR on the confidential chain:

```
imt:51e8f94d77b5e90dc9852ca6113771e11e8382ce69473a75e313e41665de7cbe:nep141:wrap.near
```

This account ID (`PRIVATE_TREASURY_ACCOUNT_ID` in solver configs) is a production constant shared by all participants.

***

## Private Relay

Confidential Intents use a separate **Private Relay** (`PRIVATE_RELAY_WS_URL`) instead of the public Message Bus. The private relay:

* Broadcasts quote requests for confidential swaps (asset IDs are IMT-wrapped)
* Collects signed quotes from connected solvers
* Uses `QUOTE_STATUS_EXTENDED` events (vs `QUOTE_STATUS` on the public relay)
* Verifies signatures against the public `intents.near` contract, but reads balances from FAR

The private relay does **not** settle intents directly — settlement is handled by the Private PoA Bridge, which mints/burns IMT tokens across the public⇄private boundary.

<Info>
  The private relay uses the same Partner JWT authentication as the public relay. Contact the Defuse team for access to the private relay URL and credentials.
</Info>

***

## Versioned nonces

Confidential intents require a **versioned nonce** format that encodes the contract salt and deadline. This differs from public intents where solvers typically use deterministic nonces based on reserves.

The nonce is constructed using `VersionedNonceBuilder` from the `@defuse-protocol/intents-sdk`:

```typescript theme={null}
import { VersionedNonceBuilder } from '@defuse-protocol/intents-sdk';

const salt = Buffer.from('e110f317', 'hex'); // 4-byte contract salt
const deadline = new Date(Date.now() + quoteDeadlineMs);
const nonce = VersionedNonceBuilder.encodeNonce(salt, deadline);
```

<Warning>
  The contract salt (`PRIVATE_INTENTS_CONTRACT_SALT`) is a production constant. Using the wrong salt will cause intent verification to fail.
</Warning>

***

## Solver response format

When quoting confidential swaps, solvers return signed intents in the `private_signed_data` field:

| Intent    | Required            | Purpose                                                                                                                                                  |
| --------- | ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `shield`  | No                  | Transfer of solver's public liquidity to the Treasury (triggers IMT mint on FAR)                                                                         |
| `swap`    | Yes                 | Must contain exactly one `token_diff`. May optionally include an `imt_burn` if the solver wants output released immediately rather than staying shielded |
| `recover` | If `shield` present | Fallback `imt_burn` if the swap fails (shares the swap's nonce — only one can land)                                                                      |

<Warning>
  A quote with a `shield` intent but no `recover` intent will be rejected. The recover intent prevents solver liquidity from being stranded if settlement fails.
</Warning>

Solvers should only act on the `quote_settle_successful` event before releasing funds.

### Unshield notifications

When a solver's funds are unshielded (private → public), the relay sends an `UNSHIELDED` status notification via the dedicated `shield_status` stream. Subscribe to this stream separately from `quote_status` to receive these notifications:

```typescript theme={null}
// Subscribe to shield status updates
await this.sendRequest('subscribe', ['shield_status', null, true]);
```

The `shield_status` event payload contains:

* `quote_hash`: The quote identifier
* `status`: `"unshielded"` when funds have been released to public

```json Event theme={null}
{
  "jsonrpc": "2.0",
  "method": "subscribe",
  "params": {
    "quote_hash": "00000000000000000000000000000000",
    "status": "unshielded"
  }
}
```

<Info>
  `shield_status` supports the same [Guaranteed Delivery](/integration/market-makers/message-bus/guaranteed-delivery) acknowledgement/redelivery mechanics as `quote_status`, opt in the same way by passing the guaranteed-delivery flag on `subscribe`.
</Info>

If your solver needs to check its own confidential balance rather than just watch for shield events, that's a separate flow: it queries `GET /v0/account/balances` on the 1Click API, which requires a **User-Session** token. See [Authenticating end users](/integration/distribution-channels/1click-api/authentication#authenticating-end-users-confidential-intents) for how to get one, and [Confidential Example → Authentication](/integration/market-makers/confidential-example#authentication) for it wired into a solver.

***

## Shielding and unshielding liquidity

Before quoting confidential swaps, market makers must **shield** (deposit) liquidity from the public NEAR chain into their private balance on FAR.

### Shield (deposit into confidential)

Transfer tokens from your public Intents balance to your confidential balance:

* `depositType`: `INTENTS`
* `recipientType`: `CONFIDENTIAL_INTENTS`
* `refundType`: `INTENTS`

```json theme={null}
{
  "dry": false,
  "swapType": "EXACT_INPUT",
  "originAsset": "nep141:wrap.near",
  "destinationAsset": "nep141:wrap.near",
  "amount": "1000000000000000000000000",
  "depositType": "INTENTS",
  "recipient": "your-solver.near",
  "recipientType": "CONFIDENTIAL_INTENTS",
  "refundTo": "your-solver.near",
  "refundType": "INTENTS"
}
```

### Unshield (withdraw to public)

Transfer tokens from your confidential balance back to public Intents:

* `depositType`: `CONFIDENTIAL_INTENTS`
* `recipientType`: `INTENTS`
* `refundType`: `CONFIDENTIAL_INTENTS`

```json theme={null}
{
  "dry": false,
  "swapType": "EXACT_INPUT",
  "originAsset": "nep141:wrap.near",
  "destinationAsset": "nep141:wrap.near",
  "amount": "1000000000000000000000000",
  "depositType": "CONFIDENTIAL_INTENTS",
  "recipient": "your-solver.near",
  "recipientType": "INTENTS",
  "refundTo": "your-solver.near",
  "refundType": "CONFIDENTIAL_INTENTS"
}
```

<Tip>
  Use the same asset for both `originAsset` and `destinationAsset` when shielding or unshielding. These are liquidity movements, not swaps, so `amount` is just the amount you're moving, not something you're pricing.
</Tip>

The [example solver repository](https://github.com/hairy-pointer/near-intents-amm-solver/tree/feat/confidential-amm-solver-mode) includes runnable scripts for depositing and withdrawing confidential liquidity via the 1Click API.

***

## Next steps

<CardGroup cols={2}>
  <Card title="Confidential Example" icon="code" href="/integration/market-makers/confidential-example">
    Detailed walkthrough of solving for confidential intents
  </Card>

  <Card title="Public Intents" icon="globe" href="/integration/market-makers/example">
    Compare with the public intents implementation
  </Card>
</CardGroup>
