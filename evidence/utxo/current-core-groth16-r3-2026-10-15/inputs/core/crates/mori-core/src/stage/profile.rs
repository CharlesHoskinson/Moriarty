//! Profile isolation between `moriarty-core/1` and `moriarty-core/2` consumers.
//! Both helpers keep the existing `mori::core::profile` error identity.
use super::{LANGUAGE_2, SEMANTICS_2};
use crate::{Error, LANGUAGE, Program, SEMANTICS, Statement};

const PROFILE_MESSAGE: &str = "Unsupported language or semantics identity.";

/// Required at every `/1` entry point before evaluation or layout. An
/// in-memory program labelled `/1` that still carries an Await is refused.
pub fn require_core1(program: &Program) -> Result<(), Error> {
    if program.language != LANGUAGE || program.semantics != SEMANTICS {
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

/// Required at every staged entry point; `/1` programs are refused.
pub fn require_core2(program: &Program) -> Result<(), Error> {
    if program.language != LANGUAGE_2 || program.semantics != SEMANTICS_2 {
        return Err(Error::new("profile", PROFILE_MESSAGE));
    }
    Ok(())
}
