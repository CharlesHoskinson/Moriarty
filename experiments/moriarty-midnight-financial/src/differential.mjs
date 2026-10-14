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

const LOAN_PINS = {
  acceptedSourcePath: 'experiments/moriarty-language/spec/examples/loan.mori',
  acceptedSourceSha256: '1e1e61158ef80d44aa326399731440971fe50de7147ae5fb04e3fb36c48fef49',
  metadataInspectedPath: 'experiments/moriarty-language/compact/generated/loan/metadata.json',
  metadataInspectedSha256: '9e13a2677797e6328ad6b7b8d1a34a447d34a9ed9828e1c80867618040648ab5',
  metadataUsedForExpectations: false
};
const SWAP_PINS = {
  acceptedSourcePath: 'experiments/moriarty-language/spec/examples/swap.mori',
  acceptedSourceSha256: '0c2217365f2e518ec70835cf05a09150253d2df334e7dcf0504fd8c8d887051e',
  metadataInspectedPath: 'experiments/moriarty-language/compact/generated/swap/metadata.json',
  metadataInspectedSha256: 'e0b96181aad293cac32668cfaa3152c5c7cf38dca55e5f567616d17511fe7a9e',
  metadataUsedForExpectations: false
};

function gcd(a, b) {
  let x = a < 0n ? -a : a;
  let y = b < 0n ? -b : b;
  while (y !== 0n) {
    const t = x % y;
    x = y;
    y = t;
  }
  return x === 0n ? 1n : x;
}

function loanEconomics() {
  const notional = 5000000000n;
  const installment = 500000000n;
  const rateN = 8n;
  const rateD = 100n;
  const perN = 31n;
  const perD = 365n;
  const num = notional * rateN * perN;
  const den = rateD * perD;
  const interest = num / den;
  const remainder = num % den;
  const g = gcd(remainder, den);
  const total = installment + interest;
  const cash = 20000000000n;
  return {
    name: 'LoanFirstPeriod',
    profile: 'moriarty-bounded-atomic/1',
    lifetime: 2n,
    horizon: 2000000000n,
    stages: ['setup', 'accrue', 'settle'],
    roles: {
      borrower: { status: 'symbolic-not-address', logicalId: 'borrower' },
      lender: { status: 'symbolic-not-address', logicalId: 'lender' },
      'setup-mint': { status: 'synthetic-local-setup-source', logicalId: 'setup-mint' }
    },
    actors: ['borrower', 'lender'],
    roleNames: ['borrower', 'lender', 'setup-mint'],
    assets: [{
      id: 'USD_TEST_ASSET',
      logicalId: 'USD_TEST_ASSET',
      denomination: 'USD_micro',
      quantum: 1n,
      color: 'USD_TEST_ASSET',
      status: 'unresolved',
      unit: 'USD_micro'
    }],
    pins: LOAN_PINS,
    notional,
    installment,
    interest,
    remainderN: remainder / g,
    remainderD: den / g,
    rateN,
    rateD,
    perN,
    perD,
    total,
    remaining: notional - installment,
    cash,
    borrowerAfter: cash - total,
    debtId: 'LoanFirstPeriod:remaining-notional',
    duePR: 'lam01:period1:PR',
    dueIP: 'lam01:period1:IP'
  };
}

function swapEconomics() {
  const amountIn = 10000n;
  const reserveA = 1000000n;
  const reserveB = 2000000n;
  const traderA = 100000n;
  const feeN = 997n;
  const feeD = 1000n;
  const fee = amountIn - (amountIn * feeN) / feeD;
  const effective = amountIn * feeN;
  const output = (effective * reserveB) / (reserveA * feeD + effective);
  return {
    name: 'ConstantProductEpoch',
    profile: 'moriarty-bounded-atomic/1',
    lifetime: 8n,
    horizon: 2000000000n,
    stages: ['setup', 'swap', 'close'],
    roles: {
      trader: { status: 'symbolic-not-address', logicalId: 'trader' },
      pool: { status: 'symbolic-not-address', logicalId: 'pool' },
      provider: { status: 'symbolic-not-address', logicalId: 'provider' },
      'setup-mint': { status: 'synthetic-local-setup-source', logicalId: 'setup-mint' }
    },
    actors: ['trader', 'pool', 'provider'],
    roleNames: ['trader', 'pool', 'provider', 'setup-mint'],
    assets: [
      {
        id: 'ASSET_A',
        logicalId: 'ASSET_A',
        denomination: 'AssetA_quantum',
        quantum: 1n,
        color: 'ASSET_A',
        status: 'unresolved',
        unit: 'AssetA_quantum'
      },
      {
        id: 'ASSET_B',
        logicalId: 'ASSET_B',
        denomination: 'AssetB_quantum',
        quantum: 1n,
        color: 'ASSET_B',
        status: 'unresolved',
        unit: 'AssetB_quantum'
      }
    ],
    pins: SWAP_PINS,
    amountIn,
    reserveA,
    reserveB,
    traderA,
    fee,
    output,
    poolAfterA: reserveA + amountIn,
    poolAfterB: reserveB - output,
    traderAfterA: traderA - amountIn,
    traderAfterB: output
  };
}

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

function hasOwn(obj, key) {
  return isPlain(obj) && Object.prototype.hasOwnProperty.call(obj, key);
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
  const allow = new Set(required);
  for (const key of required) {
    if (!Object.prototype.hasOwnProperty.call(obj, key)) {
      push(errors, 'MISSING_FIELD', `${path}.${key}`, `missing ${key}`);
    }
  }
  for (const key of Object.keys(obj)) {
    if (!allow.has(key)) {
      push(errors, 'UNKNOWN_FIELD', `${path}.${key}`, `unknown field ${key}`);
    }
  }
  return true;
}

function hasRequired(obj, required) {
  return isPlain(obj) && required.every((key) => Object.prototype.hasOwnProperty.call(obj, key));
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
  if (!expectKeys(asset, path, ASSET_KEYS, errors)) return false;
  let ready = hasRequired(asset, ASSET_KEYS);
  expectString(asset.id, `${path}.id`, errors);
  expectString(asset.logicalId, `${path}.logicalId`, errors);
  expectString(asset.denomination, `${path}.denomination`, errors);
  expectString(asset.color, `${path}.color`, errors);
  const quantum = parseU128(asset.quantum, `${path}.quantum`, errors);
  if (quantum === 0n) {
    push(errors, 'QUANTUM_MISMATCH', `${path}.quantum`, 'quantum must be positive');
  }
  if (asset.status !== 'unresolved') {
    push(errors, 'BINDING_MISMATCH', `${path}.status`, 'deployment binding is unresolved');
  }
  if (typeof asset.id === 'string' && typeof asset.logicalId === 'string' && asset.id !== asset.logicalId) {
    push(errors, 'MALFORMED_STRUCTURE', `${path}.id`, 'asset id must equal logicalId');
  }
  return ready;
}

function validateRoleMap(roles, path, errors) {
  if (!isPlain(roles)) {
    push(errors, 'MALFORMED_STRUCTURE', path, 'role map required');
    return false;
  }
  let ready = true;
  for (const name of Object.keys(roles)) {
    const rolePath = `${path}.${name}`;
    if (!expectKeys(roles[name], rolePath, ROLE_KEYS, errors)) {
      ready = false;
      continue;
    }
    expectString(roles[name].logicalId, `${rolePath}.logicalId`, errors);
    const status = roles[name].status;
    if (status !== 'symbolic-not-address' && status !== 'synthetic-local-setup-source') {
      push(errors, 'BINDING_MISMATCH', `${rolePath}.status`, 'role is symbolic or synthetic-local');
    }
  }
  return ready;
}

function validateDue(due, path, errors) {
  if (!expectKeys(due, path, DUE_KEYS, errors)) return false;
  for (const key of ['id', 'dueId', 'kind', 'debtor', 'creditor', 'denomination']) {
    expectString(due[key], `${path}.${key}`, errors);
  }
  for (const key of ['created', 'settled', 'outstanding']) {
    parseU128(due[key], `${path}.${key}`, errors);
  }
  if (due.id !== due.dueId) {
    push(errors, 'MALFORMED_STRUCTURE', `${path}.id`, 'due id must equal dueId');
  }
  return hasRequired(due, DUE_KEYS);
}

function validateLiabilities(liab, path, errors) {
  if (!expectKeys(liab, path, LIAB_KEYS, errors)) return false;
  let ready = hasRequired(liab, LIAB_KEYS);
  parseU128(liab.nominalRemaining, `${path}.nominalRemaining`, errors);
  expectString(liab.denomination, `${path}.denomination`, errors);
  expectString(liab.debtId, `${path}.debtId`, errors);
  if (expectArray(liab.dues, `${path}.dues`, errors)) {
    indexById(liab.dues, `${path}.dues`, errors);
    liab.dues.forEach((due, i) => {
      if (!isPlain(due)) {
        push(errors, 'MALFORMED_STRUCTURE', `${path}.dues[${i}]`, 'due object required');
        ready = false;
        return;
      }
      if (!validateDue(due, `${path}.dues[${i}]`, errors)) ready = false;
    });
  } else {
    ready = false;
  }
  if (expectKeys(liab.principal, `${path}.principal`, PRINCIPAL_KEYS, errors)) {
    for (const key of PRINCIPAL_KEYS) parseU128(liab.principal[key], `${path}.principal.${key}`, errors);
    if (!hasRequired(liab.principal, PRINCIPAL_KEYS)) ready = false;
  } else {
    ready = false;
  }
  if (expectKeys(liab.accrual, `${path}.accrual`, ACCRUAL_KEYS, errors)) {
    for (const key of ACCRUAL_KEYS) parseU128(liab.accrual[key], `${path}.accrual.${key}`, errors);
    if (!hasRequired(liab.accrual, ACCRUAL_KEYS)) ready = false;
  } else {
    ready = false;
  }
  return ready;
}

function checkDuplicateStrings(list, path, errors) {
  if (!Array.isArray(list)) return;
  const seen = new Set();
  for (let i = 0; i < list.length; i += 1) {
    const value = list[i];
    if (typeof value !== 'string') continue;
    if (seen.has(value)) {
      push(errors, 'DUPLICATE_IDENTITY', `${path}[${i}]`, `duplicate identity ${value}`);
    }
    seen.add(value);
  }
}

function checkTransferOrdinals(transfers, path, errors) {
  if (!Array.isArray(transfers)) return;
  const seen = new Map();
  for (let i = 0; i < transfers.length; i += 1) {
    const transfer = transfers[i];
    if (!isPlain(transfer)) continue;
    const ordinal = transfer.ordinal;
    if (typeof ordinal !== 'string') continue;
    if (seen.has(ordinal)) {
      push(errors, 'DUPLICATE_IDENTITY', `${path}[${i}].ordinal`, `duplicate transfer ordinal ${ordinal}`);
    } else {
      seen.set(ordinal, i);
    }
  }
  for (let i = 0; i < transfers.length; i += 1) {
    const need = String(i);
    if (!seen.has(need)) {
      push(errors, 'TRANSFER_MISMATCH', `${path}[${i}].ordinal`, `transfer ordinals must be unique 0..n-1`);
    }
  }
}

