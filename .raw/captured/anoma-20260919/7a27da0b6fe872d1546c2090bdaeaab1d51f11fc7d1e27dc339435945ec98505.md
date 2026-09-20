# Balance example

Derives a keyring from a seed string and shows the resulting pay address and its
token balances, using the SDK directly.

```bash
pnpm install
pnpm dev
```

The `seed` at the top of [`src/main.ts`](./src/main.ts) is an arbitrary string,
not a private key or mnemonic: the keyring is derived from it via HKDF. The
committed one is a public placeholder — edit it to get a different account, and
never put a real secret there.

The example resolves `@anomaorg/anoma-app-sdk` from the repository root, so run
`pnpm build` there first if `dist/` is missing.
