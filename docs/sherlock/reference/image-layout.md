# Images and the canonical state boundary

## Public L4 diagnostic interface

`mori_core::sherlock::layout_a` publicly re-exports typed rows and these functions:

```rust
pub fn prepare_a(program: Arc<Program>, capacities: Capacities)
    -> Result<PreparedA, PrepareFailure>;
pub fn encode_observed_a(prepared: &PreparedA, caps: &StateCapacities,
    snapshot: &Snapshot) -> StateResult<StateBlock>;
pub fn decode_observed_a(prepared: &PreparedA, capacities: &StateCapacities,
    image: &AImage<'_>, mode: DecodeMode) -> StateResult<Snapshot>;
pub fn observed_state_bytes_a(prepared: &PreparedA,
    capacities: &StateCapacities, snapshot: &Snapshot) -> StateResult<Vec<u8>>;
```

Preparation's alias is expanded above. Sources: [prepare](../../../crates/mori-core/src/sherlock/layout_a/prepare.rs#L133), [encode](../../../crates/mori-core/src/sherlock/layout_a/state_encode.rs#L253), [decode](../../../crates/mori-core/src/sherlock/layout_a/state_decode.rs#L358), [bytes](../../../crates/mori-core/src/sherlock/layout_a/state_encoding.rs#L182).

`Capacities { tables: [u128; 15], data: u128 }` supplies program dimensions. `PreparedA` contains `block`, `capacities`, `full_program`, occurrences and names; it is mutable data, not a capability. Exact table ordering: [rows.rs:7](../../../crates/mori-core/src/sherlock/layout_a/rows.rs#L7).

`StateCapacities` supplies `data`, `data_fields`, `env_cells`, `value_slots`, `frame_slots`, `control_rows`, `stack_limit`; data capacity must match preparation. `StateBlock` contains a root and six corresponding fixed vectors; root lengths identify active rows, inactive rows remain. Fuel uses 16-byte little-endian lanes. `AImage` supplies `profile`, `full_program`, `program`, `state`. [Declarations](../../../crates/mori-core/src/sherlock/layout_a/state.rs#L10).

## Trust preconditions and refusals

`PreparedA` must be unchanged output of `prepare_a`; decoded state must come from encoding a trusted snapshot with the same preparation/capacities. Constructed/mutated records are not exhaustively validated: violations can panic, exhaust host resources or return `Ok` with unbound data. Caller capacities have no ceiling; tables are padded to capacity. [Contract](../../../crates/mori-core/src/sherlock/layout_a/mod.rs#L1).

`PrepareFailure` has `Capacity` and `HostUnrepresentable`. `StateFailure` has `Capacity`, `ProgramMismatch`, `OutsideProgramClosure`, `WrongRepresentation`, `OutsideObservationDomain`, `WrongProfile`, `HostUnrepresentable`. Refusals carry locators. These are tooling errors, not semantic outcomes or transitions. Exact fields: [limits.rs](../../../crates/mori-core/src/sherlock/layout_a/limits.rs#L4), [state.rs:119](../../../crates/mori-core/src/sherlock/layout_a/state.rs#L119).

`DecodeMode::Canonical` compares against complete re-encoding, including least-equal references, visitation order, dead rows and sharing history. `DecodeMode::Step` permits noncanonical in-step rows, including Constant-space references, sharing and non-least IDs. Neither mode's `Ok` establishes canonical admission, reachability, `StepInv`, F4/G4 correctness, transition acceptance or native acceptance. A decoded snapshot cannot be loaded into `Machine`.

## Other images

`ProgramImageDraft` and `ReferenceSlice` are test-only proposals. B's `shared::MachineImage` has private fields; `OwnerEngine` admits/steps/runs it and `observe_bound_image` borrows observations. No A state importer exists. [B observation](../../../crates/mori-core/src/sherlock/helper_machine/shared/owner_host.rs#L38).

The composed literal bridge's `intermediate` and `done` are Step-mode Constant-space images (`space == 0`, `activeData == 0`). Return publication does not perform `projectA` unfolding into canonical Data/DataFields. No native or canonical `projectA` result is supplied. See [bridges and ledger](bridges-ledger.md), [observation boundaries](../explanation/observation-boundaries.md) and [chunks and financial boundaries](../explanation/chunks-and-financial-boundaries.md).
