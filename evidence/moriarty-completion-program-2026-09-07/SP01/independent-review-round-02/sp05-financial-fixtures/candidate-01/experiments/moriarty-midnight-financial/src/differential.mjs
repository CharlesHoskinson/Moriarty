const SCHEMA_VERSION = 'moriarty-financial-record/1';
const U128_MAX = (1n << 128n) - 1n;
const U128_RE = /^(0|[1-9][0-9]*)$/;

const RECORD_KEYS = [
  'schemaVersion', 'observationKind', 'networkAcceptance', 'networkEvidence',
  'derivationNote', 'sourcePins', 'deploymentBinding', 'networkFeeAccounting',
  'agreement', 'stages'
];
const SOURCE_PIN_KEYS = [
  'acceptedSourcePath', 'acceptedSourceSha256', 'metadataInspectedPath',
  'metadataInspectedSha256', 'metadataUsedForExpectations'
];
const DEPLOY_KEYS = ['status', 'roles', 'assets'];
const ROLE_KEYS = ['status', 'logicalId'];
const ASSET_KEYS = ['id', 'logicalId', 'denomination', 'quantum', 'color', 'status'];
const FEE_ACC_KEYS = ['status', 'midnightFeeAsset', 'midnightFeeUnit', 'assumedZero'];
const AGREEMENT_KEYS = ['name', 'profile', 'lifetime', 'horizon'];
const STAGE_KEYS = [
  'stage', 'revision', 'remaining', 'work', 'status', 'economicFee', 'bindings',
  'balances', 'transfers', 'actorEffects', 'liabilities', 'residualDuties'
];
const ECON_FEE_KEYS = ['actor', 'asset', 'color', 'unit', 'amount', 'networkFeeSeparate'];
const BINDING_KEYS = ['roles', 'assets'];
const BALANCE_BLOCK_KEYS = ['pre', 'post'];
const BALANCE_KEYS = ['id', 'actor', 'asset', 'unit', 'amount'];
const TRANSFER_KEYS = ['id', 'ordinal', 'from', 'to', 'color', 'unit', 'amount'];
const EFFECT_KEYS = ['id', 'actor', 'asset', 'unit', 'grossDebit', 'refund', 'netCredit', 'fee'];
const LIAB_KEYS = ['nominalRemaining', 'denomination', 'debtId', 'dues', 'principal', 'accrual'];
const DUE_KEYS = ['id', 'dueId', 'kind', 'debtor', 'creditor', 'denomination', 'created', 'settled', 'outstanding'];
const PRINCIPAL_KEYS = ['installment', 'paid', 'due', 'outstandingNotional'];
const ACCRUAL_KEYS = [
  'interestCalculated', 'interestPaid', 'interestDue', 'rateNumerator', 'rateDenominator',
  'periodNumerator', 'periodDenominator', 'floorRemainderNumerator', 'floorRemainderDenominator'
];
const RESIDUAL_KEYS = ['id', 'kind', 'amount', 'unit', 'status'];
const LOAN_STATUS_KEYS = ['episodeClosed', 'cursor'];
const SWAP_STATUS_KEYS = ['episodeClosed'];

function snapshot(value) {
  try {
    return JSON.stringify(value);
  } catch {
    return null;
  }
}

function isPlain(value) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) return false;
  const proto = Object.getPrototypeOf(value);
  return proto === Object.prototype || proto === null;
}

function push(errors, code, path, message) {
  errors.push({ code, path, message });
}

function sortErrors(errors) {
  errors.sort((a, b) => {
    if (a.path < b.path) return -1;
    if (a.path > b.path) return 1;
    if (a.code < b.code) return -1;
    if (a.code > b.code) return 1;
    if (a.message < b.message) return -1;
    if (a.message > b.message) return 1;
    return 0;
  });
}

function expectKeys(obj, path, required, errors) {
  if (!isPlain(obj)) {
    push(errors, 'MALFORMED_STRUCTURE', path, 'object required');
    return false;
  }
  const keys = Object.keys(obj);
  const allow = new Set(required);
  let ok = true;
  for (const key of required) {
    if (!Object.prototype.hasOwnProperty.call(obj, key)) {
      push(errors, 'MISSING_FIELD', `${path}.${key}`, `missing ${key}`);
      ok = false;
    }
  }
  for (const key of keys) {
    if (!allow.has(key)) {
      push(errors, 'UNKNOWN_FIELD', `${path}.${key}`, `unknown field ${key}`);
      ok = false;
    }
  }
  return ok;
}

