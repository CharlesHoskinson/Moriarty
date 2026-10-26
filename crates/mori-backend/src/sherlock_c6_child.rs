//! Sherlock U023 recursion adapter: a C6 step leaf as a PoseidonState child.
//!
//! A C6 fragment realized through the `c6_step_harness` LocalPublic shape
//! (unguarded Native public inputs, each published by one unconditional
//! `Impact`, no raw inputs, no communications commitment) becomes a child in
//! the same form the Fenrir recl9 `inner::verify_offcircuit` and
//! `inner::verify_child_incircuit` consume: instance is the binding lane
//! followed by the complete statement, proved with the pinned stdlib generic
//! `prove` under `PoseidonState<Fq>`, exactly as the A102 producer in
//! `sherlock_child`. The export (`RecursionChildInput`) is the byte shape
//! `mori_recursion::child::SherlockChild::from_bytes` imports against an
//! independently pinned key digest: processed VK bytes, complete instance
//! lanes and proof bytes. Nothing is truncated or digested.
//!
//! Scope: one C6 step as a leaf. The current parent pins the A102/A80 pair;
//! a C6 leaf enters the decider only once a reviewed parent carries a C6
//! slot under pinned keys. That parent change, terminal validity and
//! stagenet qualification are separate work, not claimed here.
//! Compile/keygen/prove/verify: UNOBSERVED.
use crate::parameters::OfflineParameters;
use crate::proving::{MAX_K, MAX_PROOF_BYTES, ProofOptions, hex};
use crate::sherlock_child::{ChildTranscript, check_child_statement, child_witness, field_hex};
use crate::{Error, LEDGER_REVISION, ZKIR_VERSION, error};
use midnight_circuits::hash::poseidon::PoseidonState;
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
use midnight_zkir_v3::{Instruction, IrSource, Preprocessed};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const C6_CHILD_FORMAT: &str = "moriarty-sherlock-c6-poseidon-child/1";
pub const C6_CHILD_BINDING_VERSION: &str = "moriarty-sherlock-c6-poseidon-child-binding/1";
pub const C6_CHILD_TRANSCRIPT: &str = "midnight-zk-stdlib-2.3.5/PoseidonState<Fq>";
pub const C6_CHILD_QUALIFICATION: &str = "local-c6-poseidon-child-proof-unqualified";
pub const C6_CHILD_OPEN_OBLIGATIONS: [&str; 3] = [
    "the parent pins the A102/A80 pair; a C6 leaf reaches the decider only after a reviewed parent carries a C6 slot under independently pinned keys",
    "crate-graph byte compatibility (midnight-proofs 0.8.3 producer versus the consumer graph) and in-circuit verifier resources are unestablished, as for the A102 child",
    "a collapsed accumulator is not validity until the reviewed terminal fold and pairing check in the decider",
];
const ADAPTER_SOURCE: &str = include_str!("sherlock_c6_child.rs");

/// Exact host claim committed by the C6 child's binding lane.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct C6ChildClaim {
    pub format: String,
    pub binding: String,
    pub transcript: String,
    pub ledger_revision: String,
    pub zkir_version: String,
    /// Build-time constant naming the C6 rule this leaf realizes.
    pub rule_id: String,
    pub statement_lanes: usize,
    /// SHA-256 of the decimal statement lanes, in order.
    pub statement_hash: String,
    pub zkir_hash: String,
    pub adapter_source_sha256: String,
    pub circuit_k: u8,
}

/// One prepared leaf: claim, binding lane and complete public instance.
#[derive(Clone, Debug)]
pub struct C6Prepared {
    pub claim: C6ChildClaim,
    pub binding: Fr,
    /// Binding, then the complete C6 statement lanes.
    pub instance: Vec<Fr>,
    statement: Vec<u128>,
    ir: IrSource,
}

impl C6Prepared {
    /// The checked `Preprocessed` carrier for this leaf's relation.
    pub fn witness(&self) -> Result<Preprocessed, Error> {
        // pi_skips come from the SDK nonproving check under the real
        // binding, as for the A102 child.
        let pi_skips = check_child_statement(&self.ir, &self.statement, self.binding)?;
        child_witness(&self.ir, &self.instance, pi_skips)
    }
}

/// Byte shape imported by `mori_recursion::child::SherlockChild::from_bytes`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecursionChildInput {
    pub vk_sha256: String,
    pub instance_lanes: usize,
    pub instance_hex: Vec<String>,
    pub proof_bytes: usize,
}

