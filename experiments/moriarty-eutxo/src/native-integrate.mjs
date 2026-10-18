/**
 * Financial continuation and witness counting for the owned producer.
 * This module does not import the bootstrap or the producer.
 * It does not start a node, set the latch, or invent a script-data hash.
 * Unit results stay held. Ledger flags stay false.
 */
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';

export const YACI_CLI = '/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/runtime/node_modules/@bloxbean/yaci-devkit-linux-x64/yaci-cli';
export const RECEIPT_HELPER_PATH = '/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/transaction-receipts/revision02/submit-with-receipt.mjs';
export const RECEIPT_HELPER_SHA256 = 'f3689ef82e078b9331907c4d13bde9ebfc3e2f501d88edb41d394c4f7479bf18';
export const HELPER_MODULE = new URL('./output-fee.mjs', import.meta.url).pathname;
export const TEMPLATE_SYSTEM_START = '2017-09-23T21:44:51Z';
export const CREATE_TIME_SYSTEM_START = '2022-09-15T04:09:11.577484Z';
export const CASH_QUANTITY = '100';
export const HEAD_QUANTITY = '1';
export const CONSTRUCTOR_WORK = 49;
export const COLLECTION_CAPACITY = 128;
export const HEAD_FIELD_COUNT = 13;

const FIXTURE_PAIRS = new Set(['ab', 'c3', 'd4', 'f5', 'cd']);
const CML_PATH = fileURLToPath(new URL('../owned/cml/cardano_multiplatform_lib.js', import.meta.url));
const ACTUAL_PROTOCOL_PARAMS = '/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/runtime/actual-protocol-parameters.json';
const requireCml = createRequire(CML_PATH);
let cmlModule = null;

function loadCml() {
  if (!cmlModule) cmlModule = requireCml(CML_PATH);
  return cmlModule;
}

function actualCostModels(cml, protocolPath) {
  const protocol = JSON.parse(readFileSync(protocolPath, 'utf8'));
  const named = protocol.costModels || {};
  const numeric = {};
  if (Array.isArray(named.PlutusV1)) numeric['0'] = named.PlutusV1;
  if (Array.isArray(named.PlutusV2)) numeric['1'] = named.PlutusV2;
  if (Array.isArray(named.PlutusV3)) numeric['2'] = named.PlutusV3;
  return cml.CostModels.from_json(JSON.stringify(numeric));
}

export function recomputeScriptDataHash(cborHex, options = {}) {
  const original = String(cborHex || '');
  const protocolPath = options.protocolPath || ACTUAL_PROTOCOL_PARAMS;
  try {
    const cml = loadCml();
    const tx = cml.Transaction.from_cbor_bytes(Buffer.from(original, 'hex'));
    const body = tx.body();
    const storedObj = body.script_data_hash();
    const storedHex = storedObj ? storedObj.to_hex() : null;
    const witness = tx.witness_set();
    const selected = selectNeededScriptLanguage({
      cborHex: original,
      referenceUtxos: options.referenceUtxos,
      spendingInputs: options.spendingInputs,
      spendScripts: options.spendScripts,
      spentOutputs: options.spentOutputs,
      referenceInputs: options.referenceInputs,
      costModels: options.costModels,
      witnessLanguages: options.witnessLanguages,
      emptyScriptLists: options.emptyScriptLists,
      redeemers: options.redeemers,
      mintPolicies: options.mintPolicies,
      inlineScripts: options.inlineScripts,
    });
    let languages = witness.languages();
    let neededLanguageSource = 'witness.languages';
    let neededLanguages = null;
    const resolutionSupplied = [options.referenceUtxos, options.spentOutputs, options.spendScripts]
      .some((row) => Array.isArray(row) && row.length > 0);
    if (selected.ok === true && Array.isArray(selected.languages) && selected.languages.length > 0) {
      const list = cml.LanguageList.new();
      for (const name of NEEDED_LANG_ORDER) {
        if (selected.languages.includes(name) && cml.Language[name] != null) list.add(cml.Language[name]);
      }
      if (Number(list.len()) === selected.languages.length) {
        languages = list;
        neededLanguageSource = 'selectNeededScriptLanguage';
        neededLanguages = selected.languages.slice();
      }
    } else if (resolutionSupplied) {
      return {
        ok: false,
        code: selected.code || 'NEEDED_SCRIPT_UNDECIDED',
        storedHex,
        recomputedHex: null,
        matchesStored: false,
        fromWitnessPresent: false,
        neededLanguageSource: 'selectNeededScriptLanguage',
        neededLanguages: null,
        neededLanguageCode: selected.code,
        nativeBudgetMeasured: false,
        signatureVerified: false,
        ledgerExecution: false,
        cborPreserved: true,
        method: 'calc_script_data_hash',
      };
    }
    let redeemers = witness.redeemers();
    const datumList = witness.plutus_datums() || cml.PlutusDataList.new();
    const mutated = typeof options.redeemerDataHex === 'string' && options.redeemerDataHex.length > 0;
    if (mutated) {
      if (!redeemers) {
        return {
          ok: false,
          code: 'SCRIPT_DATA_REDEEMERS_ABSENT',
          storedHex,
          recomputedHex: null,
          matchesStored: false,
          fromWitnessPresent: false,
          nativeBudgetMeasured: false,
          signatureVerified: false,
          ledgerExecution: false,
          cborPreserved: true,
          method: 'calc_script_data_hash',
        };
      }
      const flat = redeemers.to_flat_format();
      const map = cml.MapRedeemerKeyToRedeemerVal.new();
      for (let index = 0; index < Number(flat.len()); index += 1) {
        const row = flat.get(index);
        const units = row.ex_units();
        map.insert(
          cml.RedeemerKey.new(row.tag(), row.index()),
          cml.RedeemerVal.new(
            cml.PlutusData.from_cbor_hex(options.redeemerDataHex),
            cml.ExUnits.new(units.mem(), units.steps()),
          ),
        );
      }
      redeemers = cml.Redeemers.new_map_redeemer_key_to_redeemer_val(map);
    }
    const models = actualCostModels(cml, protocolPath);
    const digest = redeemers ? cml.calc_script_data_hash(redeemers, datumList, models, languages) : undefined;
    const fromWitness = mutated ? undefined : cml.calc_script_data_hash_from_witness(witness, models);
    const recomputedHex = digest ? digest.to_hex() : null;
    return {
      ok: Boolean(recomputedHex),
      code: recomputedHex ? 'SCRIPT_DATA_HASH_RECOMPUTED' : 'SCRIPT_DATA_HASH_ABSENT',
      storedHex,
      recomputedHex,
      matchesStored: Boolean(storedHex) && storedHex === recomputedHex,
      fromWitnessHex: fromWitness ? fromWitness.to_hex() : null,
      fromWitnessPresent: Boolean(fromWitness),
      txid: cml.hash_transaction(body).to_hex(),
      method: 'calc_script_data_hash',
      neededLanguageSource,
      neededLanguages,
      neededLanguageCode: selected.code,
      nativeBudgetMeasured: false,
      signatureVerified: false,
      ledgerExecution: false,
      cborPreserved: true,
    };
  } catch (error) {
    return {
      ok: false,
      code: 'SCRIPT_DATA_INTEGRITY_FAILED',
      detail: String(error?.message || error),
      storedHex: null,
      recomputedHex: null,
      matchesStored: false,
      fromWitnessPresent: false,
      nativeBudgetMeasured: false,
      signatureVerified: false,
      ledgerExecution: false,
      cborPreserved: true,
      method: 'calc_script_data_hash',
    };
  }
}

