use mori_core::UInt;
use mori_core::policy::{Cost, CostPlane, WorkBudget};

fn caps() -> Cost {
    Cost {
        logical: UInt(10),
        dust: UInt(10),
        service: UInt(10),
        external: UInt(10),
    }
}

fn budget() -> WorkBudget {
    WorkBudget::new(caps())
}

fn meter_error(error: mori_core::Error, message: &str) {
    assert_eq!(error.code, "mori::core::meter");
    assert_eq!(error.message, message);
    assert_eq!(error.step, None);
}

fn plane(cost: &Cost, which: CostPlane) -> u128 {
    match which {
        CostPlane::Logical => cost.logical.0,
        CostPlane::Dust => cost.dust.0,
        CostPlane::Service => cost.service.0,
        CostPlane::External => cost.external.0,
    }
}

#[test]
fn cost_planes_remain_independent() {
    let mut budget = budget();
    budget
        .reserve_recovery(CostPlane::Logical, UInt(4))
        .unwrap();
    budget.charge_ordinary(CostPlane::Dust, UInt(10)).unwrap();
    budget.charge_ordinary(CostPlane::Service, UInt(9)).unwrap();
    budget
        .charge_ordinary(CostPlane::External, UInt(3))
        .unwrap();
    budget.charge_ordinary(CostPlane::Logical, UInt(6)).unwrap();
    budget.charge_recovery(CostPlane::Logical, UInt(4)).unwrap();
    assert_eq!(budget.ordinary_used().dust, UInt(10));
    assert_eq!(budget.ordinary_used().service, UInt(9));
    assert_eq!(budget.ordinary_used().external, UInt(3));
    assert_eq!(budget.ordinary_used().logical, UInt(6));
    assert_eq!(budget.recovery_reserved().logical, UInt(4));
    assert_eq!(budget.recovery_used().logical, UInt(4));
    assert_eq!(budget.recovery_reserved().dust, UInt(0));
    assert_eq!(budget.recovery_used().service, UInt(0));
    assert_eq!(budget.recovery_used().external, UInt(0));
    let saved = budget.clone();
    meter_error(
        budget
            .charge_ordinary(CostPlane::Dust, UInt(1))
            .unwrap_err(),
        "cost plane cap exceeded",
    );
    meter_error(
        budget
            .charge_recovery(CostPlane::Dust, UInt(1))
            .unwrap_err(),
        "recovery charge exceeds reserved slice",
    );
    meter_error(
        budget
            .charge_ordinary(CostPlane::Service, UInt(2))
            .unwrap_err(),
        "cost plane cap exceeded",
    );
    assert_eq!(budget, saved);
}

#[test]
fn ordinary_logical_work_cannot_consume_reserved_recovery() {
    let mut budget = budget();
    budget
        .reserve_recovery(CostPlane::Logical, UInt(4))
        .unwrap();
    let reserved = budget.clone();
    meter_error(
        budget
            .charge_ordinary(CostPlane::Logical, UInt(7))
            .unwrap_err(),
        "ordinary work would consume reserved recovery",
    );
    assert_eq!(budget, reserved);
    budget.charge_ordinary(CostPlane::Logical, UInt(6)).unwrap();
    assert_eq!(budget.ordinary_used().logical, UInt(6));
    assert_eq!(budget.recovery_reserved().logical, UInt(4));
    assert_eq!(budget.recovery_used().logical, UInt(0));
    meter_error(
        budget
            .charge_ordinary(CostPlane::Logical, UInt(1))
            .unwrap_err(),
        "ordinary work would consume reserved recovery",
    );
    assert_eq!(budget.ordinary_used().logical, UInt(6));
    budget.charge_recovery(CostPlane::Logical, UInt(4)).unwrap();
    assert_eq!(budget.ordinary_used().logical, UInt(6));
    assert_eq!(budget.recovery_used().logical, UInt(4));
    for which in [CostPlane::Dust, CostPlane::Service, CostPlane::External] {
        assert_eq!(plane(&budget.ordinary_used(), which), 0);
        assert_eq!(plane(&budget.recovery_reserved(), which), 0);
    }
}

