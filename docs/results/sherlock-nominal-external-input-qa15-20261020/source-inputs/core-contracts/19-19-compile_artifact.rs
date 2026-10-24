#[derive(Serialize)]
pub struct CompileFailure {
    pub stage: &'static str,
    pub actual_error: Value,
    pub evaluation_performed: bool,
    pub events_forwarded: String,
    pub completed: bool,
}
impl CompileFailure {
    pub fn input(stage: &'static str, error: impl Into<String>) -> Self {
        Self { stage, actual_error: json!({"message":error.into()}),
            evaluation_performed:false, events_forwarded:"0".into(), completed:false }
    }
    fn core(stage: &'static str, error: crate::Error) -> Self {
        Self { stage, actual_error:json!(error), evaluation_performed:false,
            events_forwarded:"0".into(), completed:false }
    }
    /// Preserve the bounded draft preparation stage and its complete typed error.
    fn draft_source(error: recursive_source::DraftSourceFailure) -> Self {
        use recursive_source::DraftSourceFailure;
        let (stage, actual_error) = match error {
            DraftSourceFailure::Source(error) => ("parse", draft_diagnostics::source(error)),
            DraftSourceFailure::Types(error) => ("typecheck", draft_diagnostics::types(error)),
            DraftSourceFailure::Copy(error) => ("copy", draft_diagnostics::copy(error)),
            DraftSourceFailure::Admit(error) => ("admit", error.owner_wire()),
            DraftSourceFailure::AdmitRun(error) => ("admit-run", error.owner_wire()),
        };
        Self { stage, actual_error, evaluation_performed:false,
            events_forwarded:"0".into(), completed:false }
    }
    fn consumer(error: ConsumerError) -> Self {
        let (stage, actual_error, evaluation_performed, events_forwarded, completed) = match error {
            ConsumerError::HostGuard(error) => ("host-guard", error.owner_wire(), false, 0, false),
            ConsumerError::CoreValidation(error) => ("parse", json!(error), false, 0, false),
            ConsumerError::SourceProjection(error) => ("source-projection", json!(error), false, 0, false),
            ConsumerError::Admit(error) => ("admit", error.owner_wire(), false, 0, false),
            ConsumerError::AdmitRun(error) => ("admit-run", error.owner_wire(), false, 0, false),
            ConsumerError::Run { violation, events_forwarded } =>
                ("run", violation.owner_wire(), true, events_forwarded, false),
            ConsumerError::Observation { violation, summary, events_forwarded } =>
                ("observation", json!({"violation":violation.owner_wire(),
                    "summary":summary.owner_wire()}), true, events_forwarded, true),
        };
        Self { stage, actual_error, evaluation_performed,
            events_forwarded:events_forwarded.to_string(), completed }
    }
}
