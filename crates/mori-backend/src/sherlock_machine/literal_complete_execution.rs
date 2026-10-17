//! Unregistered two-step `Init -> PendingConstantReturn -> Done` literal program
//! instruction composer over [`emit_literal_step`] (`A102`, line 102, wire
//! `rule_tag = 3`, `charge = 1`) and [`emit_return_publication_step`] (`A80`,
//! line 80, wire `rule_tag = 0`, `charge = 0`).
//!
//! Grounds `Init` equations in `mori_core::sherlock::Machine::new` and L4:
//! entry expression `entryExprId == 0`, `ControlRows[0] = Eval(entryExprId)`,
//! empty environment (`environment.present == 0`, `environment.index == 0`,
//! `activeEnvCells == 0`), empty frame stack (`depth == 0`), empty runtime
//! arenas (`activeData == 0`, `activeDataFields == 0`, `activeValueSlots == 0`),
//! zeroed `Data`/`DataFields`/`EnvCells`/`ValueSlots`/`FrameSlots` and non-entry
//! `ControlRows` lanes, `Outcome::Pending` (`outcomeTag == 0` with zero payload),
//! and 16-byte little-endian `remainingFuel == initialFuel`. Shares one immutable
//! `program` [`LiteralImage`] and aliases one exact `intermediate` [`LiteralImage`]
//! as `A102.post` and `A80.pre` with unconditional step enables (`immediate(1)`).
//!
//! # External Premises & Unimplemented Boundary Clauses
//! - **`projectA` Canonical-Boundary Projection**: Both `A102` and `A80` execute
//!   in Step mode. `done.root.outcomeValue` carries the literal's `Constant`-space
//!   `ValueRef` (`space == 0`, `index == constant_index`) with `activeData == 0`.
//!   Unfolding reachable `Constant` nodes into `Data`/`DataFields` (`space == 1`)
//!   at a canonical `StateBlock` boundary belongs to the separate, unimplemented
//!   `projectA` operation and is not invented here.
//! - **Whole-Program `F4` / `StepInv` / `OBL.G4` / Dispatch**: Full `prepareA(source)`
//!   equality on unused program tables and non-entry expressions (`F4`), general
//!   `StepInv` (`O1`..`O4`), `OBL.G4` byte serialization, line-84 zero-fuel
//!   startup exhaustion, and 39-row `emitStepA` dispatch remain caller obligations.

use super::literal_host::{RULE_CHARGE, RULE_START_LINE, RULE_TAG};
use super::return_publication_step::{
    emit_return_publication_step, is_reserved_return_publication_variable,
    ReturnPublicationStepBlock, ReturnPublicationStepInputs, RETURN_PUBLICATION_RULE_CHARGE,
    RETURN_PUBLICATION_RULE_START_LINE, RETURN_PUBLICATION_RULE_TAG,
};
use super::*;

pub const COMPLETE_LITERAL_EXECUTION_SCOPE_STEP1_SUFFIX: &str = "literal_step";
pub const COMPLETE_LITERAL_EXECUTION_SCOPE_STEP2_SUFFIX: &str = "return_publication_step";
pub const COMPLETE_LITERAL_EXECUTION_STEP1_RULE_START_LINE: u128 = RULE_START_LINE;
pub const COMPLETE_LITERAL_EXECUTION_STEP1_RULE_TAG: u128 = RULE_TAG;
pub const COMPLETE_LITERAL_EXECUTION_STEP1_RULE_CHARGE: u128 = RULE_CHARGE;
pub const COMPLETE_LITERAL_EXECUTION_STEP2_RULE_START_LINE: u128 =
    RETURN_PUBLICATION_RULE_START_LINE;
pub const COMPLETE_LITERAL_EXECUTION_STEP2_RULE_TAG: u128 = RETURN_PUBLICATION_RULE_TAG;
pub const COMPLETE_LITERAL_EXECUTION_STEP2_RULE_CHARGE: u128 = RETURN_PUBLICATION_RULE_CHARGE;
pub const COMPLETE_LITERAL_EXECUTION_TOTAL_CHARGE: u128 =
    RULE_CHARGE + RETURN_PUBLICATION_RULE_CHARGE;

