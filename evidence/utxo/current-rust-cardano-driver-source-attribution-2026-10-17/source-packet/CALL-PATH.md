# Current driver source path

Source-only navigation at commit `00ffdca38a30ea98c91a6731e2d6bdb373d16f0b`. Unreviewed and unpublished; no code compilation,simulation,synthesis,proof,verification,export generation or ledger action was run. Native authority0 and all product obligations remain open. This is a separate source packet; the original61-claim parent and W05 body-absence observation remain frozen.

## DR03 CALL_EDGE

Private CLI run prove branch parses flags,reads source/request,compiles request.selector,parses setup,compares descriptor,loads declared proving key,constructs OsRng,then calls prove::prove. This inspection invoked none of these operations and read no key material.

`DRIVER-main.rs:181–194`, SHA-256 `9c796a758a17b0f2fe955c571188afa5bd6407fc391541b7145c97315b0f1b4c`:

```text
        "prove" => {
            let f = flags(rest, &["source", "request", "setup-record", "proving-key", "out-proof"], &[])?;
            let out_proof = path(&f, "out-proof")?;
            refuse_existing(&[out_proof])?;
            let source = input::read_source(path(&f, "source")?)?;
            let request = input::parse_request(&json_bytes(path(&f, "request")?, "request")?)?;
            let selected = compile_selected(&source, &request.selector)?;
            let setup = export::parse_setup_record(&json_bytes(path(&f, "setup-record")?, "setup record")?)?;
            relation::require_same(&setup.record.relation, selected.descriptor())?;
            let pk_bytes = input::read_bounded(path(&f, "proving-key")?, input::MAX_PROVING_KEY_BYTES, "proving key")?;
            let pk = prove::load_proving_key(&pk_bytes, &setup)?;
            drop(pk_bytes);
            let mut rng = rand_core::OsRng;
            let out = prove::prove(&selected, &setup, &pk, &request, &mut rng)?;
```

## DR04 SOURCE_DECLARATION

check_selector compares Core1 moriarty/1,moriarty-core/1,adapter,NotBound,GrossDebitAndFees,TouchedOnly before compile_selected constructs explicit options and calls compile_source::<Fr>(...,SettlementHashV2). Nonmatching selector values return DriverError::Unsupported in this source loop.

`DRIVER-relation.rs:20–49`, SHA-256 `e18ae46c5ec0650f0c5dbc3c8b39262eb432c7d2e3be7b3b14cc01fe285493d5`:

```text
pub const LANGUAGE: &str = "moriarty/1";
pub const SEMANTICS: &str = "moriarty-core/1";
pub const CAP: &str = "NotBound";
pub const ACCOUNTING: &str = "GrossDebitAndFees";
pub const FRAME: &str = "TouchedOnly";
/// SettlementHashV2 public inputs: the two 128-bit digest halves.
pub const PUBLIC_INPUTS: u8 = 2;
/// arkworks counts the constant-one instance variable: 1 + 2.
pub const INSTANCE_VARIABLES: u64 = 3;

/// Refuses every selector value outside the one supported profile. Runs before compilation.
pub fn check_selector(s: &RelationSelector) -> Result<(), DriverError> {
    let fixed = [
        ("format", s.format.as_str(), SELECTOR_FORMAT),
        ("language", s.language.as_str(), LANGUAGE),
        ("semantics", s.semantics.as_str(), SEMANTICS),
        ("adapter", s.adapter.as_str(), crate::ADAPTER),
        ("cap", s.cap.as_str(), CAP),
        ("accounting", s.accounting.as_str(), ACCOUNTING),
        ("frame", s.frame.as_str(), FRAME),
    ];
    for (what, got, want) in fixed {
        if got != want {
            return Err(DriverError::Unsupported(format!("selector {what} {got:?}; this driver supports only {want:?}")));
        }
    }
    if s.action.is_empty() || s.action.len() > 1024 {
        return Err(DriverError::Input("selector action must contain 1..1024 bytes".into()));
    }
    Ok(())
```

`DRIVER-relation.rs:258–269`, SHA-256 `e18ae46c5ec0650f0c5dbc3c8b39262eb432c7d2e3be7b3b14cc01fe285493d5`:

```text
pub fn compile_selected(source: &str, selector: &RelationSelector) -> Result<SelectedRelation, DriverError> {
    check_selector(selector)?;
    let options = CompileOptions {
        action: selector.action.clone(),
        cap: CapBinding::NotBound,
        frame: FrameCells::TouchedOnly,
        accounting: AccountingBinding::GrossDebitAndFees,
        compiler: CompilerBuild::embedded().map_err(DriverError::Compile)?,
    };
    let relation =
        mori_r1cs::compile_source::<Fr>(source, options, StatementAdapter::SettlementHashV2).map_err(DriverError::Compile)?;
    let descriptor = RelationDescriptor::of(&relation)?;
```

