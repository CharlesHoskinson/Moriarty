//! The value store: every value a declaration computes, as fixed-size rows.
//!
//! Like the AST, each row is a tag and small operands whose meaning depends
//! on the tag (see [`ValueTag`]). Lists live in [`Values::extra`]; amounts
//! live in [`Values::amounts`]. Values are immutable, so a reference to an
//! earlier declaration reuses its value instead of copying it.

use mori_ast::NodeIdx;
use soa_rs::{Soa, Soars};

use crate::DeclIdx;

/// What a value is, and how to read its operands.
///
/// "Pairs" means `b..c` indexes [`Values::extra`] as alternating key tokens
/// and value handles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ValueTag {
    /// `a`: the string token. Its text is decoded when needed.
    String,
    /// `a`: `0` for false, `1` for true.
    Bool,
    /// `a`: index into [`Values::amounts`].
    Scalar,
    /// `a`: index into [`Values::amounts`], in the asset's smallest units.
    /// `b`: the asset's declaration.
    Quantity,
    /// `a`: `0` for `None`, `1` for `SuccessOnly`.
    Tag,
    /// `b..c`: item value handles in [`Values::extra`].
    List,
    /// Pairs: the fields.
    Record,
    /// `a`: the call node. Pairs: the arguments.
    Call,
    /// A declared domain, account, asset or other entity. `a`: its
    /// declaration. Pairs: its fields.
    Entity,
}

/// One value row. `node` is the expression that produced it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Soars)]
#[soa_derive(Debug, PartialEq)]
pub struct Value {
    pub tag: ValueTag,
    pub node: NodeIdx,
    pub a: u16,
    pub b: u16,
    pub c: u16,
}

/// Index of a value in [`Values`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValueIdx(pub(crate) u16);

impl ValueIdx {
    pub fn index(self) -> usize {
        usize::from(self.0)
    }
}

#[derive(Debug, Default)]
pub struct Values {
    pub rows: Soa<Value>,
    pub extra: Vec<u16>,
    pub amounts: Vec<u128>,
}

impl Values {
    pub fn tag(&self, value: ValueIdx) -> ValueTag {
        self.rows.tag()[value.index()]
    }

    pub fn node(&self, value: ValueIdx) -> NodeIdx {
        self.rows.node()[value.index()]
    }

    /// The amount of a scalar or quantity.
    pub fn amount(&self, value: ValueIdx) -> u128 {
        self.amounts[usize::from(self.rows.a()[value.index()])]
    }

    /// The asset of a quantity.
    pub fn asset(&self, value: ValueIdx) -> DeclIdx {
        DeclIdx(self.rows.b()[value.index()])
    }

    /// The declaration of an entity.
    pub fn entity(&self, value: ValueIdx) -> DeclIdx {
        DeclIdx(self.rows.a()[value.index()])
    }

    /// The items of a list.
    pub fn items(&self, value: ValueIdx) -> impl Iterator<Item = ValueIdx> + '_ {
        self.extra[self.range(value)]
            .iter()
            .map(|&raw| ValueIdx(raw))
    }

    /// The `(key token, value)` pairs of a record, call or entity.
    pub fn pairs(&self, value: ValueIdx) -> impl Iterator<Item = (u16, ValueIdx)> + '_ {
        self.extra[self.range(value)]
            .as_chunks::<2>()
            .0
            .iter()
            .map(|&[key, value]| (key, ValueIdx(value)))
    }

    fn range(&self, value: ValueIdx) -> std::ops::Range<usize> {
        usize::from(self.rows.b()[value.index()])..usize::from(self.rows.c()[value.index()])
    }

    pub(crate) fn push(
        &mut self,
        tag: ValueTag,
        node: NodeIdx,
        a: u16,
        b: u16,
        c: u16,
    ) -> ValueIdx {
        let index = u16::try_from(self.rows.len()).expect("values fit in u16");
        self.rows.push(Value { tag, node, a, b, c });
        ValueIdx(index)
    }

    pub(crate) fn push_amount(
        &mut self,
        tag: ValueTag,
        node: NodeIdx,
        amount: u128,
        asset: u16,
    ) -> ValueIdx {
        let index = u16::try_from(self.amounts.len()).expect("amounts fit in u16");
        self.amounts.push(amount);
        self.push(tag, node, index, asset, 0)
    }

    /// Appends `entries` to `extra`, returning their range.
    pub(crate) fn push_extra(&mut self, entries: &[u16]) -> (u16, u16) {
        let start = self.extra.len();
        self.extra.extend_from_slice(entries);
        let to_u16 = |index: usize| u16::try_from(index).expect("extra fits in u16");
        (to_u16(start), to_u16(self.extra.len()))
    }
}
