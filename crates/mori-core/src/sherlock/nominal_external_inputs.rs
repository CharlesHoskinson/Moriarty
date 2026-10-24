//! R-only bounded decoder and binder for recursive nominal external arguments.
//!
//! In this proposed integrated candidate, this module is wired only through
//! `compile_nominal_source_host_bounded_with_nominal_inputs` and the paired
//! `--nominal-inputs`/`--nominal-function` CLI flags. Both the compile and
//! observe routes may select that wiring; they remain distinct downstream
//! modes, and decoding/binding here performs neither mode's execution nor
//! any native acceptance. This candidate is unrun, so no runtime or native
//! acceptance is claimed.
//! The caller must supply text already read through the existing bounded source
//! reader and already parsed through `crate::json::parse`. Those existing
//! layers provide the 1 MiB, duplicate-key, trailing-data, depth-32, and
//! 65,536-node refusals reused here; this module adds no codec or defaults.
//!
//! Binding order is exact: function lookup, supplied-name duplication,
//! declaration-order name equality, missing argument, extra argument, then
//! value decoding and recursive field validation. Constructor fields are also
//! declaration-ordered and are named in the wire so a missing, extra,
//! reordered, or renamed field is distinct. No constructor is inferred from
//! casing, no default is fabricated, no field is dropped, and no scalar is
//! coerced. List and Tree expected types are rejected because this wire
//! expresses only UInt, Bool, and nominal constructors.
//!
//! Partial values constructed before an error drop through the existing
//! iterative `Drop` implementations for ExtValue and TypeExprSrc. Installing
//! the result replaces the old entry, which likewise uses existing ExtExpr
//! iterative teardown. Deep nominal field type checking remains with the
//! existing first-T3 plus nominal body checker after installation; this
//! module nevertheless checks explicit constructor type applications,
//! declaration and constructor identity, type-application arity, declared
//! field names/order/count, and scalar-versus-constructor shape before T3.

use crate::sherlock::helper_machine::shared::CoreValue;
use crate::sherlock::helper_machine::shared::i128_class::KIntC;
use crate::sherlock::helper_machine::shared::owner_ext_source_program::ExtProgram;
use crate::sherlock::helper_machine::shared::owner_ext_t3_uses::{ExtExpr, ExtValue};
use crate::sherlock::helper_machine::shared::owner_ext_type_builder::TypeExprSrc;
use serde_json::Value;

pub const NOMINAL_EXTERNAL_INPUTS_FORMAT: &str = "sherlock-nominal-external-inputs/1";
/// Existing strict JSON parser depth bound mirrored for defense in depth.
pub const MAX_INPUT_JSON_DEPTH: usize = 32;