const NEEDED_LANG_ORDER = ['PlutusV1', 'PlutusV2', 'PlutusV3'];

function scriptRows(list, language) {
  const rows = [];
  if (!list) return rows;
  for (let index = 0; index < Number(list.len()); index += 1) {
    rows.push({ language, hash: list.get(index).hash().to_hex() });
  }
  return rows;
}

function contextFromCbor(cborHex) {
  const cml = loadCml();
  const tx = cml.Transaction.from_cbor_bytes(Buffer.from(String(cborHex || ''), 'hex'));
  const body = tx.body();
  const witness = tx.witness_set();
  const inlineScripts = [
    ...scriptRows(witness.plutus_v1_scripts(), 'PlutusV1'),
    ...scriptRows(witness.plutus_v2_scripts(), 'PlutusV2'),
    ...scriptRows(witness.plutus_v3_scripts(), 'PlutusV3'),
  ];
  const mintPolicies = [];
  const mint = body.mint();
  if (mint) {
    const keys = mint.keys();
    for (let index = 0; index < Number(keys.len()); index += 1) mintPolicies.push(keys.get(index).to_hex());
  }
  const referenceInputs = [];
  const refs = body.reference_inputs();
  if (refs) {
    for (let index = 0; index < Number(refs.len()); index += 1) {
      const input = refs.get(index);
      referenceInputs.push(`${input.transaction_id().to_hex()}#${Number(input.index())}`);
    }
  }
  const redeemers = [];
  const redeemerSet = witness.redeemers();
  if (redeemerSet) {
    const flat = redeemerSet.to_flat_format();
    for (let index = 0; index < Number(flat.len()); index += 1) {
      const row = flat.get(index);
      redeemers.push({ tag: Number(row.tag()), index: Number(row.index()) });
    }
  }
  const witnessLanguages = [];
  const languages = witness.languages();
  if (languages) {
    for (let index = 0; index < Number(languages.len()); index += 1) {
      const name = NEEDED_LANG_ORDER[Number(languages.get(index))];
      if (name) witnessLanguages.push(name);
    }
  }
  return { redeemers, mintPolicies, inlineScripts, referenceInputs, witnessLanguages };
}

function neededBase(extra = {}) {
  return {
    witnessFieldUsed: false,
    referenceUnionUsed: false,
    ledgerCallee: 'UNKNOWN',
    linkedBinaryAbi: 'UNKNOWN',
    inlineDatumInIntegrity: false,
    nativeReferenceFixture: false,
    nativeBudgetMeasured: false,
    ledgerExecution: false,
    confirmed: false,
    ...extra,
  };
}

export function selectNeededScriptLanguage(input = {}) {
  let decoded = {};
  if (input.cborHex) {
    try {
      decoded = contextFromCbor(input.cborHex);
    } catch (error) {
      return neededBase({
        ok: false,
        code: 'NEEDED_SCRIPT_UNDECIDED',
        needed: [],
        languages: null,
        detail: String(error?.message || error),
      });
    }
  }
  const emptyLists = Array.isArray(input.emptyScriptLists) ? input.emptyScriptLists : [];
  if (emptyLists.some((row) => Number(row?.decoderVersion) >= 9)) {
    return neededBase({
      ok: false,
      code: 'EMPTY_SCRIPT_LIST_REJECTED',
      needed: [],
      languages: null,
      witnessLanguages: input.witnessLanguages || decoded.witnessLanguages || [],
      decoderVersionAtLinkedNode: 'UNKNOWN',
      inlineDatumInIntegrity: false,
    });
  }
  const context = {
    redeemers: input.redeemers || decoded.redeemers || [],
    mintPolicies: input.mintPolicies || decoded.mintPolicies || [],
    inlineScripts: input.inlineScripts || decoded.inlineScripts || [],
    referenceInputs: input.referenceInputs || decoded.referenceInputs || [],
    spendingInputs: input.spendingInputs || [],
    referenceUtxos: input.referenceUtxos || [],
    witnessLanguages: input.witnessLanguages || decoded.witnessLanguages || [],
    spendScripts: input.spendScripts || [],
  };
  const needed = [];
  const unresolved = [];
  const pushRef = (purpose, pointer, scriptHash) => {
    const spendingRefs = context.spendingInputs.map((row) => row.ref || row);
    const refs = context.referenceUtxos.filter((row) => row.scriptHash === scriptHash && (
      context.referenceInputs.includes(row.ref) || spendingRefs.includes(row.ref)
    ));
    if (refs.length !== 1) {
      unresolved.push({
        purpose,
        pointer,
        scriptHash,
        code: refs.length === 0 ? 'NEEDED_SCRIPT_UNRESOLVED' : 'NEEDED_SCRIPT_AMBIGUOUS',
      });
      return;
    }
    needed.push({
      purpose,
      pointer,
      scriptHash,
      language: refs[0].language,
      source: 'reference-utxo',
      ref: refs[0].ref,
    });
  };
  for (const redeemer of context.redeemers) {
    const pointer = Number(redeemer.index);
    if (Number(redeemer.tag) === 1) {
      const scriptHash = context.mintPolicies[pointer] || null;
      if (!scriptHash) {
        unresolved.push({ purpose: 'mint', pointer, code: 'POINTER_MISMATCH' });
        continue;
      }
      const inline = context.inlineScripts.find((script) => script.hash === scriptHash);
      if (inline) {
        needed.push({ purpose: 'mint', pointer, scriptHash, language: inline.language, source: 'inline-script' });
      } else {
        pushRef('mint', pointer, scriptHash);
      }
      continue;
    }
    if (Number(redeemer.tag) === 0) {
      const fromOutput = (Array.isArray(input.spentOutputs) ? input.spentOutputs : [])
        .find((row) => Number(row.pointer) === pointer);
      const fromCaller = context.spendScripts.find((row) => Number(row.pointer) === pointer);
      const outputHash = fromOutput?.paymentScriptHash || null;
      const callerHash = fromCaller?.scriptHash || null;
      if (outputHash && callerHash && outputHash !== callerHash) {
        unresolved.push({ purpose: 'spend', pointer, code: 'SPEND_HASH_DIVERGED' });
        continue;
      }
      const scriptHash = outputHash || callerHash;
      if (!scriptHash) {
        unresolved.push({ purpose: 'spend', pointer, code: 'NEEDED_SCRIPT_UNDECIDED' });
        continue;
      }
      const inline = context.inlineScripts.find((script) => script.hash === scriptHash);
      if (inline) {
        needed.push({ purpose: 'spend', pointer, scriptHash, language: inline.language, source: 'inline-script' });
      } else {
        pushRef('spend', pointer, scriptHash);
      }
      continue;
    }
    unresolved.push({ purpose: 'other', pointer, code: 'NEEDED_SCRIPT_UNDECIDED' });
  }
  if (!context.redeemers.length || unresolved.length || !needed.length) {
    return neededBase({
      ok: false,
      code: unresolved[0]?.code || 'NEEDED_SCRIPT_UNDECIDED',
      needed: [],
      languages: null,
      witnessLanguages: context.witnessLanguages,
      unresolved,
    });
  }
  if (input.costModels && typeof input.costModels === 'object') {
    const missingCostModels = [...new Set(needed.map((row) => row.language))]
      .filter((language) => input.costModels[language] == null);
    if (missingCostModels.length) {
      return neededBase({
        ok: false,
        code: 'COST_MODEL_MISSING',
        needed: [],
        languages: null,
        missingCostModels,
        witnessLanguages: context.witnessLanguages,
      });
    }
  }
  return neededBase({
    ok: true,
    code: 'NEEDED_SCRIPT_SELECTED',
    needed,
    languages: NEEDED_LANG_ORDER.filter((name) => needed.some((row) => row.language === name)),
    witnessLanguages: context.witnessLanguages,
    unresolved: [],
  });
}

