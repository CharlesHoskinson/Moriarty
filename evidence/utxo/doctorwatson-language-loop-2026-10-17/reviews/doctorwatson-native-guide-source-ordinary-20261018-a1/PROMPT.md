Fresh nonauthor ordinary HIGH exact source review. One docs/NATIVE-CORE.md patch only: resolve Rust workspace mori executable explicitly because installed Node beta also exposes mori. All actual guide invocations become cargo run -p mori --locked --, existing commandargs/cwd/feature distinctions preserved; generated starter README still requires projectcwd plus explicit absolute Rustbinary. Check complete patch/after document and real CLI/Cargo/Node consumers; full before/afterfiles available in public packet. Does not claim install/example/proof runtime success or change public permissions, security/native limits, default/Core/beta behavior. Routine docs repair needs no new runtime check or consequential design vote. Approve exact doc after SHA904bddfaa9eabe49f5e935a6e704ca65500a2edb975e17500bd56088b783adce over before14efb29a17b61ad69e24bbb5163a7ab62df464cbfc976a20b24b9ea52ee9f00b if justified. Future integration must verify before/after bytes and all source_basis pins unchanged, diffonlythisfile, finalGitremote/tree guards; no unseen changed candidate bytes approved. No execution authority. No commands/Cargo/CLI/SDK/network/private/config/auth/proverfiles/delegation; only pinned publicpacket files readonly. Packet /home/charl/moriarty-c2-opus55-restored-20261017/native-guide-source-review-packet. Return one whole6fieldJSON with reviewToken doctorwatson-native-guide-source-ordinary-20261018-a1-whole-review.

--- FILE 0001-explicit-rust-cli-in-native-guide.patch ---
--- a/docs/NATIVE-CORE.md
+++ b/docs/NATIVE-CORE.md
@@ -4,6 +4,16 @@
 bodies. It uses the existing lexer, a flat checked expression arena, a
 reference interpreter and a direct ledger-9 ZKIRv3 lowerer. The beta/1
 frontend and TypeScript package retain their existing behavior.
+
+Run this guide's Cargo commands from the Moriarty repository root. The explicit
+`cargo run -p mori --locked --` prefix selects the Rust workspace CLI; the Node
+`@moriarty-lang/beta` package also installs a command named `mori`, but accepts a
+separate beta source profile and supplies no Rust compiler. The Rust package
+uses its default `native` feature here. A `--no-default-features` build supplies
+local Core reference tooling only; Midnight compile/call/proof, targeted
+inspection, trace/replay and JubJub profiles require `native`. This checkout
+workflow does not establish a standalone Rust installation or completed external
+proof/submission qualification.
 
 ```sh
 cargo run -p mori --locked -- check examples/core/language/counter.mori --json
@@ -14,9 +24,11 @@
   --target toolchains/ledger-9.json --out build-batch
 ```
 
-`mori init NEW_DIR` creates a Core transfer starter. `--template repay`
-creates a repayment starter. Run the generated README commands to check,
-simulate and test the complete expected post-state and ordered effects.
+`cargo run -p mori --locked -- init NEW_DIR` creates a Core transfer starter. `--template repay`
+creates a repayment starter. Before running the generated README commands from
+its project directory, replace their `mori` executable with an absolute path to
+the Rust binary built from this checkout. Those commands check, simulate and
+test the complete expected post-state and ordered effects.
 The destination must not exist, and its parent must exist. An I/O failure
 can leave an incomplete project. Choose a new destination before retrying.
 These starters use local example assets and reference execution.
@@ -67,14 +79,14 @@
 
 ## Target-bound public reference replay
 
-`simulate --trace --target` produces a separate `moriarty-reference-replay/1`
+`cargo run -p mori --locked -- simulate --trace --target` produces a separate `moriarty-reference-replay/1`
 sidecar. `replay` independently reconstructs it from separately supplied raw
 source, action, Scenario and trusted Target. For signed Targets, both commands
 require the complete profile, expected-authority, logical-time and actual
 signature group. The record never supplies those trusted inputs.
 
 ```sh
-mori simulate deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.mori \
+cargo run -p mori --locked -- simulate deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.mori \
   --action settle \
   --scenario deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.scenario.json \
   --target deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.target.json \
@@ -82,7 +94,7 @@
   --authority deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.authority.json \
   --at 15 --signature deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.signed.json \
   --trace > payment.replay.json
-mori replay deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.mori \
+cargo run -p mori --locked -- replay deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.mori \
   --action settle \
   --scenario deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.scenario.json \
   --target deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.target.json \
@@ -122,11 +134,11 @@
 ## Native public transition proofs
 
 ```sh