#[derive(Clone, Debug, PartialEq)]
pub struct LiteralCompleteExecutionInputs {
    /// One complete immutable ProgramBlock shared by both step emitters.
    pub program: LiteralImage,
    /// Initial `Machine::new` boundary pre-state (`A102.pre`).
    pub init: LiteralImage,
    /// Exact aliased Step-mode intermediate state (`A102.post == A80.pre`).
    pub intermediate: LiteralImage,
    /// Terminal Step-mode post-publication state (`A80.post`, `Outcome::Done`).
    pub done: LiteralImage,
    /// Step-1 envelope operands constrained to `(rule_tag = 3, charge = 1)`.
    pub literal_envelope: LiteralEnvelope,
    /// Step-2 envelope operands constrained to `(rule_tag = 0, charge = 0)`.
    pub return_envelope: LiteralEnvelope,
}

impl LiteralCompleteExecutionInputs {
    /// Construct complete-execution inputs with fixed `(3, 1)` and `(0, 0)` envelopes.
    pub fn new(
        program: LiteralImage,
        init: LiteralImage,
        intermediate: LiteralImage,
        done: LiteralImage,
    ) -> Self {
        Self {
            program,
            init,
            intermediate,
            done,
            literal_envelope: LiteralEnvelope {
                rule_tag: immediate(RULE_TAG),
                charge: immediate(RULE_CHARGE),
            },
            return_envelope: LiteralEnvelope {
                rule_tag: immediate(RETURN_PUBLICATION_RULE_TAG),
                charge: immediate(RETURN_PUBLICATION_RULE_CHARGE),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LiteralCompleteExecutionBlock {
    /// Complete instruction stream (`init_instructions`, then `A102`, then `A80`).
    pub instructions: Vec<Instruction>,
    /// Composer-emitted `Machine::new` `Init`, fixed-envelope, and fuel/work instructions.
    pub init_instructions: Vec<Instruction>,
    /// Exact child instruction vector emitted by `emit_literal_step` (`A102`).
    pub literal_step_instructions: Vec<Instruction>,
    /// Exact child instruction vector emitted by `emit_return_publication_step` (`A80`).
    pub return_publication_step_instructions: Vec<Instruction>,
    /// Selected entry expression row operands from `emit_literal_step`.
    pub selected_expression: Vec<Operand>,
    /// Exact `LiteralStepInputs` preserved by `emit_literal_step`.
    pub literal_step_preserved: LiteralStepInputs,
    /// Exact `ReturnPublicationStepInputs` preserved by `emit_return_publication_step`.
    pub return_publication_step_preserved: ReturnPublicationStepInputs,
    /// Total charged work operand (`literal_envelope.charge + return_envelope.charge == 1`).
    pub charged_work: Operand,
    /// Operand aliases only; this is not a transition/acceptance result.
    pub preserved: LiteralCompleteExecutionInputs,
}

impl LiteralCompleteExecutionBlock {
    /// Returns both actual child instruction vectors in `A102`-then-`A80` order.
    pub fn step_instruction_vectors(&self) -> [&[Instruction]; 2] {
        [
            &self.literal_step_instructions,
            &self.return_publication_step_instructions,
        ]
    }
}

type Result<T> = std::result::Result<T, LiteralConstructionError>;

/// Check whether `name` collides with any temporary namespace reserved by
/// [`emit_literal_complete_execution`] or its `A102` and `A80` callees under `scope`.
pub fn is_reserved_literal_complete_execution_variable(
    name: &str,
    scope: &str,
    k: usize,
    kf: usize,
) -> bool {
    let local = format!("%{scope}.local.temp.");
    let literal_scope = format!("{scope}.{COMPLETE_LITERAL_EXECUTION_SCOPE_STEP1_SUFFIX}");
    let literal_local = format!("%{literal_scope}.local.temp.");
    let literal_fetch = format!("%{literal_scope}.expression_fetch.temp.");
    let ui_valid_scope = format!("{literal_scope}.ui_valid");
    let return_scope = format!("{scope}.{COMPLETE_LITERAL_EXECUTION_SCOPE_STEP2_SUFFIX}");
    name.starts_with(&local)
        || name.starts_with(&literal_local)
        || name.starts_with(&literal_fetch)
        || is_reserved_ui_valid_variable(name, &ui_valid_scope, k, kf)
        || is_reserved_return_publication_variable(name, &return_scope)
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
    image.root.iter().chain(image.tables.iter().flatten().flatten())
}

fn field(lanes: &[LiteralLane], row: &[Operand], name: &str) -> Operand {
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

    fn eq(&mut self, a: Operand, b: Operand) {
        self.code.push(Instruction::ConstrainEq { a, b });
    }

    fn bits(&mut self, val: Operand, bits: u32) {
        self.code.push(Instruction::ConstrainBits { val, bits });
    }

    fn add(&mut self, a: Operand, b: Operand) -> Result<Operand> {
        let output = self.fresh()?;
        self.code.push(Instruction::Add {
            a,
            b,
            output: output.clone(),
        });
        Ok(Operand::Variable(output))
    }
}

/// Compose the complete two-step `Init -> PendingConstantReturn -> Done` execution
/// path for a literal program by invoking both [`emit_literal_step`] (`A102`) and
/// [`emit_return_publication_step`] (`A80`) with shared `program`, exact aliased
/// `intermediate`, and unconditional step execution.
pub fn emit_literal_complete_execution(
    schema: &LiteralSchema,
    input: &LiteralCompleteExecutionInputs,
    scope: &str,
) -> Result<LiteralCompleteExecutionBlock> {
    if &literal_schema(schema.capacities())? != schema {
        return Err(LiteralConstructionError::SchemaMismatch);
    }
    check_image(
        &input.program,
        &schema.program_header,
        &schema.program_tables,
        "program",
    )?;
    check_image(&input.init, &schema.state_root, &schema.state_tables, "init")?;
    check_image(
        &input.intermediate,
        &schema.state_root,
        &schema.state_tables,
        "intermediate",
    )?;
    check_image(&input.done, &schema.state_root, &schema.state_tables, "done")?;
    if scope.is_empty() {
        return Err(ConstructionError::EmptyScope.into());
    }

    let [k_cap, kf_cap] = [
        schema.capacities().program[13],
        schema.capacities().program[14],
    ];
    for operand in [
        &input.literal_envelope.rule_tag,
        &input.literal_envelope.charge,
        &input.return_envelope.rule_tag,
        &input.return_envelope.charge,
    ]
    .into_iter()
    .chain(operands(&input.program))
    .chain(operands(&input.init))
    .chain(operands(&input.intermediate))
    .chain(operands(&input.done))
    {
        if let Operand::Variable(name) = operand {
            if !name.0.starts_with('%') {
                return Err(ConstructionError::UnprefixedVariable.into());
            }
            if is_reserved_literal_complete_execution_variable(&name.0, scope, k_cap, kf_cap) {
                return Err(ConstructionError::TemporaryNameCollision.into());
            }
        }
    }

    let literal_scope = format!("{scope}.{COMPLETE_LITERAL_EXECUTION_SCOPE_STEP1_SUFFIX}");
    let return_scope = format!("{scope}.{COMPLETE_LITERAL_EXECUTION_SCOPE_STEP2_SUFFIX}");
    let literal_inputs = LiteralStepInputs {
        enable: immediate(1),
        program: input.program.clone(),
        pre: input.init.clone(),
        post: input.intermediate.clone(),
        envelope: input.literal_envelope.clone(),
    };
    let return_inputs = ReturnPublicationStepInputs {
        enable: immediate(1),
        program: input.program.clone(),
        pre: input.intermediate.clone(),
        post: input.done.clone(),
        envelope: input.return_envelope.clone(),
    };

    let LiteralStepBlock {
        instructions: literal_step_instructions,
        selected_expression,
        preserved: literal_step_preserved,
    } = emit_literal_step(schema, &literal_inputs, &literal_scope)?;
    let ReturnPublicationStepBlock {
        instructions: return_publication_step_instructions,
        preserved: return_publication_step_preserved,
    } = emit_return_publication_step(schema, &return_inputs, &return_scope)?;

    let mut b = Emitter {
        scope: scope.into(),
        serial: 0,
        code: Vec::new(),
    };

    // Fixed step envelopes (row 3 / charge 1, then row 0 / charge 0) and total charged work.
    b.eq(input.literal_envelope.rule_tag.clone(), immediate(RULE_TAG));
    b.eq(input.literal_envelope.charge.clone(), immediate(RULE_CHARGE));
    b.eq(
        input.return_envelope.rule_tag.clone(),
        immediate(RETURN_PUBLICATION_RULE_TAG),
    );
    b.eq(
        input.return_envelope.charge.clone(),
        immediate(RETURN_PUBLICATION_RULE_CHARGE),
    );
    let charged_work = b.add(
        input.literal_envelope.charge.clone(),
        input.return_envelope.charge.clone(),
    )?;
    b.bits(charged_work.clone(), 9);
    b.eq(
        charged_work.clone(),
        immediate(COMPLETE_LITERAL_EXECUTION_TOTAL_CHARGE),
    );

    // Machine::new entry expression binding: program.entryExprId == 0 == init.ControlRows[0].exprId.
    let entry_expr_id = field(&schema.program_header, &input.program.root, "entryExprId");
    let init_expr_id = field(
        &schema.state_tables[5].lanes,
        &input.init.tables[5][0],
        "exprId",
    );
    b.eq(entry_expr_id.clone(), immediate(0));
    b.eq(init_expr_id.clone(), entry_expr_id);
    b.eq(init_expr_id, immediate(0));

    // Machine::new StateRoot equations: empty arenas, empty stack, empty env, Pending outcome, 1 control row.
    for name in [
        "activeData",
        "activeDataFields",
        "activeEnvCells",
        "activeValueSlots",
        "depth",
        "environment.present",
        "environment.index",
        "outcomeTag",
        "outcomeValue.space",
        "outcomeValue.index",
        "outcomeError",
    ] {
        b.eq(field(&schema.state_root, &input.init.root, name), immediate(0));
    }
    b.eq(
        field(&schema.state_root, &input.init.root, "controlLength"),
        immediate(1),
    );

    // Machine::new 16-byte LE fuel initialization (remainingFuel == initialFuel) and boundary preservation.
    for byte in 0..16 {
        let rem_name = format!("remainingFuel.{byte}");
        let ini_name = format!("initialFuel.{byte}");
        let init_rem = field(&schema.state_root, &input.init.root, &rem_name);
        let init_ini = field(&schema.state_root, &input.init.root, &ini_name);
        let mid_rem = field(&schema.state_root, &input.intermediate.root, &rem_name);
        let mid_ini = field(&schema.state_root, &input.intermediate.root, &ini_name);
        let done_rem = field(&schema.state_root, &input.done.root, &rem_name);
        let done_ini = field(&schema.state_root, &input.done.root, &ini_name);
        b.eq(init_rem, init_ini.clone());
        b.eq(mid_ini.clone(), init_ini);
        b.eq(done_ini, mid_ini);
        b.eq(done_rem, mid_rem);
    }

    // Machine::new & L4 padding: all 5 state arenas and all ControlRows cells in Init are 0.
    for table in 0..6 {
        for cell in input.init.tables[table].iter().flatten() {
            b.eq(cell.clone(), immediate(0));
        }
    }

    let init_instructions = b.code;
    let mut instructions = Vec::with_capacity(
        init_instructions.len()
            + literal_step_instructions.len()
            + return_publication_step_instructions.len(),
    );
    instructions.extend(init_instructions.iter().cloned());
    instructions.extend(literal_step_instructions.iter().cloned());
    instructions.extend(return_publication_step_instructions.iter().cloned());

    Ok(LiteralCompleteExecutionBlock {
        instructions,
        init_instructions,
        literal_step_instructions,
        return_publication_step_instructions,
        selected_expression,
        literal_step_preserved,
        return_publication_step_preserved,
        charged_work,
        preserved: input.clone(),
    })
}
