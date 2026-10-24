//! Unregistered A82 (`k.admin.publish.error`) full-image instruction fragment;
//! no SDK execution, witness producer, `Machine::step`, or host acceptance flag.
//!
//! Enforces the uncharged administrative terminal error publication transition
//! `<k> uiError(C) => .K </k> <out> uiPending() => uiRejected(C) </out>`
//! (K source line 82, wire `rule_tag = 1`, `charge = 0`), which fires before
//! line 84 zero-fuel exhaustion even when `remainingFuel == 0`.
//!
//! Unlike A80 (`Return` publication), `Error` publication imposes no empty-stack
//! (`depth == 0`) restriction and performs no `ValueRef` active-prefix lookup:
//! it publishes for any represented stack depth and preserves all `FrameSlots`
//! rows, `depth`, `environment`, all five state arenas, and all 16 little-endian
//! `remainingFuel` and `initialFuel` bytes. `ControlRows[0].errorCode` accepts
//! all 256 `u8` values (`0..=255`; L4 forbids an error-code whitelist) and is
//! copied directly to `post.root.outcomeError` with `outcomeTag = 2` (`Rejected`)
//! and zeroed `outcomeValue.{space,index}`. Full `StepInv` (`O1`..`O4`), `F4`,
//! `OBL.G4` byte binding, `projectA`, and 39-way dispatch remain open caller
//! obligations.

use super::literal_host::{tag, unsigned};
use super::*;

pub const ERROR_PUBLICATION_RULE_ID: &str = "k.admin.publish.error";
pub const ERROR_PUBLICATION_RULE_START_LINE: u128 = 82;
pub const ERROR_PUBLICATION_RULE_TAG: u128 = 1;
pub const ERROR_PUBLICATION_RULE_CHARGE: u128 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct ErrorPublicationStepInputs {
    pub enable: Operand,
    /// One complete immutable ProgramBlock shared by both state images.
    pub program: LiteralImage,
    pub pre: LiteralImage,
    /// Independent witness operands; every enabled post cell is constrained.
    pub post: LiteralImage,
    pub envelope: LiteralEnvelope,
}

impl From<LiteralStepInputs> for ErrorPublicationStepInputs {
    fn from(value: LiteralStepInputs) -> Self {
        Self {
            enable: value.enable,
            program: value.program,
            pre: value.pre,
            post: value.post,
            envelope: value.envelope,
        }
    }
}