/// Export a proved leaf for the recursion consumer. The consumer must pin
/// `vk_sha256` by its own key reconstruction; this value only identifies the
/// exported bytes, exactly as in the linked-children manifest.
pub fn recursion_child_input(
    prepared: &C6Prepared,
    vk_bytes: &[u8],
    proof: &[u8],
) -> RecursionChildInput {
    RecursionChildInput {
        vk_sha256: hex(&Sha256::digest(vk_bytes)),
        instance_lanes: prepared.instance.len(),
        instance_hex: prepared.instance.iter().map(field_hex).collect(),
        proof_bytes: proof.len(),
    }
}

fn scalars(values: &[Fr]) -> Vec<outer::Scalar> {
    values.iter().map(|f| f.0).collect()
}

/// SHA-256 of the domain-separated claim as low128 + 2^128 * high128, under a
/// C6-distinct domain, so a C6 binding never collides with an A102 binding.
pub(crate) fn c6_child_binding(claim: &C6ChildClaim) -> Result<Fr, Error> {
    let hash = crate::digest(&(C6_CHILD_BINDING_VERSION, claim))?;
    let low = u128::from_le_bytes(hash[..16].try_into().expect("16 bytes"));
    let high = u128::from_le_bytes(hash[16..].try_into().expect("16 bytes"));
    Ok(Fr::from(low) + (Fr::from(u128::MAX) + Fr::from(1u64)) * Fr::from(high))
}

/// Re-emit a harness-shaped relation in the A102 allocation order: each
/// public input immediately followed by its own unconditional publication.
/// The SDK groups consecutive public inputs into one `pi_skips` entry, so
/// the harness order (all inputs, then one joint `Impact`) cannot feed the
/// checked `child_witness` carrier, whose skips are per lane. Semantics are
/// preserved: definitions still precede every use, and the joint publication
/// must name exactly the assigned inputs in order.
fn interleave_statement(ir: &IrSource) -> Result<IrSource, Error> {
    use midnight_zkir_v3::ir::Operand;
    let refuse = |m: &str| error("sherlock-c6-child", m.to_string());
    let mut head = Vec::new();
    let mut rest = Vec::new();
    let mut assigned = Vec::new();
    let mut published: Option<Vec<Operand>> = None;
    for insn in ir.instructions.iter() {
        match insn {
            Instruction::PublicInput { output, .. } => {
                assigned.push(output.clone());
                head.push(insn.clone());
                head.push(Instruction::Impact {
                    guard: Operand::Immediate(Fr::from(1u64)),
                    inputs: vec![Operand::Variable(output.clone())],
                });
            }
            Instruction::Impact { guard, inputs } => {
                if *guard != Operand::Immediate(Fr::from(1u64)) || published.is_some() {
                    return Err(refuse("Only one unconditional publication is supported."));
                }
                published = Some(inputs.clone());
            }
            _ => rest.push(insn.clone()),
        }
    }
    let published = published.ok_or_else(|| refuse("Relation publishes no statement."))?;
    let want: Vec<Operand> = assigned
        .iter()
        .map(|id| Operand::Variable(id.clone()))
        .collect();
    if published != want {
        return Err(refuse(
            "Publication differs from the assigned statement inputs.",
        ));
    }
    head.extend(rest);
    Ok(IrSource {
        version: ir.version.clone(),
        inputs: ir.inputs.clone(),
        outputs: ir.outputs.clone(),
        do_communications_commitment: ir.do_communications_commitment,
        instructions: std::sync::Arc::new(head),
    })
}

fn refuse_private_or_guarded(ir: &IrSource) -> Result<(), Error> {
    if !ir.inputs.is_empty() || ir.do_communications_commitment {
        return Err(error(
            "sherlock-c6-child",
            "Only a LocalPublic relation without raw inputs or communications is supported.",
        ));
    }
    for insn in ir.instructions.iter() {
        if matches!(
            insn,
            Instruction::PrivateInput { .. } | Instruction::PublicInput { guard: Some(_), .. }
        ) {
            return Err(error(
                "sherlock-c6-child",
                "Guarded or private inputs are outside the C6 leaf carrier.",
            ));
        }
    }
    Ok(())
}

