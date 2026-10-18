// crates/mori-core/src/stage/plan.rs:7..11 at 74a909d69639df8823a527a4d608a7516ccd69a2
pub enum Origin {
    Input,
    Let,
    Event,
}

// crates/mori-core/src/stage/plan.rs:14..19 at 74a909d69639df8823a527a4d608a7516ccd69a2
pub struct CaptureSlot {
    pub name: String,
    pub ty: Type,
    pub origin: Origin,
    pub ordinal: u16,
}

// crates/mori-core/src/stage/plan.rs:22..25 at 74a909d69639df8823a527a4d608a7516ccd69a2
pub struct StageBinder {
    pub name: String,
    pub ty: Type,
}

// crates/mori-core/src/stage/plan.rs:35..41 at 74a909d69639df8823a527a4d608a7516ccd69a2
pub struct AwaitSite {
    pub site: u16,
    pub ordinal: u16,
    pub label: String,
    pub binders: Vec<StageBinder>,
    pub captures: Vec<CaptureSlot>,
}

// crates/mori-core/src/stage/plan.rs:43..50 at 74a909d69639df8823a527a4d608a7516ccd69a2
pub struct ActionPlan {
    pub action: String,
    pub statement_count: u16,
    pub sites: Vec<AwaitSite>,
    pub segment_cost: Vec<UInt>,
    pub committed_before: Vec<UInt>,
    pub plan_total: UInt,
}

// crates/mori-core/src/stage/plan.rs:52..56 at 74a909d69639df8823a527a4d608a7516ccd69a2
pub struct StagePlan {
    pub format: String,
    pub program_hash: String,
    pub actions: Vec<ActionPlan>,
}
