> For the complete documentation index, see [llms.txt](/llms.txt)

# Set up boot node

Boot nodes serve as initial connection points for nodes joining the Midnight Network. They help new nodes discover and connect to active peers in the network. While running a boot node is optional, doing so strengthens network decentralization and improves peer discovery.

This guide provides step-by-step instructions for setting up a boot node for Midnight.

## Prerequisites[​](#prerequisites "Direct link to Prerequisites")

Before setting up your Midnight boot node, ensure you have the following:

* [Cardano-db-sync instance set up](/nodes/cardano-db-sync.md) with accessible PostgreSQL port.
* [Midnight node set up](/nodes/full-node.md#install-midnight-node) with the environment variables set.

## Set up a boot node[​](#set-up-a-boot-node "Direct link to Set up a boot node")

Use the following command to run Midnight node in boot node mode. Choose the configuration for your target network.

* Preview
* Preprod

```
midnight-node \

  --chain /home/midnight/res/preview/chain-spec-raw.json \

  --base-path /home/midnight/data \

  --listen-addr /ip4/0.0.0.0/tcp/30333 \

  --bootnodes /dns/bootnode-1.preview.midnight.network/tcp/30333/ws/p2p/12D3KooWK66i7dtGVNSwDh9tTeqov1q6LSdWsRLJvTyzTCaywYgK \

  --no-private-ip
```

```
midnight-node \

  --chain /home/midnight/res/preprod/chain-spec-raw.json \

  --base-path /home/midnight/data \

  --listen-addr /ip4/0.0.0.0/tcp/30333 \

  --bootnodes /dns/bootnode-1.preprod.midnight.network/tcp/30333/ws/p2p/12D3KooWQxxUgq7ndPfAaCFNbAxtcKYxrAzTxDfRGNktF75SxdX5 \

  --no-private-ip
```

## Known network boot nodes[​](#known-network-boot-nodes "Direct link to Known network boot nodes")

Use the boot nodes corresponding to your target network environment.

* Preview
* Preprod

```
--bootnodes /dns/bootnode-1.preview.midnight.network/tcp/30333/ws/p2p/12D3KooWK66i7dtGVNSwDh9tTeqov1q6LSdWsRLJvTyzTCaywYgK \

--bootnodes /dns/bootnode-2.preview.midnight.network/tcp/30333/ws/p2p/12D3KooWHqFfXFwb7WW4jwR8pr4BEf562v5M6c8K3CXAJq4Wx6ym
```

```
--bootnodes /dns/bootnode-1.preprod.midnight.network/tcp/30333/ws/p2p/12D3KooWQxxUgq7ndPfAaCFNbAxtcKYxrAzTxDfRGNktF75SxdX5 \

--bootnodes /dns/bootnode-2.preprod.midnight.network/tcp/30333/ws/p2p/12D3KooWNrUBs22FfmgjqFMa9ZqKED2jnxwsXWw5E4q2XVwN35TJ
```

## Next steps[​](#next-steps "Direct link to Next steps")

With the boot node set up, run the [RPC node](/nodes/rpc-node.md) to interact with the Midnight blockchain.
