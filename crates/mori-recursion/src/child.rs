// Adapted from projectfenrir recursion/stagenet-strawman/recl9.
// Uses the pinned stock 0.8.3 graph; terminal validity and gadget soundness remain separate.
//! Byte-oriented child import and full off-circuit verification on the pinned stock graph.
//!
//! Processed key bytes must match an independently reconstructed digest. Both the stdlib
//! verifier (complete public inputs and transcript end) and the accumulator pairing run.

use crate::{
    inner::{S, verify_offcircuit, vk_name},
    util::{F, sha256_hex, vk_bytes, vk_from_bytes},
};
use midnight_circuits::{hash::poseidon::PoseidonState, verifier::Accumulator};
use midnight_curves::Bls12;
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
    poly::kzg::params::ParamsVerifierKZG,
};
use midnight_zk_stdlib::{MidnightVK, Relation, ZkStdLib, verify};
use std::io;

type Pos = PoseidonState<F>;

/// The producer's binding plus statement capacity (2^14 verifier).
pub const MAX_CHILD_INSTANCE: usize = (1 << 14) - 1;
/// The producer's proof byte cap.
pub const MAX_CHILD_PROOF_BYTES: usize = 262_144;

/// A child decoded from bytes against an independently pinned key digest and lane count.
#[derive(Clone, Debug)]
pub struct SherlockChild {
    pub vk: MidnightVK,
    pub name: String,
    pub instance: Vec<F>,
    pub proof: Vec<u8>,
}

impl SherlockChild {
    pub fn from_bytes(
        vk_raw: &[u8],
        vk_sha256: &str,
        n_instance: usize,
        instance: Vec<F>,
        proof: Vec<u8>,
    ) -> io::Result<Self> {
        if sha256_hex(vk_raw) != vk_sha256 {
            return Err(io::Error::other("child key differs from the pinned digest"));
        }
        let vk = vk_from_bytes(vk_raw)?;
        if vk_bytes(&vk) != vk_raw {
            return Err(io::Error::other(
                "child key does not round-trip in this graph",
            ));
        }
        if !(2..=MAX_CHILD_INSTANCE).contains(&n_instance) || instance.len() != n_instance {
            return Err(io::Error::other(
                "child instance length differs from the pinned count",
            ));
        }
        if proof.is_empty() || proof.len() > MAX_CHILD_PROOF_BYTES {
            return Err(io::Error::other("child proof is empty or oversized"));
        }
        Ok(Self {
            name: vk_name("sherlock", &vk),
            vk,
            instance,
            proof,
        })
    }
}

/// Format-only relation for the stdlib verifier, as the ledger's `DummyRelation`: the
/// instance is the raw public-input vector. It is never synthesized or serialized.
#[derive(Clone, Debug)]
struct RawInstance;

impl Relation for RawInstance {
    type Instance = Vec<F>;
    type Witness = ();
    type Error = Error;

    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }

    fn circuit(
        &self,
        _: &ZkStdLib,
        _: &mut impl Layouter<F>,
        _: Value<Vec<F>>,
        _: Value<()>,
    ) -> Result<(), Error> {
        Err(Error::Synthesis("format-only relation".into()))
    }

    fn write_relation<W: io::Write>(&self, _: &mut W) -> io::Result<()> {
        Err(io::Error::other("format-only relation"))
    }

    fn read_relation<R: io::Read>(_: &mut R) -> io::Result<Self> {
        Err(io::Error::other("format-only relation"))
    }
}

/// Off-circuit full verification: stdlib verify, then the existing accumulator route
/// with its pairing. Returns the checked collapsed accumulator.
pub fn verify_sherlock_child_offcircuit(
    params: &ParamsVerifierKZG<Bls12>,
    child: &SherlockChild,
) -> Result<Accumulator<S>, String> {
    verify::<RawInstance, Pos>(params, &child.vk, &child.instance, None, &child.proof)
        .map_err(|e| format!("stdlib verify: {e:?}"))?;
    verify_offcircuit(
        params,
        &child.vk,
        &child.name,
        &child.instance,
        &child.proof,
        false,
    )
}
