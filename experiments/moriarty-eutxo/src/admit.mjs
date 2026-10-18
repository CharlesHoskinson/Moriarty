/** Admit a public atomic eUTXO subset. Private/PCD claims are recorded, never verified. */
export const PUBLIC_PROFILE = 'moriarty-bounded-atomic/1';
export const TIME_PREDICATE = 'TIME_PREDICATE_v1';
export const ADMITTED_EFFECTS = new Set(['Transfer', 'Fee']);
export const NATIVE_PREDICATE = 'public-single-head-wallet-transfer/1';
const HEX28 = /^[0-9a-f]{56}$/;
const HEX_NAME = /^([0-9a-f]{2}){1,32}$/;
const ADA_POLICY = /^(ada)?$/i;

export function detectProfile(source) {
  const text = String(source);
  const declared = text.match(/profile\s+"([^"]+)"/);
  return declared ? declared[1] : null;
}

function fail(code, extra = {}) {
  return { ok: false, code, production: false, ...extra };
}

function walk(e, visit) {
  visit(e);
  if (e.tag === 'FloorDiv') { walk(e.numerator, visit); walk(e.denominator, visit); }
  else if (e.tag === 'Not') walk(e.operand, visit);
  else if (e.left) { walk(e.left, visit); walk(e.right, visit); }
}

function textOf(e, constants) {
  if (e.tag === 'Literal' && e.value.tag === 'Text') return e.value.value;
  if (e.tag === 'ConstRef') {
    const c = constants.find(x => x.name === e.declaration);
    if (c?.value?.tag === 'Text') return c.value.value;
  }
  return undefined;
}

