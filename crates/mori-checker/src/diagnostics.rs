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

/// "add X and Y", "subtract Y from X", "take the min of X and Y".
fn combining(verb: &str, left: &str, right: &str) -> String {
    match verb {
        "subtract" => format!("subtract {right} from {left}"),
        _ => format!("{verb} {left} and {right}"),
    }
}

pub fn mixed_assets(
    left: Span,
    left_desc: &str,
    right: Span,
    right_desc: &str,
    verb: &str,
) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "I cannot {}.",
        combining(verb, left_desc, right_desc)
    ))
    .with_code("mori::check::mixed_assets")
    .with_label(left.label(format!("this is {left_desc}")))
    .with_label(right.primary_label(format!("this is {right_desc}")))
    .with_help(
        "Amounts of different assets never combine. Both sides must be amounts of the same asset.",
    )
}

pub fn number_and_amount(
    left: Span,
    left_desc: &str,
    right: Span,
    right_desc: &str,
    verb: &str,
    fix: Option<(Span, &str, &str)>,
) -> MoriDiagnostic {
    let diagnostic = MoriDiagnostic::error(format!(
        "I cannot {}.",
        combining(verb, left_desc, right_desc)
    ))
    .with_code("mori::check::number_and_amount")
    .with_label(left.label(format!("this is {left_desc}")))
    .with_label(right.primary_label(format!("this is {right_desc}")));
    match fix {
        Some((number, asset, source)) => diagnostic.with_help_code(
            format!("Give the number its asset, so both sides are `{asset}` amounts:"),
            corrected_line(source, Span::empty(number.end), &format!(" {asset}")),
        ),
        None => diagnostic.with_help("A plain number and an amount cannot be combined. Use amounts of the same asset on both sides."),
    }
}

pub fn amount_times_amount(left: Span, right: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("I cannot multiply two amounts.")
        .with_code("mori::check::amount_times_amount")
        .with_label(left.label("an amount"))
        .with_label(right.primary_label("another amount"))
        .with_help_code(
            "Multiply an amount by a plain number instead:",
            "2 * 10.00 USD",
        )
}

pub fn not_a_number(span: Span, desc: &str, verb: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("I cannot {verb} {desc}."))
        .with_code("mori::check::not_a_number")
        .with_label(span.primary_label(format!("this is {desc}")))
        .with_help("Arithmetic works only on plain numbers and amounts.")
}

pub fn below_zero(span: Span, shown: &str) -> MoriDiagnostic {
    MoriDiagnostic::error("This subtraction goes below zero.")
        .with_code("mori::check::below_zero")
        .with_label(span.primary_label(format!("{shown} is below zero")))
        .with_help("Numbers and amounts in Moriarty are never negative.")
}

pub fn too_large(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("This value is too large.")
        .with_code("mori::check::too_large")
        .with_label(span.primary_label("more than 2^128-1"))
        .with_help("Numbers and amounts are unsigned 128-bit integers.")
}

pub fn too_precise(
    span: Span,
    asset: &str,
    scale: u32,
    digits: usize,
    fixed: &str,
    source: &str,
) -> MoriDiagnostic {
    let places = match scale {
        0 => "no decimal places".to_owned(),
        1 => "1 decimal place".to_owned(),
        n => format!("{n} decimal places"),
    };
    let help = match scale {
        0 => "Write a whole amount, for example:".to_owned(),
        _ => format!("Use at most {places}, for example:"),
    };
    MoriDiagnostic::error(format!(
        "`{asset}` has {places}, but this amount has {digits}."
    ))
    .with_code("mori::check::too_precise")
    .with_label(span.primary_label("too many decimal places"))
    .with_help_code(help, corrected_line(source, span, fixed))
}

pub fn not_an_asset(span: Span, desc: String, declared: Span) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("I expected an asset here, but this is {desc}."))
        .with_code("mori::check::not_an_asset")
        .with_label(span.primary_label("not an asset"))
        .with_label(declared.label("declared here"))
}

pub fn not_a_domain(span: Span, desc: String, declared: Span) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("I expected a domain here, but this is {desc}."))
        .with_code("mori::check::not_a_domain")
        .with_label(span.primary_label("not a domain"))
        .with_label(declared.label("declared here"))
}

