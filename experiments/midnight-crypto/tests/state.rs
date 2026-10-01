#![cfg(feature = "state")]
use midnight_base_crypto::hash::{HashOutput, persistent_hash};
use midnight_serialize::{Deserializable, Serializable};
use midnight_transient_crypto::merkle_tree::MerkleTree;
use moriarty_midnight_crypto::canonical_message;
use serde_json::{Value, json};
#[test]
fn official_state_tree_membership_and_update_rejection_controls() {
    let fs: Value = serde_json::from_str(include_str!("../fixtures/moriarty.json")).unwrap();
    let f = &fs["fixtures"][0];
    let leaf=persistent_hash(&canonical_message(&json!({"kind":"SnapshotClaim","state":f["statement"]["lowered"]["state"],"predecessor":f["statement"]["ast"]["authenticated"]["predecessor"]}).to_string()).unwrap());
    let other = persistent_hash(b"Moriarty unrelated local checkpoint");
    let tree = MerkleTree::<()>::blank(3)
        .try_update(0, &leaf, ())
        .unwrap()
        .try_update(1, &other, ())
        .unwrap()
        .rehash();
    let root = tree.root().unwrap();
    let path = tree.find_path_for_leaf(leaf).unwrap();
    assert_eq!(path.root(), root);
    let mut bad = path.clone();
    bad.leaf.0[0] ^= 1;
    assert_ne!(bad.root(), root);
    let mut bad = path.clone();
    bad.path[0].goes_left = !bad.path[0].goes_left;
    assert_ne!(bad.root(), root);
    let alternate = tree
        .try_update(1, &HashOutput([1; 32]), ())
        .unwrap()
        .rehash();
    let mut bad = path.clone();
    bad.path[0].sibling = alternate.find_path_for_leaf(leaf).unwrap().path[0].sibling;
    assert_ne!(bad.root(), root);
    let new_leaf=persistent_hash(&canonical_message(&json!({"kind":"CandidatePostClaim","state":f["statement"]["candidate"]["candidatePost"]}).to_string()).unwrap());
    let updated = tree.try_update(0, &new_leaf, ()).unwrap().rehash();
    assert_ne!(updated.root().unwrap(), root);
    assert_ne!(path.root(), updated.root().unwrap());
    assert_eq!(
        updated.find_path_for_leaf(new_leaf).unwrap().root(),
        updated.root().unwrap()
    );
    assert_eq!(tree.index(1).unwrap().0, updated.index(1).unwrap().0);
    assert!(
        tree.try_update_hash(8, persistent_hash(b"outside"), ())
            .is_err()
    );
    let mut bytes = Vec::new();
    tree.serialize(&mut bytes).unwrap();
    let restored = MerkleTree::<()>::deserialize(&mut bytes.as_slice(), 0).unwrap();
    assert_eq!(restored.root(), tree.root());
    assert_eq!(restored.index(1).unwrap().0, tree.index(1).unwrap().0);
    println!(
        "Official local Rust Merkle membership/update/serialization checked on actual fixture state; ledger head provenance unverified"
    );
}
