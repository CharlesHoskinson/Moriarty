//! Sherlock linked A102 -> A80 child producer. One real `Machine` observes A102
//! Eval(Literal) -> Return and then A80 Return -> StepDone through
//! `produce_a102_literal_return_publication`. The A102 child is the unchanged C3
//! `ChildKey`. The A80 child is a distinct LocalPublic relation over the unmodified
//! `emit_return_publication_step` fragment with its own layout, statement, context,
//! claim domain and key; A80 values never enter the A102-only `LiteralLayout`. Its
//! witness is the checked C3 `child_witness` carrier after `check_child_statement`.
//!
//! Every link equality here (contexts, program, A102.post == A80.pre, envelopes, the
//! Init subset of the genesis pre) is host tooling. Validity needs a reviewed parent
//! that recursively verifies both children and constrains the same lanes and pinned
//! keys. Both later images are Step-mode Constant-space images, never canonical
//! StateBlocks. Roots and statement commitments belong to the consumer's pinned
//! Poseidon and are not computed here. Compile/keygen/prove/verify: UNOBSERVED.
use crate::parameters::OfflineParameters;
use crate::proving::{MAX_K, MAX_PROOF_BYTES, ProofOptions, hex};
use crate::sherlock_child::{
    CHILD_BINDING_VERSION, CHILD_FORMAT, CHILD_OPEN_OBLIGATIONS, CHILD_QUALIFICATION,
    CHILD_TRANSCRIPT, ChildKey, ChildTranscript, ChildVerification, SherlockChildProof,
    check_child_statement, child_witness, field_hex, verify_sherlock_child,
};
use crate::sherlock_machine::literal_bridge::{
    LiteralLayout, LiteralTransitionRequest, OPEN_OBLIGATIONS, PROFILE, RULE_CHARGE, RULE_TAG,
    check_literal_transition, local_max_statement_lanes,
};
use crate::sherlock_machine::literal_host::{
    CapacityRecord, ImageValues, LiteralCapacities, LiteralLane, LiteralSchema,
    LiteralSourceIdentity, LiteralTable, LiteralValues, construction, decode_digest,
    literal_schema,
};
use crate::sherlock_machine::return_publication_step::{
    RETURN_PUBLICATION_RULE_CHARGE, RETURN_PUBLICATION_RULE_START_LINE,
    RETURN_PUBLICATION_RULE_TAG, ReturnPublicationStepInputs, emit_return_publication_step,
};
use crate::sherlock_machine::return_publication_witness::produce_a102_literal_return_publication;
use crate::sherlock_machine::{LiteralEnvelope, LiteralImage};
use crate::{Error, LEDGER_REVISION, ZKIR_VERSION, error};
use midnight_curves::Bls12;
use midnight_proofs::poly::kzg::params::{ParamsKZG, ParamsVerifierKZG};
use midnight_proofs::utils::SerdeFormat;
use midnight_transient_crypto::curve::{Fr, outer};
use midnight_transient_crypto::proofs::{
    ParamsProver, ParamsProverProvider, VERIFIER_MAX_DEGREE, Zkir,
};
use midnight_zk_stdlib::{
    MidnightVK, prove as stdlib_prove, setup_pk, setup_vk, verify as stdlib_verify,
};
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use mori_core::sherlock::Program;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::Path, sync::Arc};

pub const RETURN_CHILD_FORMAT: &str = "moriarty-sherlock-a80-poseidon-child/1";
pub const RETURN_CHILD_BINDING_VERSION: &str = "moriarty-sherlock-a80-poseidon-child-binding/1";
pub const RETURN_STATEMENT_FORMAT: &str = "moriarty-sherlock-a80-return-publication-statement/1";
pub const RETURN_RELATION_DOMAIN: &str = "moriarty-sherlock-a80-relation/1";
pub const RETURN_CONTEXT_DOMAIN: &str = "moriarty-sherlock-a80-return-context/1";
pub const RETURN_SCOPE: &str =
    "one-uncharged-transition:A80-Return-to-StepDone-noncanonical-constant-space";
pub const RETURN_CHILD_QUALIFICATION: &str = "local-a80-poseidon-child-proof-unqualified";
pub const LINK_FORMAT: &str = "moriarty-sherlock-link/2";
pub const LINK_FIELD_ENCODING: &str = "bls12-381-scalar/32-byte-little-endian-lowercase-hex";
pub const LINK_PIN_AUTHORITY: &str = "vk_sha256 values pin a key only when reproduced by the consumer's own key reconstruction or an independently reviewed pin; this manifest and its digests are not approval";
pub const LINK_PROVENANCE: &str = "host-checked request and instance lane equalities over caller-supplied LinkedRequests; H1 single-Machine two-step observation is producer-asserted only when built by produce_linked_requests and is not authenticated by this manifest or verified in circuit";
pub const GENESIS_ROOT_AUTHORITY: &str = "consumer pinned Poseidon over a102 instance lanes [state_offset, state_offset + state_lanes); Init host subset only";
pub const LINK_MANIFEST_DISCLOSURES: [&str; 5] = [
    "LinkManifest::from_json and validate check only this document's fixed text, encodings, shape arithmetic and internal consistency; they read no sibling file, recompute no file digest and verify no claim, key, proof, instance or source",
    "file sha256 values are over the bytes written at export and identify them only; no digest here authenticates source, K, reference, program, carrier or key provenance",
    "children[0].carrier_source_sha256 is null because the A102 ChildClaim has no carrier field; children[1].carrier_source_sha256 is the A80 claim's own field; per-child open_obligations are the exact sealed ChildVerification lists",
    "genesis locates a102 instance pre lanes after a host subset of the Init equations on the request pre; it is not circuit Init and no Poseidon root is computed",
    "export is per-file create_new plus sync_all after an in-memory sealed verification; the directory is not fsynced, a failed export is not removed, link.json presence is not a durable commit marker, and child files have no reader profile or loader",
];
pub const LINK_EQUALITIES: [&str; 5] = [
    "children[0] = a102 and children[1] = a80 in fixed child order",
    "a102.context == LiteralLayout context and a80.context == ReturnPublicationLayout context",
    "a102.program == a80.program",
    "a102.post == a80.pre",
    "a102.envelope == (3, 1) and a80.envelope == (0, 0); work == 1",
];
pub const RETURN_OPEN_OBLIGATIONS: [&str; 4] = [
    "A80 pre and post are Step-mode Constant-space images, never canonical-boundary StateBlocks; standalone ReturnPublicationLayout::statement checks L4 shape, lane bounds and envelope (0, 0) without the private literal_host representation subset (covered in linked requests via A102 program/post equality); projectA, F3 and F5 are unimplemented",
    "the circuit enforces only the line-80 fragment under a forced enable; full StepInv/O1-O4, F4, OBL.G4 and the other 38 dispatch rows remain open",
    "the A80 context binds the caller-asserted LiteralSourceIdentity of the linked A102 request; the three digests are not authenticated",
    "crate-graph byte compatibility, consumer key pins and in-circuit verifier resources are unestablished, as for the A102 child",
];
pub const LINK_OPEN_OBLIGATIONS: [&str; 5] = [
    "every link equality here is host tooling; validity needs a reviewed parent that recursively verifies both children and constrains the same lanes and key pins in circuit",
    "binding lanes are SHA-256 claim commitments; neither child nor this module recomputes them in circuit",
    "neither child constrains Machine::new Init equations on the A102 pre; the genesis anchor is producer-asserted H1 data with a host subset check",
    "state roots, keys_id, trace and statement commitments, deferred pairing and subgroup checks, the terminal decider and anchoring are consumer work, unobserved",
    "k, rows, proof sizes, times and memory are unmeasured",
];
const STATEMENT_PREFIX: &str = "%sherlock.a80.statement";
const ENABLE_ID: &str = "%sherlock.a80.dispatch.enable";
const STEP_SCOPE: &str = "sherlock.a80.step";
const ADAPTER_SOURCE: &str = include_str!("sherlock_link.rs");
const CARRIER_SOURCE: &str = include_str!("sherlock_child.rs");

