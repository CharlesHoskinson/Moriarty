// fin_carrier.rs — PROPOSED, source-only, uncompiled, unadopted.
// Crate-private prerequisite carrier/binding unit. Not wired to owner_step,
// admit_run, FinV1 genesis, V7/V8, or public finance.
// Identity scope: FinSnapshot is bound by structural equality to the
// FinBoundSource value that produced it (names, states, assets, actions,
// capacities). This is not a digest, full ProgramView identity, admission,
// expression/occurrence identity, or financial authorization.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Action, AssetDecl, AssetId, Balance, Liability, Program, Snapshot, StateDecl, Type, UInt, Value};
    use std::collections::BTreeMap;

    fn prog() -> Program {
        Program { language: crate::LANGUAGE.into(), semantics: crate::SEMANTICS.into(),
            name: "prog".into(),
            states: vec![StateDecl { name: "count".into(), ty: Type::UInt,
                initial: Value::UInt(UInt(0)) }],
            assets: vec![AssetDecl { name: "coin".into(), identity: AssetId {
                network: "n".into(), issuer: "i".into(), reference: "r".into(), scale: 0 } }],
            expressions: vec![], actions: vec![] }
    }
    fn prog_act() -> Program {
        let mut p = prog();
        p.actions = vec![Action { name: "act".into(), inputs: vec![], statements: vec![] }];
        p
    }
    fn prog_multi() -> Program {
        Program { language: crate::LANGUAGE.into(), semantics: crate::SEMANTICS.into(),
            name: "prog".into(),
            states: vec![
                StateDecl { name: "count".into(), ty: Type::UInt, initial: Value::UInt(UInt(0)) },
                StateDecl { name: "flag".into(), ty: Type::Bool, initial: Value::Bool(false) }],
            assets: vec![
                AssetDecl { name: "zeta".into(), identity: AssetId { network: "z".into(),
                    issuer: "i".into(), reference: "r".into(), scale: 0 } },
                AssetDecl { name: "alpha".into(), identity: AssetId { network: "a".into(),
                    issuer: "i".into(), reference: "r".into(), scale: 0 } }],
            expressions: vec![],
            actions: vec![Action { name: "act".into(), inputs: vec![], statements: vec![] }] }
    }
    fn view(names: &[&str]) -> ProgramView {
        ProgramView { entry: 0, expressions: vec![], occurrences: vec![],
            occurrence_paths: vec![], children: vec![], functions: vec![],
            parameters: vec![], arms: vec![], patterns: vec![], pattern_children: vec![],
            types: vec![], type_args: vec![], constructors: vec![], ctor_fields: vec![],
            decl_param_counts: vec![], decl_ctor_counts: vec![], decl_templates: vec![],
            plan_nodes: vec![], plan_cases: vec![], field_paths: vec![],
            constants: vec![], constant_fields: vec![],
            names: names.iter().map(|s| s.to_string()).collect() }
    }
    fn layout(name_bytes: usize) -> LayoutSchemaC {
        LayoutSchemaC { layout_version: ("t".into(), "1".into()), field_id: None,
            widths: None, scalar_layout_id: None, bounds: BoundsC::default(),
            capacities: LayoutCapacitiesC { name_rows: 64, name_bytes,
                occurrence_path_slots: 64, value_slots: 64 },
            ext_capacities: None, boundary_capacities_present: false }
    }
    fn caps() -> FinCapacitiesC { FinCapacitiesC { max_states: 8, max_assets: 8,
        max_balances: 64, max_liabilities: 64 } }
    fn bound(name_bytes: usize) -> FinBoundSource {
        bind_fin_source(&prog(), &view(&["coin", "count", "prog"]), &layout(name_bytes), &caps())
            .expect("bind")
    }
    fn bound_act() -> FinBoundSource {
        bind_fin_source(&prog_act(), &view(&["act", "coin", "count", "prog"]), &layout(1024), &caps())
            .expect("bind act")
    }
    fn bound_multi() -> FinBoundSource {
        bind_fin_source(&prog_multi(), &view(&["act", "alpha", "count", "flag", "prog", "zeta"]),
            &layout(1024), &caps()).expect("bind multi")
    }
    fn empty_snap(b: &FinBoundSource) -> FinSnapshot {
        let s = Snapshot { state: BTreeMap::from([("count".to_string(), Value::UInt(UInt(0)))]),
            balances: vec![], liabilities: vec![] };
        FinSnapshot::from_core(b, &s).expect("snapshot")
    }
    fn plan() -> FinPlan { FinPlan { actions: vec![FinActionPlan { name: "act".into(),
        segment_src_cost: vec![3, 2], committed_before: vec![0, 3], plan_total: 5 }] } }
    fn snap_with_balances(b: &FinBoundSource, rows: Vec<(u128, &str, u128)>) -> FinSnapshot {
        let coin = AssetId { network: "n".into(), issuer: "i".into(), reference: "r".into(), scale: 0 };
        let _ = rows;
        let s = Snapshot { state: BTreeMap::from([("count".to_string(), Value::UInt(UInt(0)))]),
            balances: vec![
                Balance { asset: coin.clone(), account: "alice".into(), amount: UInt(10) },
                Balance { asset: coin, account: "bob".into(), amount: UInt(3) }],
            liabilities: vec![] };
        FinSnapshot::from_core(b, &s).expect("balances")
    }

    #[test]
    fn ce05_single_plan_authority_and_consistency() {
        let b = bound_act();
        let x = FinAdmittedExtras::new(b.clone(), plan()).expect("extras");
        assert_eq!(x.plan().actions[0].plan_total, 5);
        assert_eq!(x.source(), &b);
        let mut bad = plan(); bad.actions[0].plan_total = 4;
        assert_eq!(FinAdmittedExtras::new(b.clone(), bad), Err(FinBindError::PlanTotalMismatch));
        let mut bad2 = plan(); bad2.actions[0].committed_before = vec![0];
        assert_eq!(FinAdmittedExtras::new(b.clone(), bad2), Err(FinBindError::PlanLengthMismatch));
        let mut bad3 = plan(); bad3.actions[0].committed_before = vec![0, 2];
        assert_eq!(FinAdmittedExtras::new(b.clone(), bad3), Err(FinBindError::PlanPrefixMismatch));
        let mut bad4 = plan(); bad4.actions[0].segment_src_cost = vec![u128::MAX, 1];
        bad4.actions[0].committed_before = vec![0, u128::MAX]; bad4.actions[0].plan_total = 0;
        assert_eq!(FinAdmittedExtras::new(b, bad4), Err(FinBindError::PlanOverflow));
    }
    #[test]
    fn ce10_layout_name_bytes_is_sole_bound() {
        // Independently derived boundary: "coin"+"count"+"prog" = 4+5+4 = 13.
        assert!(bind_fin_source(&prog(), &view(&["coin", "count", "prog"]), &layout(13), &caps()).is_ok());
        assert_eq!(bind_fin_source(&prog(), &view(&["coin", "count", "prog"]), &layout(12), &caps()),
            Err(FinBindError::NameBytesExceeded { needed: 13, limit: 12 }));
    }
    #[test]
    fn ce07_name_id_distinct_from_state_row() {
        let b = bound(1024);
        assert_eq!(b.states()[0].row, FinStateRowId(0));
        assert_eq!(b.states()[0].name, NameId(1));
        assert_eq!(b.assets()[0].row, FinAssetRowId(0));
        assert_eq!(b.assets()[0].name, NameId(0));
        assert_eq!(b.resolve_name("count"), Ok(NameId(1)));
        assert_eq!(b.resolve_name("absent"), Err(FinBindError::UnknownName("absent".into())));
    }
    #[test]
    fn ce07_set_state_by_row_id_and_type_check() {
        let b = bound(1024); let mut s = empty_snap(&b);
        apply_set_state(&b, &mut s, FinStateRowId(0), FinScalar::UInt(7)).expect("set");
        assert_eq!(s.states()[0], FinScalar::UInt(7));
        let before = s.clone();
        assert_eq!(apply_set_state(&b, &mut s, FinStateRowId(0), FinScalar::Bool(true)),
            Err(FinMutationError::StateTypeMismatch));
        assert_eq!(apply_set_state(&b, &mut s, FinStateRowId(9), FinScalar::UInt(1)),
            Err(FinMutationError::StateRowOutOfRange));
        assert_eq!(s, before);
    }
    #[test]
    fn ce07_liability_sorted_insert_existing_check_and_zero_present() {
        let b = bound(1024); let mut s = empty_snap(&b);
        apply_liability_add(&b, &mut s, "zeta", FinAssetRowId(0), "d", "c", 5, None).expect("z");
        apply_liability_add(&b, &mut s, "alpha", FinAssetRowId(0), "d", "c", 0, None).expect("a");
        assert_eq!(s.liabilities().iter().map(|l| l.name.as_str()).collect::<Vec<_>>(), ["alpha", "zeta"]);
        assert_eq!(s.liabilities()[0].amount, 0);
        let before = s.clone();
        assert_eq!(apply_liability_add(&b, &mut s, "alpha", FinAssetRowId(0), "OTHER", "c", 1, Some(0)),
            Err(FinMutationError::LiabilityIdentityMismatch));
        assert_eq!(s, before);
        apply_liability_add(&b, &mut s, "alpha", FinAssetRowId(0), "d", "c", 2, Some(0)).expect("add");
        assert_eq!(s.liabilities()[0].amount, 2);
        let before = s.clone();
        assert_eq!(apply_liability_add(&b, &mut s, "alpha", FinAssetRowId(0), "d", "c", 1, None),
            Err(FinMutationError::ExistingRowRequired));
        assert_eq!(s, before);
    }
    #[test]
    fn ce07_unsorted_names_and_unknown_snapshot_asset_reject() {
        assert_eq!(bind_fin_source(&prog(), &view(&["count", "coin"]), &layout(1024), &caps()),
            Err(FinBindError::NamesNotSorted));
        let b = bound(1024);
        let bad = Snapshot { state: BTreeMap::from([("count".to_string(), Value::UInt(UInt(0)))]),
            balances: vec![Balance { account: "a".into(), amount: UInt(1),
                asset: AssetId { network: "x".into(), issuer: "i".into(), reference: "r".into(), scale: 0 } }],
            liabilities: vec![] };
        assert_eq!(FinSnapshot::from_core(&b, &bad), Err(FinBindError::AssetIdentityUnknown));
    }
    #[test]
    fn uc01_cross_source_snapshot_refused_everywhere() {
        let a = bound(1024);
        let mut p2 = prog();
        p2.assets[0].identity = AssetId { network: "other".into(), issuer: "i".into(),
            reference: "r".into(), scale: 0 };
        let b = bind_fin_source(&p2, &view(&["coin", "count", "prog"]), &layout(1024), &caps())
            .expect("bind other");
        let mut s = empty_snap(&a);
        let before = s.clone();
        assert_eq!(s.to_core(&b), Err(FinMutationError::SourceMismatch));
        assert_eq!(apply_set_state(&b, &mut s, FinStateRowId(0), FinScalar::UInt(1)),
            Err(FinMutationError::SourceMismatch));
        assert_eq!(apply_liability_add(&b, &mut s, "x", FinAssetRowId(0), "d", "c", 1, None),
            Err(FinMutationError::SourceMismatch));
        assert_eq!(apply_transfer(&b, &mut s, FinAssetRowId(0), "a", "b", 1),
            Err(FinMutationError::SourceMismatch));
        assert_eq!(s, before);
    }
    #[test]
    fn uc01_from_core_truncated_and_extra_states_refused() {
        let b = bound(1024);
        let truncated = Snapshot { state: BTreeMap::new(), balances: vec![], liabilities: vec![] };
        assert_eq!(FinSnapshot::from_core(&b, &truncated), Err(FinBindError::StateSetMismatch));
        let mut extra = BTreeMap::from([("count".to_string(), Value::UInt(UInt(0)))]);
        extra.insert("ghost".to_string(), Value::UInt(UInt(1)));
        let extra = Snapshot { state: extra, balances: vec![], liabilities: vec![] };
        assert_eq!(FinSnapshot::from_core(&b, &extra), Err(FinBindError::StateSetMismatch));
    }
    #[test]
    fn uc02_checked_extras_reject_action_count_and_identity() {
        let b = bound_act();
        let mut wrong = plan(); wrong.actions[0].name = "missing".into();
        assert_eq!(FinAdmittedExtras::new(b.clone(), wrong), Err(FinBindError::PlanActionMismatch));
        let mut extra = plan(); extra.actions.push(extra.actions[0].clone());
        assert_eq!(FinAdmittedExtras::new(b.clone(), extra), Err(FinBindError::PlanActionMismatch));
        assert_eq!(FinAdmittedExtras::new(b, FinPlan { actions: vec![] }),
            Err(FinBindError::PlanActionMismatch));
    }
    #[test]
    fn uc03_initial_type_cross_kind_and_name_rows_refused() {
        let mut p = prog(); p.states[0].initial = Value::Bool(true);
        assert_eq!(bind_fin_source(&p, &view(&["coin", "count", "prog"]), &layout(1024), &caps()),
            Err(FinBindError::InitialTypeMismatch("count".into())));
        let mut p2 = prog(); p2.assets[0].name = "count".into();
        assert_eq!(bind_fin_source(&p2, &view(&["count", "prog"]), &layout(1024), &caps()),
            Err(FinBindError::DuplicateDeclaration("count".into())));
        let mut l = layout(1024); l.capacities.name_rows = 2;
        assert_eq!(bind_fin_source(&prog(), &view(&["coin", "count", "prog"]), &l, &caps()),
            Err(FinBindError::NameRowsExceeded { needed: 3, limit: 2 }));
    }
    #[test]
    fn uc04_from_core_capacity_and_duplicates_refused() {
        let b = bound(1024);
        let coin = AssetId { network: "n".into(), issuer: "i".into(), reference: "r".into(), scale: 0 };
        let dup = Snapshot { state: BTreeMap::from([("count".to_string(), Value::UInt(UInt(0)))]),
            balances: vec![
                Balance { asset: coin.clone(), account: "a".into(), amount: UInt(1) },
                Balance { asset: coin.clone(), account: "a".into(), amount: UInt(2) }],
            liabilities: vec![] };
        assert_eq!(FinSnapshot::from_core(&b, &dup), Err(FinBindError::DuplicateBalance));
        let dup_l = Snapshot { state: BTreeMap::from([("count".to_string(), Value::UInt(UInt(0)))]),
            balances: vec![], liabilities: vec![
                Liability { name: "x".into(), asset: coin.clone(), debtor: "d".into(), creditor: "c".into(), amount: UInt(1) },
                Liability { name: "x".into(), asset: coin, debtor: "d".into(), creditor: "c".into(), amount: UInt(2) }] };
        assert_eq!(FinSnapshot::from_core(&b, &dup_l), Err(FinBindError::DuplicateLiability));
        let mut small = caps(); small.max_balances = 0;
        let bs = bind_fin_source(&prog(), &view(&["coin", "count", "prog"]), &layout(1024), &small)
            .expect("small");
        let one = Snapshot { state: BTreeMap::from([("count".to_string(), Value::UInt(UInt(0)))]),
            balances: vec![Balance { account: "a".into(), amount: UInt(1),
                asset: AssetId { network: "n".into(), issuer: "i".into(), reference: "r".into(), scale: 0 } }],
            liabilities: vec![] };
        assert_eq!(FinSnapshot::from_core(&bs, &one), Err(FinBindError::CapacityExceeded("balances")));
    }
    #[test]
    fn uc05_multi_asset_balance_liability_core_roundtrip() {
        let b = bound_multi();
        let zeta = AssetId { network: "z".into(), issuer: "i".into(), reference: "r".into(), scale: 0 };
        let alpha = AssetId { network: "a".into(), issuer: "i".into(), reference: "r".into(), scale: 0 };
        // Declaration order is zeta(row0), alpha(row1); Core AssetId order is alpha, zeta.
        let core = Snapshot {
            state: BTreeMap::from([("count".to_string(), Value::UInt(UInt(2))),
                ("flag".to_string(), Value::Bool(true))]),
            balances: vec![
                Balance { asset: alpha.clone(), account: "alice".into(), amount: UInt(7) },
                Balance { asset: zeta.clone(), account: "alice".into(), amount: UInt(9) }],
            liabilities: vec![Liability { name: "due".into(), asset: zeta,
                debtor: "d".into(), creditor: "c".into(), amount: UInt(4) }] };
        let fin = FinSnapshot::from_core(&b, &core).expect("from core");
        assert_eq!(fin.balances()[0].asset, FinAssetRowId(1));
        assert_eq!(fin.balances()[1].asset, FinAssetRowId(0));
        assert_eq!(fin.to_core(&b), Ok(core));
    }
    #[test]
    fn uc06_transfer_happy_effect_and_self_transfer() {
        let b = bound(1024); let mut s = snap_with_balances(&b, vec![]);
        let fx = apply_transfer(&b, &mut s, FinAssetRowId(0), "alice", "bob", 4).expect("transfer");
        assert_eq!(fx, FinEffectC::Transfer { asset: FinAssetRowId(0),
            from: "alice".into(), to: "bob".into(), amount: 4 });
        assert_eq!(s.balances().iter().map(|x| x.amount).collect::<Vec<_>>(), [6, 7]);
        let before = s.clone();
        assert_eq!(apply_transfer(&b, &mut s, FinAssetRowId(0), "alice", "alice", 4),
            Ok(FinEffectC::Transfer { asset: FinAssetRowId(0),
                from: "alice".into(), to: "alice".into(), amount: 4 }));
        assert_eq!(s, before);
    }
    #[test]
    fn uc06_transfer_refusals_preserve_snapshot() {
        let b = bound(1024); let mut s = snap_with_balances(&b, vec![]);
        let before = s.clone();
        assert_eq!(apply_transfer(&b, &mut s, FinAssetRowId(0), "alice", "bob", 11),
            Err(FinMutationError::InsufficientBalance));
        assert_eq!(apply_transfer(&b, &mut s, FinAssetRowId(0), "carol", "bob", 1),
            Err(FinMutationError::SnapshotShape));
        assert_eq!(apply_transfer(&b, &mut s, FinAssetRowId(0), "alice", "carol", 1),
            Err(FinMutationError::SnapshotShape));
        assert_eq!(apply_transfer(&b, &mut s, FinAssetRowId(7), "alice", "bob", 1),
            Err(FinMutationError::AssetRowOutOfRange));
        assert_eq!(apply_transfer(&b, &mut s, FinAssetRowId(0), "alice", "alice", 11),
            Err(FinMutationError::InsufficientBalance));
        assert_eq!(s, before);
        // Recipient overflow, separately constructed at u128::MAX.
        let coin = AssetId { network: "n".into(), issuer: "i".into(), reference: "r".into(), scale: 0 };
        let core = Snapshot { state: BTreeMap::from([("count".to_string(), Value::UInt(UInt(0)))]),
            balances: vec![
                Balance { asset: coin.clone(), account: "alice".into(), amount: UInt(10) },
                Balance { asset: coin, account: "bob".into(), amount: UInt(u128::MAX) }],
            liabilities: vec![] };
        let mut s2 = FinSnapshot::from_core(&b, &core).expect("max");
        let before2 = s2.clone();
        assert_eq!(apply_transfer(&b, &mut s2, FinAssetRowId(0), "alice", "bob", 1),
            Err(FinMutationError::Overflow));
        assert_eq!(s2, before2);
    }
    #[test]
    fn uc06_liability_missing_overflow_and_capacity_refusals() {
        let mut small = caps(); small.max_liabilities = 1;
        let b = bind_fin_source(&prog(), &view(&["coin", "count", "prog"]), &layout(1024), &small)
            .expect("small liabilities");
        let mut s = empty_snap(&b);
        apply_liability_add(&b, &mut s, "only", FinAssetRowId(0), "d", "c", u128::MAX, None).expect("only");
        let before = s.clone();
        assert_eq!(apply_liability_add(&b, &mut s, "second", FinAssetRowId(0), "d", "c", 1, None),
            Err(FinMutationError::CapacityExceeded("liabilities")));
        assert_eq!(apply_liability_add(&b, &mut s, "only", FinAssetRowId(0), "d", "c", 1, Some(9)),
            Err(FinMutationError::MissingExistingRow));
        assert_eq!(apply_liability_add(&b, &mut s, "only", FinAssetRowId(0), "d", "c", 1, Some(0)),
            Err(FinMutationError::Overflow));
        assert_eq!(apply_liability_add(&b, &mut s, "only", FinAssetRowId(7), "d", "c", 1, Some(0)),
            Err(FinMutationError::AssetRowOutOfRange));
        assert_eq!(s, before);
    }
    #[test]
    fn uc06_bool_state_happy_path() {
        let b = bound_multi();
        let core = Snapshot { state: BTreeMap::from([
                ("count".to_string(), Value::UInt(UInt(0))),
                ("flag".to_string(), Value::Bool(false))]),
            balances: vec![], liabilities: vec![] };
        let mut s = FinSnapshot::from_core(&b, &core).expect("bool snap");
        apply_set_state(&b, &mut s, FinStateRowId(1), FinScalar::Bool(true)).expect("set bool");
        assert_eq!(s.states()[1], FinScalar::Bool(true));
        let before = s.clone();
        assert_eq!(apply_set_state(&b, &mut s, FinStateRowId(1), FinScalar::UInt(1)),
            Err(FinMutationError::StateTypeMismatch));
        assert_eq!(s, before);
    }
}

