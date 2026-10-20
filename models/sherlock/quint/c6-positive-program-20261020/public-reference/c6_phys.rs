// PUBLIC REFERENCE ONLY c6_phys.rs:48-49
pub const C6_PHYS_MAGIC: u128 = 0x4336_5048_5953_31;
pub const C6_PHYS_VERSION: u128 = 1;

// PUBLIC REFERENCE ONLY c6_phys.rs:88-154
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum C6PhysError {
    BadMagic { actual: u128 },
    BadVersion { actual: u128 },
    Truncated { section: &'static str, needed: usize, actual: usize },
    TrailingLanes { section: &'static str, extra: usize },
    BadTag { section: &'static str, tag: u128 },
    BadBool { section: &'static str, value: u128 },
    NonZeroPadding { section: &'static str, lane: usize, value: u128 },
    UsizeOverflow { section: &'static str, value: u128 },
    CapacityExceeded { section: &'static str, needed: usize, limit: usize },
    WrongControl { section: &'static str },
    /// The owner would not select a Push row here (terminal, publication, fuel,
    /// helper or stale result), in owner priority order.
    NotPushSelected { section: &'static str },
    /// The state is outside the step-invariant domain this layer accepts.
    StepInvDomain { section: &'static str },
    /// Logical Accept, but the owner's physical frame check would refuse.
    PhysicalFrameCapacity { depth: usize, frames: usize },
    UnexpectedPushSelection { expected: PushSelection, actual: PushSelection },
    EventMismatch { section: &'static str },
    AfterMismatch { section: &'static str },
    CounterOverflow { section: &'static str },
}

pub type C6Result<T> = Result<T, C6PhysError>;

/// Representation caps. Counts are inclusive limits; indices are strictly below.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct C6PhysCaps {
    pub max_frames: usize,
    pub max_expressions: usize,
    pub max_names: usize,
    pub max_arms: usize,
    pub max_children: usize,
    pub max_value_slots: usize,
    pub max_constants: usize,
    pub max_data_nodes: usize,
    pub max_env_cells: usize,
    pub max_functions: usize,
    pub max_arity: usize,
    pub max_types: usize,
    pub max_occurrences: usize,
    pub max_occurrence_paths: usize,
    pub max_parameters: usize,
    pub max_patterns: usize,
    pub max_pattern_children: usize,
    pub max_constructors: usize,
    pub max_ctor_fields: usize,
    pub max_decl_templates: usize,
    pub max_plan_nodes: usize,
    pub max_plan_cases: usize,
    pub max_field_paths: usize,
    pub max_constant_fields: usize,
    pub max_name_bytes: usize,
    /// Codec work budget, not an admitted limit: type-expression nodes decoded.
    pub max_type_expr_nodes: usize,
    /// Codec work budget, not an admitted limit: declaration-template text bytes.
    pub max_template_bytes: usize,
}

/// Codec work budgets that have no admitted counterpart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct C6CodecBudgets {
    pub type_expr_nodes: usize,
    pub template_bytes: usize,
}

