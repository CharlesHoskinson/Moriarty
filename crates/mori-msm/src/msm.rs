// SPDX-License-Identifier: Apache-2.0
//! Host side of the MSM: base tables, launch sequence, result conversion.
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use cudarc::driver::{CudaSlice, CudaStream, DriverError, LaunchConfig, PushKernelArg};
use group::{Curve, Group, prime::PrimeCurveAffine};
use midnight_curves::{Fq, G1Affine, G1Projective};

use crate::dev::{Dev, dev};

/// u64 words of one device point.
const AFF: usize = 12;
const XYZZ: usize = 24;

/// Tunables (environment overrides for the sweep).
#[derive(Clone, Copy, Debug)]
pub struct Cfg {
    /// Window bits.
    pub c: u32,
    /// Sorted entries per thread in the first and later reduction rounds.
    pub k1: u32,
    pub k2: u32,
    /// Buckets per thread in the bucket reduction.
    pub seg: u32,
    /// Most batch-affine passes before the XYZZ rounds (0 disables them).
    pub affine_passes: u32,
}

fn env_u32(k: &str) -> Option<u32> {
    std::env::var(k).ok().and_then(|s| s.parse().ok())
}

impl Cfg {
    pub fn for_n(n: usize) -> Cfg {
        let lg = usize::BITS - n.max(2).leading_zeros() - 1;
        let c = env_u32("MORI_MSM_C").unwrap_or((lg.saturating_sub(4)).clamp(8, 16));
        Cfg {
            c,
            k1: env_u32("MORI_MSM_K1").unwrap_or(64),
            k2: env_u32("MORI_MSM_K2").unwrap_or(16),
            seg: env_u32("MORI_MSM_SEG").unwrap_or(16).min(1 << (c - 1)),
            affine_passes: env_u32("MORI_MSM_AFFINE").unwrap_or(0),
        }
    }
}

/// A base table resident on the device (affine, blst layout).
pub struct Table {
    pub pts: CudaSlice<u64>,
    pub n: usize,
}

impl Table {
    pub fn new(d: &Dev, bases: &[G1Projective]) -> Result<Table, DriverError> {
        let mut aff = vec![G1Affine::identity(); bases.len()];
        G1Projective::batch_normalize(bases, &mut aff);
        Table::from_affine(d, &aff)
    }

    pub fn from_affine(d: &Dev, aff: &[G1Affine]) -> Result<Table, DriverError> {
        const _: () = assert!(std::mem::size_of::<G1Affine>() == 96);
        let words =
            unsafe { std::slice::from_raw_parts(aff.as_ptr() as *const u64, aff.len() * AFF) };
        Ok(Table {
            pts: d.stream.clone_htod(words)?,
            n: aff.len(),
        })
    }
}

fn cfg1(n: usize, threads: u32) -> LaunchConfig {
    let blocks = n.div_ceil(threads as usize).max(1) as u32;
    LaunchConfig {
        grid_dim: (blocks, 1, 1),
        block_dim: (threads, 1, 1),
        shared_mem_bytes: 0,
    }
}

fn cfg_strided(n: usize) -> LaunchConfig {
    let blocks = n.div_ceil(256).clamp(1, 8192) as u32;
    LaunchConfig {
        grid_dim: (blocks, 1, 1),
        block_dim: (256, 1, 1),
        shared_mem_bytes: 0,
    }
}

/// Device buffers kept across calls (allocation on this unified-memory device costs
/// tens of milliseconds per hundred MB). Buffers only grow.
#[derive(Default)]
pub struct Ws {
    b32: HashMap<&'static str, CudaSlice<u32>>,
    b64: HashMap<&'static str, CudaSlice<u64>>,
}

