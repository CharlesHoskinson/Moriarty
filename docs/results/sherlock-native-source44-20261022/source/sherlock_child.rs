//! recl9: the consumer of a Moriarty Sherlock A102 Poseidon child proof.
//!
//! The child is the unchanged Moriarty `LiteralLayout::emit(LocalPublic)` relation
//! proved with `PoseidonState<Fq>`. Its instance is the Moriarty binding followed by the
//! complete positional A102 statement. It crosses into this crate graph only as bytes:
//! processed `MidnightVK` bytes, proof bytes and 32-byte little-endian lane hex. No
//! Moriarty or ledger crate is linked here, so no Rust type crosses the midnight-proofs
//! 0.8.3 / 0.8.2 boundary; the key digest is pinned from Moriarty's independent rebuild.
//!
//! `verify_sherlock_child_offcircuit` is full off-circuit verification: the stdlib
//! verifier (public-input count, transcript end) and the existing accumulator route
//! (prepare, resolve, collapse, one pairing). `SherlockChildRelation` verifies the child
//! in circuit with `inner::verify_child_incircuit` and exposes the whole child instance,
//! then the collapsed 12-field accumulator. It defers the pairing: its proof is not
//! validity of the child until a reviewed terminal fold and pairing check. It is not the
//! StrawAnchor node, its statement is not a rollup statement, and no decider consumes it.

use std::{collections::BTreeMap, io};

use midnight_circuits::{
    hash::poseidon::PoseidonState, instructions::*, types::AssignedNative, verifier::{fixed_bases, Accumulator},
};
use midnight_curves::Bls12;
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
    poly::kzg::params::ParamsVerifierKZG,
};
use midnight_zk_stdlib::{verify, MidnightVK, Relation, ZkStdLib, ZkStdLibArch};

use crate::{
    decider::bls_arch,
    inner::{acc_pi_len, acc_pis, prepare_offcircuit, prepare_own_offcircuit, verify_child_incircuit, verify_offcircuit, vk_name, S},
    util::{read_bytes, read_u32, sha256_hex, vk_bytes, vk_from_bytes, write_bytes, F},
};

const MAGIC: &[u8; 8] = b"RECL9S01";

type A = AssignedNative<F>;
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
            return Err(io::Error::other("child key does not round-trip in this graph"));
        }
        if !(2..=MAX_CHILD_INSTANCE).contains(&n_instance) || instance.len() != n_instance {
            return Err(io::Error::other("child instance length differs from the pinned count"));
        }
        if proof.is_empty() || proof.len() > MAX_CHILD_PROOF_BYTES {
            return Err(io::Error::other("child proof is empty or oversized"));
        }
        Ok(Self { name: vk_name("sherlock", &vk), vk, instance, proof })
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

    fn circuit(&self, _: &ZkStdLib, _: &mut impl Layouter<F>, _: Value<Vec<F>>, _: Value<()>) -> Result<(), Error> {
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
    verify_offcircuit(params, &child.vk, &child.name, &child.instance, &child.proof, false)
}

/// Full verification for the parent fold, retaining the ORIGINAL own accumulator.
/// The stdlib route remains independent. Only a clone is resolved/collapsed for the
/// second route's pairing; returning that clone would change the later fold transcript.
/// `child.name` must be the same namespace used by the parent's in-circuit preparation.
/// No result is cached across calls or accepted from the caller.
pub(crate) fn verify_sherlock_child_own_offcircuit(
    params: &ParamsVerifierKZG<Bls12>,
    child: &SherlockChild,
) -> Result<Accumulator<S>, String> {
    verify::<RawInstance, Pos>(params, &child.vk, &child.instance, None, &child.proof)
        .map_err(|e| format!("stdlib verify: {e:?}"))?;
    let own = prepare_own_offcircuit(&child.vk, &child.name, &child.instance, &child.proof)?;
    let mut checked = own.clone();
    checked.resolve_fixed_bases(&fixed_bases::<S>(&child.name, child.vk.vk()));
    checked.collapse();
    if !checked.check(params, &BTreeMap::new()) {
        return Err("pairing check failed".into());
    }
    Ok(own)
}