use super::{BoundsC, LayoutCapacitiesC, LayoutSchemaC, ProgramView};
use crate::stage::{StagePlan, StagedProgram, LANGUAGE_2, SEMANTICS_2, STAGE_PLAN_FORMAT};
use crate::{Action, AssetId, Expr, ExprId, InputDecl, Program, Snapshot, Statement, Type, UInt, Value};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct NameId(pub usize);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct FinStateRowId(pub usize);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct FinAssetRowId(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FinScalar { UInt(u128), Bool(bool) }
impl FinScalar {
    pub fn ty(&self) -> Type { match self { Self::UInt(_) => Type::UInt, Self::Bool(_) => Type::Bool } }
}
impl From<&Value> for FinScalar {
    fn from(v: &Value) -> Self { match v { Value::UInt(UInt(n)) => Self::UInt(*n), Value::Bool(b) => Self::Bool(*b) } }
}
impl From<FinScalar> for Value {
    fn from(v: FinScalar) -> Self { match v { FinScalar::UInt(n) => Value::UInt(UInt(n)), FinScalar::Bool(b) => Value::Bool(b) } }
}

/// No name_bytes field by design: LayoutSchemaC.capacities.name_bytes governs (CE10).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FinCapacitiesC { pub max_states: usize, pub max_assets: usize,
    pub max_balances: usize, pub max_liabilities: usize }

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FinPlan { pub actions: Vec<FinActionPlan> }
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FinActionPlan { pub name: String, pub segment_src_cost: Vec<u128>,
    pub committed_before: Vec<u128>, pub plan_total: u128 }

