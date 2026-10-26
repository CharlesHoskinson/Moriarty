// Adapted from the pinned Fenrir B2 oracle; see ../sources.json.
//! Sparse Poseidon Merkle trees and the indexed nullifier tree.
//!
//! Adapted from the spike `fenrir-recursive-rollup/src/trees.rs`
//! (Apache-2.0), generalized to a runtime depth. Node hash:
//! `H(DOM_NODE, left, right)`; empty leaf is zero. The indexed
//! nullifier tree keeps the spike logic: 248-bit keys, reserved
//! sentinels 0 and 2^248 - 1, a head sentinel leaf at index 0, and
//! insertion by updating the predecessor (low leaf) then appending.

use super::fp::{F, dom, f_u64, hash, max_key};
use ff::{Field, PrimeField};
use std::collections::BTreeMap;

pub fn node_hash(left: F, right: F) -> F {
    hash(&[dom::node(), left, right])
}

fn key_bytes(value: F) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(value.to_repr().as_ref());
    bytes.reverse();
    bytes
}

pub fn root_from_path(mut leaf: F, mut index: u32, path: &[F]) -> F {
    for sibling in path {
        leaf = if index & 1 == 0 {
            node_hash(leaf, *sibling)
        } else {
            node_hash(*sibling, leaf)
        };
        index >>= 1;
    }
    leaf
}

#[derive(Clone, Debug)]
pub struct SparseTree {
    pub depth: usize,
    nodes: BTreeMap<(usize, u32), F>,
    defaults: Vec<F>,
}

impl SparseTree {
    pub fn new(depth: usize) -> Self {
        assert!((1..=32).contains(&depth));
        let mut defaults = vec![F::ZERO; depth + 1];
        for d in 0..depth {
            defaults[d + 1] = node_hash(defaults[d], defaults[d]);
        }
        Self {
            depth,
            nodes: BTreeMap::new(),
            defaults,
        }
    }

    fn get(&self, d: usize, index: u32) -> F {
        *self.nodes.get(&(d, index)).unwrap_or(&self.defaults[d])
    }

    pub fn root(&self) -> F {
        self.get(self.depth, 0)
    }

    pub fn leaf(&self, index: u32) -> F {
        self.get(0, index)
    }

    pub fn path(&self, mut index: u32) -> Vec<F> {
        (0..self.depth)
            .map(|d| {
                let s = self.get(d, index ^ 1);
                index >>= 1;
                s
            })
            .collect()
    }

    pub fn set(&mut self, mut index: u32, mut value: F) {
        self.nodes.insert((0, index), value);
        for d in 0..self.depth {
            let sibling = self.get(d, index ^ 1);
            value = if index & 1 == 0 {
                node_hash(value, sibling)
            } else {
                node_hash(sibling, value)
            };
            index >>= 1;
            self.nodes.insert((d + 1, index), value);
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NullifierLeaf {
    pub value: F,
    pub next_index: u32,
    pub next_value: F,
}

pub fn nullifier_leaf_hash(leaf: &NullifierLeaf) -> F {
    hash(&[
        dom::nf_leaf(),
        leaf.value,
        f_u64(leaf.next_index as u64),
        leaf.next_value,
    ])
}

#[derive(Clone, Debug)]
pub struct NullifierInsert {
    pub predecessor_index: u32,
    pub predecessor: NullifierLeaf,
    pub predecessor_path: Vec<F>,
    pub append_path: Vec<F>,
}

#[derive(Clone, Debug)]
pub struct IndexedTree {
    pub tree: SparseTree,
    pub count: u32,
    leaves: BTreeMap<u32, NullifierLeaf>,
    ordered: BTreeMap<[u8; 32], u32>,
}

impl IndexedTree {
    pub fn new(depth: usize) -> Self {
        let mut tree = SparseTree::new(depth);
        let head = NullifierLeaf {
            value: F::ZERO,
            next_index: 0,
            next_value: max_key(),
        };
        tree.set(0, nullifier_leaf_hash(&head));
        Self {
            tree,
            count: 1,
            leaves: BTreeMap::from([(0, head)]),
            ordered: BTreeMap::from([(key_bytes(F::ZERO), 0)]),
        }
    }

    pub fn root(&self) -> F {
        self.tree.root()
    }

    pub fn contains(&self, value: F) -> bool {
        self.ordered.contains_key(&key_bytes(value))
    }

    pub fn leaf_at(&self, index: u32) -> Option<&NullifierLeaf> {
        self.leaves.get(&index)
    }

    /// Insert a key. Errors: "reserved", "double_spend", "full".
    /// On error the tree is unchanged (all checks precede mutation,
    /// except that a full tree is detected before mutation too).
    pub fn insert(&mut self, value: F) -> Result<NullifierInsert, &'static str> {
        if value == F::ZERO || key_bytes(value) >= key_bytes(max_key()) {
            return Err("reserved");
        }
        if self.contains(value) {
            return Err("double_spend");
        }
        let next_count = self.count.checked_add(1).ok_or("full")?;
        let (&_, &index) = self
            .ordered
            .range(..key_bytes(value))
            .next_back()
            .ok_or("no_predecessor")?;
        let pred = self.leaves[&index].clone();
        let pred_path = self.tree.path(index);
        let updated = NullifierLeaf {
            value: pred.value,
            next_index: self.count,
            next_value: value,
        };
        self.tree.set(index, nullifier_leaf_hash(&updated));
        self.leaves.insert(index, updated);
        let append_path = self.tree.path(self.count);
        let fresh = NullifierLeaf {
            value,
            next_index: pred.next_index,
            next_value: pred.next_value,
        };
        self.tree.set(self.count, nullifier_leaf_hash(&fresh));
        self.leaves.insert(self.count, fresh);
        self.ordered.insert(key_bytes(value), self.count);
        self.count = next_count;
        Ok(NullifierInsert {
            predecessor_index: index,
            predecessor: pred,
            predecessor_path: pred_path,
            append_path,
        })
    }
}