-mori prove examples/core/native-payment/payment.mori --action run \
+cargo run -p mori --locked -- prove examples/core/native-payment/payment.mori --action run \
   --scenario examples/core/native-payment/scenario.json \
   --target toolchains/ledger-9.json --params-dir /path/to/pinned/offline-srs \
   --max-k 10 --out payment-proof.json
-mori verify examples/core/native-payment/payment.mori --action run \
+cargo run -p mori --locked -- verify examples/core/native-payment/payment.mori --action run \
   --scenario examples/core/native-payment/scenario.json \
   --target toolchains/ledger-9.json --params-dir /path/to/pinned/offline-srs \
   --max-k 10 --proof payment-proof.json
@@ -216,7 +228,7 @@
 source, target, artifact, frame and IR digests and raw logs are recorded in
 [the Task 3 report](../.superpowers/sdd/native-liability-task-3-report.md).
 
-`test DIRECTORY` loads `mori-tests.json`, profile `moriarty-tests/1`. Each
+`cargo run -p mori --locked -- test DIRECTORY` loads `mori-tests.json`, profile `moriarty-tests/1`. Each
 case supplies a source, scenario, action and either exact accepted poststate
 and complete effects or an exact rejection code. Empty manifests refuse.
 The JSON report retains every case in manifest order, including its full actual
@@ -233,7 +245,7 @@
 
 ## Owner intent and resource APIs
 
-`mori intent` uses `--action`, `--scenario`, `--authority`, `--scheme`,
+`cargo run -p mori --locked -- intent` uses `--action`, `--scenario`, `--authority`, `--scheme`,
 `--public-key` and `--framing raw`. It produces the inspectable canonical
 `moriarty-owner-intent/2` statement and framed signing bytes. `--review`
 prints domain, key, predecessor, nonce, validity and financial caps before
@@ -241,7 +253,7 @@
 address, target hash, signer, expected key, nonce/predecessor, validity and
 per-asset gross/fee/net/closing-liability constraints.
 
-`mori verify-intent` takes the same source/action/scenario, an expected
+`cargo run -p mori --locked -- verify-intent` takes the same source/action/scenario, an expected
 `--authority`, a `--signature` file containing `{intent, signature_hex}`
 and current logical time `--at`. It verifies the actual signature, compares
 all bindings, executes locally and checks complete effects. BIP340 and