function validateStage(stage, path, errors, agreementName) {
  if (!isPlain(stage)) {
    push(errors, 'MALFORMED_STRUCTURE', path, 'stage object required');
    return false;
  }
  expectKeys(stage, path, STAGE_KEYS, errors);
  let ready = hasRequired(stage, STAGE_KEYS);
  if ('stage' in stage) expectString(stage.stage, `${path}.stage`, errors);
  if ('revision' in stage) parseU128(stage.revision, `${path}.revision`, errors);
  if ('remaining' in stage) parseU128(stage.remaining, `${path}.remaining`, errors);
  if ('work' in stage) parseU128(stage.work, `${path}.work`, errors);
  const statusKeys = agreementName === 'LoanFirstPeriod' ? LOAN_STATUS_KEYS : SWAP_STATUS_KEYS;
  if (isPlain(stage.status)) {
    expectKeys(stage.status, `${path}.status`, statusKeys, errors);
    for (const key of statusKeys) {
      if (key in stage.status) parseU128(stage.status[key], `${path}.status.${key}`, errors);
    }
    if (!hasRequired(stage.status, statusKeys)) ready = false;
  } else if (hasOwn(stage, 'status')) {
    push(errors, 'MALFORMED_STRUCTURE', `${path}.status`, 'status object required');
    ready = false;
  } else {
    ready = false;
  }
  if (isPlain(stage.economicFee)) {
    expectKeys(stage.economicFee, `${path}.economicFee`, ECON_FEE_KEYS, errors);
    for (const key of ['actor', 'asset', 'color', 'unit']) {
      if (key in stage.economicFee) expectString(stage.economicFee[key], `${path}.economicFee.${key}`, errors);
    }
    if ('amount' in stage.economicFee) parseU128(stage.economicFee.amount, `${path}.economicFee.amount`, errors);
    if ('networkFeeSeparate' in stage.economicFee) {
      expectBool(stage.economicFee.networkFeeSeparate, `${path}.economicFee.networkFeeSeparate`, errors, true);
    }
    if (!hasRequired(stage.economicFee, ECON_FEE_KEYS)) ready = false;
  } else if (hasOwn(stage, 'economicFee')) {
    push(errors, 'MALFORMED_STRUCTURE', `${path}.economicFee`, 'economicFee object required');
    ready = false;
  } else {
    ready = false;
  }
  if (isPlain(stage.bindings)) {
    expectKeys(stage.bindings, `${path}.bindings`, BINDING_KEYS, errors);
    if (expectArray(stage.bindings.roles, `${path}.bindings.roles`, errors)) {
      stage.bindings.roles.forEach((role, i) => expectString(role, `${path}.bindings.roles[${i}]`, errors));
      checkDuplicateStrings(stage.bindings.roles, `${path}.bindings.roles`, errors);
    } else {
      ready = false;
    }
    if (expectArray(stage.bindings.assets, `${path}.bindings.assets`, errors)) {
      indexById(stage.bindings.assets, `${path}.bindings.assets`, errors);
      stage.bindings.assets.forEach((asset, i) => {
        if (!validateAsset(asset, `${path}.bindings.assets[${i}]`, errors)) ready = false;
      });
    } else {
      ready = false;
    }
  } else if (hasOwn(stage, 'bindings')) {
    push(errors, 'MALFORMED_STRUCTURE', `${path}.bindings`, 'bindings object required');
    ready = false;
  } else {
    ready = false;
  }
  if (isPlain(stage.balances)) {
    expectKeys(stage.balances, `${path}.balances`, BALANCE_BLOCK_KEYS, errors);
    for (const side of ['pre', 'post']) {
      const sidePath = `${path}.balances.${side}`;
      if (!expectArray(stage.balances[side], sidePath, errors)) {
        ready = false;
        continue;
      }
      indexById(stage.balances[side], sidePath, errors);
      stage.balances[side].forEach((row, i) => {
        const rowPath = `${sidePath}[${i}]`;
        if (!isPlain(row)) {
          push(errors, 'MALFORMED_STRUCTURE', rowPath, 'balance object required');
          ready = false;
          return;
        }
        expectKeys(row, rowPath, BALANCE_KEYS, errors);
        for (const key of ['id', 'actor', 'asset', 'unit']) {
          if (key in row) expectString(row[key], `${rowPath}.${key}`, errors);
        }
        if ('amount' in row) parseU128(row.amount, `${rowPath}.amount`, errors);
        if (row.id !== `${row.actor}|${row.asset}|${row.unit}`) {
          push(errors, 'MALFORMED_STRUCTURE', `${rowPath}.id`, 'balance id must be actor|asset|unit');
        }
      });
    }
  } else if (hasOwn(stage, 'balances')) {
    push(errors, 'MALFORMED_STRUCTURE', `${path}.balances`, 'balances object required');
    ready = false;
  } else {
    ready = false;
  }
  if (expectArray(stage.transfers, `${path}.transfers`, errors)) {
    indexById(stage.transfers, `${path}.transfers`, errors);
    stage.transfers.forEach((transfer, i) => {
      const tPath = `${path}.transfers[${i}]`;
      if (!isPlain(transfer)) {
        push(errors, 'MALFORMED_STRUCTURE', tPath, 'transfer object required');
        ready = false;
        return;
      }
      expectKeys(transfer, tPath, TRANSFER_KEYS, errors);
      for (const key of ['id', 'from', 'to', 'color', 'unit']) {
        if (key in transfer) expectString(transfer[key], `${tPath}.${key}`, errors);
      }
      if ('ordinal' in transfer) parseU128(transfer.ordinal, `${tPath}.ordinal`, errors);
      if ('amount' in transfer) parseU128(transfer.amount, `${tPath}.amount`, errors);
    });
    checkTransferOrdinals(stage.transfers, `${path}.transfers`, errors);
  } else {
    ready = false;
  }
  if (expectArray(stage.actorEffects, `${path}.actorEffects`, errors)) {
    indexById(stage.actorEffects, `${path}.actorEffects`, errors);
    stage.actorEffects.forEach((effect, i) => {
      const ePath = `${path}.actorEffects[${i}]`;
      if (!isPlain(effect)) {
        push(errors, 'MALFORMED_STRUCTURE', ePath, 'actor effect object required');
        ready = false;
        return;
      }
      expectKeys(effect, ePath, EFFECT_KEYS, errors);
      for (const key of ['id', 'actor', 'asset', 'unit']) {
        if (key in effect) expectString(effect[key], `${ePath}.${key}`, errors);
      }
      for (const key of ['grossDebit', 'refund', 'netCredit', 'fee']) {
        if (key in effect) parseU128(effect[key], `${ePath}.${key}`, errors);
      }
      if (effect.id !== `${effect.actor}|${effect.asset}|${effect.unit}`) {
        push(errors, 'MALFORMED_STRUCTURE', `${ePath}.id`, 'effect id must be actor|asset|unit');
      }
    });
  } else {
    ready = false;
  }
  if (isPlain(stage.liabilities)) {
    if (!validateLiabilities(stage.liabilities, `${path}.liabilities`, errors)) ready = false;
  } else if (hasOwn(stage, 'liabilities')) {
    push(errors, 'MALFORMED_STRUCTURE', `${path}.liabilities`, 'liabilities object required');
    ready = false;
  } else {
    ready = false;
  }
  if (expectArray(stage.residualDuties, `${path}.residualDuties`, errors)) {
    indexById(stage.residualDuties, `${path}.residualDuties`, errors);
    stage.residualDuties.forEach((duty, i) => {
      const dPath = `${path}.residualDuties[${i}]`;
      if (!isPlain(duty)) {
        push(errors, 'MALFORMED_STRUCTURE', dPath, 'residual duty object required');
        ready = false;
        return;
      }
      expectKeys(duty, dPath, RESIDUAL_KEYS, errors);
      for (const key of ['id', 'kind', 'unit', 'status']) {
        if (key in duty) expectString(duty[key], `${dPath}.${key}`, errors);
      }
      if ('amount' in duty) parseU128(duty.amount, `${dPath}.amount`, errors);
    });
  } else {
    ready = false;
  }
  return ready;
}

function validateRecord(record, label, errors) {
  if (!isPlain(record)) {
    push(errors, 'MALFORMED_STRUCTURE', label, 'financial record object required');
    return false;
  }
  expectKeys(record, label, RECORD_KEYS, errors);
  let ready = hasRequired(record, RECORD_KEYS);
  if (record.schemaVersion !== SCHEMA_VERSION) {
    push(errors, 'VALUE_MISMATCH', `${label}.schemaVersion`, 'unsupported schema');
  }
  if (record.observationKind !== 'synthetic-local') {
    push(errors, 'VALUE_MISMATCH', `${label}.observationKind`, 'observationKind must be synthetic-local');
  }
  if ('networkAcceptance' in record) {
    expectBool(record.networkAcceptance, `${label}.networkAcceptance`, errors, false);
  }
  if (record.networkEvidence !== 'incompleteNetworkEvidence') {
    push(errors, 'VALUE_MISMATCH', `${label}.networkEvidence`, 'network evidence is incomplete');
  }
  if ('derivationNote' in record) expectString(record.derivationNote, `${label}.derivationNote`, errors);
  if (isPlain(record.sourcePins)) {
    expectKeys(record.sourcePins, `${label}.sourcePins`, SOURCE_PIN_KEYS, errors);
    for (const key of ['acceptedSourcePath', 'acceptedSourceSha256', 'metadataInspectedPath', 'metadataInspectedSha256']) {
      if (key in record.sourcePins) expectString(record.sourcePins[key], `${label}.sourcePins.${key}`, errors);
    }
    if ('metadataUsedForExpectations' in record.sourcePins) {
      expectBool(record.sourcePins.metadataUsedForExpectations, `${label}.sourcePins.metadataUsedForExpectations`, errors, false);
    }
    if (!hasRequired(record.sourcePins, SOURCE_PIN_KEYS)) ready = false;
  } else if (hasOwn(record, 'sourcePins')) {
    push(errors, 'MALFORMED_STRUCTURE', `${label}.sourcePins`, 'sourcePins object required');
    ready = false;
  } else {
    ready = false;
  }
  if (isPlain(record.deploymentBinding)) {
    expectKeys(record.deploymentBinding, `${label}.deploymentBinding`, DEPLOY_KEYS, errors);
    if (record.deploymentBinding.status !== 'unresolved') {
      push(errors, 'BINDING_MISMATCH', `${label}.deploymentBinding.status`, 'deployment binding is unresolved');
    }
    if (!validateRoleMap(record.deploymentBinding.roles, `${label}.deploymentBinding.roles`, errors)) {
      ready = false;
    }
    if (expectArray(record.deploymentBinding.assets, `${label}.deploymentBinding.assets`, errors)) {
      indexById(record.deploymentBinding.assets, `${label}.deploymentBinding.assets`, errors);
      record.deploymentBinding.assets.forEach((asset, i) => {
        if (!validateAsset(asset, `${label}.deploymentBinding.assets[${i}]`, errors)) ready = false;
      });
    } else {
      ready = false;
    }
    if (!hasRequired(record.deploymentBinding, DEPLOY_KEYS)) ready = false;
  } else if (hasOwn(record, 'deploymentBinding')) {
    push(errors, 'MALFORMED_STRUCTURE', `${label}.deploymentBinding`, 'deploymentBinding object required');
    ready = false;
  } else {
    ready = false;
  }
  if (isPlain(record.networkFeeAccounting)) {
    expectKeys(record.networkFeeAccounting, `${label}.networkFeeAccounting`, FEE_ACC_KEYS, errors);
    if (record.networkFeeAccounting.status !== 'unresolved') {
      push(errors, 'VALUE_MISMATCH', `${label}.networkFeeAccounting.status`, 'network fee accounting is unresolved');
    }
    expectNull(record.networkFeeAccounting.midnightFeeAsset, `${label}.networkFeeAccounting.midnightFeeAsset`, errors);
    expectNull(record.networkFeeAccounting.midnightFeeUnit, `${label}.networkFeeAccounting.midnightFeeUnit`, errors);
    expectBool(record.networkFeeAccounting.assumedZero, `${label}.networkFeeAccounting.assumedZero`, errors, false);
    if (!hasRequired(record.networkFeeAccounting, FEE_ACC_KEYS)) ready = false;
  } else if (hasOwn(record, 'networkFeeAccounting')) {
    push(errors, 'MALFORMED_STRUCTURE', `${label}.networkFeeAccounting`, 'networkFeeAccounting object required');
    ready = false;
  } else {
    ready = false;
  }
  if (isPlain(record.agreement)) {
    expectKeys(record.agreement, `${label}.agreement`, AGREEMENT_KEYS, errors);
    for (const key of ['name', 'profile']) {
      if (key in record.agreement) expectString(record.agreement[key], `${label}.agreement.${key}`, errors);
    }
    if ('lifetime' in record.agreement) parseU128(record.agreement.lifetime, `${label}.agreement.lifetime`, errors);
    if ('horizon' in record.agreement) parseU128(record.agreement.horizon, `${label}.agreement.horizon`, errors);
    if (!hasRequired(record.agreement, AGREEMENT_KEYS)) ready = false;
  } else if (hasOwn(record, 'agreement')) {
    push(errors, 'MALFORMED_STRUCTURE', `${label}.agreement`, 'agreement object required');
    ready = false;
  } else {
    ready = false;
  }
  const agreementName = isPlain(record.agreement) ? record.agreement.name : undefined;
  if (expectArray(record.stages, `${label}.stages`, errors)) {
    const names = new Set();
    record.stages.forEach((stage, i) => {
      const path = `${label}.stages[${isPlain(stage) && typeof stage.stage === 'string' ? stage.stage : i}]`;
      if (isPlain(stage) && typeof stage.stage === 'string') {
        if (names.has(stage.stage)) {
          push(errors, 'DUPLICATE_IDENTITY', path, `duplicate stage ${stage.stage}`);
        }
        names.add(stage.stage);
      }
      if (!validateStage(stage, path, errors, agreementName)) ready = false;
    });
  } else {
    ready = false;
  }
  return ready;
}

