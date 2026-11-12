//! S10x: trace heights of an OpenVM internal node over N leaf proofs (multi-leaf node), and the
//! effect of the fixed-size RANGE table on the leaf and on the node.
//!
//! Usage: s10x_node <w1_log_rows> <range> <arities> <prove_arity> <gate_rows>
//!   w1_log_rows : log2 rows of the W1 leaf (Poseidon2 AIR, app params).
//!   range       : 0 = leaf is W1 alone; 1 = leaf is W1 plus the stock VariableRangeCheckerAir
//!                 at range_max_bits 16 (the FP-1 RANGE table: (x, w), w in 0..=16, 2^17 rows),
//!                 with zero multiplicities (table footprint only, no sender AIR); 2 = as 1 plus a
//!                 sender AIR of 2^14 rows x 4 columns, each cell a 16-bit RANGE lookup.
//!   arities     : comma list, ascending, each in {1,2,3,4,5,6,7,8,12,15,16,32}.
//!   prove_arity : 0, or one arity of the list whose node is proved and verified at the OD-19
//!                 internal set (only if its stacked layout fits).
//!   gate_rows   : stop after the first arity whose tallest padded AIR height exceeds this.
//!
//! Node params: OD-19 internal set (UD, log blowup 3, queries 97/82/81, fold PoW 0, mu PoW 9,
//! query PoW 21, LogUp PoW 18, l_skip 2, n_stack 17, w_stack 512). Node heights depend on the
//! children, not on the node's own params (D10-HEIGHTS.md section 1), so tracegen continues past
//! the stacked-height cap 2^(l_skip + n_stack) = 2^19; the cap is reported, not raised.
//! Prints one JSON object per line on stdout (leaf, then one per arity).

use std::{cmp::Reverse, collections::BTreeMap, io::Write, sync::Arc, time::Instant};

use openvm_circuit_primitives::var_range::{VariableRangeCheckerBus, VariableRangeCheckerChip};
use openvm_cpu_backend::CpuBackend;
use openvm_poseidon2_air::{Poseidon2Config, Poseidon2SubChip};
use openvm_recursion_circuit::system::{
    AggregationSubCircuit, CachedTraceCtx, VerifierSubCircuit, VerifierTraceGen,
};
use openvm_stark_backend::{
    calculate_n_logup,
    keygen::types::{MultiStarkProvingKey, MultiStarkVerifyingKey},
    p3_field::{PrimeCharacteristicRing, PrimeField32},
    p3_matrix::Matrix,
    proof::Proof,
    prover::{
        stacked_pcs::StackedLayout, AirProvingContext, ColMajorMatrix, DeviceDataTransporter,
        ProvingContext,
    },
    interaction::InteractionBuilder,
    p3_air::{Air, BaseAir},
    p3_matrix::dense::RowMajorMatrix,
    AirRef, BaseAirWithPublicValues, PartitionedBaseAir, StarkEngine, SystemParams,
    WhirProximityStrategy,
};
use openvm_stark_sdk::config::{
    app_params_with_100_bits_security,
    baby_bear_poseidon2::{
        default_duplex_sponge_recorder, BabyBearPoseidon2Config, BabyBearPoseidon2CpuEngine,
        DuplexSponge, DuplexSpongeRecorder,
    },
    params_with_100_bits_security, MAX_APP_LOG_STACKED_HEIGHT,
};
use p3_baby_bear::BabyBear;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::{json, Value};

type F = BabyBear;
type SC = BabyBearPoseidon2Config;
type Eng = BabyBearPoseidon2CpuEngine<DuplexSponge>;

/// Sender AIR for range mode 2: SEND_W columns, each row sends (x_i, 16) to the RANGE table.
const SEND_W: usize = 4;
const SEND_LOG_ROWS: usize = 14;
#[derive(Clone, Copy, Debug)]
struct RangeSendAir {
    bus: VariableRangeCheckerBus,
}
impl<T> BaseAir<T> for RangeSendAir {
    fn width(&self) -> usize {
        SEND_W
    }
}
impl<T: openvm_stark_backend::p3_field::Field> BaseAirWithPublicValues<T> for RangeSendAir {}
impl<T: openvm_stark_backend::p3_field::Field> PartitionedBaseAir<T> for RangeSendAir {}
impl<AB: InteractionBuilder> Air<AB> for RangeSendAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let row = main.row_slice(0).expect("window");
        let local: Vec<AB::Var> = row.to_vec();
        drop(row);
        for &x in local.iter() {
            self.bus.range_check(x, 16).eval(builder, AB::Expr::ONE);
        }
    }
}

