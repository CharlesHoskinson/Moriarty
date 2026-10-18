//! Admission frontend: the actual current Core parser, validator and stage loader.
//!
//! Legacy Source/5 or Source/6 and the Node beta are not frontends here. A `/1` IR
//! value built in memory (for example a DAG with shared nodes, which the parser never
//! produces) is admitted through [`admit_program`] and the same Core `validate`.
use crate::error::CompileError;
use crate::identity::{self, Digest32, ProgramIdentity};
use crate::staged::StagedAdmission;
use mori_core::Program;

/// An admitted `/1` typed Program. Only this module constructs one.
#[derive(Debug, Clone)]
pub struct AdmittedCore1 {
    program: Program,
    identity: ProgramIdentity,
}

impl AdmittedCore1 {
    pub fn program(&self) -> &Program {
        &self.program
    }
    pub fn identity(&self) -> &ProgramIdentity {
        &self.identity
    }
}

/// Result of routing a source text by its declared profile.
#[derive(Debug, Clone)]
pub enum Admitted {
    Core1(AdmittedCore1),
    Staged2(StagedAdmission),
}

/// Route by `mori_core::language`, then admit with the profile's own Core entry point.
pub fn admit_source(source: &str) -> Result<Admitted, CompileError> {
    let profile = mori_core::language(source).map_err(CompileError::Core)?;
    let source_sha256 = identity::source_hash(source);
    match profile {
        mori_core::LANGUAGE => {
            // `parse` validates before returning (parser.rs:157-159).
            let program = mori_core::parse(source).map_err(CompileError::Core)?;
            admit_checked(program, Some(source_sha256)).map(Admitted::Core1)
        }
        mori_core::stage::LANGUAGE_2 => {
            let staged = mori_core::stage::load(source).map_err(CompileError::Core)?;
            Ok(Admitted::Staged2(StagedAdmission::new(staged, source_sha256)))
        }
        other => Err(CompileError::Profile(other.to_string())),
    }
}

/// Admit an in-memory `/1` Program through the same Core checks.
pub fn admit_program(program: Program) -> Result<AdmittedCore1, CompileError> {
    admit_checked(program, None)
}

fn admit_checked(program: Program, source_sha256: Option<Digest32>) -> Result<AdmittedCore1, CompileError> {
    mori_core::stage::require_core1(&program).map_err(CompileError::Core)?;
    mori_core::validate(&program).map_err(CompileError::Core)?;
    let program_hash = identity::program_hash(&program)?;
    let identity = ProgramIdentity {
        language: program.language.clone(),
        semantics: program.semantics.clone(),
        program_hash,
        source_sha256,
    };
    Ok(AdmittedCore1 { program, identity })
}