impl Ws {
    fn u32(
        &mut self,
        s: &Arc<CudaStream>,
        name: &'static str,
        len: usize,
    ) -> Result<CudaSlice<u32>, DriverError> {
        match self.b32.remove(name) {
            Some(b) if b.len() >= len.max(1) => Ok(b),
            _ => unsafe { s.alloc::<u32>(len.max(1)) },
        }
    }
    fn u64(
        &mut self,
        s: &Arc<CudaStream>,
        name: &'static str,
        len: usize,
    ) -> Result<CudaSlice<u64>, DriverError> {
        match self.b64.remove(name) {
            Some(b) if b.len() >= len.max(1) => Ok(b),
            _ => unsafe { s.alloc::<u64>(len.max(1)) },
        }
    }
    fn put32(&mut self, name: &'static str, b: CudaSlice<u32>) {
        self.b32.insert(name, b);
    }
    fn put64(&mut self, name: &'static str, b: CudaSlice<u64>) {
        self.b64.insert(name, b);
    }
}

/// Launches a kernel of the loaded module with the given arguments.
macro_rules! launch {
    ($r:expr, $name:expr, $cfg:expr; $($arg:expr),* $(,)?) => {{
        let func = $r.d.func($name);
        let mut b = $r.s.launch_builder(&func);
        $( b.arg($arg); )*
        unsafe { b.launch($cfg) }?;
    }};
}

static WS: Mutex<Option<Ws>> = Mutex::new(None);

struct Run<'a> {
    d: &'a Dev,
    s: &'a Arc<CudaStream>,
}

impl Run<'_> {
    /// Exclusive scan of `inp[..n]` into `out`; returns (total, nonzero count, max).
    fn scan(
        &self,
        ws: &mut Ws,
        inp: &CudaSlice<u32>,
        out: &mut CudaSlice<u32>,
        n: usize,
    ) -> Result<(u32, u32, u32), DriverError> {
        let nb = n.div_ceil(1024).max(1);
        let mut tmp = ws.u32(self.s, "scan_tmp", nb * 256)?;
        let mut sums = ws.u32(self.s, "scan_sums", nb + 1)?;
        let mut nz = ws.u32(self.s, "scan_nz", 1)?;
        let mut mx = ws.u32(self.s, "scan_mx", 1)?;
        self.s.memset_zeros(&mut nz)?;
        self.s.memset_zeros(&mut mx)?;
        let (n32, nb32) = (n as u32, nb as u32);
        launch!(self, "scan_blocks", LaunchConfig { grid_dim: (nb32, 1, 1), block_dim: (256, 1, 1), shared_mem_bytes: 0 }; inp, &mut *out, &n32, &mut tmp, &mut sums, &mut nz, &mut mx);
        launch!(self, "scan_sums", cfg1(1, 1); &mut sums, &nb32);
        launch!(self, "scan_add", cfg_strided(n); &mut *out, &n32, &sums);
        let tot = self.s.clone_dtoh(&sums.slice(nb..nb + 1))?[0];
        let nzv = self.s.clone_dtoh(&nz.slice(0..1))?[0];
        let mxv = self.s.clone_dtoh(&mx.slice(0..1))?[0];
        ws.put32("scan_tmp", tmp);
        ws.put32("scan_sums", sums);
        ws.put32("scan_nz", nz);
        ws.put32("scan_mx", mx);
        Ok((tot, nzv, mxv))
    }
}

/// Fp words of one device field element.
const FPW: usize = 6;
const INV_NAMES: [(&str, &str); 6] = [
    ("inv_pre0", "inv_tot0"),
    ("inv_pre1", "inv_tot1"),
    ("inv_pre2", "inv_tot2"),
    ("inv_pre3", "inv_tot3"),
    ("inv_pre4", "inv_tot4"),
    ("inv_pre5", "inv_tot5"),
];

