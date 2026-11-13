//! E6 item 2 (OD-18, owner decision 2026-11-12): the `L_Fr` transcript of the wrap runs on use
//! code u = 7 and reproduces the ENC G3 `L_Fr` vectors pinned on the stark-sdk side
//! (`research/sherlock-enc-fp3-20261112/VECTORS.txt`, `g3.Lfr.*`), on the native mirror and on
//! the generic circuit transcript (`transcript::Transcript` over the `Native` back end, the code
//! the `Gadget` back end runs; E5 MockProver showed the two back ends agree). The `L_Fr` Merkle codes are u = 5 (leaf) and 6 (node).
use midnight_proofs::circuit::Value;
use sherlock_wrap::{
    alg::Native,
    native::{F, NativeTranscript, U_LFR_LEAF, U_LFR_NODE, U_LFR_TRANSCRIPT, cap_var, f_to_hex},
    transcript::Transcript,
};

const S2: &str = "00000000000000000000000000000000000000000000001d0000000000000000";
const FRESH3: [&str; 3] = [
    "39860bcfe756854514595b63ce95d5c2c2fd4edfcb369b711ec14a1f76acda89",
    "6b65309e059b3d01d40576c9f1f962ea91bccb4e0583c469b1589c26555ec999",
    "21790135f388b123857958f8b61cd542ef96277f118fd0d9fe5c183c65b5b100",
];
const OBS1: &str = "71b1bed2cd66b39c069db59e7c827ac7626c61b928dcc3610d0773c366de355d";
const OBS2_3: [&str; 3] = [
    "08a4bccbdcfc1aeee6e31a3902b75e7ec0c744ee6a42ca2aa7dbab0c93caece8",
    "6e2e334d21a05391c7600b357ada7f1cd77bc56250914df86355863048101c19",
    "0669914356acda504ea5af22e1ed3c3905e69dffcf6305f7406f071bd05e60fa",
];

fn h(xs: &[F]) -> Vec<String> {
    xs.iter().map(f_to_hex).collect()
}

#[test]
fn od18_codes() {
    assert_eq!((U_LFR_LEAF, U_LFR_NODE, U_LFR_TRANSCRIPT), (5, 6, 7));
    assert_eq!(f_to_hex(&cap_var(U_LFR_TRANSCRIPT)), S2);
}

#[test]
fn od18_native_transcript_vectors() {
    let mut t = NativeTranscript::new(U_LFR_TRANSCRIPT);
    assert_eq!(h(&[t.sample(), t.sample(), t.sample()]), FRESH3);
    let mut t = NativeTranscript::new(U_LFR_TRANSCRIPT);
    t.observe(F::from(11u64));
    assert_eq!(h(&[t.sample()]), [OBS1]);
    let mut t = NativeTranscript::new(U_LFR_TRANSCRIPT);
    t.observe(F::from(11u64));
    t.observe(F::from(12u64));
    assert_eq!(h(&[t.sample(), t.sample(), t.sample()]), OBS2_3);
    // u = 3 (the code of L) gives another first squeeze.
    let mut t3 = NativeTranscript::new(3);
    assert_ne!(f_to_hex(&t3.sample()), FRESH3[0]);
}

#[test]
fn od18_circuit_transcript_vectors() {
    let get = |v: Value<F>| {
        let mut o = None;
        v.map(|x| o = Some(x));
        o.unwrap()
    };
    let mut a = Native::default();
    let mut t = Transcript::new(&mut a, U_LFR_TRANSCRIPT).unwrap();
    let s: Vec<F> = (0..3).map(|_| get(t.sample(&mut a).unwrap())).collect();
    assert_eq!(h(&s), FRESH3);
    let mut t = Transcript::new(&mut a, U_LFR_TRANSCRIPT).unwrap();
    for x in [11u64, 12] {
        t.observe(&mut a, &Value::known(F::from(x))).unwrap();
    }
    let s: Vec<F> = (0..3).map(|_| get(t.sample(&mut a).unwrap())).collect();
    assert_eq!(h(&s), OBS2_3);
    assert!(a.failures.is_empty());
}
