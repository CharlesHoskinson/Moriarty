//! X-EXP 2026-11-12: D3 terms of the frozen internal set (OD-19) under the node shape bounds
//! before and after the owner ruling of 2026-11-12 20:35 (200 interactions per AIR, 64 AIRs,
//! 3,000 columns). Same calculator call as ud_sizing.rs, with the bounds as arguments.
//! Run: cargo test -p openvm-stark-backend --release --test x_exp_bounds -- --nocapture --test-threads=1

use openvm_stark_backend::{soundness::*, SystemParams, WhirProximityStrategy};
use openvm_stark_sdk::config::{base_field_order, challenge_field_bits, params_with_100_bits_security};

const G: f64 = 101.0;
const A: f64 = 100.0;

fn n_logup_bound(l_skip: usize, num_airs: usize, ipa: usize, max_log_height: usize, mic: usize) -> usize {
    let field_bound = (mic as f64).log2().ceil() as usize - l_skip;
    let param_bound = (num_airs as f64).log2().ceil() as usize + (ipa as f64).log2().ceil() as usize
        + max_log_height
        - l_skip;
    field_bound.min(param_bound)
}

fn calc(p: &SystemParams, h: usize, ext: &WhirSoundnessExtension, cons: usize, airs: usize, cols: usize, ipa: usize) -> SoundnessCalculator {
    let n_logup = n_logup_bound(p.l_skip, airs, ipa, h, p.logup.max_interaction_count as usize);
    SoundnessCalculator::calculate_extended(
        p, base_field_order(), challenge_field_bits(), cons, airs, p.max_constraint_degree, h, cols,
        p.w_stack, n_logup, ext,
    )
}

fn internal() -> SystemParams {
    let mut p = params_with_100_bits_security(3, 2, 17, 512, 0, 9, WhirProximityStrategy::UniqueDecoding, 4, 21, 4);
    for (r, q) in p.whir.rounds.iter_mut().zip([97usize, 82, 81]) {
        r.num_queries = q;
    }
    p.logup.pow_bits = 18;
    p
}

#[test]
fn x_exp_internal_bounds() {
    let ext = WhirSoundnessExtension { folding_pow_bits_per_fold: None, num_ood_samples: 1, radius_r: None };
    let h = 19;
    for (name, cons, airs, cols, ipa) in [
        ("frozen bounds 1000c/50airs/2000cols/100int", 1000usize, 50usize, 2000usize, 100usize),
        ("ruling bounds 1000c/64airs/3000cols/200int", 1000, 64, 3000, 200),
        ("x14 condition rows 1000c/64airs/3300cols/200int", 1000, 64, 3300, 200),
        ("x14 condition rows 1000c/64airs/3500cols/200int", 1000, 64, 3500, 200),
    ] {
        let mut p = internal();
        let s = calc(&p, h, &ext, cons, airs, cols, ipa);
        let w = &s.whir_details;
        let t = [
            ("logup", s.logup_bits, true), ("gkr_sumcheck", s.gkr_sumcheck_bits, false),
            ("gkr_batching", s.gkr_batching_bits, false), ("zerocheck", s.zerocheck_sumcheck_bits, false),
            ("constraint_batching", s.constraint_batching_bits, false), ("stacked", s.stacked_reduction_bits, false),
            ("mu", w.mu_batching_bits, true), ("fold_rbr", w.fold_rbr_bits, true), ("query", w.query_bits, true),
            ("shift_rbr", w.shift_rbr_bits, false), ("gamma", w.gamma_batching_bits, false), ("ood", w.ood_rbr_bits, false),
        ];
        println!("\n=== X-EXP internal set, {name} (h=2^{h}, n_logup={}) ===", n_logup_bound(p.l_skip, airs, ipa, h, p.logup.max_interaction_count as usize));
        println!("pow: folding={} mu={} query={} logup={}", p.whir.folding_pow_bits, p.whir.mu_pow_bits, p.whir.query_phase_pow_bits, p.logup.pow_bits);
        for (n, b, gr) in &t {
            let flag = if *gr && *b < G { "  BELOW 101 (grindable)" } else if *b < A { "  BELOW 100" } else { "" };
            println!("TERM {:20} {:.4}{}{}", n, b, if *gr { "  (grindable)" } else { "" }, flag);
        }
        println!("TERM TOTAL {:.4}", s.total_bits);
        let g = t.iter().filter(|x| x.2).all(|x| x.1 >= G);
        let a = t.iter().all(|x| x.1 >= A);
        println!("GATE grindable>=101: {}  every>=100: {}", if g { "PASS" } else { "FAIL" }, if a { "PASS" } else { "FAIL" });
        // report (do not adopt) the smallest PoW values that would restore each grindable term
        let rule = p.logup.pow_bits;
        while calc(&p, h, &ext, cons, airs, cols, ipa).logup_bits < G && p.logup.pow_bits < 30 {
            p.logup.pow_bits += 1;
        }
        println!("FIX logup PoW needed for >=101: {} (frozen {})", p.logup.pow_bits, rule);
        let mu0 = p.whir.mu_pow_bits;
        while calc(&p, h, &ext, cons, airs, cols, ipa).whir_details.mu_batching_bits < G && p.whir.mu_pow_bits < 30 {
            p.whir.mu_pow_bits += 1;
        }
        println!("FIX mu PoW needed for >=101: {} (frozen {})", p.whir.mu_pow_bits, mu0);
    }
}
