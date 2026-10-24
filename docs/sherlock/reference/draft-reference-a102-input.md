# Retained draft source and A102 input

The source package adds
`mori_core::sherlock_compile_artifact::draft_reference`:

```rust
pub fn compile_draft_with_reference(
    source: &str, config_source: &str, target_source: &str,
) -> Result<DraftReferencePreparation, CompileFailure>;

impl DraftReferencePreparation {
    pub fn preparation(&self) -> &DraftCompileArtifact;
    pub fn reference_program(&self) -> &Arc<Program>;
    pub fn observe_b_genesis(&self)
        -> Result<HostObservation<'_>, HostViolation>;
}
```

`Program` here is the public `sherlock::Program` A reference syntax, not Core/1's
program or a B image. Both carrier fields are private. The immutable report was
projected from the retained pair. The source Arc is exactly the parsed/typechecked
syntax passed to the existing bounded A-to-B copier. There is no B-to-A decoder,
second admission or source evaluation into a replacement literal.

The existing compile_draft_host signature and report behavior remain. Both public
functions now share private prepare_draft_pair and project_draft_pair, forwarding
all explicit parser/type/copy/layout/bounds/fuel/stack fields and preserving error
staging. The new function retains its pair; the existing function returns only its
report. Host H1, recursive Drop/report allocation and correspondence duties remain.

The [complete actual caller](../../results/sherlock-real-source-a102-input-20261019/source/consumer/real_source_a102_input.rs)
shows error-preserving preparation, immutable report access, actual B genesis
observation and Arc handoff to the existing checked A102 constructor. Its
[source](../../results/sherlock-real-source-a102-input-20261019/source/inputs/literal-seven.mori) is:

```mori
language "moriarty-sherlock-draft/1";
program LiteralSeven { entry 7; }
```

All B fields are explicit in the retained
[configuration](../../results/sherlock-real-source-a102-input-20261019/source/inputs/draft-config.json).
The separate A102 capacities are program
`[0,0,1,0,1,0,0,0,0,0,4,4,5,1,0]`, state `[0,0,0,0,0,1]`, capacity stack limit0,
initial fuel1, runtime stack0 and statement limit252. These are selected case inputs,
not defaults or a resource profile for arbitrary programs. The complete
[expected images and lanes](../../results/sherlock-real-source-a102-input-20261019/source/EXPECTED-A102-INPUT-AND-STATEMENT.json)
use decimal strings for UInt128 public statement values.

Observed ordinary validation passed new Core compilation, direct consumer compilation
and this one complete input comparison. The target descriptor explicitly selects
no network. The consumer directly compiled the pure literal_host/literal_witness
modules; SDK relation-source files were include_str data, not compiled SDK code.
[Receipts and scope](../../results/sherlock-real-source-a102-input-20261019/README.md)
retain source-review and execution roles. This is no proof/ledger/native claim.