function parseU128(value, path, errors) {
  if (typeof value !== 'string') {
    push(errors, 'NONCANONICAL_INTEGER', path, 'unsigned integer must be a canonical decimal string');
    return null;
  }
  if (!U128_RE.test(value)) {
    push(errors, 'NONCANONICAL_INTEGER', path, 'noncanonical unsigned integer');
    return null;
  }
  const parsed = BigInt(value);
  if (parsed > U128_MAX) {
    push(errors, 'INTEGER_OVERFLOW', path, 'integer exceeds 2^128-1');
    return null;
  }
  return parsed;
}

function expectString(value, path, errors) {
  if (typeof value !== 'string' || value.length === 0) {
    push(errors, 'MALFORMED_STRUCTURE', path, 'nonempty string required');
    return false;
  }
  return true;
}

function expectBool(value, path, errors, required) {
  if (typeof value !== 'boolean') {
    push(errors, 'MALFORMED_STRUCTURE', path, 'boolean required');
    return false;
  }
  if (value !== required) {
    const code = path.endsWith('networkAcceptance') ? 'NETWORK_CLAIM_FORBIDDEN' : 'VALUE_MISMATCH';
    push(errors, code, path, `value must be ${required}`);
    return false;
  }
  return true;
}

function expectNull(value, path, errors) {
  if (value !== null) {
    push(errors, 'VALUE_MISMATCH', path, 'unresolved network fee field must be null');
    return false;
  }
  return true;
}

function expectArray(value, path, errors) {
  if (!Array.isArray(value)) {
    push(errors, 'MALFORMED_STRUCTURE', path, 'array required');
    return false;
  }
  return true;
}

function indexById(list, path, errors) {
  const map = new Map();
  if (!Array.isArray(list)) return map;
  for (let i = 0; i < list.length; i += 1) {
    const item = list[i];
    const idPath = `${path}[${i}].id`;
    if (!isPlain(item) || typeof item.id !== 'string' || item.id.length === 0) {
      push(errors, 'MALFORMED_STRUCTURE', idPath, 'indexed object requires unique id');
      continue;
    }
    if (map.has(item.id)) {
      push(errors, 'DUPLICATE_IDENTITY', `${path}[${item.id}]`, `duplicate identity ${item.id}`);
      continue;
    }
    map.set(item.id, item);
  }
  return map;
}

function validateAsset(asset, path, errors) {
  if (!expectKeys(asset, path, ASSET_KEYS, errors)) return;
  expectString(asset.id, `${path}.id`, errors);
  expectString(asset.logicalId, `${path}.logicalId`, errors);
  expectString(asset.denomination, `${path}.denomination`, errors);
  expectString(asset.color, `${path}.color`, errors);
  parseU128(asset.quantum, `${path}.quantum`, errors);
  if (asset.status !== 'unresolved') {
    push(errors, 'BINDING_MISMATCH', `${path}.status`, 'deployment binding is unresolved');
  }
  if (asset.id !== asset.logicalId) {
    push(errors, 'MALFORMED_STRUCTURE', `${path}.id`, 'asset id must equal logicalId');
  }
}

function validateRoleMap(roles, path, errors) {
  if (!isPlain(roles)) {
    push(errors, 'MALFORMED_STRUCTURE', path, 'role map required');
    return;
  }
  for (const name of Object.keys(roles)) {
    const rolePath = `${path}.${name}`;
    if (!expectKeys(roles[name], rolePath, ROLE_KEYS, errors)) continue;
    expectString(roles[name].logicalId, `${rolePath}.logicalId`, errors);
    const status = roles[name].status;
    if (status !== 'symbolic-not-address' && status !== 'synthetic-local-setup-source') {
      push(errors, 'BINDING_MISMATCH', `${rolePath}.status`, 'role is symbolic or synthetic-local');
    }
  }
}

function validateDue(due, path, errors) {
  if (!expectKeys(due, path, DUE_KEYS, errors)) return;
  for (const key of ['id', 'dueId', 'kind', 'debtor', 'creditor', 'denomination']) {
    expectString(due[key], `${path}.${key}`, errors);
  }
  for (const key of ['created', 'settled', 'outstanding']) {
    parseU128(due[key], `${path}.${key}`, errors);
  }
  if (due.id !== due.dueId) {
    push(errors, 'MALFORMED_STRUCTURE', `${path}.id`, 'due id must equal dueId');
  }
}

