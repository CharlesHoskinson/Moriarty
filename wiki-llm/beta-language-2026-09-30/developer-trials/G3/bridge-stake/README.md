# Cross-stake bridge authoring

`vault-bridge.mori` states a home rail asset and a foreign bridge-claim asset, then separate bridge and staking actions. Those actions are SpecifiedOnly. `mori test .` expects unsupported execution, not balances.

The JSON files under `scenarios/` are closed local stipulations. They are not source intent and they are not applied to these actions.

`invalid-cross-stake.mori`, `invalid-nominal-sum.mori`, and `invalid-alias.mori` are intentional rejections.