impl From<ErrorPublicationStepInputs> for LiteralStepInputs {
    fn from(value: ErrorPublicationStepInputs) -> Self {
        Self {
            enable: value.enable,
            program: value.program,
            pre: value.pre,
            post: value.post,
            envelope: value.envelope,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ErrorPublicationStepBlock {
    pub instructions: Vec<Instruction>,
    /// Operand aliases only; this is not a transition/acceptance result.
    pub preserved: ErrorPublicationStepInputs,
}

type Result<T> = std::result::Result<T, LiteralConstructionError>;

/// Check whether `name` collides with the temporary namespace reserved by
/// [`emit_error_publication_step`] under `scope`.
pub fn is_reserved_error_publication_variable(name: &str, scope: &str) -> bool {
    let local = format!("%{scope}.local.temp.");
    name.starts_with(&local)
}

fn check_image(
    image: &LiteralImage,
    root: &[LiteralLane],
    tables: &[LiteralTable],
    block: &'static str,
) -> Result<()> {
    if image.root.len() != root.len() || image.tables.len() != tables.len() {
        return Err(LiteralConstructionError::ImageShape {
            block,
            table: None,
            row: None,
        });
    }
    for (i, table) in tables.iter().enumerate() {
        if image.tables[i].len() != table.capacity {
            return Err(LiteralConstructionError::ImageShape {
                block,
                table: Some(i),
                row: None,
            });
        }
        for (row, cells) in image.tables[i].iter().enumerate() {
            if cells.len() != table.lanes.len() {
                return Err(LiteralConstructionError::ImageShape {
                    block,
                    table: Some(i),
                    row: Some(row),
                });
            }
        }
    }
    Ok(())
}

fn operands(image: &LiteralImage) -> impl Iterator<Item = &Operand> {
    image
        .root
        .iter()
        .chain(image.tables.iter().flatten().flatten())
}

fn field(lanes: &[LiteralLane], row: &[Operand], name: &str) -> Operand {
    // Only constructor-owned schema names reach here, after exact schema check.
    row[lanes
        .iter()
        .position(|lane| lane.name == name)
        .expect("fixed L4 column")]
    .clone()
}

struct Emitter {
    scope: String,
    serial: usize,
    code: Vec<Instruction>,
}

impl Emitter {
    fn fresh(&mut self) -> Result<Identifier> {
        let current = self.serial;
        self.serial = self
            .serial
            .checked_add(1)
            .ok_or(LiteralConstructionError::SizeOverflow)?;
        Ok(Identifier(format!("%{}.local.temp.{current}", self.scope)))
    }

    fn select(&mut self, bit: Operand, a: Operand, b: Operand) -> Result<Operand> {
        let output = self.fresh()?;
        self.code.push(Instruction::CondSelect {
            bit,
            a,
            b,
            output: output.clone(),
        });
        Ok(Operand::Variable(output))
    }

    fn eq(&mut self, a: Operand, b: Operand) {
        self.code.push(Instruction::ConstrainEq { a, b });
    }

    fn gated_eq(&mut self, enable: &Operand, a: Operand, b: Operand) -> Result<()> {
        let guarded = self.select(enable.clone(), a, b.clone())?;
        self.eq(guarded, b);
        Ok(())
    }

    fn boolean(&mut self, val: Operand) {
        self.code.push(Instruction::ConstrainToBoolean { val });
    }

    fn bits(&mut self, val: Operand, bits: u32) {
        self.code.push(Instruction::ConstrainBits { val, bits });
    }

    fn less(&mut self, a: Operand, b: Operand) -> Result<Operand> {
        let output = self.fresh()?;
        self.code.push(Instruction::LessThan {
            a,
            b,
            bits: 128,
            output: output.clone(),
        });
        let value = Operand::Variable(output);
        self.boolean(value.clone());
        Ok(value)
    }

    fn ranges(&mut self, lanes: &[LiteralLane], row: &[Operand]) -> Result<()> {
        for (lane, value) in lanes.iter().zip(row) {
            if lane.boolean {
                self.boolean(value.clone());
            } else {
                self.bits(value.clone(), lane.bits);
            }
            if let Some(maximum) = lane.maximum {
                let exceeds = self.less(immediate(maximum), value.clone())?;
                self.eq(exceeds, immediate(0));
            }
        }
        Ok(())
    }

    fn image_ranges(
        &mut self,
        image: &LiteralImage,
        root: &[LiteralLane],
        tables: &[LiteralTable],
    ) -> Result<()> {
        self.ranges(root, &image.root)?;
        for (table, rows) in tables.iter().zip(&image.tables) {
            for row in rows {
                self.ranges(&table.lanes, row)?;
            }
        }
        Ok(())
    }
}

/// Emits real ZKIR instructions for source line 82 (`k.admin.publish.error`,
/// wire `rule_tag = 1`, `charge = 0`): `Pending` `Control::Error(C)` for any
/// `u8` payload `C` (`0..=255`) and any represented frame stack transitions to
/// `control = None` (`controlLength == 0`, zeroed `ControlRows`) and
/// `Outcome::Rejected(C)` (`outcomeTag == 2`, `outcomeValue.{space,index} == 0`,
/// `outcomeError == C`), preserving `depth`, `environment`, `stackLimit`, all
/// 16 little-endian `remainingFuel` and `initialFuel` bytes, and all five state
/// arenas (`Data`, `DataFields`, `EnvCells`, `ValueSlots`, `FrameSlots`).
///
/// All cells receive unconditional scalar range constraints even when `enable`
/// is disabled, matching the G3 fetch, `emit_literal_step`, and
/// `emit_return_publication_step` contract.
pub fn emit_error_publication_step(
    schema: &LiteralSchema,
    input: &ErrorPublicationStepInputs,
    scope: &str,
) -> Result<ErrorPublicationStepBlock> {
    if &literal_schema(schema.capacities())? != schema {
        return Err(LiteralConstructionError::SchemaMismatch);
    }
    check_image(
        &input.program,
        &schema.program_header,
        &schema.program_tables,
        "program",
    )?;
    check_image(&input.pre, &schema.state_root, &schema.state_tables, "pre")?;
    check_image(
        &input.post,
        &schema.state_root,
        &schema.state_tables,
        "post",
    )?;
    if scope.is_empty() {
        return Err(ConstructionError::EmptyScope.into());
    }
    for operand in [
        &input.enable,
        &input.envelope.rule_tag,
        &input.envelope.charge,
    ]
    .into_iter()
    .chain(operands(&input.program))
    .chain(operands(&input.pre))
    .chain(operands(&input.post))
    {
        if let Operand::Variable(name) = operand {
            if !name.0.starts_with('%') {
                return Err(ConstructionError::UnprefixedVariable.into());
            }
            if is_reserved_error_publication_variable(&name.0, scope) {
                return Err(ConstructionError::TemporaryNameCollision.into());
            }
        }
    }

    let mut b = Emitter {
        scope: scope.into(),
        serial: 0,
        code: Vec::new(),
    };
    let enable = &input.enable;
    b.boolean(enable.clone());
    b.image_ranges(
        &input.program,
        &schema.program_header,
        &schema.program_tables,
    )?;
    b.image_ranges(&input.pre, &schema.state_root, &schema.state_tables)?;
    b.image_ranges(&input.post, &schema.state_root, &schema.state_tables)?;
    b.ranges(
        &[tag("rule_tag", 39), unsigned("charge", 8, None)],
        &[
            input.envelope.rule_tag.clone(),
            input.envelope.charge.clone(),
        ],
    )?;

    // Envelope: wire tag 1 (line 82) and administrative charge 0.
    b.gated_eq(
        enable,
        input.envelope.rule_tag.clone(),
        immediate(ERROR_PUBLICATION_RULE_TAG),
    )?;
    b.gated_eq(
        enable,
        input.envelope.charge.clone(),
        immediate(ERROR_PUBLICATION_RULE_CHARGE),
    )?;

    // Pre-state Root outcome must be Pending (0) with zeroed inactive outcome lanes.
    for name in [
        "outcomeTag",
        "outcomeValue.space",
        "outcomeValue.index",
        "outcomeError",
    ] {
        b.gated_eq(
            enable,
            field(&schema.state_root, &input.pre.root, name),
            immediate(0),
        )?;
    }

    // Pre-state Control must be a 1-row spine with ControlRows[0] = Error(errorCode),
    // every inactive column of row 0 (including all pendingFrame columns) equal to 0,
    // and all inactive rows (1..C) equal to 0. errorCode accepts any u8 (0..=255).
    b.gated_eq(
        enable,
        field(&schema.state_root, &input.pre.root, "controlLength"),
        immediate(1),
    )?;
    let controls = &schema.state_tables[5].lanes;
    for (row, values) in input.pre.tables[5].iter().enumerate() {
        for (lane, value) in controls.iter().zip(values) {
            if row == 0 && lane.name == "tag" {
                b.gated_eq(enable, value.clone(), immediate(9))?;
            } else if row == 0 && lane.name == "errorCode" {
                continue;
            } else {
                b.gated_eq(enable, value.clone(), immediate(0))?;
            }
        }
    }

    let err_code = field(controls, &input.pre.tables[5][0], "errorCode");

    // Complete Post-state Root constraints: controlLength = 0, outcome = Rejected(err_code),
    // inactive outcomeValue.{space,index} = 0, and exact preservation of all other Root
    // lanes (including depth, environment, stackLimit, and all 16 LE fuel bytes).
    for (lane, (pre, post)) in schema
        .state_root
        .iter()
        .zip(input.pre.root.iter().zip(&input.post.root))
    {
        let expected = match lane.name.as_str() {
            "controlLength" => immediate(0),
            "outcomeTag" => immediate(2),
            "outcomeValue.space" => immediate(0),
            "outcomeValue.index" => immediate(0),
            "outcomeError" => err_code.clone(),
            _ => pre.clone(),
        };
        b.gated_eq(enable, post.clone(), expected)?;
    }

    // Complete preservation of all five state arenas: Data, DataFields, EnvCells, ValueSlots, FrameSlots.
    for table in 0..5 {
        for (pre, post) in input.pre.tables[table]
            .iter()
            .flatten()
            .zip(input.post.tables[table].iter().flatten())
        {
            b.gated_eq(enable, post.clone(), pre.clone())?;
        }
    }

    // Complete release zeroing of post ControlRows[0..C] as controlLength becomes 0.
    for row in &input.post.tables[5] {
        for post in row {
            b.gated_eq(enable, post.clone(), immediate(0))?;
        }
    }

    Ok(ErrorPublicationStepBlock {
        instructions: b.code,
        preserved: input.clone(),
    })
}
