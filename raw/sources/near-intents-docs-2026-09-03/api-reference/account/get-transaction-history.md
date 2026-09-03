> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Get transaction history

> Returns paginated public and confidential transaction history. History is invite-only for now. For the initial request, omit both nextCursor and prevCursor to retrieve the latest history. For subsequent requests, pass either nextCursor or prevCursor from the previous response.



## OpenAPI

````yaml https://1click.chaindefuser.com/docs/v0/openapi.yaml get /v0/account/history
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
  /v0/account/history:
    get:
      tags:
        - Account
      summary: Get transaction history
      description: >-
        Returns paginated public and confidential transaction history. History
        is invite-only for now. For the initial request, omit both nextCursor
        and prevCursor to retrieve the latest history. For subsequent requests,
        pass either nextCursor or prevCursor from the previous response.
      operationId: getHistory
      parameters:
        - name: prevCursor
          required: false
          in: query
          description: >-
            Pass the prevCursor value from a previous response to fetch older
            items. Omit both cursors for the initial request. Do not pass
            together with nextCursor.
          schema:
            type: string
        - name: nextCursor
          required: false
          in: query
          description: >-
            Pass the nextCursor value from a previous response to poll for newer
            items. Omit both cursors for the initial request. Do not pass
            together with prevCursor.
          schema:
            type: string
        - name: status
          required: false
          in: query
          description: Filter by statuses
          schema:
            example:
              - PROCESSING
              - SUCCESS
            type: array
            items:
              type: string
              enum:
                - PENDING_DEPOSIT
                - INCOMPLETE_DEPOSIT
                - PROCESSING
                - SUCCESS
                - REFUNDED
                - FAILED
        - name: limit
          required: false
          in: query
          description: Maximum number of items to return per page
          schema:
            minimum: 1
            maximum: 100
            default: 20
            example: 20
            type: integer
        - name: depositAddress
          required: false
          in: query
          description: Filter by deposit address
          schema:
            type: string
        - name: depositMemo
          required: false
          in: query
          description: >-
            Filter by deposit memo. Only has effect when depositAddress is also
            provided.
          schema:
            type: string
        - name: search
          required: false
          in: query
          description: Search by deposit address, recipient, sender, or tx hash
          schema:
            type: string
        - name: depositType
          required: false
          in: query
          description: >-
            Correlated deposit type filter. Index-aligned with recipientType and
            refundType. Use "null" as a wildcard for that position. Multiple
            values form OR conditions.
          schema:
            example:
              - ORIGIN_CHAIN
              - INTENTS
            type: array
            items:
              type: string
              enum:
                - ORIGIN_CHAIN
                - INTENTS
                - CONFIDENTIAL_INTENTS
                - 'null'
        - name: recipientType
          required: false
          in: query
          description: >-
            Correlated recipient type filter. Index-aligned with depositType and
            refundType. Use "null" as a wildcard for that position.
          schema:
            example:
              - DESTINATION_CHAIN
              - INTENTS
            type: array
            items:
              type: string
              enum:
                - DESTINATION_CHAIN
                - INTENTS
                - CONFIDENTIAL_INTENTS
                - 'null'
        - name: refundType
          required: false
          in: query
          description: >-
            Correlated refund type filter. Index-aligned with depositType and
            recipientType. Use "null" as a wildcard for that position.
          schema:
            example:
              - ORIGIN_CHAIN
              - INTENTS
            type: array
            items:
              type: string
              enum:
                - ORIGIN_CHAIN
                - INTENTS
                - CONFIDENTIAL_INTENTS
                - 'null'
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/HistoryResponse'
        '401':
          description: Unauthorized - supplied user session token is invalid or expired
        '403':
          description: History is invite-only for now
components:
  schemas:
    HistoryResponse:
      type: object
      properties:
        items:
          description: >-
            History items for the current page. When fetching older items (via
            prevCursor), fewer items than limit means you reached the end — no
            need to fetch further. For newer items (via nextCursor), new data
            can always appear later.
          type: array
          items:
            $ref: '#/components/schemas/HistoryItem'
        nextCursor:
          type: string
          description: Pass it back as nextCursor to poll for newer items.
        prevCursor:
          type: string
          description: Pass it back as prevCursor to fetch older items.
      required:
        - items
    HistoryItem:
      type: object
      properties:
        status:
          type: string
          description: Execution status
          enum:
            - PENDING_DEPOSIT
            - INCOMPLETE_DEPOSIT
            - PROCESSING
            - SUCCESS
            - REFUNDED
            - FAILED
          example: SUCCESS
        depositType:
          type: string
          description: Deposit type
          enum:
            - ORIGIN_CHAIN
            - INTENTS
            - CONFIDENTIAL_INTENTS
          example: ORIGIN_CHAIN
        recipientType:
          type: string
          description: Recipient type
          enum:
            - DESTINATION_CHAIN
            - INTENTS
            - CONFIDENTIAL_INTENTS
          example: DESTINATION_CHAIN
        createdAt:
          type: string
          description: Creation timestamp (ISO 8601)
          example: '2025-01-15T10:30:00.000000Z'
        depositAddress:
          type: string
          description: Deposit address
        depositMemo:
          type: string
          description: Deposit memo
          nullable: true
        originAsset:
          type: string
          description: ID of the origin asset
          example: nep141:arb-0xaf88d065e77c8cc2239327c5edb3a432268e5831.omft.near
        amountInFormatted:
          type: string
          description: Amount of the origin asset in readable format
          example: '100.50'
        amountInUsd:
          type: string
          description: Amount of the origin asset equivalent in USD
          example: '100.50'
        destinationAsset:
          type: string
          description: ID of the destination asset
          example: nep141:sol-5ce3bf3a31af18be40ba30f721101b4341690186.omft.near
        amountOutFormatted:
          type: string
          description: Amount of the destination asset in readable format
          example: '100.23'
        amountOutUsd:
          type: string
          description: Amount of the destination asset equivalent in USD
          example: '100.23'
        recipient:
          type: string
          description: Recipient address
        quoteTransactions:
          type: array
          description: Deposit transactions with sender and tx hash
          items:
            type: object
            properties:
              sender:
                type: string
              txHash:
                type: string
        refundTo:
          type: string
          description: Refund-to address
        refundType:
          type: string
          description: Refund type
          enum:
            - ORIGIN_CHAIN
            - INTENTS
            - CONFIDENTIAL_INTENTS
        refundReason:
          type: string
          description: Refund reason, null when no refund occurred
          nullable: true
        refundedAmountFormatted:
          type: string
          description: Refunded amount in readable format
        refundedAmountUsd:
          type: string
          description: Refunded amount equivalent in USD
        refundFee:
          type: string
          description: Refund fee in base units of the origin asset
          nullable: true
        refundFeeFormatted:
          type: string
          description: Formatted refund fee
      required:
        - status
        - depositType
        - recipientType
        - createdAt
        - depositAddress
        - depositMemo
        - refundType

````