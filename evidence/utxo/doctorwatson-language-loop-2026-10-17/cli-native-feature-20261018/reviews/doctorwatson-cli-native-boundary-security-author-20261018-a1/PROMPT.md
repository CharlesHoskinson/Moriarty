You are designated security-author OUTAGE FALLBACK Gemini4ArgonHIGH, neverOpus, afteractualOpus4.8rate rejection06:05:37UTC. No recoveryobserved. This is SOURCE AUTHORING, NOT approval/admission/securityreview. Primarybaseline+acceptedcontract INLINE. Onlyoptionalreadonlypacket /home/charl/moriarty-c2-opus55-restored-20261017/cli-native-cycle/security-test-author-packet. Do notexecutecommands/tests/Cargo/CLI/SDK/models/network or editfiles/readoutside/private/config/caches/provider/key/wallet. ReturnONE completeobjectschema token doctorwatson-cli-native-boundary-security-author-20261018-a1-whole-source-proposal with fullrustSource and completecatalog/derivation/limits; no truncatedproposals. Usefullindependentacceptedcontract, actualbaselineClapArgs/fieldnames/error/main API only; noinventedflags. No candidateimpl/actualoutputs read. Alloriginalgoal/resource/financial/proof/ledgerexits stayopen.

CONTRACT
Independently author test source and literal command catalog for accepted CLI feature design, without seeing new candidate refusal implementations. Baseline is main2c2b9182/tree61bc8233 (before CLI featurepatch). Userauthorizes sourceprep; defaultnative preserved, explicitno-defaultCoreCLInewconfig. In NO-NATIVE build only: allnative-onlycommandvalidinvocations refusecode mori::command::native-unavailable andfixedmessage This mori build has no Midnight support. Rebuild with the native feature enabled.,emptystdout,stderrdiagnostic,exit1 beforeopening namedinput/output. Fullclap schemas/flags remain; parse/conflicts exit2 beforedispatch. Mixednativepaths: inspect --target refusesavailability BEFORE sourceI/O evenCore2; simulate --trace keepsmissingtargetargumenterrorfirst thenvalidtrace target refusesbeforeinputI/O; target/signedargswithouttrace keepoldargumentrefusal; intent JubJub --profile retains target+at companioncheckbeforeavailability; verify-intent profile/targetgroupchecks likewise. UnauthenticatedlocalCore routes remainunchanged. No silentfallback or nativefeaturetest invocation. Default/nativeavailability tests mustNOT run (gatewholeintegrationfile #![cfg(not(feature = "native"))]); nolivekeys/params/signatures used. No new cryptographic hostilecasesrequired. Source expects only ordinaryfeatureavailability, actual no-read-guarantee limits explicit: missingpaths+unchangedsentinels andsourceaudit do not prove syscallabsence, no suchclaim. Never snapshot--help.
Return complete security-authored crates/mori/tests/native_feature_boundary.rs asliteralstring plus literal casesJSON identifyingevery testedargv,exit/stdout/errorcode/IO invariant and derivation fromdesign+baselineArgs. Use stdonly+existingdeps. RealCARGO_BIN_EXE_mori subprocess permittedunderproposednarrowguidelineexception; all pathsrelativeunder ownedtempdir, create/cleanup onlyownedtempdir/sentinels. No externalpaths/keys/wallet/auth/SRS/PK/provercache. SourceONLY; noCLI/Tests/Cargo/compiler/model/network/SDKexecution; nofilesystemoutsidepacket. Freeze expectedcatalog before any sourceconsumercheck; do notderivefromcandidate oractualoutput. No patchedproductioncodeprovided. Describe alluncoveredroutes/exactstderrformat/fullsyscallno-read/defaultnative/proof/target gaps. This authorcontext isnotapproval; needs fresh nonauthor applicableaudits.

BASELINE crates/mori/src/main.rs
use std::process::ExitCode;

use clap::Parser;

use crate::cli::Cli;
use crate::shell::Shell;

mod cli;
mod commands;
mod display;
mod error;
mod json;
mod shell;
mod styles;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let mut shell = Shell::new(cli.global.verbosity(), cli.global.color.into());

    match commands::run(cli.command, &mut shell) {
        Ok(code) => code,
        Err(report) => {
            shell.report(&report);
            ExitCode::FAILURE
        }
    }
}

END BASELINE

BASELINE crates/mori/src/cli.rs
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::commands::{
    check, compile, compile_call, expand, format, init, inspect, intent, keygen_call,
    keygen_signed_call, lsp, mcp, prepare_call, prepare_signed_call, prove, prove_call,
    prove_signed_call, replay, simulate, test, verify, verify_call, verify_intent,
    verify_signed_call,
};
use crate::shell::Verbosity;
use crate::styles;

/// Moriarty: a financial agreement language for Midnight.
#[derive(Debug, Parser)]
#[command(name = "mori", version, styles = styles::CLAP)]
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
    /// Create a new project from a starter template
    Init(init::Args),
    /// Check syntax, names, types and action support
    Check(check::Args),
    /// Compile a bounded Moriarty program into ledger-9 ZKIRv3 circuits
    Compile(compile::Args),
    /// Compile a source-static ordinary native contract-call relation
    CompileCall(compile_call::Args),
    /// Derive all source action keys and an explicit local installed snapshot
    KeygenCall(keygen_call::Args),
    /// Derive the complete local signed action map under a fixed JubJub policy
    KeygenSignedCall(keygen_signed_call::Args),
    /// Reconstruct a signature-free canonical local call signing package
    PrepareSignedCall(prepare_signed_call::Args),
    /// Prove the actual local typed SDK call with an external FENS attachment
    ProveSignedCall(prove_signed_call::Args),
    /// Cold public-only reconstruction and Real verification of a signed call
    VerifySignedCall(verify_signed_call::Args),
    /// Cold rederive keys and prove an actual local installed SDK call
    ProveCall(prove_call::Args),
    /// Cold rederive public keys and Real-verify an installed call without PK files
    VerifyCall(verify_call::Args),
    /// Prove a public local action using pinned offline parameters
    Prove(prove::Args),
    /// Prepare an unsigned, unproven local native call and public offer
    PrepareCall(prepare_call::Args),
    /// Verify a native proof against the expected source, target and scenario
    Verify(verify::Args),
    /// Format source files in place
    #[command(visible_alias = "fmt")]
    Format(format::Args),
    /// Show identity claims, operative bounds and open premises
    Inspect(inspect::Args),
    /// Show the native core IR after checking an action against a scenario
    Expand(expand::Args),
    /// Run an action locally against a scenario
    Simulate(simulate::Args),
    /// Independently replay a public record against separately trusted inputs
    Replay(replay::Args),
    /// Run the test cases in a project directory
    Test(test::Args),
    /// Produce the owner signing bytes for an action
    Intent(intent::Args),
    /// Verify a signature, then prepare the action locally
    VerifyIntent(verify_intent::Args),
    /// Run the language server over stdio
    Lsp(lsp::Args),
    /// Run the read-only MCP server over stdio
    Mcp(mcp::Args),
}

END BASELINE

BASELINE crates/mori/src/error.rs
//! Command failures that do not point at source code.

use std::io;
use std::path::PathBuf;

use mori_diagnostics::Diagnostic;

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("{code}: {message}")]
    Operation { code: String, message: String },
    #[error("could not read `{}`", crate::display::text(&path.display().to_string()))]
    Read {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("could not write `{}`", crate::display::text(&path.display().to_string()))]
    Write {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("`{}` does not exist", crate::display::text(&path.display().to_string()))]
    NotFound { path: PathBuf },
    #[error("could not list `{}`", crate::display::text(&path.display().to_string()))]
    Walk {
        path: PathBuf,
        #[source]
        source: ignore::Error,
    },
}

impl Diagnostic for CliError {}

END BASELINE

BASELINE crates/mori/src/shell.rs
//! Cargo-style status output on stderr.

use std::fmt::Display;
use std::io::{self, Stderr, Write};

use anstream::{AutoStream, ColorChoice};
use anstyle::Style;
use mori_diagnostics::Report;
use mori_diagnostics::render::{self, WIDTH};

use crate::styles;

/// Width of the right-aligned verb column, matching cargo.
const VERB_WIDTH: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    Quiet,
    Normal,
    Verbose,
}

/// Writes status lines such as `    Checking invoice.mori` to stderr.
///
/// Command results go to stdout and never pass through the shell. Output is
/// best effort: if stderr cannot be written, there is nowhere to report it.
pub struct Shell {
    err: AutoStream<Stderr>,
    verbosity: Verbosity,
}

impl Shell {
    pub fn new(verbosity: Verbosity, color: ColorChoice) -> Self {
        Self {
            err: AutoStream::new(io::stderr(), color),
            verbosity,
        }
    }

    /// Prints a right-aligned green verb followed by a message.
    pub fn status(&mut self, verb: &str, message: impl Display) {
        if self.verbosity != Verbosity::Quiet {
            self.aligned(verb, styles::HEADER, message);
        }
    }

