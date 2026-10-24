//! Ordinary public typed ExtProgram compilation through the fixed checked owner.
//! SOURCE_ONLY_UNRUN: consumes the actual returned owner admission ABI.
//! The owner source and full runtime/schema union remain unreviewed; this file
//! does not establish their compilation, correctness or admission acceptance.
//! No source language/version, CLI grammar, guard or admission policy is added.
use crate::sherlock::helper_machine::shared::owner_ext_source_program::{ExtDomain, ExtProgram};
use crate::sherlock::helper_machine::shared::owner_ext_type_builder::ExtTypeBuildError;
use crate::sherlock::helper_machine::shared::owner_host::{
    HostObservation, host_owner_engine, observe_bound_image,
};
use crate::sherlock::helper_machine::shared::{
    Admitted, BoundsC, HostViolation, LayoutSchemaC, LocatorC, MachineImage, OwnerEngine,
    PreparationReportC, RunIntC, Tooling, admit_ext_program,
};
use crate::sherlock_nominal_source::{DraftError, DraftParseBudget, parse_nominal_source_bounded};

/// Exact caller inputs. The owner interprets every capacity/domain/run input;
/// this ordinary adapter neither supplies defaults nor preempts its diagnostics.
/// These typed values do not establish whole-process host resource bounds.
pub struct ExtCompileConfig {
    pub bounds: BoundsC,
    pub layout: LayoutSchemaC,
    pub domain: ExtDomain,
    pub fuel: RunIntC,
    pub stack: RunIntC,
}

/// Complete original owner payloads, kept separate by preparation/genesis stage.
/// Preparation errors are tooling/host diagnostics, never fabricated run outcomes.
pub enum ExtCompileFailure {
    Preparation(ExtTypeBuildError),
    Genesis(Tooling),
}

/// Text parsing and the original typed compiler failure remain distinct.
/// The nested compiler error preserves its preparation/genesis stage and full
/// owner payload; neither stage is converted into a runtime outcome.
pub enum NominalCompileFailure {
    Parse(DraftError),
    Compile(ExtCompileFailure),
}

/// Parse bounded nominal source text and compile its actual owned program.
///
/// The explicit `moriarty-sherlock-nominal/1` grammar and every caller parser
/// extent are consumed by the existing bounded parser. Its declarations,
/// constructor fields, ordered arms and recursive function bodies then enter
/// [`compile_ext_host`] unchanged. This operation performs no evaluation or
/// recursive call unfolding and supplies no parser or owner capacity defaults.
///
/// The returned artifact is the existing admitted HostOnly tables/genesis pair.
/// Source name/spans are not added to that typed artifact. CLI selection,
/// witness/constraint/native export and whole-process H1 bounds remain separate.
pub fn compile_nominal_host(
    source: &str,
    parser_budget: DraftParseBudget,
    config: ExtCompileConfig,
    locator: Option<LocatorC>,
) -> Result<ExtCompileArtifact, NominalCompileFailure> {
    let parsed = parse_nominal_source_bounded(source, parser_budget)
        .map_err(NominalCompileFailure::Parse)?;
    compile_ext_host(parsed.program, config, locator).map_err(NominalCompileFailure::Compile)
}

/// Owns only the actual admitted object and its actual unstepped owner image.
/// No public constructor can wrap a ProgramView or a pending C/P assembly.
pub struct ExtCompileArtifact {
    config: ExtCompileConfig,
    admitted: Admitted,
    genesis: MachineImage,
    preparation: PreparationReportC,
}

/// Borrowed complete typed report over the same AB-bound owner pair. The owner
/// projection retains every table/image field; nothing is rebuilt from fixtures.
/// JSON/witness/DAG projection remains the existing SECOND owner dependency.
pub struct ExtCompileReport<'a> {
    pub execution_profile: &'static str,
    pub configuration: &'a ExtCompileConfig,
    pub owner: HostObservation<'a>,
    pub preparation: &'a PreparationReportC,
    pub plans: super::Plans,
    pub evaluation_performed: bool,
}

/// Compile the existing uniquely-owned ExtProgram source carrier. Generic
/// declarations, type arguments, nested ordered patterns, arbitrary names,
/// multiargument functions and named recursion enter the owner unchanged.
///
/// The FIRST preparation operation is the complete checked owner admission;
/// no source traversal, type instantiation, AST copy or report allocation is
/// performed before it. The returned owner source supplies the existing
/// Pass B/T -> C1-C6 -> P -> validation/reprepare/admission sequence. Incomplete
/// rows are never exposed or converted into an Admitted value here.
///
/// This typed API is an increment toward the public Moriarty source compiler.
/// It does not parse text, promote the dedicated draft grammar, prove compiler
/// correspondence or establish H1/global untrusted-source resource guarantees.
pub fn compile_ext_host(
    program: ExtProgram,
    config: ExtCompileConfig,
    locator: Option<LocatorC>,
) -> Result<ExtCompileArtifact, ExtCompileFailure> {
    let (admitted, preparation) = admit_ext_program(
        program,
        config.domain,
        &config.bounds,
        &config.layout,
        locator,
    )
    .map_err(ExtCompileFailure::Preparation)?;
    let owner = host_owner_engine();
    let genesis = owner
        .admit_run(
            &admitted,
            &config.fuel,
            &config.stack,
            &config.domain.finance,
        )
        .map_err(ExtCompileFailure::Genesis)?;
    Ok(ExtCompileArtifact {
        config,
        admitted,
        genesis,
        preparation,
    })
}

impl ExtCompileArtifact {
    pub fn configuration(&self) -> &ExtCompileConfig {
        &self.config
    }
    pub fn admitted(&self) -> &Admitted {
        &self.admitted
    }
    pub fn genesis(&self) -> &MachineImage {
        &self.genesis
    }
    pub fn preparation(&self) -> &PreparationReportC {
        &self.preparation
    }

    /// The existing owner checks pair binding before borrowing any payload.
    pub fn observe(&self) -> Result<HostObservation<'_>, HostViolation> {
        observe_bound_image(&self.admitted, &self.genesis)
    }

    pub fn report(&self) -> Result<ExtCompileReport<'_>, HostViolation> {
        let owner = self.observe()?;
        Ok(ExtCompileReport {
            execution_profile: super::EXECUTION_PROFILE,
            configuration: &self.config,
            owner,
            preparation: &self.preparation,
            plans: super::Plans {
                witness: super::Plan {
                    status: "missing",
                    artifact: None,
                    reason: "generic source witness plan is not supplied by typed host compilation",
                },
                constraint: super::Plan {
                    status: "missing",
                    artifact: None,
                    reason: "generic source constraint plan is not supplied by typed host compilation",
                },
                export: super::Plan {
                    status: "unsupported",
                    artifact: None,
                    reason: "HostOnly; native/target export is not implemented by this API",
                },
            },
            evaluation_performed: false,
        })
    }

    /// Transfer the real pair to an existing consumer; no source body is run.
    pub fn into_parts(self) -> (Admitted, MachineImage, PreparationReportC) {
        (self.admitted, self.genesis, self.preparation)
    }
}