/// Prepare one C6 leaf from its realized relation and complete statement.
/// No key or proof work happens here.
pub fn prepare_c6_child(
    ir: &IrSource,
    rule_id: &str,
    statement: &[u128],
) -> Result<C6Prepared, Error> {
    if rule_id.trim().is_empty() {
        return Err(error("sherlock-c6-child", "Empty C6 rule id."));
    }
    if statement.is_empty() || statement.len() + 1 > (1usize << VERIFIER_MAX_DEGREE) {
        return Err(error(
            "limit",
            "Public statement exceeds the pinned verifier capacity.",
        ));
    }
    refuse_private_or_guarded(ir)?;
    let ir = interleave_statement(ir)?;
    let zkir_hash = hex(&Sha256::digest(
        serde_json::to_string(&ir).unwrap_or_default().as_bytes(),
    ));
    let statement_hash = hex(&Sha256::digest(
        statement
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect::<Vec<u8>>()
            .as_slice(),
    ));
    let mut claim = C6ChildClaim {
        format: C6_CHILD_FORMAT.into(),
        binding: C6_CHILD_BINDING_VERSION.into(),
        transcript: C6_CHILD_TRANSCRIPT.into(),
        ledger_revision: LEDGER_REVISION.into(),
        zkir_version: ZKIR_VERSION.into(),
        rule_id: rule_id.to_string(),
        statement_lanes: statement.len(),
        statement_hash,
        zkir_hash,
        adapter_source_sha256: hex(&Sha256::digest(ADAPTER_SOURCE.as_bytes())),
        circuit_k: MAX_K,
    };
    crate::proving::validate_json_profile(&claim)?;
    let circuit_k = ir.k();
    if circuit_k == 0 || circuit_k > MAX_K {
        return Err(error(
            "limit",
            format!("Circuit k={circuit_k} exceeds the explicit native k cap."),
        ));
    }
    claim.circuit_k = circuit_k;
    let binding = c6_child_binding(&claim)?;
    let instance = std::iter::once(binding)
        .chain(statement.iter().map(|v| Fr::from(*v)))
        .collect();
    Ok(C6Prepared {
        claim,
        binding,
        instance,
        statement: statement.to_vec(),
        ir: ir.clone(),
    })
}

/// Reconstructed C6 leaf key; no supplied key is authority.
pub struct C6ChildKey {
    prepared: C6Prepared,
    ir: IrSource,
    params: ParamsProver,
    vk: MidnightVK,
    vk_bytes: Vec<u8>,
    verifier: ParamsVerifierKZG<Bls12>,
}

impl C6ChildKey {
    /// Native: k selection, pinned SRS load and `setup_vk`.
    pub fn reconstruct(
        ir: IrSource,
        rule_id: &str,
        statement: &[u128],
        options: &ProofOptions,
    ) -> Result<Self, Error> {
        if options.max_k == 0 || options.max_k > MAX_K {
            return Err(error("limit", "Native k cap must be between 1 and 19."));
        }
        let prepared = prepare_c6_child(&ir, rule_id, statement)?;
        let ir = prepared.ir.clone();
        if prepared.claim.circuit_k > options.max_k {
            return Err(error(
                "limit",
                format!(
                    "Circuit k={} exceeds the explicit native k cap.",
                    prepared.claim.circuit_k
                ),
            ));
        }
        let k = prepared.claim.circuit_k;
        let provider = OfflineParameters::load(&options.params_dir, k)?;
        let params = futures::executor::block_on(provider.get_params(k))
            .map_err(|e| error("parameters", e.to_string()))?;
        let kzg: &ParamsKZG<Bls12> = &params.0;
        let vk = setup_vk(kzg, &ir);
        if vk.k() != k {
            return Err(error(
                "keygen",
                "C6 child key degree differs from the circuit.",
            ));
        }
        let vk_bytes = write_vk(&vk)?;
        let reread = MidnightVK::read(&mut &vk_bytes[..], SerdeFormat::Processed)
            .map_err(|e| error("keygen", e.to_string()))?;
        if write_vk(&reread)? != vk_bytes {
            return Err(error("keygen", "C6 child key bytes do not round-trip."));
        }
        let verifier = kzg.verifier_params();
        Ok(Self {
            prepared,
            ir,
            params,
            vk,
            vk_bytes,
            verifier,
        })
    }

