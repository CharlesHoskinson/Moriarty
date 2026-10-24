# Compile dedicated draft fixtures to HostOnly artifacts

Use the public `compile_draft_host` API to prepare a closed draft program's
admitted B tables and genesis image. This guide uses two complete fixtures,
ChainTotal and BranchTotal, through the same entry point. The current library
and canonical external consumer compiled, and both public fixture tests passed;
see the [observed result](../../results/sherlock-public-draft-compiler-20261018/README.md).
The complete file-reading/file-writing example below is **UNRUN** and has no
execution receipt. The [reference](../reference/draft-source-compiler.md) describes the
supported constructs, exact config extents, failures and correspondence report.

## Select both fixture programs

Use a checkout containing the draft compiler source and
[draft-compile fixtures](../../../crates/mori-core/tests/fixtures/draft-compile/).
ChainTotal retains recursive list processing and a nested constructor call:

```mori
language "moriarty-sherlock-draft/1";
program ChainTotal {
  fn sum(xs: List): UInt {
    match xs { nil => 0, cons(h,t) => h + sum(t) }
  }
  entry sum(cons(2,cons(5,nil)));
}
```

BranchTotal uses the same pipeline with Tree constructors and two recursive
calls in its branch arm:

```mori
language "moriarty-sherlock-draft/1";
program BranchTotal {
  fn fold(t: Tree): UInt {
    match t { leaf(n) => n, branch(l,r) => fold(l) + fold(r) }
  }
  entry fold(branch(leaf(4),branch(leaf(1),leaf(3))));
}
```

Read the committed files for their exact bytes. Use the supplied
`sherlock-draft-host-config/1` config and target descriptor without substituting
the Core1 `sherlock-host-config/1` fixture. All 16 preprocessing extents are
explicit; the six additional copy extents are required too. Fixture values are
test inputs, not defaults or recommended settings. Target JSON labels this
HostOnly preparation; it provides no wallet, stagenet or ledger identity.

## Compile and serialize each result

In a Rust client depending on this compiler revision and `serde_json`, the
following complete example reads files relative to the repository root and
writes two distinct JSON artifacts into a new directory. It calls the actual
public producer, preserves a structured compilation failure, and does not run B
steps or substitute an evaluated result.

```rust
use mori_core::sherlock_compile_artifact::compile_draft_host;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixtures = "crates/mori-core/tests/fixtures/draft-compile";
    let config = std::fs::read_to_string(format!("{fixtures}/config.json"))?;
    let target = std::fs::read_to_string(format!("{fixtures}/target.json"))?;
    let mut outputs = Vec::new();

    for name in ["chain-total", "branch-total"] {
        let source = std::fs::read_to_string(format!("{fixtures}/{name}.mori"))?;
        let artifact = match compile_draft_host(&source, &config, &target) {
            Ok(artifact) => artifact,
            Err(failure) => {
                eprintln!("{}", serde_json::to_string_pretty(&failure)?);
                std::process::exit(1);
            }
        };
        // R256 checked the returned field; this complete I/O example is UNRUN.
        let profile = serde_json::to_value(&artifact.supported_profile)?;
        eprintln!("{name} supported profile: {}",
                  serde_json::to_string_pretty(&profile)?);
        outputs.push((name, serde_json::to_string_pretty(&artifact)?));
    }

    let out = "draft-host-artifacts";
    std::fs::create_dir(out)?;
    for (name, json) in outputs {
        let mut file = std::fs::File::create_new(format!("{out}/{name}.json"))?;
        std::io::Write::write_all(&mut file, json.as_bytes())?;
    }
    Ok(())
}
```

`create_dir` requires a new output directory and `create_new` refuses an existing
file. This example prepares and serializes both artifacts before reserving that
directory. An I/O failure can leave a partial directory; successful preparation
is distinct from file publication. Changing any source/config/target byte also
changes the corresponding digest.

## Inspect the supported-profile field