function validateLiabilities(liab, path, errors) {
  if (!expectKeys(liab, path, LIAB_KEYS, errors)) return;
  parseU128(liab.nominalRemaining, `${path}.nominalRemaining`, errors);
  expectString(liab.denomination, `${path}.denomination`, errors);
  expectString(liab.debtId, `${path}.debtId`, errors);
  if (expectArray(liab.dues, `${path}.dues`, errors)) {
    indexById(liab.dues, `${path}.dues`, errors);
    liab.dues.forEach((due, i) => validateDue(due, `${path}.dues[${i}]`, errors));
  }
  if (expectKeys(liab.principal, `${path}.principal`, PRINCIPAL_KEYS, errors)) {
    for (const key of PRINCIPAL_KEYS) parseU128(liab.principal[key], `${path}.principal.${key}`, errors);
  }
  if (expectKeys(liab.accrual, `${path}.accrual`, ACCRUAL_KEYS, errors)) {
    for (const key of ACCRUAL_KEYS) parseU128(liab.accrual[key], `${path}.accrual.${key}`, errors);
  }
}

function validateStage(stage, path, errors, agreementName) {
  if (!expectKeys(stage, path, STAGE_KEYS, errors)) return;
  expectString(stage.stage, `${path}.stage`, errors);
  parseU128(stage.revision, `${path}.revision`, errors);
  parseU128(stage.remaining, `${path}.remaining`, errors);
  parseU128(stage.work, `${path}.work`, errors);
  const statusKeys = agreementName === 'LoanFirstPeriod' ? LOAN_STATUS_KEYS : SWAP_STATUS_KEYS;
  if (expectKeys(stage.status, `${path}.status`, statusKeys, errors)) {
    for (const key of statusKeys) parseU128(stage.status[key], `${path}.status.${key}`, errors);
  }
  if (expectKeys(stage.economicFee, `${path}.economicFee`, ECON_FEE_KEYS, errors)) {
    for (const key of ['actor', 'asset', 'color', 'unit']) {
      expectString(stage.economicFee[key], `${path}.economicFee.${key}`, errors);
    }
    parseU128(stage.economicFee.amount, `${path}.economicFee.amount`, errors);
    expectBool(stage.economicFee.networkFeeSeparate, `${path}.economicFee.networkFeeSeparate`, errors, true);
  }
  if (expectKeys(stage.bindings, `${path}.bindings`, BINDING_KEYS, errors)) {
    if (expectArray(stage.bindings.roles, `${path}.bindings.roles`, errors)) {
      stage.bindings.roles.forEach((role, i) => expectString(role, `${path}.bindings.roles[${i}]`, errors));
    }
    if (expectArray(stage.bindings.assets, `${path}.bindings.assets`, errors)) {
      indexById(stage.bindings.assets, `${path}.bindings.assets`, errors);
      stage.bindings.assets.forEach((asset, i) => validateAsset(asset, `${path}.bindings.assets[${i}]`, errors));
    }
  }
  if (expectKeys(stage.balances, `${path}.balances`, BALANCE_BLOCK_KEYS, errors)) {
    for (const side of ['pre', 'post']) {
      const sidePath = `${path}.balances.${side}`;
      if (!expectArray(stage.balances[side], sidePath, errors)) continue;
      indexById(stage.balances[side], sidePath, errors);
      stage.balances[side].forEach((row, i) => {
        const rowPath = `${sidePath}[${i}]`;
        if (!expectKeys(row, rowPath, BALANCE_KEYS, errors)) return;
        for (const key of ['id', 'actor', 'asset', 'unit']) expectString(row[key], `${rowPath}.${key}`, errors);
        parseU128(row.amount, `${rowPath}.amount`, errors);
        if (row.id !== `${row.actor}|${row.asset}|${row.unit}`) {
          push(errors, 'MALFORMED_STRUCTURE', `${rowPath}.id`, 'balance id must be actor|asset|unit');
        }
      });
    }
  }
  if (expectArray(stage.transfers, `${path}.transfers`, errors)) {
    indexById(stage.transfers, `${path}.transfers`, errors);
    stage.transfers.forEach((transfer, i) => {
      const tPath = `${path}.transfers[${i}]`;
      if (!expectKeys(transfer, tPath, TRANSFER_KEYS, errors)) return;
      for (const key of ['id', 'from', 'to', 'color', 'unit']) expectString(transfer[key], `${tPath}.${key}`, errors);
      parseU128(transfer.ordinal, `${tPath}.ordinal`, errors);
      parseU128(transfer.amount, `${tPath}.amount`, errors);
    });
  }
  if (expectArray(stage.actorEffects, `${path}.actorEffects`, errors)) {
    indexById(stage.actorEffects, `${path}.actorEffects`, errors);
    stage.actorEffects.forEach((effect, i) => {
      const ePath = `${path}.actorEffects[${i}]`;
      if (!expectKeys(effect, ePath, EFFECT_KEYS, errors)) return;
      for (const key of ['id', 'actor', 'asset', 'unit']) expectString(effect[key], `${ePath}.${key}`, errors);
      for (const key of ['grossDebit', 'refund', 'netCredit', 'fee']) {
        parseU128(effect[key], `${ePath}.${key}`, errors);
      }
      if (effect.id !== `${effect.actor}|${effect.asset}|${effect.unit}`) {
        push(errors, 'MALFORMED_STRUCTURE', `${ePath}.id`, 'effect id must be actor|asset|unit');
      }
    });
  }
  validateLiabilities(stage.liabilities, `${path}.liabilities`, errors);
  if (expectArray(stage.residualDuties, `${path}.residualDuties`, errors)) {
    indexById(stage.residualDuties, `${path}.residualDuties`, errors);
    stage.residualDuties.forEach((duty, i) => {
      const dPath = `${path}.residualDuties[${i}]`;
      if (!expectKeys(duty, dPath, RESIDUAL_KEYS, errors)) return;
      for (const key of ['id', 'kind', 'unit', 'status']) expectString(duty[key], `${dPath}.${key}`, errors);
      parseU128(duty.amount, `${dPath}.amount`, errors);
    });
  }
}