/// Single-child wrapper: the child key fixed as constants, the full child instance and
/// the collapsed accumulator public. The accumulator pairing is deferred.
#[derive(Clone, Debug)]
pub struct SherlockChildRelation {
    pub child_vk: MidnightVK,
    pub n_child_instance: u32,
}

#[derive(Clone, Debug)]
pub struct SherlockChildWitness {
    pub child_instance: Vec<F>,
    pub child_proof: Vec<u8>,
}

impl SherlockChildRelation {
    pub fn name(&self) -> String {
        vk_name("sherlock", &self.child_vk)
    }

    pub fn validate(&self) -> io::Result<()> {
        if !(2..=MAX_CHILD_INSTANCE).contains(&(self.n_child_instance as usize)) {
            return Err(io::Error::other("child instance length outside 2..=16383"));
        }
        Ok(())
    }

    pub fn public_len(&self) -> usize {
        self.n_child_instance as usize + acc_pi_len()
    }

    /// Off-circuit public instance in the in-circuit order: the child instance, then
    /// its collapsed accumulator. No pairing is checked here.
    pub fn instance(&self, w: &SherlockChildWitness) -> Result<Vec<F>, String> {
        if w.child_instance.len() != self.n_child_instance as usize {
            return Err("child instance length".into());
        }
        let acc = prepare_offcircuit(&self.child_vk, &self.name(), &w.child_instance, &w.child_proof, false)?;
        Ok([w.child_instance.clone(), acc_pis(&acc)].concat())
    }
}

impl Relation for SherlockChildRelation {
    type Instance = Vec<F>;
    type Witness = SherlockChildWitness;
    type Error = Error;

    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }

    fn used_chips(&self) -> ZkStdLibArch {
        bls_arch(4)
    }

    fn circuit(
        &self,
        s: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _instance: Value<Vec<F>>,
        w: Value<SherlockChildWitness>,
    ) -> Result<(), Error> {
        let n = self.n_child_instance as usize;
        w.as_ref().map(|w| w.child_instance.len()).error_if_known_and(|len| *len != n)?;
        let vals = w.as_ref().map(|w| w.child_instance.clone());
        let inst: Vec<A> = s.assign_many(l, &vals.transpose_vec(n))?;
        let proof = w.as_ref().map(|w| w.child_proof.clone());
        let acc = verify_child_incircuit(s, l, &self.child_vk, &self.name(), &inst, proof, false)?;
        for x in &inst {
            s.constrain_as_public_input(l, x)?;
        }
        s.verifier().constrain_as_public_input(l, &acc)
    }

    fn write_relation<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(MAGIC)?;
        w.write_all(&self.n_child_instance.to_le_bytes())?;
        write_bytes(w, &vk_bytes(&self.child_vk))
    }

    fn read_relation<R: io::Read>(r: &mut R) -> io::Result<Self> {
        let mut magic = [0u8; 8];
        r.read_exact(&mut magic)?;
        if &magic != MAGIC {
            return Err(io::Error::other("not a recl9 sherlock child relation"));
        }
        let n_child_instance = read_u32(r)?;
        let child_vk = vk_from_bytes(&read_bytes(r, 1 << 20)?)?;
        let rel = SherlockChildRelation { child_vk, n_child_instance };
        rel.validate()?;
        let mut rest = Vec::new();
        r.read_to_end(&mut rest)?;
        if !rest.is_empty() {
            return Err(io::Error::other("trailing bytes after sherlock child relation"));
        }
        Ok(rel)
    }
}

#[cfg(test)]
mod tests {
    //! UNRUN, ignored: they need a Moriarty child artifact, its independently pinned key
    //! digest and lane count, and the pinned 2^14 SRS. Resource admission is separate.
    use super::*;
    use crate::{inner::reconstruct_acc, util::f_from_hex};
    use ff::Field;
    use midnight_proofs::{dev::MockProver, poly::kzg::params::ParamsKZG, utils::SerdeFormat};
    use midnight_zk_stdlib::{cost_model, MidnightCircuit};
    use std::{collections::BTreeMap, fs, io::BufReader, path::PathBuf};

    /// Ledger-9 base-crypto pin of bls_midnight_2p14 (Moriarty parameters.rs index 14).
    const SRS_2P14_SHA256: &str = "fc253016885ec830e97808c9ec920bb5cab5c21af590380a6cb5eb0538e2b244";

