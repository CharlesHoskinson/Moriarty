//! Complete host section of sherlock-c6-phys/1 (C6P1), Chunk 4.
//!
//! Layout `HOST`, schema `C6_HOST_SCHEMA`:
//! - 0 magic, 1 version, 2 `HOST_SECTION_TAG`, 3 `C6_HOST_SCHEMA`, 4 child count (4);
//! - 5..=12 directory (tag, exact length): META, PROG, CTX, STATE in that order;
//! - 13 reserved, zero; then the four children contiguously, nothing after.
//! META: magic, version, `META_SECTION_TAG`; admission identity; admission bounds
//!   (22, independent of layout.bounds); layout: version pair, opt field_id, opt
//!   widths (three raw ordered lists of (text, u32), duplicates kept), opt
//!   scalar_layout_id, bounds (22), capacities (4), boundary bool; run_id identity,
//!   fuel_input, stack_limit_input. Text/bytes: length lane then one lane per byte.
//!   Opt: 0 | 1 payload. Digest: 0 | 1 format pair, bytes (opaque, unverified).
//!   Identity: profile u8, grammar 0/1, finance 0/1, layout_version, layout_digest,
//!   bounds_digest, opt field_id, membership digest. RunInt [0,0] Negative,
//!   [1,n] Nat, [2,0] NatAboveU128. Bool 0/1; u8/u32/usize refuse overflow.
//! CTX (22): magic, version, `CTX_SECTION_TAG`, ext presence, 16 ExtCapacitiesC
//!   fields in declaration order (zero when absent), budgets type_expr_nodes,
//!   template_bytes. Reserved financial/persistent caps are representation only.
//! PROG: exact `encode_program`. STATE: exact 3C `encode_state`.
//!
//! `decode_host` first-error order: 1. total lane limit (`LimitExceeded`);
//! 2. fewer than 14 lanes `Truncated`; 3. `BadMagic`; 4. `BadVersion`; 5. tag;
//! 6. schema; 7. count; per child in order: tag, length, minimum, offset sum;
//! reserved lane; 8. `Truncated`, `TrailingLanes` (no allocation before);
//! 9. META, CTX, then config comparison (layout, context.ext, context.budgets),
//! then PROG and STATE with caps derived only from the caller config.
//! META first-error order is lane-stream order; at a string: length, per-string
//! limit, cumulative metadata bytes, remaining lanes, bytes, UTF-8.
//!
//! Decoding constructs only owned diagnostic snapshots. Binding compares against
//! a real fixed-owner capture of an AB-bound pair; it establishes program/admission/
//! run-input membership, not StepInv, reachability, transitions or import.

use super::c6_phys::{C6CodecBudgets, C6PhysError, C6Result, C6Root, C6_PHYS_MAGIC, C6_PHYS_VERSION};
use super::c6_phys_data::{cap_count, rd, rd_bool, u, zero, C6DataTables};
use super::c6_phys_helper::C6HelperTables;
use super::c6_phys_program::{decode_program, encode_program, PROGRAM_HEADER_LANES, PROGRAM_SECTION_TAG};
use super::c6_phys_state::{
    compare_uncached_witness, decode_state, encode_state, retained_layout_agrees_with_host, C6OwnedState,
    C6StateContext, C6StateError, C6StateLimits, C6WitnessError, C6WitnessPreservation, STATE_HEADER_LANES,
    STATE_SECTION_TAG,
};
use super::complete_observation::{capture, AdmissionSnapshot, HostSnapshot, ImageSnapshot, LayoutSnapshot, WidthsSnapshot};
use super::shared::host_implied_prefix::SegBase;
use super::shared::{
    Admitted, BoundsC, DigestC, ExtCapacitiesC, FinanceInput, GrammarDomain, HostViolation, IdentityC,
    LayoutCapacitiesC, LayoutSchemaC, MachineImage, RunIdC, RunIntC,
};
use super::witness_plan::{
    construct_plan_for, evaluate_uncached, LifetimeUnit, WitnessEvaluation, WitnessFailure, WitnessPlan,
};

pub const HOST_SECTION_TAG: u128 = 0x484F_5354;
pub const META_SECTION_TAG: u128 = 0x4D45_5441;
pub const CTX_SECTION_TAG: u128 = 0x4354_5854;
pub const C6_HOST_SCHEMA: u128 = 1;
pub const HOST_CHILD_COUNT: u128 = 4;
pub const HOST_HEADER_LANES: usize = 14;
pub const META_HEADER_LANES: usize = 3;
pub const CTX_LANES: usize = 22;
const HOST_S: &str = "host";
const META_S: &str = "host.meta";
const CTX_S: &str = "host.context";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C6HostChild { Metadata, Program, Context, State }

