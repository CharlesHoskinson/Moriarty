# Indexed owner boundary repair

The local-continuation-01 public preflight stopped with `INVALID_INDEXED_OWNER` before launcher or private base creation. Independent containment found all three Docker containers stopped; no new reservation or transaction was created. That attempt remains consumed.

The actual indexer revision encodes unshielded owners as network-specific Bech32m, and the installed public-data SDK preserves that string. Earlier observer fixtures substituted SQLite raw owner bytes. Decode indexed owners with installed wallet-sdk-address-format 3.1.2, pin its package and entry bytes, require the local `undeployed` network, exact canonical re-encoding and 32-byte identity, then compare every native owner, type, value and origin as before. No raw-hex fallback. Native decoding remains unchanged. A future Preview observer must explicitly support the Preview network.

The regression reconstructs the API owner from retained actual SQLite bytes and exact source conversion; it is not a captured HTTP response. Reject corrupted checksums, uppercase/noncanonical encodings, raw hex, wrong network/type, extra HRP suffix, wrong byte lengths and valid but unequal owners. Keep all finality, canonical block, resource, storage and financial gates. A separate successor allocation needs two actual source/resource votes; this repair is not execution or acceptance.

The adjacent actual wallet sync decoder and state application also preserve Bech32m owners. The same strict public codec now serves the minted-wallet predicate; its regression uses the actual installed wallet decoder. Other amount, token, origin and output-number fields retain their existing native checks. This second source finding was reproduced before another live attempt.