    /// Like [`Shell::status`], but only under `--verbose`.
    pub fn verbose_status(&mut self, verb: &str, message: impl Display) {
        if self.verbosity == Verbosity::Verbose {
            self.aligned(verb, styles::HEADER, message);
        }
    }

    #[expect(dead_code, reason = "used once a command prints notes")]
    pub fn note(&mut self, message: impl Display) {
        if self.verbosity != Verbosity::Quiet {
            self.prefixed("note", styles::HEADER, message);
        }
    }

    #[expect(dead_code, reason = "used once a command emits warnings")]
    pub fn warn(&mut self, message: impl Display) {
        if self.verbosity != Verbosity::Quiet {
            self.prefixed("warning", styles::WARN, message);
        }
    }

    /// Prints an error line. Errors are shown even under `--quiet`.
    pub fn error(&mut self, message: impl Display) {
        self.prefixed("error", styles::ERROR, message);
    }

    /// Renders an error report, wrapped to the terminal width up to 80 columns.
    /// Errors are shown even under `--quiet`.
    pub fn report(&mut self, report: &Report) {
        let width = terminal_size::terminal_size_of(io::stderr())
            .map_or(WIDTH, |(width, _)| usize::from(width.0));
        let _ = writeln!(self.err, "{}", render::render_colored(report, width));
    }

    /// Prints `hunks` as `cargo fmt --check` does, indented to line up with a
    /// diagnostic's message.
    pub fn diff(&mut self, file: &str, hunks: &[mori_fmt::Hunk]) {
        const INDENT: &str = "    ";
        for hunk in hunks {
            let _ = writeln!(self.err, "{INDENT}Diff in {file}:{}:", hunk.line);
            for (change, line) in &hunk.lines {
                let (sign, style) = match change {
                    mori_fmt::Change::Context => (' ', Style::new()),
                    mori_fmt::Change::Removed => ('-', styles::REMOVED),
                    mori_fmt::Change::Added => ('+', styles::ADDED),
                };
                let _ = writeln!(self.err, "{INDENT}{style}{sign}{line}{style:#}");
            }
        }
        let _ = writeln!(self.err);
    }

    fn aligned(&mut self, verb: &str, style: Style, message: impl Display) {
        let message = crate::display::text(&message.to_string());
        let _ = writeln!(self.err, "{style}{verb:>VERB_WIDTH$}{style:#} {message}");
    }

    fn prefixed(&mut self, label: &str, style: Style, message: impl Display) {
        let message = crate::display::text(&message.to_string());
        let _ = writeln!(self.err, "{style}{label}:{style:#} {message}");
    }
}

END BASELINE

BASELINE crates/mori/tests/core_continuations.rs
fn diagnostic(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr)
        .split_whitespace()
        .filter(|w| !matches!(*w, "×" | "│"))
        .collect::<Vec<_>>()
        .join(" ")
}
END BASELINE

BASELINE crates/mori/src/commands/compile.rs
use super::core_io;
use crate::{error::CliError, shell::Shell};
use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

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
END BASELINE

BASELINE crates/mori/src/commands/compile_call.rs
use super::{core_io, proof_io};
use crate::{error::CliError, shell::Shell};
use std::{
    io::Read,
    path::{Path, PathBuf},
    process::ExitCode,
};

#[derive(Debug, clap::Args)]
pub struct Args {
    pub file: PathBuf,
    #[arg(long)]
    pub action: Option<String>,
    #[arg(long)]
    pub deployment: Option<PathBuf>,
    /// New output directory, containing tagged IR and static manifest
    #[arg(long)]
    pub out: Option<PathBuf>,
    #[arg(long)]
    pub all_actions: bool,
    #[arg(long)]
    pub static_bindings: Option<PathBuf>,
    #[arg(long)]
    pub authorization: Option<String>,
    #[arg(long)]
    pub signer_policy: Option<PathBuf>,
    #[arg(long)]
    pub work_limits: Option<PathBuf>,
    #[arg(long)]
    pub inspect_static: Option<PathBuf>,
}
pub(crate) fn artifact_bytes(path: &Path) -> super::Result<Vec<u8>> {
    let file = std::fs::File::open(path).map_err(|source| CliError::Read {
        path: path.into(),
        source,
    })?;
    let mut bytes = vec![];
    file.take((mori_ledger::codec::OUTPUT_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|source| CliError::Read {
            path: path.into(),
            source,
        })?;
    if bytes.len() > mori_ledger::codec::OUTPUT_BYTES {
        return Err(core_io::operation("relation artifact exceeds output bound"));
    }
    Ok(bytes)
}
END BASELINE

BASELINE crates/mori/src/commands/keygen_call.rs
use super::{call_io, core_io};
use crate::shell::Shell;
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    pub file: PathBuf,
    #[arg(long)]
    pub seed_deployment: PathBuf,
    #[arg(long)]
    pub seed_state: PathBuf,
    #[arg(long)]
    pub parameters: PathBuf,
    #[arg(long)]
    pub action: String,
    #[arg(long)]
    pub params_dir: PathBuf,
    #[arg(long)]
    pub work_limits: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
}
END BASELINE

BASELINE crates/mori/src/commands/keygen_signed_call.rs
use super::{call_io, core_io};
use crate::shell::Shell;
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    pub file: PathBuf,
    #[arg(long)]
    pub signer_policy: PathBuf,
    #[arg(long)]
    pub seed_deployment: PathBuf,
    #[arg(long)]
    pub seed_state: PathBuf,
    #[arg(long)]
    pub parameters: PathBuf,
    #[arg(long)]
    pub action: String,
    #[arg(long)]
    pub params_dir: PathBuf,
    #[arg(long)]
    pub work_limits: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
}
END BASELINE

BASELINE crates/mori/src/commands/prepare_call.rs
use super::{core_io, proof_io};
use crate::{error::CliError, shell::Shell};
use std::{
    io::Read,
    path::{Path, PathBuf},
    process::ExitCode,
};
#[derive(Debug, clap::Args)]
pub struct Args {
    pub file: PathBuf,
    #[arg(long)]
    pub action: String,
    #[arg(long)]
    pub deployment: PathBuf,
    #[arg(long)]
    pub predecessor: PathBuf,
    #[arg(long)]
    pub parameters: PathBuf,
    #[arg(long)]
    pub request: PathBuf,
    /// Source-static native relation directory produced by compile-call
    #[arg(long)]
    pub relation: Option<PathBuf>,
    #[arg(long)]
    pub out: PathBuf,
}
fn binary(path: &Path) -> super::Result<Vec<u8>> {
    let file = std::fs::File::open(path).map_err(|source| CliError::Read {
        path: path.into(),
        source,
    })?;
    let mut bytes = Vec::new();
    file.take((mori_ledger::codec::INPUT_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|source| CliError::Read {
            path: path.into(),
            source,
        })?;
    if bytes.len() > mori_ledger::codec::INPUT_BYTES {
        return Err(core_io::operation("native public input exceeds 1 MiB"));
    }
    Ok(bytes)
}
END BASELINE

BASELINE crates/mori/src/commands/prepare_signed_call.rs
use super::core_io;
use crate::shell::Shell;
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: super::signed_call_io::Input,
    #[arg(long)]
    pub public_keyset: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
}
END BASELINE

BASELINE crates/mori/src/commands/prove.rs
use super::{core_io, proof_io};
use crate::shell::Shell;
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: proof_io::Input,
    /// New proof bundle file; refuses to overwrite existing files
    #[arg(long)]
    pub out: PathBuf,
}
END BASELINE

BASELINE crates/mori/src/commands/prove_call.rs
use super::{call_io, core_io};
use crate::shell::Shell;
use rand::SeedableRng;
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: call_io::Input,
    #[arg(long)]
    pub keyset: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
}
END BASELINE

BASELINE crates/mori/src/commands/prove_signed_call.rs
use super::{call_io, core_io};
use crate::shell::Shell;
use rand::SeedableRng;
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: super::signed_call_io::Input,
    #[arg(long)]
    pub keyset: PathBuf,
    #[arg(long)]
    pub signed_attachment: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
}
END BASELINE

BASELINE crates/mori/src/commands/verify.rs
use super::{core_io, proof_io};
use crate::shell::Shell;
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: proof_io::Input,
    /// Native proof bundle to verify without modifying it
    #[arg(long)]
    pub proof: PathBuf,
}
END BASELINE

BASELINE crates/mori/src/commands/verify_call.rs
use super::{call_io, core_io};
use crate::shell::Shell;
use std::{io::Write, path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: call_io::Input,
    #[arg(long)]
    pub public_keyset: PathBuf,
    #[arg(long)]
    pub proof: PathBuf,
}
END BASELINE

BASELINE crates/mori/src/commands/verify_signed_call.rs
use super::core_io;
use crate::shell::Shell;
use std::{io::Write, path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: super::signed_call_io::Input,
    #[arg(long)]
    pub public_keyset: PathBuf,
    #[arg(long)]
    pub proof: PathBuf,
    #[arg(long)]
    pub signed_attachment: PathBuf,
}
END BASELINE