/// In-place inversion of `n` nonzero field elements: chunks of 32 prefix products
/// level by level, a Fermat inversion per value at the top (at most 4,096 of them).
fn batch_invert(
    r: &Run<'_>,
    ws: &mut Ws,
    v: &mut CudaSlice<u64>,
    n: usize,
    lvl: usize,
) -> Result<(), DriverError> {
    let n32 = n as u32;
    if n <= 4096 || lvl + 1 == INV_NAMES.len() {
        launch!(r, "inv_fermat", cfg1(n, 64); &mut *v, &n32);
        return Ok(());
    }
    let g = 32u32;
    let nt = n.div_ceil(g as usize);
    let nt32 = nt as u32;
    let (pn, tn) = INV_NAMES[lvl];
    let mut pre = ws.u64(r.s, pn, n * FPW)?;
    let mut tot = ws.u64(r.s, tn, nt * FPW)?;
    launch!(r, "inv_up", cfg1(nt, 64); &*v, &n32, &g, &mut pre, &mut tot, &nt32);
    batch_invert(r, ws, &mut tot, nt, lvl + 1)?;
    launch!(r, "inv_down", cfg1(nt, 64); &mut *v, &n32, &g, &pre, &tot, &nt32);
    ws.put64(pn, pre);
    ws.put64(tn, tot);
    Ok(())
}

/// Counters of the last run (receipts and tests).
#[derive(Clone, Copy, Debug, Default)]
pub struct Trace {
    pub entries: usize,
    pub buckets: usize,
    pub max_bucket: usize,
    pub rounds: u32,
    pub first_round_out: usize,
    pub affine_passes: u32,
    pub affine_out: usize,
}

/// Longest key run the final run-aligned pass accepts.
const RUN_MAX: usize = 8;

/// MSM of `scalars` (blst_fr Montgomery words) over the first `scalars.len()` table points.
pub fn run(
    d: &Dev,
    t: &Table,
    scalars: &[Fq],
    cfg: Cfg,
) -> Result<(G1Projective, Trace), DriverError> {
    let mut g = WS.lock().unwrap();
    let ws = g.get_or_insert_with(Ws::default);
    run_ws(d, ws, t, scalars, cfg)
}