`DRIVER-lib.rs:54–55`, SHA-256 `1b13cdc57fa21ab78a5bd6c33ba64d4628d8ed388f4a7121b40b18c5cae4b830`:

```text
/// The only statement adapter this driver drives.
pub const ADAPTER: &str = "SettlementHashV2";
```

## DR05 CALL_EDGE

Frontend calls Core language,then Core1 parse/admit_checked or Stage2 load/new. Compiler compile_source sends Core1 to compile and returns StagedNotLowered(lowering_plan()) for the Staged2 variant. This is a separate source branch,not executable staged lowering or an observed refusal.

`R1CS-frontend.rs:34–50`, SHA-256 `039f0867c3efb717737909395228846aa1eef47346b08ac0671f800fed3c1bf9`:

```text
/// Route by `mori_core::language`, then admit with the profile's own Core entry point.
pub fn admit_source(source: &str) -> Result<Admitted, CompileError> {
    let profile = mori_core::language(source).map_err(CompileError::Core)?;
    let source_sha256 = identity::source_hash(source);
    match profile {
        mori_core::LANGUAGE => {
            // `parse` validates before returning (parser.rs:157-159).
            let program = mori_core::parse(source).map_err(CompileError::Core)?;
            admit_checked(program, Some(source_sha256)).map(Admitted::Core1)
        }
        mori_core::stage::LANGUAGE_2 => {
            let staged = mori_core::stage::load(source).map_err(CompileError::Core)?;
            Ok(Admitted::Staged2(StagedAdmission::new(staged, source_sha256)))
        }
        other => Err(CompileError::Profile(other.to_string())),
    }
}
```

`R1CS-frontend.rs:52–68`, SHA-256 `039f0867c3efb717737909395228846aa1eef47346b08ac0671f800fed3c1bf9`:

```text
/// Admit an in-memory `/1` Program through the same Core checks.
pub fn admit_program(program: Program) -> Result<AdmittedCore1, CompileError> {
    admit_checked(program, None)
}

fn admit_checked(program: Program, source_sha256: Option<Digest32>) -> Result<AdmittedCore1, CompileError> {
    mori_core::stage::require_core1(&program).map_err(CompileError::Core)?;
    mori_core::validate(&program).map_err(CompileError::Core)?;
    let program_hash = identity::program_hash(&program)?;
    let identity = ProgramIdentity {
        language: program.language.clone(),
        semantics: program.semantics.clone(),
        program_hash,
        source_sha256,
    };
    Ok(AdmittedCore1 { program, identity })
}
```

`R1CS-circuit.rs:264–271`, SHA-256 `3856551a3466e6790c0142df9b2025487d2330f9da20caa3796f1ff27bd5649c`:

```text
/// Routes source by its declared profile and compiles `/1`. `/2` is admitted by the
/// actual stage loader and then refused with its staged lowering plan.
pub fn compile_source<F: PrimeField>(source: &str, options: CompileOptions, adapter: StatementAdapter) -> Result<CompiledRelation, CompileError> {
    match frontend::admit_source(source)? {
        Admitted::Core1(admitted) => compile::<F>(admitted, options, adapter),
        Admitted::Staged2(staged) => Err(CompileError::StagedNotLowered(Box::new(staged.lowering_plan()?))),
    }
}
```

## DR09 CALL_EDGE

prepare_claim source order is context_from_input→propose_values(selected.relation,transition)→claim_from_values. Its Result output is PreparedClaim,whose fields are TransitionValues,ContextV2,statement bytes,statement digest,and two u128 public inputs.

`DRIVER-claim.rs:35–44`, SHA-256 `f54545d8b8286cad1c3549afab82a3b15020c0b498041d093ce8e9f3cb4541d3`:

```text
/// A claim ready for the constraint system: values, context, statement and public inputs.
#[derive(Debug, Clone)]
pub struct PreparedClaim {
    pub values: TransitionValues,
    pub context: ContextV2,
    pub statement: Vec<u8>,
    pub statement_digest: [u8; 32],
    /// `[x1, x2]`: big-endian halves of `statement_digest`.
    pub public_inputs: [u128; 2],
}
```

`DRIVER-claim.rs:107–117`, SHA-256 `f54545d8b8286cad1c3549afab82a3b15020c0b498041d093ce8e9f3cb4541d3`:

```text
/// Proposes the values and builds the statement and public inputs with the compiler.
pub fn prepare_claim(
    selected: &SelectedRelation,
    transition: &TransitionInput,
    context: &ContextInput,
    vk_digest: [u8; 32],
) -> Result<PreparedClaim, DriverError> {
    let ctx = context_from_input(context, vk_digest)?;
    let values = propose_values(selected.relation(), transition)?;
    claim_from_values(selected, values, ctx)
}
```

