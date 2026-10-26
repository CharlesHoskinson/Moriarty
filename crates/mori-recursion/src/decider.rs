// Adapted from projectfenrir recursion/stagenet-strawman/recl9.
// Compiled on the pinned stock graph; installed-key equality is checked separately.
//! The ledger-9 decider relation D (residue pairing route, subgroup checks
//! always on, no switch). Copied from decider/src/decider.rs; recl9 builds it
//! on the pinned stock Midnight graph; its W is the Sherlock linked parent.

use std::io;

use group::{GroupEncoding, prime::PrimeCurveAffine};
use midnight_circuits::{
    instructions::{AssertionInstructions, AssignmentInstructions, PublicInputInstructions},
    types::{AssignedNative, InnerValue},
};
use midnight_curves::{G1Projective, G2Affine};
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};
use midnight_zk_stdlib::{MidnightVK, Relation, ZkStdLib, ZkStdLibArch};

use crate::{
    inner::{acc_pi_len, point_pi_len, verify_child_incircuit, vk_name},
    pairing::{AG1, PairingGadget, kzg_decider_lines},
    template::{Slot, Template},
    tower::{Tower, TowerOpts},
    util::{F, read_bytes, read_f, read_u32, vk_bytes, vk_from_bytes, write_bytes, write_f},
};

const MAGIC: &[u8; 8] = b"RECL9D02";

/// Whether slot 1 (communication commitment) is free or bound in circuit to
/// Poseidon(comm_rand, stmt) for the compiled acceptValidity shape
/// (one Native input, no outputs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommBinding {
    Free = 0,
    BoundPoseidonStmtV1 = 1,
}

fn comm_binding_tag(binding: CommBinding) -> u8 {
    binding as u8
}

fn comm_binding_from_tag(tag: u8) -> io::Result<CommBinding> {
    match tag {
        0 => Ok(CommBinding::Free),
        1 => Ok(CommBinding::BoundPoseidonStmtV1),
        other => Err(io::Error::other(format!(
            "unknown decider comm_binding tag {other}"
        ))),
    }
}

/// The decider and node architecture: the stock ledger-9 one.
pub fn bls_arch(nr_cols: u8) -> ZkStdLibArch {
    ZkStdLibArch {
        bls12_381: true,
        poseidon: true,
        nr_pow2range_cols: nr_cols,
        ..ZkStdLibArch::default()
    }
}

#[derive(Clone, Debug)]
pub struct DeciderRelation {
    /// Range-check columns, 1..=4 (the ledger rebuilds the CS from the arch).
    pub nr_cols: u8,
    /// The wrapper's verifying key, pinned as circuit constants.
    pub w_vk: MidnightVK,
    /// Total length of W's instance (statement fields, then 12 accumulator fields).
    pub n_w_instance: u32,
    /// [tau]_2 of the Midnight SRS (checked on the host at keygen).
    pub s_g2: G2Affine,
    /// DOM_STMT field constant.
    pub dom_stmt: F,
    /// Which W instance fields enter stmt, in order, after DOM_STMT,
    /// instance_id and batch_no.
    pub stmt_fields: Vec<u32>,
    /// Public-input layout of the `acceptValidity` call.
    pub template: Template,
    pub comm_binding: CommBinding,
}

#[derive(Clone, Debug)]
pub struct DeciderWitness {
    pub w_instance: Vec<F>,
    pub w_proof: Vec<u8>,
    pub instance_id: F,
    pub batch_no: F,
    pub comm_rand: F,
}

fn validate_known_shapes(
    n_w_instance: usize,
    n_public: usize,
    instance: &Value<Vec<F>>,
    witness: &Value<DeciderWitness>,
) -> Result<(), Error> {
    instance.error_if_known_and(|fields| fields.len() != n_public)?;
    witness.error_if_known_and(|w| w.w_instance.len() != n_w_instance)
}