fn run_ws(
    d: &Dev,
    ws: &mut Ws,
    t: &Table,
    scalars: &[Fq],
    cfg: Cfg,
) -> Result<(G1Projective, Trace), DriverError> {
    const _: () = assert!(std::mem::size_of::<Fq>() == 32);
    let n = scalars.len();
    assert!(n <= t.n && n < (1 << 31));
    let s = &d.stream;
    let r = Run { d, s };
    let mut tr = Trace::default();
    let prof = std::env::var_os("MORI_MSM_PROFILE").is_some();
    let t0 = std::time::Instant::now();
    let mut marks: Vec<(&str, f64)> = vec![];
    let mut mark = |name: &'static str| -> Result<(), DriverError> {
        if prof {
            s.synchronize()?;
            marks.push((name, t0.elapsed().as_secs_f64() * 1e3));
        }
        Ok(())
    };
    let c = cfg.c;
    let nw = 255 / c + 1;
    let half = 1usize << (c - 1);
    let nb = nw as usize * half;
    let ne = nw as usize * n;

    // 1. Digits and histogram.
    let words = unsafe { std::slice::from_raw_parts(scalars.as_ptr() as *const u64, n * 4) };
    let mut dsc = ws.u64(s, "scalars", n * 4)?;
    s.memcpy_htod(words, &mut dsc.slice_mut(0..n * 4))?;
    mark("upload")?;
    let mut keys = ws.u32(s, "keys", ne)?;
    let mut vals = ws.u32(s, "vals", ne)?;
    let mut counts = ws.u32(s, "counts", nb)?;
    s.memset_zeros(&mut counts.slice_mut(0..nb))?;
    let n32 = n as u32;
    launch!(r, "msm_digits", cfg_strided(n); &dsc, &n32, &c, &nw, &mut keys, &mut vals, &mut counts);
    ws.put64("scalars", dsc);
    mark("digits")?;

    // 2. Bucket offsets, distinct buckets, largest bucket.
    let mut offs = ws.u32(s, "offs", nb)?;
    let (m, distinct, maxb) = r.scan(ws, &counts, &mut offs, nb)?;
    ws.put32("counts", counts);
    let (m, distinct, maxb) = (m as usize, distinct as usize, maxb as usize);
    tr.entries = m;
    tr.buckets = distinct;
    tr.max_bucket = maxb;
    mark("scan")?;
    if m == 0 {
        ws.put32("keys", keys);
        ws.put32("vals", vals);
        ws.put32("offs", offs);
        return Ok((G1Projective::identity(), tr));
    }

    // 3. Counting sort by key.
    let mut skeys = ws.u32(s, "skeys", m)?;
    let mut svals = ws.u32(s, "svals", m)?;
    let ne32 = ne as u32;
    launch!(r, "msm_scatter", cfg_strided(ne); &keys, &vals, &ne32, &mut offs, &mut skeys, &mut svals);
    ws.put32("keys", keys);
    ws.put32("vals", vals);
    ws.put32("offs", offs);
    mark("scatter")?;

    // 3b. Batch-affine passes: pairs of equal keys added in affine coordinates with one
    //     batch inversion per pass. Stops when a pass would remove under 10 percent.
    let null = 0u64;
    let (mut pk, mut pk_name) = (skeys, "skeys");
    let mut paff: Option<(CudaSlice<u64>, &'static str)> = None;
    let mut l0 = m;
    let mut bound0 = maxb;
    for pass in 0..cfg.affine_passes {
        let par = pass % 2;
        let nq = if par == 0 { l0.div_ceil(2) } else { l0 / 2 + 1 };
        let (l32, nq32) = (l0 as u32, nq as u32);
        let mut cnt = ws.u32(s, "pa_cnt", nq)?;
        launch!(r, "pa_count", cfg1(nq, 128); &pk, &l32, &par, &nq32, &mut cnt);
        let mut off = ws.u32(s, "pa_off", nq)?;
        let (lo, _, _) = r.scan(ws, &cnt, &mut off, nq)?;
        ws.put32("pa_cnt", cnt);
        let lo = lo as usize;
        if lo * 10 > l0 * 9 {
            ws.put32("pa_off", off);
            break;
        }
        let mut den = ws.u64(s, "pa_den", nq * FPW)?;
        match &paff {
            None => {
                launch!(r, "pa_denom", cfg_strided(nq); &pk, &svals, &t.pts, &null, &l32, &par, &nq32, &mut den)
            }
            Some((p, _)) => {
                launch!(r, "pa_denom", cfg_strided(nq); &pk, &null, &null, p, &l32, &par, &nq32, &mut den)
            }
        }
        batch_invert(&r, ws, &mut den, nq, 0)?;
        let on: [(&str, &str); 2] = [("pa_k0", "pa_p0"), ("pa_k1", "pa_p1")];
        let (kn, pn) = on[pass as usize % 2];
        let mut ok = ws.u32(s, kn, lo)?;
        let mut op = ws.u64(s, pn, lo * AFF)?;
        match &paff {
            None => {
                launch!(r, "pa_apply", cfg_strided(nq); &pk, &svals, &t.pts, &null, &l32, &par, &nq32, &den, &off, &mut ok, &mut op)
            }
            Some((p, _)) => {
                launch!(r, "pa_apply", cfg_strided(nq); &pk, &null, &null, p, &l32, &par, &nq32, &den, &off, &mut ok, &mut op)
            }
        }
        ws.put64("pa_den", den);
        ws.put32("pa_off", off);
        ws.put32(pk_name, pk);
        if let Some((p, name)) = paff.take() {
            ws.put64(name, p);
        }
        (pk, pk_name) = (ok, kn);
        paff = Some((op, pn));
        l0 = lo;
        // A run of x entries keeps at most ceil(x / 2) + 1 after a pass.
        bound0 = bound0.div_ceil(2) + 1;
        tr.affine_passes += 1;
        tr.affine_out = lo;
        mark("affine")?;
    }
    // Explicit affine points are the "bases" of the first XYZZ round, indexed by iota.
    let mut ident: Option<CudaSlice<u32>> = None;
    if paff.is_some() {
        let mut v = ws.u32(s, "iota", l0)?;
        let l32 = l0 as u32;
        launch!(r, "iota", cfg_strided(l0); &mut v, &l32);
        ident = Some(v);
    }
    let first_vals = ident.as_ref().unwrap_or(&svals);
    let first_bases = paff.as_ref().map(|(p, _)| p).unwrap_or(&t.pts);

    // 4. Load-balanced segmented reduction rounds. A run of length x becomes at most
    //    ceil(x / k) + 1 entries per round; stop when every run is at most RUN_MAX.
    let mut bound = bound0;
    let (mut ck, mut cp, mut l) = (pk, None::<CudaSlice<u64>>, l0);
    let mut slot = 0usize;
    const KN: [&str; 2] = ["rk0", "rk1"];
    const PN: [&str; 2] = ["rp0", "rp1"];
    while bound > RUN_MAX || tr.rounds == 0 {
        let k = if tr.rounds == 0 { cfg.k1 } else { cfg.k2 };
        let shift = if tr.rounds % 2 == 1 { k / 2 } else { 0 };
        let nt = (l + shift as usize).div_ceil(k as usize);
        let (l32, nt32) = (l as u32, nt as u32);
        let mut cnt = ws.u32(s, "cnt", nt)?;
        launch!(r, "seg_count", cfg1(nt, 128); &ck, &l32, &k, &shift, &mut cnt, &nt32);
        let mut off = ws.u32(s, "off", nt)?;
        let (l2, _, _) = r.scan(ws, &cnt, &mut off, nt)?;
        ws.put32("cnt", cnt);
        let l2 = l2 as usize;
        let mut ok = ws.u32(s, KN[slot], l2)?;
        let mut op = ws.u64(s, PN[slot], l2 * XYZZ)?;
        match &cp {
            None => {
                launch!(r, "seg_reduce_affine", cfg1(nt, 64); &ck, first_vals, &l32, &k, &shift, &nt32, first_bases, &off, &mut ok, &mut op)
            }
            Some(p) => {
                launch!(r, "seg_reduce_xyzz", cfg1(nt, 64); &ck, p, &l32, &k, &shift, &nt32, &off, &mut ok, &mut op)
            }
        }
        ws.put32("off", off);
        // Return the input buffers to the pool.
        if tr.rounds == 0 {
            ws.put32(pk_name, ck);
            tr.first_round_out = l2;
        } else {
            ws.put32(KN[1 - slot], ck);
            ws.put64(PN[1 - slot], cp.take().unwrap());
        }
        ck = ok;
        cp = Some(op);
        slot = 1 - slot;
        l = l2;
        bound = bound.div_ceil(k as usize) + 1;
        tr.rounds += 1;
        mark("round")?;
    }
    if let Some(v) = ident {
        ws.put32("iota", v);
    }
    if let Some((p, name)) = paff {
        ws.put64(name, p);
    }
    ws.put32("svals", svals);

    // 5. Run-aligned final accumulation into the dense bucket array.
    let mut buckets = ws.u64(s, "buckets", nb * XYZZ)?;
    s.memset_zeros(&mut buckets.slice_mut(0..nb * XYZZ))?;
    mark("bucket_zero")?;
    let l32 = l as u32;
    let p = cp.take().unwrap();
    if std::env::var_os("MORI_MSM_RUN_SCATTER").is_some() {
        launch!(r, "run_scatter", cfg_strided(l); &ck, &p, &l32, &mut buckets);
    } else {
        let mut fl = ws.u32(s, "cnt", l)?;
        launch!(r, "run_flags", cfg_strided(l); &ck, &l32, &mut fl);
        let mut off = ws.u32(s, "off", l)?;
        let (nr, _, _) = r.scan(ws, &fl, &mut off, l)?;
        let mut st = ws.u32(s, "starts", nr as usize)?;
        launch!(r, "run_compact", cfg_strided(l); &ck, &l32, &off, &mut st);
        launch!(r, "run_sum", cfg1(nr as usize, 64); &ck, &p, &st, &nr, &l32, &mut buckets);
        ws.put32("cnt", fl);
        ws.put32("off", off);
        ws.put32("starts", st);
    }
    ws.put32(KN[1 - slot], ck);
    ws.put64(PN[1 - slot], p);
    mark("buckets")?;

    // 6. Weighted window sums: recursive running sums on the device, the short tail and
    //    the window combination on the host.
    let len = cfg.seg.max(2);
    let mut levels: Vec<Vec<G1Projective>> = vec![]; // levels[lvl][w] = sum of U at that level
    let (mut src, mut cnt, mut stride, mut off) = (buckets, half, half, 0usize);
    let mut lvl = 0;
    loop {
        let segs = cnt.div_ceil(len as usize).max(1);
        let nseg = segs * nw as usize;
        let mut u = ws.u64(s, ["u0", "u1"][lvl % 2], nseg * XYZZ)?;
        let mut rr = ws.u64(s, ["r0", "r1"][lvl % 2], nseg * XYZZ)?;
        let (c32, st32, of32, sg32) = (cnt as u32, stride as u32, off as u32, segs as u32);
        launch!(r, "wsum_level", cfg1(nseg, 64); &src, &nw, &c32, &st32, &of32, &len, &sg32, &mut u, &mut rr);
        levels.push(sum_per_window(&r, ws, &u, nw as usize, segs)?);
        ws.put64(["u0", "u1"][lvl % 2], u);
        if lvl == 0 {
            ws.put64("buckets", src);
        } else {
            ws.put64(["r0", "r1"][(lvl - 1) % 2], src);
        }
        if segs == 1 {
            ws.put64(["r0", "r1"][lvl % 2], rr);
            break;
        }
        // Next level: weights s for R_s, s >= 1, as (s' + 1) R_(s' + 1).
        src = rr;
        cnt = segs - 1;
        stride = segs;
        off = 1;
        lvl += 1;
    }
    mark("wsum")?;
    // W_w = sum_lvl len^lvl * levels[lvl][w]; result = sum_w 2^(c w) W_w.
    let mut res = G1Projective::identity();
    for w in (0..nw as usize).rev() {
        let mut ww = G1Projective::identity();
        for lv in levels.iter().rev() {
            for _ in 0..len.trailing_zeros() {
                ww = ww.double();
            }
            ww += lv[w];
        }
        for _ in 0..c {
            res = res.double();
        }
        res += ww;
    }
    mark("final")?;
    if prof {
        eprintln!(
            "mori-msm n={n} c={c} m={m} buckets={distinct} maxb={maxb} affine={}/{} rounds={} r1out={} ms={marks:?}",
            tr.affine_passes, tr.affine_out, tr.rounds, tr.first_round_out
        );
    }
    Ok((res, tr))
}

