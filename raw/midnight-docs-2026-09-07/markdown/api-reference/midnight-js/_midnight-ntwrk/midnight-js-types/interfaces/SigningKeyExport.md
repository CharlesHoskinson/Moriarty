# SigningKeyExport

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / SigningKeyExport

# Interface: SigningKeyExport

Represents the exported signing key data structure. All metadata is included in the encrypted payload to prevent tampering.

## Properties[​](#properties "Direct link to Properties")

### encryptedPayload[​](#encryptedpayload "Direct link to encryptedPayload")

> `readonly` **encryptedPayload**: `string`

Encrypted payload containing version, metadata, and signing keys. Format: base64-encoded AES-256-GCM encrypted JSON.

***

### format[​](#format "Direct link to format")

> `readonly` **format**: `"midnight-signing-key-export"`

Format identifier. Must be 'midnight-signing-key-export'.

***

### salt[​](#salt "Direct link to salt")

> `readonly` **salt**: `string`

Salt used for key derivation (hex-encoded, 32 bytes / 64 characters). Required for decryption with the export password.