/// Caller-supplied existing parser extents. There is intentionally no
/// `Default`: Root's adopted caller maps these from DraftParseBudget.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NominalInputLimits {
    pub max_ast_occurrences: usize,
    pub max_structural_ast_depth: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct BoundArgumentProvenance {
    pub index: usize,
    pub name: String,
    pub declared_type: String,
    pub value_kind: &'static str,
    /// Concrete visible bound value, rendered from the decoded ExtValue:
    /// `UInt(42)`, `Bool(true)`, `Link(UInt(7),End())`. Entry constants only;
    /// not a witness, hash, or circuit identity.
    pub bound_value: String,
    pub value_nodes: usize,
    pub type_nodes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct NominalBindingProvenance {
    pub format: &'static str,
    pub function_index: usize,
    pub function_name: String,
    pub arguments: Vec<BoundArgumentProvenance>,
    pub synthesized_expr_nodes: usize,
    pub synthesized_value_nodes: usize,
    pub synthesized_type_nodes: usize,
    pub synthesized_structural_depth: usize,
    pub combined_ast_occurrences: usize,
}

/// Owned binding result. `entry` is the only synthesized expression. The
/// caller owns installation into ExtProgram; see install_bound_entry.
pub struct BoundNominalEntry {
    pub entry: ExtExpr,
    pub binding: NominalBindingProvenance,
}

/// Install by replacing ExtProgram.entry. The previous entry drops through
/// existing iterative ExtExpr Drop. Functions, types, assets, states, and
/// financial inputs are untouched.
pub fn install_bound_entry(
    program: &mut ExtProgram,
    bound: BoundNominalEntry,
) -> NominalBindingProvenance {
    let binding = bound.binding;
    program.entry = bound.entry;
    binding
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NominalInputError {
    LimitsInvalid {
        field: &'static str,
    },
    CountOverflow,
    InputDepthExceeded {
        depth: usize,
        limit: usize,
    },
    CombinedAstBudgetExceeded {
        existing: usize,
        synthesized: usize,
        limit: usize,
    },
    StructuralDepthExceeded {
        depth: usize,
        limit: usize,
    },
    MissingJsonField {
        object: &'static str,
        field: &'static str,
    },
    ExtraJsonField {
        object: &'static str,
        field: String,
    },
    JsonTypeMismatch {
        object: &'static str,
        field: &'static str,
        expected: &'static str,
    },
    FormatMismatch {
        actual: Option<String>,
    },
    UnsupportedValueKind {
        kind: String,
    },
    UnsupportedTypeKind {
        kind: String,
    },
    EmptyName {
        context: &'static str,
    },
    InvalidUInt {
        argument_index: usize,
        path: Vec<usize>,
        text: String,
    },
    UIntOutOfRange {
        argument_index: usize,
        path: Vec<usize>,
        text: String,
    },
    UnknownFunction {
        name: String,
    },
    AmbiguousFunction {
        name: String,
        count: usize,
    },
    DuplicateArgument {
        name: String,
        index: usize,
    },
    ArgumentNameMismatch {
        index: usize,
        expected: String,
        actual: String,
    },
    MissingArgument {
        index: usize,
        name: String,
    },
    ExtraArgument {
        index: usize,
        name: String,
    },
    UnknownType {
        name: String,
    },
    AmbiguousType {
        name: String,
        count: usize,
    },
    UnknownConstructor {
        type_name: String,
        ctor: String,
    },
    AmbiguousConstructor {
        type_name: String,
        ctor: String,
        count: usize,
    },
    TypeArityMismatch {
        type_name: String,
        expected: usize,
        actual: usize,
    },
    UnsupportedExpectedType {
        argument_index: usize,
        path: Vec<usize>,
        expected: String,
    },
    ValueKindMismatch {
        argument_index: usize,
        path: Vec<usize>,
        expected: String,
        actual: &'static str,
    },
    TypeMismatch {
        argument_index: usize,
        path: Vec<usize>,
        expected: String,
        actual: String,
    },
    MissingField {
        type_name: String,
        ctor: String,
        field_index: usize,
        field_name: String,
    },
    ExtraField {
        type_name: String,
        ctor: String,
        field_index: usize,
        actual_name: Option<String>,
    },
    FieldNameMismatch {
        type_name: String,
        ctor: String,
        field_index: usize,
        expected: String,
        actual: String,
    },
}

type Result<T> = std::result::Result<T, NominalInputError>;

#[derive(Default)]
struct Counts {
    value_nodes: usize,
    type_nodes: usize,
    max_value_depth: usize,
}

impl Counts {
    fn value(&mut self, depth: usize) -> Result<()> {
        self.value_nodes = self
            .value_nodes
            .checked_add(1)
            .ok_or(NominalInputError::CountOverflow)?;
        self.max_value_depth = self.max_value_depth.max(depth);
        Ok(())
    }
    fn ty(&mut self) -> Result<()> {
        self.type_nodes = self
            .type_nodes
            .checked_add(1)
            .ok_or(NominalInputError::CountOverflow)?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct Expected<'a> {
    ty: &'a TypeExprSrc,
    subst: Option<(&'a [String], &'a [TypeExprSrc])>,
}

fn resolved<'a>(expected: Expected<'a>) -> Expected<'a> {
    if let TypeExprSrc::Param { name } = expected.ty {
        if let Some((params, args)) = expected.subst {
            if let Some(index) = params.iter().position(|p| p == name) {
                return Expected {
                    ty: &args[index],
                    subst: None,
                };
            }
        }
    }
    expected
}

fn render_type(ty: &TypeExprSrc) -> String {
    match ty {
        TypeExprSrc::UInt => "UInt".into(),
        TypeExprSrc::Bool => "Bool".into(),
        TypeExprSrc::List => "List".into(),
        TypeExprSrc::Tree => "Tree".into(),
        TypeExprSrc::Named { name } => name.clone(),
        TypeExprSrc::Param { name } => name.clone(),
        TypeExprSrc::Apply { name, args } => {
            let args: Vec<String> = args.iter().map(render_type).collect();
            format!("{}<{}>", name, args.join(","))
        }
    }
}

/// Render the concrete decoded value. Recursion depth is bounded by the
/// decoder's depth-32 / structural-depth refusals enforced before this runs.
fn render_bound_value(value: &ExtValue) -> String {
    match value {
        ExtValue::Core { value } => match value {
            CoreValue::UInt { n } => match n {
                KIntC::InRange(v) => format!("UInt({})", v),
                _ => "UInt(out-of-range)".into(),
            },
            CoreValue::Bool { b } => format!("Bool({})", b),
            _ => "UnsupportedCore".into(),
        },
        ExtValue::Constructor { ctor, fields, .. } => {
            let fields: Vec<String> = fields.iter().map(render_bound_value).collect();
            format!("{}({})", ctor, fields.join(","))
        }
    }
}

fn type_head(ty: &TypeExprSrc) -> Option<(&str, &[TypeExprSrc])> {
    match ty {
        TypeExprSrc::Named { name } => Some((name.as_str(), &[])),
        TypeExprSrc::Apply { name, args } => Some((name.as_str(), args.as_slice())),
        _ => None,
    }
}

fn concrete_matches(concrete: &TypeExprSrc, expected: Expected<'_>) -> bool {
    let mut stack = vec![(concrete, expected)];
    while let Some((actual, pattern)) = stack.pop() {
        let pattern = resolved(pattern);
        match (actual, pattern.ty) {
            (TypeExprSrc::UInt, TypeExprSrc::UInt)
            | (TypeExprSrc::Bool, TypeExprSrc::Bool)
            | (TypeExprSrc::List, TypeExprSrc::List)
            | (TypeExprSrc::Tree, TypeExprSrc::Tree) => {}
            (TypeExprSrc::Named { name: a }, TypeExprSrc::Named { name: b }) if a == b => {}
            (
                TypeExprSrc::Apply { name: a, args: aa },
                TypeExprSrc::Apply { name: b, args: bb },
            ) if a == b && aa.len() == bb.len() => {
                for pair in aa.iter().zip(bb.iter()).rev() {
                    stack.push((
                        pair.0,
                        Expected {
                            ty: pair.1,
                            subst: pattern.subst,
                        },
                    ));
                }
            }
            _ => return false,
        }
    }
    true
}

fn object<'a>(
    value: &'a Value,
    object: &'static str,
) -> Result<&'a serde_json::Map<String, Value>> {
    value
        .as_object()
        .ok_or(NominalInputError::JsonTypeMismatch {
            object,
            field: "",
            expected: "object",
        })
}

