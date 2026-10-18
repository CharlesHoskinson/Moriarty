//! Source-static native-call constraints. Ordinary checking is not a proof.
pub(crate) mod arithmetic;
pub(crate) mod compiler;
mod event;
mod process;
use crate::{
    Error, Result, codec, layout,
    profile::{Deployment, StaticBindings},
};
use midnight_transient_crypto::proofs::ProofPreimage;
use midnight_zkir_v3::{Instruction, IrSource, ir::Operand};
use mori_core::{ExprId, Program};
pub use process::{ProcessedOps, process};
use serde::{Deserialize, Serialize};

pub const FORMAT: &str = "moriarty-native-call-relation/1";
fn refusal(code: &'static str) -> Error {
    Error::new("native-relation", code)
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Group {
    pub field_count: usize,
    pub origin: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub format: String,
    pub source_hash: String,
    pub program_hash: String,
    pub action: String,
    pub abi: layout::LedgerAbi,
    pub ir_hash: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub signed: Option<crate::signed::SignedManifest>,
    #[serde(default)]
    pub input_count: usize,
    pub groups: Vec<Group>,
    pub qualification: String,
    pub compiler_hash: String,
    pub open_predicates: Vec<String>,
}
#[derive(Debug, Clone, Serialize)]
pub struct CheckedReadback {
    pub scope: &'static str,
    pub processed_call_use: &'static str,
    pub ir_hash: String,
    pub activity: Vec<Option<usize>>,
    pub raw_op_count: usize,
    pub processed_op_count: usize,
    pub noop_count: usize,
    pub inactive_fields: usize,
    pub noop_cost_hex: String,
    pub processed_measured_gas_hex: String,
    pub processed_declared_gas_hex: String,
    pub parent_binding_commitment_hex: String,
    pub public_inputs: Vec<String>,
}
pub struct Artifact {
    pub ir: IrSource,
    pub manifest: Manifest,
}
impl Artifact {
    pub fn ir_bytes(&self) -> Result<Vec<u8>> {
        codec::encode(&self.ir)
    }
    pub fn manifest_bytes(&self) -> Result<Vec<u8>> {
        let b = if self.manifest.signed.is_some() {
            let mut value =
                serde_json::to_value(&self.manifest).map_err(|_| refusal("manifest-json"))?;
            value
                .as_object_mut()
                .ok_or(refusal("manifest-json"))?
                .remove("input_count");
            serde_json::to_vec_pretty(&value).map_err(|_| refusal("manifest-json"))?
        } else {
            serde_json::to_vec_pretty(&self.manifest).map_err(|_| refusal("manifest-json"))?
        };
        if b.len() > codec::OUTPUT_BYTES {
            return Err(refusal("artifact-size"));
        }
        Ok(b)
    }
}
pub fn compile(source: &str, p: &Program, d: &Deployment, action: &str) -> Result<Artifact> {
    if source.len() > codec::INPUT_BYTES
        || codec::hash(source.as_bytes()) != d.source_hash
        || mori_core::parse(source).map_err(|_| refusal("source-parse"))? != *p
    {
        return Err(refusal("source-identity"));
    }
    layout::admit_source(p, d)?;
    compile_bindings(source, p, &StaticBindings::legacy(d), action)
}
pub(crate) fn compile_bindings(
    source: &str,
    p: &Program,
    d: &StaticBindings,
    action: &str,
) -> Result<Artifact> {
    let validated = layout::validate_static(source, p, d)?;
    let abi = validated.abi().clone();
    let (ir, groups, input_count) = compiler::compile(&validated, action)?;
    let ir_hash = codec::hash(&codec::encode(&ir)?);
    let manifest = Manifest {
        format: FORMAT.into(),
        source_hash: d.source_hash.clone(),
        program_hash: abi.program_hash.clone(),
        action: action.into(),
        abi,
        ir_hash,
        input_count,
        signed: None,
        groups,
        qualification: "ordinary-ir-unproven".into(),
        compiler_hash: codec::hash(
            concat!(
                include_str!("mod.rs"),
                include_str!("compiler.rs"),
                include_str!("arithmetic.rs"),
                include_str!("event.rs"),
                include_str!("process.rs")
            )
            .as_bytes(),
        ),
        open_predicates: [
            "authenticated predecessor and deployed key",
            "payer/account/funding ownership",
            "global prefix balances and custody correspondence",
            "economic intention authority",
            "final intent/history/PCD",
            "cryptographic proof and ledger settlement",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
    };
    let a = Artifact { ir, manifest };
    if a.ir_bytes()?
        .len()
        .checked_add(a.manifest_bytes()?.len())
        .is_none_or(|n| n > codec::OUTPUT_BYTES)
    {
        return Err(refusal("artifact-size"));
    }
    Ok(a)
}
pub fn check(a: &Artifact, p: &ProofPreimage) -> Result<Vec<Option<usize>>> {
    let width = a
        .manifest
        .signed
        .as_ref()
        .map_or(a.manifest.input_count, |m| m.flattened_field_count);
    if !a.ir.do_communications_commitment
        || !a.ir.outputs.is_empty()
        || a.ir
            .instructions
            .iter()
            .filter(|i| matches!(i, midnight_zkir_v3::Instruction::Output { .. }))
            .count()
            != 1
        || a.ir
            .instructions
            .iter()
            .any(|i| matches!(i, midnight_zkir_v3::Instruction::PrivateInput { .. }))
        || p.inputs.len() != width
        || !p.private_transcript.is_empty()
        || p.key_location.0.as_ref() != a.manifest.action
        || p.public_transcript_inputs.len() > codec::OUTPUT_BYTES / 32
        || p.public_transcript_outputs.len() > codec::OUTPUT_BYTES / 32
        || codec::hash(&a.ir_bytes()?) != a.manifest.ir_hash
    {
        return Err(refusal("preimage-domain"));
    }
    let activity = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| p.check(&a.ir)))
        .map_err(|_| refusal("sdk-check-panic"))?
        .map_err(|_| refusal("unsatisfied"))?;
    if activity.len() != a.manifest.groups.len()
        || activity
            .iter()
            .zip(&a.manifest.groups)
            .any(|(s, g)| s.is_some_and(|n| n != g.field_count))
    {
        return Err(refusal("activity-layout"));
    }
    Ok(activity)
}