    pub fn prepared(&self) -> &C6Prepared {
        &self.prepared
    }
    pub fn vk_bytes(&self) -> &[u8] {
        &self.vk_bytes
    }
    pub fn vk_sha256(&self) -> String {
        hex(&Sha256::digest(&self.vk_bytes))
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

    /// Native: SDK check under the real binding, carrier, `setup_pk`, prove,
    /// self-verify.
    pub fn prove(&self) -> Result<C6ChildProof, Error> {
        let p = &self.prepared;
        let pi_skips = check_child_statement(&self.ir, &p.statement, p.instance[0])?;
        let witness = child_witness(&self.ir, &p.instance, pi_skips)?;
        let pk = setup_pk(&self.ir, &self.vk);
        let kzg: &ParamsKZG<Bls12> = &self.params.0;
        let proof = stdlib_prove::<IrSource, PoseidonState<outer::Scalar>>(
            kzg,
            &pk,
            &self.ir,
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
        Ok(C6ChildProof {
            claim: p.claim.clone(),
            instance: p.instance.clone(),
            vk_bytes: self.vk_bytes.clone(),
            proof,
        })
    }
}

/// One produced C6 leaf: claim, complete public instance, processed key
/// bytes and proof, in the shape the recursion consumer imports.
#[derive(Clone, Debug)]
pub struct C6ChildProof {
    pub claim: C6ChildClaim,
    /// Binding, then the complete C6 statement lanes.
    pub instance: Vec<Fr>,
    /// `MidnightVK::write(Processed)`, the Fenrir `util::vk_from_bytes` form.
    pub vk_bytes: Vec<u8>,
    pub proof: Vec<u8>,
}

/// Section split of a harness statement: header+host prefix (program), then
/// pre lanes, then post lanes. Digests are over these sections in order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DigestSections {
    pub prog_lanes: usize,
    pub pre_lanes: usize,
    pub post_lanes: usize,
}

/// State outcome entering a state digest: tag 0 = running (value 0),
/// tag 1 = halted with the u128 result as value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StateOutcome {
    pub tag: u128,
    pub value: u128,
}
impl StateOutcome {
    pub const RUNNING: Self = Self { tag: 0, value: 0 };
    pub fn halted(result: u128) -> Self {
        Self {
            tag: 1,
            value: result,
        }
    }
}

/// `mori_recursion::parent::dom`: ASCII tag packed little-endian into a field.
pub fn dom(tag: &str) -> Fr {
    assert!(tag.len() <= 16, "domain tag longer than 16 bytes");
    let mut b = [0u8; 16];
    b[..tag.len()].copy_from_slice(tag.as_bytes());
    Fr::from(u128::from_le_bytes(b))
}

/// Numeric rule lane: low 128 bits of SHA-256 of the rule id.
pub fn rule_lane(rule_id: &str) -> Fr {
    let hash = Sha256::digest(rule_id.as_bytes());
    let low = u128::from_le_bytes(hash[..16].try_into().expect("16 bytes"));
    Fr::from(low)
}

