//! Output-only UIREQ002 metadata for the existing dedicated source route.
//! No grammar, admission policy, resource guard, runtime or numerical default.
use serde::Serialize;
use super::config::{Bounds, Grammar, Layout, Profile, RunInput};
use super::draft_contract::{DraftExecutionConfig, DraftPreprocessingExtents, SOURCE_PROFILE, SOURCE_SEMANTICS};

#[derive(Serialize)]
pub struct SupportedProfileReport {
    pub format: &'static str,
    pub source_language: &'static str,
    pub source_semantics: &'static str,
    pub execution_profile: Profile,
    pub qualification: &'static str,
    pub grammar: GrammarReport,
    pub numeric_domains: NumericDomains,
    pub schedule: ScheduleReport,
    pub extents: ExtentsReport,
    pub admission_scope: AdmissionScope,
    pub dispositions: Dispositions,
}

#[derive(Serialize)]
pub struct GrammarReport {
    pub domain: Grammar,
    pub binding: &'static str,
    pub expressions: &'static [&'static str],
    pub patterns: &'static [&'static str],
    pub constructors: &'static [&'static str],
    pub named_calls: &'static str,
    pub matching: &'static str,
    pub evaluation_order: &'static str,
    pub higher_order_closures: &'static str,
    pub letrec: &'static str,
    pub generic_nominal_adts: &'static str,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct NumericDomains {
    pub UInt: &'static str,
    pub Bool: &'static str,
    pub List: &'static str,
    pub Tree: &'static str,
    pub equality: &'static str,
    pub pointer_identity: &'static str,
}

#[derive(Serialize)]
pub struct ScheduleReport {
    pub profile: Profile,
    pub legacy_completion_charge: &'static str,
    pub explicit_helper_transition_charge: &'static str,
    pub startup_terminal_error_publication_charge: &'static str,
    pub helper_families: &'static [&'static str],
    pub counters: &'static str,
    pub reference_comparison: &'static str,
    pub physical_resources: &'static str,
}

/// These are exact existing caller fields, not fresh admission or measurements.
#[derive(Serialize)]
pub struct ExtentsReport {
    pub status: &'static str,
    pub bounds: Bounds,
    pub layout: Layout,
    pub fuel: RunInput,
    pub stack: RunInput,
    pub preprocessing: DraftPreprocessingExtents,
    pub serialization_scope: &'static str,
    pub frame_scope: &'static str,
    pub work_scope: &'static str,
}

#[derive(Serialize)]
pub struct AdmissionScope {
    pub source_typecheck: &'static str,
    pub owner_closed_body_admission: &'static str,
    pub unique_function_admission: &'static str,
    pub finance: &'static str,
}

#[derive(Serialize)]
pub struct Dispositions {
    pub supported: &'static str,
    pub source_preparation_failures: &'static [&'static str],
    pub owner_tooling_failures: &'static [&'static str],
    pub tooling_boundary: &'static str,
    pub runtime_outcomes: &'static [&'static str],
    pub runtime_boundary: &'static str,
    pub host_violation: &'static str,
    pub native_export: &'static str,
}

