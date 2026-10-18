Fresh nonauthor ordinary source and actual-result review. Read PRIMARY and needed physical context. All three changed bodies and complete current checker wrapper/helper/module are inline; other direct/doc contexts have declared full/ranged/pointer coverage. Separate sourceAssessment/actualResultAssessment and actual reads/gaps. No execution, writes, provider calls, security/native judgment or parent closure. Return one complete OUTPUT-SCHEMA object; substantive extras and unbounded reasons allowed, no forced verdict or empty findings.

# Fresh exact ordinary CLI profile-guidance source/result review

Head f2975f1fbfbaf060207a9d26331ad52745ce5247; tree 3db626c5806d44d7f0a2bf49936d146d66549196; base 8269b5b7ceeec5169c706aa31ea33f390d49c969. Subject SHA256 83218395f6224fce90a3f08d393887edc1ddbe6073e69ec8420f915c0d060f72; review token `doctorwatson-cli-profile-guidance-main8269-source-result-ordinary-20261018-a1-whole-review`. Full actual three changed bodies below. Ordinary source and result review only; no approval supplied, forced empty findings, security/native admission or parent closure. Historical packet remains unchanged.

````json
{"scope":"Ordinary public CLI profile help and source-derived walkthrough; exact three changed bodies on actual main8269 integration f297, no behavior/profile/feature/native-gate change.","sourcePredicates":["Compile/simulate help names actual Core1/Core2 headers and keeps beta/Source6 input boundaries explicit.","Default expand remains Core1; opt-in source6 flag describes beta transfer proposal only and requires source6-expansion.","Guide command spellings, profiles, flags, reported shapes, links and caveats agree with actual current consumers. PR227 now routes check.rs through check_source::dispatch; inspect both complete bodies and module mapping. Applicable CheckReport is unchanged.","All examples retain their local qualifications, raw Source6/Node Core5 distinction, explicit Target/native requirements, and no proof/ledger implication.","Only help metadata/unit regression and doc guidance change; older full guide content is supplied for context, not renewed certification."],"resultPredicates":["One selected Clap unit passed with seven other unit tests filtered. No whole-suite/native/Source6 runtime claim.","Five exact shell blocks passed bash -n with empty output; none of their product commands ran.","Source/result/context pin coverage and received output/cleanup receipts agree; limitations or missing evidence must be reported."],"exposure":"Preparer has prior source exposure and is preparing input only. Doc and Clap expectations are source-derived; no new independent financial oracle or fresh reviewer vote is claimed.","preserve":"All original U0\u2013U7/M1\u2013M17/MC/SP/G/148requirements/292scenarios/20families/financial/composition/native/target and security ownership limits.","history":"Historical d7a packet and receipts remain immutable. No historical approval/result is transferred to f297; this packet uses new exact f297 terminal receipts."}
````

## Exact changed bodies and direct consumer context

All three changed bodies are complete. All 37 prepared-context paths are now present. Actual check.rs, shared check_source.rs and commands/mod.rs are complete below, refreshed at f297 after PR227. The CheckReport definitions are byte-identical to the historical d7a source. Other contexts retain exact quote ranges/pointers, with fresh whole-body Git/SHA pins; all 40 full physical bodies are available. No whole model read is inferred from file presence.

### C01 crates/mori/src/cli.rs — complete-changed-body

Whole SHA256 a1046abb0191c29bbc5b10e169ba3ce81a4b71324a48246d0abde34eb81c3988.

````text
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::commands::{
    capabilities, check, compile, compile_call, expand, format, init, inspect, intent, keygen_call,
    keygen_signed_call, lsp, mcp, prepare_call, prepare_signed_call, prove, prove_call,
    prove_signed_call, replay, simulate, test, verify, verify_call, verify_intent,
    verify_signed_call,
};
use crate::shell::Verbosity;
use crate::styles;

/// Moriarty: a financial agreement language for Midnight.
#[derive(Debug, Parser)]
#[command(name = "mori", version, styles = styles::CLAP)]
#[cfg_attr(
    all(not(feature = "native"), not(feature = "source6-expansion")),
    command(
        after_help = "Built without native: local Core reference tooling only. Midnight compile, call, proof, targeted inspect, trace/replay and JubJub profiles require the native feature."
    )
)]
#[cfg_attr(
    all(not(feature = "native"), feature = "source6-expansion"),
    command(
        after_help = "Built without native: local Core reference tooling and explicit Source/6 transfer expansion only. Midnight compile, call, proof, targeted inspect, trace/replay and JubJub profiles require the native feature."
    )
)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalArgs,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Args)]
#[command(next_help_heading = "Global Options")]
pub struct GlobalArgs {
    /// Print no status output
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,

    /// Print extra status output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// When to use color
    #[arg(long, global = true, value_name = "WHEN", default_value = "auto")]
    pub color: Color,
}

impl GlobalArgs {
    pub fn verbosity(&self) -> Verbosity {
        if self.quiet {
            Verbosity::Quiet
        } else if self.verbose {
            Verbosity::Verbose
        } else {
            Verbosity::Normal
        }
    }
}

/// Selects one action of an agreement and the scenario to run it against.
#[derive(Debug, Args)]
pub struct ActionInput {
    /// Source file
    pub file: PathBuf,

    /// Action to run
    #[arg(long, value_name = "NAME")]
    pub action: String,

    /// Scenario JSON with the local state and inputs
    #[arg(long, value_name = "FILE")]
    pub scenario: PathBuf,
}

/// Output format for the signing commands.
#[derive(Debug, Args)]
pub struct SigningOutputArgs {
    /// Print a human review of what is being signed or verified
    #[arg(long, conflicts_with = "json")]
    pub review: bool,

    /// Print the result as JSON
    #[arg(long)]
    pub json: bool,
}

/// Signed native owner-intent profile selected explicitly by name.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SignedProfile {
    #[value(name = "moriarty-jubjub-owner-intent/1")]
    JubjubOwnerIntent,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Color {
    Auto,
    Always,
    Never,
}

impl From<Color> for anstream::ColorChoice {
    fn from(color: Color) -> Self {
        match color {
            Color::Auto => Self::Auto,
            Color::Always => Self::Always,
            Color::Never => Self::Never,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Describe compiled features and public source consumers without running them
    Capabilities(capabilities::Args),
    /// Create a new project from a starter template
    Init(init::Args),
    /// Check syntax, names, types and action support
    Check(check::Args),
    /// Compile supported Core/1 or Core/2 source into ledger-9 ZKIRv3 circuits
    ///
    /// Use a language "moriarty/1"; or language "moriarty/2"; source header.
    /// Beta/1 and Source/6 text are not compiler inputs.
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    Compile(compile::Args),
    /// Compile a source-static ordinary native contract-call relation
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    CompileCall(compile_call::Args),
    /// Derive all source action keys and an explicit local installed snapshot
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    KeygenCall(keygen_call::Args),
    /// Derive the complete local signed action map under a fixed JubJub policy
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    KeygenSignedCall(keygen_signed_call::Args),
    /// Reconstruct a signature-free canonical local call signing package
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    PrepareSignedCall(prepare_signed_call::Args),
    /// Prove the actual local typed SDK call with an external FENS attachment
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    ProveSignedCall(prove_signed_call::Args),
    /// Cold public-only reconstruction and Real verification of a signed call
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    VerifySignedCall(verify_signed_call::Args),
    /// Cold rederive keys and prove an actual local installed SDK call
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    ProveCall(prove_call::Args),
    /// Cold rederive public keys and Real-verify an installed call without PK files
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    VerifyCall(verify_call::Args),
    /// Prove a public local action using pinned offline parameters
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    Prove(prove::Args),
    /// Prepare an unsigned, unproven local native call and public offer
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    PrepareCall(prepare_call::Args),
    /// Verify a native proof against the expected source, target and scenario
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    Verify(verify::Args),
    /// Format source files in place
    #[command(visible_alias = "fmt")]
    Format(format::Args),
    /// Show identity claims, operative bounds and open premises
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Without native, source-only inspection is available; --target requires the native feature."
        )
    )]
    Inspect(inspect::Args),
    /// Show Core/1 IR or explicitly emit beta Source/6 transfer text
    ///
    /// Default expansion accepts moriarty/1 Core source. The explicit
    /// --emit-source6-transfer option emits a local beta transfer proposal.
    Expand(expand::Args),
    /// Run a Core/1 action or Core/2 stage locally against a scenario
    ///
    /// Use a language "moriarty/1"; or language "moriarty/2"; source header.
    /// moriarty-beta/1 and Source/6 text are not simulation inputs.
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Without native, local Core simulation and staged start/resume are available; --trace requires the native feature."
        )
    )]
    Simulate(simulate::Args),
    /// Independently replay a public record against separately trusted inputs
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    Replay(replay::Args),
    /// Run the test cases in a project directory
    Test(test::Args),
    /// Produce the owner signing bytes for an action
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Without native, legacy raw Core intents are available; the JubJub --profile requires the native feature."
        )
    )]
    Intent(intent::Args),
    /// Verify a signature, then prepare the action locally
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Without native, legacy Core signature/reference verification is available; the JubJub --profile requires the native feature."
        )
    )]
    VerifyIntent(verify_intent::Args),
    /// Run the language server over stdio
    Lsp(lsp::Args),
    /// Run the read-only MCP server over stdio
    Mcp(mcp::Args),
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::CommandFactory;

    #[test]
    fn help_distinguishes_core_consumers_from_source6_proposals() {
        let command = Cli::command();
        let compile = command.find_subcommand("compile").unwrap();
        let compile_help = compile.get_long_about().unwrap().to_string();
        for header in [r#"language "moriarty/1";"#, r#"language "moriarty/2";"#] {
            assert!(compile_help.contains(header), "{compile_help}");
        }
        assert!(
            compile_help.contains("Beta/1 and Source/6 text are not compiler inputs."),
            "{compile_help}"
        );

        let simulate = command.find_subcommand("simulate").unwrap();
        let simulate_help = simulate.get_long_about().unwrap().to_string();
        for profile in ["moriarty/1", "moriarty/2"] {
            assert!(simulate_help.contains(profile), "{simulate_help}");
        }
        assert!(
            simulate_help.contains("moriarty-beta/1 and Source/6 text are not simulation inputs."),
            "{simulate_help}"
        );

        let expand = command.find_subcommand("expand").unwrap();
        let expand_help = expand.get_long_about().unwrap().to_string();
        assert!(expand_help.contains("moriarty/1"), "{expand_help}");
        let source6 = expand
            .get_arguments()
            .find(|argument| argument.get_id().as_str() == "emit_source6_transfer")
            .unwrap();
        let source6_help = source6.get_long_help().unwrap().to_string();
        for boundary in [
            "moriarty-beta/1",
            "local-stipulation-only",
            "Source/6 output is not compiled or simulated by this option.",
        ] {
            assert!(source6_help.contains(boundary), "{source6_help}");
        }
    }
}
````

### C02 crates/mori/src/commands/expand.rs — complete-changed-body

Whole SHA256 2684b2251a90fda63cd4dd2b79d4809d9bde3545461d2b100295889eac73ba11.

````text
use std::process::ExitCode;

use crate::cli::ActionInput;
use crate::shell::Shell;

#[cfg(feature = "source6-expansion")]
mod source6;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: ActionInput,

    /// Emit a local Source/6 transfer proposal; requires source6-expansion
    #[arg(
        long,
        long_help = "Emit raw local Source/6 transfer proposal text from checked moriarty-beta/1 source. Requires the source6-expansion build feature. Output is local-stipulation-only; default Core expansion is unchanged. Source/6 output is not compiled or simulated by this option."
    )]
    pub emit_source6_transfer: bool,
}

pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    if args.emit_source6_transfer {
        return run_source6(args.input, shell);
    }
    let (_, program) = super::core_io::program(&args.input.file)?;
    // Validate the selected scenario/action without discarding the generic core.
    let scenario = super::core_io::read_json(&args.input.scenario)?;
    mori_core::simulate(&program, &args.input.action, &scenario)
        .map_err(super::core_io::operation)?;
    super::core_io::print(&program)?;
    Ok(ExitCode::SUCCESS)
}

#[cfg(feature = "source6-expansion")]
fn run_source6(input: ActionInput, shell: &mut Shell) -> super::Result<ExitCode> {
    source6::run(input, shell)
}

#[cfg(not(feature = "source6-expansion"))]
fn run_source6(_input: ActionInput, _shell: &mut Shell) -> super::Result<ExitCode> {
    Err(crate::error::CliError::Operation {
        code: "mori::command::source6-unavailable".into(),
        message: "Source/6 transfer expansion requires the source6-expansion build feature.".into(),
    }
    .into())
}
````

### C03 docs/NATIVE-CORE.md — complete-changed-body

Whole SHA256 8088e2b009f8e6fc4f98f129f1e499740bc36c4cec8d06fe0fe6ebfceeadeddb.

````text
# Native Moriarty language and commands

The native `language "moriarty/1";` profile adds composable bounded action
bodies. It uses the existing lexer, a flat checked expression arena, a
reference interpreter and a direct ledger-9 ZKIRv3 lowerer. The beta/1
frontend and TypeScript package retain their existing behavior.

Run this guide's Cargo commands from the Moriarty repository root. The explicit
`cargo run -p mori --locked --` prefix selects the Rust workspace CLI; the Node
`@moriarty-lang/beta` package also installs a command named `mori`, but accepts a
separate beta source profile and supplies no Rust compiler. The Rust package
uses its default `native` feature for the native command examples below.
The profile walkthrough disables it: Core reference tooling and beta source
checking remain available; explicit Source/6 expansion additionally requires
`source6-expansion`. Midnight compile/call/proof, targeted inspection,
trace/replay and JubJub profiles require `native`. This checkout
workflow does not establish a standalone Rust installation or completed external
proof/submission qualification.

## Choose a source profile before running a command

From the repository root, query the Rust CLI without the native feature:

```sh
cargo run -p mori --locked --no-default-features -- capabilities --json
```

