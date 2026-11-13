//! I9: the ENC wrap-side vectors (`enc-fp3` VECTORS.txt, 55 pinned values, and the FP-3
//! collection values in `fp3-closure/VALUES.txt`) on the wrap's own code.
//!
//! Part A, Fr vectors. They run on the `Alg` gadgets of the wrap: `merkle::hash_slice` /
//! `compress` (the Midnight sponge, use codes 0, 1, 2, 4), `permute`, the `pack` of BabyBear
//! words, `decode::dec` (9 x 30-bit limbs with `Canon`) and the `L_Fr` transcript (u = 7).
//! The ENC-2 log entries are replayed from their pinned limbs: `DIGEST(0)`, `H2`,
//! `SPONGE(3)`, `PERM`, `PUB(1)`, `DIGEST(2)`. Each result is checked against its pinned
//! value, natively and in one `MockProver` relation; a changed pinned value is refused.
//!
//! Part B, `pv` binding and lane map. `enc_wrap` (openvm `sherlock-wrap-export`) builds `L`
//! words whose `pv` is the pinned ENC-2 log and checks its `P2` fingerprint against the
//! pinned one. Its lane map is non-empty: lane 2 takes `C_P` from `DIGEST(0)`, lane 3
//! `input_pub` from `DIGEST(2)`, and lane 1 carries `PUB(1)`. At E3 9539418 the stock decoder
//! also checks `R` (plan D-10), and the ENC-2 log is not an `R` log (entry 1 is `H2`): stock
//! refuses every case at `R entry 1`, and so must the wrap. The lane map is then checked
//! against the stock lane-map step alone (`apply_lane_map`): its verdict and statement equal
//! the wrap's lane-map refusals and statement. Every ENC-2 entry is replayed: the honest log
//! replays, and a changed `DIGEST` output is refused. `MockProver` refuses the honest vector.
use std::{collections::HashMap, fs, io};

use ff::{Field, PrimeField};
use midnight_proofs::{
    circuit::{Layouter, Value},
    dev::MockProver,
    plonk::Error,
};
use midnight_zk_stdlib::{MidnightCircuit, Relation, ZkStdLib, ZkStdLibArch};
use serde_json::Value as J;
use sherlock_wrap::{
    alg::{Alg, Gadget, Native},
    decode::{DecodeShape, Dims, LIMBS, dec, decode, r_minus_1_limbs},
    decode_test::DecodeRelation,
    merkle::{compress, hash_slice},
    native::{F, U_LFR_TRANSCRIPT, cap_var, f_from_hex},
    smoke::wrap_arch,
    transcript::Transcript,
    wrapin::fixture,
};

const BB_P: u64 = 0x7800_0001;

fn enc_dir() -> String {
    format!("{}/Projects/Moriarty-wt/tested/research/sherlock-enc-fp3-20261112", std::env::var("HOME").unwrap())
}

fn pins(path: &str) -> HashMap<String, String> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| l.split_once(" = ").map(|(k, v)| (k.trim().to_string(), v.trim().to_string())))
        .collect()
}

fn nums(s: &str) -> Vec<u64> {
    s.trim_start_matches('[').trim_end_matches(']').split(',').map(|x| x.trim().parse().unwrap()).collect()
}

fn hx(s: &str) -> F {
    f_from_hex(s.trim().trim_matches('"')).unwrap()
}

fn hxs(s: &str) -> Vec<F> {
    s.trim_start_matches('[').trim_end_matches(']').split(',').map(hx).collect()
}

/// ENC-1 rule P-1: ASCII name, zero padded to 16 bytes, little-endian `u128`.
fn tag_fr(name: &str) -> F {
    let mut b = [0u8; 16];
    b[..name.len()].copy_from_slice(name.as_bytes());
    F::from_u128(u128::from_le_bytes(b))
}