pub fn validate_fin_plan(p: &FinPlan) -> Result<(), FinBindError> {
    for a in &p.actions {
        if a.segment_src_cost.len() != a.committed_before.len() { return Err(FinBindError::PlanLengthMismatch); }
        let mut acc: u128 = 0;
        for (i, c) in a.segment_src_cost.iter().enumerate() {
            if a.committed_before[i] != acc { return Err(FinBindError::PlanPrefixMismatch); }
            acc = acc.checked_add(*c).ok_or(FinBindError::PlanOverflow)?;
        }
        if acc != a.plan_total { return Err(FinBindError::PlanTotalMismatch); }
    }
    Ok(())
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FinBindError { ProfileMismatch, NamesNotSorted,
    NameBytesExceeded { needed: u128, limit: usize },
    NameRowsExceeded { needed: usize, limit: usize },
    UnknownName(String), DuplicateDeclaration(String), AssetIdentityDuplicate,
    CapacityExceeded(&'static str), StateSetMismatch, AssetIdentityUnknown,
    InitialTypeMismatch(String), PlanActionMismatch, DuplicateBalance, DuplicateLiability,
    PlanLengthMismatch, PlanPrefixMismatch, PlanTotalMismatch, PlanOverflow }
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FinMutationError { StateRowOutOfRange, StateTypeMismatch, AssetRowOutOfRange,
    LiabilityIdentityMismatch, ExistingRowRequired, MissingExistingRow, Overflow,
    SnapshotShape, InsufficientBalance, SourceMismatch, CapacityExceeded(&'static str) }

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FinStateBinding { pub row: FinStateRowId, pub name: NameId, pub text: String,
    pub ty: Type, pub initial: FinScalar }
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FinAssetBinding { pub row: FinAssetRowId, pub name: NameId, pub text: String, pub identity: AssetId }
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FinActionBinding { pub name: NameId, pub text: String }

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FinBoundSource { names: Vec<String>, states: Vec<FinStateBinding>,
    assets: Vec<FinAssetBinding>, actions: Vec<FinActionBinding>, capacities: FinCapacitiesC }
impl FinBoundSource {
    pub fn names(&self) -> &[String] { &self.names }
    pub fn states(&self) -> &[FinStateBinding] { &self.states }
    pub fn assets(&self) -> &[FinAssetBinding] { &self.assets }
    pub fn actions(&self) -> &[FinActionBinding] { &self.actions }
    pub fn capacities(&self) -> FinCapacitiesC { self.capacities }
    pub fn resolve_name(&self, text: &str) -> Result<NameId, FinBindError> {
        self.names.binary_search_by(|n| n.as_str().cmp(text)).map(NameId)
            .map_err(|_| FinBindError::UnknownName(text.to_string()))
    }
    pub fn asset_row_of(&self, id: &AssetId) -> Result<FinAssetRowId, FinBindError> {
        self.assets.iter().find(|a| &a.identity == id).map(|a| a.row)
            .ok_or(FinBindError::AssetIdentityUnknown)
    }
}

/// One stored plan. Construction is the checked extras path; it is not
/// financial admission and makes no V8 claim.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FinAdmittedExtras { plan: FinPlan, source: FinBoundSource }
impl FinAdmittedExtras {
    pub fn new(source: FinBoundSource, plan: FinPlan) -> Result<Self, FinBindError> {
        validate_fin_plan(&plan)?;
        if plan.actions.len() != source.actions.len() { return Err(FinBindError::PlanActionMismatch); }
        for (planned, bound) in plan.actions.iter().zip(source.actions.iter()) {
            if planned.name != bound.text { return Err(FinBindError::PlanActionMismatch); }
        }
        Ok(Self { plan, source })
    }
    pub fn plan(&self) -> &FinPlan { &self.plan }
    pub fn source(&self) -> &FinBoundSource { &self.source }
}

pub fn bind_fin_source(program: &Program, view: &ProgramView, layout: &LayoutSchemaC,
    caps: &FinCapacitiesC) -> Result<FinBoundSource, FinBindError> {
    if program.language != crate::LANGUAGE || program.semantics != crate::SEMANTICS {
        return Err(FinBindError::ProfileMismatch); }
    if view.names.windows(2).any(|w| w[0] >= w[1]) { return Err(FinBindError::NamesNotSorted); }
    if view.names.len() > layout.capacities.name_rows {
        return Err(FinBindError::NameRowsExceeded { needed: view.names.len(),
            limit: layout.capacities.name_rows }); }
    let bytes: u128 = view.names.iter().map(|n| n.len() as u128).sum();
    if bytes > layout.capacities.name_bytes as u128 {
        return Err(FinBindError::NameBytesExceeded { needed: bytes, limit: layout.capacities.name_bytes }); }
    if program.states.len() > caps.max_states { return Err(FinBindError::CapacityExceeded("states")); }
    if program.assets.len() > caps.max_assets { return Err(FinBindError::CapacityExceeded("assets")); }
    let mut bound = FinBoundSource { names: view.names.clone(), states: Vec::new(),
        assets: Vec::new(), actions: Vec::new(), capacities: *caps };
    // Resolve into a local first; no borrow is held across push.
    let _program_name = bound.resolve_name(&program.name)?;
    for (i, s) in program.states.iter().enumerate() {
        if bound.states.iter().any(|x| x.text == s.name)
            || bound.assets.iter().any(|x| x.text == s.name) {
            return Err(FinBindError::DuplicateDeclaration(s.name.clone())); }
        let initial = FinScalar::from(&s.initial);
        if initial.ty() != s.ty { return Err(FinBindError::InitialTypeMismatch(s.name.clone())); }
        let name_id = bound.resolve_name(&s.name)?;
        bound.states.push(FinStateBinding { row: FinStateRowId(i), name: name_id,
            text: s.name.clone(), ty: s.ty, initial });
    }
    for (i, a) in program.assets.iter().enumerate() {
        if bound.assets.iter().any(|x| x.text == a.name)
            || bound.states.iter().any(|x| x.text == a.name) {
            return Err(FinBindError::DuplicateDeclaration(a.name.clone())); }
        if bound.assets.iter().any(|x| x.identity == a.identity) {
            return Err(FinBindError::AssetIdentityDuplicate); }
        let name_id = bound.resolve_name(&a.name)?;
        bound.assets.push(FinAssetBinding { row: FinAssetRowId(i), name: name_id,
            text: a.name.clone(), identity: a.identity.clone() });
    }
    for act in &program.actions {
        if bound.actions.iter().any(|x| x.text == act.name) {
            return Err(FinBindError::DuplicateDeclaration(act.name.clone())); }
        let name_id = bound.resolve_name(&act.name)?;
        bound.actions.push(FinActionBinding { name: name_id, text: act.name.clone() });
    }
    Ok(bound)
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FinStagedPrepareError {
    Stage(crate::Error),
    Bind(FinBindError),
}
impl From<FinBindError> for FinStagedPrepareError {
    fn from(e: FinBindError) -> Self { Self::Bind(e) }
}

/// Checked `/2` declaration binding derived from the sealed staged source.
/// Stage namespaces are preserved without changing `/1`: states+assets
/// share one declaration namespace, actions are separate, and the program
/// name is independent. `FinBoundSource.names` is the ordinary interned-
/// spelling table already used by `/1`: one sorted unique `NameId` per
/// distinct spelling. Equal spellings across namespaces therefore share
/// a `NameId`, while typed `FinStateBinding`/`FinAssetBinding` rows and
/// the `FinActionBinding` list distinguish roles; `resolve_name` resolves
/// a spelling, not a role. Same-kind and state/asset collisions are still
/// rejected by the unchanged binding loops below; cross-kind equality is
/// not a duplicate. `name_rows` is the interned-table length and
/// `name_bytes` is the sum of its distinct spelling lengths, exactly the
/// `/1` accounting over its sorted unique view names. Refusal order is
/// name_rows, then name_bytes, then state capacity, then asset capacity.
/// Guards stage validation already excludes (profile, initial type,
/// same-kind duplicates, asset-identity duplicates) remain defence in
/// depth on this route, not load-route coverage.
pub fn bind_fin_staged_source(
    staged: &StagedProgram,
    layout: &LayoutSchemaC,
    caps: &FinCapacitiesC,
) -> Result<FinBoundSource, FinBindError> {
    let program = staged.program();
    if program.language != LANGUAGE_2 || program.semantics != SEMANTICS_2 {
        return Err(FinBindError::ProfileMismatch);
    }
    let mut names: Vec<String> = Vec::new();
    names.push(program.name.clone());
    names.extend(program.states.iter().map(|s| s.name.clone()));
    names.extend(program.assets.iter().map(|a| a.name.clone()));
    names.extend(program.actions.iter().map(|a| a.name.clone()));
    names.sort();
    names.dedup();
    if names.len() > layout.capacities.name_rows {
        return Err(FinBindError::NameRowsExceeded {
            needed: names.len(),
            limit: layout.capacities.name_rows,
        });
    }
    let bytes: u128 = names.iter().map(|n| n.len() as u128).sum();
    if bytes > layout.capacities.name_bytes as u128 {
        return Err(FinBindError::NameBytesExceeded {
            needed: bytes,
            limit: layout.capacities.name_bytes,
        });
    }
    if program.states.len() > caps.max_states {
        return Err(FinBindError::CapacityExceeded("states"));
    }
    if program.assets.len() > caps.max_assets {
        return Err(FinBindError::CapacityExceeded("assets"));
    }
    let mut bound = FinBoundSource {
        names,
        states: Vec::new(),
        assets: Vec::new(),
        actions: Vec::new(),
        capacities: *caps,
    };
    let _program_name = bound.resolve_name(&program.name)?;
    for (i, s) in program.states.iter().enumerate() {
        if bound.states.iter().any(|x| x.text == s.name)
            || bound.assets.iter().any(|x| x.text == s.name)
        {
            return Err(FinBindError::DuplicateDeclaration(s.name.clone()));
        }
        let initial = FinScalar::from(&s.initial);
        if initial.ty() != s.ty {
            return Err(FinBindError::InitialTypeMismatch(s.name.clone()));
        }
        let name_id = bound.resolve_name(&s.name)?;
        bound.states.push(FinStateBinding {
            row: FinStateRowId(i),
            name: name_id,
            text: s.name.clone(),
            ty: s.ty,
            initial,
        });
    }
    for (i, a) in program.assets.iter().enumerate() {
        if bound.assets.iter().any(|x| x.text == a.name)
            || bound.states.iter().any(|x| x.text == a.name)
        {
            return Err(FinBindError::DuplicateDeclaration(a.name.clone()));
        }
        if bound.assets.iter().any(|x| x.identity == a.identity) {
            return Err(FinBindError::AssetIdentityDuplicate);
        }
        let name_id = bound.resolve_name(&a.name)?;
        bound.assets.push(FinAssetBinding {
            row: FinAssetRowId(i),
            name: name_id,
            text: a.name.clone(),
            identity: a.identity.clone(),
        });
    }
    for act in &program.actions {
        if bound.actions.iter().any(|x| x.text == act.name) {
            return Err(FinBindError::DuplicateDeclaration(act.name.clone()));
        }
        let name_id = bound.resolve_name(&act.name)?;
        bound.actions.push(FinActionBinding { name: name_id, text: act.name.clone() });
    }
    Ok(bound)
}

/// The sole FinPlan authority on this path: copied in action order from the
/// sealed StagePlan (source-logical costs, never B fuel/schedule). No
/// caller-authored plan can enter `FinPreparedStagedSource`.
pub fn derive_fin_plan_from_stage(staged: &StagedProgram) -> Result<FinPlan, FinBindError> {
    let program = staged.program();
    let plan = staged.plan();
    if plan.format != STAGE_PLAN_FORMAT || plan.actions.len() != program.actions.len() {
        return Err(FinBindError::PlanActionMismatch);
    }
    let mut actions = Vec::with_capacity(plan.actions.len());
    for (planned, source_action) in plan.actions.iter().zip(program.actions.iter()) {
        if planned.action != source_action.name
            || usize::from(planned.statement_count) != source_action.statements.len()
        {
            return Err(FinBindError::PlanActionMismatch);
        }
        actions.push(FinActionPlan {
            name: planned.action.clone(),
            segment_src_cost: planned.segment_cost.iter().map(|u| u.0).collect(),
            committed_before: planned.committed_before.iter().map(|u| u.0).collect(),
            plan_total: planned.plan_total.0,
        });
    }
    let fin_plan = FinPlan { actions };
    validate_fin_plan(&fin_plan)?;
    Ok(fin_plan)
}

/// Paired prepared source: the whole sealed `/2` program/plan plus the
/// source-derived extras. `StagedProgram` has no PartialEq/Eq, so this
/// wrapper deliberately derives only Clone+Debug; `same_source` compares
/// the comparable components (Program, StagePlan, hashes, extras).
#[derive(Clone, Debug)]
pub struct FinPreparedStagedSource {
    staged: StagedProgram,
    extras: FinAdmittedExtras,
}
impl FinPreparedStagedSource {
    pub fn prepare(
        staged: StagedProgram,
        layout: &LayoutSchemaC,
        caps: &FinCapacitiesC,
    ) -> Result<Self, FinStagedPrepareError> {
        let bound = bind_fin_staged_source(&staged, layout, caps)?;
        let fin_plan = derive_fin_plan_from_stage(&staged)?;
        let extras = FinAdmittedExtras::new(bound, fin_plan)?;
        Ok(Self { staged, extras })
    }
    pub fn staged(&self) -> &StagedProgram { &self.staged }
    pub fn program(&self) -> &Program { self.staged.program() }
    pub fn stage_plan(&self) -> &StagePlan { self.staged.plan() }
    pub fn program_hash(&self) -> [u8; 32] { self.staged.program_hash() }
    pub fn plan_hash(&self) -> [u8; 32] { self.staged.plan_hash() }
    pub fn extras(&self) -> &FinAdmittedExtras { &self.extras }
    pub fn bound_source(&self) -> &FinBoundSource { self.extras.source() }
    pub fn fin_plan(&self) -> &FinPlan { self.extras.plan() }
    pub fn same_source(&self, other: &Self) -> bool {
        self.program() == other.program()
            && self.stage_plan() == other.stage_plan()
            && self.program_hash() == other.program_hash()
            && self.plan_hash() == other.plan_hash()
            && self.extras == other.extras
    }
    pub fn action_index(&self, name: &str) -> Option<usize> {
        self.program().actions.iter().position(|a| a.name == name)
    }
    pub fn action(&self, name: &str) -> Option<&Action> {
        self.program().actions.iter().find(|a| a.name == name)
    }
    pub fn action_inputs(&self, name: &str) -> Option<&[InputDecl]> {
        self.action(name).map(|a| a.inputs.as_slice())
    }
    pub fn statement(&self, action: &str, ordinal: usize) -> Option<&Statement> {
        self.action(action).and_then(|a| a.statements.get(ordinal))
    }
    pub fn expression(&self, id: ExprId) -> Option<&Expr> {
        self.program().expressions.get(usize::from(id))
    }
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FinInputBinding {
    pub name: String,
    pub ty: Type,
    pub value: Value,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FinGenesisRequest {
    pub action: String,
    pub scenario: crate::Scenario,
    pub cap: Option<UInt>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FinGenesisError {
    UnknownAction(String),
    PlanActionMismatch,
    NotFinancialAdmission,
    Scenario(crate::Error),
    CapExceeded { plan_total: u128, cap: u128 },
    Bind(FinBindError),
    CommittedBeforeMismatch,
    GenesisInvalid,
}
impl From<FinBindError> for FinGenesisError {
    fn from(e: FinBindError) -> Self { Self::Bind(e) }
}

/// Financial genesis uses the actual run tooling for R3/R4/R5 refusal and
/// FinGenesisError for source/scenario/cap/invalid-genesis refusal. Neither
/// is converted into a fabricated image or outcome. Debug is manual so the
/// non-Debug Tooling payload is named without inventing a Tooling format.
pub enum FinRunError {
    Genesis(FinGenesisError),
    Tooling(super::Tooling),
}
impl std::fmt::Debug for FinRunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Genesis(e) => f.debug_tuple("Genesis").field(e).finish(),
            Self::Tooling(_) => f.write_str("Tooling(..)"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FinBalanceRow { pub asset: FinAssetRowId, pub account: String, pub amount: u128 }
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FinLiabilityRow { pub name: String, pub asset: FinAssetRowId,
    pub debtor: String, pub creditor: String, pub amount: u128 }

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FinSnapshot { source: FinBoundSource, states: Vec<FinScalar>,
    balances: Vec<FinBalanceRow>, liabilities: Vec<FinLiabilityRow> }
impl FinSnapshot {
    pub fn source(&self) -> &FinBoundSource { &self.source }
    pub fn states(&self) -> &[FinScalar] { &self.states }
    pub fn balances(&self) -> &[FinBalanceRow] { &self.balances }
    pub fn liabilities(&self) -> &[FinLiabilityRow] { &self.liabilities }

    pub fn from_core(b: &FinBoundSource, s: &Snapshot) -> Result<Self, FinBindError> {
        if s.state.len() != b.states.len() { return Err(FinBindError::StateSetMismatch); }
        let mut states = Vec::with_capacity(b.states.len());
        for d in &b.states {
            let v = s.state.get(&d.text).ok_or(FinBindError::StateSetMismatch)?;
            let fv = FinScalar::from(v);
            if fv.ty() != d.ty { return Err(FinBindError::StateSetMismatch); }
            states.push(fv);
        }
        if s.balances.len() > b.capacities.max_balances { return Err(FinBindError::CapacityExceeded("balances")); }
        if s.liabilities.len() > b.capacities.max_liabilities { return Err(FinBindError::CapacityExceeded("liabilities")); }
        let mut balances: Vec<FinBalanceRow> = Vec::with_capacity(s.balances.len());
        for x in &s.balances { balances.push(FinBalanceRow { asset: b.asset_row_of(&x.asset)?,
            account: x.account.clone(), amount: x.amount.0 }); }
        // Internal and Core-canonical order: (AssetId, account), not declaration row.
        balances.sort_by(|x, y| {
            let xi = &b.assets[x.asset.0].identity;
            let yi = &b.assets[y.asset.0].identity;
            xi.cmp(yi).then_with(|| x.account.cmp(&y.account))
        });
        if balances.windows(2).any(|w| {
            b.assets[w[0].asset.0].identity == b.assets[w[1].asset.0].identity
                && w[0].account == w[1].account }) {
            return Err(FinBindError::DuplicateBalance); }
        let mut liabilities: Vec<FinLiabilityRow> = Vec::with_capacity(s.liabilities.len());
        for x in &s.liabilities { liabilities.push(FinLiabilityRow { name: x.name.clone(),
            asset: b.asset_row_of(&x.asset)?, debtor: x.debtor.clone(),
            creditor: x.creditor.clone(), amount: x.amount.0 }); }
        liabilities.sort_by(|x, y| x.name.cmp(&y.name));
        if liabilities.windows(2).any(|w| w[0].name == w[1].name) {
            return Err(FinBindError::DuplicateLiability); }
        Ok(Self { source: b.clone(), states, balances, liabilities })
    }

    pub fn to_core(&self, b: &FinBoundSource) -> Result<Snapshot, FinMutationError> {
        check_fin_snapshot(b, self)?;
        let mut state = std::collections::BTreeMap::new();
        for (d, v) in b.states.iter().zip(self.states.iter()) {
            state.insert(d.text.clone(), Value::from(*v));
        }
        if state.len() != b.states.len() { return Err(FinMutationError::SnapshotShape); }
        let mut balances = Vec::with_capacity(self.balances.len());
        for x in &self.balances {
            let binding = b.assets.get(x.asset.0).ok_or(FinMutationError::AssetRowOutOfRange)?;
            balances.push(crate::Balance { account: x.account.clone(),
                asset: binding.identity.clone(), amount: UInt(x.amount) });
        }
        let mut liabilities = Vec::with_capacity(self.liabilities.len());
        for x in &self.liabilities {
            let binding = b.assets.get(x.asset.0).ok_or(FinMutationError::AssetRowOutOfRange)?;
            liabilities.push(crate::Liability { name: x.name.clone(),
                asset: binding.identity.clone(), debtor: x.debtor.clone(),
                creditor: x.creditor.clone(), amount: UInt(x.amount) });
        }
        Ok(Snapshot { state, balances, liabilities })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FinEffectC { State { state: FinStateRowId, before: FinScalar, after: FinScalar },
    Liability { name: String, before: u128, after: u128 },
    Transfer { asset: FinAssetRowId, from: String, to: String, amount: u128 } }

pub fn check_fin_snapshot(b: &FinBoundSource, s: &FinSnapshot) -> Result<(), FinMutationError> {
    if s.source != *b { return Err(FinMutationError::SourceMismatch); }
    if s.states.len() != b.states.len() { return Err(FinMutationError::SnapshotShape); }
    for (d, v) in b.states.iter().zip(s.states.iter()) {
        if v.ty() != d.ty { return Err(FinMutationError::StateTypeMismatch); } }
    if s.balances.len() > b.capacities.max_balances {
        return Err(FinMutationError::CapacityExceeded("balances")); }
    if s.liabilities.len() > b.capacities.max_liabilities {
        return Err(FinMutationError::CapacityExceeded("liabilities")); }
    if s.balances.iter().any(|x| x.asset.0 >= b.assets.len()) { return Err(FinMutationError::AssetRowOutOfRange); }
    if s.liabilities.iter().any(|x| x.asset.0 >= b.assets.len()) { return Err(FinMutationError::AssetRowOutOfRange); }
    for w in s.balances.windows(2) {
        let left = (&b.assets[w[0].asset.0].identity, &w[0].account);
        let right = (&b.assets[w[1].asset.0].identity, &w[1].account);
        if left >= right { return Err(FinMutationError::SnapshotShape); }
    }
    if s.liabilities.windows(2).any(|w| w[0].name >= w[1].name) {
        return Err(FinMutationError::SnapshotShape); }
    Ok(())
}

pub fn apply_set_state(b: &FinBoundSource, s: &mut FinSnapshot, row: FinStateRowId,
    value: FinScalar) -> Result<FinEffectC, FinMutationError> {
    check_fin_snapshot(b, s)?;
    let d = b.states.get(row.0).ok_or(FinMutationError::StateRowOutOfRange)?;
    if value.ty() != d.ty { return Err(FinMutationError::StateTypeMismatch); }
    let slot = s.states.get_mut(row.0).ok_or(FinMutationError::SnapshotShape)?;
    let before = *slot; *slot = value;
    Ok(FinEffectC::State { state: row, before, after: value })
}

pub fn apply_liability_add(b: &FinBoundSource, s: &mut FinSnapshot, name: &str,
    asset: FinAssetRowId, debtor: &str, creditor: &str, amount: u128,
    existing_row: Option<usize>) -> Result<FinEffectC, FinMutationError> {
    check_fin_snapshot(b, s)?;
    if asset.0 >= b.assets.len() { return Err(FinMutationError::AssetRowOutOfRange); }
    match existing_row {
        Some(r) => {
            if s.liabilities.binary_search_by(|l| l.name.as_str().cmp(name)) != Ok(r) {
                let exists = s.liabilities.get(r);
                return Err(match exists {
                    Some(row) if row.name == name => FinMutationError::SnapshotShape,
                    Some(_) => FinMutationError::LiabilityIdentityMismatch,
                    None => FinMutationError::MissingExistingRow,
                });
            }
            let row = s.liabilities.get(r).ok_or(FinMutationError::MissingExistingRow)?;
            if row.name != name || row.asset != asset || row.debtor != debtor || row.creditor != creditor {
                return Err(FinMutationError::LiabilityIdentityMismatch); }
            let before = row.amount;
            let after = before.checked_add(amount).ok_or(FinMutationError::Overflow)?;
            let slot = s.liabilities.get_mut(r).ok_or(FinMutationError::MissingExistingRow)?;
            slot.amount = after;
            Ok(FinEffectC::Liability { name: name.into(), before, after })
        }
        None => {
            let pos = match s.liabilities.binary_search_by(|l| l.name.as_str().cmp(name)) {
                Ok(_) => return Err(FinMutationError::ExistingRowRequired),
                Err(p) => p };
            if s.liabilities.len() >= b.capacities.max_liabilities {
                return Err(FinMutationError::CapacityExceeded("liabilities")); }
            s.liabilities.insert(pos, FinLiabilityRow { name: name.into(), asset,
                debtor: debtor.into(), creditor: creditor.into(), amount });
            Ok(FinEffectC::Liability { name: name.into(), before: 0, after: amount })
        }
    }
}

pub fn apply_transfer(b: &FinBoundSource, s: &mut FinSnapshot, asset: FinAssetRowId,
    from: &str, to: &str, amount: u128) -> Result<FinEffectC, FinMutationError> {
    check_fin_snapshot(b, s)?;
    if asset.0 >= b.assets.len() { return Err(FinMutationError::AssetRowOutOfRange); }
    let f = s.balances.iter().position(|x| x.asset == asset && x.account == from)
        .ok_or(FinMutationError::SnapshotShape)?;
    let t = s.balances.iter().position(|x| x.asset == asset && x.account == to)
        .ok_or(FinMutationError::SnapshotShape)?;
    let from_row = s.balances.get(f).ok_or(FinMutationError::SnapshotShape)?;
    let debit = from_row.amount.checked_sub(amount).ok_or(FinMutationError::InsufficientBalance)?;
    if f != t {
        let to_row = s.balances.get(t).ok_or(FinMutationError::SnapshotShape)?;
        let credit = to_row.amount.checked_add(amount).ok_or(FinMutationError::Overflow)?;
        if let Some(slot) = s.balances.get_mut(f) { slot.amount = debit; }
        if let Some(slot) = s.balances.get_mut(t) { slot.amount = credit; }
    }
    Ok(FinEffectC::Transfer { asset, from: from.into(), to: to.into(), amount })
}
