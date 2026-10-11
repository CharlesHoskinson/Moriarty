use mori_core::intent::{AssetLimit, check_effects};
use mori_core::policy::{Cost, CostPlane, FeeCapability, Meter};
use mori_core::{AssetId, Effect, UInt};

fn asset() -> AssetId {
    AssetId {
        network: "local".into(),
        issuer: "issuer".into(),
        reference: "usd".into(),
        scale: 2,
    }
}

#[test]
fn refund_does_not_erase_gross_and_fees_reduce_net() {
    let limits = vec![AssetLimit {
        asset: asset(),
        beneficiary: "bob".into(),
        gross_cap: UInt(11),
        fee_cap: UInt(1),
        net_floor: UInt(9),
        liability_cap: UInt(0),
    }];
    let effects = vec![
        Effect::Transfer {
            asset: asset(),
            from: "alice".into(),
            to: "bob".into(),
            amount: UInt(10),
        },
        Effect::Fee {
            asset: asset(),
            from: "alice".into(),
            to: "pool".into(),
            amount: UInt(1),
        },
    ];
    assert!(check_effects("alice", &limits, &effects, &[]).is_ok());
    let mut refund = effects.clone();
    refund.push(Effect::Transfer {
        asset: asset(),
        from: "bob".into(),
        to: "alice".into(),
        amount: UInt(2),
    });
    assert!(check_effects("alice", &limits, &refund, &[]).is_err());
    refund.push(Effect::Transfer {
        asset: asset(),
        from: "alice".into(),
        to: "bob".into(),
        amount: UInt(2),
    });
    assert!(check_effects("alice", &limits, &refund, &[]).is_err());
}

#[test]
fn cost_planes_and_failed_debits_are_independent() {
    let caps = Cost {
        logical: UInt(4),
        dust: UInt(10),
        service: UInt(2),
        external: UInt(3),
    };
    let mut meter = Meter::new(caps);
    meter.charge(CostPlane::Dust, UInt(7)).unwrap();
    assert!(meter.charge(CostPlane::Dust, UInt(4)).is_err());
    assert_eq!(meter.used().dust, UInt(7));
    meter.charge(CostPlane::Logical, UInt(4)).unwrap();
    assert!(meter.charge(CostPlane::Logical, UInt(1)).is_err());
}

#[test]
fn timeout_does_not_release_sponsor_capacity() {
    let mut grant = FeeCapability::new(
        "grant",
        "local",
        "anchor",
        "program",
        "pay",
        "alice",
        UInt(10),
    );
    grant
        .reserve(
            "attempt-a",
            "local",
            "anchor",
            "program",
            "pay",
            "alice",
            UInt(7),
        )
        .unwrap();
    assert!(
        grant
            .reserve(
                "attempt-b",
                "local",
                "anchor",
                "program",
                "pay",
                "alice",
                UInt(4)
            )
            .is_err()
    );
    grant.mark_unknown("attempt-a").unwrap();
    assert!(grant.release_unused("attempt-a").is_err());
    grant.settle("attempt-a", UInt(6)).unwrap();
    assert_eq!(grant.spent(), UInt(6));
    grant
        .reserve(
            "attempt-b",
            "local",
            "anchor",
            "program",
            "pay",
            "alice",
            UInt(4),
        )
        .unwrap();
    assert!(grant.settle("attempt-a", UInt(1)).is_err());
}

#[test]
fn role_policy_requires_current_epoch_distinct_members_and_exact_call() {
    use mori_core::policy::{FederationPolicy, Role, RoleGrant};
    use std::collections::{BTreeMap, BTreeSet};
    let mut policy = FederationPolicy {
        network: "local".into(),
        anchor: "anchor".into(),
        epoch: UInt(3),
        grants: BTreeMap::from([(
            Role::Clear,
            RoleGrant {
                members: BTreeSet::from(["a".into(), "b".into()]),
                quorum: 2,
                calls: BTreeSet::from([("program".into(), "clear".into())]),
            },
        )]),
        revoked: BTreeSet::new(),
    };
    let members = vec!["a".into(), "b".into()];
    policy
        .authorize(
            Role::Clear,
            &members,
            UInt(3),
            "local",
            "anchor",
            "program",
            "clear",
        )
        .unwrap();
    assert!(
        policy
            .authorize(
                Role::Prove,
                &members,
                UInt(3),
                "local",
                "anchor",
                "program",
                "clear"
            )
            .is_err()
    );
    assert!(
        policy
            .authorize(
                Role::Clear,
                &members,
                UInt(2),
                "local",
                "anchor",
                "program",
                "clear"
            )
            .is_err()
    );
    assert!(
        policy
            .authorize(
                Role::Clear,
                &members,
                UInt(3),
                "other",
                "anchor",
                "program",
                "clear"
            )
            .is_err()
    );
    assert!(
        policy
            .authorize(
                Role::Clear,
                &["a".into(), "a".into()],
                UInt(3),
                "local",
                "anchor",
                "program",
                "clear"
            )
            .is_err()
    );
    assert!(
        policy
            .authorize(
                Role::Clear,
                &members,
                UInt(3),
                "local",
                "anchor",
                "program",
                "relay"
            )
            .is_err()
    );
    policy.revoked.insert("b".into());
    assert!(
        policy
            .authorize(
                Role::Clear,
                &members,
                UInt(3),
                "local",
                "anchor",
                "program",
                "clear"
            )
            .is_err()
    );
}

#[test]
fn sponsor_scope_and_overspend_refusals_leave_capacity_unchanged() {
    let mut grant = FeeCapability::new(
        "grant",
        "local",
        "anchor",
        "program",
        "pay",
        "alice",
        UInt(10),
    );
    assert!(
        grant
            .reserve(
                "attempt",
                "local",
                "other-anchor",
                "program",
                "pay",
                "alice",
                UInt(7)
            )
            .is_err()
    );
    assert_eq!(grant.available().unwrap(), UInt(10));
    grant
        .reserve(
            "attempt",
            "local",
            "anchor",
            "program",
            "pay",
            "alice",
            UInt(7),
        )
        .unwrap();
    assert!(grant.settle("attempt", UInt(8)).is_err());
    assert_eq!(grant.available().unwrap(), UInt(3));
    assert_eq!(grant.spent(), UInt(0));
}