/// Plain per-window sums of `per` points per window (window-major), down to a few
/// points on the device, then on the host.
fn sum_per_window(
    r: &Run<'_>,
    ws: &mut Ws,
    a: &CudaSlice<u64>,
    nw: usize,
    mut per: usize,
) -> Result<Vec<G1Projective>, DriverError> {
    let g = 16usize;
    let mut cur: Option<CudaSlice<u64>> = None;
    let mut slot = 0;
    while per > 4 {
        let nper = per.div_ceil(g);
        let mut out = ws.u64(r.s, ["sw0", "sw1"][slot], nw * nper * XYZZ)?;
        let (nw32, c32, st32, g32) = (nw as u32, per as u32, per as u32, g as u32);
        let src = cur.as_ref().unwrap_or(a);
        launch!(r, "sum_rows", cfg1(nw * nper, 64); src, &nw32, &c32, &st32, &g32, &mut out);
        if let Some(old) = cur.take() {
            ws.put64(["sw0", "sw1"][1 - slot], old);
        }
        cur = Some(out);
        slot = 1 - slot;
        per = nper;
    }
    let h =
        r.s.clone_dtoh(&cur.as_ref().unwrap_or(a).slice(0..nw * per * XYZZ))?;
    if let Some(old) = cur.take() {
        ws.put64(["sw0", "sw1"][1 - slot], old);
    }
    Ok((0..nw)
        .map(|w| {
            (0..per)
                .map(|t| xyzz_to_projective(&h[(w * per + t) * XYZZ..(w * per + t + 1) * XYZZ]))
                .fold(G1Projective::identity(), |x, y| x + y)
        })
        .collect())
}