struct Hasher(Sha256);
impl Hasher {
    fn new(domain: &str) -> Self {
        let mut h = Self(Sha256::new());
        h.bytes(domain.as_bytes());
        h
    }
    fn bytes(&mut self, b: &[u8]) {
        self.0.update((b.len() as u64).to_le_bytes());
        self.0.update(b);
    }
    fn num(&mut self, n: u128) {
        self.0.update(n.to_le_bytes());
    }
    fn finish(self) -> [u8; 32] {
        self.0.finalize().into()
    }
}
fn imm(n: u128) -> Operand {
    Operand::Immediate(Fr::from(n))
}
fn scalars(values: &[Fr]) -> Vec<outer::Scalar> {
    values.iter().map(|f| f.0).collect()
}
fn sha(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}
fn json<T: Serialize>(value: &T) -> Result<Vec<u8>, Error> {
    serde_json::to_vec(value).map_err(|e| error("serialization", e.to_string()))
}
fn lane(name: &str, bits: u32, maximum: Option<u128>) -> LiteralLane {
    LiteralLane {
        name: name.into(),
        bits,
        boolean: false,
        maximum,
    }
}
fn image_lane_count(root: &[LiteralLane], tables: &[LiteralTable]) -> Option<usize> {
    tables.iter().try_fold(root.len(), |n, t| {
        n.checked_add(t.capacity.checked_mul(t.lanes.len())?)
    })
}
fn image_lanes(
    out: &mut Vec<(String, LiteralLane)>,
    block: &str,
    root: &[LiteralLane],
    tables: &[LiteralTable],
) {
    out.extend(root.iter().map(|l| (format!("{block}.root"), l.clone())));
    for t in tables {
        for r in 0..t.capacity {
            out.extend(
                t.lanes
                    .iter()
                    .map(|l| (format!("{block}.{}.{r}", t.name), l.clone())),
            );
        }
    }
}
fn take_image(
    it: &mut impl Iterator<Item = Operand>,
    root: &[LiteralLane],
    tables: &[LiteralTable],
) -> Result<LiteralImage, Error> {
    let mut take = |n: usize| -> Result<Vec<Operand>, Error> {
        let row: Vec<Operand> = (&mut *it).take(n).collect();
        if row.len() == n {
            Ok(row)
        } else {
            Err(error("sherlock-shape", "Statement operands ended early."))
        }
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

/// Value-independent A80 relation identity, distinct from the A102 `LiteralLayout`.
/// Statement: two context limbs, ProgramBlock, pre, post, rule_tag, charge; all public.
#[derive(Clone, Debug)]
pub struct ReturnPublicationLayout {
    schema: LiteralSchema,
    capacities: LiteralCapacities,
    identity: LiteralSourceIdentity,
    lanes: Vec<(String, LiteralLane)>,
    program_lanes: usize,
    state_lanes: usize,
    layout_hash: [u8; 32],
    relation_hash: [u8; 32],
    context: [u8; 32],
}
impl ReturnPublicationLayout {
    /// The checked lane count is admitted under the caller's explicit bound before
    /// any per-lane allocation, exactly as `LiteralLayout::new`.
    pub fn new(
        capacities: &LiteralCapacities,
        identity: &LiteralSourceIdentity,
        max_statement_lanes: usize,
    ) -> Result<Self, Error> {
        for d in [
            &identity.k_source_sha256,
            &identity.reference_source_sha256,
            &identity.program_source_sha256,
        ] {
            decode_digest(d)?;
        }
        let schema = literal_schema(capacities).map_err(construction)?;
        let (program_lanes, state_lanes, count) =
            image_lane_count(&schema.program_header, &schema.program_tables)
                .zip(image_lane_count(&schema.state_root, &schema.state_tables))
                .and_then(|(p, s)| Some((p, s, p.checked_add(s.checked_mul(2)?)?.checked_add(4)?)))
                .ok_or_else(|| error("limit", "Statement lane count overflows usize."))?;
        if count > max_statement_lanes {
            return Err(error(
                "limit",
                "Statement lane count exceeds the caller's explicit bound.",
            ));
        }
        let mut lanes = Vec::with_capacity(count);
        lanes.push(("context".to_string(), lane("limb.0", 128, None)));
        lanes.push(("context".to_string(), lane("limb.1", 128, None)));
        image_lanes(
            &mut lanes,
            "program",
            &schema.program_header,
            &schema.program_tables,
        );
        image_lanes(&mut lanes, "pre", &schema.state_root, &schema.state_tables);
        image_lanes(&mut lanes, "post", &schema.state_root, &schema.state_tables);
        lanes.push(("envelope".into(), lane("rule_tag", 6, Some(38))));
        lanes.push(("envelope".into(), lane("charge", 8, None)));
        if lanes.len() != count {
            return Err(error(
                "sherlock-shape",
                "Statement layout differs from its admitted lane count.",
            ));
        }
        let mut h = Hasher::new(RETURN_STATEMENT_FORMAT);
        for c in capacities.program.iter().chain(&capacities.state) {
            h.num(*c as u128);
        }
        h.num(capacities.stack_limit);
        for (path, lane) in &lanes {
            h.bytes(path.as_bytes());
            h.bytes(lane.name.as_bytes());
            h.num(lane.bits.into());
            h.num(lane.boolean.into());
            h.num(lane.maximum.is_some().into());
            h.num(lane.maximum.unwrap_or(0));
        }
        let layout_hash = h.finish();
        let mut h = Hasher::new(RETURN_RELATION_DOMAIN);
        for source in [
            include_str!("sherlock_machine/mod.rs"),
            include_str!("sherlock_machine/literal_host.rs"),
            include_str!("sherlock_machine/literal_step.rs"),
            include_str!("sherlock_machine/return_publication_step.rs"),
            ADAPTER_SOURCE,
        ] {
            h.bytes(source.as_bytes());
        }
        let relation_hash = h.finish();
        let mut h = Hasher::new(RETURN_CONTEXT_DOMAIN);
        for part in [
            RETURN_STATEMENT_FORMAT,
            RETURN_CHILD_BINDING_VERSION,
            PROFILE,
            RETURN_SCOPE,
            LEDGER_REVISION,
            ZKIR_VERSION,
            identity.k_source_sha256.as_str(),
            identity.reference_source_sha256.as_str(),
            identity.program_source_sha256.as_str(),
        ] {
            h.bytes(part.as_bytes());
        }
        for n in [
            RETURN_PUBLICATION_RULE_START_LINE,
            RETURN_PUBLICATION_RULE_TAG,
            RETURN_PUBLICATION_RULE_CHARGE,
        ] {
            h.num(n);
        }
        h.bytes(&layout_hash);
        h.bytes(&relation_hash);
        let context = h.finish();
        Ok(Self {
            schema,
            capacities: capacities.clone(),
            identity: identity.clone(),
            lanes,
            program_lanes,
            state_lanes,
            layout_hash,
            relation_hash,
            context,
        })
    }
    pub fn statement_len(&self) -> usize {
        self.lanes.len()
    }
    pub fn context_hex(&self) -> String {
        hex(&self.context)
    }
    pub fn limbs(&self) -> [u128; 2] {
        let (lo, hi) = self.context.split_at(16);
        [
            u128::from_le_bytes(lo.try_into().expect("16 bytes")),
            u128::from_le_bytes(hi.try_into().expect("16 bytes")),
        ]
    }
    pub fn record(&self) -> CapacityRecord {
        CapacityRecord {
            program: self.capacities.program.iter().map(|c| *c as u64).collect(),
            state: self.capacities.state.iter().map(|c| *c as u64).collect(),
            stack_limit: self.capacities.stack_limit.to_string(),
        }
    }
    /// Ordered public statement: shape, lane ranges and the (0, 0) envelope only.
    /// The relation itself is checked by the SDK and the circuit, never here.
    pub fn statement(&self, v: &LiteralValues) -> Result<Vec<u128>, Error> {
        let s = &self.schema;
        for (image, root, tables, block) in [
            (&v.program, &s.program_header, &s.program_tables, "program"),
            (&v.pre, &s.state_root, &s.state_tables, "pre"),
            (&v.post, &s.state_root, &s.state_tables, "post"),
        ] {
            let ok = image.root.len() == root.len()
                && image.tables.len() == tables.len()
                && image.tables.iter().zip(tables).all(|(rows, t)| {
                    rows.len() == t.capacity && rows.iter().all(|r| r.len() == t.lanes.len())
                });
            if !ok {
                return Err(error(
                    "sherlock-shape",
                    format!("{block} image differs from the L4 schema."),
                ));
            }
        }
        let mut out = self.limbs().to_vec();
        for image in [&v.program, &v.pre, &v.post] {
            out.extend(
                image
                    .root
                    .iter()
                    .chain(image.tables.iter().flatten().flatten())
                    .copied(),
            );
        }
        out.extend([v.rule_tag, v.charge]);
        if out.len() != self.lanes.len() {
            return Err(error(
                "sherlock-shape",
                "Statement length differs from the layout.",
            ));
        }
        for (value, (path, lane)) in out.iter().zip(&self.lanes) {
            let fits = if lane.boolean {
                *value <= 1
            } else {
                lane.bits >= 128 || *value >> lane.bits == 0
            };
            if !fits || lane.maximum.is_some_and(|m| *value > m) {
                return Err(error(
                    "sherlock-range",
                    format!("{path}.{} is outside its L4 lane.", lane.name),
                ));
            }
        }
        if v.rule_tag != RETURN_PUBLICATION_RULE_TAG || v.charge != RETURN_PUBLICATION_RULE_CHARGE {
            return Err(error(
                "sherlock-envelope",
                "Only wire tag 0 with charge 0 (line 80) is admitted.",
            ));
        }
        Ok(out)
    }
    /// LocalPublic relation in the shape of `LiteralLayout::emit(LocalPublic)`: bound
    /// statement, pinned context, forced A80 enable, the unmodified line-80 fragment
    /// and the Output terminator. No SDK execution.
    pub fn emit(&self) -> Result<(IrSource, String), Error> {
        let mut code = Vec::new();
        let mut operands = Vec::with_capacity(self.lanes.len());
        for position in 0..self.lanes.len() {
            let id = Identifier(format!("{STATEMENT_PREFIX}.{position}"));
            let operand = Operand::Variable(id.clone());
            code.push(Instruction::PublicInput {
                guard: None,
                val_t: IrType::Native,
                output: id,
            });
            code.push(Instruction::Impact {
                guard: imm(1),
                inputs: vec![operand.clone()],
            });
            operands.push(operand);
        }
        let [lo, hi] = self.limbs();
        code.push(Instruction::ConstrainEq {
            a: operands[0].clone(),
            b: imm(lo),
        });
        code.push(Instruction::ConstrainEq {
            a: operands[1].clone(),
            b: imm(hi),
        });
        let s = &self.schema;
        let mut rest = operands[2..].iter().cloned();
        let program = take_image(&mut rest, &s.program_header, &s.program_tables)?;
        let pre = take_image(&mut rest, &s.state_root, &s.state_tables)?;
        let post = take_image(&mut rest, &s.state_root, &s.state_tables)?;
        let (rule_tag, charge) = match (rest.next(), rest.next(), rest.next()) {
            (Some(t), Some(c), None) => (t, c),
            _ => {
                return Err(error(
                    "sherlock-shape",
                    "Envelope operands differ from the layout.",
                ));
            }
        };
        let enable_id = Identifier(ENABLE_ID.into());
        code.push(Instruction::TestEq {
            a: rule_tag.clone(),
            b: imm(RETURN_PUBLICATION_RULE_TAG),
            output: enable_id.clone(),
        });
        let enable = Operand::Variable(enable_id);
        code.push(Instruction::ConstrainToBoolean {
            val: enable.clone(),
        });
        code.push(Instruction::ConstrainEq {
            a: enable.clone(),
            b: imm(1),
        });
        let block = emit_return_publication_step(
            s,
            &ReturnPublicationStepInputs {
                enable,
                program,
                pre,
                post: post.clone(),
                envelope: LiteralEnvelope {
                    rule_tag: rule_tag.clone(),
                    charge: charge.clone(),
                },
            },
            STEP_SCOPE,
        )
        .map_err(construction)?;
        code.extend(block.instructions);
        let vals: Vec<Operand> = post
            .root
            .iter()
            .chain(post.tables.iter().flatten().flatten())
            .cloned()
            .chain([rule_tag, charge])
            .collect();
        let outputs = vec![IrType::Native; vals.len()];
        code.push(Instruction::Output { vals });
        let ir = IrSource {
            version: Default::default(),
            inputs: Vec::new(),
            outputs,
            do_communications_commitment: false,
            instructions: Arc::new(code),
        };
        let mut disk =
            serde_json::to_value(&ir).map_err(|e| error("serialization", e.to_string()))?;
        disk["version"] = serde_json::json!({"major": 3, "minor": 0});
        let zkir =
            serde_json::to_string(&disk).map_err(|e| error("serialization", e.to_string()))?;
        let parsed = IrSource::load(zkir.as_bytes()).map_err(|e| error("zkir", e.to_string()))?;
        if parsed != ir {
            return Err(error("zkir", "Pinned upstream round-trip changed the IR."));
        }
        Ok((ir, zkir))
    }
}

/// Complete A80 request: the linked A102 capacities and identity with A80 values.
#[derive(Clone, Debug)]
pub struct ReturnPublicationRequest {
    pub capacities: LiteralCapacities,
    pub identity: LiteralSourceIdentity,
    pub values: LiteralValues,
}

/// Exact host claim committed by the A80 child's binding lane.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReturnChildClaim {
    pub format: String,
    pub binding: String,
    pub transcript: String,
    pub statement_format: String,
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
    pub adapter_source_sha256: String,
    pub carrier_source_sha256: String,
    pub circuit_k: u8,
    /// Decimal statement lanes; the child instance is the binding followed by these.
    pub public_values: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct ReturnChildProof {
    pub claim: ReturnChildClaim,
    pub instance: Vec<Fr>,
    /// `MidnightVK::write(Processed)`.
    pub vk_bytes: Vec<u8>,
    pub proof: Vec<u8>,
}
impl ReturnChildProof {
    pub fn instance_hex(&self) -> Vec<String> {
        self.instance.iter().map(field_hex).collect()
    }
}

struct ReturnPrepared {
    ir: IrSource,
    claim: ReturnChildClaim,
    statement: Vec<u128>,
    instance: Vec<Fr>,
}
struct ReturnNonNativePrepared {
    ir: IrSource,
    claim_template: ReturnChildClaim,
    statement: Vec<u128>,
}
impl ReturnNonNativePrepared {
    fn claim_with_k(&self, circuit_k: u8) -> ReturnChildClaim {
        let mut claim = self.claim_template.clone();
        claim.circuit_k = circuit_k;
        claim
    }
    fn instance_for_claim(&self, claim: &ReturnChildClaim) -> Result<Vec<Fr>, Error> {
        let binding = return_binding(claim)?;
        Ok(std::iter::once(binding)
            .chain(self.statement.iter().map(|v| Fr::from(*v)))
            .collect())
    }
}
fn prepare_return_non_native(
    request: &ReturnPublicationRequest,
    options: &ProofOptions,
) -> Result<ReturnNonNativePrepared, Error> {
    if options.max_k == 0 || options.max_k > MAX_K {
        return Err(error("limit", "Native k cap must be between 1 and 19."));
    }
    let layout = ReturnPublicationLayout::new(
        &request.capacities,
        &request.identity,
        local_max_statement_lanes(),
    )?;
    let statement = layout.statement(&request.values)?;
    if statement
        .len()
        .checked_add(1)
        .is_none_or(|n| n > (1usize << VERIFIER_MAX_DEGREE))
    {
        return Err(error(
            "limit",
            "Public statement exceeds the pinned verifier capacity.",
        ));
    }
    let (ir, zkir) = layout.emit()?;
    let claim_template = ReturnChildClaim {
        format: RETURN_CHILD_FORMAT.into(),
        binding: RETURN_CHILD_BINDING_VERSION.into(),
        transcript: CHILD_TRANSCRIPT.into(),
        statement_format: RETURN_STATEMENT_FORMAT.into(),
        profile: PROFILE.into(),
        scope: RETURN_SCOPE.into(),
        ledger_revision: LEDGER_REVISION.into(),
        zkir_version: ZKIR_VERSION.into(),
        rule_start_line: RETURN_PUBLICATION_RULE_START_LINE as u32,
        rule_tag: RETURN_PUBLICATION_RULE_TAG as u32,
        charge: RETURN_PUBLICATION_RULE_CHARGE as u32,
        capacities: layout.record(),
        identity: layout.identity.clone(),
        layout_hash: hex(&layout.layout_hash),
        relation_hash: hex(&layout.relation_hash),
        context_hash: layout.context_hex(),
        zkir_hash: sha(zkir.as_bytes()),
        adapter_source_sha256: sha(ADAPTER_SOURCE.as_bytes()),
        carrier_source_sha256: sha(CARRIER_SOURCE.as_bytes()),
        // Widest admitted k until the native cost selection below.
        circuit_k: MAX_K,
        public_values: statement.iter().map(|v| v.to_string()).collect(),
    };
    crate::proving::validate_json_profile(&claim_template)?;
    Ok(ReturnNonNativePrepared {
        ir,
        claim_template,
        statement,
    })
}
fn finish_prepare_return(
    base: ReturnNonNativePrepared,
    options: &ProofOptions,
) -> Result<ReturnPrepared, Error> {
    // Native cost selection, never ordinary metadata.
    let circuit_k = base.ir.k();
    if circuit_k == 0 || circuit_k > options.max_k {
        return Err(error(
            "limit",
            format!("Circuit k={circuit_k} exceeds the explicit native k cap."),
        ));
    }
    let claim = base.claim_with_k(circuit_k);
    let instance = base.instance_for_claim(&claim)?;
    Ok(ReturnPrepared {
        ir: base.ir,
        claim,
        statement: base.statement,
        instance,
    })
}
fn prepare_return(
    request: &ReturnPublicationRequest,
    options: &ProofOptions,
) -> Result<ReturnPrepared, Error> {
    let base = prepare_return_non_native(request, options)?;
    finish_prepare_return(base, options)
}

/// The C3 binding arithmetic under the distinct A80 domain.
fn return_binding(claim: &ReturnChildClaim) -> Result<Fr, Error> {
    let hash = crate::digest(&(RETURN_CHILD_BINDING_VERSION, claim))?;
    let low = u128::from_le_bytes(hash[..16].try_into().expect("16 bytes"));
    let high = u128::from_le_bytes(hash[16..].try_into().expect("16 bytes"));
    Ok(Fr::from(low) + (Fr::from(u128::MAX) + Fr::from(1u64)) * Fr::from(high))
}
fn write_vk(vk: &MidnightVK) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();
    vk.write(&mut bytes, SerdeFormat::Processed)
        .map_err(|e| error("keygen", e.to_string()))?;
    Ok(bytes)
}

/// Reconstructed A80 circuit, statement and key; no supplied key is authority.
pub struct ReturnChildKey {
    prepared: ReturnPrepared,
    params: ParamsProver,
    vk: MidnightVK,
    vk_bytes: Vec<u8>,
    verifier: ParamsVerifierKZG<Bls12>,
}
impl ReturnChildKey {
    /// Native: k selection, pinned SRS load and `setup_vk`.
    pub fn reconstruct(
        request: &ReturnPublicationRequest,
        options: &ProofOptions,
    ) -> Result<Self, Error> {
        Self::from_prepared(prepare_return(request, options)?, options)
    }
    fn from_prepared(prepared: ReturnPrepared, options: &ProofOptions) -> Result<Self, Error> {
        let k = prepared.claim.circuit_k;
        let provider = OfflineParameters::load(&options.params_dir, k)?;
        let params = futures::executor::block_on(provider.get_params(k))
            .map_err(|e| error("parameters", e.to_string()))?;
        let kzg: &ParamsKZG<Bls12> = &params.0;
        let vk = setup_vk(kzg, &prepared.ir);
        if vk.k() != k {
            return Err(error(
                "keygen",
                "A80 child key degree differs from the circuit.",
            ));
        }
        let vk_bytes = write_vk(&vk)?;
        let reread = MidnightVK::read(&mut &vk_bytes[..], SerdeFormat::Processed)
            .map_err(|e| error("keygen", e.to_string()))?;
        if write_vk(&reread)? != vk_bytes {
            return Err(error("keygen", "A80 child key bytes do not round-trip."));
        }
        let verifier = kzg.verifier_params();
        Ok(Self {
            prepared,
            params,
            vk,
            vk_bytes,
            verifier,
        })
    }
    pub fn claim(&self) -> &ReturnChildClaim {
        &self.prepared.claim
    }
    pub fn instance(&self) -> &[Fr] {
        &self.prepared.instance
    }
    pub fn vk_bytes(&self) -> &[u8] {
        &self.vk_bytes
    }
    pub fn vk_sha256(&self) -> String {
        sha(&self.vk_bytes)
    }
    /// Stdlib verification under the child transcript.
    pub fn verify(&self, instance: &[Fr], proof: &[u8]) -> Result<(), Error> {
        if proof.is_empty() || proof.len() > MAX_PROOF_BYTES {
            return Err(error("proof", "Expected a nonempty bounded child proof."));
        }
        stdlib_verify::<IrSource, ChildTranscript>(
            &self.verifier,
            &self.vk,
            &scalars(instance),
            None,
            proof,
        )
        .map_err(|e| error("verification", format!("{e:?}")))
    }
    /// Native: SDK check under the real binding, C3 carrier, `setup_pk`, prove, self-verify.
    pub fn prove(&self) -> Result<ReturnChildProof, Error> {
        let p = &self.prepared;
        let pi_skips = check_child_statement(&p.ir, &p.statement, p.instance[0])?;
        let witness = child_witness(&p.ir, &p.instance, pi_skips)?;
        let pk = setup_pk(&p.ir, &self.vk);
        let kzg: &ParamsKZG<Bls12> = &self.params.0;
        let proof = stdlib_prove::<IrSource, ChildTranscript>(
            kzg,
            &pk,
            &p.ir,
            &scalars(&p.instance),
            witness,
            rand::rngs::OsRng,
        )
        .map_err(|e| error("proving", format!("{e:?}")))?;
        if proof.is_empty() || proof.len() > MAX_PROOF_BYTES {
            return Err(error(
                "limit",
                "Child proof exceeds the nonempty 262144-byte cap.",
            ));
        }
        self.verify(&p.instance, &proof)?;
        Ok(ReturnChildProof {
            claim: p.claim.clone(),
            instance: p.instance.clone(),
            vk_bytes: self.vk_bytes.clone(),
            proof,
        })
    }
}

/// Independent verification: proof bounds first, then claim and instance rebuilt and
/// compared before SRS, then the key rebuilt and compared bytewise, then stdlib verify.
pub fn verify_return_child(
    request: &ReturnPublicationRequest,
    child: &ReturnChildProof,
    options: &ProofOptions,
) -> Result<ChildVerification, Error> {
    if child.proof.is_empty() || child.proof.len() > MAX_PROOF_BYTES {
        return Err(error("proof", "Expected a nonempty bounded child proof."));
    }
    let prepared = prepare_return(request, options)?;
    if child.claim != prepared.claim {
        return Err(error(
            "claim",
            "Claim differs from the independently reconstructed A80 child statement.",
        ));
    }
    if child.instance != prepared.instance {
        return Err(error(
            "statement",
            "Child instance differs from the reconstructed binding and statement.",
        ));
    }
    let key = ReturnChildKey::from_prepared(prepared, options)?;
    if child.vk_bytes != key.vk_bytes {
        return Err(error(
            "keygen",
            "Exported child key differs from the reconstructed key.",
        ));
    }
    key.verify(&child.instance, &child.proof)?;
    let claim = key.claim();
    Ok(ChildVerification {
        qualification: RETURN_CHILD_QUALIFICATION,
        transcript: CHILD_TRANSCRIPT,
        context_hash: claim.context_hash.clone(),
        zkir_hash: claim.zkir_hash.clone(),
        adapter_source_sha256: claim.adapter_source_sha256.clone(),
        vk_sha256: key.vk_sha256(),
        circuit_k: claim.circuit_k,
        instance_lanes: child.instance.len(),
        proof_bytes: child.proof.len() as u32,
        open_obligations: RETURN_OPEN_OBLIGATIONS
            .iter()
            .chain(LINK_OPEN_OBLIGATIONS.iter())
            .copied()
            .collect(),
    })
}

/// Two consecutive transitions of one real Machine as two child requests.
#[derive(Clone, Debug)]
pub struct LinkedRequests {
    pub a102: LiteralTransitionRequest,
    pub a80: ReturnPublicationRequest,
}

/// Lane offsets inside each complete child instance (binding first).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LinkOffsets {
    pub binding: usize,
    pub context: usize,
    pub program: usize,
    pub pre: usize,
    pub post: usize,
    pub rule_tag: usize,
    pub charge: usize,
}

