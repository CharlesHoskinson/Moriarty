# Policy Engine & API Keys — Implementation Guide

> Detailed implementation guide for OWS policy enforcement and API key-based agent access.
> This document complements the numbered specs; the canonical behavior should also be reflected in `03-policy-engine.md` and `05-key-isolation.md`.

This guide is non-normative. If it conflicts with `00-specification.md`, `03-policy-engine.md`, or `08-conformance-and-security.md`, the normative documents win.

## Table of Contents

1. [Core Design](#core-design)
2. [How It Works](#how-it-works)
3. [Cryptographic Design](#cryptographic-design)
4. [Policy Evaluation](#policy-evaluation)
5. [Declarative Rules](#declarative-rules)
6. [Custom Executable Policies](#custom-executable-policies)
7. [Storage Formats](#storage-formats)
8. [Signing Flow](#signing-flow)
9. [Module Boundaries](#module-boundaries)
10. [CLI Commands](#cli-commands)
11. [Bindings](#bindings)
12. [Audit Logging](#audit-logging)

---

## Core Design

**The credential determines the access tier.** No flags, no bypass modes — just two kinds of credentials:

```
sign_transaction(wallet, chain, tx, credential)
                                       │
                          ┌────────────┴────────────┐
                          │                          │
                     passphrase                 ows_key_abc...
                          │                          │
                     owner mode                 agent mode
                     no policy                  policies enforced
                     scrypt decrypt             HKDF decrypt
```

- **Owner** provides the wallet passphrase. Full access. No policy evaluation.
- **Agent** provides an API token (`ows_key_...`). Policies attached to that token are evaluated before any key material is touched.

Different agents get different tokens with different policies. The identity is the credential.

---

## How It Works

### Owner creates an API key

```bash
ows key create --name "claude-agent" \
  --wallet agent-treasury \
  --policy spending-limit \
  --policy base-only
```

1. Owner enters wallet passphrase
2. OWS decrypts the wallet mnemonic
3. Generates a random token `ows_key_<base62>`
4. Re-encrypts the mnemonic under HKDF(token)
5. Stores key file with token hash, policy IDs, and encrypted mnemonic copy
6. Displays token once — owner provisions it to the agent

### Agent signs a transaction

```typescript
import { signTransaction } from "@open-wallet-standard/core";

const result = signTransaction(
  "agent-treasury", "base", "0x02f8...", "ows_key_a1b2c3d4..."
);
```

1. OWS detects `ows_key_` prefix → agent mode
2. SHA256(token) → looks up key file
3. Verifies wallet is in scope, checks expiry
4. Loads policies attached to this key
5. Evaluates all policies against the transaction
6. If denied → returns `PolicyDenied` error (key material never touched)
7. If allowed → HKDF(token) → decrypts mnemonic → signs → wipes → returns signature

### Owner signs (unchanged)

```bash
ows sign tx --wallet agent-treasury --chain base --tx 0x02f8...
```

Passphrase authentication. No policy evaluation. Existing behavior preserved exactly.

### Revoking access

```bash
ows key revoke --id 7a2f1b3c --confirm
```

Deletes the key file. The encrypted mnemonic copy is gone. The token becomes useless. Other API keys and the owner's passphrase are unaffected.

---

## Cryptographic Design

### Token-as-capability

When the owner creates an API key, OWS re-encrypts the mnemonic under a key derived from the API token. The token is both the authentication credential and the decryption capability.

### Key derivation

```
token = ows_key_<random 256 bits, base62-encoded>
salt  = random 32 bytes
prk   = HKDF-Extract(salt, token)
key   = HKDF-Expand(prk, "ows-api-key-v1", 32)  →  AES-256-GCM key
```

Reuses the existing `CryptoEnvelope` struct with a new KDF identifier:

```json
{
  "cipher": "aes-256-gcm",
  "cipherparams": { "iv": "..." },
  "ciphertext": "...",
  "auth_tag": "...",
  "kdf": "hkdf-sha256",
  "kdfparams": { "dklen": 32, "salt": "...", "info": "ows-api-key-v1" }
}
```

### Security properties

| Threat | Mitigation |
|---|---|
| Token stolen, no disk access | Useless — encrypted key file not accessible |
| Disk access, no token | Can't decrypt — HKDF + AES-256-GCM |
| Token + disk access | Can decrypt, but requires bypassing OWS entirely |
| Owner passphrase changed | API keys unaffected (independently encrypted) |
| API key revoked | Encrypted copy deleted — token decrypts nothing |
| Multiple API keys | Independent encrypted copies; revoking one doesn't affect others |

---

## Policy Evaluation

### Flow

```
1. Detect credential type (passphrase vs ows_key_ token)
2. If passphrase → skip policy, decrypt with scrypt, sign (owner mode)
3. If token:
   a. SHA256(token) → look up key file
   b. Check expires_at
   c. Check wallet is in key's wallet_ids
   d. Load policies from key's policy_ids
   e. For each policy:
      - Evaluate declarative rules (in-process, fast)
      - If rules pass and executable is set, run executable (subprocess)
      - If policy has both, both must pass
   f. AND semantics — all policies must allow, short-circuit on first deny
   g. deny action → block request. warn action → log, allow.
4. If denied → return PolicyDenied error (never decrypt)
5. If allowed → HKDF decrypt → sign → record spend → audit log
```

### PolicyContext

JSON object available to both declarative evaluation and custom executables:

```json
{
  "operation": "sign_transaction",
  "transaction": {
    "to": "0x742d35Cc6634C0532925a3b844Bc9e7595f2bD0C",
    "value": "100000000000000000",
    "data": "0x",
    "raw_hex": "02f8..."
  },
  "chain_id": "eip155:8453",
  "wallet": {
    "id": "3198bc9c-...",
    "name": "agent-treasury",
    "accounts": [
      {
        "account_id": "eip155:8453:0xab16a96D...",
        "address": "0xab16a96D...",
        "chain_id": "eip155:8453"
      }
    ]
  },
  "timestamp": "2026-03-22T10:35:22Z",
  "key_id": "7a2f1b3c-...",
  "key_name": "claude-agent",
  "spending": {
    "daily_total_wei": "50000000000000000",
    "daily_remaining_wei": "950000000000000000"
  },
  "policy_config": {}
}
```

- `transaction` is chain-specific. EVM gets parsed `to`, `value`, `data`. Other chains get `raw_hex` only.
- `spending` is populated from state store. Custom executables can use it without managing state.
- `policy_config` contains the static `config` object from the policy file.

### PolicyResult

```json
{ "allow": true }
```

```json
{ "allow": false, "reason": "Daily spending limit exceeded: 0.95 / 1.0 ETH" }
```

---

## Declarative Rules

Two built-in rule types, evaluated in-process (microseconds). Value limits, recipient allowlists, and cumulative spend are not declarative — use **`executable`** policies for those.

### `allowed_chains`

```json
{ "type": "allowed_chains", "chain_ids": ["eip155:8453", "eip155:84532"] }
```

Denies if `chain_id` is not in the list.

### `expires_at`

```json
{ "type": "expires_at", "timestamp": "2026-04-01T00:00:00Z" }
```

Denies if current time is past the timestamp.

---

## Custom Executable Policies

For anything declarative rules can't express.

```bash
echo '<PolicyContext JSON>' | /path/to/policy-executable
```

- Receives PolicyContext on stdin, writes PolicyResult to stdout
- Non-zero exit → deny
- Invalid JSON on stdout → deny
- No exit within 5 seconds → kill + deny
- Stderr captured and logged

If a policy has both `rules` and `executable`, declarative rules evaluate first as a fast pre-filter. The executable only runs if rules pass.

### Example: transaction simulation

```python
#!/usr/bin/env python3
import json, sys, urllib.request

ctx = json.load(sys.stdin)
tx = ctx["transaction"]
rpc = {"eip155:8453": "https://mainnet.base.org"}.get(ctx["chain_id"])
if not rpc:
    json.dump({"allow": False, "reason": f"No RPC for {ctx['chain_id']}"}, sys.stdout)
    sys.exit(0)

payload = json.dumps({
    "jsonrpc": "2.0", "id": 1, "method": "eth_call",
    "params": [{"to": tx["to"], "value": hex(int(tx["value"])), "data": tx["data"]}, "latest"]
}).encode()
try:
    resp = json.load(urllib.request.urlopen(
        urllib.request.Request(rpc, data=payload, headers={"Content-Type": "application/json"}), timeout=4))
    if "error" in resp:
        json.dump({"allow": False, "reason": f"Reverted: {resp['error']['message']}"}, sys.stdout)
    else:
        json.dump({"allow": True}, sys.stdout)
except Exception as e:
    json.dump({"allow": False, "reason": str(e)}, sys.stdout)
```

---

## Storage Formats

### Policy file (`~/.ows/policies/<id>.json`)

```json
{
  "id": "base-agent-limits",
  "name": "Base Agent Safety Limits",
  "version": 1,
  "created_at": "2026-03-22T10:00:00Z",
  "rules": [
    { "type": "allowed_chains", "chain_ids": ["eip155:8453", "eip155:84532"] },
    { "type": "expires_at", "timestamp": "2026-12-31T23:59:59Z" }
  ],
  "executable": null,
  "config": null,
  "action": "deny"
}
```

Permissions: `755` (policies are not secret).

### API key file (`~/.ows/keys/<id>.json`)

```json
{
  "id": "7a2f1b3c-4d5e-6f7a-8b9c-0d1e2f3a4b5c",
  "name": "claude-agent",
  "token_hash": "e3b0c44298fc1c149afbf4c8996fb924...",
  "created_at": "2026-03-22T10:30:00Z",
  "wallet_ids": ["3198bc9c-6672-5ab3-d995-4942343ae5b6"],
  "policy_ids": ["base-agent-limits"],
  "expires_at": null,
  "wallet_secrets": {
    "3198bc9c-...": {
      "cipher": "aes-256-gcm",
      "cipherparams": { "iv": "..." },
      "ciphertext": "...",
      "auth_tag": "...",
      "kdf": "hkdf-sha256",
      "kdfparams": { "dklen": 32, "salt": "...", "info": "ows-api-key-v1" }
    }
  }
}
```

Permissions: `700` (directory), `600` (files).

### No wallet file changes

Wallets don't gain a `policy_ids` field. Policies attach to API keys only.

---

## Signing Flow

The credential parameter determines the path:

```
sign_transaction(wallet, chain, tx, credential, ...)
  → if credential starts with "ows_key_":
      │  agent mode
      │  SHA256(credential) → look up key file
      │  check expires_at, check wallet in scope
      │  load key.policy_ids → load policy files
      │  build PolicyContext(tx, chain, wallet, spending state)
      │  evaluate_policies() → if denied: return error
      │  HKDF(credential) → decrypt mnemonic from key.wallet_secrets
      │  HD derive → sign → record spend → audit → return
      │
  → else:
      │  owner mode (existing behavior, unchanged)
      │  scrypt(credential) → decrypt mnemonic from wallet file
      │  HD derive → sign → return
```

### Implementation: single credential parameter

The bindings accept either a passphrase or an API token in the same parameter, branching on the `ows_key_` prefix:

```rust
// ows-lib/src/ops.rs
pub fn sign_transaction(
    wallet: &str,
    chain: &str,
    tx_hex: &str,
    credential: &str,       // passphrase OR ows_key_... token
    index: Option<u32>,
    vault_path: Option<&Path>,
) -> Result<SignResult, OwsLibError> {
    if credential.starts_with("ows_key_") {
        sign_with_api_key(credential, wallet, chain, tx_hex, index, vault_path)
    } else {
        sign_with_passphrase(wallet, chain, tx_hex, credential, index, vault_path)
    }
}
```

---

## Module Boundaries

### Crate-level layout

```
ows-core/src/
  policy.rs          — Policy, PolicyRule, PolicyAction, PolicyContext, PolicyResult types
  api_key.rs         — ApiKeyFile type
  error.rs           — PolicyDenied, ApiKeyNotFound, ApiKeyExpired variants

ows-signer/src/
  crypto.rs          — encrypt_with_hkdf(), decrypt_with_hkdf(), dispatch on kdf field

ows-lib/src/
  policy_store.rs    — CRUD for ~/.ows/policies/
  policy_engine.rs   — evaluate_policies(), declarative rule evaluation, executable subprocess
  key_store.rs       — CRUD for ~/.ows/keys/, token generation, SHA-256 hashing
  key_ops.rs         — create_api_key(), sign_with_api_key()
  ops.rs             — credential branch (ows_key_ prefix check) at top of signing functions

ows-cli/src/
  commands/policy.rs — ows policy create/list/show/delete
  commands/key.rs    — ows key create/list/revoke
  audit.rs           — policy_id, api_key_id fields added to AuditEntry
```

### Dependency flow between modules

```
key_ops.rs
  ├── key_store.rs       (token lookup, key file I/O)
  ├── policy_engine.rs   (evaluate policies)
  │     └── policy_store.rs   (load policy files)
  └── crypto.rs          (decrypt_with_hkdf — existing module, new function)
```

Each module has a narrow interface:

- **`policy_store`**: `save_policy()`, `load_policy()`, `list_policies()`, `delete_policy()`
- **`policy_engine`**: `evaluate_policies(policies, context) → PolicyResult`
- **`key_store`**: `generate_token()`, `hash_token()`, `save_api_key()`, `load_api_key_by_token_hash()`, `list_api_keys()`, `delete_api_key()`
- **`key_ops`**: `create_api_key()`, `sign_with_api_key()`

---

## CLI Commands

### Policy management

```bash
ows policy create --file <path>           # register a policy
ows policy list                            # list all policies
ows policy show --id <id>                  # show policy details
ows policy delete --id <id> --confirm      # delete a policy
```

### Key management

```bash
ows key create --name "claude-agent" \
  --wallet agent-treasury \
  --policy spending-limit \
  --policy base-only
# Prompts for wallet passphrase
# Outputs: ows_key_a1b2c3d4... (shown once)

ows key list                               # list keys (no tokens shown)
ows key revoke --id <id> --confirm         # delete key file
```

### Signing (unchanged surface)

```bash
# Owner (existing)
ows sign tx --wallet treasury --chain base --tx 0x...

# Agent (pass token via env)
OWS_PASSPHRASE=ows_key_abc... ows sign tx --wallet treasury --chain base --tx 0x...
```

The CLI's existing passphrase input path (env var, stdin, prompt) works for tokens too — no new flags needed.

---

## Bindings

### Signing functions (no new signatures needed)

The existing functions accept the token in the passphrase parameter:

```typescript
// Node.js — existing API, works with tokens
signTransaction("treasury", "base", "0x...", "ows_key_abc...");
signMessage("treasury", "base", "hello", "ows_key_abc...");
signAndSend("treasury", "base", "0x...", "ows_key_abc...");
```

### Management functions

```typescript
// Policy management
createPolicy(jsonStr: string, vaultPath?: string): PolicyInfo
listPolicies(vaultPath?: string): PolicyInfo[]
deletePolicy(id: string, vaultPath?: string): void

// Key management
createApiKey(name: string, wallet: string, passphrase: string,
             policies: string[], vaultPath?: string): { id: string, token: string }
listApiKeys(vaultPath?: string): ApiKeyInfo[]
revokeApiKey(id: string, vaultPath?: string): void
```

Python bindings mirror the same functions.

---

## Audit Logging

Extend `AuditEntry` with policy fields:

```json
{
  "timestamp": "2026-03-22T10:35:22Z",
  "wallet_id": "3198bc9c-...",
  "operation": "policy_denied",
  "chain_id": "eip155:8453",
  "details": "Daily spending limit exceeded: 0.95 / 1.0 ETH",
  "api_key_id": "7a2f1b3c-...",
  "policy_id": "spending-limit"
}
```

Operations: `policy_evaluated`, `policy_denied`, `policy_timeout`.