/// OD-19 internal set (UD-SIZING.md 5.1), as in l_heights.rs.
fn internal_ud() -> SystemParams {
    let mut p = params_with_100_bits_security(
        3, 2, 17, 512, 0, 9, WhirProximityStrategy::UniqueDecoding, 4, 21, 4,
    );
    assert_eq!(p.whir.rounds.len(), 3, "WHIR round count");
    for (r, q) in p.whir.rounds.iter_mut().zip([97usize, 82, 81]) {
        r.num_queries = q;
    }
    p.logup.pow_bits = 18;
    p
}

fn params_json(p: &SystemParams) -> Value {
    json!({
        "l_skip": p.l_skip, "n_stack": p.n_stack, "w_stack": p.w_stack, "log_blowup": p.log_blowup,
        "k_whir": p.k_whir(), "proximity": format!("{:?}", p.whir.proximity),
        "queries": p.whir.rounds.iter().map(|r| r.num_queries).collect::<Vec<_>>(),
        "log_final_poly_len": p.log_final_poly_len(), "num_whir_rounds": p.num_whir_rounds(),
        "folding_pow_bits": p.whir.folding_pow_bits, "mu_pow_bits": p.whir.mu_pow_bits,
        "query_phase_pow_bits": p.whir.query_phase_pow_bits, "logup_pow_bits": p.logup.pow_bits,
    })
}

/// FinalPolyQueryEvalAir valid rows per child proof (recursion whir/final_poly_query_eval
/// compute_round_offsets), from the child's params.
fn fpqe_rows_per_proof(p: &SystemParams) -> usize {
    let nr = p.num_whir_rounds();
    let k = p.k_whir();
    let fpl = 1usize << p.log_final_poly_len();
    p.whir
        .rounds
        .iter()
        .enumerate()
        .map(|(r, round)| (round.num_queries + 1) * (k * (nr - (r + 1)) + fpl))
        .sum()
}

fn log2_exact(h: usize) -> usize {
    assert!(h.is_power_of_two(), "height {h} not a power of two");
    h.trailing_zeros() as usize
}

/// Rows up to and including the last row with a nonzero cell (a lower bound on valid rows; the
/// recursion trace generators zero-fill their padding).
fn nonzero_rows<M: Matrix<F>>(m: &M) -> usize {
    let (h, w) = (m.height(), m.width());
    for r in (0..h).rev() {
        for c in 0..w {
            if m.get(r, c).unwrap().as_canonical_u32() != 0 {
                return r + 1;
            }
        }
    }
    0
}

fn shape(
    pk: &MultiStarkProvingKey<SC>,
    ctxs: &[AirProvingContext<CpuBackend<SC>>],
    l_skip: usize,
    n_stack: usize,
) -> Value {
    assert_eq!(ctxs.len(), pk.per_air.len(), "one ctx per AIR");
    let mut airs = vec![];
    let mut per: Vec<(usize, usize, Vec<usize>, usize, usize)> = vec![];
    let mut cells = 0usize;
    for (i, (apk, ctx)) in pk.per_air.iter().zip(ctxs).enumerate() {
        let h = Matrix::height(&ctx.common_main);
        let w = Matrix::width(&ctx.common_main);
        let cw: Vec<usize> = ctx.cached_mains.iter().map(|c| Matrix::width(&c.trace)).collect();
        let ni = apk.vk.symbolic_constraints.interactions.len();
        cells += h * (w + cw.iter().sum::<usize>());
        let nz = nonzero_rows(&ctx.common_main);
        airs.push(json!({"idx": i, "name": apk.air_name, "height": h, "log_height": log2_exact(h),
                         "nonzero_rows": nz, "width_common": w, "width_cached": cw,
                         "width_preprocessed": apk.vk.params.width.preprocessed.unwrap_or(0),
                         "num_interactions": ni}));
        per.push((i, w, cw, log2_exact(h), ni));
    }
    let mut sorted = per.clone();
    sorted.sort_by_key(|t| Reverse(t.3));
    let lsh = n_stack + l_skip;
    let common = StackedLayout::new(l_skip, lsh, sorted.iter().map(|t| (t.1, t.3)).collect())
        .map(|l| l.width())
        .map_err(|e| format!("{e:?}"));
    let other: Vec<Result<usize, String>> = sorted
        .iter()
        .flat_map(|t| t.2.iter().map(move |&cw| (cw, t.3)))
        .map(|(cw, lh)| {
            StackedLayout::new(l_skip, lsh, vec![(cw, lh)])
                .map(|l| l.width())
                .map_err(|e| format!("{e:?}"))
        })
        .collect();
    let layout_ok = common.is_ok() && other.iter().all(|o| o.is_ok());
    let w_stack = if layout_ok {
        Some(common.clone().unwrap() + other.iter().map(|o| *o.as_ref().unwrap()).sum::<usize>())
    } else {
        None
    };
    let total_int: u64 = per.iter().map(|t| (t.4 as u64) << t.3.max(l_skip)).sum();
    let n_logup = calculate_n_logup(l_skip, total_int);
    let mut hist: BTreeMap<usize, usize> = BTreeMap::new();
    for t in &per {
        *hist.entry(t.3).or_default() += 1;
    }
    let (imax, maxlh) = per.iter().map(|t| (t.0, t.3)).max_by_key(|x| x.1).unwrap();
    json!({
        "airs": airs, "num_airs": per.len(), "main_cells": cells,
        "log_stacked_height": lsh, "layout_ok": layout_ok,
        "layout_err": common.err().or_else(|| other.iter().find_map(|o| o.clone().err())),
        "w_stack_needed": w_stack,
        "total_interactions_lifted": total_int, "n_logup": n_logup,
        "gkr_rounds_T": if total_int == 0 { 0 } else { l_skip + n_logup },
        "log_height_histogram": hist, "max_log_height": maxlh,
        "max_air": pk.per_air[imax].air_name, "max_height": 1usize << maxlh,
        "max_air_nonzero_rows": airs[imax]["nonzero_rows"].clone(),
    })
}