impl C6HostChild {
    pub const ALL: [Self; 4] = [Self::Metadata, Self::Program, Self::Context, Self::State];
    const fn index(self) -> usize { match self { Self::Metadata => 0, Self::Program => 1, Self::Context => 2, Self::State => 3 } }
    pub const fn tag(self) -> u128 {
        match self { Self::Metadata => META_SECTION_TAG, Self::Program => PROGRAM_SECTION_TAG,
            Self::Context => CTX_SECTION_TAG, Self::State => STATE_SECTION_TAG }
    }
    pub const fn minimum_lanes(self) -> usize {
        match self { Self::Metadata => META_HEADER_LANES, Self::Program => PROGRAM_HEADER_LANES,
            Self::Context => CTX_LANES, Self::State => STATE_HEADER_LANES }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum C6HostError {
    Outer(C6PhysError),
    LimitExceeded { scope: &'static str, needed: usize, limit: usize },
    BadSectionCount { actual: u128 },
    BadChildTag { child: C6HostChild, expected: u128, actual: u128 },
    ChildLengthRange { child: C6HostChild, declared: usize, minimum: usize },
    Child { child: C6HostChild, offset: usize, error: C6PhysError },
    State { offset: usize, error: C6StateError },
    /// The encoded or host configuration differs from the caller's original one.
    ConfigMismatch { field: &'static str },
}
pub type C6HostResult<T> = Result<T, C6HostError>;

fn outer(error: C6PhysError) -> C6HostError { C6HostError::Outer(error) }
fn child_err(child: C6HostChild, offset: usize, error: C6PhysError) -> C6HostError {
    C6HostError::Child { child, offset, error }
}
fn add(a: usize, b: usize) -> C6HostResult<usize> {
    a.checked_add(b).ok_or(outer(C6PhysError::UsizeOverflow { section: HOST_S, value: u(b) }))
}

/// External caller authority; decoded lanes never raise these.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct C6HostLimits {
    pub max_total_lanes: usize,
    /// Passed to 3C as `C6StateLimits::max_total_lanes`.
    pub max_state_lanes: usize,
    /// Cumulative metadata text plus selected-digest bytes.
    pub max_metadata_bytes: usize,
    pub max_string_bytes: usize,
    /// Cumulative entries across the three width lists.
    pub max_width_entries: usize,
}

fn admit_total(needed: usize, limits: &C6HostLimits) -> C6HostResult<()> {
    let limit = limits.max_total_lanes;
    if needed > limit { Err(C6HostError::LimitExceeded { scope: "host.total_lanes", needed, limit }) } else { Ok(()) }
}

/// The ORIGINAL caller configuration: every layout field (raw widths, order kept),
/// the original ext capacities (absent distinct from present zero) and budgets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C6HostConfig { pub layout: LayoutSnapshot, pub ext: Option<ExtCapacitiesC>, pub budgets: C6CodecBudgets }

impl C6HostConfig {
    pub fn from_retained_layout(layout: &LayoutSchemaC, budgets: C6CodecBudgets) -> Self {
        let LayoutSchemaC { layout_version, field_id, widths, scalar_layout_id, bounds, capacities, ext_capacities,
            boundary_capacities_present } = layout;
        let widths = widths.as_ref().map(|w| WidthsSnapshot { index_widths: w.index_width_entries().to_vec(),
            counter_widths: w.counter_width_entries().to_vec(), record_widths: w.record_width_entries().to_vec() });
        Self { layout: LayoutSnapshot { layout_version: layout_version.clone(), field_id: field_id.clone(), widths,
            scalar_layout_id: scalar_layout_id.clone(), bounds: *bounds, capacities: *capacities,
            boundary_capacities_present: *boundary_capacities_present }, ext: *ext_capacities, budgets }
    }
    /// Effective 3C context through the real `from_parts` derivation.
    pub fn state_context(&self, limits: &C6HostLimits) -> C6StateContext {
        C6StateContext::from_parts(&self.layout.bounds, &self.layout.capacities, self.ext, self.budgets,
            C6StateLimits { max_total_lanes: limits.max_state_lanes })
    }
}

/// Every non-program admission field plus the complete run identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C6HostMetadata { pub identity: IdentityC, pub bounds: BoundsC, pub layout: LayoutSnapshot, pub run_id: RunIdC }

impl C6HostMetadata {
    pub fn from_host(host: &HostSnapshot) -> Self {
        let AdmissionSnapshot { identity, bounds, view: _, layout } = &host.admission;
        Self { identity: identity.clone(), bounds: *bounds, layout: layout.clone(), run_id: host.image.run_id.clone() }
    }
}

struct MetaWriter<'a> { out: Vec<u128>, limits: &'a C6HostLimits, bytes: usize, width_entries: usize }
impl MetaWriter<'_> {
    fn v(&mut self, x: u128) { self.out.push(x); }
    fn us(&mut self, x: usize) { self.v(u(x)); }
    fn blob(&mut self, data: &[u8]) -> C6Result<()> {
        cap_count("host.meta.string_bytes", data.len(), self.limits.max_string_bytes)?;
        let next = self.bytes.checked_add(data.len())
            .ok_or(C6PhysError::UsizeOverflow { section: META_S, value: u(data.len()) })?;
        cap_count("host.meta.bytes", next, self.limits.max_metadata_bytes)?;
        self.bytes = next;
        self.us(data.len());
        self.out.extend(data.iter().map(|&b| u128::from(b)));
        Ok(())
    }
    fn pair(&mut self, p: &(String, String)) -> C6Result<()> { self.blob(p.0.as_bytes())?; self.blob(p.1.as_bytes()) }
    fn opt_pair(&mut self, p: &Option<(String, String)>) -> C6Result<()> {
        match p { None => { self.v(0); Ok(()) } Some(p) => { self.v(1); self.pair(p) } }
    }
    fn digest(&mut self, d: &DigestC) -> C6Result<()> {
        match d {
            DigestC::Unselected => { self.v(0); Ok(()) }
            DigestC::Selected { format, bytes } => { self.v(1); self.pair(format)?; self.blob(bytes) }
        }
    }
    fn identity(&mut self, i: &IdentityC) -> C6Result<()> {
        let IdentityC { profile_id, grammar_domain, finance, layout_version, layout_digest, bounds_digest, field_id,
            program_membership_digest } = i;
        self.v(u128::from(*profile_id));
        self.v(match grammar_domain { GrammarDomain::DedicatedSorts => 0, GrammarDomain::GenericNominalExtensionReserved => 1 });
        self.v(match finance { FinanceInput::Absent => 0, FinanceInput::TypedExtensionReserved => 1 });
        self.pair(layout_version)?;
        self.digest(layout_digest)?;
        self.digest(bounds_digest)?;
        self.opt_pair(field_id)?;
        self.digest(program_membership_digest)
    }
    fn bounds(&mut self, b: &BoundsC) {
        let BoundsC { code_rows, function_rows, code_child_slots, parameter_slots, pattern_rows, match_plan_rows,
            match_arm_rows, type_rows, constructor_rows, constructor_field_slots, max_arity, constant_nodes,
            constant_field_slots, data_nodes, data_field_slots, environment_cells, frames, helper_tasks,
            serialization_bytes, max_reference_work, max_helper_work, step_cap } = *b;
        for x in [code_rows, function_rows, code_child_slots, parameter_slots, pattern_rows, match_plan_rows,
            match_arm_rows, type_rows, constructor_rows, constructor_field_slots, max_arity, constant_nodes,
            constant_field_slots, data_nodes, data_field_slots, environment_cells, frames, helper_tasks,
            serialization_bytes] { self.us(x); }
        for x in [max_reference_work, max_helper_work, step_cap] { self.v(x); }
    }
    fn widths(&mut self, w: &Option<WidthsSnapshot>) -> C6Result<()> {
        let Some(WidthsSnapshot { index_widths, counter_widths, record_widths }) = w else { self.v(0); return Ok(()); };
        self.v(1);
        for list in [index_widths, counter_widths, record_widths] {
            self.width_entries = self.width_entries.checked_add(list.len())
                .ok_or(C6PhysError::UsizeOverflow { section: META_S, value: u(list.len()) })?;
            cap_count("host.meta.width_entries", self.width_entries, self.limits.max_width_entries)?;
            self.us(list.len());
            for (name, width) in list { self.blob(name.as_bytes())?; self.v(u128::from(*width)); }
        }
        Ok(())
    }
    fn run_int(&mut self, r: RunIntC) {
        let (t, n) = match r { RunIntC::Negative => (0, 0), RunIntC::Nat(n) => (1, n), RunIntC::NatAboveU128 => (2, 0) };
        self.v(t);
        self.v(n);
    }
}

pub fn encode_metadata(m: &C6HostMetadata, limits: &C6HostLimits) -> C6Result<Vec<u128>> {
    let C6HostMetadata { identity, bounds, layout, run_id } = m;
    let LayoutSnapshot { layout_version, field_id, widths, scalar_layout_id, bounds: layout_bounds, capacities,
        boundary_capacities_present } = layout;
    let RunIdC { identity: run_identity, fuel_input, stack_limit_input } = run_id;
    let LayoutCapacitiesC { name_rows, name_bytes, occurrence_path_slots, value_slots } = *capacities;
    let mut w = MetaWriter { out: vec![C6_PHYS_MAGIC, C6_PHYS_VERSION, META_SECTION_TAG], limits, bytes: 0, width_entries: 0 };
    w.identity(identity)?;
    w.bounds(bounds);
    w.pair(layout_version)?;
    w.opt_pair(field_id)?;
    w.widths(widths)?;
    w.opt_pair(scalar_layout_id)?;
    w.bounds(layout_bounds);
    for x in [name_rows, name_bytes, occurrence_path_slots, value_slots] { w.us(x); }
    w.v(u128::from(*boundary_capacities_present));
    w.identity(run_identity)?;
    w.run_int(*fuel_input);
    w.run_int(*stack_limit_input);
    Ok(w.out)
}

struct MetaReader<'a> { l: &'a [u128], at: usize, limits: &'a C6HostLimits, bytes: usize, width_entries: usize }
impl MetaReader<'_> {
    fn raw(&mut self) -> C6Result<u128> {
        let v = *self.l.get(self.at)
            .ok_or(C6PhysError::Truncated { section: META_S, needed: self.at + 1, actual: self.l.len() })?;
        self.at += 1;
        Ok(v)
    }
    fn us(&mut self) -> C6Result<usize> { let v = self.raw()?; rd(META_S, v) }
    fn remaining(&self) -> usize { self.l.len() - self.at }
    fn tag(&mut self, max: u128) -> C6Result<u128> {
        let t = self.raw()?;
        if t > max { Err(C6PhysError::BadTag { section: META_S, tag: t }) } else { Ok(t) }
    }
    fn truncated(&self, len: usize) -> C6PhysError {
        C6PhysError::Truncated { section: META_S, needed: self.at.saturating_add(len), actual: self.l.len() }
    }
    fn blob(&mut self) -> C6Result<Vec<u8>> {
        let len = self.us()?;
        cap_count("host.meta.string_bytes", len, self.limits.max_string_bytes)?;
        let next = self.bytes.checked_add(len).ok_or(C6PhysError::UsizeOverflow { section: META_S, value: u(len) })?;
        cap_count("host.meta.bytes", next, self.limits.max_metadata_bytes)?;
        self.bytes = next;
        if len > self.remaining() { return Err(self.truncated(len)); }
        let mut out = Vec::with_capacity(len);
        for _ in 0..len {
            let lane = self.raw()?;
            out.push(u8::try_from(lane).map_err(|_| C6PhysError::BadTag { section: META_S, tag: lane })?);
        }
        Ok(out)
    }
    fn text(&mut self) -> C6Result<String> {
        let bytes = self.blob()?;
        let len = bytes.len();
        String::from_utf8(bytes).map_err(|_| C6PhysError::BadTag { section: META_S, tag: u(len) })
    }
    fn pair(&mut self) -> C6Result<(String, String)> { Ok((self.text()?, self.text()?)) }
    fn opt_pair(&mut self) -> C6Result<Option<(String, String)>> {
        Ok(if self.tag(1)? == 0 { None } else { Some(self.pair()?) })
    }
    fn digest(&mut self) -> C6Result<DigestC> {
        Ok(if self.tag(1)? == 0 { DigestC::Unselected } else { DigestC::Selected { format: self.pair()?, bytes: self.blob()? } })
    }
    fn identity(&mut self) -> C6Result<IdentityC> {
        let profile = self.raw()?;
        let profile_id = u8::try_from(profile).map_err(|_| C6PhysError::UsizeOverflow { section: META_S, value: profile })?;
        let grammar_domain = if self.tag(1)? == 0 { GrammarDomain::DedicatedSorts } else { GrammarDomain::GenericNominalExtensionReserved };
        let finance = if self.tag(1)? == 0 { FinanceInput::Absent } else { FinanceInput::TypedExtensionReserved };
        Ok(IdentityC { profile_id, grammar_domain, finance, layout_version: self.pair()?, layout_digest: self.digest()?,
            bounds_digest: self.digest()?, field_id: self.opt_pair()?, program_membership_digest: self.digest()? })
    }
    fn bounds(&mut self) -> C6Result<BoundsC> {
        Ok(BoundsC { code_rows: self.us()?, function_rows: self.us()?, code_child_slots: self.us()?,
            parameter_slots: self.us()?, pattern_rows: self.us()?, match_plan_rows: self.us()?,
            match_arm_rows: self.us()?, type_rows: self.us()?, constructor_rows: self.us()?,
            constructor_field_slots: self.us()?, max_arity: self.us()?, constant_nodes: self.us()?,
            constant_field_slots: self.us()?, data_nodes: self.us()?, data_field_slots: self.us()?,
            environment_cells: self.us()?, frames: self.us()?, helper_tasks: self.us()?,
            serialization_bytes: self.us()?, max_reference_work: self.raw()?, max_helper_work: self.raw()?,
            step_cap: self.raw()? })
    }
    fn width_list(&mut self) -> C6Result<Vec<(String, u32)>> {
        let n = self.us()?;
        self.width_entries = self.width_entries.checked_add(n).ok_or(C6PhysError::UsizeOverflow { section: META_S, value: u(n) })?;
        cap_count("host.meta.width_entries", self.width_entries, self.limits.max_width_entries)?;
        if n > self.remaining() { return Err(self.truncated(n)); }
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            let name = self.text()?;
            let w = self.raw()?;
            out.push((name, u32::try_from(w).map_err(|_| C6PhysError::UsizeOverflow { section: META_S, value: w })?));
        }
        Ok(out)
    }
    fn widths(&mut self) -> C6Result<Option<WidthsSnapshot>> {
        if self.tag(1)? == 0 { return Ok(None); }
        Ok(Some(WidthsSnapshot { index_widths: self.width_list()?, counter_widths: self.width_list()?,
            record_widths: self.width_list()? }))
    }
    fn run_int(&mut self) -> C6Result<RunIntC> {
        let t = self.tag(2)?;
        let n = self.raw()?;
        if t != 1 && n != 0 { return Err(C6PhysError::NonZeroPadding { section: META_S, lane: self.at - 1, value: n }); }
        Ok(match t { 0 => RunIntC::Negative, 1 => RunIntC::Nat(n), _ => RunIntC::NatAboveU128 })
    }
}

