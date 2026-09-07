> For the complete documentation index, see [llms.txt](/llms.txt)

# UTXO model

The UTXO model is a different way to reason about digital value. If the account model feels familiar because it mirrors traditional banking, UTXO can feel unfamiliar at first. Once you understand it, the design trade-offs become clear and explain why Midnight uses it as a foundation for privacy and parallelism.

UTXO stands for "Unspent Transaction Output." Think of each UTXO as a discrete digital coin, similar to physical cash in a wallet. Each has a specific value, belongs to a specific owner, and must be spent in its entirety. This constraint enables several of the model's most useful properties.

## The digital cash metaphor[​](#the-digital-cash-metaphor "Direct link to The digital cash metaphor")

The cash metaphor is not only a teaching tool; it closely maps to how UTXOs work:

Imagine you're at a coffee shop and your latte costs $3.50. You reach into your wallet and find you have:

* One $20 bill
* One $5 bill
* Two $1 bills

You can't tear the $5 bill to extract exactly $3.50. Instead, you must hand over the entire $5 bill and receive $1.50 in change. The original $5 bill leaves your possession forever—it's been "consumed" in the transaction—and you receive new bills as change.

The UTXO model implements this exact pattern digitally:

```
// Your wallet contains discrete UTXOs (like bills):

AliceWallet = [

    UTXO_1: { value: 100 NIGHT, id: "abc123..." },

    UTXO_2: { value: 50 NIGHT, id: "def456..." },

    UTXO_3: { value: 25 NIGHT, id: "ghi789..." }

]



// To send 60 NIGHT, you must consume entire UTXOs:

Transaction {

    inputs: [UTXO_1],  // Consume the 100 NIGHT UTXO entirely

    outputs: [

        { value: 60, recipient: Bob },      // Payment

        { value: 40, recipient: Alice }     // Change back to yourself

    ]

}
```

## Core UTXO concepts[​](#core-utxo-concepts "Direct link to Core UTXO concepts")

The following principles explain how the UTXO model works in Midnight.

### UTXOs are indivisible[​](#utxos-are-indivisible "Direct link to UTXOs are indivisible")

This is not only a design choice; it is a core property of the model. Each UTXO is treated as a complete unit:

```
// This is IMPOSSIBLE in UTXO systems:

❌ UTXO_100.spend(30)  // Can't spend part of a UTXO



// You MUST do this instead:

✅ Transaction {

    inputs: [UTXO_100],  // Consume entirely

    outputs: [

        { value: 30, owner: recipient },

        { value: 70, owner: self }  // Change

    ]

}
```

This indivisibility enables atomic transactions, parallel processing, and, in Midnight, support for shielded tokens.

### The nullifier set in Midnight[​](#the-nullifier-set-in-midnight "Direct link to The nullifier set in Midnight")

Midnight differs from Bitcoin in how it tracks spent outputs. Instead of only marking UTXOs as spent, Midnight uses a nullifier set:

```
// When a UTXO is spent in Midnight:

1. UTXO_abc123 is consumed in a transaction

2. A nullifier is computed: nullifier = Hash(UTXO_abc123, ownerSecret)

3. This nullifier is added to the global nullifier set

4. Future transactions check: isNullified(UTXO) before accepting



// The nullifier set grows over time:

NullifierSet = {

    "0xn1a2b3c4...",  // From spent UTXO_1

    "0xn5d6e7f8...",  // From spent UTXO_2

    "0xn9g0h1i2...",  // From spent UTXO_3

    // ... millions more

}
```

This approach is important because:

* **No pruning requirement**: Unlike Bitcoin, Midnight does not rely on pruning spent outputs from historical data.
* **Privacy compatible**: A nullifier can be validated without publicly linking to the spent UTXO.
* **Double-spend protection**: Once a nullifier appears in the set, that spend cannot be accepted again.
* **Efficient verification**: Nodes can check membership efficiently during validation.

### Transaction atomicity through output creation[​](#transaction-atomicity-through-output-creation "Direct link to Transaction atomicity through output creation")

Every UTXO transaction is atomic: all inputs are consumed and all outputs are created together, or nothing happens.

```
// A multi-party payment showcasing atomicity:

Transaction {

    inputs: [

        UTXO_A: 100 NIGHT,  // From Alice

        UTXO_B: 50 NIGHT    // Also from Alice

    ],

    outputs: [

        { value: 40, owner: Bob },

        { value: 30, owner: Carol },

        { value: 20, owner: Dave },

        { value: 60, owner: Alice }  // Change

    ]

}

// Either ALL of this happens, or NONE of it does
```

### The UTXO lifecycle in Midnight[​](#the-utxo-lifecycle-in-midnight "Direct link to The UTXO lifecycle in Midnight")

