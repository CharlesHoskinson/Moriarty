use mori_core::{AssetId, SourceLiabilityIdentity, Statement, stage};
use serde::Deserialize;
use std::collections::BTreeSet;

const CORE3_SOURCE: &str =
    include_str!("../../fixtures/sealed-registry-metadata/RegistryFrame.mori");
const CORE3_EXPECTED: &str =
    include_str!("../../fixtures/sealed-registry-metadata/REGISTRY-EXPECTED.json");
const CORE2_SOURCE: &str =
    include_str!("../../fixtures/sealed-registry-metadata/Core2Registry.mori");
const CORE2_EXPECTED: &str =
    include_str!("../../fixtures/sealed-registry-metadata/CORE2-REGISTRY-EXPECTED.json");

type Identity = (String, AssetId, String, String);

#[derive(Deserialize)]
struct ExpectedRegistry {
    registry_as_semantic_set: Vec<ExpectedIdentity>,
}

#[derive(Deserialize)]
struct ExpectedIdentity {
    name: String,
    asset: AssetId,
    debtor: String,
    creditor: String,
}

fn expected_set(text: &str) -> BTreeSet<Identity> {
    let expected: ExpectedRegistry = serde_json::from_str(text).expect("frozen metadata set");
    expected
        .registry_as_semantic_set
        .into_iter()
        .map(|entry| (entry.name, entry.asset, entry.debtor, entry.creditor))
        .collect()
}

fn identity_set(entries: &[SourceLiabilityIdentity]) -> BTreeSet<Identity> {
    entries
        .iter()
        .map(|entry| {
            (
                entry.name.clone(),
                entry.asset.clone(),
                entry.debtor.clone(),
                entry.creditor.clone(),
            )
        })
        .collect()
}

#[test]
fn core3_registry_metadata_matches_complete_independent_identity_set() {
    let sp = stage::load_v3(CORE3_SOURCE).expect("load distinct-reference Core3 source");
    let program_before = sp.program().clone();
    let plan_before = sp.plan().clone();
    let hashes_before = (sp.program_hash(), sp.plan_hash());
    assert_eq!(program_before.actions.len(), 8);
    assert_eq!(program_before.duties.len(), 6);
    assert_eq!(
        program_before
            .actions
            .iter()
            .flat_map(|action| &action.statements)
            .filter(|statement| matches!(statement, Statement::Liability { .. }))
            .count(),
        2
    );

    let metadata = sp.liability_registry().expect("read Core3 metadata");
    let expected = expected_set(CORE3_EXPECTED);
    // Seven complete identities include six declared names and cross from two
    // matching statements in different actions. No action is executed/selected.
    assert_eq!(metadata.len(), 7);
    assert_eq!(expected.len(), 7);
    assert_eq!(identity_set(&metadata), expected);
    assert_eq!(sp.program(), &program_before);
    assert_eq!(sp.plan(), &plan_before);
    assert_eq!((sp.program_hash(), sp.plan_hash()), hashes_before);
}

#[test]
fn registry_metadata_is_owned_and_cannot_mutate_sealed_program() {
    let sp = stage::load_v3(CORE3_SOURCE).expect("load distinct-reference Core3 source");
    let program_before = sp.program().clone();
    let plan_before = sp.plan().clone();
    let hashes_before = (sp.program_hash(), sp.plan_hash());
    let original = sp.liability_registry().expect("read original metadata");
    let mut detached = sp
        .liability_registry()
        .expect("read separately owned metadata");
    let entry = detached.first_mut().expect("nonempty frozen set");
    entry.name = "detached-name".into();
    entry.asset.network = "detached-network".into();
    entry.asset.issuer = "detached-issuer".into();
    entry.asset.reference = "detached-reference".into();
    entry.asset.scale = 1;
    entry.debtor = "detached-debtor".into();
    entry.creditor = "detached-creditor".into();
    detached.pop();
    assert_ne!(detached, original);

    let reread = sp.liability_registry().expect("read fresh owned metadata");
    assert_eq!(reread, original);
    assert_eq!(identity_set(&reread), expected_set(CORE3_EXPECTED));
    assert_eq!(sp.program(), &program_before);
    assert_eq!(sp.plan(), &plan_before);
    assert_eq!((sp.program_hash(), sp.plan_hash()), hashes_before);
}

#[test]
fn core2_registry_metadata_preserves_statement_only_identities() {
    let sp = stage::load(CORE2_SOURCE).expect("load Core2 metadata source");
    let program_before = sp.program().clone();
    assert_eq!(program_before.semantics, stage::SEMANTICS_2);
    assert!(program_before.duties.is_empty());
    let metadata = sp.liability_registry().expect("read Core2 metadata");
    let expected = expected_set(CORE2_EXPECTED);
    assert_eq!(metadata.len(), 2);
    assert_eq!(expected.len(), 2);
    assert_eq!(identity_set(&metadata), expected);
    assert_eq!(sp.program(), &program_before);
}
