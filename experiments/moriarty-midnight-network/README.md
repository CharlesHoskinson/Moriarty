# Midnight settlement environment

This is a reproduction of the official Midnight local-dev stack at commit
902561ddc27a4b096f19835ab1528f38ace515f1 and the official create-mn-app
hello-world template at commit bdc86733d2a9d2e381cb050aec4fdde7b33559b4
(version 0.5.1). Exact provenance is in origins.json. The generated scaffold and
its package-lock are preserved under hello-world/.

Wallet secrets and runtime wallet state must remain in
/home/charl/.local/share/moriarty/test-wallets, with directory mode 700 and seed
files mode 600. None of these commands prints a seed or mnemonic.

Run the commands below from `/home/charl/Moriarty/.worktrees/r3-native` unless
a command explicitly changes directory. The `hello-world-dedicated-v2` state
directory is the external runtime directory used for the recorded dedicated
wallet deployment and call.

## Bootstrap

    mkdir -p /home/charl/.local/share/moriarty/test-wallets
    chmod 700 /home/charl/.local/share/moriarty/test-wallets
    umask 077
    test -f /home/charl/.local/share/moriarty/test-wallets/local-undeployed.seed || openssl rand -hex 32 > /home/charl/.local/share/moriarty/test-wallets/local-undeployed.seed
    test -f /home/charl/.local/share/moriarty/test-wallets/public-preprod.seed || openssl rand -hex 32 > /home/charl/.local/share/moriarty/test-wallets/public-preprod.seed
    chmod 600 /home/charl/.local/share/moriarty/test-wallets/*.seed
    git clone https://github.com/midnightntwrk/midnight-local-dev.git experiments/moriarty-midnight-network/local-dev
    git -C experiments/moriarty-midnight-network/local-dev checkout --detach 902561ddc27a4b096f19835ab1528f38ace515f1
    git -C experiments/moriarty-midnight-network/local-dev apply ../local-dev.patch
    cp experiments/moriarty-midnight-network/local-dev.env.example experiments/moriarty-midnight-network/local-dev/.env
    npm ci --ignore-scripts --prefix experiments/moriarty-midnight-network/local-dev

## Start the digest-pinned stack

The second Compose file changes only image references to the tested immutable
digests. The local-dev patch supplies names, loopback ports, and resource caps.

    docker compose --env-file experiments/moriarty-midnight-network/local-dev/.env -p moriarty-midnight-local-dev -f experiments/moriarty-midnight-network/local-dev/standalone.yml -f experiments/moriarty-midnight-network/compose.digests.yml pull
    docker compose --env-file experiments/moriarty-midnight-network/local-dev/.env -p moriarty-midnight-local-dev -f experiments/moriarty-midnight-network/local-dev/standalone.yml -f experiments/moriarty-midnight-network/compose.digests.yml up -d --wait

## Fund the dedicated local wallet

    cd experiments/moriarty-midnight-network/local-dev
    timeout --signal=TERM --kill-after=30s 20m npm start -- --fund-config ./accounts.moriarty.json
    cd -

This transfers 50,000 NIGHT and registers the recipient NIGHT UTXO for DUST.

## Compile, deploy, call, and read

The scaffold package override pins onchain-runtime-v3 to 3.0.0. npm dedupe is
required so Compact runtime and Midnight.js share one physical WASM class
instance. The seed-file environment variable is a local scaffold adaptation
that selects the dedicated funded wallet without exposing its secret.

    cd experiments/moriarty-midnight-network/hello-world
    npm ci --ignore-scripts
    npm dedupe
    curl --proto '=https' --tlsv1.2 -LSsf --max-time 60 -o /tmp/compact-installer.sh https://github.com/midnightntwrk/compact/releases/download/compact-v0.5.2/compact-installer.sh
    chmod 700 /tmp/compact-installer.sh
    COMPACT_NO_MODIFY_PATH=1 /tmp/compact-installer.sh --quiet
    PATH=/home/charl/.local/bin:$PATH compact update 0.31.1
    PATH=/home/charl/.local/bin:$PATH npm run compile
    cd -
    mkdir -p /home/charl/.local/share/moriarty/test-wallets/hello-world-dedicated-v2
    chmod 700 /home/charl/.local/share/moriarty/test-wallets/hello-world-dedicated-v2
    cd /home/charl/.local/share/moriarty/test-wallets/hello-world-dedicated-v2
    export MIDNIGHT_WALLET_SEED_FILE=/home/charl/.local/share/moriarty/test-wallets/local-undeployed.seed
    export MIDNIGHT_INDEXER_URL=http://127.0.0.1:18088/api/v4/graphql
    export MIDNIGHT_INDEXER_WS_URL=ws://127.0.0.1:18088/api/v4/graphql/ws
    export MIDNIGHT_NODE_URL=http://127.0.0.1:19944
    export MIDNIGHT_PROOF_SERVER_URL=http://127.0.0.1:16300
    timeout --signal=TERM --kill-after=30s 15m /home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/.bin/tsx /home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/src/deploy.ts
    /home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules/.bin/tsx /home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/src/cli.ts

In the CLI choose 1 and enter a test message, choose 2 to read the settled
state, then choose 4 to stop the wallet.

## Verify existing receipts without submitting a transaction

    cd /home/charl/Moriarty/.worktrees/r3-native
    node experiments/moriarty-midnight-network/verify-local.mjs

The verifier rejects HTTP, GraphQL, and JSON-RPC errors. It checks indexer
status and receipt values, compares every indexer block hash to
chain_getBlockHash(height), and requires the finalized height to cover every
receipt block.

## Resume Public Preprod after the human faucet action

    cd experiments/moriarty-midnight-network/local-dev
    timeout --signal=TERM --kill-after=30s 20m npm run public-wallet -- /home/charl/.local/share/moriarty/test-wallets/public-preprod.seed

The official faucet requires Cloudflare Turnstile. Do not claim a Preprod
transaction until its identifier, hash, block, indexer result, and finality
evidence are captured.

## Stop

    docker compose --env-file experiments/moriarty-midnight-network/local-dev/.env -p moriarty-midnight-local-dev -f experiments/moriarty-midnight-network/local-dev/standalone.yml -f experiments/moriarty-midnight-network/compose.digests.yml down
