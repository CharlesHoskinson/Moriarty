pub fn admit_run(a: &Admitted, fuel: &RunIntC, stack_limit: &RunIntC,
    finance: &FinanceInput) -> Result<MachineImage, Tooling> {
    let bounds = a.body.bounds;
    let caps = a.body.layout.capacities;
    // R1 finance.
    if *finance != FinanceInput::Absent {
        return Err(Tooling::UnsupportedFeature {
            feature: FeatureKind::FinancialInput, locator: None });
    }
    // RunId (schema identity.RunId) from the original three-class inputs,
    // built before R2 returns and before R3/R4 convert fuel and stack_limit.
    let run_id = Arc::new(RunIdC { identity: a.body.identity.clone(),
        fuel_input: *fuel, stack_limit_input: *stack_limit });
    // R2 negative -> genesis_invalid, not a refusal.
    if matches!(fuel, RunIntC::Negative) || matches!(stack_limit, RunIntC::Negative) {
        return Ok(genesis(a, run_id, false, 0, 0));
    }
    // R3 fuel bound.
    let fuel_v: u128 = match fuel {
        RunIntC::Nat(v) => *v,
        RunIntC::NatAboveU128 => return Err(cap_err_above_u128(
            CapacityKind::MaxReferenceWork, bounds.max_reference_work, None)),
        // Already returned by R2 above; same genesis disposition, no panic.
        RunIntC::Negative => return Ok(genesis(a, run_id, false, 0, 0)),
    };
    if fuel_v > bounds.max_reference_work {
        return Err(cap_err(CapacityKind::MaxReferenceWork, fuel_v,
            bounds.max_reference_work, None));
    }
    // R4 stack bound.
    let stack_v: u128 = match stack_limit {
        RunIntC::Nat(v) => *v,
        RunIntC::NatAboveU128 => return Err(cap_err_above_u128(
            CapacityKind::Frames, bounds.frames as u128, None)),
        // Already returned by R2 above; same genesis disposition, no panic.
        RunIntC::Negative => return Ok(genesis(a, run_id, false, 0, 0)),
    };
    if stack_v > bounds.frames as u128 {
        return Err(cap_err(CapacityKind::Frames, stack_v, bounds.frames as u128, None));
    }
    // R5 value slots.
    if fuel_v > caps.value_slots as u128 {
        return Err(cap_err(CapacityKind::ValueSlots, fuel_v, caps.value_slots as u128, None));
    }
    // R6 genesis_valid. stack_v <= frames (usize) so conversion cannot fail;
    // fail closed instead of unwrapping.
    let stack_usize = usize::try_from(stack_v).map_err(|_| cap_err(
        CapacityKind::Frames, stack_v, bounds.frames as u128, None))?;
    Ok(genesis(a, run_id, true, fuel_v, stack_usize))
}
