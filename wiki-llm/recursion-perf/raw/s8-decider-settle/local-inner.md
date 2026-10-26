# Local mori-recursion inner.rs
> Source: file:///home/charl/code/Moriarty-perf/crates/mori-recursion/src/inner.rs
> Collected: 2026-10-26
> Published: Unknown

// Adapted from projectfenrir recursion/stagenet-strawman/recl9.
// Uses the pinned stock 0.8.3 graph; terminal validity and gadget soundness remain separate.
//! Verifying a child proof (Poseidon transcript) in circuit and off circuit,
//! with the midnight-circuits 7.2.4 verifier gadget. Mirrors the ledger-10
//! `verify_proof` + `DeciderKind::{None, Collapsed}` code
//! (zkir-v3/src/decider.rs, ir_instructions/verify_proof.rs), which runs on
//! the same released crates.

use std::collections::BTreeMap;

use ff::Field;
use group::Group;
use midnight_circuits::{
    hash::poseidon::PoseidonState,
    instructions::{AssertionInstructions, AssignmentInstructions, PublicInputInstructions},
    types::{AssignedForeignPoint, AssignedNative, Instantiable},
    verifier::{
        Accumulator, AssignedAccumulator, BlstrsEmulation, Msm, SelfEmulation, fixed_bases,
    },
};
use midnight_curves::Bls12;
use midnight_proofs::{
    circuit::{Layouter, Value},
    plonk::{self, Error},
    poly::kzg::{KZGCommitmentScheme, params::ParamsVerifierKZG},
    transcript::{CircuitTranscript, Transcript},
};
use midnight_zk_stdlib::{MidnightVK, ZkStdLib};

use crate::util::{F, sha256_hex, vk_bytes};

pub type S = BlstrsEmulation;
pub type C = <S as SelfEmulation>::C;
pub type AP = <S as SelfEmulation>::AssignedPoint;

/// Number of fields of a resolved, collapsed accumulator (two points, two
/// scalars): 12.
pub fn acc_pi_len() -> usize {
    acc_pis(&Accumulator::<S>::trivial(&[])).len()
}

pub fn acc_pis(acc: &Accumulator<S>) -> Vec<F> {
    <AssignedAccumulator<S> as Instantiable<F>>::as_public_input(acc)
}

/// Fields of one G1 point in the stdlib public-input encoding: 5.
pub fn point_pi_len() -> usize {
    (acc_pi_len() / 2) - 1
}

/// Prefix of the fixed-base names of a key: derived from its bytes.
pub fn vk_name(prefix: &str, vk: &MidnightVK) -> String {
    format!("{prefix}_{}", &sha256_hex(&vk_bytes(vk))[..16])
}

/// Inverse of `acc_pis` for a collapsed accumulator; checks the field count,
/// that both points decode into the prime-order subgroup and that both side
/// scalars are one.
pub fn reconstruct_acc(fields: &[F]) -> Option<Accumulator<S>> {
    if fields.len() != acc_pi_len() {
        return None;
    }
    let half = fields.len() / 2;
    let side = |s: &[F]| -> Option<Msm<S>> {
        let (pt, sc) = s.split_at(s.len() - 1);
        if sc[0] != F::ONE {
            return None;
        }
        let base = <AP as Instantiable<F>>::from_public_input(pt)?;
        Some(Msm::new(&[base], &[sc[0]], &BTreeMap::new()))
    };
    Some(Accumulator::new(
        side(&fields[..half])?,
        side(&fields[half..])?,
    ))
}

/// Off-circuit: prepare `proof` against `instance`, fold the carried
/// accumulator (last 12 instance fields) if `carried`, resolve the fixed
/// bases and collapse. The pairing is not checked here.
pub fn prepare_offcircuit(
    vk: &MidnightVK,
    name: &str,
    instance: &[F],
    proof: &[u8],
    carried: bool,
) -> Result<Accumulator<S>, String> {
    prepare_offcircuit_with(vk, name, instance, proof, carried, true)
}