impl DeciderRelation {
    pub fn w_name(&self) -> String {
        vk_name("w", &self.w_vk)
    }

    pub fn validate(&self) -> io::Result<()> {
        if !(1..=4).contains(&self.nr_cols) {
            return Err(io::Error::other("nr_cols must be in 1..=4"));
        }
        let n_state = (self.n_w_instance as usize)
            .checked_sub(acc_pi_len())
            .ok_or_else(|| io::Error::other("W instance shorter than an accumulator"))?;
        if self.stmt_fields.iter().any(|&i| i as usize >= n_state) {
            return Err(io::Error::other(
                "stmt field index outside W's statement fields",
            ));
        }
        if bool::from(self.s_g2.is_identity()) {
            return Err(io::Error::other("s_g2 is the identity"));
        }
        if self.template.has_values() {
            return Err(io::Error::other("value slots belong to the run decider"));
        }
        if self.comm_binding == CommBinding::BoundPoseidonStmtV1
            && self.template.slots.get(1) != Some(&Slot::CommunicationCommitment)
        {
            return Err(io::Error::other(
                "bound relation needs communication_commitment at slot 1",
            ));
        }
        self.template.validate()
    }

    /// Off-circuit statement digest.
    pub fn stmt(&self, w_instance: &[F], instance_id: F, batch_no: F) -> F {
        use midnight_circuits::{hash::poseidon::PoseidonChip, instructions::hash::HashCPU};
        let mut pre = vec![self.dom_stmt, instance_id, batch_no];
        pre.extend(self.stmt_fields.iter().map(|&i| w_instance[i as usize]));
        <PoseidonChip<F> as HashCPU<F, F>>::hash(&pre)
    }

    /// Off-circuit Poseidon(comm_rand, stmt), exactly the in-circuit bound
    /// opening for BoundPoseidonStmtV1.
    pub fn comm_commitment_for(&self, stmt: F, comm_rand: F) -> F {
        use midnight_circuits::{hash::poseidon::PoseidonChip, instructions::hash::HashCPU};
        <PoseidonChip<F> as HashCPU<F, F>>::hash(&[comm_rand, stmt])
    }
}

