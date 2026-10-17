# HostOnly execution configuration

`mori_core::sherlock_compile_artifact::config::ExecutionConfig` describes `sherlock-host-config/1`. `parse_config` parses it; `ExecutionConfig::consumer()` converts its wire fields to existing consumer types. This is the source-compiler configuration, not a native target profile. [Exact declarations](../../../crates/mori-core/src/sherlock/compile_artifact/config.rs#L73), [compiler API](host-source-compiler.md).

All nine top-level fields are required: `format`, `profile`, `grammar`, `finance`, `bounds`, `layout`, `fuel`, `stack`, `host`. Nested structs deny unknown fields. Fixed string tags are exactly the values below. Numeric fields are canonical nonnegative decimal strings, checked into `usize` or `u128`; JSON numeric tokens are not substituted. No omitted numerical field has a default.

`layout.field_id`, `layout.widths`, `layout.scalar_layout_id` are required explicit nulls. `boundary_capacities_present` is false for this HostOnly format. The one supplied bounds object is also used for layout bounds. Fuel/stack use `RunInput::{Nat { value }, Negative, NatAboveU128}` tagged by `kind`; the specimen selects Nat. Configuration contains no action/scenario inputs, wallet data or network qualification.

## Exact observed fixture

The complete specimen is [config.json](../../../crates/mori-core/tests/sherlock_compile_artifact_positive/config.json), SHA-256 `ad1448ba95d1f5c7b201b9d309e0d1a125d97fda2d7fc9b4425597b3470862ad`. These are that fixture's values, not defaults or recommended resource settings. Retain the original file bytes when reproducing its identity; formatting/newline changes alter the file digest.

```json
{
  "format": "sherlock-host-config/1",
  "profile": "sherlock-helper-microstep/1",
  "grammar": "DedicatedSorts",
  "finance": "Absent",
  "bounds": {
    "code_rows": "64",
    "function_rows": "4",
    "code_child_slots": "16",
    "parameter_slots": "8",
    "pattern_rows": "8",
    "match_plan_rows": "8",
    "match_arm_rows": "8",
    "type_rows": "8",
    "constructor_rows": "8",
    "constructor_field_slots": "16",
    "max_arity": "8",
    "constant_nodes": "16",
    "constant_field_slots": "16",
    "data_nodes": "64",
    "data_field_slots": "64",
    "environment_cells": "64",
    "frames": "64",
    "helper_tasks": "64",
    "serialization_bytes": "65536",
    "max_reference_work": "4096",
    "max_helper_work": "4096",
    "step_cap": "4096"
  },
  "layout": {
    "layout_version": [
      "moriarty",
      "sherlock-helper-microstep-hostonly/1"
    ],
    "field_id": null,
    "widths": null,
    "scalar_layout_id": null,
    "capacities": {
      "name_rows": "32",
      "name_bytes": "256",
      "occurrence_path_slots": "256",
      "value_slots": "1024"
    },
    "boundary_capacities_present": false
  },
  "fuel": {
    "kind": "Nat",
    "value": "1000"
  },
  "stack": {
    "kind": "Nat",
    "value": "32"
  },
  "host": {
    "max_source_actions": "4",
    "max_source_expressions": "256",
    "max_source_statements": "64",
    "max_action_inputs": "8",
    "max_supplied_inputs": "8",
    "max_projection_occurrences": "512",
    "max_generated_ast_nodes": "512",
    "max_ast_nesting": "128",
    "max_input_name_bytes": "64",
    "max_total_input_name_bytes": "512"
  }
}
```

The separate target specimen is exactly `{"kind":"public-host-test-descriptor"}` followed by LF (39 bytes), SHA-256 `795994c1e64aa1703482fa09a995d5e94b09b2741c7f4885a3a3f6b33c8be9b1`. It labels a HostOnly test descriptor; it supplies no address, account or stagenet identity. See the [published comparison inputs](../../results/sherlock-source-compile-20261017/independent/26-compiler-compare-source/inputs/target.json).
