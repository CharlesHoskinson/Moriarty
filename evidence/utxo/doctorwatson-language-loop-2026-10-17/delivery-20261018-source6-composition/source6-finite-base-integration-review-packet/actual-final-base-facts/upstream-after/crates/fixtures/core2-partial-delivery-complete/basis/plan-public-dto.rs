// DTO-only excerpt crates/mori-core/src/stage/plan.rs:7..11 at 9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb
pub enum Origin {
    Input,
    Let,
    Event,
}

// DTO-only excerpt crates/mori-core/src/stage/plan.rs:14..19 at 9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb
pub struct CaptureSlot {
    pub name: String,
    pub ty: Type,
    pub origin: Origin,
    pub ordinal: u16,
}

// DTO-only excerpt crates/mori-core/src/stage/plan.rs:22..25 at 9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb
pub struct StageBinder {
    pub name: String,
    pub ty: Type,
}

// DTO-only excerpt crates/mori-core/src/stage/plan.rs:35..41 at 9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb
pub struct AwaitSite {
    pub site: u16,
    pub ordinal: u16,
    pub label: String,
    pub binders: Vec<StageBinder>,
    pub captures: Vec<CaptureSlot>,
}

// DTO-only excerpt crates/mori-core/src/stage/plan.rs:43..50 at 9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb
pub struct ActionPlan {
    pub action: String,
    pub statement_count: u16,
    pub sites: Vec<AwaitSite>,
    pub segment_cost: Vec<UInt>,
    pub committed_before: Vec<UInt>,
    pub plan_total: UInt,
}

// DTO-only excerpt crates/mori-core/src/stage/plan.rs:52..56 at 9a140f7eb7844ca2b952d8f4142a5cd7fd820dbb
pub struct StagePlan {
    pub format: String,
    pub program_hash: String,
    pub actions: Vec<ActionPlan>,
}
