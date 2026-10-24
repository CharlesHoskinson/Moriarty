pub trait OwnerEngine {
    fn prepare_core(&self, p: &CoreProgram, b: &BoundsC, l: &LayoutSchemaC)
        -> Result<ProgramView, Tooling>;
    fn check_program_view(&self, v: &ProgramView, b: &BoundsC, l: &LayoutSchemaC)
        -> Result<(), ProgramViewViolation>;
    fn admit(&self, p: &CoreProgram, b: &BoundsC, l: &LayoutSchemaC)
        -> Result<Admitted, Tooling>;
    fn admit_run(&self, a: &Admitted, fuel: &RunIntC, stack_limit: &RunIntC, finance: &FinanceInput)
        -> Result<MachineImage, Tooling>;
    fn rule_id_of(&self, raw: &RawRuleId) -> RuleIdC;
    fn raw_rule_id(&self, rule: &RuleIdC) -> RawRuleId;
    fn select(&self, pre: &PreImage<'_>) -> Result<Selected, HostViolation>;
    fn family_of(&self, rule: &RawRuleId) -> Option<FamilyId>; // dispatcher;
    // None for Disabled/ExtensionOnly -> Disabled/ExtensionRuleReached.
    // PS plan validation, P6 cap refusal, PS physical check and P7 apply are
    // NOT trait methods (F-TRAIT-1): no caller-authored Selected/EffectPlan
    // reaches them. owner_step alone runs AB,P0,P1,P2,PS,P3,P4,P6,P7 over the
    // owner-private free functions, so images mutate only through step/run.
    fn step(&self, a: &Admitted, img: &mut MachineImage)
        -> Result<StepResult, HostViolation>; // AB,P0,P1,P2,PS,P3,P4,P6,P7
    fn run(&self, a: &Admitted, img: &mut MachineImage,
        sink: &mut dyn FnMut(&StepEventC)) -> Result<RunSummary, HostViolation>;
    fn check_step_inv(&self, pre: &PreImage<'_>)
        -> Result<(), Vec<StepInvViolation>>;
}
