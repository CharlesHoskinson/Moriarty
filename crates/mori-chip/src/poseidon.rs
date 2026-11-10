//! HASH (experiment E7): a Poseidon-only chip, fed by bus provider rows.
//!
//! The permutation is the one of `midnight-circuits` 7.3.0 (the one
//! `midnight-zk-stdlib` 2.4.0 composes): width 3, rate 2, S-box `x^5`, 8 full and
//! 60 partial rounds, the crate's MDS matrix and round constants, in the crate's
//! "shifted" form (`RC[0]` is added to the input, round `r` adds `RC[r + 1]`, the
//! last round adds zero). The sponge is the stdlib fixed-length sponge: the
//! capacity element starts at the input length, inputs are added two at a time
//! to the rate elements, one permutation per chunk, output = state element 0. So
//! a digest made here equals `PoseidonChip::hash` (`hash_native`) on the same
//! inputs; `tests::differential_*` checks this on random inputs.
//!
//! What differs from the stdlib chip is the layout, chosen for the node cost (a
//! leaf column costs the node about 6,000 to 8,000 rows, `experiments-2.md` 2.3):
//! no `NativeChip` (no coefficient columns, no constant column, no instance
//! equality), no partial-round skips (3 round-constant columns, not 8), and no
//! S-box hints (a round is one degree-6 gate, the leaf's degree already). The
//! state lives in shared advice columns 0..3; a permutation is 70 rows: one
//! absorb row, 68 round rows, one output row.
//!
//! Absorb row (columns 0..5 = the bus tuple `(HASH, key, in0, in1, out)`):
//! - `q_first`: the next row's state is `(in0, in1, len) + RC[0]`; `len` is folded
//!   into the round-constant column 2 of the absorb row, so it is fixed by the
//!   circuit, never by the prover. On a provider row (`q_prov = 1`) the gate also
//!   pins `chip = HASH` and `key = 2` (a provider serves only 2-input hashes, so
//!   its output is a function of `(in0, in1)`).
//! - `q_chain`: the next row's state is `prev + (in0, in1, 0) + RC[0]`, where
//!   `prev` is the previous permutation's output row (the row above).
//! - `q_tail`: as `q_chain` for a last chunk of one input (`in1` not absorbed).
//!
//! `out` of a provider row is copy-constrained to the output row's element 0.
//! Inputs given as cells are copy-constrained into `in0`, `in1`.
//!
//! One absorb column (E8, `one_abs`): the three absorb selectors become codes 1
//! (first), 2 (chain) and 3 (tail) of one fixed column ([`crate::vsel`]); each
//! absorb gate is multiplied by the degree-3 Lagrange indicator of its code, so
//! the gates and their names are unchanged and the key has 2 fewer fixed columns.
use ff::Field;
use midnight_circuits::hash::poseidon::constants::PoseidonField;
use midnight_proofs::{
    circuit::{AssignedCell, Region, Value},
    plonk::{Column, ConstraintSystem, Error, Expression, Fixed, Selector},
    poly::Rotation,
};

use crate::{
    F,
    bus::BusConfig,
    vsel::{VSel, code_column},
};

/// Chip id of HASH on the bus (as in [`crate::hashchip`]).
pub const CHIP_HASH: u64 = 3;
/// State width.
pub const WIDTH: usize = 3;
/// Rounds per permutation (8 full, 60 partial).
pub const ROUNDS: usize = 68;
/// Rows per permutation: absorb row, round rows, output row.
pub const ROWS_PER_PERM: usize = ROUNDS + 2;
/// Key of a bus-served hash: its input count.
pub const BUS_KEY: u64 = 2;

fn is_full(r: usize) -> bool {
    !(4..64).contains(&r)
}

/// Constants added by round `r` (shifted form).
pub fn rc_round(r: usize) -> [F; WIDTH] {
    if r == ROUNDS - 1 {
        [F::ZERO; WIDTH]
    } else {
        <F as PoseidonField>::ROUND_CONSTANTS[r + 1]
    }
}

pub fn rc0() -> [F; WIDTH] {
    <F as PoseidonField>::ROUND_CONSTANTS[0]
}

fn pow5(x: F) -> F {
    x.square().square() * x
}