    fn env(name: &str) -> String {
        std::env::var(name).unwrap_or_else(|_| panic!("{name} must be set explicitly"))
    }

    fn verifier_params() -> ParamsVerifierKZG<Bls12> {
        let path = PathBuf::from(env("MIDNIGHT_SRS_DIR")).join("bls_midnight_2p14");
        let bytes = fs::read(&path).unwrap();
        assert_eq!(sha256_hex(&bytes), SRS_2P14_SHA256, "pinned ledger verifier SRS");
        ParamsKZG::<Bls12>::read_custom(&mut BufReader::new(&bytes[..]), SerdeFormat::RawBytesUnchecked)
            .unwrap()
            .verifier_params()
    }

    fn child() -> SherlockChild {
        let dir = PathBuf::from(env("SHERLOCK_CHILD_DIR"));
        let n: usize = env("SHERLOCK_CHILD_INSTANCE_LANES").parse().unwrap();
        let hexes: Vec<String> = serde_json::from_slice(&fs::read(dir.join("child.instance.json")).unwrap()).unwrap();
        let instance = hexes.iter().map(|h| f_from_hex(h).unwrap()).collect();
        SherlockChild::from_bytes(
            &fs::read(dir.join("child.vk")).unwrap(),
            &env("SHERLOCK_CHILD_VK_SHA256"),
            n,
            instance,
            fs::read(dir.join("child.proof")).unwrap(),
        )
        .unwrap()
    }

