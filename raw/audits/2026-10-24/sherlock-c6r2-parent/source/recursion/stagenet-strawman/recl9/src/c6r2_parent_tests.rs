//! c6r2 parent test SOURCE. NEW_PROPOSED_NOT_ADOPTED_UNEXECUTED: uncompiled and
//! UNRUN; no result is claimed. Three classes are kept apart:
//! H host only (not ignored): exact host-mirror refusal variants, an independent
//!   raw-index output vector, decoding refusals that stop before vk_from_bytes,
//!   tag distinctness and shape arithmetic. No key, proof, SRS or circuit is used.
//! S native SEMANTIC (ignored, caller k): a test-only harness assigns two plain
//!   frame+19 statements and calls the SAME production constrain_c6r2_parent_lanes.
//!   Its public vector is the unchecked projection of the (mutated) pair, so only a
//!   parent equation can reject. SynthesisReject or Panic is a harness mismatch,
//!   never a rejection. The harness is NOT the verified relation.
//! V native RELATION (ignored): genuine caller artifacts only, and none exist.
//!   Shape, carry and proof cases are kernel or deferred-pairing tests, not parent law.
//! Census mutants run off-root (m = 3, levels 1, 2, 4, 5), so E7 cannot also fire.
//! E7 runs at level m for m = 1 (leaf-frame children), 2 and 3. Leaf IR, H*,
//! RuleIdC census, witness producer, W and D tests belong to their own changes.
//! All values are arbitrary test data, not roots, chains or keysets.
use super::*;
use std::{
    collections::BTreeMap,
    fs,
    io::BufReader,
    panic::{catch_unwind, AssertUnwindSafe},
    path::{Path, PathBuf},
};

use midnight_curves::Bls12;
use midnight_proofs::{
    dev::MockProver,
    poly::kzg::params::{ParamsKZG, ParamsVerifierKZG},
    utils::SerdeFormat,
};
use midnight_zk_stdlib::MidnightCircuit;

use crate::{
    inner::{point_pi_len, reconstruct_acc},
    sherlock_link::{DOM_KEYS, DOM_PROG, DOM_STATE, DOM_STMT, DOM_TRACE},
    util::{f_from_hex, sha256_hex},
};

// H: host only.

/// Census range-root level. NON_ROOT never contains it.
const M: u8 = 3;
const NON_ROOT: [u8; 4] = [1, 2, 4, 5];
const LT: Side = Side::Left;
const RT: Side = Side::Right;

/// Arbitrary test law. keyset_k is not derived from a key; only the relation
/// enforces that derivation.
fn law_at(level: u8, m: u8) -> C6R2ParentLaw {
    let child_keyset = if level == 1 { F::ZERO } else { F::from(8000u64 + level as u64) };
    C6R2ParentLaw { level, range_root_level: m, keyset_k: F::from(9001u64), child_keyset }
}

#[derive(Clone, Copy, Debug)]
enum End {
    Cut,
    Final,
}

fn base(law: &C6R2ParentLaw) -> [F; C6R2_STMT_LANES] {
    let mut c = [F::ZERO; C6R2_STMT_LANES];
    c[SCHEMA] = c6r2_schema();
    c[ROLE] = F::from(law.child_role() as u64);
    c[LEVEL] = F::from(law.child_level() as u64);
    c[KEYSET] = law.child_keyset;
    c[PROG] = F::from(41u64);
    c[ADMIT] = F::from(42u64);
    c
}

/// A valid framed pair at EVERY level, the range root included:
/// R.TX_HI - L.TX_LO == 4, and exactly one of R.POST_AT_TX_ENTRY or R.HAS_FINAL
/// (Cut ends at a tx-entry state, Final ends at the final leaf).
fn valid(law: &C6R2ParentLaw, end: End) -> (Vec<F>, Vec<F>) {
    let (mut l, mut r) = (base(law), base(law));
    for (lo, hi, a, b, c) in [
        (PRE_MACHINE, POST_MACHINE, 100u128, 101, 102),
        (TX_LO, TX_HI, 8, 10, 12),
        (MICRO_LO, MICRO_HI, 1000, 1 << 100, u128::MAX - 3),
        (FUEL_PRE, FUEL_POST, 5000, 4990, 4985),
    ] {
        l[lo] = F::from_u128(a);
        l[hi] = F::from_u128(b);
        r[lo] = F::from_u128(b);
        r[hi] = F::from_u128(c);
    }
    for (i, a, b) in [(CONSUMED, 10u64, 5u64), (HELPER_WORK, 3, 2)] {
        l[i] = F::from(a);
        r[i] = F::from(b);
    }
    l[POST_AT_TX_ENTRY] = F::ONE;
    let (post_at, has_final, class) = match end {
        End::Cut => (1u64, 0u64, 0u64),
        End::Final => (0, 1, 1),
    };
    r[POST_AT_TX_ENTRY] = F::from(post_at);
    r[HAS_FINAL] = F::from(has_final);
    r[TERMINAL_CLASS] = F::from(class);
    let frame = vec![F::ZERO; law.child_frame()];
    ([frame.clone(), l.to_vec()].concat(), [frame, r.to_vec()].concat())
}

/// The agreed OUTPUT, transcribed with raw lane indices independently of the table.
fn expected(law: &C6R2ParentLaw, l: &[F], r: &[F]) -> Vec<F> {
    let f = if law.level == 1 { 1 } else { 0 };
    let (l, r) = (&l[f..], &r[f..]);
    vec![
        dom("C6R2.STMT"), F::from(2u64), F::from(law.level as u64), law.keyset_k,
        l[4], l[5], l[6], r[7], l[8], r[9], l[10], r[11], l[12], r[13],
        l[14] + r[14], l[15] + r[15], r[16], r[17], r[18],
    ]
}

