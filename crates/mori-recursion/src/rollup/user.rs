// Adapted from pinned recl9 gadgets; v3 changes described in b3-circuit.md.
//! Current STRAWMAN wallet relation. The old `leaf` relation is intentionally separate.
use super::i1::{
    fp::{self, F, dom},
    types::{NoteOpening, Point, Slot, Tx, TxPublic},
};
use ff::{Field, PrimeField};
use group::Group;
use midnight_circuits::{
    ecc::native::{AssignedNativePoint, AssignedScalarOfNativeCurve},
    instructions::*,
    types::{AssignedBit, AssignedNative},
};
use midnight_curves::{JubjubExtended as Jubjub, JubjubSubgroup};
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::Error,
};
use midnight_zk_stdlib::{Relation, ZkStdLib, ZkStdLibArch};
use std::io::{self, Read, Write};

pub const USER_FIELDS: usize = 16;
#[derive(Clone, Debug, PartialEq)]
pub struct UserInstance {
    pub fields: [F; USER_FIELDS],
}
impl UserInstance {
    /// [anchor,nf0,nf1,cm0,cm1,rkx,rky,kind,fee,token,amount,dest_present,dest,ct,expiry,flags].
    pub fn from_public(p: &TxPublic) -> Self {
        let slot = |v: &[Slot], i: usize| v.get(i).map(|s| s.value).unwrap_or(F::ZERO);
        let rk = p.rk.unwrap_or(Point {
            x: F::ZERO,
            y: F::ZERO,
        });
        Self {
            fields: [
                p.anchor_root,
                slot(&p.nullifiers, 0),
                slot(&p.nullifiers, 1),
                slot(&p.commitments, 0),
                slot(&p.commitments, 1),
                rk.x,
                rk.y,
                F::from(p.kind.code()),
                F::from(p.fee),
                F::from(p.pub_token),
                F::from(p.pub_amount),
                F::from(p.wd_dest.is_some() as u64),
                F::from(p.wd_dest.unwrap_or(0) as u64),
                p.ct_digest,
                F::from(p.expiry),
                F::from(p.active_flags),
            ],
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct UserRelation;
impl Relation for UserRelation {
    type Instance = UserInstance;
    type Witness = Tx;
    type Error = Error;
    fn used_chips(&self) -> ZkStdLibArch {
        ZkStdLibArch {
            poseidon: true,
            jubjub: true,
            nr_pow2range_cols: 4,
            ..Default::default()
        }
    }
    fn format_instance(i: &UserInstance) -> Result<Vec<F>, Error> {
        Ok(i.fields.to_vec())
    }
    fn circuit(
        &self,
        s: &ZkStdLib,
        l: &mut impl Layouter<F>,
        _: Value<UserInstance>,
        w: Value<Tx>,
    ) -> Result<(), Error> {
        for field in user_rules(s, l, w)? {
            s.constrain_as_public_input(l, &field)?;
        }
        Ok(())
    }
    fn write_relation<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"STU1")
    }
    fn read_relation<R: Read>(r: &mut R) -> io::Result<Self> {
        let mut b = [0; 4];
        r.read_exact(&mut b)?;
        if b != *b"STU1" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "wallet relation version",
            ));
        }
        Ok(Self)
    }
}

