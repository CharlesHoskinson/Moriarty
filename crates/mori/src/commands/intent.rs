use std::process::ExitCode;

use clap::ValueEnum;

use crate::cli::{ActionInput, SigningOutputArgs};
use crate::shell::Shell;

/// Quote display identities without giving source text terminal control.
fn review_identity(value: &str) -> String {
    format!(
        "\"{}\"",
        value
            .chars()
            .flat_map(char::escape_default)
            .collect::<String>()
    )
}

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: ActionInput,

    /// Signature scheme of the owner key
    #[arg(long)]
    pub scheme: Scheme,

    /// Owner public key, hex encoded
    #[arg(long, value_name = "HEX")]
    pub public_key: String,

    /// How the signing bytes are framed
    #[arg(long)]
    pub framing: Framing,
    /// Domain, predecessor, nonce, validity and per-asset authority JSON
    #[arg(long)]
    pub authority: std::path::PathBuf,

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

pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    if !matches!(args.framing, Framing::Raw) {
        return Err(super::core_io::operation(
            "core/1 intents currently require raw canonical framing",
        ));
    }
    let (source, program) = super::core_io::program(&args.input.file)?;
    let scenario = super::core_io::read_json(&args.input.scenario)?;
    let authority: mori_core::intent::Authorization = super::core_io::read_json(&args.authority)?;
    let scheme = match args.scheme {
        Scheme::SchnorrBip340 => mori_core::intent::Scheme::SchnorrBip340,
        Scheme::EcdsaSecp256k1Sha256 => mori_core::intent::Scheme::EcdsaSecp256k1Sha256,
    };
    if authority.scheme != scheme || authority.public_key_hex != args.public_key {
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