pub fn decode_metadata(l: &[u128], limits: &C6HostLimits) -> C6Result<C6HostMetadata> {
    if l.len() < META_HEADER_LANES {
        return Err(C6PhysError::Truncated { section: META_S, needed: META_HEADER_LANES, actual: l.len() });
    }
    if l[0] != C6_PHYS_MAGIC { return Err(C6PhysError::BadMagic { actual: l[0] }); }
    if l[1] != C6_PHYS_VERSION { return Err(C6PhysError::BadVersion { actual: l[1] }); }
    if l[2] != META_SECTION_TAG { return Err(C6PhysError::BadTag { section: META_S, tag: l[2] }); }
    let mut r = MetaReader { l, at: META_HEADER_LANES, limits, bytes: 0, width_entries: 0 };
    let identity = r.identity()?;
    let bounds = r.bounds()?;
    let layout = LayoutSnapshot { layout_version: r.pair()?, field_id: r.opt_pair()?, widths: r.widths()?,
        scalar_layout_id: r.opt_pair()?, bounds: r.bounds()?,
        capacities: LayoutCapacitiesC { name_rows: r.us()?, name_bytes: r.us()?, occurrence_path_slots: r.us()?,
            value_slots: r.us()? },
        boundary_capacities_present: { let v = r.raw()?; rd_bool(META_S, v)? } };
    let run_id = RunIdC { identity: r.identity()?, fuel_input: r.run_int()?, stack_limit_input: r.run_int()? };
    if r.at != l.len() { return Err(C6PhysError::TrailingLanes { section: META_S, extra: l.len() - r.at }); }
    Ok(C6HostMetadata { identity, bounds, layout, run_id })
}

pub fn encode_context(ext: Option<ExtCapacitiesC>, budgets: C6CodecBudgets) -> [u128; CTX_LANES] {
    let mut o = [0u128; CTX_LANES];
    o[..3].copy_from_slice(&[C6_PHYS_MAGIC, C6_PHYS_VERSION, CTX_SECTION_TAG]);
    if let Some(e) = ext {
        let ExtCapacitiesC { plan_case_slots, field_path_slots, pattern_child_slots, input_rows, asset_rows,
            state_rows, account_rows, liability_rows, await_site_rows, await_binder_slots, balance_cells,
            journal_rows, pending_op_rows, observations_per_invocation, max_lifetime_work, resume_rows } = e;
        o[3] = 1;
        o[4..18].copy_from_slice(&[plan_case_slots, field_path_slots, pattern_child_slots, input_rows, asset_rows,
            state_rows, account_rows, liability_rows, await_site_rows, await_binder_slots, balance_cells,
            journal_rows, pending_op_rows, observations_per_invocation].map(u));
        o[18] = max_lifetime_work;
        o[19] = u(resume_rows);
    }
    let C6CodecBudgets { type_expr_nodes, template_bytes } = budgets;
    o[20] = u(type_expr_nodes);
    o[21] = u(template_bytes);
    o
}

pub fn decode_context(l: &[u128]) -> C6Result<(Option<ExtCapacitiesC>, C6CodecBudgets)> {
    if l.len() < CTX_LANES { return Err(C6PhysError::Truncated { section: CTX_S, needed: CTX_LANES, actual: l.len() }); }
    if l.len() > CTX_LANES { return Err(C6PhysError::TrailingLanes { section: CTX_S, extra: l.len() - CTX_LANES }); }
    if l[0] != C6_PHYS_MAGIC { return Err(C6PhysError::BadMagic { actual: l[0] }); }
    if l[1] != C6_PHYS_VERSION { return Err(C6PhysError::BadVersion { actual: l[1] }); }
    if l[2] != CTX_SECTION_TAG { return Err(C6PhysError::BadTag { section: CTX_S, tag: l[2] }); }
    let f = |k: usize| rd(CTX_S, l[4 + k]);
    let ext = if rd_bool(CTX_S, l[3])? {
        Some(ExtCapacitiesC { plan_case_slots: f(0)?, field_path_slots: f(1)?, pattern_child_slots: f(2)?,
            input_rows: f(3)?, asset_rows: f(4)?, state_rows: f(5)?, account_rows: f(6)?, liability_rows: f(7)?,
            await_site_rows: f(8)?, await_binder_slots: f(9)?, balance_cells: f(10)?, journal_rows: f(11)?,
            pending_op_rows: f(12)?, observations_per_invocation: f(13)?, max_lifetime_work: l[18],
            resume_rows: f(15)? })
    } else {
        zero(CTX_S, l, 4, 20)?;
        None
    };
    Ok((ext, C6CodecBudgets { type_expr_nodes: f(16)?, template_bytes: f(17)? }))
}

