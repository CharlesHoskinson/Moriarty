fn field_loc(name: &str) -> Option<LocatorC> {
    Some(LocatorC::Field {
        name: name.to_string(),
    })
}