BASELINE crates/mori/src/commands/replay.rs
//! Actual CLI replay consumer; all roots and signature attachment are separate inputs.
use crate::{
    cli::{ActionInput, SignedProfile},
    shell::Shell,
};
use mori_backend::{
    Target,
    authorization::{ExpectedAuthority, SignedIntent},
    replay::{ReplayAuthorization, ReplayError, ReplayRecord},
};
use mori_core::UInt;
use std::{path::PathBuf, process::ExitCode};

#[derive(Debug, clap::Args, Default)]
pub struct AuthorizationArgs {
    /// Strict signed owner-intent profile (signed Targets require the complete group)
    #[arg(long)]
    pub profile: Option<SignedProfile>,
    /// Separately trusted expected account/domain authority
    #[arg(long, value_name = "FILE")]
    pub authority: Option<PathBuf>,
    /// Separately trusted logical time (not authenticated ledger time)
    #[arg(long)]
    pub at: Option<u128>,
    /// Actual external signed wrapper; never loaded from the record
    #[arg(long, value_name = "FILE")]
    pub signature: Option<PathBuf>,
}
pub enum LoadedAuthorization {
    Unsigned,
    Signed {
        authority: Box<ExpectedAuthority>,
        at: UInt,
        signature: Box<SignedIntent>,
    },
}
impl AuthorizationArgs {
    pub fn present(&self) -> bool {
        self.profile.is_some()
            || self.authority.is_some()
            || self.at.is_some()
            || self.signature.is_some()
    }
    pub fn load(&self, target: &Target) -> super::Result<LoadedAuthorization> {
        if target.signed_policy.is_none() {
            if self.present() {
                return Err(super::core_io::operation(
                    "Signed replay arguments require a trusted Target with signed_policy",
                ));
            }
            return Ok(LoadedAuthorization::Unsigned);
        }
        let (Some(_), Some(authority), Some(at), Some(signature)) =
            (&self.profile, &self.authority, self.at, &self.signature)
        else {
            return Err(super::core_io::operation(
                "Signed Target requires --profile moriarty-jubjub-owner-intent/1, --authority, --at and --signature together",
            ));
        };
        Ok(LoadedAuthorization::Signed {
            authority: Box::new(super::core_io::authority(authority)?),
            at: UInt(at),
            signature: Box::new(super::core_io::signed_intent(signature)?),
        })
    }
}
impl LoadedAuthorization {
    pub fn as_input(&self) -> ReplayAuthorization<'_> {
        match self {
            Self::Unsigned => ReplayAuthorization::Unsigned,
            Self::Signed {
                authority,
                at,
                signature,
            } => ReplayAuthorization::Signed {
                authority,
                at: *at,
                signature,
            },
        }
    }
}
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: ActionInput,
    /// Separately trusted Target JSON
    #[arg(long, value_name = "FILE")]
    pub target: PathBuf,
    /// Public sidecar to compare; never supplies trusted roots
    #[arg(long, value_name = "FILE")]
    pub record: PathBuf,
    /// Export a bounded local first-divergence diagnostic to a new file
    #[arg(long, value_name = "FILE")]
    pub challenge_out: Option<PathBuf>,
    #[command(flatten)]
    pub authorization: AuthorizationArgs,
}
END BASELINE

BASELINE crates/mori/src/commands/intent.rs
use std::process::ExitCode;

use clap::ValueEnum;

use super::core_io::review_identity;
use crate::cli::{ActionInput, SignedProfile, SigningOutputArgs};
use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: ActionInput,

    /// Legacy host profile: signature scheme of the owner key
    #[arg(long)]
    pub scheme: Option<Scheme>,

    /// Legacy host profile: owner public key, hex encoded
    #[arg(long, value_name = "HEX")]
    pub public_key: Option<String>,

    /// Legacy host profile: how the signing bytes are framed
    #[arg(long)]
    pub framing: Option<Framing>,
    /// Domain, predecessor, nonce, validity and per-asset authority JSON
    #[arg(long)]
    pub authority: std::path::PathBuf,

    /// Signed native owner-intent profile; requires --target and --at
    #[arg(long)]
    pub profile: Option<SignedProfile>,
    /// Trusted Target JSON with signed_policy (signed profile only)
    #[arg(long, value_name = "FILE")]
    pub target: Option<std::path::PathBuf>,
    /// Logical time checked against the validity interval; not signed
    #[arg(long)]
    pub at: Option<u128>,

    #[command(flatten)]
    pub output: SigningOutputArgs,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Scheme {
    #[value(name = "schnorr_bip340")]
    SchnorrBip340,
    #[value(name = "ecdsa_secp256k1_sha256")]
    EcdsaSecp256k1Sha256,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Framing {
    Raw,
    MidnightSignData,
}

pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let legacy = [
        args.scheme.is_some(),
        args.public_key.is_some(),
        args.framing.is_some(),
    ];
    if args.profile.is_some() {
        if legacy.iter().any(|present| *present) {
            return Err(super::core_io::operation(
                "the signed profile refuses legacy --scheme/--public-key/--framing arguments",
            ));
        }
        return signed(args, shell);
    }
    if args.target.is_some() || args.at.is_some() {
        return Err(super::core_io::operation(
            "--target and --at belong to --profile moriarty-jubjub-owner-intent/1; legacy intents never accept a Target",
        ));
    }
    let (Some(scheme), Some(public_key), Some(framing)) =
        (args.scheme, args.public_key.clone(), args.framing)
    else {
        return Err(super::core_io::operation(
            "legacy moriarty-owner-intent/2 requires --scheme, --public-key and --framing together",
        ));
    };
    if !matches!(framing, Framing::Raw) {
        return Err(super::core_io::operation(
            "core/1 intents currently require raw canonical framing",
        ));
    }
    let (source, program) = super::core_io::program(&args.input.file)?;
    let scenario = super::core_io::read_json(&args.input.scenario)?;
    let authority: mori_core::intent::Authorization = super::core_io::read_json(&args.authority)?;
    let scheme = match scheme {
        Scheme::SchnorrBip340 => mori_core::intent::Scheme::SchnorrBip340,
        Scheme::EcdsaSecp256k1Sha256 => mori_core::intent::Scheme::EcdsaSecp256k1Sha256,
    };
    if authority.scheme != scheme || authority.public_key_hex != public_key {
        return Err(super::core_io::operation(
            "signing key or scheme does not match expected authority",
        ));
    }
    let prepared = mori_core::intent::prepare_intent(
        &program,
        &mori_core::intent::hash_bytes(source.as_bytes()),
        &args.input.action,
        &scenario,
        &authority,
    )
    .map_err(super::core_io::operation)?;
    // Reviewing a proposal also checks the exact candidate's local financial effects.
    let execution = mori_core::simulate(&program, &args.input.action, &scenario)
        .map_err(super::core_io::operation)?;
    mori_core::intent::check_effects(
        &authority.signer,
        &authority.limits,
        &execution.effects,
        &execution.post.liabilities,
    )
    .map_err(super::core_io::operation)?;
    if args.output.review {
        println!(
            "Authorize action {} of program {}",
            review_identity(&prepared.intent.action),
            review_identity(&prepared.intent.program_hash)
        );
        println!(
            "Network: {}; anchor: {}; contract: {}",
            review_identity(&authority.network),
            review_identity(&authority.anchor),
            review_identity(&authority.program_address)
        );
        println!(
            "Signer: {}; key: {}",
            review_identity(&authority.signer),
            review_identity(&authority.public_key_hex)
        );
        println!(
            "Predecessor: {}; nonce: {}; validity: {}..{}",
            review_identity(&authority.predecessor),
            authority.nonce.0,
            authority.not_before.0,
            authority.not_after.0
        );
        for limit in &authority.limits {
            println!(
                "Asset {}/{}/{} (scale {}), beneficiary {}: gross <= {}, fee <= {}, net >= {}, closing liability <= {}",
                review_identity(&limit.asset.network),
                review_identity(&limit.asset.issuer),
                review_identity(&limit.asset.reference),
                limit.asset.scale,
                review_identity(&limit.beneficiary),
                limit.gross_cap.0,
                limit.fee_cap.0,
                limit.net_floor.0,
                limit.liability_cap.0
            );
        }
        println!("Exact canonical statement and signing bytes:");
    }
    super::core_io::print(&prepared)?;
    Ok(ExitCode::SUCCESS)
}

