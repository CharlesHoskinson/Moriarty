# @anomaorg/arm-bindings

WebAssembly bindings for [`anoma-rm-risc0`](https://github.com/anoma), the Anoma
resource machine. Provides resources, digests, nullifier keys, authorization
signatures, Merkle trees and encryption primitives to TypeScript.

Ships three targets from one Rust crate: wasm for browsers and Node, and
uniffi/JSI bindings for React Native on iOS and Android.

## Install

```bash
pnpm add @anomaorg/arm-bindings
```

## Usage

The wasm module must be initialised once before any binding is used.

```ts
import { initArmBindings, Digest, randomBytes } from "@anomaorg/arm-bindings";

await initArmBindings();

const nonce = Digest.fromBytes(randomBytes());
console.log(nonce.toHex());
```

## Platform entries

`main` is `./index`, deliberately without an extension: that is what makes
Metro probe for platform variants, so Expo picks up `index.native.js` on iOS
and Android and `index.web.js` on web, while every other bundler and Node fall
through to `index.js`. There is no `exports` map, because Metro honours it in
preference to `main` and would then skip the probing.

| Entry             | Resolved by                     | Backing bindings         |
| ----------------- | ------------------------------- | ------------------------ |
| `index.js`        | Vite, webpack, Rollup, Node     | `bundled/dist`           |
| `index.web.js`    | Metro, platform web             | `generated/index.web`    |
| `index.native.js` | Metro, platform ios and android | `generated/index.native` |

Only the first is pre-compiled. Metro reads `generated/` as TypeScript source,
so the wasm and `react-native` imports ubrn wrote stay untouched; every other
toolchain needs the built ESM in `bundled/dist`.

Do not add a `browser` field. Vite's default `resolve.mainFields` and Expo's
`resolverMainFields` both read it before `main`, so it would hand the
Metro-only web entry to Vite.

Expo web additionally needs the wasm registered as an asset, since
`generated/index.web` imports it as one:

```js
config.resolver.assetExts.push("wasm");
```

## Web bundlers

`initArmBindings()` locates the wasm binary relative to the module's own URL, so
bundlers that understand `new URL(..., import.meta.url)` (Vite, webpack 5,
Rollup) resolve it without extra configuration.

One exception: `vite dev` pre-bundles dependencies into
`node_modules/.vite/deps/`, which moves that URL out from under the binary.
Exclude the package to keep the path intact — `vite build` is unaffected.

```typescript
export default defineConfig({
  optimizeDeps: { exclude: ["@anomaorg/arm-bindings"] },
});
```

## Building from source

Requires a Rust toolchain and the `wasm-bindgen` CLI, pinned in
[`wasm-cargo-patch.toml`](./wasm-cargo-patch.toml).

```bash
pnpm build          # release wasm + optimise + bundle
pnpm build:dev      # debug wasm, no optimise
pnpm smoke          # load the built bundle and round-trip a digest
```

[`scripts/smoke.mjs`](./scripts/smoke.mjs) imports `bundled/dist` exactly as a
consumer would, so it fails if the wasm lookup or the published `files` list
breaks.

Three directories, one crate. [`rust/`](./rust) is the crate itself.
`ubrn build web` generates the TypeScript bindings into `generated/`, and
The release profile in
[`wasm-cargo-patch.toml`](./wasm-cargo-patch.toml) keeps it small. `tsdown` then bundles
[`bundled/index.ts`](./bundled/index.ts) into `bundled/dist`. Neither
`generated/` nor `bundled/dist` is committed.

`ubrn build ios` and `ubrn build android` emit the native halves — the
xcframework, `cpp/`, `ios/`, `android/` and the podspec — likewise generated,
likewise not committed.

The wasm-bindgen crate version is pinned in
[`wasm-cargo-patch.toml`](./wasm-cargo-patch.toml) and must be bumped together
with the CLI version installed in
[`.github/workflows/ci-arm-bindings.yml`](../.github/workflows/ci-arm-bindings.yml).

## License

[Apache-2.0](./LICENSE).