struct Mutant {
    name: String,
    /// (side, ABSOLUTE index into the framed statement, value).
    edits: Vec<(Side, usize, F)>,
    want: C6R2ParentRefusal,
}

/// Applies the edits and reports whether any lane value actually changed.
fn apply(m: &Mutant, l: &mut [F], r: &mut [F]) -> bool {
    let mut changed = false;
    for &(side, i, v) in &m.edits {
        let c: &mut [F] = if side == LT { &mut *l } else { &mut *r };
        changed |= c[i] != v;
        c[i] = v;
    }
    changed
}

/// Off-root single-lane mutants of valid(law, Final). Each breaks exactly the named
/// equation. Range mutants keep the difference or sum check satisfied, and use only
/// output-only lanes where continuity would otherwise also fire (L.FUEL_PRE, not
/// R.FUEL_PRE). L.HAS_FINAL has no isolating bool mutant because E5 fires.
fn census(law: &C6R2ParentLaw) -> Vec<Mutant> {
    use C6R2ParentRefusal as E;
    let (l, r) = valid(law, End::Final);
    let f = law.child_frame();
    let get = |s: Side, i: usize| if s == LT { l[f + i] } else { r[f + i] };
    let mut v = Vec::new();
    if f == 1 {
        for s in [LT, RT] {
            v.push(Mutant { name: format!("{s:?}_frame"), edits: vec![(s, 0, F::ONE)], want: E::Frame(s) });
        }
    }
    let mut push = |name: String, s: Side, i: usize, x: F, want: E| v.push(Mutant { name, edits: vec![(s, f + i, x)], want });
    for s in [LT, RT] {
        for (i, want) in [(SCHEMA, E::Schema(s)), (ROLE, E::Role(s)), (LEVEL, E::Level(s)), (KEYSET, E::Keyset(s))] {
            push(format!("{s:?}_lane{i}"), s, i, get(s, i) + F::ONE, want);
        }
        for &(lo, hi, _, name) in &ORDERED {
            let (i, x) = if s == LT { (lo, get(s, hi) + F::ONE) } else { (hi, get(s, lo) - F::ONE) };
            push(format!("{s:?}_{name}_lo_above_hi"), s, i, x, E::Order(s, name));
        }
    }
    let two32 = F::from(1u64 << 32);
    let two128 = F::from_u128(u128::MAX) + F::ONE;
    for (s, i, x, want) in [
        (LT, TX_LO, -F::ONE, E::Range(LT, "tx")),
        (RT, TX_HI, two32, E::Range(RT, "tx")),
        (LT, MICRO_LO, -F::ONE, E::Range(LT, "micro")),
        (RT, MICRO_HI, two128, E::Range(RT, "micro")),
        (LT, FUEL_PRE, two128, E::Range(LT, "fuel_pre")),
        (RT, FUEL_POST, two128, E::Range(RT, "fuel_post")),
        (LT, CONSUMED, -F::ONE, E::Range(LT, "consumed")),
        (RT, CONSUMED, -F::ONE, E::Range(RT, "consumed")),
        (LT, HELPER_WORK, -F::ONE, E::Range(LT, "helper_work")),
        (RT, HELPER_WORK, -F::ONE, E::Range(RT, "helper_work")),
        (LT, POST_AT_TX_ENTRY, F::from(2u64), E::NotBool(LT, "post_at_tx_entry")),
        (RT, POST_AT_TX_ENTRY, F::from(2u64), E::NotBool(RT, "post_at_tx_entry")),
        (RT, HAS_FINAL, F::from(2u64), E::NotBool(RT, "has_final")),
        (RT, PROG, get(RT, PROG) + F::ONE, E::Shared("prog")),
        (RT, ADMIT, get(RT, ADMIT) + F::ONE, E::Shared("admit")),
        (RT, PRE_MACHINE, get(RT, PRE_MACHINE) + F::ONE, E::Discontinuity("machine")),
        (RT, TX_LO, get(RT, TX_LO) + F::ONE, E::Discontinuity("tx")),
        (RT, MICRO_LO, get(RT, MICRO_LO) + F::ONE, E::Discontinuity("micro")),
        (RT, FUEL_PRE, get(RT, FUEL_PRE) + F::ONE, E::Discontinuity("fuel")),
        (LT, HAS_FINAL, F::ONE, E::FinalNotRightmost),
        (LT, TERMINAL_CLASS, F::ONE, E::TerminalBeforeEnd),
        (LT, CONSUMED, F::from_u128(u128::MAX), E::Overflow("consumed")),
        (LT, HELPER_WORK, F::from_u128(u128::MAX), E::Overflow("helper_work")),
    ] {
        push(format!("{s:?}_lane{i}_{want:?}"), s, i, x, want);
    }
    v
}

/// E7 mutants, each paired with the valid end it mutates.
fn root_mutants(law: &C6R2ParentLaw) -> Vec<(End, Mutant)> {
    use C6R2ParentRefusal as E;
    let f = law.child_frame();
    let (l, r) = valid(law, End::Cut);
    let one = |name: &str, s: Side, i: usize, x: F, want: E| Mutant { name: name.into(), edits: vec![(s, f + i, x)], want };
    vec![
        (End::Cut, one("count_3", RT, TX_HI, r[f + TX_HI] - F::ONE, E::RangeRootCount)),
        (End::Cut, one("count_5", RT, TX_HI, r[f + TX_HI] + F::ONE, E::RangeRootCount)),
        (End::Cut, one("count_3_left", LT, TX_LO, l[f + TX_LO] + F::ONE, E::RangeRootCount)),
        (End::Cut, one("boundary_neither", RT, POST_AT_TX_ENTRY, F::ZERO, E::RangeRootBoundary)),
        (End::Final, one("boundary_both", RT, POST_AT_TX_ENTRY, F::ONE, E::RangeRootBoundary)),
    ]
}