#[test]
fn recovery_charge_cannot_use_ordinary_spare_capacity() {
    let mut budget = budget();
    budget
        .reserve_recovery(CostPlane::Service, UInt(3))
        .unwrap();
    let saved = budget.clone();
    meter_error(
        budget
            .charge_recovery(CostPlane::Service, UInt(4))
            .unwrap_err(),
        "recovery charge exceeds reserved slice",
    );
    assert_eq!(budget, saved);
    budget.charge_recovery(CostPlane::Service, UInt(3)).unwrap();
    assert_eq!(budget.ordinary_used().service, UInt(0));
    assert_eq!(budget.recovery_reserved().service, UInt(3));
    assert_eq!(budget.recovery_used().service, UInt(3));
    budget.charge_ordinary(CostPlane::Service, UInt(7)).unwrap();
    assert_eq!(budget.ordinary_used().service, UInt(7));
    meter_error(
        budget
            .charge_ordinary(CostPlane::Service, UInt(1))
            .unwrap_err(),
        "ordinary work would consume reserved recovery",
    );
    assert_eq!(budget.ordinary_used().service, UInt(7));
    assert_eq!(budget.recovery_used().service, UInt(3));
    assert_eq!(budget.ordinary_used().logical, UInt(0));
    assert_eq!(budget.recovery_reserved().logical, UInt(0));
}

#[test]
fn overflow_and_empty_recovery_reservation_keep_budget() {
    let mut budget = WorkBudget::new(Cost {
        logical: UInt(u128::MAX),
        dust: UInt(4),
        service: UInt(4),
        external: UInt(4),
    });
    budget.charge_ordinary(CostPlane::Logical, UInt(1)).unwrap();
    let charged = budget.clone();
    meter_error(
        budget
            .charge_ordinary(CostPlane::Logical, UInt(u128::MAX))
            .unwrap_err(),
        "cost counter overflow",
    );
    meter_error(
        budget
            .reserve_recovery(CostPlane::Logical, UInt(u128::MAX))
            .unwrap_err(),
        "cost counter overflow",
    );
    meter_error(
        budget
            .reserve_recovery(CostPlane::Dust, UInt(0))
            .unwrap_err(),
        "recovery reservation must be positive",
    );
    meter_error(
        budget
            .reserve_recovery(CostPlane::Dust, UInt(5))
            .unwrap_err(),
        "recovery reservation exceeds remaining capacity",
    );
    assert_eq!(budget, charged);
    budget.reserve_recovery(CostPlane::Dust, UInt(4)).unwrap();
    budget
        .charge_ordinary(CostPlane::Dust, UInt(1))
        .unwrap_err();
    assert_eq!(budget.ordinary_used().dust, UInt(0));
    assert_eq!(budget.recovery_reserved().dust, UInt(4));
    assert_eq!(budget.ordinary_used().logical, UInt(1));
}

#[test]
fn cumulative_plane_accounting_stays_inside_each_cap() {
    let mut budget = budget();
    budget
        .reserve_recovery(CostPlane::Logical, UInt(3))
        .unwrap();
    budget
        .reserve_recovery(CostPlane::External, UInt(2))
        .unwrap();
    budget.charge_ordinary(CostPlane::Logical, UInt(4)).unwrap();
    budget.charge_recovery(CostPlane::Logical, UInt(2)).unwrap();
    budget.charge_ordinary(CostPlane::Dust, UInt(10)).unwrap();
    budget.charge_ordinary(CostPlane::Service, UInt(1)).unwrap();
    budget
        .charge_ordinary(CostPlane::External, UInt(8))
        .unwrap();
    budget
        .charge_recovery(CostPlane::External, UInt(2))
        .unwrap();
    for which in [
        CostPlane::Logical,
        CostPlane::Dust,
        CostPlane::Service,
        CostPlane::External,
    ] {
        let ordinary = plane(&budget.ordinary_used(), which);
        let reserved = plane(&budget.recovery_reserved(), which);
        let recovery = plane(&budget.recovery_used(), which);
        assert!(recovery <= reserved);
        assert!(ordinary + reserved <= 10);
        assert!(ordinary + recovery <= 10);
    }
    let saved = budget.clone();
    meter_error(
        budget
            .charge_ordinary(CostPlane::Logical, UInt(4))
            .unwrap_err(),
        "ordinary work would consume reserved recovery",
    );
    meter_error(
        budget
            .reserve_recovery(CostPlane::Logical, UInt(4))
            .unwrap_err(),
        "recovery reservation exceeds remaining capacity",
    );
    assert_eq!(budget, saved);
}
