// DTO-only excerpt crates/mori-core/src/stage/mod.rs:94..97 at 9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb
pub struct NamedValue {
    pub name: String,
    pub value: Value,
}

// DTO-only excerpt crates/mori-core/src/stage/mod.rs:100..106 at 9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb
pub struct StageLogical {
    pub profile: String,
    /// Mandatory key; `None` is written as `null`.
    #[serde(deserialize_with = "Option::deserialize")]
    pub cap: Option<UInt>,
    pub committed: UInt,
}

// DTO-only excerpt crates/mori-core/src/stage/mod.rs:111..120 at 9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb
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

// DTO-only excerpt crates/mori-core/src/stage/mod.rs:123..128 at 9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb
pub struct StageEvent {
    pub format: String,
    pub action: String,
    pub label: String,
    pub values: Vec<NamedValue>,
}

// DTO-only excerpt crates/mori-core/src/stage/mod.rs:133..136 at 9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb
pub struct PendingFrame {
    pub site: u16,
    pub captures: Vec<NamedValue>,
}

// DTO-only excerpt crates/mori-core/src/stage/mod.rs:148..153 at 9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb
pub struct StageSegment {
    pub action: String,
    pub frame_pre: PendingFrame,
    pub frame_post: PendingFrame,
    pub execution: Execution,
}

// DTO-only excerpt crates/mori-core/src/stage/mod.rs:159..173 at 9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb
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

// DTO-only excerpt crates/mori-core/src/stage/mod.rs:177..181 at 9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb
pub struct StageRejection {
    pub error: Error,
    pub attempted: UInt,
    pub committed: UInt,
}