fn leaf_proofs(
    n: usize,
    log_rows: usize,
    range: u32,
) -> (MultiStarkVerifyingKey<SC>, Vec<Proof<SC>>, Value) {
    let mut rng = StdRng::seed_from_u64(0x5e7_10c);
    let chip = Poseidon2SubChip::<F, 1>::new(Poseidon2Config::<F>::default().constants);
    let mut airs: Vec<AirRef<SC>> = vec![chip.air.clone()];
    let bus = VariableRangeCheckerBus::new(1, 16);
    let rc = if range >= 1 {
        let rc = VariableRangeCheckerChip::new(bus);
        airs.push(Arc::new(rc.air));
        Some(rc)
    } else {
        None
    };
    if range == 2 {
        airs.push(Arc::new(RangeSendAir { bus }));
    }
    let params = app_params_with_100_bits_security(MAX_APP_LOG_STACKED_HEIGHT);
    let engine = Eng::new(params.clone());
    let (pk, vk) = engine.keygen(&airs);
    let d_pk = engine.device().transport_pk_to_device(&pk);
    let mut proofs = vec![];
    let mut leaf_shape = Value::Null;
    let mut times = vec![];
    for i in 0..n {
        let inputs: Vec<[F; 16]> = (0..1usize << log_rows)
            .map(|_| std::array::from_fn(|_| F::from_u32(rng.random::<u32>() % (1 << 30))))
            .collect();
        let mut traces = vec![chip.generate_trace(inputs)];
        let mut send = None;
        if range == 2 {
            let n = 1usize << SEND_LOG_ROWS;
            let mut v = F::zero_vec(n * SEND_W);
            for x in v.iter_mut() {
                let u = rng.random::<u32>() & 0xffff;
                rc.as_ref().unwrap().add_count(u, 16);
                *x = F::from_u32(u);
            }
            send = Some(RowMajorMatrix::new(v, SEND_W));
        }
        if let Some(rc) = &rc {
            traces.push(rc.generate_trace::<F>());
        }
        if let Some(t) = send {
            traces.push(t);
        }
        let ctxs: Vec<AirProvingContext<CpuBackend<SC>>> = traces
            .iter()
            .map(|t| {
                AirProvingContext::simple_no_pis(
                    engine.device().transport_matrix_to_device(&ColMajorMatrix::from_row_major(t)),
                )
            })
            .collect();
        if i == 0 {
            leaf_shape = shape(&pk, &ctxs, params.l_skip, params.n_stack);
        }
        let t = Instant::now();
        let proof = engine
            .prove(&d_pk, ProvingContext::new(ctxs.into_iter().enumerate().collect()))
            .expect("leaf prove");
        times.push(t.elapsed().as_secs_f64());
        engine.verify(&vk, &proof).expect("leaf verify");
        proofs.push(proof);
        eprintln!("leaf {i} proved+verified {:.1}s", times[i]);
    }
    let out = json!({"kind": "leaf", "range": range, "w1_log_rows": log_rows, "n_proofs": n,
                     "params": params_json(&params), "prove_s": times, "verified": true,
                     "fpqe_rows_per_child_proof": fpqe_rows_per_proof(&params),
                     "shape": leaf_shape});
    (vk, proofs, out)
}