fn exact_keys(
    map: &serde_json::Map<String, Value>,
    object: &'static str,
    allowed: &[&'static str],
) -> Result<()> {
    for key in map.keys() {
        if !allowed.iter().any(|a| a == key) {
            return Err(NominalInputError::ExtraJsonField {
                object,
                field: key.clone(),
            });
        }
    }
    for field in allowed {
        if !map.contains_key(*field) {
            return Err(NominalInputError::MissingJsonField { object, field });
        }
    }
    Ok(())
}

fn json_string<'a>(
    map: &'a serde_json::Map<String, Value>,
    object: &'static str,
    field: &'static str,
) -> Result<&'a str> {
    map.get(field)
        .and_then(Value::as_str)
        .ok_or(NominalInputError::JsonTypeMismatch {
            object,
            field,
            expected: "string",
        })
}

fn non_empty<'a>(text: &'a str, context: &'static str) -> Result<&'a str> {
    if text.is_empty() {
        Err(NominalInputError::EmptyName { context })
    } else {
        Ok(text)
    }
}

fn parse_uint(text: &str, argument_index: usize, path: &[usize]) -> Result<u128> {
    let invalid = || NominalInputError::InvalidUInt {
        argument_index,
        path: path.to_vec(),
        text: text.to_string(),
    };
    if text.is_empty()
        || !text.bytes().all(|b| b.is_ascii_digit())
        || (text.len() > 1 && text.starts_with('0'))
    {
        return Err(invalid());
    }
    text.parse::<u128>()
        .map_err(|_| NominalInputError::UIntOutOfRange {
            argument_index,
            path: path.to_vec(),
            text: text.to_string(),
        })
}

fn build_type(value: &Value, counts: &mut Counts, depth: usize) -> Result<TypeExprSrc> {
    if depth > MAX_INPUT_JSON_DEPTH {
        return Err(NominalInputError::InputDepthExceeded {
            depth,
            limit: MAX_INPUT_JSON_DEPTH,
        });
    }
    counts.ty()?;
    let map = object(value, "type")?;
    let kind = json_string(map, "type", "kind")?.to_string();
    match kind.as_str() {
        "UInt" => {
            exact_keys(map, "type", &["kind"])?;
            Ok(TypeExprSrc::UInt)
        }
        "Bool" => {
            exact_keys(map, "type", &["kind"])?;
            Ok(TypeExprSrc::Bool)
        }
        "Named" => {
            exact_keys(map, "type", &["kind", "name"])?;
            let name = non_empty(json_string(map, "type", "name")?, "type name")?.to_string();
            Ok(TypeExprSrc::Named { name })
        }
        "Apply" => {
            exact_keys(map, "type", &["kind", "name", "args"])?;
            let name = non_empty(json_string(map, "type", "name")?, "type name")?.to_string();
            let raw = map.get("args").and_then(Value::as_array).ok_or(
                NominalInputError::JsonTypeMismatch {
                    object: "type",
                    field: "args",
                    expected: "array",
                },
            )?;
            let mut args = Vec::with_capacity(raw.len());
            for arg in raw {
                args.push(build_type(arg, counts, depth + 1)?);
            }
            Ok(TypeExprSrc::Apply { name, args })
        }
        _ => Err(NominalInputError::UnsupportedTypeKind { kind }),
    }
}