/// As `prepare_offcircuit`, optionally leaving the (resolved) accumulator uncollapsed,
/// mirroring `verify_child_incircuit_with(.., collapse = false)`.
pub fn prepare_offcircuit_with(
    vk: &MidnightVK,
    name: &str,
    instance: &[F],
    proof: &[u8],
    carried: bool,
    collapse: bool,
) -> Result<Accumulator<S>, String> {
    let bases = fixed_bases::<S>(name, vk.vk());
    let mut tr = CircuitTranscript::<PoseidonState<F>>::init_from_bytes(proof);
    let dual =
        plonk::prepare::<F, KZGCommitmentScheme<Bls12>, CircuitTranscript<PoseidonState<F>>>(
            vk.vk(),
            &[&[C::identity()]],
            &[&[instance]],
            &mut tr,
        )
        .map_err(|e| format!("prepare: {e:?}"))?;
    let own = Accumulator::<S>::from_dual_msm(dual, name, &bases);
    let mut acc = if carried {
        let tail = &instance[instance.len() - acc_pi_len()..];
        let c = reconstruct_acc(tail).ok_or("carried accumulator malformed")?;
        Accumulator::accumulate(&[own, c])
    } else {
        own
    };
    acc.resolve_fixed_bases(&bases);
    if collapse {
        acc.collapse();
    }
    Ok(acc)
}

/// Off-circuit full verification (prepare, fold, one pairing).
pub fn verify_offcircuit(
    params: &ParamsVerifierKZG<Bls12>,
    vk: &MidnightVK,
    name: &str,
    instance: &[F],
    proof: &[u8],
    carried: bool,
) -> Result<Accumulator<S>, String> {
    let acc = prepare_offcircuit(vk, name, instance, proof, carried)?;
    if acc.check(params, &BTreeMap::new()) {
        Ok(acc)
    } else {
        Err("pairing check failed".into())
    }
}

/// In circuit: verify a child proof under a key fixed as constants, fold its
/// carried accumulator if `carried`, resolve the key's fixed bases and
/// collapse. Returns the collapsed accumulator (one base, scalar one, per
/// side).
pub fn verify_child_incircuit(
    std: &ZkStdLib,
    l: &mut impl Layouter<F>,
    vk: &MidnightVK,
    name: &str,
    instance: &[AssignedNative<F>],
    proof: Value<Vec<u8>>,
    carried: bool,
) -> Result<AssignedAccumulator<S>, Error> {
    verify_child_incircuit_with(std, l, vk, name, instance, proof, carried, true)
}

/// As `verify_child_incircuit`, optionally leaving the resolved accumulator uncollapsed so a
/// caller can fold it with others and collapse once (one shared MSM).
#[allow(clippy::too_many_arguments)]
pub fn verify_child_incircuit_with(
    std: &ZkStdLib,
    l: &mut impl Layouter<F>,
    vk: &MidnightVK,
    name: &str,
    instance: &[AssignedNative<F>],
    proof: Value<Vec<u8>>,
    carried: bool,
    collapse: bool,
) -> Result<AssignedAccumulator<S>, Error> {
    let verifier = std.verifier();
    let bls = std.bls12_381();
    let pvk = vk.vk();
    let assigned_vk =
        verifier.assign_fixed_vk(l, name, pvk.get_domain(), pvk.cs(), pvk.transcript_repr())?;
    let mut assigned_bases = BTreeMap::new();
    for (n, base) in fixed_bases::<S>(name, pvk) {
        assigned_bases.insert(n, bls.assign_fixed(l, base)?);
    }
    let committed = [bls.assign_fixed(l, C::identity())?];
    let own = verifier.prepare(l, &assigned_vk, &committed, &[instance], proof)?;

    let mut acc = if carried {
        let n = acc_pi_len();
        if instance.len() < n {
            return Err(Error::Synthesis(
                "instance too short for a carried accumulator".into(),
            ));
        }
        let tail = &instance[instance.len() - n..];
        let val = Value::from_iter(tail.iter().map(|w| w.value().copied())).map_with_result(
            |fs: Vec<F>| {
                reconstruct_acc(&fs)
                    .ok_or_else(|| Error::Synthesis("carried accumulator malformed".into()))
            },
        )?;
        let carried_acc = verifier.assign_collapsed_accumulator(l, &[], val)?;
        let enc = verifier.as_public_input(l, &carried_acc)?;
        for (a, b) in tail.iter().zip(enc.iter()) {
            std.assert_equal(l, a, b)?;
        }
        verifier.accumulate(l, &[own, carried_acc])?
    } else {
        own
    };
    acc.resolve_fixed_bases(&assigned_bases);
    if collapse {
        acc.collapse(l, bls, bls.scalar_field_chip())?;
    }
    Ok(acc)
}