## DR11 CALL_EDGE

propose_values matches CoreReference and calls witness::propose(relation,scenario,None); Records calls witness::from_records(relation.plan,inputs,pre,post,effects,None). Both map errors as DriverError::Proposal.

`DRIVER-claim.rs:77–86`, SHA-256 `f54545d8b8286cad1c3549afab82a3b15020c0b498041d093ce8e9f3cb4541d3`:

```text
/// Proposes transition values. A Core refusal or a record that does not fit the relation's
/// shape is a refusal to propose, never a "false" claim.
pub fn propose_values(relation: &CompiledRelation, transition: &TransitionInput) -> Result<TransitionValues, DriverError> {
    match transition {
        TransitionInput::CoreReference { scenario } => witness::propose(relation, scenario, None).map_err(DriverError::Proposal),
        TransitionInput::Records { inputs, pre, post, effects } => {
            witness::from_records(relation.plan(), inputs, pre, post, effects, None).map_err(DriverError::Proposal)
        }
    }
}
```

## DR12 CALL_EDGE

witness::propose obtains relation.program and selected action,then branches on plan.cap/cap: NotBound/None calls mori_core::simulate; PublicCap/Some calls simulate_metered; mismatch branches return Cap errors. It passes the execution to from_execution.

`R1CS-witness.rs:164–176`, SHA-256 `dea2932d39a90594c9bcca2e6cd7ad034b335aa468d0623540f860a31d202155`:

```text
pub fn propose(relation: &CompiledRelation, scenario: &Scenario, cap: Option<UInt>) -> Result<TransitionValues, MappingError> {
    let program = relation.program();
    let action = relation.plan().action.as_str();
    let execution = match (relation.plan().cap, cap) {
        (CapBinding::NotBound, None) => mori_core::simulate(program, action, scenario).map_err(MappingError::ReferenceRefused)?,
        (CapBinding::PublicCap, Some(c)) => {
            mori_core::simulate_metered(program, action, scenario, c).map_err(|refused| MappingError::ReferenceRefused(refused.error))?.execution
        }
        (CapBinding::PublicCap, None) => return Err(MappingError::Cap("relation binds a public cap; supply one".into())),
        (CapBinding::NotBound, Some(_)) => return Err(MappingError::Cap("relation binds no cap".into())),
    };
    from_execution(relation, scenario, &execution, cap)
}
```

## DR13 CALL_EDGE

from_execution compares execution action/semantics and passes scenario.inputs plus execution pre/post/effects into from_records. from_records returns TransitionValues with ordered claimed inputs,pre/post body mappings,claimed effects,and cap.

`R1CS-witness.rs:154–160`, SHA-256 `dea2932d39a90594c9bcca2e6cd7ad034b335aa468d0623540f860a31d202155`:

```text
/// Maps an `Execution` (reference output or frozen oracle record) for `relation`.
pub fn from_execution(relation: &CompiledRelation, scenario: &Scenario, execution: &Execution, cap: Option<UInt>) -> Result<TransitionValues, MappingError> {
    if execution.action != relation.plan().action || execution.semantics != mori_core::SEMANTICS {
        return Err(MappingError::Shape("execution names another action or semantics".into()));
    }
    from_records(relation.plan(), &scenario.inputs, &execution.pre, &execution.post, &execution.effects, cap)
}
```

`R1CS-witness.rs:113–152`, SHA-256 `dea2932d39a90594c9bcca2e6cd7ad034b335aa468d0623540f860a31d202155`:

```text
pub fn from_records(
    plan: &ActionPlan,
    inputs: &BTreeMap<String, Value>,
    pre: &Snapshot,
    post: &Snapshot,
    effects: &[Effect],
    cap: Option<UInt>,
) -> Result<TransitionValues, MappingError> {
    if inputs.len() != plan.inputs.len() {
        return Err(MappingError::Shape("scenario inputs differ from the action's inputs".into()));
    }
    let mut claimed_inputs = Vec::with_capacity(plan.inputs.len());
    for i in &plan.inputs {
        let v = inputs.get(&i.name).ok_or_else(|| MappingError::Shape(format!("input {} missing", i.name)))?;
        if v.ty() != i.ty {
            return Err(MappingError::Shape(format!("input {} has the wrong type", i.name)));
        }
        claimed_inputs.push(v.clone());
    }
    frame_unchanged(plan, pre, post)?;
    if effects.len() != plan.effects.len() {
        return Err(MappingError::Shape(format!("record has {} effects, the action has {}", effects.len(), plan.effects.len())));
    }
    let mut claimed_effects = Vec::with_capacity(effects.len());
    for (index, (shape, e)) in plan.effects.iter().zip(effects).enumerate() {
        claimed_effects.push(effect(plan, index, shape, e)?);
    }
    match (plan.cap, cap) {
        (CapBinding::PublicCap, None) => return Err(MappingError::Cap("relation binds a public cap; supply one".into())),
        (CapBinding::NotBound, Some(_)) => return Err(MappingError::Cap("relation binds no cap".into())),
        _ => {}
    }
    Ok(TransitionValues {
        inputs: claimed_inputs,
        pre: body(plan, pre, "pre")?,
        post: body(plan, post, "post")?,
        effects: claimed_effects,
        cap,
    })
}
```