pub fn unknown_call(
    span: Span,
    name: &str,
    suggestion: Option<&str>,
    source: &str,
) -> MoriDiagnostic {
    let diagnostic = MoriDiagnostic::error(format!("There is no built-in call named `{name}`."))
        .with_code("mori::check::unknown_call")
        .with_label(span.primary_label("unknown call"));
    match suggestion {
        Some(suggestion) => diagnostic.with_help_code(
            format!("Did you mean `{suggestion}`?"),
            corrected_line(source, span, suggestion),
        ),
        None => diagnostic.with_help(
            "The built-in calls are `atoms`, `min`, `max`, `rounds`, `transfer`, `repay`, \
             and family calls such as `amm.swap_exact_input`.",
        ),
    }
}

pub fn unknown_argument(
    span: Span,
    call: &crate::calls::Call,
    key: &str,
    suggestion: Option<&str>,
) -> MoriDiagnostic {
    let help = match suggestion {
        Some(suggestion) => format!("Did you mean `{suggestion}`? `{}` takes:", call.name),
        None => format!("`{}` takes:", call.name),
    };
    MoriDiagnostic::error(format!("`{}` has no argument named `{key}`.", call.name))
        .with_code("mori::check::unknown_argument")
        .with_label(span.primary_label("unknown argument"))
        .with_help_code(help, call.signature())
}

pub fn missing_argument(span: Span, call: &crate::calls::Call, arg: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("`{}` is missing the argument `{arg}`.", call.name))
        .with_code("mori::check::missing_argument")
        .with_label(span.primary_label(format!("needs `{arg}`")))
        .with_help_code(format!("`{}` takes:", call.name), call.signature())
}

pub fn wrong_argument(
    span: Span,
    call: &str,
    arg: &str,
    role: crate::calls::Role,
    desc: &str,
) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "The argument `{arg}` of `{call}` must be {}, but this is {desc}.",
        role.describe()
    ))
    .with_code("mori::check::wrong_argument")
    .with_label(span.primary_label(format!("this is {desc}")))
}

pub fn expected_record(span: Span, keyword: &str, desc: &str) -> MoriDiagnostic {
    let article = if keyword.starts_with(['a', 'e', 'i', 'o', 'u']) {
        "An"
    } else {
        "A"
    };
    MoriDiagnostic::error(format!(
        "{article} `{keyword}` declaration needs a record of fields, but this is {desc}."
    ))
    .with_code("mori::check::expected_record")
    .with_label(span.primary_label("not a record"))
    .with_help_code(
        "Write its fields in braces, for example:",
        record_example(keyword),
    )
}

/// A typical record for a declaration kind.
fn record_example(keyword: &str) -> &'static str {
    match keyword {
        "domain" => "{ id: \"preview\", chain: \"midnight\", network: \"preview\" }",
        "asset" => "{ domain: Preview, id: \"usd\", scale: 2, representation: \"native\" }",
        "obligation" => "{ domain: Preview, id: \"loan\", asset: USD }",
        _ => "{ domain: Preview, id: \"alice\" }",
    }
}

pub fn invalid_scale(span: Span, desc: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "An asset's `scale` must be a whole number from 0 to 18, but this is {desc}."
    ))
    .with_code("mori::check::invalid_scale")
    .with_label(span.primary_label("invalid scale"))
}

pub fn unknown_type(
    span: Span,
    name: &str,
    suggestion: Option<&str>,
    source: &str,
) -> MoriDiagnostic {
    let diagnostic = MoriDiagnostic::error(format!("I do not know the type `{name}`."))
        .with_code("mori::check::unknown_type")
        .with_label(span.primary_label("unknown type"));
    match suggestion {
        Some(suggestion) => diagnostic.with_help_code(
            format!("Did you mean `{suggestion}`?"),
            corrected_line(source, span, suggestion),
        ),
        None => diagnostic.with_help(
            "The types are `Qty<ASSET>`, `Scalar`, `UInt128`, `String`, `Bool`, and \
             declared kinds such as `Account` or `Asset`.",
        ),
    }
}