#[test]
fn host_valid_pairs_accept_with_the_independent_raw_index_vector() {
    for (m, level) in [(1u8, 1u8), (1, 2), (1, 3), (2, 2), (3, 1), (3, 2), (3, 3), (3, 4), (3, 5)] {
        for end in [End::Cut, End::Final] {
            let law = law_at(level, m);
            let (l, r) = valid(&law, end);
            assert_eq!(l.len(), if level == 1 { 20 } else { 19 });
            let got = c6r2_parent_law(&law, &l, &r).unwrap_or_else(|e| panic!("m {m} level {level} {end:?}: {e:?}"));
            assert_eq!(got.to_vec(), expected(&law, &l, &r), "m {m} level {level} {end:?}");
            assert_eq!(project(&law, &l, &r), got);
        }
    }
}

#[test]
fn host_fixture_preconditions_hold() {
    for level in [1u8, 2] {
        let law = law_at(level, M);
        let f = law.child_frame();
        let (l, r) = valid(&law, End::Final);
        let (l, r) = (&l[f..], &r[f..]);
        // Every aliased, non-shared output lane tells L apart from R.
        for i in [6usize, 7, 8, 9, 10, 11, 12, 13, 16, 17, 18] {
            assert_ne!(l[i], r[i], "level {level} lane {i}");
        }
        for i in [14usize, 15] {
            assert!(l[i] != F::ZERO && r[i] != F::ZERO, "both summands of lane {i} nonzero");
        }
        // Isolating range mutants need R.TX_LO >= 1, R.MICRO_LO >= 1, L.MICRO_HI < u128::MAX.
        assert!(le_width(&r[8], 32).unwrap() >= 1 && le_width(&r[10], 128).unwrap() >= 1);
        assert!(le_width(&l[11], 128).unwrap() < u128::MAX);
    }
}

#[test]
fn host_census_refuses_each_isolated_mutant_with_its_exact_variant_off_root() {
    for level in NON_ROOT {
        let law = law_at(level, M);
        assert!(!law.is_range_root());
        let ms = census(&law);
        assert_eq!(ms.len(), if level == 1 { 37 } else { 35 }, "census at level {level}");
        for m in ms {
            let (mut l, mut r) = valid(&law, End::Final);
            assert!(apply(&m, &mut l, &mut r), "{} must change a lane", m.name);
            assert_eq!(c6r2_parent_law(&law, &l, &r), Err(m.want), "{} at level {level}", m.name);
        }
    }
}

#[test]
fn host_range_root_law_at_m1_with_leaf_frames_and_at_m2_m3() {
    for m in [1u8, 2, 3] {
        let law = law_at(m, m);
        assert!(law.is_range_root());
        for end in [End::Cut, End::Final] {
            let (l, r) = valid(&law, end);
            assert!(c6r2_parent_law(&law, &l, &r).is_ok(), "m {m} {end:?}");
        }
        let ms = root_mutants(&law);
        assert_eq!(ms.len(), 5);
        for (end, mu) in ms {
            let (mut l, mut r) = valid(&law, end);
            assert!(apply(&mu, &mut l, &mut r), "{} must change a lane", mu.name);
            assert_eq!(c6r2_parent_law(&law, &l, &r), Err(mu.want), "{} at m = level = {m}", mu.name);
        }
    }
}

#[test]
fn host_range_root_mutants_are_accepted_off_root() {
    for m in [1u8, 2, 3] {
        for level in [m - 1, m + 1, m + 2] {
            if level == 0 {
                continue;
            }
            let law = law_at(level, m);
            for (end, mu) in root_mutants(&law) {
                let (mut l, mut r) = valid(&law, end);
                apply(&mu, &mut l, &mut r);
                assert!(c6r2_parent_law(&law, &l, &r).is_ok(), "{} at m {m} level {level}", mu.name);
            }
        }
    }
}

#[test]
fn host_shape_and_parameter_refusals() {
    use C6R2ParentRefusal as E;
    let (l1, r1) = valid(&law_at(1, M), End::Final);
    assert_eq!(c6r2_parent_law(&law_at(1, M), &l1[1..], &r1), Err(E::Shape));
    assert_eq!(c6r2_parent_law(&law_at(1, M), &l1, &[r1.clone(), vec![F::ZERO]].concat()), Err(E::Shape));
    let (l2, r2) = valid(&law_at(2, M), End::Final);
    assert_eq!(c6r2_parent_law(&law_at(2, M), &l1, &r2), Err(E::Shape));
    for bad in [law_at(0, M), law_at(1, 0), law_at(1, 254), law_at(1, 255), law_at(4, 1), law_at(6, 3)] {
        assert!(bad.check().is_err(), "{bad:?}");
        assert_eq!(c6r2_parent_law(&bad, &l2, &r2), Err(E::Parameters));
    }
    assert!(C6R2ParentLaw { child_keyset: F::ONE, ..law_at(1, M) }.check().is_err());
    assert!(law_at(255, 253).check().is_ok() && law_at(1, 253).check().is_ok() && law_at(5, 3).check().is_ok());
    let (a, b) = (law_at(1, M), law_at(2, M));
    assert_eq!((a.child_frame(), b.child_frame()), (1, 0));
    assert_eq!((a.child_kind(), b.child_kind()), (ChildKind::Leaf, ChildKind::Recursive));
    assert_eq!((a.child_role(), b.child_role()), (C6R2_ROLE_LEAF, C6R2_ROLE_PARENT));
    assert_eq!((a.child_level(), b.child_level()), (0, 1));
    assert!(law_at(M, M).is_range_root() && !law_at(M + 1, M).is_range_root());
}