function addAmount(map, key, amount) {
  map.set(key, (map.get(key) || 0n) + amount);
}

function requireValue(actual, expected, path, code, errors) {
  if (actual !== expected) {
    push(errors, code, path, `expected ${JSON.stringify(expected)} observed ${JSON.stringify(actual)}`);
  }
}

function requireU128(actual, expected, path, code, errors) {
  requireValue(actual, String(expected), path, code, errors);
}

function stagesByName(record) {
  const map = new Map();
  if (!Array.isArray(record.stages)) return map;
  for (const stage of record.stages) {
    if (isPlain(stage) && typeof stage.stage === 'string') map.set(stage.stage, stage);
  }
  return map;
}

function rowId(actor, asset, unit) {
  return `${actor}|${asset}|${unit}`;
}

function checkAdmittedPins(pins, path, errors, admitted) {
  if (!isPlain(pins)) return;
  for (const key of SOURCE_PIN_KEYS) {
    requireValue(pins[key], admitted[key], `${path}.${key}`, 'VALUE_MISMATCH', errors);
  }
}

function checkAdmittedRoles(roles, path, errors, admitted) {
  if (!isPlain(roles)) return;
  const expNames = Object.keys(admitted).sort();
  const obsNames = Object.keys(roles).sort();
  if (JSON.stringify(expNames) !== JSON.stringify(obsNames)) {
    push(errors, 'BINDING_MISMATCH', path, 'role set must match the admitted case');
  }
  for (const name of expNames) {
    if (!isPlain(roles[name])) continue;
    requireValue(roles[name].logicalId, admitted[name].logicalId, `${path}.${name}.logicalId`, 'BINDING_MISMATCH', errors);
    requireValue(roles[name].status, admitted[name].status, `${path}.${name}.status`, 'BINDING_MISMATCH', errors);
  }
}

function checkAdmittedAssets(list, path, errors, admitted) {
  if (!Array.isArray(list)) return;
  const map = indexById(list, path, []);
  for (const asset of admitted) {
    const found = map.get(asset.id);
    if (!found) {
      push(errors, 'BINDING_MISMATCH', `${path}[${asset.id}]`, `missing admitted asset ${asset.id}`);
      continue;
    }
    requireValue(found.logicalId, asset.logicalId, `${path}[${asset.id}].logicalId`, 'BINDING_MISMATCH', errors);
    requireValue(found.denomination, asset.denomination, `${path}[${asset.id}].denomination`, 'DENOMINATION_MISMATCH', errors);
    requireU128(found.quantum, asset.quantum, `${path}[${asset.id}].quantum`, 'QUANTUM_MISMATCH', errors);
    requireValue(found.color, asset.color, `${path}[${asset.id}].color`, 'COLOR_MISMATCH', errors);
    requireValue(found.status, asset.status, `${path}[${asset.id}].status`, 'BINDING_MISMATCH', errors);
  }
  for (const id of map.keys()) {
    if (!admitted.some((asset) => asset.id === id)) {
      push(errors, 'BINDING_MISMATCH', `${path}[${id}]`, `extra asset ${id}`);
    }
  }
}

function checkLifecycle(record, label, errors, lifetime, stageNames) {
  const stages = stagesByName(record);
  for (let i = 0; i < stageNames.length; i += 1) {
    const name = stageNames[i];
    const stage = stages.get(name);
    if (!stage) {
      push(errors, 'MISSING_FIELD', `${label}.stages[${name}]`, `missing stage ${name}`);
      continue;
    }
    const path = `${label}.stages[${name}]`;
    requireU128(stage.revision, BigInt(i), `${path}.revision`, 'REVISION_MISMATCH', errors);
    const expectedRemaining = lifetime - BigInt(i);
    requireU128(stage.remaining, expectedRemaining, `${path}.remaining`, 'REMAINING_MISMATCH', errors);
    requireU128(stage.work, expectedRemaining, `${path}.work`, 'WORK_MISMATCH', errors);
  }
  for (const name of stages.keys()) {
    if (!stageNames.includes(name)) {
      push(errors, 'UNKNOWN_FIELD', `${label}.stages[${name}]`, `extra stage ${name}`);
    }
  }
}

function checkContinuity(record, label, errors, stageNames) {
  const stages = stagesByName(record);
  for (let i = 0; i < stageNames.length - 1; i += 1) {
    const current = stages.get(stageNames[i]);
    const next = stages.get(stageNames[i + 1]);
    if (!isPlain(current) || !isPlain(next) || !isPlain(current.balances) || !isPlain(next.balances)) continue;
    const post = indexById(current.balances.post, `${label}.stages[${stageNames[i]}].balances.post`, []);
    const pre = indexById(next.balances.pre, `${label}.stages[${stageNames[i + 1]}].balances.pre`, []);
    const ids = new Set([...post.keys(), ...pre.keys()]);
    for (const id of ids) {
      const postAmt = post.has(id) ? post.get(id).amount : undefined;
      const preAmt = pre.has(id) ? pre.get(id).amount : undefined;
      if (postAmt !== preAmt) {
        push(
          errors,
          'VALUE_MISMATCH',
          `${label}.stages[${stageNames[i + 1]}].balances.pre[${id}].amount`,
          'stage pre balance must continue previous post'
        );
      }
    }
  }
}

function checkCoverage(stage, path, errors, actors, assets) {
  if (!isPlain(stage)) return;
  const required = [];
  for (const actor of actors) {
    for (const asset of assets) {
      required.push(rowId(actor, asset.id, asset.unit));
    }
  }
  const requiredSet = new Set(required);
  if (isPlain(stage.balances)) {
    for (const side of ['pre', 'post']) {
      const list = stage.balances[side];
      if (!Array.isArray(list)) {
        push(errors, 'MISSING_FIELD', `${path}.balances.${side}`, `missing ${side} balances`);
        continue;
      }
      const ids = new Set(list.filter(isPlain).map((row) => row.id));
      for (const id of required) {
        if (!ids.has(id)) {
          push(errors, 'MISSING_FIELD', `${path}.balances.${side}`, `missing required balance ${id}`);
        }
      }
      for (const id of ids) {
        if (!requiredSet.has(id)) {
          push(errors, 'BINDING_MISMATCH', `${path}.balances.${side}[${id}]`, `extra balance ${id}`);
        }
      }
    }
  } else {
    push(errors, 'MISSING_FIELD', `${path}.balances`, 'missing required balances');
  }
  if (!Array.isArray(stage.actorEffects)) {
    push(errors, 'MISSING_FIELD', `${path}.actorEffects`, 'missing required actor effects');
    return;
  }
  const effectIds = new Set(stage.actorEffects.filter(isPlain).map((row) => row.id));
  for (const id of required) {
    if (!effectIds.has(id)) {
      push(errors, 'MISSING_FIELD', `${path}.actorEffects`, `missing required actor effect ${id}`);
    }
  }
  for (const id of effectIds) {
    if (!requiredSet.has(id)) {
      push(errors, 'BINDING_MISMATCH', `${path}.actorEffects[${id}]`, `extra actor effect ${id}`);
    }
  }
}

function checkStageBindings(stage, path, errors, roleNames, assets) {
  if (!isPlain(stage) || !isPlain(stage.bindings)) return;
  if (Array.isArray(stage.bindings.roles)) {
    const sortedObs = [...stage.bindings.roles].filter((item) => typeof item === 'string').sort();
    const uniqueObs = [...new Set(sortedObs)];
    const sortedExp = [...roleNames].sort();
    if (JSON.stringify(uniqueObs) !== JSON.stringify(sortedExp)) {
      push(errors, 'BINDING_MISMATCH', `${path}.bindings.roles`, 'stage roles must match the admitted case');
    }
  }
  checkAdmittedAssets(stage.bindings.assets, `${path}.bindings.assets`, errors, assets);
}