function validateRecord(record, label, errors) {
  if (!isPlain(record)) {
    push(errors, 'MALFORMED_STRUCTURE', label, 'financial record object required');
    return false;
  }
  if (!expectKeys(record, label, RECORD_KEYS, errors)) {
    if (!isPlain(record)) return false;
  }
  if (record.schemaVersion !== SCHEMA_VERSION) {
    push(errors, 'VALUE_MISMATCH', `${label}.schemaVersion`, 'unsupported schema');
  }
  if (record.observationKind !== 'synthetic-local') {
    push(errors, 'VALUE_MISMATCH', `${label}.observationKind`, 'observationKind must be synthetic-local');
  }
  expectBool(record.networkAcceptance, `${label}.networkAcceptance`, errors, false);
  if (record.networkEvidence !== 'incompleteNetworkEvidence') {
    push(errors, 'VALUE_MISMATCH', `${label}.networkEvidence`, 'network evidence is incomplete');
  }
  expectString(record.derivationNote, `${label}.derivationNote`, errors);
  if (expectKeys(record.sourcePins, `${label}.sourcePins`, SOURCE_PIN_KEYS, errors)) {
    for (const key of ['acceptedSourcePath', 'acceptedSourceSha256', 'metadataInspectedPath', 'metadataInspectedSha256']) {
      expectString(record.sourcePins[key], `${label}.sourcePins.${key}`, errors);
    }
    expectBool(record.sourcePins.metadataUsedForExpectations, `${label}.sourcePins.metadataUsedForExpectations`, errors, false);
  }
  if (expectKeys(record.deploymentBinding, `${label}.deploymentBinding`, DEPLOY_KEYS, errors)) {
    if (record.deploymentBinding.status !== 'unresolved') {
      push(errors, 'BINDING_MISMATCH', `${label}.deploymentBinding.status`, 'deployment binding is unresolved');
    }
    validateRoleMap(record.deploymentBinding.roles, `${label}.deploymentBinding.roles`, errors);
    if (expectArray(record.deploymentBinding.assets, `${label}.deploymentBinding.assets`, errors)) {
      indexById(record.deploymentBinding.assets, `${label}.deploymentBinding.assets`, errors);
      record.deploymentBinding.assets.forEach((asset, i) => {
        validateAsset(asset, `${label}.deploymentBinding.assets[${i}]`, errors);
      });
    }
  }
  if (expectKeys(record.networkFeeAccounting, `${label}.networkFeeAccounting`, FEE_ACC_KEYS, errors)) {
    if (record.networkFeeAccounting.status !== 'unresolved') {
      push(errors, 'VALUE_MISMATCH', `${label}.networkFeeAccounting.status`, 'network fee accounting is unresolved');
    }
    expectNull(record.networkFeeAccounting.midnightFeeAsset, `${label}.networkFeeAccounting.midnightFeeAsset`, errors);
    expectNull(record.networkFeeAccounting.midnightFeeUnit, `${label}.networkFeeAccounting.midnightFeeUnit`, errors);
    expectBool(record.networkFeeAccounting.assumedZero, `${label}.networkFeeAccounting.assumedZero`, errors, false);
  }
  if (expectKeys(record.agreement, `${label}.agreement`, AGREEMENT_KEYS, errors)) {
    for (const key of ['name', 'profile']) expectString(record.agreement[key], `${label}.agreement.${key}`, errors);
    parseU128(record.agreement.lifetime, `${label}.agreement.lifetime`, errors);
    parseU128(record.agreement.horizon, `${label}.agreement.horizon`, errors);
  }
  if (expectArray(record.stages, `${label}.stages`, errors)) {
    const names = new Set();
    record.stages.forEach((stage, i) => {
      const path = `${label}.stages[${stage && stage.stage ? stage.stage : i}]`;
      if (isPlain(stage) && typeof stage.stage === 'string') {
        if (names.has(stage.stage)) {
          push(errors, 'DUPLICATE_IDENTITY', path, `duplicate stage ${stage.stage}`);
        }
        names.add(stage.stage);
      }
      validateStage(stage, path, errors, record.agreement && record.agreement.name);
    });
  }
  return isPlain(record)
    && Array.isArray(record.stages)
    && isPlain(record.agreement)
    && isPlain(record.deploymentBinding)
    && isPlain(record.networkFeeAccounting)
    && isPlain(record.sourcePins);
}