/// (X, Y, ZZ, ZZZ) to Jacobian (X ZZ ZZZ^2, Y ZZZ^4, ZZ ZZZ), the blst_p1 layout.
fn xyzz_to_projective(h: &[u64]) -> G1Projective {
    use blst::{blst_fp, blst_fp_mul, blst_fp_sqr, blst_p1};
    let f = |i: usize| blst_fp {
        l: h[i * 6..i * 6 + 6].try_into().unwrap(),
    };
    let (x, y, zz, zzz) = (f(0), f(1), f(2), f(3));
    if zz.l == [0; 6] {
        return G1Projective::identity();
    }
    let mut zzz2 = blst_fp::default();
    let mut t = blst_fp::default();
    let mut p = blst_p1::default();
    unsafe {
        blst_fp_sqr(&mut zzz2, &zzz);
        blst_fp_mul(&mut t, &x, &zz);
        blst_fp_mul(&mut p.x, &t, &zzz2);
        blst_fp_sqr(&mut t, &zzz2);
        blst_fp_mul(&mut p.y, &y, &t);
        blst_fp_mul(&mut p.z, &zz, &zzz);
    }
    const _: () = assert!(std::mem::size_of::<G1Projective>() == std::mem::size_of::<blst_p1>());
    unsafe { std::mem::transmute::<blst_p1, G1Projective>(p) }
}