/// Capacity-derived shape shared by both children; n = P + 2S + 5.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkShape {
    pub program_lanes: usize,
    pub state_lanes: usize,
    pub instance_lanes: usize,
    pub a102_context: [u128; 2],
    pub a80_context: [u128; 2],
}
impl LinkShape {
    /// Checked offsets. Fields are public, so a shape whose arithmetic overflows or
    /// whose `instance_lanes` differs from P + 2S + 5 is refused, never sliced.
    pub fn offsets(&self) -> Result<LinkOffsets, Error> {
        let (p, s) = (self.program_lanes, self.state_lanes);
        let at = |base: usize, states: usize| {
            s.checked_mul(states)
                .and_then(|n| n.checked_add(p)?.checked_add(base))
                .ok_or_else(|| error("sherlock-link", "Link shape offsets overflow usize."))
        };
        let o = LinkOffsets {
            binding: 0,
            context: 1,
            program: 3,
            pre: at(3, 0)?,
            post: at(3, 1)?,
            rule_tag: at(3, 2)?,
            charge: at(4, 2)?,
        };
        if o.charge.checked_add(1) != Some(self.instance_lanes) {
            return Err(error(
                "sherlock-link",
                "Link shape instance length differs from P + 2S + 5.",
            ));
        }
        Ok(o)
    }
}