/// Round `r` on the CPU.
pub fn round_native(r: usize, s: [F; WIDTH]) -> [F; WIDTH] {
    let m = <F as PoseidonField>::MDS;
    let u = if is_full(r) {
        s.map(pow5)
    } else {
        [s[0], s[1], pow5(s[2])]
    };
    let c = rc_round(r);
    std::array::from_fn(|i| c[i] + m[i][0] * u[0] + m[i][1] * u[1] + m[i][2] * u[2])
}

/// The fixed-length sponge on the CPU, in this chip's form.
pub fn sponge_native(inputs: &[F]) -> F {
    assert!(!inputs.is_empty());
    let mut st = [F::ZERO, F::ZERO, F::from(inputs.len() as u64)];
    for chunk in inputs.chunks(2) {
        for (i, x) in chunk.iter().enumerate() {
            st[i] += x;
        }
        for (i, c) in rc0().iter().enumerate() {
            st[i] += c;
        }
        st = (0..ROUNDS).fold(st, |s, r| round_native(r, s));
    }
    st[0]
}

/// Rows of a sponge over `n` inputs.
pub fn rows_for(n: usize) -> usize {
    n.div_ceil(2) * ROWS_PER_PERM
}

#[derive(Clone, Debug)]
pub struct PoseidonCfg {
    pub bus: BusConfig,
    pub rc: [Column<Fixed>; WIDTH],
    pub q_full: Selector,
    pub q_part: Selector,
    pub q_first: VSel,
    pub q_chain: VSel,
    pub q_tail: VSel,
}

/// An input of a sponge: a cell to copy in, or a free value.
#[derive(Clone, Debug)]
pub enum HIn {
    Cell(AssignedCell<F, F>),
    Val(Value<F>),
    /// Negative tests: copy-constrained to the cell, but witnessed as the value
    /// (a prover that hashes something other than the cell it claims to).
    CellAs(AssignedCell<F, F>, Value<F>),
}

impl HIn {
    pub fn value(&self) -> Value<F> {
        match self {
            HIn::Cell(c) => c.value().copied(),
            HIn::Val(v) | HIn::CellAs(_, v) => *v,
        }
    }
}

/// Negative-test knobs. Permutations are counted in assignment order from 0;
/// rows within a permutation: 0 absorb, `1 + r` round `r`, 69 output.
#[derive(Clone, Debug, Default)]
pub struct Tamper {
    /// `(perm, row, column, delta)`: add `delta` to a state cell; later rows are
    /// computed from the tampered value, so only the producing gate breaks.
    pub state: Option<(usize, usize, usize, F)>,
    /// `(perm, row, column, delta)`: add `delta` to a round-constant cell.
    pub rc: Option<(usize, usize, usize, F)>,
    /// Permutation whose absorb row does not add its inputs to the state.
    pub skip_absorb: Option<usize>,
    /// Key written on provider rows instead of [`BUS_KEY`].
    pub key: Option<F>,
    /// Chip id written on provider rows instead of [`CHIP_HASH`].
    pub chip: Option<F>,
    /// Added to the provider row's `out` cell (its copy from the output row stays).
    pub out: Option<F>,
}