/// Build the digest leaf relation from a harness-shaped C6 relation: every
/// statement lane becomes a private input, the fragment body is unchanged,
/// and the public statement is the five lanes
/// `[prog_d, pre_d, post_d, rule_id, h]` (in-circuit `TransientHash`
/// Poseidon over the full sections), each published in A102 order.
/// The binding lane precedes them in the child instance, as for A102.
pub fn c6_child_relation(
    ir: &IrSource,
    sections: DigestSections,
    pre_outcome: StateOutcome,
    post_outcome: StateOutcome,
) -> Result<IrSource, Error> {
    use midnight_zkir_v3::ir::Operand;
    let refuse = |m: &str| error("sherlock-c6-child", m.to_string());
    let total = sections
        .prog_lanes
        .checked_add(sections.pre_lanes)
        .and_then(|n| n.checked_add(sections.post_lanes))
        .ok_or_else(|| refuse("Section lane count overflows."))?;
    let mut private = Vec::new();
    let mut body = Vec::new();
    let mut published: Option<Vec<Operand>> = None;
    for insn in ir.instructions.iter() {
        match insn {
            Instruction::PublicInput {
                guard: None,
                val_t,
                output,
            } => {
                private.push(output.clone());
                body.push(Instruction::PrivateInput {
                    guard: None,
                    val_t: val_t.clone(),
                    output: output.clone(),
                });
            }
            Instruction::PublicInput { .. } => {
                return Err(refuse("Guarded public inputs are outside the digest leaf."));
            }
            Instruction::Impact { guard, inputs } => {
                if *guard != Operand::Immediate(Fr::from(1u64)) || published.is_some() {
                    return Err(refuse("Only one unconditional publication is supported."));
                }
                published = Some(inputs.clone());
            }
            _ => body.push(insn.clone()),
        }
    }
    let published = published.ok_or_else(|| refuse("Relation publishes no statement."))?;
    let want: Vec<Operand> = private
        .iter()
        .map(|id| Operand::Variable(id.clone()))
        .collect();
    if published != want || private.len() != total {
        return Err(refuse(
            "Publication or sections differ from the statement inputs.",
        ));
    }
    let section = |range: std::ops::Range<usize>| -> Vec<Operand> {
        private[range]
            .iter()
            .map(|id| Operand::Variable(id.clone()))
            .collect()
    };
    // In-circuit Poseidon accepts only a few inputs per TransientHash in
    // the ZKIR conversion (long single hashes fail synthesis), so every
    // digest is a tree of arity-4 hashes. The off-circuit mirror is
    // `hash_tree_offcircuit`; the node must use the same tree.
    fn hash_tree(
        body: &mut Vec<Instruction>,
        name: &str,
        level: usize,
        mut inputs: Vec<Operand>,
    ) -> Operand {
        if inputs.is_empty() {
            inputs.push(Operand::Immediate(Fr::from(0u64)));
        }
        if inputs.len() <= 4 {
            let output = midnight_zkir_v3::Identifier(format!("%c6.digest.{name}.{level}"));
            body.push(Instruction::TransientHash {
                inputs,
                output: output.clone(),
            });
            return Operand::Variable(output);
        }
        let mut next = Vec::new();
        for (i, chunk) in inputs.chunks(4).enumerate() {
            let output = midnight_zkir_v3::Identifier(format!("%c6.digest.{name}.{level}.{i}"));
            body.push(Instruction::TransientHash {
                inputs: chunk.to_vec(),
                output: output.clone(),
            });
            next.push(Operand::Variable(output));
        }
        hash_tree(body, name, level + 1, next)
    }
    let hash = |body: &mut Vec<Instruction>, name: &str, inputs: Vec<Operand>| -> Operand {
        hash_tree(body, name, 0, inputs)
    };
    // Digest constants are pinned private lanes, not immediates: circuit
    // synthesis rejects immediate operands inside TransientHash.
    let const_lane = |body: &mut Vec<Instruction>, name: &str, value: Fr| -> Operand {
        let id = midnight_zkir_v3::Identifier(format!("%c6.const.{name}"));
        body.push(Instruction::PrivateInput {
            guard: None,
            val_t: midnight_zkir_v3::ir_types::IrType::Native,
            output: id.clone(),
        });
        body.push(Instruction::ConstrainEq {
            a: Operand::Variable(id.clone()),
            b: Operand::Immediate(value),
        });
        Operand::Variable(id)
    };
    let prog_d = hash(&mut body, "prog", section(0..sections.prog_lanes));
    // lanes_d hashes the full carrier of one state (shared prefix + its lanes);
    // the state digest is Poseidon(dom("SHK_SD"), tag, value, lanes_d).
    let mut pre_carrier = section(0..sections.prog_lanes);
    pre_carrier.extend(section(
        sections.prog_lanes..sections.prog_lanes + sections.pre_lanes,
    ));
    let pre_lanes_d = hash(&mut body, "prelanes", pre_carrier);
    let pre_dom = const_lane(&mut body, "pre.dom", dom("SHK_SD"));
    let pre_tag = const_lane(&mut body, "pre.tag", Fr::from(pre_outcome.tag));
    let pre_val = const_lane(&mut body, "pre.value", Fr::from(pre_outcome.value));
    let pre_d = hash(
        &mut body,
        "pre",
        vec![pre_dom, pre_tag, pre_val, pre_lanes_d],
    );
    let mut post_carrier = section(0..sections.prog_lanes);
    post_carrier.extend(section(sections.prog_lanes + sections.pre_lanes..total));
    let post_lanes_d = hash(&mut body, "postlanes", post_carrier);
    let post_dom = const_lane(&mut body, "post.dom", dom("SHK_SD"));
    let post_tag = const_lane(&mut body, "post.tag", Fr::from(post_outcome.tag));
    let post_val = const_lane(&mut body, "post.value", Fr::from(post_outcome.value));
    let post_d = hash(
        &mut body,
        "post",
        vec![post_dom, post_tag, post_val, post_lanes_d],
    );
    // Public lanes come after the body: the VM executes in order, so the
    // digests must be computed before the ConstrainEq that ties them.
    let mut head = body;
    let mut tail = Vec::new();
    for (i, digest) in [prog_d, pre_d, post_d].into_iter().enumerate() {
        let id = midnight_zkir_v3::Identifier(format!("%c6.public.digest.{i}"));
        tail.push(Instruction::PublicInput {
            guard: None,
            val_t: midnight_zkir_v3::ir_types::IrType::Native,
            output: id.clone(),
        });
        tail.push(Instruction::Impact {
            guard: Operand::Immediate(Fr::from(1u64)),
            inputs: vec![Operand::Variable(id.clone())],
        });
        tail.push(Instruction::ConstrainEq {
            a: Operand::Variable(id),
            b: digest,
        });
    }
    for (i, lane) in ["rule", "h"].into_iter().enumerate() {
        let id = midnight_zkir_v3::Identifier(format!("%c6.public.{lane}"));
        tail.push(Instruction::PublicInput {
            guard: None,
            val_t: midnight_zkir_v3::ir_types::IrType::Native,
            output: id.clone(),
        });
        tail.push(Instruction::Impact {
            guard: Operand::Immediate(Fr::from(1u64)),
            inputs: vec![Operand::Variable(id)],
        });
        let _ = i;
    }
    head.extend(tail);
    Ok(IrSource {
        version: ir.version.clone(),
        inputs: Vec::new(),
        outputs: Vec::new(),
        do_communications_commitment: false,
        instructions: std::sync::Arc::new(head),
    })
}

