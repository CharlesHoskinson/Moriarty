fn exceeded(measure: HostMeasure, required: Option<u128>, bound: usize) -> HostGuardError {
    HostGuardError::Exceeded { measure, required, bound: bound as u128 }
}

fn within(measure: HostMeasure, required: u128, bound: usize) -> Result<u128, HostGuardError> {
    if required <= bound as u128 {
        Ok(required)
    } else {
        Err(exceeded(measure, Some(required), bound))
    }
}

fn count(value: usize) -> u128 {
    value as u128
}

