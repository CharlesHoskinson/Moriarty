> For the complete documentation index, see [llms.txt](/llms.txt)

# Edda Midnight starter template

[Edda Midnight Starter Template](https://github.com/eddalabs/midnight-starter-template) is a full-stack monorepo for building DApps on the Midnight Network. It ships a working counter application with a Compact smart contract, a React (Vite) frontend, and a CLI - ready to clone, build, and deploy to Vercel. A live demo runs at [counter.nebula.builders](https://counter.nebula.builders).

The project includes educational materials with video walkthroughs in English, Spanish, and Portuguese, making it a practical entry point for developers new to Midnight.

The Edda Labs maintainers built this as a fork of [MeshJS/midnight-starter-template](https://github.com/MeshJS/midnight-starter-template), adding the Apache-2.0 license, deployment automation, and the educational-material directory.

Community-maintained - not audited

[Edda Labs](https://github.com/eddalabs) maintains this template, not the Midnight Foundation. It has not been audited. The smart contract is a minimal counter example intended for learning - evaluate any production use independently.

## Prerequisites[​](#prerequisites "Direct link to Prerequisites")

Before using the template, ensure that you have:

* [Node.js v22+](https://nodejs.org/) installed.
* [pnpm v10+](https://pnpm.io/installation) installed.
* [Docker](https://docs.docker.com/desktop/) running (for the proof server and standalone network).
* [Git Large File Storage (LFS)](https://git-lfs.com/) installed - required for zero-knowledge proof artifacts (`.prover`, `.verifier`, `.bzkir`, `.zkir` files).
* The [Compact developer tools](/getting-started/installation.md) installed and updated to `0.30.0`.
* The [Lace wallet](https://chromewebstore.google.com/detail/lace/gafhhkghbfjjkeiendhlofajokpaflmk) browser extension configured for the Preview network.

## Compatibility[​](#compatibility "Direct link to Compatibility")

This template targets specific Compact compiler, runtime, and tooling versions. Verify these against your local environment before building.

| Component          | Version                               | Notes                                                                      |
| ------------------ | ------------------------------------- | -------------------------------------------------------------------------- |
| Compact compiler   | `0.31.0`                              | Set via `compact update +0.31.0`                                           |
| Compact language   | `>= 0.23`                             | Per `pragma` in the contract source                                        |
| Node.js            | 22+                                   | Pinned in `.node-version` and `engines.node`                               |
| pnpm               | `10.14.0`                             | Pinned via `packageManager` in `package.json`                              |
| Build orchestrator | Turborepo                             | Top-level scripts dispatch through `turbo run`                             |
| Networks           | undeployed, Preview, Preprod, Mainnet | Standalone undeployed network runs via Docker                              |
| License            | Apache-2.0                            | (Upstream Mesh template carries no license; Edda's fork added Apache-2.0.) |

## Monorepo structure[​](#monorepo-structure "Direct link to Monorepo structure")

The template organizes code into three workspace packages:

| Package               | Purpose                                                                                                                                                        |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `counter-contract`    | Compact smart contract with a single `increment` circuit that advances a public `round` counter. Compiles to TypeScript bindings and ZK proof artifacts.       |
| `counter-cli`         | CLI tool for deploying and interacting with the contract from the terminal. Supports all three networks.                                                       |
| `frontend-vite-react` | React 19 + Vite frontend with Lace wallet integration, TailwindCSS styling, and TanStack Router. Connects to the deployed contract and displays counter state. |

## Installation[​](#installation "Direct link to Installation")

Clone the repository:

```
git clone https://github.com/eddalabs/midnight-starter-template.git

cd midnight-starter-template
```

Download the zero-knowledge proof artifacts via Git Large File Storage (LFS):

```
git lfs install

git lfs pull
```

Update the Compact Toolchain to the version the template targets:

```
compact update +0.31.0
```

Install dependencies and build all workspace packages:

```
pnpm install

pnpm build
```

Both `counter-cli/` and `frontend-vite-react/` ship a `.env_template` file. Copy each to `.env` and fill in the values before running locally.

Git LFS required

The repository stores zero-knowledge proof artifacts (prover keys, verifier keys, ZKIR files) via Git LFS. Without `git lfs install` and `git lfs pull`, these files appear as text pointers instead of binary data, and contract deployment fails with "mismatched verifier keys" errors.

## Run locally[​](#run-locally "Direct link to Run locally")

Start the frontend against the Preview network:

```
pnpm dev:frontend
```

For a fully local setup with no external network dependency, start the standalone network first:

```
pnpm setup-standalone

pnpm dev:frontend
```

The standalone network runs Midnight node, indexer, and proof server containers in Docker.

## Testing[​](#testing "Direct link to Testing")

The CLI ships Vitest-based test suites that run the contract end-to-end on each network:

```
pnpm --filter @eddalabs/counter-cli test-undeployed

pnpm --filter @eddalabs/counter-cli test-preview

pnpm --filter @eddalabs/counter-cli test-preprod
```

These scripts spin up the relevant infrastructure (containers for `undeployed`, faucet calls for `preview` / `preprod`), deploy the contract, and assert on circuit behavior. They serve as a useful reference for setting up your own contract test harness.

## The smart contract[​](#the-smart-contract "Direct link to The smart contract")

The contract is a minimal counter with a single exported circuit:

```
pragma language_version >= 0.23;



import CompactStandardLibrary;



export ledger round: Counter;



export circuit increment(): [] {

  round.increment(1);

}
```

The contract is deliberately minimal - the template's value is the full-stack scaffolding around it (wallet connection, provider wiring, deployment pipeline), not the contract logic. Replace it with your own contract and the surrounding infrastructure adapts.

## Deploy to Vercel[​](#deploy-to-vercel "Direct link to Deploy to Vercel")

The Edda Labs team documents the full deployment procedure in their [DEPLOYMENT\_PROCEDURE.md](https://github.com/eddalabs/midnight-starter-template/blob/main/DEPLOYMENT_PROCEDURE.md). The key steps:

1. Enable **Git LFS** in your Vercel project settings before the first build.
2. Set the build command to `npm run build-production` (the script is workspace-aware and pulls LFS artifacts before building).
3. Set the output directory to `frontend-vite-react/dist`.
4. Add `VITE_CONTRACT_ADDRESS` as an environment variable with your deployed contract address.

Git LFS in Vercel

If you deploy without enabling Git LFS in Vercel's settings, then the build completes but the DApp fails at runtime. The proof artifacts are LFS pointers instead of binary files. Enable LFS first, then redeploy without cache.

## Educational materials[​](#educational-materials "Direct link to Educational materials")

The repository includes an `educational-material/` directory with structured session guides covering:

* zero-knowledge proofs and privacy-enhanced DApps
* devnet, Preview, and Testnet environments
* Video walkthroughs on the [Edda Labs YouTube channel](https://www.youtube.com/@eddalabs)
* Available in English, Spanish, and Portuguese

## Resources[​](#resources "Direct link to Resources")

* **[Project repository](https://github.com/eddalabs/midnight-starter-template)**: Source, README, and contributor guide.
* **[Live demo](https://counter.nebula.builders)**: The deployed counter DApp running on the Preview environment.
* **[Deployment procedure](https://github.com/eddalabs/midnight-starter-template/blob/main/DEPLOYMENT_PROCEDURE.md)**: Step-by-step Vercel deployment, including the Git LFS toggle.
* **[Educational material](https://github.com/eddalabs/midnight-starter-template/tree/main/educational-material)**: Session guides for devnet, Preview, and Testnet workflows.
* **[Edda Labs](https://www.eddalabs.io)**: The maintainer organization. Sibling repo: [`eddalabs/midnight-contracts`](https://github.com/eddalabs/midnight-contracts).
* **[Upstream template (MeshJS)](https://github.com/MeshJS/midnight-starter-template)**: The original starter that Edda's fork builds on.

## Report issues[​](#report-issues "Direct link to Report issues")

For issues with the template, file on [Edda Labs' tracker](https://github.com/eddalabs/midnight-starter-template/issues).

For issues with this documentation page, file on the [Midnight docs repository](https://github.com/midnightntwrk/midnight-docs/issues).
