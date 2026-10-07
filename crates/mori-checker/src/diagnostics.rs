//! Diagnostics reported while checking.

use mori_diagnostics::{MoriDiagnostic, corrected_line};
use mori_span::Span;

pub fn unknown_name(
    span: Span,
    name: &str,
    suggestion: Option<&str>,
    source: &str,
) -> MoriDiagnostic {
    let diagnostic = MoriDiagnostic::error(format!("I cannot find a declaration named `{name}`."))
        .with_code("mori::check::unknown_name")
        .with_label(span.label("not declared above"));
    match suggestion {
        Some(suggestion) => diagnostic.with_help_code(
            format!("Did you mean `{suggestion}`?"),
            corrected_line(source, span, suggestion),
        ),
        None => diagnostic.with_help(format!(
            "Declare `{name}` above this line. A declaration can only use the names \
             declared above it."
        )),
    }
}

pub fn used_before_declared(span: Span, name: &str, declared: Span) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("`{name}` is used before it is declared."))
        .with_code("mori::check::used_before_declared")
        .with_label(span.primary_label("used here"))
        .with_label(declared.label("declared further down"))
        .with_help(format!(
            "A declaration can only use the names declared above it. Move the \
             declaration of `{name}` above this line."
        ))
}

pub fn refers_to_itself(span: Span, name: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("`{name}` refers to itself."))
        .with_code("mori::check::refers_to_itself")
        .with_label(span.label("used in its own declaration"))
        .with_help("A value cannot be defined in terms of itself.")
}

pub fn duplicate_name(span: Span, earlier: Span, name: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("`{name}` is declared twice."))
        .with_code("mori::check::duplicate_name")
        .with_label(earlier.label("first declared here"))
        .with_label(span.primary_label("declared again here"))
        .with_help("Each declaration and action in an agreement needs its own name.")
}

pub fn action_is_not_a_value(span: Span, name: &str, action: Span) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "`{name}` is an action, so it cannot be used as a value."
    ))
    .with_code("mori::check::action_is_not_a_value")
    .with_label(span.primary_label("used as a value here"))
    .with_label(action.label("declared as an action here"))
    .with_help("Only declarations can be used in expressions. Refer to the intent the action uses instead.")
}

pub fn uses_non_intent(span: Span, name: &str, keyword: &str, declared: Span) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "An action must use an intent, but `{name}` is a `{keyword}`."
    ))
    .with_code("mori::check::uses_non_intent")
    .with_label(span.primary_label("this must name an intent"))
    .with_label(declared.label(format!("declared with `{keyword}` here")))
    .with_help_code(
        "An action runs an intent declared above it:",
        "intent invoice = { ... };\naction pay uses invoice;",
    )
}

pub fn reserved_by_source6(span: Span, name: &str, role: &str, source: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("`{name}` cannot be used as an {role} name."))
        .with_code("mori::check::reserved_by_source6")
        .with_label(span.label("reserved by Source/6"))
        .with_help_code(
            format!(
                "Agreements and actions are expanded into Source/6, which reserves \
                 `{name}`. Choose another name, for example:"
            ),
            corrected_line(source, span, &format!("{name}_{role}")),
        )
}