/// Compare caller-supplied bytes against a fresh independently supplied source
/// and static deployment compile. No caller manifest chooses its own relation.
pub fn admit(
    source: &str,
    p: &Program,
    d: &Deployment,
    action: &str,
    ir: &[u8],
    manifest: &[u8],
) -> Result<Artifact> {
    if ir
        .len()
        .checked_add(manifest.len())
        .is_none_or(|n| n > codec::OUTPUT_BYTES)
    {
        return Err(refusal("artifact-size"));
    }
    let a = compile(source, p, d, action)?;
    if a.ir_bytes()? != ir || a.manifest_bytes()? != manifest {
        return Err(refusal("artifact-identity"));
    }
    Ok(a)
}

pub(crate) const SOURCE_STATIC_FORMAT: &str = "moriarty-mapped-role-source-static-relation/2";
pub(crate) const SOURCE_STATIC_QUALIFICATION: &str = "source-static-mapped-debits-unauthenticated";
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct DynamicField {
    pub(crate) group: usize,
    pub(crate) offset: usize,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct StatementEmission {
    pub(crate) statement: usize,
    pub(crate) expression: ExprId,
    pub(crate) instruction_start: usize,
    pub(crate) instruction_end: usize,
    pub(crate) group_start: usize,
    pub(crate) group_end: usize,
    pub(crate) dynamic_fields: Vec<DynamicField>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct SourceStaticManifest {
    pub(crate) format: String,
    pub(crate) source_hash: String,
    pub(crate) program_hash: String,
    pub(crate) action: String,
    pub(crate) abi: layout::LedgerAbi,
    pub(crate) ir_hash: String,
    pub(crate) input_count: usize,
    pub(crate) groups: Vec<Group>,
    pub(crate) qualification: String,
    pub(crate) compiler_hash: String,
    pub(crate) open_predicates: Vec<String>,
    pub(crate) debit_sources: Vec<crate::profile::RoleBinding>,
    pub(crate) statement_emissions: Vec<StatementEmission>,
}
/// Export bytes only; cannot construct a native relation artifact or key capability.
/// ```compile_fail
/// let artifact = mori_ledger::relation::SourceStaticArtifact {
///     ir: todo!(), manifest: todo!(),
/// };
/// let _: mori_ledger::relation::Artifact = artifact.into();
/// ```
pub(crate) struct SourceStaticArtifact {
    ir: IrSource,
    manifest: SourceStaticManifest,
}
impl SourceStaticArtifact {
    pub(crate) fn manifest(&self) -> &SourceStaticManifest {
        &self.manifest
    }
    pub(crate) fn ir_bytes(&self) -> Result<Vec<u8>> {
        codec::encode(&self.ir)
    }
    pub(crate) fn manifest_bytes(&self) -> Result<Vec<u8>> {
        crate::call_artifacts::json_bytes(&self.manifest, codec::OUTPUT_BYTES as u64)
    }
}
fn statement_root(s: &mori_core::Statement) -> Result<ExprId> {
    use mori_core::Statement as S;
    Ok(match s {
        S::Await { .. } => return Err(layout::core1_refusal()),
        S::Let { value, .. } | S::Next { value, .. } | S::Emit { value, .. } => *value,
        S::Requires { predicate } | S::Ensures { predicate } => *predicate,
        S::Transfer { amount, .. }
        | S::Fee { amount, .. }
        | S::Liability { amount, .. }
        | S::Discharge { amount, .. } => *amount,
    })
}
/// Emission bookkeeping closure only. Entries partition the body between the
/// ABI-root prologue and the final Output; group k is the k-th Impact; every
/// Variable Impact input is a listed replacement. A listed replacement may
/// still be Immediate. Activity, values, kind, payer and debits are not bound.
fn validate_statement_emissions(
    statements: &[mori_core::Statement],
    instructions: &[Instruction],
    groups: &[Group],
    emissions: &[StatementEmission],
) -> Result<()> {
    let refuse = || refusal("source-static-emission");
    let body_end = match instructions.split_last() {
        Some((Instruction::Output { .. }, body)) => body.len(),
        _ => return Err(refuse()),
    };
    let impacts = instructions
        .iter()
        .enumerate()
        .filter_map(|(i, instruction)| match instruction {
            Instruction::Impact { inputs, .. } => Some((i, inputs)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let prologue = groups.iter().take_while(|g| g.origin == "abi-root").count();
    if emissions.len() != statements.len()
        || impacts.len() != groups.len()
        || impacts
            .iter()
            .zip(groups)
            .any(|((_, inputs), g)| inputs.len() != g.field_count)
        || groups[prologue..].iter().any(|g| g.origin == "abi-root")
    {
        return Err(refuse());
    }
    let groups_before = |instruction: usize| impacts.partition_point(|(i, _)| *i < instruction);
    let mut instruction = impacts[..prologue].last().map_or(0, |(i, _)| i + 1);
    let mut group = prologue;
    let mut listed: Vec<(usize, usize)> = Vec::new();
    for (index, (e, s)) in emissions.iter().zip(statements).enumerate() {
        if e.statement != index
            || e.expression != statement_root(s)?
            || e.instruction_start != instruction
            || e.group_start != group
            || e.instruction_end < e.instruction_start
            || e.instruction_end > body_end
            || groups_before(e.instruction_start) != e.group_start
            || groups_before(e.instruction_end) != e.group_end
        {
            return Err(refuse());
        }
        for f in &e.dynamic_fields {
            if !(e.group_start..e.group_end).contains(&f.group)
                || f.offset >= groups[f.group].field_count
                || listed
                    .last()
                    .is_some_and(|last| *last >= (f.group, f.offset))
            {
                return Err(refuse());
            }
            listed.push((f.group, f.offset));
        }
        instruction = e.instruction_end;
        group = e.group_end;
    }
    if instruction != body_end
        || group != groups.len()
        || impacts.iter().enumerate().any(|(k, (_, inputs))| {
            inputs.iter().enumerate().any(|(j, v)| {
                matches!(v, Operand::Variable(_)) && listed.binary_search(&(k, j)).is_err()
            })
        })
    {
        return Err(refuse());
    }
    Ok(())
}
pub(crate) fn compile_source_static_roles(
    validated: &layout::ValidatedSourceStaticBindings<'_>,
    action: &str,
) -> Result<SourceStaticArtifact> {
    layout::require_core1(validated.program())?;
    let (ir, groups, input_count, statement_emissions) =
        compiler::compile_source_static_roles(validated, action)?;
    let p = validated.program();
    let bindings = validated.bindings();
    let source_action = p
        .actions
        .iter()
        .find(|a| a.name == action)
        .ok_or(refusal("action"))?;
    validate_statement_emissions(
        &source_action.statements,
        ir.instructions.as_slice(),
        &groups,
        &statement_emissions,
    )?;
    let names = source_action
        .statements
        .iter()
        .filter_map(|s| match s {
            mori_core::Statement::Transfer { from, .. }
            | mori_core::Statement::Fee { from, .. } => Some(from),
            _ => None,
        })
        .collect::<std::collections::BTreeSet<_>>();
    let debit_sources = names
        .into_iter()
        .map(|name| {
            bindings
                .roles
                .iter()
                .find(|r| &r.role == name)
                .cloned()
                .ok_or(refusal("source-debit-role"))
        })
        .collect::<Result<Vec<_>>>()?;
    let artifact = SourceStaticArtifact {
        manifest: SourceStaticManifest {
            format: SOURCE_STATIC_FORMAT.into(),
            source_hash: bindings.source_hash.clone(),
            program_hash: bindings.program_hash.clone(),
            action: action.into(),
            abi: validated.abi().clone(),
            ir_hash: codec::hash(&codec::encode(&ir)?),
            input_count,
            groups,
            qualification: SOURCE_STATIC_QUALIFICATION.into(),
            compiler_hash: crate::proving::source_closure(),
            open_predicates: [
                "debit-source custody and funding ownership",
                "role and beneficiary authorization",
                "authenticated predecessor and deployment",
                "economic intention authority",
                "final intent/history/PCD",
                "native proof and ledger settlement",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            debit_sources,
            statement_emissions,
        },
        ir,
    };
    if artifact
        .ir_bytes()?
        .len()
        .checked_add(artifact.manifest_bytes()?.len())
        .is_none_or(|n| n > codec::OUTPUT_BYTES)
    {
        return Err(refusal("artifact-size"));
    }
    Ok(artifact)
}

#[cfg(test)]
mod emission_tests {
    use super::{arithmetic::imm, *};
    use midnight_zkir_v3::Identifier;
    use mori_core::Statement;

    struct Specimen {
        statements: Vec<Statement>,
        instructions: Vec<Instruction>,
        groups: Vec<Group>,
        emissions: Vec<StatementEmission>,
    }
    impl Specimen {
        fn validate(&self) -> Result<()> {
            validate_statement_emissions(
                &self.statements,
                &self.instructions,
                &self.groups,
                &self.emissions,
            )
        }
    }
    fn var(name: &str) -> Operand {
        Operand::Variable(Identifier(name.into()))
    }
    fn impact(inputs: Vec<Operand>) -> Instruction {
        Instruction::Impact {
            guard: imm(1),
            inputs,
        }
    }
    fn group(field_count: usize, origin: &str) -> Group {
        Group {
            field_count,
            origin: origin.into(),
        }
    }
    fn entry(
        statement: usize,
        expression: ExprId,
        instructions: (usize, usize),
        groups: (usize, usize),
        dynamic: &[(usize, usize)],
    ) -> StatementEmission {
        StatementEmission {
            statement,
            expression,
            instruction_start: instructions.0,
            instruction_end: instructions.1,
            group_start: groups.0,
            group_end: groups.1,
            dynamic_fields: dynamic
                .iter()
                .map(|&(group, offset)| DynamicField { group, offset })
                .collect(),
        }
    }
    /// Hand-built layout, not compiler output: one ABI-root group, a transfer
    /// with one Variable replacement and one constant group, a groupless Let.
    fn specimen() -> Specimen {
        Specimen {
            statements: vec![
                Statement::Transfer {
                    asset: "PAY".into(),
                    from: "alice".into(),
                    to: "treasury".into(),
                    amount: 0,
                },
                Statement::Let {
                    name: "local".into(),
                    value: 2,
                },
            ],
            instructions: vec![
                Instruction::ConstrainBits {
                    val: var("%input.0"),
                    bits: 128,
                },
                impact(vec![imm(1), imm(2)]),
                Instruction::Assert { cond: imm(1) },
                impact(vec![imm(3), var("%native.0")]),
                impact(vec![imm(4), imm(5), imm(6)]),
                Instruction::Assert { cond: imm(1) },
                Instruction::Output { vals: vec![] },
            ],
            groups: vec![
                group(2, "abi-root"),
                group(2, "source-claim"),
                group(3, "source-claim"),
            ],
            emissions: vec![
                entry(0, 0, (2, 5), (1, 3), &[(1, 1)]),
                entry(1, 2, (5, 6), (3, 3), &[]),
            ],
        }
    }
    fn refuses(label: &str, mutate: impl FnOnce(&mut Specimen)) {
        let mut s = specimen();
        mutate(&mut s);
        let error = s.validate().err().unwrap_or_else(|| panic!("{label}"));
        assert_eq!(
            (error.layer, error.code),
            ("native-relation", "source-static-emission"),
            "{label}"
        );
    }

    #[test]
    fn exact_emission_partition_is_admitted() {
        specimen().validate().unwrap();
    }

    #[test]
    fn constant_replacement_may_be_listed() {
        // A literal `next s = 5` replaces a template slot with an Immediate.
        let mut s = specimen();
        s.emissions[0].dynamic_fields.push(DynamicField {
            group: 2,
            offset: 0,
        });
        s.validate().unwrap();
    }

    #[test]
    fn statement_kind_and_payer_are_not_bound_by_emission_check() {
        let mut s = specimen();
        s.statements[0] = Statement::Fee {
            asset: "PAY".into(),
            from: "bob".into(),
            to: "treasury".into(),
            amount: 0,
        };
        s.validate().unwrap();
    }

    #[test]
    fn statementless_action_has_only_prologue_groups() {
        let mut s = Specimen {
            statements: vec![],
            instructions: vec![impact(vec![imm(1)]), Instruction::Output { vals: vec![] }],
            groups: vec![group(1, "abi-root")],
            emissions: vec![],
        };
        s.validate().unwrap();
        s.groups[0].origin = "source-claim".into();
        let error = s.validate().err().unwrap();
        assert_eq!(error.code, "source-static-emission");
    }

    #[test]
    fn assembly_violations_refuse() {
        refuses("missing entry", |s| {
            s.emissions.pop();
        });
        refuses("extra entry", |s| {
            let extra = s.emissions[1].clone();
            s.emissions.push(extra);
        });
        refuses("entries reordered", |s| s.emissions.swap(0, 1));
        refuses("statement index", |s| s.emissions[1].statement = 0);
        refuses("root expression", |s| s.emissions[0].expression = 1);
        refuses("prologue attributed to statement", |s| {
            s.emissions[0].instruction_start = 1;
            s.emissions[0].group_start = 0;
        });
        refuses("statement instruction left in prologue", |s| {
            s.emissions[0].instruction_start = 3;
        });
        refuses("instruction gap", |s| s.emissions[1].instruction_start = 6);
        refuses("group range detached from Impact", |s| {
            s.emissions[0].group_end = 2;
            s.emissions[1].group_start = 2;
        });
        refuses("reversed range", |s| s.emissions[1].instruction_end = 4);
        refuses("entry stops before Output", |s| {
            s.emissions[1].instruction_end = 5;
        });
        refuses("entry covers Output", |s| {
            s.emissions[1].instruction_end = 7
        });
        refuses("missing Output", |s| {
            s.instructions.pop();
        });
        refuses("Impact without group", |s| {
            s.instructions.insert(5, impact(vec![imm(7)]));
        });
        refuses("width differs from Impact", |s| s.groups[2].field_count = 4);
        refuses("abi-root origin in body", |s| {
            s.groups[2].origin = "abi-root".into()
        });
        refuses("field in another entry", |s| {
            s.emissions[0].dynamic_fields.clear();
            s.emissions[1].dynamic_fields.push(DynamicField {
                group: 1,
                offset: 1,
            });
        });
        refuses("offset beyond width", |s| {
            s.emissions[0].dynamic_fields[0].offset = 2;
        });
        refuses("duplicate field", |s| {
            s.emissions[0].dynamic_fields.push(DynamicField {
                group: 1,
                offset: 1,
            });
        });
        refuses("descending fields", |s| {
            s.emissions[0].dynamic_fields.insert(
                0,
                DynamicField {
                    group: 2,
                    offset: 0,
                },
            );
        });
        refuses("Variable replacement omitted", |s| {
            s.emissions[0].dynamic_fields.clear();
        });
        refuses("Variable in unlisted slot", |s| {
            s.instructions[4] = impact(vec![imm(4), var("%native.1"), imm(6)]);
        });
    }
}
