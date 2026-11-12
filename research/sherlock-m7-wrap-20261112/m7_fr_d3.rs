//! M7: D3 calculator terms of an Fr-base STARK L_Fr (option E), copy of the D10 helpers with the field swapped.
//! Reuses the D3 extension of the calculator (calculate_extended). Every printed number is
//! the calculator's own output.
//! Run: cargo test -p openvm-stark-backend --release --test d10_d3 -- --nocapture --test-threads=1

use openvm_stark_backend::{soundness::*, ProximityRegime, SystemParams, WhirProximityStrategy};
use openvm_stark_sdk::config::params_with_100_bits_security;

// L_Fr: base field BLS12-381 Fr, no extension (challenge field = Fr).
const LOG2_R: f64 = 254.8570894130472;
fn base_field_order() -> f64 { 2f64.powf(LOG2_R) }
fn challenge_field_bits() -> f64 { LOG2_R }

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
    println!("\n=== {name} ===");
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

fn mk(b: usize, w: usize, q: &[usize], qpow: usize) -> (SystemParams, WhirSoundnessExtension) {
    let mut p = params_with_100_bits_security(b, 2, 18, w, 0, 0, WhirProximityStrategy::UniqueDecoding, 4, qpow, 4);
    set_queries(&mut p, q);
    p.logup.pow_bits = 0;
    let nf = p.whir.rounds.len() * p.whir.k;
    (p, WhirSoundnessExtension { folding_pow_bits_per_fold: Some(vec![0; nf]), num_ood_samples: 1, radius_r: None })
}

/// L_Fr at the OD-19 query set with every PoW 0 except query PoW 20, then the smallest queries
/// per round (query term >= 101) per log blowup, at heights 2^20 and 2^22, w_stack 18 and 48.
#[test]
fn m7_fr_d3() {
    for h in [20usize, 22] {
        for w in [18usize, 48] {
            let (p, e) = mk(3, w, &[98, 83, 82], 20);
            print(&format!("L_Fr UD b3 98/83/82 PoW 0 except query 20, h 2^{h}, w_stack {w}"), &p, h, &e);
            for b in 1..=4usize {
                let (p0, _) = mk(b, w, &[1, 1, 1], 20);
                let nr = p0.whir.rounds.len();
                let mut q = vec![1usize; nr];
                for r in 0..nr {
                    for n in 1..400 {
                        q[r] = n;
                        let (p, e) = mk(b, w, &q, 20);
                        if calc(&p, h, &e).whir_details.rounds[r].query_bits >= G { break; }
                    }
                }
                let (p, e) = mk(b, w, &q, 20);
                print(&format!("L_Fr UD b{b} min queries {q:?}, h 2^{h}, w_stack {w}"), &p, h, &e);
            }
        }
    }
}
