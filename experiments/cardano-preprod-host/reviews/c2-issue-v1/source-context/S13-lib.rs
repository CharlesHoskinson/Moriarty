//! UNADOPTED K2 driver: one callable path from an admitted `moriarty/1` source to a Groth16
//! proof of the actual compiled `SettlementHashV2` relation, and to the public artefacts the
//! Cardano C2/S2 consumers read.
//!
//! Source only: nothing in this crate has been compiled, tested or run.
//!
//! Pipeline, the same for every admitted program:
//!
//! 1. [`input`]: bounded reads and strict JSON (Core's duplicate-key/depth/node limits, then
//!    `deny_unknown_fields` DTOs).
//! 2. [`relation`]: `mori_r1cs::compile_source::<Fr>` with the only options SettlementHashV2
//!    admits and the embedded compiler build. Every identity comes from that run.
//! 3. [`claim`]: a transition is *proposed* (Core reference or explicit records) and the
//!    statement is built by the compiler's own `statement_v2`.
//! 4. [`setup`] / [`prove`] / [`verify`]: separate entry points. Setup and prove take an
//!    explicit RNG; verify takes none and never generates keys. Prove refuses unless the
//!    compiled relation itself is satisfied and its prove-mode matrices equal the probe's.
//! 5. [`export`] and [`encoding`]: canonical public records and strict parsers.
//! 6. [`fixed`] and [`inspect`] (K4H successor): the relation's fixed setup-mode matrices, the
//!    row-by-row decision of any full assignment against them, and the ordinary statement
//!    receipt that exposes decoded statement values. No key, proof or randomness.
//!
//! Trust boundaries (see `DESIGN.md` 3.3): the proof establishes, under setup honesty and
//! Groth16 knowledge soundness, that the statement's constant header and body satisfy this
//! relation. Context bytes are bound but not authenticated. VK provenance is declared by the
//! setup record. Nothing here is a ledger, admission, settlement or Preprod result.
#![forbid(unsafe_code)]

pub mod claim;
pub mod encoding;
pub mod error;
pub mod export;
pub mod fixed;
pub mod hexutil;
pub mod input;
pub mod inspect;
pub mod prove;
pub mod relation;
pub mod setup;
pub mod verify;

pub use claim::{PreparedClaim, StatementView, prepare_claim};
pub use error::DriverError;
pub use export::{ParsedSetup, ProofExport, SetupRecord};
pub use input::{ContextInput, ProveRequest, RelationSelector, TransitionInput};
pub use relation::{RelationDescriptor, SelectedRelation, compile_selected};

/// Declared identity of this driver, written into every setup record. It is a name, not a
/// hash: the root pins these bytes by hash from outside. `+k4h1` marks the K4H successor
/// bytes (fixed-matrix tier, receipts, digest-bound setup); the package version and the lock
/// are unchanged.
pub const DRIVER_ID: &str = "mori-k2-driver/0.1.0-k2s1+k4h1";

/// The only statement adapter this driver drives.
pub const ADAPTER: &str = "SettlementHashV2";