/// Off-circuit digests of one statement, exactly the in-circuit
/// `TransientHash` values (Poseidon over the section lanes).
pub fn statement_digests(
    statement: &[u128],
    sections: DigestSections,
    pre_outcome: StateOutcome,
    post_outcome: StateOutcome,
) -> Result<[Fr; 3], Error> {
    use midnight_circuits::hash::poseidon::PoseidonChip;
    use midnight_circuits::instructions::hash::HashCPU;
    let total = sections.prog_lanes + sections.pre_lanes + sections.post_lanes;
    if statement.len() != total {
        return Err(error(
            "sherlock-c6-child",
            "Statement length differs from the digest sections.",
        ));
    }
    let lanes: Vec<outer::Scalar> = statement.iter().map(|v| Fr::from(*v).0).collect();
    let hash = |xs: &[outer::Scalar]| {
        Fr(<PoseidonChip<outer::Scalar> as HashCPU<
            outer::Scalar,
            outer::Scalar,
        >>::hash(xs))
    };
    // Arity-4 tree mirror of the in-circuit digest (see c6_child_relation).
    let hash_tree = |xs: &[outer::Scalar]| -> Fr {
        let mut level: Vec<outer::Scalar> = if xs.is_empty() {
            vec![outer::Scalar::from(0u64)]
        } else {
            xs.to_vec()
        };
        loop {
            if level.len() <= 4 {
                return hash(&level);
            }
            level = level.chunks(4).map(|chunk| hash(chunk).0).collect();
        }
    };
    let lanes_d = |range: std::ops::Range<usize>| {
        let mut carrier: Vec<outer::Scalar> = lanes[..sections.prog_lanes].to_vec();
        carrier.extend(&lanes[range]);
        hash_tree(&carrier)
    };
    let state_d = |outcome: StateOutcome, lanes_d: Fr| {
        hash(&[
            dom("SHK_SD").0,
            Fr::from(outcome.tag).0,
            Fr::from(outcome.value).0,
            lanes_d.0,
        ])
    };
    Ok([
        hash_tree(&lanes[..sections.prog_lanes]),
        state_d(
            pre_outcome,
            lanes_d(sections.prog_lanes..sections.prog_lanes + sections.pre_lanes),
        ),
        state_d(
            post_outcome,
            lanes_d(sections.prog_lanes + sections.pre_lanes..total),
        ),
    ])
}

/// One prepared digest leaf: instance is binding + the five public lanes.
#[derive(Clone, Debug)]
pub struct C6DigestPrepared {
    pub claim: C6ChildClaim,
    pub binding: Fr,
    pub instance: Vec<Fr>,
    pub public_statement: [Fr; 5],
    statement: Vec<u128>,
    pre_outcome: StateOutcome,
    post_outcome: StateOutcome,
    ir: IrSource,
}

impl C6DigestPrepared {
    /// Checked carrier: SDK nonproving check (private transcript is the full
    /// statement, public transcript the five lanes), then the `Preprocessed`
    /// the stdlib prover reads.
    pub fn witness(&self) -> Result<Preprocessed, Error> {
        use midnight_transient_crypto::proofs::{KeyLocation, ProofPreimage};
        use midnight_zkir_v3::ir_types::IrValue;
        use std::collections::HashMap;
        let private_transcript: Vec<Fr> = self.private_lanes_in_order();
        let preimage = ProofPreimage {
            inputs: Vec::new(),
            private_transcript,
            public_transcript_inputs: self.public_statement.to_vec(),
            public_transcript_outputs: self.public_statement.to_vec(),
            binding_input: self.binding,
            communications_commitment: None,
            key_location: KeyLocation("moriarty-sherlock-c6-poseidon-child-check/1".into()),
        };
        let pi_skips = preimage
            .check(&self.ir)
            .map_err(|e| error("proving", e.to_string()))?;
        if pi_skips.len() != 5 || pi_skips.iter().any(Option::is_some) {
            return Err(error(
                "statement",
                "Nonproving check returned a different or skipped public statement.",
            ));
        }
        let mut memory = HashMap::with_capacity(self.statement.len() + 11);
        // Public lanes are also read from witness memory by the circuit.
        for (i, name) in ["digest.0", "digest.1", "digest.2", "rule", "h"]
            .iter()
            .enumerate()
        {
            memory.insert(
                midnight_zkir_v3::Identifier(format!("%c6.public.{name}")),
                IrValue::Native(self.public_statement[i]),
            );
        }
        for (id, value) in self.private_assignments() {
            if memory.insert(id, IrValue::Native(value)).is_some() {
                return Err(error("sherlock-c6-child", "Repeated private identifier."));
            }
        }
        Ok(Preprocessed {
            memory,
            pis: scalars(&self.instance),
            pi_skips,
            binding_input: self.binding.0,
            comm_comm: None,
        })
    }
}