impl PoseidonCfg {
    /// `one_abs`: the three absorb selectors share one code column.
    pub fn configure(meta: &mut ConstraintSystem<F>, bus: BusConfig, one_abs: bool) -> Self {
        let a = bus.cols.a;
        let rc: [Column<Fixed>; WIDTH] = std::array::from_fn(|_| meta.fixed_column());
        let q_full = meta.selector();
        let q_part = meta.selector();
        let (q_first, q_chain, q_tail) = if one_abs {
            let c = code_column(meta, 3);
            (c[0], c[1], c[2])
        } else {
            (
                VSel::Simple(meta.selector()),
                VSel::Simple(meta.selector()),
                VSel::Simple(meta.selector()),
            )
        };
        let mds = <F as PoseidonField>::MDS;
        let cst = |v: F| Expression::Constant(v);
        let p5 = |x: Expression<F>| x.clone() * x.clone().square().square();
        for (name, q, full) in [
            ("hash full round", q_full, true),
            ("hash partial round", q_part, false),
        ] {
            meta.create_gate(name, |m| {
                let q = m.query_selector(q);
                let s: [Expression<F>; WIDTH] =
                    std::array::from_fn(|i| m.query_advice(a[i], Rotation::cur()));
                let nx: [Expression<F>; WIDTH] =
                    std::array::from_fn(|i| m.query_advice(a[i], Rotation::next()));
                let c: [Expression<F>; WIDTH] =
                    std::array::from_fn(|i| m.query_fixed(rc[i], Rotation::cur()));
                let u = if full {
                    s.map(p5)
                } else {
                    [s[0].clone(), s[1].clone(), p5(s[2].clone())]
                };
                crate::gate(
                    (0..WIDTH)
                        .map(|i| {
                            let lin = (0..WIDTH)
                                .fold(c[i].clone(), |acc, j| acc + cst(mds[i][j]) * u[j].clone());
                            q.clone() * (nx[i].clone() - lin)
                        })
                        .collect(),
                )
            });
        }
        // Absorb gates: (selector, keep previous state, absorb in1).
        for (name, q, chain, two) in [
            ("hash absorb first", q_first, false, true),
            ("hash absorb chain", q_chain, true, true),
            ("hash absorb tail", q_tail, true, false),
        ] {
            let q_prov = bus.q_prov;
            meta.create_gate(name, move |m| {
                let q = q.expr(m);
                let input = [
                    m.query_advice(a[2], Rotation::cur()),
                    if two {
                        m.query_advice(a[3], Rotation::cur())
                    } else {
                        cst(F::ZERO)
                    },
                    cst(F::ZERO),
                ];
                let mut out: Vec<Expression<F>> = (0..WIDTH)
                    .map(|i| {
                        let nx = m.query_advice(a[i], Rotation::next());
                        let prev = if chain {
                            m.query_advice(a[i], Rotation::prev())
                        } else {
                            cst(F::ZERO)
                        };
                        let c = m.query_fixed(rc[i], Rotation::cur());
                        q.clone() * (nx - prev - input[i].clone() - c)
                    })
                    .collect();
                if !chain {
                    let qp = m.query_fixed(q_prov, Rotation::cur());
                    let chip = m.query_advice(a[0], Rotation::cur());
                    let key = m.query_advice(a[1], Rotation::cur());
                    out.push(q.clone() * qp.clone() * (chip - cst(F::from(CHIP_HASH))));
                    out.push(q * qp * (key - cst(F::from(BUS_KEY))));
                }
                crate::gate(out)
            });
        }
        for i in [0, 2, 3, 4] {
            meta.enable_equality(a[i]);
        }
        Self {
            bus,
            rc,
            q_full,
            q_part,
            q_first,
            q_chain,
            q_tail,
        }
    }

