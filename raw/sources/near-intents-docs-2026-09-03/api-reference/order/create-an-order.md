> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Create an order

> Creates a confidential order.

**Errors.** Branch on `code`, never on `title` or `detail`. New codes arrive without a major
version, so treat an unrecognized code as the HTTP status alone.

| Status | Codes |
| --- | --- |
| 400 | `malformed-request`, `validation-failed`, `request-rejected`, `order-rejected` |
| 401 | `authentication-required` |
| 403 | `client-generated-id` |
| 409 | `resource-type-mismatch` |
| 429 | `rate-limit-exceeded` |
| 500 | `internal-error` |
| 503 | `service-unavailable` |



## OpenAPI

````yaml https://1click.chaindefuser.com/docs/v0/openapi.yaml post /v0/orders
openapi: 3.0.0
info:
  title: 1Click Swap API
  description: API for One-Click Swaps
  version: 0.1.10
  contact: {}
servers:
  - url: https://1click.chaindefuser.com
security: []
tags: []
paths:
  /v0/orders:
    post:
      tags:
        - Order
      summary: Create an order
      description: >-
        Creates a confidential order.


        **Errors.** Branch on `code`, never on `title` or `detail`. New codes
        arrive without a major

        version, so treat an unrecognized code as the HTTP status alone.


        | Status | Codes |

        | --- | --- |

        | 400 | `malformed-request`, `validation-failed`, `request-rejected`,
        `order-rejected` |

        | 401 | `authentication-required` |

        | 403 | `client-generated-id` |

        | 409 | `resource-type-mismatch` |

        | 429 | `rate-limit-exceeded` |

        | 500 | `internal-error` |

        | 503 | `service-unavailable` |
      operationId: createOrder
      parameters: []
      requestBody:
        required: true
        content:
          application/json:
            schema:
              type: object
              required:
                - data
              properties:
                data:
                  type: object
                  required:
                    - type
                    - attributes
                  properties:
                    type:
                      type: string
                      enum:
                        - orders
                    attributes:
                      oneOf:
                        - $ref: '#/components/schemas/LimitOrderCreateRequest'
                      discriminator:
                        propertyName: orderType
                        mapping:
                          LIMIT:
                            $ref: '#/components/schemas/LimitOrderCreateRequest'
      responses:
        '201':
          description: Order created.
          content:
            application/json:
              schema:
                type: object
                required:
                  - data
                  - meta
                properties:
                  data:
                    type: object
                    required:
                      - type
                      - id
                      - attributes
                      - links
                    properties:
                      type:
                        type: string
                        enum:
                          - orders
                      id:
                        type: string
                        description: >-
                          Opaque resource identifier. Clients should treat IDs
                          as opaque and not rely on the prefix or length.
                      attributes:
                        oneOf:
                          - $ref: '#/components/schemas/LimitOrderAttributes'
                        discriminator:
                          propertyName: orderType
                          mapping:
                            LIMIT:
                              $ref: '#/components/schemas/LimitOrderAttributes'
                      links:
                        type: object
                        required:
                          - self
                        properties:
                          self:
                            type: string
                  meta:
                    $ref: '#/components/schemas/JsonApiMeta'
        '400':
          description: >-
            Malformed request, Validation failed, Request rejected, Order
            rejected
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/JsonApiErrorDocument'
        '401':
          description: Authentication required
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/JsonApiErrorDocument'
        '403':
          description: Client generated id
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/JsonApiErrorDocument'
        '409':
          description: Resource type mismatch
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/JsonApiErrorDocument'
        '429':
          description: Rate limit exceeded
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/JsonApiErrorDocument'
        '500':
          description: Internal error
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/JsonApiErrorDocument'
        '503':
          description: Service unavailable
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/JsonApiErrorDocument'
      security:
        - X-API-Key: []
        - JWT-auth: []
