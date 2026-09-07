# makeContractExecutableRuntime

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / makeContractExecutableRuntime

# Variable: makeContractExecutableRuntime

> `const` **makeContractExecutableRuntime**: (`zkConfigProvider`, `options`) => [`ManagedRuntime`](#)<`ContractExecutable.ContractExecutable.Context`, `ConfigError.ConfigError`>

Constructs an Effect managed runtime configured to execute contract executables.

## Parameters[​](#parameters "Direct link to Parameters")

### zkConfigProvider[​](#zkconfigprovider "Direct link to zkConfigProvider")

[`ZKConfigProvider`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/ZKConfigProvider.md)<`string`>

The [ZKConfigProvider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/ZKConfigProvider.md) that is to be adapted.

### options[​](#options "Direct link to options")

[`ContractExecutableRuntimeOptions`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/ContractExecutableRuntimeOptions.md)

Values that will be mapped into and made available within the constructed runtime.

## Returns[​](#returns "Direct link to Returns")

[`ManagedRuntime`](#)<`ContractExecutable.ContractExecutable.Context`, `ConfigError.ConfigError`>

An Effect [ManagedRuntime](#) that can be used to execute [ContractExecutable](#) instances.
