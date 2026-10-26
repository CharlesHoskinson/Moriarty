//! The general-run decider: the terminal ledger-9 relation over the last
//! Sherlock IVC node proof (`ivc.rs`), installed as the `acceptRun` verifier
//! of `tools/sherlock-sdk/sherlock-run.compact`.
//!
//! It verifies the node proof under the node key fixed as constants, pins the
//! node's `self_vk_repr` lane to that key, folds the carried accumulator,
//! resolves every node fixed base and checks the KZG pairing in circuit (the
//! residue route of `decider.rs`). It then opens the final state digest as a
//! halted state with a `u128` result and exposes the public values
//! `[prog_d, init_d, fin_d, n_steps, result]` in the `acceptRun` transcript
//! template (`Slot::Value`), with `stmt = H(DOM, prog_d, init_d, fin_d, n, result)`
//! and the communication commitment bound to `Poseidon(comm_rand, inputs)`.
//!
//! The relation is program-independent: one key serves every program; a
//! deployment binds its program and initial-state digests through the
//! ledger reads in the template.
use std::{collections::BTreeMap, io};

use ff::{Field, PrimeField};
use group::{Group, GroupEncoding, prime::PrimeCurveAffine};

use midnight_circuits::{
    instructions::{
        AssertionInstructions, AssignmentInstructions, DecompositionInstructions,
        PublicInputInstructions,
    },
    types::{AssignedNative, InnerValue},
    verifier::{Accumulator, fixed_bases},
};
use midnight_curves::{G1Projective, G2Affine};
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};
use midnight_zk_stdlib::{MidnightVK, Relation, ZkStdLib, ZkStdLibArch};

use crate::{
    bls_arch,
    inner::{C, S, point_pi_len},
    ivc::{IVC_STATE, SELF_NAME, state_digest_halted},
    pairing::{AG1, PairingGadget, kzg_decider_lines},
    parent::{dom, poseidon},
    template::{Slot, Template},
    tower::{Tower, TowerOpts},
    util::{F, read_bytes, read_f, read_u32, vk_bytes, vk_from_bytes, write_bytes, write_f},
};

const MAGIC: &[u8; 8] = b"SHKRUND1";
pub const DOM_RUN: &str = "SHK_RUN";
/// Public values: prog_d, init_d, fin_d, n_steps, result_be. `result_be` is the
/// field representation of the contract's `Bytes<16>` big-endian result: the
/// little-endian integer of the 16 big-endian bytes of the u128 result.
pub const N_VALUES: usize = 5;
pub const V_PROG: u32 = 0;
pub const V_INIT: u32 = 1;
pub const V_FIN: u32 = 2;
pub const V_STEPS: u32 = 3;
pub const V_RESULT: u32 = 4;
/// A communication-commitment input that is the statement digest.
pub const CC_STMT: u32 = u32::MAX;

#[derive(Clone, Debug)]
pub struct RunDeciderRelation {
    pub nr_cols: u8,
    pub node_vk: MidnightVK,
    pub s_g2: G2Affine,
    pub template: Template,
    /// The `acceptRun` inputs in order, as `CC_STMT` or a value index; the
    /// communication commitment is Poseidon(comm_rand, these fields).
    pub cc_inputs: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct RunDeciderWitness {
    pub w_instance: Vec<F>,
    pub w_proof: Vec<u8>,
    pub w_acc: Accumulator<S>,
    pub result: u128,
    pub lanes_d: F,
    pub comm_rand: F,
}

impl RunDeciderRelation {
    /// Fixed-base names of the node key (the carried accumulator's scalars).
    pub fn node_base_names(&self) -> Vec<String> {
        fixed_bases::<S>(SELF_NAME, self.node_vk.vk())
            .into_keys()
            .collect()
    }

    pub fn n_w_instance(&self) -> usize {
        1 + IVC_STATE
            + <midnight_circuits::verifier::AssignedAccumulator<S> as midnight_circuits::types::Instantiable<F>>::as_public_input(
                &self.trivial_carried(),
            )
            .len()
    }

    fn trivial_carried(&self) -> Accumulator<S> {
        use midnight_circuits::verifier::Msm;
        let zeros: BTreeMap<String, F> = self
            .node_base_names()
            .into_iter()
            .map(|n| (n, F::ZERO))
            .collect();
        Accumulator::new(
            Msm::new(&[C::default()], &[F::ONE], &BTreeMap::new()),
            Msm::new(&[C::default()], &[F::ONE], &zeros),
        )
    }