#[test]
fn host_widths_are_exact_at_the_boundaries() {
    assert_eq!(le_width(&F::from_u128(u128::MAX), 128), Some(u128::MAX));
    assert_eq!(le_width(&(F::from_u128(u128::MAX) + F::ONE), 128), None);
    assert_eq!(le_width(&-F::ONE, 128), None);
    assert_eq!(le_width(&F::from(u32::MAX as u64), 32), Some(u32::MAX as u128));
    assert_eq!(le_width(&F::from(1u64 << 32), 32), None);
}

/// Shape arithmetic only. The W relation and D validate are later changes.
#[test]
fn host_shapes_are_20_and_31_with_the_w_tail_arithmetic() {
    assert_eq!(law_at(1, M).child_shape(), ChildShape { statement_lanes: 20, kind: ChildKind::Leaf });
    assert_eq!(law_at(1, M).child_shape().instance_lanes(), Ok(20));
    assert_eq!(law_at(2, M).child_shape(), ChildShape { statement_lanes: 19, kind: ChildKind::Recursive });
    assert_eq!(law_at(2, M).child_shape().instance_lanes(), Ok(31));
    assert_eq!((acc_pi_len(), c6r2_public_len()), (12, 31));
    assert_eq!(24 - acc_pi_len(), 12);
}

/// Covers exactly the listed domains. Decider dom_stmt values pinned by digest are
/// not in supplied source and are an open item.
#[test]
fn host_c6r2_tags_are_disjoint_from_the_listed_domains() {
    let mut all: Vec<(String, F)> = crate::i1::fp::dom::all().into_iter().map(|(n, f)| (n.to_string(), f)).collect();
    all.push(("DOM_IN3".into(), crate::i1::fp::field_from_tag("DOM_IN3")));
    for t in [DOM_KEYS, DOM_PROG, DOM_STATE, DOM_TRACE, DOM_STMT] {
        all.push((t.into(), dom(t)));
    }
    // The two tags defined here, then five reserved by the unadopted design.
    for t in [C6R2_SCHEMA_TAG, C6R2_KEYSET_TAG, "C6R2.PROG", "C6R2.ADMIT", "C6R2.MSTATE", "C6R2.W", "C6R2.DSTMT"] {
        assert!(t.len() <= 16 && t.starts_with("C6R2."));
        all.push((t.into(), dom(t)));
    }
    for i in 0..all.len() {
        for j in i + 1..all.len() {
            assert_ne!(all[i].1, all[j].1, "{} vs {}", all[i].0, all[j].0);
        }
    }
}

fn repr32(x: F) -> [u8; 32] {
    let mut b = [0u8; 32];
    b.copy_from_slice(x.to_repr().as_ref());
    b
}

fn header(level: u8, m: u8, kind: u8, lanes: u32, frame: u8, keyset: [u8; 32], child_keyset: F) -> Vec<u8> {
    let mut v = MAGIC.to_vec();
    v.extend([level, m, kind]);
    v.extend(lanes.to_le_bytes());
    v.push(frame);
    v.extend(keyset);
    write_f(&mut v, &child_keyset).unwrap();
    v
}

/// arch || k || nb_public_inputs || VERSION || inner k: the prefix that
/// check_child_vk_frame reads. Nothing after it is ever reached in these tests.
fn key_frame(k: u8, count: u32, version: u8, inner_k: u8) -> Vec<u8> {
    let mut v = Vec::new();
    ZkStdLibArch::default().write(&mut v).unwrap();
    v.push(k);
    v.extend(count.to_le_bytes());
    v.extend([version, inner_k]);
    v
}

fn with_key(mut h: Vec<u8>, key: &[u8]) -> Vec<u8> {
    write_bytes(&mut h, key).unwrap();
    h
}

fn refuse(b: &[u8]) -> io::Error {
    C6R2ParentRelation::read_relation(&mut &b[..]).err().expect("decoding must refuse")
}

#[track_caller]
fn refused_with(b: &[u8], needle: &str) {
    let e = refuse(b);
    assert!(e.to_string().contains(needle), "{e} lacks {needle}");
}

