//! Profile isolation between `moriarty-core/1`, `moriarty-core/2` and
//! `moriarty-core/3` consumers. Helpers keep `mori::core::profile` identity.
use super::{LANGUAGE_2, LANGUAGE_3, SEMANTICS_2, SEMANTICS_3};
use crate::{Error, LANGUAGE, Program, SEMANTICS, Statement};

const PROFILE_MESSAGE: &str = "Unsupported language or semantics identity.";

/// Required at every `/1` entry point before evaluation or layout. An
/// in-memory program labelled `/1` that still carries an Await is refused.
/// A `/3` program is refused here because its language differs from `/1`.
pub fn require_core1(program: &Program) -> Result<(), Error> {
    if program.language != LANGUAGE || program.semantics != SEMANTICS {
        return Err(Error::new("profile", PROFILE_MESSAGE));
    }
    // F2: duties exist only on a parser-built `/3` program. A manually
    // constructed `/1` value carrying duties is refused here as at
    // `runtime::validate`, with the same profile identity.
    if !program.duties.is_empty() {
        return Err(Error::new("profile", PROFILE_MESSAGE));
    }
    if program
        .actions
        .iter()
        .flat_map(|action| &action.statements)
        .any(|statement| matches!(statement, Statement::Await { .. }))
    {
        return Err(Error::new("profile", "Await requires moriarty-core/2."));
    }
    Ok(())
}

/// Required at every `/2` staged entry point; `/1` and `/3` are refused.
pub fn require_core2(program: &Program) -> Result<(), Error> {
    if program.language != LANGUAGE_2 || program.semantics != SEMANTICS_2 {
        return Err(Error::new("profile", PROFILE_MESSAGE));
    }
    if !program.duties.is_empty() {
        return Err(Error::new("profile", PROFILE_MESSAGE));
    }
    Ok(())
}

/// Required at every `/3` staged entry point; `/1` and `/2` are refused.
pub fn require_core3(program: &Program) -> Result<(), Error> {
    if program.language != LANGUAGE_3 || program.semantics != SEMANTICS_3 {
        return Err(Error::new("profile", PROFILE_MESSAGE));
    }
    Ok(())
}

/// Staged dispatcher used only by `stage::resume` R2 and `stage::load*`
/// callers that hold a program already admitted by `load` or `load_v3`.
/// It never converts a program between profiles.
pub(crate) fn require_staged(program: &Program) -> Result<(), Error> {
    match (program.language.as_str(), program.semantics.as_str()) {
        (LANGUAGE_2, SEMANTICS_2) => require_core2(program),
        (LANGUAGE_3, SEMANTICS_3) => require_core3(program),
        _ => Err(Error::new("profile", PROFILE_MESSAGE)),
    }
}
