pub struct CoreActionPrepared {
    pub preparation: ScalarActionReport,
    pub admitted: Admitted,
    pub image: MachineImage,
}

impl CoreActionPrepared {
    /// Read actual admission/image fields; no fabricated completed RunSummary.
    pub fn observe(&self) -> Result<HostObservation<'_>, HostViolation> {
        observe_bound_image(&self.admitted, &self.image)
    }
}

/// Prepare and admit the existing scalar projection without running it.
/// Existing preflight, generated-AST checks and admission order are retained.
pub fn prepare_scalar_action_host(
    source: &crate::Program,
    action_index: usize,
    inputs: &BTreeMap<String, crate::Value>,
    config: &ConsumerConfig,
) -> Result<CoreActionPrepared, ConsumerError> {
    let preflight = preflight_source(source, action_index, inputs, config)?;
    let mapped = prepare_scalar_action(source, action_index)
        .map_err(ConsumerError::SourceProjection)?;
    let (body_nodes, _) = check_generated(&mapped, &preflight, &config.host)?;

    let arguments: Result<Vec<ExprRef>, ConsumerError> = mapped.report.inputs.iter().map(|site| {
        // Preflight and check_generated already bind these same immutable names
        // to the declared scalar inputs. Keep the typed missing-input error on
        // this lookup too; do not fabricate a literal if the binding is absent.
        let value = inputs.get(&site.name).ok_or_else(|| {
            ConsumerError::HostGuard(HostGuardError::MissingInput { name: site.name.clone() })
        })?;
        let value = match value {
            crate::Value::UInt(value) => ScalarValue::UInt(value.0),
            crate::Value::Bool(value) => ScalarValue::Bool(*value),
        };
        Ok(Arc::new(Expr::Literal(value)))
    }).collect();
    let entry = Arc::new(Expr::Call(mapped.function.name.clone(), arguments?));
    check_combined_generated_ast_nodes(body_nodes, &entry, &config.host)?;

    let preparation = mapped.report;
    let canonical = ScalarProgram { functions: vec![mapped.function], entry };
    let owner_source = source_input_bridge::from_program(&canonical);
    let engine = host_owner_engine();
    let admitted = engine.admit(&owner_source, &config.bounds, &config.layout)
        .map_err(ConsumerError::Admit)?;
    let image = engine.admit_run(&admitted, &config.fuel, &config.stack, &FinanceInput::Absent)
        .map_err(ConsumerError::AdmitRun)?;

    Ok(CoreActionPrepared { preparation, admitted, image })
}

