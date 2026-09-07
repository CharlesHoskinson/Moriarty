> For the complete documentation index, see [llms.txt](/llms.txt)

# Midnight Explorer

[Midnight Explorer](https://github.com/Tech-Expansion/midnight-explorer-web) is an open-source blockchain explorer for the Midnight Network. It provides a web interface to search, trace, and analyze on-chain activity, including transactions, blocks, smart contracts, and liquidity pools. [TexLabs](https://texlabs.org) maintains the project under the [Tech-Expansion](https://github.com/Tech-Expansion) GitHub organization and hosts a public instance at [midnightexplorer.com](https://www.midnightexplorer.com).

Community-maintained - not audited

TexLabs maintains this project, not the Midnight Foundation. It has not been audited. Evaluate it independently before relying on its data for production decisions.

## Prerequisites[​](#prerequisites "Direct link to Prerequisites")

Before running the explorer locally, ensure that you have:

* Node.js 20 or later.
* npm.
* Docker, if you plan to use the containerized deployment.

## Compatibility[​](#compatibility "Direct link to Compatibility")

| Component                           | Version                        | Notes                                                                                     |
| ----------------------------------- | ------------------------------ | ----------------------------------------------------------------------------------------- |
| Explorer version                    | `0.1.0`                        | No formal releases; code accessed from the `main` branch.                                 |
| `@midnight-ntwrk/compact-runtime`   | `^0.9.0`                       | Declared in `package.json`.                                                               |
| `@midnight-ntwrk/midnight-js-types` | `^2.0.2`                       | Declared in `package.json`.                                                               |
| `@midnight-ntwrk/zswap`             | `^4.0.0`                       | Declared in `package.json`.                                                               |
| Networks                            | Preview, Preprod, Mainnet Lite | Selected from the header network switcher. The public instance serves Preprod by default. |
| License                             | Apache 2.0                     |                                                                                           |

## Features[​](#features "Direct link to Features")

The explorer provides the following views:

* **Universal search**: look up blocks, transactions, addresses, and contracts by hash or ID.
* **Block explorer**: browse the latest blocks with full block details and transaction lists.
* **Transaction viewer**: inspect transaction data, inputs, outputs, and contract interactions.
* **Address explorer**: view address history, balances, and associated contracts.
* **Smart contracts**: browse deployed contracts and their on-chain state.
* **Pool analytics**: track liquidity pool metrics and activity.
* **Network charts**: visualize on-chain statistics and trends over time.
* **Live network stats**: monitor the current epoch, slot, total blocks, transactions, and average block time.
* **Network switcher**: switch between Midnight environments from the header.

## Run the explorer locally[​](#run-the-explorer-locally "Direct link to Run the explorer locally")

Install the dependencies and start the development server:

```
npm install

npm run dev
```

The `dev` script starts Next.js on port `8081`. Open `http://localhost:8081` in your browser.

The project uses Next.js 15 (App Router), TypeScript, Tailwind CSS v4, Radix UI, TanStack Query, Recharts, and PostgreSQL.

## Deployment[​](#deployment "Direct link to Deployment")

The repository ships a `Dockerfile` and an Nginx compose file (`docker-compose.nginx.yml`) for containerized hosting. The README documents HTTPS setup with Let's Encrypt, certificate auto-renewal, and a blue-green strategy for zero-downtime updates. See the [project README](https://github.com/Tech-Expansion/midnight-explorer-web#readme) for the full deployment commands.

## Additional resources[​](#additional-resources "Direct link to Additional resources")

* [Project repository](https://github.com/Tech-Expansion/midnight-explorer-web): source, README, and deployment guide.
* [Public explorer instance](https://www.midnightexplorer.com): hosted by TexLabs and serving Preprod by default.

## Report issues[​](#report-issues "Direct link to Report issues")

For issues with the explorer, file on [TexLabs' tracker](https://github.com/Tech-Expansion/midnight-explorer-web/issues).

For issues with this documentation page, file on the [Midnight docs repository](https://github.com/midnightntwrk/midnight-docs/issues).
