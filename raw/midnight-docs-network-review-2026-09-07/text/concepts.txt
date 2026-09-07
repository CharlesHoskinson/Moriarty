> For the complete documentation index, see [llms.txt](/llms.txt)

# Concepts

Understand the core ideas behind Midnight, including confidentiality with zero-knowledge proofs, contracts, and verifiable computation.

**Core concepts cover these terms**

| Concept                       | Explanation                                                                                                                                                              |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Who interacts**             | **Accounts** define who participates on Midnight. They manage keys, addresses, and authorization. This shows *who acts in the system*.                                   |
| **Where data lives**          | **Ledgers** store state. Midnight keeps a public ledger for visible data and a private ledger for shielded data. This shows *where state is stored and who can see it*.  |
| **How value moves**           | **The UTXO model** defines spendable pieces of value or state. Midnight extends this model with private state elements. This shows *how value flows through the system*. |
| **How apps connect**          | **Web3** explains how wallets, connectors, and dApps communicate with contracts. This shows *how users and applications interact with Midnight*.                         |
| **How privacy is enforced**   | **Zero-knowledge proofs** verify correctness without exposing sensitive data. This shows *how private actions stay verifiable*.                                          |
| **How computation is proved** | **Kachina** is Midnight’s proving system. It converts private computation into verifiable proofs. This shows *how we trust execution we cannot see*.                     |
| **How it all comes together** | **ZSwap** applies these concepts in practice. It uses private state, proofs, and confidential execution. This shows *how a real private dApp works end-to-end*.          |

### [Accounts](/concepts/account.md)

Learn how Midnight accounts, addresses, and keys relate to each other.

[Read →](/concepts/account.md)

### [Ledgers](/concepts/ledgers.md)

See how public and private ledgers track state and interact securely.

[Read →](/concepts/ledgers.md)

### [UTXO model](/concepts/utxo.md)

Review how UTXOs represent value and how Midnight extends the model.

[Read →](/concepts/utxo.md)

### [Web3](/concepts/web3.md)

Understand how Midnight fits into wallets, dApps, and existing tooling.

[Read →](/concepts/web3.md)

### [Zero-knowledge proofs](/concepts/zero-knowledge-proofs.md)

Understand how ZK proofs protect data while proving correctness.

[Read →](/concepts/zero-knowledge-proofs.md)

### [Kachina](/concepts/kachina.md)

Learn about the proving system that powers Midnight’s confidential computation.

[Read →](/concepts/kachina.md)

### [ZSwap](/concepts/zswap.md)

See how confidential swaps preserve privacy while enabling efficient exchange.

[Read →](/concepts/zswap.md)

***

Midnight uses zero-knowledge proofs to keep sensitive data private while still verifying contract logic. Its smart contracts operate across public and private ledgers, reducing transaction correlation and supporting secure, confidential on-chain atomic swaps of tokens and metadata.

### [Learn the building blocks](/concepts/how-midnight-works/building-blocks.md)

Review circuits, ledgers, and assignments that form Midnight’s foundation.

[Explore →](/concepts/how-midnight-works/building-blocks.md)

### [Explore Compact contracts](/concepts/how-midnight-works/smart-contracts.md)

See how Compact defines confidential logic and interacts with verified data.

[Open →](/concepts/how-midnight-works/smart-contracts.md)

### [Keep data private](/concepts/how-midnight-works/keeping-data-private.md)

Learn the patterns that restrict data visibility to authorized participants.

[Learn →](/concepts/how-midnight-works/keeping-data-private.md)

### [Understand Compact semantics](/concepts/how-midnight-works/semantics.md)

Trace how the runtime enforces rules over public and private data.

[Read →](/concepts/how-midnight-works/semantics.md)

### [Examine ZSwap](/concepts/how-midnight-works/zswap.md)

See how confidential swaps maintain privacy and efficiency.

[See →](/concepts/how-midnight-works/zswap.md)

### [Assess the impact](/concepts/how-midnight-works/impact.md)

Consider how privacy-preserving computation changes decentralized systems.

[Assess →](/concepts/how-midnight-works/impact.md)
