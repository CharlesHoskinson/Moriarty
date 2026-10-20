impl OwnerWire for NeededNatC {
    fn owner_wire(&self) -> Value {
        match self {
            Self::Exact(item0) => json!({"Exact": item0.owner_wire()}),
            Self::AboveU128 => json!("AboveU128"),
        }
    }
}
