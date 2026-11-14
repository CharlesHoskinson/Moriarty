//! I7: the E3 map (the D-9 decoder) over `Alg`. Native verdict equal to the stock verdict on
//! every exported case (`lfr-e3-decode.json`, written by `lfr_e3`), the honest statement equal
//! to the stock statement; then the gadget decoder alone: cost model and `MockProver` on the
//! honest case, a `PUB` mismatch, a non-canonical lane and the 9539418 lane-map and `R`
//! refusals, and the replay of entries outside `R` and the lane map (`open_replay_*`, a
//! changed `PERM`, `SPONGE(5)` and `DIGEST(SHK_REG)` output). T9: the T4 checks (`X`,
//! `layout_d`, the N-11 prologue lanes, lanes 0 and 1 off the lane map, the opening-proof form of
//! `R`, a chunk's anchor write, the `PATH` replay with wrap-private siblings), each with its
//! named mutant.
use std::fs;

use midnight_proofs::{circuit::Value, dev::MockProver};
use midnight_zk_stdlib::{MidnightCircuit, cost_model};
use serde_json::Value as J;
use sherlock_wrap::{
    alg::Native,
    decode::{DecodeShape, Dims, decode_with},
    decode_test::DecodeRelation,
    native::{F, U_LFR_TRANSCRIPT, f_from_hex},
    transcript::Transcript,
    wrapin::fixture,
};

struct Case {
    name: String,
    words: Vec<F>,
    ok: bool,
    statement: Option<Vec<F>>,
    /// The stock lane-map step alone (`apply_lane_map`), when the case records it.
    lane_map_ok: Option<bool>,
    /// The wrap-private `PATH` siblings of the case's log.
    sibs: Vec<Vec<F>>,
}

fn hexes(v: &J) -> Vec<F> {
    v.as_array().unwrap().iter().map(|x| f_from_hex(x.as_str().unwrap()).unwrap()).collect()
}

fn cases() -> (Dims, Vec<Case>, J) {
    let doc: J = serde_json::from_slice(&fs::read(fixture("lfr-e3-decode")).expect("run lfr_e3 0")).unwrap();
    let dm = &doc["dims"];
    let d = Dims {
        b: dm["b"].as_u64().unwrap() as usize,
        v: dm["v"].as_u64().unwrap() as usize,
        p: dm["p"].as_u64().unwrap() as usize,
        l_tx: dm["l_tx"].as_u64().unwrap() as usize,
    };
    assert_eq!(d, Dims::FULL);
    let cs = doc["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| Case {
            name: c["name"].as_str().unwrap().into(),
            words: hexes(&c["words"]),
            ok: c["ok"].as_bool().unwrap(),
            statement: c["statement"].as_array().map(|_| hexes(&c["statement"])),
            lane_map_ok: c["lane_map_ok"].as_bool(),
            sibs: c["sibs"].as_array().map(|l| l.iter().map(hexes).collect()).unwrap_or_default(),
        })
        .collect();
    (d, cs, doc)
}

fn native(sh: &DecodeShape, c: &Case) -> (Vec<String>, Vec<F>) {
    let mut a = Native::default();
    let mut t = Transcript::new(&mut a, U_LFR_TRANSCRIPT).unwrap();
    let pv: Vec<Value<F>> = c.words.iter().map(|x| Value::known(*x)).collect();
    t.observe_all(&mut a, &pv).unwrap();
    let s = decode_with(&mut a, &mut t, sh, &pv, &Value::known(c.sibs.clone())).unwrap();
    let mut out = vec![];
    for x in s {
        x.map(|x| out.push(x));
    }
    (a.failures, out)
}

#[test]
fn i7_decode_native_equals_stock() {
    let (_, cs, doc) = cases();
    let honest = cs.iter().find(|c| c.name == "honest").unwrap();
    let sh = DecodeShape::from_doc(&doc, honest.words[0]);
    for c in &cs {
        let (fails, s) = native(&sh, c);
        let ok = fails.is_empty();
        println!("E5DECODE {}: stock {} native {} {:?}", c.name, c.ok, ok, fails.iter().take(3).collect::<Vec<_>>());
        assert_eq!(ok, c.ok, "verdict of {}", c.name);
        if let Some(st) = &c.statement {
            assert_eq!(&s, st, "statement of {}", c.name);
        }
        if let Some(lm) = c.lane_map_ok {
            let twice = fails.iter().any(|f| f.contains("lane mapped twice"));
            assert_eq!(!lm, twice, "lane-map step of {}", c.name);
            println!("E5DECODE {}: stock lane-map step {lm}, wrap lane mapped twice {twice}", c.name);
        }
    }
    println!("E5DECODE cases {} all equal", cs.len());
}

#[test]
fn i7_decode_gadget_mock() {
    let (_, cs, doc) = cases();
    let honest = cs.iter().find(|c| c.name == "honest").unwrap();
    let sh = DecodeShape::from_doc(&doc, honest.words[0]);
    let rel = DecodeRelation { sh: sh.clone() };
    let k = 18;
    let m = cost_model(&rel, Some(k));
    println!("E5DECODE gadget rows {} (k {k})", m.rows);
    let st = honest.statement.clone().unwrap();
    let run = |c: &Case, pi: &Vec<F>| {
        let w = (c.words.clone(), c.sibs.clone());
        let mc = MidnightCircuit::new(&rel, Value::known(pi.clone()), Value::known(w), Some(k));
        MockProver::<F>::run(&mc, vec![vec![], pi.clone()]).unwrap().verify().is_ok()
    };
    assert!(run(honest, &st), "honest");
    let mut bad = st.clone();
    bad[1] += F::from(1u64);
    assert!(!run(honest, &bad), "changed statement");
    // T9 positives: the opening-proof form, a chunk with its anchor write, a log with PATH entries.
    for name in ["open_honest", "chunk_honest", "path_honest"] {
        let c = cs.iter().find(|c| c.name == name).unwrap_or_else(|| panic!("case {name}"));
        assert!(c.ok, "{name}");
        assert!(run(c, c.statement.as_ref().unwrap()), "{name} accepted");
        println!("E5DECODE mock {name} accepted");
    }
    for name in [
        "pub_lane_changed",
        "lane_not_canonical",
        "lane_map_sponge_not_3",
        "lane_map_twice",
        "lane_map_not_u16",
        "r_anchor_output_changed",
        "chunk_j_0",
        "open_replay_perm",
        "open_replay_sponge",
        "open_replay_digest",
        // T9 named mutants (ARCHITECTURE kit L AND WRAP; T4 list)
        "wrap_x_not_cached_commitment",
        "wrap_x_not_canonical",
        "wrap_layout_d_other",
        "wrap_popeq_lane_not_a_in",
        "wrap_anchor_template_lane",
        "wrap_lane_map_lane_0",
        "wrap_lane_map_lane_1",
        "wrap_open_e_in_4",
        "wrap_chunk_a_out_not_written",
        "wrap_chunk_write_template",
        "wrap_path_root_changed",
        "wrap_path_sibling_changed",
        "wrap_path_index_changed",
    ] {
        let c = cs.iter().find(|c| c.name == name).unwrap_or_else(|| panic!("case {name}"));
        assert!(!c.ok, "{name} is a refusal");
        let (_, s) = native(&sh, c);
        assert!(!run(c, &s), "{name} refused");
        println!("E5DECODE mock {name} refused");
    }
    println!("E5DECODE mock honest ok, changed statement refused");
}
