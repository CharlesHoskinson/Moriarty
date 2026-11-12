//! Reader of the `wrapin` v1 document written by the exporter (openvm worktree, crate
//! `sherlock-wrap-export`): params, vk, proof, layouts and the stock transcript record.

use std::{fs, path::Path};

use ff::PrimeField;
use serde_json::Value;

use crate::native::{F, f_from_hex};

pub struct WrapIn {
    pub doc: Value,
}

/// A transcript event of the stock record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev {
    Obs(F),
    Commit(F),
    Sample(F),
}

/// An Fr value in serde form: 32 bytes, little-endian canonical.
pub fn fr(v: &Value) -> F {
    let b: Vec<u8> = v.as_array().expect("Fr bytes").iter().map(|x| x.as_u64().unwrap() as u8).collect();
    assert_eq!(b.len(), 32);
    let mut repr = <F as PrimeField>::Repr::default();
    repr.as_mut().copy_from_slice(&b);
    Option::from(F::from_repr(repr)).expect("canonical Fr")
}

/// A digest (`[F; 1]`).
pub fn digest(v: &Value) -> F {
    let a = v.as_array().expect("digest");
    assert_eq!(a.len(), 1);
    fr(&a[0])
}

pub fn frs(v: &Value) -> Vec<F> {
    v.as_array().expect("Fr list").iter().map(fr).collect()
}

pub fn us(v: &Value) -> usize {
    v.as_u64().expect("usize") as usize
}

impl WrapIn {
    pub fn load(path: &Path) -> Self {
        let doc: Value = serde_json::from_slice(&fs::read(path).expect("wrapin file")).unwrap();
        assert_eq!(doc["version"], 1);
        assert!(doc["stock_verdict"].as_bool().unwrap(), "stock verifier refused the fixture");
        Self { doc }
    }

    pub fn log(&self) -> Vec<Ev> {
        self.doc["log"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| {
                let x = f_from_hex(e[1].as_str().unwrap()).unwrap();
                match e[0].as_str().unwrap() {
                    "o" => Ev::Obs(x),
                    "c" => Ev::Commit(x),
                    "s" => Ev::Sample(x),
                    k => panic!("event {k}"),
                }
            })
            .collect()
    }

    pub fn u_transcript(&self) -> u128 {
        self.doc["domains"]["u_transcript"].as_u64().unwrap() as u128
    }
    pub fn u_leaf(&self) -> u128 {
        self.doc["domains"]["u_leaf"].as_u64().unwrap() as u128
    }
    pub fn u_node(&self) -> u128 {
        self.doc["domains"]["u_node"].as_u64().unwrap() as u128
    }
    pub fn params(&self) -> &Value {
        &self.doc["vk"]["inner"]["params"]
    }
    pub fn proof(&self) -> &Value {
        &self.doc["proof"]
    }
    pub fn derived(&self, k: &str) -> usize {
        us(&self.doc["derived"][k])
    }
}

/// The fixture directory (`$E5_FIXTURES`, default `~/.cache/e5-wrap/fixtures`).
pub fn fixture(name: &str) -> std::path::PathBuf {
    let dir = std::env::var("E5_FIXTURES").unwrap_or_else(|_| {
        format!("{}/.cache/e5-wrap/fixtures", std::env::var("HOME").unwrap())
    });
    Path::new(&dir).join(format!("{name}.json"))
}