pub fn link_shape(a102: &LiteralLayout, a80: &ReturnPublicationLayout) -> Result<LinkShape, Error> {
    if a102.schema != a80.schema || a102.statement_len() != a80.statement_len() {
        return Err(error(
            "sherlock-link",
            "A102 and A80 layouts differ in schema or statement length.",
        ));
    }
    let shape = LinkShape {
        program_lanes: a80.program_lanes,
        state_lanes: a80.state_lanes,
        instance_lanes: a80
            .statement_len()
            .checked_add(1)
            .ok_or_else(|| error("limit", "Statement lane count overflows usize."))?,
        a102_context: a102.limbs(),
        a80_context: a80.limbs(),
    };
    shape.offsets()?;
    Ok(shape)
}

/// Host request linkage and admission of both statements by their own layouts.
pub fn check_linked_requests(
    r: &LinkedRequests,
) -> Result<(LiteralLayout, ReturnPublicationLayout, LinkShape), Error> {
    let refuse = |m: &str| error("sherlock-link", m.to_string());
    if r.a102.capacities != r.a80.capacities || r.a102.identity != r.a80.identity {
        return Err(refuse(
            "A102 and A80 requests differ in capacities or identity.",
        ));
    }
    if r.a102.values.program != r.a80.values.program {
        return Err(refuse("A102 and A80 ProgramBlock images differ."));
    }
    if r.a102.values.post != r.a80.values.pre {
        return Err(refuse("A102 post differs from A80 pre."));
    }
    let a102 = LiteralLayout::new(
        &r.a102.capacities,
        &r.a102.identity,
        local_max_statement_lanes(),
    )?;
    a102.statement(&r.a102.values)?;
    let a80 = ReturnPublicationLayout::new(
        &r.a80.capacities,
        &r.a80.identity,
        local_max_statement_lanes(),
    )?;
    a80.statement(&r.a80.values)?;
    let shape = link_shape(&a102, &a80)?;
    Ok((a102, a80, shape))
}

