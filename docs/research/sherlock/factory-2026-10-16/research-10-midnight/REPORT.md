# Midnight network deployment and readback

## Source claims

The official guide promotes a local deployment to Preprod by changing network ID, endpoints, wallet funding, and proof-server configuration. It names Preview, Preprod, and Mainnet; it does not establish a “stagenet.” [Source locator: Prerequisites, “Configuring providers,” and “Promoting a deployment to Preprod.”]

Midnight.js separates local private-state storage, indexer GraphQL access, ZK artifacts (`keys/` and `zkir/`), proof requests, wallet balancing, and network submission. The Preprod instructions say to run your own proof server because it sees witness data in cleartext. [Source locator: “The transaction pipeline and its providers,” steps 2–8 under “Configuring providers,” and Preprod step 3.]

Its local verification deploys, reconnects, calls a circuit, checks transaction ID/block height, then queries indexer state and decodes `state.data` with generated `ledger()`. It separately tests node block-header subscriptions and extrinsic/event extraction. [Source locator: “Verification” under deploying and “Observing contract and chain state.”]

## Unknowns and limits

The guide is Compact-specific; it does not show arbitrary custom-ZKIR artifacts being loaded, proved, accepted, or read back. It gives no Sherlock operator-specific network, endpoint, wallet, or admission details. “Stagenet” needs reconciliation with current Preview/Preprod naming. It does not define `EffectStream` semantics or effect/prefix oracle correspondence.

## Factory proposal (inference)

Proposal: add a bounded qualification contract per supported network. Compile the exact candidate artifacts; use a test wallet, node, indexer, and proof service; deploy and call one deterministic state-changing circuit; check tx ID/block height and compare indexer-decoded state plus node transaction/event evidence to an independent Rust state/effect-prefix oracle. Record artifact hashes, network ID, endpoints, and observations. Require proof of the exact non-Compact artifact route before calling this custom-ZKIR qualification.

Counterpoint: the guide demonstrates local Compact deployment; its Preprod instructions do not show Sherlock artifacts or an operator deployment passing.