function checkClosedReferences(stage, path, errors, roleNames, actors, assets) {
  if (!isPlain(stage)) return;
  const actorSet = new Set(actors);
  const roleSet = new Set(roleNames);
  const assetSet = new Set(assets.map((item) => item.id));
  const unitByAsset = new Map(assets.map((item) => [item.id, item.unit]));
  const colorSet = new Set(assets.map((item) => item.color));

  function participant(value, fieldPath, allowMint) {
    if (typeof value !== 'string') return;
    if (allowMint && value === 'setup-mint' && roleSet.has('setup-mint')) return;
    if (!actorSet.has(value)) {
      push(errors, 'BINDING_MISMATCH', fieldPath, `unknown financial participant ${value}`);
    }
  }

  function assetRef(value, fieldPath) {
    if (typeof value !== 'string') return;
    if (!assetSet.has(value)) {
      push(errors, 'BINDING_MISMATCH', fieldPath, `unknown asset ${value}`);
    }
  }

  function unitRef(value, assetId, fieldPath) {
    if (typeof value !== 'string') return;
    const expectedUnit = unitByAsset.get(assetId);
    if (expectedUnit === undefined || value !== expectedUnit) {
      push(errors, 'DENOMINATION_MISMATCH', fieldPath, `unit must match admitted asset ${assetId}`);
    }
  }

  function colorRef(value, fieldPath) {
    if (typeof value !== 'string') return;
    if (!colorSet.has(value)) {
      push(errors, 'COLOR_MISMATCH', fieldPath, `unknown color ${value}`);
    }
  }

  if (isPlain(stage.economicFee)) {
    participant(stage.economicFee.actor, `${path}.economicFee.actor`, false);
    assetRef(stage.economicFee.asset, `${path}.economicFee.asset`);
    colorRef(stage.economicFee.color, `${path}.economicFee.color`);
    unitRef(stage.economicFee.unit, stage.economicFee.asset, `${path}.economicFee.unit`);
  }
  if (isPlain(stage.balances)) {
    for (const side of ['pre', 'post']) {
      const list = stage.balances[side];
      if (!Array.isArray(list)) continue;
      for (const row of list) {
        if (!isPlain(row)) continue;
        const rowPath = `${path}.balances.${side}[${row.id || ''}]`;
        participant(row.actor, `${rowPath}.actor`, false);
        assetRef(row.asset, `${rowPath}.asset`);
        unitRef(row.unit, row.asset, `${rowPath}.unit`);
      }
    }
  }
  if (Array.isArray(stage.actorEffects)) {
    for (const row of stage.actorEffects) {
      if (!isPlain(row)) continue;
      const rowPath = `${path}.actorEffects[${row.id || ''}]`;
      participant(row.actor, `${rowPath}.actor`, false);
      assetRef(row.asset, `${rowPath}.asset`);
      unitRef(row.unit, row.asset, `${rowPath}.unit`);
    }
  }
  if (Array.isArray(stage.transfers)) {
    for (const transfer of stage.transfers) {
      if (!isPlain(transfer)) continue;
      const tPath = `${path}.transfers[${transfer.id || ''}]`;
      participant(transfer.from, `${tPath}.from`, true);
      participant(transfer.to, `${tPath}.to`, false);
      colorRef(transfer.color, `${tPath}.color`);
      unitRef(transfer.unit, transfer.color, `${tPath}.unit`);
    }
  }
  if (isPlain(stage.liabilities) && Array.isArray(stage.liabilities.dues)) {
    for (const due of stage.liabilities.dues) {
      if (!isPlain(due)) continue;
      const duePath = `${path}.liabilities.dues[${due.id || ''}]`;
      participant(due.debtor, `${duePath}.debtor`, false);
      participant(due.creditor, `${duePath}.creditor`, false);
    }
  }
}

function lookupBalance(list, id) {
  if (!Array.isArray(list)) return undefined;
  const found = list.find((row) => isPlain(row) && row.id === id);
  return found ? found.amount : undefined;
}

function lookupEffect(list, id) {
  if (!Array.isArray(list)) return undefined;
  return list.find((row) => isPlain(row) && row.id === id);
}

function requireBalance(stage, path, actor, asset, unit, side, amount, errors) {
  const id = rowId(actor, asset, unit);
  const actual = lookupBalance(stage.balances && stage.balances[side], id);
  requireU128(actual, amount, `${path}.balances.${side}[${id}].amount`, 'VALUE_MISMATCH', errors);
}

function requireEffect(stage, path, actor, asset, unit, fields, errors) {
  const id = rowId(actor, asset, unit);
  const effect = lookupEffect(stage.actorEffects, id);
  if (!effect) {
    push(errors, 'MISSING_FIELD', `${path}.actorEffects[${id}]`, `missing actor effect ${id}`);
    return;
  }
  for (const [key, amount] of Object.entries(fields)) {
    const code = key === 'grossDebit'
      ? 'GROSS_DEBIT_MISMATCH'
      : key === 'refund'
        ? 'REFUND_MISMATCH'
        : key === 'netCredit'
          ? 'NET_CREDIT_MISMATCH'
          : 'FEE_MISMATCH';
    requireU128(effect[key], amount, `${path}.actorEffects[${id}].${key}`, code, errors);
  }
}

function requireFee(stage, path, errors, amount, actor, asset, unit, color) {
  if (!isPlain(stage.economicFee)) return;
  requireValue(stage.economicFee.actor, actor, `${path}.economicFee.actor`, 'FEE_MISMATCH', errors);
  requireValue(stage.economicFee.asset, asset, `${path}.economicFee.asset`, 'FEE_MISMATCH', errors);
  requireValue(stage.economicFee.color, color, `${path}.economicFee.color`, 'COLOR_MISMATCH', errors);
  requireValue(stage.economicFee.unit, unit, `${path}.economicFee.unit`, 'DENOMINATION_MISMATCH', errors);
  requireU128(stage.economicFee.amount, amount, `${path}.economicFee.amount`, 'FEE_MISMATCH', errors);
  const id = rowId(actor, asset, unit);
  const effect = lookupEffect(stage.actorEffects, id);
  if (effect) {
    requireU128(effect.fee, amount, `${path}.actorEffects[${id}].fee`, 'FEE_MISMATCH', errors);
    if (stage.economicFee.amount !== effect.fee) {
      push(errors, 'FEE_MISMATCH', `${path}.economicFee.amount`, 'economic fee must equal actor fee effect');
    }
  }
}

function findTransfer(stage, from, to, color) {
  if (!Array.isArray(stage.transfers)) return undefined;
  return stage.transfers.find((item) => (
    isPlain(item) && item.from === from && item.to === to && item.color === color
  ));
}

function requireTransfer(stage, path, spec, errors) {
  const found = findTransfer(stage, spec.from, spec.to, spec.color);
  if (!found) {
    push(errors, 'OMITTED_TRANSFER', `${path}.transfers`, `missing transfer ${spec.from}->${spec.to} ${spec.color}`);
    return;
  }
  if (spec.id) {
    requireValue(found.id, spec.id, `${path}.transfers[${found.id || ''}].id`, 'TRANSFER_MISMATCH', errors);
  }
  requireU128(found.ordinal, spec.ordinal, `${path}.transfers[${found.id || ''}].ordinal`, 'TRANSFER_MISMATCH', errors);
  requireValue(found.unit, spec.unit, `${path}.transfers[${found.id || ''}].unit`, 'DENOMINATION_MISMATCH', errors);
  requireU128(found.amount, spec.amount, `${path}.transfers[${found.id || ''}].amount`, 'TRANSFER_MISMATCH', errors);
}

function requireTransferSet(stage, path, specs, errors) {
  for (const spec of specs) requireTransfer(stage, path, spec, errors);
  if (!Array.isArray(stage.transfers)) return;
  const allowedIds = new Set(specs.map((item) => item.id));
  const allowedKeys = new Set(specs.map((item) => `${item.from}|${item.to}|${item.color}`));
  for (const transfer of stage.transfers) {
    if (!isPlain(transfer)) continue;
    const key = `${transfer.from}|${transfer.to}|${transfer.color}`;
    if (!allowedIds.has(transfer.id) || !allowedKeys.has(key)) {
      push(errors, 'EXTRA_TRANSFER', `${path}.transfers[${transfer.id || ''}]`, `extra transfer ${transfer.id || key}`);
    }
  }
}

function requireDuty(stage, path, spec, errors) {
  const duties = indexById(stage.residualDuties || [], `${path}.residualDuties`, []);
  const found = duties.get(spec.id);
  if (!found) {
    push(errors, 'RESIDUAL_DUTY_MISMATCH', `${path}.residualDuties[${spec.id}]`, `omitted residual duty ${spec.id}`);
    return;
  }
  requireValue(found.kind, spec.kind, `${path}.residualDuties[${spec.id}].kind`, 'RESIDUAL_DUTY_MISMATCH', errors);
  requireU128(found.amount, spec.amount, `${path}.residualDuties[${spec.id}].amount`, 'RESIDUAL_DUTY_MISMATCH', errors);
  requireValue(found.unit, spec.unit, `${path}.residualDuties[${spec.id}].unit`, 'RESIDUAL_DUTY_MISMATCH', errors);
  requireValue(found.status, spec.status, `${path}.residualDuties[${spec.id}].status`, 'RESIDUAL_DUTY_MISMATCH', errors);
}

function requireDue(stage, path, spec, errors) {
  const dues = indexById(stage.liabilities && stage.liabilities.dues, `${path}.liabilities.dues`, []);
  const found = dues.get(spec.id);
  if (!found) {
    push(errors, 'DUE_MISMATCH', `${path}.liabilities.dues[${spec.id}]`, `omitted due ${spec.id}`);
    return;
  }
  requireValue(found.kind, spec.kind, `${path}.liabilities.dues[${spec.id}].kind`, 'DUE_MISMATCH', errors);
  requireValue(found.debtor, spec.debtor, `${path}.liabilities.dues[${spec.id}].debtor`, 'DUE_MISMATCH', errors);
  requireValue(found.creditor, spec.creditor, `${path}.liabilities.dues[${spec.id}].creditor`, 'DUE_MISMATCH', errors);
  requireValue(found.denomination, spec.denomination, `${path}.liabilities.dues[${spec.id}].denomination`, 'DENOMINATION_MISMATCH', errors);
  requireU128(found.created, spec.created, `${path}.liabilities.dues[${spec.id}].created`, 'DUE_MISMATCH', errors);
  requireU128(found.settled, spec.settled, `${path}.liabilities.dues[${spec.id}].settled`, 'DUE_MISMATCH', errors);
  requireU128(found.outstanding, spec.outstanding, `${path}.liabilities.dues[${spec.id}].outstanding`, 'DUE_MISMATCH', errors);
}

function requireDutySet(stage, path, specs, errors) {
  for (const spec of specs) requireDuty(stage, path, spec, errors);
  const duties = Array.isArray(stage.residualDuties) ? stage.residualDuties : [];
  const allowed = new Set(specs.map((item) => item.id));
  const map = indexById(duties, `${path}.residualDuties`, []);
  for (const id of map.keys()) {
    if (!allowed.has(id)) {
      push(errors, 'RESIDUAL_DUTY_MISMATCH', `${path}.residualDuties[${id}]`, `extra residual duty ${id}`);
    }
  }
}

function requireDueSet(stage, path, specs, errors) {
  for (const spec of specs) requireDue(stage, path, spec, errors);
  const dues = isPlain(stage.liabilities) && Array.isArray(stage.liabilities.dues)
    ? stage.liabilities.dues
    : [];
  const allowed = new Set(specs.map((item) => item.id));
  const map = indexById(dues, `${path}.liabilities.dues`, []);
  for (const id of map.keys()) {
    if (!allowed.has(id)) {
      push(errors, 'DUE_MISMATCH', `${path}.liabilities.dues[${id}]`, `extra due ${id}`);
    }
  }
}

function checkMintException(stage, path, errors, allowed) {
  if (!Array.isArray(stage.transfers)) return;
  for (const transfer of stage.transfers) {
    if (!isPlain(transfer)) continue;
    if (transfer.from !== 'setup-mint') continue;
    const ok = allowed.some((item) => (
      item.to === transfer.to && item.color === transfer.color && String(item.amount) === transfer.amount
    ));
    if (stage.stage !== 'setup' || !ok) {
      push(errors, 'BINDING_MISMATCH', `${path}.transfers`, 'setup-mint is limited to declared setup transfers');
    }
  }
}

