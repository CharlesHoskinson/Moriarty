impl OwnerWire for owner_ext_type_builder::ExtTypeBuildError {
    fn owner_wire(&self) -> Value {
        match self {
            Self::CoreTooling(t) => json!({"CoreTooling": t.owner_wire()}),
            Self::ExtCapacityNotAdmitted { kind, needed, limit, locator } => json!({"ExtCapacityNotAdmitted": {"kind": kind.owner_wire(), "needed": needed.owner_wire(), "limit": limit.owner_wire(), "locator": locator.owner_wire()}}),
            Self::ExtAdmissionRejected { reason, detail, locator } => json!({"ExtAdmissionRejected": {"reason": reason.owner_wire(), "detail": detail.owner_wire(), "locator": locator.owner_wire()}}),
            Self::ExtUnsupportedFeature { feature, locator } => json!({"ExtUnsupportedFeature": {"feature": feature.owner_wire(), "locator": locator.owner_wire()}}),
            Self::Host(h) => json!({"Host": h.owner_wire()}),
        }
    }
}