/// The pinned inputs and expected values of part A.
#[derive(Clone, Debug)]
struct Pins {
    p: HashMap<String, String>,
    c: HashMap<String, String>,
}

impl Pins {
    fn load() -> Self {
        let p = pins(&format!("{}/VECTORS.txt", enc_dir()));
        assert_eq!(p.len(), 55, "55 pinned vectors");
        let c = pins(&format!("{}/fp3-closure/VALUES.txt", enc_dir()));
        Pins { p, c }
    }
}

/// One checked vector: name, computed cell, pinned value.
type Check<V> = (String, V, F);

fn pack<A: Alg>(a: &mut A, ws: &[u64]) -> Result<A::V, Error> {
    let mut t = vec![];
    for (i, &w) in ws.iter().enumerate() {
        let c = a.wit(Value::known(F::from(w)))?;
        a.range(&c, &num_bigint::BigUint::from(BB_P), "BabyBear word")?;
        t.push((F::from(2u64).pow_vartime([31 * i as u64]), c));
    }
    a.lin(&t, F::ZERO)
}

fn digest<A: Alg>(a: &mut A, tag: &str, x: &A::V) -> Result<A::V, Error> {
    let t = a.cst(tag_fr(tag))?;
    hash_slice(a, 4, &[t, x.clone()])
}

/// The logged values of a pinned `enc(e)`: witnessed limbs, 30-bit range, `dec` with `Canon`.
fn enc_values<A: Alg>(a: &mut A, e: &[u64]) -> Result<Vec<A::V>, Error> {
    let r1 = r_minus_1_limbs();
    let mut out = vec![];
    for v in e[2..].chunks(LIMBS) {
        let mut ls = vec![];
        for &l in v {
            let c = a.wit(Value::known(F::from(l)))?;
            a.range(&c, &(num_bigint::BigUint::from(1u8) << 30), "limb range")?;
            ls.push(c);
        }
        out.push(dec(a, &ls, &r1)?);
    }
    Ok(out)
}

