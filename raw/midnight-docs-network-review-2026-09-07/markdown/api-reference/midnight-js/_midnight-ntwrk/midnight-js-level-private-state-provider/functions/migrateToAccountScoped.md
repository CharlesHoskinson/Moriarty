# migrateToAccountScoped

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-level-private-state-provider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider.md) / migrateToAccountScoped

# Function: migrateToAccountScoped()

> **migrateToAccountScoped**(`config`): `Promise`<[`MigrationResult`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/interfaces/MigrationResult.md)>

Migrates existing unscoped private state and signing key data to account-scoped sublevels.

This function copies data from the legacy unscoped locations to the new account-scoped locations. The original data is preserved (not deleted) to allow for safe rollback if needed. To remove old data after successful migration, manually clear the unscoped sublevels.

Note: Running this function multiple times is safe but will re-copy all data, overwriting any changes made in the scoped location since the last migration.

## Parameters[​](#parameters "Direct link to Parameters")

### config[​](#config "Direct link to config")

`Partial`<[`LevelPrivateStateProviderConfig`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/interfaces/LevelPrivateStateProviderConfig.md)> & `Pick`<[`LevelPrivateStateProviderConfig`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/interfaces/LevelPrivateStateProviderConfig.md), `"accountId"`>

## Returns[​](#returns "Direct link to Returns")

`Promise`<[`MigrationResult`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/interfaces/MigrationResult.md)>