/// Signed native preparation: trusted reconstruction of the unsigned payload
/// and its native message. Nothing is signed here.
fn signed(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    let (Some(target), Some(at)) = (&args.target, args.at) else {
        return Err(super::core_io::operation(
            "--profile moriarty-jubjub-owner-intent/1 requires --target and --at together",
        ));
    };
    let source = super::core_io::read_source(&args.input.file)?;
    let scenario = super::core_io::read_json(&args.input.scenario)?;
    let target = super::core_io::signed_target(target)?;
    let authority = super::core_io::authority(&args.authority)?;
    let prepared = mori_backend::signed_relation::prepare_intent(
        &source,
        &args.input.action,
        &scenario,
        &target,
        &authority,
        mori_core::UInt(at),
    )
    .map_err(super::core_io::operation)?;
    if args.output.review {
        let p = &prepared.payload;
        println!(
            "Profile {}: local host reconstruction of the owner intent; not ledger acceptance.",
            review_identity(&p.profile)
        );
        println!(
            "Authorize action {} of program {} (source {})",
            review_identity(&p.action),
            review_identity(&p.program_hash),
            review_identity(&p.source_hash)
        );
        println!(
            "Network: {}; anchor: {}; contract: {}; target: {}",
            review_identity(&p.network),
            review_identity(&p.anchor),
            review_identity(&p.program_address),
            review_identity(&p.target_hash)
        );
        println!(
            "Owner: {}; beneficiary: {}; key: {}",
            review_identity(&p.owner),
            review_identity(&p.beneficiary),
            review_identity(&p.public_key_hex)
        );
        println!(
            "Predecessor: {}; nonce: {}; validity: {}..{} (checked at logical time {at}, which is not signed)",
            review_identity(&p.predecessor),
            p.nonce.0,
            p.not_before.0,
            p.not_after.0
        );
        for limit in &p.limits {
            println!(
                "Asset {}/{}/{} (scale {}), beneficiary {}: gross <= {}, fee <= {}, net >= {}, closing owner liability <= {}",
                review_identity(&limit.asset.network),
                review_identity(&limit.asset.issuer),
                review_identity(&limit.asset.reference),
                limit.asset.scale,
                review_identity(&limit.beneficiary),
                limit.gross_cap.0,
                limit.fee_cap.0,
                limit.net_floor.0,
                limit.liability_cap.0
            );
        }
        println!(
            "Relation context: {}; payload SHA256: {}",
            review_identity(&prepared.context_hash),
            review_identity(&prepared.payload_hash)
        );
        println!(
            "Signed native message (MOI1, FENS domain, 32-byte little-endian): {}",
            review_identity(&prepared.message_hex)
        );
        println!(
            "A native proof is existential over the private signature R/s. The key-to-account association, nonce consumption and predecessor history remain unverified."
        );
        println!("Exact prepared statement:");
    }
    super::core_io::print(&prepared)?;
    Ok(ExitCode::SUCCESS)
}

END BASELINE

BASELINE crates/mori/src/commands/verify_intent.rs
use std::path::PathBuf;
use std::process::ExitCode;

use crate::cli::{ActionInput, SignedProfile, SigningOutputArgs};
use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: ActionInput,

    /// Signature JSON produced by the external signer (the exact signed
    /// wrapper under --profile moriarty-jubjub-owner-intent/1)
    #[arg(long, value_name = "FILE")]
    pub signature: PathBuf,
    /// Expected account authority and domain binding JSON
    #[arg(long)]
    pub authority: PathBuf,
    /// Current logical time used to check the signed validity interval
    #[arg(long)]
    pub at: u128,
    /// Signed native owner-intent profile; requires --target
    #[arg(long)]
    pub profile: Option<SignedProfile>,
    /// Trusted Target JSON with signed_policy (signed profile only)
    #[arg(long, value_name = "FILE")]
    pub target: Option<PathBuf>,

    /// Cap local statement and uncached expression attempts (other costs unmeasured)
    #[arg(long, value_name = "CANONICAL_U128", value_parser = logical_cap, conflicts_with_all = ["profile", "target"])]
    pub logical_cap: Option<mori_core::UInt>,

    #[command(flatten)]
    pub output: SigningOutputArgs,
}

fn logical_cap(value: &str) -> Result<mori_core::UInt, String> {
    serde_json::from_value(serde_json::Value::String(value.into()))
        .map_err(|_| "expected canonical unsigned decimal string in u128 range".into())
}

pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    match (args.profile, &args.target) {
        (Some(_), Some(target)) => return signed(&args, target),
        (Some(_), None) => {
            return Err(super::core_io::operation(
                "--profile moriarty-jubjub-owner-intent/1 requires --target",
            ));
        }
        (None, Some(_)) => {
            return Err(super::core_io::operation(
                "--target belongs to --profile moriarty-jubjub-owner-intent/1; legacy intents never accept a Target",
            ));
        }
        (None, None) => {}
    }
    let (source, program) = super::core_io::program(&args.input.file)?;
    let scenario = super::core_io::read_json(&args.input.scenario)?;
    let authority = super::core_io::read_json(&args.authority)?;
    let signed = super::core_io::read_json(&args.signature)?;
    if let Some(cap) = args.logical_cap {
        return match mori_core::intent::verify_and_simulate_metered(
            &program,
            &mori_core::intent::hash_bytes(source.as_bytes()),
            &args.input.action,
            &scenario,
            &authority,
            &signed,
            mori_core::UInt(args.at),
            cap,
        ) {
            Ok(metered) => {
                if args.output.review {
                    println!(
                        "Signature and local authority constraints verified at logical time {}.",
                        args.at
                    );
                    println!(
                        "Complete local reference effects follow; ledger acceptance requires its own verification."
                    );
                }
                super::core_io::print(&serde_json::json!({
                    "status": "accepted", "signature_valid": true,
                    "qualification": "local-signature-and-reference-execution",
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
    match mori_core::intent::verify_and_simulate(
        &program,
        &mori_core::intent::hash_bytes(source.as_bytes()),
        &args.input.action,
        &scenario,
        &authority,
        &signed,
        mori_core::UInt(args.at),
    ) {
        Ok(execution) => {
            if args.output.review {
                println!(
                    "Signature and local authority constraints verified at logical time {}.",
                    args.at
                );
                println!(
                    "Complete local reference effects follow; ledger acceptance requires its own verification."
                );
            }
            super::core_io::print(
                &serde_json::json!({"status":"accepted", "signature_valid":true, "qualification":"local-signature-and-reference-execution", "execution":execution}),
            )?;
            Ok(ExitCode::SUCCESS)
        }
        Err(error) => {
            super::core_io::print(&serde_json::json!({"status":"rejected", "error":error}))?;
            Ok(ExitCode::FAILURE)
        }
    }
}

/// Strict signed acceptance: trusted reconstruction, exact wrapper equality,
/// host FENS under the reconstructed message and the ordinary compiled
/// relation check. No unsigned or legacy fallback.
fn signed(args: &Args, target: &std::path::Path) -> super::Result<ExitCode> {
    let source = super::core_io::read_source(&args.input.file)?;
    let scenario = super::core_io::read_json(&args.input.scenario)?;
    let target = super::core_io::signed_target(target)?;
    let authority = super::core_io::authority(&args.authority)?;
    let signed = super::core_io::signed_intent(&args.signature)?;
    match mori_backend::signed_relation::accept_intent(
        &source,
        &args.input.action,
        &scenario,
        &target,
        &authority,
        mori_core::UInt(args.at),
        &signed,
    ) {
        Ok(accepted) => {
            if args.output.review {
                println!(
                    "Exact signed payload, strict FENS attachment and compiled relation accepted at logical time {}.",
                    args.at
                );
                println!(
                    "Host reference check only: not a native proof, key-to-account authority or ledger acceptance."
                );
            }
            super::core_io::print(&serde_json::json!({
                "status": "accepted",
                "signature_valid": true,
                "profile": signed.payload.profile,
                "qualification": "local-signed-owner-intent-host-and-relation-check",
                "payload_hash": accepted.payload_hash,
                "context_hash": accepted.context_hash,
                "message_hex": accepted.message_hex,
                "at": accepted.at,
                "execution": accepted.execution,
            }))?;
            Ok(ExitCode::SUCCESS)
        }
        Err(error) => {
            super::core_io::print(&serde_json::json!({"status":"rejected", "error":error}))?;
            Ok(ExitCode::FAILURE)
        }
    }
}

END BASELINE

BASELINE crates/mori/src/commands/inspect.rs
use std::{collections::BTreeSet, path::PathBuf, process::ExitCode};

use mori_backend::{Artifact, CircuitArtifact, PublicBinding};
use mori_core::{Action, BinaryOp, Expr, ExprId, Program, Statement, Type};
use serde_json::{Value, json};

use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,
    /// Compile a public enforcement map against this exact target descriptor
    #[arg(long, value_name = "FILE")]
    pub target: Option<PathBuf>,
}

pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    let source = super::core_io::read_source(&args.file)?;
    if args.target.is_none()
        && mori_core::language(&source).is_ok_and(|language| language == "moriarty/2")
    {
        let staged = match mori_core::stage::load(&source) {
            Ok(staged) => staged,
            Err(error) => {
                super::simulate::print_stage_report(json!({"status":"rejected", "error":error}))?;
                return Ok(ExitCode::FAILURE);
            }
        };
        let program = staged.program();
        super::simulate::print_stage_report(json!({
            "status": "accepted", "language": program.language, "semantics": program.semantics,
            "agreement": program.name, "states": program.states, "assets": program.assets,
            "actions": program.actions, "expressions": program.expressions,
            "stage_plan": staged.plan(), "enforcement_report": null
        }))?;
        return Ok(ExitCode::SUCCESS);
    }
    if mori_core::language(&source).is_ok_and(|language| language == "moriarty/2") {
        // The enforcement map reports the /3 relation. A Core2 --target refuses
        // with Core's typed profile error before reading the target, backend
        // layout or any output; source-only Core2 inspect is handled above.
        let staged = mori_core::stage::load(&source).map_err(super::core_io::operation)?;
        let refusal = match mori_core::stage::require_core1(staged.program()) {
            Err(error) => error,
            Ok(()) => core1_refusal(),
        };
        return Err(super::core_io::operation(refusal));
    }
    let program = mori_core::parse(&source).map_err(super::core_io::operation)?;
    let enforcement = match args.target {
        Some(path) => {
            let target: mori_backend::Target = super::core_io::read_json(&path)?;
            // Use the same whole-program admission and explicit mode as compile.
            // No scenario, key, signature, proof or native execution is consumed.
            let artifact = if target.signed_policy.is_some() {
                mori_backend::compile_signed(&program, &target)
            } else {
                mori_backend::compile(&program, &target)
            }
            .map_err(super::core_io::operation)?;
            enforcement_report(&source, &program, &artifact)?
        }
        None => Value::Null,
    };
    let qualification = if enforcement.is_null() {
        "source-only-unqualified"
    } else {
        "local-compiled-unqualified"
    };
    // Export validates depth/work/byte bounds and escapes public labels before
    // printing anything. A compiler or report-size refusal leaves stdout empty.
    super::core_io::print(&json!({
        "format": "moriarty-enforcement-report/1",
        "qualification": qualification,
        "language": program.language, "semantics": program.semantics,
        "agreement": program.name, "states": program.states,
        "assets": program.assets, "actions": program.actions,
        "expressions": program.expressions,
        "enforcement_report": enforcement,
        "open_premises": [
            "a supplied exact target is required for the compiled enforcement map",
            "ledger authentication and acceptance",
            "native proof soundness and compiler correspondence",
            "account and sponsor authority"
        ]
    }))?;
    Ok(ExitCode::SUCCESS)
}

fn encoded(predicates: &[&str]) -> Value {
    json!({"status": "encoded-not-verified", "predicates": predicates})
}
fn reference(predicates: &[&str]) -> Value {
    json!({"status": "available-not-executed", "predicates": predicates})
}

fn enforcement_report(
    source: &str,
    program: &Program,
    artifact: &Artifact,
) -> super::Result<Value> {
    let actions = program
        .actions
        .iter()
        .zip(&artifact.circuits)
        .map(|(action, circuit)| action_report(program, action, circuit))
        .collect::<super::Result<Vec<_>>>()?;
    Ok(json!({
        "scope": "source-derived-public-transition-relation",
        "identity": {
            "source_hash": mori_core::intent::hash_bytes(source.as_bytes()),
            "program_hash": artifact.program_hash, "semantics": artifact.semantics,
            "artifact_format": artifact.format, "abi": artifact.abi,
            "relation": artifact.relation, "authorization_mode": artifact.authorization_mode,
            "status": "host-computed-identities-not-certificates"
        },
        "target": artifact.target,
        "source_checks": {
            "status": "checked-locally",
            "predicates": [
                "exact language and semantic versions; finite acyclic typed expressions",
                "unique declared names, state initializer types and supported statements",
                "complete distinct asset network/issuer/reference/scale identities; scale 0..38",
                "every asset network equals the supplied target network",
                "exact supported ledger and ZKIR pins; bounded nonempty network and anchor",
                "positive target instruction and artifact limits; actual emitted counts fit",
                "source-wide unique duty identities and closed bounded liability registry"
            ],
            "initial_state": "declaration-only; not an authenticated predecessor or genesis",
            "signed_admission": if artifact.target.signed_policy.is_some() {
                "source-wide owner debits and debtor identity; signed label and asset limits checked"
            } else { "absent-unsigned" }
        },
        "numeric_profile": {
            "UInt": {"min": "0", "max": u128::MAX.to_string(), "representation": "canonical decimal strings; checked unsigned 128-bit integral atomic units"},
            "Bool": "true or false; boolean range constraints",
            "rounding": "exact integral operations only; division and directed rounding unsupported",
            "unit_dimensions": "asset identity and scale retained; generic UInt expressions have no asset dimension checker"
        },
        "resources": artifact.resources,
        "resource_scope": "emitted instruction, ZKIR and artifact-envelope counts; not transaction bytes, measured proving k, DUST or wall-clock replay cost",
        "gates": artifact.gates,
        "actions": actions,
        "unknown": unknown_premises()
    }))
}

fn action_report(
    program: &Program,
    action: &Action,
    circuit: &CircuitArtifact,
) -> super::Result<Value> {
    let fields: Vec<_> = circuit
        .public_inputs
        .iter()
        .enumerate()
        .map(|(position, binding)| public_field(position, binding))
        .collect();
    let authorization_fields: Vec<_> = circuit.authorization_bindings.iter().map(|binding| {
        let rule = authorization_rule(&binding.name);
        json!({
            "position": binding.position, "binding": binding,
            "source": "target-selected signed owner-intent profile",
            "reference": reference(&[rule]), "constraint": encoded(&[rule, "public transcript Impact binding"]),
            "state": {"status": "unauthenticated", "predicate": "supplied authority, time and history values require independent authenticated acceptance"},
            "unknown": ["owner-key-authority", "history-replay-revocation", "clock-and-external-inputs", "host-commitment-reconstruction", "native-proof-soundness", "compiler-correspondence"]
        })
    }).collect();
    let statements = action.statements.iter().enumerate().map(|(step, statement)| {
        let rule = statement_rule(statement)?;
        Ok(json!({"step": step, "source": statement, "reference": reference(&[rule]), "constraint": encoded(&[rule]),
            "state": {"status": "computed-unqualified", "predicate": "ordered local transition on supplied pre-state; no ledger acceptance"},
            "unknown": ["ledger-authentication", "native-proof-soundness", "compiler-correspondence"]}))
    }).collect::<super::Result<Vec<_>>>()?;
    Ok(json!({
        "action": circuit.action, "context_hash": circuit.context_hash,
        "instruction_count": circuit.instruction_count, "replay_steps": circuit.replay_steps,
        "fields": fields, "authorization_fields": authorization_fields,
        "private_input_descriptors": circuit.private_inputs,
        "authorization_constraints": if authorization_fields.is_empty() {
            json!({"status": "absent-unsigned", "predicates": []})
        } else {
            encoded(&[
                "source-fixed context and typed financial values bound into the owner-intent message",
                "nonidentity typed JubJub key and R; canonical response scalar; FENS signature equation",
                "supplied public key coordinates equal the private typed key coordinates",
                "supplied logical time lies in the signed validity interval; time is outside the signed message",
                "per-action complete owner transfer plus fee gross sum <= gross cap; refunds do not reduce gross",
                "per-action owner fees <= fee cap; beneficiary net credit minus owner fees >= net floor",
                "sum of all closing registered owner duties for each asset <= liability cap"
            ])
        },
        "statements": statements,
        "expression_constraints": expression_constraints(program, action)?,
        "effect_frame": circuit.effect_frame, "balance_frame": circuit.balance_frame,
        "liability_frame": circuit.liability_frame,
        "frame_scope": "all declared states, touched balances and the program-wide source duty registry; preservation of other ledger accounts/duties requires the authenticated adapter"
    }))
}

fn public_field(position: usize, binding: &PublicBinding) -> Value {
    let range = match binding.ty {
        Type::UInt => "unsigned 128-bit range",
        Type::Bool => "boolean range",
    };
    let rule = match binding.role.as_str() {
        "context" => "equality to source/target/action context hash limbs computed by the host",
        "input" => "supplied typed action input; no external observation authentication",
        "pre_state" => "supplied typed predecessor state; no equality to the declared initializer",
        "pre_balance" => "supplied opening balance in the touched account/asset frame",
        "pre_liability_presence" => {
            "opening duty presence bit; absent presence forces opening amount zero"
        }
        "pre_liability_amount" => "opening duty amount; absent presence forces opening amount zero",
        "post_state" => "equality to the sequentially computed state or unchanged opening state",
        "post_balance" => "equality to opening balance after ordered checked debits and credits",
        "post_liability_presence" => {
            "equality to computed duty presence; unchanged duties persist; absent amount is zero"
        }
        "post_liability_amount" => {
            "equality to checked duty evolution or unchanged opening amount; absent amount is zero"
        }
        "effect" => "equality to the source statement's computed ordered effect value",
        _ => unreachable!("fresh compiler artifacts use the fixed public ABI roles"),
    };
    let supplied = binding.role.starts_with("pre_") || binding.role == "input";
    let mut unknown = vec!["native-proof-soundness", "compiler-correspondence"];
    if binding.role == "context" {
        unknown.push("host-commitment-reconstruction");
    } else if binding.role == "input" {
        unknown.push("clock-and-external-inputs");
    } else {
        unknown.push("ledger-authentication");
    }
    json!({
        "position": position, "binding": binding,
        "source": "checked declarations, action body and source-fixed public ABI",
        "reference": reference(&[rule]),
        "constraint": encoded(&[range, "public transcript Impact binding", rule]),
        "state": {"status": if supplied { "unauthenticated" } else if binding.role == "context" { "host-reconstructed" } else { "computed-unqualified" },
            "predicate": rule},
        "unknown": unknown
    })
}

fn authorization_rule(name: &str) -> &'static str {
    match name {
        "payload.lo" | "payload.hi" => {
            "128-bit host payload hash limb included in the signed message; SHA computation is outside the circuit"
        }
        "public_key.x" | "public_key.y" => {
            "native field coordinate equality to the typed signature key; owner/key authentication is external"
        }
        "message" => {
            "native field equality to the computed owner-intent message and signature challenge"
        }
        "nonce" => {
            "128-bit nonce bound into the signed message; unique ledger consumption is external"
        }
        "not_before" | "not_after" => {
            "128-bit signed validity bound; interval checked against supplied at"
        }
        "predecessor.lo" | "predecessor.hi" => {
            "128-bit predecessor hash limb bound into the signed message; history freshness is external"
        }
        "at" => {
            "128-bit supplied logical time within signed interval; clock authentication is external"
        }
        _ if name.ends_with(".gross_cap") => {
            "128-bit signed gross cap bounds this action's owner transfers plus fees"
        }
        _ if name.ends_with(".fee_cap") => "128-bit signed fee cap bounds this action's owner fees",
        _ if name.ends_with(".net_floor") => {
            "128-bit signed floor bounds this action's beneficiary net credit minus owner fees"
        }
        _ if name.ends_with(".liability_cap") => {
            "128-bit signed cap bounds all closing registered owner duties for this asset"
        }
        _ => unreachable!("fresh signed compiler artifacts use the fixed authorization ABI"),
    }
}

/// Legacy enforcement-map helpers are `/1`-only. An Await reaching them gets
/// Core's typed `/1` profile refusal, never a panic or an omitted statement.
fn core1_refusal() -> mori_core::Error {
    mori_core::Error {
        code: "mori::core::profile".into(),
        message: "Await requires moriarty-core/2.".into(),
        offset: None,
        step: None,
    }
}

fn statement_rule(statement: &Statement) -> super::Result<&'static str> {
    Ok(match statement {
        Statement::Await { .. } => return Err(super::core_io::operation(core1_refusal())),
        Statement::Let { .. } => {
            "evaluate the expression and bind a typed local name for subsequent statements"
        }
        Statement::Requires { .. } | Statement::Ensures { .. } => {
            "assert the boolean predicate at this ordered statement, after any preceding updates"
        }
        Statement::Next { .. } => {
            "publish the exact typed before/after state values and use the new value in subsequent statements"
        }
        Statement::Emit { .. } => "publish the exact typed computed event value",
        Statement::Transfer { .. } | Statement::Fee { .. } => {
            "checked 128-bit source debit and recipient credit; publish exact amount with source-fixed asset and parties"
        }
        Statement::Liability { .. } => {
            "source-fixed duty identity; checked opening amount plus accrual; closing presence true including present-zero"
        }
        Statement::Discharge { .. } => {
            "require opening duty presence; checked opening amount minus discharge; retain presence even at zero"
        }
    })
}

fn expression_constraints(program: &Program, action: &Action) -> super::Result<Vec<Value>> {
    // Report only reachable expression nodes. Some nodes are shared by a
    // statement's DAG; iterating the sorted set is bounded and deterministic.
    let mut pending: Vec<ExprId> = action
        .statements
        .iter()
        .map(|s| match s {
            Statement::Await { .. } => Err(super::core_io::operation(core1_refusal())),
            Statement::Requires { predicate } | Statement::Ensures { predicate } => Ok(*predicate),
            Statement::Let { value, .. }
            | Statement::Next { value, .. }
            | Statement::Emit { value, .. } => Ok(*value),
            Statement::Transfer { amount, .. }
            | Statement::Fee { amount, .. }
            | Statement::Liability { amount, .. }
            | Statement::Discharge { amount, .. } => Ok(*amount),
        })
        .collect::<super::Result<Vec<_>>>()?;
    let mut reachable = BTreeSet::new();
    while let Some(id) = pending.pop() {
        if !reachable.insert(id) {
            continue;
        }
        match &program.expressions[id as usize] {
            Expr::Binary { lhs, rhs, .. } => pending.extend([*lhs, *rhs]),
            Expr::Not { value } => pending.push(*value),
            _ => {}
        }
    }
    Ok(reachable.into_iter().map(|id| {
        let node = &program.expressions[id as usize];
        let rule = match node {
            Expr::LiabilityPresent { .. } => "read current duty presence under its source-fixed identity",
            Expr::LiabilityAmount { .. } => "read current duty atomic amount under its source-fixed identity",
            Expr::Literal { .. } => "source-fixed typed literal immediate",
            Expr::Name { .. } => "use the typed binding at this ordered statement",
            Expr::Not { .. } => "boolean negation",
            Expr::Binary { op, .. } => match op {
                BinaryOp::Add => "integer addition with unsigned 128-bit result range; overflow refuses",
                BinaryOp::Sub => "integer subtraction with unsigned 128-bit result range; underflow refuses",
                BinaryOp::Mul => "64-bit limb multiplication with zero high product and unsigned 128-bit result range; overflow refuses",
                BinaryOp::Eq | BinaryOp::Ne => "typed equality or inequality with boolean result",
                BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => "unsigned 128-bit comparison with boolean result",
                BinaryOp::And | BinaryOp::Or => "boolean conjunction or disjunction with boolean result",
            }
        };
        json!({"expression": id, "source": node, "reference": reference(&[rule]), "constraint": encoded(&[rule]),
            "unknown": ["native-proof-soundness", "compiler-correspondence"]})
    }).collect())
}

fn unknown_premises() -> Vec<Value> {
    [
        ("ledger-authentication", "opening state/balances/duty presence and full ledger frame, custody, funding and phase correspondence"),
        ("history-replay-revocation", "authenticated predecessor, unique nonce/head consumption, revocation and cumulative authority across calls; no recursive history proof"),
        ("clock-and-external-inputs", "supplied logical time and document/oracle/recipient observations are not authenticated by an input or interval check"),
        ("native-proof-soundness", "no proof generation or verification performed; native constraint soundness requires its own evidence"),
        ("compiler-correspondence", "source/reference/constraint and actual ledger correspondence are not established by compilation, hashes or this map"),
        ("fraud-proof-and-replay-cost", "no deployed fraud proof, proving k, transaction-byte or measured replay-time certificate"),
        ("owner-key-authority", "unsigned mode has no signature relation; signed mode encodes a signature equation but owner/account/key authority remains external"),
        ("sponsor-authority-and-settlement", "sponsor grant authentication, DUST reservation/spend and observed financial settlement are not encoded by these source statements"),
        ("live-deployment-and-finality", "no live network/anchor/root-of-trust compatibility, installed verifier/circuit coverage, submission or finality checked"),
        ("host-commitment-reconstruction", "source, target, asset, payload and predecessor SHA/JSON identities are host reconstructions; their hash computation is not proved in-circuit"),
        ("financial-unit-and-rounding-conformance", "asset dimensions, canonical price orientation and directed rounding beyond this exact integral Core profile remain open"),
        ("private-composition", "public transition fields and private signature descriptors do not establish confidentiality or private recursive handoff"),
    ].into_iter().map(|(id, predicate)| json!({"id": id, "status": "unproven", "predicate": predicate})).collect()
}

END BASELINE

BASELINE crates/mori/src/commands/simulate.rs
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
}