struct Entry {
    table: Table,
    f0: G1Affine,
    f1: G1Affine,
}

/// Base tables keyed by (address, length) of the caller's bases, checked against the
/// first and last point so a reused allocation is never served stale.
static TABLES: Mutex<Option<HashMap<(usize, usize), Entry>>> = Mutex::new(None);

/// Runs `f` with the resident table of `bases`, building it (and clearing its masks)
/// when the caller's bases are new or changed.
pub(crate) fn with_table<R>(
    bases: &[G1Projective],
    f: impl FnOnce(&Dev, &Table) -> R,
) -> Option<R> {
    let d = dev()?;
    if bases.is_empty() {
        return None;
    }
    let key = (bases.as_ptr() as usize, bases.len());
    let (f0, f1) = (bases[0].to_affine(), bases[bases.len() - 1].to_affine());
    let mut tables = TABLES.lock().unwrap();
    let tables = tables.get_or_insert_with(HashMap::new);
    let fresh = tables.get(&key).is_none_or(|e| e.f0 != f0 || e.f1 != f1);
    if fresh {
        tables.remove(&key);
        crate::mask::forget(key);
        let table = Table::new(d, bases).ok()?;
        tables.insert(key, Entry { table, f0, f1 });
    }
    Some(f(d, &tables.get(&key).unwrap().table))
}

/// `sum scalars[i] * bases[i]` without the mask (bases may be longer; the prefix is used).
pub fn msm_raw(scalars: &[Fq], bases: &[G1Projective]) -> Option<G1Projective> {
    let n = scalars.len();
    if n == 0 || n > bases.len() {
        return None;
    }
    with_table(bases, |d, t| match run(d, t, scalars, Cfg::for_n(n)) {
        Ok((p, _)) => Some(p),
        Err(err) => {
            eprintln!("mori-msm: {err:?}");
            None
        }
    })?
}