/// Canonical encoder. The host layout must equal the caller's original layout.
pub fn encode_host(host: &HostSnapshot, config: &C6HostConfig, limits: &C6HostLimits) -> C6HostResult<Vec<u128>> {
    use C6HostChild::*;
    if host.admission.layout != config.layout { return Err(C6HostError::ConfigMismatch { field: "layout" }); }
    let ctx = config.state_context(limits);
    let mut offset = HOST_HEADER_LANES;
    let meta = encode_metadata(&C6HostMetadata::from_host(host), limits).map_err(|e| child_err(Metadata, offset, e))?;
    offset = add(offset, meta.len())?;
    let prog = encode_program(&host.admission.view, &ctx.caps().phys).map_err(|e| child_err(Program, offset, e))?;
    offset = add(offset, prog.len())?;
    let context = encode_context(config.ext, config.budgets).to_vec();
    offset = add(offset, CTX_LANES)?;
    let state = encode_state(&C6OwnedState::from_image(&host.image), &ctx)
        .map_err(|error| C6HostError::State { offset, error })?;
    let total = add(offset, state.len())?;
    admit_total(total, limits)?;
    let parts = [meta, prog, context, state];
    let mut out = Vec::with_capacity(total);
    out.extend([C6_PHYS_MAGIC, C6_PHYS_VERSION, HOST_SECTION_TAG, C6_HOST_SCHEMA, HOST_CHILD_COUNT]);
    for (child, part) in C6HostChild::ALL.into_iter().zip(&parts) { out.push(child.tag()); out.push(u(part.len())); }
    out.push(0);
    for part in parts { out.extend(part); }
    Ok(out)
}

/// Owned diagnostic decode result. Not admitted; carries the caller config used.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C6DecodedHost { host: HostSnapshot, config: C6HostConfig }
impl C6DecodedHost {
    pub fn host(&self) -> &HostSnapshot { &self.host }
    pub fn config(&self) -> &C6HostConfig { &self.config }
    pub fn into_host(self) -> HostSnapshot { self.host }
}

/// Exhaustive reconstruction; a new image or 3C field is a compile error here.
fn image_from_parts(run_id: RunIdC, state: C6OwnedState) -> ImageSnapshot {
    let C6OwnedState { root, frames, data, helper } = state;
    let C6Root { control, control_origin, env_root, stack_limit, outcome, initial_fuel, remaining_work, consumed_work,
        helper_work, helper_visits, reference_cursor, alloc, active, seg_active, seg_alloc, seg_consumed_work } = root;
    let C6DataTables { data, data_fields, env, value_slots } = data;
    let C6HelperTables { helper, result, tasks } = helper;
    ImageSnapshot { run_id, control, control_origin, env_root, stack_limit, outcome, initial_fuel, remaining_work,
        consumed_work, helper_work, helper_visits, reference_cursor, alloc, active, helper, result,
        seg_base: SegBase::from_parts(seg_active, seg_alloc, seg_consumed_work), data, data_fields, env, frames,
        tasks, value_slots }
}

