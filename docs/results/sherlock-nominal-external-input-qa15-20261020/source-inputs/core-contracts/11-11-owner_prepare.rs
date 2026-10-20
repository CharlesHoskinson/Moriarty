fn cap_err_above_u128(kind: CapacityKind, limit: u128, locator: Option<LocatorC>) -> Tooling {
    Tooling::CapacityNotAdmitted { kind, needed: NeededNatC::AboveU128, limit, locator }
}
