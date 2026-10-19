//! Sherlock A102 Literal native statement bridge. One complete immutable
//! ProgramBlock, complete pre/post StateBlocks and the wire envelope form one
//! statement; the real emit_literal_step fragment runs with a forced in-circuit
//! enable. Scope: exactly one charged Eval(Literal)->Return transition, not other
//! rules, continuity or program finish. Only a listed host subset of public-data
//! checks runs, off-chain, and no ledger operation VK enforces it. The post is a
//! Step-mode image, never a canonical-boundary image. projectA, StepInv, F4, G4
//! and literal uiValid remain open. Compile/keygen/prove/verify: UNOBSERVED.
use super::*;
use crate::parameters::OfflineParameters;
use crate::proving::{MAX_K, MAX_PROOF_BYTES, ProofOptions, hex};
use super::literal_host::construction;
pub use super::literal_host::{
    BINDING_VERSION, CapacityRecord, ImageValues, LiteralLayout, LiteralSourceIdentity,
    LiteralTransitionRequest, LiteralValues, OPEN_OBLIGATIONS, PROFILE, RULE_CHARGE,
    RULE_START_LINE, RULE_TAG, SCOPE, STATEMENT_FORMAT,
};
use crate::{Error, LEDGER_REVISION, ZKIR_VERSION, error};
use midnight_transient_crypto::proofs::{
    KeyLocation, PARAMS_VERIFIER, Proof, ProofPreimage, VERIFIER_MAX_DEGREE, Zkir,
};
use midnight_zkir_v3::ir::TypedIdentifier;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const STEP_SCOPE: &str = "sherlock.a102.step";
const KEY_LOCATION: &str = "moriarty-sherlock-a102-literal/1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CircuitMode {
    /// PublicInput/Impact positional statement, communications disabled.
    LocalPublic,
    /// Statement lanes are call inputs under the communications commitment.
    LedgerCall,
}

