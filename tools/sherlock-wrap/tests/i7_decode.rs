//! I7: the E3 map (the D-9 decoder) over `Alg`. Native verdict equal to the stock verdict on
//! every exported case (`lfr-e3-decode.json`, written by `lfr_e3`), the honest statement equal
//! to the stock statement; then the gadget decoder alone: cost model and `MockProver` on the
//! honest case, a `PUB` mismatch, a non-canonical lane and the 9539418 lane-map and `R`
//! refusals. Cases named `open_replay_*` are the open divergence (stock replays every entry).
use std::fs;

use midnight_proofs::{circuit::Value, dev::MockProver};
use midnight_zk_stdlib::{MidnightCircuit, cost_model};
use serde_json::Value as J;
use sherlock_wrap::{
    alg::Native,
    decode::{DecodeShape, Dims, decode},
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
}

fn hexes(v: &J) -> Vec<F> {
    v.as_array().unwrap().iter().map(|x| f_from_hex(x.as_str().unwrap()).unwrap()).collect()
}

fn cases() -> (Dims, Vec<Case>) {
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
        })
        .collect();
    (d, cs)
}

fn native(sh: &DecodeShape, pw: &[F]) -> (Vec<String>, Vec<F>) {
    let mut a = Native::default();
    let mut t = Transcript::new(&mut a, U_LFR_TRANSCRIPT).unwrap();
    let pv: Vec<Value<F>> = pw.iter().map(|x| Value::known(*x)).collect();
    t.observe_all(&mut a, &pv).unwrap();
    let s = decode(&mut a, &mut t, sh, &pv).unwrap();
    let mut out = vec![];
    for x in s {
        x.map(|x| out.push(x));
    }
    (a.failures, out)
}

#[test]
fn i7_decode_native_equals_stock() {
    let (d, cs) = cases();
    let honest = cs.iter().find(|c| c.name == "honest").unwrap();
    let sh = DecodeShape { d, pre_hash: honest.words[0] };
    let mut open = 0;
    for c in &cs {
        let (fails, s) = native(&sh, &c.words);
        let ok = fails.is_empty();
        println!("E5DECODE {}: stock {} native {} {:?}", c.name, c.ok, ok, fails.iter().take(3).collect::<Vec<_>>());
        if c.name.starts_with("open_replay_") {
            // Open (E3 J6 in the wrap): stock replays every entry, the wrap replays R and the
            // mapped entries only. A changed output of another entry: stock refuses, wrap accepts.
            assert!(!c.ok && ok, "open divergence of {}", c.name);
            open += 1;
            continue;
        }
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
    assert!(open >= 1);
    println!("E5DECODE cases {} equal, {open} open (replay outside R and the lane map)", cs.len() - open);
}

#[test]
fn i7_decode_gadget_mock() {
    let (d, cs) = cases();
    let honest = cs.iter().find(|c| c.name == "honest").unwrap();
    let sh = DecodeShape { d, pre_hash: honest.words[0] };
    let rel = DecodeRelation { sh: sh.clone() };
    let k = 18;
    let m = cost_model(&rel, Some(k));
    println!("E5DECODE gadget rows {} (k {k})", m.rows);
    let st = honest.statement.clone().unwrap();
    let run = |w: &Vec<F>, pi: &Vec<F>| {
        let c = MidnightCircuit::new(&rel, Value::known(pi.clone()), Value::known(w.clone()), Some(k));
        MockProver::<F>::run(&c, vec![vec![], pi.clone()]).unwrap().verify().is_ok()
    };
    assert!(run(&honest.words, &st), "honest");
    let mut bad = st.clone();
    bad[1] += F::from(1u64);
    assert!(!run(&honest.words, &bad), "changed statement");
    for name in ["pub_lane_changed", "lane_not_canonical", "lane_map_sponge_not_3", "lane_map_twice", "lane_map_not_u16", "r_anchor_output_changed", "chunk_j_0"] {
        let c = cs.iter().find(|c| c.name == name).unwrap();
        assert!(!c.ok);
        let (_, s) = native(&sh, &c.words);
        assert!(!run(&c.words, &s), "{name} refused");
        println!("E5DECODE mock {name} refused");
    }
    println!("E5DECODE mock honest ok, changed statement refused");
}