#[test]
fn host_decoding_refuses_before_any_key_is_deserialized() {
    let k9 = repr32(F::from(9u64));
    let p1 = header(1, M, 0, 19, 1, k9, F::ZERO);
    let p2 = header(2, M, 1, 19, 0, k9, F::from(7u64));
    refused_with(b"C6R1P001", "not a c6r2 parent relation");
    assert_eq!(refuse(MAGIC).kind(), io::ErrorKind::UnexpectedEof);
    assert_eq!(refuse(&p1).kind(), io::ErrorKind::UnexpectedEof);
    // Parameters, mode, semantic lane count (19, never the framing 20) and frame byte.
    refused_with(&header(0, M, 0, 19, 1, k9, F::ZERO), "level must be >= 1");
    for m in [0u8, 254, 255] {
        refused_with(&header(1, m, 0, 19, 1, k9, F::ZERO), "1..=253");
    }
    refused_with(&header(4, 1, 1, 19, 0, k9, F::ONE), "m + 2");
    refused_with(&header(1, M, 2, 19, 1, k9, F::ZERO), "kind byte");
    refused_with(&header(1, M, 1, 19, 1, k9, F::ZERO), "child mode");
    refused_with(&header(2, M, 0, 19, 0, k9, F::ONE), "child mode");
    refused_with(&header(1, M, 0, 19, 1, k9, F::ONE), "present-zero");
    for lanes in [18u32, 20, 31, 34] {
        refused_with(&header(1, M, 0, lanes, 1, k9, F::ZERO), "lane count");
    }
    refused_with(&header(1, M, 0, 19, 0, k9, F::ZERO), "frame byte");
    refused_with(&header(1, M, 0, 19, 2, k9, F::ZERO), "frame byte");
    refused_with(&header(2, M, 1, 19, 1, k9, F::ONE), "frame byte");
    refused_with(&header(1, M, 0, 19, 1, [0xff; 32], F::ZERO), "non-canonical");
    // Length bound, then the key frame (S2, S3, inner header) before vk_from_bytes.
    let mut huge = p1.clone();
    huge.extend(u32::MAX.to_le_bytes());
    refused_with(&huge, "exceeds 1048576");
    let mut arch = Vec::new();
    ZkStdLibArch::default().write(&mut arch).unwrap();
    refused_with(&with_key(p1.clone(), &arch), "truncated");
    refused_with(&with_key(p1.clone(), &key_frame(10, 20, 3, 10)[..arch.len() + 6]), "truncated");
    for k in [0u8, 1, 26, 255] {
        refused_with(&with_key(p1.clone(), &key_frame(k, 20, 3, k)), "degree byte outside");
    }
    for (h, count) in [(&p1, 19u32), (&p1, 31), (&p2, 20), (&p2, 19)] {
        refused_with(&with_key(h.clone(), &key_frame(10, count, 3, 10)), "public-input count");
    }
    refused_with(&with_key(p1.clone(), &key_frame(10, 20, 2, 10)), "inner version");
    refused_with(&with_key(p2.clone(), &key_frame(10, 31, 3, 11)), "inner degree");
}

/// S1 at function level only: no relation exists without a genuine key. validate,
/// and therefore new and write_relation, runs this same check on vk_bytes(child_vk).
#[test]
fn host_child_key_byte_bound_is_enforced_by_the_shared_frame_check() {
    let e = check_child_vk_frame(&vec![0u8; MAX_CHILD_VK_BYTES + 1], 20).unwrap_err();
    assert!(e.to_string().contains("byte bound"), "{e}");
}

// S and V: native, ignored.

#[derive(Debug, PartialEq, Eq)]
enum Verdict {
    Accept,
    ConstraintReject,
    SynthesisReject,
    Panic,
}

fn caller_k() -> u32 {
    let k: u32 = std::env::var("C6R2_PARENT_MOCK_K")
        .expect("C6R2_PARENT_MOCK_K: caller-supplied MidnightCircuit k; this source chooses none")
        .parse()
        .expect("C6R2_PARENT_MOCK_K must be a u32");
    assert!((2..=256).contains(&k), "k - 1 must neither be zero nor truncate as u8");
    k
}

/// Accept and ConstraintReject are verify() verdicts on a constructed prover.
fn mock<R: Relation<Instance = Vec<F>>>(rel: &R, pis: Vec<F>, w: R::Witness, k: u32) -> Verdict {
    let circuit = MidnightCircuit::new(rel, Value::known(pis.clone()), Value::known(w), Some(k));
    match catch_unwind(AssertUnwindSafe(|| MockProver::run(&circuit, vec![vec![], pis.clone()]))) {
        Err(_) => Verdict::Panic,
        Ok(Err(_)) => Verdict::SynthesisReject,
        Ok(Ok(p)) => match catch_unwind(AssertUnwindSafe(|| p.verify())) {
            Ok(Ok(())) => Verdict::Accept,
            Ok(Err(_)) => Verdict::ConstraintReject,
            Err(_) => Verdict::Panic,
        },
    }
}

/// TEST-ONLY harness: not an accept-all relation and not the verified parent.
/// It assigns two framed statements, calls the production parent law and exposes
/// the SAME returned cells. It never serializes.
#[derive(Clone, Debug)]
struct SemanticHarness {
    law: C6R2ParentLaw,
    arch: ZkStdLibArch,
    lanes: usize,
}

impl SemanticHarness {
    fn new(law: C6R2ParentLaw, arch: ZkStdLibArch) -> Self {
        Self { law, arch, lanes: law.child_statement_lanes() }
    }
}

impl Relation for SemanticHarness {
    type Instance = Vec<F>;
    type Witness = (Vec<F>, Vec<F>);
    type Error = Error;

    fn format_instance(i: &Vec<F>) -> Result<Vec<F>, Error> {
        Ok(i.clone())
    }

    fn circuit(&self, s: &ZkStdLib, l: &mut impl Layouter<F>, _i: Value<Vec<F>>, w: Value<(Vec<F>, Vec<F>)>) -> Result<(), Error> {
        let left: Vec<A> = s.assign_many(l, &w.as_ref().map(|p| p.0.clone()).transpose_vec(self.lanes))?;
        let right: Vec<A> = s.assign_many(l, &w.as_ref().map(|p| p.1.clone()).transpose_vec(self.lanes))?;
        for x in &constrain_c6r2_parent_lanes(s, l, &self.law, &left, &right)? {
            s.constrain_as_public_input(l, x)?;
        }
        Ok(())
    }

    fn used_chips(&self) -> ZkStdLibArch {
        self.arch.clone()
    }

    fn write_relation<W: io::Write>(&self, _: &mut W) -> io::Result<()> {
        Err(io::Error::other("semantic test harness is not serializable"))
    }

    fn read_relation<R: io::Read>(_: &mut R) -> io::Result<Self> {
        Err(io::Error::other("semantic test harness is not serializable"))
    }
}