function addAmount(map, key, amount) {
  map.set(key, (map.get(key) || 0n) + amount);
}

function checkConservation(record, label, errors) {
  if (!Array.isArray(record.stages)) return;
  for (const stage of record.stages) {
    if (!isPlain(stage) || !isPlain(stage.balances) || !Array.isArray(stage.transfers) || !Array.isArray(stage.actorEffects)) {
      continue;
    }
    const base = `${label}.stages[${stage.stage}]`;
    const outgoing = new Map();
    const incoming = new Map();
    for (const transfer of stage.transfers) {
      if (!isPlain(transfer) || !U128_RE.test(transfer.amount)) continue;
      const amount = BigInt(transfer.amount);
      if (amount > U128_MAX) continue;
      addAmount(outgoing, `${transfer.from}|${transfer.color}|${transfer.unit}`, amount);
      addAmount(incoming, `${transfer.to}|${transfer.color}|${transfer.unit}`, amount);
    }
    const pre = indexById(stage.balances.pre, `${base}.balances.pre`, []);
    const post = indexById(stage.balances.post, `${base}.balances.post`, []);
    for (const id of pre.keys()) {
      if (!post.has(id)) {
        push(errors, 'MISSING_FIELD', `${base}.balances.post[${id}]`, 'post balance missing');
        continue;
      }
      const preAmt = parseU128(pre.get(id).amount, `${base}.balances.pre[${id}].amount`, errors);
      const postAmt = parseU128(post.get(id).amount, `${base}.balances.post[${id}].amount`, errors);
      if (preAmt === null || postAmt === null) continue;
      const out = outgoing.get(id) || 0n;
      const inn = incoming.get(id) || 0n;
      if (postAmt !== preAmt - out + inn) {
        push(errors, 'BALANCE_INCONSISTENT', `${base}.balances.post[${id}].amount`, 'post balance does not match transfers');
      }
    }
    for (const id of post.keys()) {
      if (!pre.has(id)) {
        push(errors, 'MISSING_FIELD', `${base}.balances.pre[${id}]`, 'pre balance missing');
      }
    }
    const effects = indexById(stage.actorEffects, `${base}.actorEffects`, []);
    for (const [id, effect] of effects) {
      const gross = parseU128(effect.grossDebit, `${base}.actorEffects[${id}].grossDebit`, errors);
      const refund = parseU128(effect.refund, `${base}.actorEffects[${id}].refund`, errors);
      const net = parseU128(effect.netCredit, `${base}.actorEffects[${id}].netCredit`, errors);
      const fee = parseU128(effect.fee, `${base}.actorEffects[${id}].fee`, errors);
      if (gross === null || refund === null || net === null || fee === null) continue;
      const out = outgoing.get(id) || 0n;
      const inn = incoming.get(id) || 0n;
      if (gross !== out) {
        push(errors, 'GROSS_DEBIT_MISMATCH', `${base}.actorEffects[${id}].grossDebit`, 'gross debit must equal outgoing transfers');
      }
      if (net !== inn) {
        push(errors, 'NET_CREDIT_MISMATCH', `${base}.actorEffects[${id}].netCredit`, 'net credit must equal incoming transfers');
      }
      if (refund !== 0n) {
        push(errors, 'REFUND_MISMATCH', `${base}.actorEffects[${id}].refund`, 'refund cannot substitute for gross debit');
      }
      if (fee > gross) {
        push(errors, 'FEE_MISMATCH', `${base}.actorEffects[${id}].fee`, 'fee exceeds gross debit');
      }
    }
  }
}

