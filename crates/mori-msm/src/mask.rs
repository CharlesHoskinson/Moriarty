// SPDX-License-Identifier: Apache-2.0
//! The skew mask of the gpu-wrap integration, unchanged: MSM(s) = MSM(s + r) - MSM(r)
//! with the same fixed splitmix64 vector r and MSM(r) cached per base table and length.
use std::{collections::HashMap, sync::Mutex};

use midnight_curves::{Fq, G1Projective};

type Key = (usize, usize);

#[allow(clippy::type_complexity)]
static MASKS: Mutex<Option<HashMap<(Key, usize), (Vec<Fq>, G1Projective)>>> = Mutex::new(None);

/// The mask vector of `mori-gpu` (same seed and mixing, so the same r).
pub fn mask_vector(n: usize) -> Vec<Fq> {
    let mut x: u64 = 0x6d6f_7269_6770_7531;
    let mut next = || {
        x = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = x;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    };
    (0..n)
        .map(|_| Fq::from_raw([next(), next(), next(), next() >> 2]))
        .collect()
}

pub(crate) fn forget(key: Key) {
    if let Some(m) = MASKS.lock().unwrap().as_mut() {
        m.retain(|(k, _), _| *k != key);
    }
}

/// Masked MSM (the shipped path); `MORI_GPU_MSM_MASK=0` runs the raw MSM.
pub fn msm_g1(scalars: &[Fq], bases: &[G1Projective]) -> Option<G1Projective> {
    if std::env::var("MORI_GPU_MSM_MASK").as_deref() == Ok("0") {
        return crate::msm::msm_raw(scalars, bases);
    }
    let n = scalars.len();
    if n == 0 || n > bases.len() {
        return None;
    }
    // Check the table first, so a rebuild clears stale masks before they are read.
    crate::msm::with_table(bases, |_, _| ())?;
    let key = (bases.as_ptr() as usize, bases.len());
    let have = MASKS
        .lock()
        .unwrap()
        .as_ref()
        .is_some_and(|m| m.contains_key(&(key, n)));
    if !have {
        let r = mask_vector(n);
        let rr = crate::msm::msm_raw(&r, bases)?;
        MASKS
            .lock()
            .unwrap()
            .get_or_insert_with(HashMap::new)
            .insert((key, n), (r, rr));
    }
    let masked: Vec<Fq> = {
        let m = MASKS.lock().unwrap();
        let (r, _) = m.as_ref().unwrap().get(&(key, n)).unwrap();
        scalars.iter().zip(r).map(|(a, b)| *a + b).collect()
    };
    let sum = crate::msm::msm_raw(&masked, bases)?;
    let m = MASKS.lock().unwrap();
    let (_, rr) = m.as_ref().unwrap().get(&(key, n))?;
    Some(sum - rr)
}