## DR15 CALL_EDGE

claim_from_values calls statement_v2,checks statement length against262144,hashes statement through blake2b256,forms digest halves,compares compiler public_inputs_v2 with field conversions,and returns PreparedClaim. It does not invoke prove::check_claim in this body.

`DRIVER-claim.rs:15–16`, SHA-256 `f54545d8b8286cad1c3549afab82a3b15020c0b498041d093ce8e9f3cb4541d3`:

```text
/// Statement ceiling (bytes). Larger statements are refused before hashing or synthesis.
pub const MAX_STATEMENT_BYTES: usize = 262_144;
```

`DRIVER-claim.rs:46–53`, SHA-256 `f54545d8b8286cad1c3549afab82a3b15020c0b498041d093ce8e9f3cb4541d3`:

```text
/// Big-endian 128-bit halves of a 32-byte digest (`abi2.rs:297-305`).
pub fn digest_halves(d: &[u8; 32]) -> [u128; 2] {
    let mut hi = [0u8; 16];
    let mut lo = [0u8; 16];
    hi.copy_from_slice(&d[..16]);
    lo.copy_from_slice(&d[16..]);
    [u128::from_be_bytes(hi), u128::from_be_bytes(lo)]
}
```

`DRIVER-claim.rs:91–105`, SHA-256 `f54545d8b8286cad1c3549afab82a3b15020c0b498041d093ce8e9f3cb4541d3`:

```text
pub fn claim_from_values(selected: &SelectedRelation, values: TransitionValues, ctx: ContextV2) -> Result<PreparedClaim, DriverError> {
    let relation = selected.relation();
    let statement = relation.statement_v2(&values, &ctx).map_err(DriverError::Proposal)?;
    if statement.len() > MAX_STATEMENT_BYTES {
        return Err(DriverError::Input(format!("statement of {} bytes exceeds {MAX_STATEMENT_BYTES}", statement.len())));
    }
    let statement_digest = mori_r1cs::blake2b::blake2b256(&statement);
    let public_inputs = digest_halves(&statement_digest);
    // The compiler's own public-input path must agree with the digest halves.
    let compiled = relation.public_inputs_v2::<Fr>(&values, &ctx).map_err(DriverError::Proposal)?;
    if compiled != vec![Fr::from(public_inputs[0]), Fr::from(public_inputs[1])] {
        return Err(DriverError::Backend("compiler public inputs differ from the statement digest halves".into()));
    }
    Ok(PreparedClaim { values, context: ctx, statement, statement_digest, public_inputs })
}
```

## DR19 CALL_EDGE

prove calls prepare_claim then check_claim,draws two Fr values from its RNG argument,and calls Groth16::create_proof_with_reduction_and_matrices with the supplied key and returned matrices/assignment/counts. It next calls pairing_verify and returns an error if false before build_export.

`DRIVER-prove.rs:123–143`, SHA-256 `4f24098c563f09449d34d21b7e46f5b97bd7cd56c4e734e81b79f8703132ad26`:

```text
    let claim = claim::prepare_claim(selected, &request.transition, &request.context, setup.vk_digest)?;
    let satisfied = check_claim(selected, &claim)?;

    let r = Fr::rand(rng);
    let s = Fr::rand(rng);
    let proof = Groth16::<Bls12_381>::create_proof_with_reduction_and_matrices(
        pk,
        r,
        s,
        &satisfied.matrices,
        satisfied.num_inputs,
        satisfied.num_constraints,
        &satisfied.full_assignment,
    )
    .map_err(|e| DriverError::Backend(format!("Groth16 prove: {e}")))?;
    drop(satisfied);

    if !verify::pairing_verify(&setup.vk, &proof, claim.public_inputs)? {
        return Err(DriverError::ProofRejected("self-verification of the new proof failed; nothing exported".into()));
    }
    let export = export::build_export(setup, &claim, &proof)?;
```

## DR21 SOURCE_DECLARATION

ProofExport declares descriptor identity strings,context,statementHex/digest,public input strings,proofHex,CardanoExport and premises. build_export fills these from setup descriptor/PreparedClaim/proof and calls encode_proof,context_export and cardano_export.

