fn cap_err(kind: CapacityKind, needed: u128, limit: u128, locator: Option<LocatorC>) -> Tooling {
    Tooling::CapacityNotAdmitted { kind, needed: NeededNatC::Exact(needed), limit, locator }
}
