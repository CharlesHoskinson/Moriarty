//! C1 harness: MEM window + dense backend + a CORE stand-in (append and read
//! ports), with the boundary `(len, root, tail_d)` of each arena as instance
//! values. Positive cases, named negative mutants, and the row measurement.
//!
//! The CORE stand-in is not part of MEM: an append port requests
//! `(MEM, 8 addr + kind, a, b, c)` at `addr = len_pre + j` (a chain of increment
//! rows ending at `len_post`); a read port checks `addr < len_cur` and routes the
//! read to the window (bus request) when `addr >= base` or to the backend (cold
//! read) when `addr < base`. The expected kind of a port is the fixed `kfix`
//! (the rule's constant in a real leaf).
use ff::Field;
use midnight_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    dev::{MockProver, cost_model::circuit_model},
    plonk::{Circuit, Column, ConstraintSystem, Error, Expression, Instance, VirtualCells},
    poly::Rotation,
};

use super::*;
use crate::{
    bus::{BusConfig, CHIP_MEM},
    membackend::{
        ColdRead, KIND_DATA, KIND_ENV, KIND_FIELD, KIND_SLOT, MemBackend, Names, Src, WinW,
        WindowCfg, bus_key, fr, log_rows, overwrite, rows_begin, rows_last, val,
    },
};

#[derive(Clone, Debug)]
struct ReadCase {
    addr: F,
    /// Expected kind (the port's `kfix`).
    kind: u64,
    /// Record data words as the reader claims them.
    d: [F; 3],
    /// `len_cur` is `len_post` (else `len_pre`).
    at_post: bool,
    cold: bool,
    sibs: Vec<F>,
}

#[derive(Clone, Debug)]
struct ArenaCase {
    kfix: u64,
    pre: State,
    post: State,
    win: WinW,
    leaf: DenseLeafW,
    appends: Vec<Rec>,
    reads: Vec<ReadCase>,
}

#[derive(Clone, Debug)]
struct Leaf {
    top: usize,
    cases: Vec<ArenaCase>,
    /// Named cells overwritten after synthesis.
    muts: Vec<(String, F)>,
}

#[derive(Clone, Debug)]
struct Cfg {
    pc: PoseidonCfg,
    win: WindowCfg,
    dense: DenseCfg,
    /// `append`, `append request`, `read`, `read request`.
    h: [VSel; 4],
    inst: Column<Instance>,
}

const AP: usize = 0;
const AQ: usize = 1;
const RD: usize = 2;
const RQ: usize = 3;

fn kk(v: u64) -> Expression<F> {
    Expression::Constant(F::from(v))
}

impl Circuit<F> for Leaf {
    type Config = Cfg;
    type FloorPlanner = SimpleFloorPlanner;
    type Params = ();