/// H1 host producer, no native work: one real Machine, two observed steps.
pub fn produce_linked_requests(
    program: Arc<Program>,
    capacities: &LiteralCapacities,
    initial_fuel: u128,
    stack_limit: usize,
    identity: &LiteralSourceIdentity,
) -> Result<(LinkedRequests, LinkShape), Error> {
    // Both layouts are admitted under the existing bound before prepare, Machine or
    // capacity-sized encoding. This does not bound H1 recursion or Program allocation.
    let admitted = link_shape(
        &LiteralLayout::new(capacities, identity, local_max_statement_lanes())?,
        &ReturnPublicationLayout::new(capacities, identity, local_max_statement_lanes())?,
    )?;
    let produced = produce_a102_literal_return_publication(
        program,
        capacities,
        initial_fuel,
        stack_limit,
        identity,
    )
    .map_err(|e| error("sherlock-witness", format!("{e:?}")))?;
    let a102 = produced.literal_transition.request.clone();
    if a102.capacities != *capacities || a102.identity != *identity {
        return Err(error(
            "sherlock-link",
            "Produced A102 request differs from the caller's capacities or identity.",
        ));
    }
    let a80 = ReturnPublicationRequest {
        capacities: capacities.clone(),
        identity: identity.clone(),
        values: produced.values,
    };
    let requests = LinkedRequests { a102, a80 };
    let (_, _, shape) = check_linked_requests(&requests)?;
    if shape != admitted {
        return Err(error(
            "sherlock-link",
            "Produced link shape differs from the pre-admitted layouts.",
        ));
    }
    Ok((requests, shape))
}

/// Nonproving SDK `ProofPreimage::check` of both relations with zero-binding fixtures;
/// no `ir.k()`, SRS, key or proof.
pub fn check_linked_relations(r: &LinkedRequests) -> Result<LinkShape, Error> {
    let (_, a80, shape) = check_linked_requests(r)?;
    check_literal_transition(&r.a102)?;
    let statement = a80.statement(&r.a80.values)?;
    let (ir, _) = a80.emit()?;
    check_child_statement(&ir, &statement, Fr::from(0u64))?;
    Ok(shape)
}