    /// Hash `inputs` from row `offset` of `region`. With `provide`, the first
    /// absorb row is a bus provider row `(HASH, 2, in0, in1, out)` (two inputs
    /// only). Returns the output cell and the rows used. `perm` counts
    /// permutations for [`Tamper`].
    #[allow(clippy::too_many_arguments)]
    pub fn sponge(
        &self,
        region: &mut Region<'_, F>,
        offset: usize,
        inputs: &[HIn],
        provide: bool,
        tamper: &Tamper,
        perm: &mut usize,
    ) -> Result<(AssignedCell<F, F>, usize), Error> {
        let n = inputs.len();
        // A 1-input sponge would leave `in1` of the first absorb row free (the
        // first-absorb gate adds it), so it is not supported.
        assert!(n >= 2, "the chip hashes 2 or more inputs");
        assert!(!provide || n == 2, "a provider row serves 2-input hashes");
        let a = self.bus.cols.a;
        let zero = || Value::known(F::ZERO);
        let mut st: Value<[F; WIDTH]> = Value::known([F::ZERO, F::ZERO, F::from(n as u64)]);
        let mut o = offset;
        let mut out_slot = None;
        let mut last = None;
        for (ci, chunk) in inputs.chunks(2).enumerate() {
            let p = *perm;
            *perm += 1;
            let rc_t = |row: usize, i: usize| match tamper.rc {
                Some((tp, tr, tc, d)) if (tp, tr, tc) == (p, row, i) => d,
                _ => F::ZERO,
            };
            let st_t = |row: usize, s: Value<[F; WIDTH]>| match tamper.state {
                Some((tp, tr, tc, d)) if (tp, tr) == (p, row) => s.map(|mut s| {
                    s[tc] += d;
                    s
                }),
                _ => s,
            };
            // Absorb row.
            let sel = if ci == 0 {
                self.q_first
            } else if chunk.len() == 2 {
                self.q_chain
            } else {
                self.q_tail
            };
            sel.enable(region, o)?;
            let mut c0 = rc0();
            if ci == 0 {
                c0[2] += F::from(n as u64);
            }
            for i in 0..WIDTH {
                region.assign_fixed(
                    || "hash rc",
                    self.rc[i],
                    o,
                    || Value::known(c0[i] + rc_t(0, i)),
                )?;
            }
            if provide {
                region.assign_fixed(|| "q_prov", self.bus.q_prov, o, || Value::known(F::ONE))?;
                let chip = tamper.chip.unwrap_or(F::from(CHIP_HASH));
                region.assign_advice(|| "hash chip", a[0], o, || Value::known(chip))?;
                let key = tamper.key.unwrap_or(F::from(BUS_KEY));
                region.assign_advice(|| "hash key", a[1], o, || Value::known(key))?;
            } else {
                region.assign_fixed(|| "q_prov", self.bus.q_prov, o, zero)?;
                region.assign_advice(|| "hash chip", a[0], o, zero)?;
                region.assign_advice(|| "hash key", a[1], o, zero)?;
            }
            for (k, col) in [a[2], a[3]].into_iter().enumerate() {
                match chunk.get(k) {
                    Some(HIn::Cell(c)) => {
                        c.copy_advice(|| "hash in", region, col, o)?;
                    }
                    Some(HIn::CellAs(c, v)) => {
                        c.copy_advice(|| "hash in", region, col, o)?;
                        region.assign_advice(|| "hash in forged", col, o, || *v)?;
                    }
                    Some(HIn::Val(v)) => {
                        region.assign_advice(|| "hash in", col, o, || *v)?;
                    }
                    None => {
                        region.assign_advice(|| "hash in pad", col, o, zero)?;
                    }
                }
            }
            if ci == 0 && provide {
                out_slot = Some(o);
            } else {
                region.assign_advice(|| "hash out", a[4], o, zero)?;
            }
            let skip = tamper.skip_absorb == Some(p);
            let ins: Vec<Value<F>> = chunk.iter().map(HIn::value).collect();
            let add = Value::<Vec<F>>::from_iter(ins);
            st = st.zip(add).map(|(mut s, add)| {
                if !skip {
                    for (i, x) in add.iter().enumerate() {
                        s[i] += x;
                    }
                }
                for (i, c) in rc0().iter().enumerate() {
                    s[i] += c;
                }
                s
            });
            o += 1;
            // Round rows and the output row.
            for row in 1..ROWS_PER_PERM {
                st = st_t(row, st);
                let cells = (0..WIDTH)
                    .map(|i| region.assign_advice(|| "hash state", a[i], o, || st.map(|s| s[i])))
                    .collect::<Result<Vec<_>, _>>()?;
                if row < ROWS_PER_PERM - 1 {
                    let r = row - 1;
                    if is_full(r) {
                        self.q_full.enable(region, o)?;
                    } else {
                        self.q_part.enable(region, o)?;
                    }
                    let c = rc_round(r);
                    for i in 0..WIDTH {
                        region.assign_fixed(
                            || "hash rc",
                            self.rc[i],
                            o,
                            || Value::known(c[i] + rc_t(row, i)),
                        )?;
                    }
                    st = st.map(|s| round_native(r, s));
                } else {
                    last = Some(cells[0].clone());
                }
                o += 1;
            }
            // Next chunk: `st` carries over (q_chain / q_tail read the output row).
            // After the output row the state is the permutation output; the next
            // absorb adds to it.
        }
        let out = last.expect("one permutation");
        if let Some(row) = out_slot {
            let d = tamper.out.unwrap_or(F::ZERO);
            let c =
                region.assign_advice(|| "hash out", a[4], row, || out.value().map(|v| *v + d))?;
            region.constrain_equal(c.cell(), out.cell())?;
        }
        Ok((out, o - offset))
    }
}