`DRIVER-export.rs:217–240`, SHA-256 `8eb9268d81d2b50513f9f1e54d400aef08993ed1eee78ecc119c9398455dcb9c`:

```text
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProofExport {
    pub format: String,
    /// SHA-256 of the exact setup-record file bytes (declared link, checked by `verify`).
    pub setup_record_sha256: String,
    pub program_hash: String,
    pub source_sha256: Option<String>,
    pub action: String,
    pub action_index: u16,
    pub compiler_build_digest: String,
    pub r1cs_sha256: String,
    pub predicate_digest: String,
    pub abi2_shape_digest: String,
    pub vk_digest_blake2b256: String,
    pub context: ContextExport,
    pub statement_hex: String,
    pub statement_digest_blake2b256: String,
    pub public_inputs: PublicInputsExport,
    /// The 192-byte C2 proof, lowercase hex (C2 `Issue { vk, proof }` takes the record's VK).
    pub proof_hex: String,
    pub cardano: CardanoExport,
    pub premises: Vec<String>,
}
```

`DRIVER-export.rs:249–271`, SHA-256 `8eb9268d81d2b50513f9f1e54d400aef08993ed1eee78ecc119c9398455dcb9c`:

```text
pub fn build_export(setup: &ParsedSetup, claim: &PreparedClaim, proof: &Proof<Bls12_381>) -> Result<ProofExport, DriverError> {
    let desc = &setup.record.relation;
    Ok(ProofExport {
        format: PROOF_FORMAT.into(),
        setup_record_sha256: setup.record_sha256.hex(),
        program_hash: desc.program_hash.clone(),
        source_sha256: desc.source_sha256.clone(),
        action: desc.action.clone(),
        action_index: desc.action_index,
        compiler_build_digest: desc.compiler_build_digest.clone(),
        r1cs_sha256: desc.r1cs_sha256.clone(),
        predicate_digest: desc.predicate_digest.clone(),
        abi2_shape_digest: desc.abi2_shape_digest.clone(),
        vk_digest_blake2b256: hexutil::encode(&setup.vk_digest),
        context: context_export(&claim.context),
        statement_hex: hexutil::encode(&claim.statement),
        statement_digest_blake2b256: hexutil::encode(&claim.statement_digest),
        public_inputs: PublicInputsExport { x1: claim.public_inputs[0].to_string(), x2: claim.public_inputs[1].to_string() },
        proof_hex: hexutil::encode(&encoding::encode_proof(proof)?),
        cardano: cardano_export(desc, &setup.vk_digest, &claim.context, &claim.statement_digest),
        premises: PREMISES.iter().map(|s| (*s).to_string()).collect(),
    })
}
```

## DR24 CALL_EDGE

After build_export,prove serializes document_bytes,parses the bytes,checks the export and compares statement/proof roundtrip values. document_bytes calls Core json::export and appends newline. check_export contains local descriptor/context/digest/input/proof/Cardano/premise comparisons and returns CheckedExport.

`DRIVER-prove.rs:143–151`, SHA-256 `4f24098c563f09449d34d21b7e46f5b97bd7cd56c4e734e81b79f8703132ad26`:

```text
    let export = export::build_export(setup, &claim, &proof)?;
    let export_bytes = export::document_bytes(&export)?;
    // The bytes must pass the verifier's own non-cryptographic checks.
    let reparsed = export::parse_proof_export(&export_bytes)?;
    let checked = export::check_export(setup, &reparsed)?;
    if checked.statement != claim.statement || hexutil::encode(&encoding::encode_proof(&checked.proof)?) != export.proof_hex {
        return Err(DriverError::Backend("proof export does not round-trip".into()));
    }
    Ok(ProveOutput { export, export_bytes })
```

`DRIVER-export.rs:77–83`, SHA-256 `8eb9268d81d2b50513f9f1e54d400aef08993ed1eee78ecc119c9398455dcb9c`:

```text
/// Canonical bytes of a public document: Core's ASCII-safe pretty JSON plus a final newline.
pub fn document_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, DriverError> {
    let text = mori_core::json::export(value).map_err(|e| DriverError::Backend(format!("export: {e}")))?;
    let mut out = text.into_bytes();
    out.push(b'\n');
    Ok(out)
}
```

`DRIVER-export.rs:242–246`, SHA-256 `8eb9268d81d2b50513f9f1e54d400aef08993ed1eee78ecc119c9398455dcb9c`:

```text
pub fn parse_proof_export(bytes: &[u8]) -> Result<ProofExport, DriverError> {
    let e: ProofExport = parse_json(bytes, "proof export")?;
    fixed("proof export format", &e.format, PROOF_FORMAT)?;
    Ok(e)
}
```