    pub fn validate(&self) -> io::Result<()> {
        if !(1..=4).contains(&self.nr_cols) {
            return Err(io::Error::other("nr_cols must be in 1..=4"));
        }
        if bool::from(self.s_g2.is_identity()) {
            return Err(io::Error::other("s_g2 is the identity"));
        }
        self.template.validate()?;
        for s in &self.template.slots {
            if let Slot::Value(i) = s
                && *i as usize >= N_VALUES
            {
                return Err(io::Error::other("value slot index out of range"));
            }
        }
        if self
            .cc_inputs
            .iter()
            .any(|&i| i != CC_STMT && i as usize >= N_VALUES)
        {
            return Err(io::Error::other("cc input index out of range"));
        }
        Ok(())
    }

    /// The public values for a node instance and its halted result.
    pub fn values(w_instance: &[F], result: u128) -> [F; N_VALUES] {
        [
            w_instance[1],
            w_instance[2],
            w_instance[3],
            w_instance[4],
            F::from_u128(result_be(result)),
        ]
    }

    pub fn stmt(values: &[F; N_VALUES]) -> F {
        let mut pre = vec![dom(DOM_RUN)];
        pre.extend_from_slice(values);
        poseidon(&pre)
    }

    pub fn comm(&self, stmt: F, values: &[F; N_VALUES], comm_rand: F) -> F {
        let mut pre = vec![comm_rand];
        for &i in &self.cc_inputs {
            pre.push(if i == CC_STMT {
                stmt
            } else {
                values[i as usize]
            });
        }
        poseidon(&pre)
    }

    /// The ledger public-input vector of the call.
    pub fn public_inputs(&self, binding_input: F, w: &RunDeciderWitness) -> Option<Vec<F>> {
        let v = Self::values(&w.w_instance, w.result);
        let stmt = Self::stmt(&v);
        let cc = self.comm(stmt, &v, w.comm_rand);
        self.template
            .public_inputs_with(binding_input, cc, stmt, &v)
    }
}

/// The u128 whose little-endian bytes are the big-endian bytes of `r`.
pub fn result_be(r: u128) -> u128 {
    u128::from_le_bytes(r.to_be_bytes())
}

impl Relation for RunDeciderRelation {
    type Instance = Vec<F>;
    type Witness = RunDeciderWitness;
    type Error = Error;

    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }

    fn circuit(
        &self,
        std: &ZkStdLib,
        l: &mut impl Layouter<F>,
        instance: Value<Vec<F>>,
        witness: Value<RunDeciderWitness>,
    ) -> Result<(), Error> {
        let verifier = std.verifier();
        let ecc = std.bls12_381();
        let n = self.n_w_instance();

        // 1. Node instance, own key pinned.
        let mut w_inst: Vec<AssignedNative<F>> = Vec::with_capacity(n);
        for i in 0..n {
            w_inst.push(std.assign(l, witness.as_ref().map(|w| w.w_instance[i]))?);
        }
        std.assert_equal_to_fixed(l, &w_inst[0], self.node_vk.vk().transcript_repr())?;

        // 2. Verify the node proof, fold its carried accumulator, resolve, collapse.
        let pvk = self.node_vk.vk();
        let assigned_vk = verifier.assign_fixed_vk(
            l,
            SELF_NAME,
            pvk.get_domain(),
            pvk.cs(),
            pvk.transcript_repr(),
        )?;
        let mut bases = BTreeMap::new();
        for (name, base) in fixed_bases::<S>(SELF_NAME, pvk) {
            bases.insert(name, ecc.assign_fixed(l, base)?);
        }
        let committed = [ecc.assign_fixed(l, C::identity())?];
        let own = verifier.prepare(
            l,
            &assigned_vk,
            &committed,
            &[w_inst.as_slice()],
            witness.as_ref().map(|w| w.w_proof.clone()),
        )?;
        let carried = verifier.assign_collapsed_accumulator(
            l,
            &self.node_base_names(),
            witness.as_ref().map(|w| w.w_acc.clone()),
        )?;
        let enc = verifier.as_public_input(l, &carried)?;
        if enc.len() != n - 1 - IVC_STATE {
            return Err(Error::Synthesis(
                "carried accumulator encoding length".into(),
            ));
        }
        for (a, b) in w_inst[1 + IVC_STATE..].iter().zip(enc.iter()) {
            std.assert_equal(l, a, b)?;
        }
        let mut acc = verifier.accumulate(l, &[own, carried])?;
        acc.resolve_fixed_bases(&bases);
        acc.collapse(l, ecc, ecc.scalar_field_chip())?;

        // 3. Pairing of the collapsed points (as in decider.rs).
        let enc = verifier.as_public_input(l, &acc)?;
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
        std.assert_equal_to_fixed(l, &enc[np], F::ONE)?;
        std.assert_equal_to_fixed(l, &enc[2 * np + 1], F::ONE)?;
        let g = PairingGadget::new(Tower::new(ecc.base_field_chip(), TowerOpts::default()));
        let (lt, ln) = kzg_decider_lines(&self.s_g2);
        let pa = g.prepare_assigned_g1(l, ecc, &a)?;
        let pb = g.prepare_assigned_g1(l, ecc, &b)?;
        g.assert_pairing_product_is_one(l, &[(pa, lt), (pb, ln)])?;

        // 4. The final state is halted with a u128 result.
        let result: AssignedNative<F> =
            std.assign(l, witness.as_ref().map(|w| F::from_u128(w.result)))?;
        // 16 bytes: range check result < 2^128 and its big-endian field form.
        let le = std.assigned_to_le_bytes(l, &result, Some(16))?;
        let result_be: AssignedNative<F> = std.assigned_from_be_bytes(l, &le)?;
        let lanes_d: AssignedNative<F> = std.assign(l, witness.as_ref().map(|w| w.lanes_d))?;
        let sd_dom: AssignedNative<F> = std.assign_fixed(l, state_digest_halted().0)?;
        let halted: AssignedNative<F> = std.assign_fixed(l, state_digest_halted().1)?;
        let fin = std.poseidon(l, &[sd_dom, halted, result.clone(), lanes_d])?;
        std.assert_equal(l, &fin, &w_inst[3])?;

        // 5. Public values, stmt and the bound communication commitment.
        let values = [
            w_inst[1].clone(),
            w_inst[2].clone(),
            w_inst[3].clone(),
            w_inst[4].clone(),
            result_be,
        ];
        let d: AssignedNative<F> = std.assign_fixed(l, dom(DOM_RUN))?;
        let mut pre = vec![d];
        pre.extend(values.iter().cloned());
        let stmt = std.poseidon(l, &pre)?;
        let rand: AssignedNative<F> = std.assign(l, witness.as_ref().map(|w| w.comm_rand))?;
        let mut cpre = vec![rand];
        for &i in &self.cc_inputs {
            cpre.push(if i == CC_STMT {
                stmt.clone()
            } else {
                values[i as usize].clone()
            });
        }
        let cc = std.poseidon(l, &cpre)?;

        // 6. Public inputs in template order.
        for (idx, slot) in self.template.slots.iter().enumerate() {
            let x: AssignedNative<F> = match slot {
                Slot::BindingInput => std.assign(l, instance.as_ref().map(|v| v[idx]))?,
                Slot::CommunicationCommitment => {
                    let x: AssignedNative<F> = std.assign(l, instance.as_ref().map(|v| v[idx]))?;
                    std.assert_equal(l, &x, &cc)?;
                    x
                }
                Slot::Const(c) => std.assign_fixed(l, *c)?,
                Slot::Stmt => stmt.clone(),
                Slot::Value(i) => values[*i as usize].clone(),
            };
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
        write_bytes(w, &vk_bytes(&self.node_vk))?;
        w.write_all(self.s_g2.to_bytes().as_ref())?;
        w.write_all(&(self.cc_inputs.len() as u32).to_le_bytes())?;
        for i in &self.cc_inputs {
            w.write_all(&i.to_le_bytes())?;
        }
        write_f(w, &dom(DOM_RUN))?;
        self.template.write(w)
    }

    fn read_relation<R: io::Read>(r: &mut R) -> io::Result<Self> {
        let mut magic = [0u8; 8];
        r.read_exact(&mut magic)?;
        if &magic != MAGIC {
            return Err(io::Error::other("not a SHKRUND1 run decider relation"));
        }
        let mut b = [0u8; 1];
        r.read_exact(&mut b)?;
        let node_vk = vk_from_bytes(&read_bytes(r, 1 << 20)?)?;
        let mut enc = <G2Affine as GroupEncoding>::Repr::default();
        r.read_exact(enc.as_mut())?;
        let s_g2: G2Affine = Option::from(G2Affine::from_bytes(&enc))
            .ok_or_else(|| io::Error::other("s_g2 not a valid G2 point"))?;
        let n = read_u32(r)? as usize;
        if n > 16 {
            return Err(io::Error::other("too many cc inputs"));
        }
        let mut cc_inputs = Vec::with_capacity(n);
        for _ in 0..n {
            cc_inputs.push(read_u32(r)?);
        }
        if read_f(r)? != dom(DOM_RUN) {
            return Err(io::Error::other("run decider domain differs"));
        }
        let template = Template::read(r)?;
        let rel = Self {
            nr_cols: b[0],
            node_vk,
            s_g2,
            template,
            cc_inputs,
        };
        rel.validate()?;
        Ok(rel)
    }
}