function checkConservationStage(stage, path, errors) {
  if (!isPlain(stage) || !isPlain(stage.balances) || !Array.isArray(stage.transfers) || !Array.isArray(stage.actorEffects)) {
    return;
  }
  const outgoing = new Map();
  const incoming = new Map();
  for (const transfer of stage.transfers) {
    if (!isPlain(transfer) || !U128_RE.test(transfer.amount)) continue;
    const amount = BigInt(transfer.amount);
    if (amount > U128_MAX) continue;
    addAmount(outgoing, `${transfer.from}|${transfer.color}|${transfer.unit}`, amount);
    addAmount(incoming, `${transfer.to}|${transfer.color}|${transfer.unit}`, amount);
    if (transfer.from !== 'setup-mint') {
      const fromId = `${transfer.from}|${transfer.color}|${transfer.unit}`;
      const preHas = Array.isArray(stage.balances.pre) && stage.balances.pre.some((row) => isPlain(row) && row.id === fromId);
      const effectHas = stage.actorEffects.some((row) => isPlain(row) && row.id === fromId);
      if (!preHas) push(errors, 'MISSING_FIELD', `${path}.balances.pre`, `missing transfer participant ${fromId}`);
      if (!effectHas) push(errors, 'MISSING_FIELD', `${path}.actorEffects`, `missing transfer participant ${fromId}`);
    }
    const toId = `${transfer.to}|${transfer.color}|${transfer.unit}`;
    const preTo = Array.isArray(stage.balances.pre) && stage.balances.pre.some((row) => isPlain(row) && row.id === toId);
    const effectTo = stage.actorEffects.some((row) => isPlain(row) && row.id === toId);
    if (!preTo) push(errors, 'MISSING_FIELD', `${path}.balances.pre`, `missing transfer participant ${toId}`);
    if (!effectTo) push(errors, 'MISSING_FIELD', `${path}.actorEffects`, `missing transfer participant ${toId}`);
  }
  const pre = indexById(stage.balances.pre, `${path}.balances.pre`, []);
  const post = indexById(stage.balances.post, `${path}.balances.post`, []);
  for (const id of pre.keys()) {
    if (!post.has(id)) {
      push(errors, 'MISSING_FIELD', `${path}.balances.post[${id}]`, 'post balance missing');
      continue;
    }
    const preAmt = parseU128(pre.get(id).amount, `${path}.balances.pre[${id}].amount`, errors);
    const postAmt = parseU128(post.get(id).amount, `${path}.balances.post[${id}].amount`, errors);
    if (preAmt === null || postAmt === null) continue;
    const out = outgoing.get(id) || 0n;
    const inn = incoming.get(id) || 0n;
    if (postAmt !== preAmt - out + inn) {
      push(errors, 'BALANCE_INCONSISTENT', `${path}.balances.post[${id}].amount`, 'post balance does not match transfers');
    }
  }
  for (const id of post.keys()) {
    if (!pre.has(id)) {
      push(errors, 'MISSING_FIELD', `${path}.balances.pre[${id}]`, 'pre balance missing');
    }
  }
  const effects = indexById(stage.actorEffects, `${path}.actorEffects`, []);
  for (const [id, effect] of effects) {
    const gross = parseU128(effect.grossDebit, `${path}.actorEffects[${id}].grossDebit`, errors);
    const refund = parseU128(effect.refund, `${path}.actorEffects[${id}].refund`, errors);
    const net = parseU128(effect.netCredit, `${path}.actorEffects[${id}].netCredit`, errors);
    const fee = parseU128(effect.fee, `${path}.actorEffects[${id}].fee`, errors);
    if (gross === null || refund === null || net === null || fee === null) continue;
    const out = outgoing.get(id) || 0n;
    const inn = incoming.get(id) || 0n;
    if (gross !== out) {
      push(errors, 'GROSS_DEBIT_MISMATCH', `${path}.actorEffects[${id}].grossDebit`, 'gross debit must equal outgoing transfers');
    }
    if (net !== inn) {
      push(errors, 'NET_CREDIT_MISMATCH', `${path}.actorEffects[${id}].netCredit`, 'net credit must equal incoming transfers');
    }
    if (refund !== 0n) {
      push(errors, 'REFUND_MISMATCH', `${path}.actorEffects[${id}].refund`, 'refund cannot substitute for gross debit');
    }
    if (fee > gross) {
      push(errors, 'FEE_MISMATCH', `${path}.actorEffects[${id}].fee`, 'fee exceeds gross debit');
    }
  }
}

function checkLoanLiabilities(stage, path, errors, econ, values) {
  if (!isPlain(stage) || !isPlain(stage.liabilities)) return;
  const liab = stage.liabilities;
  requireU128(liab.nominalRemaining, values.nominalRemaining, `${path}.liabilities.nominalRemaining`, 'LIABILITY_MISMATCH', errors);
  requireValue(liab.denomination, 'USD_micro', `${path}.liabilities.denomination`, 'DENOMINATION_MISMATCH', errors);
  requireValue(liab.debtId, econ.debtId, `${path}.liabilities.debtId`, 'LIABILITY_MISMATCH', errors);
  if (isPlain(liab.principal)) {
    requireU128(liab.principal.installment, econ.installment, `${path}.liabilities.principal.installment`, 'LIABILITY_MISMATCH', errors);
    requireU128(liab.principal.paid, values.principalPaid, `${path}.liabilities.principal.paid`, 'LIABILITY_MISMATCH', errors);
    requireU128(liab.principal.due, values.principalDue, `${path}.liabilities.principal.due`, 'LIABILITY_MISMATCH', errors);
    requireU128(liab.principal.outstandingNotional, values.outstandingNotional, `${path}.liabilities.principal.outstandingNotional`, 'LIABILITY_MISMATCH', errors);
  }
  if (isPlain(liab.accrual)) {
    requireU128(liab.accrual.interestCalculated, values.interestCalculated, `${path}.liabilities.accrual.interestCalculated`, 'DUE_MISMATCH', errors);
    requireU128(liab.accrual.interestPaid, values.interestPaid, `${path}.liabilities.accrual.interestPaid`, 'DUE_MISMATCH', errors);
    requireU128(liab.accrual.interestDue, values.interestDue, `${path}.liabilities.accrual.interestDue`, 'DUE_MISMATCH', errors);
    requireU128(liab.accrual.rateNumerator, econ.rateN, `${path}.liabilities.accrual.rateNumerator`, 'LIABILITY_MISMATCH', errors);
    requireU128(liab.accrual.rateDenominator, econ.rateD, `${path}.liabilities.accrual.rateDenominator`, 'LIABILITY_MISMATCH', errors);
    requireU128(liab.accrual.periodNumerator, econ.perN, `${path}.liabilities.accrual.periodNumerator`, 'LIABILITY_MISMATCH', errors);
    requireU128(liab.accrual.periodDenominator, econ.perD, `${path}.liabilities.accrual.periodDenominator`, 'LIABILITY_MISMATCH', errors);
    requireU128(liab.accrual.floorRemainderNumerator, econ.remainderN, `${path}.liabilities.accrual.floorRemainderNumerator`, 'LIABILITY_MISMATCH', errors);
    requireU128(liab.accrual.floorRemainderDenominator, econ.remainderD, `${path}.liabilities.accrual.floorRemainderDenominator`, 'LIABILITY_MISMATCH', errors);
  }
}

function replayLoan(record, label, errors) {
  const econ = loanEconomics();
  const asset = econ.assets[0];
  checkAdmittedPins(record.sourcePins, `${label}.sourcePins`, errors, econ.pins);
  if (isPlain(record.agreement)) {
    requireValue(record.agreement.name, econ.name, `${label}.agreement.name`, 'VALUE_MISMATCH', errors);
    requireValue(record.agreement.profile, econ.profile, `${label}.agreement.profile`, 'VALUE_MISMATCH', errors);
    requireU128(record.agreement.lifetime, econ.lifetime, `${label}.agreement.lifetime`, 'VALUE_MISMATCH', errors);
    requireU128(record.agreement.horizon, econ.horizon, `${label}.agreement.horizon`, 'VALUE_MISMATCH', errors);
  }
  if (isPlain(record.deploymentBinding)) {
    requireValue(record.deploymentBinding.status, 'unresolved', `${label}.deploymentBinding.status`, 'BINDING_MISMATCH', errors);
    checkAdmittedRoles(record.deploymentBinding.roles, `${label}.deploymentBinding.roles`, errors, econ.roles);
    checkAdmittedAssets(record.deploymentBinding.assets, `${label}.deploymentBinding.assets`, errors, econ.assets);
  }
  checkLifecycle(record, label, errors, econ.lifetime, econ.stages);
  checkContinuity(record, label, errors, econ.stages);
  const stages = stagesByName(record);
  const setup = stages.get('setup');
  const accrue = stages.get('accrue');
  const settle = stages.get('settle');
  if (setup) {
    const path = `${label}.stages[setup]`;
    requireValue(setup.status && setup.status.episodeClosed, '0', `${path}.status.episodeClosed`, 'STATUS_MISMATCH', errors);
    requireValue(setup.status && setup.status.cursor, '0', `${path}.status.cursor`, 'STATUS_MISMATCH', errors);
    requireFee(setup, path, errors, 0n, 'borrower', asset.id, asset.unit, asset.color);
    requireBalance(setup, path, 'borrower', asset.id, asset.unit, 'pre', 0n, errors);
    requireBalance(setup, path, 'lender', asset.id, asset.unit, 'pre', 0n, errors);
    requireBalance(setup, path, 'borrower', asset.id, asset.unit, 'post', econ.cash, errors);
    requireBalance(setup, path, 'lender', asset.id, asset.unit, 'post', 0n, errors);
    requireTransferSet(setup, path, [{
      id: 'loan:setup:mint-borrower',
      from: 'setup-mint', to: 'borrower', color: asset.color, unit: asset.unit, amount: econ.cash, ordinal: 0n
    }], errors);
    requireEffect(setup, path, 'borrower', asset.id, asset.unit, {
      grossDebit: 0n, refund: 0n, netCredit: econ.cash, fee: 0n
    }, errors);
    requireEffect(setup, path, 'lender', asset.id, asset.unit, {
      grossDebit: 0n, refund: 0n, netCredit: 0n, fee: 0n
    }, errors);
    checkLoanLiabilities(setup, path, errors, econ, {
      nominalRemaining: econ.notional,
      outstandingNotional: econ.notional,
      principalPaid: 0n,
      principalDue: 0n,
      interestCalculated: 0n,
      interestPaid: 0n,
      interestDue: 0n
    });
    requireDueSet(setup, path, [], errors);
    requireDutySet(setup, path, [{
      id: 'remaining-notional',
      kind: 'nominal-agreement-debt',
      amount: econ.notional,
      unit: 'USD_micro',
      status: 'open'
    }], errors);
    checkMintException(setup, path, errors, [
      { to: 'borrower', color: asset.color, amount: econ.cash }
    ]);
    checkStageBindings(setup, path, errors, econ.roleNames, econ.assets);
    checkClosedReferences(setup, path, errors, econ.roleNames, econ.actors, econ.assets);
    checkCoverage(setup, path, errors, econ.actors, econ.assets);
    checkConservationStage(setup, path, errors);
  }
  if (accrue) {
    const path = `${label}.stages[accrue]`;
    requireValue(accrue.status && accrue.status.episodeClosed, '0', `${path}.status.episodeClosed`, 'STATUS_MISMATCH', errors);
    requireValue(accrue.status && accrue.status.cursor, '1', `${path}.status.cursor`, 'STATUS_MISMATCH', errors);
    requireFee(accrue, path, errors, 0n, 'borrower', asset.id, asset.unit, asset.color);
    requireBalance(accrue, path, 'borrower', asset.id, asset.unit, 'pre', econ.cash, errors);
    requireBalance(accrue, path, 'lender', asset.id, asset.unit, 'pre', 0n, errors);
    requireBalance(accrue, path, 'borrower', asset.id, asset.unit, 'post', econ.cash, errors);
    requireBalance(accrue, path, 'lender', asset.id, asset.unit, 'post', 0n, errors);
    requireTransferSet(accrue, path, [], errors);
    requireEffect(accrue, path, 'borrower', asset.id, asset.unit, {
      grossDebit: 0n, refund: 0n, netCredit: 0n, fee: 0n
    }, errors);
    requireEffect(accrue, path, 'lender', asset.id, asset.unit, {
      grossDebit: 0n, refund: 0n, netCredit: 0n, fee: 0n
    }, errors);
    checkLoanLiabilities(accrue, path, errors, econ, {
      nominalRemaining: econ.remaining,
      outstandingNotional: econ.remaining,
      principalPaid: 0n,
      principalDue: econ.installment,
      interestCalculated: econ.interest,
      interestPaid: 0n,
      interestDue: econ.interest
    });
    requireDueSet(accrue, path, [
      {
        id: econ.duePR, kind: 'principal', debtor: 'borrower', creditor: 'lender',
        denomination: 'USD_micro', created: econ.installment, settled: 0n, outstanding: econ.installment
      },
      {
        id: econ.dueIP, kind: 'interest', debtor: 'borrower', creditor: 'lender',
        denomination: 'USD_micro', created: econ.interest, settled: 0n, outstanding: econ.interest
      }
    ], errors);
    requireDutySet(accrue, path, [{
      id: 'remaining-notional',
      kind: 'nominal-agreement-debt',
      amount: econ.remaining,
      unit: 'USD_micro',
      status: 'open'
    }], errors);
    checkMintException(accrue, path, errors, []);
    checkStageBindings(accrue, path, errors, econ.roleNames, econ.assets);
    checkClosedReferences(accrue, path, errors, econ.roleNames, econ.actors, econ.assets);
    checkCoverage(accrue, path, errors, econ.actors, econ.assets);
    checkConservationStage(accrue, path, errors);
  }
  if (settle) {
    const path = `${label}.stages[settle]`;
    requireValue(settle.status && settle.status.episodeClosed, '1', `${path}.status.episodeClosed`, 'STATUS_MISMATCH', errors);
    requireValue(settle.status && settle.status.cursor, '2', `${path}.status.cursor`, 'STATUS_MISMATCH', errors);
    requireFee(settle, path, errors, 0n, 'borrower', asset.id, asset.unit, asset.color);
    requireBalance(settle, path, 'borrower', asset.id, asset.unit, 'pre', econ.cash, errors);
    requireBalance(settle, path, 'lender', asset.id, asset.unit, 'pre', 0n, errors);
    requireBalance(settle, path, 'borrower', asset.id, asset.unit, 'post', econ.borrowerAfter, errors);
    requireBalance(settle, path, 'lender', asset.id, asset.unit, 'post', econ.total, errors);
    requireTransferSet(settle, path, [{
      id: 'loan:settle:borrower-lender',
      from: 'borrower', to: 'lender', color: asset.color, unit: asset.unit, amount: econ.total, ordinal: 0n
    }], errors);
    requireEffect(settle, path, 'borrower', asset.id, asset.unit, {
      grossDebit: econ.total, refund: 0n, netCredit: 0n, fee: 0n
    }, errors);
    requireEffect(settle, path, 'lender', asset.id, asset.unit, {
      grossDebit: 0n, refund: 0n, netCredit: econ.total, fee: 0n
    }, errors);
    checkLoanLiabilities(settle, path, errors, econ, {
      nominalRemaining: econ.remaining,
      outstandingNotional: econ.remaining,
      principalPaid: econ.installment,
      principalDue: 0n,
      interestCalculated: econ.interest,
      interestPaid: econ.interest,
      interestDue: 0n
    });
    requireDueSet(settle, path, [
      {
        id: econ.duePR, kind: 'principal', debtor: 'borrower', creditor: 'lender',
        denomination: 'USD_micro', created: 0n, settled: econ.installment, outstanding: 0n
      },
      {
        id: econ.dueIP, kind: 'interest', debtor: 'borrower', creditor: 'lender',
        denomination: 'USD_micro', created: 0n, settled: econ.interest, outstanding: 0n
      }
    ], errors);
    requireDutySet(settle, path, [{
      id: 'remaining-notional',
      kind: 'nominal-agreement-debt',
      amount: econ.remaining,
      unit: 'USD_micro',
      status: 'open'
    }], errors);
    checkMintException(settle, path, errors, []);
    checkStageBindings(settle, path, errors, econ.roleNames, econ.assets);
    checkClosedReferences(settle, path, errors, econ.roleNames, econ.actors, econ.assets);
    checkCoverage(settle, path, errors, econ.actors, econ.assets);
    checkConservationStage(settle, path, errors);
  }
}