`capabilities` also prints JSON without `--json`. The report's `format` is
`moriarty-cli-capabilities/1`; `producer.buildFeatures.native` is `false` for
this command. Select a consumer row by its `id`, then read its `inputProfile`,
`outputProfile`, `availableInBuild` and `limitations`. `availableInBuild` means
that a source path is compiled in. `catalogue.sourceRevision` records the
curated inspection basis; it is not the running binary's revision. Target
qualification is reported separately. See the [query source](../crates/mori/src/commands/capabilities.rs)
and [public consumer tests](../crates/mori/tests/capabilities.rs).

| Source header | Rust consumer | Result to expect |
| --- | --- | --- |
| `language "moriarty/1";` | `check`, `simulate` | Core/1 source checking and local reference execution |
| `language "moriarty/2";` | `check`, `simulate --action …`, `simulate --resume … --event …` | Core/2 source checking and local reference stages |
| `profile "moriarty-beta/1";` | `check`; opt-in `expand --emit-source6-transfer` for the supported transfer subset | Frontend checking; raw Source/6 proposal text from explicit expansion |

For a complete Core/1 output example:

```sh
cargo run -p mori --locked --no-default-features -- check crates/fixtures/core-report-contract/counter.mori --json
cargo run -p mori --locked --no-default-features -- simulate crates/fixtures/core-report-contract/counter.mori \
  --action run --scenario crates/fixtures/core-report-contract/counter-amount7.scenario.json
```

The check returns `status: "accepted"` and `language: "moriarty/1"`.
Simulation returns `status: "accepted"`, `qualification: "local-reference-execution"`
and `execution.post.state.count: {"type":"UInt","value":"7"}`. It publishes
both the state change and the `count` event. The [complete expected stdout](../crates/fixtures/core-report-contract/counter.unmetered.stdout.json)
and [real CLI assertions](../crates/mori/tests/core_report_contract.rs) include
all steps, effects and pre/post fields.

For Core/2, start the shipped delivery example and save only its continuation
object for resume. These commands use Python 3 for JSON extraction and a new
scratch directory for output files:

```sh
scratch=$(mktemp -d)
cargo run -p mori --locked --no-default-features -- check examples/core/continuations/delivery.mori --json
cargo run -p mori --locked --no-default-features -- simulate examples/core/continuations/delivery.mori \
  --action deliver --scenario examples/core/continuations/delivery-scenario.json --logical-cap 11 > "$scratch/start.json"
python3 -c 'import json,sys; json.dump(json.load(sys.stdin)["continuation"],sys.stdout)' \
  < "$scratch/start.json" > "$scratch/continuation.json"
cargo run -p mori --locked --no-default-features -- simulate examples/core/continuations/delivery.mori \
  --resume "$scratch/continuation.json" --event examples/core/continuations/delivery-true-resume-complete.json
```

The start result has `status: "waiting"`, `committed: "5"`, 10 units in
escrow and a 10-unit `release` residual duty. The true event produces
`status: "complete"`, `committed: "11"` and `residual: []`; bob receives 10,
escrow becomes 0, and the liability remains present with amount 0. Both results
retain `qualification: "local-reference-stage"`, `authentication: "none"` and
`consumption: "untracked"`. See the [delivery source](../examples/core/continuations/delivery.mori),
[normal CLI assertions](../crates/mori/tests/core_continuations.rs) and
[continuation guide](CORE-CONTINUATIONS.md) for complete frames and local limits.

For Rust beta transfer expansion, enable its separate feature explicitly:

```sh
cargo run -p mori --locked --no-default-features --features source6-expansion -- capabilities --json
cargo run -p mori --locked --no-default-features --features source6-expansion -- check crates/mori/tests/fixtures/source6-expansion/invoice.mori --json
cargo run -p mori --locked --no-default-features --features source6-expansion -- --quiet expand crates/mori/tests/fixtures/source6-expansion/invoice.mori \
  --action pay --scenario crates/mori/tests/fixtures/source6-expansion/invoice.scenario.json --emit-source6-transfer
```

The `beta-to-source6-transfer` consumer is available in this build. The check
returns `status: "accepted"` and action support `local_s0`. Expansion prints
[these exact Source/6 fixture bytes](../crates/mori/tests/fixtures/source6-expansion/invoice.source6),
with header `profile "moriarty-financial-agreement-source/6";`. The CLI's
`--quiet` suppresses its expansion status; Cargo may still print build messages
to stderr. The [CLI tests](../crates/mori/tests/source6_expand.rs) assert the
complete text. This option emits a local transfer proposal; Rust `simulate`
and `compile` do not accept that Source/6 text as input.

The separate Node beta executable performs local Core/5 preparation through
its own `simulate` command. With Node 24+ and the package built as described in
[getting started](../packages/moriarty-beta/GETTING-STARTED.md), run from the
repository root:

```sh
node packages/moriarty-beta/dist/cli.js check packages/moriarty-beta/examples/local/repay/repayment.mori --json
node packages/moriarty-beta/dist/cli.js simulate packages/moriarty-beta/examples/local/repay/repayment.mori \
  --action repay_loan --scenario packages/moriarty-beta/examples/local/repay/scenario.json
node packages/moriarty-beta/dist/cli.js test packages/moriarty-beta/examples/local/repay
```

These return `AuthoringChecked`, `PreparedUnqualified` and `TestsPassed`,
respectively. The [repayment manifest](../packages/moriarty-beta/examples/local/repay/mori.tests.json)
checks complete ordered effects and post-state: payment 3000 clears accrued
interest 1000, reduces principal to 98000 and leaves the payer with 197000.
See the [Node CLI tests](../packages/moriarty-beta/tests/cli.test.mjs) and
[preparation implementation](../packages/moriarty-beta/src/bridge.ts).

Compilation is a separate [Rust consumer](../crates/mori/src/commands/compile.rs)
requiring `native`, an admitted Core source and an explicit target descriptor.
Core/1 produces artifact/3; Core/2
uses the distinct staged artifact/4 described in the continuation guide.
Source checking, reference simulation, Source/6 emission and Node preparation
retain their own result qualifications; none establishes proof or ledger
acceptance.

## Native command examples

```sh
cargo run -p mori --locked -- check examples/core/language/counter.mori --json
cargo run -p mori --locked -- simulate examples/core/language/counter.mori \
  --action increment --scenario examples/core/language/counter.scenario.json
cargo run -p mori --locked -- test examples/core/batch-clear
cargo run -p mori --locked -- compile examples/core/batch-clear/batch.mori \
  --target toolchains/ledger-9.json --out build-batch
```

`cargo run -p mori --locked -- init NEW_DIR` creates a Core transfer starter. `--template repay`
creates a repayment starter. Before running the generated README commands from
its project directory, replace their `mori` executable with an absolute path to
the Rust binary built from this checkout. Those commands check, simulate and
test the complete expected post-state and ordered effects.
The destination must not exist, and its parent must exist. An I/O failure
can leave an incomplete project. Choose a new destination before retrying.
These starters use local example assets and reference execution.

`check` reads at most 1 MiB plus one refusal byte through the shared bounded
reader; both Core and beta retain their 65,536-byte source grammar limit.
Human diagnostic filenames use the existing escaped display policy, including
non-ASCII characters. Filesystem paths, raw source hashes and decoded JSON
filename identities remain exact.

Compilation does not need a scenario. The output directory must be new.
Circuit files become visible individually with complete bytes and refuse
existing or raced leaf paths. `artifact.json` is published last, after every
circuit succeeds. An I/O failure can leave the reserved directory and earlier
complete circuits; retry with a new destination. Publication assumes stable
output directories and path ancestors.
Each action produces a genuine upstream-readable `.zkir` circuit; the
manifest contains the canonical program identity, target, public ABI,
ordered effect and touched balance frames, instruction count and artifact
bytes. This relation constrains public state updates, inputs, guards,
transfers, fees and events. Public values are bound through explicit Impact
instructions. UInt multiplication uses limbs to reject field wrap.

`toolchains/ledger-9.json` pins the compiler libraries to ledger revision
`04c9c5d9bcebb8d4427d8589fb54d58a55599c14` and ZKIR `3.0.0-rc.2`.
Its local network/anchor are an explicitly unqualified development example.
Supply an exact target JSON with the actual network/anchor for another
installation. Unsupported revisions and cross-network assets refuse.
Artifact bytes are compiler output size; they are not encoded transaction
size. Compilation does not execute a prover or certify costs. Proof receipts
report the circuit k actually executed; fraud k, replay timing,
ledger frame/phase correspondence and deployment remain open.

The [language reference](../crates/mori-core/README.md) defines the grammar
and exact local semantics. States and inputs are UInt or Bool; financial
amounts are checked u128 atomic units. Asset identity carries network,
issuer, reference and scale. Scenarios must provide every touched balance,
including zero entries. Transfers and fees preserve per-asset balances.
Liability creation and explicit partial discharge compile under the closed
source identity registry described below and run in the reference interpreter.
No loops, branch statements, division, records or dynamic account selection are admitted.
The native core profile does not yet use the beta formatter or editor.

`simulate` returns ordered steps, complete effects and canonical pre/post
snapshots. A refused action commits no effects. The library's `replay`
recomputes each step and reports the first divergence. This is a reference
replay API; no external fraud-game implementation is included.

## Target-bound public reference replay

`cargo run -p mori --locked -- simulate --trace --target` produces a separate `moriarty-reference-replay/1`
sidecar. `replay` independently reconstructs it from separately supplied raw
source, action, Scenario and trusted Target. For signed Targets, both commands
require the complete profile, expected-authority, logical-time and actual
signature group. The record never supplies those trusted inputs.

```sh
cargo run -p mori --locked -- simulate deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.mori \
  --action settle \
  --scenario deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.scenario.json \
  --target deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.target.json \
  --profile moriarty-jubjub-owner-intent/1 \
  --authority deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.authority.json \
  --at 15 --signature deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.signed.json \
  --trace > payment.replay.json
cargo run -p mori --locked -- replay deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.mori \
  --action settle \
  --scenario deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.scenario.json \
  --target deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.target.json \
  --profile moriarty-jubjub-owner-intent/1 \
  --authority deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.authority.json \
  --at 15 --signature deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.signed.json \
  --record payment.replay.json
```

Trace stdout is the entire valid record. Shell redirection controls its destination
and can overwrite a file. Existing ordinary `simulate` output remains unchanged.
Replay succeeds with `qualification: local-reference-replay`; divergence exits1
with the first step ordinal and an RFC6901 field such as
`/body/execution/steps/1/effects/0/amount`. File/schema/argument errors use the
existing stderr diagnostics. Source or execution refusal publishes no record or
proposed financial effects.

The sidecar binds unchanged full Execution and revision3 Claim bytes, actual
selected ZKIR, source/program/Scenario/Target identities, network/Anchor and
signed public authority/time. It includes every ordered step and effect, full
pre/post and untouched/present-zero duties. Semantic comparison precedes mutable
execution/message/public-value/trace/record hashes, so rehashing false content
cannot hide the first difference. Signed replay validates the separately supplied
attachment; valid alternative attachments for one payload produce the same
public record. The record contains no attachment R/s, private witness or proof keys.

Every read and final ASCII-safe export respects Core's1MiB/depth32/nodes65536
profile, including the print newline. Oversized records refuse intact.
`source_step_count` counts completed source statements only, including guards and
local bindings; it measures no expression visits, curve work, time or native cost.
The [design](superpowers/specs/2026-10-11-bound-replay-design.md) defines exact hash
framing and comparison order; the [local result](evidence/2026-10-11-bound-replay.md)
records independent oracles and compatibility checks. Complete Fenrir fraud-game
interfaces, ledger authentication/history/time/nonce consumption, transaction
size, replay timing/work, DUST and live qualification remain open.

## Native public transition proofs

```sh
cargo run -p mori --locked -- prove examples/core/native-payment/payment.mori --action run \
  --scenario examples/core/native-payment/scenario.json \
  --target toolchains/ledger-9.json --params-dir /path/to/pinned/offline-srs \
  --max-k 10 --out payment-proof.json
cargo run -p mori --locked -- verify examples/core/native-payment/payment.mori --action run \
  --scenario examples/core/native-payment/scenario.json \
  --target toolchains/ledger-9.json --params-dir /path/to/pinned/offline-srs \
  --max-k 10 --proof payment-proof.json
```

The current public local UInt/Bool, transfer, fee, state, event and registered
liability profile connects to the pinned native prover and verifier. Earlier
liability-free native experiments used revision 1; they do not demonstrate a
revision 2 liability proof roundtrip. Both commands print a receipt with complete
local execution, steps, effects, prestate and poststate. `prove` genuinely
self-verifies before publishing a new bundle. Existing output files and
symlinks refuse; publication atomically preserves a concurrently created
destination. `verify` is read-only and derives its own verifier key from
recompiled source and trusted parameters. A supplied key or status flag does
not provide verification authority.

The proof commits to original source bytes, canonical program, exact target,
scenario, full execution, action and actual circuit k. Hashing uses SHA-256;
the domain-separated claim digest is interpreted little-endian and reduced
into the pinned field, without truncating to 128 bits. The native statement
also includes every trusted compiled ABI value. Changed context cannot be
repaired by relabelling bundle metadata. Empty, oversized or noncanonical hex
refuses; the pinned native verifier rejects trailing proof bytes.

Parameters come exclusively from the caller's explicit directory, named
`bls_midnight_2p{k}`. A bounded immutable snapshot must match the exact upstream
SHA-256 before decoding. There is no download or parameter search. The default
caller cap is k9 and the maximum is k19; measured degree is checked before
parameter loading. The payment example needs k10. These objective public tool
bounds are independent of maintainer experiment authorization.

This receipt qualifies a **local native transition proof**. Host reconstruction
and the binding commitment preserve untouched balances in the full local
report; the circuit constrains its declared touched frame. This does not prove
an in-circuit hash of the entire report or authenticate ledger state, owner
authority or predecessor history. Registered residual duties and
liability/discharge lowering now use the complete source registry relation. Unknown scenario identities, duplicate names or changed
immutable tuples refuse before parameters. Actual native liability proof results
remain separate from ordinary checks and circuit measurements.
Private execution, native recursion, compiler soundness, fraud limits, target
qualification and deployment remain separate requirements.

