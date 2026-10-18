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
correspondence need their own observations. Final result review and original
UIREQ-022 adoption remain pending.

For the existing scalar Core1 path and its historical observed fixtures, use
[prepare a compile-only HostOnly artifact](prepare-host-source-artifact.md).


Original UIREQ-022 was subsequently adopted after corrected final result review.
See the [Root adoption](../../results/sherlock-public-draft-compiler-20261018/ROOT-ORIGINAL022-ADOPTION.json).
This accepts the original named-mapping report clause. All 26 semantic
correspondence obligations remain unproved; runtime, CLI, witness/constraint,
native and target evidence retain their separate requirements.
