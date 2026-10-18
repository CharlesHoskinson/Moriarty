use mori_diagnostics::MoriDiagnostic;
use mori_span::Span;

pub fn action(span: Span, name: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("No checked action named `{name}` was found."))
        .with_code("mori::source6::unknown_action")
        .with_label(span.primary_label("select an action declared in this agreement"))
        .with_help("Pass the name after `action`, for example `pay`.")
}

pub fn unsupported(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("This Source/6 expansion slice supports only beta `transfer` actions.")
        .with_code("mori::source6::unsupported_operation")
        .with_label(span.primary_label("this checked operation has no expansion in this slice"))
        .with_help("Use the existing TypeScript bridge for other local S0 operations; family operations remain specified only.")
}

pub fn shape(span: Span, expected: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("Source/6 expansion expected {expected}."))
        .with_code("mori::source6::checked_shape")
        .with_label(span.primary_label("the checked representation does not match the transfer mapping"))
        .with_help("The expansion must consume the current checker output without changing its meaning.")
}

pub fn scenario(span: Span, field: &str, expected: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("Local scenario field `{field}` must contain {expected}."))
        .with_code("mori::source6::scenario_shape")
        .with_label(span.primary_label("scenario formation for this action; cells remain local claims"))
        .with_help("Supply the explicit local transfer cells in owner, recipient and fee-recipient order.")
}

pub fn bound(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("Generated Source/6 exceeds its 65,536-byte presentation bound.")
        .with_code("mori::source6::source_bound")
        .with_label(span.primary_label("expansion cannot be represented within Source/6's source limit"))
        .with_help("Use a proposal that fits the existing Source/6 profile.")
}
