//! D10: D3 calculator terms of L at the measured w_stack (copy of the UD sizing helpers).
//! Reuses the D3 extension of the calculator (calculate_extended). Every printed number is
//! the calculator's own output.
//! Run: cargo test -p openvm-stark-backend --release --test d10_d3 -- --nocapture --test-threads=1

use openvm_stark_backend::{soundness::*, ProximityRegime, SystemParams, WhirProximityStrategy};
use openvm_stark_sdk::config::{base_field_order, challenge_field_bits, params_with_100_bits_security};

const G: f64 = 101.0; // grindable gate
const A: f64 = 100.0; // every-term gate

fn n_logup_bound(l_skip: usize, num_airs: usize, ipa: usize, max_log_height: usize, mic: usize) -> usize {
    let field_bound = (mic as f64).log2().ceil() as usize - l_skip;
    let param_bound = (num_airs as f64).log2().ceil() as usize + (ipa as f64).log2().ceil() as usize
        + max_log_height
        - l_skip;
    field_bound.min(param_bound)
}

// Recursion shape bounds (PROPOSAL 11.2): 1,000 constraints, 50 AIRs, 2,000 columns, 100 interactions.
fn calc(p: &SystemParams, h: usize, ext: &WhirSoundnessExtension) -> SoundnessCalculator {
    let n_logup = match std::env::var("D10_NLOGUP") { Ok(v) => v.parse().unwrap(), Err(_) => n_logup_bound(p.l_skip, 50, 100, h, p.logup.max_interaction_count as usize) };
    println!("n_logup used {n_logup} (bound {})", n_logup_bound(p.l_skip, 50, 100, h, p.logup.max_interaction_count as usize));
    SoundnessCalculator::calculate_extended(
        p, base_field_order(), challenge_field_bits(), 1000, 50, p.max_constraint_degree, h, 2000,
        p.w_stack, n_logup, ext,
    )
}

fn terms(s: &SoundnessCalculator) -> Vec<(&'static str, f64, bool)> {
    let w = &s.whir_details;
    vec![
        ("logup", s.logup_bits, true),
        ("gkr_sumcheck", s.gkr_sumcheck_bits, false),
        ("gkr_batching", s.gkr_batching_bits, false),
        ("zerocheck", s.zerocheck_sumcheck_bits, false),
        ("constraint_batching", s.constraint_batching_bits, false),
        ("stacked", s.stacked_reduction_bits, false),
        ("mu", w.mu_batching_bits, true),
        ("fold_rbr", w.fold_rbr_bits, true),
        ("query", w.query_bits, true),
        ("shift_rbr", w.shift_rbr_bits, false),
        ("gamma", w.gamma_batching_bits, false),
        ("ood", w.ood_rbr_bits, false),
    ]
}

fn gates(s: &SoundnessCalculator) -> (bool, bool) {
    let t = terms(s);
    // shift_rbr combines query (grindable) and gamma; gate it at 101 like query, as D3 does for query.
    let g = t.iter().filter(|x| x.2).all(|x| x.1 >= G);
    let a = t.iter().all(|x| x.1 >= A);
    (g, a)
}

fn print(name: &str, p: &SystemParams, h: usize, ext: &WhirSoundnessExtension) -> SoundnessCalculator {
    let s = calc(p, h, ext);
    println!("\n=== UD {name} ===");
    println!(
        "params: log_blowup={} l_skip={} n_stack={} w_stack={} k_whir={} proximity={:?} queries={:?} rounds={}",
        p.log_blowup, p.l_skip, p.n_stack, p.w_stack, p.whir.k, p.whir.proximity,
        p.whir.rounds.iter().map(|r| r.num_queries).collect::<Vec<_>>(), p.whir.rounds.len()
    );
    println!(
        "pow: folding={} per_fold={:?} mu={} query={} logup={} ood_samples={} radius_r={:?}",
        p.whir.folding_pow_bits, ext.folding_pow_bits_per_fold, p.whir.mu_pow_bits,
        p.whir.query_phase_pow_bits, p.logup.pow_bits, ext.num_ood_samples, ext.radius_r
    );
    let t = terms(&s);
    for (n, b, gr) in &t {
        println!("TERM {:20} {:.4}{}", n, b, if *gr { "  (grindable)" } else { "" });
    }
    let (wn, wb, _) = t.iter().cloned().fold(("", f64::INFINITY, false), |a, x| if x.1 < a.1 { x } else { a });
    println!("TERM TOTAL                 {:.4}  WEAKEST {} {:.4}", s.total_bits, wn, wb);
    for (i, r) in s.whir_details.rounds.iter().enumerate() {
        println!(
            "ROUND {i} rate=2^-{} q={} fold_pow={:?} log2L={:.4} next_log2L={:.4} prox={:.4} sumcheck={:.4} fold_rbr={:.4} query={:.4} gamma={:.4} shift={:.4} ood={}",
            r.log_inv_rate, r.num_queries, r.folding_pow_bits, r.log2_list_size, r.next_log2_list_size,
            r.min_proximity_gaps_bits, r.min_sumcheck_bits, r.min_fold_rbr_bits, r.query_bits,
            r.gamma_batching_bits, r.shift_rbr_bits,
            r.ood_bits.map_or("none".to_string(), |b| format!("{b:.4}"))
        );
    }
    let (g, a) = gates(&s);
    println!("GATE grindable>=101: {}  every>=100: {}", if g { "PASS" } else { "FAIL" }, if a { "PASS" } else { "FAIL" });
    s
}

fn set_queries(p: &mut SystemParams, q: &[usize]) {
    for (r, &n) in p.whir.rounds.iter_mut().zip(q) {
        r.num_queries = n;
    }
}


fn ws_list() -> Vec<usize> {
    std::env::var("D10_WS").unwrap_or("48".into()).split(',').map(|x| x.parse().unwrap()).collect()
}

/// OD-19 L set (UD-SIZING.md 5.1): UD, blowup 3, queries 98/83/82, fold PoW 0, mu PoW 6,
/// query PoW 20, LogUp PoW 18, 1 OOD sample.
#[test]
fn d10_l_ud() {
    for w in ws_list() {
        let mut p = params_with_100_bits_security(3, 2, 18, w, 0, 6, WhirProximityStrategy::UniqueDecoding, 4, 20, 4);
        set_queries(&mut p, &[98, 83, 82]);
        p.logup.pow_bits = 18;
        let nf = p.whir.rounds.len() * p.whir.k;
        let ext = WhirSoundnessExtension { folding_pow_bits_per_fold: Some(vec![0; nf]), num_ood_samples: 1, radius_r: None };
        print(&format!("D10 L UD b3 w_stack {w}"), &p, 20, &ext);
    }
}

/// Old set: PROPOSAL 3.4 LD set of L (control of UD sizing), mu PoW 22.
#[test]
fn d10_l_ld() {
    for w in ws_list() {
        let mut p = params_with_100_bits_security(4, 2, 18, w, 22, 22, WhirProximityStrategy::ListDecoding { m: 3 }, 4, 20, 4);
        set_queries(&mut p, &[52, 27, 18]);
        p.logup.pow_bits = 21;
        let ext = WhirSoundnessExtension {
            folding_pow_bits_per_fold: Some(vec![15, 14, 13, 12, 19, 18, 17, 16, 22, 21, 20, 19]),
            num_ood_samples: 2,
            radius_r: Some(3),
        };
        print(&format!("D10 L LD PROPOSAL 3.4 w_stack {w}"), &p, 20, &ext);
    }
}
