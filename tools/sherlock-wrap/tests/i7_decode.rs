//! I7: the E3 map (the D-9 decoder) over `Alg`. Native verdict equal to the stock verdict on
//! every exported case (`lfr-e3-decode.json`, written by `lfr_e3`), the honest statement equal
//! to the stock statement; then the gadget decoder alone: cost model and `MockProver` on the
//! honest case, a `PUB` mismatch and a non-canonical lane.
use std::{fs, io};

use midnight_circuits::instructions::{AssignmentInstructions, PublicInputInstructions};
use midnight_proofs::{
    circuit::{Layouter, Value},
    dev::MockProver,
    plonk::Error,
};
use midnight_zk_stdlib::{MidnightCircuit, Relation, ZkStdLib, ZkStdLibArch, cost_model};
use serde_json::Value as J;
use sherlock_wrap::{
    alg::{Gadget, Native},
    decode::{DecodeShape, Dims, decode},
    native::{F, U_LFR_TRANSCRIPT, f_from_hex},
    smoke::wrap_arch,
    transcript::Transcript,
    wrapin::fixture,
};

struct Case {
    name: String,
    words: Vec<F>,
    ok: bool,
    statement: Option<Vec<F>>,
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

#[derive(Clone, Debug)]
struct DecodeRelation {
    sh: DecodeShape,
}

impl Relation for DecodeRelation {
    type Instance = Vec<F>;
    type Witness = Vec<F>;
    type Error = Error;
    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }
    fn circuit(&self, std: &ZkStdLib, l: &mut impl Layouter<F>, _: Value<Vec<F>>, w: Value<Vec<F>>) -> Result<(), Error> {
        let n = self.sh.d.n_public();
        let mut pw = vec![];
        for i in 0..n {
            pw.push(std.assign(l, w.as_ref().map(|w| w[i]))?);
        }
        let s = {
            let mut a = Gadget::new(std, &mut *l);
            let mut t = Transcript::new(&mut a, U_LFR_TRANSCRIPT)?;
            t.observe_all(&mut a, &pw)?;
            decode(&mut a, &mut t, &self.sh, &pw)?
        };
        for c in &s {
            std.constrain_as_public_input(l, c)?;
        }
        Ok(())
    }
    fn used_chips(&self) -> ZkStdLibArch {
        wrap_arch(1)
    }
    fn write_relation<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"decode")
    }
    fn read_relation<R: io::Read>(_: &mut R) -> io::Result<Self> {
        Err(io::Error::other("decode"))
    }
}

#[test]
fn i7_decode_native_equals_stock() {
    let (d, cs) = cases();
    let honest = cs.iter().find(|c| c.name == "honest").unwrap();
    let sh = DecodeShape { d, pre_hash: honest.words[0] };
    for c in &cs {
        let (fails, s) = native(&sh, &c.words);
        let ok = fails.is_empty();
        println!("E5DECODE {}: stock {} native {} {:?}", c.name, c.ok, ok, fails.iter().take(3).collect::<Vec<_>>());
        assert_eq!(ok, c.ok, "verdict of {}", c.name);
        if let Some(st) = &c.statement {
            assert_eq!(&s, st, "statement of {}", c.name);
        }
    }
    println!("E5DECODE cases {} all equal", cs.len());
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
    for name in ["pub_lane_changed", "lane_not_canonical"] {
        let c = cs.iter().find(|c| c.name == name).unwrap();
        assert!(!c.ok);
        let (_, s) = native(&sh, &c.words);
        assert!(!run(&c.words, &s), "{name} refused");
        println!("E5DECODE mock {name} refused");
    }
    println!("E5DECODE mock honest ok, changed statement refused");
}