Understanding the lifecycle helps explain why this model is effective:

```
// 1. BIRTH: UTXO created in a transaction

NewUTXO = {

    value: 100 NIGHT,

    owner: AlicePublicKey,

    id: Hash(transaction, outputIndex),

    commitment: PedersenCommit(value, randomness)  // For privacy

}



// 2. LIFE: UTXO exists in the unspent set

UnspentSet.add(NewUTXO)

// Can be queried, proven, but not modified



// 3. DEATH: UTXO consumed in a transaction

Transaction.consume(NewUTXO)

nullifier = ComputeNullifier(NewUTXO, AlicePrivateKey)

NullifierSet.add(nullifier)



// 4. AFTERLIFE: Nullifier prevents resurrection

if (NullifierSet.contains(nullifier)) {

    reject("UTXO already spent!")

}
```

## Why this system[​](#why-this-system "Direct link to Why this system")

Account balances can look simpler at first glance. In practice, the UTXO model provides advantages that become clear when building for privacy, concurrency, and provability.

### Parallelism as a model property[​](#parallelism-as-a-model-property "Direct link to Parallelism as a model property")

The UTXO model naturally enables parallelism:

```
// Account Model: Forced Sequential Processing

// These transactions MUST process in order:

Tx1: Alice.balance -= 50  // Must complete first

Tx2: Alice.balance -= 30  // Must wait for Tx1

// Why? Both modify the same global state



// UTXO Model: Natural Parallel Processing

// These transactions can process SIMULTANEOUSLY:

Tx1: Consume UTXO_A (50 NIGHT) → Send to Bob

Tx2: Consume UTXO_B (30 NIGHT) → Send to Carol

// Why? They touch completely independent objects
```

This is more than a minor optimization. In high-throughput scenarios, this architectural difference can materially increase throughput.

### Privacy built into the foundation[​](#privacy-built-into-the-foundation "Direct link to Privacy built into the foundation")

The UTXO model naturally supports privacy features that are harder to implement in account-based systems:

```
// Account Model: Everything linked to one address

AliceAccount: {

    balance: 1000,

    history: [every transaction ever]

}

// Privacy requires complex workarounds



// UTXO Model: Natural isolation

UTXO_1: { value: 100, owner: AddressA }  // For receiving salary

UTXO_2: { value: 50, owner: AddressB }   // For online shopping

UTXO_3: { value: 25, owner: AddressC }   // For donations

// Shielded and unshielded token UTXOs can be used as required
```

In Midnight, token UTXOs can be shielded or unshielded, which lets users choose privacy at the transaction level rather than at the account level.

### State management and efficiency[​](#state-management-and-efficiency "Direct link to State management and efficiency")

The UTXO approach to state differs from account models:

```
// What nodes need to track:

ActiveState = {

    // Account Model: Every account that exists

    accounts: Map<Address, Balance>,  // Grows forever

    

    // UTXO Model: Only unspent outputs

    utxos: Set<UTXO>,  // Naturally bounded

    nullifiers: Set<Hash>  // Prevents double-spends

}



// The key insight: In UTXO, spent history can be archived

// In accounts, all history affects current state
```

## Deep comparison[​](#deep-comparison "Direct link to Deep comparison")

This comparison summarizes practical implications:

| Aspect                  | Account model              | UTXO model                  | Why it matters for Midnight         |
| ----------------------- | -------------------------- | --------------------------- | ----------------------------------- |
| Value storage           | Single mutable balance     | Immutable discrete coins    | Enables shielded tokens             |
| Transaction model       | State updates              | State transitions           | Natural audit trail and provability |
| Concurrency             | Lock → Process → Unlock    | No locks needed             | Massive throughput potential        |
| Privacy approach        | Mix entire balance         | Shielded token UTXOs        | Granular privacy control            |
| Double-spend prevention | Balance arithmetic check   | Nullifier set membership    | Works even with hidden values       |
| State growth            | Unbounded (all accounts)   | Bounded (active UTXOs only) | Long-term sustainability            |
| Determinism             | Depends on execution order | Order-independent           | Predictable outcomes                |

## Key insights: the UTXO paradigm shift[​](#key-insights-the-utxo-paradigm-shift "Direct link to Key insights: the UTXO paradigm shift")

The UTXO model is more than a different way to track balances. It changes how value flow is modeled:

* From mutable to immutable: Instead of changing balances in place, the system creates and consumes discrete values. This supports strong auditability and proof systems.
* From sequential to parallel: By avoiding shared balance updates, the model enables natural concurrency across independent UTXOs.
* From monolithic to granular: Each UTXO is independent, enabling fine-grained control over privacy, spending, and ownership.
* From accounts to capabilities: UTXOs can represent value, rights, or other discrete digital assets.
