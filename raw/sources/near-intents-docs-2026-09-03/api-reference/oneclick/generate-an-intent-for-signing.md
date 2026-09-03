> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Generate an intent for signing

> Generates an unsigned intent payload that needs to be signed by the user.

  This endpoint takes a quote or limit-order deposit address, validates the target state and caller ownership when required, and returns an intent payload formatted according to the specified signing standard (e.g., NEP413, ERC191).

  The generated intent must be signed by the user's wallet and then submitted via the `/submit-intent` endpoint to complete the action (e.g. swap).

  **Request Type Variants:**
  - `swap_transfer`: Generate an intent for a swap operation (requires depositAddress, signerId, standard)



## OpenAPI

````yaml https://1click.chaindefuser.com/docs/v0/openapi.yaml post /v0/generate-intent
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
  /v0/generate-intent:
    post:
      tags:
        - OneClick
      summary: Generate an intent for signing
      description: >-
        Generates an unsigned intent payload that needs to be signed by the
        user.

          This endpoint takes a quote or limit-order deposit address, validates the target state and caller ownership when required, and returns an intent payload formatted according to the specified signing standard (e.g., NEP413, ERC191).

          The generated intent must be signed by the user's wallet and then submitted via the `/submit-intent` endpoint to complete the action (e.g. swap).

          **Request Type Variants:**
          - `swap_transfer`: Generate an intent for a swap operation (requires depositAddress, signerId, standard)
      operationId: generateIntent
      parameters: []
      requestBody:
        required: true
        content:
          application/json:
            schema:
              oneOf:
                - $ref: '#/components/schemas/GenerateSwapTransferIntentRequest'
              discriminator:
                propertyName: type
                mapping:
                  swap_transfer:
                    $ref: '#/components/schemas/GenerateSwapTransferIntentRequest'
      responses:
        '200':
          description: Successfully generated intent payload
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/GenerateIntentResponse'
        '400':
          description: Bad Request - Invalid input or quote state
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/BadRequestResponse'
        '401':
          description: Unauthorized - API key or JWT token is invalid
      security:
        - X-API-Key: []
        - JWT-auth: []
components:
  schemas:
    GenerateSwapTransferIntentRequest:
      type: object
      properties:
        type:
          type: string
          description: The type of intent action
          enum:
            - swap_transfer
        standard:
          description: The standard used for signing the intent
          example: nep413
          allOf:
            - $ref: '#/components/schemas/IntentStandardEnum'
        signerId:
          type: string
          description: The account ID of the signer
          example: user.near
        depositAddress:
          type: string
          description: The deposit address from the quote response
          example: '0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb0'
      required:
        - type
        - standard
        - signerId
        - depositAddress
    GenerateIntentResponse:
      type: object
      properties:
        intent:
          oneOf:
            - $ref: '#/components/schemas/MultiPayloadNarrowed'
        correlationId:
          type: string
          description: Unique identifier for tracing this request
          example: 550e8400-e29b-41d4-a716-446655440000
      required:
        - intent
        - correlationId
    BadRequestResponse:
      type: object
      properties:
        message:
          type: string
          example: error message
      required:
        - message
    IntentStandardEnum:
      type: string
      enum:
        - nep413
        - erc191
        - raw_ed25519
        - webauthn
        - ton_connect
        - sep53
        - tip191
    MultiPayloadNarrowed:
      description: >-
        Narrowed MultiPayload union with only 'standard' and 'payload'
        properties exposed
      oneOf:
        - $ref: '#/components/schemas/MultiPayloadNep413Narrowed'
        - $ref: '#/components/schemas/MultiPayloadErc191Narrowed'
        - $ref: '#/components/schemas/MultiPayloadTip191Narrowed'
        - $ref: '#/components/schemas/MultiPayloadRawEd25519Narrowed'
        - $ref: '#/components/schemas/MultiPayloadWebauthnNarrowed'
        - $ref: '#/components/schemas/MultiPayloadTonConnectNarrowed'
        - $ref: '#/components/schemas/MultiPayloadSep53Narrowed'
      discriminator:
        propertyName: standard
        mapping:
          nep413:
            $ref: '#/components/schemas/MultiPayloadNep413Narrowed'
          erc191:
            $ref: '#/components/schemas/MultiPayloadErc191Narrowed'
          tip191:
            $ref: '#/components/schemas/MultiPayloadTip191Narrowed'
          raw_ed25519:
            $ref: '#/components/schemas/MultiPayloadRawEd25519Narrowed'
          webauthn:
            $ref: '#/components/schemas/MultiPayloadWebauthnNarrowed'
          ton_connect:
            $ref: '#/components/schemas/MultiPayloadTonConnectNarrowed'
          sep53:
            $ref: '#/components/schemas/MultiPayloadSep53Narrowed'
    MultiPayloadNep413Narrowed:
      type: object
      required:
        - standard
        - payload
      properties:
        standard:
          type: string
          enum:
            - nep413
        payload:
          $ref: '#/components/schemas/Nep413Payload'
      additionalProperties: false
    MultiPayloadErc191Narrowed:
      type: object
      required:
        - standard
        - payload
      properties:
        standard:
          type: string
          enum:
            - erc191
        payload:
          $ref: '#/components/schemas/Erc191Payload'
      additionalProperties: false
    MultiPayloadTip191Narrowed:
      type: object
      required:
        - standard
        - payload
      properties:
        standard:
          type: string
          enum:
            - tip191
        payload:
          $ref: '#/components/schemas/Tip191Payload'
      additionalProperties: false
    MultiPayloadRawEd25519Narrowed:
      description: Raw Ed25519 payload is an inline string
      type: object
      required:
        - standard
        - payload
      properties:
        standard:
          type: string
          enum:
            - raw_ed25519
        payload:
          type: string
      additionalProperties: false
    MultiPayloadWebauthnNarrowed:
      description: Webauthn payload is an inline string
      type: object
      required:
        - standard
        - payload
      properties:
        standard:
          type: string
          enum:
            - webauthn
        payload:
          type: string
      additionalProperties: false
    MultiPayloadTonConnectNarrowed:
      type: object
      required:
        - standard
        - payload
      properties:
        standard:
          type: string
          enum:
            - ton_connect
        payload:
          $ref: '#/components/schemas/TonConnectPayloadSchema'
      additionalProperties: false
    MultiPayloadSep53Narrowed:
      description: Sep53 payload is an inline string
      type: object
      required:
        - standard
        - payload
      properties:
        standard:
          type: string
          enum:
            - sep53
        payload:
          type: string
      additionalProperties: false
    Nep413Payload:
      description: See [NEP-413](https://github.com/near/NEPs/blob/master/neps/nep-0413.md)
      type: object
      required:
        - message
        - nonce
        - recipient
      properties:
        callbackUrl:
          type: string
          nullable: true
        message:
          type: string
        nonce:
          type: string
          description: 'Encoding: base64'
        recipient:
          type: string
      additionalProperties: false
    Erc191Payload:
      description: >-
        See
        [ERC-191](https://github.com/ethereum/ercs/blob/master/ERCS/erc-191.md)
      type: string
    Tip191Payload:
      description: >-
        See
        [TIP-191](https://github.com/tronprotocol/tips/blob/master/tip-191.md)
      type: string
    TonConnectPayloadSchema:
      description: >-
        See
        <https://docs.tonconsole.com/academy/sign-data#choosing-the-right-format>
      oneOf:
        - type: object
          required:
            - text
            - type
          properties:
            text:
              type: string
            type:
              type: string
              enum:
                - text
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