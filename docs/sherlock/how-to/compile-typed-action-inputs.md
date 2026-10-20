# Compile a Core action with typed inputs

Status: source candidate, UNRUN. The public permissionless compiler requires no
project reviewer records. Internal source review and build qualification remain
separate from public program validity.

Supply a separate typed input map, the existing explicit host config and target
descriptor, and a new output directory:

```sh
mori compile program.mori --target target.json --out out \
  --execution-profile sherlock-helper-microstep/1 --action quote \
  --execution-config core-config.json --execution-inputs quote.inputs.json
```

The input file has the same map/value shape as Scenario.inputs:

```json
{"flat":{"type":"UInt","value":"4"},"units":{"type":"UInt","value":"9"}}
```

UInt uses a canonical decimal string; Bool uses a JSON Boolean. Supply every
declared argument with its exact name and type. Unknown names are passed to the
existing consumer and refused. Do not put Scenario state, balances or liabilities
around this map. The strict reader and JSON/Value codecs keep their existing
limits and refusal rules.

Compile-only prepares and admits the source-shaped scalar program without
running it. Add --observe for the existing host owner execution. Successful
publication creates sherlock-artifact.json containing the complete HostOnly
report. Input values are visible in input_policy.binding.values; they are also
concrete entry Call arguments. Bindings can change the admitted program identity.

Without --execution-inputs, the existing parameter-free route and output bytes
are retained. Explicit {} selects the new mode even for a zero-input action.
Draft and nominal --entry profiles do not accept this option.

This scalar route projects a final emit's value. It does not execute the ordered
Core emit effect or other financial effects. Witness/constraint plans and native
export remain missing or unsupported. The artifact is not a native proof or an
input accepted by current prove/verify commands.

See the [wire and error reference](../reference/typed-action-inputs.md) and
[binding explanation](../explanation/concrete-action-input-binding.md).