@@ -256,7 +268,7 @@
 Both fresh full-source and actual-result audits approved the selected local
 package, retained in [PR28](https://github.com/CharlesHoskinson/Moriarty/pull/28).
 
-The integrated signed profile uses `mori intent SOURCE --profile
+The integrated signed profile uses `cargo run -p mori --locked -- intent SOURCE --profile
 moriarty-jubjub-owner-intent/1 --action ACTION --scenario SCENARIO --target
 TARGET --authority AUTHORITY --at TIME`. Its `--review` output includes the exact
 payload, message, complete financial projection and authorization projection.
@@ -381,7 +393,7 @@
 
 ## Source-static native call relation (ordinary checks)
 
-`mori compile-call SOURCE --action ACTION --deployment DEPLOYMENT --out DIR`
+`cargo run -p mori --locked -- compile-call SOURCE --action ACTION --deployment DEPLOYMENT --out DIR`
 compiles current typed Core statements to pinned SDK ZKIRv3. The artifact binds
 source/action, typed input communication framing, checked UInt128 arithmetic,
 mutable state prefixes, captured lets, requires/ensures, native duty identities
@@ -389,7 +401,7 @@
 IR and the complete manifest depend on source and static native mappings; they
 can be reused across requests, predecessor states and duty-presence openings.
 
-`mori prepare-call ... --relation DIR` recompiles that trusted source/static
+`cargo run -p mori --locked -- prepare-call ... --relation DIR` recompiles that trusted source/static
 binding and compares both artifact leaves before ordinary SDK checking. It checks
 one activity record per native Op, complete raw consumption and skipped widths,
 then inserts and merges Noops, runs the actual native processed transcript, and
@@ -447,7 +459,7 @@
 
 ## All-action public source-static export and inspection
 
-`mori compile-call FILE --all-actions --static-bindings FILE --authorization
+`cargo run -p mori --locked -- compile-call FILE --all-actions --static-bindings FILE --authorization
 unsigned|signed-v2 --work-limits FILE --out NEW_DIR` compiles every admitted
 source action to tagged native-call ZKIR through the existing native relation
 compiler. `signed-v2` requires `--signer-policy FILE`; unsigned forbids it. There
@@ -516,7 +528,7 @@
 
 ## Metered local owner-intent verification
 
-`mori verify-intent FILE --action NAME --scenario FILE --authority FILE
+`cargo run -p mori --locked -- verify-intent FILE --action NAME --scenario FILE --authority FILE
 --signature FILE --at TIME --logical-cap CANONICAL_U128` applies the existing
 `moriarty-reference-logical-cost/1` meter to the Core
 `moriarty-owner-intent/2` reference consumer. The optional cap is an unsigned,


--- FILE SOURCE-PINS.json ---
{
  "format": "moriarty-native-guide-explicit-rust-cli-proposal/1",
  "requested_and_actual_author": "GPT-6.1 Sol high",
  "before": {
    "path": "docs/NATIVE-CORE.md",
    "bytes": 35785,
    "sha256": "14efb29a17b61ad69e24bbb5163a7ab62df464cbfc976a20b24b9ea52ee9f00b"
  },
  "after": {
    "path": "docs/NATIVE-CORE.md",
    "bytes": 36959,
    "sha256": "904bddfaa9eabe49f5e935a6e704ca65500a2edb975e17500bd56088b783adce"
  },
  "patch": {
    "path": "0001-explicit-rust-cli-in-native-guide.patch",
    "sha256": "93b381975b780cf6f320db27f9cc176e9fca428d7dde4e5444c6a1cc05da923a"
  },
  "source_basis": {
    "docs/NATIVE-CORE.md": "14efb29a17b61ad69e24bbb5163a7ab62df464cbfc976a20b24b9ea52ee9f00b",
    "Cargo.toml": "91e489de70caf1cd51fa77169a269617e402bca14f47f7c5cbd0bfe6e11b7c76",
    "crates/mori/Cargo.toml": "f37c347a8583f86b70d48b2fae7f7de56d2db8f26e5e495e604a3c4e1a68a4c4",
    "crates/mori/src/cli.rs": "018679b5d0a45da43d965c23d2886b890d9154266e149104ece698a9bcb782a7",
    "crates/mori/src/commands/init.rs": "17d8fb1b7ab93a497892605d6d9db6a62a09e120157d39df4d372b5f48775274",
    "packages/moriarty-beta/package.json": "d7b1f8f111fb081f18a80a965019d8a95795fe086df626992e62674fbb234338",
    "docs/MORIARTY-PRODUCT-CONTRACT.md": "d5674c650741103284e0aa8cded4c0e8a0a1be38faa0c65aedb043710caa4396"
  },
  "previous_proposal_patch_sha256": "8900908ee8f62d7ca1208908b38fbc682e7bc0da6015060306a2dd444818108e"
}


--- FILE VERIFICATION-NOTE.md ---
# Explicit Rust CLI selection — verification note

GPT-6.1 Sol high prepared a documentation-only, unapplied patch to
`docs/NATIVE-CORE.md`, using the earlier public-developer-path proposal and
current checkout source. Fresh independent Gemini audit remains with root.

Predicate: every native-guide command invocation selects the same Rust CLI
explicitly while preserving its existing operands, package, cwd and feature
scope. All guide invocations now use `cargo run -p mori --locked --` from the
repository root. Generated starter README commands retain their project cwd;
the guide instructs replacing their executable with the absolute Rust binary
path before following that README. The separately installed Node beta remains
a distinct source-profile consumer. Native feature defaults and the narrower
`--no-default-features` build are explicitly distinguished using current CLI
help source; no feature flag was added to existing example commands.

Source/data checks performed: all shell blocks compare exactly to the original
after normalizing only the executable selector; continuations, arguments,
relative paths, placeholders and redirects are unchanged. No unqualified
`mori ...` invocation remains in the guide. The existing workspace QA command
is unchanged. The patch touches only this document. Current checkout content
matches the before SHA-256 below. SOURCE-PINS.json records full source-basis,
before/after and patch digests; complete before/after copies are provided.

Before: `14efb29a17b61ad69e24bbb5163a7ab62df464cbfc976a20b24b9ea52ee9f00b`.
After: `904bddfaa9eabe49f5e935a6e704ca65500a2edb975e17500bd56088b783adce`.
Patch: `93b381975b780cf6f320db27f9cc176e9fca428d7dde4e5444c6a1cc05da923a`.

Only file inspection, text/hash checks and the required read-only plugin
status were executed. Cargo, tests, SDK, runtime, package, network and private
reads were not performed. No Git or checkout mutation was made. The patch
claims no standalone installation or runtime success. All original release,
U/M/MC/SP/G, financial, composition and UTXO exits and counters remain open or
unchanged according to their existing records. Security/proof/resource judgments
remain owner-unassessed; this patch changes no such policy or requirement.


--- FILE baseline/crates/mori/Cargo.toml ---
[package]
name = "mori"
version = "0.1.0"
description = "Command-line interface for the Moriarty toolchain"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true

[features]
default = ["native"]
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

[dependencies]
anstream.workspace = true
anstyle.workspace = true
clap.workspace = true
ignore.workspace = true
mori-ast.workspace = true
mori-checker.workspace = true
mori-core.workspace = true
mori-backend = { workspace = true, optional = true }
mori-ledger = { workspace = true, optional = true }
mori-diagnostics = { workspace = true, features = ["fancy"] }
mori-fmt.workspace = true
mori-lexer.workspace = true
mori-parser.workspace = true
mori-span.workspace = true
rayon.workspace = true
futures = { workspace = true, optional = true }
rand = { workspace = true, optional = true }
serde.workspace = true
serde_json.workspace = true
sha2.workspace = true
terminal_size.workspace = true
thiserror.workspace = true

midnight-ledger = { workspace = true, optional = true }
midnight-base-crypto = { workspace = true, optional = true }
midnight-coin-structure = { workspace = true, optional = true }
midnight-onchain-state = { workspace = true, optional = true }
midnight-onchain-runtime = { workspace = true, optional = true }
midnight-storage = { workspace = true, optional = true }
midnight-transient-crypto = { workspace = true, optional = true }
midnight-serialize = { workspace = true, optional = true }

[dev-dependencies]
indoc.workspace = true
insta.workspace = true

[[test]]
name = "check_compile_io"
required-features = ["native"]

[[test]]
name = "check_static_compile_io"
required-features = ["native"]

[[test]]
name = "prepare_call"
required-features = ["native"]

[[test]]
name = "installed_call"
required-features = ["native"]

[[test]]
name = "signed_installed_call"
required-features = ["native"]

[[test]]
name = "native_proofs"
required-features = ["native"]

[[test]]
name = "signed_intents"
required-features = ["native"]

[[test]]
name = "replay"
required-features = ["native"]

[[test]]
name = "replay_challenge"
required-features = ["native"]

[[test]]
name = "enforcement_report"
required-features = ["native"]

[[test]]
name = "mapped_role_static_io"
required-features = ["native"]


--- FILE baseline/crates/mori/src/cli.rs ---
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
#[cfg_attr(
    not(feature = "native"),
    command(after_help = "Built without native: local Core reference tooling only. Midnight compile, call, proof, targeted inspect, trace/replay and JubJub profiles require the native feature.")
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
    /// Create a new project from a starter template
    Init(init::Args),
    /// Check syntax, names, types and action support
    Check(check::Args),
    /// Compile a bounded Moriarty program into ledger-9 ZKIRv3 circuits
    #[cfg_attr(
        not(feature = "native"),
        command(after_help = "Unavailable in this build: this command requires the native feature.")
    )]
    Compile(compile::Args),
    /// Compile a source-static ordinary native contract-call relation
    #[cfg_attr(
        not(feature = "native"),
        command(after_help = "Unavailable in this build: this command requires the native feature.")
    )]
    CompileCall(compile_call::Args),
    /// Derive all source action keys and an explicit local installed snapshot
    #[cfg_attr(
        not(feature = "native"),
        command(after_help = "Unavailable in this build: this command requires the native feature.")
    )]
    KeygenCall(keygen_call::Args),
    /// Derive the complete local signed action map under a fixed JubJub policy
    #[cfg_attr(
        not(feature = "native"),
        command(after_help = "Unavailable in this build: this command requires the native feature.")
    )]
    KeygenSignedCall(keygen_signed_call::Args),
    /// Reconstruct a signature-free canonical local call signing package
    #[cfg_attr(
        not(feature = "native"),
        command(after_help = "Unavailable in this build: this command requires the native feature.")
    )]
    PrepareSignedCall(prepare_signed_call::Args),
    /// Prove the actual local typed SDK call with an external FENS attachment
    #[cfg_attr(
        not(feature = "native"),
        command(after_help = "Unavailable in this build: this command requires the native feature.")
    )]
    ProveSignedCall(prove_signed_call::Args),
    /// Cold public-only reconstruction and Real verification of a signed call
    #[cfg_attr(
        not(feature = "native"),
        command(after_help = "Unavailable in this build: this command requires the native feature.")
    )]
    VerifySignedCall(verify_signed_call::Args),
    /// Cold rederive keys and prove an actual local installed SDK call
    #[cfg_attr(
        not(feature = "native"),
        command(after_help = "Unavailable in this build: this command requires the native feature.")
    )]
    ProveCall(prove_call::Args),
    /// Cold rederive public keys and Real-verify an installed call without PK files
    #[cfg_attr(
        not(feature = "native"),
        command(after_help = "Unavailable in this build: this command requires the native feature.")
    )]
    VerifyCall(verify_call::Args),
    /// Prove a public local action using pinned offline parameters
    #[cfg_attr(
        not(feature = "native"),
        command(after_help = "Unavailable in this build: this command requires the native feature.")
    )]
    Prove(prove::Args),
    /// Prepare an unsigned, unproven local native call and public offer
    #[cfg_attr(
        not(feature = "native"),
        command(after_help = "Unavailable in this build: this command requires the native feature.")
    )]
    PrepareCall(prepare_call::Args),
    /// Verify a native proof against the expected source, target and scenario
    #[cfg_attr(
        not(feature = "native"),
        command(after_help = "Unavailable in this build: this command requires the native feature.")
    )]
    Verify(verify::Args),
    /// Format source files in place
    #[command(visible_alias = "fmt")]
    Format(format::Args),
    /// Show identity claims, operative bounds and open premises
    #[cfg_attr(
        not(feature = "native"),
        command(after_help = "Without native, source-only inspection is available; --target requires the native feature.")
    )]
    Inspect(inspect::Args),
    /// Show the native core IR after checking an action against a scenario
    Expand(expand::Args),
    /// Run an action locally against a scenario
    #[cfg_attr(
        not(feature = "native"),
        command(after_help = "Without native, local Core simulation and staged start/resume are available; --trace requires the native feature.")
    )]
    Simulate(simulate::Args),
    /// Independently replay a public record against separately trusted inputs
    #[cfg_attr(
        not(feature = "native"),
        command(after_help = "Unavailable in this build: this command requires the native feature.")
    )]
    Replay(replay::Args),
    /// Run the test cases in a project directory
    Test(test::Args),
    /// Produce the owner signing bytes for an action
    #[cfg_attr(
        not(feature = "native"),
        command(after_help = "Without native, legacy raw Core intents are available; the JubJub --profile requires the native feature.")
    )]
    Intent(intent::Args),
    /// Verify a signature, then prepare the action locally
    #[cfg_attr(
        not(feature = "native"),
        command(after_help = "Without native, legacy Core signature/reference verification is available; the JubJub --profile requires the native feature.")
    )]
    VerifyIntent(verify_intent::Args),
    /// Run the language server over stdio
    Lsp(lsp::Args),
    /// Run the read-only MCP server over stdio
    Mcp(mcp::Args),
}


--- FILE baseline/packages/moriarty-beta/package.json ---
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