function checkSwapLiabilities(stage, path, errors) {
  if (!isPlain(stage) || !isPlain(stage.liabilities)) return;
  const liab = stage.liabilities;
  requireU128(liab.nominalRemaining, 0n, `${path}.liabilities.nominalRemaining`, 'LIABILITY_MISMATCH', errors);
  requireValue(liab.denomination, 'AssetB_quantum', `${path}.liabilities.denomination`, 'DENOMINATION_MISMATCH', errors);
  requireValue(liab.debtId, 'ConstantProductEpoch:none', `${path}.liabilities.debtId`, 'LIABILITY_MISMATCH', errors);
  if (Array.isArray(liab.dues) && liab.dues.length !== 0) {
    push(errors, 'DUE_MISMATCH', `${path}.liabilities.dues`, 'swap dues must be empty');
  }
  if (isPlain(liab.principal)) {
    for (const key of PRINCIPAL_KEYS) {
      requireU128(liab.principal[key], 0n, `${path}.liabilities.principal.${key}`, 'LIABILITY_MISMATCH', errors);
    }
  }
  if (isPlain(liab.accrual)) {
    requireU128(liab.accrual.interestCalculated, 0n, `${path}.liabilities.accrual.interestCalculated`, 'DUE_MISMATCH', errors);
    requireU128(liab.accrual.interestPaid, 0n, `${path}.liabilities.accrual.interestPaid`, 'DUE_MISMATCH', errors);
    requireU128(liab.accrual.interestDue, 0n, `${path}.liabilities.accrual.interestDue`, 'DUE_MISMATCH', errors);
    requireU128(liab.accrual.rateNumerator, 0n, `${path}.liabilities.accrual.rateNumerator`, 'LIABILITY_MISMATCH', errors);
    requireU128(liab.accrual.rateDenominator, 1n, `${path}.liabilities.accrual.rateDenominator`, 'LIABILITY_MISMATCH', errors);
    requireU128(liab.accrual.periodNumerator, 0n, `${path}.liabilities.accrual.periodNumerator`, 'LIABILITY_MISMATCH', errors);
    requireU128(liab.accrual.periodDenominator, 1n, `${path}.liabilities.accrual.periodDenominator`, 'LIABILITY_MISMATCH', errors);
    requireU128(liab.accrual.floorRemainderNumerator, 0n, `${path}.liabilities.accrual.floorRemainderNumerator`, 'LIABILITY_MISMATCH', errors);
    requireU128(liab.accrual.floorRemainderDenominator, 1n, `${path}.liabilities.accrual.floorRemainderDenominator`, 'LIABILITY_MISMATCH', errors);
  }
}