impl C6DigestPrepared {
    /// Private values in the relation's PrivateInput order: statement lanes
    /// for the carrier inputs, pinned constants for `%c6.const.*` lanes.
    fn private_assignments(&self) -> Vec<(midnight_zkir_v3::Identifier, Fr)> {
        let consts = [
            ("pre.dom", dom("SHK_SD")),
            ("pre.tag", Fr::from(self.pre_outcome.tag)),
            ("pre.value", Fr::from(self.pre_outcome.value)),
            ("post.dom", dom("SHK_SD")),
            ("post.tag", Fr::from(self.post_outcome.tag)),
            ("post.value", Fr::from(self.post_outcome.value)),
        ];
        let mut lanes = self.statement.iter();
        let mut out = Vec::new();
        for insn in self.ir.instructions.iter() {
            if let Instruction::PrivateInput { output, .. } = insn {
                let value = if let Some(name) = output.0.strip_prefix("%c6.const.") {
                    consts
                        .iter()
                        .find(|(n, _)| *n == name)
                        .map(|(_, v)| *v)
                        .expect("const lane name")
                } else {
                    Fr::from(*lanes.next().expect("statement covers private inputs"))
                };
                out.push((output.clone(), value));
            }
        }
        out
    }

    fn private_lanes_in_order(&self) -> Vec<Fr> {
        self.private_assignments()
            .into_iter()
            .map(|(_, v)| v)
            .collect()
    }
}

/// Prepare one digest leaf. No key or proof work happens here.
#[allow(clippy::too_many_arguments)]
pub fn prepare_digest_c6_child(
    ir: &IrSource,
    rule_id: &str,
    statement: &[u128],
    sections: DigestSections,
    pre_outcome: StateOutcome,
    post_outcome: StateOutcome,
    height: u64,
) -> Result<C6DigestPrepared, Error> {
    if rule_id.trim().is_empty() {
        return Err(error("sherlock-c6-child", "Empty C6 rule id."));
    }
    refuse_private_or_guarded(ir)?;
    let relation = c6_child_relation(ir, sections, pre_outcome, post_outcome)?;
    let digests = statement_digests(statement, sections, pre_outcome, post_outcome)?;
    let public_statement = [
        digests[0],
        digests[1],
        digests[2],
        rule_lane(rule_id),
        Fr::from(height),
    ];
    let base = prepare_c6_child(ir, rule_id, statement)?;
    // The digest relation, not the raw-lane relation, sets the circuit degree.
    let mut claim = base.claim;
    claim.circuit_k = relation.k();
    let binding = c6_child_binding(&claim)?;
    // The ZKIR circuit pushes the binding as its first public input (as
    // for A102), so the instance is the binding followed by the five lanes
    // the node consumes.
    let instance = std::iter::once(binding)
        .chain(public_statement.iter().copied())
        .collect();
    Ok(C6DigestPrepared {
        claim,
        binding,
        instance,
        public_statement,
        statement: statement.to_vec(),
        pre_outcome,
        post_outcome,
        ir: relation,
    })
}

/// Reconstructed digest-leaf key; no supplied key is authority.
pub struct C6DigestKey {
    prepared: C6DigestPrepared,
    params: ParamsProver,
    vk: MidnightVK,
    vk_bytes: Vec<u8>,
    verifier: ParamsVerifierKZG<Bls12>,
}

