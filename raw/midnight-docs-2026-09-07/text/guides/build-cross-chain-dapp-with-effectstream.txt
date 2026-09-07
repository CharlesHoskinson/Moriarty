> For the complete documentation index, see [llms.txt](/llms.txt)

# Build a cross-chain DApp with EffectStream

Use this guide to build a DApp whose state lives on Midnight and an EVM chain at once. You run a working template, learn how it joins the two chains, then add a field of your own end to end.

[Index contract state with EffectStream](/guides/index-state-with-effectstream.md) covers reading one Midnight contract. This guide covers the case EffectStream exists for: correlating two chains, and writing to them.

## Prerequisites[​](#prerequisites "Direct link to Prerequisites")

These apply to every procedure in this guide:

* [Bun](https://bun.sh). See [Set up Bun for Midnight development](/guides/install-bun-runtime-midnight).
* [Foundry](https://www.getfoundry.sh/). `forge` compiles the Solidity artifacts, and the orchestrator checks for it on PATH before starting.
* The Compact compiler, which the orchestrator also checks on PATH. The template pins `0.31.0`, so run `compact update 0.31.0`.
* Roughly 8 GB of free memory, and a raised Node heap for the frontend build. Vite transforms about 9000 modules and exceeds Node's default limit, so export `NODE_OPTIONS=--max-old-space-size=8192` before starting.

The template launches its own Midnight stack, so you need no separately running node or proof server.

<!-- -->

## How a rollup joins two chains[​](#how-a-rollup-joins-two-chains "Direct link to How a rollup joins two chains")

Splitting a DApp across chains usually means a bridge: a message-passing contract, a relayer, and a light client that lets one chain verify the other. That machinery exists to make execution atomic across chains.

EffectStream targets the weaker requirement. When you need a consistent view of two chains rather than atomic execution across them, neither chain has to know the other exists. Both are ingested independently, and a state machine you write merges them into one database.

The template splits an NFT along that line. An ERC-721 contract on the EVM chain owns transfers, because everyone needs to agree on who holds a token. A Compact circuit on Midnight takes private inputs and discloses one resulting property, because the values behind it stay private. The two sides share a key, `(contract_address, token_id)`, which the user supplies to both. The rollup joins on that key.

One consequence shapes the code you write. EffectStream reads Midnight's public ledger, so what a `disclose()` call publishes is what your state machine can see. Designing the circuit designs the sync surface.

A second one shows up in the logs. Each chain syncs independently, so a Midnight property can arrive before the EVM transfer that created the token. The `midnightContractState` transition handles this by inserting a placeholder row, and you will see it skip events while the two chains catch up.

<!-- -->

## Run the template[​](#run-the-template "Direct link to Run the template")

Start the whole stack and mint a token.

### Procedure[​](#procedure "Direct link to Procedure")

1. Clone the repository and enter the template:

   ```
   git clone https://github.com/effectstream/effectstream.git

   cd effectstream/templates/evm-midnight-v2
   ```

2. Install dependencies:

   ```
   bun install
   ```

3. Start the stack. This compiles the Compact circuit, compiles and deploys the Solidity contracts, deploys the Midnight contract, then starts the database, sync node, batcher, and frontend:

   ```
   bun run dev
   ```

4. Open the DApp at `http://localhost:10599`, mint a token, and set a property on it.

### Verification[​](#verification "Direct link to Verification")

The sync node runs independently of the frontend, so check it directly. The merged view returns each token with its EVM owner and its Midnight properties:

```
curl http://localhost:9999/api/erc721
```

The sync node's own logs show both chains advancing together, which is the rollup working:

```
INFO effectstream-sync-block-merge: finalized block 145 @ 0xdf3de2... | {"mainNtp":[145,145],"mainEvmRPC":[794,797]}

[Midnight:undeployed] Fetching blocks from 32 to 32.
```

<!-- -->

## Local service endpoints[​](#local-service-endpoints "Direct link to Local service endpoints")

Every service the template starts.

| Service                | URL                                                    |
| ---------------------- | ------------------------------------------------------ |
| Frontend               | `http://localhost:10599`                               |
| Sync node API          | `http://localhost:9999`                                |
| Sync node OpenAPI docs | `http://localhost:9999/documentation`                  |
| Batcher                | `http://localhost:3334`                                |
| Orchestrator API       | `http://localhost:4747`                                |
| EVM chain (main)       | `http://localhost:8545`                                |
| EVM chain (parallel)   | `http://localhost:8546`                                |
| Midnight node RPC      | `http://localhost:9944`                                |
| Midnight indexer       | `http://localhost:8088/api/v3/graphql`                 |
| Midnight proof server  | `http://localhost:6300`                                |
| Database               | `postgres://postgres:postgres@localhost:5432/postgres` |

<!-- -->

## The ingestion pipeline[​](#the-ingestion-pipeline "Direct link to The ingestion pipeline")

Four files carry a chain event from the wire into your API. Read them in order to understand the template.

`packages/node/config.dev.ts` declares networks, sync protocols, and primitives. One primitive per chain, each naming a `stateMachinePrefix`:

```
.addPrimitive(

  (syncProtocols) => syncProtocols.parallelMidnight,

  (network, deployments, syncProtocol) => ({

    name: "MidnightContractState",

    type: PrimitiveTypeMidnightGeneric,

    startBlockHeight: 1,

    contractAddress: readMidnightContract("contract-round-value", {

      networkId: midnightNetworkConfig.id,

    }).contractAddress,

    stateMachinePrefix: "midnightContractState",

    contract: { ledger: CounterContract.ledger },

    networkId: midnightNetworkConfig.id,

  }),

)
```

`contract: { ledger: CounterContract.ledger }` hands the primitive the reader that `compact compile` generates. That reader decodes anything Compact can express, which is the difference from the declarative schema in [Index contract state with EffectStream](/guides/index-state-with-effectstream.md#what-the-ledger-schema-can-read).

`packages/node/grammar.ts` maps each prefix to a parser. Both prefixes use builtin grammars, so the template writes none of its own.

`packages/node/state-machine.ts` holds one state transition function per prefix. Each receives the parsed payload and writes to the database.

`packages/node/api.ts` serves the merged result over HTTP.

<!-- -->

## Add a field end to end[​](#add-a-field-end-to-end "Direct link to Add a field end to end")

Carry one new value from the Compact circuit to the API. Nothing generates this path for you, so a single field touches the circuit, the database schema, the queries, the state transition, and the route. Knowing that cost up front is part of choosing this pattern.

### Procedure[​](#procedure-1 "Direct link to Procedure")

1. Open `packages/contracts-midnight/contract-round-value/src/counter.compact`. Add a ledger field, take a matching argument in `increment`, and assign it through `disclose()`. The additions are marked:

   ```
   pragma language_version >= 0.17;



   import CompactStandardLibrary;



   export ledger round: Counter;

   export ledger contract_address: Bytes<64>;

   export ledger token_id: Bytes<64>;

   export ledger property_name: Bytes<32>;

   export ledger value: Bytes<32>;

   export ledger rarity: Bytes<32>;          // added



   export circuit increment(

     contract_address_: Bytes<64>,

     token_id_: Bytes<64>,

     property_name_: Bytes<32>,

     value_: Bytes<32>,

     rarity_: Bytes<32>,                     // added

   ): [] {

     round.increment(1);

     contract_address = disclose(contract_address_);

     token_id = disclose(token_id_);

     property_name = disclose(property_name_);

     value = disclose(value_);

     rarity = disclose(rarity_);             // added

   }
   ```

   Adding an argument changes the circuit's signature, so every caller needs the new value. The frontend calls this circuit in `packages/frontend/client/src/increment.ts`, and the batcher calls it in `packages/batcher/midnight-balancing.ts`.

2. Recompile the circuit so the generated ledger reader includes the new field:

   ```
   bun run build:midnight
   ```

3. Add a column to the migration in `packages/database/migrations/`.

4. Add the column to the matching query in `packages/database/sql/sm_example.sql`, then regenerate the typed queries:

   ```
   bun run build:pgtypes
   ```

5. Decode the field in the `midnightContractState` transition in `packages/node/state-machine.ts`. A `Bytes<32>` ledger field arrives as fixed-width bytes, so `decodeField` turns it back into a string. Add a line next to the existing ones:

   ```
   const contract_address = decodeField(payload.contract_address);

   const token_id = decodeField(payload.token_id);

   const property_name = decodeField(payload.property_name);

   const value = decodeField(payload.value);

   const rarity = decodeField(payload.rarity);        // added
   ```

   Then pass `rarity` into the `insertEvmMidnightProperty` call further down the same transition, alongside the fields already written there.

6. Return the column from `GET /api/erc721` in `packages/node/api.ts`.

7. Restart the stack and set a property from the frontend.

### Verification[​](#verification-1 "Direct link to Verification")

The endpoint returns the new field alongside the existing ones.

```
curl http://localhost:9999/api/erc721
```

## Additional resources[​](#additional-resources "Direct link to Additional resources")

* [Index contract state with EffectStream](/guides/index-state-with-effectstream.md): the read path, one Midnight contract, no local stack.
* [EffectStream documentation](https://effectstream.github.io/docs/): the full API surface, the batcher, and the other cross-chain templates.
* [Security and best practices](/guides/security-best-practices.md#on-chain-visibility): what `disclose()` makes public, which is what a sync node reads.
