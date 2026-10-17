# Prepare a compile-only HostOnly artifact

Use this when you want the current library to parse one parameter-free scalar Core/1 action and inspect the actual admitted B program tables. It prepares an artifact without running B steps. The library path has actual fixed-input results; the CLI binary remains unbuilt and the observation branch unexecuted. Mandatory R27 source corrections remain pending at this evidence cut. See the [scope and receipts](../../results/sherlock-source-compile-20261017/).

## Select the public inputs

From a checkout containing this source revision, use the exact committed [config fixture](../../../crates/mori-core/tests/sherlock_compile_artifact_positive/config.json) and [FixedQuote source](../../../crates/mori-core/tests/sherlock_compile_artifact_positive/fixed-quote.mori). The latter is:

```mori
language "moriarty/1";
agreement FixedQuote {
  action quote() {
    let base = 7 * 11;
    let total = base + 5;
    emit "quoted"(total);
  }
}
```

Keep the lets/operators. There are no action arguments or scenario inputs on this route. The supplied emit is a scalar value projection, not a financial effect. The [configuration reference](../reference/host-source-config.md) contains the complete fixture and explains which values are exact specimen choices.

## Call the library with observation disabled

In a Rust client already depending on this revision of `mori-core`, the following body reads those files relative to the checkout root and calls the same public entry point used by the recorded capture:

```rust
use mori_core::sherlock_compile_artifact::compile_host;

fn main() {
    let source = std::fs::read_to_string(
        "crates/mori-core/tests/sherlock_compile_artifact_positive/fixed-quote.mori",
    ).expect("read the fixed source");
    let config = std::fs::read_to_string(
        "crates/mori-core/tests/sherlock_compile_artifact_positive/config.json",
    ).expect("read the exact config fixture");
    let target = "{\"kind\":\"public-host-test-descriptor\"}\n";

    match compile_host(&source, "quote", &config, target, false) {
        Ok(artifact) => println!("{}",
            mori_core::json::export(&artifact).expect("serialize artifact")),
        Err(failure) => {
            eprintln!("{}",
                mori_core::json::export(&failure).expect("serialize failure"));
            std::process::exit(1);
        }
    }
}
```

This is a relative-path adaptation of the actual capture's API call, not an independently executed documentation example. It creates no output directory, contract or transaction. Save successful stdout as JSON if you want a local artifact file. Changing fixture/config/target bytes changes their recorded identities.

## Inspect the result

Check `format == "sherlock-host-artifact/1"`, `execution_profile == "sherlock-helper-microstep/1"`, and `admission.program_view`. Preparation retains source occurrence information; admission contains the actual program view and owner identity/layout. `observation` is `not-requested` with `evaluation_performed: false`; it is not a completed runtime summary. Witness/constraint plans are not implemented and native export is unsupported. The [artifact reference](../reference/host-source-compiler.md) lists every top-level field.

To inspect the second recorded shape, load [fixed-eligibility.mori](../../../crates/mori-core/tests/sherlock_compile_artifact_positive/fixed-eligibility.mori) and select action `inspect`, keeping the same config/target and `false`. It retains comparison/boolean structure. Its generated hygienic names are compared only through the published explicit bijection; arbitrary table renaming is not part of the artifact contract.

## Consult the independent comparisons

The archived actual [Quote](../../results/sherlock-source-compile-20261017/actual-capture/quote.stdout) and [Eligibility](../../results/sherlock-source-compile-20261017/actual-capture/eligibility.stdout) artifacts have selected-field matches. Their [comparison result](../../results/sherlock-source-compile-20261017/actual-capture/COMPARISON-RESULT.json) deliberately keeps `comparison_complete=false`. All twelve ProgramView fields and selected identity/envelope values match; full preparation/admission wrapper fields and runtime/event/image outcomes are not validated by that result.

For the documented read-only replay of those exact archived comparisons, use the two commands in the [published package](../../results/sherlock-source-compile-20261017/#independent-actual-program-table-comparison). They compare saved JSON and do not compile or execute Moriarty. A different source or config needs its own independent expectations; the sealed comparator is specific to these two specimens.

General ADTs, recursion, financial effects, persistent resumes, full K/Quint/Lean-Clean correspondence, witness/constraints, native proof/verification and stagenet remain separate implementation and acceptance work. This compile-only artifact is not a checkpoint or installable contract.
