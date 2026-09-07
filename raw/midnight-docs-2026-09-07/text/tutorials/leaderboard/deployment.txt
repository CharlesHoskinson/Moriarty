> For the complete documentation index, see [llms.txt](/llms.txt)

# Part 4: Production deployment

In this section, you deploy the leaderboard frontend to Vercel.

## Vercel build configuration[​](#vercel-build-configuration "Direct link to Vercel build configuration")

Create the Vercel build configuration. This tells Vercel how to build the monorepo: it compiles the contract and API packages first, then builds the Vite frontend.

```
touch vercel.json
```

The `buildCommand` chains the TypeScript compilation for each workspace package. The `outputDirectory` points to the Vite build output.

```
{

  "buildCommand": "cd contract && npm run build && cd ../api && npm run build && cd ../leaderboard-ui && npm run build",

  "outputDirectory": "leaderboard-ui/dist",

  "installCommand": "npm install",

  "framework": null

}
```

### Compiled contract output[​](#compiled-contract-output "Direct link to Compiled contract output")

Vercel does not have the Compact compiler installed. Make sure `contract/managed/` is not in your `.gitignore` so your push to GitHub includes the compiled circuit keys and TypeScript bindings.

### Production environment file[​](#production-environment-file "Direct link to Production environment file")

Create the production environment file with your deployed contract address.

```
touch leaderboard-ui/.env.preprod
```

The `leaderboard-ui` build script runs `vite build --mode preprod`, which loads `.env.preprod` instead of `.env`.

```
VITE_NETWORK_ID=preprod

VITE_INDEXER_URL=https://indexer.preprod.midnight.network/api/v4/graphql

VITE_INDEXER_WS_URL=wss://indexer.preprod.midnight.network/api/v4/graphql/ws

VITE_DEFAULT_CONTRACT=<your deployed contract address>
```

### Deploy to Vercel[​](#deploy-to-vercel "Direct link to Deploy to Vercel")

Import the repository and deploy it from the Vercel dashboard.

1. Go to [vercel.com](https://vercel.com) and sign in with GitHub.
2. Click **Add New Project** and import your repository.
3. Make sure the **Root Directory** setting is empty (not set to a subdirectory).
4. Click **Deploy**.

## Test the production deployment[​](#test-the-production-deployment "Direct link to Test the production deployment")

Visit your Vercel URL in Chrome with Lace installed. The leaderboard loads from the indexer. Connect your wallet and submit a score to verify the full pipeline works end to end.

## Summary[​](#summary "Direct link to Summary")

The production stack relies on the following services.

| Component                   | Service                |
| --------------------------- | ---------------------- |
| Frontend                    | Vercel                 |
| Chain and indexer           | Midnight Preprod       |
| Wallet and proof generation | Lace browser extension |
| Test tokens                 | Preprod faucet         |

This architecture scales to any Midnight DApp. The contract defines the on-chain logic. The API layer wraps it for TypeScript consumption. The frontend connects via Lace, and the wallet's proof server handles proof generation.