/// Called on the same config identity carried by the admitted public artifact.
/// Describes the selected semantics; it never evaluates a source expression or
/// certifies closed-body/unique-function admission, runtime or physical bounds.
pub(super) fn report(config: &DraftExecutionConfig) -> SupportedProfileReport {
    SupportedProfileReport {
        format: "sherlock-supported-profile/1",
        source_language: SOURCE_PROFILE,
        source_semantics: SOURCE_SEMANTICS,
        execution_profile: config.profile,
        qualification: "HostOnly prepared-and-admitted dedicated first-order source; runtime and semantic correspondence unobserved",
        grammar: GrammarReport {
            domain: config.grammar,
            binding: "lexical variables; closed entry; immutable source-ordered global named function table",
            expressions: &["Literal", "Var", "Let", "If", "Call", "Match", "Binary.Add", "Binary.Sub", "Binary.Mul", "Binary.Eq", "Binary.Lt", "Binary.Cons", "Binary.Branch", "Unary.Leaf", "Unary.Not"],
            patterns: &["Var", "Nil", "Cons", "Leaf", "Branch"],
            constructors: &["Nil", "Cons(UInt128,List)", "Leaf(UInt128)", "Branch(Tree,Tree)"],
            named_calls: "first declaration lookup; self and mutual recursion remain calls, without source-recursion unrolling",
            matching: "first matching source arm; ordered binders; scrutinee evaluated once by the selected execution semantics",
            evaluation_order: "strict left-to-right operands, arguments and constructor fields",
            higher_order_closures: "extension; unsupported by this source grammar",
            letrec: "extension; unsupported by this source grammar",
            generic_nominal_adts: "separate declaration/constructor extension; not supplied by dedicated List/Tree sorts",
        },
        numeric_domains: NumericDomains {
            UInt: "UInt128 integers in [0,340282366920938463463374607431768211455]; checked add/sub/mul; no modular wrap",
            Bool: "false or true",
            List: "finite immutable List of UInt128",
            Tree: "finite immutable binary Tree with UInt128 leaf payloads",
            equality: "structural over the dedicated value sum; different variants compare false",
            pointer_identity: "not source-visible; cyclic values unsupported",
        },
        schedule: ScheduleReport {
            profile: config.profile,
            legacy_completion_charge: "1",
            explicit_helper_transition_charge: "1",
            startup_terminal_error_publication_charge: "0",
            helper_families: &["Scalar", "EnvGet", "EnvPut", "Find", "Bind", "Append", "Validate", "Equal", "Match"],
            counters: "consumed_work includes charged helper work; helper_work, helper_visits and reference_cursor remain distinct",
            reference_comparison: "sherlock-k-a/1 keeps pure helpers unmetered; equal A/B fuel and runtime correspondence are not claimed",
            physical_resources: "host heap/stack/time, circuit rows/degree and proof/target costs are unmeasured",
        },
        extents: ExtentsReport {
            status: "caller-declared logical capacities and preprocessing extents; no global H1 or output-serialization bound is certified",
            bounds: config.bounds,
            layout: config.layout.clone(),
            fuel: config.fuel,
            stack: config.stack,
            preprocessing: config.preprocessing,
            serialization_scope: "bounds.serialization_bytes is the declared owner image/checkpoint capacity; public report JSON size and encoding costs are not measured or enforced by this metadata",
            frame_scope: "bounds.frames is logical image capacity; stack is the separately supplied logical run stack limit",
            work_scope: "max_reference_work, max_helper_work and step_cap are separate logical capacities; fuel is the separately supplied run budget",
        },
        admission_scope: AdmissionScope {
            source_typecheck: "all function bodies and entry are checked for lexical variables, signatures and sorts under max_type_depth; calls inspect signatures without recursive callee descent",
            owner_closed_body_admission: "explicit implementation work remains; this report adds no whole-program owner admission checker",
            unique_function_admission: "explicit implementation work remains; first declaration lookup is preserved, not a unique-name guarantee",
            finance: "Absent; transfers, fees, liabilities, wait and resume are outside this dedicated pure grammar",
        },
        dispositions: Dispositions {
            supported: "the listed dedicated grammar under the selected caller-supplied profile and extents; this artifact is prepared/admitted only",
            source_preparation_failures: &["parse", "typecheck", "copy"],
            owner_tooling_failures: &["AdmissionRejected", "CapacityNotAdmitted", "UnsupportedFeature"],
            tooling_boundary: "owner tooling failures retain their locator when supplied, and remain outside runtime Outcome; unsupported source syntax may instead be a located parse diagnostic",
            runtime_outcomes: &["Running", "Halted", "Rejected", "Exhausted"],
            runtime_boundary: "Rejected(error4) frame refusal and Exhausted(kind) runtime limits are not AdmissionRejected, CapacityNotAdmitted or tooling UnsupportedFeature",
            host_violation: "separate owner invariant/observation failure; never a successful application outcome",
            native_export: "unsupported; witness and constraint plans missing; no native or target result",
        },
    }
}