/// Host linkage of two complete child instances. A parent relation must enforce
/// the same equalities in circuit; this is tooling. `shape` is caller data: its
/// arithmetic is checked before any index, but its contexts are trusted, so only a
/// shape from `check_linked_requests` is meaningful.
pub fn check_link_lanes(a102: &[Fr], a80: &[Fr], shape: &LinkShape) -> Result<(), Error> {
    let refuse = |m: &str| error("sherlock-link", m.to_string());
    let o = shape.offsets()?;
    if a102.len() != shape.instance_lanes || a80.len() != shape.instance_lanes {
        return Err(refuse("Child instance length differs from the link shape."));
    }
    let limbs = |c: [u128; 2]| [Fr::from(c[0]), Fr::from(c[1])];
    if a102[o.context..o.program] != limbs(shape.a102_context)[..]
        || a80[o.context..o.program] != limbs(shape.a80_context)[..]
    {
        return Err(refuse(
            "A child context differs from its reconstructed layout.",
        ));
    }
    if a102[o.program..o.pre] != a80[o.program..o.pre] {
        return Err(refuse("A102 and A80 ProgramBlock lanes differ."));
    }
    if a102[o.post..o.rule_tag] != a80[o.pre..o.post] {
        return Err(refuse("A102 post lanes differ from A80 pre lanes."));
    }
    let envelope = |i: &[Fr], tag: u128, charge: u128| {
        i[o.rule_tag] == Fr::from(tag) && i[o.charge] == Fr::from(charge)
    };
    if !envelope(a102, RULE_TAG, RULE_CHARGE)
        || !envelope(
            a80,
            RETURN_PUBLICATION_RULE_TAG,
            RETURN_PUBLICATION_RULE_CHARGE,
        )
    {
        return Err(refuse(
            "Child envelopes differ from A102 (3, 1) then A80 (0, 0).",
        ));
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub struct LinkedChildren {
    pub a102: SherlockChildProof,
    pub a80: ReturnChildProof,
}

/// Native: both keys reconstructed, both children proved and self-verified, then linked.
pub fn prove_linked_children(
    r: &LinkedRequests,
    options: &ProofOptions,
) -> Result<LinkedChildren, Error> {
    let (_, _, shape) = check_linked_requests(r)?;
    let a102 = ChildKey::reconstruct(&r.a102, options)?.prove()?;
    let a80 = ReturnChildKey::reconstruct(&r.a80, options)?.prove()?;
    check_link_lanes(&a102.instance, &a80.instance, &shape)?;
    Ok(LinkedChildren { a102, a80 })
}

#[derive(Clone, Debug)]
pub struct LinkVerification {
    shape: LinkShape,
    a102: ChildVerification,
    a80: ChildVerification,
    seal: link_seal::Seal,
}

/// Host linkage before any SRS, then native independent verification of each child.
pub fn verify_linked_children(
    r: &LinkedRequests,
    c: &LinkedChildren,
    options: &ProofOptions,
) -> Result<LinkVerification, Error> {
    let (shape, a102, a80, seal) = link_seal::verify(r, c, options)?;
    Ok(LinkVerification {
        shape,
        a102,
        a80,
        seal,
    })
}
impl LinkVerification {
    pub fn shape(&self) -> LinkShape {
        self.shape
    }
    pub fn a102(&self) -> &ChildVerification {
        &self.a102
    }
    pub fn a80(&self) -> &ChildVerification {
        &self.a80
    }
}

/// Sole source of `LinkVerification` authority: a `Seal` exists only after both children
/// pass their independent cryptographic verification here, and commits to the exact
/// requests, claims, instances, processed keys, proofs and verification records of both
/// children in fixed order; export recomputes it. In-memory host authority only; not
/// approval of the shape, manifest, genesis, source identity or any parent relation.
mod link_seal {
    use super::{
        CHILD_QUALIFICATION, CHILD_TRANSCRIPT, ChildVerification, Error, Hasher, LinkShape,
        LinkedChildren, LinkedRequests, ProofOptions, RETURN_CHILD_QUALIFICATION, check_link_lanes,
        check_linked_requests, error, field_hex, sha, verify_return_child, verify_sherlock_child,
    };
    const SEAL_DOMAIN: &str = "moriarty-sherlock-link-verification-seal/1";

    #[derive(Clone, Debug)]
    pub(super) struct Seal([u8; 32]);
    impl Seal {
        pub(super) fn check(
            &self,
            r: &LinkedRequests,
            c: &LinkedChildren,
            a102: &ChildVerification,
            a80: &ChildVerification,
        ) -> Result<(), Error> {
            if commit(r, c, a102, a80)? != self.0 {
                return Err(error(
                    "sherlock-link",
                    "Requests, children or verification differ from the verified link.",
                ));
            }
            Ok(())
        }
    }

    pub(super) fn verify(
        r: &LinkedRequests,
        c: &LinkedChildren,
        options: &ProofOptions,
    ) -> Result<(LinkShape, ChildVerification, ChildVerification, Seal), Error> {
        let (_, _, shape) = check_linked_requests(r)?;
        check_link_lanes(&c.a102.instance, &c.a80.instance, &shape)?;
        let a102 = verify_sherlock_child(&r.a102, &c.a102, options)?;
        let a80 = verify_return_child(&r.a80, &c.a80, options)?;
        let seal = Seal(commit(r, c, &a102, &a80)?);
        Ok((shape, a102, a80, seal))
    }

    fn commit(
        r: &LinkedRequests,
        c: &LinkedChildren,
        a102: &ChildVerification,
        a80: &ChildVerification,
    ) -> Result<[u8; 32], Error> {
        let (l102, l80, _) = check_linked_requests(r)?;
        let (q, n) = (&c.a102.claim, &c.a80.claim);
        let rows = [
            (
                "a102",
                CHILD_QUALIFICATION,
                crate::digest(&(l102.record(), &r.a102.identity))?,
                l102.statement(&r.a102.values)?,
                crate::digest(q)?,
                (
                    &q.context_hash,
                    &q.zkir_hash,
                    &q.adapter_source_sha256,
                    q.circuit_k,
                ),
                &c.a102.instance,
                &c.a102.vk_bytes,
                &c.a102.proof,
                a102,
            ),
            (
                "a80",
                RETURN_CHILD_QUALIFICATION,
                crate::digest(&(l80.record(), &r.a80.identity))?,
                l80.statement(&r.a80.values)?,
                crate::digest(n)?,
                (
                    &n.context_hash,
                    &n.zkir_hash,
                    &n.adapter_source_sha256,
                    n.circuit_k,
                ),
                &c.a80.instance,
                &c.a80.vk_bytes,
                &c.a80.proof,
                a80,
            ),
        ];
        let mut h = Hasher::new(SEAL_DOMAIN);
        for (
            role,
            qualification,
            request,
            statement,
            claim,
            (context, zkir, adapter, k),
            instance,
            vk,
            proof,
            ver,
        ) in rows
        {
            if ver.qualification != qualification
                || ver.transcript != CHILD_TRANSCRIPT
                || ver.context_hash != *context
                || ver.zkir_hash != *zkir
                || ver.adapter_source_sha256 != *adapter
                || ver.circuit_k != k
                || ver.vk_sha256 != sha(vk)
                || ver.instance_lanes != instance.len()
                || ver.proof_bytes as usize != proof.len()
            {
                return Err(error(
                    "sherlock-link",
                    "A verification record differs from its child.",
                ));
            }
            for part in [role, qualification, CHILD_TRANSCRIPT] {
                h.bytes(part.as_bytes());
            }
            h.bytes(&request);
            h.num(statement.len() as u128);
            for v in &statement {
                h.num(*v);
            }
            h.bytes(&claim);
            h.num(instance.len() as u128);
            for lane in instance.iter() {
                h.bytes(field_hex(lane).as_bytes());
            }
            h.bytes(vk);
            h.bytes(proof);
            h.bytes(&crate::digest(ver)?);
        }
        Ok(h.finish())
    }
}

/// Genesis locator: height 0 and the A102 pre lanes inside the a102 instance. The root
/// is the consumer's pinned Poseidon over those lanes and is not computed here.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GenesisAnchor {
    pub height: u64,
    pub child: String,
    pub state_offset: usize,
    pub state_lanes: usize,
    pub root_authority: String,
}

fn check_init(schema: &LiteralSchema, pre: &ImageValues) -> Result<(), Error> {
    let refuse = |m: &str| error("sherlock-genesis", m.to_string());
    let root = |name: &str| {
        schema
            .state_root
            .iter()
            .position(|l| l.name == name)
            .and_then(|i| pre.root.get(i).copied())
            .ok_or_else(|| refuse("Missing L4 root lane."))
    };
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
        if root(name)? != 0 {
            return Err(refuse("A102 pre violates the Machine::new zero equations."));
        }
    }
    if root("controlLength")? != 1 {
        return Err(refuse("A102 pre needs controlLength 1."));
    }
    for b in 0..16 {
        if root(&format!("remainingFuel.{b}"))? != root(&format!("initialFuel.{b}"))? {
            return Err(refuse("A102 pre needs remainingFuel == initialFuel."));
        }
    }
    if pre.tables.iter().flatten().flatten().any(|c| *c != 0) {
        return Err(refuse(
            "A102 pre tables must be all zero for Eval(0) with empty arenas.",
        ));
    }
    Ok(())
}

