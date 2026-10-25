// Adapted from projectfenrir recursion/stagenet-strawman/recl9.
// Compiled on the pinned stock graph; installed-key equality is checked separately.
//! The transcript template: the ledger-9 contract-call public-input vector
//! `[binding_input, communication_commitment, field_repr(guaranteed ops),
//! field_repr(fallible ops)]` of `acceptValidity(stmt)`, with every slot
//! either free (binding input, communication commitment), a constant, or the
//! statement digest.

use std::io::{self, Read, Write};

use serde::{Deserialize, Serialize};

use crate::util::{F, f_from_hex, f_to_hex, read_f, read_u32, write_f};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slot {
    BindingInput,
    CommunicationCommitment,
    Const(F),
    Stmt,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Template {
    pub slots: Vec<Slot>,
}

/// JSON form, `decider/template.json` (and the accepted shape of
/// `spec/template.json`): `{"slots": [{"kind": "binding_input"},
/// {"kind": "communication_commitment"}, {"kind": "const", "value": "<hex32le>"},
/// {"kind": "stmt"}, ...], ...}`.
#[derive(Serialize, Deserialize)]
pub struct TemplateJson {
    pub slots: Vec<SlotJson>,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub provisional: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SlotJson {
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Template {
    pub fn validate(&self) -> io::Result<()> {
        let count = |p: fn(&Slot) -> bool| self.slots.iter().filter(|s| p(s)).count();
        if self.slots.len() < 3 {
            return Err(io::Error::other("template too short"));
        }
        if self.slots[0] != Slot::BindingInput || self.slots[1] != Slot::CommunicationCommitment {
            return Err(io::Error::other(
                "slots 0 and 1 must be binding_input and communication_commitment",
            ));
        }
        if count(|s| matches!(s, Slot::BindingInput)) != 1
            || count(|s| matches!(s, Slot::CommunicationCommitment)) != 1
        {
            return Err(io::Error::other(
                "exactly one binding_input and one communication_commitment",
            ));
        }
        if count(|s| matches!(s, Slot::Stmt)) != 1 {
            return Err(io::Error::other("exactly one stmt slot"));
        }
        Ok(())
    }

    pub fn stmt_index(&self) -> usize {
        self.slots.iter().position(|s| *s == Slot::Stmt).unwrap()
    }

    /// The ledger public-input vector for given free values.
    pub fn public_inputs(&self, binding_input: F, comm: F, stmt: F) -> Vec<F> {
        self.slots
            .iter()
            .map(|s| match s {
                Slot::BindingInput => binding_input,
                Slot::CommunicationCommitment => comm,
                Slot::Const(c) => *c,
                Slot::Stmt => stmt,
            })
            .collect()
    }

    /// Does `pis` fit the template (constants equal), and which stmt does it carry?
    pub fn match_public_inputs(&self, pis: &[F]) -> Option<F> {
        if pis.len() != self.slots.len() {
            return None;
        }
        for (s, v) in self.slots.iter().zip(pis) {
            if let Slot::Const(c) = s
                && c != v
            {
                return None;
            }
        }
        Some(pis[self.stmt_index()])
    }

    pub fn from_json(j: &TemplateJson) -> io::Result<Self> {
        let mut slots = Vec::new();
        for s in &j.slots {
            slots.push(match s.kind.as_str() {
                "binding_input" => Slot::BindingInput,
                "communication_commitment" => Slot::CommunicationCommitment,
                "stmt" => Slot::Stmt,
                "const" => Slot::Const(f_from_hex(
                    s.value
                        .as_deref()
                        .ok_or_else(|| io::Error::other("const slot without value"))?,
                )?),
                k => return Err(io::Error::other(format!("unknown slot kind {k}"))),
            });
        }
        let t = Template { slots };
        t.validate()?;
        Ok(t)
    }

    /// Parse either this crate's `{"slots": [...]}` form or the anchor lane's
    /// capture form `{"n", "constants": {"idx": hex}, "free": [0, 1, s],
    /// "stmt_index": s}` (chain/probe/template/template.json).
    pub fn from_any_json(bytes: &[u8]) -> io::Result<Self> {
        let v: serde_json::Value = serde_json::from_slice(bytes).map_err(io::Error::other)?;
        if v.get("slots").is_some() {
            let j: TemplateJson = serde_json::from_value(v).map_err(io::Error::other)?;
            return Self::from_json(&j);
        }
        let n = v["n"]
            .as_u64()
            .ok_or_else(|| io::Error::other("template: no n"))? as usize;
        let s = v["stmt_index"]
            .as_u64()
            .ok_or_else(|| io::Error::other("template: no stmt_index"))? as usize;
        let mut free: Vec<u64> = v["free"]
            .as_array()
            .ok_or_else(|| io::Error::other("template: no free"))?
            .iter()
            .map(|x| x.as_u64().unwrap_or(u64::MAX))
            .collect();
        free.sort();
        let mut want = vec![0u64, 1, s as u64];
        want.sort();
        if free != want {
            return Err(io::Error::other(
                "template: free slots must be exactly 0, 1 and stmt_index",
            ));
        }
        let consts = v["constants"]
            .as_object()
            .ok_or_else(|| io::Error::other("template: no constants"))?;
        let mut slots = Vec::with_capacity(n);
        for i in 0..n {
            slots.push(match i {
                0 => Slot::BindingInput,
                1 => Slot::CommunicationCommitment,
                i if i == s => Slot::Stmt,
                i => Slot::Const(f_from_hex(
                    consts
                        .get(&i.to_string())
                        .and_then(|x| x.as_str())
                        .ok_or_else(|| {
                            io::Error::other(format!("template: no constant for slot {i}"))
                        })?,
                )?),
            });
        }
        if consts.len() != n - 3 {
            return Err(io::Error::other(
                "template: constant count does not match n - 3",
            ));
        }
        let t = Template { slots };
        t.validate()?;
        Ok(t)
    }

    pub fn to_json(&self, source: &str, provisional: bool) -> TemplateJson {
        TemplateJson {
            slots: self
                .slots
                .iter()
                .map(|s| match s {
                    Slot::BindingInput => SlotJson {
                        kind: "binding_input".into(),
                        value: None,
                    },
                    Slot::CommunicationCommitment => SlotJson {
                        kind: "communication_commitment".into(),
                        value: None,
                    },
                    Slot::Stmt => SlotJson {
                        kind: "stmt".into(),
                        value: None,
                    },
                    Slot::Const(c) => SlotJson {
                        kind: "const".into(),
                        value: Some(f_to_hex(c)),
                    },
                })
                .collect(),
            source: source.into(),
            provisional,
        }
    }

    pub fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        w.write_all(&(self.slots.len() as u32).to_le_bytes())?;
        for s in &self.slots {
            match s {
                Slot::BindingInput => w.write_all(&[0])?,
                Slot::CommunicationCommitment => w.write_all(&[1])?,
                Slot::Const(c) => {
                    w.write_all(&[2])?;
                    write_f(w, c)?;
                }
                Slot::Stmt => w.write_all(&[3])?,
            }
        }
        Ok(())
    }

    pub fn read<R: Read>(r: &mut R) -> io::Result<Self> {
        let n = read_u32(r)? as usize;
        if n > 4096 {
            return Err(io::Error::other("template too long"));
        }
        let mut slots = Vec::with_capacity(n);
        for _ in 0..n {
            let mut tag = [0u8; 1];
            r.read_exact(&mut tag)?;
            slots.push(match tag[0] {
                0 => Slot::BindingInput,
                1 => Slot::CommunicationCommitment,
                2 => Slot::Const(read_f(r)?),
                3 => Slot::Stmt,
                t => return Err(io::Error::other(format!("bad slot tag {t}"))),
            });
        }
        let t = Template { slots };
        t.validate()?;
        Ok(t)
    }
}