#[cfg(test)]
mod tests {
    //! The chip alone with the bus: differential tests against the stdlib
    //! Poseidon on random inputs, and MockProver negatives.
    use midnight_proofs::{
        circuit::{Layouter, SimpleFloorPlanner},
        dev::MockProver,
        plonk::{Circuit, Instance},
    };
    use rand_chacha::ChaCha20Rng;
    use rand_chacha::rand_core::SeedableRng;

    use super::*;
    use crate::{
        Cols,
        bus::{Tuple, tuple},
        hashchip::hash_native,
    };

    #[derive(Clone, Debug)]
    struct HashOnly {
        /// In-region sponges; their outputs are checked against the instance.
        sponges: Vec<Vec<F>>,
        /// Bus-served 2-input hashes `(in0, in1)`.
        served: Vec<(F, F)>,
        /// Request rows (raw tuples).
        requests: Vec<Tuple>,
        tamper: Tamper,
        /// One absorb code column (E8) instead of three selectors.
        one: bool,
    }

    #[derive(Clone, Debug)]
    struct Cfg {
        h: PoseidonCfg,
        inst: Column<Instance>,
    }

    impl Circuit<F> for HashOnly {
        type Config = Cfg;
        type FloorPlanner = SimpleFloorPlanner;
        type Params = bool;
        fn without_witnesses(&self) -> Self {
            self.clone()
        }
        fn params(&self) -> bool {
            self.one
        }
        fn configure(_meta: &mut ConstraintSystem<F>) -> Cfg {
            unreachable!("configured with params")
        }
        fn configure_with_params(meta: &mut ConstraintSystem<F>, one: bool) -> Cfg {
            let cols = Cols::configure(meta);
            let bus = BusConfig::configure(meta, cols);
            let h = PoseidonCfg::configure(meta, bus, one);
            let inst = meta.instance_column();
            meta.enable_equality(inst);
            Cfg { h, inst }
        }
        fn synthesize(&self, cfg: Cfg, mut l: impl Layouter<F>) -> Result<(), Error> {
            let outs = l.assign_region(
                || "hash",
                |mut r| {
                    let mut o = 0;
                    let mut perm = 0;
                    let mut outs = Vec::new();
                    for (x, y) in &self.served {
                        let ins = [HIn::Val(Value::known(*x)), HIn::Val(Value::known(*y))];
                        let (_, n) =
                            cfg.h
                                .sponge(&mut r, o, &ins, true, &self.tamper, &mut perm)?;
                        o += n;
                    }
                    for s in &self.sponges {
                        // Inputs as cells assigned elsewhere (copied in).
                        let mut ins = Vec::new();
                        for (i, v) in s.iter().enumerate() {
                            let c = r.assign_advice(
                                || "src",
                                cfg.h.bus.cols.a[2 + i % 2],
                                o + i / 2,
                                || Value::known(*v),
                            )?;
                            ins.push(HIn::Cell(c));
                        }
                        o += s.len().div_ceil(2);
                        let (out, n) =
                            cfg.h
                                .sponge(&mut r, o, &ins, false, &self.tamper, &mut perm)?;
                        outs.push(out);
                        o += n;
                    }
                    for t in &self.requests {
                        cfg.h.bus.request(&mut r, o, Value::known(*t))?;
                        o += 1;
                    }
                    Ok(outs)
                },
            )?;
            for (i, c) in outs.iter().enumerate() {
                l.constrain_instance(c.cell(), cfg.inst, i)?;
            }
            Ok(())
        }
    }

    /// Both absorb modes must give the same verdict; the error text of the
    /// one-column mode (the final one) is returned.
    fn run(c: &HashOnly) -> Result<(), String> {
        let mut three = c.clone();
        three.one = false;
        let r3 = run1(&three);
        let mut one = c.clone();
        one.one = true;
        let r1 = run1(&one);
        assert_eq!(r1.is_ok(), r3.is_ok(), "modes disagree: {r1:?} / {r3:?}");
        if let (Err(e1), Err(e3)) = (&r1, &r3) {
            // The same gates fail in both modes.
            for g in [
                "absorb first",
                "absorb chain",
                "absorb tail",
                "full round",
                "partial round",
            ] {
                assert_eq!(e1.contains(g), e3.contains(g), "{g}: {e1} / {e3}");
            }
        }
        r1
    }

