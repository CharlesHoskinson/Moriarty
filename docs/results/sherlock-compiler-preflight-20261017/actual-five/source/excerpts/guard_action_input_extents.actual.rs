pub fn preflight_action_inputs<'a>(
    program: &'a crate::Program,
    action_index: usize,
    supplied: &BTreeMap<String, crate::Value>,
    config: &ConsumerConfig,
) -> Result<&'a crate::Action, ConsumerError> {
    validate_config(config).map_err(ConsumerError::HostGuard)?;
    let host = &config.host;
    let action = program.actions.get(action_index).ok_or(ConsumerError::HostGuard(
        HostGuardError::NoSuchAction { action_index, action_count: program.actions.len() },
    ))?;
    within(HostMeasure::ActionInputs, count(action.inputs.len()), host.max_action_inputs)
        .map_err(ConsumerError::HostGuard)?;
    within(HostMeasure::SuppliedInputs, count(supplied.len()), host.max_supplied_inputs)
        .map_err(ConsumerError::HostGuard)?;
    // Name-byte extents over declared and supplied names, allocation-free.
    let mut total_name_bytes: u128 = 0;
    for input in &action.inputs {
        let bytes = count(input.name.len());
        within(HostMeasure::InputNameBytes, bytes, host.max_input_name_bytes)
            .map_err(ConsumerError::HostGuard)?;
        total_name_bytes = total_name_bytes.checked_add(bytes).ok_or(ConsumerError::HostGuard(
            HostGuardError::CountOverflow { measure: HostMeasure::TotalInputNameBytes },
        ))?;
    }
    for name in supplied.keys() {
        let bytes = count(name.len());
        within(HostMeasure::InputNameBytes, bytes, host.max_input_name_bytes)
            .map_err(ConsumerError::HostGuard)?;
        total_name_bytes = total_name_bytes.checked_add(bytes).ok_or(ConsumerError::HostGuard(
            HostGuardError::CountOverflow { measure: HostMeasure::TotalInputNameBytes },
        ))?;
    }
    within(HostMeasure::TotalInputNameBytes, total_name_bytes, host.max_total_input_name_bytes)
        .map_err(ConsumerError::HostGuard)?;
    Ok(action)
}