With the installed report field, borrow `artifact.supported_profile` from the
returned artifact as shown above. No argument is added to
`compile_draft_host(&source, &config, &target)`. The example prints the actual
returned report; no captured printout or successful run of this file-publication
example is claimed. R256's five phases and both exact report tests passed,
including the test that changes every bound and selected other extents. Normal
Core compilation and all 723 selected input pins are recorded in that result.
R257 approved the supplied source and exact plan. After withholding the
inaccurate R263 reply, Root adopted original UIREQ-002 following fresh R272
literal-result review. The two passing tests were not rerun. See the
[profile result and adoption](../../results/sherlock-supported-profile-20261018/README.md). Public developers
need no reviewer permission to inspect this returned report.

Inspect `format`, source identities and `execution_profile`, then `grammar`,
`numeric_domains`, `schedule`, `extents`, `admission_scope` and `dispositions`.
Compare all 22 `extents.bounds` fields, layout capacities, fuel/stack and all 16
preprocessing fields with the exact config supplied to this call. Consult the
[complete report reference](../reference/draft-source-compiler.md#supported-profile-report)
for the field inventory and its serialization/frame/work qualifications. Values
in the fixture are test inputs, not defaults. The report must describe this
invocation rather than a constant copy of the fixture's capacities.

Read the owner closed-body/unique-function and extension dispositions literally:
source typechecking is not a new whole-program owner admission checker, and
first declaration lookup is not unique-name admission. Tooling failures,
runtime limits and HostViolation remain distinct. A reported logical extent is
not physical H1, runtime, circuit or target evidence.

## Inspect the artifacts

Check each artifact's `format` is `sherlock-host-artifact/2`, selection is
`{ "kind": "entry" }`, and input policy is `closed-entry`. Inspect
`preparation.functions`, complete provenance and type summary, then
`admission.program_view` and `genesis`. Calls and list/tree constructors must
remain represented; a constant 7 or 8 is not a replacement for either program.

The `correspondence` object names all 26 construct obligations and their value,
failure, order, resource and finance mappings. Inspect `resources`, `finance`
and `proof` with those obligations: status `named-unproved` reports a pending
semantic obligation, not a discharged proof. A/B fuel/helper clocks remain
distinct. Finance effects and duties are inapplicable in this pure profile.

Both artifacts carry `observation.status: "not-requested"` with
`evaluation_performed: false`, host observation `genesis-only`, witness and
constraint plans `missing`, export `unsupported`, native/target `unrun` and
acceptance `unestablished`. The genesis image is not an observed execution or an
importable checkpoint. No circuit, proof, native verification or transaction is
produced.

## Understand the observed checks

The public tests use the same API and two fixtures. They compare the complete
independently sealed source/preparation/ProgramView expectations and the complete
separate correspondence reference contract. Both exact tests passed after the
normal Core and external consumer builds. The four sealed fields include full
source identity/preparation and all twelve program tables; the complete
26-obligation correspondence object and selected artifact metadata also match.
The original oracle remains unchanged; the correspondence reference data is
ordinary author data. This result exercises the public producer on two distinct
programs, without running B steps or the CLI and without running this guide’s
file publication example. Runtime results, failures and resource-prefix
correspondence need their own observations. Original UIREQ-022 is adopted; the
new supported-profile field has separate passing R256 observations, with
original UIREQ-002 reporting scope adopted after fresh R272 result review.

## Pending command shape: UNRUN

The installed simulate source is intended to call the runtime API and
print the separate [simulation report](../reference/draft-source-compiler.md#simulate-source-integration-observed-consumers-cli-unrun).
For a checkout containing that integration, the intended shape is:

```text
mori simulate SOURCE --execution-profile sherlock-helper-microstep/1 --source-profile moriarty-sherlock-draft/1 --entry --execution-config CONFIG --target TARGET
```

`SOURCE`, `CONFIG` and `TARGET` denote actual file paths. The current
[runtime fixtures](../../../crates/mori-core/tests/fixtures/draft-runtime-independent/)
are explicit specimen inputs, not numerical defaults. No output or successful
invocation of this command is shown: the enclosing public CLI/global dispatch
remains **UNRUN**. R322 separately passed a direct consumer of the actual selected
handler with production file reads and JSON export/printing, saving both complete
reports; this does not execute the enclosing command. The existing complete Rust
file-publication example above also remains unrun.

The installed command source selects a dedicated entry without action/scenario, resume,
trace, logical-cap or signed replay arguments. It prints preparation and actual
post-run execution separately and retains no event trace. Its source maps actual
Halted to exit zero and Rejected/Exhausted/Running to failure. Read the reference
for the complete report/failure contract. Command exit/printing behavior remains
source intent; report values/serialization have a separate consumer observation,
not a captured command printout. Developer use requires no maintainer approval.

R299 passed its seventeen-phase ordinary operation and one Rust test over 24
frozen shared production argument/selection rows, with 1,611 unchanged input
pins. R301 separately compiled the normal Core report composition at exit zero
with 559 unchanged pins and nine warnings. Root preserves its original dep-info
FAIL_STOP record and additive hash/normalized-alias correction; no compilation
was rerun. R300 source/operation review and R311 result/source review returned
SUCCESS, and Root installed the fourteen source files. Neither observed consumer
runs this command, global options or files.

R310 separately passed consumer compilation, listing and one test looping two
programs through real run → report → serialized bytes → JSON. All 457 pins were
unchanged. Inspected assertions check independent UInt 7/8/source identities and
same-run preparation/post-run summary/image/resource bindings; raw stdout contains
test status, not a saved report or trace. R315 completed successfully; Root
installed all six consumer files, verified the combined twenty-file source/test
set and applied reviewed reporting compatibility. The existing reader is now
CURRENT11/35 with no findings; the earlier CURRENT10/stale025 negative is retained.
R322 subsequently passed four phases covering the actual selected handler's
production Args, file reads, JSON export/printing and a saved-files-only verifier.
Both complete reports are persisted (ChainTotal 59,814 bytes; BranchTotal 64,160),
with independent UInt 7/8 and declared bindings checked under frozen R324 predicates.
Full root CLI/global dispatch and file publication remain UNRUN; B04 stays
unchecked, with no exact B-clock/full-trace or new requirement closure. See the [source and receipt archive](../../results/sherlock-simulate-report-20261018/README.md);
no command output is fabricated here.

## Inspect the observed public runtime cases

The separate [public runtime API](../reference/draft-source-compiler.md#public-draft-runtime-api)
is installed. Its [independent public test consumer](../../../crates/mori-core/tests/sherlock_draft_runtime_independent.rs)
uses the exact [runtime fixtures](../../../crates/mori-core/tests/fixtures/draft-runtime-independent/),
which retain fuel 4096 and stack 64 and use value-slot capacity 4096. These are
explicit specimen inputs, not defaults. They differ from this guide's compile-only
fixture. `run_draft_host` takes source/config/target plus a callback for full
`StepEventC` events; it runs the same admitted pair rather than compiling a second
pair or replacing source calls with an evaluated literal.

The [adopted R279 result](../../results/sherlock-public-runtime-20261018/README.md)
records consumer compilation, listing and both exact tests passing, with 636
selected pins unchanged and the existing Core build reused. The inspected tests
assert actual UInt 7/8, terminal state, recursive-call order/counts and relational
counter/event/resource conservation. Raw stdout contains test status, not a
printed value trace or exact B-clock totals. Full payload prefixes and independent
exact B-clock evidence remain absent. The retained earlier capacity refusal and
listing correction are part of that archive.

This compile-only guide still does not run the runtime API or its file-publication
example. Inspect `completed`/`observe()` for post-run data; `preparation` remains
the unstepped snapshot. An `Ok` run wrapper can contain Rejected/Exhausted as well
as Halted. Public developer use does not require maintainer review permission.
The installed API and two passing host cases add no CLI, general H1, proof,
circuit, native or full-language acceptance claim.

For the existing scalar Core1 path and its historical observed fixtures, use
[prepare a compile-only HostOnly artifact](prepare-host-source-artifact.md).


Original UIREQ-022 was subsequently adopted after corrected final result review.
See the [Root adoption](../../results/sherlock-public-draft-compiler-20261018/ROOT-ORIGINAL022-ADOPTION.json).
This accepts the original named-mapping report clause. All 26 semantic
correspondence obligations remain unproved; runtime, CLI, witness/constraint,
native and target evidence retain their separate requirements.