fn vectors<A: Alg>(a: &mut A, pin: &Pins) -> Result<Vec<Check<A::V>>, Error> {
    let p = &pin.p;
    let mut ck: Vec<Check<A::V>> = vec![];
    // ENC-1 tag values (key constants of the wrap).
    for (i, n) in ["SHK_PROGC", "SHK_LOCK", "SHK_INPUT", "SHK_OUTCOME"].iter().enumerate() {
        let c = a.cst(tag_fr(n))?;
        ck.push((format!("enc1.tag[{i}].{n}"), c, hx(&p[&format!("enc1.tag[{i}].{n}")])));
    }
    // ENC-1 public digests: PoseidonFr_4(tagFr, pack(d2)).
    for (name, d2, tag) in [
        ("enc1.C_P", "enc1.d2_progc", "SHK_PROGC"),
        ("enc1.A_1", "enc1.d2_lock_j1", "SHK_LOCK"),
        ("enc1.input_pub", "enc1.input_d", "SHK_INPUT"),
        ("enc1.outcome_pub", "enc1.outhide_d2", "SHK_OUTCOME"),
    ] {
        let x = pack(a, &nums(&p[d2]))?;
        let y = digest(a, tag, &x)?;
        ck.push((name.into(), y, hx(&p[name])));
    }
    for n in ["SHK_PROGC", "SHK_LOCK", "SHK_INPUT", "SHK_OUTCOME"] {
        let x = pack(a, &[1, 2, 3, 4, 5, 6, 7, 8])?;
        let y = digest(a, n, &x)?;
        let k = format!("enc1.digest_1to8.{n}");
        ck.push((k.clone(), y, hx(&p[&k])));
    }
    // u = 0: H2(3, 5).
    let (x3, x5, x7) = (a.cst(F::from(3u64))?, a.cst(F::from(5u64))?, a.cst(F::from(7u64))?);
    let h = hash_slice(a, 0, &[x3.clone(), x5.clone()])?;
    ck.push(("u0.h2(3,5)".into(), h, hx(&p["u0.h2(3,5)"])));
    // ENC-2 log replay from the pinned limbs.
    let e: Vec<Vec<u64>> = (0..6).map(|i| nums(&p[&format!("enc2.enc[{i}]")])).collect();
    let v0 = enc_values(a, &e[0])?;
    let x = pack(a, &nums(&p["enc1.d2_progc"]))?;
    let y = digest(a, "SHK_PROGC", &v0[0])?;
    a.assert_eq(&v0[0], &x, "DIGEST(0) d2")?;
    a.assert_eq(&v0[1], &y, "DIGEST(0) replay")?;
    ck.push(("enc2.enc[0] DIGEST(0) output = C_P".into(), v0[1].clone(), hx(&p["enc1.C_P"])));
    let v1 = enc_values(a, &e[1])?;
    let h = hash_slice(a, 0, &[v1[0].clone(), v1[1].clone()])?;
    a.assert_eq(&v1[2], &h, "H2 replay")?;
    ck.push(("enc2.enc[1] H2 output = u0.h2(3,5)".into(), v1[2].clone(), hx(&p["u0.h2(3,5)"])));
    let v2 = enc_values(a, &e[2])?;
    let s3 = hash_slice(a, 0, &[v2[0].clone(), v2[1].clone(), v2[2].clone()])?;
    a.assert_eq(&v2[3], &s3, "SPONGE(3) replay")?;
    let s3c = hash_slice(a, 0, &[x3.clone(), x5.clone(), x7.clone()])?;
    a.assert_eq(&v2[3], &s3c, "SPONGE(3) of (3, 5, 7)")?;
    ck.push(("enc2.enc[2] SPONGE(3) inputs".into(), v2[2].clone(), F::from(7u64)));
    let v3 = enc_values(a, &e[3])?;
    let out = a.permute(&[v3[0].clone(), v3[1].clone(), v3[2].clone()])?;
    for k in 0..3 {
        a.assert_eq(&v3[3 + k], &out[k], "PERM replay")?;
    }
    ck.push(("enc2.enc[3] PERM input 2".into(), v3[2].clone(), F::from(7u64)));
    let v4 = enc_values(a, &e[4])?;
    ck.push(("enc2.enc[4] PUB(1) value".into(), v4[0].clone(), F::from(0x1234u64)));
    let v5 = enc_values(a, &e[5])?;
    let x = pack(a, &nums(&p["enc1.input_d"]))?;
    a.assert_eq(&v5[0], &x, "DIGEST(2) d2")?;
    let y = digest(a, "SHK_INPUT", &v5[0])?;
    a.assert_eq(&v5[1], &y, "DIGEST(2) replay")?;
    ck.push(("enc2.enc[5] DIGEST(2) output = input_pub".into(), v5[1].clone(), hx(&p["enc1.input_pub"])));
    // Canon: limbs(r - 1) decode to r - 1.
    let r1 = nums(&p["enc2.limbs(r-1)"]);
    assert_eq!(r1, r_minus_1_limbs());
    let mut e_r1 = vec![0, 0];
    e_r1.extend(&r1);
    let vr = enc_values(a, &e_r1)?;
    ck.push(("enc2.limbs(r-1)".into(), vr[0].clone(), -F::ONE));
    // u = 1 leaf of 20 BabyBear words, u = 2 node (7, 9).
    let ws: Vec<u64> = (1..=20).collect();
    let mut packed = vec![];
    for c in ws.chunks(8) {
        packed.push(pack(a, c)?);
    }
    let lh = hash_slice(a, 1, &packed)?;
    ck.push(("g3.L.u1_leaf20".into(), lh, hx(&p["g3.L.u1_leaf20"])));
    let (n7, n9) = (a.cst(F::from(7u64))?, a.cst(F::from(9u64))?);
    let nh = compress(a, 2, &n7, &n9)?;
    ck.push(("g3.L.u2_node(7,9)".into(), nh, hx(&p["g3.L.u2_node(7,9)"])));
    // L_Fr transcript, u = 7.
    let t = Transcript::new(a, U_LFR_TRANSCRIPT)?;
    ck.push(("g3.Lfr.s2".into(), t.state[2].clone(), hx(&p["g3.Lfr.s2"])));
    assert_eq!(hx(&p["g3.Lfr.s2"]), cap_var(7));
    let mut t = Transcript::new(a, U_LFR_TRANSCRIPT)?;
    for (i, want) in hxs(&p["g3.Lfr.fresh3"]).into_iter().enumerate() {
        let s = t.sample(a)?;
        ck.push((format!("g3.Lfr.fresh3[{i}]"), s, want));
    }
    let mut t = Transcript::new(a, U_LFR_TRANSCRIPT)?;
    let o11 = a.cst(F::from(11u64))?;
    t.observe(a, &o11)?;
    let s = t.sample(a)?;
    ck.push(("g3.Lfr.obs1_sample".into(), s, hxs(&p["g3.Lfr.obs1_sample"])[0]));
    let mut t = Transcript::new(a, U_LFR_TRANSCRIPT)?;
    let o12 = a.cst(F::from(12u64))?;
    t.observe(a, &o11)?;
    t.observe(a, &o12)?;
    for (i, want) in hxs(&p["g3.Lfr.obs2_sample3"]).into_iter().enumerate() {
        let s = t.sample(a)?;
        ck.push((format!("g3.Lfr.obs2_sample3[{i}]"), s, want));
    }
    // FP-3 collection: claim_pub = SPONGE(3) (u = 0) over (C_P, input_pub, outcome_pub);
    // set_pub = PoseidonFr_4(SHK_SET, pack(set_root)); u = 0 parity on [1 .. n].
    let c = &pin.c;
    for i in 0..3 {
        let g = |k: &str| hx(&c[&format!("coll.run[{i}].{k}")]);
        let xs = [a.wit(Value::known(g("c_p")))?, a.wit(Value::known(g("input_pub")))?, a.wit(Value::known(g("outcome_pub")))?];
        let y = hash_slice(a, 0, &xs)?;
        ck.push((format!("coll.run[{i}].claim_pub"), y, g("claim_pub")));
    }
    let root = a.wit(Value::known(hx(&c["coll.set_root_pack"])))?;
    let y = digest(a, "SHK_SET", &root)?;
    ck.push(("coll.set_pub".into(), y, hx(&c["coll.set_pub"])));
    for n in [0usize, 1, 2, 3, 5, 16] {
        let mut xs = vec![];
        for k in 1..=n {
            xs.push(a.cst(F::from(k as u64))?);
        }
        let y = if n == 0 { a.cst(F::ZERO)? } else { hash_slice(a, 0, &xs)? };
        let k = format!("parity.u0[{n}]");
        ck.push((k.clone(), y, hx(&c[&k])));
    }
    Ok(ck)
}

