> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Signed Intent Execution

> Authorize a swap by signing an intent off-chain instead of an on-chain deposit

Use signed intent execution when a quote uses `depositType: INTENTS` or `depositType: CONFIDENTIAL_INTENTS`. In both cases, the funds are already inside NEAR Intents, so the user authorizes the swap by signing an intent message off-chain instead of sending an on-chain deposit.

<Tip>
  Foreign-to-foreign confidential swaps (using the `confidentiality` parameter with `ORIGIN_CHAIN` deposits) do **not** use signed intent execution, they follow the standard on-chain deposit flow in [Making a Request](./making-a-request). See [Confidential Swaps](./confidential-swaps) for both paths.
</Tip>

| Deposit channel              | depositType            | How the deposit is made                                                          |
| ---------------------------- | ---------------------- | -------------------------------------------------------------------------------- |
| Origin chain transfer        | `ORIGIN_CHAIN`         | Send tokens to `depositAddress`, optionally notify via `POST /v0/deposit/submit` |
| Signed intent                | `INTENTS`              | Generate intent, user signs, `POST /v0/submit-intent`                            |
| Signed intent (confidential) | `CONFIDENTIAL_INTENTS` | Generate intent, user signs, `POST /v0/submit-intent`                            |

For public Intents balances, this can speed execution because there is no on-chain deposit to wait for. For Confidential Intents balances, this is the required path because there is no RPC path for confidential intents.

<Info>
  `POST /v0/generate-intent` and `POST /v0/submit-intent` use partner authentication (`X-API-Key` recommended, `JWT-auth` legacy). They do not use the end-user `User-Session` token; the user's authorization is the wallet signature submitted as `signedData`.
</Info>

<Steps>
  <Step title="Quote">
    Request a quote with `POST /v0/quote` using `depositType` `INTENTS` or `CONFIDENTIAL_INTENTS`. Save the returned `depositAddress`; it links the signed intent back to the quote.

    See [Making a Request](./making-a-request#request-token) for the full quote flow — the only required change is setting `depositType` (and matching `refundType` / `recipientType` when those balances also live in Intents).
  </Step>

  <Step title="Generate">
    Call `POST /v0/generate-intent` with the `depositAddress`, the user's `signerId`, and their wallet's signing `standard` (`nep413`, `erc191`, `raw_ed25519`, `webauthn`, `ton_connect`, `sep53`, or `tip191`). It returns the unsigned intent payload.

    <CodeGroup>
      ```bash cURL theme={null}
      curl -X POST https://1click.chaindefuser.com/v0/generate-intent \
        -H "Content-Type: application/json" \
        -H "X-API-Key: YOUR_API_KEY" \
        -d '{
          "type": "swap_transfer",
          "standard": "nep413",
          "signerId": "user.near",
          "depositAddress": "address-from-quote-response"
        }'
      ```

      ```typescript TypeScript theme={null}
      const response = await fetch('https://1click.chaindefuser.com/v0/generate-intent', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-API-Key': 'YOUR_API_KEY'
        },
        body: JSON.stringify({
          type: 'swap_transfer',
          standard: 'nep413',
          signerId: 'user.near',
          depositAddress: 'address-from-quote-response'
        })
      });
      const { intent, correlationId } = await response.json();
      ```
    </CodeGroup>

    <Accordion title="Example response">
      ```json theme={null}
      {
        "intent": {
          "standard": "nep413",
          "payload": {
            "recipient": "intents.near",
            "nonce": "Vij2xgAlKBKzAEiS6N1S/hcQLaHzX2s1fNKhBDblXT4=",
            "message": "{\"deadline\":\"2026-07-16T12:00:00.000Z\",\"intents\":[{\"intent\":\"token_diff\",\"diff\":{\"nep141:wrap.near\":\"-1000000000000000000000000\",\"nep141:usdt.tether-token.near\":\"1000000\"}}],\"signer_id\":\"user.near\"}"
          }
        },
        "correlationId": "550e8400-e29b-41d4-a716-446655440000"
      }
      ```
    </Accordion>

    For the exact request fields and response shape, see [Generate an intent for signing](/api-reference/oneclick/generate-an-intent-for-signing).
  </Step>

  <Step title="Sign">
    The user signs the returned `intent` payload with their wallet off-chain. Do not modify the payload between generation and signing; the signature must cover the exact payload returned by the API.

    The wallet returns `public_key` and `signature`. Combine those with the generated `intent` fields to form the full `signedData` MultiPayload you submit next. See [Signing Intents](/integration/verifier-contract/signing-intents) for the payload shape across supported standards.
  </Step>

  <Step title="Submit">
    Call `POST /v0/submit-intent` with the signed payload (`type: swap_transfer`, `signedData`: the signed MultiPayload). It returns the `intentHash`.

    <CodeGroup>
      ```bash cURL theme={null}
      curl -X POST https://1click.chaindefuser.com/v0/submit-intent \
        -H "Content-Type: application/json" \
        -H "X-API-Key: YOUR_API_KEY" \
        -d '{
          "type": "swap_transfer",
          "signedData": {
            "standard": "nep413",
            "payload": {
              "recipient": "intents.near",
              "nonce": "Vij2xgAlKBKzAEiS6N1S/hcQLaHzX2s1fNKhBDblXT4=",
              "message": "{\"deadline\":\"2026-07-16T12:00:00.000Z\",\"intents\":[{\"intent\":\"token_diff\",\"diff\":{\"nep141:wrap.near\":\"-1000000000000000000000000\",\"nep141:usdt.tether-token.near\":\"1000000\"}}],\"signer_id\":\"user.near\"}"
            },
            "public_key": "ed25519:YOUR_PUBLIC_KEY",
            "signature": "ed25519:YOUR_SIGNATURE"
          }
        }'
      ```

      ```typescript TypeScript theme={null}
      // `intent` is the object returned by /v0/generate-intent
      const response = await fetch('https://1click.chaindefuser.com/v0/submit-intent', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-API-Key': 'YOUR_API_KEY'
        },
        body: JSON.stringify({
          type: 'swap_transfer',
          signedData: {
            ...intent,
            public_key: 'ed25519:YOUR_PUBLIC_KEY',
            signature: 'ed25519:YOUR_SIGNATURE'
          }
        })
      });
      const { intentHash, correlationId } = await response.json();
      ```
    </CodeGroup>

    <Accordion title="Example response">
      ```json theme={null}
      {
        "intentHash": "44XpLRAuZKoVGs9T4qbSNv33MDKMePPAibA52geVLWFw",
        "correlationId": "550e8400-e29b-41d4-a716-446655440000"
      }
      ```
    </Accordion>

    For the exact `signedData` schema and response shape, see [Submit a signed intent](/api-reference/oneclick/submit-a-signed-intent).
  </Step>

  <Step title="Track">
    Poll `GET /v0/status` with the `depositAddress`, as with any swap, see [Making a Request → Monitor status](./making-a-request#monitor-status) for the status values. Include `depositMemo` if the quote response included one.
  </Step>
</Steps>
