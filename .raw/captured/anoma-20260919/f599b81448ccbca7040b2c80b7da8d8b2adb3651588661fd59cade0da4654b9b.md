# Anoma Pay SDK

TypeScript SDK for the Anoma Pay protocol, supported by Rust WebAssembly library. Contains domain logic, API clients, WASM bindings, and utilities shared across Anoma Pay applications.

## Tech Stack

- [TypeScript ~6.0](https://www.typescriptlang.org/)
- [Viem](http://viem.sh/)
- [Zod](https://zod.dev/)
- [Rust](https://doc.rust-lang.org/)
- [wasm-bindgen](https://github.com/wasm-bindgen/wasm-bindgen/)
- [binaryen](https://github.com/WebAssembly/binaryen/)
- [uniffi-bindgen-react-native](https://github.com/jhugman/uniffi-bindgen-react-native/)

## Setup

Requirements:

- Node.js (>= 22)
- pnpm (installable via `npm i --global pnpm` with `npm`, which is bundled with Node.js)
- Rust toolchain, the `wasm-bindgen` CLI and `wasm-opt` (binaryen) — for ARM
  bindings builds only

Install dependencies from the repository root:

```bash
pnpm install
```

## Usage

Import from the SDK using subpath exports:

```typescript
import { TransferBuilder } from "@anomaorg/anoma-pay-sdk/domain/transfer/models/TransferBuilder";
import { formatBalance } from "@anomaorg/anoma-pay-sdk/lib";
import type { AppResource } from "@anomaorg/anoma-pay-sdk/types";
```

### Vite

Vite apps need one line of config:

```typescript
export default defineConfig({
  optimizeDeps: { exclude: ["@anomaorg/arm-bindings"] },
});
```

`@anomaorg/arm-bindings` locates its WebAssembly binary relative to
`import.meta.url`. Vite's dev-mode dependency pre-bundling copies the module
into `node_modules/.vite/deps/`, which moves that URL while leaving the
relative path intact, so the lookup misses and the dev server answers with
`index.html` — surfacing as:

```
CompileError: WebAssembly.instantiate(): expected magic word 00 61 73 6d, found 3c 21 64 6f
```

Excluding the package keeps it unbundled and the path correct. This affects
`vite dev` only: `vite build` resolves `new URL(..., import.meta.url)` natively
and emits the binary as an asset, so production builds need no configuration.
Bundlers other than Vite are unaffected.

## Examples

Runnable examples live in [`examples/`](./examples). Start with
[`examples/balance`](./examples/balance): derives a keyring from a seed and
displays the anomapay address and token balances.

## Building ARM bindings

Cryptographic operations are implemented in Rust in
[`arm-bindings/`](./arm-bindings) and exposed to TypeScript through
uniffi-bindgen-react-native.

```bash
pnpm run build:arm          # release build
pnpm run build:arm:dev      # debug build
```

Output lands in `arm-bindings/generated/` and `arm-bindings/dist/`, neither of
which is committed. CI builds it whenever a PR touches `arm-bindings/` or
`patches/` (see
[`.github/workflows/ci-arm-bindings.yml`](./.github/workflows/ci-arm-bindings.yml)).

The SDK consumes the published `@anomaorg/arm-bindings` package, so building
the bindings is only needed when changing the Rust code itself.

## Releases

[release-please](https://github.com/googleapis/release-please) tracks both
packages from [`release-please-config.json`](./release-please-config.json) and
opens a separate release PR for each. The SDK tags as `vX.Y.Z`; the bindings tag
as `arm-bindings-vX.Y.Z`.

Merging a release PR triggers the matching publish workflow —
[`publish-sdk`](./.github/workflows/publish-sdk.yml) or
[`publish-arm-bindings`](./.github/workflows/publish-arm-bindings.yml) — each of
which builds, tests and publishes to npm with provenance. Both can also be run
by hand with `dry-run` for a rehearsal.

Because provenance is enabled, publishing only works from CI; `npm publish` from
a laptop will fail.

## License

[Apache-2.0](./LICENSE), matching the upstream `anoma-rm-risc0` crates.

## Pay Address Format

Anoma Pay uses a custom address format that encodes multiple public keys into a single Base64URL string:

| Field                    | Size (bytes) |
| ------------------------ | ------------ |
| Authority Public Key     | 33           |
| Discovery Public Key     | 33           |
| Encryption Public Key    | 33           |
| Nullifier Key Commitment | 32           |
| CRC32 Checksum           | 4            |
| **Total**                | **135**      |

The raw bytes are concatenated in the order shown above and then encoded using Base64URL encoding. The CRC32 checksum provides integrity verification when decoding addresses.

## Audits

Our software undergoes regular audits:

1. Informal Systems
   - Company Website: https://informal.systems
   - Commit ID: [957e8bf1e89a824f7c9be911b49533b9da1f5e72](https://github.com/anoma/pay-interface-app/tree/957e8bf1e89a824f7c9be911b49533b9da1f5e72)
   - Started: 2025-12-01
   - Finished: 2025-12-16
   - Last revised: 2025-12-19

   [Audit Report (pdf)](./audits/2025-12-19_Informal_Systems_AnomaPay_Phase_I.pdf)

## Project Structure

```
src/
├── api           # API clients (Backend, Indexer)
├── domain        # Core domain logic
│   ├── history       # Transaction history types
│   ├── keys          # Key hierarchy structure and services
│   ├── payroll       # Payroll schema and utilities
│   ├── queue         # Transfer queue management
│   ├── resources     # Resource machine handling
│   └── transfer      # Transfer models and services
├── lib           # Shared utility functions
├── lib-constants.ts  # SDK library constants
├── types.ts      # Shared type definitions
└── version.ts      # Current SDK version, generated when running `pnpm build`
```

## Linting

```bash
pnpm lint
```

## Type Checking

```bash
pnpm tsc-check
```

## Unit Tests

```bash
pnpm test        # watch mode
pnpm test:run    # single run
```

## Repository Conventions

- All commits must follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) standard