/// Host subset of the Init equations on the A102 pre and its exact instance lanes.
pub fn anchor_genesis(
    r: &LinkedRequests,
    c: &LinkedChildren,
    shape: &LinkShape,
) -> Result<GenesisAnchor, Error> {
    let (a102, _, derived) = check_linked_requests(r)?;
    if *shape != derived {
        return Err(error(
            "sherlock-link",
            "Link shape differs from the reconstructed layouts.",
        ));
    }
    let o = shape.offsets()?;
    let pre = &r.a102.values.pre;
    check_init(&a102.schema, pre)?;
    let lanes: Vec<Fr> = pre
        .root
        .iter()
        .chain(pre.tables.iter().flatten().flatten())
        .map(|v| Fr::from(*v))
        .collect();
    if c.a102.instance.get(o.pre..o.post) != Some(&lanes[..]) {
        return Err(error(
            "sherlock-genesis",
            "A102 instance pre lanes differ from the request.",
        ));
    }
    Ok(GenesisAnchor {
        height: 0,
        child: "a102".into(),
        state_offset: o.pre,
        state_lanes: shape.state_lanes,
        root_authority: GENESIS_ROOT_AUTHORITY.into(),
    })
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LinkChild {
    pub name: String,
    pub claim_file: String,
    pub claim_sha256: String,
    pub vk_file: String,
    pub vk_sha256: String,
    pub proof_file: String,
    pub proof_sha256: String,
    pub instance_file: String,
    pub instance_sha256: String,
    pub qualification: String,
    pub claim_format: String,
    pub binding: String,
    pub instance_lanes: usize,
    pub proof_bytes: u32,
    pub context_hash: String,
    pub zkir_hash: String,
    pub adapter_source_sha256: String,
    /// The claimed carrier digest; `None` for a102, whose claim has no such field.
    #[serde(deserialize_with = "Option::deserialize")]
    pub carrier_source_sha256: Option<String>,
    pub circuit_k: u8,
    pub rule_tag: u32,
    pub charge: u32,
    pub open_obligations: Vec<String>,
}

/// Canonical `link.json` (schema `moriarty-sherlock-link/2`) for the Fenrir consumer.
/// `from_json` validates intrinsic consistency only; see `LINK_MANIFEST_DISCLOSURES`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LinkManifest {
    pub format: String,
    pub transcript: String,
    pub field_encoding: String,
    pub provenance: String,
    pub program_lanes: usize,
    pub state_lanes: usize,
    pub instance_lanes: usize,
    pub offsets: LinkOffsets,
    pub links: Vec<String>,
    pub children: Vec<LinkChild>,
    pub genesis: GenesisAnchor,
    pub pin_authority: String,
    pub open_obligations: Vec<String>,
    pub disclosures: Vec<String>,
}
impl LinkManifest {
    pub fn from_json(text: &str) -> Result<Self, Error> {
        let value = mori_core::json::parse(text).map_err(|e| error("json", e.to_string()))?;
        let manifest: Self =
            serde_json::from_value(value).map_err(|e| error("json", e.to_string()))?;
        manifest.validate()?;
        Ok(manifest)
    }
    /// Intrinsic consistency only: fixed text, checked shape and offsets, both children in
    /// fixed role order with deterministic filenames, digest encodings, envelopes, per-child
    /// obligations and the genesis locator. Reads no file; verifies no digest, claim or proof.
    pub fn validate(&self) -> Result<(), Error> {
        fn same(actual: &[String], expected: &[&str]) -> bool {
            actual
                .iter()
                .map(String::as_str)
                .eq(expected.iter().copied())
        }
        let refuse = |m: &str| error("sherlock-link", m.to_string());
        if self.format != LINK_FORMAT
            || self.transcript != CHILD_TRANSCRIPT
            || self.field_encoding != LINK_FIELD_ENCODING
            || self.provenance != LINK_PROVENANCE
            || self.pin_authority != LINK_PIN_AUTHORITY
            || !same(&self.links, &LINK_EQUALITIES)
            || !same(&self.open_obligations, &LINK_OPEN_OBLIGATIONS)
            || !same(&self.disclosures, &LINK_MANIFEST_DISCLOSURES)
        {
            return Err(refuse(
                "Manifest format, encoding, authority or disclosure text differs.",
            ));
        }
        // Offsets read only the lane counts; contexts are not part of this document.
        let shape = LinkShape {
            program_lanes: self.program_lanes,
            state_lanes: self.state_lanes,
            instance_lanes: self.instance_lanes,
            a102_context: [0; 2],
            a80_context: [0; 2],
        };
        if shape.offsets()? != self.offsets {
            return Err(refuse(
                "Manifest offsets differ from its checked lane counts.",
            ));
        }
        let g = &self.genesis;
        if g.height != 0
            || g.child != "a102"
            || g.state_offset != self.offsets.pre
            || g.state_lanes != self.state_lanes
            || g.root_authority != GENESIS_ROOT_AUTHORITY
        {
            return Err(refuse("Genesis locator differs from the a102 pre lanes."));
        }
        let roles = [
            (
                "a102",
                CHILD_QUALIFICATION,
                CHILD_FORMAT,
                CHILD_BINDING_VERSION,
                (RULE_TAG, RULE_CHARGE),
                false,
                OPEN_OBLIGATIONS
                    .iter()
                    .chain(&CHILD_OPEN_OBLIGATIONS)
                    .copied()
                    .collect::<Vec<_>>(),
            ),
            (
                "a80",
                RETURN_CHILD_QUALIFICATION,
                RETURN_CHILD_FORMAT,
                RETURN_CHILD_BINDING_VERSION,
                (RETURN_PUBLICATION_RULE_TAG, RETURN_PUBLICATION_RULE_CHARGE),
                true,
                RETURN_OPEN_OBLIGATIONS
                    .iter()
                    .chain(&LINK_OPEN_OBLIGATIONS)
                    .copied()
                    .collect(),
            ),
        ];
        if self.children.len() != roles.len() {
            return Err(refuse("Manifest needs exactly the a102 and a80 children."));
        }
        for (
            child,
            (name, qualification, claim_format, binding, (tag, charge), carrier, obligations),
        ) in self.children.iter().zip(roles)
        {
            let digests = [
                &child.claim_sha256,
                &child.vk_sha256,
                &child.proof_sha256,
                &child.instance_sha256,
                &child.context_hash,
                &child.zkir_hash,
                &child.adapter_source_sha256,
            ];
            if child.name != name
                || [
                    &child.claim_file,
                    &child.vk_file,
                    &child.proof_file,
                    &child.instance_file,
                ]
                .into_iter()
                .ne(&child_files(name))
                || child.qualification != qualification
                || child.claim_format != claim_format
                || child.binding != binding
                || child.carrier_source_sha256.is_some() != carrier
                || !digests
                    .into_iter()
                    .chain(&child.carrier_source_sha256)
                    .all(|d| decode_digest(d).is_ok())
                || child.instance_lanes != self.instance_lanes
                || child.proof_bytes == 0
                || child.proof_bytes as usize > MAX_PROOF_BYTES
                || child.circuit_k == 0
                || child.circuit_k > MAX_K
                || u128::from(child.rule_tag) != tag
                || u128::from(child.charge) != charge
                || !same(&child.open_obligations, &obligations)
            {
                return Err(refuse(
                    "A child entry differs from its fixed role, encoding or bounds.",
                ));
            }
        }
        Ok(())
    }
}

