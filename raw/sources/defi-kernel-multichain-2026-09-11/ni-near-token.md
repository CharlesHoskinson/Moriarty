> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Using NEAR Tokens

> How to use NEAR tokens with NEAR Intents

The Verifier contract only accepts [NEP-141](https://nomicon.io/Standards/Tokens/FungibleToken/Core) tokens, so native NEAR must be wrapped before depositing. The [`wrap.near`](https://nearblocks.io/address/wrap.near) contract converts NEAR to wNEAR at a 1:1 ratio.

See [wNEAR docs](https://github.com/near/core-contracts/tree/master/w-near) for a detailed reference guide.

<Tip>
  If you're using the [1Click Swap
  API](/integration/distribution-channels/1click-api/about-1click-api) or
  another managed integration, wrapping is handled for you - skip this guide.
</Tip>

## Wrap and deposit

If you're integrating directly with the Verifier contract, you must wrap your NEAR first before depositing:

<Steps>
  <Step title="Wrap your NEAR">
    Call `near_deposit` on the `wrap.near` contract, attaching the amount of NEAR you want to wrap. This example wraps 0.01 NEAR:

    <Tabs>
      <Tab title="NEAR CLI">
        ```bash theme={null}
        near call wrap.near near_deposit '{}' \
          --deposit 0.01 \
          --gas 30000000000000 \
          --useAccount your-account.near \
          --networkId mainnet
        ```
      </Tab>

      <Tab title="NEAR API JS">
        ```typescript theme={null}
        import { Account, JsonRpcProvider, teraToGas, nearToYocto, KeyPairString } from "near-api-js";

        const accountId = "your-account.near";
        const privateKey = "ed25519:3D4YudU..." as KeyPairString;

        const provider = new JsonRpcProvider({ url: "https://rpc.fastnear.com" });
        const account = new Account(accountId, provider, privateKey);

        await account.callFunction({
          contractId: "wrap.near",
          methodName: "near_deposit",
          args: {},
          gas: teraToGas("30"),
          deposit: nearToYocto("0.01"),
        });
        ```

        See contract interaction example in [near-api-examples](https://github.com/near-examples/near-api-examples/blob/main/near-api-js/examples/contract-interaction.ts) repository.
      </Tab>
    </Tabs>
  </Step>

  <Step title="Deposit wNEAR into the Verifier contract">
    Call `ft_transfer_call` on `wrap.near` to deposit your wNEAR. wNEAR has 24 decimals, so 0.01 wNEAR = `10000000000000000000000`.

    <Tabs>
      <Tab title="NEAR CLI">
        ```bash theme={null}
        near call wrap.near ft_transfer_call \
          '{"receiver_id": "intents.near", "amount": "10000000000000000000000", "msg": ""}' \
          --deposit 0.000000000000000000000001 \
          --gas 100000000000000 \
          --useAccount your-account.near \
          --networkId mainnet
        ```
      </Tab>

      <Tab title="NEAR API JS">
        ```typescript theme={null}
        await account.callFunction({
          contractId: "wrap.near",
          methodName: "ft_transfer_call",
          args: {
            receiver_id: "intents.near",
            amount: "10000000000000000000000",
            msg: "",
          },
          gas: teraToGas("100"),
          deposit: 1n, // 1 yoctoNEAR
        });
        ```

        See contract interaction example in [near-api-examples](https://github.com/near-examples/near-api-examples/blob/main/near-api-js/examples/contract-interaction.ts) repository.
      </Tab>
    </Tabs>

    <Info>
      In the Verifier contract, wrapped NEAR is identified as `nep141:wrap.near`.
    </Info>

    See [Deposits](/integration/verifier-contract/deposits-and-withdrawals/deposits) for details on the `msg` parameter and other deposit options.
  </Step>
</Steps>