function hexKey(value) {
  const text = String(value || '').trim().toLowerCase();
  return /^[0-9a-f]{56}$/.test(text) ? text : null;
}

function collectKeyHashes(rows) {
  if (!Array.isArray(rows)) return [];
  return rows.map(hexKey).filter(Boolean);
}

export function verifyExactBodyWitness(bodyCborHex, witnessCborHex, context = {}) {
  const bodyHex = String(bodyCborHex || '');
  const witnessHex = String(witnessCborHex || '');
  const refused = (code, extra = {}) => ({
    ok: false,
    code,
    signatureVerified: false,
    signatureCheckRan: extra.signatureCheckRan === true,
    bodyBound: extra.bodyBound === true,
    bodyHashHex: extra.bodyHashHex || null,
    witnessCount: extra.witnessCount ?? 0,
    verifiedCount: extra.verifiedCount ?? 0,
    nativeBudgetMeasured: false,
    ledgerExecution: false,
    ledgerSubmitted: false,
    confirmed: false,
    method: 'cml-publickey-verify',
    detail: extra.detail || null,
  });
  try {
    const cml = loadCml();
    const tx = cml.Transaction.from_cbor_bytes(Buffer.from(bodyHex, 'hex'));
    const bodyHash = cml.hash_transaction(tx.body());
    const bodyHashHex = bodyHash.to_hex();
    const bodyHashBytes = bodyHash.to_raw_bytes();
    const signerList = tx.body().required_signers();
    const requiredSigners = [];
    if (signerList) {
      for (let index = 0; index < Number(signerList.len()); index += 1) {
        requiredSigners.push(signerList.get(index).to_hex());
      }
    }
    const witness = cml.TransactionWitnessSet.from_cbor_bytes(Buffer.from(witnessHex, 'hex'));
    const list = witness.vkeywitnesses();
    const count = list ? Number(list.len()) : 0;
    if (!list || count < 1) {
      return refused('WITNESS_SIGNATURE_ABSENT', {
        signatureCheckRan: true,
        bodyBound: true,
        bodyHashHex,
        witnessCount: count,
        requiredSignerCount: requiredSigners.length,
      });
    }
    let verified = 0;
    const verifiedKeyHashes = [];
    for (let index = 0; index < count; index += 1) {
      const row = list.get(index);
      let passed = false;
      let keyHash = null;
      try {
        const vkey = row.vkey();
        keyHash = vkey.hash().to_hex();
        passed = vkey.verify(bodyHashBytes, row.ed25519_signature()) === true;
      } catch {
        passed = false;
      }
      if (passed) {
        verified += 1;
        if (keyHash) verifiedKeyHashes.push(keyHash);
      }
    }
    const ordinary = collectKeyHashes(context.ordinaryKeyHashes || context.ordinaryInputs);
    const collateral = collectKeyHashes(context.collateralKeyHashes || context.collateralInputs);
    const extras = collectKeyHashes([
      ...(context.withdrawalKeyHashes || []),
      ...(context.certificateKeyHashes || []),
      ...(context.voteKeyHashes || []),
      ...(context.poolOwnerKeyHashes || []),
      ...(context.extraKeyHashes || []),
    ]);
    const required = requiredSigners.map((hash) => hash.toLowerCase());
    const neededKeyHashes = [...new Set([...ordinary, ...collateral, ...required, ...extras])];
    let collateralCount = 0;
    try {
      const collateralInputs = tx.body().collateral_inputs();
      collateralCount = collateralInputs ? Number(collateralInputs.len()) : 0;
    } catch {
      collateralCount = 0;
    }
    const collateralOmitted = collateralCount > 0 && collateral.length === 0;
    const requiredSignersBound = required.length > 0 && required.every((hash) => verifiedKeyHashes.includes(hash));
    const neededCovered = neededKeyHashes.length > 0
      && !collateralOmitted
      && neededKeyHashes.every((hash) => verifiedKeyHashes.includes(hash));
    let signatureVerified = verified === count && neededCovered;
    let code = 'WITNESS_SIGNATURE_REJECTED';
    if (signatureVerified) code = 'WITNESS_SIGNATURE_VERIFIED';
    else if (verified === count && required.length > 0 && !requiredSignersBound) code = 'WITNESS_SIGNER_MISMATCH';
    else if (verified === count) code = 'WITNESS_KEYS_INCOMPLETE';
    if (context.inputsResolved === false) {
      signatureVerified = false;
      if (code === 'WITNESS_SIGNATURE_VERIFIED') code = 'WITNESS_CONTEXT_UNRESOLVED';
    }
    return {
      ok: true,
      code,
      signatureVerified,
      signatureCheckRan: true,
      bodyBound: true,
      bodyHashHex,
      witnessCount: count,
      verifiedCount: verified,
      requiredSignerCount: requiredSigners.length,
      requiredSignersBound,
      neededKeyHashes,
      neededKeyCount: neededKeyHashes.length,
      collateralOmitted,
      unitFlagPromoted: false,
      nativeBudgetMeasured: false,
      ledgerExecution: false,
      ledgerSubmitted: false,
      confirmed: false,
      method: 'cml-publickey-verify',
    };
  } catch (error) {
    return refused('WITNESS_SIGNATURE_CHECK_FAILED', { detail: String(error?.message || error) });
  }
}