    fn run1(c: &HashOnly) -> Result<(), String> {
        let inst: Vec<F> = c.sponges.iter().map(|s| hash_native(s)).collect();
        let p = MockProver::run(c, vec![inst]).map_err(|e| format!("{e:?}"))?;
        p.verify().map_err(|e| {
            e.iter()
                .take(6)
                .map(|f| format!("{f}"))
                .collect::<Vec<_>>()
                .join(" | ")
        })
    }

    fn rng() -> ChaCha20Rng {
        ChaCha20Rng::seed_from_u64(0x5e7)
    }

    fn rand_vec(rng: &mut ChaCha20Rng, n: usize) -> Vec<F> {
        (0..n).map(|_| F::random(&mut *rng)).collect()
    }

    fn served_case(rng: &mut ChaCha20Rng, n: usize) -> HashOnly {
        let served: Vec<(F, F)> = (0..n)
            .map(|_| (F::random(&mut *rng), F::random(&mut *rng)))
            .collect();
        let requests = served
            .iter()
            .map(|(x, y)| tuple(CHIP_HASH, F::from(BUS_KEY), *x, *y, hash_native(&[*x, *y])))
            .collect();
        HashOnly {
            sponges: vec![],
            served,
            requests,
            tamper: Tamper::default(),
            one: true,
        }
    }

    /// The CPU form of this chip equals the stdlib Poseidon sponge on random
    /// inputs of every length 1..=40.
    #[test]
    fn differential_cpu() {
        let mut g = rng();
        for n in 1..=40 {
            for _ in 0..4 {
                let v = rand_vec(&mut g, n);
                assert_eq!(sponge_native(&v), hash_native(&v), "n = {n}");
            }
        }
    }

    /// In-circuit: random sponges of 2, 3, 4, 8 and 31 inputs equal the stdlib
    /// hash (the instance holds `hash_native`), and random bus-served 2-input
    /// hashes serve requests carrying the stdlib hash.
    #[test]
    fn differential_circuit() {
        let mut g = rng();
        let mut c = served_case(&mut g, 4);
        c.sponges = [2usize, 3, 4, 8, 31]
            .iter()
            .map(|&n| rand_vec(&mut g, n))
            .collect();
        run(&c).unwrap();
    }

    #[test]
    fn wrong_served_output_rejected() {
        let mut g = rng();
        let mut c = served_case(&mut g, 2);
        c.requests[1][4] += F::ONE;
        let e = run(&c).unwrap_err();
        assert!(e.contains("Lookup") || e.contains("lookup"), "{e}");
    }

    #[test]
    fn wrong_sponge_output_rejected() {
        let mut g = rng();
        let mut c = served_case(&mut g, 0);
        c.sponges = vec![rand_vec(&mut g, 3)];
        let inst = vec![hash_native(&c.sponges[0]) + F::ONE];
        for one in [false, true] {
            c.one = one;
            let p = MockProver::run(&c, vec![inst.clone()]).unwrap();
            assert!(p.verify().is_err());
        }
    }

    fn sponge_case(g: &mut ChaCha20Rng, n: usize, tamper: Tamper) -> HashOnly {
        HashOnly {
            sponges: vec![rand_vec(g, n)],
            served: vec![],
            requests: vec![],
            tamper,
            one: true,
        }
    }

