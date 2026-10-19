//! Shared source-profile dispatch and the existing check report formats.

use mori_diagnostics::MoriDiagnostic;

use crate::json::CheckReport;

/// Consume checked values while the Beta AST they borrow is still alive.
/// Reports remain optional: a human consumer can render diagnostics directly.
pub(super) trait SourceCheckConsumer {
    type Output;

    /// Called after Beta parsing, before its checker starts.
    fn parsed(&mut self, _tokens: usize, _nodes: usize) {}

    fn beta(
        self,
        checked: Option<&mori_checker::Checked<'_>>,
        diagnostics: Vec<MoriDiagnostic>,
    ) -> Self::Output;

    fn core(
        self,
        result: Result<mori_core::Program, mori_core::Error>,
        staged: bool,
    ) -> Self::Output;
}

/// Recognize the additive `/3` route without changing legacy header errors.
/// A failed profile probe is never a successful program admission.
pub(super) fn public_core_language(source: &str) -> Result<&'static str, mori_core::Error> {
    match mori_core::language(source) {
        Ok(language) => Ok(language),
        Err(legacy_error) => match mori_core::language_v3(source) {
            Ok(mori_core::stage::LANGUAGE_3) => Ok(mori_core::stage::LANGUAGE_3),
            _ => Err(legacy_error),
        },
    }
}

/// Select one exact staged loader; never retry a failed loader as another profile.
/// Non-`/3` sources keep the existing `/2` loader and its original diagnostics.
pub(super) fn load_staged_source(
    source: &str,
) -> Result<mori_core::stage::StagedProgram, mori_core::Error> {
    match mori_core::language_v3(source) {
        Ok(mori_core::stage::LANGUAGE_3) => mori_core::stage::load_v3(source),
        _ => mori_core::stage::load(source),
    }
}

/// Select Beta, Core1, staged Core2 or staged Core3 for every check consumer.
pub(super) fn dispatch<C: SourceCheckConsumer>(source: &str, mut consumer: C) -> C::Output {
    if mori_lexer::first_token_text(source).is_ok_and(|token| token == "language") {
        let language = match public_core_language(source) {
            Ok(language) => language,
            Err(error) => return consumer.core(Err(error), false),
        };
        let staged = matches!(
            language,
            mori_core::stage::LANGUAGE_2 | mori_core::stage::LANGUAGE_3
        );
        let result = if staged {
            load_staged_source(source).map(|program| program.program().clone())
        } else {
            mori_core::parse(source)
        };
        return consumer.core(result, staged);
    }
    match mori_parser::parse(source) {
        Err(error) => consumer.beta(None, vec![error]),
        Ok(ast) => {
            consumer.parsed(ast.tokens.len(), ast.nodes.len());
            let mori_checker::CheckResult {
                checked,
                diagnostics,
            } = mori_checker::check(&ast);
            consumer.beta(Some(&checked), diagnostics)
        }
    }
}

/// Preserve the Beta struct's serialization order and diagnostic/span mapping.
pub(super) fn beta_report(
    file: String,
    source: &str,
    checked: Option<&mori_checker::Checked<'_>>,
    diagnostics: &[MoriDiagnostic],
) -> CheckReport {
    CheckReport::new(file, source, checked, diagnostics)
}

/// Construct the existing Core JSON object only when a consumer requests it.
pub(super) fn core_report(
    file: &str,
    source: &str,
    result: &Result<mori_core::Program, mori_core::Error>,
    staged: bool,
) -> serde_json::Value {
    let mut report = match result {
        Ok(program) => {
            serde_json::json!({"status":"accepted", "file":file,"source_hash":mori_core::intent::hash_bytes(source.as_bytes()),"language":program.language,"agreement":program.name,"actions":program.actions,"diagnostics":[]})
        }
        Err(error) => {
            serde_json::json!({"status":"rejected","file":file,"diagnostics":[error]})
        }
    };
    if let Ok(program) = result
        && program.language == mori_core::stage::LANGUAGE_3
    {
        report["semantics"] = serde_json::Value::String(program.semantics.clone());
    }
    if staged {
        let object = report
            .as_object_mut()
            .expect("stage reports are JSON objects");
        for (name, value) in mori_core::stage::STAGE_LABELS {
            object.insert(name.into(), serde_json::Value::String(value.into()));
        }
    }
    report
}