/// mono: the body of `UserRelation::circuit` (rec/src/user.rs), unchanged,
/// returning its 16 checked public cells instead of making them public.
pub fn user_rules(s: &ZkStdLib, l: &mut impl Layouter<F>, w: Value<Tx>) -> Result<Vec<A>, Error> {
    let g = Gadgets { s };
    let p = g.public(l, w.as_ref().map(|t| t.public.clone()))?;
    s.assert_false(l, &p.deposit)?;
    for flag in [&p.rk_present, &p.signature_present, &p.spender_present] {
        s.assert_true(l, flag)?;
    }
    let secret_value = w.as_ref().map(|t| {
        t.witness
            .spender_secret_key_jfr
            .as_ref()
            .and_then(|v| fp::jfr_from_hex(v))
    });
    let alpha_value = w.as_ref().map(|t| {
        t.witness
            .spender_alpha_jfr
            .as_ref()
            .and_then(|v| fp::jfr_from_hex(v))
    });
    for value in [secret_value, alpha_value] {
        let present: AssignedBit<F> = s.assign(l, value.map(|v| v.is_some()))?;
        s.assert_true(l, &present)?;
    }
    let sk: A = s.assign(
        l,
        secret_value.map(|v| fp::jfr_to_f(v.unwrap_or(fp::JFr::ZERO))),
    )?;
    let alpha: A = s.assign(
        l,
        alpha_value.map(|v| fp::jfr_to_f(v.unwrap_or(fp::JFr::ZERO))),
    )?;
    g.scalar(l, &sk)?;
    g.scalar(l, &alpha)?;
    s.assert_not_equal_to_fixed(l, &sk, F::ZERO)?;
    let (pk, pk_coords) = g.point(
        l,
        w.as_ref()
            .map(|t| t.witness.spender_public_key.unwrap_or(Point::identity())),
        None,
    )?;
    let (rk, rk_coords) = g.point(
        l,
        w.as_ref().map(|t| t.public.rk.unwrap_or(Point::identity())),
        None,
    )?;
    for (j, coordinate) in rk_coords.iter().enumerate() {
        s.assert_equal(l, coordinate, &p.fields[5 + j])?;
    }
    let generator = s.jubjub().assign_fixed(l, JubjubSubgroup::generator())?;
    let sk_scalar: AssignedScalarOfNativeCurve<Jubjub> = s.jubjub().convert(l, &sk)?;
    let derived_pk = s
        .jubjub()
        .msm(l, &[sk_scalar], std::slice::from_ref(&generator))?;
    s.jubjub().assert_equal(l, &pk, &derived_pk)?;
    let alpha_scalar: AssignedScalarOfNativeCurve<Jubjub> = s.jubjub().convert(l, &alpha)?;
    let alpha_g = s.jubjub().msm(l, &[alpha_scalar], &[generator])?;
    let derived_rk = EccInstructions::add(s.jubjub(), l, &pk, &alpha_g)?;
    s.jubjub().assert_equal(l, &rk, &derived_rk)?;
    let nk = g.hash(l, dom::nk(), &[sk])?;
    let supplied_nk: A = s.assign(l, w.as_ref().map(|t| t.witness.spender_nk))?;
    s.assert_equal(l, &nk, &supplied_nk)?;
    let zero: A = s.assign_fixed(l, F::ZERO)?;
    let mut ins = [zero.clone(), zero.clone()];
    let mut outs = [zero.clone(), zero.clone()];
    g.shape(l, w.as_ref().map(|t| t.witness.inputs.len()), 2)?;
    g.shape(l, w.as_ref().map(|t| t.witness.outputs.len()), 2)?;
    for i in 0..2 {
        let iw = w.as_ref().map(|t| t.witness.inputs.get(i).cloned());
        let enabled: AssignedBit<F> = s.assign(
            l,
            iw.as_ref().map(|v| v.as_ref().is_some_and(|v| v.enabled)),
        )?;
        s.assert_equal(l, &enabled, &p.nf_enabled[i])?;
        let opening = iw.as_ref().map(|v| {
            v.as_ref()
                .map(|v| v.note.clone())
                .unwrap_or_else(NoteOpening::zero)
        });
        let (cm, token, value, owner, rho) = g.opening(l, opening, &enabled)?;
        s.cond_assert_equal(l, &enabled, &token, &p.fields[9])?;
        for j in 0..2 {
            s.cond_assert_equal(l, &enabled, &owner[j], &pk_coords[j])?;
        }
        let index: A = s.assign(
            l,
            iw.as_ref()
                .map(|v| F::from(v.as_ref().map(|v| v.leaf_index as u64).unwrap_or(0))),
        )?;
        g.shape(
            l,
            iw.as_ref()
                .map(|v| v.as_ref().map(|v| v.membership_path.len()).unwrap_or(0)),
            fp::CM_DEPTH,
        )?;
        let path = g.assign_path(
            l,
            iw.as_ref().map(|v| {
                v.as_ref()
                    .map(|v| v.membership_path.clone())
                    .unwrap_or_default()
            }),
            fp::CM_DEPTH,
        )?;
        let root = g.path(l, &cm, &index, &path)?;
        s.cond_assert_equal(l, &enabled, &root, &p.fields[0])?;
        let h = g.hash(l, dom::nf(), &[nk.clone(), rho])?;
        let bits = s.assigned_to_le_bits(l, &h, None, true)?;
        let nf: A = s.assigned_from_le_bits(l, &bits[..248])?;
        s.cond_assert_equal(l, &enabled, &nf, &p.fields[1 + i])?;
        g.add_token(l, &mut ins, &token, &value)?;
        let ow = w.as_ref().map(|t| t.witness.outputs.get(i).cloned());
        let enabled: AssignedBit<F> = s.assign(
            l,
            ow.as_ref().map(|v| v.as_ref().is_some_and(|v| v.enabled)),
        )?;
        s.assert_equal(l, &enabled, &p.cm_enabled[i])?;
        let opening = ow.as_ref().map(|v| {
            v.as_ref()
                .map(|v| v.note.clone())
                .unwrap_or_else(NoteOpening::zero)
        });
        let (cm, token, value, _, _) = g.opening(l, opening, &enabled)?;
        s.cond_assert_equal(l, &enabled, &token, &p.fields[9])?;
        s.cond_assert_equal(l, &enabled, &cm, &p.fields[3 + i])?;
        g.add_token(l, &mut outs, &token, &value)?;
    }
    outs[0] = s.add(l, &outs[0], &p.fields[8])?;
    let wd = s.select(l, &p.withdrawal, &p.fields[10], &zero)?;
    g.add_token(l, &mut outs, &p.fields[9], &wd)?;
    for i in 0..2 {
        g.range(l, &ins[i], 66)?;
        g.range(l, &outs[i], 66)?;
        s.assert_equal(l, &ins[i], &outs[i])?;
    }
    Ok(p.fields)
}