impl Args {
    /// Legacy start inputs. Clap requires both whenever --resume is absent.
    fn legacy(&self) -> super::Result<(&str, &Path)> {
        match (self.action.as_deref(), self.scenario.as_deref()) {
            (Some(action), Some(scenario)) => Ok((action, scenario)),
            _ => Err(super::core_io::operation(
                "--action and --scenario are required without --resume",
            )),
        }
    }
}

fn logical_cap(value: &str) -> Result<mori_core::UInt, String> {
    serde_json::from_value(serde_json::Value::String(value.into()))
        .map_err(|_| "expected canonical unsigned decimal string in u128 range".into())
}

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
            .ok_or_else(|| super::core_io::operation("--trace requires --target"))?;
        let target = super::core_io::read_json(path)?;
        let authorization = args.authorization.load(&target)?;
        let source = super::core_io::read_source(&args.file)?;
        let scenario = super::core_io::read_json(scenario_path)?;
        return match mori_backend::replay::generate(
            &source,
            action,
            &scenario,
            &target,
            authorization.as_input(),
        ) {
            Ok(record) => {
                super::core_io::print(&record)?;
                Ok(ExitCode::SUCCESS)
            }
            Err(error) => {
                super::core_io::print(
                    &serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":error}),
                )?;
                Ok(ExitCode::FAILURE)
            }
        };
    }
    if args.target.is_some() || args.authorization.present() {
        return Err(super::core_io::operation(
            "--target and signed replay arguments require --trace",
        ));
    }
    let source = super::core_io::read_source(&args.file)?;
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

fn print_stage_result(
    result: Result<mori_core::stage::StageOutcome, mori_core::stage::StageRejection>,
) -> super::Result<ExitCode> {
    let exit = if result.is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    };
    super::core_io::print(&mori_core::stage::outcome_report(result))?;
    Ok(exit)
}