export function admitPublicAtomic(input, manifest) {
  if (!manifest || manifest.schemaVersion !== 'moriarty-eutxo-public-manifest/1') {
    return fail('MANIFEST_SCHEMA');
  }
  if (manifest.production === true) return fail('PRODUCTION_CLAIM_FORBIDDEN');
  const sourceText = typeof input?.source === 'string' && !input.manifest ? input.source : undefined;
  if (sourceText !== undefined) {
    const profile = detectProfile(sourceText);
    if (profile !== PUBLIC_PROFILE) {
      return fail('PROFILE_UNSUPPORTED', { profile, sourceProfile: profile });
    }
    return fail('BOUND_PROGRAM_REQUIRED', { profile });
  }
  const bound = input?.manifest ? input : input?.bound;
  if (!bound?.manifest) return fail('BOUND_PROGRAM_REQUIRED');
  const m = bound.manifest;
  if (m.profile !== PUBLIC_PROFILE) return fail('PROFILE_UNSUPPORTED', { profile: m.profile });

  const kinds = [...new Set(m.effectSchemas.map(e => e.kind))];
  const unsupportedEffects = kinds.filter(k => !ADMITTED_EFFECTS.has(k));
  if (unsupportedEffects.length) {
    return fail('EFFECT_KIND_UNSUPPORTED', { kinds: unsupportedEffects });
  }

  const obs = m.observationSchema || [];
  const unsupportedObs = obs.filter(o => !(o.name === 'now' && o.type.tag === 'UInt128'));
  if (unsupportedObs.length) {
    return fail('OBSERVATION_UNSUPPORTED', { names: unsupportedObs.map(o => o.name) });
  }
  if (obs.length !== 1 || obs[0].name !== 'now') {
    return fail('OBSERVATION_UNSUPPORTED', { names: obs.map(o => o.name), reason: 'public subset requires exactly observation now: UInt128' });
  }

  const inventoryState = m.stateSchema.filter(s => s.type.tag === 'Text' || s.type.tag === 'Bool' || s.type.tag === 'Quantity' || s.type.tag === 'Amount');
  if (inventoryState.length) {
    const amountFields = inventoryState.filter(s => s.type.tag === 'Amount' || s.type.tag === 'Quantity');
    if (amountFields.length) {
      return fail('STATE_INVENTORY_UNSUPPORTED', {
        fields: amountFields.map(s => s.name),
        reason: 'Amount/Quantity state would imply unsupported ledger inventory, pool, or loan projection',
      });
    }
    return fail('STATE_TYPE_UNSUPPORTED', { fields: inventoryState.map(s => s.name) });
  }

  if (!HEX28.test(manifest.thread?.policy) || !HEX_NAME.test(manifest.thread?.name)) {
    return fail('THREAD_MAPPING_UNBOUND');
  }
  const assetKeys = [];
  for (const b of m.settlementBindings) {
    if (b.quantum?.value !== '1') {
      return fail('SETTLEMENT_QUANTUM_UNSUPPORTED', {
        asset: b.asset,
        quantum: b.quantum?.value,
        reason: 'public native subset admits quantum 1 only; ledgerAmount conversion is not implemented',
      });
    }
    const asset = manifest.assets?.[b.asset];
    if (!asset) return fail('ASSET_MAPPING_UNBOUND', { asset: b.asset });
    if (ADA_POLICY.test(asset.policy || '')) {
      const feeOk = manifest.feePolicy === true || manifest.feePolicy?.[b.asset] === true;
      if (!feeOk) return fail('ADA_SETTLEMENT_UNSUPPORTED', { asset: b.asset });
    }
    if (!HEX28.test(asset.policy) || !HEX_NAME.test(asset.name)) {
      return fail('ASSET_MAPPING_UNBOUND', { asset: b.asset });
    }
    assetKeys.push(`${asset.policy}:${asset.name}`);
  }
  if (new Set(assetKeys).size !== assetKeys.length) {
    return fail('ASSET_MAPPING_NOT_INJECTIVE', { keys: assetKeys });
  }

  const constants = m.constants;
  const requiredActors = new Set();
  for (const action of m.core.actions) {
    for (const ins of action.instructions) {
      const exprs = ins.tag === 'Guard' ? [ins.condition]
        : ins.tag === 'Emit' ? ins.fields.map(f => f.expression)
        : [ins.expression];
      for (const e of exprs) {
        walk(e, node => {
          if (node.tag === 'Eq') {
            const texts = [textOf(node.left, constants), textOf(node.right, constants)].filter(Boolean);
            const refsActor = (node.left.tag === 'ArgRef' && node.left.declaration === 'actor')
              || (node.right.tag === 'ArgRef' && node.right.declaration === 'actor');
            if (refsActor) texts.forEach(t => requiredActors.add(t));
          }
        });
      }
      if (ins.tag === 'Emit' && (ins.kind === 'Transfer' || ins.kind === 'Fee')) {
        for (const field of ins.fields) {
          if (field.label === 'from' || field.label === 'to') {
            const t = textOf(field.expression, constants);
            if (t) requiredActors.add(t);
            else if (field.expression.tag !== 'ArgRef') return fail('ACTOR_MAPPING_UNBOUND', { field: field.label, reason: 'non-literal actor' });
          }
        }
      }
    }
  }
  for (const actor of requiredActors) {
    if (!HEX28.test(manifest.actors?.[actor] || '')) {
      return fail('ACTOR_MAPPING_UNBOUND', { actor, required: [...requiredActors] });
    }
  }
  const actorEntries = Object.entries(manifest.actors || {});
  const vkhs = actorEntries.map(([, hex]) => hex);
  if (new Set(vkhs).size !== vkhs.length) {
    return fail('ACTOR_MAPPING_NOT_INJECTIVE', { actors: actorEntries.map(([n]) => n) });
  }
  if (!manifest.domain?.network || !manifest.domain?.deployment) return fail('DOMAIN_MAPPING_UNBOUND');

  return {
    ok: true,
    production: false,
    capability: 'public-atomic-experiment',
    profile: PUBLIC_PROFILE,
    nativePredicate: NATIVE_PREDICATE,
    nativeSubset: {
      scriptInputs: 'exactly-one-script-credential-input',
      otherInputs: 'VerificationKey-only',
      effectBinding: 'net-delta-per-mapped-vk-and-admitted-asset-equals-source-effects',
      headValue: 'preserved-including-ADA-except-declared-supported-custody',
      custody: 'wallet-transfer-only; script-held inventory unsupported',
      adaSettlement: 'unsupported-unless-explicit-feePolicy',
      quantum: 'exactly-1-nominal-per-ledger-unit',
      grossDebit: 'unsupported-wallet-policy; net-delta-is-not-gross-input',
      multiHead: 'unsupported',
      frozenComplete: false,
    },
    timePredicate: TIME_PREDICATE,
    timePredicateDoc: 'obs.now is the inclusive last POSIX millisecond of a finite tx validity interval. Plutus V3 ranges are semi-open [lower, upper) when the upper bound is exclusive. Unbounded or empty intervals reject. This backs the observation with ledger interval data; it does not authenticate wall-clock time.',
    genesisPrecondition: {
      kind: 'external-consumed-seed-oneshot-mint',
      trust: 'Thread NFT issuance is an external native precondition. Root uses the official Aiken consumed-seed one-shot mint policy. This spend validator does not authenticate first issuance, does not verify PCD, and does not claim frozen Complete.',
      pcdVerification: 'explicitly-not-performed',
    },
    bound,
    manifest,
    requiredActors: [...requiredActors],
    unsupportedPrivateClaims: [...m.requiredClaims],
    privateVerification: 'explicitly-not-performed',
    admittedEffects: kinds,
  };
}
