> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Simulating Intents

> Test intents without modifying blockchain state using the simulate_intents function

The [`simulate_intents`](https://near.github.io/intents/defuse/intents/trait.Intents.html#tymethod.simulate_intents) function runs intent code **without** modifying the Verifier contract's state. It accepts the same [`MultiPayload`](https://near.github.io/intents/defuse_core/payload/multi/enum.MultiPayload.html) input as `execute_intents`.

* For intent structure and types, see [Intent Types and Execution](/integration/verifier-contract/intent-types-and-execution).
* For signing intents before simulation, see [Signing Intents](/integration/verifier-contract/signing-intents).

## When to use simulations

<CardGroup cols={3}>
  <Card title="Validate signatures" icon="key">
    Verify that your intent's digital signature format is valid before executing
  </Card>

  <Card title="Test withdrawals" icon="arrow-right-from-bracket">
    Ensure a withdrawal will work before committing to it
  </Card>

  <Card title="Check fees" icon="receipt">
    See the fees that will be paid to the Verifier contract for a trade
  </Card>
</CardGroup>

## Example simulation

Here's a valid, signed intent to trade 100 USDC for 100 USDT between Charlie and Drake:

```json theme={null}
{
  "signed": [
    {
      "standard": "nep413",
      "payload": {
        "recipient": "intents.near",
        "nonce": "Vij2xgAlKBKzwJcGrQQYQhiLk1HU5AVNH1M3PhtxosE=",
        "message": "{\"deadline\":\"2025-05-23T07:40:13.735337Z\",\"intents\":[{\"intent\":\"token_diff\",\"diff\":{\"nep141:usdc.near\":\"-100\",\"nep141:usdt.near\":\"100\"}}],\"signer_id\":\"charlie.near\"}"
      },
      "public_key": "ed25519:C3jXhkGhEx88Gj7XKtUziJKXEBMRaJ67bWFkxJikVxZ2",
      "signature": "ed25519:617X6QMiwFohRHqEuFwZ8aGU6Gn8PsH1DM3grArCYSKSvLz4wBPPGLzLPX3SLstLB331ESGPUToaPkUE7DvgefUu"
    },
    {
      "standard": "nep413",
      "payload": {
        "recipient": "intents.near",
        "nonce": "Vij2xgAlKBKzwJcGrQQYQhjtxxGCsZQhM2DP7btPexE=",
        "message": "{\"deadline\":\"2025-05-23T07:40:13.753085Z\",\"intents\":[{\"intent\":\"token_diff\",\"diff\":{\"nep141:usdc.near\":\"100\",\"nep141:usdt.near\":\"-100\"}}],\"signer_id\":\"drake.near\"}"
      },
      "public_key": "ed25519:C3jXhkGhEx88Gj7XKtUziJKXEBMRaJ67bWFkxJikVxZ2",
      "signature": "ed25519:3mLCEKyhofYLVakC9qgyb2FWh4L3jQxnUNyBHxYMTC13bo9y4AeqRh29dDYC4ZAQk4Z4mA2QZL8y7KGGKp5Pc3S1"
    }
  ]
}
```

Calling `simulate_intents` produces:

```json theme={null}
{
  "intents_executed": [
    {
      "intent_hash": "5GpL6PsUQVHFYAk5FWEwBUaEQqcZkc2SjTvPYHgHAnx8",
      "account_id": "charlie.near",
      "nonce": "Vij2xgAlKBKzwJcGrQQYQhiLk1HU5AVNH1M3PhtxosE="
    },
    {
      "intent_hash": "4ejradLAAPBhBVAn6tBYExpuj2VCn5f5VEBfVajNXiXk",
      "account_id": "drake.near",
      "nonce": "Vij2xgAlKBKzwJcGrQQYQhjtxxGCsZQhM2DP7btPexE="
    }
  ],
  "logs": [
    "EVENT_JSON:{\"data\":[{\"account_id\":\"charlie.near\",\"diff\":{\"nep141:usdc.near\":\"-100\",\"nep141:usdt.near\":\"100\"},\"intent_hash\":\"5GpL6PsUQVHFYAk5FWEwBUaEQqcZkc2SjTvPYHgHAnx8\"}],\"event\":\"token_diff\",\"standard\":\"dip4\",\"version\":\"0.3.0\"}",
    "EVENT_JSON:{\"data\":[{\"account_id\":\"drake.near\",\"diff\":{\"nep141:usdc.near\":\"100\",\"nep141:usdt.near\":\"-100\"},\"intent_hash\":\"4ejradLAAPBhBVAn6tBYExpuj2VCn5f5VEBfVajNXiXk\"}],\"event\":\"token_diff\",\"standard\":\"dip4\",\"version\":\"0.3.0\"}",
    "EVENT_JSON:{\"data\":[{\"account_id\":\"charlie.near\",\"intent_hash\":\"5GpL6PsUQVHFYAk5FWEwBUaEQqcZkc2SjTvPYHgHAnx8\",\"nonce\":\"Vij2xgAlKBKzwJcGrQQYQhiLk1HU5AVNH1M3PhtxosE=\"},{\"account_id\":\"drake.near\",\"intent_hash\":\"4ejradLAAPBhBVAn6tBYExpuj2VCn5f5VEBfVajNXiXk\",\"nonce\":\"Vij2xgAlKBKzwJcGrQQYQhjtxxGCsZQhM2DP7btPexE=\"}],\"event\":\"intents_executed\",\"standard\":\"dip4\",\"version\":\"0.3.0\"}"
  ],
  "min_deadline": "2025-05-23T07:40:13.735337Z",
  "state": {
    "fee": 100,
    "current_salt": "252812b3"
  }
}
```

## Response fields

| Field                            | Description                                                                                         |
| -------------------------------- | --------------------------------------------------------------------------------------------------- |
| `intents_executed`               | Array of intent events collected during simulation.                                                 |
| `intents_executed[].intent_hash` | Unique hash identifying the intent.                                                                 |
| `intents_executed[].account_id`  | NEAR account that signed the intent.                                                                |
| `intents_executed[].nonce`       | Base64-encoded [nonce](https://github.com/near/intents/tree/main/defuse#nonces) used by the intent. |
| `logs`                           | Array of DIP-4 event JSON strings that would be emitted during execution.                           |
| `min_deadline`                   | Earliest deadline among all intents in the batch.                                                   |
| `state.fee`                      | Current fee in pips.                                                                                |
| `state.current_salt`             | Current 4-byte [salt](https://github.com/near/intents/tree/main/defuse#salt) value (hex-encoded).   |

<Info>
  Fees are expressed in pips—100 pips equals 0.01%.
</Info>

<Tip>
  Use `current_salt` from the simulation response when constructing [versioned nonces](https://github.com/near/intents/tree/main/defuse#nonces). A nonce is only valid if its embedded salt matches one of the salts in the contract's registry.
</Tip>

Simulation outputs may include additional data in future updates. Contact the NEAR Intents team if your application requires more detailed output.

## Accuracy of simulations

Simulated results are designed to closely match actual execution outcomes through extensive testing.

However, due to the asynchronous nature of the NEAR blockchain, it's not possible to simulate intents exactly as they would execute in reality.

<Tip>
  To date, simulation and execution results have always matched. If you discover a case where they differ, please contact the NEAR Intents team and report it as a bug.
</Tip>

<Warning>
  Simulations reflect only side effects within the Verifier contract and **exclude** those from external asynchronous calls.
</Warning>
