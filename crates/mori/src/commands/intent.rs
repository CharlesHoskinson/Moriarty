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
#[cfg(feature = "native")]
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

#[cfg(not(feature = "native"))]
fn signed(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    let (Some(target), Some(at)) = (&args.target, args.at) else {
        return Err(super::core_io::operation(
            "--profile moriarty-jubjub-owner-intent/1 requires --target and --at together",
        ));
    };
    let _ = (target, at);
    super::native_unavailable()
}