pub fn wrong_type_arguments(span: Span, message: String) -> MoriDiagnostic {
    MoriDiagnostic::error(message)
        .with_code("mori::check::wrong_type_arguments")
        .with_label(span.primary_label("wrong type arguments"))
}

pub fn type_mismatch(
    type_span: Span,
    expected: &str,
    value_span: Span,
    desc: &str,
) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "The type says {expected}, but the value is {desc}."
    ))
    .with_code("mori::check::type_mismatch")
    .with_label(type_span.label("the declared type"))
    .with_label(value_span.primary_label(format!("this is {desc}")))
}

pub fn wrong_domain(
    arg: Span,
    expected: &str,
    value: Span,
    desc: &str,
    actual: &str,
) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "The type says domain `{expected}`, but {desc} is on `{actual}`."
    ))
    .with_code("mori::check::wrong_domain")
    .with_label(arg.label("the declared domain"))
    .with_label(value.primary_label(format!("this is on `{actual}`")))
}

pub fn unknown_field(
    span: Span,
    keyword: &str,
    name: &str,
    suggestion: Option<&str>,
    signature: &str,
) -> MoriDiagnostic {
    let kind = capitalize(&a_kind(keyword));
    let help = match suggestion {
        Some(suggestion) => format!("Did you mean `{suggestion}`? {kind} takes:"),
        None => format!("{kind} takes:"),
    };
    MoriDiagnostic::error(format!("{kind} has no field named `{name}`."))
        .with_code("mori::check::unknown_field")
        .with_label(span.primary_label("unknown field"))
        .with_help_code(help, signature)
}

pub fn missing_field(span: Span, keyword: &str, name: &str, signature: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "This {} is missing the field `{name}`.",
        keyword.replace('_', " ")
    ))
    .with_code("mori::check::missing_field")
    .with_label(span.primary_label(format!("needs `{name}`")))
    .with_help_code(
        format!(
            "{} takes these fields; `?` marks optional ones:",
            capitalize(&a_kind(keyword))
        ),
        signature,
    )
}

pub fn wrong_field(
    span: Span,
    keyword: &str,
    name: &str,
    role: crate::declarations::FieldRole,
    desc: &str,
) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "The field `{name}` of {} must be {}, but this is {desc}.",
        a_kind(keyword),
        role.describe()
    ))
    .with_code("mori::check::wrong_field")
    .with_label(span.primary_label(format!("this is {desc}")))
}

pub fn wrong_list_item(
    span: Span,
    keyword: &str,
    name: &str,
    role: crate::declarations::FieldRole,
    desc: &str,
) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "The field `{name}` of {} must be {}, but this item is {desc}.",
        a_kind(keyword),
        role.describe()
    ))
    .with_code("mori::check::wrong_field")
    .with_label(span.primary_label(format!("this is {desc}")))
}

pub fn invalid_id(span: Span, id: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("The id `{id}` cannot be used in Source/6."))
        .with_code("mori::check::invalid_id")
        .with_label(span.primary_label("not a valid Source/6 id"))
        .with_help(
            "Ids start with an ASCII letter, continue with letters, digits or `_`, are at most \
             64 characters, and avoid Source/6's reserved words.",
        )
}

pub fn duplicate_id(
    span: Span,
    earlier: Span,
    keyword: &str,
    id: &str,
    domain: Option<&str>,
) -> MoriDiagnostic {
    let place = domain
        .map(|domain| format!(" on `{domain}`"))
        .unwrap_or_default();
    MoriDiagnostic::error(format!(
        "Another {}{place} already has the id `{id}`.",
        keyword.replace('_', " ")
    ))
    .with_code("mori::check::duplicate_id")
    .with_label(earlier.label("first used here"))
    .with_label(span.primary_label("used again here"))
    .with_help("Each economic identity is declared once. Refer to the existing declaration instead of declaring an alias.")
}

