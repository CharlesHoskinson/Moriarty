//! Source-only regressions for exact value/ownership of TypeExprSrc::clone.
//! Expectations are independent preorder tuples, not subject Clone/Eq/Debug.
use mori_core::sherlock::helper_machine::shared::owner_ext_type_builder::TypeExprSrc as T;

type Atom = (u8, String, usize);
fn atom(tag: u8, name: &str, arity: usize) -> Atom { (tag, name.to_owned(), arity) }
fn shape(root: &T) -> Vec<Atom> {
    let mut pending = vec![root];
    let mut result = Vec::new();
    while let Some(value) = pending.pop() {
        match value {
            T::UInt => result.push(atom(1, "", 0)),
            T::Bool => result.push(atom(2, "", 0)),
            T::List => result.push(atom(3, "", 0)),
            T::Tree => result.push(atom(4, "", 0)),
            T::Named { name } => result.push(atom(5, name, 0)),
            T::Param { name } => result.push(atom(6, name, 0)),
            T::Apply { name, args } => {
                result.push(atom(7, name, args.len()));
                pending.extend(args.iter().rev());
            }
        }
    }
    result
}

#[test]
fn leaf_clone_preserves_frozen_tags_and_utf8_names() {
    let cases = vec![
        (T::UInt, vec![atom(1, "", 0)]),
        (T::Bool, vec![atom(2, "", 0)]),
        (T::List, vec![atom(3, "", 0)]),
        (T::Tree, vec![atom(4, "", 0)]),
        (T::Named { name: "Asset".into() }, vec![atom(5, "Asset", 0)]),
        (T::Named { name: "".into() }, vec![atom(5, "", 0)]),
        (T::Named { name: "名字💠".into() }, vec![atom(5, "名字💠", 0)]),
        (T::Param { name: "T".into() }, vec![atom(6, "T", 0)]),
        (T::Param { name: "".into() }, vec![atom(6, "", 0)]),
    ];
    for (original, frozen) in cases {
        let cloned = original.clone();
        assert_eq!(shape(&cloned), frozen);
        assert_eq!(shape(&original), frozen);
    }
}

#[test]
fn leaf_clone_names_remain_independently_owned() {
    for (original, tag) in [
        (T::Named { name: "owned".into() }, 5),
        (T::Param { name: "owned".into() }, 6),
    ] {
        let mut cloned = original.clone();
        match &mut cloned {
            T::Named { name } | T::Param { name } => name.push_str("-changed"),
            _ => panic!("clone changed the name-bearing variant"),
        }
        assert_eq!(shape(&original), vec![atom(tag, "owned", 0)]);
        assert_eq!(shape(&cloned), vec![atom(tag, "owned-changed", 0)]);
    }
}

#[test]
fn apply_clone_preserves_frozen_order_arity_and_nested_leaves() {
    let original = T::Apply { name: "Map".into(), args: vec![
        T::Named { name: "Key".into() },
        T::Apply { name: "Seq".into(), args: vec![T::Param { name: "T".into() }, T::Bool] },
        T::Apply { name: "Empty".into(), args: vec![] },
        T::UInt, T::List, T::Tree,
    ] };
    let frozen = vec![atom(7, "Map", 6), atom(5, "Key", 0), atom(7, "Seq", 2),
        atom(6, "T", 0), atom(2, "", 0), atom(7, "Empty", 0),
        atom(1, "", 0), atom(3, "", 0), atom(4, "", 0)];
    let mut cloned = original.clone();
    assert_eq!(shape(&cloned), frozen);
    if let T::Apply { args, .. } = &mut cloned {
        if let T::Named { name } = &mut args[0] { name.push('!'); }
        else { panic!("clone changed the first argument variant"); }
    } else { panic!("clone changed Apply"); }
    let mut changed = frozen.clone();
    changed[1] = atom(5, "Key!", 0);
    assert_eq!(shape(&cloned), changed);
    assert_eq!(shape(&original), frozen);
}

#[test]
fn deep_apply_clone_retains_all4096levels_without_recursive_clone() {
    let mut original = T::Param { name: "T".into() };
    for _ in 0..4096 { original = T::Apply { name: "Box".into(), args: vec![original] }; }
    let cloned = original.clone();
    let mut frozen = vec![atom(7, "Box", 1); 4096];
    frozen.push(atom(6, "T", 0));
    assert_eq!(shape(&cloned), frozen);
    assert_eq!(shape(&original), frozen);
    drop(cloned);
    drop(original);
}