impl Relation for DeciderRelation {
    /// The ledger contract-call public-input vector.
    type Instance = Vec<F>;
    type Witness = DeciderWitness;
    type Error = Error;

    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }

    fn circuit(
        &self,
        std: &ZkStdLib,
        l: &mut impl Layouter<F>,
        instance: Value<Vec<F>>,
        witness: Value<DeciderWitness>,
    ) -> Result<(), Error> {
        // Host shape checks assign no cells, so keygen with unknown values has
        // the same layout. Reject malformed known vectors before indexing them.
        validate_known_shapes(
            self.n_w_instance as usize,
            self.template.slots.len(),
            &instance,
            &witness,
        )?;
        let ecc = std.bls12_381();

        // 1. W's instance (private witness of D), verify W, fold, resolve, collapse.
        let mut w_inst: Vec<AssignedNative<F>> = Vec::new();
        for i in 0..self.n_w_instance as usize {
            w_inst.push(std.assign(l, witness.as_ref().map(|w| w.w_instance[i]))?);
        }
        let acc = verify_child_incircuit(
            std,
            l,
            &self.w_vk,
            &self.w_name(),
            &w_inst,
            witness.as_ref().map(|w| w.w_proof.clone()),
            true,
        )?;

        // 2. The two collapsed points, re-assigned with the subgroup check and
        //    tied to the accumulator through its encoding.
        let enc = std.verifier().as_public_input(l, &acc)?;
        let np = point_pi_len();
        if enc.len() != 2 * (np + 1) {
            return Err(Error::Synthesis(
                "unexpected accumulator encoding length".into(),
            ));
        }
        let accv = acc.value();
        let a_val = accv.as_ref().map(|a| a.lhs().bases()[0]);
        let b_val = accv.as_ref().map(|a| a.rhs().bases()[0]);
        let a: AG1 =
            AssignmentInstructions::<F, AG1>::assign(ecc, l, a_val.map(G1Projective::from))?;
        let b: AG1 =
            AssignmentInstructions::<F, AG1>::assign(ecc, l, b_val.map(G1Projective::from))?;
        let ea = ecc.as_public_input(l, &a)?;
        let eb = ecc.as_public_input(l, &b)?;
        for (x, y) in ea.iter().zip(&enc[..np]) {
            std.assert_equal(l, x, y)?;
        }
        for (x, y) in eb.iter().zip(&enc[np + 1..2 * np + 1]) {
            std.assert_equal(l, x, y)?;
        }
        std.assert_equal_to_fixed(l, &enc[np], F::from(1u64))?;
        std.assert_equal_to_fixed(l, &enc[2 * np + 1], F::from(1u64))?;

        // 3. e(A, [tau]2) * e(B, -[1]2) = 1, residue method.
        let g = PairingGadget::new(Tower::new(ecc.base_field_chip(), TowerOpts::default()));
        let (lt, ln) = kzg_decider_lines(&self.s_g2);
        let pa = g.prepare_assigned_g1(l, ecc, &a)?;
        let pb = g.prepare_assigned_g1(l, ecc, &b)?;
        g.assert_pairing_product_is_one(l, &[(pa, lt), (pb, ln)])?;

        // 4. stmt = H(DOM_STMT, instance_id, batch_no, selected W fields).
        let dom: AssignedNative<F> = std.assign_fixed(l, self.dom_stmt)?;
        let iid: AssignedNative<F> = std.assign(l, witness.as_ref().map(|w| w.instance_id))?;
        let bno: AssignedNative<F> = std.assign(l, witness.as_ref().map(|w| w.batch_no))?;
        let mut pre = vec![dom, iid, bno];
        pre.extend(self.stmt_fields.iter().map(|&i| w_inst[i as usize].clone()));
        let stmt = std.poseidon(l, &pre)?;

        // 4b. Bound mode only: cc must open as Poseidon(comm_rand, stmt) over
        //     the one-field encoded input and empty output of acceptValidity.
        let cc_bound = match self.comm_binding {
            CommBinding::Free => None,
            CommBinding::BoundPoseidonStmtV1 => {
                let rand: AssignedNative<F> =
                    std.assign(l, witness.as_ref().map(|w| w.comm_rand))?;
                Some(std.poseidon(l, &[rand, stmt.clone()])?)
            }
        };

        // 5. Public inputs in template order.
        for (idx, slot) in self.template.slots.iter().enumerate() {
            let x: AssignedNative<F> = match slot {
                Slot::BindingInput | Slot::CommunicationCommitment => {
                    std.assign(l, instance.as_ref().map(|v| v[idx]))?
                }
                Slot::Const(c) => std.assign_fixed(l, *c)?,
                Slot::Stmt => stmt.clone(),
                Slot::Value(_) => {
                    return Err(Error::Synthesis("value slot in the parent decider".into()));
                }
            };
            if let (Slot::CommunicationCommitment, Some(want)) = (slot, &cc_bound) {
                std.assert_equal(l, &x, want)?;
            }
            std.constrain_as_public_input(l, &x)?;
        }
        Ok(())
    }

    fn used_chips(&self) -> ZkStdLibArch {
        bls_arch(self.nr_cols)
    }

    fn write_relation<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(MAGIC)?;
        w.write_all(&[self.nr_cols])?;
        w.write_all(&[comm_binding_tag(self.comm_binding)])?;
        w.write_all(&self.n_w_instance.to_le_bytes())?;
        write_bytes(w, &vk_bytes(&self.w_vk))?;
        w.write_all(self.s_g2.to_bytes().as_ref())?;
        write_f(w, &self.dom_stmt)?;
        w.write_all(&(self.stmt_fields.len() as u32).to_le_bytes())?;
        for i in &self.stmt_fields {
            w.write_all(&i.to_le_bytes())?;
        }
        self.template.write(w)
    }

    fn read_relation<R: io::Read>(r: &mut R) -> io::Result<Self> {
        let mut magic = [0u8; 8];
        r.read_exact(&mut magic)?;
        if &magic != MAGIC {
            return Err(io::Error::other(
                "not a RECL9D02 decider relation (pre-opening RECL9D01 bytes are refused)",
            ));
        }
        let mut b = [0u8; 1];
        r.read_exact(&mut b)?;
        let nr_cols = b[0];
        r.read_exact(&mut b)?;
        let comm_binding = comm_binding_from_tag(b[0])?;
        let n_w_instance = read_u32(r)?;
        let w_vk = vk_from_bytes(&read_bytes(r, 1 << 20)?)?;
        let mut enc = <G2Affine as GroupEncoding>::Repr::default();
        r.read_exact(enc.as_mut())?;
        let s_g2: G2Affine = Option::from(G2Affine::from_bytes(&enc))
            .ok_or_else(|| io::Error::other("s_g2 not a valid G2 point"))?;
        let dom_stmt = read_f(r)?;
        let n = read_u32(r)? as usize;
        if n > 1024 {
            return Err(io::Error::other("too many stmt fields"));
        }
        let mut stmt_fields = Vec::with_capacity(n);
        for _ in 0..n {
            stmt_fields.push(read_u32(r)?);
        }
        let template = Template::read(r)?;
        let rel = DeciderRelation {
            nr_cols,
            w_vk,
            n_w_instance,
            s_g2,
            dom_stmt,
            stmt_fields,
            template,
            comm_binding,
        };
        rel.validate()?;
        Ok(rel)
    }
}