    fn mock(rel: &SherlockChildRelation, pis: Vec<F>, w: SherlockChildWitness, k: u32) -> bool {
        let circuit = MidnightCircuit::new(rel, Value::known(pis.clone()), Value::known(w), Some(k));
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            MockProver::run(&circuit, vec![vec![], pis.clone()]).map(|p| p.verify())
        }));
        matches!(res, Ok(Ok(Ok(()))))
    }

    #[test]
    #[ignore = "requires a Moriarty child artifact, its independent key digest and the pinned 2^14 SRS"]
    fn sherlock_child_verifies_offcircuit_and_rejects_tampering() {
        let vp = verifier_params();
        let c = child();
        let acc = verify_sherlock_child_offcircuit(&vp, &c).expect("honest child");
        assert_eq!(acc_pis(&acc).len(), acc_pi_len());
        let n = c.instance.len();
        for lane in [0, 1, n - 1] {
            let mut bad = c.clone();
            bad.instance[lane] += F::ONE;
            assert!(verify_sherlock_child_offcircuit(&vp, &bad).is_err(), "lane {lane}");
        }
        let mut short = c.clone();
        short.instance.pop();
        assert!(verify_sherlock_child_offcircuit(&vp, &short).is_err());
        let mut flipped = c.clone();
        flipped.proof[c.proof.len() / 2] ^= 1;
        assert!(verify_sherlock_child_offcircuit(&vp, &flipped).is_err());
        let mut trailing = c.clone();
        trailing.proof.push(0);
        assert!(verify_sherlock_child_offcircuit(&vp, &trailing).is_err());
        assert!(SherlockChild::from_bytes(&vk_bytes(&c.vk), &"0".repeat(64), n, c.instance.clone(), c.proof.clone()).is_err());
        assert!(SherlockChild::from_bytes(&vk_bytes(&c.vk), &sha256_hex(&vk_bytes(&c.vk)), n + 1, c.instance.clone(), c.proof.clone()).is_err());
    }

    fn assert_same_own(a: &Accumulator<S>, b: &Accumulator<S>) {
        for (lhs, rhs) in [(a.lhs(), b.lhs()), (a.rhs(), b.rhs())] {
            assert_eq!(lhs.bases(), rhs.bases(), "ordered original bases");
            assert_eq!(lhs.scalars(), rhs.scalars(), "ordered original scalars");
            assert_eq!(lhs.fixed_base_scalars(), rhs.fixed_base_scalars(), "complete named fixed-base map");
        }
    }

    #[test]
    #[ignore = "requires actual pinned child and 2^14 SRS; no proof production; separate resource admission"]
    fn verified_own_reuse_preserves_raw_encoding_and_both_refusal_routes() {
        let vp = verifier_params();
        let mut c = child();
        // Exercise the namespace used by the actual parent, not the legacy wrapper's label.
        c.name = vk_name("sherlock_a102", &c.vk);
        let original = prepare_own_offcircuit(&c.vk, &c.name, &c.instance, &c.proof).unwrap();
        let own = verify_sherlock_child_own_offcircuit(&vp, &c).unwrap();
        assert_same_own(&own, &original);
        assert_eq!(acc_pis(&own), acc_pis(&original), "raw transcript encoding retained");
        let before = acc_pis(&own);
        let mut checked = own.clone();
        checked.resolve_fixed_bases(&fixed_bases::<S>(&c.name, c.vk.vk()));
        checked.collapse();
        assert!(checked.check(&vp, &BTreeMap::new()));
        assert_eq!(acc_pis(&checked), acc_pis(&verify_sherlock_child_offcircuit(&vp, &c).unwrap()));
        assert_same_own(&own, &original);
        assert_eq!(acc_pis(&own), before, "pairing clone must not mutate original fold input");
        assert_ne!(acc_pis(&own), acc_pis(&checked), "collapsed encoding is not the fold input");
        let mut bad_instance = c.clone();
        bad_instance.instance[1] += F::ONE;
        let mut bad_proof = c.clone();
        bad_proof.proof[c.proof.len() / 2] ^= 1;
        let mut trailing = c.clone();
        trailing.proof.push(0);
        for bad in [bad_instance, bad_proof, trailing] {
            assert!(verify_sherlock_child_offcircuit(&vp, &bad).is_err());
            assert!(verify_sherlock_child_own_offcircuit(&vp, &bad).is_err());
        }
    }

    #[test]
    #[ignore = "MockProver of the in-circuit BLS verifier over the full child instance; resource admission separate"]
    fn sherlock_child_wrapper_binds_the_child_statement_and_defers_the_pairing() {
        let vp = verifier_params();
        let c = child();
        let rel = SherlockChildRelation { child_vk: c.vk.clone(), n_child_instance: c.instance.len() as u32 };
        rel.validate().unwrap();
        let mut relb = Vec::new();
        rel.write_relation(&mut relb).unwrap();
        let back = SherlockChildRelation::read_relation(&mut &relb[..]).unwrap();
        assert_eq!(vk_bytes(&back.child_vk), vk_bytes(&rel.child_vk));
        assert_eq!(back.n_child_instance, rel.n_child_instance);
        let k = cost_model(&rel, None).k;
        let honest = SherlockChildWitness { child_instance: c.instance.clone(), child_proof: c.proof.clone() };
        let pis = rel.instance(&honest).unwrap();
        assert_eq!(pis.len(), rel.public_len());
        assert!(mock(&rel, pis.clone(), honest.clone(), k), "honest wrapper");
        let tail = &pis[pis.len() - acc_pi_len()..];
        assert!(reconstruct_acc(tail).unwrap().check(&vp, &BTreeMap::new()), "honest deferred pairing");
        // A public child lane differing from the verified witness lane.
        let mut bad = pis.clone();
        bad[1] += F::ONE;
        assert!(!mock(&rel, bad, honest.clone(), k));
        // A public accumulator field differing from the in-circuit accumulator.
        let mut bad = pis.clone();
        let m = bad.len();
        bad[m - 2] += F::ONE;
        assert!(!mock(&rel, bad, honest.clone(), k));
        // A false child statement with a self-consistent claimed accumulator. The
        // wrapper accumulates; only the deferred pairing must refuse it.
        let mut forged = honest.clone();
        forged.child_instance[1] += F::ONE;
        match rel.instance(&forged) {
            Ok(fpis) => {
                let verdict = mock(&rel, fpis.clone(), forged, k);
                eprintln!("forged child statement: wrapper MockProver accepts = {verdict}");
                let acc = reconstruct_acc(&fpis[fpis.len() - acc_pi_len()..]);
                assert!(
                    acc.is_none_or(|a| !a.check(&vp, &BTreeMap::new())),
                    "the deferred pairing must refuse a false child statement"
                );
            }
            Err(e) => eprintln!("forged child statement refused before accumulation: {e}"),
        }
    }
}