pub type A = AssignedNative<F>;
pub struct PublicAssigned {
    pub fields: Vec<A>,
    pub nf_enabled: [AssignedBit<F>; 2],
    pub cm_enabled: [AssignedBit<F>; 2],
    pub deposit: AssignedBit<F>,
    pub withdrawal: AssignedBit<F>,
    pub rk_present: AssignedBit<F>,
    pub signature_present: AssignedBit<F>,
    pub spender_present: AssignedBit<F>,
}
pub struct Gadgets<'a> {
    pub s: &'a ZkStdLib,
}
impl Gadgets<'_> {
    pub fn range(&self, l: &mut impl Layouter<F>, x: &A, n: usize) -> Result<(), Error> {
        super::close::range(self.s, l, x, n)
    }
    pub fn shape(
        &self,
        l: &mut impl Layouter<F>,
        n: Value<usize>,
        expected: usize,
    ) -> Result<(), Error> {
        let n: A = self.s.assign(l, n.map(|n| F::from(n as u64)))?;
        self.s
            .assert_equal_to_fixed(l, &n, F::from(expected as u64))
    }
    pub fn hash(&self, l: &mut impl Layouter<F>, domain: F, fields: &[A]) -> Result<A, Error> {
        let mut v = vec![self.s.assign_fixed(l, domain)?];
        v.extend_from_slice(fields);
        self.s.poseidon(l, &v)
    }
    pub fn strict_if(
        &self,
        l: &mut impl Layouter<F>,
        enabled: &AssignedBit<F>,
        a: &A,
        b: &A,
        n: usize,
    ) -> Result<(), Error> {
        let d = self.s.sub(l, b, a)?;
        let d = self.s.add_constant(l, &d, -F::ONE)?;
        let zero: A = self.s.assign_fixed(l, F::ZERO)?;
        let d = self.s.select(l, enabled, &d, &zero)?;
        self.range(l, &d, n)
    }
    pub fn leq(&self, l: &mut impl Layouter<F>, a: &A, b: &A, n: usize) -> Result<(), Error> {
        let d = self.s.sub(l, b, a)?;
        self.range(l, &d, n)
    }
    pub fn scalar(&self, l: &mut impl Layouter<F>, x: &A) -> Result<(), Error> {
        let mut b = (-fp::JFr::ONE).to_repr();
        let mut carry = 1u16;
        for byte in b.as_mut() {
            let x = *byte as u16 + carry;
            *byte = x as u8;
            carry = x >> 8;
        }
        let mut repr = <F as PrimeField>::Repr::default();
        repr.as_mut().copy_from_slice(b.as_ref());
        let order: A = self.s.assign_fixed(l, F::from_repr(repr).unwrap())?;
        self.range(l, x, fp::JFr::NUM_BITS as usize)?;
        let yes: AssignedBit<F> = self.s.assign_fixed(l, true)?;
        self.strict_if(l, &yes, x, &order, fp::JFr::NUM_BITS as usize)
    }
    pub fn point(
        &self,
        l: &mut impl Layouter<F>,
        value: Value<Point>,
        active: Option<&AssignedBit<F>>,
    ) -> Result<(AssignedNativePoint<Jubjub>, [A; 2]), Error> {
        let coords: [A; 2] = [
            self.s.assign(l, value.map(|v| v.x))?,
            self.s.assign(l, value.map(|v| v.y))?,
        ];
        let subgroup = value.map(|v| {
            super::i1::keys::point_to_subgroup(v).unwrap_or_else(JubjubSubgroup::identity)
        });
        let point: AssignedNativePoint<Jubjub> = self.s.jubjub().assign(l, subgroup)?;
        self.s
            .assert_equal(l, &coords[0], &self.s.jubjub().x_coordinate(&point))?;
        self.s
            .assert_equal(l, &coords[1], &self.s.jubjub().y_coordinate(&point))?;
        let identity = self
            .s
            .jubjub()
            .is_equal_to_fixed(l, &point, JubjubSubgroup::identity())?;
        if let Some(active) = active {
            let forbidden = self.s.and(l, &[active.clone(), identity])?;
            self.s.assert_false(l, &forbidden)?;
        } else {
            self.s.assert_false(l, &identity)?;
        }
        Ok((point, coords))
    }
    pub fn assign_path(
        &self,
        l: &mut impl Layouter<F>,
        path: Value<Vec<F>>,
        depth: usize,
    ) -> Result<Vec<A>, Error> {
        // Shape checks belong to the caller. Safe indexing lets malformed
        // witnesses reach circuit verification instead of panicking on the host.
        (0..depth)
            .map(|i| {
                self.s.assign(
                    l,
                    path.as_ref().map(|v| v.get(i).copied().unwrap_or(F::ZERO)),
                )
            })
            .collect()
    }
    pub fn path(
        &self,
        l: &mut impl Layouter<F>,
        leaf: &A,
        index: &A,
        path: &[A],
    ) -> Result<A, Error> {
        let bits = self
            .s
            .assigned_to_le_bits(l, index, Some(path.len()), true)?;
        let mut root = leaf.clone();
        for (i, sib) in path.iter().enumerate() {
            let (left, right) = self.s.cond_swap(l, &bits[i], &root, sib)?;
            root = self.hash(l, dom::node(), &[left, right])?;
        }
        Ok(root)
    }
    pub fn add_token(
        &self,
        l: &mut impl Layouter<F>,
        sum: &mut [A; 2],
        token: &A,
        value: &A,
    ) -> Result<(), Error> {
        let zero: A = self.s.assign_fixed(l, F::ZERO)?;
        for (i, tok) in [fp::TOK_USD, fp::TOK_NIGHT].into_iter().enumerate() {
            let yes = self.s.is_equal_to_fixed(l, token, F::from(tok))?;
            let amount = self.s.select(l, &yes, value, &zero)?;
            sum[i] = self.s.add(l, &sum[i], &amount)?;
        }
        Ok(())
    }
    pub fn opening(
        &self,
        l: &mut impl Layouter<F>,
        w: Value<NoteOpening>,
        active: &AssignedBit<F>,
    ) -> Result<(A, A, A, [A; 2], A), Error> {
        let token: A = self.s.assign(l, w.as_ref().map(|v| F::from(v.token)))?;
        let amount: A = self.s.assign(l, w.as_ref().map(|v| F::from(v.value)))?;
        self.range(l, &token, 64)?;
        self.range(l, &amount, 64)?;
        let owner = [
            self.s.assign(l, w.as_ref().map(|v| v.owner_public_key.x))?,
            self.s.assign(l, w.as_ref().map(|v| v.owner_public_key.y))?,
        ];
        let (_, checked_owner) =
            self.point(l, w.as_ref().map(|v| v.owner_public_key), Some(active))?;
        for j in 0..2 {
            self.s.assert_equal(l, &owner[j], &checked_owner[j])?;
        }
        let rho: A = self.s.assign(l, w.as_ref().map(|v| v.rho))?;
        let r: A = self.s.assign(l, w.as_ref().map(|v| v.r))?;
        let zero: A = self.s.assign_fixed(l, F::ZERO)?;
        let one: A = self.s.assign_fixed(l, F::ONE)?;
        let inactive = self.s.not(l, active)?;
        for x in [&token, &amount, &owner[0], &rho, &r] {
            self.s.cond_assert_equal(l, &inactive, x, &zero)?;
        }
        self.s.cond_assert_equal(l, &inactive, &owner[1], &one)?;
        let usd = self.s.is_equal_to_fixed(l, &token, F::from(fp::TOK_USD))?;
        let night = self
            .s
            .is_equal_to_fixed(l, &token, F::from(fp::TOK_NIGHT))?;
        let allowed = self.s.or(l, &[usd, night])?;
        let invalid = self.s.not(l, &allowed)?;
        let forbidden = self.s.and(l, &[active.clone(), invalid])?;
        self.s.assert_false(l, &forbidden)?;
        self.strict_if(l, active, &zero, &amount, 64)?;
        let cm = self.hash(
            l,
            dom::cm(),
            &[
                token.clone(),
                amount.clone(),
                owner[0].clone(),
                owner[1].clone(),
                rho.clone(),
                r,
            ],
        )?;
        Ok((cm, token, amount, owner, rho))
    }
    pub fn public(
        &self,
        l: &mut impl Layouter<F>,
        w: Value<TxPublic>,
    ) -> Result<PublicAssigned, Error> {
        self.shape(l, w.as_ref().map(|p| p.nullifiers.len()), 2)?;
        self.shape(l, w.as_ref().map(|p| p.commitments.len()), 2)?;
        let fields: Vec<A> = (0..USER_FIELDS)
            .map(|i| {
                self.s.assign(
                    l,
                    w.as_ref().map(|p| UserInstance::from_public(p).fields[i]),
                )
            })
            .collect::<Result<_, _>>()?;
        let nf_enabled: [AssignedBit<F>; 2] = [
            self.s.assign(
                l,
                w.as_ref()
                    .map(|p| p.nullifiers.first().is_some_and(|v| v.enabled)),
            )?,
            self.s.assign(
                l,
                w.as_ref()
                    .map(|p| p.nullifiers.get(1).is_some_and(|v| v.enabled)),
            )?,
        ];
        let cm_enabled: [AssignedBit<F>; 2] = [
            self.s.assign(
                l,
                w.as_ref()
                    .map(|p| p.commitments.first().is_some_and(|v| v.enabled)),
            )?,
            self.s.assign(
                l,
                w.as_ref()
                    .map(|p| p.commitments.get(1).is_some_and(|v| v.enabled)),
            )?,
        ];
        let rk_present: AssignedBit<F> = self.s.assign(l, w.as_ref().map(|p| p.rk.is_some()))?;
        let signature_present: AssignedBit<F> = self
            .s
            .assign(l, w.as_ref().map(|p| p.signature.is_some()))?;
        let spender_present: AssignedBit<F> =
            self.s.assign(l, w.as_ref().map(|p| p.spender.is_some()))?;
        let dest_present: AssignedBit<F> =
            self.s.assign(l, w.as_ref().map(|p| p.wd_dest.is_some()))?;
        let mut flags: A = self.s.assign_fixed(l, F::ZERO)?;
        for (bit, coefficient) in nf_enabled
            .iter()
            .chain(cm_enabled.iter())
            .chain([&signature_present, &dest_present])
            .zip([1u64, 2, 4, 8, 16, 32])
        {
            let b: A = self.s.convert(l, bit)?;
            let term = self.s.mul_by_constant(l, &b, F::from(coefficient))?;
            flags = self.s.add(l, &flags, &term)?;
        }
        self.s.assert_equal(l, &flags, &fields[15])?;
        let zero: A = self.s.assign_fixed(l, F::ZERO)?;
        for i in 0..2 {
            for (en, x) in [
                (&nf_enabled[i], &fields[1 + i]),
                (&cm_enabled[i], &fields[3 + i]),
            ] {
                let off = self.s.not(l, en)?;
                self.s.cond_assert_equal(l, &off, x, &zero)?;
                self.strict_nonzero(l, en, x)?;
            }
            self.range(l, &fields[1 + i], 248)?;
        }
        for i in [8, 9, 10, 14] {
            self.range(l, &fields[i], 64)?;
        }
        self.range(l, &fields[12], 32)?;
        let kind = &fields[7];
        let mut kinds = Vec::new();
        for i in 0..6 {
            kinds.push(self.s.is_equal_to_fixed(l, kind, F::from(i))?);
        }
        let allowed = self.s.or(l, &kinds)?;
        self.s.assert_true(l, &allowed)?;
        let deposit = kinds[4].clone();
        let withdrawal = kinds[5].clone();
        let spend = self.s.not(l, &deposit)?;
        let basic = self.s.or(l, &kinds[..4])?;
        self.s.cond_assert_equal(l, &basic, &fields[10], &zero)?;
        let usd = self
            .s
            .is_equal_to_fixed(l, &fields[9], F::from(fp::TOK_USD))?;
        let night = self
            .s
            .is_equal_to_fixed(l, &fields[9], F::from(fp::TOK_NIGHT))?;
        let token_allowed = self.s.or(l, &[usd, night])?;
        self.s.assert_true(l, &token_allowed)?;
        let fee: A = self.s.assign_fixed(l, F::from(fp::FLAT_FEE))?;
        self.s.cond_assert_equal(l, &spend, &fields[8], &fee)?;
        self.s.cond_assert_equal(l, &deposit, &fields[8], &zero)?;
        self.s.assert_equal(l, &dest_present, &withdrawal)?;
        let off = self.s.not(l, &dest_present)?;
        self.s.cond_assert_equal(l, &off, &fields[12], &zero)?;
        let wd_field: A = self.s.convert(l, &dest_present)?;
        self.s.assert_equal(l, &wd_field, &fields[11])?;
        let transfer = self.s.or(l, &[deposit.clone(), withdrawal.clone()])?;
        self.strict_if(l, &transfer, &zero, &fields[10], 64)?;
        let has_input = self.s.or(l, &nf_enabled)?;
        let no_input = self.s.not(l, &has_input)?;
        let forbidden = self.s.and(l, &[spend.clone(), no_input])?;
        self.s.assert_false(l, &forbidden)?;
        for en in &nf_enabled {
            let forbidden = self.s.and(l, &[deposit.clone(), en.clone()])?;
            self.s.assert_false(l, &forbidden)?;
        }
        let ni0: A = self.s.convert(l, &nf_enabled[0])?;
        let ni1: A = self.s.convert(l, &nf_enabled[1])?;
        let no0: A = self.s.convert(l, &cm_enabled[0])?;
        let no1: A = self.s.convert(l, &cm_enabled[1])?;
        let input_count = self.s.add(l, &ni0, &ni1)?;
        let output_count = self.s.add(l, &no0, &no1)?;
        let one: A = self.s.assign_fixed(l, F::ONE)?;
        let two: A = self.s.assign_fixed(l, F::from(2))?;
        let double = self.s.or(l, &[kinds[0].clone(), kinds[3].clone()])?;
        self.s.cond_assert_equal(l, &double, &input_count, &two)?;
        self.s.cond_assert_equal(l, &kinds[1], &input_count, &one)?;
        self.s
            .cond_assert_equal(l, &kinds[1], &output_count, &one)?;
        self.s.cond_assert_equal(l, &deposit, &output_count, &one)?;
        let has_output = self.s.or(l, &cm_enabled)?;
        let no_output = self.s.not(l, &has_output)?;
        let required = self.s.or(l, &[basic, deposit.clone()])?;
        let forbidden = self.s.and(l, &[required, no_output])?;
        self.s.assert_false(l, &forbidden)?;
        Ok(PublicAssigned {
            fields,
            nf_enabled,
            cm_enabled,
            deposit,
            withdrawal,
            rk_present,
            signature_present,
            spender_present,
        })
    }
    fn strict_nonzero(
        &self,
        l: &mut impl Layouter<F>,
        en: &AssignedBit<F>,
        x: &A,
    ) -> Result<(), Error> {
        let z = self.s.is_equal_to_fixed(l, x, F::ZERO)?;
        let forbidden = self.s.and(l, &[en.clone(), z])?;
        self.s.assert_false(l, &forbidden)
    }
}