The current artifact, ABI, relation, proof and binding revisions are explicitly
`moriarty-zkir-artifact/3`, `moriarty-public-abi/3`, `moriarty-zkir-relation/3`,
`moriarty-native-transition-proof/3` and `moriarty-native-transition-binding/3`.
Each carries an explicit authorization mode: `unsigned`, or
`moriarty-jubjub-owner-intent/1` when the trusted Target has a `signed_policy`.
Current consumers refuse revision 1 and 2 artifacts and proofs; unchanged source
can recompile under revision 3. Historical revision 1 and 2 measurements and
native results retain their original scope. The signed profile's contract is
[the native owner-intent design](superpowers/specs/2026-10-11-native-owner-intent-design.md);
its native proof results remain separate from ordinary checks.

Each program registers zero through sixteen unique duty names across all its
actions. A name binds the exact asset network/issuer/reference/scale, debtor and
creditor. Identical declarations share one slot; conflicting tuples, unresolved
discharges and a seventeenth name refuse. Every action publishes every registered
opening and closing presence/UInt128 amount slot, including untouched duties.
Absent means `(false, 0)`; present-zero is distinct and consumes capacity.
Accrual sets presence true, including accrual of zero. Discharge requires presence
and preserves it, including full discharge to zero; discharge implies no payment.
Sequential effects publish before/after presence and amount in source order.
Intermediate UInt128 overflow or underflow refuses the entire local action.
Scenario closure is a compiler profile restriction, not authentication of the
complete real predecessor or absence of obligations outside the source registry.

Experiment observation, 2026-10-11: upstream IR/model inspection of exact authored
circuits, without parameters or proving, measured the following shapes under
`Target::ledger9("local", "native-liability-qa")`. Actual registry width is used;
there is no padding to sixteen.

| Source/action | Width | Optimal k / model k | Model rows |
|---|---:|---:|---:|
| `zero-lifecycle.mori` / close | 1 | 10 / 10 | 724 |
| `settlement.mori` / seed | 2 | 10 / 10 | 605 |
| `settlement.mori` / repay | 2 | 11 / 11 | 1,059 |
| `settlement.mori` / observe | 2 | 9 / 9 | 328 |
| Complete sixteen-duty fixture / originate | 16 | 12 / 12 | 3,568 |
| Complete sixteen-duty fixture / progress | 16 | 11 / 11 | 1,773 |
| Complete sixteen-duty fixture / observe | 16 | 11 / 11 | 1,352 |

The sixteen-duty source is independently authored in the ordinary measurement test
in `crates/mori-backend/tests/proving.rs`; its statements create all sixteen exact
duties, and its progress/observe actions retain all sixteen slots. These shapes
measure neither transaction/replay cost nor key generation/proving feasibility.
The engineering registry cap is not an empirical performance measurement. Exact
source, target, artifact, frame and IR digests and raw logs are recorded in
[the Task 3 report](../.superpowers/sdd/native-liability-task-3-report.md).

`cargo run -p mori --locked -- test DIRECTORY` loads `mori-tests.json`, profile `moriarty-tests/1`. Each
case supplies a source, scenario, action and either exact accepted poststate
and complete effects or an exact rejection code. Empty manifests refuse.
The JSON report retains every case in manifest order, including its full actual
execution or error. An execution includes the prestate, poststate, every step
and ordered effects. When every case report is exportable, expectation mismatches
retain all results and produce `status: "failed"` with exit 1; a matching suite produces `status: "passed"`
with exit 0. Small reports retain their existing pretty JSON format.
The report is ASCII with every string escaped, but it is not a bounded
`mori_core::json` reader-profile document: it may exceed 1 MiB, depth 32 or
65,536 nodes. Each case is exported and bounded individually.
The batch example has hand-authored expected effects for one uniform price,
per-party fees and stale-observation refusal. Oracle authenticity remains
an external premise; supplying an input called `oracle` is not authentication.

## Owner intent and resource APIs

`cargo run -p mori --locked -- intent` uses `--action`, `--scenario`, `--authority`, `--scheme`,
`--public-key` and `--framing raw`. It produces the inspectable canonical
`moriarty-owner-intent/2` statement and framed signing bytes. `--review`
prints domain, key, predecessor, nonce, validity and financial caps before
the exact statement. The authority file declares network, anchor, contract
address, target hash, signer, expected key, nonce/predecessor, validity and
per-asset gross/fee/net/closing-liability constraints.