`DRIVER-export.rs:284–320`, SHA-256 `8eb9268d81d2b50513f9f1e54d400aef08993ed1eee78ecc119c9398455dcb9c`:

```text
pub fn check_export(setup: &ParsedSetup, e: &ProofExport) -> Result<CheckedExport, DriverError> {
    fixed("proof export format", &e.format, PROOF_FORMAT)?;
    let desc = &setup.record.relation;
    same("setupRecordSha256", &setup.record_sha256.hex(), &e.setup_record_sha256)?;
    same("programHash", &desc.program_hash, &e.program_hash)?;
    same("sourceSha256", &format!("{:?}", desc.source_sha256), &format!("{:?}", e.source_sha256))?;
    same("action", &desc.action, &e.action)?;
    same("actionIndex", &desc.action_index.to_string(), &e.action_index.to_string())?;
    same("compilerBuildDigest", &desc.compiler_build_digest, &e.compiler_build_digest)?;
    same("r1csSha256", &desc.r1cs_sha256, &e.r1cs_sha256)?;
    same("predicateDigest", &desc.predicate_digest, &e.predicate_digest)?;
    same("abi2ShapeDigest", &desc.abi2_shape_digest, &e.abi2_shape_digest)?;
    same("vkDigestBlake2b256", &hexutil::encode(&setup.vk_digest), &e.vk_digest_blake2b256)?;

    let statement = hexutil::decode_bounded(&e.statement_hex, MAX_STATEMENT_BYTES, "statementHex")?;
    let view = claim::parse_statement(&statement)?;
    claim::check_statement_against(desc, &setup.vk_digest, &view)?;
    if context_export(&view.context) != e.context {
        return Err(DriverError::mismatch("context fields", "the statement's context bytes", "different declared fields"));
    }
    let statement_digest = mori_r1cs::blake2b::blake2b256(&statement);
    same("statementDigestBlake2b256", &hexutil::encode(&statement_digest), &e.statement_digest_blake2b256)?;
    let public_inputs = claim::digest_halves(&statement_digest);
    let x1 = hexutil::parse_u128_decimal(&e.public_inputs.x1, "x1")?;
    let x2 = hexutil::parse_u128_decimal(&e.public_inputs.x2, "x2")?;
    same("x1", &public_inputs[0].to_string(), &x1.to_string())?;
    same("x2", &public_inputs[1].to_string(), &x2.to_string())?;
    let proof_bytes = hexutil::decode_exact::<PROOF_LEN>(&e.proof_hex, "proofHex")?;
    let proof = encoding::decode_proof(&proof_bytes)?;
    if cardano_export(desc, &setup.vk_digest, &view.context, &statement_digest) != e.cardano {
        return Err(DriverError::mismatch("cardano section", "values derived from the statement and record", "different values"));
    }
    if e.premises.len() != PREMISES.len() || e.premises.iter().zip(PREMISES).any(|(a, b)| a != b) {
        return Err(DriverError::mismatch("premises", "the fixed premise list", "a different list"));
    }
    Ok(CheckedExport { proof, public_inputs, statement, statement_digest, view })
}
```

## DR25 CALL_EDGE

CLI prove writes out.export_bytes with write_new and prints selected export fields. write_new uses create_new and Unix mode644 for the public path,then write_all/sync_all. This task wrote only source captures/analysis,not a proof export or ledger artifact.

`DRIVER-main.rs:106–120`, SHA-256 `9c796a758a17b0f2fe955c571188afa5bd6407fc391541b7145c97315b0f1b4c`:

```text
/// Creates `p` exclusively and writes `bytes`. An existing path is refused, never replaced.
fn write_new(p: &Path, bytes: &[u8], private: bool) -> Result<(), DriverError> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(if private { 0o600 } else { 0o644 });
    }
    #[cfg(not(unix))]
    let _ = private;
    let mut file = options.open(p).map_err(|e| DriverError::Io(format!("cannot create {}: {e}", p.display())))?;
    file.write_all(bytes).map_err(|e| DriverError::Io(format!("cannot write {}: {e}", p.display())))?;
    file.sync_all().map_err(|e| DriverError::Io(format!("cannot sync {}: {e}", p.display())))?;
    Ok(())
```

`DRIVER-main.rs:194–202`, SHA-256 `9c796a758a17b0f2fe955c571188afa5bd6407fc391541b7145c97315b0f1b4c`:

```text
            let out = prove::prove(&selected, &setup, &pk, &request, &mut rng)?;
            write_new(out_proof, &out.export_bytes, false)?;
            print_json(&serde_json::json!({
                "proofExportSha256": mori_r1cs::identity::digest_bytes(&out.export_bytes).hex(),
                "statementDigestBlake2b256": out.export.statement_digest_blake2b256,
                "x1": out.export.public_inputs.x1,
                "x2": out.export.public_inputs.x2,
            }))?;
            Ok(())
```