function cmpString(expected, observed, path, code, errors) {
  if (expected !== observed) {
    push(errors, code, path, `expected ${JSON.stringify(expected)} observed ${JSON.stringify(observed)}`);
  }
}

function cmpU128Field(expected, observed, path, code, errors) {
  if (expected !== observed) {
    push(errors, code, path, `expected ${expected} observed ${observed}`);
  }
}

function compareIndexed(expList, obsList, path, omittedCode, extraCode, compareItem, errors) {
  const exp = indexById(expList, `expected.${path}`, []);
  const obs = indexById(obsList, `observed.${path}`, []);
  for (const id of exp.keys()) {
    if (!obs.has(id)) push(errors, omittedCode, `observed.${path}[${id}]`, `omitted ${id}`);
    else compareItem(exp.get(id), obs.get(id), `observed.${path}[${id}]`);
  }
  for (const id of obs.keys()) {
    if (!exp.has(id)) push(errors, extraCode, `observed.${path}[${id}]`, `extra ${id}`);
  }
}

function compareAsset(exp, obs, path, errors) {
  cmpString(exp.logicalId, obs.logicalId, `${path}.logicalId`, 'BINDING_MISMATCH', errors);
  cmpString(exp.denomination, obs.denomination, `${path}.denomination`, 'DENOMINATION_MISMATCH', errors);
  cmpU128Field(exp.quantum, obs.quantum, `${path}.quantum`, 'QUANTUM_MISMATCH', errors);
  cmpString(exp.color, obs.color, `${path}.color`, 'COLOR_MISMATCH', errors);
  cmpString(exp.status, obs.status, `${path}.status`, 'BINDING_MISMATCH', errors);
}