pub fn wrong_asset_domain(
    span: Span,
    asset: &str,
    asset_domain: &str,
    keyword: &str,
    domain: &str,
) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "The asset `{asset}` is on `{asset_domain}`, but this {} is on `{domain}`.",
        keyword.replace('_', " ")
    ))
    .with_code("mori::check::wrong_asset_domain")
    .with_label(span.primary_label(format!("on `{asset_domain}`")))
    .with_help("An asset used by a declaration must be on the same domain.")
}

pub fn empty_pool(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("A pool needs at least one asset.")
        .with_code("mori::check::empty_pool")
        .with_label(span.primary_label("no assets"))
        .with_help_code("List the assets the pool holds:", "assets: [USD, GOLD]")
}

pub fn duplicate_pool_asset(span: Span, earlier: Span, asset: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("The asset `{asset}` appears twice in this pool."))
        .with_code("mori::check::duplicate_pool_asset")
        .with_label(earlier.label("first here"))
        .with_label(span.primary_label("and again here"))
}

/// "an asset", "a share class".
fn a_kind(keyword: &str) -> String {
    let words = keyword.replace('_', " ");
    let article = if words.starts_with(['a', 'e', 'i', 'o', 'u']) {
        "an"
    } else {
        "a"
    };
    format!("{article} {words}")
}

fn capitalize(text: &str) -> String {
    let mut chars = text.chars();
    chars
        .next()
        .map(|first| first.to_uppercase().chain(chars).collect())
        .unwrap_or_default()
}

/// Items joined on one line, or one per line when that would be too long.
pub fn layout(open: &str, items: &[String], close: &str) -> String {
    let line = format!("{open}{}{close}", items.join(", "));
    if line.len() <= 60 {
        return line;
    }
    let open = open.trim_end();
    let close = close.trim_start();
    let body: Vec<String> = items.iter().map(|item| format!("  {item},")).collect();
    format!("{open}\n{}\n{close}", body.join("\n"))
}

pub fn inverted_rounds(from: Span, start: u128, to: Span, end: u128) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "This round window ends before it starts: `from` is {start} but `to` is {end}."
    ))
    .with_code("mori::check::inverted_rounds")
    .with_label(from.label("starts here"))
    .with_label(to.primary_label("ends earlier"))
    .with_help("`from` must be less than or equal to `to`.")
}

pub fn same_account(
    earlier: Span,
    earlier_role: &str,
    span: Span,
    role: &str,
    account: &str,
) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "`{earlier_role}` and `{role}` are the same account, `{account}`."
    ))
    .with_code("mori::check::same_account")
    .with_label(earlier.label(format!("`{earlier_role}`")))
    .with_label(span.primary_label(format!("`{role}` is the same account")))
    .with_help(
        "A transfer needs three different accounts. Even when the fee is zero, `fee_to` \
         must be its own account.",
    )
}

pub fn different_domains(
    span: Span,
    role: &str,
    domain: &str,
    first: Span,
    first_role: &str,
    expected: &str,
) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "`{role}` is on `{domain}`, but `{first_role}` is on `{expected}`."
    ))
    .with_code("mori::check::different_domains")
    .with_label(first.label(format!("on `{expected}`")))
    .with_label(span.primary_label(format!("on `{domain}`")))
    .with_help("Everything an operation touches must be on one domain.")
}

pub fn wrong_asset(
    span: Span,
    what: &str,
    expected: &str,
    actual: &str,
    because: &str,
) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "{what} must be a `{expected}` amount because {because}, but it is a `{actual}` amount."
    ))
    .with_code("mori::check::wrong_asset")
    .with_label(span.primary_label(format!("a `{actual}` amount")))
}

pub fn not_in_pool(span: Span, asset: &str, pool: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "`{asset}` is not one of the pool `{pool}`'s assets."
    ))
    .with_code("mori::check::not_in_pool")
    .with_label(span.primary_label("not in the pool"))
    .with_help("A pool only trades the assets listed in its `assets` field.")
}