fn write_new(dir: &Path, name: &str, bytes: &[u8]) -> Result<String, Error> {
    let io = |e: std::io::Error| error("link-io", e.to_string());
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(dir.join(name))
        .map_err(io)?;
    file.write_all(bytes)
        .and_then(|_| file.sync_all())
        .map_err(io)?;
    Ok(sha(bytes))
}
fn child_files(name: &str) -> [String; 4] {
    ["claim.json", "vk", "proof", "instance.json"].map(|kind| format!("{name}.{kind}"))
}
/// Manifest entry over the exact bytes the writer persists, with metadata from the claim
/// and the sealed verification record. No filesystem effect.
#[allow(clippy::too_many_arguments)]
fn link_child(
    name: &str,
    claim: &[u8],
    vk: &[u8],
    proof: &[u8],
    instance: &[u8],
    v: &ChildVerification,
    (claim_format, binding, carrier): (&str, &str, Option<&str>),
    envelope: (u128, u128),
) -> LinkChild {
    let [claim_file, vk_file, proof_file, instance_file] = child_files(name);
    LinkChild {
        name: name.into(),
        claim_file,
        claim_sha256: sha(claim),
        vk_file,
        vk_sha256: sha(vk),
        proof_file,
        proof_sha256: sha(proof),
        instance_file,
        instance_sha256: sha(instance),
        qualification: v.qualification.into(),
        claim_format: claim_format.into(),
        binding: binding.into(),
        instance_lanes: v.instance_lanes,
        proof_bytes: v.proof_bytes,
        context_hash: v.context_hash.clone(),
        zkir_hash: v.zkir_hash.clone(),
        adapter_source_sha256: v.adapter_source_sha256.clone(),
        carrier_source_sha256: carrier.map(Into::into),
        circuit_k: v.circuit_k,
        rule_tag: envelope.0 as u32,
        charge: envelope.1 as u32,
        open_obligations: v.open_obligations.iter().map(|s| s.to_string()).collect(),
    }
}

/// Host only: persist a verified link. `dir` must not exist and no file is
/// overwritten; per child `<name>.claim.json`, `<name>.vk` (MidnightVK Processed),
/// `<name>.proof`, `<name>.instance.json` (lane hex), a102 then a80, `link.json` last.
pub fn write_link_dir(
    dir: &Path,
    r: &LinkedRequests,
    c: &LinkedChildren,
    v: &LinkVerification,
) -> Result<LinkManifest, Error> {
    let refuse = |m: &str| error("sherlock-link", m.to_string());
    let (_, _, shape) = check_linked_requests(r)?;
    if shape != v.shape {
        return Err(refuse("Verification shape differs from the requests."));
    }
    check_link_lanes(&c.a102.instance, &c.a80.instance, &shape)?;
    v.seal.check(r, c, &v.a102, &v.a80)?;
    for (vk, lanes, proof, ver) in [
        (
            &c.a102.vk_bytes,
            c.a102.instance.len(),
            &c.a102.proof,
            &v.a102,
        ),
        (&c.a80.vk_bytes, c.a80.instance.len(), &c.a80.proof, &v.a80),
    ] {
        if sha(vk) != ver.vk_sha256
            || lanes != ver.instance_lanes
            || proof.len() != ver.proof_bytes as usize
        {
            return Err(refuse("Children differ from the supplied verification."));
        }
    }
    let genesis = anchor_genesis(r, c, &shape)?;
    let (a102_claim, a80_claim) = (json(&c.a102.claim)?, json(&c.a80.claim)?);
    let (a102_instance, a80_instance) =
        (json(&c.a102.instance_hex())?, json(&c.a80.instance_hex())?);
    let a102_meta = (
        c.a102.claim.format.as_str(),
        c.a102.claim.binding.as_str(),
        None,
    );
    let a80_meta = (
        c.a80.claim.format.as_str(),
        c.a80.claim.binding.as_str(),
        Some(c.a80.claim.carrier_source_sha256.as_str()),
    );
    let children = vec![
        link_child(
            "a102",
            &a102_claim,
            &c.a102.vk_bytes,
            &c.a102.proof,
            &a102_instance,
            &v.a102,
            a102_meta,
            (RULE_TAG, RULE_CHARGE),
        ),
        link_child(
            "a80",
            &a80_claim,
            &c.a80.vk_bytes,
            &c.a80.proof,
            &a80_instance,
            &v.a80,
            a80_meta,
            (RETURN_PUBLICATION_RULE_TAG, RETURN_PUBLICATION_RULE_CHARGE),
        ),
    ];
    let manifest = LinkManifest {
        format: LINK_FORMAT.into(),
        transcript: CHILD_TRANSCRIPT.into(),
        field_encoding: LINK_FIELD_ENCODING.into(),
        provenance: LINK_PROVENANCE.into(),
        program_lanes: shape.program_lanes,
        state_lanes: shape.state_lanes,
        instance_lanes: shape.instance_lanes,
        offsets: shape.offsets()?,
        links: LINK_EQUALITIES.iter().map(|s| s.to_string()).collect(),
        children,
        genesis,
        pin_authority: LINK_PIN_AUTHORITY.into(),
        open_obligations: LINK_OPEN_OBLIGATIONS
            .iter()
            .map(|s| s.to_string())
            .collect(),
        disclosures: LINK_MANIFEST_DISCLOSURES
            .iter()
            .map(|s| s.to_string())
            .collect(),
    };
    // Intrinsic consistency and the existing reader profile before any filesystem effect.
    manifest.validate()?;
    crate::proving::validate_json_profile(&manifest)?;
    let link = json(&manifest)?;
    fs::create_dir(dir).map_err(|e| error("link-io", e.to_string()))?;
    for (child, files) in manifest.children.iter().zip([
        [&a102_claim, &c.a102.vk_bytes, &c.a102.proof, &a102_instance],
        [&a80_claim, &c.a80.vk_bytes, &c.a80.proof, &a80_instance],
    ]) {
        for (name, bytes) in [
            &child.claim_file,
            &child.vk_file,
            &child.proof_file,
            &child.instance_file,
        ]
        .into_iter()
        .zip(files)
        {
            write_new(dir, name, bytes)?;
        }
    }
    write_new(dir, "link.json", &link)?;
    Ok(manifest)
}

#[cfg(test)]
#[path = "sherlock_link_normal_tests.rs"]
mod normal_tests;