function held(code, extra = {}) {
  return {
    ok: extra.ok !== false,
    code,
    heldForExecution: true,
    permanentRefusal: false,
    ledgerExecution: false,
    ledgerFunding: false,
    nativeBudgetMeasured: false,
    fundedInNodeInitialFunds: false,
    bootstrapComplete: false,
    parityExecuted: false,
    selfApproval: false,
    ...extra,
  };
}

export function repeatedFixtureKey(hex) {
  const text = String(hex || '').trim().toLowerCase().replace(/^5820/, '');
  if (!/^[0-9a-f]{64}$/.test(text)) return false;
  if (FIXTURE_PAIRS.has(text.slice(0, 2)) && text === text.slice(0, 2).repeat(32)) return true;
  return text === text[0].repeat(64);
}

function readLength(bytes, offset, additional) {
  if (additional < 24) return { value: additional, offset };
  if (additional === 24) return { value: bytes[offset], offset: offset + 1 };
  if (additional === 25) return { value: (bytes[offset] << 8) | bytes[offset + 1], offset: offset + 2 };
  if (additional === 26) return { value: bytes.readUInt32BE(offset), offset: offset + 4 };
  if (additional === 27) {
    const hi = bytes.readUInt32BE(offset);
    const lo = bytes.readUInt32BE(offset + 4);
    if (hi !== 0) throw new Error('length-overflow');
    return { value: lo, offset: offset + 8 };
  }
  throw new Error('unsupported-length');
}

export function countVkeyWitnesses(cborHex) {
  try {
    const text = String(cborHex || '').trim().toLowerCase();
    if (text.length < 2 || text.length % 2 !== 0 || !/^[0-9a-f]+$/.test(text)) return null;
    const bytes = Buffer.from(text, 'hex');
    let offset = 0;
    function readItem() {
      if (offset >= bytes.length) throw new Error('short');
      const initial = bytes[offset];
      offset += 1;
      const major = initial >> 5;
      const additional = initial & 0x1f;
      if (additional === 31) throw new Error('indefinite');
      const length = readLength(bytes, offset, additional);
      offset = length.offset;
      if (major === 0) return { type: 'uint', value: length.value };
      if (major === 1) return { type: 'nint' };
      if (major === 2 || major === 3) {
        offset += length.value;
        if (offset > bytes.length) throw new Error('bytes');
        return { type: 'bytes' };
      }
      if (major === 4) {
        const items = [];
        for (let index = 0; index < length.value; index += 1) items.push(readItem());
        return { type: 'array', items };
      }
      if (major === 5) {
        const entries = [];
        for (let index = 0; index < length.value; index += 1) {
          const key = readItem();
          const value = readItem();
          entries.push([key, value]);
        }
        return { type: 'map', entries };
      }
      if (major === 6) {
        readItem();
        return { type: 'tag' };
      }
      if (major === 7) return { type: 'simple' };
      throw new Error('major');
    }
    const root = readItem();
    if (offset !== bytes.length) return null;
    let witnessSet = null;
    if (root.type === 'array' && root.items[1]?.type === 'map') witnessSet = root.items[1];
    else if (root.type === 'map') witnessSet = root;
    else return null;
    const vkeys = witnessSet.entries.find(([key]) => key.type === 'uint' && key.value === 0);
    if (!vkeys) return 0;
    if (vkeys[1].type !== 'array') return null;
    return vkeys[1].items.length;
  } catch {
    return null;
  }
}

function emptyEnvelope() {
  return {
    ok: false,
    scriptDataHash: null,
    redeemers: [],
    vkeyWitnessCount: null,
    headOutputIndex: null,
    payoutOutputIndex: null,
    changeOutputIndex: null,
    outputCount: null,
  };
}

export function decodeCbor(cborHex) {
  const text = String(cborHex || '').trim().toLowerCase();
  if (text.length < 2 || text.length % 2 !== 0 || !/^[0-9a-f]+$/.test(text)) throw new Error('cbor-hex');
  const bytes = Buffer.from(text, 'hex');
  let offset = 0;
  function readLen(additional) {
    if (additional < 24) return BigInt(additional);
    if (additional === 24) return BigInt(bytes[offset++]);
    if (additional === 25) {
      const value = bytes.readUInt16BE(offset);
      offset += 2;
      return BigInt(value);
    }
    if (additional === 26) {
      const value = bytes.readUInt32BE(offset);
      offset += 4;
      return BigInt(value);
    }
    if (additional === 27) {
      const hi = bytes.readUInt32BE(offset);
      const lo = bytes.readUInt32BE(offset + 4);
      offset += 8;
      return (BigInt(hi) << 32n) + BigInt(lo);
    }
    throw new Error('cbor-length');
  }
  function asCount(value) {
    if (value > 1000000n) throw new Error('cbor-count');
    return Number(value);
  }
  function readItem() {
    if (offset >= bytes.length) throw new Error('cbor-short');
    const initial = bytes[offset];
    offset += 1;
    if (initial === 0xff) return { t: 'break' };
    const major = initial >> 5;
    const additional = initial & 0x1f;
    if (additional === 31) {
      if (major === 2 || major === 3) {
        const chunks = [];
        while (bytes[offset] !== 0xff) {
          const chunk = readItem();
          if (chunk.t !== (major === 2 ? 'bytes' : 'text')) throw new Error('cbor-chunk');
          chunks.push(major === 2 ? Buffer.from(chunk.hex, 'hex') : chunk.text);
        }
        offset += 1;
        if (major === 2) {
          const buf = Buffer.concat(chunks);
          return { t: 'bytes', n: buf.length, hex: buf.toString('hex') };
        }
        return { t: 'text', text: chunks.join('') };
      }
      if (major === 4) {
        const items = [];
        while (bytes[offset] !== 0xff) items.push(readItem());
        offset += 1;
        return { t: 'array', items };
      }
      if (major === 5) {
        const entries = [];
        while (bytes[offset] !== 0xff) entries.push([readItem(), readItem()]);
        offset += 1;
        return { t: 'map', entries };
      }
      throw new Error('cbor-indefinite');
    }
    const length = readLen(additional);
    if (major === 0) return { t: 'uint', v: length };
    if (major === 1) return { t: 'nint', v: length };
    if (major === 2 || major === 3) {
      const size = asCount(length);
      const start = offset;
      offset += size;
      if (offset > bytes.length) throw new Error('cbor-bytes');
      if (major === 2) return { t: 'bytes', n: size, hex: bytes.subarray(start, offset).toString('hex') };
      return { t: 'text', text: bytes.subarray(start, offset).toString('utf8') };
    }
    if (major === 4) {
      const items = [];
      for (let index = 0; index < asCount(length); index += 1) items.push(readItem());
      return { t: 'array', items };
    }
    if (major === 5) {
      const entries = [];
      for (let index = 0; index < asCount(length); index += 1) entries.push([readItem(), readItem()]);
      return { t: 'map', entries };
    }
    if (major === 6) return { t: 'tag', tag: length, inner: readItem() };
    if (major === 7) return { t: 'simple', v: length };
    throw new Error('cbor-major');
  }
  const root = readItem();
  if (offset !== bytes.length) throw new Error('cbor-trailing');
  return root;
}