fn archs() -> [ZkStdLibArch; 2] {
    [ZkStdLibArch::default(), bls_arch(4)]
}

/// The public vector is the unchecked projection of the given pair.
fn semantic(law: C6R2ParentLaw, arch: ZkStdLibArch, l: Vec<F>, r: Vec<F>, k: u32) -> Verdict {
    let pis = project(&law, &l, &r).to_vec();
    mock(&SemanticHarness::new(law, arch), pis, (l, r), k)
}

#[test]
#[ignore = "native MockProver UNRUN; caller-supplied k; no resource or execution decision here"]
fn semantic_valid_pairs_accept() {
    let k = caller_k();
    for arch in archs() {
        for (m, level) in [(M, 1u8), (M, 2), (M, 3), (M, 4), (M, 5), (1, 1)] {
            for end in [End::Cut, End::Final] {
                let law = law_at(level, m);
                let (l, r) = valid(&law, end);
                assert_eq!(semantic(law, arch.clone(), l, r, k), Verdict::Accept, "m {m} level {level} {end:?}");
            }
        }
    }
}

#[test]
#[ignore = "native MockProver UNRUN; caller-supplied k; no resource or execution decision here"]
fn semantic_census_mutants_are_constraint_rejections_off_root() {
    let k = caller_k();
    let mut wrong = Vec::new();
    for arch in archs() {
        for level in NON_ROOT {
            let law = law_at(level, M);
            for mu in census(&law) {
                let (mut l, mut r) = valid(&law, End::Final);
                apply(&mu, &mut l, &mut r);
                let got = semantic(law, arch.clone(), l, r, k);
                if got != Verdict::ConstraintReject {
                    wrong.push(format!("bls {} level {level} {}: {got:?}", arch.bls12_381, mu.name));
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
#[ignore = "native MockProver UNRUN; caller-supplied k; no resource or execution decision here"]
fn semantic_range_root_law_rejects_only_at_level_m() {
    let k = caller_k();
    let mut wrong = Vec::new();
    for m in [1u8, 2, 3] {
        for level in [m - 1, m, m + 1] {
            if level == 0 {
                continue;
            }
            let law = law_at(level, m);
            let want = if level == m { Verdict::ConstraintReject } else { Verdict::Accept };
            for (end, mu) in root_mutants(&law) {
                let (mut l, mut r) = valid(&law, end);
                apply(&mu, &mut l, &mut r);
                let got = semantic(law, bls_arch(4), l, r, k);
                if got != want {
                    wrong.push(format!("m {m} level {level} {}: {got:?}", mu.name));
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

#[test]
#[ignore = "native MockProver UNRUN; caller-supplied k; no resource or execution decision here"]
fn semantic_every_public_parent_lane_is_bound_to_its_cell() {
    let k = caller_k();
    for level in [1u8, 2] {
        let law = law_at(level, M);
        let (l, r) = valid(&law, End::Final);
        let pis = project(&law, &l, &r).to_vec();
        for i in 0..C6R2_STMT_LANES {
            let mut bad = pis.clone();
            bad[i] += F::ONE;
            let got = mock(&SemanticHarness::new(law, bls_arch(4)), bad, (l.clone(), r.clone()), k);
            assert_eq!(got, Verdict::ConstraintReject, "level {level} public lane {i}");
        }
    }
}

/// E0 is a synthesis refusal (Error::Synthesis), not a constraint rejection.
#[test]
#[ignore = "native MockProver UNRUN; caller-supplied k; no resource or execution decision here"]
fn semantic_wrong_statement_length_is_a_synthesis_refusal() {
    let k = caller_k();
    let law = law_at(1, M);
    let (l, r) = valid(&law, End::Final);
    let h = SemanticHarness { law, arch: ZkStdLibArch::default(), lanes: C6R2_STMT_LANES };
    let got = mock(&h, vec![F::ZERO; C6R2_STMT_LANES], (l[1..].to_vec(), r[1..].to_vec()), k);
    assert_eq!(got, Verdict::SynthesisReject);
}

/// Caller-supplied GENUINE artifacts; nothing here is generated or guessed.
/// C6R2_PARENT_FIXTURE_DIR holds child.vk, plus child{i}.instance.json (64-char
/// little-endian hex lanes) and child{i}.proof for i in 0..C6R2_CHILD_COUNT,
/// adjacent and proved under that ONE key. C6R2_CHILD_VK_SHA256 is an independent
/// key pin. C6R2_PARENT_LEVEL, C6R2_RANGE_ROOT_LEVEL and C6R2_CHILD_KEYSET_HEX come
/// from the child relation's own keygen. C6R2_SRS_FILE and C6R2_SRS_SHA256 pin the SRS.
struct Fixture {
    rel: C6R2ParentRelation,
    raw_vk: Vec<u8>,
    children: Vec<(Vec<F>, Vec<u8>)>,
}

fn env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} must be set explicitly"))
}

fn read_child(dir: &Path, stem: &str) -> (Vec<F>, Vec<u8>) {
    let hexes: Vec<String> = serde_json::from_slice(&fs::read(dir.join(format!("{stem}.instance.json"))).unwrap()).unwrap();
    (hexes.iter().map(|h| f_from_hex(h).unwrap()).collect(), fs::read(dir.join(format!("{stem}.proof"))).unwrap())
}

fn fixture() -> Fixture {
    let dir = PathBuf::from(env("C6R2_PARENT_FIXTURE_DIR"));
    let raw = fs::read(dir.join("child.vk")).unwrap();
    assert_eq!(sha256_hex(&raw), env("C6R2_CHILD_VK_SHA256"), "independently pinned child key digest");
    let level: u8 = env("C6R2_PARENT_LEVEL").parse().unwrap();
    let m: u8 = env("C6R2_RANGE_ROOT_LEVEL").parse().unwrap();
    let child_keyset = f_from_hex(&env("C6R2_CHILD_KEYSET_HEX")).unwrap();
    let probe = C6R2ParentLaw { level, range_root_level: m, keyset_k: F::ZERO, child_keyset };
    let vk = decode_child_vk(&raw, probe.child_shape()).unwrap();
    let rel = C6R2ParentRelation::new(level, m, vk, child_keyset).unwrap();
    let count: usize = env("C6R2_CHILD_COUNT").parse().unwrap();
    let children = (0..count).map(|i| read_child(&dir, &format!("child{i}"))).collect();
    Fixture { rel, raw_vk: raw, children }
}

fn pair(fx: &Fixture, a: usize, b: usize) -> C6R2ParentWitness {
    let ((li, lp), (ri, rp)) = (&fx.children[a], &fx.children[b]);
    C6R2ParentWitness { left_instance: li.clone(), left_proof: lp.clone(), right_instance: ri.clone(), right_proof: rp.clone() }
}

fn verifier_params() -> ParamsVerifierKZG<Bls12> {
    let bytes = fs::read(PathBuf::from(env("C6R2_SRS_FILE"))).unwrap();
    assert_eq!(sha256_hex(&bytes), env("C6R2_SRS_SHA256"), "caller-pinned verifier SRS");
    ParamsKZG::<Bls12>::read_custom(&mut BufReader::new(&bytes[..]), SerdeFormat::RawBytesUnchecked)
        .unwrap()
        .verifier_params()
}

fn deferred(fx: &Fixture, w: &C6R2ParentWitness) -> midnight_circuits::verifier::Accumulator<crate::inner::S> {
    let children = fx.rel.fixed_children().unwrap();
    prepare_binary_deferred(&children, [&w.left_instance[..], &w.right_instance[..]], [&w.left_proof[..], &w.right_proof[..]])
        .unwrap()
}

#[test]
#[ignore = "needs genuine c6r2 child artifacts (none exist), a pinned SRS and caller k"]
fn relation_honest_adjacent_pair_accepts_binds_all_31_lanes_and_defers_pairing() {
    let (fx, vp, k) = (fixture(), verifier_params(), caller_k());
    let mut bytes = Vec::new();
    fx.rel.write_relation(&mut bytes).unwrap();
    let back = C6R2ParentRelation::read_relation(&mut &bytes[..]).unwrap();
    assert_eq!(
        (back.level, back.range_root_level, back.child_kind, back.keyset_k, back.child_keyset),
        (fx.rel.level, fx.rel.range_root_level, fx.rel.child_kind, fx.rel.keyset_k, fx.rel.child_keyset)
    );
    assert_eq!(vk_bytes(&back.child_vk), fx.raw_vk);
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(C6R2ParentRelation::read_relation(&mut &trailing[..]).is_err());
    let w = pair(&fx, 0, 1);
    let pis = fx.rel.instance(&w).expect("adjacent genuine children");
    assert_eq!(pis.len(), 31);
    assert_eq!(mock(&fx.rel, pis.clone(), w.clone(), k), Verdict::Accept);
    assert!(reconstruct_acc(&pis[C6R2_STMT_LANES..]).expect("collapsed tail").check(&vp, &BTreeMap::new()), "honest deferred pairing");
    // Genuine proofs and an unchanged witness: each rejection isolates public exposure.
    for i in 0..pis.len() {
        let mut bad = pis.clone();
        bad[i] += F::ONE;
        assert_eq!(mock(&fx.rel, bad, w.clone(), k), Verdict::ConstraintReject, "public lane {i}");
    }
}

/// Both proofs are genuine and their combined accumulator passes the pairing, so the
/// rejection comes from a parent composition equation, not from proof invalidity.
#[test]
#[ignore = "needs >= 3 genuine adjacent c6r2 children (none exist), a pinned SRS and caller k"]
fn relation_genuine_nonadjacent_or_swapped_children_violate_the_parent_law() {
    use C6R2ParentRefusal as E;
    let (fx, vp, k) = (fixture(), verifier_params(), caller_k());
    let law = fx.rel.law();
    let (f, n) = (law.child_frame(), law.child_statement_lanes());
    assert!(fx.children.len() >= 3, "needs three adjacent genuine children");
    // T3 precondition: the children chain, and their three pre-states differ.
    let pre = |i: usize| fx.children[i].0[f + PRE_MACHINE];
    for i in 0..2 {
        assert_eq!(fx.children[i].0[f + POST_MACHINE], pre(i + 1), "children {i} and {} adjacent", i + 1);
    }
    assert!(pre(0) != pre(1) && pre(0) != pre(2) && pre(1) != pre(2), "non-degenerate children");
    for (a, b) in [(0usize, 2usize), (1, 0)] {
        let w = pair(&fx, a, b);
        let refusal = c6r2_parent_law(&law, &w.left_instance[..n], &w.right_instance[..n]).expect_err("must break a composition law");
        assert!(!matches!(refusal, E::Parameters | E::Shape | E::Frame(_) | E::Schema(_) | E::Role(_) | E::Level(_) | E::Keyset(_)), "{refusal:?}");
        assert!(fx.rel.instance(&w).is_err());
        let acc = deferred(&fx, &w);
        assert!(acc.check(&vp, &BTreeMap::new()), "both child proofs genuine");
        let pis = [project(&law, &w.left_instance[..n], &w.right_instance[..n]).to_vec(), acc_pis(&acc)].concat();
        assert_eq!(mock(&fx.rel, pis, w, k), Verdict::ConstraintReject, "({a},{b}) {refusal:?}");
    }
}

/// E1 at P_1 with a genuine leaf: the proof and pairing hold, the frame law refuses.
#[test]
#[ignore = "needs a P_1 fixture plus a genuine leaf child_nonzero_binding (none exists), a pinned SRS and caller k"]
fn relation_p1_rejects_a_genuine_leaf_whose_binding_input_is_nonzero() {
    use C6R2ParentRefusal as E;
    let (fx, vp, k) = (fixture(), verifier_params(), caller_k());
    let law = fx.rel.law();
    assert_eq!(law.level, 1, "needs a P_1 fixture");
    let (bi, bp) = read_child(Path::new(&env("C6R2_PARENT_FIXTURE_DIR")), "child_nonzero_binding");
    assert_ne!(bi[0], F::ZERO, "precondition: binding_input != 0");
    assert_eq!(&bi[1..], &fx.children[0].0[1..], "precondition: the same 19 lanes as child0");
    let (ri, rp) = fx.children[1].clone();
    let w = C6R2ParentWitness { left_instance: bi, left_proof: bp, right_instance: ri, right_proof: rp };
    assert_eq!(c6r2_parent_law(&law, &w.left_instance, &w.right_instance), Err(E::Frame(Side::Left)));
    let acc = deferred(&fx, &w);
    assert!(acc.check(&vp, &BTreeMap::new()), "genuine leaf proofs");
    let pis = [project(&law, &w.left_instance, &w.right_instance).to_vec(), acc_pis(&acc)].concat();
    assert_eq!(mock(&fx.rel, pis, w, k), Verdict::ConstraintReject);
}

/// Shape, S3 and kernel carry refusals. None of these is a parent-equation test.
#[test]
#[ignore = "needs genuine c6r2 child artifacts (none exist) and caller k"]
fn relation_shape_s3_and_kernel_refusals_are_not_parent_equations() {
    let (fx, k) = (fixture(), caller_k());
    let law = fx.rel.law();
    let w = pair(&fx, 0, 1);
    let honest = fx.rel.instance(&w).unwrap();
    // S3: the same key under the other child mode has the wrong public-input count.
    let other = if law.level == 1 { 2 } else { 1 };
    let other_keyset = if other == 1 { F::ZERO } else { F::ONE };
    assert!(C6R2ParentRelation::new(other, law.range_root_level, fx.rel.child_vk.clone(), other_keyset).is_err());
    let mut short = w.clone();
    short.left_instance.pop();
    assert!(fx.rel.instance(&short).is_err());
    assert_eq!(mock(&fx.rel, honest.clone(), short, k), Verdict::SynthesisReject);
    let mut empty = w.clone();
    empty.right_proof.clear();
    assert!(fx.rel.instance(&empty).is_err());
    assert_eq!(mock(&fx.rel, honest.clone(), empty, k), Verdict::SynthesisReject);
    let mut bad = w.clone();
    if law.level == 1 {
        // A leaf presented with a fabricated 12-lane tail: refused by ChildShape.
        bad.left_instance.extend_from_slice(&honest[C6R2_STMT_LANES..]);
    } else {
        // Kernel carry decoding: a carried tail whose lhs scalar is not one.
        bad.left_instance[C6R2_STMT_LANES + point_pi_len()] = F::from(2u64);
    }
    assert!(fx.rel.instance(&bad).is_err());
    assert_eq!(mock(&fx.rel, honest, bad, k), Verdict::SynthesisReject);
}

/// NOT a parent-equation test. PRE_MACHINE is output-only for the left child, so the
/// forged statement is refused only by the deferred pairing (terminal W and D).
#[test]
#[ignore = "needs genuine c6r2 child artifacts (none exist), a pinned SRS and caller k"]
fn relation_forged_output_only_lane_is_refused_only_by_the_deferred_pairing() {
    let (fx, vp, k) = (fixture(), verifier_params(), caller_k());
    let law = fx.rel.law();
    let (f, n) = (law.child_frame(), law.child_statement_lanes());
    let mut forged = pair(&fx, 0, 1);
    forged.left_instance[f + PRE_MACHINE] += F::ONE;
    assert!(c6r2_parent_law(&law, &forged.left_instance[..n], &forged.right_instance[..n]).is_ok());
    match fx.rel.instance(&forged) {
        Ok(pis) => {
            let verdict = mock(&fx.rel, pis.clone(), forged, k);
            eprintln!("forged PRE_MACHINE: MockProver verdict {verdict:?} (reported, not asserted)");
            let acc = reconstruct_acc(&pis[C6R2_STMT_LANES..]);
            assert!(acc.is_none_or(|a| !a.check(&vp, &BTreeMap::new())), "deferred pairing must refuse");
        }
        Err(e) => eprintln!("forged PRE_MACHINE refused before accumulation: {e}"),
    }
}

/// Key bytes that differ from the pin are refused, or give a different K_k.
#[test]
#[ignore = "needs a genuine c6r2 child key (none exists)"]
fn relation_child_key_bytes_differing_from_the_pin_are_refused() {
    let fx = fixture();
    let mut flipped = fx.raw_vk.clone();
    let last = flipped.len() - 1;
    flipped[last] ^= 1;
    assert_ne!(sha256_hex(&flipped), env("C6R2_CHILD_VK_SHA256"));
    if let Ok(vk) = decode_child_vk(&flipped, fx.rel.child_shape()) {
        assert_ne!(c6r2_keyset(fx.rel.level, fx.rel.range_root_level, &vk), fx.rel.keyset_k, "a different key gives a different K_k");
    }
}