components:
  schemas:
    LimitOrderCreateRequest:
      type: object
      properties:
        orderType:
          description: Order type.
          allOf:
            - $ref: '#/components/schemas/LimitOrderTypeEnum'
        baseAsset:
          type: string
          description: ID of the base asset in the trading pair.
          example: nep141:arb-0xaf88d065e77c8cc2239327c5edb3a432268e5831.omft.near
        quoteAsset:
          type: string
          description: ID of the quote asset used to price the base asset.
          example: nep141:sol-5ce3bf3a31af18be40ba30f721101b4341690186.omft.near
        quantity:
          type: string
          description: >-
            Base-asset order quantity in the asset smallest unit. A BUY targets
            this output; a SELL targets this input.
          example: '1000000'
        side:
          description: Order side. SELL spends the base asset; BUY spends the quote asset.
          allOf:
            - $ref: '#/components/schemas/OrderSideEnum'
        price:
          type: string
          description: >-
            Positive human-unit limit price expressed as quote asset units per
            one base asset
          example: 100 or 0.5
        depositType:
          description: >-
            ORIGIN_CHAIN returns a chain address; INTENTS and
            CONFIDENTIAL_INTENTS use the generate-intent and submit-intent flow.
          allOf:
            - $ref: '#/components/schemas/DepositTypeEnum'
        refundTo:
          type: string
          description: Address used for refunds.
          example: '0x2527D02599Ba641c19FEa793cD0F167589a0f10D'
        refundType:
          description: Type of refund address.
          allOf:
            - $ref: '#/components/schemas/RefundTypeEnum'
        recipient:
          type: string
          description: Address or Intents account that receives successfully filled output.
          example: 13QkxhNMrTPxoCkRdYdJ65tFuwXPhL5gLS2Z5Nr6gjRK
        recipientType:
          description: Type of recipient address.
          allOf:
            - $ref: '#/components/schemas/RecipientTypeEnum'
        deadline:
          format: date-time
          type: string
          description: Timestamp in ISO format that identifies when the order can expire.
          example: '2019-08-24T14:15:22Z'
        depositMode:
          description: Deposit address mode.
          default: SIMPLE
          allOf:
            - $ref: '#/components/schemas/DepositModeEnum'
        confidentiality:
          description: Confidentiality mode. Required for orders.
          allOf:
            - $ref: '#/components/schemas/ConfidentialityLevelEnum'
        timeInForce:
          description: Time in force for the order.
          default: GTC
          allOf:
            - $ref: '#/components/schemas/TimeInForceEnum'
        appFees:
          description: >-
            Application fees included in the submitted limit price and deducted
            from input.
          type: array
          items:
            $ref: '#/components/schemas/AppFee'
      required:
        - orderType
        - baseAsset
        - quoteAsset
        - quantity
        - side
        - price
        - depositType
        - refundTo
        - refundType
        - recipient
        - recipientType
        - confidentiality
    LimitOrderAttributes:
      type: object
      properties:
        orderType:
          description: Order type.
          allOf:
            - $ref: '#/components/schemas/LimitOrderTypeEnum'
        depositAddress:
          type: string
          description: Deposit address for funding the order.
          example: 0x2527D...a0f10D
        depositMemo:
          type: string
          description: Memo to include with the deposit, when required.
          example: '123456'
        depositMode:
          description: Deposit address mode.
          allOf:
            - $ref: '#/components/schemas/DepositModeEnum'
        depositType:
          description: Type of deposit address.
          allOf:
            - $ref: '#/components/schemas/DepositTypeEnum'
        fillStatus:
          description: >-
            Current fill status. This does not show whether payout funds
            arrived.
          allOf:
            - $ref: '#/components/schemas/OrderFillStatusEnum'
        payoutStatus:
          description: Aggregated progress of the main withdrawal and refund.
          allOf:
            - $ref: '#/components/schemas/OrderPayoutStatusEnum'
        isPayoutStatusFinal:
          type: boolean
          description: >-
            Whether payoutStatus is COMPLETED or FAILED. This field is derived
            and has no additional meaning.
        payouts:
          description: Current withdrawal and refund legs.
          allOf:
            - $ref: '#/components/schemas/OrderPayouts'
        depositedAmount:
          type: string
          description: >-
            Cumulative deposited input amount in the smallest unit of the base
            asset for SELL or the quote asset for BUY.
          example: '1000000000'
        depositedAmountFormatted:
          type: string
          description: >-
            Cumulative deposited input amount formatted using the input asset
            decimals.
          example: '1.0'
        baseAsset:
          type: string
          description: ID of the base asset in the trading pair.
        quoteAsset:
          type: string
          description: ID of the quote asset used to price the base asset.
        quantity:
          type: string
          description: >-
            Base-asset order quantity in the asset smallest unit. A BUY targets
            this output; a SELL targets this input.
        swapView:
          description: >-
            Read-only swap representation derived from the order side and limit
            price.
          oneOf:
            - $ref: '#/components/schemas/LimitOrderExactInputSwapView'
            - $ref: '#/components/schemas/LimitOrderExactOutputSwapView'
          discriminator:
            propertyName: swapType
            mapping:
              EXACT_INPUT:
                $ref: '#/components/schemas/LimitOrderExactInputSwapView'
              EXACT_OUTPUT:
                $ref: '#/components/schemas/LimitOrderExactOutputSwapView'
        side:
          description: Submitted order side.
          allOf:
            - $ref: '#/components/schemas/OrderSideEnum'
        price:
          type: string
          description: Submitted human-unit limit price.
        appFees:
          description: Normalized application and protocol fees.
          type: array
          items:
            $ref: '#/components/schemas/AppFee'
        recipient:
          type: string
          description: Recipient address supplied when creating the order.
        recipientType:
          description: Type of recipient address.
          allOf:
            - $ref: '#/components/schemas/RecipientTypeEnum'
        refundTo:
          type: string
          description: Refund address supplied when creating the order.
        refundType:
          description: Type of refund address.
          allOf:
            - $ref: '#/components/schemas/RefundTypeEnum'
        estimatedWithdrawFee:
          type: string
          description: >-
            Withdrawal-fee estimate captured at creation, in destination-asset
            smallest units. It covers one withdrawal. The fee charged for each
            withdrawal may differ.
        estimatedWithdrawFeeFormatted:
          type: string
          description: >-
            Withdrawal-fee estimate formatted using the destination asset
            decimals.
        estimatedRefundFee:
          type: string
          description: >-
            Refund-fee estimate captured at creation, in input-asset smallest
            units. The fee charged for the refund may differ.
        estimatedRefundFeeFormatted:
          type: string
          description: Refund-fee estimate formatted using the input asset decimals.
        confidentiality:
          description: Confidentiality mode for this order.
          allOf:
            - $ref: '#/components/schemas/ConfidentialityLevelEnum'
        timeInForce:
          description: Time in force for the order.
          allOf:
            - $ref: '#/components/schemas/TimeInForceEnum'
        deadline:
          format: date-time
          type: string
          description: Timestamp in ISO format that identifies when the order can expire.
          example: '2019-08-24T14:15:22Z'
        createdAt:
          format: date-time
          type: string
          description: Timestamp in ISO format that identifies when the order was created.
          example: '2019-08-24T14:15:22Z'
        partialFills:
          description: >-
            Successful fills. Fills that are in flight or failed are not
            reported.
          type: array
          items:
            $ref: '#/components/schemas/LimitOrderPartialFill'
      required:
        - orderType
        - depositAddress
        - depositMode
        - depositType
        - fillStatus
        - payoutStatus
        - isPayoutStatusFinal
        - payouts
        - depositedAmount
        - depositedAmountFormatted
        - baseAsset
        - quoteAsset
        - quantity
        - swapView
        - side
        - price
        - appFees
        - recipient
        - recipientType
        - refundTo
        - refundType
        - estimatedWithdrawFee
        - estimatedWithdrawFeeFormatted
        - estimatedRefundFee
        - estimatedRefundFeeFormatted
        - confidentiality
        - timeInForce
        - deadline
        - createdAt
    JsonApiMeta:
      type: object
      properties:
        correlationId:
          type: string
          description: Unique identifier for request tracing and debugging.
          example: 550e8400-e29b-41d4-a716-446655440000
        timestamp:
          format: date-time
          type: string
          description: Timestamp in ISO format when the response was generated.
          example: '2019-08-24T14:15:22Z'
      required:
        - correlationId
        - timestamp
    JsonApiErrorDocument:
      type: object
      properties:
        errors:
          description: >-
            Always populated, and never sent with data. One entry per failed
            field, so a request with several invalid attributes returns several
            errors. The codes an operation can return are listed in its
            description.
          type: array
          items:
            $ref: '#/components/schemas/JsonApiError'
        meta:
          $ref: '#/components/schemas/JsonApiMeta'
      required:
        - errors
        - meta
    LimitOrderTypeEnum:
      type: string
      enum:
        - LIMIT
    OrderSideEnum:
      type: string
      enum:
        - SELL
        - BUY
    DepositTypeEnum:
      type: string
      enum:
        - ORIGIN_CHAIN
        - INTENTS
        - CONFIDENTIAL_INTENTS
    RefundTypeEnum:
      type: string
      enum:
        - ORIGIN_CHAIN
        - INTENTS
        - CONFIDENTIAL_INTENTS
    RecipientTypeEnum:
      type: string
      enum:
        - DESTINATION_CHAIN
        - INTENTS
        - CONFIDENTIAL_INTENTS
    DepositModeEnum:
      type: string
      enum:
        - SIMPLE
        - MEMO
    ConfidentialityLevelEnum:
      type: string
      enum:
        - public
        - basic
        - advanced
    TimeInForceEnum:
      type: string
      enum:
        - GTC
    AppFee:
      type: object
      properties:
        recipient:
          type: string
          description: Account ID within Intents to which this fee will be transferred
          example: recipient.near
        fee:
          type: number
          description: >-
            Fee for this recipient as part of amountIn in basis points (1/100th
            of a percent), e.g. 100 for 1% fee
          example: 100
      required:
        - recipient
        - fee
    OrderFillStatusEnum:
      type: string
      enum:
        - AWAITING_DEPOSIT
        - PENDING_CANCEL
        - UNTRIGGERED
        - OPEN
        - PARTIALLY_FILLED
        - FILLED
        - CANCELED
        - EXPIRED
    OrderPayoutStatusEnum:
      type: string
      enum:
        - NOT_STARTED
        - IN_PROGRESS
        - COMPLETED
        - FAILED
    OrderPayouts:
      type: object
      properties:
        withdrawal:
          description: >-
            Latest successful withdrawal, or current withdrawal when none
            succeeded.
          nullable: true
          oneOf:
            - $ref: '#/components/schemas/OrderPayoutInProgress'
            - $ref: '#/components/schemas/OrderPayoutCompleted'
            - $ref: '#/components/schemas/PayoutFailed'
          discriminator:
            propertyName: status
            mapping:
              IN_PROGRESS:
                $ref: '#/components/schemas/OrderPayoutInProgress'
              COMPLETED:
                $ref: '#/components/schemas/OrderPayoutCompleted'
              FAILED:
                $ref: '#/components/schemas/PayoutFailed'
        allWithdrawals:
          type: array
          description: All successful withdrawals, ordered from oldest to newest.
          items:
            oneOf:
              - $ref: '#/components/schemas/OrderPayoutInProgress'
              - $ref: '#/components/schemas/OrderPayoutCompleted'
              - $ref: '#/components/schemas/PayoutFailed'
            discriminator:
              propertyName: status
              mapping:
                IN_PROGRESS:
                  $ref: '#/components/schemas/OrderPayoutInProgress'
                COMPLETED:
                  $ref: '#/components/schemas/OrderPayoutCompleted'
                FAILED:
                  $ref: '#/components/schemas/PayoutFailed'
        refund:
          description: Main refund, or null when no refund exists.
          nullable: true
          oneOf:
            - $ref: '#/components/schemas/OrderPayoutInProgress'
            - $ref: '#/components/schemas/OrderPayoutCompleted'
            - $ref: '#/components/schemas/PayoutFailed'
          discriminator:
            propertyName: status
            mapping:
              IN_PROGRESS:
                $ref: '#/components/schemas/OrderPayoutInProgress'
              COMPLETED:
                $ref: '#/components/schemas/OrderPayoutCompleted'
              FAILED:
                $ref: '#/components/schemas/PayoutFailed'
      required:
        - withdrawal
        - allWithdrawals
        - refund
    LimitOrderExactInputSwapView:
      type: object
      properties:
        swapType:
          type: string
          enum:
            - EXACT_INPUT
        originAsset:
          type: string
        destinationAsset:
          type: string
        amountIn:
          type: string
        minAmountOut:
          type: string
      required:
        - swapType
        - originAsset
        - destinationAsset
        - amountIn
        - minAmountOut
    LimitOrderExactOutputSwapView:
      type: object
      properties:
        swapType:
          type: string
          enum:
            - EXACT_OUTPUT
        originAsset:
          type: string
        destinationAsset:
          type: string
        maxAmountIn:
          type: string
        amountOut:
          type: string
      required:
        - swapType
        - originAsset
        - destinationAsset
        - maxAmountIn
        - amountOut
    LimitOrderPartialFill:
      type: object
      properties:
        id:
          type: string
          description: Public identifier for this fill attempt.
        amountIn:
          type: string
          description: Gross input allocated to the fill, in input-asset smallest units.
        amountOut:
          type: string
          description: Net output received for the fill, in output-asset smallest units.
        createdAt:
          format: date-time
          type: string
          description: ISO 8601 creation timestamp.
        updatedAt:
          format: date-time
          type: string
          description: ISO 8601 last-update timestamp.
      required:
        - id
        - amountIn
        - amountOut
        - createdAt
        - updatedAt
    JsonApiError:
      type: object
      properties:
        status:
          type: string
          description: HTTP status code as a string, per JSON:API.
          example: '400'
        code:
          type: string
          description: >-
            Stable machine-readable identifier. Branch on this, never on title
            or detail. New codes arrive without a major version, so treat an
            unrecognized code as the HTTP status alone. Each response documents
            the codes it can carry; this type is shared by every endpoint and so
            cannot list them.
          example: validation-failed
        title:
          type: string
          description: Short human-readable summary.
          example: Validation failed
        detail:
          type: string
          description: Human-readable explanation of this occurrence.
          example: Must be a positive decimal string.
        source:
          $ref: '#/components/schemas/JsonApiErrorSource'
      required:
        - status
        - code
        - title
    OrderPayoutInProgress:
      type: object
      properties:
        status:
          type: string
          enum:
            - IN_PROGRESS
      required:
        - status
    OrderPayoutCompleted:
      type: object
      properties:
        status:
          type: string
          enum:
            - COMPLETED
        txHash:
          type: string
          description: Destination-chain transaction hash.
      required:
        - status
        - txHash
    PayoutFailed:
      type: object
      properties:
        status:
          type: string
          enum:
            - FAILED
        reason:
          type: string
          description: >-
            Terminal failure reason. This payout leg is not retried. Not
            populated yet - reserved for a defined set of public reasons.
      required:
        - status
    JsonApiErrorSource:
      type: object
      properties:
        pointer:
          type: string
          description: >-
            JSON Pointer to the member of the request document that caused the
            error.
          example: /data/attributes/price
      required:
        - pointer
  securitySchemes:
    X-API-Key:
      type: apiKey
      in: header
      name: X-API-Key
      description: API key for partner authentication (recommended)
    JWT-auth:
      scheme: bearer
      bearerFormat: JWT
      type: http
      description: JWT token in Authorization header (legacy, use X-API-Key instead)

````