/// `--resume`: reads the source, then the continuation, then the event.
/// Bounded-read, strict-decode, load and admission failures are labelled stage
/// rejections; a source the filesystem cannot read keeps the legacy error.
/// Nothing is recorded, so the same file can be resumed again (untracked).
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
fn no_work(error: mori_core::Error) -> mori_core::stage::StageRejection {
    mori_core::stage::StageRejection {
        error,
        attempted: mori_core::UInt(0),
        committed: mori_core::UInt(0),
    }
}

END BASELINE

BASELINE crates/mori/src/commands/call_io.rs
//! Explicit installed-call inputs and create-new publication. No proving
//! provider, parameter fallback or alternate TransitionClaim consumer exists.
use super::{core_io, proof_io};
use crate::{error::CliError, shell::Shell};
use mori_ledger::{
    call_artifacts::{self, ArtifactLeaves},
    profile::{CallRequest, InstalledDeployment, NativeWorkLimits},
};
use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};
#[derive(Debug, clap::Args)]
pub struct Input {
    pub file: PathBuf,
    #[arg(long)]
    pub deployment: PathBuf,
    #[arg(long)]
    pub predecessor: PathBuf,
    #[arg(long)]
    pub parameters: PathBuf,
    #[arg(long)]
    pub request: PathBuf,
    #[arg(long)]
    pub action: String,
    #[arg(long)]
    pub params_dir: PathBuf,
    #[arg(long)]
    pub work_limits: PathBuf,
}
pub struct Inputs {
    pub source: String,
    pub program: mori_core::Program,
    pub deployment: InstalledDeployment,
    pub predecessor: Vec<u8>,
    pub parameters: Vec<u8>,
    pub request: CallRequest,
    pub limits: NativeWorkLimits,
}
pub fn limits(path: &Path) -> super::Result<NativeWorkLimits> {
    let text = text(path, mori_ledger::codec::INPUT_BYTES as u64)?;
    let value = NativeWorkLimits::from_json(&text).map_err(core_io::operation)?;
    if text.len() as u64 > value.max_input_bytes {
        return Err(core_io::operation(
            "work limits input exceeds explicit input bound",
        ));
    }
    Ok(value)
}
pub fn binary(path: &Path, cap: u64) -> super::Result<Vec<u8>> {
    call_artifacts::read_bounded_file(path, cap).map_err(core_io::operation)
}
pub fn text(path: &Path, cap: u64) -> super::Result<String> {
    String::from_utf8(binary(
        path,
        cap.min(mori_ledger::codec::INPUT_BYTES as u64),
    )?)
    .map_err(core_io::operation)
}
pub fn program(path: &Path, cap: u64) -> super::Result<(String, mori_core::Program)> {
    let source = text(path, cap)?;
    let program = mori_core::parse(&source).map_err(core_io::operation)?;
    Ok((source, program))
}
impl Input {
    pub fn read(&self) -> super::Result<Inputs> {
        let limits = limits(&self.work_limits)?;
        let (source, program) = program(&self.file, limits.max_input_bytes)?;
        let deployment =
            InstalledDeployment::from_json(&text(&self.deployment, limits.max_input_bytes)?)
                .map_err(core_io::operation)?;
        let request = CallRequest::from_json(&text(&self.request, limits.max_input_bytes)?)
            .map_err(core_io::operation)?;
        if self.action != request.action {
            return Err(core_io::operation("selected action differs from request"));
        }
        let action = program
            .actions
            .iter()
            .find(|a| a.name == self.action)
            .ok_or_else(|| core_io::operation("unknown source action"))?;
        if request.inputs.len() != action.inputs.len()
            || action
                .inputs
                .iter()
                .any(|i| request.inputs.get(&i.name).is_none_or(|v| v.ty() != i.ty))
        {
            return Err(core_io::operation(
                "request input names or types differ from source",
            ));
        }
        let predecessor = binary(&self.predecessor, limits.max_input_bytes)?;
        let parameters = binary(&self.parameters, limits.max_input_bytes)?;
        Ok(Inputs {
            source,
            program,
            deployment,
            predecessor,
            parameters,
            request,
            limits,
        })
    }
}
pub fn require_new(out: &Path) -> super::Result<()> {
    proof_io::require_new(out)
}
pub fn publish(out: &Path, artifact: &ArtifactLeaves) -> super::Result<()> {
    std::fs::create_dir(out).map_err(|source| CliError::Write {
        path: out.into(),
        source,
    })?;
    publish_leaves(out, artifact)
}
fn publish_leaves(out: &Path, artifact: &ArtifactLeaves) -> super::Result<()> {
    for (name, bytes) in &artifact.leaves {
        proof_io::publish_new(&out.join(name), bytes)?;
    }
    proof_io::publish_new(&out.join(artifact.marker_name), &artifact.marker)
}
pub fn status(shell: &mut Shell, label: &str, action: &str, out: &Path) -> ExitCode {
    shell.status(
        label,
        format!(
            "local unsigned source-derived installed call {} into {}",
            core_io::review_identity(action),
            core_io::review_identity(&out.to_string_lossy())
        ),
    );
    ExitCode::SUCCESS
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn existing_output_directory_is_preserved_without_marker() {
        let out =
            std::env::temp_dir().join(format!("mori-installed-publish-{}", std::process::id()));
        std::fs::create_dir(&out).unwrap();
        std::fs::write(out.join("sentinel"), b"keep").unwrap();
        // Pure publication bytes, no native result/capability construction.
        let artifact = ArtifactLeaves {
            leaves: Default::default(),
            marker_name: "public-keyset.json",
            marker: b"publication-only".to_vec(),
        };
        assert!(publish(&out, &artifact).is_err());
        assert_eq!(std::fs::read(out.join("sentinel")).unwrap(), b"keep");
        assert!(!out.join("public-keyset.json").exists());
        std::fs::remove_dir_all(out).unwrap();
    }
    #[test]
    fn partial_leaf_collision_preserves_files_and_never_writes_own_marker() {
        for (case, leaf) in ["a.bin", "b.bin", "public-keyset.json"]
            .into_iter()
            .enumerate()
        {
            let out = std::env::temp_dir().join(format!(
                "mori-installed-leaf-collision-{}-{case}",
                std::process::id()
            ));
            std::fs::create_dir(&out).unwrap();
            std::fs::write(out.join(leaf), b"keep").unwrap();
            let artifact = ArtifactLeaves {
                leaves: [
                    ("a.bin".into(), b"first".to_vec()),
                    ("b.bin".into(), b"second".to_vec()),
                ]
                .into_iter()
                .collect(),
                marker_name: "public-keyset.json",
                marker: b"publication-only".to_vec(),
            };
            assert!(publish_leaves(&out, &artifact).is_err());
            assert_eq!(std::fs::read(out.join(leaf)).unwrap(), b"keep");
            if leaf != "public-keyset.json" {
                assert!(!out.join("public-keyset.json").exists());
            }
            std::fs::remove_dir_all(out).unwrap();
        }
    }
}

END BASELINE

BASELINE crates/mori/src/commands/signed_call_io.rs
use super::{call_io, core_io};
use mori_ledger::{profile::NativeWorkLimits, signed::*};
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Input {
    pub file: PathBuf,
    #[arg(long)]
    pub signer_policy: PathBuf,
    #[arg(long)]
    pub deployment: PathBuf,
    #[arg(long)]
    pub predecessor: PathBuf,
    #[arg(long)]
    pub parameters: PathBuf,
    #[arg(long)]
    pub request: PathBuf,
    #[arg(long)]
    pub action: String,
    #[arg(long)]
    pub params_dir: PathBuf,
    #[arg(long)]
    pub work_limits: PathBuf,
    #[arg(long)]
    pub derive_supplied_frame: bool,
}
pub struct Inputs {
    pub source: String,
    pub program: mori_core::Program,
    pub policy: FixedSignerPolicy,
    pub deployment: SignedInstalledDeployment,
    pub predecessor: Vec<u8>,
    pub parameters: Vec<u8>,
    pub request: SignedCallRequest,
    pub limits: NativeWorkLimits,
}
impl Input {
    fn validate_frame_mode(&self, version: SignedVersion) -> super::Result<()> {
        if self.derive_supplied_frame && version != SignedVersion::ConsumingV2 {
            return Err(core_io::operation(
                "--derive-supplied-frame requires consuming signed version2",
            ));
        }
        Ok(())
    }
    pub fn read(&self) -> super::Result<Inputs> {
        let limits = call_io::limits(&self.work_limits)?;
        validate_work_limits(&limits).map_err(core_io::ledger_operation)?;
        let (source, program) = call_io::program(&self.file, limits.max_input_bytes)?;
        let policy = FixedSignerPolicy::from_json(&call_io::text(
            &self.signer_policy,
            limits.max_input_bytes.min(SIGNED_JSON_BYTES as u64),
        )?)
        .map_err(core_io::ledger_operation)?;
        let deployment = SignedInstalledDeployment::from_json(&call_io::text(
            &self.deployment,
            limits.max_input_bytes,
        )?)
        .map_err(core_io::ledger_operation)?;
        self.validate_frame_mode(deployment.version().map_err(core_io::ledger_operation)?)?;
        let request = SignedCallRequest::from_json(&call_io::text(
            &self.request,
            limits.max_input_bytes.min(SIGNED_JSON_BYTES as u64),
        )?)
        .map_err(core_io::ledger_operation)?;
        if request.action != self.action || deployment.signer_policy != policy {
            return Err(core_io::operation(
                "independent selected action or signer policy differs",
            ));
        }
        // Original-source whole-policy admission precedes imported key/SRS work.
        SignedStaticBindings::from_source(
            &source,
            &program,
            &deployment.bindings,
            &policy,
            deployment.version().map_err(core_io::ledger_operation)?,
        )
        .map_err(core_io::ledger_operation)?
        .admit_request(&request, &self.action)
        .map_err(core_io::ledger_operation)?;
        let predecessor = call_io::binary(&self.predecessor, limits.max_input_bytes)?;
        let parameters = call_io::binary(&self.parameters, limits.max_input_bytes)?;
        Ok(Inputs {
            source,
            program,
            policy,
            deployment,
            predecessor,
            parameters,
            request,
            limits,
        })
    }
}
pub fn descriptor(
    input: &Input,
    i: &Inputs,
    public: &SignedPublicLocalDeployment,
) -> super::Result<SignedInstalledDeployment> {
    if input.derive_supplied_frame {
        derive_signed_supplied_frame(
            &i.source,
            &i.program,
            &i.deployment,
            &i.policy,
            &i.predecessor,
            &i.parameters,
            public,
            &i.limits,
        )
        .map_err(core_io::ledger_operation)
    } else {
        Ok(i.deployment.clone())
    }
}