/// The two collapsed points of an off-circuit accumulator (lhs, rhs).
pub fn acc_points(acc: &Accumulator<S>) -> (C, C) {
    (acc.lhs().bases()[0], acc.rhs().bases()[0])
}

#[allow(dead_code)]
fn _type_check(p: &AP) -> &AssignedForeignPoint<F, C, C> {
    p
}

/// recl9: off-circuit own accumulator of one child proof (no carried
/// accumulator), fixed bases not yet resolved.
pub fn prepare_own_offcircuit(
    vk: &MidnightVK,
    name: &str,
    instance: &[F],
    proof: &[u8],
) -> Result<Accumulator<S>, String> {
    let bases = fixed_bases::<S>(name, vk.vk());
    let mut tr = CircuitTranscript::<PoseidonState<F>>::init_from_bytes(proof);
    let dual =
        plonk::prepare::<F, KZGCommitmentScheme<Bls12>, CircuitTranscript<PoseidonState<F>>>(
            vk.vk(),
            &[&[C::identity()]],
            &[&[instance]],
            &mut tr,
        )
        .map_err(|e| format!("prepare: {e:?}"))?;
    Ok(Accumulator::<S>::from_dual_msm(dual, name, &bases))
}

/// recl9: the node's accumulator off circuit, in exactly the in-circuit
/// order: prepare every child under the same key, accumulate the own
/// accumulators (Poseidon challenge over all their encodings), resolve the
/// shared fixed bases once, collapse.
pub fn children_acc_offcircuit(
    vk: &MidnightVK,
    name: &str,
    instances: &[Vec<F>],
    proofs: &[Vec<u8>],
) -> Result<Accumulator<S>, String> {
    if instances.len() != proofs.len() || instances.is_empty() {
        return Err("children shape".into());
    }
    let owns = instances
        .iter()
        .zip(proofs)
        .map(|(i, p)| prepare_own_offcircuit(vk, name, i, p))
        .collect::<Result<Vec<_>, _>>()?;
    let mut acc = if owns.len() == 1 {
        owns[0].clone()
    } else {
        Accumulator::accumulate(&owns)
    };
    acc.resolve_fixed_bases(&fixed_bases::<S>(name, vk.vk()));
    acc.collapse();
    Ok(acc)
}

/// recl9: verify several child proofs under one key fixed as constants, in
/// circuit. One assigned VK, one fixed-base map and one committed-instance
/// constant are shared; the own accumulators are accumulated, the fixed bases
/// resolved once and the result collapsed to one base per side.
pub fn verify_children_incircuit(
    std: &ZkStdLib,
    l: &mut impl Layouter<F>,
    vk: &MidnightVK,
    name: &str,
    instances: &[Vec<AssignedNative<F>>],
    proofs: &[Value<Vec<u8>>],
) -> Result<AssignedAccumulator<S>, Error> {
    if instances.len() != proofs.len() || instances.is_empty() {
        return Err(Error::Synthesis("children shape".into()));
    }
    let verifier = std.verifier();
    let bls = std.bls12_381();
    let pvk = vk.vk();
    let assigned_vk =
        verifier.assign_fixed_vk(l, name, pvk.get_domain(), pvk.cs(), pvk.transcript_repr())?;
    let mut assigned_bases = BTreeMap::new();
    for (n, base) in fixed_bases::<S>(name, pvk) {
        assigned_bases.insert(n, bls.assign_fixed(l, base)?);
    }
    let committed = [bls.assign_fixed(l, C::identity())?];
    let mut owns = Vec::with_capacity(instances.len());
    for (inst, proof) in instances.iter().zip(proofs) {
        owns.push(verifier.prepare(
            l,
            &assigned_vk,
            &committed,
            &[inst.as_slice()],
            proof.clone(),
        )?);
    }
    let mut acc = if owns.len() == 1 {
        owns.remove(0)
    } else {
        verifier.accumulate(l, &owns)?
    };
    acc.resolve_fixed_bases(&assigned_bases);
    acc.collapse(l, bls, bls.scalar_field_chip())?;
    Ok(acc)
}
