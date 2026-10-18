//! Exact first-batch wire configuration. No missing numerical value has a default.
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::sherlock::helper_machine::core_action_consumer::{ConsumerConfig, ConsumerProfile, SourceHostBounds};
use crate::sherlock::helper_machine::shared::{BoundsC, FinanceInput, GrammarDomain, LayoutCapacitiesC, LayoutSchemaC, RunIntC};

fn canonical_decimal(text: &str) -> bool {
    text == "0" || (!text.is_empty() && !text.starts_with('0') && text.bytes().all(|b| b.is_ascii_digit()))
}

macro_rules! decimal {
    ($name:ident, $ty:ty) => {
        #[derive(Clone, Copy)]
        pub struct $name(pub $ty);
        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.collect_str(&self.0)
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let text = String::deserialize(deserializer)?;
                if !canonical_decimal(&text) {
                    return Err(serde::de::Error::custom("expected canonical nonnegative decimal string"));
                }
                text.parse::<$ty>().map(Self).map_err(serde::de::Error::custom)
            }
        }
    };
}
decimal!(UsizeDecimal, usize);
decimal!(U128Decimal, u128);

// String-only markers follow the schema's literal string domain. Custom
// deserialization avoids accepting externally tagged unit-enum map forms.
macro_rules! fixed_tag {
    ($name:ident, $variant:ident, $text:literal) => {
        #[derive(Clone, Copy)]
        pub enum $name { $variant }
        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str($text)
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let text = String::deserialize(deserializer)?;
                if text == $text { Ok(Self::$variant) }
                else { Err(serde::de::Error::custom(concat!("expected ", $text))) }
            }
        }
    };
}
fixed_tag!(ConfigFormat, V1, "sherlock-host-config/1");
fixed_tag!(Profile, HelperMicrostep1, "sherlock-helper-microstep/1");
fixed_tag!(Grammar, DedicatedSorts, "DedicatedSorts");
fixed_tag!(Finance, Absent, "Absent");

/// Required JSON null; unlike Option this does not accept an absent field.
#[derive(Clone, Copy)]
pub struct RequiredNull;
impl Serialize for RequiredNull {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> { serializer.serialize_unit() }
}
impl<'de> Deserialize<'de> for RequiredNull {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <()>::deserialize(deserializer)?;
        Ok(Self)
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum RunInput { Nat { value: U128Decimal }, Negative, NatAboveU128 }
impl RunInput {
    pub(crate) fn owner(self) -> RunIntC {
        match self { Self::Nat { value } => RunIntC::Nat(value.0),
            Self::Negative => RunIntC::Negative, Self::NatAboveU128 => RunIntC::NatAboveU128 }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bounds {
    pub code_rows: UsizeDecimal,
    pub function_rows: UsizeDecimal,
    pub code_child_slots: UsizeDecimal,
    pub parameter_slots: UsizeDecimal,
    pub pattern_rows: UsizeDecimal,
    pub match_plan_rows: UsizeDecimal,
    pub match_arm_rows: UsizeDecimal,
    pub type_rows: UsizeDecimal,
    pub constructor_rows: UsizeDecimal,
    pub constructor_field_slots: UsizeDecimal,
    pub max_arity: UsizeDecimal,
    pub constant_nodes: UsizeDecimal,
    pub constant_field_slots: UsizeDecimal,
    pub data_nodes: UsizeDecimal,
    pub data_field_slots: UsizeDecimal,
    pub environment_cells: UsizeDecimal,
    pub frames: UsizeDecimal,
    pub helper_tasks: UsizeDecimal,
    pub serialization_bytes: UsizeDecimal,
    pub max_reference_work: U128Decimal,
    pub max_helper_work: U128Decimal,
    pub step_cap: U128Decimal,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HostBounds {
    pub max_source_actions: UsizeDecimal,
    pub max_source_expressions: UsizeDecimal,
    pub max_source_statements: UsizeDecimal,
    pub max_action_inputs: UsizeDecimal,
    pub max_supplied_inputs: UsizeDecimal,
    pub max_projection_occurrences: UsizeDecimal,
    pub max_generated_ast_nodes: UsizeDecimal,
    pub max_ast_nesting: UsizeDecimal,
    pub max_input_name_bytes: UsizeDecimal,
    pub max_total_input_name_bytes: UsizeDecimal,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Capacities {
    pub name_rows: UsizeDecimal,
    pub name_bytes: UsizeDecimal,
    pub occurrence_path_slots: UsizeDecimal,
    pub value_slots: UsizeDecimal,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Layout {
    pub layout_version: [String; 2],
    pub field_id: RequiredNull,
    pub widths: RequiredNull,
    pub scalar_layout_id: RequiredNull,
    pub capacities: Capacities,
    pub boundary_capacities_present: bool,
}