## Read limits

The revised21-file manifest pins13 driver/R1CS Rust bodies plus8 explicit Cargo/CoreLib comparison bodies. Twenty were read fully; R1CS-circuit.rs has exact partial ranges recorded in SOURCE-CHECKS.json. U01–U05 name remaining body/dependency boundaries and cite their callers. Four overlapping captured parent files are mechanically byte-equal in PARENT-BYTE-COMPARISON.json; this grants no semantic/runtime equivalence or retroactive approval. The Core1 selector refusal and Staged2 compiler refusal are distinct source branches (DR06). No source grammar or actual Cargo build/dependency resolution is inferred from imported names or the captured Cargo path declarations.

## Exact source roots and Cargo declaration boundary

The driver/R1CS capture remains at00ffd. main8174 files are separate git-show objects available in the same local repository object store; a root crates/mori-core path at main8174 must not be assumed present at00ffd. SOURCE-ROOTS.json records exact Git paths,local root metadata,lexical path normalization,and three explicit byte comparisons. Workspace Cargo and Core Cargo pairs are equal; Core lib.rs differs.

**RG01 SOURCE_DECLARATION** — Main8174 root workspace declares crates/* members and excludes experiments; current00ffd vendored workspace has byte-identical Cargo text at a different Git prefix. Core Cargo files likewise match byte-for-byte and declare inherited workspace dependencies. This is source packaging,not an observed current build.

`MAIN-workspace-Cargo.toml:1–12`, SHA-256 `dea75cec40896a7aab9f561b9bd26ee5457ff9c46c6dd7522a6a5bccac048e2a`, Git `8174e52975d8db9bf59636fea0fa338c0f47c855:Cargo.toml`:

```text
[workspace]
resolver = "3"
members = ["crates/*"]
# Standalone experiment and evidence crates keep their own builds, and
# crates/fixtures holds shared test programs, not a crate.
exclude = ["experiments", "raw", "wiki-llm", "deliverables", "crates/fixtures"]

[workspace.package]
edition = "2024"
rust-version = "1.99"
license = "Apache-2.0"
repository = "https://github.com/CharlesHoskinson/Moriarty"
```

`VENDORED-workspace-Cargo.toml:1–12`, SHA-256 `dea75cec40896a7aab9f561b9bd26ee5457ff9c46c6dd7522a6a5bccac048e2a`, Git `00ffdca38a30ea98c91a6731e2d6bdb373d16f0b:experiments/moriarty-cardano-rust/moriarty/Cargo.toml`:

```text
[workspace]
resolver = "3"
members = ["crates/*"]
# Standalone experiment and evidence crates keep their own builds, and
# crates/fixtures holds shared test programs, not a crate.
exclude = ["experiments", "raw", "wiki-llm", "deliverables", "crates/fixtures"]

[workspace.package]
edition = "2024"
rust-version = "1.99"
license = "Apache-2.0"
repository = "https://github.com/CharlesHoskinson/Moriarty"
```

`MAIN-core-Cargo.toml:1–19`, SHA-256 `71d7eeb2520b07edb18033e22a816b55d76ee80cc8ffd6d06bd5e0eca8cf5030`, Git `8174e52975d8db9bf59636fea0fa338c0f47c855:crates/mori-core/Cargo.toml`:

```text
[package]
name = "mori-core"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
mori-lexer.workspace = true
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
sha2.workspace = true
k256.workspace = true

[dev-dependencies]
proptest.workspace = true
midnight-base-crypto.workspace = true
midnight-serialize.workspace = true
```

`VENDORED-core-Cargo.toml:1–19`, SHA-256 `71d7eeb2520b07edb18033e22a816b55d76ee80cc8ffd6d06bd5e0eca8cf5030`, Git `00ffdca38a30ea98c91a6731e2d6bdb373d16f0b:experiments/moriarty-cardano-rust/moriarty/crates/mori-core/Cargo.toml`:

```text
[package]
name = "mori-core"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[dependencies]
mori-lexer.workspace = true
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
sha2.workspace = true
k256.workspace = true

[dev-dependencies]
proptest.workspace = true
midnight-base-crypto.workspace = true
midnight-serialize.workspace = true
```

**RG02 INFERENCE** — Driver Cargo declares a standalone workspace plus mori-r1cs and mori-core relative paths; R1CS Cargo declares a standalone workspace and ../../../moriarty/crates/mori-core. Lexical normalization of those path strings points to the current vendored experiments/moriarty-cardano-rust/moriarty/crates/mori-core prefix; no Cargo resolution/build was run.

`DRIVER-Cargo.toml:11–26`, SHA-256 `8e4d6049d6e67e7c5e7a321badb50232883f776d466e96daf5777b529ec33948`, Git `00ffdca38a30ea98c91a6731e2d6bdb373d16f0b:experiments/moriarty-cardano-rust/k2/Cargo.toml`:

```text
# Standalone root: not a member of the Moriarty workspace or of the R1 crate.
[workspace]

[lib]
name = "mori_k2_driver"
path = "src/lib.rs"

[[bin]]
name = "mori-k2"
path = "src/main.rs"

[dependencies]
# Actual compiler and Core, by path, in the proposed execution layout
# <root>/{moriarty,r1cs,k2}. No identity of either is copied into this crate.
mori-r1cs = { path = "../r1cs/source/mori-r1cs" }
mori-core = { path = "../moriarty/crates/mori-core" }
```

`R1CS-Cargo.toml:11–20`, SHA-256 `5a58874fc0452c34a1f794df552d126367f18f7ccb851b2a109171a7cc9b6e88`, Git `00ffdca38a30ea98c91a6731e2d6bdb373d16f0b:experiments/moriarty-cardano-rust/r1cs/source/mori-r1cs/Cargo.toml`:

```text
# Standalone root: this crate is not a member of the Moriarty workspace and never edits it.
[workspace]

[dependencies]
# Actual current Core is the admission frontend (parse/validate/stage::load) and the
# reference used only to *propose* witness values. The relative path assumes the
# isolated execution layout in ../../EXECUTION-REQUEST.json: a pinned copy of the
# Moriarty files listed in ../../CORE-CLOSURE-PINS.json at <root>/moriarty and this
# task directory at <root>/r1cs. It deliberately names no live, mutable worktree.
mori-core = { path = "../../../moriarty/crates/mori-core" }
```

**RG03 SOURCE_DECLARATION** — Main8174 Core lib declares Sherlock public/source/bounded modules and private cfg(test) drafts; current00ffd vendored Core lib has17 lines and lacks those declarations in this captured body. Common parse/simulate/validate exports appear in both. Files differ; no semantic compatibility or source-upgrade adoption is inferred.

`MAIN-core-lib.rs:1–41`, SHA-256 `df45168fa300562f532577739731c257d016a387fc943b25975ae46e3968a2fa`, Git `8174e52975d8db9bf59636fea0fa338c0f47c855:crates/mori-core/src/lib.rs`:

```text
//! Bounded composable Moriarty language and deterministic local semantics.
pub mod intent;
mod ir;
pub mod json;
mod logical_cost;
mod parser;
pub mod policy;
pub mod sherlock;

// Unregistered draft source capability: Core-only ordinary host test proposal.
// These lexer-dependent modules stay outside Sherlock's standalone Std graph.
#[cfg(test)]
#[path = "sherlock/draft_source.rs"]
mod sherlock_draft;
#[cfg(test)]
#[path = "sherlock/draft_typecheck.rs"]
mod sherlock_draft_types;
#[cfg(test)]
#[path = "sherlock/draft_source_tests.rs"]
mod sherlock_draft_tests;
#[cfg(test)]
#[path = "sherlock/draft_image_bridge.rs"]
mod sherlock_draft_image;
#[cfg(test)]
#[path = "sherlock/draft_image_bridge_tests.rs"]
mod sherlock_draft_image_tests;
#[path = "sherlock/source_adapter.rs"]
pub mod sherlock_source;
// Caller-bounded scalar diagnostic proposal; H1 qualification is explicit in its API.
#[path = "sherlock/bounded_scalar_source.rs"]
pub mod sherlock_bounded_source;
pub mod stage;
mod replay;
mod runtime;
mod source_sponsor;
pub use ir::*;
pub use logical_cost::*;
pub use parser::{language, parse};
pub use replay::{DetailedReplayError, ReplayDifference, compare_execution, replay_detailed};
pub use runtime::{replay, simulate, validate};
pub use source_sponsor::*;
```

`VENDORED-core-lib.rs:1–17`, SHA-256 `9d9203931d2a61d9063bb5fc357d6160e5e1e162ff9693ede8fa56e4c0f185cd`, Git `00ffdca38a30ea98c91a6731e2d6bdb373d16f0b:experiments/moriarty-cardano-rust/moriarty/crates/mori-core/src/lib.rs`:

```text
//! Bounded composable Moriarty language and deterministic local semantics.
pub mod intent;
mod ir;
pub mod json;
mod logical_cost;
mod parser;
pub mod policy;
pub mod stage;
mod replay;
mod runtime;
mod source_sponsor;
pub use ir::*;
pub use logical_cost::*;
pub use parser::{language, parse};
pub use replay::{DetailedReplayError, ReplayDifference, compare_execution, replay_detailed};
pub use runtime::{replay, simulate, validate};
pub use source_sponsor::*;
```