#[derive(Clone, Debug)]
struct VecRelation {
    pin: Pins,
    /// Index of a pinned value to change (negative control).
    bad: Option<usize>,
}

impl Relation for VecRelation {
    type Instance = Vec<F>;
    type Witness = ();
    type Error = Error;
    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }
    fn circuit(&self, std: &ZkStdLib, l: &mut impl Layouter<F>, _: Value<Vec<F>>, _: Value<()>) -> Result<(), Error> {
        let mut a = Gadget::new(std, &mut *l);
        let ck = vectors(&mut a, &self.pin)?;
        for (i, (name, v, want)) in ck.iter().enumerate() {
            let w = if Some(i) == self.bad { *want + F::ONE } else { *want };
            a.assert_eq_c(v, w, name)?;
        }
        Ok(())
    }
    fn used_chips(&self) -> ZkStdLibArch {
        wrap_arch(1)
    }
    fn write_relation<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(b"enc-vectors")
    }
    fn read_relation<R: io::Read>(_: &mut R) -> io::Result<Self> {
        Err(io::Error::other("enc-vectors"))
    }
}

#[test]
fn i9_enc_fr_vectors() {
    let pin = Pins::load();
    let mut a = Native::default();
    let ck = vectors(&mut a, &pin).unwrap();
    assert!(a.failures.is_empty(), "native replay checks: {:?}", a.failures);
    let mut bad = 0;
    for (name, v, want) in &ck {
        let mut got = F::ZERO;
        v.map(|x| got = x);
        if got != *want {
            println!("E5ENC MISMATCH {name}");
            bad += 1;
        }
    }
    assert_eq!(bad, 0);
    let pinned: std::collections::BTreeSet<String> =
        ck.iter().map(|(n, ..)| n.split(['[', ' ']).next().unwrap().to_string()).collect();
    println!("E5ENC native: {} checks equal their pinned values ({} vector names); replay asserts hold", ck.len(), pinned.len());
    let k = 15;
    let run = |bad: Option<usize>| {
        let rel = VecRelation { pin: pin.clone(), bad };
        let c = MidnightCircuit::new(&rel, Value::known(vec![]), Value::known(()), Some(k));
        MockProver::<F>::run(&c, vec![vec![], vec![]]).unwrap().verify().is_ok()
    };
    assert!(run(None), "gadget vectors");
    let n = ck.len();
    for i in [0, 4, n / 2, n - 1] {
        assert!(!run(Some(i)), "changed pinned value {} refused", ck[i].0);
    }
    let mut na = Native::default();
    let mut t = vec![];
    let e5 = nums(&pin.p["enc2.enc[5]"]);
    let mut changed = e5.clone();
    changed[12] ^= 1;
    let _ = enc_values(&mut na, &changed).map(|v| t = v);
    let mut got = F::ZERO;
    t[1].map(|x| got = x);
    assert_ne!(got, hx(&pin.p["enc1.input_pub"]), "a changed logged limb changes the value");
    let mut r = r_minus_1_limbs();
    r[0] += 1;
    let mut er = vec![0, 0];
    er.extend(&r);
    let mut na = Native::default();
    enc_values(&mut na, &er).unwrap();
    assert!(!na.failures.is_empty(), "limbs of r refused (Canon)");
    let mut er = vec![0, 0, 1 << 30];
    er.extend([0; 8]);
    let mut na = Native::default();
    enc_values(&mut na, &er).unwrap();
    assert!(!na.failures.is_empty(), "limb 2^30 refused");
    println!("E5ENC gadget: MockProver OK on {n} checks (k {k}); 4 changed pinned values refused; Canon refuses limbs(r), the range refuses a 2^30 limb");
}