function mapByUintKey(map) {
  const out = new Map();
  if (!map || map.t !== 'map') return out;
  for (const [key, value] of map.entries) {
    if (key.t === 'uint' && key.v <= BigInt(Number.MAX_SAFE_INTEGER)) out.set(Number(key.v), value);
  }
  return out;
}

function uintValue(item) {
  if (!item || item.t !== 'uint' || item.v > BigInt(Number.MAX_SAFE_INTEGER)) return null;
  return Number(item.v);
}

function readValueShape(item) {
  if (!item) return { lovelaceOnly: false, hasAssets: false };
  if (item.t === 'uint') return { lovelaceOnly: true, hasAssets: false };
  if (item.t === 'array') {
    const assets = item.items[1];
    const hasAssets = Boolean(assets && assets.t === 'map' && assets.entries.length > 0);
    return { lovelaceOnly: !hasAssets, hasAssets };
  }
  return { lovelaceOnly: false, hasAssets: false };
}

function readInlineDatum(item) {
  if (!item || item.t !== 'array' || item.items.length < 2) return null;
  if (uintValue(item.items[0]) !== 1) return null;
  const wrapped = item.items[1];
  if (!wrapped || wrapped.t !== 'tag' || wrapped.tag !== 24n || wrapped.inner?.t !== 'bytes') return null;
  const data = decodeCbor(wrapped.inner.hex);
  if (data.t === 'tag' && data.tag >= 121n && data.tag <= 127n && data.inner?.t === 'array') {
    return { datumConstructor: Number(data.tag - 121n), datumFieldCount: data.inner.items.length };
  }
  return null;
}

function readOutputs(item) {
  if (!item || item.t !== 'array') return [];
  return item.items.map((output) => {
    let value = null;
    let datum = null;
    if (output.t === 'array') {
      value = output.items[1] || null;
    } else if (output.t === 'map') {
      const fields = mapByUintKey(output);
      value = fields.get(1) || null;
      datum = readInlineDatum(fields.get(2));
    }
    return {
      ...readValueShape(value),
      datumConstructor: datum ? datum.datumConstructor : null,
      datumFieldCount: datum ? datum.datumFieldCount : null,
    };
  });
}

function classifyOutputs(outputs) {
  let head = null;
  let payout = null;
  let change = null;
  outputs.forEach((output, index) => {
    if (output.datumConstructor === 0 && output.datumFieldCount === 13) head = index;
    else if (output.lovelaceOnly) change = index;
    else if (output.hasAssets) payout = index;
  });
  if (head == null) {
    const asset = outputs.findIndex((output) => output.hasAssets);
    if (asset >= 0) head = asset;
    else if (outputs.length === 1) head = 0;
  }
  return {
    headOutputIndex: head,
    payoutOutputIndex: payout,
    changeOutputIndex: change,
    outputCount: outputs.length,
  };
}

function readRedeemers(item) {
  const rows = [];
  if (!item) return rows;
  if (item.t === 'map') {
    for (const [key, value] of item.entries) {
      if (key.t !== 'array' || key.items.length !== 2 || value.t !== 'array' || value.items.length < 2) continue;
      const units = value.items[1];
      if (units.t !== 'array' || units.items.length !== 2) continue;
      const tag = uintValue(key.items[0]);
      const index = uintValue(key.items[1]);
      const memory = uintValue(units.items[0]);
      const steps = uintValue(units.items[1]);
      if (tag == null || index == null || memory == null || steps == null) continue;
      rows.push({ tag, index, memory, steps });
    }
  } else if (item.t === 'array') {
    for (const entry of item.items) {
      if (entry.t !== 'array' || entry.items.length !== 4 || entry.items[3]?.t !== 'array') continue;
      const tag = uintValue(entry.items[0]);
      const index = uintValue(entry.items[1]);
      const memory = uintValue(entry.items[3].items[0]);
      const steps = uintValue(entry.items[3].items[1]);
      if (tag == null || index == null || memory == null || steps == null) continue;
      rows.push({ tag, index, memory, steps });
    }
  }
  return rows;
}

function countStrictVkeys(item) {
  if (!item) return 0;
  if (item.t !== 'array') return null;
  let count = 0;
  for (const witness of item.items) {
    if (witness.t !== 'array' || witness.items.length !== 2) return null;
    const [vkey, signature] = witness.items;
    if (vkey.t !== 'bytes' || signature.t !== 'bytes' || vkey.n !== 32 || signature.n !== 64) return null;
    count += 1;
  }
  return count;
}

export function readConwayEnvelope(cborHex) {
  try {
    const root = decodeCbor(cborHex);
    if (!root || root.t !== 'array' || root.items.length < 2) return emptyEnvelope();
    const body = root.items[0];
    const witness = root.items[1];
    if (body.t !== 'map' || witness.t !== 'map') return emptyEnvelope();
    const bodyMap = mapByUintKey(body);
    const witnessMap = mapByUintKey(witness);
    const hashItem = bodyMap.get(11);
    const scriptDataHash = hashItem && hashItem.t === 'bytes' && hashItem.n === 32 ? hashItem.hex : null;
    const outputs = readOutputs(bodyMap.get(1));
    return {
      ok: true,
      scriptDataHash,
      redeemers: readRedeemers(witnessMap.get(5)),
      vkeyWitnessCount: countStrictVkeys(witnessMap.get(0)),
      outputs,
      ...classifyOutputs(outputs),
    };
  } catch {
    return emptyEnvelope();
  }
}

export function countShelleyVkeyWitnesses(cborHex) {
  try {
    const root = decodeCbor(cborHex);
    const witness = root?.t === 'map'
      ? root
      : (root?.t === 'array' && root.items[1]?.t === 'map' ? root.items[1] : null);
    if (!witness) return null;
    return countStrictVkeys(mapByUintKey(witness).get(0));
  } catch {
    return null;
  }
}

function cborHead(major, length) {
  const value = typeof length === 'bigint' ? length : BigInt(length);
  if (value < 0n) throw new Error('cbor-negative');
  if (value < 24n) return Buffer.from([(major << 5) | Number(value)]);
  if (value < 256n) return Buffer.from([(major << 5) | 24, Number(value)]);
  if (value < 65536n) {
    const buf = Buffer.alloc(3);
    buf[0] = (major << 5) | 25;
    buf.writeUInt16BE(Number(value), 1);
    return buf;
  }
  if (value < 4294967296n) {
    const buf = Buffer.alloc(5);
    buf[0] = (major << 5) | 26;
    buf.writeUInt32BE(Number(value), 1);
    return buf;
  }
  const buf = Buffer.alloc(9);
  buf[0] = (major << 5) | 27;
  buf.writeUInt32BE(Number(value >> 32n), 1);
  buf.writeUInt32BE(Number(value & 0xffffffffn), 5);
  return buf;
}