#[cfg(test)]
mod comm_binding_tests {
    use super::{CommBinding, comm_binding_from_tag, comm_binding_tag};

    #[test]
    fn comm_binding_tags_are_explicit_and_closed() {
        assert_eq!(comm_binding_tag(CommBinding::Free), 0);
        assert_eq!(comm_binding_tag(CommBinding::BoundPoseidonStmtV1), 1);
        assert_eq!(comm_binding_from_tag(0).unwrap(), CommBinding::Free);
        assert_eq!(
            comm_binding_from_tag(1).unwrap(),
            CommBinding::BoundPoseidonStmtV1
        );
        assert!(comm_binding_from_tag(2).is_err());
        assert!(comm_binding_from_tag(255).is_err());
    }
}

#[cfg(test)]
mod shape_tests {
    use super::*;

    fn witness(width: usize) -> Value<DeciderWitness> {
        Value::known(DeciderWitness {
            w_instance: vec![F::from(0); width],
            w_proof: vec![1],
            instance_id: F::from(0),
            batch_no: F::from(0),
            comm_rand: F::from(0),
        })
    }

    #[test]
    fn malformed_widths_are_errors_before_any_witness_indexing() {
        for (n_w, n_public) in [(28, 15), (29, 30)] {
            for width in [0, n_w - 1, n_w + 1] {
                assert!(
                    validate_known_shapes(
                        n_w,
                        n_public,
                        &Value::known(vec![F::from(0); n_public]),
                        &witness(width)
                    )
                    .is_err()
                );
            }
            for width in [0, n_public - 1, n_public + 1] {
                assert!(
                    validate_known_shapes(
                        n_w,
                        n_public,
                        &Value::known(vec![F::from(0); width]),
                        &witness(n_w)
                    )
                    .is_err()
                );
            }
            assert!(
                validate_known_shapes(
                    n_w,
                    n_public,
                    &Value::known(vec![F::from(0); n_public]),
                    &witness(n_w)
                )
                .is_ok()
            );
            assert!(
                validate_known_shapes(n_w, n_public, &Value::unknown(), &Value::unknown()).is_ok()
            );
        }
    }
}