pub fn attachment(
    path: &std::path::Path,
    l: &NativeWorkLimits,
) -> super::Result<SignedCallAttachment> {
    SignedCallAttachment::from_json(&call_io::text(
        path,
        l.max_input_bytes.min(SIGNED_JSON_BYTES as u64),
    )?)
    .map_err(core_io::ledger_operation)
}
pub fn status(
    shell: &mut crate::shell::Shell,
    label: &str,
    action: &str,
    out: &std::path::Path,
) -> ExitCode {
    shell.status(label,format!("local signed call with fixed compiled JubJub policy and supplied state (unqualified) {} into {}",core_io::review_identity(action),core_io::review_identity(&out.to_string_lossy())));
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    #[test]
    fn nonce_v2_cli_frame_option_is_explicit_and_v2_only() {
        for command in [
            "prepare-signed-call",
            "prove-signed-call",
            "verify-signed-call",
        ] {
            let mut arguments = vec![
                "mori",
                command,
                "source.mori",
                "--signer-policy",
                "policy.json",
                "--deployment",
                "deployment.json",
                "--predecessor",
                "state.bin",
                "--parameters",
                "ledger-parameters.bin",
                "--request",
                "request.json",
                "--action",
                "first",
                "--params-dir",
                "unused",
                "--work-limits",
                "limits.json",
            ];
            match command {
                "prepare-signed-call" => {
                    arguments.extend(["--public-keyset", "public", "--out", "output"])
                }
                "prove-signed-call" => arguments.extend([
                    "--keyset",
                    "keys",
                    "--signed-attachment",
                    "attachment.json",
                    "--out",
                    "output",
                ]),
                _ => arguments.extend([
                    "--public-keyset",
                    "public",
                    "--proof",
                    "proof",
                    "--signed-attachment",
                    "attachment.json",
                ]),
            }
            for flag in [false, true] {
                let mut args = arguments.clone();
                if flag {
                    args.push("--derive-supplied-frame");
                }
                let cli = crate::cli::Cli::try_parse_from(args).unwrap();
                let input = match cli.command {
                    crate::cli::Command::PrepareSignedCall(a) => a.input,
                    crate::cli::Command::ProveSignedCall(a) => a.input,
                    crate::cli::Command::VerifySignedCall(a) => a.input,
                    _ => panic!(),
                };
                assert_eq!(input.derive_supplied_frame, flag);
                input
                    .validate_frame_mode(SignedVersion::ConsumingV2)
                    .unwrap();
                assert_eq!(
                    input.validate_frame_mode(SignedVersion::LegacyV1).is_err(),
                    flag
                );
            }
        }
    }
}

END BASELINE

BASELINE crates/mori/src/commands/proof_io.rs
use super::core_io;
use crate::{cli::ActionInput, error::CliError};
use std::{
    fs::{File, OpenOptions},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

#[derive(Debug, clap::Args)]
pub struct Input {
    #[command(flatten)]
    pub action: ActionInput,
    /// Exact target profile JSON
    #[arg(long)]
    pub target: PathBuf,
    /// Explicit directory containing checksum-pinned offline SRS files
    #[arg(long)]
    pub params_dir: PathBuf,
    /// Maximum permitted native circuit degree (1 through 19)
    #[arg(long, default_value_t = 9)]
    pub max_k: u8,
    /// Exact signed owner-intent wrapper (required with a signed Target)
    #[arg(long, value_name = "FILE")]
    pub signature: Option<PathBuf>,
    /// Expected owner authority JSON (required with a signed Target)
    #[arg(long, value_name = "FILE")]
    pub authority: Option<PathBuf>,
    /// Logical time checked against the signed interval (required with a signed Target)
    #[arg(long)]
    pub at: Option<u128>,
}
/// The complete signed group accompanying a trusted signed Target.
pub struct Signed {
    pub authority: mori_backend::authorization::ExpectedAuthority,
    pub at: mori_core::UInt,
    pub intent: mori_backend::authorization::SignedIntent,
}
pub type Inputs = (
    String,
    mori_core::Scenario,
    mori_backend::Target,
    Option<Signed>,
);
impl Input {
    /// The trusted Target selects the mode. A signed Target requires the whole
    /// signed group; an unsigned Target refuses any part of it.
    pub fn read(&self) -> super::Result<Inputs> {
        let target: mori_backend::Target = core_io::read_json(&self.target)?;
        let signed = match (
            target.signed_policy.is_some(),
            &self.signature,
            &self.authority,
            self.at,
        ) {
            (true, Some(signature), Some(authority), Some(at)) => Some(Signed {
                authority: core_io::authority(authority)?,
                at: mori_core::UInt(at),
                intent: core_io::signed_intent(signature)?,
            }),
            (true, ..) => {
                return Err(core_io::operation(
                    "a signed Target requires --signature, --authority and --at together; it never falls back to unsigned proofs",
                ));
            }
            (false, None, None, None) => None,
            (false, ..) => {
                return Err(core_io::operation(
                    "--signature, --authority and --at require a signed Target; unsigned proofs never accept signed inputs",
                ));
            }
        };
        Ok((
            core_io::read_source(&self.action.file)?,
            core_io::read_json(&self.action.scenario)?,
            target,
            signed,
        ))
    }
    pub fn options(&self) -> mori_backend::proving::ProofOptions {
        mori_backend::proving::ProofOptions {
            params_dir: self.params_dir.clone(),
            max_k: self.max_k,
        }
    }
}
pub fn require_new(path: &Path) -> super::Result<()> {
    match std::fs::symlink_metadata(path) {
        Ok(_) => Err(CliError::Write {
            path: path.into(),
            source: std::io::Error::new(ErrorKind::AlreadyExists, "destination already exists"),
        }
        .into()),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(()),
        Err(source) => Err(CliError::Write {
            path: path.into(),
            source,
        }
        .into()),
    }
}
struct Staging(PathBuf);
impl Drop for Staging {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
static NEXT: AtomicU64 = AtomicU64::new(0);
/// Publish only complete bytes. A same-directory hard link atomically refuses
/// any destination created since the early check, including dangling symlinks.
pub fn publish_new(path: &Path, bytes: &[u8]) -> super::Result<()> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let stage = parent.join(format!(
        ".mori-proof-{}-{}.tmp",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let mut file: File = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&stage)
        .map_err(|source| CliError::Write {
            path: stage.clone(),
            source,
        })?;
    let staging = Staging(stage);
    file.write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(|source| CliError::Write {
            path: staging.0.clone(),
            source,
        })?;
    drop(file);
    std::fs::hard_link(&staging.0, path).map_err(|source| CliError::Write {
        path: path.into(),
        source,
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn concurrent_publication_keeps_one_complete_winner() {
        let dir = std::env::temp_dir().join(format!(
            "mori-proof-race-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&dir).unwrap();
        let path = dir.join("proof.json");
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
        let handles: Vec<_> = [
            b"complete first proof".as_slice(),
            b"complete second proof".as_slice(),
        ]
        .into_iter()
        .map(|bytes| {
            let path = path.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                (bytes, publish_new(&path, bytes).is_ok())
            })
        })
        .collect();
        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        assert_eq!(results.iter().filter(|(_, ok)| *ok).count(), 1);
        assert_eq!(
            std::fs::read(&path).unwrap(),
            results.iter().find(|(_, ok)| *ok).unwrap().0
        );
        assert_eq!(
            std::fs::read_dir(&dir).unwrap().count(),
            1,
            "no staging file survives normal refusal"
        );
        std::fs::remove_dir_all(dir).unwrap();
    }
}

END BASELINE
