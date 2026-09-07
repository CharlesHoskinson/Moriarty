> For the complete documentation index, see [llms.txt](/llms.txt)

# Cryptography

Outside of the Midnight Ledger, which contains its own specialized cryptographic circuits, the Midnight node relies on several foundational cryptographic algorithms for securing core functions such as consensus, state transition integrity, and network communication.

The primary hash function used is *Blake2 256*, which provides strong cryptographic guarantees and is employed for general-purpose hashing — including block hashes and within various state transition functions. It strikes a balance between performance and security, making it suitable for runtime-critical operations.

For signature verification, Midnight uses three distinct schemes depending on the role:

* **sr25519** is based on the Schnorrkel and Ristretto x25519 construction and is used to sign block authorship messages in AURA. It supports efficient key derivation, signature aggregation, and strong security guarantees.

* **ECDSA** is used to sign Partnerchain-related consensus messages. This ensures interoperability with external systems where ECDSA is the standard.

* **Ed25519** is used in two parts of the Midnight node:

  <!-- -->

  * to sign finality messages in GRANDPA, enabling fast and secure verification of validator communication during finalization.
  * to sign messages exchanged through the underlying libp2p protocols in the Polkadot SDK.

The Midnight node also uses **twoxhash** as a non-cryptographic hash function for generating storage keys. Although it is not suitable for cryptographic use, twoxhash is optimized for speed and low collision rates, which makes it a good fit for internal key-value storage structures. See [Storage](/concepts/network-architecture/storage.md) for more details.
