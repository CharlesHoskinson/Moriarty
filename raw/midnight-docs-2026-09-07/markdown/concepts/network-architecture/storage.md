> For the complete documentation index, see [llms.txt](/llms.txt)

# Storage

Midnight is built on the Polkadot SDK (formerly Substrate) and uses ParityDB as its default database backend.

* **ParityDB**: A fast key-value store designed for blockchain use cases that stores all on-chain state.
* **Merkle-Patricia trie**: The underlying data structure for state commitments. It preserves state integrity and supports efficient inclusion proofs for on-chain data (for example, contract state or account balances).
* **twoxhash**: A non-cryptographic hash function that generates storage keys within the trie. It is optimized for speed and low collision rates, which makes it well suited for internal key-value lookups. While not suitable for cryptographic guarantees, it significantly improves trie performance.

All state transitions are committed into this trie and persisted via ParityDB.