`cargo run -p mori --locked -- verify-intent` takes the same source/action/scenario, an expected
`--authority`, a `--signature` file containing `{intent, signature_hex}`
and current logical time `--at`. It verifies the actual signature, compares
all bindings, executes locally and checks complete effects. BIP340 and
canonical low-S secp256k1 ECDSA use SHA-256 message semantics. The framed
message matches the pinned Midnight native verifier. The key, authority and
current logical time must come from the caller's trusted context. This
command does not authenticate ledger account ownership, consume a nonce,
prove history compliance or establish chain acceptance. JubJub in-circuit
verification is implemented in the separate signed revision3 compiler profile;
a bounded integrated native result is recorded in the
[October 11 signed payment evidence](../deliverables/native-owner-intent-2026-10-11/RESULT.md).
Both fresh full-source and actual-result audits approved the selected local
package, retained in [PR28](https://github.com/CharlesHoskinson/Moriarty/pull/28).

The integrated signed profile uses `cargo run -p mori --locked -- intent SOURCE --profile
moriarty-jubjub-owner-intent/1 --action ACTION --scenario SCENARIO --target
TARGET --authority AUTHORITY --at TIME`. Its `--review` output includes the exact
payload, message, complete financial projection and authorization projection.
Preparation produces `moriarty-jubjub-prepared-intent/1` without signing it.
`verify-intent` additionally takes `--signature SIGNED_WRAPPER`. For `prove` and
`verify`, the trusted signed Target selects the mode and requires `--signature`,
`--authority` and `--at` together, alongside explicit `--params-dir` and `--max-k`;
these proof commands have no `--profile` argument. Legacy scheme/key/framing arguments are
separate and refuse in this profile. A signed Target cannot fall back to unsigned
preparation or proving.

The trusted Target policy fixes owner, beneficiary and program address. Separate
expected authority supplies the key, domain, predecessor label, nonce, interval
and all exact asset limits; the verifier supplies current logical `at`. Source-wide
admission restricts every transfer/fee payer and duty debtor to the owner, including
unselected actions, with at most 16 distinct AssetIds and 16 registered duties.
The same compiled circuit binds every original financial slot, computes the exact
MOI1 message, constrains strict FENS JubJub verification and checks UInt128 gross,
fee, beneficiary net and all closing owner duties. Refunds retain gross debit;
fees reduce net once; discharge grants neither payment nor creditor consent.
State-only and observation actions still require authorization.

Ordinary CLI/compiler/reference checks reconstruct and verify the exact supplied
attachment before parameter access. Host JSON/SHA256 reconstruction, authenticated
account/key association, actual predecessor completeness, ledger time and history
remain external obligations. The native statement is existential over private R/s;
it does not commit attached signature bytes. Source Tasks1/2 are merged. The
[October 11 bounded signed payment](../deliverables/native-owner-intent-2026-10-11/RESULT.md)
produced and independently verified the full original k13 circuit with complete
financial readback, six produced-proof hostile refusals, six reused-proof native
negatives and an accepted alternative valid attachment. Both exact resource votes
preceded the one-shot run; both fresh full-source and actual-result audits
approved the selected package in [PR28](https://github.com/CharlesHoskinson/Moriarty/pull/28).
This local result does not establish ledger settlement, ownership, authenticated
time/history, nonce consumption, recursion or full supported-language coverage.
Task3's full16-asset/16-duty production preparation still refuses the1MiB report
bound; that shape and the JSON profile remain unchanged.
All signing scalars in QA are fixed public laboratory data, with no wallet custody
or private handoff.

`mori_core::policy` supplies local typed fee-capability reservations,
independent logical/DUST/service/external cost counters, and epoch/domain/
role/call-scoped federation policy checks. An unknown submission retains
reserved capacity; a timeout cannot release it. Actual fee reconciliation
requires an authenticated observation supplied by the caller. These Rust
APIs do not supply a ledger Capacity Exchange or native DUST transaction.
They are not yet source-level capability types or certified circuit primitives.

## QA and delivery scope

Run `cargo test --workspace --locked` for native language, compiler and
compatibility tests. Backend tests deserialize actual upstream ZKIR and
check reference preimages, with tampered state/effects/context and integer
boundary controls. These tests do not establish adversarial-witness proof
soundness or general compiler correspondence.

The [Quint stack](../experiments/moriarty-language/formal/quint/language/README.md)
models complete-effect acceptance, scoped authority, sponsorship, pending
recovery, bounded batch clearing and native local proof bindings. Its deterministic witnesses and seeded
simulations are separate evidence. Symbolic signature/proof premises do not
verify cryptography, and the correspondence table names runtime gaps.

This slice updates Moriarty itself. It supplies foundations for M1, M2,
M5–M7, M9, M10 and the financial library requirements. It does not close all
U0/U1/U2 obligations or M1–M17. The retained October11 selected payment produced
and independently verified a genuine local integrated signed native proof.
Broader native/U1 qualification, deploy/call, authenticated persisted acceptance
and the full launch libraries remain required subsequent language/toolchain work. External targets are
integration constraints; no Fenrir node, consensus or application code is
part of this implementation.

The [2026-10-11 local liability result](evidence/2026-10-11-native-liabilities.md) records one revision2 native repayment proof, independent complete-effect verification and four actual native negative controls under the separately approved k11 allocation. This extends the observed local subset; authenticated signed authority, predecessor/history and ledger qualification remain open.

## Integrated signed shape observations

Experiment observation, 2026-10-11: exact revision3 signed circuits were loaded
and measured through the pinned upstream IR/model only. No SRS decoding, key
generation, proving, native verification, MockProver or ignored test ran.

| Signed source/action | Assets / duties | Public values / message fields | Instructions / steps | Optimal k / model k | Model rows |
|---|---:|---:|---:|---:|---:|
| golden-repayment / repay | 1 / 2 | 43 / 75 | 232 / 6 | 12 / 12 | 4,047 |
| independent-payment / settle | 1 / 3 | 47 / 83 | 254 / 6 | 13 / 13 | 4,193 |
| signed-observation / observe | 1 / 3 | 32 / 53 | 177 / 1 | 12 / 12 | 3,176 |
| zero-registry / settle | 0 / 0 | 18 / 27 | 92 / 2 | 12 / 12 | 2,435 |
| full-sixteen-registry / settle | 1 / 16 | 158 / 305 | 849 / 20 | 14 / 14 | 10,153 |
| max-sixteen-assets-duties / settle | 16 / 16 | 338 / 635 | 1884 / 50 | 15 / 15 | 23,814 |
| max-sixteen-observation / observe | 16 / 16 | 144 / 247 | 913 / 1 | 14 / 14 | 10,476 |

Every row retains all original financial/public/duty slots and all signed
authorization inputs. There are three typed private inputs, five flattened
private fields, and one unconditional Impact per public value. MOI1 has
13+2N+6A fields and FENS has six challenge fields. The complete maximum settlement
has 48 touched balance frames and 50 ordered effects. All measured degrees are
at most k19; a shape above that ceiling must refuse without reducing its frame.

The maximum settlement's production preparation report exceeds the existing
1 MiB bounded JSON exporter and refuses. Its internal ordinary reconstruction,
full payload/message/signature and relation checker still pass; this supplies
measurement bindings and does not establish public preparation acceptance. The
maximum observation retains all 16 duty slots and all 16 asset-limit entries and
prepares successfully. The full settlement was preserved without shrinking its
slots or effects.

An isolated strict signature basis, extracted from the actual compiler while
replacing MOI1 by a separately public digest, measures 1,529 rows at k11 with
30 instructions, three public fields and the same five private fields. This
excludes financial constraints, message framing, interval and caps, so it cannot
stand in for integrated circuit costs. Row counts and k do not measure SRS/key
size, proving time, memory, transaction/replay costs or native feasibility.

Exact source/scenario/Target/authority/artifact/layout/IR and payload/signature
bindings, full current ordinary QA results and Task4's specified native controls
are in [the signed Task3 report](../.superpowers/sdd/native-owner-task-3-report.md).
Source Tasks1/2 are merged. Both fresh Sol HIGH and Grok HIGH audits approved
Tasks3/4 and the selected actual-result package on head27fe45c0, merged in
[PR28](https://github.com/CharlesHoskinson/Moriarty/pull/28).
The retained [selected payment result](../deliverables/native-owner-intent-2026-10-11/RESULT.md)
establishes one genuine local integrated signed native case; broader native/U1
and ledger qualification remain open. Reference replay adds no native allocation
or proof-verification claim.

## Source-static native call relation (ordinary checks)

`cargo run -p mori --locked -- compile-call SOURCE --action ACTION --deployment DEPLOYMENT --out DIR`
compiles current typed Core statements to pinned SDK ZKIRv3. The artifact binds
source/action, typed input communication framing, checked UInt128 arithmetic,
mutable state prefixes, captured lets, requires/ensures, native duty identities
and amounts, recipient claim totals, and exact typed event encoding/hash.
IR and the complete manifest depend on source and static native mappings; they
can be reused across requests, predecessor states and duty-presence openings.

`cargo run -p mori --locked -- prepare-call ... --relation DIR` recompiles that trusted source/static
binding and compares both artifact leaves before ordinary SDK checking. It checks
one activity record per native Op, complete raw consumption and skipped widths,
then inserts and merges Noops, runs the actual native processed transcript, and
compares full ChargedState, effects, events, adjusted measured/declared gas and
native public inputs. The result is `local-relation-checked-unproven`. A changed
binding input or matched communication randomness/commitment is a different
public instance; ordinary checking does not authenticate either context.

`raw-call.bin` and `raw-preimage.bin` preserve the original active transcript.
`processed-call.bin` embeds the rebound `processed-preimage.bin` and is for
inspection only. SDK `Transaction::prove` consumes a raw call and performs its own
Noop insertion. This slice does not provide a production proof provider, keys,
proof verification or deployment. Final intent/binding/DUST/TTL changes still
require reconstruction; these artifacts convey no permission to submit a call.

Event admission uses the SDK's `serialize_to_node_list_bounded(MAX_LOG_EMITTED)`
on each source-static scalar serialization branch. Storage encoding overhead is
included. A branch the native VM would drop is constrained inactive; valid shorter
values remain admitted under the same artifact. If even the minimum encoding
fails, compilation refuses. The 37-record/263-public-field UInt event golden is
for the short-name, single-event fixture with all 17 branches admitted. Event
names/actions near the native bound can have fewer admitted branches. Bool has
one source-valid scalar branch.

Full account prefix balances, custody/funding ownership, authenticated native
predecessors and deployed keys, economic intention authority, final history/PCD,
and ledger settlement remain separate open predicates. Existing host admission
checks these local examples' source effects and economic bounds; those checks do
not become circuit authority. Ordinary checks and the bounded Quint projection
model establish no cryptographic proof or general compiler correspondence theorem.


The finite escrow example now reads its current liabilities before partial
delivery, completion and recovery. It requires delivery equal remaining units
and refund equal remaining units times price. Funding refuses existing duties,
including present-zero tombstones. Postconditions compare residual duty amounts
with the updated state. `liability_present("name")` returns Bool, and
`liability_amount("name")` returns UInt atomic units. These literal-name reads
require a source-fixed identity and inspect the current statement snapshot.
The ordinary relation compiler reuses its existing duty frame. The ledger-call
source compiler now lowers these leaves through membership and full duty-cell
openings with source-fixed identity, Bool membership and UInt128 amount
constraints. The operation producer orders reads before each statement effect;
lets retain captured values and fresh statements observe the updated duty.
Source compilation and operation construction do not authenticate an incoming
predecessor or establish native proof or ledger acceptance. Native execution and
constraint-level counterfeit rejection for these new consumers remain unperformed.

The complete escrow example still exceeds the ledger-call one-public-payer
profile: funding uses buyer and seller, and later actions pay from escrow.
Admission checks every action, so selecting a later action does not remove this
refusal. Supported single-payer source consumers can compile duty guards, amounts,
state updates and events. Existing ABI, phase policy and resource limits remain
in force; changed source/compiler/template commitments require fresh descriptors.

## All-action public source-static export and inspection

`cargo run -p mori --locked -- compile-call FILE --all-actions --static-bindings FILE --authorization
unsigned|signed-v2 --work-limits FILE --out NEW_DIR` compiles every admitted
source action to tagged native-call ZKIR through the existing native relation
compiler. `signed-v2` requires `--signer-policy FILE`; unsigned forbids it. There
is no default authorization or signed-v1 fallback. The legacy selected-action
`--action NAME --deployment FILE --out NEW_DIR` remains available.

Replace `--out` with `--inspect-static BUNDLE_DIR` to reopen against independently
supplied source, public bindings, authorization/policy and work limits. The
reader bounds and hashes every fixed source/ordinal leaf, rejects extra names
and symlinks, recompiles caller inputs, and compares complete exact bytes before
canonical tagged decode. Success means `source-static-recompile-admitted`, with
no installed or proving capability. See the [public examples](../examples/core/static-compile/README.md).

The new profile rejects explicit null, unknown fields, duplicate decoded keys
and trailing JSON; absent `economic_bounds` means None. Legacy serialization and
relation-manifest bytes retain their existing rules. Explicit work limits /2
count conservative max(raw, typed) input sizes, the limits text itself, all
action groups/public fields, every leaf and the final marker/aggregate before
output creation. The example's degree/SRS/key values of `1` are positive schema
fillers, never measurements or native resource authority.

Export reports `source-static-unproven-uninstalled`. Network, contract address
and public mappings remain supplied local premises. Source/CLI closure changes
invalidate historical exact signing contexts, IR, key identities and current
audit applicability. This delivery opens, regenerates and qualifies no keys,
parameters or SRS. M1/M2/U2, native soundness, full semantic correspondence,
64 KiB transaction/k19 qualification, funding/state/history authentication, PCD
and financial ledger settlement remain open.

### Mapped-role statement provenance

`--authorization unsigned-mapped-roles` exports each action with
`statement_emissions`, one entry per source statement in source order. Each
entry contains `statement`, `expression`, `instruction_start`, `instruction_end`,
`group_start`, `group_end` and `dynamic_fields`.

`statement` indexes the action's statements in `abi.actions`. `expression` is
that statement's root Core expression ID, rather than a recursive dependency
map. Instruction and group ranges are zero-based, half-open `[start, end)`
ranges local to the action's IR and manifest. Groups count only `Impact`
instructions. Prelude and epilogue positions remain outside source statement
ranges; a statement with no groups still has an entry.

Each `dynamic_fields` element is `{group, offset}`: an absolute action-local
group index and a zero-based offset into that Impact's input fields. It records
an actual template replacement, including when the replacement operand is an
`Immediate`. These are replacement positions, rather than a variable-only list.
Kind, asset, `from` and `to` resolve through the indexed ABI statement.
As Opus's design review
states, "The map is emission provenance, not semantics." It records neither
dependencies nor group activity and does not establish value equivalence or
debit binding.

Opus's compatibility disposition changes the mapped relation leaf format to
`moriarty-mapped-role-source-static-relation/2`, while retaining bundle profile
`moriarty-mapped-role-source-static-bundle/1` and both existing qualifications.
The ordinary and signed manifest schemas remain unchanged, but source-closure
changes update their compiler hashes and manifest bytes, plus static bundle
build identities. Old bundles require re-export for the new build; earlier
ordinary artifacts require regenerated manifests. Future native requalification
is separate and unperformed. The [2026-10-15 evidence](audits/2026-10-15/core-source-provenance/README.md)
records ten passing focused checks, including public export/reopen and actual
IR field positions. Full source/result reviews are recorded in that dossier;
these source-static observations close no U0/U2 or other parent exit.


## Metered local owner-intent verification

`cargo run -p mori --locked -- verify-intent FILE --action NAME --scenario FILE --authority FILE
--signature FILE --at TIME --logical-cap CANONICAL_U128` applies the existing
`moriarty-reference-logical-cost/1` meter to the Core
`moriarty-owner-intent/2` reference consumer. The optional cap is an unsigned,
locally trusted operational limit. It does not enter the canonical owner intent,
its signing frame or Execution. Calls without the flag retain their existing
output and API behavior.

The consumer prepares the caller's exact intent, checks its signature and
inclusive validity interval, executes once under the cap, then checks complete
ordered effects and closing liabilities. Preparation and authentication refusal
report used `0`. Runtime refusal retains attempted statement/expression work.
A late gross, fee, net or residual-duty refusal retains all consumed work and
returns no execution, effects or post-state. Refunds do not reduce gross debits
or consumed work. Metered success retains `signature_valid: true` and
`local-signature-and-reference-execution`, and adds the logical profile and
`logical: {cap, used}`. Rejection reports the error and cost without successful
execution. Human review reports verification only after acceptance.

Caps must be canonical unsigned decimal u128 strings. Combining the cap with
`--profile` or `--target` refuses before input file reads. This option supplies
no native/Jubjub profile, relation-check fallback or native execution authority.
The profile measures statement attempts and first expression evaluations in each
statement only. Parsing, signature verification, authority checks, serialization,
DUST, service and external costs remain outside this measurement.

Fixed public BIP340 and low-S ECDSA fixtures exercise independently derived
Counter work `16`, Payment work `4` and TwoAssetDuty work `18`, including fees,
refunds and a closing duty of `5` after partial discharge. Test fixtures retain
public canonical messages, public keys and signatures; tests perform reference
verification only. Key-to-account authority, nonce consumption, authenticated
funding/history, native proof validity, PCD and financial Preview settlement
remain open.

The additive Core/CLI source changes alter the existing native source-closure
identity. Historical native signing contexts, IR, key identities and audit
applicability require their own current exact-source qualification. This local
delivery reads, regenerates and qualifies no native keys, parameters or SRS.
````

### C04 crates/mori/Cargo.toml — exact-line-ranges

````text
LINES 9–27

[features]
core2-disjoint-draft = ["mori-core/core2-disjoint-draft"]
default = ["native"]
source6-expansion = ["dep:mori-source6", "mori-source6/source6-transfer-slice"]
native = [
    "dep:mori-backend",
    "dep:mori-ledger",
    "dep:futures",
    "dep:rand",
    "dep:midnight-ledger",
    "dep:midnight-base-crypto",
    "dep:midnight-coin-structure",
    "dep:midnight-onchain-state",
    "dep:midnight-onchain-runtime",
    "dep:midnight-storage",
    "dep:midnight-transient-crypto",
    "dep:midnight-serialize",
]
````

### C05 crates/mori/src/commands/capabilities.rs — exact-line-ranges

````text
LINES 44–102
/// Metadata only. The profile tuple is (input, semantics, output); an optional
/// build requirement carries its actual cfg value, not a runtime probe.
fn consumer(
    id: &str,
    entrypoint: &str,
    profiles: (Option<&str>, Option<&str>, Option<&str>),
    build: Option<(&str, bool)>,
    status: (&str, &str),
    limitations: &[&str],
    source_refs: &[&str],
) -> Value {
    let requirements: Vec<_> = build.iter().map(|(name, _)| *name).collect();
    json!({
        "id": id, "entrypoint": entrypoint,
        "inputProfile": profiles.0, "semanticsProfile": profiles.1,
        "outputProfile": profiles.2, "requiresBuildFeatures": requirements,
        "implementation": status.0,
        "availableInBuild": status.0 == "source-present" && build.is_none_or(|(_, enabled)| enabled),
        "qualification": status.1, "limitations": limitations, "sourceRefs": source_refs
    })
}

pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    use mori_core::stage::{LANGUAGE_2, SEMANTICS_2};
    use mori_core::{LANGUAGE, SEMANTICS};

    // JSON is the sole format; the flag is an explicit client spelling.
    let _ = args.json;
    let native = cfg!(feature = "native");
    let source6 = cfg!(feature = "source6-expansion");
    let disjoint = cfg!(feature = "core2-disjoint-draft");
    let report = json!({
        "format": "moriarty-cli-capabilities/1",
        "producer": {
            "implementation": "rust-workspace-cli", "packageVersion": env!("CARGO_PKG_VERSION"),
            "binarySourceRevision": null, "responseKind": "command-self-report",
            "buildFeatures": {"native": native, "source6-expansion": source6, "core2-disjoint-draft": disjoint}
        },
        "catalogue": {
            "sourceRevision": SOURCE_REVISION, "coverage": "selected-public-profile-consumers",
            "basis": "source-inspection", "runtimeEvidence": "not-included",
            "note": "Curated consumer descriptions use this inspection basis, not an inferred binary revision. Linked profile constants and cfg flags come from the build. Routing/editor changes require a catalogue update."
        },
        "profiles": [
            profile("rust-core1", LANGUAGE, Some(SEMANTICS), "mori_core::parse", "language", &["UInt", "Bool"]),
            profile("rust-core2", LANGUAGE_2, Some(SEMANTICS_2), "mori_core::stage::load", "language", &["UInt", "Bool"]),
            profile("rust-beta-frontend", mori_parser::PROFILE, None, "mori_parser::parse then mori_checker::check", "profile", &[])
        ],
        "consumers": [
            consumer("core1-check", "check FILE --json", (Some(LANGUAGE), Some(SEMANTICS), None), None,
                ("source-present", "source-check"), &["No compilation or target qualification."], &["crates/mori/src/commands/check.rs"]),
            consumer("core2-check", "check FILE --json", (Some(LANGUAGE_2), Some(SEMANTICS_2), None), None,
                ("source-present", "source-check"), &["Uses staged admission and planning."], &["crates/mori/src/commands/check.rs"]),
            consumer("beta-check", "check FILE --json", (Some(mori_parser::PROFILE), None, None), None,
                ("source-present", "frontend-check-only"), &["Named-action admission is determined by the checker; no Core semantics are inferred."], &["crates/mori/src/commands/check.rs"]),
            consumer("core1-reference", "simulate FILE --action NAME --scenario FILE", (Some(LANGUAGE), Some(SEMANTICS), None), None,
                ("source-present", "local-reference-execution"), &["--trace is a separate native-feature consumer."], &["crates/mori/src/commands/simulate.rs"]),
            consumer("core2-reference", "simulate FILE --action NAME --scenario FILE; simulate FILE --resume FILE --event FILE", (Some(LANGUAGE_2), Some(SEMANTICS_2), None), None,
                ("source-present", "local-reference-stage"), &["Stdout has no outer format. Waiting alone contains continuation.format=moriarty-continuation/1; complete and rejected do not.", "Authentication is none; consumption is untracked."], &["crates/mori/src/commands/simulate.rs", "crates/mori-core/src/stage/mod.rs"]),
LINES 111–116
            consumer("beta-to-source6-transfer", "expand FILE --emit-source6-transfer --action NAME --scenario FILE", (Some(mori_parser::PROFILE), None, Some("Source/6")), Some(("source6-expansion", source6)),
                ("source-present", "local-stipulation-only"), &["Checked LocalS0 transfer subset and moriarty-local-scenario/1 only.", "Raw proposal output; no Source/6 input parser or Core/5 preparation is supplied."], &["crates/mori/src/commands/expand.rs", "crates/mori/src/commands/expand/source6.rs"]),
            consumer("core1-compile", "compile FILE --target FILE --out DIR", (Some(LANGUAGE), Some(SEMANTICS), Some("moriarty-zkir-artifact/3")), Some(("native", native)),
                ("source-present", "compiler-source-present"), &["Exact target descriptor and compiler subset admission required; nothing was compiled by this query."], &["crates/mori/src/commands/compile.rs"]),
            consumer("core2-compile", "compile FILE --target FILE --out DIR", (Some(LANGUAGE_2), Some(SEMANTICS_2), Some("moriarty-zkir-artifact/4")), Some(("native", native)),
                ("source-present", "staged-compiler-source-present"), &["Distinct staged artifact/4, ABI/4 and relation/4; unsigned target only.", "Legacy /3 proving consumers do not accept the distinct /4 type; no proof or ledger result is inferred."], &["crates/mori/src/commands/compile.rs", "crates/mori-backend/src/stage/mod.rs"]),
LINES 122–135
        "targets": [{
            "id": "midnight-ledger9-zkirv3", "consumerIds": ["core1-compile", "core2-compile"],
            "descriptorRequired": true, "configuredTarget": null, "qualification": "not-checked",
            "notes": ["No target, network, anchor, parameters, prover, verifier or deployment was inspected."]
        }],
        "otherExecutables": [{
            "id": "node-beta-cli", "package": "@moriarty-lang/beta", "binaryName": "mori",
            "relationship": "separate executable; no Rust capability inheritance", "availability": "not-inspected"
        }],
        "limitations": [
            "availableInBuild means the source path is compiled in, not that a request passed admission or execution.",
            "This selected catalogue is not exhaustive; omitted capabilities and grammar remain unknown.",
            "No runtime, proof, installation, deployment or ledger qualification is established."
        ]
````

### C06 crates/mori/tests/capabilities.rs — exact-line-ranges

````text
LINES 70–105
#[test]
fn public_query_distinguishes_profiles_build_consumers_and_unchecked_targets() {
    let f = Files::new();
    fs::write(f.0.join("user-note.txt"), "user content\n").unwrap();
    let before = f.snapshot();
    let output = f.run(&["capabilities", "--json"]);
    let report = json(&output);
    assert_eq!(before, f.snapshot());
    assert!(output.stdout.ends_with(b"\n"));
    assert!(output.stdout.len() < 65_536);
    for args in [
        vec!["capabilities"],
        vec!["--quiet", "capabilities", "--json"],
    ] {
        let next = f.run(&args);
        assert_eq!(json(&next), report);
        assert_eq!(next.stdout, output.stdout);
        assert_eq!(before, f.snapshot());
    }
    let fields: BTreeSet<_> = report
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        fields,
        BTreeSet::from([
            "format",
            "producer",
            "catalogue",
            "profiles",
            "consumers",
            "targets",
            "otherExecutables",
            "limitations"
````

### C07 crates/mori/src/commands/check.rs — complete-context-body

````text
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use mori_diagnostics::{MoriDiagnostic, NamedSource};

use crate::json::CheckReport;
use crate::shell::Shell;

use super::check_source::{self, SourceCheckConsumer};

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,

    /// Print the result as JSON
    #[arg(long)]
    pub json: bool,
}

/// Checks syntax, names, values, types, declarations, calls and intents.
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let name = args.file.display().to_string();
    let source = super::core_io::read_source(&args.file)?;

    if !args.json {
        shell.status("Checking", &name);
    }
    let started = Instant::now();
    let outcome = check_source::dispatch(
        &source,
        CliConsumer {
            name: &name,
            source: &source,
            json: args.json,
            shell,
        },
    )?;
    if outcome.beta {
        if let Some(report) = outcome.report {
            super::core_io::print(&report)?;
        } else if outcome.accepted {
            shell.status(
                "Finished",
                format!("check in {:.2}s", started.elapsed().as_secs_f64()),
            );
        }
    }
    Ok(if outcome.accepted {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

struct CliOutcome {
    accepted: bool,
    beta: bool,
    report: Option<CheckReport>,
}

struct CliConsumer<'a> {
    name: &'a str,
    source: &'a str,
    json: bool,
    shell: &'a mut Shell,
}

impl SourceCheckConsumer for CliConsumer<'_> {
    type Output = super::Result<CliOutcome>;

    fn parsed(&mut self, tokens: usize, nodes: usize) {
        if !self.json {
            self.shell
                .verbose_status("Parsed", format!("{tokens} tokens, {nodes} nodes"));
        }
    }

    fn beta(
        self,
        checked: Option<&mori_checker::Checked<'_>>,
        diagnostics: Vec<MoriDiagnostic>,
    ) -> Self::Output {
        let report = self.json.then(|| {
            check_source::beta_report(self.name.to_owned(), self.source, checked, &diagnostics)
        });
        let accepted = finish(self.shell, self.name, self.source, diagnostics, self.json);
        Ok(CliOutcome {
            accepted,
            beta: true,
            report,
        })
    }

    fn core(
        self,
        result: Result<mori_core::Program, mori_core::Error>,
        staged: bool,
    ) -> Self::Output {
        if self.json {
            let report = check_source::core_report(self.name, self.source, &result, staged);
            super::core_io::print(&report)?;
        } else {
            match &result {
                Ok(_) => self.shell.status("Finished", "bounded core check"),
                Err(error) => self.shell.error(error),
            }
        }
        Ok(CliOutcome {
            accepted: result.is_ok(),
            beta: false,
            report: None,
        })
    }
}

/// Renders the diagnostics, unless the report is JSON. True when there are none.
fn finish(
    shell: &mut Shell,
    name: &str,
    source: &str,
    diagnostics: Vec<MoriDiagnostic>,
    json: bool,
) -> bool {
    if diagnostics.is_empty() {
        return true;
    }
    if !json {
        let count = diagnostics.len();
        for diagnostic in diagnostics {
            shell.report(&diagnostic.with_source_code(NamedSource::new(
                crate::display::text(name),
                source.to_owned(),
            )));
        }
        let plural = if count == 1 { "" } else { "s" };
        shell.error(format!(
            "could not check `{name}` due to {count} previous error{plural}"
        ));
    }
    false
}
````

### C08 crates/mori/src/commands/simulate.rs — exact-line-ranges

````text
LINES 1–39
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,

    /// Action to run
    #[arg(long, value_name = "NAME", required_unless_present = "resume")]
    pub action: Option<String>,

    /// Scenario JSON with the local state and inputs
    #[arg(long, value_name = "FILE", required_unless_present = "resume")]
    pub scenario: Option<PathBuf>,
    /// Resume a moriarty/2 continuation (local, unauthenticated, consumption untracked)
    #[arg(
        long,
        value_name = "FILE",
        requires = "event",
        conflicts_with_all = ["scenario", "trace", "target", "profile", "authority", "at", "signature"]
    )]
    pub resume: Option<PathBuf>,
    /// Stage event JSON for --resume (at most 64 KiB)
    #[arg(long, value_name = "FILE", requires = "resume")]
    pub event: Option<PathBuf>,
    /// Generate a complete public replay sidecar against a trusted Target
    #[arg(long)]
    pub trace: bool,
    /// Cap local statement and uncached expression attempts (other costs unmeasured)
    #[arg(long, value_name = "CANONICAL_U128", value_parser = logical_cap, conflicts_with = "trace")]
    pub logical_cap: Option<mori_core::UInt>,
    /// Trusted Target JSON (required with --trace)
    #[arg(long, value_name = "FILE")]
    pub target: Option<std::path::PathBuf>,
    #[command(flatten)]
    pub authorization: super::replay::AuthorizationArgs,
LINES 59–69
pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    // Clap has already refused every --resume/--event conflict, so a
    // malformed invocation reads no file.
    if let Some(resume) = &args.resume {
        return run_resume(&args, resume);
    }
    let (action, scenario_path) = args.legacy()?;
    if args.trace {
        let path = args
            .target
            .as_ref()
LINES 110–159
    // Hook: /2 routing. A /2 start keeps every core failure labelled.
    if mori_core::language(&source) == Ok(mori_core::stage::LANGUAGE_2) {
        return run_stage_start(&args, &source, action, scenario_path);
    }
    let program = mori_core::parse(&source).map_err(super::core_io::operation)?;
    let scenario = super::core_io::read_json(scenario_path)?;
    if let Some(cap) = args.logical_cap {
        return match mori_core::simulate_metered(&program, action, &scenario, cap) {
            Ok(metered) => {
                super::core_io::print(&serde_json::json!({
                    "status": "accepted", "qualification": "local-reference-execution",
                    "execution": metered.execution, "profile": metered.cost.profile,
                    "logical": {"cap": metered.cost.cap, "used": metered.cost.used}
                }))?;
                Ok(ExitCode::SUCCESS)
            }
            Err(refused) => {
                super::core_io::print(&serde_json::json!({
                    "status": "rejected", "error": refused.error,
                    "profile": refused.cost.profile,
                    "logical": {"cap": refused.cost.cap, "used": refused.cost.used}
                }))?;
                Ok(ExitCode::FAILURE)
            }
        };
    }
    match mori_core::simulate(&program, action, &scenario) {
        Ok(execution) => {
            super::core_io::print(
                &serde_json::json!({"status":"accepted", "qualification":"local-reference-execution", "execution":execution}),
            )?;
            Ok(ExitCode::SUCCESS)
        }
        Err(error) => {
            super::core_io::print(&serde_json::json!({"status":"rejected", "error":error}))?;
            Ok(ExitCode::FAILURE)
        }
    }
}

/// Render the same explicit qualification on every staged public object.
pub(super) fn print_stage_report(mut report: serde_json::Value) -> super::Result<()> {
    let object = report
        .as_object_mut()
        .expect("stage reports are JSON objects");
    for (name, value) in mori_core::stage::STAGE_LABELS {
        object.insert(name.into(), serde_json::Value::String(value.into()));
    }
    super::core_io::print(&report)
}
LINES 177–235
fn run_resume(args: &Args, resume: &Path) -> super::Result<ExitCode> {
    use mori_core::stage::{self, StageOutcome, StageRejection, decode};
    let event_path = args
        .event
        .as_deref()
        .ok_or_else(|| super::core_io::operation("--resume requires --event"))?;
    let source = super::core_io::read_source(&args.file)?;
    let result = (|| -> Result<StageOutcome, StageRejection> {
        let program = stage::load(&source).map_err(no_work)?;
        let continuation = decode::read_bounded(
            resume,
            decode::MAX_CONTINUATION_BYTES,
            decode::CONTINUATION_TOO_LARGE,
        )
        .and_then(|text| decode::continuation(&text))
        .map_err(no_work)?;
        let event =
            decode::read_bounded(event_path, decode::MAX_EVENT_BYTES, decode::EVENT_TOO_LARGE)
                .and_then(|text| decode::event(&text))
                .map_err(no_work)?;
        stage::resume(
            &program,
            continuation,
            event,
            args.action.as_deref(),
            args.logical_cap,
        )
    })();
    print_stage_result(result)
}

/// A `/2` start. Load and scenario failures are labelled stage rejections; the
/// scenario keeps the legacy 1 MiB strict reader.
fn run_stage_start(
    args: &Args,
    source: &str,
    action: &str,
    scenario: &Path,
) -> super::Result<ExitCode> {
    use mori_core::stage::{self, StageOutcome, StageRejection, decode};
    let result = (|| -> Result<StageOutcome, StageRejection> {
        let program = stage::load(source).map_err(no_work)?;
        let text = decode::read_bounded(scenario, 1_048_576, "Scenario JSON exceeds 1 MiB")
            .map_err(no_work)?;
        let value = mori_core::json::parse(&text).map_err(no_work)?;
        let scenario = serde_json::from_value(value).map_err(|error| {
            no_work(mori_core::Error {
                code: "mori::core::json".into(),
                message: error.to_string(),
                offset: None,
                step: None,
            })
        })?;
        stage::start(&program, action, scenario, args.logical_cap)
    })();
    print_stage_result(result)
}

/// A refusal before any segment evaluation: no work attempted or committed.
````

### C09 crates/mori/src/commands/expand/source6.rs — exact-line-ranges

````text
LINES 89–162
fn read_scenario(path: &std::path::Path) -> crate::commands::Result<TransferScenario> {
    let raw: serde_json::Value = core_io::read_json(path)?;
    let object = raw.as_object().ok_or_else(|| {
        scenario_error(
            "source6-scenario-profile",
            "Expected a local transfer scenario object.",
        )
    })?;
    if object.get("profile").and_then(serde_json::Value::as_str)
        != Some("moriarty-local-scenario/1")
        || object.get("kind").and_then(serde_json::Value::as_str) != Some("local-stipulation")
    {
        return Err(scenario_error(
            "source6-scenario-profile",
            "Source/6 transfer expansion requires moriarty-local-scenario/1 local-stipulation input.",
        ));
    }
    for field in ["obligation", "candidate_effects"] {
        if object.contains_key(field) {
            return Err(scenario_error(
                "source6-scenario-unsupported",
                format!("The --emit-source6-transfer subset does not support `{field}`."),
            ));
        }
    }
    if object
        .get("balances")
        .and_then(serde_json::Value::as_array)
        .is_some_and(|balances| balances.len() != 3)
    {
        return Err(scenario_error(
            "source6-scenario-cells",
            "The transfer subset requires exactly three balance cells: owner, recipient, fee recipient.",
        ));
    }
    serde_json::from_value(raw).map_err(|error| {
        scenario_error(
            "source6-scenario-shape",
            format!("Local transfer scenario shape: {error}"),
        )
    })
}

pub(super) fn run(input: ActionInput, shell: &mut Shell) -> crate::commands::Result<ExitCode> {
    let source = core_io::read_source(&input.file)?;
    let name = crate::display::text(&input.file.display().to_string());
    let selected = match select_transfer(&source, &input.action) {
        Ok(selected) => selected,
        Err(diagnostics) => {
            for diagnostic in diagnostics {
                shell.report(&diagnostic.with_source_code(NamedSource::new(&name, source.clone())));
            }
            return Ok(ExitCode::FAILURE);
        }
    };
    let scenario = read_scenario(&input.scenario)?;
    let expansion = match selected.expand(&scenario.local()) {
        Ok(expansion) => expansion,
        Err(diagnostic) => {
            shell.report(&diagnostic.with_source_code(NamedSource::new(name, source)));
            return Ok(ExitCode::FAILURE);
        }
    };
    // The library includes its final newline. This writes its actual proposal.
    std::io::stdout()
        .lock()
        .write_all(expansion.source6.as_bytes())
        .map_err(core_io::operation)?;
    shell.status(
        "Expanded",
        format!("Source/6 transfer proposal ({})", expansion.qualification),
    );
    Ok(ExitCode::SUCCESS)
}
````

### C10 crates/mori/src/commands/compile.rs — exact-line-ranges

````text
LINES 1–78
#[cfg(feature = "native")]
use super::core_io;
#[cfg(feature = "native")]
use crate::error::CliError;
use crate::shell::Shell;
#[cfg(feature = "native")]
use std::path::Path;
use std::{path::PathBuf, process::ExitCode};

#[derive(Debug, clap::Args)]
pub struct Args {
    pub file: PathBuf,
    /// Exact target profile JSON with network, anchor and compiler limits
    #[arg(long)]
    pub target: PathBuf,
    /// New output directory; refuses to overwrite existing artifacts
    #[arg(long)]
    pub out: PathBuf,
}
#[cfg(feature = "native")]
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let source = core_io::read_source(&args.file)?;
    if mori_core::language(&source).is_ok_and(|language| language == "moriarty/2") {
        let target: mori_backend::Target = core_io::read_json(&args.target)?;
        let artifact = mori_backend::stage::compile_staged(&source, &target, &args.out)
            .map_err(core_io::operation)?;
        let manifest = serde_json::to_string_pretty(&artifact).map_err(core_io::operation)?;
        std::fs::create_dir(&args.out).map_err(|source| CliError::Write {
            path: args.out.clone(),
            source,
        })?;
        publish_staged_artifact(&artifact, &manifest, &args.out)?;
        shell.status(
            "Compiled",
            format!(
                "{} circuits into {}",
                artifact.circuits.len(),
                args.out.display()
            ),
        );
        return Ok(ExitCode::SUCCESS);
    }
    let program = mori_core::parse(&source).map_err(core_io::operation)?;
    let target: mori_backend::Target = core_io::read_json(&args.target)?;
    // The trusted Target selects the mode explicitly; a signed Target never
    // reaches the unsigned compiler.
    let reported = mori_backend::source_projection::compile_reported(&program, &target)
        .map_err(core_io::operation)?;
    let artifact = reported.artifact;
    let projection = serde_json::to_string_pretty(&serde_json::json!({
        "source_hash": mori_core::intent::hash_bytes(source.as_bytes()),
        "compilation": reported.source_projection,
    }))
    .map_err(core_io::operation)?;
    // Compile and serialize fully before touching the destination.
    let manifest = serde_json::to_string_pretty(&artifact).map_err(core_io::operation)?;
    std::fs::create_dir(&args.out).map_err(|source| CliError::Write {
        path: args.out.clone(),
        source,
    })?;
    publish_artifact(&artifact, &manifest, Some(&projection), &args.out)?;
    shell.status(
        "Compiled",
        format!(
            "{} circuits into {}",
            artifact.circuits.len(),
            args.out.display()
        ),
    );
    Ok(ExitCode::SUCCESS)
}

// The caller has reserved a new directory after all input/compiler preflight.
#[cfg(feature = "native")]
fn publish_artifact(
    artifact: &mori_backend::Artifact,
    manifest: &str,
    projection: Option<&str>,
````

### C11 crates/mori-core/src/stage/report.rs — complete-context-body

````text
//! Complete qualified JSON envelopes for the local Core/2 outcome API.
use super::{STAGE_LABELS, StageOutcome, StageRejection};
use serde_json::{Value, json};

/// Render an outcome or rejection without executing a program or performing I/O.
/// Every envelope retains the explicit local-stage qualifications.
pub fn outcome_report(result: Result<StageOutcome, StageRejection>) -> Value {
    let mut report = match result {
        Ok(StageOutcome::Waiting {
            advanced,
            segment,
            continuation,
            attempted,
            residual,
        }) => {
            let committed = continuation.logical.committed;
            json!({"status":"waiting","advanced":advanced,"segment":segment,
                "continuation":continuation,"attempted":attempted,
                "committed":committed,"residual":residual})
        }
        Ok(StageOutcome::Complete {
            segment,
            committed,
            attempted,
            residual,
        }) => json!({"status":"complete","segment":segment,"attempted":attempted,
                "committed":committed,"residual":residual}),
        Err(rejected) => json!({"status":"rejected","error":rejected.error,
            "attempted":rejected.attempted,"committed":rejected.committed}),
    };
    let object = report
        .as_object_mut()
        .expect("stage reports are JSON objects");
    for (name, value) in STAGE_LABELS {
        object.insert(name.into(), Value::String(value.into()));
    }
    report
}
````

### C12 crates/mori/tests/core_report_contract.rs — exact-line-ranges

````text
LINES 84–110
fn counter_unmetered_and_positive_caps_match_complete_public_reports() {
    let project = Project::new(
        "counter.mori",
        fixture!("counter.mori"),
        "counter-amount7.scenario.json",
        fixture!("counter-amount7.scenario.json"),
    );
    for (name, cap, expected) in [
        (
            "counter-unmetered",
            None,
            fixture!("counter.unmetered.stdout.json"),
        ),
        (
            "counter-cap16",
            Some("16"),
            fixture!("counter.cap16.stdout.json"),
        ),
        (
            "counter-capmax",
            Some("340282366920938463463374607431768211455"),
            fixture!("counter.capmax.stdout.json"),
        ),
    ] {
        project.assert_case(name, "run", cap, expected);
    }
}
````

### C13 crates/mori/tests/core_continuations.rs — exact-line-ranges

````text
LINES 145–180
fn complete(r: &Value, expected: &Value) {
    assert_eq!(r["status"], "complete");
    assert_eq!(
        r["attempted"],
        expected["attempted"].as_u64().unwrap().to_string()
    );
    assert_eq!(
        r["committed"],
        expected["committed"].as_u64().unwrap().to_string()
    );
    assert_eq!(r["residual"], expected["residual"]);
    assert_eq!(r["segment"]["frame_post"], json!({"site":0,"captures":[]}));
    segment(&r["segment"], &expected["segment"]);
}
#[test]
fn delivery_false_observation_preserves_boundary_then_completes_at_eleven() {
    let o = oracle();
    let e = &o["escrow"];
    let f = Files::new(DELIVERY);
    let started = accepted(&f.start("deliver", &e["opening"], Some("11")));
    waiting(&started, &e["prefix_waiting"]);
    assert_eq!(started["continuation"]["logical"]["cap"], "11");
    let c = &started["continuation"];
    let observed = accepted(&f.resume(c, &e["false_resume"]["event"], None));
    assert_eq!(observed["status"], "waiting");
    assert_eq!(observed["advanced"], false);
    assert_eq!(observed["segment"], Value::Null);
    assert_eq!(observed["attempted"], "2");
    assert_eq!(observed["continuation"], *c);
    assert_eq!(observed["residual"], started["residual"]);
    let done = accepted(&f.resume(c, &e["true_resume_complete"]["event"], Some("deliver")));
    complete(&done, &e["true_resume_complete"]);
    assert_eq!(done["residual"], json!([]));
    assert_eq!(
        done["segment"]["frame_post"],
        json!({"site":0,"captures":[]})
````

### C14 crates/mori/tests/source6_expand.rs — exact-line-ranges

````text
LINES 63–102
fn source6_arguments() -> [&'static str; 7] {
    [
        "expand",
        "invoice.mori",
        "--action",
        "pay",
        "--scenario",
        "scenario.json",
        "--emit-source6-transfer",
    ]
}

#[test]
fn checked_beta_reaches_real_expansion_library_and_raw_stdout() {
    let files = Files::new();
    let check = files.invoke(&["check", "invoice.mori", "--json"]);
    assert_eq!(check.status.code(), Some(0));
    assert!(check.stderr.is_empty());
    let report: Value = serde_json::from_slice(&check.stdout).unwrap();
    assert_eq!(report["status"], "accepted");
    assert_eq!(report["actions"][0]["name"], "pay");
    assert_eq!(report["actions"][0]["support"], "local_s0");

    let expansion = files.invoke(&source6_arguments());
    assert_eq!(expansion.status.code(), Some(0));
    assert_eq!(expansion.stdout.as_slice(), fixture!("invoice.source6"));
    assert_eq!(expansion.stderr.as_slice(), fixture!("source6.stderr.txt"));
}

#[test]
fn explicit_source6_quiet_output_is_raw_source6_text() {
    let files = Files::new();
    let mut arguments = vec!["--quiet"];
    arguments.extend(source6_arguments());
    let expansion = files.invoke(&arguments);
    assert_eq!(expansion.status.code(), Some(0));
    assert_eq!(expansion.stdout.as_slice(), fixture!("invoice.source6"));
    assert!(expansion.stderr.is_empty());
}

````

### C15 crates/fixtures/core-report-contract/counter.mori — complete-context-body

````text
language "moriarty/1"; agreement Counter {
state count: UInt = 0;
action run(amount: UInt) {
let total = count + amount;
requires total <= 100;
next count = total;
ensures count == total;
emit "count"(count);
} }
````

### C16 crates/fixtures/core-report-contract/counter-amount7.scenario.json — complete-context-body

````text
{"state":{},"inputs":{"amount":{"type":"UInt","value":"7"}},"balances":[],"liabilities":[]}
````

### C17 crates/fixtures/core-report-contract/counter.unmetered.stdout.json — json-pointer-values

````text
{"/status":"accepted","/qualification":"local-reference-execution","/execution/post":{"balances":[],"liabilities":[],"state":{"count":{"type":"UInt","value":"7"}}},"/execution/effects":[{"after":{"type":"UInt","value":"7"},"before":{"type":"UInt","value":"0"},"kind":"State","name":"count"},{"kind":"Emit","name":"count","value":{"type":"UInt","value":"7"}}]}
````

### C18 examples/core/continuations/delivery.mori — complete-context-body

````text
language "moriarty/2";
agreement ContinuationEscrowOracle {
  asset PAY = { network: "local", issuer: "continuation-escrow-oracle", reference: "PAY", scale: 0 };
  state marker: UInt = 77;
  action deliver(amount: UInt) {
    transfer(PAY, "alice", "escrow", amount);
    liability("release", PAY, "escrow", "bob", amount);
    await delivery(ready: Bool) when ready;
    transfer(PAY, "escrow", "bob", amount);
    discharge("release", amount);
  }
}
````

### C19 examples/core/continuations/delivery-scenario.json — complete-context-body

````text
{
  "balances": [
    {
      "account": "alice",
      "amount": "100",
      "asset": {
        "issuer": "continuation-escrow-oracle",
        "network": "local",
        "reference": "PAY",
        "scale": 0
      }
    },
    {
      "account": "bob",
      "amount": "0",
      "asset": {
        "issuer": "continuation-escrow-oracle",
        "network": "local",
        "reference": "PAY",
        "scale": 0
      }
    },
    {
      "account": "escrow",
      "amount": "0",
      "asset": {
        "issuer": "continuation-escrow-oracle",
        "network": "local",
        "reference": "PAY",
        "scale": 0
      }
    },
    {
      "account": "observer",
      "amount": "42",
      "asset": {
        "issuer": "continuation-escrow-oracle",
        "network": "local",
        "reference": "PAY",
        "scale": 0
      }
    }
  ],
  "inputs": {
    "amount": {
      "type": "UInt",
      "value": "10"
    }
  },
  "liabilities": [],
  "state": {
    "marker": {
      "type": "UInt",
      "value": "77"
    }
  }
}
````

### C20 examples/core/continuations/delivery-true-resume-complete.json — complete-context-body

````text
{
  "action": "deliver",
  "format": "moriarty-stage-event/1",
  "label": "delivery",
  "values": [
    {
      "name": "ready",
      "value": {
        "type": "Bool",
        "value": true
      }
    }
  ]
}
````

### C21 examples/core/continuations/normal-oracle.json — json-pointer-values

````text
{"/escrow/prefix_waiting/status":"waiting","/escrow/prefix_waiting/committed":5,"/escrow/prefix_waiting/residual":[{"amount":"10","asset":{"issuer":"continuation-escrow-oracle","network":"local","reference":"PAY","scale":0},"creditor":"bob","debtor":"escrow","name":"release"}],"/escrow/prefix_waiting/segment/post":{"balances":[{"account":"alice","amount":"90","asset":{"issuer":"continuation-escrow-oracle","network":"local","reference":"PAY","scale":0}},{"account":"bob","amount":"0","asset":{"issuer":"continuation-escrow-oracle","network":"local","reference":"PAY","scale":0}},{"account":"escrow","amount":"10","asset":{"issuer":"continuation-escrow-oracle","network":"local","reference":"PAY","scale":0}},{"account":"observer","amount":"42","asset":{"issuer":"continuation-escrow-oracle","network":"local","reference":"PAY","scale":0}}],"liabilities":[{"amount":"10","asset":{"issuer":"continuation-escrow-oracle","network":"local","reference":"PAY","scale":0},"creditor":"bob","debtor":"escrow","name":"release"}],"state":{"marker":{"type":"UInt","value":"77"}}},"/escrow/true_resume_complete/status":"complete","/escrow/true_resume_complete/committed":11,"/escrow/true_resume_complete/residual":[],"/escrow/true_resume_complete/segment/post":{"balances":[{"account":"alice","amount":"90","asset":{"issuer":"continuation-escrow-oracle","network":"local","reference":"PAY","scale":0}},{"account":"bob","amount":"10","asset":{"issuer":"continuation-escrow-oracle","network":"local","reference":"PAY","scale":0}},{"account":"escrow","amount":"0","asset":{"issuer":"continuation-escrow-oracle","network":"local","reference":"PAY","scale":0}},{"account":"observer","amount":"42","asset":{"issuer":"continuation-escrow-oracle","network":"local","reference":"PAY","scale":0}}],"liabilities":[{"amount":"0","asset":{"issuer":"continuation-escrow-oracle","network":"local","reference":"PAY","scale":0},"creditor":"bob","debtor":"escrow","name":"release"}],"state":{"marker":{"type":"UInt","value":"77"}}}}
````

### C22 docs/BETA-SOURCE6-EXPANSION.md — exact-line-ranges

````text
LINES 5–11
Existing default command, unchanged:

```sh
mori expand core.mori --action noop --scenario core.json
```

This still validates a Core action against its Core scenario and prints its generic Core Program JSON. A Rust beta file supplied without the new option still reaches the existing Core parser and refuses; no default beta output is replaced. The separate Node beta package and its default JSON expansion output are unchanged.
LINES 20–20
The explicit command reads a typed local transfer scenario and first calls `mori-source6::select_transfer`, which uses the real Rust beta parser and checker, before reading the scenario file. The resulting `SelectedTransfer::expand` is the same emission path used by the `expand_transfer` convenience API. Stdout is the actual generated Source/6 text with its existing final newline; `--quiet` leaves stderr empty. Without `--quiet`, stderr reports `Expanded Source/6 transfer proposal (local-stipulation-only)`. It does not emit a Core success result. Without the build feature, the explicit option reports `mori::command::source6-unavailable`.
LINES 30–30
Boundary: Rust beta/1 → local transfer proposal presentation → Source/6 text. The real experimental TypeScript `parseAndLowerSource6` must still consume actual emitted bytes; its Core/5 preparation remains separately unqualified. This option provides no Source/6/Core/5→Core/1/Core/2/Sherlock/native embedding, no handwritten target IR and no target/proof/post-state success. The default Core and separate Node consumers retain their own profiles.
````

### C23 docs/CORE-CONTINUATIONS.md — exact-line-ranges

````text
LINES 140–149
checks authenticates the continuation or records its consumption.

Every staged JSON report and `/4` manifest carries the Opus-authored labels exactly:

```json
{"qualification":"local-reference-stage","authentication":"none","consumption":"untracked"}
```

The [Opus-authored stage contract](../crates/mori-core/src/stage/mod.rs) records
their scope. Consult the [security ownership record](audits/2026-10-14/SECURITY-ROUTING.md)
LINES 179–197
For delivery, start costs 5 and a true resume costs 6, for 11 committed units.
A false resume reports 2 attempted units while committed work remains 5. A later
true resume still completes at 11 committed units. Reserve has segment costs
11, 15 and 10, for 36 committed units. Attempted work on a false or rejected
invocation is reported separately.

These units are not circuit cost, DUST, service fees or proof measurements.
The unsigned compiler source path emits `/4` staged artifacts with `N + 1` circuits
for N awaits; a false observation has no circuit. Zero-await `/2` actions have
one circuit. The CLI compilation interface is:

```sh
mori compile examples/core/continuations/delivery.mori \
  --target target.json --out new-output-directory
```

The target is supplied separately. Staged manifests and ZKIR source retain their
unperformed qualification gates. The local CLI results above do not establish
native proof or target qualification.
````

### C24 crates/mori/tests/fixtures/source6-expansion/invoice.mori — complete-context-body

````text
profile "moriarty-beta/1";
agreement Invoice {
 domain Preview = { id: "Midnight", chain: "midnight", network: "preview" };
 account Buyer = { domain: Preview, id: "Owner" };
 account Seller = { domain: Preview, id: "Recipient" };
 account Treasury = { domain: Preview, id: "Fee" };
 asset USD = { domain: Preview, id: "A", scale: 2, representation: "canonical", symbol: "USD" };
 const price: Qty<USD> = 10.00 USD;
 const fee = 0.10 USD;
 intent Payment = {
  domain: Preview, asset: USD, signer: Buyer, key: "key1", nonce: "n1", pre_head: "h0",
  valid: rounds(domain: Preview, from: 0, to: 10), gross_cap: price + fee, fee_cap: fee, net_floor: price,
  operation: transfer(from: Buyer, to: Seller, fee_to: Treasury, value: price, fee: fee),
  source_hash: "src1", policy_digest: "policy1", failure: SuccessOnly,
  observations: [], disclosures: [], retained_effects: [], retained_duties: [], delegation: None, recovery: None,
 };
 action pay uses Payment;
}
````

### C25 crates/mori/tests/fixtures/source6-expansion/invoice.scenario.json — complete-context-body

````text
{
  "profile": "moriarty-local-scenario/1",
  "kind": "local-stipulation",
  "domain": "Midnight",
  "asset": "A",
  "head": "h0",
  "predecessor": "genesis",
  "round": "1",
  "balances": [
    { "account": "Owner", "amount": "10000" },
    { "account": "Recipient", "amount": "0" },
    { "account": "Fee", "amount": "0" }
  ],
  "allowance": { "owner": "Owner", "remaining": "10000", "spent": "0" },
  "replay": "unused",
  "work_remaining": "10",
  "work_spent": "0",
  "post_head": "h1"
}
````

### C26 crates/mori/tests/fixtures/source6-expansion/invoice.source6 — complete-context-body

````text
profile "moriarty-financial-agreement-source/6";
agreement Invoice {
domain Midnight;
settlement A scale 2;
selected TransferLiteralFee source_hash "src1" digest "policy1";
intent {
signer Owner key "key1";
nonce "n1";
pre_head "h0";
valid 0..10;
gross_cap 1010;
fee_cap 10;
net_floor 1000;
failure success_only;
signed_action transfer from Owner to Recipient fee_to Fee value 1000 fee 10;
observations empty;
disclosures empty;
retained_effects empty;
retained_duties empty;
delegation none;
recovery none;
}
authenticated {
head "h0";
predecessor "genesis";
round 1;
balance Owner 10000;
balance Recipient 0;
balance Fee 0;
allowance Owner remaining 10000 spent 0;
replay unused;
work_remaining 10;
work_spent 0;
}
submit transfer from Owner to Recipient fee_to Fee value 1000 fee 10;
effects {
debit Owner 1010;
credit Recipient 1000;
credit Fee 10;
use_allowance Owner 1010;
use_replay "n1";
advance_head "h0" "h1";
}
post_head "h1";
}
````

### C27 packages/moriarty-beta/package.json — complete-context-body

````text
{
  "name": "@moriarty-lang/beta",
  "version": "0.1.0-beta.1",
  "type": "module",
  "description": "Bounded Moriarty authoring and unqualified local S0 preparation",
  "engines": {
    "node": ">=24"
  },
  "bin": {
    "mori": "dist/cli.js"
  },
  "exports": {
    ".": {
      "types": "./dist/types/packages/moriarty-beta/src/index.d.ts",
      "import": "./dist/index.js"
    }
  },
  "files": [
    "dist",
    "README.md",
    "GETTING-STARTED.md",
    "SIGNED-INTENT.md",
    "editor",
    "ai",
    "examples"
  ],
  "scripts": {
    "build": "node build.mjs",
    "test": "node --test tests/*.test.mjs",
    "typecheck": "tsc --noEmit",
    "prepack": "npm run build && node editor/vscode/prepare.mjs"
  },
  "devDependencies": {
    "esbuild": "0.28.2",
    "typescript": "7.0.2",
    "@types/node": "26.6.3"
  },
  "types": "./dist/types/packages/moriarty-beta/src/index.d.ts"
}
````

### C28 packages/moriarty-beta/GETTING-STARTED.md — exact-line-ranges

````text
LINES 13–25
Use Node 24 or later. From `packages/moriarty-beta` in a Moriarty checkout:

```sh
npm ci
npm run build
node dist/cli.js init /tmp/invoice
node dist/cli.js check /tmp/invoice/invoice.mori
node dist/cli.js test /tmp/invoice
node dist/cli.js simulate /tmp/invoice/invoice.mori --action pay --scenario /tmp/invoice/scenario.json
node dist/cli.js init /tmp/loan --template repay
node dist/cli.js test /tmp/loan
node dist/cli.js simulate /tmp/loan/repayment.mori --action repay_loan --scenario /tmp/loan/scenario.json
```
````

### C29 packages/moriarty-beta/src/cli.ts — exact-line-ranges

````text
LINES 83–86
async function main():Promise<void>{
 const [command,...raw]=process.argv.slice(2);
 if((command==='intent'||command==='verify-intent')&&raw.includes('--review')&&raw.includes('--json'))throw new Error('--review and --json cannot be combined');
 if(!command||command==='help'||command==='--help'){process.stdout.write('Moriarty beta: init DIR [--template transfer|repay] | check FILE [--json] | fmt FILE [--write] | inspect FILE | expand/simulate FILE --action NAME --scenario FILE | test DIR | intent FILE --action NAME --scenario FILE --scheme SCHEME --public-key HEX --framing raw|midnight-sign-data --crypto-binary /ABS/BINARY [--review|--json] | verify-intent FILE --action NAME --scenario FILE --signature FILE --crypto-binary /ABS/BINARY [--review|--json] | lsp | mcp\nLocal financial results remain PreparedUnqualified.\n');return;}
LINES 90–102
 if(!Object.hasOwn(allowed,command))throw new Error(`Unknown command ${command}`);
 const positional:string[]=[],options=new Map<string,string|boolean>();
 for(let i=0;i<raw.length;i++){
  const arg=raw[i];if(arg.startsWith('--')){
   if(!allowed[command].includes(arg)||options.has(arg))throw new Error(`Unknown or duplicate option ${arg}`);
   if(['--action','--scenario','--template','--scheme','--public-key','--framing','--crypto-binary','--signature'].includes(arg)){const v=raw[++i];if(!v||v.startsWith('--'))throw new Error(`Missing value for ${arg}`);options.set(arg,v);}else options.set(arg,true);
  }else positional.push(arg);
 }
 if(positional.length!==1)throw new Error(`${command} requires one path`);
 const path=resolve(positional[0]);
 if(command==='init'){
  const template=options.get('--template')??'transfer';if(!['transfer','repay'].includes(String(template)))throw new Error('Template must be transfer or repay');
  const repay=template==='repay',filename=repay?'repayment.mori':'invoice.mori',action=repay?'repay_loan':'pay';
LINES 107–111
  output({status:'Initialized',directory:path,qualification:'local-stipulation-only'});return;
 }
 if(command==='test'){output(runCases(path));return;}
 if((command==='expand'||command==='simulate')&&(!options.has('--action')||!options.has('--scenario')))throw new Error(`${command} requires --action and --scenario`);
 const source=read(path);
LINES 132–137
 const result=command==='check'?check(source):command==='inspect'?inspect(source):command==='expand'?expand(source,options.get('--action') as string,read(resolve(options.get('--scenario') as string))):simulate(source,options.get('--action') as string,read(resolve(options.get('--scenario') as string)));
 if(command==='check'&&!options.has('--json'))readableCheck(source,result as CheckSummary);else output(result);
 const status=(result as {status?:string}).status;
 if(status&&!['AuthoringChecked','Expanded','PreparedUnqualified'].includes(status))process.exitCode=1;
}
main().catch((error)=>{
````

### C30 packages/moriarty-beta/src/bridge.ts — exact-line-ranges

````text
LINES 236–240
export function simulate(source:string,action:string,scenarioText:string):Simulated{
 const expansion=expand(source,action,scenarioText);if(expansion.status!=='Expanded')return expansion;
 const result=prepareSource6S0Unqualified(expansion.source6);
 return {status:result.status,sourceHash:expansion.sourceHash,scenarioHash:expansion.scenarioHash,result,qualification:'local-stipulation-only'};
}
````

### C31 packages/moriarty-beta/src/index.ts — exact-line-ranges

````text
LINES 1–5
/** Public APIs take text. This module has no CLI or network side effects. */
import { analyze, check, format, type Value } from './frontend.ts';
import { LocalError } from './json.ts';
export { check, format };
export { expand, simulate } from './bridge.ts';
````

### C32 packages/moriarty-beta/tests/cli.test.mjs — exact-line-ranges

````text
LINES 56–64
test('repayment template has independent complete money expectations',()=>{
 const parent=mkdtempSync(join(tmpdir(),'mori-repay-init-')),dir=join(parent,'repay');
 try{let r=run(['init',dir,'--template','repay']);assert.equal(r.status,0,r.stderr);
 r=run(['test',dir]);assert.equal(r.status,0,r.stderr);assert.equal(JSON.parse(r.stdout).status,'TestsPassed');
 const m=JSON.parse(readFileSync(join(dir,'mori.tests.json'),'utf8'));
 assert.equal(m.cases[0].expect.post.obligations[0].principal,'98000');
 assert.equal(m.cases[0].expect.post.balances[0].amount,'197000');
 }finally{rmSync(parent,{recursive:true,force:true});}
});
````

### C33 packages/moriarty-beta/examples/local/repay/repayment.mori — complete-context-body

````text
profile "moriarty-beta/1";
agreement LoanAgreement {
 domain Preview = { id: "Midnight", chain: "midnight", network: "preview" };
 account Payer = { domain: Preview, id: "Payer" };
 account Creditor = { domain: Preview, id: "Creditor" };
 asset USD = { domain: Preview, id: "A", scale: 2, representation: "canonical" };
 obligation Debt = { domain: Preview, asset: USD, id: "Loan" };
 const amount = 30.00 USD;
 intent Repayment = { domain: Preview, asset: USD, signer: Payer, key: "key1", nonce: "n1", pre_head: "h0",
 valid: rounds(domain: Preview, from: 0, to: 10), gross_cap: amount, fee_cap: 0.00 USD, net_floor: 0.00 USD,
 operation: repay(obligation: Debt, payer: Payer, amount: amount), source_hash: "src1", policy_digest: "policy1",
 failure: SuccessOnly, observations: [], disclosures: [], retained_effects: [], retained_duties: [], delegation: None, recovery: None };
 action repay_loan uses Repayment;
}
````

### C34 packages/moriarty-beta/examples/local/repay/scenario.json — complete-context-body

````text
{
  "profile": "moriarty-local-scenario/1",
  "kind": "local-stipulation",
  "domain": "Midnight",
  "asset": "A",
  "head": "h0",
  "predecessor": "genesis",
  "round": "1",
  "balances": [
    {
      "account": "Payer",
      "amount": "200000"
    },
    {
      "account": "Creditor",
      "amount": "0"
    }
  ],
  "allowance": {
    "owner": "Payer",
    "remaining": "200000",
    "spent": "0"
  },
  "replay": "unused",
  "work_remaining": "10",
  "work_spent": "0",
  "post_head": "h1",
  "obligation": {
    "id": "Loan",
    "debtor": "Payer",
    "creditor": "Creditor",
    "asset": "A",
    "principal": "100000",
    "accrued": "1000",
    "outstanding": "101000",
    "status": "Outstanding"
  }
}
````

### C35 packages/moriarty-beta/examples/local/repay/mori.tests.json — complete-context-body

````text
{
  "profile": "moriarty-beta-tests/1",
  "cases": [
    {
      "name": "interest first partial repayment",
      "source": "repayment.mori",
      "action": "repay_loan",
      "scenario": "scenario.json",
      "expect": {
        "status": "PreparedUnqualified",
        "effects": [
          {
            "kind": "Debit",
            "account": "Payer",
            "asset": "A",
            "amount": "3000"
          },
          {
            "kind": "Credit",
            "account": "Creditor",
            "asset": "A",
            "amount": "3000"
          },
          {
            "kind": "SetObligation",
            "id": "Loan",
            "principal": "98000",
            "accrued": "0",
            "outstanding": "98000",
            "status": "Outstanding"
          },
          {
            "kind": "UseAllowance",
            "owner": "Payer",
            "amount": "3000"
          },
          {
            "kind": "UseReplay",
            "key": "[\"Midnight\",\"Payer\",\"n1\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "h0",
            "successor": "h1"
          }
        ],
        "post": {
          "core": "moriarty-core/5",
          "domain": "Midnight",
          "asset": "A",
          "head": "h1",
          "round": "1",
          "workRemaining": "9",
          "workSpent": "1",
          "balances": [
            {
              "account": "Payer",
              "amount": "197000"
            },
            {
              "account": "Creditor",
              "amount": "3000"
            }
          ],
          "allowances": [
            {
              "owner": "Payer",
              "remaining": "197000",
              "spent": "3000"
            }
          ],
          "obligations": [
            {
              "id": "Loan",
              "debtor": "Payer",
              "creditor": "Creditor",
              "asset": "A",
              "principal": "98000",
              "accrued": "0",
              "outstanding": "98000",
              "status": "Outstanding"
            }
          ],
          "consumedReplay": [
            "[\"Midnight\",\"Payer\",\"n1\"]"
          ]
        }
      }
    }
  ]
}
````

### C36 crates/mori/src/json.rs — exact-line-ranges

````text
LINES 14–25
pub struct CheckReport {
    /// `accepted` when there are no diagnostics, otherwise `rejected`.
    pub status: &'static str,
    pub file: String,
    /// SHA-256 of the file's bytes, in lowercase hex.
    pub source_hash: String,
    /// Absent when the file did not parse.
    pub agreement: Option<Named>,
    pub declarations: Vec<DeclarationReport>,
    pub actions: Vec<ActionReport>,
    pub diagnostics: Vec<DiagnosticReport>,
}
LINES 58–67
pub struct ActionReport {
    pub name: String,
    pub intent: String,
    /// `local_s0` when it runs locally, `specified_only` when it is only
    /// checked; absent when its intent did not check.
    pub support: Option<&'static str>,
    pub span: Span,
    pub name_span: Span,
}

````

### C37 crates/mori/src/commands/mod.rs — complete-context-body

````text
//! One module per subcommand. Each defines its `Args` and a `run` function.

use std::process::ExitCode;

use mori_diagnostics::Report;

use crate::cli::Command;
use crate::shell::Shell;

mod call_io;
pub mod capabilities;
pub mod check;
mod check_source;
pub mod compile;
pub mod compile_call;
mod core_io;
pub mod expand;
pub mod format;
pub mod init;
pub mod inspect;
pub mod intent;
pub mod keygen_call;
pub mod keygen_signed_call;
pub mod lsp;
pub mod mcp;
pub mod prepare_call;
pub mod prepare_signed_call;
mod proof_io;
pub mod prove;
pub mod prove_call;
pub mod prove_signed_call;
pub mod replay;
mod signed_call_io;
pub mod simulate;
pub mod test;
pub mod verify;
pub mod verify_call;
pub mod verify_intent;
pub mod verify_signed_call;

pub type Result<T> = std::result::Result<T, Report>;

pub fn run(command: Command, shell: &mut Shell) -> Result<ExitCode> {
    match command {
        Command::Capabilities(args) => capabilities::run(args, shell),
        Command::Init(args) => init::run(args, shell),
        Command::Check(args) => check::run(args, shell),
        Command::Compile(args) => compile::run(args, shell),
        Command::KeygenSignedCall(args) => keygen_signed_call::run(args, shell),
        Command::PrepareSignedCall(args) => prepare_signed_call::run(args, shell),
        Command::ProveSignedCall(args) => prove_signed_call::run(args, shell),
        Command::VerifySignedCall(args) => verify_signed_call::run(args, shell),
        Command::KeygenCall(args) => keygen_call::run(args, shell),
        Command::ProveCall(args) => prove_call::run(args, shell),
        Command::VerifyCall(args) => verify_call::run(args, shell),
        Command::CompileCall(args) => compile_call::run(args, shell),
        Command::PrepareCall(args) => prepare_call::run(args, shell),
        Command::Prove(args) => prove::run(args, shell),
        Command::Verify(args) => verify::run(args, shell),
        Command::Format(args) => format::run(args, shell),
        Command::Inspect(args) => inspect::run(args, shell),
        Command::Expand(args) => expand::run(args, shell),
        Command::Simulate(args) => simulate::run(args, shell),
        Command::Replay(args) => replay::run(args, shell),
        Command::Test(args) => test::run(args, shell),
        Command::Intent(args) => intent::run(args, shell),
        Command::VerifyIntent(args) => verify_intent::run(args, shell),
        Command::Lsp(args) => lsp::run(args, shell),
        Command::Mcp(args) => mcp::run(args, shell),
    }
}

#[cfg(not(feature = "native"))]
fn native_unavailable<T>() -> Result<T> {
    Err(crate::error::CliError::Operation {
        code: "mori::command::native-unavailable".into(),
        message: "This mori build has no Midnight support. Rebuild with the native feature enabled."
            .into(),
    }
    .into())
}
````

### C38 crates/mori/src/commands/core_io.rs — exact-line-ranges

````text
LINES 237–252
pub fn read_json<T: DeserializeOwned>(path: &Path) -> super::Result<T> {
    let source = read_source(path)?;
    let value = mori_core::json::parse(&source).map_err(operation)?;
    serde_json::from_value(value).map_err(operation)
}
pub fn program(path: &Path) -> super::Result<(String, mori_core::Program)> {
    let source = read_source(path)?;
    let program = mori_core::parse(&source).map_err(operation)?;
    Ok((source, program))
}
pub fn print<T: Serialize>(value: &T) -> super::Result<()> {
    let display = mori_core::json::export(value).map_err(operation)?;
    println!("{display}");
    Ok(())
}
/// Trusted signed Target, separately supplied expected authority and logical time.
````

### C39 crates/mori-backend/src/stage/mod.rs — complete-context-body

````text
//! Staged `moriarty/2` compilation to `/4` artifacts: N + 1 successful-segment
//! circuits per action with N awaits, through the ordinary `/3` lowerer.
//! `/4` is a distinct type with no conversion to the `/3` Artifact, so proving,
//! signed and native consumers cannot accept it. Unsigned only: there is no
//! event authentication, stage authority or consumption, and all ten gates
//! remain unperformed.
mod artifact;
pub(crate) mod context;
pub(crate) mod frame;
mod lower;

pub use artifact::{
    ActionLogical, LOGICAL_ENFORCEMENT, MAX_STAGED_CIRCUITS, PendingPositions,
    STAGE_CONTEXT_FORMAT, STAGE_GATES, STAGED_ABI_VERSION, STAGED_ARTIFACT_FORMAT,
    STAGED_RELATION_VERSION, SegmentLogical, StagedArtifact, StagedCircuit, StagedResources,
};
pub use lower::compile_staged;

use crate::{Error, error};
use mori_core::Program;

/// Explicit `/1` entry refusal for legacy backend consumers, with the existing
/// `mori::backend::program` wrapping of Core's profile error.
pub(crate) fn require_core1(program: &Program) -> Result<(), Error> {
    mori_core::stage::require_core1(program).map_err(|e| error("program", e.to_string()))
}

/// Typed refusal for an Await reaching a `/1`-only statement match.
pub(crate) fn core1_refusal(program: &Program) -> Error {
    match require_core1(program) {
        Err(e) => e,
        Ok(()) => error(
            "program",
            "mori::core::profile: Await statements require the moriarty/2 staged compiler.",
        ),
    }
}
````

### C40 crates/mori/src/commands/check_source.rs — complete-context-body

````text
//! Shared source-profile dispatch and the existing check report formats.

use mori_diagnostics::MoriDiagnostic;

use crate::json::CheckReport;

/// Consume checked values while the Beta AST they borrow is still alive.
/// Reports remain optional: a human consumer can render diagnostics directly.
pub(super) trait SourceCheckConsumer {
    type Output;

    /// Called after Beta parsing, before its checker starts.
    fn parsed(&mut self, _tokens: usize, _nodes: usize) {}

    fn beta(
        self,
        checked: Option<&mori_checker::Checked<'_>>,
        diagnostics: Vec<MoriDiagnostic>,
    ) -> Self::Output;

    fn core(
        self,
        result: Result<mori_core::Program, mori_core::Error>,
        staged: bool,
    ) -> Self::Output;
}

/// Select exactly the same Beta, Core1 or staged Core2 path as `mori check`.
pub(super) fn dispatch<C: SourceCheckConsumer>(source: &str, mut consumer: C) -> C::Output {
    if mori_lexer::first_token_text(source).is_ok_and(|token| token == "language") {
        let staged = mori_core::language(source).is_ok_and(|language| language == "moriarty/2");
        let result = if staged {
            mori_core::stage::load(source).map(|program| program.program().clone())
        } else {
            mori_core::parse(source)
        };
        return consumer.core(result, staged);
    }
    match mori_parser::parse(source) {
        Err(error) => consumer.beta(None, vec![error]),
        Ok(ast) => {
            consumer.parsed(ast.tokens.len(), ast.nodes.len());
            let mori_checker::CheckResult {
                checked,
                diagnostics,
            } = mori_checker::check(&ast);
            consumer.beta(Some(&checked), diagnostics)
        }
    }
}

/// Preserve the Beta struct's serialization order and diagnostic/span mapping.
pub(super) fn beta_report(
    file: String,
    source: &str,
    checked: Option<&mori_checker::Checked<'_>>,
    diagnostics: &[MoriDiagnostic],
) -> CheckReport {
    CheckReport::new(file, source, checked, diagnostics)
}

/// Construct the existing Core JSON object only when a consumer requests it.
pub(super) fn core_report(
    file: &str,
    source: &str,
    result: &Result<mori_core::Program, mori_core::Error>,
    staged: bool,
) -> serde_json::Value {
    let mut report = match result {
        Ok(program) => {
            serde_json::json!({"status":"accepted", "file":file,"source_hash":mori_core::intent::hash_bytes(source.as_bytes()),"language":program.language,"agreement":program.name,"actions":program.actions,"diagnostics":[]})
        }
        Err(error) => {
            serde_json::json!({"status":"rejected","file":file,"diagnostics":[error]})
        }
    };
    if staged {
        let object = report
            .as_object_mut()
            .expect("stage reports are JSON objects");
        for (name, value) in mori_core::stage::STAGE_LABELS {
            object.insert(name.into(), serde_json::Value::String(value.into()));
        }
    }
    report
}
````

## New exact terminal receipts

Intake occurred only after root reported tool77398 terminal0. Source/context hash matching here is transport verification, not extra execution.

New actual evidence: one selected Clap pass in 56.05499841499841 seconds, zero failed/ignored/measured, seven filtered. Five bash -n checks passed with empty streams. All six owned-session arrays are empty; final existsAfter=false. Exact current CHECK-PLAN/FINAL-RESULT and per-check receipts are physical; no historical runtime transferred.

Actual selected argv:

````json
["/usr/bin/timeout", "--signal=TERM", "--kill-after=5s", "90s", "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo", "test", "--manifest-path", "/home/charl/Moriarty/.worktrees/utxo-core2-complete-20261018/Cargo.toml", "--frozen", "-j1", "-p", "mori", "--no-default-features", "--bin", "mori", "cli::tests::help_distinguishes_core_consumers_from_source6_proposals", "--", "--exact", "--test-threads=1"]
````

Complete actual selected Clap stdout:

````text

running 1 test
test cli::tests::help_distinguishes_core_consumers_from_source6_proposals ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

````

Clap stderr is nonempty build progress; complete physical stream SHA256 3549cbb2b35e97d6453556a7a2be39ed6d3c3276b2080213f562151c66b403ab. Final two lines:

````text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 56.03s
     Running unittests src/main.rs (/tmp/doctorwatson-cli-help-0vl4mjnv/target/debug/deps/mori-beaeadcc1a61f15a)
````

All CHECK-PLAN/SOURCE-PINS/FINAL-RESULT and all six START/CHILD/RESULT/full stdout/stderr exports are physical under actual-receipts/. The full actual driver is ACTUAL-CHECK-DRIVER.py. Root pin metadata entries are not claimed as bodies read. Source-preparation/ preserves earlier source-derived expectations and five exact shell inputs unchanged, including historical UNRUN/main79 labels. Only current selected metadata test and shell syntax have new runtime evidence. No documented product command, Node beta, extraction script, install, compiler/SDK/native/proof/ledger example was run. Assess command/context claims from actual sources; identify any unsatisfied predicate. Report actual quotation ranges/JSON pointers and any extra bodies read, plus unread/insufficient scope. Full source bodies are available but not presumed read. All U0–U7/M1–M17/MC/SP/G/148requirements/292scenarios/20families/finance/composition/native/target and security-owner limits remain.