function compareStage(exp, obs, stageName, errors) {
  const path = `observed.stages[${stageName}]`;
  cmpU128Field(exp.revision, obs.revision, `${path}.revision`, 'REVISION_MISMATCH', errors);
  cmpU128Field(exp.remaining, obs.remaining, `${path}.remaining`, 'REMAINING_MISMATCH', errors);
  cmpU128Field(exp.work, obs.work, `${path}.work`, 'WORK_MISMATCH', errors);
  for (const key of Object.keys(exp.status || {})) {
    cmpU128Field(exp.status[key], obs.status && obs.status[key], `${path}.status.${key}`, 'STATUS_MISMATCH', errors);
  }
  cmpString(exp.economicFee.actor, obs.economicFee.actor, `${path}.economicFee.actor`, 'FEE_MISMATCH', errors);
  cmpString(exp.economicFee.asset, obs.economicFee.asset, `${path}.economicFee.asset`, 'FEE_MISMATCH', errors);
  cmpString(exp.economicFee.color, obs.economicFee.color, `${path}.economicFee.color`, 'COLOR_MISMATCH', errors);
  cmpString(exp.economicFee.unit, obs.economicFee.unit, `${path}.economicFee.unit`, 'DENOMINATION_MISMATCH', errors);
  cmpU128Field(exp.economicFee.amount, obs.economicFee.amount, `${path}.economicFee.amount`, 'FEE_MISMATCH', errors);
  if (JSON.stringify(exp.bindings.roles) !== JSON.stringify(obs.bindings.roles)) {
    push(errors, 'BINDING_MISMATCH', `${path}.bindings.roles`, 'role bindings differ');
  }
  compareIndexed(exp.bindings.assets, obs.bindings.assets, `stages[${stageName}].bindings.assets`, 'BINDING_MISMATCH', 'BINDING_MISMATCH', (a, b, p) => compareAsset(a, b, p, errors), errors);
  for (const side of ['pre', 'post']) {
    compareIndexed(exp.balances[side], obs.balances[side], `stages[${stageName}].balances.${side}`, 'VALUE_MISMATCH', 'VALUE_MISMATCH', (a, b, p) => {
      cmpString(a.actor, b.actor, `${p}.actor`, 'VALUE_MISMATCH', errors);
      cmpString(a.asset, b.asset, `${p}.asset`, 'COLOR_MISMATCH', errors);
      cmpString(a.unit, b.unit, `${p}.unit`, 'DENOMINATION_MISMATCH', errors);
      cmpU128Field(a.amount, b.amount, `${p}.amount`, 'VALUE_MISMATCH', errors);
    }, errors);
  }
  compareIndexed(exp.transfers, obs.transfers, `stages[${stageName}].transfers`, 'OMITTED_TRANSFER', 'EXTRA_TRANSFER', (a, b, p) => {
    cmpU128Field(a.ordinal, b.ordinal, `${p}.ordinal`, 'TRANSFER_MISMATCH', errors);
    cmpString(a.from, b.from, `${p}.from`, 'TRANSFER_MISMATCH', errors);
    cmpString(a.to, b.to, `${p}.to`, 'TRANSFER_MISMATCH', errors);
    cmpString(a.color, b.color, `${p}.color`, 'COLOR_MISMATCH', errors);
    cmpString(a.unit, b.unit, `${p}.unit`, 'DENOMINATION_MISMATCH', errors);
    cmpU128Field(a.amount, b.amount, `${p}.amount`, 'TRANSFER_MISMATCH', errors);
  }, errors);
  compareIndexed(exp.actorEffects, obs.actorEffects, `stages[${stageName}].actorEffects`, 'VALUE_MISMATCH', 'VALUE_MISMATCH', (a, b, p) => {
    cmpU128Field(a.grossDebit, b.grossDebit, `${p}.grossDebit`, 'GROSS_DEBIT_MISMATCH', errors);
    cmpU128Field(a.refund, b.refund, `${p}.refund`, 'REFUND_MISMATCH', errors);
    cmpU128Field(a.netCredit, b.netCredit, `${p}.netCredit`, 'NET_CREDIT_MISMATCH', errors);
    cmpU128Field(a.fee, b.fee, `${p}.fee`, 'FEE_MISMATCH', errors);
  }, errors);
  cmpU128Field(exp.liabilities.nominalRemaining, obs.liabilities.nominalRemaining, `${path}.liabilities.nominalRemaining`, 'LIABILITY_MISMATCH', errors);
  cmpString(exp.liabilities.denomination, obs.liabilities.denomination, `${path}.liabilities.denomination`, 'DENOMINATION_MISMATCH', errors);
  cmpString(exp.liabilities.debtId, obs.liabilities.debtId, `${path}.liabilities.debtId`, 'LIABILITY_MISMATCH', errors);
  for (const key of PRINCIPAL_KEYS) {
    cmpU128Field(exp.liabilities.principal[key], obs.liabilities.principal[key], `${path}.liabilities.principal.${key}`, 'LIABILITY_MISMATCH', errors);
  }
  for (const key of ACCRUAL_KEYS) {
    const code = key.startsWith('interest') ? 'DUE_MISMATCH' : 'LIABILITY_MISMATCH';
    cmpU128Field(exp.liabilities.accrual[key], obs.liabilities.accrual[key], `${path}.liabilities.accrual.${key}`, code, errors);
  }
  compareIndexed(exp.liabilities.dues, obs.liabilities.dues, `stages[${stageName}].liabilities.dues`, 'DUE_MISMATCH', 'DUE_MISMATCH', (a, b, p) => {
    cmpString(a.kind, b.kind, `${p}.kind`, 'DUE_MISMATCH', errors);
    cmpString(a.debtor, b.debtor, `${p}.debtor`, 'DUE_MISMATCH', errors);
    cmpString(a.creditor, b.creditor, `${p}.creditor`, 'DUE_MISMATCH', errors);
    cmpString(a.denomination, b.denomination, `${p}.denomination`, 'DENOMINATION_MISMATCH', errors);
    cmpU128Field(a.created, b.created, `${p}.created`, 'DUE_MISMATCH', errors);
    cmpU128Field(a.settled, b.settled, `${p}.settled`, 'DUE_MISMATCH', errors);
    cmpU128Field(a.outstanding, b.outstanding, `${p}.outstanding`, 'DUE_MISMATCH', errors);
  }, errors);
  compareIndexed(exp.residualDuties, obs.residualDuties, `stages[${stageName}].residualDuties`, 'RESIDUAL_DUTY_MISMATCH', 'RESIDUAL_DUTY_MISMATCH', (a, b, p) => {
    cmpString(a.kind, b.kind, `${p}.kind`, 'RESIDUAL_DUTY_MISMATCH', errors);
    cmpU128Field(a.amount, b.amount, `${p}.amount`, 'RESIDUAL_DUTY_MISMATCH', errors);
    cmpString(a.unit, b.unit, `${p}.unit`, 'RESIDUAL_DUTY_MISMATCH', errors);
    cmpString(a.status, b.status, `${p}.status`, 'RESIDUAL_DUTY_MISMATCH', errors);
  }, errors);
}

