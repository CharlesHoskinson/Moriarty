> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Refresh Access Token

> Exchange a refresh token for a new access token

Once the `accessToken` from [Authenticate User with Signed Data](/api-reference/user-auth/authenticate-user-with-signed-data) expires, exchange `refreshToken` for a new `accessToken` without re-signing.

## Example request

<CodeGroup>
  ```bash cURL theme={null}
  curl -X POST https://1click.chaindefuser.com/v0/auth/refresh \
    -H "Content-Type: application/json" \
    -d '{ "refreshToken": "YOUR_REFRESH_TOKEN" }'
  ```

  ```javascript JavaScript theme={null}
  const response = await fetch('https://1click.chaindefuser.com/v0/auth/refresh', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ refreshToken: 'YOUR_REFRESH_TOKEN' })
  });

  const auth = await response.json();
  ```
</CodeGroup>

## Example response

```json theme={null}
{
  "accessToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expiresIn": 3600
}
```

<Info>
  The response only contains a new `accessToken`, `refreshToken` itself does not rotate. Keep using the same `refreshToken` until it expires (`refreshExpiresIn` from the original authenticate call).
</Info>


## OpenAPI

````yaml post /v0/auth/refresh
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
  /v0/auth/refresh:
    post:
      tags:
        - User Auth
      summary: Refresh access token
      description: Exchange refresh token for a new access token
      operationId: refresh
      parameters: []
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/RefreshRequestDto'
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/RefreshResponseDto'
        '401':
          description: Invalid or expired refresh token
components:
  schemas:
    RefreshRequestDto:
      type: object
      properties:
        refreshToken:
          type: string
          description: Refresh token obtained from authenticate endpoint
      required:
        - refreshToken
    RefreshResponseDto:
      type: object
      properties:
        accessToken:
          type: string
          description: New JWT access token
        expiresIn:
          type: number
          description: Access token expiration time in seconds
      required:
        - accessToken
        - expiresIn

````