function cborUint(value) {
  return cborHead(0, value);
}

function cborBytes(buf) {
  return Buffer.concat([cborHead(2, buf.length), buf]);
}

function cborArray(items) {
  return Buffer.concat([cborHead(4, items.length), ...items]);
}

function cborMap(pairs) {
  return Buffer.concat([cborHead(5, pairs.length), ...pairs.flat()]);
}

function cborTag(tag, inner) {
  return Buffer.concat([cborHead(6, tag), inner]);
}

function flagValue(argv, name) {
  const index = argv.indexOf(name);
  return index >= 0 ? argv[index + 1] : '';
}

function parseRefBytes(text) {
  const matched = String(text || '').match(/^([0-9a-f]{64})#([0-9]+)$/i);
  if (!matched) return null;
  return { tx: Buffer.from(matched[1], 'hex'), index: matched[2] };
}

function cborRef(ref) {
  return cborArray([cborBytes(ref.tx), cborUint(ref.index)]);
}

function addressBytes(address) {
  return Buffer.concat([Buffer.from([0x60]), createHash('sha256').update(String(address)).digest().subarray(0, 28)]);
}

function parseAssetToken(text) {
  const matched = String(text || '').match(/^([1-9][0-9]*) ([0-9a-f]{56})\.([0-9a-f]+)$/i);
  if (!matched) return null;
  return { quantity: matched[1], policy: matched[2].toLowerCase(), name: matched[3].toLowerCase() };
}

function cborAssets(assets) {
  const byPolicy = new Map();
  for (const asset of assets) {
    if (!byPolicy.has(asset.policy)) byPolicy.set(asset.policy, []);
    byPolicy.get(asset.policy).push(asset);
  }
  return cborMap([...byPolicy].map(([policy, names]) => [
    cborBytes(Buffer.from(policy, 'hex')),
    cborMap(names.map((asset) => [cborBytes(Buffer.from(asset.name, 'hex')), cborUint(asset.quantity)])),
  ]));
}

function cborDatum(constructor, fieldCount) {
  if (!Number.isInteger(constructor) || constructor < 0 || constructor > 6) return null;
  const fields = [];
  for (let index = 0; index < fieldCount; index += 1) fields.push(cborUint(0));
  return cborTag(24, cborBytes(cborTag(121 + constructor, cborArray(fields))));
}

function cborOutput(output) {
  const addr = cborBytes(addressBytes(output.address));
  const coin = cborUint(output.lovelace);
  const value = output.assets.length ? cborArray([coin, cborAssets(output.assets)]) : coin;
  if (output.datumConstructor == null) return cborArray([addr, value]);
  const datum = cborDatum(output.datumConstructor, output.datumFieldCount);
  if (!datum) return cborArray([addr, value]);
  return cborMap([
    [cborUint(0), addr],
    [cborUint(1), value],
    [cborUint(2), cborArray([cborUint(1), datum])],
  ]);
}

function collectArgvOutputs(argv) {
  const outputs = [];
  for (let index = 0; index < argv.length; index += 1) {
    if (argv[index] === '--tx-out') {
      const parts = String(argv[index + 1] || '').split('+');
      const assets = parts.slice(2).map(parseAssetToken).filter(Boolean);
      outputs.push({
        address: parts[0],
        lovelace: parts[1] || '0',
        assets,
        datumConstructor: null,
        datumFieldCount: null,
      });
    } else if (argv[index] === '--tx-out-inline-datum-file' && outputs.length > 0) {
      const parsed = JSON.parse(readFileSync(argv[index + 1], 'utf8'));
      const fieldCount = Array.isArray(parsed?.fields) ? parsed.fields.length : 0;
      if (Number.isInteger(parsed?.constructor)) {
        outputs[outputs.length - 1].datumConstructor = parsed.constructor;
        outputs[outputs.length - 1].datumFieldCount = fieldCount;
      }
    }
  }
  return outputs;
}

function unitPair(text) {
  const matched = String(text || '').match(/^\(([0-9]+),([0-9]+)\)$/);
  if (!matched) return null;
  const memory = Number(matched[1]);
  const steps = Number(matched[2]);
  if (!Number.isSafeInteger(memory) || !Number.isSafeInteger(steps)) return null;
  return { memory, steps };
}

function compareSpendRefs(left, right) {
  const cmp = Buffer.compare(left.tx, right.tx);
  if (cmp !== 0) return cmp;
  return Number(left.index) - Number(right.index);
}

function normalizeRefText(value) {
  if (typeof value === 'string') return value.trim().toLowerCase();
  if (value?.txId != null && value.index != null) return `${value.txId}#${value.index}`.toLowerCase();
  return '';
}

export function sortedOrdinaryRefs(refs) {
  return [...(Array.isArray(refs) ? refs : [])]
    .map((ref) => {
      const text = normalizeRefText(ref);
      const parsed = parseRefBytes(text);
      return parsed ? { text, tx: parsed.tx, index: parsed.index } : null;
    })
    .filter(Boolean)
    .sort(compareSpendRefs)
    .map((ref) => ref.text);
}

export function redeemerPointerAccepts(row, context = {}) {
  const tag = Number(row?.tag);
  const index = Number(row?.index);
  if (!Number.isInteger(index) || index < 0) return false;
  if (tag === 1) {
    const count = Number.isInteger(context.mintPolicyCount) ? context.mintPolicyCount : 1;
    return count <= 1 ? index === 0 : index < count;
  }
  if (tag !== 0) return false;
  const scriptRef = normalizeRefText(context.scriptRef);
  const sorted = sortedOrdinaryRefs(context.ordinaryInputs);
  if (!scriptRef || sorted.length === 0) return false;
  return sorted.indexOf(scriptRef) === index;
}

function collectUnitInputs(list) {
  const inputs = [];
  const collateral = [];
  let current = null;
  for (let index = 0; index < list.length; index += 1) {
    const flag = list[index];
    if (flag === '--tx-in') {
      const text = String(list[index + 1] || '');
      const ref = parseRefBytes(text);
      current = ref ? { ...ref, script: false, text: text.toLowerCase() } : null;
      if (current) inputs.push(current);
      index += 1;
    } else if (flag === '--tx-in-collateral') {
      const ref = parseRefBytes(list[index + 1]);
      if (ref) collateral.push(ref);
      current = null;
      index += 1;
    } else if (flag === '--read-only-tx-in-reference' || flag === '--tx-in-reference') {
      current = null;
      index += 1;
    } else if (current && (
      flag === '--tx-in-script-file'
      || flag === '--tx-in-redeemer-file'
      || flag === '--tx-in-inline-datum-file'
      || flag === '--tx-in-execution-units'
    )) {
      current.script = true;
    }
  }
  inputs.sort(compareSpendRefs);
  return { inputs, collateral };
}

export function encodeUnitConwayEnvelope(argv) {
  const list = Array.isArray(argv) ? argv : [];
  const collected = collectUnitInputs(list);
  const inputs = collected.inputs;
  const collateral = collected.collateral;
  const fee = flagValue(list, '--fee') || '0';
  const mintUnits = unitPair(flagValue(list, '--mint-execution-units'));
  const spendUnits = unitPair(flagValue(list, '--tx-in-execution-units'));
  const redeemers = [];
  if (mintUnits) redeemers.push({ tag: 1, index: 0, ...mintUnits });
  if (spendUnits) {
    const scripts = inputs.filter((row) => row.script);
    const spendIndex = scripts.length === 1
      ? inputs.indexOf(scripts[0])
      : (inputs.length === 1 ? 0 : null);
    if (spendIndex != null) redeemers.push({ tag: 0, index: spendIndex, ...spendUnits });
  }
  const protocol = flagValue(list, '--protocol-params-file');
  const outputs = collectArgvOutputs(list);
  const invalidHereafter = flagValue(list, '--invalid-hereafter');
  const invalidBefore = flagValue(list, '--invalid-before');
  const finish = (hash) => {
    const bodyPairs = [
      [cborUint(0), cborTag(258, cborArray(inputs.map(cborRef)))],
      [cborUint(1), cborArray(outputs.map(cborOutput))],
      [cborUint(2), cborUint(fee)],
    ];
    if (/^[1-9][0-9]*$/.test(invalidHereafter)) bodyPairs.push([cborUint(3), cborUint(invalidHereafter)]);
    if (/^[1-9][0-9]*$/.test(invalidBefore)) bodyPairs.push([cborUint(8), cborUint(invalidBefore)]);
    if (hash) bodyPairs.push([cborUint(11), cborBytes(hash)]);
    if (collateral.length) bodyPairs.push([cborUint(13), cborTag(258, cborArray(collateral.map(cborRef)))]);
    const witnessPairs = redeemers.length
      ? [[cborUint(5), cborMap(redeemers.map((row) => [
        cborArray([cborUint(row.tag), cborUint(row.index)]),
        cborArray([cborUint(0), cborArray([cborUint(row.memory), cborUint(row.steps)])]),
      ]))]]
      : [];
    const tx = cborArray([
      cborMap(bodyPairs),
      cborMap(witnessPairs),
      Buffer.from([0xf5]),
      Buffer.from([0xf6]),
    ]);
    return JSON.stringify({
      type: 'Tx ConwayEra',
      description: 'unsigned financial body',
      cborHex: tx.toString('hex'),
    });
  };
  if (protocol && redeemers.length) {
    const draft = JSON.parse(finish(null));
    const integrity = recomputeScriptDataHash(draft.cborHex, { protocolPath: protocol });
    if (integrity.recomputedHex && /^[0-9a-f]{64}$/.test(integrity.recomputedHex)) {
      const candidate = Buffer.from(integrity.recomputedHex, 'hex');
      const built = JSON.parse(finish(candidate));
      const check = recomputeScriptDataHash(built.cborHex, { protocolPath: protocol });
      if (check.matchesStored === true && check.nativeBudgetMeasured === false && check.cborPreserved === true) {
        return finish(candidate);
      }
    }
  }
  return finish(null);
}

export function encodeUnitWitnessCborHex() {
  const witness = cborMap([[
    cborUint(0),
    cborArray([cborArray([cborBytes(Buffer.alloc(32, 0x11)), cborBytes(Buffer.alloc(64, 0x22))])]),
  ]]);
  return witness.toString('hex');
}

export function bindSignedWitnessCount({
  unitOnly = true,
  signedCborHex = null,
  signingKeyCount = 0,
  bodyCborHex = null,
  neededKeys = null,
} = {}) {
  if (unitOnly !== false && !signedCborHex) {
    return {
      ok: true,
      code: 'WITNESS_COUNT_BOUND_UNIT_ONLY',
      witnessCountVerified: false,
      signatureVerified: false,
      signingKeyCount,
      ledgerExecution: false,
      nativeBudgetMeasured: false,
      unitOnly: true,
    };
  }
  const counted = countShelleyVkeyWitnesses(signedCborHex);
  const unit = unitOnly !== false;
  if (counted == null || counted !== signingKeyCount || signingKeyCount < 1) {
    return {
      ok: false,
      code: 'WITNESS_SET_NOT_COUNTED',
      witnessCountVerified: false,
      signatureVerified: false,
      counted,
      signingKeyCount,
      ledgerExecution: false,
      nativeBudgetMeasured: false,
      unitOnly: unit,
    };
  }
  const crypto = bodyCborHex
    ? verifyExactBodyWitness(bodyCborHex, signedCborHex, neededKeys || {})
    : {
      signatureVerified: false,
      signatureCheckRan: false,
      bodyBound: false,
      bodyHashHex: null,
      code: 'WITNESS_BODY_UNBOUND',
    };
  return {
    ok: true,
    code: unit ? 'WITNESS_SET_COUNTED_UNIT' : 'WITNESS_SET_COUNTED',
    witnessCountVerified: true,
    signatureVerified: crypto.signatureVerified === true,
    signatureCheckRan: crypto.signatureCheckRan === true,
    bodyBound: crypto.bodyBound === true,
    bodyHashHex: crypto.bodyHashHex || null,
    signatureCode: crypto.code || null,
    requiredSignersBound: crypto.requiredSignersBound === true,
    requiredSignerCount: crypto.requiredSignerCount || 0,
    counted,
    signingKeyCount,
    ledgerExecution: false,
    ledgerSubmitted: false,
    confirmed: false,
    nativeBudgetMeasured: false,
    unitOnly: unit,
  };
}

export function classifyClockStamp(systemStart) {
  if (systemStart === CREATE_TIME_SYSTEM_START) {
    return { ok: false, code: 'CREATE_TIME_CLOCK_NOT_REWRITTEN', parityExecuted: false };
  }
  if (systemStart === TEMPLATE_SYSTEM_START) {
    return { ok: false, code: 'STALE_TEMPLATE_CLOCK', parityExecuted: false };
  }
  if (typeof systemStart !== 'string' || Number.isNaN(Date.parse(systemStart))) {
    return { ok: false, code: 'CLOCK_STAMP_ABSENT', parityExecuted: false };
  }
  return { ok: true, code: 'OWNED_CLOCK_STAMP', parityExecuted: false, systemStart };
}

export function compareClockParity({ cliPosix = null, lucidPosix = null, scriptContextLower = null } = {}) {
  const present = [cliPosix, lucidPosix, scriptContextLower].every((value) => value != null && value !== '');
  if (!present) {
    return held('POSIX_PARITY_NOT_RUN', {
      ok: false,
      cliBound: false,
      lucidBound: false,
      scriptBound: false,
    });
  }
  const same = String(cliPosix) === String(lucidPosix) && String(lucidPosix) === String(scriptContextLower);
  return {
    ok: same,
    code: same ? 'POSIX_PARITY_MATCH' : 'POSIX_PARITY_MISMATCH',
    parityExecuted: true,
    ledgerExecution: false,
    measured: false,
    heldForExecution: false,
    permanentRefusal: false,
  };
}

export function financialContinuation(plan = {}) {
  const ready = plan.inputsReady === true;
  const magic = Number.isInteger(plan.magic) ? plan.magic : null;
  return held(ready ? 'FINANCIAL_CONTINUATION_BOUND' : 'FINANCIAL_CONTINUATION_BLOCKED', {
    ok: ready,
    inputSource: 'owned-bootstrap',
    callerSuppliedGenesisIgnored: true,
    phases: [
      'cash-mint',
      'cash-holding',
      'head-open',
      'financial-spend',
      'race-both-before-poll',
      'receipt-helper',
      'selected-utxo-readback',
    ],
    kernel: {
      cashQuantity: CASH_QUANTITY,
      headQuantity: HEAD_QUANTITY,
      constructorWork: CONSTRUCTOR_WORK,
      collectionCapacity: COLLECTION_CAPACITY,
      headFields: HEAD_FIELD_COUNT,
      quantitiesAreLovelace: false,
    },
    race: {
      bothSignedSubmitsBeforeAnyPoll: true,
      sharedHeadOnly: true,
      feeCollateralDisjoint: true,
    },
    receipt: {
      helperPath: RECEIPT_HELPER_PATH,
      helperSha256: RECEIPT_HELPER_SHA256,
      receiptStatusMarkerIsNotConfirmation: true,
    },
    plutusRebuild: ['exunits', 'scriptDataHash', 'fee', 'change', 'minAda', 'outputs', 'cbor', 'txid'],
    helperModule: HELPER_MODULE,
    frozenHelperUnchanged: true,
    dijkstraGenesisWritten: false,
    networkMagic: magic,
    genesisMagicBound: magic != null && magic !== 42,
    encodingFixturesAreNotRoleWallets: true,
    twelveInputFrameSatisfied: false,
  });
}

function phaseLink(name, spends, produces) {
  return { phase: name, spends, produces, ledgerExecution: false };
}

export function bindFinancialFromDispatch(dispatch = {}, plan = {}) {
  const results = Array.isArray(dispatch.results) ? dispatch.results : [];
  const funder = results.find((item) => item.id === 'query-funder-utxo') || null;
  const roles = ['issuer', 'lender', 'borrower'].map((name) => ({
    name,
    result: results.find((item) => item.id === `funding-confirm-${name}`) || null,
  }));
  const txins = [funder?.txIn, ...roles.map((role) => role.result?.txIn)].filter((item) => typeof item === 'string');
  const disjoint = txins.length === 4 && new Set(txins).size === 4;
  const financialSlots = results.filter((item) => typeof item?.id === 'string' && item.id.startsWith('fin-') && item.ok === true && typeof item.txIn === 'string');
  const twelveInputFrameSatisfied = financialSlots.length === 12 && new Set(financialSlots.map((item) => item.txIn)).size === 12;
  const lender = roles.find((role) => role.name === 'lender');
  const chain = disjoint ? {
    cashMint: phaseLink('cash-mint', funder.txIn, 'cash-holding'),
    cashHolding: phaseLink('cash-holding', 'cash-mint-output', 'head-open-seed'),
    headOpen: phaseLink('head-open', lender?.result?.txIn || null, 'head'),
    financialSpend: phaseLink('financial-spend', 'head-open-output', 'terminal'),
    race: phaseLink('race-both-before-poll', 'head-only', 'two-submits'),
    receipt: phaseLink('receipt-helper', 'assembled-tx', 'local-txid'),
    readback: phaseLink('selected-utxo-readback', 'selected-tx-in', 'full-13-frame'),
    cashQuantity: CASH_QUANTITY,
    headQuantity: HEAD_QUANTITY,
    quantitiesAreLovelace: false,
    constructorWork: CONSTRUCTOR_WORK,
    collectionCapacity: COLLECTION_CAPACITY,
    headFields: HEAD_FIELD_COUNT,
  } : null;
  return held(disjoint ? 'FINANCIAL_ROLES_BOUND_UNIT_ONLY' : 'FINANCIAL_CONTINUATION_HELD', {
    ok: true,
    roleUtxosDisjoint: disjoint,
    roleCount: roles.filter((role) => role.result).length,
    funderTxIn: funder?.txIn || null,
    quantitiesAreLovelace: false,
    cashQuantity: CASH_QUANTITY,
    headQuantity: HEAD_QUANTITY,
    constructorWork: CONSTRUCTOR_WORK,
    collectionCapacity: COLLECTION_CAPACITY,
    headFields: HEAD_FIELD_COUNT,
    twelveInputFrameSatisfied,
    reason: twelveInputFrameSatisfied
      ? 'twelve-producer-role-txins'
      : (disjoint ? 'four-genesis-identities-later-phases-link' : 'confirmed-role-txins-are-not-disjoint'),
    chain,
    networkMagic: Number.isInteger(plan.magic) ? plan.magic : null,
    unitOnly: dispatch.unitOnly !== false,
  });
}

export function planPlutusCostRebuild(costResult, { unitOnly = true } = {}) {
  const base = {
    applied: false,
    nativeBudgetMeasured: false,
    submitBlocked: true,
    scriptDataHash: null,
    scriptDataHashInvented: false,
    heldForExecution: true,
    permanentRefusal: false,
    ledgerExecution: false,
  };
  if (!costResult || costResult.ok !== true) {
    return { ...base, ok: true, code: 'EVALUATION_HELD', reason: 'calculate-plutus-script-cost-absent' };
  }
  const memory = costResult.cost?.memory;
  const steps = costResult.cost?.steps;
  if (memory === 1 && steps === 1 && costResult.executionSource === 'supplied') {
    return { ...base, ok: false, code: 'SUPPLIED_UNITS_NOT_A_MEASUREMENT' };
  }
  if (unitOnly !== false) {
    return {
      ...base,
      ok: true,
      code: 'PLUTUS_COST_REBUILD_HELD',
      observedMemory: memory ?? null,
      observedSteps: steps ?? null,
    };
  }
  return {
    ...base,
    ok: true,
    code: 'PLUTUS_COST_REBUILD_READY',
    observedMemory: memory ?? null,
    observedSteps: steps ?? null,
    reason: 'rebuild-awaits-final-body-and-cli-script-data-hash',
  };
}

export function readPinnedNodeConfig(path) {
  const parsed = JSON.parse(readFileSync(path, 'utf8'));
  return {
    namesDijkstra: Object.prototype.hasOwnProperty.call(parsed, 'DijkstraGenesisFile'),
    shelleyGenesisFile: parsed.ShelleyGenesisFile || null,
  };
}
