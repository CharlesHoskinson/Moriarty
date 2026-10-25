// Adapted from pinned recl9 segment composition.
//! The STRAWMAN 16-field segment interface (rec/src/segment.rs without the
//! accumulator tail types, which mono does not have).
use super::{
    super::util::{f_from_hex, f_to_hex},
    i1::fp::{F, dom, hash},
};
use ff::{Field, PrimeField};
use midnight_proofs::plonk::Error as TreeError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Segment {
    pub fields: [F; 16],
}
impl Serialize for Segment {
    fn serialize<T: serde::Serializer>(&self, s: T) -> Result<T::Ok, T::Error> {
        self.fields
            .iter()
            .map(f_to_hex)
            .collect::<Vec<_>>()
            .serialize(s)
    }
}
impl<'de> Deserialize<'de> for Segment {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let strings = Vec::<String>::deserialize(d)?;
        let fields = strings
            .iter()
            .map(|s| f_from_hex(s))
            .collect::<Result<Vec<_>, _>>()
            .map_err(serde::de::Error::custom)?;
        Ok(Self {
            fields: fields
                .try_into()
                .map_err(|_| serde::de::Error::custom("16 fields required"))?,
        })
    }
}
impl Segment {
    pub fn pad(mut self) -> Self {
        self.fields = [F::ZERO; 16];
        self.fields[0] = F::ONE;
        self
    }
    pub fn validate(&self) -> Result<(), TreeError> {
        let f = &self.fields;
        if f[0] == F::ONE {
            if f[1..].iter().any(|x| *x != F::ZERO) {
                return Err(err("noncanonical pad"));
            }
            return Ok(());
        }
        if f[0] != F::ZERO || f[15] != F::ZERO {
            return Err(err("pad/reserved field"));
        }
        for i in [3, 4] {
            bounded(f[i], 80)?;
        }
        for i in [6, 7, 10, 11, 12, 13, 14] {
            bounded(f[i], 64)?;
        }
        if integer(f[3]) > integer(f[4]) || integer(f[6]) > integer(f[7]) {
            return Err(err("segment order"));
        }
        Ok(())
    }
}
pub fn err(s: &'static str) -> TreeError {
    TreeError::Synthesis(s.to_string())
}
pub fn integer(f: F) -> num_bigint::BigUint {
    num_bigint::BigUint::from_bytes_le(f.to_repr().as_ref())
}
fn bounded(f: F, n: usize) -> Result<(), TreeError> {
    if integer(f).bits() > n as u64 {
        Err(err("segment integer out of range"))
    } else {
        Ok(())
    }
}
pub fn compose(children: &[Segment]) -> Result<Segment, TreeError> {
    if !(2..=4).contains(&children.len()) {
        return Err(err("segment arity"));
    }
    let mut real = Vec::new();
    let mut padded = false;
    for s in children {
        s.validate()?;
        if s.fields[0] == F::ONE {
            padded = true;
        } else {
            if padded {
                return Err(err("real child after pad"));
            }
            real.push(s);
        }
    }
    if real.is_empty() {
        return Ok(children[0].clone().pad());
    }
    for pair in real.windows(2) {
        for (a, b) in [(2, 1), (7, 6), (9, 8)] {
            if pair[0].fields[a] != pair[1].fields[b] {
                return Err(err("segment continuity"));
            }
        }
        if integer(pair[0].fields[4]) >= integer(pair[1].fields[3]) {
            return Err(err("segment positions not strictly increasing"));
        }
    }
    let first = real[0];
    let last = real[real.len() - 1];
    let mut f = first.fields;
    for i in [2, 4, 7, 9] {
        f[i] = last.fields[i];
    }
    let mut digests = vec![if children.len() == 4 {
        dom::in4()
    } else if children.len() == 2 {
        dom::in2()
    } else {
        super::i1::fp::field_from_tag("DOM_IN3")
    }];
    digests.extend(children.iter().map(|s| {
        if s.fields[0] == F::ONE {
            super::i1::types::pad_leaf()
        } else {
            s.fields[5]
        }
    }));
    f[5] = hash(&digests);
    for (i, value) in f.iter_mut().enumerate().take(15).skip(10) {
        *value = real.iter().map(|s| s.fields[i]).sum();
        bounded(*value, 64)?;
    }
    let result = Segment { fields: f };
    result.validate()?;
    Ok(result)
}
