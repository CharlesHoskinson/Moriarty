> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Changelog

> What's new in NEAR Intents.

## August 2026

<Update label="Aug 28, 2026" tags={["Launch", "Feature", "API"]}>
  ### 1Click API: Limit Orders

  Wallets and apps can rest a confidential swap at a price the user sets, through
  the 1Click Swap API. This is not Perpetuals on near.com.

  <Card title="Limit Orders" icon="clock" href="/integration/distribution-channels/1click-api/orders" horizontal>
    Offer a swap at the user's price through the 1Click Swap API.
  </Card>
</Update>

## July 2026

<Update label="Jul 13, 2026" tags={["Launch", "Feature", "Docs"]}>
  ### Confidential Intents B2B launch

  Confidential Intents is live for B2B partners. Confidential swaps execute on a
  private chain with shielded balances and settle back to public NEAR through
  the Private PoA Bridge.

  <Card title="Confidential Intents" icon="lock" href="/integration/market-makers/confidential-intents" horizontal>
    Two-chain architecture, shield and unshield flows, versioned nonces, and the solver response format.
  </Card>

  ### Changelog on the docs website

  This changelog is now part of the docs site. New features, API changes, and
  launches land here as dated entries.

  <Card title="Changelog" icon="sparkles" href="/changelog/overview" horizontal>
    Bookmark this page to track what's new in NEAR Intents.
  </Card>

  ### WebSocket-only whitelisted solvers on Confidential Intents

  Solver access on the confidential relay is now gated by an allowlist.
  Whitelisted solvers connect over WebSocket only, subscribing to relay streams
  and responding with signed quotes; solvers not on the list are denied entirely.

  <Card title="Private Relay" icon="plug" href="/integration/market-makers/confidential-intents#private-relay" horizontal>
    Whitelisted solvers receive quote requests and respond via `quote_response` over `/ws`. Contact the Defuse team to get whitelisted.
  </Card>

  ### FE (near.com): Limit Orders on Perpetuals

  Perpetuals on near.com now support limit orders. Set your price and the order
  fills when the market reaches it.

  <Card title="Trade Perps on NEAR" icon="chart-line" href="https://near.com/perps" horizontal>
    Place limit orders on the Perpetuals interface at near.com.
  </Card>
</Update>

<Update label="Jul 3, 2026" tags={["Feature", "API"]}>
  ### Shield Incident API

  A partner-facing API to pull active Shield incidents and submit new ones. Use
  `GET`/`POST https://shield.chaindefuser.com/incident` with a `SHIELD` API key.

  <Card title="Shield Incident API guide" icon="paper-plane" href="/security-compliance/shield-incident-api" horizontal>
    Endpoints, request and response shapes, scope fields, and how to get a token.
  </Card>
</Update>

<Update label="Jul 1, 2026" tags={["Feature", "API"]}>
  ### Confidentiality parameter

  Quote requests now accept a `confidentiality` field that makes a normal quote a
  confidential swap.

  <Card title="Confidentiality parameter" icon="user-secret" href="/changelog/entries/2026-07-02-confidentiality-parameter" horizontal>
    What changed, the two integration paths, and an example request.
  </Card>
</Update>

## June 2026

<Update label="Jun 27, 2026" tags={["Feature", "API"]}>
  ### Verify Quote Signatures

  1Click signs quote payloads so partners can verify the payload origin and detect
  tampering before using fields like `depositAddress` in 1click quote response
  payloads.

  <Card title="Verify Quote Signatures" icon="shield-check" href="/integration/distribution-channels/1click-api/verify-quote-signature" horizontal>
    What is signed, how to verify, and code examples.
  </Card>
</Update>