struct Ctx<'a> {
    program: &'a ExtProgram,
    argument_index: usize,
    limits: NominalInputLimits,
}

#[allow(clippy::too_many_arguments)]
fn build_value(
    ctx: &Ctx<'_>,
    value: &Value,
    expected: Expected<'_>,
    path: &mut Vec<usize>,
    counts: &mut Counts,
    depth: usize,
) -> Result<ExtValue> {
    if depth > MAX_INPUT_JSON_DEPTH {
        return Err(NominalInputError::InputDepthExceeded {
            depth,
            limit: MAX_INPUT_JSON_DEPTH,
        });
    }
    if depth > ctx.limits.max_structural_ast_depth {
        return Err(NominalInputError::StructuralDepthExceeded {
            depth,
            limit: ctx.limits.max_structural_ast_depth,
        });
    }
    counts.value(depth)?;
    let expected = resolved(expected);
    match expected.ty {
        TypeExprSrc::UInt | TypeExprSrc::Bool => {
            let want_uint = matches!(expected.ty, TypeExprSrc::UInt);
            let map = object(value, "value")?;
            let kind = json_string(map, "value", "kind")?;
            if kind != if want_uint { "UInt" } else { "Bool" } {
                return Err(NominalInputError::ValueKindMismatch {
                    argument_index: ctx.argument_index,
                    path: path.clone(),
                    expected: render_type(expected.ty),
                    actual: match kind {
                        "UInt" => "UInt",
                        "Bool" => "Bool",
                        "Constructor" => "Constructor",
                        _ => "unsupported",
                    },
                });
            }
            exact_keys(map, "value", &["kind", "value"])?;
            if want_uint {
                let text = json_string(map, "value", "value")?;
                Ok(ExtValue::Core {
                    value: CoreValue::UInt {
                        n: KIntC::InRange(parse_uint(text, ctx.argument_index, path)?),
                    },
                })
            } else {
                let b = map.get("value").and_then(Value::as_bool).ok_or(
                    NominalInputError::JsonTypeMismatch {
                        object: "value",
                        field: "value",
                        expected: "bool",
                    },
                )?;
                Ok(ExtValue::Core {
                    value: CoreValue::Bool { b },
                })
            }
        }
        TypeExprSrc::Named { .. } | TypeExprSrc::Apply { .. } => {
            build_constructor(ctx, value, expected, path, counts, depth)
        }
        other => Err(NominalInputError::UnsupportedExpectedType {
            argument_index: ctx.argument_index,
            path: path.clone(),
            expected: render_type(other),
        }),
    }
}