function replaySwap(record, label, errors) {
  const econ = swapEconomics();
  const assetA = econ.assets[0];
  const assetB = econ.assets[1];
  checkAdmittedPins(record.sourcePins, `${label}.sourcePins`, errors, econ.pins);
  if (isPlain(record.agreement)) {
    requireValue(record.agreement.name, econ.name, `${label}.agreement.name`, 'VALUE_MISMATCH', errors);
    requireValue(record.agreement.profile, econ.profile, `${label}.agreement.profile`, 'VALUE_MISMATCH', errors);
    requireU128(record.agreement.lifetime, econ.lifetime, `${label}.agreement.lifetime`, 'VALUE_MISMATCH', errors);
    requireU128(record.agreement.horizon, econ.horizon, `${label}.agreement.horizon`, 'VALUE_MISMATCH', errors);
  }
  if (isPlain(record.deploymentBinding)) {
    requireValue(record.deploymentBinding.status, 'unresolved', `${label}.deploymentBinding.status`, 'BINDING_MISMATCH', errors);
    checkAdmittedRoles(record.deploymentBinding.roles, `${label}.deploymentBinding.roles`, errors, econ.roles);
    checkAdmittedAssets(record.deploymentBinding.assets, `${label}.deploymentBinding.assets`, errors, econ.assets);
  }
  checkLifecycle(record, label, errors, econ.lifetime, econ.stages);
  checkContinuity(record, label, errors, econ.stages);
  const stages = stagesByName(record);
  const setup = stages.get('setup');
  const trade = stages.get('swap');
  const close = stages.get('close');
  const zeroEffects = { grossDebit: 0n, refund: 0n, netCredit: 0n, fee: 0n };
  if (setup) {
    const path = `${label}.stages[setup]`;
    requireValue(setup.status && setup.status.episodeClosed, '0', `${path}.status.episodeClosed`, 'STATUS_MISMATCH', errors);
    requireFee(setup, path, errors, 0n, 'trader', assetA.id, assetA.unit, assetA.color);
    requireBalance(setup, path, 'pool', assetA.id, assetA.unit, 'pre', 0n, errors);
    requireBalance(setup, path, 'pool', assetB.id, assetB.unit, 'pre', 0n, errors);
    requireBalance(setup, path, 'trader', assetA.id, assetA.unit, 'pre', 0n, errors);
    requireBalance(setup, path, 'trader', assetB.id, assetB.unit, 'pre', 0n, errors);
    requireBalance(setup, path, 'provider', assetA.id, assetA.unit, 'pre', 0n, errors);
    requireBalance(setup, path, 'provider', assetB.id, assetB.unit, 'pre', 0n, errors);
    requireBalance(setup, path, 'pool', assetA.id, assetA.unit, 'post', econ.reserveA, errors);
    requireBalance(setup, path, 'pool', assetB.id, assetB.unit, 'post', econ.reserveB, errors);
    requireBalance(setup, path, 'trader', assetA.id, assetA.unit, 'post', econ.traderA, errors);
    requireBalance(setup, path, 'trader', assetB.id, assetB.unit, 'post', 0n, errors);
    requireBalance(setup, path, 'provider', assetA.id, assetA.unit, 'post', 0n, errors);
    requireBalance(setup, path, 'provider', assetB.id, assetB.unit, 'post', 0n, errors);
    requireTransferSet(setup, path, [
      {
        id: 'swap:setup:mint-pool-a',
        from: 'setup-mint', to: 'pool', color: assetA.color, unit: assetA.unit, amount: econ.reserveA, ordinal: 0n
      },
      {
        id: 'swap:setup:mint-pool-b',
        from: 'setup-mint', to: 'pool', color: assetB.color, unit: assetB.unit, amount: econ.reserveB, ordinal: 1n
      },
      {
        id: 'swap:setup:mint-trader-a',
        from: 'setup-mint', to: 'trader', color: assetA.color, unit: assetA.unit, amount: econ.traderA, ordinal: 2n
      }
    ], errors);
    requireEffect(setup, path, 'pool', assetA.id, assetA.unit, { ...zeroEffects, netCredit: econ.reserveA }, errors);
    requireEffect(setup, path, 'pool', assetB.id, assetB.unit, { ...zeroEffects, netCredit: econ.reserveB }, errors);
    requireEffect(setup, path, 'trader', assetA.id, assetA.unit, { ...zeroEffects, netCredit: econ.traderA }, errors);
    requireEffect(setup, path, 'trader', assetB.id, assetB.unit, zeroEffects, errors);
    requireEffect(setup, path, 'provider', assetA.id, assetA.unit, zeroEffects, errors);
    requireEffect(setup, path, 'provider', assetB.id, assetB.unit, zeroEffects, errors);
    checkSwapLiabilities(setup, path, errors);
    requireDutySet(setup, path, [{
      id: 'reserve-for-close', kind: 'action-allowance-reservation', amount: 1n, unit: 'action', status: 'open'
    }], errors);
    checkMintException(setup, path, errors, [
      { to: 'pool', color: assetA.color, amount: econ.reserveA },
      { to: 'pool', color: assetB.color, amount: econ.reserveB },
      { to: 'trader', color: assetA.color, amount: econ.traderA }
    ]);
    checkStageBindings(setup, path, errors, econ.roleNames, econ.assets);
    checkClosedReferences(setup, path, errors, econ.roleNames, econ.actors, econ.assets);
    checkCoverage(setup, path, errors, econ.actors, econ.assets);
    checkConservationStage(setup, path, errors);
  }
  if (trade) {
    const path = `${label}.stages[swap]`;
    requireValue(trade.status && trade.status.episodeClosed, '0', `${path}.status.episodeClosed`, 'STATUS_MISMATCH', errors);
    requireFee(trade, path, errors, econ.fee, 'trader', assetA.id, assetA.unit, assetA.color);
    requireBalance(trade, path, 'pool', assetA.id, assetA.unit, 'pre', econ.reserveA, errors);
    requireBalance(trade, path, 'pool', assetB.id, assetB.unit, 'pre', econ.reserveB, errors);
    requireBalance(trade, path, 'trader', assetA.id, assetA.unit, 'pre', econ.traderA, errors);
    requireBalance(trade, path, 'trader', assetB.id, assetB.unit, 'pre', 0n, errors);
    requireBalance(trade, path, 'provider', assetA.id, assetA.unit, 'pre', 0n, errors);
    requireBalance(trade, path, 'provider', assetB.id, assetB.unit, 'pre', 0n, errors);
    requireBalance(trade, path, 'pool', assetA.id, assetA.unit, 'post', econ.poolAfterA, errors);
    requireBalance(trade, path, 'pool', assetB.id, assetB.unit, 'post', econ.poolAfterB, errors);
    requireBalance(trade, path, 'trader', assetA.id, assetA.unit, 'post', econ.traderAfterA, errors);
    requireBalance(trade, path, 'trader', assetB.id, assetB.unit, 'post', econ.traderAfterB, errors);
    requireBalance(trade, path, 'provider', assetA.id, assetA.unit, 'post', 0n, errors);
    requireBalance(trade, path, 'provider', assetB.id, assetB.unit, 'post', 0n, errors);
    requireTransferSet(trade, path, [
      {
        id: 'swap:trade:trader-pool-a',
        from: 'trader', to: 'pool', color: assetA.color, unit: assetA.unit, amount: econ.amountIn, ordinal: 0n
      },
      {
        id: 'swap:trade:pool-trader-b',
        from: 'pool', to: 'trader', color: assetB.color, unit: assetB.unit, amount: econ.output, ordinal: 1n
      }
    ], errors);
    requireEffect(trade, path, 'pool', assetA.id, assetA.unit, { ...zeroEffects, netCredit: econ.amountIn }, errors);
    requireEffect(trade, path, 'pool', assetB.id, assetB.unit, { ...zeroEffects, grossDebit: econ.output }, errors);
    requireEffect(trade, path, 'trader', assetA.id, assetA.unit, {
      grossDebit: econ.amountIn, refund: 0n, netCredit: 0n, fee: econ.fee
    }, errors);
    requireEffect(trade, path, 'trader', assetB.id, assetB.unit, { ...zeroEffects, netCredit: econ.output }, errors);
    requireEffect(trade, path, 'provider', assetA.id, assetA.unit, zeroEffects, errors);
    requireEffect(trade, path, 'provider', assetB.id, assetB.unit, zeroEffects, errors);
    checkSwapLiabilities(trade, path, errors);
    requireDutySet(trade, path, [{
      id: 'reserve-for-close', kind: 'action-allowance-reservation', amount: 1n, unit: 'action', status: 'open'
    }], errors);
    checkMintException(trade, path, errors, []);
    checkStageBindings(trade, path, errors, econ.roleNames, econ.assets);
    checkClosedReferences(trade, path, errors, econ.roleNames, econ.actors, econ.assets);
    checkCoverage(trade, path, errors, econ.actors, econ.assets);
    checkConservationStage(trade, path, errors);
  }
  if (close) {
    const path = `${label}.stages[close]`;
    requireValue(close.status && close.status.episodeClosed, '1', `${path}.status.episodeClosed`, 'STATUS_MISMATCH', errors);
    requireFee(close, path, errors, 0n, 'trader', assetA.id, assetA.unit, assetA.color);
    requireBalance(close, path, 'pool', assetA.id, assetA.unit, 'pre', econ.poolAfterA, errors);
    requireBalance(close, path, 'pool', assetB.id, assetB.unit, 'pre', econ.poolAfterB, errors);
    requireBalance(close, path, 'trader', assetA.id, assetA.unit, 'pre', econ.traderAfterA, errors);
    requireBalance(close, path, 'trader', assetB.id, assetB.unit, 'pre', econ.traderAfterB, errors);
    requireBalance(close, path, 'provider', assetA.id, assetA.unit, 'pre', 0n, errors);
    requireBalance(close, path, 'provider', assetB.id, assetB.unit, 'pre', 0n, errors);
    requireBalance(close, path, 'pool', assetA.id, assetA.unit, 'post', 0n, errors);
    requireBalance(close, path, 'pool', assetB.id, assetB.unit, 'post', 0n, errors);
    requireBalance(close, path, 'trader', assetA.id, assetA.unit, 'post', econ.traderAfterA, errors);
    requireBalance(close, path, 'trader', assetB.id, assetB.unit, 'post', econ.traderAfterB, errors);
    requireBalance(close, path, 'provider', assetA.id, assetA.unit, 'post', econ.poolAfterA, errors);
    requireBalance(close, path, 'provider', assetB.id, assetB.unit, 'post', econ.poolAfterB, errors);
    requireTransferSet(close, path, [
      {
        id: 'swap:close:pool-provider-a',
        from: 'pool', to: 'provider', color: assetA.color, unit: assetA.unit, amount: econ.poolAfterA, ordinal: 0n
      },
      {
        id: 'swap:close:pool-provider-b',
        from: 'pool', to: 'provider', color: assetB.color, unit: assetB.unit, amount: econ.poolAfterB, ordinal: 1n
      }
    ], errors);
    requireEffect(close, path, 'pool', assetA.id, assetA.unit, { ...zeroEffects, grossDebit: econ.poolAfterA }, errors);
    requireEffect(close, path, 'pool', assetB.id, assetB.unit, { ...zeroEffects, grossDebit: econ.poolAfterB }, errors);
    requireEffect(close, path, 'trader', assetA.id, assetA.unit, zeroEffects, errors);
    requireEffect(close, path, 'trader', assetB.id, assetB.unit, zeroEffects, errors);
    requireEffect(close, path, 'provider', assetA.id, assetA.unit, { ...zeroEffects, netCredit: econ.poolAfterA }, errors);
    requireEffect(close, path, 'provider', assetB.id, assetB.unit, { ...zeroEffects, netCredit: econ.poolAfterB }, errors);
    checkSwapLiabilities(close, path, errors);
    requireDutySet(close, path, [{
      id: 'reserve-for-close', kind: 'action-allowance-reservation', amount: 1n, unit: 'action', status: 'discharged'
    }], errors);
    checkMintException(close, path, errors, []);
    checkStageBindings(close, path, errors, econ.roleNames, econ.assets);
    checkClosedReferences(close, path, errors, econ.roleNames, econ.actors, econ.assets);
    checkCoverage(close, path, errors, econ.actors, econ.assets);
    checkConservationStage(close, path, errors);
  }
}

function replayRecord(record, label, errors) {
  const name = isPlain(record.agreement) ? record.agreement.name : undefined;
  if (name === 'LoanFirstPeriod') {
    replayLoan(record, label, errors);
    return;
  }
  if (name === 'ConstantProductEpoch') {
    replaySwap(record, label, errors);
    return;
  }
  push(errors, 'VALUE_MISMATCH', `${label}.agreement.name`, 'unsupported fixed financial case');
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
  if (!isPlain(exp) || !isPlain(obs)) return;
  cmpString(exp.logicalId, obs.logicalId, `${path}.logicalId`, 'BINDING_MISMATCH', errors);
  cmpString(exp.denomination, obs.denomination, `${path}.denomination`, 'DENOMINATION_MISMATCH', errors);
  cmpU128Field(exp.quantum, obs.quantum, `${path}.quantum`, 'QUANTUM_MISMATCH', errors);
  cmpString(exp.color, obs.color, `${path}.color`, 'COLOR_MISMATCH', errors);
  cmpString(exp.status, obs.status, `${path}.status`, 'BINDING_MISMATCH', errors);
}

function compareRoleMap(expRoles, obsRoles, path, errors) {
  if (!isPlain(expRoles) || !isPlain(obsRoles)) return;
  const expNames = Object.keys(expRoles).sort();
  const obsNames = Object.keys(obsRoles).sort();
  if (JSON.stringify(expNames) !== JSON.stringify(obsNames)) {
    push(errors, 'BINDING_MISMATCH', path, 'role set differs');
  }
  for (const name of expNames) {
    if (!isPlain(expRoles[name]) || !isPlain(obsRoles[name])) continue;
    cmpString(expRoles[name].logicalId, obsRoles[name].logicalId, `${path}.${name}.logicalId`, 'BINDING_MISMATCH', errors);
    cmpString(expRoles[name].status, obsRoles[name].status, `${path}.${name}.status`, 'BINDING_MISMATCH', errors);
  }
}

