//! straw-recl9: a recursive ledger-9 proof of a StrawAnchor batch.
//!
//! Structure (arity chosen by measurement, see REPORT.md):
//!   leaf  L_j : verifier-free proof of 4 transactions (wallet L0 rules +
//!               transaction L1 rules + link, composed to one segment);
//!               arch {poseidon, jubjub}; Poseidon transcript.
//!   node  N   : verifies the 4 leaf proofs in circuit (VerifierGadget with
//!               the shift fix), composes their segments, applies the batch
//!               wrapper checks, exposes the 17 statement fields and the
//!               collapsed 12-field KZG accumulator; arch {bls12_381,
//!               poseidon}; Poseidon transcript.
//!   decider D : verifies N in circuit, folds N's carried accumulator,
//!               collapses, subgroup-checks A and B, checks the pairing with
//!               the residue gadget, computes stmt and lays out the 15-slot
//!               acceptValidity template; Blake2b transcript; the ledger
//!               verifies D.
pub use straw_history as i1;

pub mod batch;
pub mod compose;
pub mod decider;
pub mod flat;
pub mod inner;
pub mod leaf;
pub mod node;
pub mod pairing;
pub mod reference;
pub mod segment;
pub mod sherlock_child;
pub mod sherlock_link;
pub mod srs;
pub mod stage;
pub mod template;
pub mod tower;
pub mod transition;
pub mod user;
pub mod util;
