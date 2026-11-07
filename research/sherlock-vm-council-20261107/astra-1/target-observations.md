# Direct target observations

Read-only JSON-RPC observations on 2026-11-07 at approximately 20:57Z. Raw responses are retained in `preview-rpc.json` and `stagenet-rpc.json`. Each request used `midnight_ledgerVersion`, `state_getRuntimeVersion` and `chain_getBlockHash(0)`.

| Endpoint | Ledger response | Runtime spec | Genesis |
| --- | --- | --- | --- |
| `https://rpc.preview.midnight.network` | `1.0.1 (tag: crate-ledger-9.1.0.1, rev: 3e69c5e3092fa29c1e8b1728a68fc3ca98774e34)` | 2001100, transaction version 4 | `0x3c096de209e06a1a8c52be7bda109ee8c891a42288c0a9db5495f9616dd13796` |
| `https://rpc.stagenet.shielded.tools` | `=1.0.0` | 2000000, transaction version 4 | `0x2f76825abc239fecf6107c9df99016de57037b451ae57a4394b76c8cf53a9491` |

Both calls exited 0 and returned no JSON-RPC error. These are observations from the named servers. They are not authenticated chain-state proofs, settlement evidence or proof that local binaries match deployed binaries.

The scout reports that the ledger revision returned by Preview is not publicly fetchable, and that the public node branch differs from the deployed runtime. I inspected that report but did not independently exhaust public Git refs. Treat the exact source-to-runtime reproduction as unresolved until a verified source/build manifest closes it. Version strings alone are insufficient. [wiki/midnight-ledger-9-hardfork.md:161-170]

Proposed requirement: a local compatibility result identifies the tested binary/source closure and the live target observation separately. If their equivalence is unestablished, the result states that limitation and cannot claim an exact production reproduction. Target acceptance remains an independently observed obligation.