pub fn instrument_missing_field(
    span: Span,
    call: &str,
    instrument: &str,
    field: &str,
) -> MoriDiagnostic {
    let article = if field.starts_with(['a', 'e', 'i', 'o', 'u']) {
        "an"
    } else {
        "a"
    };
    MoriDiagnostic::error(format!(
        "`{call}` needs the instrument `{instrument}` to have {article} `{field}` field."
    ))
    .with_code("mori::check::instrument_missing_field")
    .with_label(span.primary_label(format!("`{instrument}` has no `{field}`")))
    .with_help_code(
        format!("Add `{field}` to the instrument's declaration, naming an asset:"),
        format!("{field}: USD,"),
    )
}

pub fn intent_without_operation(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("This intent has no `operation`.")
        .with_code("mori::check::intent_without_operation")
        .with_label(span.primary_label("needs an `operation`"))
        .with_help_code(
            "An intent authorizes one operation, written as a call:",
            "operation: transfer(\n  from: alice, to: bob, fee_to: carol,\n  value: 10 USD, fee: 0 USD,\n),",
        )
}

pub fn operation_not_a_call(span: Span, desc: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "An intent's `operation` must be a call, but this is {desc}."
    ))
    .with_code("mori::check::operation_not_a_call")
    .with_label(span.primary_label(format!("this is {desc}")))
    .with_help("Write the operation as a call, like `transfer(...)` or `repay(...)`.")
}

pub fn unsupported_operation(span: Span, name: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("`{name}` cannot be an intent's operation."))
        .with_code("mori::check::unsupported_operation")
        .with_label(span.primary_label("not an operation"))
        .with_help(
            "An intent runs `transfer` or `repay`, or a family call such as \
             `amm.swap_exact_input`.",
        )
}

pub fn empty_claim(span: Span, field: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("The intent's `{field}` cannot be empty."))
        .with_code("mori::check::empty_claim")
        .with_label(span.primary_label("empty"))
        .with_help("This value is part of what the owner signs, so it must be filled in.")
}

pub fn s0_policy(span: Span, field: &str, expected: &str, desc: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "A `transfer` or `repay` intent requires `{field}: {expected}`, but this is {desc}."
    ))
    .with_code("mori::check::s0_policy")
    .with_label(span.primary_label(format!("must be `{expected}`")))
    .with_help(
        "Intents that run locally use the simplest policy: they succeed or fail as a \
         whole, with nothing observed, disclosed, retained, delegated or recovered.",
    )
}

pub fn wrong_signer(
    span: Span,
    signer: &str,
    role: &str,
    account: &str,
    account_span: Span,
) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "The signer must be the `{role}` account, `{account}`, but it is `{signer}`."
    ))
    .with_code("mori::check::wrong_signer")
    .with_label(account_span.label(format!("the `{role}` account")))
    .with_label(span.primary_label("the signer"))
    .with_help("Only the owner of the funds can sign for them.")
}

pub fn wrong_obligation_asset(span: Span, owed: &str, because: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "This obligation is owed in `{owed}`, but {because}."
    ))
    .with_code("mori::check::wrong_obligation_asset")
    .with_label(span.primary_label(format!("owed in `{owed}`")))
    .with_help("A repayment intent's `asset` must be the asset the obligation is owed in.")
}

pub fn wrong_intent_asset(span: Span, declared: &str, because: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "The intent's `asset` is `{declared}`, but {because}."
    ))
    .with_code("mori::check::wrong_intent_asset")
    .with_label(span.primary_label(format!("`{declared}`")))
}

pub fn invalid_validity(span: Span, s0: bool) -> MoriDiagnostic {
    let (message, example) = if s0 {
        (
            "A `transfer` or `repay` intent's `valid` must be a `rounds(...)` window.",
            "valid: rounds(domain: Preview, from: 0, to: 100),",
        )
    } else {
        (
            "An intent's `valid` must be a `rounds(...)` window or two round numbers.",
            "valid: rounds(domain: Preview, from: 0, to: 100),\nvalid: [0, 100],",
        )
    };
    MoriDiagnostic::error(message)
        .with_code("mori::check::invalid_validity")
        .with_label(span.primary_label("not a round window"))
        .with_help_code("For example:", example)
}

pub fn too_large_for_s0(span: Span, field: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "`{field}` is too large for an operation that runs locally."
    ))
    .with_code("mori::check::too_large_for_s0")
    .with_label(span.primary_label("more than 2^127-1"))
}
