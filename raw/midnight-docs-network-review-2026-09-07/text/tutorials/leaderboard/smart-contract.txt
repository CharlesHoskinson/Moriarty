> For the complete documentation index, see [llms.txt](/llms.txt)

# Part 1: The smart contract

In this section, you write the Compact smart contract that powers the leaderboard.

## Initialize the project[​](#initialize-the-project "Direct link to Initialize the project")

Create the project directory, then set up the root `package.json` with workspace configuration and Midnight SDK dependencies.

```
mkdir midnight-leaderboard

cd midnight-leaderboard

touch package.json
```

This configures npm workspaces and includes all Midnight SDK dependencies.

```
{

  "name": "midnight-leaderboard",

  "version": "0.1.0",

  "private": true,

  "type": "module",

  "workspaces": ["contract", "api", "leaderboard-ui"],

  "scripts": {

    "compile": "cd contract && npm run compact",

    "build": "cd contract && npm run build && cd ../api && npm run build"

  },

  "devDependencies": {

    "@vitejs/plugin-react": "^5.1.4",

    "typescript": "^5.9.3",

    "vite": "^7.3.1",

    "vite-plugin-top-level-await": "^1.6.0",

    "vite-plugin-wasm": "^3.5.0"

  },

  "dependencies": {

    "@midnight-ntwrk/compact-js": "2.5.1",

    "@midnight-ntwrk/dapp-connector-api": "4.0.1",

    "@midnight-ntwrk/midnight-js-contracts": "4.1.1",

    "@midnight-ntwrk/midnight-js-fetch-zk-config-provider": "4.1.1",

    "@midnight-ntwrk/midnight-js-http-client-proof-provider": "4.1.1",

    "@midnight-ntwrk/midnight-js-indexer-public-data-provider": "4.1.1",

    "@midnight-ntwrk/midnight-js-network-id": "4.1.1",

    "@midnight-ntwrk/midnight-js-protocol": "4.1.1",

    "@midnight-ntwrk/midnight-js-types": "4.1.1",

    "@midnight-ntwrk/midnight-js-utils": "4.1.1",

    "fp-ts": "^2.16.11",

    "graphql": "^16.11.0",

    "effect": "^3.14.0",

    "pino": "^10.3.1",

    "rxjs": "^7.8.2",

    "semver": "^7.7.4"

  },

  "overrides": {

    "smoldot": "npm:@aspect-build/empty@0.0.0"

  }

}
```

Create the contract directory structure and its `package.json`, which defines the compile and build scripts for the Compact contract.

```
mkdir -p contract/src

touch contract/package.json
```

The `compact` script invokes the Compact compiler. The `build` script compiles TypeScript and copies the managed output into `dist/`.

```
{

  "name": "leaderboard-contract",

  "version": "0.1.0",

  "private": true,

  "type": "module",

  "main": "dist/index.js",

  "types": "./dist/index.d.ts",

  "exports": {

    ".": {

      "types": "./dist/index.d.ts",

      "import": "./dist/index.js",

      "default": "./dist/index.js"

    }

  },

  "scripts": {

    "compact": "compact compile leaderboard.compact managed/leaderboard",

    "build": "rm -rf dist && tsc --project tsconfig.build.json && cp -Rf ./managed ./dist/managed"

  }

}
```

The witnesses use `TextEncoder`, which requires the `"DOM"` lib entry. The `compact-js` subpath exports require `"Bundler"` module resolution. The `"types": []` prevents TypeScript from automatically including type definitions from the monorepo root that belong to other packages.

```
touch contract/tsconfig.build.json
```

This TypeScript configuration targets ES2022 and outputs declaration files alongside the compiled JavaScript.

```
{

  "compilerOptions": {

    "outDir": "dist",

    "declaration": true,

    "lib": ["ESNext", "DOM"],

    "target": "ES2022",

    "module": "ESNext",

    "moduleResolution": "Bundler",

    "allowJs": true,

    "strict": true,

    "isolatedModules": true,

    "sourceMap": true,

    "esModuleInterop": true,

    "skipLibCheck": true,

    "types": []

  },

  "include": ["src"]

}
```

## Write the contract[​](#write-the-contract "Direct link to Write the contract")

Create the contract source file. This file contains the entire Compact smart contract.

```
touch contract/leaderboard.compact
```

