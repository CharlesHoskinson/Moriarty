// crates/mori-core/src/stage/mod.rs:96..99 at 74a909d69639df8823a527a4d608a7516ccd69a2
pub struct NamedValue {
    pub name: String,
    pub value: Value,
}

// crates/mori-core/src/stage/mod.rs:102..108 at 74a909d69639df8823a527a4d608a7516ccd69a2
pub struct StageLogical {
    pub profile: String,
    /// Mandatory key; `None` is written as `null`.
    #[serde(deserialize_with = "Option::deserialize")]
    pub cap: Option<UInt>,
    pub committed: UInt,
}

// crates/mori-core/src/stage/mod.rs:113..122 at 74a909d69639df8823a527a4d608a7516ccd69a2
pub struct Continuation {
    pub format: String,
    pub program_hash: String,
    pub plan_hash: String,
    pub action: String,
    pub site: u16,
    pub captures: Vec<NamedValue>,
    pub logical: StageLogical,
    pub boundary: Snapshot,
}

// crates/mori-core/src/stage/mod.rs:125..130 at 74a909d69639df8823a527a4d608a7516ccd69a2
pub struct StageEvent {
    pub format: String,
    pub action: String,
    pub label: String,
    pub values: Vec<NamedValue>,
}

// crates/mori-core/src/stage/mod.rs:135..138 at 74a909d69639df8823a527a4d608a7516ccd69a2
pub struct PendingFrame {
    pub site: u16,
    pub captures: Vec<NamedValue>,
}

// crates/mori-core/src/stage/mod.rs:150..155 at 74a909d69639df8823a527a4d608a7516ccd69a2
pub struct StageSegment {
    pub action: String,
    pub frame_pre: PendingFrame,
    pub frame_post: PendingFrame,
    pub execution: Execution,
}

// crates/mori-core/src/stage/mod.rs:161..175 at 74a909d69639df8823a527a4d608a7516ccd69a2
pub enum StageOutcome {
    Waiting {
        advanced: bool,
        segment: Option<StageSegment>,
        continuation: Continuation,
        attempted: UInt,
        residual: Vec<Liability>,
    },
    Complete {
        segment: StageSegment,
        committed: UInt,
        attempted: UInt,
        residual: Vec<Liability>,
    },
}

// crates/mori-core/src/stage/mod.rs:179..183 at 74a909d69639df8823a527a4d608a7516ccd69a2
pub struct StageRejection {
    pub error: Error,
    pub attempted: UInt,
    pub committed: UInt,
}