    fn without_witnesses(&self) -> Self {
        self.clone()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Cfg {
        let cols = crate::Cols::configure(meta);
        let bus = BusConfig::configure(meta, cols);
        let pc = PoseidonCfg::configure(meta, bus, true);
        let win = WindowCfg::configure(meta, bus);
        let dense = DenseCfg::configure(meta, cols);
        let inst = meta.instance_column();
        meta.enable_equality(inst);
        let hv = pack(meta, &[1, 1, 2, 2]);
        let h: [VSel; 4] = std::array::from_fn(|i| hv[i]);
        let a = cols.a;
        let kfix = win.kfix;
        let cur = |m: &mut VirtualCells<'_, F>, i: usize| m.query_advice(a[i], Rotation::cur());
        let prv = |m: &mut VirtualCells<'_, F>, i: usize| m.query_advice(a[i], Rotation::prev());
        meta.create_gate("core append", |m| {
            let s = h[AP].expr(m);
            let kf = m.query_fixed(kfix, Rotation::cur());
            let (addr, kind, key) = (cur(m, 5), cur(m, 6), cur(m, 7));
            crate::gate(vec![
                s.clone() * (key - addr * kk(8) - kind.clone()),
                s * (kind - kf),
            ])
        });
        meta.create_gate("core append request", |m| {
            let s = h[AQ].expr(m);
            let mut v = vec![s.clone() * (cur(m, 0) - kk(CHIP_MEM))];
            for (i, p) in [(1, 7), (2, 8), (3, 9), (4, 10)] {
                v.push(s.clone() * (cur(m, i) - prv(m, p)));
            }
            crate::gate(v)
        });
        meta.create_gate("core read", |m| {
            let s = h[RD].expr(m);
            let kf = m.query_fixed(kfix, Rotation::cur());
            let (addr, lc, base, cold, diff, d2, kind, key) = (
                cur(m, 3),
                cur(m, 4),
                cur(m, 5),
                cur(m, 6),
                cur(m, 7),
                cur(m, 8),
                cur(m, 9),
                cur(m, 10),
            );
            crate::gate(vec![
                s.clone() * cold.clone() * (kk(1) - cold.clone()),
                s.clone()
                    * (diff
                        - cold.clone() * (base.clone() - kk(1) - addr.clone())
                        - (kk(1) - cold) * (addr.clone() - base)),
                s.clone() * (d2 - (lc - kk(1) - addr.clone())),
                s.clone() * (key - addr * kk(8) - kind.clone()),
                s * (kind - kf),
            ])
        });
        meta.create_gate("core read request", |m| {
            let s = h[RQ].expr(m);
            let warm = kk(1) - prv(m, 6);
            let mut v = vec![s.clone() * (cur(m, 0) - warm.clone() * kk(CHIP_MEM))];
            for (i, p) in [(1, 10), (2, 0), (3, 1), (4, 2)] {
                v.push(s.clone() * (cur(m, i) - warm.clone() * prv(m, p)));
            }
            crate::gate(v)
        });
        Cfg {
            pc,
            win,
            dense,
            h,
            inst,
        }
    }

    fn synthesize(&self, cfg: Cfg, mut l: impl Layouter<F>) -> Result<(), Error> {
        let a = cfg.win.cols.a;
        let nb = self.top + 3;
        let io = l.assign_region(
            || "c1",
            |mut r| {
                rows_begin();
                let r = &mut r;
                let mut o = 0usize;
                let mut names: Names = Vec::new();
                let mut io = Vec::new();
                for (ai, c) in self.cases.iter().enumerate() {
                    let tag = format!("a{ai}");
                    let mut mark = o;
                    let lap = |o: usize, mark: &mut usize, label: &str| {
                        log_rows(label, o - *mark);
                        *mark = o;
                    };
                    let wnd = cfg.win.window(
                        r,
                        &mut o,
                        &Src::known(c.pre.len),
                        &Src::known(c.post.len),
                        self.top,
                        c.kfix,
                        &c.win,
                        &mut names,
                        &tag,
                    )?;
                    let (vpre, vpost) = (wnd.len_pre.clone(), wnd.len_post.clone());
                    lap(o, &mut mark, "mem.frontier_and_window");
                    // CORE stand-in: appends at len_pre, len_pre + 1, ...
                    let mut addr = vpre.clone();
                    for rec in &c.appends {
                        let row = o;
                        cfg.h[AP].enable(r, row)?;
                        r.assign_fixed(|| "kfix", cfg.win.kfix, row, || Value::known(rec[0]))?;
                        addr.copy_advice(|| "addr", r, a[5], row)?;
                        let key = bus_key(val(&addr), rec[0]);
                        r.assign_advice(|| "kind", a[6], row, || Value::known(rec[0]))?;
                        r.assign_advice(|| "key", a[7], row, || Value::known(key))?;
                        for i in 0..3 {
                            r.assign_advice(|| "d", a[8 + i], row, || Value::known(rec[1 + i]))?;
                        }
                        cfg.h[AQ].enable(r, row + 1)?;
                        let t = [F::from(CHIP_MEM), key, rec[1], rec[2], rec[3]];
                        cfg.win.bus.request(r, row + 1, Value::known(t))?;
                        o += 2;
                        addr = cfg.win.inc(r, &mut o, &addr)?;
                    }
                    r.constrain_equal(addr.cell(), vpost.cell())?;
                    lap(o, &mut mark, "core.appends");
                    // Boundary cells of the backend.
                    let rp =
                        r.assign_advice(|| "root_pre", a[5], o, || Value::known(c.pre.root))?;
                    let tp =
                        r.assign_advice(|| "tail_pre", a[6], o, || Value::known(c.pre.tail))?;
                    o += 1;
                    let post = cfg.dense.persist(
                        &cfg.pc,
                        r,
                        &mut o,
                        &wnd,
                        &[rp.clone(), tp.clone()],
                        &c.leaf,
                        &mut names,
                    )?;
                    lap(o, &mut mark, "backend.persist");
                    for (ri, rd) in c.reads.iter().enumerate() {
                        let row = o;
                        cfg.h[RD].enable(r, row)?;
                        r.assign_fixed(
                            || "kfix",
                            cfg.win.kfix,
                            row,
                            || Value::known(F::from(rd.kind)),
                        )?;
                        let lc = if rd.at_post { &vpost } else { &vpre };
                        let base = val(&wnd.base);
                        let lcv = val(lc);
                        let coldf = F::from(rd.cold as u64);
                        let diff = if rd.cold {
                            base - F::ONE - rd.addr
                        } else {
                            rd.addr - base
                        };
                        let kind = F::from(rd.kind);
                        let key = bus_key(rd.addr, kind);
                        let dc: Vec<Cell> = (0..3)
                            .map(|i| r.assign_advice(|| "d", a[i], row, || Value::known(rd.d[i])))
                            .collect::<Result<_, _>>()?;
                        let ad = r.assign_advice(|| "addr", a[3], row, || Value::known(rd.addr))?;
                        lc.copy_advice(|| "len_cur", r, a[4], row)?;
                        wnd.base.copy_advice(|| "base", r, a[5], row)?;
                        let cc = r.assign_advice(|| "cold", a[6], row, || Value::known(coldf))?;
                        let dfc = r.assign_advice(|| "diff", a[7], row, || Value::known(diff))?;
                        let d2c = r.assign_advice(
                            || "d2",
                            a[8],
                            row,
                            || Value::known(lcv - F::ONE - rd.addr),
                        )?;
                        let kc = r.assign_advice(|| "kind", a[9], row, || Value::known(kind))?;
                        r.assign_advice(|| "key", a[10], row, || Value::known(key))?;
                        cfg.h[RQ].enable(r, row + 1)?;
                        let t = if rd.cold {
                            [F::ZERO; 5]
                        } else {
                            [F::from(CHIP_MEM), key, rd.d[0], rd.d[1], rd.d[2]]
                        };
                        cfg.win.bus.request(r, row + 1, Value::known(t))?;
                        o += 2;
                        cfg.win.bits(r, &mut o, &Src::Cell(dfc), nb)?;
                        cfg.win.bits(r, &mut o, &Src::Cell(d2c), nb)?;
                        lap(o, &mut mark, "core.read_port");
                        let cr = ColdRead {
                            enable: cc,
                            addr: ad,
                            rec: [kc, dc[0].clone(), dc[1].clone(), dc[2].clone()],
                        };
                        let rw = DenseReadW {
                            top: self.top,
                            sibs: rd.sibs.clone(),
                        };
                        cfg.dense.cold_read(
                            &cfg.pc,
                            r,
                            &mut o,
                            &[rp.clone(), tp.clone()],
                            &cr,
                            &rw,
                            &mut names,
                        )?;
                        names.push((format!("{tag}.read{ri}"), cr.enable.clone()));
                        mark = o;
                    }
                    io.extend([vpre, rp, tp, vpost, post[0].clone(), post[1].clone()]);
                }
                for (nm, v) in &self.muts {
                    let c = names
                        .iter()
                        .find(|(n, _)| n == nm)
                        .unwrap_or_else(|| panic!("no cell named {nm}"));
                    overwrite(r, &c.1, *v)?;
                }
                log_rows("total", o);
                Ok(io)
            },
        )?;
        for (i, c) in io.iter().enumerate() {
            l.constrain_instance(c.cell(), cfg.inst, i)?;
        }
        Ok(())
    }
}

impl Leaf {
    fn instance(&self) -> Vec<F> {
        self.cases
            .iter()
            .flat_map(|c| {
                [
                    c.pre.len,
                    c.pre.root,
                    c.pre.tail,
                    c.post.len,
                    c.post.root,
                    c.post.tail,
                ]
            })
            .collect()
    }
}

fn run(c: &Leaf) -> Result<(), String> {
    let p = MockProver::run(c, vec![c.instance()]).map_err(|e| format!("synth: {e:?}"))?;
    p.verify().map_err(|e| {
        e.iter()
            .take(6)
            .map(|f| format!("{f}"))
            .collect::<Vec<_>>()
            .join(" | ")
    })
}

fn rec(kind: u64, x: u64) -> Rec {
    [F::from(kind), F::from(x), F::from(x + 1000), F::from(7u64)]
}

/// A read of `addr` against the model before the leaf (`pre`), which appended
/// `appends`.
fn read_of(pre: &DenseModel, appends: &[Rec], addr: u128, at_post: bool) -> ReadCase {
    let base = pre.len / 8 * 8;
    let (r, cold, sibs) = if addr < base {
        let seg = pre.segs.get(&(addr >> 3)).expect("flushed");
        (seg[(addr & 7) as usize], true, pre.sibs_cold(addr))
    } else if addr < pre.len {
        (
            pre.tail[(addr - base) as usize],
            false,
            vec![F::ZERO; pre.top + 3],
        )
    } else {
        let r = appends
            .get((addr - pre.len) as usize)
            .copied()
            .unwrap_or([F::ZERO; 4]);
        (r, false, vec![F::ZERO; pre.top + 3])
    };
    let mut kind = 0;
    for k in 1..=4u64 {
        if r[0] == F::from(k) {
            kind = k;
        }
    }
    ReadCase {
        addr: fr(addr),
        kind: kind.max(1),
        d: [r[1], r[2], r[3]],
        at_post,
        cold,
        sibs,
    }
}

/// One leaf of `m` appending `appends` with `w` flush units and reads of
/// `reads` (address, `len_cur` is post); `m` advances.
fn case(m: &mut DenseModel, appends: &[Rec], w: usize, reads: &[(u128, bool)]) -> ArenaCase {
    let pre_m = m.clone();
    let pre = m.state();
    let (win, leaf) = m.leaf(appends, w);
    let post = m.state();
    ArenaCase {
        kfix: 0,
        pre,
        post,
        win,
        leaf,
        appends: appends.to_vec(),
        reads: reads
            .iter()
            .map(|&(a, p)| read_of(&pre_m, appends, a, p))
            .collect(),
    }
}

fn leaf1(top: usize, c: ArenaCase) -> Leaf {
    Leaf {
        top,
        cases: vec![c],
        muts: vec![],
    }
}

/// A model with `n` records appended natively.
fn model(top: usize, n: u64) -> DenseModel {
    let mut m = DenseModel::new(top);
    for i in 0..n {
        m.append(rec(1 + i % 4, i));
    }
    m
}

fn recs(from: u64, n: u64) -> Vec<Rec> {
    (from..from + n).map(|i| rec(1 + i % 4, i)).collect()
}

const T: usize = 8;

fn expect_err(c: &Leaf, needle: &str) {
    let e = run(c).expect_err("mutant accepted");
    assert!(e.contains(needle), "expected {needle}: {e}");
}

// ---------------------------------------------------------------- positives

#[test]
fn c1_pass_through_and_every_tail_occupancy() {
    for r0 in 0..8u64 {
        // No append: the boundary passes through unchanged.
        let mut m = model(T, 16 + r0);
        let c = case(&mut m, &[], 1, &[]);
        assert_eq!(c.pre, c.post, "pass-through");
        run(&leaf1(T, c)).unwrap();
        // One append (r0 = 7 flushes).
        let c = case(&mut m, &recs(100, 1), 1, &[]);
        assert_eq!(c.post.root != c.pre.root, r0 == 7);
        run(&leaf1(T, c)).unwrap();
    }
}

#[test]
fn c1_cross_boundary_reads_and_chain() {
    let mut m = model(T, 21);
    // Cold read of addr 3, tail read of 17, new-record read of 22, all after.
    let c = case(
        &mut m,
        &recs(200, 6),
        1,
        &[(3, true), (17, true), (22, true), (20, false)],
    );
    assert!(c.reads[0].cold && !c.reads[1].cold);
    run(&leaf1(T, c)).unwrap();
    // The next leaf imports the post tail (r = 3) and appends 8 more (flush).
    let c2 = case(&mut m, &recs(300, 8), 1, &[(26, true)]);
    run(&leaf1(T, c2)).unwrap();
}

#[test]
fn c1_two_flush_units() {
    let mut m = model(T, 14);
    let c = case(&mut m, &recs(400, 17), 2, &[(30, true)]);
    assert_eq!(m.len, 31);
    run(&leaf1(T, c)).unwrap();
}

/// The last segment with two flush units: neither unit flushes, and the index
/// chain keeps every walk index below 2^61 (TOP = 61, `len_pre = 2^64 - 2`).
#[test]
fn c1_last_segment_two_units() {
    let mut m = DenseModel::at_len(61, (1u128 << 64) - 8);
    for i in 0..6 {
        m.append(rec(1, i));
    }
    let c = case(&mut m, &recs(9, 1), 2, &[]);
    assert_eq!(m.len, (1u128 << 64) - 1);
    run(&leaf1(61, c)).unwrap();
}

/// SRC-023 at memory level: the boundary after a run of appends is the same for
/// any partition into leaves (native model, then each leaf in the circuit).
#[test]
fn c1_partition_independence() {
    let all = recs(0, 37);
    let mut finals = Vec::new();
    for parts in [
        vec![37usize],
        vec![8, 8, 8, 8, 5],
        vec![1, 7, 0, 9, 3, 8, 1, 8],
        vec![15, 15, 7],
    ] {
        let mut m = DenseModel::new(T);
        let mut at = 0;
        for &p in &parts {
            let w = (m.tail.len() + p) / 8 + 1;
            let c = case(&mut m, &all[at..at + p], w, &[]);
            if p <= 8 {
                run(&leaf1(T, c)).unwrap();
            }
            at += p;
        }
        finals.push(m.state());
    }
    assert!(finals.windows(2).all(|w| w[0] == w[1]));
}

#[test]
fn c1_top61_and_top63_positive() {
    for top in [61, 63] {
        let mut m = model(top, 13);
        let c = case(&mut m, &recs(500, 5), 1, &[(2, true), (14, true)]);
        run(&leaf1(top, c)).unwrap();
    }
}

#[test]
fn c1_counter_edge_2_64_minus_1() {
    let mut m = DenseModel::at_len(61, (1u128 << 64) - 8);
    for i in 0..6 {
        m.append(rec(1, i));
    }
    assert_eq!(m.len, (1u128 << 64) - 2);
    let c = case(&mut m, &recs(9, 1), 1, &[]);
    assert_eq!(m.len, (1u128 << 64) - 1);
    run(&leaf1(61, c)).unwrap();
}

#[test]
fn c1_four_arenas() {
    let mut cases = Vec::new();
    for kind in 1..=4u64 {
        let mut m = DenseModel::new(T);
        for i in 0..(5 + kind) {
            m.append(rec(kind, i));
        }
        let ap: Vec<Rec> = (0..4).map(|i| rec(kind, 50 + i)).collect();
        let mut c = case(&mut m, &ap, 1, &[(1, true)]);
        c.kfix = kind;
        cases.push(c);
    }
    run(&Leaf {
        top: T,
        cases,
        muts: vec![],
    })
    .unwrap();
}

// ---------------------------------------------------------------- mutants

/// forged_tail_across_leaf_boundary: leaf 2 imports a tail record that differs
/// from leaf 1's post tail.
#[test]
fn c1_mut_forged_tail_across_leaf_boundary() {
    let mut m = model(T, 16);
    let _ = case(&mut m, &recs(10, 3), 1, &[]);
    let mut c2 = case(&mut m, &recs(20, 2), 1, &[]);
    c2.win.rows[1].d[0] += F::ONE;
    // The post tail is recomputed from the forged rows, so the only broken link
    // is the tail opening against `pre.tail_d`.
    let post: Vec<Rec> = c2.win.rows[..5]
        .iter()
        .map(|w| [F::from(w.kind), w.d[0], w.d[1], w.d[2]])
        .collect();
    c2.post.tail = tail_d(&post);
    let e = run(&leaf1(T, c2)).expect_err("forged tail");
    assert!(e.contains("quality") && !e.contains("gate"), "{e}");
}

/// non_canonical_tail: the pre tail commits a non-pad record above `r`.
#[test]
fn c1_mut_non_canonical_tail() {
    let mut m = model(T, 19);
    let mut c = case(&mut m, &recs(10, 1), 1, &[]);
    let mut l: Vec<F> = (0..3)
        .map(|i| rec_d(&rec(1 + (16 + i) % 4, 16 + i)))
        .collect();
    l.extend((0..5).map(|i| if i == 2 { rec_d(&rec(1, 99)) } else { pad_d() }));
    c.pre.tail = tree8(&l);
    assert!(run(&leaf1(T, c)).is_err());
}

/// flush_skipped: the segment fills but the root is kept.
#[test]
fn c1_mut_flush_skipped() {
    let mut m = model(T, 22);
    let c = case(&mut m, &recs(10, 3), 1, &[]);
    let mut l = leaf1(T, c.clone());
    l.muts.push(("flush0.root".into(), c.pre.root));
    l.cases[0].post.root = c.pre.root;
    expect_err(&l, "dense flush");
}

/// flush_spurious: no boundary is crossed but the root changes.
#[test]
fn c1_mut_flush_spurious() {
    let mut m = model(T, 17);
    let c = case(&mut m, &recs(10, 2), 1, &[]);
    let mut l = leaf1(T, c);
    l.muts.push(("flush0.root".into(), F::from(5u64)));
    l.cases[0].post.root = F::from(5u64);
    expect_err(&l, "dense flush");
}

/// address_reuse_flush_into_occupied_slot: the pre root already holds a
/// segment at `q_pre`; the old walk from EMPTY_SEG cannot reach it.
#[test]
fn c1_mut_address_reuse_occupied_slot() {
    let mut m = model(T, 20);
    m.insert_seg(2, std::array::from_fn(|i| rec(1, 900 + i as u64)));
    let c = case(&mut m, &recs(10, 5), 1, &[]);
    expect_err(&leaf1(T, c), "dense flush");
}

/// empty_seg_forgery: as above, with the old walk started at the occupying
/// segment root (so it reaches the pre root).
#[test]
fn c1_mut_empty_seg_forgery() {
    let mut m = model(T, 20);
    let seg: [Rec; 8] = std::array::from_fn(|i| rec(1, 900 + i as u64));
    m.insert_seg(2, seg);
    let occ = tree8(&seg.iter().map(rec_d).collect::<Vec<_>>());
    let mut c = case(&mut m, &recs(10, 5), 1, &[]);
    c.leaf.units[0].x0 = Some(occ);
    expect_err(&leaf1(T, c), "dense walk empty");
}

/// TOP = 1 (one walk level): the honest leaf of the gap test below. Five
/// records, three appends, so segment 0 flushes.
#[test]
fn c1_top1_positive() {
    let mut m = model(1, 5);
    let c = case(&mut m, &recs(10, 3), 1, &[]);
    assert_ne!(c.post.root, c.pre.root);
    run(&leaf1(1, c)).unwrap();
}

/// gap_top1_empty_seg_forgery (Gemini Quint lane, REPORT-dense.md section 4):
/// `empty_seg_forgery` at TOP = 1, where the only level of the old walk is
/// also its last. Slot 0 is occupied; the old walk starts at the occupying
/// segment root and reaches the pre root.
#[test]
fn c1_gap_top1_empty_seg_forgery() {
    let mut m = model(1, 5);
    let seg: [Rec; 8] = std::array::from_fn(|i| rec(1, 900 + i as u64));
    m.insert_seg(0, seg);
    let occ = tree8(&seg.iter().map(rec_d).collect::<Vec<_>>());
    let mut c = case(&mut m, &recs(10, 3), 1, &[]);
    c.leaf.units[0].x0 = Some(occ);
    expect_err(&leaf1(1, c), "dense walk empty");
}

/// address_reuse_window: window row 1 claims row 0's address.
#[test]
fn c1_mut_address_reuse_window_row() {
    let mut m = model(T, 16);
    let c = case(&mut m, &recs(10, 3), 1, &[]);
    let mut l = leaf1(T, c);
    // The key follows the forged address, so the row gate holds.
    l.muts.push(("a0.win1.addr".into(), F::from(16u64)));
    l.muts
        .push(("a0.win1.key".into(), bus_key(F::from(16u64), F::from(4u64))));
    let e = run(&l).expect_err("address reuse");
    assert!(
        e.contains("mem win first") && !e.contains("mem win row"),
        "{e}"
    );
}

/// read_at_len: a read of `addr = len_cur`.
#[test]
fn c1_mut_read_at_len() {
    let mut m = model(T, 16);
    let c = case(&mut m, &recs(10, 3), 1, &[(19, true)]);
    expect_err(&leaf1(T, c), "mem bits last");
}

/// read_beyond_len: a read past `len_cur` (and past the active window).
#[test]
fn c1_mut_read_beyond_len() {
    let mut m = model(T, 16);
    let c = case(&mut m, &recs(10, 3), 1, &[(21, true)]);
    let e = run(&leaf1(T, c)).expect_err("beyond");
    assert!(e.contains("mem bits last") && e.contains("ookup"), "{e}");
}

/// future_record_read: a read at `len_cur = len_pre` of a record this leaf
/// appends later (present in the window).
#[test]
fn c1_mut_future_record_read() {
    let mut m = model(T, 16);
    let c = case(&mut m, &recs(10, 3), 1, &[(17, false)]);
    expect_err(&leaf1(T, c), "mem bits last");
}

/// wrong_kind_warm: a FIELD read of a window record of another kind.
#[test]
fn c1_mut_wrong_kind_warm() {
    let mut m = model(T, 16);
    let mut c = case(&mut m, &recs(12, 3), 1, &[(16, true)]);
    assert_eq!(c.reads[0].kind, KIND_DATA);
    c.reads[0].kind = KIND_FIELD;
    expect_err(&leaf1(T, c), "ookup");
}

/// wrong_kind_cold: an ENV read of an old record of another kind.
#[test]
fn c1_mut_wrong_kind_cold() {
    let mut m = model(T, 16);
    let mut c = case(&mut m, &recs(12, 1), 1, &[(5, true)]);
    assert!(c.reads[0].cold && c.reads[0].kind == KIND_FIELD);
    c.reads[0].kind = KIND_ENV;
    expect_err(&leaf1(T, c), "dense geq");
}

/// cold_value_forged: an old record read with a wrong data word.
#[test]
fn c1_mut_cold_value_forged() {
    let mut m = model(T, 16);
    let mut c = case(&mut m, &recs(12, 1), 1, &[(5, true)]);
    c.reads[0].d[1] += F::ONE;
    expect_err(&leaf1(T, c), "dense geq");
}

/// cold_route_forged: a window address routed to the old-memory path.
#[test]
fn c1_mut_cold_route_forged() {
    let mut m = model(T, 16);
    let mut c = case(&mut m, &recs(12, 3), 1, &[(17, true)]);
    c.reads[0].cold = true;
    expect_err(&leaf1(T, c), "mem bits last");
}

/// counter_wrap_2_64: TOP = 61, `len_pre = 2^64 - 1`, one append.
#[test]
fn c1_mut_counter_wrap_2_64() {
    let mut m = DenseModel::at_len(61, (1u128 << 64) - 8);
    for i in 0..7 {
        m.append(rec(1, i));
    }
    let c = case(&mut m, &recs(9, 1), 1, &[]);
    assert_eq!(m.len, 1u128 << 64);
    expect_err(&leaf1(61, c), "mem bits last");
}

/// counter_field_wrap: `len_pre = p - 1` (a field wrap) is not an integer below
/// 2^(TOP + 3).
#[test]
fn c1_mut_counter_field_wrap() {
    let mut m = model(T, 16);
    let c = case(&mut m, &[], 1, &[]);
    let mut l = leaf1(T, c);
    l.cases[0].pre.len = -F::ONE;
    l.cases[0].post.len = -F::ONE;
    assert!(run(&l).is_err());
}

/// pad_row_on_bus: an inactive window row offers `(MEM, key, ..)`.
#[test]
fn c1_mut_pad_row_on_bus() {
    let mut m = model(T, 16);
    let c = case(&mut m, &recs(12, 2), 1, &[]);
    let mut l = leaf1(T, c);
    l.muts.push(("a0.win5.chip".into(), F::from(CHIP_MEM)));
    expect_err(&l, "mem win row");
}

/// kind_out_of_range: a window row with kind 9 (would alias another address).
#[test]
fn c1_mut_kind_out_of_range() {
    let mut m = model(T, 16);
    let c = case(&mut m, &recs(12, 2), 1, &[]);
    let mut l = leaf1(T, c);
    // Key 8 * 16 + 9 = 8 * 17 + 1: the row would serve (17, DATA). The key is
    // consistent with the kind, so only the kind range constraint is broken.
    l.muts.push(("a0.win0.kind".into(), F::from(9u64)));
    l.muts
        .push(("a0.win0.key".into(), bus_key(F::from(16u64), F::from(9u64))));
    let e = run(&l).expect_err("kind range");
    assert!(
        e.contains("mem win row") && !e.contains("mem win first"),
        "{e}"
    );
}

/// arena_kind_pinned: in the four-arena variant a row of the DATA arena holds a
/// FIELD record.
#[test]
fn c1_mut_arena_kind_pinned() {
    let mut m = DenseModel::new(T);
    let ap = vec![rec(KIND_DATA, 1), rec(KIND_FIELD, 2)];
    let mut c = case(&mut m, &ap, 1, &[]);
    c.kfix = KIND_DATA;
    let e = run(&leaf1(T, c)).expect_err("arena kind");
    assert!(
        e.contains("mem win row") || e.contains("core append"),
        "{e}"
    );
}

/// close_advances_len: `len_post` one above the appends.
#[test]
fn c1_mut_close_advances_len() {
    let mut m = model(T, 16);
    let c = case(&mut m, &recs(12, 2), 1, &[]);
    let mut l = leaf1(T, c);
    l.cases[0].post.len += F::ONE;
    assert!(run(&l).is_err());
}

/// append_value_mismatch: the window holds another value than the append.
#[test]
fn c1_mut_append_value_mismatch() {
    let mut m = model(T, 16);
    let mut c = case(&mut m, &recs(12, 2), 1, &[]);
    c.appends[1][2] += F::ONE;
    expect_err(&leaf1(T, c), "ookup");
}

#[test]
fn c1_kinds_named() {
    assert_eq!((KIND_DATA, KIND_FIELD, KIND_ENV, KIND_SLOT), (1, 2, 3, 4));
    assert_ne!(empty_seg(), F::ZERO);
    assert_ne!(empty_seg(), pad_d());
}

// ---------------------------------------------------------------- rows

fn sections() -> serde_json::Value {
    let mut out = serde_json::Map::new();
    for (l, n) in rows_last() {
        let e = out.entry(l).or_insert(serde_json::json!([]));
        e.as_array_mut().unwrap().push(serde_json::json!(n));
    }
    serde_json::Value::Object(out)
}

fn shape(top: usize, w: usize, n_app: u64, n_cold: usize, arenas: usize) -> Leaf {
    let mut cases = Vec::new();
    for _ in 0..arenas {
        let mut m = model(top, 8 * (n_cold as u64).max(1) + 3);
        let reads: Vec<(u128, bool)> = (0..n_cold).map(|i| (8 * i as u128 + 1, true)).collect();
        cases.push(case(&mut m, &recs(700, n_app), w, &reads));
    }
    Leaf {
        top,
        cases,
        muts: vec![],
    }
}

/// C1 measurement: rows of each section (from the assignment offsets) and
/// `cost_model` rows and k of the harness, at TOP = 61 and 63.
#[test]
#[ignore = "C1 rows: cost_model of the dense memory harness (tens of seconds)"]
fn c1_rows() {
    let mut cfgs = vec![];
    for top in [61usize, 63] {
        for (w, n_app, n_cold, arenas) in [
            (1usize, 0u64, 0usize, 1usize),
            (1, 1, 0, 1),
            (1, 8, 0, 1),
            (1, 8, 1, 1),
            (2, 16, 0, 1),
            (3, 24, 0, 1),
            (4, 32, 0, 1),
            (5, 40, 0, 1),
            (1, 8, 4, 1),
            (1, 4, 0, 4),
        ] {
            cfgs.push((top, w, n_app, n_cold, arenas));
        }
    }
    for (top, w, n_app, n_cold, arenas) in cfgs {
        let c = shape(top, w, n_app, n_cold, arenas);
        run(&c).unwrap();
        let secs = sections();
        let m = circuit_model::<F, 48, 32>(&c);
        println!(
            "C1-ROWS {}",
            serde_json::json!({
                "top": top, "w": w, "appends": n_app, "cold_reads": n_cold, "arenas": arenas,
                "k": m.k, "rows": m.rows, "advice": m.advice_columns, "fixed": m.fixed_columns,
                "lookups": m.lookups, "permutations": m.permutations, "max_deg": m.max_deg,
                "sections": secs,
            })
        );
    }
}