function compareStage(exp, obs, stageName, errors) {
  if (!isPlain(exp) || !isPlain(obs)) return;
  const path = `observed.stages[${stageName}]`;
  cmpU128Field(exp.revision, obs.revision, `${path}.revision`, 'REVISION_MISMATCH', errors);
  cmpU128Field(exp.remaining, obs.remaining, `${path}.remaining`, 'REMAINING_MISMATCH', errors);
  cmpU128Field(exp.work, obs.work, `${path}.work`, 'WORK_MISMATCH', errors);
  if (isPlain(exp.status) && isPlain(obs.status)) {
    for (const key of Object.keys(exp.status)) {
      cmpU128Field(exp.status[key], obs.status[key], `${path}.status.${key}`, 'STATUS_MISMATCH', errors);
    }
  }
  if (isPlain(exp.economicFee) && isPlain(obs.economicFee)) {
    cmpString(exp.economicFee.actor, obs.economicFee.actor, `${path}.economicFee.actor`, 'FEE_MISMATCH', errors);
    cmpString(exp.economicFee.asset, obs.economicFee.asset, `${path}.economicFee.asset`, 'FEE_MISMATCH', errors);
    cmpString(exp.economicFee.color, obs.economicFee.color, `${path}.economicFee.color`, 'COLOR_MISMATCH', errors);
    cmpString(exp.economicFee.unit, obs.economicFee.unit, `${path}.economicFee.unit`, 'DENOMINATION_MISMATCH', errors);
    cmpU128Field(exp.economicFee.amount, obs.economicFee.amount, `${path}.economicFee.amount`, 'FEE_MISMATCH', errors);
  }
  if (isPlain(exp.bindings) && isPlain(obs.bindings)) {
    if (JSON.stringify(exp.bindings.roles) !== JSON.stringify(obs.bindings.roles)) {
      push(errors, 'BINDING_MISMATCH', `${path}.bindings.roles`, 'role bindings differ');
    }
    compareIndexed(
      exp.bindings.assets,
      obs.bindings.assets,
      `stages[${stageName}].bindings.assets`,
      'BINDING_MISMATCH',
      'BINDING_MISMATCH',
      (a, b, p) => compareAsset(a, b, p, errors),
      errors
    );
  }
  if (isPlain(exp.balances) && isPlain(obs.balances)) {
    for (const side of ['pre', 'post']) {
      if (!Array.isArray(exp.balances[side]) || !Array.isArray(obs.balances[side])) continue;
      compareIndexed(exp.balances[side], obs.balances[side], `stages[${stageName}].balances.${side}`, 'VALUE_MISMATCH', 'VALUE_MISMATCH', (a, b, p) => {
        cmpString(a.actor, b.actor, `${p}.actor`, 'VALUE_MISMATCH', errors);
        cmpString(a.asset, b.asset, `${p}.asset`, 'COLOR_MISMATCH', errors);
        cmpString(a.unit, b.unit, `${p}.unit`, 'DENOMINATION_MISMATCH', errors);
        cmpU128Field(a.amount, b.amount, `${p}.amount`, 'VALUE_MISMATCH', errors);
      }, errors);
    }
  }
  if (Array.isArray(exp.transfers) && Array.isArray(obs.transfers)) {
    compareIndexed(exp.transfers, obs.transfers, `stages[${stageName}].transfers`, 'OMITTED_TRANSFER', 'EXTRA_TRANSFER', (a, b, p) => {
      cmpU128Field(a.ordinal, b.ordinal, `${p}.ordinal`, 'TRANSFER_MISMATCH', errors);
      cmpString(a.from, b.from, `${p}.from`, 'TRANSFER_MISMATCH', errors);
      cmpString(a.to, b.to, `${p}.to`, 'TRANSFER_MISMATCH', errors);
      cmpString(a.color, b.color, `${p}.color`, 'COLOR_MISMATCH', errors);
      cmpString(a.unit, b.unit, `${p}.unit`, 'DENOMINATION_MISMATCH', errors);
      cmpU128Field(a.amount, b.amount, `${p}.amount`, 'TRANSFER_MISMATCH', errors);
    }, errors);
  }
  if (Array.isArray(exp.actorEffects) && Array.isArray(obs.actorEffects)) {
    compareIndexed(exp.actorEffects, obs.actorEffects, `stages[${stageName}].actorEffects`, 'VALUE_MISMATCH', 'VALUE_MISMATCH', (a, b, p) => {
      cmpU128Field(a.grossDebit, b.grossDebit, `${p}.grossDebit`, 'GROSS_DEBIT_MISMATCH', errors);
      cmpU128Field(a.refund, b.refund, `${p}.refund`, 'REFUND_MISMATCH', errors);
      cmpU128Field(a.netCredit, b.netCredit, `${p}.netCredit`, 'NET_CREDIT_MISMATCH', errors);
      cmpU128Field(a.fee, b.fee, `${p}.fee`, 'FEE_MISMATCH', errors);
    }, errors);
  }
  if (isPlain(exp.liabilities) && isPlain(obs.liabilities)) {
    cmpU128Field(exp.liabilities.nominalRemaining, obs.liabilities.nominalRemaining, `${path}.liabilities.nominalRemaining`, 'LIABILITY_MISMATCH', errors);
    cmpString(exp.liabilities.denomination, obs.liabilities.denomination, `${path}.liabilities.denomination`, 'DENOMINATION_MISMATCH', errors);
    cmpString(exp.liabilities.debtId, obs.liabilities.debtId, `${path}.liabilities.debtId`, 'LIABILITY_MISMATCH', errors);
    if (isPlain(exp.liabilities.principal) && isPlain(obs.liabilities.principal)) {
      for (const key of PRINCIPAL_KEYS) {
        cmpU128Field(exp.liabilities.principal[key], obs.liabilities.principal[key], `${path}.liabilities.principal.${key}`, 'LIABILITY_MISMATCH', errors);
      }
    }
    if (isPlain(exp.liabilities.accrual) && isPlain(obs.liabilities.accrual)) {
      for (const key of ACCRUAL_KEYS) {
        const code = key.startsWith('interest') ? 'DUE_MISMATCH' : 'LIABILITY_MISMATCH';
        cmpU128Field(exp.liabilities.accrual[key], obs.liabilities.accrual[key], `${path}.liabilities.accrual.${key}`, code, errors);
      }
    }
    if (Array.isArray(exp.liabilities.dues) && Array.isArray(obs.liabilities.dues)) {
      compareIndexed(exp.liabilities.dues, obs.liabilities.dues, `stages[${stageName}].liabilities.dues`, 'DUE_MISMATCH', 'DUE_MISMATCH', (a, b, p) => {
        cmpString(a.kind, b.kind, `${p}.kind`, 'DUE_MISMATCH', errors);
        cmpString(a.debtor, b.debtor, `${p}.debtor`, 'DUE_MISMATCH', errors);
        cmpString(a.creditor, b.creditor, `${p}.creditor`, 'DUE_MISMATCH', errors);
        cmpString(a.denomination, b.denomination, `${p}.denomination`, 'DENOMINATION_MISMATCH', errors);
        cmpU128Field(a.created, b.created, `${p}.created`, 'DUE_MISMATCH', errors);
        cmpU128Field(a.settled, b.settled, `${p}.settled`, 'DUE_MISMATCH', errors);
        cmpU128Field(a.outstanding, b.outstanding, `${p}.outstanding`, 'DUE_MISMATCH', errors);
      }, errors);
    }
  }
  if (Array.isArray(exp.residualDuties) && Array.isArray(obs.residualDuties)) {
    compareIndexed(exp.residualDuties, obs.residualDuties, `stages[${stageName}].residualDuties`, 'RESIDUAL_DUTY_MISMATCH', 'RESIDUAL_DUTY_MISMATCH', (a, b, p) => {
      cmpString(a.kind, b.kind, `${p}.kind`, 'RESIDUAL_DUTY_MISMATCH', errors);
      cmpU128Field(a.amount, b.amount, `${p}.amount`, 'RESIDUAL_DUTY_MISMATCH', errors);
      cmpString(a.unit, b.unit, `${p}.unit`, 'RESIDUAL_DUTY_MISMATCH', errors);
      cmpString(a.status, b.status, `${p}.status`, 'RESIDUAL_DUTY_MISMATCH', errors);
    }, errors);
  }
}

function compareRecords(expected, observed, errors) {
  if (!isPlain(expected) || !isPlain(observed)) return;
  cmpString(expected.schemaVersion, observed.schemaVersion, 'observed.schemaVersion', 'VALUE_MISMATCH', errors);
  cmpString(expected.observationKind, observed.observationKind, 'observed.observationKind', 'VALUE_MISMATCH', errors);
  cmpString(expected.derivationNote, observed.derivationNote, 'observed.derivationNote', 'VALUE_MISMATCH', errors);
  if (isPlain(expected.sourcePins) && isPlain(observed.sourcePins)) {
    for (const key of SOURCE_PIN_KEYS) {
      cmpString(expected.sourcePins[key], observed.sourcePins[key], `observed.sourcePins.${key}`, 'VALUE_MISMATCH', errors);
    }
  }
  if (isPlain(expected.agreement) && isPlain(observed.agreement)) {
    cmpString(expected.agreement.name, observed.agreement.name, 'observed.agreement.name', 'VALUE_MISMATCH', errors);
    cmpString(expected.agreement.profile, observed.agreement.profile, 'observed.agreement.profile', 'VALUE_MISMATCH', errors);
    cmpU128Field(expected.agreement.lifetime, observed.agreement.lifetime, 'observed.agreement.lifetime', 'VALUE_MISMATCH', errors);
    cmpU128Field(expected.agreement.horizon, observed.agreement.horizon, 'observed.agreement.horizon', 'VALUE_MISMATCH', errors);
  }
  if (isPlain(expected.deploymentBinding) && isPlain(observed.deploymentBinding)) {
    if (expected.deploymentBinding.status !== observed.deploymentBinding.status) {
      push(errors, 'BINDING_MISMATCH', 'observed.deploymentBinding.status', 'deployment binding differs');
    }
    compareRoleMap(
      expected.deploymentBinding.roles,
      observed.deploymentBinding.roles,
      'observed.deploymentBinding.roles',
      errors
    );
    compareIndexed(
      expected.deploymentBinding.assets,
      observed.deploymentBinding.assets,
      'deploymentBinding.assets',
      'BINDING_MISMATCH',
      'BINDING_MISMATCH',
      (a, b, p) => compareAsset(a, b, p, errors),
      errors
    );
  }
  if (isPlain(expected.networkFeeAccounting) && isPlain(observed.networkFeeAccounting)) {
    cmpString(expected.networkFeeAccounting.status, observed.networkFeeAccounting.status, 'observed.networkFeeAccounting.status', 'VALUE_MISMATCH', errors);
    cmpString(expected.networkFeeAccounting.midnightFeeAsset, observed.networkFeeAccounting.midnightFeeAsset, 'observed.networkFeeAccounting.midnightFeeAsset', 'VALUE_MISMATCH', errors);
    cmpString(expected.networkFeeAccounting.midnightFeeUnit, observed.networkFeeAccounting.midnightFeeUnit, 'observed.networkFeeAccounting.midnightFeeUnit', 'VALUE_MISMATCH', errors);
    if (expected.networkFeeAccounting.assumedZero !== observed.networkFeeAccounting.assumedZero) {
      push(errors, 'VALUE_MISMATCH', 'observed.networkFeeAccounting.assumedZero', 'network fee assumption differs');
    }
  }
  if (!Array.isArray(expected.stages) || !Array.isArray(observed.stages)) return;
  const expStages = new Map();
  const obsStages = new Map();
  for (const stage of expected.stages) {
    if (isPlain(stage) && typeof stage.stage === 'string') expStages.set(stage.stage, stage);
  }
  for (const stage of observed.stages) {
    if (isPlain(stage) && typeof stage.stage === 'string') obsStages.set(stage.stage, stage);
  }
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

function emptyResult(errors) {
  sortErrors(errors);
  return {
    ok: errors.length === 0,
    errors,
    networkAcceptance: false,
    networkEvidence: 'incompleteNetworkEvidence'
  };
}

export function compareFinancialEffects(expected, observed) {
  const errors = [];
  const snapExpected = snapshot(expected);
  const snapObserved = snapshot(observed);
  try {
    const expectedReady = validateRecord(expected, 'expected', errors);
    const observedReady = validateRecord(observed, 'observed', errors);
    if (expectedReady) replayRecord(expected, 'expected', errors);
    if (observedReady) replayRecord(observed, 'observed', errors);
    if (expectedReady && observedReady) compareRecords(expected, observed, errors);
  } catch {
    if (errors.length === 0) {
      push(errors, 'MALFORMED_STRUCTURE', '', 'comparison aborted on malformed input');
    }
  }
  if (snapshot(expected) !== snapExpected || snapshot(observed) !== snapObserved) {
    push(errors, 'INPUT_MUTATED', '', 'compareFinancialEffects mutated an input');
  }
  return emptyResult(errors);
}