struct Case {
    name: String,
    words: Vec<F>,
    ok: bool,
    statement: Option<Vec<F>>,
    lane_map_ok: Option<bool>,
    lane_map_statement: Option<Vec<F>>,
}

fn hexes(v: &J) -> Vec<F> {
    v.as_array().unwrap().iter().map(|x| f_from_hex(x.as_str().unwrap()).unwrap()).collect()
}

fn native_decode(sh: &DecodeShape, pw: &[F]) -> (Vec<String>, Vec<F>) {
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
fn i9_enc_pv_lane_map() {
    let doc: J = serde_json::from_slice(&fs::read(fixture("enc-wrap-decode")).expect("run enc_wrap")).unwrap();
    let pin = Pins::load();
    for k in ["d_empty", "pv_head12", "pv_fingerprint", "lane2_c_p", "lane3_input_pub", "limbs_r_minus_1"] {
        assert!(doc["checks"][k].as_bool().unwrap(), "{k}");
    }
    let cs: Vec<Case> = doc["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| Case {
            name: c["name"].as_str().unwrap().into(),
            words: hexes(&c["words"]),
            ok: c["ok"].as_bool().unwrap(),
            statement: c["statement"].as_array().map(|_| hexes(&c["statement"])),
            lane_map_ok: c["lane_map_ok"].as_bool(),
            lane_map_statement: c["lane_map_statement"].as_array().map(|_| hexes(&c["lane_map_statement"])),
        })
        .collect();
    let honest = cs.iter().find(|c| c.name == "honest").unwrap();
    let d = Dims::FULL;
    let sh = DecodeShape { d, pre_hash: honest.words[0] };
    // pv binding: the packed words hold the pinned pv.
    let pv: Vec<u64> = doc["pv"].as_array().unwrap().iter().map(|x| x.as_u64().unwrap()).collect();
    let mut words = vec![];
    for (i, x) in honest.words[1..].iter().enumerate() {
        let k = (d.words() - 8 * i).min(8);
        let b = sherlock_wrap::native::to_big(x);
        for j in 0..k {
            let w: num_bigint::BigUint = (&b >> (31 * j)) % (num_bigint::BigUint::from(1u64) << 31);
            words.push(w.to_u64_digits().first().copied().unwrap_or(0));
        }
    }
    assert_eq!(words[114..d.call_at()].to_vec(), pv, "pv words");
    let lane_map_fail = |f: &Vec<String>| f.iter().any(|x| x.contains("lane map") && !x.contains("native replay"));
    let mut n_lm = 0;
    for c in &cs {
        let (fails, s) = native_decode(&sh, &c.words);
        let ok = fails.is_empty();
        println!("E5ENC decode {}: stock {} wrap {} {:?}", c.name, c.ok, ok, fails.iter().take(2).collect::<Vec<_>>());
        assert_eq!(ok, c.ok, "verdict of {}", c.name);
        assert!(!c.ok && c.statement.is_none() && fails.iter().any(|f| f == "R entry 1"), "{}: R entry 1", c.name);
        if let Some(lm) = c.lane_map_ok {
            assert_eq!(!lane_map_fail(&fails), lm, "lane-map step of {}", c.name);
            if let Some(st) = &c.lane_map_statement {
                assert_eq!(&s, st, "lane-map statement of {}", c.name);
            }
            n_lm += 1;
        }
    }
    let st = honest.lane_map_statement.clone().unwrap();
    assert_eq!(st[2], hx(&pin.p["enc1.C_P"]));
    assert_eq!(st[3], hx(&pin.p["enc1.input_pub"]));
    assert_eq!(st[1], F::from(0x1234u64));
    // Every entry of the ENC-2 log is replayed: the honest log replays (its only refusal is R);
    // a changed logged C_P is refused by the replay of DIGEST(0).
    let (fails, _) = native_decode(&sh, &honest.words);
    assert!(!fails.iter().any(|f| f.contains("native replay")), "the honest ENC-2 log replays");
    let dc = cs.iter().find(|c| c.name == "digest_output_changed").unwrap();
    let (fails, _) = native_decode(&sh, &dc.words);
    assert!(fails.iter().any(|f| f == "native replay"), "DIGEST replay");
    let k = 18;
    let rel = DecodeRelation { sh: sh.clone() };
    let (_, s) = native_decode(&sh, &honest.words);
    let c = MidnightCircuit::new(&rel, Value::known(s.clone()), Value::known(honest.words.clone()), Some(k));
    assert!(MockProver::<F>::run(&c, vec![vec![], s]).unwrap().verify().is_err(), "honest ENC refused (not an R log)");
    println!(
        "E5ENC decode: {} cases equal stock (all refused at R entry 1); lane-map step equal on {n_lm} cases; pv words equal the pinned pv; lane 2 = C_P, lane 3 = input_pub, lane 1 = 0x1234; the honest ENC-2 log replays, a changed C_P is refused by the replay; MockProver refuses the honest vector",
        cs.len()
    );
}