    /// One tampered state cell, rounds recomputed from it: exactly the round gate
    /// that produced it fails (full and partial rounds, each column, output row).
    #[test]
    fn tampered_state_rejected() {
        let mut g = rng();
        for (row, gate) in [
            (2, "hash full round"),
            (5, "hash full round"),
            (6, "hash partial round"),
            (40, "hash partial round"),
            (65, "hash partial round"),
            (66, "hash full round"),
            (69, "hash full round"),
        ] {
            for col in 0..WIDTH {
                let t = Tamper {
                    state: Some((0, row, col, F::ONE)),
                    ..Default::default()
                };
                let e = run(&sponge_case(&mut g, 2, t)).unwrap_err();
                assert!(e.contains(gate), "row {row} col {col}: {e}");
                assert!(!e.contains("absorb"), "row {row} col {col}: {e}");
            }
        }
        // Second permutation of a 3-input sponge.
        let t = Tamper {
            state: Some((1, 30, 2, F::ONE)),
            ..Default::default()
        };
        let e = run(&sponge_case(&mut g, 3, t)).unwrap_err();
        assert!(e.contains("hash partial round"), "{e}");
        // The first state after absorb is checked by the absorb gate.
        let t = Tamper {
            state: Some((1, 1, 0, F::ONE)),
            ..Default::default()
        };
        let e = run(&sponge_case(&mut g, 4, t)).unwrap_err();
        assert!(e.contains("hash absorb chain"), "{e}");
    }

    /// A wrong round constant in any round (witness computed with the right
    /// one) is refused by that round's gate; on the absorb row (capacity, RC[0])
    /// by the absorb gate.
    #[test]
    fn wrong_round_constant_rejected() {
        let mut g = rng();
        for (row, gate) in [
            (1, "hash full round"),
            (4, "hash full round"),
            (5, "hash partial round"),
            (64, "hash partial round"),
            (65, "hash full round"),
            (68, "hash full round"),
            (0, "hash absorb first"),
        ] {
            for col in 0..WIDTH {
                let t = Tamper {
                    rc: Some((0, row, col, F::ONE)),
                    ..Default::default()
                };
                let e = run(&sponge_case(&mut g, 2, t)).unwrap_err();
                assert!(e.contains(gate), "row {row} col {col}: {e}");
            }
        }
    }

    /// An absorb row that does not add its inputs: first, chain and tail.
    #[test]
    fn skipped_absorb_rejected() {
        let mut g = rng();
        for (n, perm, gate) in [
            (2, 0, "hash absorb first"),
            (4, 1, "hash absorb chain"),
            (3, 1, "hash absorb tail"),
            (31, 9, "hash absorb chain"),
            (31, 15, "hash absorb tail"),
        ] {
            let t = Tamper {
                skip_absorb: Some(perm),
                ..Default::default()
            };
            let e = run(&sponge_case(&mut g, n, t)).unwrap_err();
            assert!(e.contains(gate), "n {n} perm {perm}: {e}");
        }
    }

    /// A provider row must carry `chip = HASH` and `key = 2`.
    #[test]
    fn provider_key_is_pinned() {
        let mut g = rng();
        let mut c = served_case(&mut g, 1);
        c.tamper.key = Some(F::from(5u64));
        c.requests[0][1] = F::from(5u64);
        let e = run(&c).unwrap_err();
        assert!(e.contains("hash absorb first"), "{e}");
    }

    /// Gemini Quint verdict H-C4: a provider row must carry `chip = HASH`
    /// (a request with the forged chip id would otherwise be served), and its
    /// `out` is the copy of the output row's element 0.
    #[test]
    fn provider_chip_is_pinned_and_out_is_copied() {
        let mut g = rng();
        let mut c = served_case(&mut g, 1);
        c.tamper.chip = Some(F::from(crate::bus::CHIP_ALU));
        c.requests[0][0] = F::from(crate::bus::CHIP_ALU);
        let e = run(&c).unwrap_err();
        assert!(e.contains("hash absorb first"), "{e}");
        let mut c = served_case(&mut g, 1);
        c.tamper.out = Some(F::ONE);
        c.requests[0][4] += F::ONE;
        let e = run(&c).unwrap_err();
        assert!(e.contains("quality") || e.contains("ermutation"), "{e}");
        assert!(!e.contains("ookup") && !e.contains("gate"), "{e}");
    }

    /// A request for a hash nobody provides (inputs swapped) is refused.
    #[test]
    fn swapped_inputs_rejected() {
        let mut g = rng();
        let mut c = served_case(&mut g, 1);
        let (x, y) = c.served[0];
        c.requests[0] = tuple(CHIP_HASH, F::from(BUS_KEY), y, x, hash_native(&[x, y]));
        assert!(run(&c).is_err());
    }
}
