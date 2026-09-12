> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Swap SDK

> Client libraries for integrating with the 1Click Swap API

The 1Click SDK provides client libraries in multiple languages for seamless cross-chain token swaps using the [1Click Swap API](/integration/distribution-channels/1click-api/about-1click-api).

<CardGroup cols={3}>
  <Card title="TypeScript" icon="js" href="https://github.com/defuse-protocol/one-click-sdk-typescript">
    npm package
  </Card>

  <Card title="Go" icon="golang" href="https://github.com/defuse-protocol/one-click-sdk-go">
    Go module
  </Card>

  <Card title="Rust" icon="rust" href="https://github.com/defuse-protocol/one-click-sdk-rs">
    Cargo crate
  </Card>
</CardGroup>

***

## Using the SDK

<Warning>
  There is no testnet version of NEAR Intents - use small amounts for test swaps.
</Warning>

<Steps>
  <Step title="Prerequisites">
    * A small amount of tokens on ANY [supported chain](/resources/chain-support).
    * A [JWT token](/integration/distribution-channels/1click-api/authentication) to avoid the 0.2% fee.
  </Step>

  <Step title="Install the SDK package">
    <CodeGroup>
      ```bash TypeScript theme={null}
      npm install @defuse-protocol/one-click-sdk-typescript
      ```

      ```bash Go theme={null}
      go get github.com/defuse-protocol/one-click-sdk-go
      ```

      ```bash Rust theme={null}
      # Clone the repo next to your project
      git clone https://github.com/defuse-protocol/one-click-sdk-rs.git

      # Add to your Cargo.toml under [dependencies]
      # one-click-sdk-rs = { path = "./one-click-sdk-rs" }
      ```
    </CodeGroup>
  </Step>

  <Step title="Configure client">
    <CodeGroup>
      ```typescript TypeScript theme={null}
      import { OpenAPI } from '@defuse-protocol/one-click-sdk-typescript';

      // The base URL defaults to https://1click.chaindefuser.com
      OpenAPI.BASE = 'https://1click.chaindefuser.com';

      // Required for getQuote, submitDepositTx, and getExecutionStatus
      OpenAPI.TOKEN = 'YOUR_JWT_TOKEN';
      ```

      ```go Go theme={null}
      import (
        "context"
        "fmt"
        openapiclient "github.com/defuse-protocol/one-click-sdk-go"
      )

      configuration := openapiclient.NewConfiguration()
      apiClient := openapiclient.NewAPIClient(configuration)

      authCtx := context.WithValue(
        context.Background(),
        openapiclient.ContextAccessToken,
        "YOUR_JWT_TOKEN",
      )
      ```

      ```rust Rust theme={null}
      use one_click_sdk_rs::apis::configuration::Configuration;

      // base_path already defaults to https://1click.chaindefuser.com
      let config = Configuration {
          bearer_access_token: Some("YOUR_JWT_TOKEN".to_string()),
          ..Default::default()
      };
      ```
    </CodeGroup>
  </Step>

  <Step title="Query supported tokens">
    <CodeGroup>
      ```typescript TypeScript theme={null}
      import { OneClickService } from '@defuse-protocol/one-click-sdk-typescript';

      const tokens = await OneClickService.getTokens();
      console.log(tokens);
      ```

      ```go Go theme={null}
      // GetTokens does not require authentication
      tokens, _, err := apiClient.OneClickAPI.GetTokens(context.Background()).Execute()
      if err != nil {
        fmt.Fprintf(os.Stderr, "GetTokens error: %v\n", err)
        panic(err)
      }
      fmt.Printf("Supported tokens: %v\n", tokens)
      ```

      ```rust Rust theme={null}
      use one_click_sdk_rs::apis::one_click_api;

      // get_tokens does not require authentication
      let tokens = one_click_api::get_tokens(&config).await?;
      println!("{:?}", tokens);
      ```
    </CodeGroup>
  </Step>

  <Step title="Request a quote">
    <CodeGroup>
      ```typescript TypeScript theme={null}
      import { OneClickService, QuoteRequest } from '@defuse-protocol/one-click-sdk-typescript';

      // Example: swap USDC on Arbitrum for USDC on Solana
      const quoteRequest: QuoteRequest = {
        dry: false,                                    // true = simulate only, no depositAddress returned
        swapType: QuoteRequest.swapType.EXACT_INPUT,
        slippageTolerance: 100,                        // basis points (100 = 1%)
        originAsset: 'nep141:arb-0xaf88d065e77c8cc2239327c5edb3a432268e5831.omft.near',
        depositType: QuoteRequest.depositType.ORIGIN_CHAIN,
        destinationAsset: 'nep141:sol-5ce3bf3a31af18be40ba30f721101b4341690186.omft.near',
        amount: '1000000',                             // 1 USDC in smallest units (6 decimals)
        refundTo: '0xYourArbitrumAddress',
        refundType: QuoteRequest.refundType.ORIGIN_CHAIN,
        recipient: 'YourSolanaAddress',
        recipientType: QuoteRequest.recipientType.DESTINATION_CHAIN,
        deadline: new Date(Date.now() + 3 * 60 * 1000).toISOString(),
      };

      const quote = await OneClickService.getQuote(quoteRequest);
      ```

      ```go Go theme={null}
      import "time"

      // Example: swap USDC on Arbitrum for USDC on Solana
      quoteRequest := *openapiclient.NewQuoteRequest(
        false,                   // dry
        "EXACT_INPUT",           // swapType
        float32(100),            // slippageTolerance (basis points, 100 = 1%)
        "nep141:arb-0xaf88d065e77c8cc2239327c5edb3a432268e5831.omft.near", // originAsset
        "ORIGIN_CHAIN",          // depositType
        "nep141:sol-5ce3bf3a31af18be40ba30f721101b4341690186.omft.near",   // destinationAsset
        "1000000",               // amount (1 USDC, 6 decimals)
        "0xYourArbitrumAddress", // refundTo
        "ORIGIN_CHAIN",          // refundType
        "YourSolanaAddress",     // recipient
        "DESTINATION_CHAIN",     // recipientType
        time.Now().Add(3*time.Minute), // deadline
      )

      quote, _, err := apiClient.OneClickAPI.GetQuote(authCtx).QuoteRequest(quoteRequest).Execute()
      if err != nil {
        fmt.Fprintf(os.Stderr, "GetQuote error: %v\n", err)
        panic(err)
      }
      ```

      ```rust Rust theme={null}
      use one_click_sdk_rs::apis::one_click_api;
      use one_click_sdk_rs::models::{DepositType, QuoteRequest, RecipientType, RefundType, SwapType};

      // Example: swap USDC on Arbitrum for USDC on Solana
      let quote_request = QuoteRequest::new(
          false,                        // dry
          SwapType::ExactInput,
          100.0,                        // slippage_tolerance (basis points, 100 = 1%)
          "nep141:arb-0xaf88d065e77c8cc2239327c5edb3a432268e5831.omft.near".to_string(), // origin_asset
          DepositType::OriginChain,
          "nep141:sol-5ce3bf3a31af18be40ba30f721101b4341690186.omft.near".to_string(), // destination_asset
          "1000000".to_string(),        // amount (1 USDC, 6 decimals)
          "0xYourArbitrumAddress".to_string(), // refund_to
          RefundType::OriginChain,
          "YourSolanaAddress".to_string(), // recipient
          RecipientType::DestinationChain,
          "2025-12-31T23:59:59Z".to_string(), // deadline (ISO 8601)
      );

      let quote = one_click_api::get_quote(&config, quote_request).await?;
      ```
    </CodeGroup>

    Save `quote.quote.depositAddress` (and `quote.quote.depositMemo` if present). You will use these for deposit and status checks.
  </Step>

  <Step title="Send tokens">
    Transfer tokens to the `depositAddress` (on the origin chain) from the quote response. The swap begins automatically upon receipt.

    Save your transaction hash for optional submission and tracking.
  </Step>

  <Step title="Submit transaction hash (optional)">
    Submitting the transaction hash lets the service detect your deposit faster and start processing sooner.

    <CodeGroup>
      ```typescript TypeScript theme={null}
      await OneClickService.submitDepositTx({
        depositAddress: quote.quote.depositAddress!,
        txHash: '0xYourTransactionHash',
      });
      ```

      ```go Go theme={null}
      submitRequest := *openapiclient.NewSubmitDepositTxRequest(
        "0xYourTransactionHash",
        *quote.Quote.DepositAddress,
      )

      _, _, err = apiClient.OneClickAPI.
        SubmitDepositTx(authCtx).
        SubmitDepositTxRequest(submitRequest).
        Execute()
      if err != nil {
        fmt.Fprintf(os.Stderr, "SubmitDepositTx error: %v\n", err)
      }
      ```

      ```rust Rust theme={null}
      use one_click_sdk_rs::models::SubmitDepositTxRequest;

      let submit_request = SubmitDepositTxRequest::new(
          "0xYourTransactionHash".to_string(),
          quote.quote.deposit_address.clone().unwrap_or_default(),
      );

      one_click_api::submit_deposit_tx(&config, submit_request).await?;
      ```
    </CodeGroup>
  </Step>

  <Step title="Monitor status">
    Poll until `status` reaches a terminal state (`SUCCESS`, `REFUNDED`, or `FAILED`).

    <CodeGroup>
      ```typescript TypeScript theme={null}
      const status = await OneClickService.getExecutionStatus(quote.quote.depositAddress!);
      console.log(status.status);
      ```

      ```go Go theme={null}
      status, _, err := apiClient.OneClickAPI.
        GetExecutionStatus(authCtx).
        DepositAddress(*quote.Quote.DepositAddress).
        Execute()
      if err != nil {
        fmt.Fprintf(os.Stderr, "GetExecutionStatus error: %v\n", err)
        panic(err)
      }
      fmt.Println(*status.Status)
      ```

      ```rust Rust theme={null}
      let status = one_click_api::get_execution_status(
          &config,
          quote.quote.deposit_address.as_deref().unwrap_or_default(),
      ).await?;

      println!("{:?}", status.status);
      ```
    </CodeGroup>

    <Accordion title="Status values">
      | Status               | Description                                   |
      | -------------------- | --------------------------------------------- |
      | `PENDING_DEPOSIT`    | Awaiting your token deposit                   |
      | `KNOWN_DEPOSIT_TX`   | Deposit transaction detected                  |
      | `PROCESSING`         | Swap being executed                           |
      | `SUCCESS`            | Tokens delivered to destination address       |
      | `INCOMPLETE_DEPOSIT` | Deposit below required amount                 |
      | `REFUNDED`           | Swap failed, funds returned to refund address |
      | `FAILED`             | Swap encountered an error                     |
    </Accordion>
  </Step>
</Steps>