fn node_run<const N: usize>(
    params: SystemParams,
    child_vk: &MultiStarkVerifyingKey<SC>,
    proofs: &[Proof<SC>],
    prove: bool,
) -> Value
where
    VerifierSubCircuit<N>: VerifierTraceGen<CpuBackend<SC>, SC, ()> + AggregationSubCircuit,
{
    assert_eq!(proofs.len(), N);
    let engine = Eng::new(params.clone());
    let circuit = <VerifierSubCircuit<N> as VerifierTraceGen<CpuBackend<SC>, SC, ()>>::new(
        Arc::new(child_vk.clone()),
        Default::default(),
    );
    let child_pcs = circuit.commit_child_vk(&engine, child_vk);
    let airs = circuit.airs::<SC>();
    let t = Instant::now();
    let (pk, vk) = engine.keygen(&airs);
    let keygen_s = t.elapsed().as_secs_f64();
    let t = Instant::now();
    let ctxs = circuit.generate_proving_ctxs_base::<DuplexSpongeRecorder>(
        child_vk,
        CachedTraceCtx::PcsData(child_pcs),
        proofs,
        &(),
        default_duplex_sponge_recorder(),
    );
    let tracegen_s = t.elapsed().as_secs_f64();
    let sh = shape(&pk, &ctxs, params.l_skip, params.n_stack);
    let layout_ok = sh["layout_ok"].as_bool() == Some(true);
    let mut out = json!({"kind": "node", "arity": N, "params": params_json(&params),
                         "keygen_s": keygen_s, "tracegen_s": tracegen_s, "shape": sh});
    if prove {
        if layout_ok {
            let d_pk = engine.device().transport_pk_to_device(&pk);
            let t = Instant::now();
            let res =
                engine.prove(&d_pk, ProvingContext::new(ctxs.into_iter().enumerate().collect()));
            let prove_s = t.elapsed().as_secs_f64();
            match res {
                Ok(proof) => {
                    let ok = engine.verify(&vk, &proof);
                    let lh: Vec<Option<usize>> = proof
                        .trace_vdata
                        .iter()
                        .map(|v| v.as_ref().map(|v| v.log_height))
                        .collect();
                    out["prove"] = json!({"prove_s": prove_s, "verify_ok": ok.is_ok(),
                                          "verify_err": ok.err().map(|e| format!("{e:?}")),
                                          "proof_log_heights": lh});
                }
                Err(e) => {
                    out["prove"] = json!({"prove_s": prove_s, "prove_err": format!("{e:?}")})
                }
            }
        } else {
            out["prove"] = json!({"skipped": "stacked layout exceeds the internal set"});
        }
    }
    out
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let log_rows: usize = a[1].parse().unwrap();
    let range: u32 = a[2].parse().unwrap();
    let arities: Vec<usize> = a[3].split(',').map(|s| s.parse().unwrap()).collect();
    let prove_arity: usize = a[4].parse().unwrap();
    let gate: usize = a[5].parse().unwrap();
    let t0 = Instant::now();
    let nmax = *arities.iter().max().unwrap();
    let (vk, proofs, leaf) = leaf_proofs(nmax, log_rows, range);
    let mut so = std::io::stdout();
    writeln!(so, "{}", serde_json::to_string(&leaf).unwrap()).unwrap();
    so.flush().unwrap();
    for &n in &arities {
        let p = internal_ud();
        let pr = n == prove_arity;
        let ps = &proofs[..n];
        let mut out = match n {
            1 => node_run::<1>(p, &vk, ps, pr),
            2 => node_run::<2>(p, &vk, ps, pr),
            3 => node_run::<3>(p, &vk, ps, pr),
            4 => node_run::<4>(p, &vk, ps, pr),
            5 => node_run::<5>(p, &vk, ps, pr),
            6 => node_run::<6>(p, &vk, ps, pr),
            7 => node_run::<7>(p, &vk, ps, pr),
            8 => node_run::<8>(p, &vk, ps, pr),
            12 => node_run::<12>(p, &vk, ps, pr),
            15 => node_run::<15>(p, &vk, ps, pr),
            16 => node_run::<16>(p, &vk, ps, pr),
            32 => node_run::<32>(p, &vk, ps, pr),
            _ => panic!("arity {n} not instantiated"),
        };
        out["range"] = json!(range);
        out["w1_log_rows"] = json!(log_rows);
        out["elapsed_s"] = json!(t0.elapsed().as_secs_f64());
        let over = out["shape"]["max_height"].as_u64().unwrap() as usize > gate;
        out["over_gate"] = json!(over);
        writeln!(so, "{}", serde_json::to_string(&out).unwrap()).unwrap();
        so.flush().unwrap();
        eprintln!("node arity {n} done {:.1}s over_gate={over}", t0.elapsed().as_secs_f64());
        if over {
            eprintln!("STOP: arity {n} exceeds the gate {gate}");
            break;
        }
    }
}