/// Existing local verifier capacity: the binding plus the statement fit 2^VERIFIER_MAX_DEGREE.
pub fn local_max_statement_lanes() -> usize {
    (1usize << VERIFIER_MAX_DEGREE) - 1
}
fn take_image(it: &mut impl Iterator<Item = Operand>, root: &[LiteralLane], tables: &[LiteralTable]) -> Result<LiteralImage, Error> {
    let mut take = |n: usize| -> Result<Vec<Operand>, Error> {
        let row: Vec<Operand> = (&mut *it).take(n).collect();
        if row.len() == n { Ok(row) } else { Err(error("sherlock-shape", "Statement operands ended early.")) }
    };
    let root = take(root.len())?;
    let mut rows = Vec::with_capacity(tables.len());
    for table in tables {
        let mut table_rows = Vec::with_capacity(table.capacity);
        for _ in 0..table.capacity {
            table_rows.push(take(table.lanes.len())?);
        }
        rows.push(table_rows);
    }
    Ok(LiteralImage { root, tables: rows })
}
impl LiteralLayout {
    /// Emit the full relation: bound statement operands, forced A102 enable,
    /// the unmodified literal fragment and the mode's Output terminator.
    pub fn emit(&self, mode: CircuitMode) -> Result<(IrSource, String), Error> {
        let local = mode == CircuitMode::LocalPublic;
        let mut code = Vec::new();
        let mut inputs: Vec<TypedIdentifier> = Vec::new();
        let mut operands = Vec::with_capacity(self.lanes.len());
        for position in 0..self.lanes.len() {
            let id = Identifier(format!("%sherlock.a102.statement.{position}"));
            let operand = Operand::Variable(id.clone());
            if local {
                code.push(Instruction::PublicInput { guard: None, val_t: IrType::Native, output: id });
                code.push(Instruction::Impact { guard: immediate(1), inputs: vec![operand.clone()] });
            } else {
                inputs.push(serde_json::from_value(serde_json::json!({"name": id, "type": IrType::Native}))
                    .map_err(|e| error("zkir", e.to_string()))?);
            }
            operands.push(operand);
        }
        let [lo, hi] = self.limbs();
        code.push(Instruction::ConstrainEq { a: operands[0].clone(), b: immediate(lo) });
        code.push(Instruction::ConstrainEq { a: operands[1].clone(), b: immediate(hi) });
        let s = &self.schema;
        let mut rest = operands[2..].iter().cloned();
        let program = take_image(&mut rest, &s.program_header, &s.program_tables)?;
        let pre = take_image(&mut rest, &s.state_root, &s.state_tables)?;
        let post = take_image(&mut rest, &s.state_root, &s.state_tables)?;
        let (rule_tag, charge) = match (rest.next(), rest.next(), rest.next()) {
            (Some(t), Some(c), None) => (t, c),
            _ => return Err(error("sherlock-shape", "Envelope operands differ from the layout.")),
        };
        let enable_id = Identifier("%sherlock.a102.dispatch.enable".into());
        code.push(Instruction::TestEq { a: rule_tag.clone(), b: immediate(RULE_TAG), output: enable_id.clone() });
        let enable = Operand::Variable(enable_id);
        code.push(Instruction::ConstrainToBoolean { val: enable.clone() });
        code.push(Instruction::ConstrainEq { a: enable.clone(), b: immediate(1) });
        let block = emit_literal_step(s, &LiteralStepInputs {
            enable, program, pre, post: post.clone(),
            envelope: LiteralEnvelope { rule_tag: rule_tag.clone(), charge: charge.clone() },
        }, STEP_SCOPE).map_err(construction)?;
        code.extend(block.instructions);
        let vals: Vec<Operand> = if local {
            post.root.iter().chain(post.tables.iter().flatten().flatten()).cloned().chain([rule_tag, charge]).collect()
        } else {
            Vec::new()
        };
        let outputs = vec![IrType::Native; vals.len()];
        code.push(Instruction::Output { vals });
        let ir = IrSource {
            version: Default::default(),
            inputs,
            outputs,
            do_communications_commitment: !local,
            instructions: Arc::new(code),
        };
        let mut disk = serde_json::to_value(&ir).map_err(|e| error("serialization", e.to_string()))?;
        disk["version"] = serde_json::json!({"major": 3, "minor": 0});
        let zkir = serde_json::to_string(&disk).map_err(|e| error("serialization", e.to_string()))?;
        let parsed = IrSource::load(zkir.as_bytes()).map_err(|e| error("zkir", e.to_string()))?;
        if parsed != ir {
            return Err(error("zkir", "Pinned upstream round-trip changed the IR."));
        }
        Ok((ir, zkir))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiteralClaim {
    pub format: String,
    pub binding: String,
    pub profile: String,
    pub scope: String,
    pub ledger_revision: String,
    pub zkir_version: String,
    pub rule_start_line: u32,
    pub rule_tag: u32,
    pub charge: u32,
    pub capacities: CapacityRecord,
    pub identity: LiteralSourceIdentity,
    pub layout_hash: String,
    pub relation_hash: String,
    pub context_hash: String,
    pub zkir_hash: String,
    pub circuit_k: u8,
    /// Decimal: two context limbs, ProgramBlock, pre, post, rule_tag, charge.
    pub public_values: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiteralProofBundle {
    pub claim: LiteralClaim,
    pub proof_hex: String,
}
impl LiteralProofBundle {
    pub fn from_json(text: &str) -> Result<Self, Error> {
        let value = mori_core::json::parse(text).map_err(|e| error("json", e.to_string()))?;
        serde_json::from_value(value).map_err(|e| error("json", e.to_string()))
    }
}
#[derive(Clone, Debug, Serialize)]
pub struct LiteralVerification {
    pub qualification: &'static str,
    pub context_hash: String,
    pub zkir_hash: String,
    pub circuit_k: u8,
    pub proof_bytes: u32,
    pub open_obligations: Vec<&'static str>,
}
struct Prepared {
    ir: IrSource,
    values: Vec<Fr>,
    binding: Fr,
    claim: LiteralClaim,
}
impl Prepared {
    fn statement(&self) -> Vec<Fr> {
        std::iter::once(self.binding).chain(self.values.iter().copied()).collect()
    }
}
fn prepare(request: &LiteralTransitionRequest, options: &ProofOptions) -> Result<Prepared, Error> {
    if options.max_k == 0 || options.max_k > MAX_K {
        return Err(error("limit", "Native k cap must be between 1 and 19."));
    }
    let layout = LiteralLayout::new(&request.capacities, &request.identity, local_max_statement_lanes())?;
    let statement = layout.statement(&request.values)?;
    if statement.len() + 1 > (1usize << VERIFIER_MAX_DEGREE) {
        return Err(error("limit", "Public statement exceeds the pinned verifier capacity."));
    }
    let (ir, zkir) = layout.emit(CircuitMode::LocalPublic)?;
    let mut claim = LiteralClaim {
        format: STATEMENT_FORMAT.into(),
        binding: BINDING_VERSION.into(),
        profile: PROFILE.into(),
        scope: SCOPE.into(),
        ledger_revision: LEDGER_REVISION.into(),
        zkir_version: ZKIR_VERSION.into(),
        rule_start_line: RULE_START_LINE as u32,
        rule_tag: RULE_TAG as u32,
        charge: RULE_CHARGE as u32,
        capacities: layout.record(),
        identity: layout.identity.clone(),
        layout_hash: hex(&layout.layout_hash),
        relation_hash: hex(&layout.relation_hash),
        context_hash: layout.context_hex(),
        zkir_hash: hex(&Sha256::digest(zkir.as_bytes())),
        // Widest admitted k until the native cost selection below.
        circuit_k: MAX_K,
        public_values: statement.iter().map(|v| v.to_string()).collect(),
    };
    // Mirror proving.rs: the bundle with the largest admitted proof must fit the
    // bounded JSON profile before any native k, key or proof work.
    crate::proving::validate_json_profile(&LiteralProofBundle { claim: claim.clone(), proof_hex: "00".repeat(MAX_PROOF_BYTES) })?;
    // Native cost selection, never ordinary metadata.
    let circuit_k = ir.k();
    if circuit_k == 0 || circuit_k > options.max_k {
        return Err(error("limit", format!("Circuit k={circuit_k} exceeds the explicit native k cap.")));
    }
    claim.circuit_k = circuit_k;
    let hash = crate::digest(&(BINDING_VERSION, &claim))?;
    let low = u128::from_le_bytes(hash[..16].try_into().expect("16 bytes"));
    let high = u128::from_le_bytes(hash[16..].try_into().expect("16 bytes"));
    let binding = Fr::from(low) + (Fr::from(u128::MAX) + Fr::from(1u64)) * Fr::from(high);
    Ok(Prepared { ir, values: statement.into_iter().map(Fr::from).collect(), binding, claim })
}
fn decode_proof(text: &str) -> Result<Proof, Error> {
    if text.is_empty() || text.len() > MAX_PROOF_BYTES * 2 || text.len() % 2 != 0 {
        return Err(error("proof", "Expected nonempty bounded even-length proof hex."));
    }
    let digit = |b: u8| match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        _ => None,
    };
    text.as_bytes().chunks_exact(2)
        .map(|p| Some((digit(p[0])? << 4) | digit(p[1])?))
        .collect::<Option<Vec<u8>>>()
        .map(Proof)
        .ok_or_else(|| error("proof", "Proof hex must use canonical lowercase digits."))
}
/// Native work: pinned SRS load, keygen, prove and producer-side verify.
pub fn prove_literal_transition(request: &LiteralTransitionRequest, options: &ProofOptions) -> Result<LiteralProofBundle, Error> {
    let prepared = prepare(request, options)?;
    let params = OfflineParameters::load(&options.params_dir, prepared.claim.circuit_k)?;
    let expected = prepared.statement();
    let preimage = ProofPreimage {
        inputs: vec![],
        private_transcript: vec![],
        public_transcript_inputs: prepared.values.clone(),
        public_transcript_outputs: prepared.values.clone(),
        binding_input: prepared.binding,
        communications_commitment: None,
        key_location: KeyLocation(KEY_LOCATION.into()),
    };
    let proof = futures::executor::block_on(async {
        let (pk, vk) = prepared.ir.keygen(&params).await.map_err(|e| error("keygen", e.to_string()))?;
        let (proof, actual, skips) = prepared.ir.prove(rand::rngs::OsRng, &params, pk, &preimage).await
            .map_err(|e| error("proving", e.to_string()))?;
        if actual != expected || skips.len() != prepared.values.len() || skips.iter().any(Option::is_some) {
            return Err(error("statement", "Native producer returned a different or skipped public statement."));
        }
        if proof.0.is_empty() || proof.0.len() > MAX_PROOF_BYTES {
            return Err(error("limit", "Native proof exceeds the nonempty 262144-byte cap."));
        }
        vk.verify(&PARAMS_VERIFIER, &proof, expected.into_iter()).map_err(|e| error("verification", e.to_string()))?;
        Ok(proof)
    })?;
    let bundle = LiteralProofBundle { claim: prepared.claim, proof_hex: hex(&proof.0) };
    crate::proving::validate_json_profile(&bundle)?;
    Ok(bundle)
}
/// Independent verification: rebuild layout, statement, circuit, claim and VK;
/// no bundle field, exported key or status is verification authority.
pub fn verify_literal_transition(request: &LiteralTransitionRequest, bundle: &LiteralProofBundle, options: &ProofOptions) -> Result<LiteralVerification, Error> {
    let prepared = prepare(request, options)?;
    if bundle.claim != prepared.claim {
        return Err(error("claim", "Claim differs from the independently reconstructed A102 statement."));
    }
    let proof = decode_proof(&bundle.proof_hex)?;
    let params = OfflineParameters::load(&options.params_dir, prepared.claim.circuit_k)?;
    let expected = prepared.statement();
    futures::executor::block_on(async {
        let vk = prepared.ir.keygen_vk(&params).await.map_err(|e| error("keygen", e.to_string()))?;
        vk.verify(&PARAMS_VERIFIER, &proof, expected.into_iter()).map_err(|e| error("verification", e.to_string()))
    })?;
    Ok(LiteralVerification {
        qualification: "local-native-a102-literal-transition-proof-unqualified",
        context_hash: prepared.claim.context_hash.clone(),
        zkir_hash: prepared.claim.zkir_hash.clone(),
        circuit_k: prepared.claim.circuit_k,
        proof_bytes: proof.0.len() as u32,
        open_obligations: OPEN_OBLIGATIONS.to_vec(),
    })
}

#[cfg(test)]
mod unrun_fixture_tests {
    //! UNRUN ordinary construction checks: no k, keygen, proof or verification.
    //! Capacities, stack limit and placeholder digests are construction data only.
    use super::*;
    fn zero(root: &[LiteralLane], tables: &[LiteralTable]) -> ImageValues {
        ImageValues {
            root: vec![0; root.len()],
            tables: tables.iter().map(|t| vec![vec![0; t.lanes.len()]; t.capacity]).collect(),
        }
    }
    fn set(image: &mut ImageValues, lanes: &[LiteralLane], at: Option<(usize, usize)>, name: &str, value: u128) {
        let column = lanes.iter().position(|l| l.name == name).unwrap();
        match at {
            None => image.root[column] = value,
            Some((t, r)) => image.tables[t][r][column] = value,
        }
    }
    fn fixture() -> (LiteralLayout, LiteralValues) {
        let digest = "00".repeat(32);
        let identity = LiteralSourceIdentity {
            k_source_sha256: digest.clone(),
            reference_source_sha256: digest.clone(),
            program_source_sha256: digest,
        };
        let capacities = LiteralCapacities {
            program: [0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 4, 5, 1, 0],
            state: [0, 0, 0, 0, 0, 1],
            stack_limit: 5,
        };
        let layout = LiteralLayout::new(&capacities, &identity, local_max_statement_lanes()).unwrap();
        let s = layout.schema.clone();
        let mut program = zero(&s.program_header, &s.program_tables);
        for (name, v) in [("activeOccurrences", 1), ("activeExpressions", 1), ("activeTypes", 4),
            ("activeConstructors", 4), ("activeConstructorFieldTypes", 5), ("activeConstants", 1)] {
            set(&mut program, &s.program_header, None, name, v);
        }
        for (t, rows) in [
            (10usize, vec![vec![0u128, 0, 0], vec![1, 0, 0], vec![2, 0, 2], vec![3, 2, 2]]),
            (11, vec![vec![2, 0, 0, 0], vec![2, 1, 0, 2], vec![3, 0, 2, 1], vec![3, 1, 3, 2]]),
            (12, vec![vec![0], vec![2], vec![0], vec![3], vec![3]]),
        ] {
            for (r, row) in rows.into_iter().enumerate() {
                program.tables[t][r] = row;
            }
        }
        set(&mut program, &s.program_tables[13].lanes, Some((13, 0)), "uintBytes.0", 7);
        let mut pre = zero(&s.state_root, &s.state_tables);
        for (name, v) in [("controlLength", 1), ("stackLimit", 5), ("remainingFuel.0", 1), ("initialFuel.0", 1)] {
            set(&mut pre, &s.state_root, None, name, v);
        }
        let mut post = pre.clone();
        set(&mut post, &s.state_root, None, "remainingFuel.0", 0);
        set(&mut post, &s.state_tables[5].lanes, Some((5, 0)), "tag", 1);
        (layout, LiteralValues { program, pre, post, rule_tag: 3, charge: 1 })
    }
    #[test]
    fn uint7_literal_fixture_builds_complete_statement_and_both_relations() {
        let (layout, values) = fixture();
        let statement = layout.statement(&values).unwrap();
        assert_eq!(statement.len(), layout.statement_len());
        assert_eq!(&statement[..2], &layout.limbs()[..]);
        let (local, _) = layout.emit(CircuitMode::LocalPublic).unwrap();
        assert!(!local.do_communications_commitment && local.inputs.is_empty());
        let (ledger, _) = layout.emit(CircuitMode::LedgerCall).unwrap();
        assert!(ledger.do_communications_commitment && ledger.outputs.is_empty());
        assert_eq!(ledger.inputs.len(), statement.len());
    }
    #[test]
    fn explicit_statement_lane_bound_refuses_before_layout_allocation() {
        let (layout, _) = fixture();
        let n = layout.statement_len();
        assert!(LiteralLayout::new(&layout.capacities, &layout.identity, n).is_ok());
        assert!(LiteralLayout::new(&layout.capacities, &layout.identity, n - 1).is_err());
    }
    #[test]
    fn wrong_rule_or_builtin_rows_refuse_before_native_work() {
        let (layout, values) = fixture();
        let mut bad = values.clone();
        bad.rule_tag = 4;
        assert!(layout.statement(&bad).is_err());
        let mut bad = values;
        bad.program.tables[11][0][0] = 3;
        assert!(layout.statement(&bad).is_err());
    }
}