#[allow(clippy::too_many_arguments)]
fn build_constructor(
    ctx: &Ctx<'_>,
    value: &Value,
    expected: Expected<'_>,
    path: &mut Vec<usize>,
    counts: &mut Counts,
    depth: usize,
) -> Result<ExtValue> {
    let map = object(value, "value")?;
    let kind = json_string(map, "value", "kind")?;
    if kind != "Constructor" {
        return Err(NominalInputError::ValueKindMismatch {
            argument_index: ctx.argument_index,
            path: path.clone(),
            expected: render_type(expected.ty),
            actual: match kind {
                "UInt" => "UInt",
                "Bool" => "Bool",
                _ => "unsupported",
            },
        });
    }
    exact_keys(map, "value", &["kind", "type", "ctor", "fields"])?;
    let concrete = build_type(&map["type"], counts, depth)?;
    if !concrete_matches(&concrete, expected) {
        return Err(NominalInputError::TypeMismatch {
            argument_index: ctx.argument_index,
            path: path.clone(),
            expected: render_type(expected.ty),
            actual: render_type(&concrete),
        });
    }
    let (type_name, type_args) =
        type_head(&concrete).ok_or(NominalInputError::UnsupportedExpectedType {
            argument_index: ctx.argument_index,
            path: path.clone(),
            expected: render_type(&concrete),
        })?;
    let type_name = type_name.to_string();
    let decls: Vec<_> = ctx
        .program
        .types
        .iter()
        .filter(|d| d.name == type_name)
        .collect();
    if decls.is_empty() {
        return Err(NominalInputError::UnknownType { name: type_name });
    }
    if decls.len() > 1 {
        return Err(NominalInputError::AmbiguousType {
            name: type_name,
            count: decls.len(),
        });
    }
    let decl = decls[0];
    if decl.parameters.len() != type_args.len() {
        return Err(NominalInputError::TypeArityMismatch {
            type_name,
            expected: decl.parameters.len(),
            actual: type_args.len(),
        });
    }
    let ctor_name = non_empty(json_string(map, "value", "ctor")?, "constructor")?.to_string();
    let ctors: Vec<_> = decl.ctors.iter().filter(|c| c.name == ctor_name).collect();
    if ctors.is_empty() {
        return Err(NominalInputError::UnknownConstructor {
            type_name,
            ctor: ctor_name,
        });
    }
    if ctors.len() > 1 {
        return Err(NominalInputError::AmbiguousConstructor {
            type_name,
            ctor: ctor_name,
            count: ctors.len(),
        });
    }
    let ctor = ctors[0];
    let raw_fields =
        map.get("fields")
            .and_then(Value::as_array)
            .ok_or(NominalInputError::JsonTypeMismatch {
                object: "value",
                field: "fields",
                expected: "array",
            })?;
    if raw_fields.len() < ctor.fields.len() {
        let field_index = raw_fields.len();
        return Err(NominalInputError::MissingField {
            type_name,
            ctor: ctor_name,
            field_index,
            field_name: ctor.fields[field_index].name.clone(),
        });
    }
    if raw_fields.len() > ctor.fields.len() {
        let field_index = ctor.fields.len();
        let actual_name = raw_fields[field_index]
            .as_object()
            .and_then(|m| m.get("name"))
            .and_then(Value::as_str)
            .map(str::to_string);
        return Err(NominalInputError::ExtraField {
            type_name,
            ctor: ctor_name,
            field_index,
            actual_name,
        });
    }
    let subst_args: &[TypeExprSrc] = match &concrete {
        TypeExprSrc::Apply { args, .. } => args.as_slice(),
        _ => &[],
    };
    let mut fields = Vec::with_capacity(raw_fields.len());
    for (index, raw) in raw_fields.iter().enumerate() {
        let field_map = object(raw, "field")?;
        exact_keys(field_map, "field", &["name", "value"])?;
        let actual = json_string(field_map, "field", "name")?;
        let declared = &ctor.fields[index];
        if actual != declared.name {
            return Err(NominalInputError::FieldNameMismatch {
                type_name: type_name.clone(),
                ctor: ctor_name.clone(),
                field_index: index,
                expected: declared.name.clone(),
                actual: actual.to_string(),
            });
        }
        path.push(index);
        let built = build_value(
            ctx,
            &field_map["value"],
            Expected {
                ty: &declared.ty,
                subst: Some((&decl.parameters, subst_args)),
            },
            path,
            counts,
            depth + 1,
        );
        path.pop();
        fields.push(built?);
    }
    Ok(ExtValue::Constructor {
        ty: concrete,
        ctor: ctor_name,
        fields,
    })
}

fn value_kind(value: &ExtValue) -> &'static str {
    match value {
        ExtValue::Core { value } => match value {
            CoreValue::UInt { .. } => "UInt",
            CoreValue::Bool { .. } => "Bool",
            _ => "UnsupportedCore",
        },
        ExtValue::Constructor { .. } => "Constructor",
    }
}