The contract defines a `ScoreEntry` struct stored in a `Map` keyed by an auto-incrementing counter. A `witness` function lets the frontend feed private data into the circuit on demand.

```
pragma language_version 0.23;



import CompactStandardLibrary;



struct ScoreEntry {

  score: Uint<64>,

  displayName: Bytes<32>,

  ownerHash: Bytes<32>

}



export ledger scores: Map<Uint<64>, ScoreEntry>;

export ledger nextId: Counter;



witness localSecretKey(): Bytes<32>;

witness getCustomName(): Bytes<32>;
```

`scores` is a public ledger field readable from the indexer. `nextId` auto-increments to assign unique entry IDs. The two `witness` declarations tell the compiler that the TypeScript host provides `localSecretKey` and `getCustomName` at runtime. `localSecretKey` returns a random secret stored in the user's private state. `getCustomName` returns a custom display name.

### ownerCommitment[​](#ownercommitment "Direct link to ownerCommitment")

This helper circuit derives an on-chain identity from the user's secret key. It hashes the secret with a domain separator using `persistentHash`, producing a deterministic commitment that can be stored on the ledger and verified later without revealing the secret.

```
export circuit ownerCommitment(sk: Bytes<32>): Bytes<32> {

  return persistentHash<Vector<2, Bytes<32>>>([pad(32, "leaderboard:owner:"), sk]);

}
```

### submitScore[​](#submitscore "Direct link to submitScore")

The main circuit creates a new leaderboard entry with the caller's score, display name, and owner commitment.

```
export circuit submitScore(

  score: Uint<64>,

  useCustomName: Boolean

): [] {

  const sk = localSecretKey();

  const ownerHash = ownerCommitment(sk);



  nextId.increment(1);

  const entryId = disclose(nextId.read() as Uint<64>);



  if (disclose(useCustomName)) {

    const customName = getCustomName();

    scores.insert(entryId, ScoreEntry {

      score: disclose(score),

      displayName: disclose(customName),

      ownerHash: disclose(ownerHash)

    });

  } else {

    scores.insert(entryId, ScoreEntry {

      score: disclose(score),

      displayName: disclose(persistentHash<Bytes<32>>(sk)),

      ownerHash: disclose(ownerHash)

    });

  }

}
```

`localSecretKey()` calls the witness to retrieve the user's secret from private state. The secret never leaves the user's machine and never appears on-chain.

`ownerCommitment(sk)` hashes the secret with a domain separator to produce a deterministic identity. The domain separator `"leaderboard:owner:"` ensures this hash cannot collide with hashes used for other purposes.

`disclose()` marks data as safe to store publicly on-chain. Every value in a ledger write must be disclosed.

The `if/else` branch controls what gets stored as the display name. When `useCustomName` is true, the contract calls the `getCustomName()` witness to retrieve a name from the TypeScript host. The application layer uses this for both custom-named and public-address submissions, deciding which value to feed through the witness. When `useCustomName` is false, the contract does not invoke a witness and sets the display name to `persistentHash(sk)`. This produces raw hash bytes that the UI renders as a generated name like "Crimson Tiger."

### verifyOwnership[​](#verifyownership "Direct link to verifyOwnership")

The verification circuit lets a user prove they own a leaderboard entry without revealing their secret key.

```
export circuit verifyOwnership(targetEntryId: Uint<64>): [] {

  assert(scores.member(disclose(targetEntryId)), "entry not found");

  const entry = scores.lookup(disclose(targetEntryId));

  const callerHash = ownerCommitment(localSecretKey());

  assert(callerHash == entry.ownerHash, "not the owner");

}
```

The caller proves that `ownerCommitment(theirSecretKey)` matches the `ownerHash` stored in the entry. The secret key never leaves the user's machine. The proof server generates the proof locally. You can use it to claim a prize, verify identity, or present to a badge system. It does not write anything to the ledger.

## Compile the contract[​](#compile-the-contract "Direct link to Compile the contract")

Install the project dependencies and compile the contract. The compiler generates TypeScript bindings, circuit keys, and ZKIR files in `contract/managed/leaderboard/`.

```
npm install

npm run compile
```

Expected output:

```
Compiling 2 circuits:

  circuit "submitScore" (k=13, rows=4720)

  circuit "verifyOwnership" (k=13, rows=2352)
```