impl C6DigestKey {
    #[allow(clippy::too_many_arguments)]
    pub fn reconstruct(
        ir: &IrSource,
        rule_id: &str,
        statement: &[u128],
        sections: DigestSections,
        pre_outcome: StateOutcome,
        post_outcome: StateOutcome,
        height: u64,
        options: &ProofOptions,
    ) -> Result<Self, Error> {
        if options.max_k == 0 || options.max_k > MAX_K {
            return Err(error("limit", "Native k cap must be between 1 and 19."));
        }
        let prepared = prepare_digest_c6_child(
            ir,
            rule_id,
            statement,
            sections,
            pre_outcome,
            post_outcome,
            height,
        )?;
        if prepared.claim.circuit_k > options.max_k {
            return Err(error(
                "limit",
                format!(
                    "Circuit k={} exceeds the explicit native k cap.",
                    prepared.claim.circuit_k
                ),
            ));
        }
        let k = prepared.claim.circuit_k;
        let provider = OfflineParameters::load(&options.params_dir, k)?;
        let params = futures::executor::block_on(provider.get_params(k))
            .map_err(|e| error("parameters", e.to_string()))?;
        let kzg: &ParamsKZG<Bls12> = &params.0;
        let vk = setup_vk(kzg, &prepared.ir);
        if vk.k() != k {
            return Err(error(
                "keygen",
                "C6 child key degree differs from the circuit.",
            ));
        }
        let vk_bytes = write_vk(&vk)?;
        let reread = MidnightVK::read(&mut &vk_bytes[..], SerdeFormat::Processed)
            .map_err(|e| error("keygen", e.to_string()))?;
        if write_vk(&reread)? != vk_bytes {
            return Err(error("keygen", "C6 child key bytes do not round-trip."));
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

    pub fn prepared(&self) -> &C6DigestPrepared {
        &self.prepared
    }
    pub fn vk_bytes(&self) -> &[u8] {
        &self.vk_bytes
    }
    pub fn vk_sha256(&self) -> String {
        hex(&Sha256::digest(&self.vk_bytes))
    }

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

    pub fn prove(&self) -> Result<C6ChildProof, Error> {
        let p = &self.prepared;
        let witness = p.witness()?;
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
        Ok(C6ChildProof {
            claim: p.claim.clone(),
            instance: p.instance.clone(),
            vk_bytes: self.vk_bytes.clone(),
            proof,
        })
    }
}

/// Prove one C6 digest leaf (gpr G4 interface).
#[allow(clippy::too_many_arguments)]
pub fn prove_c6_child(
    ir: &IrSource,
    rule_id: &str,
    statement: &[u128],
    sections: DigestSections,
    pre_outcome: StateOutcome,
    post_outcome: StateOutcome,
    height: u64,
    options: &ProofOptions,
) -> Result<C6ChildProof, Error> {
    C6DigestKey::reconstruct(
        ir,
        rule_id,
        statement,
        sections,
        pre_outcome,
        post_outcome,
        height,
        options,
    )?
    .prove()
}

/// Verify one C6 digest leaf against an independently reconstructed key.
#[allow(clippy::too_many_arguments)]
pub fn verify_c6_child(
    ir: &IrSource,
    rule_id: &str,
    statement: &[u128],
    sections: DigestSections,
    pre_outcome: StateOutcome,
    post_outcome: StateOutcome,
    height: u64,
    child: &C6ChildProof,
    options: &ProofOptions,
) -> Result<(), Error> {
    if child.proof.is_empty() || child.proof.len() > MAX_PROOF_BYTES {
        return Err(error("proof", "Expected a nonempty bounded child proof."));
    }
    let key = C6DigestKey::reconstruct(
        ir,
        rule_id,
        statement,
        sections,
        pre_outcome,
        post_outcome,
        height,
        options,
    )?;
    if child.claim != key.prepared.claim || child.instance != key.prepared.instance {
        return Err(error(
            "statement",
            "Child differs from the independently reconstructed C6 leaf.",
        ));
    }
    if child.vk_bytes != key.vk_bytes {
        return Err(error(
            "keygen",
            "Exported child key differs from the reconstructed key.",
        ));
    }
    key.verify(&child.instance, &child.proof)
}

/// Export one proved leaf as the files the node consumes: `leaf.vk`
/// (Processed MidnightVK bytes), `leaf.proof`, and `leaf.instance.json`
/// (the five instance lanes as 32-byte little-endian hex, the
/// `mori_recursion::util::f_to_hex` form). Files are created new; an
/// existing export is never overwritten.
pub fn write_leaf_files(dir: &std::path::Path, child: &C6ChildProof) -> Result<(), Error> {
    use std::io::Write;
    if child.instance.len() != 6 {
        return Err(error(
            "sherlock-c6-child",
            "Leaf instance must be the binding plus five lanes for export.",
        ));
    }
    std::fs::create_dir_all(dir).map_err(|e| error("export", e.to_string()))?;
    let write_new = |name: &str, bytes: &[u8]| -> Result<(), Error> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(dir.join(name))
            .map_err(|e| error("export", e.to_string()))?;
        file.write_all(bytes)
            .map_err(|e| error("export", e.to_string()))?;
        file.sync_all().map_err(|e| error("export", e.to_string()))
    };
    write_new("leaf.vk", &child.vk_bytes)?;
    write_new("leaf.proof", &child.proof)?;
    // The node-facing file carries the five statement lanes (no binding).
    let instance: Vec<String> = child.instance[1..].iter().map(field_hex).collect();
    let json = serde_json::to_vec(&instance).map_err(|e| error("export", e.to_string()))?;
    write_new("leaf.instance.json", &json)
}

fn write_vk(vk: &MidnightVK) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();
    vk.write(&mut bytes, SerdeFormat::Processed)
        .map_err(|e| error("keygen", e.to_string()))?;
    Ok(bytes)
}