/// Decode the adopted carrier and bind it to one uniquely named function.
///
/// `existing_ast_occurrences` is supplied by the caller from the unchanged
/// parsed program, normally provenance expressions plus patterns. Synthesized
/// accounting is one Call, one Literal per argument, every ExtValue node, and
/// every decoded TypeExprSrc node. This accounting is enforced before the
/// caller may install the entry or start first-T3/body work.
pub fn decode_and_bind_nominal_inputs(
    program: &ExtProgram,
    function_name: &str,
    input: &Value,
    limits: NominalInputLimits,
    existing_ast_occurrences: usize,
) -> Result<BoundNominalEntry> {
    if limits.max_ast_occurrences == 0 {
        return Err(NominalInputError::LimitsInvalid {
            field: "max_ast_occurrences",
        });
    }
    if limits.max_structural_ast_depth == 0 {
        return Err(NominalInputError::LimitsInvalid {
            field: "max_structural_ast_depth",
        });
    }
    non_empty(function_name, "function name")?;
    let root = object(input, "nominal-inputs")?;
    exact_keys(root, "nominal-inputs", &["format", "arguments"])?;
    let format = json_string(root, "nominal-inputs", "format")?;
    if format != NOMINAL_EXTERNAL_INPUTS_FORMAT {
        return Err(NominalInputError::FormatMismatch {
            actual: Some(format.to_string()),
        });
    }
    let supplied = root.get("arguments").and_then(Value::as_array).ok_or(
        NominalInputError::JsonTypeMismatch {
            object: "nominal-inputs",
            field: "arguments",
            expected: "array",
        },
    )?;
    let matches: Vec<usize> = program
        .functions
        .iter()
        .enumerate()
        .filter(|(_, f)| f.name == function_name)
        .map(|(i, _)| i)
        .collect();
    if matches.is_empty() {
        return Err(NominalInputError::UnknownFunction {
            name: function_name.to_string(),
        });
    }
    if matches.len() > 1 {
        return Err(NominalInputError::AmbiguousFunction {
            name: function_name.to_string(),
            count: matches.len(),
        });
    }
    let function_index = matches[0];
    let function = &program.functions[function_index];

    let mut supplied_names = Vec::with_capacity(supplied.len());
    for raw in supplied {
        let map = object(raw, "argument")?;
        exact_keys(map, "argument", &["name", "value"])?;
        supplied_names
            .push(non_empty(json_string(map, "argument", "name")?, "argument name")?.to_string());
    }
    for (index, name) in supplied_names.iter().enumerate() {
        if supplied_names[..index].iter().any(|n| n == name) {
            return Err(NominalInputError::DuplicateArgument {
                name: name.clone(),
                index,
            });
        }
    }
    let common = supplied_names.len().min(function.parameters.len());
    for index in 0..common {
        if supplied_names[index] != function.parameters[index].name {
            return Err(NominalInputError::ArgumentNameMismatch {
                index,
                expected: function.parameters[index].name.clone(),
                actual: supplied_names[index].clone(),
            });
        }
    }
    if supplied_names.len() < function.parameters.len() {
        let index = supplied_names.len();
        return Err(NominalInputError::MissingArgument {
            index,
            name: function.parameters[index].name.clone(),
        });
    }
    if supplied_names.len() > function.parameters.len() {
        let index = function.parameters.len();
        return Err(NominalInputError::ExtraArgument {
            index,
            name: supplied_names[index].clone(),
        });
    }

    let mut counts = Counts::default();
    let mut values = Vec::with_capacity(function.parameters.len());
    let mut arguments = Vec::with_capacity(function.parameters.len());
    for (index, parameter) in function.parameters.iter().enumerate() {
        let before_values = counts.value_nodes;
        let before_types = counts.type_nodes;
        let ctx = Ctx {
            program,
            argument_index: index,
            limits,
        };
        let map = object(&supplied[index], "argument")?;
        let mut path = Vec::new();
        let value = build_value(
            &ctx,
            &map["value"],
            Expected {
                ty: &parameter.ty,
                subst: None,
            },
            &mut path,
            &mut counts,
            1,
        )?;
        arguments.push(BoundArgumentProvenance {
            index,
            name: parameter.name.clone(),
            declared_type: render_type(&parameter.ty),
            value_kind: value_kind(&value),
            bound_value: render_bound_value(&value),
            value_nodes: counts.value_nodes - before_values,
            type_nodes: counts.type_nodes - before_types,
        });
        values.push(value);
    }

    let synthesized_expr_nodes = 1usize
        .checked_add(values.len())
        .ok_or(NominalInputError::CountOverflow)?;
    let synthesized = synthesized_expr_nodes
        .checked_add(counts.value_nodes)
        .and_then(|n| n.checked_add(counts.type_nodes))
        .ok_or(NominalInputError::CountOverflow)?;
    let combined = existing_ast_occurrences
        .checked_add(synthesized)
        .ok_or(NominalInputError::CountOverflow)?;
    if combined > limits.max_ast_occurrences {
        return Err(NominalInputError::CombinedAstBudgetExceeded {
            existing: existing_ast_occurrences,
            synthesized,
            limit: limits.max_ast_occurrences,
        });
    }
    let synthesized_structural_depth = counts
        .max_value_depth
        .checked_add(2)
        .ok_or(NominalInputError::CountOverflow)?;
    if synthesized_structural_depth > limits.max_structural_ast_depth {
        return Err(NominalInputError::StructuralDepthExceeded {
            depth: synthesized_structural_depth,
            limit: limits.max_structural_ast_depth,
        });
    }

    Ok(BoundNominalEntry {
        entry: ExtExpr::Call {
            name: function_name.to_string(),
            args: values
                .into_iter()
                .map(|value| ExtExpr::Literal { value })
                .collect(),
        },
        binding: NominalBindingProvenance {
            format: NOMINAL_EXTERNAL_INPUTS_FORMAT,
            function_index,
            function_name: function_name.to_string(),
            arguments,
            synthesized_expr_nodes,
            synthesized_value_nodes: counts.value_nodes,
            synthesized_type_nodes: counts.type_nodes,
            synthesized_structural_depth,
            combined_ast_occurrences: combined,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sherlock::helper_machine::shared::owner_ext_t3_uses::{ExtFunction, ExtParamSrc};
    use crate::sherlock::helper_machine::shared::owner_ext_type_builder::{
        CtorDeclSrc, CtorFieldSrc, TypeDeclSrc,
    };

    fn chain_ty(arg: TypeExprSrc) -> TypeExprSrc {
        TypeExprSrc::Apply {
            name: "Chain".into(),
            args: vec![arg],
        }
    }
    fn function(name: &str) -> ExtFunction {
        ExtFunction {
            name: name.into(),
            parameters: vec![
                ExtParamSrc {
                    name: "chain".into(),
                    ty: chain_ty(TypeExprSrc::UInt),
                },
                ExtParamSrc {
                    name: "n".into(),
                    ty: TypeExprSrc::UInt,
                },
                ExtParamSrc {
                    name: "flag".into(),
                    ty: TypeExprSrc::Bool,
                },
            ],
            return_type: TypeExprSrc::UInt,
            body: ExtExpr::Var { name: "n".into() },
        }
    }
    fn program() -> ExtProgram {
        ExtProgram {
            types: vec![TypeDeclSrc {
                name: "Chain".into(),
                parameters: vec!["T".into()],
                ctors: vec![
                    CtorDeclSrc {
                        name: "End".into(),
                        fields: vec![],
                    },
                    CtorDeclSrc {
                        name: "Link".into(),
                        fields: vec![
                            CtorFieldSrc {
                                name: "head".into(),
                                ty: TypeExprSrc::Param { name: "T".into() },
                            },
                            CtorFieldSrc {
                                name: "tail".into(),
                                ty: chain_ty(TypeExprSrc::Param { name: "T".into() }),
                            },
                        ],
                    },
                ],
            }],
            assets: vec![],
            states: vec![],
            inputs: vec![],
            functions: vec![function("f")],
            entry: ExtExpr::Var {
                name: "unused".into(),
            },
        }
    }
    fn limits() -> NominalInputLimits {
        NominalInputLimits {
            max_ast_occurrences: 10_000,
            max_structural_ast_depth: 32,
        }
    }
    fn good() -> Value {
        serde_json::json!({
            "format": "sherlock-nominal-external-inputs/1",
            "arguments": [
                {"name": "chain", "value": {"kind": "Constructor",
                    "type": {"kind": "Apply", "name": "Chain", "args": [{"kind": "UInt"}]},
                    "ctor": "Link",
                    "fields": [
                        {"name": "head", "value": {"kind": "UInt", "value": "7"}},
                        {"name": "tail", "value": {"kind": "Constructor",
                            "type": {"kind": "Apply", "name": "Chain", "args": [{"kind": "UInt"}]},
                            "ctor": "End", "fields": []}}
                    ]}},
                {"name": "n", "value": {"kind": "UInt", "value": "42"}},
                {"name": "flag", "value": {"kind": "Bool", "value": true}}
            ]
        })
    }
    fn shape(value: &ExtValue) -> String {
        match value {
            ExtValue::Core { value } => match value {
                CoreValue::UInt { n } => match n {
                    KIntC::InRange(v) => format!("UInt({})", v),
                    _ => "UInt(out-of-range)".into(),
                },
                CoreValue::Bool { b } => format!("Bool({})", b),
                _ => "other".into(),
            },
            ExtValue::Constructor { ctor, fields, .. } => {
                let fields: Vec<String> = fields.iter().map(shape).collect();
                format!("{}({})", ctor, fields.join(","))
            }
        }
    }
    fn bind(input: &Value) -> Result<BoundNominalEntry> {
        decode_and_bind_nominal_inputs(&program(), "f", input, limits(), 0)
    }

    #[test]
    fn binds_recursive_generic_value_in_declared_order() {
        let bound = bind(&good()).unwrap();
        assert_eq!(bound.binding.function_name, "f");
        assert_eq!(bound.binding.arguments.len(), 3);
        assert_eq!(
            bound.binding.arguments[0].bound_value,
            "Link(UInt(7),End())"
        );
        assert_eq!(bound.binding.arguments[1].bound_value, "UInt(42)");
        match &bound.entry {
            ExtExpr::Call { name, args } => {
                assert_eq!(name, "f");
                assert_eq!(args.len(), 3);
                match &args[0] {
                    ExtExpr::Literal { value } => {
                        assert_eq!(shape(value), "Link(UInt(7),End())");
                    }
                    _ => panic!("first argument must be a literal"),
                }
                match &args[1] {
                    ExtExpr::Literal { value } => assert_eq!(shape(value), "UInt(42)"),
                    _ => panic!("second argument must be a literal"),
                }
                match &args[2] {
                    ExtExpr::Literal { value } => assert_eq!(shape(value), "Bool(true)"),
                    _ => panic!("third argument must be a literal"),
                }
            }
            _ => panic!("entry must be the synthesized call"),
        }
    }

    #[test]
    fn missing_extra_mismatch_and_duplicate_arguments_are_distinct() {
        let mut missing = good();
        missing["arguments"].as_array_mut().unwrap().pop();
        assert!(matches!(
            bind(&missing),
            Err(NominalInputError::MissingArgument { index: 2, .. })
        ));

        let mut extra = good();
        extra["arguments"]
            .as_array_mut()
            .unwrap()
            .push(serde_json::json!(
            {"name": "surplus", "value": {"kind": "UInt", "value": "1"}}));
        assert!(matches!(
            bind(&extra),
            Err(NominalInputError::ExtraArgument { index: 3, .. })
        ));

        let mut swapped = good();
        swapped["arguments"][0]["name"] = serde_json::json!("n");
        swapped["arguments"][1]["name"] = serde_json::json!("chain");
        assert!(matches!(
            bind(&swapped),
            Err(NominalInputError::ArgumentNameMismatch { index: 0, .. })
        ));

        let mut duplicate = good();
        duplicate["arguments"][1]["name"] = serde_json::json!("chain");
        assert!(matches!(
            bind(&duplicate),
            Err(NominalInputError::DuplicateArgument { index: 1, .. })
        ));
    }

    #[test]
    fn constructor_field_and_type_application_errors_are_distinct() {
        let mut unknown_ctor = good();
        unknown_ctor["arguments"][0]["value"]["ctor"] = serde_json::json!("Nope");
        assert!(matches!(
            bind(&unknown_ctor),
            Err(NominalInputError::UnknownConstructor { .. })
        ));

        let mut missing_field = good();
        missing_field["arguments"][0]["value"]["fields"]
            .as_array_mut()
            .unwrap()
            .pop();
        assert!(matches!(
            bind(&missing_field),
            Err(NominalInputError::MissingField { field_index: 1, .. })
        ));

        let mut renamed = good();
        renamed["arguments"][0]["value"]["fields"][1]["name"] = serde_json::json!("wrong");
        assert!(matches!(
            bind(&renamed),
            Err(NominalInputError::FieldNameMismatch { field_index: 1, .. })
        ));

        let mut wrong_application = good();
        wrong_application["arguments"][0]["value"]["type"]["args"][0] =
            serde_json::json!({"kind": "Bool"});
        assert!(matches!(
            bind(&wrong_application),
            Err(NominalInputError::TypeMismatch { .. })
        ));
    }

    #[test]
    fn invalid_uint_unknown_function_unsupported_type_and_budget_reject() {
        let mut invalid = good();
        invalid["arguments"][1]["value"]["value"] = serde_json::json!("042");
        assert!(matches!(
            bind(&invalid),
            Err(NominalInputError::InvalidUInt { .. })
        ));

        let mut overflow = good();
        overflow["arguments"][1]["value"]["value"] =
            serde_json::json!("340282366920938463463374607431768211456");
        assert!(matches!(
            bind(&overflow),
            Err(NominalInputError::UIntOutOfRange { .. })
        ));

        assert!(matches!(
            decode_and_bind_nominal_inputs(&program(), "missing", &good(), limits(), 0),
            Err(NominalInputError::UnknownFunction { .. })
        ));

        let mut p = program();
        p.functions[0].parameters[0].ty = TypeExprSrc::List;
        assert!(matches!(
            decode_and_bind_nominal_inputs(&p, "f", &good(), limits(), 0),
            Err(NominalInputError::UnsupportedExpectedType { .. })
        ));

        assert!(matches!(
            decode_and_bind_nominal_inputs(
                &program(),
                "f",
                &good(),
                NominalInputLimits {
                    max_ast_occurrences: 1,
                    max_structural_ast_depth: 32
                },
                0,
            ),
            Err(NominalInputError::CombinedAstBudgetExceeded { .. })
        ));
    }
}