function compareRecords(expected, observed, errors) {
  cmpString(expected.agreement.name, observed.agreement.name, 'observed.agreement.name', 'VALUE_MISMATCH', errors);
  cmpString(expected.agreement.profile, observed.agreement.profile, 'observed.agreement.profile', 'VALUE_MISMATCH', errors);
  cmpU128Field(expected.agreement.lifetime, observed.agreement.lifetime, 'observed.agreement.lifetime', 'VALUE_MISMATCH', errors);
  cmpU128Field(expected.agreement.horizon, observed.agreement.horizon, 'observed.agreement.horizon', 'VALUE_MISMATCH', errors);
  if (expected.deploymentBinding.status !== observed.deploymentBinding.status) {
    push(errors, 'BINDING_MISMATCH', 'observed.deploymentBinding.status', 'deployment binding differs');
  }
  const expRoles = Object.keys(expected.deploymentBinding.roles).sort();
  const obsRoles = Object.keys(observed.deploymentBinding.roles).sort();
  if (JSON.stringify(expRoles) !== JSON.stringify(obsRoles)) {
    push(errors, 'BINDING_MISMATCH', 'observed.deploymentBinding.roles', 'role set differs');
  }
  compareIndexed(
    expected.deploymentBinding.assets,
    observed.deploymentBinding.assets,
    'deploymentBinding.assets',
    'BINDING_MISMATCH',
    'BINDING_MISMATCH',
    (a, b, p) => compareAsset(a, b, p, errors),
    errors
  );
  const expStages = new Map(expected.stages.map((stage) => [stage.stage, stage]));
  const obsStages = new Map(observed.stages.map((stage) => [stage.stage, stage]));
  for (const name of expStages.keys()) {
    if (!obsStages.has(name)) {
      push(errors, 'MISSING_FIELD', `observed.stages[${name}]`, `omitted stage ${name}`);
    } else {
      compareStage(expStages.get(name), obsStages.get(name), name, errors);
    }
  }
  for (const name of obsStages.keys()) {
    if (!expStages.has(name)) {
      push(errors, 'UNKNOWN_FIELD', `observed.stages[${name}]`, `extra stage ${name}`);
    }
  }
}

export function compareFinancialEffects(expected, observed) {
  const snapExpected = snapshot(expected);
  const snapObserved = snapshot(observed);
  const errors = [];
  const expectedOk = validateRecord(expected, 'expected', errors);
  const observedOk = validateRecord(observed, 'observed', errors);
  if (expectedOk) checkConservation(expected, 'expected', errors);
  if (observedOk) checkConservation(observed, 'observed', errors);
  if (expectedOk && observedOk) compareRecords(expected, observed, errors);
  if (snapshot(expected) !== snapExpected || snapshot(observed) !== snapObserved) {
    push(errors, 'INPUT_MUTATED', '', 'compareFinancialEffects mutated an input');
  }
  sortErrors(errors);
  return {
    ok: errors.length === 0,
    errors,
    networkAcceptance: false,
    networkEvidence: 'incompleteNetworkEvidence'
  };
}
