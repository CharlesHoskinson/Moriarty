# Components and consumers

Historical source pins remain; PR168's merge is recorded in [bridges](bridges-ledger.md). Exports do not establish execution.

| Component | Source and consumer | Boundary |
| --- | --- | --- |
| K-A host reference | `mori_core::sherlock`; [module registration](../../../crates/mori-core/src/sherlock/mod.rs#L10), `Machine` re-export | Typed `sherlock-k-a/1` reference; SDK-free host evaluation |
| Observation comparison | `mori_core::sherlock::compare`; [first_difference](../../../crates/mori-core/src/sherlock/compare.rs#L96) | Compares two supplied complete typed observations; no execution or trace validation |
| Host L4 rows | `mori_core::sherlock::layout_a`; [exports and preconditions](../../../crates/mori-core/src/sherlock/layout_a/mod.rs#L1) | Trusted typed preparation, observation encoding/decoding; no machine importer or native ABI |
| B helper owner | `mori_core::sherlock::helper_machine`; [registration](../../../crates/mori-core/src/sherlock/helper_machine/mod.rs#L1), `shared::OwnerEngine`, `shared::owner_host::host_owner_engine` | Separate helper microstep schedule; DedicatedSorts, finance Absent, HostOnly, genesis and one uncut segment |
| Core scalar adapter | `mori_core::sherlock_source`; [registration](../../../crates/mori-core/src/lib.rs#L27), `source_projection_report`, `prepare_scalar_action` | Source-static scalar projection; supported stateless action subset only |
| Core-to-B consumer | `helper_machine::core_action_consumer::{prepare_scalar_action_host,run_scalar_action}`; [declarations](../../../crates/mori-core/src/sherlock/helper_machine/core_action_consumer.rs) | Separate real preparation/admission and explicit run; scalar emit value becomes a return value, not an executed financial effect |
| Host compiler artifact | `mori_core::sherlock_compile_artifact::{compile_host,parse_config}`; [module](../../../crates/mori-core/src/sherlock/compile_artifact.rs) | Typed HostOnly input/artifact route; two compile-only positive programs observed, mandatory guard/diagnostic fixes still open |
| A102 literal bridge | `mori_backend::sherlock_machine::literal_host` and `literal_bridge` | Complete public statement for one charged literal transition; native entry points are separate from host carriers |
| A102→A80 composition | `literal_complete_execution`, `literal_complete_bridge` | Shared program and exactly shared intermediate image; terminal image remains noncanonical Step-mode |
| Ledger adapters | `mori_ledger::{sherlock_literal,sherlock_transaction,sherlock_literal_complete,sherlock_transaction_complete}`; [exports](../../../crates/mori-ledger/src/lib.rs#L18) | Separate literal and composed typed APIs; source does not establish native execution or settlement |
| DoctorWatson | [scripts/doctorwatson.sh](../../../scripts/doctorwatson.sh), `models/sherlock/qa/` | Default corpus/source QA; direct runner selects host/image/stream; shell flags remain `--host`/`--design` |

## Public compiler consumer

Core/1 CLI calls `compile_reported` and publishes `source-projection.json`, circuits and `artifact.json`. Static metadata does not change signed/unsigned compilation or implement Sherlock lowering. Core/2 uses its staged compiler. [Command](../../../crates/mori/src/commands/compile.rs#L18), [wrapper](../../../crates/mori-backend/src/source_projection.rs#L31).

The [design](../design.md) selects B HostOnly admission of parameter-free Core/1 scalar actions through this compile command. The first implementation now exists in source. Its [actual production-library result](../../results/sherlock-source-compile-20261017/README.md) records metadata/rlib compilation and seven positive external tests, including real preparation/admission of FixedQuote and FixedEligibility. The CLI binary and explicit observation branch remain unbuilt/unrun in that result; two mandatory source review corrections remain open. An independent frozen oracle also matched all twelve program fields for both cases and selected metadata; full artifact and runtime comparison remain open. The [new goal was armed](../README.md) after the spec and finite-model entry conditions closed.

## Nonpublic and proposed surfaces

`reference_slice` is a public host observation/grouping module. The adopted opt-in [resource manifest report API](resource-manifest-api.md) adds reporting and caller-owned JSON writing; its scoped results and explicit earlier-evidence applicability are recorded in the linked reference. `program_image`, `image_reference`, `machine_image`, `helper_schedule`, `helper_call_adapter`, `arithmetic_witness` remain test-only. Internal `pub` items are not runtime exports; `advance_m`/`ProgramImageDraft` define no adopted ABI. Historical comments cannot override registration or establish execution.

## Product scope

Full Sherlock v1, formal, ACTUS/DeFi, complete-effects, signed-intent, transition/history, residual-duty and target obligations remain required. Maintainer review/resource controls are not public compile/prove/deploy permissions. Private-fork work/data remain outside this corpus. See [product contract](../../MORIARTY-PRODUCT-CONTRACT.md) and [acceptance](../explanation/evidence-and-acceptance.md).