/// Standalone inverse; see the module documentation for the first-error order.
pub fn decode_host(lanes: &[u128], config: &C6HostConfig, limits: &C6HostLimits) -> C6HostResult<C6DecodedHost> {
    use C6HostChild::*;
    admit_total(lanes.len(), limits)?;
    if lanes.len() < HOST_HEADER_LANES {
        return Err(outer(C6PhysError::Truncated { section: HOST_S, needed: HOST_HEADER_LANES, actual: lanes.len() }));
    }
    if lanes[0] != C6_PHYS_MAGIC { return Err(outer(C6PhysError::BadMagic { actual: lanes[0] })); }
    if lanes[1] != C6_PHYS_VERSION { return Err(outer(C6PhysError::BadVersion { actual: lanes[1] })); }
    if lanes[2] != HOST_SECTION_TAG { return Err(outer(C6PhysError::BadTag { section: HOST_S, tag: lanes[2] })); }
    if lanes[3] != C6_HOST_SCHEMA { return Err(outer(C6PhysError::BadTag { section: "host.schema", tag: lanes[3] })); }
    if lanes[4] != HOST_CHILD_COUNT { return Err(C6HostError::BadSectionCount { actual: lanes[4] }); }
    let mut o = [HOST_HEADER_LANES; 5];
    for child in C6HostChild::ALL {
        let at = 5 + 2 * child.index();
        if lanes[at] != child.tag() {
            return Err(C6HostError::BadChildTag { child, expected: child.tag(), actual: lanes[at] });
        }
        let declared = rd(HOST_S, lanes[at + 1]).map_err(outer)?;
        if declared < child.minimum_lanes() {
            return Err(C6HostError::ChildLengthRange { child, declared, minimum: child.minimum_lanes() });
        }
        o[child.index() + 1] = add(o[child.index()], declared)?;
    }
    zero(HOST_S, lanes, 13, 14).map_err(outer)?;
    if lanes.len() < o[4] { return Err(outer(C6PhysError::Truncated { section: HOST_S, needed: o[4], actual: lanes.len() })); }
    if lanes.len() > o[4] { return Err(outer(C6PhysError::TrailingLanes { section: HOST_S, extra: lanes.len() - o[4] })); }
    let meta = decode_metadata(&lanes[o[0]..o[1]], limits).map_err(|e| child_err(Metadata, o[0], e))?;
    let (ext, budgets) = decode_context(&lanes[o[2]..o[3]]).map_err(|e| child_err(Context, o[2], e))?;
    for (same, field) in [(meta.layout == config.layout, "layout"), (ext == config.ext, "context.ext"),
        (budgets == config.budgets, "context.budgets")] {
        if !same { return Err(C6HostError::ConfigMismatch { field }); }
    }
    let ctx = config.state_context(limits);
    let view = decode_program(&lanes[o[1]..o[2]], &ctx.caps().phys).map_err(|e| child_err(Program, o[1], e))?;
    let state = decode_state(&lanes[o[3]..o[4]], &ctx).map_err(|error| C6HostError::State { offset: o[3], error })?;
    let C6HostMetadata { identity, bounds, layout, run_id } = meta;
    Ok(C6DecodedHost { host: HostSnapshot { admission: AdmissionSnapshot { identity, bounds, view, layout },
        image: image_from_parts(run_id, state) }, config: config.clone() })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C6HostProvenance { CallerSnapshot, OwnerCapture }

/// Encoded complete host with its provenance. Lanes are data, not evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C6HostCapture { host: HostSnapshot, config: C6HostConfig, lanes: Vec<u128>, provenance: C6HostProvenance }
impl C6HostCapture {
    pub fn from_snapshot(host: &HostSnapshot, config: &C6HostConfig, limits: &C6HostLimits) -> C6HostResult<Self> {
        Ok(Self { lanes: encode_host(host, config, limits)?, host: host.clone(), config: config.clone(),
            provenance: C6HostProvenance::CallerSnapshot })
    }
    pub fn host(&self) -> &HostSnapshot { &self.host }
    pub fn config(&self) -> &C6HostConfig { &self.config }
    pub fn lanes(&self) -> &[u128] { &self.lanes }
    pub fn provenance(&self) -> C6HostProvenance { self.provenance }
}

pub enum C6HostCaptureError { Observation(HostViolation), RetainedLayout(C6StateError), Codec(C6HostError) }

/// Fixed-owner capture (AB first) of a bound pair with its original retained layout.
pub fn capture_host(admitted: &Admitted, image: &MachineImage, retained: &LayoutSchemaC, budgets: C6CodecBudgets,
    limits: &C6HostLimits) -> Result<C6HostCapture, C6HostCaptureError> {
    let host = capture(admitted, image).map_err(C6HostCaptureError::Observation)?;
    retained_layout_agrees_with_host(retained, &host).map_err(C6HostCaptureError::RetainedLayout)?;
    let config = C6HostConfig::from_retained_layout(retained, budgets);
    let state_limits = C6StateLimits { max_total_lanes: limits.max_state_lanes };
    if C6StateContext::from_layout(retained, budgets, state_limits) != config.state_context(limits) {
        return Err(C6HostCaptureError::Codec(C6HostError::ConfigMismatch { field: "context.derivation" }));
    }
    let lanes = encode_host(&host, &config, limits).map_err(C6HostCaptureError::Codec)?;
    Ok(C6HostCapture { host, config, lanes, provenance: C6HostProvenance::OwnerCapture })
}

/// Decoded host whose admission identity, bounds, layout, full program (structure
/// and encoded lanes), original context and run identity/inputs equal a capture of
/// the AB-bound pair at bind time. Image state is NOT bound; `current_image_equal`
/// only reports equality with that pair's current image. No StepInv, transition,
/// reachability, checkpoint, MachineImage import or native authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C6OwnerBoundHost { decoded: HostSnapshot, owner: HostSnapshot, config: C6HostConfig, current_image_equal: bool }
impl C6OwnerBoundHost {
    pub fn decoded_host(&self) -> &HostSnapshot { &self.decoded }
    pub fn owner_capture(&self) -> &HostSnapshot { &self.owner }
    pub fn config(&self) -> &C6HostConfig { &self.config }
    pub fn current_image_equal(&self) -> bool { self.current_image_equal }
}

pub enum C6BindError {
    Observation(HostViolation),
    RetainedLayout(C6StateError),
    Program(C6PhysError),
    Mismatch { field: &'static str },
}

pub fn bind_decoded_host(decoded: &C6DecodedHost, admitted: &Admitted, image: &MachineImage, retained: &LayoutSchemaC,
    budgets: C6CodecBudgets, limits: &C6HostLimits) -> Result<C6OwnerBoundHost, C6BindError> {
    let owner = capture(admitted, image).map_err(C6BindError::Observation)?;
    retained_layout_agrees_with_host(retained, &owner).map_err(C6BindError::RetainedLayout)?;
    let config = C6HostConfig::from_retained_layout(retained, budgets);
    let (d, o) = (&decoded.host, &owner);
    let (drun, orun) = (&d.image.run_id, &o.image.run_id);
    for (same, field) in [
        (config == decoded.config, "context"),
        (d.admission.identity == o.admission.identity, "admission.identity"),
        (d.admission.bounds == o.admission.bounds, "admission.bounds"),
        (d.admission.layout == o.admission.layout, "admission.layout"),
        (d.admission.view == o.admission.view, "admission.view"),
        (drun.identity == orun.identity, "run_id.identity"),
        (drun.fuel_input == orun.fuel_input, "run_id.fuel_input"),
        (drun.stack_limit_input == orun.stack_limit_input, "run_id.stack_limit_input"),
        (d.admission == o.admission && drun == orun, "identity"),
    ] {
        if !same { return Err(C6BindError::Mismatch { field }); }
    }
    let phys = config.state_context(limits).caps().phys;
    let ours = encode_program(&o.admission.view, &phys).map_err(C6BindError::Program)?;
    if encode_program(&d.admission.view, &phys).map_err(C6BindError::Program)? != ours {
        return Err(C6BindError::Mismatch { field: "admission.view.lanes" });
    }
    let current_image_equal = d.image == o.image;
    Ok(C6OwnerBoundHost { decoded: d.clone(), owner, config, current_image_equal })
}

fn first_host_difference(a: &HostSnapshot, b: &HostSnapshot) -> Option<&'static str> {
    let (x, y) = (&a.admission, &b.admission);
    [(x.identity == y.identity, "admission.identity"), (x.bounds == y.bounds, "admission.bounds"),
        (x.view == y.view, "admission.view"), (x.layout == y.layout, "admission.layout"),
        (a.image.run_id == b.image.run_id, "image.run_id"),
        (C6OwnedState::from_image(&a.image) == C6OwnedState::from_image(&b.image), "image.state")]
        .into_iter().find(|&(same, _)| !same).map(|(_, field)| field)
        .or_else(|| (a != b).then_some("snapshot"))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum C6HostWitnessError {
    Host(C6HostError),
    HostMismatch { field: &'static str },
    StateWitness(C6WitnessError),
    OriginalPlan(WitnessFailure),
    DecodedPlan(WitnessFailure),
    PlanMismatch,
    OriginalEvaluation(WitnessFailure),
    DecodedEvaluation(WitnessFailure),
    CrossEvaluation(WitnessFailure),
    EvaluationMismatch { cross: bool },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C6HostWitness {
    pub lanes: Vec<u128>,
    pub decoded: HostSnapshot,
    pub plan: WitnessPlan,
    pub evaluation: WitnessEvaluation,
    pub state: C6WitnessPreservation,
}

/// Whole-host encode, STANDALONE decode, re-encode, the 3C live-out/state check
/// on the original, then uncached plans and evaluations of the original and the
/// genuinely decoded host plus the cross evaluation. Existing implementation only.
pub fn compare_full_host_witness(host: &HostSnapshot, config: &C6HostConfig, limits: &C6HostLimits,
    lifetime: LifetimeUnit) -> Result<C6HostWitness, C6HostWitnessError> {
    use C6HostWitnessError as E;
    let lanes = encode_host(host, config, limits).map_err(E::Host)?;
    let decoded = decode_host(&lanes, config, limits).map_err(E::Host)?.into_host();
    if let Some(field) = first_host_difference(host, &decoded) { return Err(E::HostMismatch { field }); }
    if encode_host(&decoded, config, limits).map_err(E::Host)? != lanes { return Err(E::HostMismatch { field: "lanes" }); }
    let state = compare_uncached_witness(host, &config.state_context(limits), lifetime).map_err(E::StateWitness)?;
    let plan = construct_plan_for(host, lifetime).map_err(E::OriginalPlan)?;
    let decoded_plan = construct_plan_for(&decoded, lifetime).map_err(E::DecodedPlan)?;
    if plan != decoded_plan || plan != state.plan { return Err(E::PlanMismatch); }
    let evaluation = evaluate_uncached(&plan, host).map_err(E::OriginalEvaluation)?;
    let decoded_evaluation = evaluate_uncached(&decoded_plan, &decoded).map_err(E::DecodedEvaluation)?;
    let cross = evaluate_uncached(&plan, &decoded).map_err(E::CrossEvaluation)?;
    if evaluation != decoded_evaluation { return Err(E::EvaluationMismatch { cross: false }); }
    if evaluation != cross { return Err(E::EvaluationMismatch { cross: true }); }
    Ok(C6HostWitness { lanes, decoded, plan, evaluation, state })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sherlock::helper_machine::c6_phys::fixtures;
    use crate::sherlock::helper_machine::complete_observation::{record_step, DataSnapshot, ObservedStep};
    use crate::sherlock::helper_machine::shared::i128_class::KIntC;
    use crate::sherlock::helper_machine::shared::owner_ext_source_program::{ExtDomain, ExtProgram};
    use crate::sherlock::helper_machine::shared::owner_ext_t3_uses::{ExtExpr, ExtFunction, ExtParamSrc, ExtValue};
    use crate::sherlock::helper_machine::shared::owner_ext_type_builder::{CtorDeclSrc, CtorFieldSrc, TypeDeclSrc, TypeExprSrc};
    use crate::sherlock::helper_machine::shared::owner_host::host_owner_engine;
    use crate::sherlock::helper_machine::shared::{admit_ext_program, ControlC, CoreValue, OutcomeC, OwnerEngine};
    use C6PhysError as P;

    fn limits() -> C6HostLimits {
        C6HostLimits { max_total_lanes: 1 << 20, max_state_lanes: 1 << 20, max_metadata_bytes: 4096,
            max_string_bytes: 256, max_width_entries: 64 }
    }
    fn budgets() -> C6CodecBudgets { C6CodecBudgets { type_expr_nodes: 256, template_bytes: 4096 } }

    /// SYNTHETIC, unadmitted: reserved domains, raw unsorted duplicate widths,
    /// Unicode/NUL text, all digest and RunInt classes. Representation only.
    fn synthetic_meta() -> C6HostMetadata {
        let identity = IdentityC { profile_id: 255, grammar_domain: GrammarDomain::GenericNominalExtensionReserved,
            finance: FinanceInput::TypedExtensionReserved, layout_version: ("ν\0x".into(), "".into()),
            layout_digest: DigestC::Unselected,
            bounds_digest: DigestC::Selected { format: ("f".into(), "1".into()), bytes: vec![] },
            field_id: Some(("fid".into(), "".into())),
            program_membership_digest: DigestC::Selected { format: ("g".into(), "2".into()), bytes: vec![0, 255, 7] } };
        let bounds = BoundsC { max_reference_work: u128::MAX, step_cap: 1, frames: 3, ..BoundsC::default() };
        C6HostMetadata { identity: identity.clone(), bounds,
            layout: LayoutSnapshot { layout_version: ("lv".into(), "1".into()), field_id: None,
                widths: Some(WidthsSnapshot { index_widths: vec![("b".into(), u32::MAX), ("a".into(), 1), ("b".into(), 2)],
                    counter_widths: vec![], record_widths: vec![("r".into(), 0)] }),
                scalar_layout_id: Some(("s".into(), "x".into())), bounds,
                capacities: LayoutCapacitiesC { name_rows: 1, name_bytes: 2, occurrence_path_slots: 3, value_slots: 4 },
                boundary_capacities_present: true },
            run_id: RunIdC { identity, fuel_input: RunIntC::NatAboveU128, stack_limit_input: RunIntC::Nat(u128::MAX) } }
    }

    #[test]
    fn metadata_literal_lanes_inverse_mutations_and_refusals() {
        let (m, lim) = (synthetic_meta(), limits());
        let lanes = encode_metadata(&m, &lim).unwrap();
        assert_eq!(lanes[..19], [C6_PHYS_MAGIC, C6_PHYS_VERSION, META_SECTION_TAG, 255, 1, 1, 4, 0xCE, 0xBD, 0, 0x78,
            0, 0, 1, 1, 0x66, 1, 0x31, 0]);
        let n = lanes.len();
        assert_eq!(lanes[n - 4..], [2, 0, 1, u128::MAX]);
        let decoded = decode_metadata(&lanes, &lim).unwrap();
        assert_eq!(decoded, m);
        assert_eq!(encode_metadata(&decoded, &lim).unwrap(), lanes);
        let mutations: [(&str, fn(&mut C6HostMetadata)); 19] = [
            ("profile", |m| m.identity.profile_id = 2),
            ("grammar", |m| m.identity.grammar_domain = GrammarDomain::DedicatedSorts),
            ("finance", |m| m.identity.finance = FinanceInput::Absent),
            ("digest selected empty", |m| m.identity.layout_digest = DigestC::Selected { format: ("f".into(), "1".into()), bytes: vec![] }),
            ("digest unselected", |m| m.identity.bounds_digest = DigestC::Unselected),
            ("digest bytes", |m| if let DigestC::Selected { bytes, .. } = &mut m.identity.program_membership_digest { bytes.push(0); }),
            ("field id", |m| m.identity.field_id = None),
            ("admission bounds only", |m| m.bounds.step_cap = 2),
            ("layout bounds only", |m| m.layout.bounds.frames = 4),
            ("width order", |m| if let Some(w) = &mut m.layout.widths { w.index_widths.swap(0, 1); }),
            ("width duplicate", |m| if let Some(w) = &mut m.layout.widths { w.index_widths.pop(); }),
            ("widths none", |m| m.layout.widths = None),
            ("widths some empty", |m| m.layout.widths = Some(WidthsSnapshot { index_widths: vec![], counter_widths: vec![], record_widths: vec![] })),
            ("capacities", |m| m.layout.capacities.value_slots += 1),
            ("boundary", |m| m.layout.boundary_capacities_present = false),
            ("run identity", |m| m.run_id.identity.profile_id = 0),
            ("fuel negative", |m| m.run_id.fuel_input = RunIntC::Negative),
            ("fuel nat zero", |m| m.run_id.fuel_input = RunIntC::Nat(0)),
            ("stack above", |m| m.run_id.stack_limit_input = RunIntC::NatAboveU128),
        ];
        let mut seen = vec![lanes.clone()];
        for (name, mutate) in mutations {
            let mut x = m.clone();
            mutate(&mut x);
            let l = encode_metadata(&x, &lim).unwrap();
            assert!(!seen.contains(&l), "{name} collides");
            assert_eq!(decode_metadata(&l, &lim), Ok(x), "{name}");
            seen.push(l);
        }
        let dec = |l: &[u128]| decode_metadata(l, &lim);
        let set = |k: usize, v: u128| { let mut x = lanes.clone(); x[k] = v; x };
        assert_eq!(dec(&set(2, CTX_SECTION_TAG)), Err(P::BadTag { section: META_S, tag: CTX_SECTION_TAG }));
        assert_eq!(dec(&set(3, 256)), Err(P::UsizeOverflow { section: META_S, value: 256 }));
        assert_eq!(dec(&set(4, 2)), Err(P::BadTag { section: META_S, tag: 2 }));
        assert_eq!(dec(&set(7, 0x1CE)), Err(P::BadTag { section: META_S, tag: 0x1CE }));
        assert_eq!(dec(&set(7, 0xFF)), Err(P::BadTag { section: META_S, tag: 4 }), "invalid UTF-8");
        assert_eq!(dec(&set(13, 2)), Err(P::BadTag { section: META_S, tag: 2 }));
        assert_eq!(dec(&lanes[..n - 1]), Err(P::Truncated { section: META_S, needed: n, actual: n - 1 }));
        let mut trailing = lanes.clone();
        trailing.push(0);
        assert_eq!(dec(&trailing), Err(P::TrailingLanes { section: META_S, extra: 1 }));
        assert_eq!(dec(&set(n - 3, 5)), Err(P::NonZeroPadding { section: META_S, lane: n - 3, value: 5 }));
        let w = lanes.iter().position(|&x| x == u128::from(u32::MAX)).unwrap();
        assert_eq!(dec(&set(w, 1 << 32)), Err(P::UsizeOverflow { section: META_S, value: 1 << 32 }));
        for (tight, error) in [
            (C6HostLimits { max_string_bytes: 3, ..lim }, P::CapacityExceeded { section: "host.meta.string_bytes", needed: 4, limit: 3 }),
            (C6HostLimits { max_metadata_bytes: 5, ..lim }, P::CapacityExceeded { section: "host.meta.bytes", needed: 6, limit: 5 }),
            (C6HostLimits { max_width_entries: 3, ..lim }, P::CapacityExceeded { section: "host.meta.width_entries", needed: 4, limit: 3 }),
        ] {
            assert_eq!(encode_metadata(&m, &tight).unwrap_err(), error);
            assert_eq!(decode_metadata(&lanes, &tight).unwrap_err(), error);
        }
    }

    #[test]
    fn context_section_presence_every_field_and_padding() {
        let b = budgets();
        let (none, zeroed) = (encode_context(None, b), encode_context(Some(ExtCapacitiesC::all_zero()), b));
        assert_eq!(none[..4], [C6_PHYS_MAGIC, C6_PHYS_VERSION, CTX_SECTION_TAG, 0]);
        assert_eq!((zeroed[3], &none[4..20], &none[20..]), (1, &zeroed[4..20], &[256u128, 4096][..]));
        assert_ne!(none, zeroed);
        assert_eq!(decode_context(&none), Ok((None, b)));
        assert_eq!(decode_context(&zeroed), Ok((Some(ExtCapacitiesC::all_zero()), b)));
        for k in 4..20 {
            let mut l = zeroed;
            l[k] = 7;
            let (ext, _) = decode_context(&l).unwrap();
            assert_ne!(ext, Some(ExtCapacitiesC::all_zero()), "lane {k}");
            assert_eq!(encode_context(ext, b), l);
            let mut p = none;
            p[k] = 1;
            assert_eq!(decode_context(&p), Err(P::NonZeroPadding { section: CTX_S, lane: k, value: 1 }));
        }
        let full = ExtCapacitiesC { max_lifetime_work: u128::MAX, journal_rows: 3, ..ExtCapacitiesC::all_zero() };
        assert_eq!(decode_context(&encode_context(Some(full), b)), Ok((Some(full), b)));
        let mut bad = zeroed;
        bad[3] = 2;
        assert_eq!(decode_context(&bad), Err(P::BadBool { section: CTX_S, value: 2 }));
        assert_eq!(decode_context(&zeroed[..21]), Err(P::Truncated { section: CTX_S, needed: 22, actual: 21 }));
    }

    fn run(a: &Admitted, fuel: RunIntC, stack: u128) -> MachineImage {
        fixtures::must(host_owner_engine().admit_run(a, &fuel, &RunIntC::Nat(stack), &FinanceInput::Absent), "admit run")
    }

    /// Real generic nominal admission retaining its ORIGINAL layout; `unused` adds
    /// a never-called function while entry and result stay identical.
    fn admitted(unused: bool, fuel: RunIntC, stack: u128) -> (Admitted, MachineImage, LayoutSchemaC) {
        let bounds = BoundsC { code_rows: 128, function_rows: 16, code_child_slots: 128, parameter_slots: 32,
            pattern_rows: 32, match_plan_rows: 32, match_arm_rows: 32, type_rows: 16, constructor_rows: 16,
            constructor_field_slots: 32, max_arity: 8, constant_nodes: 128, constant_field_slots: 128,
            data_nodes: 128, data_field_slots: 128, environment_cells: 128, frames: 64, helper_tasks: 64,
            serialization_bytes: 1_000_000, max_reference_work: 1_000_000, max_helper_work: 1_000_000, step_cap: 10_000 };
        let layout = LayoutSchemaC { layout_version: ("c6-phys-host-test".into(), "1".into()), field_id: None,
            widths: None, scalar_layout_id: None, bounds,
            capacities: LayoutCapacitiesC { name_rows: 128, name_bytes: 4096, occurrence_path_slots: 4096, value_slots: 4096 },
            ext_capacities: Some(ExtCapacitiesC { plan_case_slots: 64, field_path_slots: 64, pattern_child_slots: 64,
                ..ExtCapacitiesC::all_zero() }),
            boundary_capacities_present: false };
        let field = |name: &str, ty: TypeExprSrc| CtorFieldSrc { name: name.into(), ty };
        let var = |name: &str| ExtExpr::Var { name: name.into() };
        let lit = |n: u128| ExtExpr::Literal { value: ExtValue::Core { value: CoreValue::UInt { n: KIntC::InRange(n) } } };
        let pair = || TypeExprSrc::Named { name: "Pair".into() };
        let box_uint = || TypeExprSrc::Apply { name: "Box".into(), args: vec![TypeExprSrc::UInt] };
        let param = |name: &str| ExtParamSrc { name: name.into(), ty: TypeExprSrc::UInt };
        let mut functions = vec![ExtFunction { name: "mk".into(), parameters: vec![param("x"), param("y")],
            return_type: pair(), body: ExtExpr::Construct { ty: pair(), ctor: "MkPair".into(), fields: vec![var("x"),
                ExtExpr::Construct { ty: box_uint(), ctor: "MkBox".into(), fields: vec![var("y")] }] } }];
        if unused {
            functions.push(ExtFunction { name: "unused".into(), parameters: vec![param("z")],
                return_type: TypeExprSrc::UInt, body: var("z") });
        }
        let program = ExtProgram {
            types: vec![
                TypeDeclSrc { name: "Box".into(), parameters: vec!["A".into()], ctors: vec![CtorDeclSrc {
                    name: "MkBox".into(), fields: vec![field("item", TypeExprSrc::Param { name: "A".into() })] }] },
                TypeDeclSrc { name: "Pair".into(), parameters: Vec::new(), ctors: vec![CtorDeclSrc {
                    name: "MkPair".into(), fields: vec![field("fst", TypeExprSrc::UInt), field("snd", box_uint())] }] },
            ],
            assets: Vec::new(), states: Vec::new(), inputs: Vec::new(), functions,
            entry: ExtExpr::Let { name: "p".into(),
                value: Box::new(ExtExpr::Call { name: "mk".into(), args: vec![lit(7), lit(8)] }), body: Box::new(var("p")) },
        };
        let (a, _report) = fixtures::must(admit_ext_program(program, ExtDomain::GENERIC_ABSENT, &bounds, &layout, None),
            "generic nominal admission");
        let image = run(&a, fuel, stack);
        (a, image, layout)
    }

    fn must_capture(a: &Admitted, img: &MachineImage, layout: &LayoutSchemaC) -> C6HostCapture {
        match capture_host(a, img, layout, budgets(), &limits()) { Ok(c) => c, Err(_) => panic!("capture_host") }
    }

    #[test]
    fn owner_capture_standalone_inverse_outer_refusals_and_context_binding() {
        use C6HostChild::*;
        let lim = limits();
        let (a, img, layout) = admitted(false, RunIntC::Nat(fixtures::FUEL), 64);
        let cap = must_capture(&a, &img, &layout);
        assert_eq!(cap.provenance(), C6HostProvenance::OwnerCapture);
        let (host, config, lanes) = (cap.host(), cap.config(), cap.lanes());
        let decoded = decode_host(lanes, config, &lim).unwrap();
        assert_eq!(decoded.host(), host);
        assert_eq!(encode_host(decoded.host(), config, &lim).unwrap(), lanes);
        let ctx = config.state_context(&lim);
        let meta = encode_metadata(&C6HostMetadata::from_host(host), &lim).unwrap();
        let prog = encode_program(&host.admission.view, &ctx.caps().phys).unwrap();
        let ctxl = encode_context(layout.ext_capacities, budgets()).to_vec();
        let state = encode_state(&C6OwnedState::from_image(&host.image), &ctx).unwrap();
        let (m, p, s) = (meta.len(), prog.len(), state.len());
        assert_eq!(lanes[..HOST_HEADER_LANES], [C6_PHYS_MAGIC, C6_PHYS_VERSION, HOST_SECTION_TAG, C6_HOST_SCHEMA, 4,
            META_SECTION_TAG, u(m), PROGRAM_SECTION_TAG, u(p), CTX_SECTION_TAG, 22, STATE_SECTION_TAG, u(s), 0]);
        let mut at = HOST_HEADER_LANES;
        for part in [&meta, &prog, &ctxl, &state] { assert_eq!(&lanes[at..at + part.len()], &part[..]); at += part.len(); }
        assert_eq!(at, lanes.len());

        let n = lanes.len();
        let dec = |l: &[u128], c: &C6HostConfig| decode_host(l, c, &lim).map(C6DecodedHost::into_host);
        let set = |k: usize, v: u128| { let mut x = lanes.to_vec(); x[k] = v; x };
        let short = C6HostLimits { max_total_lanes: n - 1, ..lim };
        let over = Err(C6HostError::LimitExceeded { scope: "host.total_lanes", needed: n, limit: n - 1 });
        assert_eq!(decode_host(lanes, config, &short).map(|_| ()), over);
        assert_eq!(encode_host(host, config, &short).map(|_| ()), over);
        assert_eq!(dec(&set(0, 0), config), Err(outer(P::BadMagic { actual: 0 })));
        assert_eq!(dec(&set(3, 2), config), Err(outer(P::BadTag { section: "host.schema", tag: 2 })));
        assert_eq!(dec(&set(4, 3), config), Err(C6HostError::BadSectionCount { actual: 3 }));
        assert_eq!(dec(&set(5, CTX_SECTION_TAG), config),
            Err(C6HostError::BadChildTag { child: Metadata, expected: META_SECTION_TAG, actual: CTX_SECTION_TAG }));
        assert_eq!(dec(&set(10, 21), config), Err(C6HostError::ChildLengthRange { child: Context, declared: 21, minimum: 22 }));
        assert_eq!(dec(&set(13, 1), config), Err(outer(P::NonZeroPadding { section: HOST_S, lane: 13, value: 1 })));
        let mut trailing = lanes.to_vec();
        trailing.push(0);
        assert_eq!(dec(&trailing, config), Err(outer(P::TrailingLanes { section: HOST_S, extra: 1 })));
        assert_eq!(dec(&set(HOST_HEADER_LANES + 4, 2), config),
            Err(C6HostError::Child { child: Metadata, offset: HOST_HEADER_LANES, error: P::BadTag { section: META_S, tag: 2 } }));

        // Equal derived 3C caps, different reserved financial field: refused here.
        let mut reserved = config.clone();
        if let Some(e) = &mut reserved.ext { e.journal_rows = 1; }
        assert_eq!(reserved.state_context(&lim).descriptor(), ctx.descriptor());
        assert_eq!(reserved.state_context(&lim).caps(), ctx.caps());
        assert_eq!(dec(lanes, &reserved), Err(C6HostError::ConfigMismatch { field: "context.ext" }));
        let mut absent = config.clone();
        absent.ext = None;
        assert_eq!(dec(lanes, &absent), Err(C6HostError::ConfigMismatch { field: "context.ext" }));
        let mut version = config.clone();
        version.layout.layout_version.1 = "2".into();
        assert_eq!(dec(lanes, &version), Err(C6HostError::ConfigMismatch { field: "layout" }));
        assert_eq!(encode_host(host, &version, &lim), Err(C6HostError::ConfigMismatch { field: "layout" }));
        let mut bud = config.clone();
        bud.budgets.template_bytes += 1;
        assert_eq!(dec(lanes, &bud), Err(C6HostError::ConfigMismatch { field: "context.budgets" }));
        let mut rlanes = lanes.to_vec();
        rlanes[HOST_HEADER_LANES + m + p + 4 + 11] = 1;
        assert_eq!(dec(&rlanes, &reserved).map(|h| h == *host), Ok(true), "ext is context, not host state");

        // Negative-input genesis keeps Negative in RunId while registers are zero.
        let neg_img = run(&a, RunIntC::Negative, 64);
        let neg = must_capture(&a, &neg_img, &layout);
        let back = decode_host(neg.lanes(), neg.config(), &lim).unwrap().into_host();
        assert_eq!((back.image.run_id.fuel_input, back.image.run_id.stack_limit_input), (RunIntC::Negative, RunIntC::Nat(64)));
        assert_eq!((back.image.initial_fuel, back.image.stack_limit, back.image.control), (0, 0, ControlC::Error { code: 1 }));
        assert_eq!(&back, neg.host());
        assert_ne!(neg.lanes(), lanes);
    }

    #[test]
    fn owner_binding_refuses_other_program_unrelated_pair_and_other_run_inputs() {
        let lim = limits();
        let (a1, mut i1, l1) = admitted(false, RunIntC::Nat(fixtures::FUEL), 64);
        let (a2, i2, l2) = admitted(true, RunIntC::Nat(fixtures::FUEL), 64);
        let (c1, c2) = (must_capture(&a1, &i1, &l1), must_capture(&a2, &i2, &l2));
        assert_eq!(c1.host().admission.identity, c2.host().admission.identity);
        assert_eq!(c1.host().admission.identity.program_membership_digest, DigestC::Unselected);
        assert_eq!(c1.config(), c2.config());
        assert_ne!(c1.host().admission.view, c2.host().admission.view);
        assert_ne!(c1.lanes(), c2.lanes());
        let d1 = decode_host(c1.lanes(), c1.config(), &lim).unwrap();
        let bind = |a: &Admitted, i: &MachineImage, l: &LayoutSchemaC| bind_decoded_host(&d1, a, i, l, budgets(), &lim);
        let field = |r: Result<C6OwnerBoundHost, C6BindError>| match r { Err(C6BindError::Mismatch { field }) => field, _ => "other" };
        match bind(&a1, &i1, &l1) {
            Ok(b) => { assert!(b.current_image_equal()); assert_eq!(b.decoded_host(), c1.host()); }
            Err(_) => panic!("own pair binds"),
        }
        assert!(bind(&a1.clone(), &i1, &l1).is_ok(), "Arc clone of the same admission");
        assert_eq!(field(bind(&a2, &i2, &l2)), "admission.view");
        let (a3, _i3, _l3) = admitted(false, RunIntC::Nat(fixtures::FUEL), 64);
        assert!(matches!(bind(&a3, &i1, &l1), Err(C6BindError::Observation(_))), "unrelated pair fails AB");
        assert_eq!(field(bind(&a1, &run(&a1, RunIntC::Nat(100), 64), &l1)), "run_id.fuel_input");
        assert_eq!(field(bind(&a1, &run(&a1, RunIntC::Nat(fixtures::FUEL), 32), &l1)), "run_id.stack_limit_input");
        assert_eq!(field(bind(&a1, &run(&a1, RunIntC::Negative, 64), &l1)), "run_id.fuel_input");
        let mut no_ext = l1.clone();
        no_ext.ext_capacities = None;
        assert_eq!(field(bind(&a1, &i1, &no_ext)), "context");
        assert!(matches!(record_step(&a1, &mut i1), Ok(ObservedStep::Event(_))));
        match bind(&a1, &i1, &l1) { Ok(b) => assert!(!b.current_image_equal()), Err(_) => panic!("membership binds") }
    }

    #[test]
    fn owner_trace_full_host_decode_preserves_uncached_witness() {
        let lim = limits();
        let (a, mut img, layout) = admitted(false, RunIntC::Nat(fixtures::FUEL), 64);
        let config = C6HostConfig::from_retained_layout(&layout, budgets());
        let check = |host: &HostSnapshot, lifetime: LifetimeUnit| match compare_full_host_witness(host, &config, &lim, lifetime) {
            Ok(w) => { assert_eq!(&w.decoded, host); w.lanes }
            Err(e) => panic!("{lifetime:?}: {e:?}"),
        };
        let (mut steps, mut helpers, mut ctor, mut last) = (0usize, 0usize, 0usize, None);
        for _ in 0..4096 {
            match record_step(&a, &mut img) {
                Ok(ObservedStep::Event(r)) => {
                    let (b, af) = (check(&r.before, LifetimeUnit::RecordBefore), check(&r.after, LifetimeUnit::RecordAfter));
                    assert_eq!(b != af, r.before != r.after);
                    steps += 1;
                    helpers += usize::from(r.before.image.helper.is_some());
                    ctor += usize::from(r.before.image.data.iter().any(|d| matches!(d, DataSnapshot::Constructor { .. })));
                }
                Ok(ObservedStep::Terminal { before, after }) => {
                    check(&before, LifetimeUnit::RecordBefore);
                    check(&after, LifetimeUnit::RecordAfter);
                    check(&after, LifetimeUnit::Snapshot);
                    last = Some(after);
                    break;
                }
                Err(_) => panic!("owner step"),
            }
        }
        let last = last.expect("terminal within fuel and 4096 steps");
        assert!(matches!(last.image.outcome, OutcomeC::Halted { .. }));
        assert!(steps > 0 && helpers > 0 && ctor > 0);
    }
}
