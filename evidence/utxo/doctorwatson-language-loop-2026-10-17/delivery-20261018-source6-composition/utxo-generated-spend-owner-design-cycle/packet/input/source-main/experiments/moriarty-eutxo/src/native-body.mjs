/**
 * Offline Conway body encoding. cardano-cli build-raw writes an unsigned body.
 * No node, no signing key, no submission. A passing txid is not ledger acceptance.
 * Collateral is a ledger-body field. The Plutus V3 Transaction type has no collateral field.
 */
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, mkdtempSync, readFileSync, realpathSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { blake2b } from '../owned/node_modules/@noble/hashes/blake2b.js';
import { readConwayEnvelope } from './native-integrate.mjs';

export const CARDANO_CLI = '/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/runtime/devnet/cardano-node/bin/cardano-cli';
export const ENCODING_VKEY_HEX = `5820${'ab'.repeat(32)}`;
const HEX28 = /^[0-9a-f]{56}$/;
const HEX32 = /^[0-9a-f]{64}$/;
const FIXTURE_PAIRS = new Set(['ab', 'c3', 'd4', 'f5', 'cd']);

export function genesisMagicText(value) {
  const magic = value != null && typeof value === 'object' ? (value.networkMagic ?? value.magic) : value;
  if (magic == null || magic === '') return { ok: false, code: 'GENESIS_MAGIC_ABSENT', genesisMagicBound: false };
  if (!Number.isInteger(magic) || magic < 0 || magic === 42) {
    return { ok: false, code: 'MAGIC_SOURCE_REFUSED', genesisMagicBound: false };
  }
  return { ok: true, text: String(magic), genesisMagicBound: true };
}

function repeatedFixtureMaterial(vkeyHex) {
  const body = String(vkeyHex || '').replace(/^5820/, '').toLowerCase();
  if (!/^[0-9a-f]{64}$/.test(body)) return true;
  const banned = FIXTURE_PAIRS.has(body.slice(0, 2)) && body === body.slice(0, 2).repeat(32);
  return banned || body === body[0].repeat(64);
}

export function paymentKeyHashOfVkeyCbor(cborHex) {
  const hex = String(cborHex || '').trim().toLowerCase();
  if (!/^5820[0-9a-f]{64}$/.test(hex)) return null;
  return Buffer.from(blake2b(Buffer.from(hex.slice(4), 'hex'), { dkLen: 28 })).toString('hex');
}

function resolveSigner(spec) {
  const absent = spec?.paymentVkeyCbor == null || String(spec.paymentVkeyCbor).trim() === '';
  if (absent) {
    const derived = paymentKeyHashOfVkeyCbor(ENCODING_VKEY_HEX);
    if (!derived) return { error: closed('KEYHASH_UNRESOLVED', { spawned: false }) };
    return { hash: derived, vkeyHex: ENCODING_VKEY_HEX, encodingFixture: true };
  }
  const vkeyHex = String(spec.paymentVkeyCbor).trim().toLowerCase();
  if (vkeyHex === ENCODING_VKEY_HEX || repeatedFixtureMaterial(vkeyHex)) {
    return { error: closed('ENCODING_FIXTURE_NOT_A_ROLE', { spawned: false, encodingFixture: true }) };
  }
  const derived = paymentKeyHashOfVkeyCbor(vkeyHex);
  if (!derived) return { error: closed('KEYHASH_UNRESOLVED', { spawned: false }) };
  const supplied = spec?.requiredSignerHash;
  if (supplied != null && String(supplied) !== '') {
    if (!HEX28.test(String(supplied))) return { error: closed('MALFORMED_ID', { spawned: false }) };
    if (String(supplied) !== derived) {
      return {
        error: closed('KEYHASH_MISMATCH', {
          spawned: false,
          supplied,
          derivedPaymentHash: derived,
          reinterpretedKeyHashAsPubkey: false,
        }),
      };
    }
  }
  return { hash: derived, vkeyHex, encodingFixture: false };
}

function earlyRequiredSigner(spec) {
  const supplied = spec?.requiredSignerHash;
  if (supplied == null || String(supplied) === '') return null;
  if (!HEX28.test(String(supplied))) return closed('MALFORMED_ID', { spawned: false });
  const explicit = spec?.paymentVkeyCbor != null && String(spec.paymentVkeyCbor).trim() !== '';
  const vkeyHex = explicit ? String(spec.paymentVkeyCbor).trim().toLowerCase() : ENCODING_VKEY_HEX;
  const derived = paymentKeyHashOfVkeyCbor(vkeyHex);
  if (!derived) return closed('KEYHASH_UNRESOLVED', { spawned: false });
  if (String(supplied) !== derived) {
    return closed('KEYHASH_MISMATCH', {
      spawned: false,
      supplied: String(supplied),
      derivedPaymentHash: derived,
      reinterpretedKeyHashAsPubkey: false,
    });
  }
  return null;
}

function closed(code, extra = {}) {
  return {
    ok: false,
    code,
    ledgerExecution: false,
    submitted: false,
    confirmed: false,
    signed: false,
    nativeAcceptance: false,
    scriptContextHasCollateralField: false,
    ...extra,
  };
}

function refOf(input) {
  if (!input || !HEX32.test(String(input.txId || '')) || !Number.isInteger(input.index) || input.index < 0) return null;
  return `${input.txId}#${input.index}`;
}

function run(cli, args) {
  return spawnSync(cli, args, { encoding: 'utf8', maxBuffer: 16 * 1024 * 1024 });
}

function isRealCli(cliPath) {
  try {
    return Boolean(cliPath) && realpathSync(cliPath) === realpathSync(CARDANO_CLI);
  } catch {
    return false;
  }
}

function canonicalDigit(value) {
  if (typeof value === 'bigint') {
    const text = value.toString();
    return /^(?:0|[1-9][0-9]*)$/.test(text) ? text : null;
  }
  if (typeof value === 'number') {
    if (!Number.isSafeInteger(value) || value < 0) return null;
    return String(value);
  }
  const text = typeof value === 'string' ? value : '';
  return /^(?:0|[1-9][0-9]*)$/.test(text) ? text : null;
}

function inputValueRows(spec) {
  if (!Array.isArray(spec?.inputValues) || spec.inputValues.length === 0) return { rows: null, balance: null };
  const rows = [];
  for (const row of spec.inputValues) {
    const ref = String(row?.ref || '');
    if (!/^[0-9a-f]{64}#\d+$/.test(ref)) return { error: 'BALANCE_INPUT_MALFORMED' };
    const lovelaceText = canonicalDigit(row.lovelaceText ?? row.lovelace);
    if (lovelaceText == null) return { error: 'BALANCE_INPUT_MALFORMED' };
    rows.push({ ref, lovelaceText });
  }
  return { rows };
}

function balanceCover(rows, outputs, feeText) {
  let sum = 0n;
  for (const row of rows) sum += BigInt(row.lovelaceText);
  let need = BigInt(feeText);
  for (const output of outputs) need += BigInt(output);
  return {
    balanced: sum >= need,
    inputLovelaceText: sum.toString(),
    outputAndFeeText: need.toString(),
  };
}

function remainderLovelace(rows, outputs, feeText) {
  const cover = balanceCover(rows, outputs, feeText);
  if (cover.balanced !== true) return { ok: false, code: 'BALANCE_SHORT', balance: cover };
  const change = BigInt(cover.inputLovelaceText) - BigInt(cover.outputAndFeeText);
  if (change === 0n) return { ok: true, text: null, balance: cover };
  const text = change.toString();
  if (text === '100' || text === String(feeText) || outputs.some((item) => String(item) === text)) {
    return { ok: false, code: 'QUANTITY_COLLAPSED', balance: cover };
  }
  const exact = balanceCover(rows, [...outputs, text], feeText);
  return { ok: true, text, balance: { ...exact, balanced: exact.inputLovelaceText === exact.outputAndFeeText } };
}

function conwayFields(cborHex, fallbackCount) {
  const decoded = readConwayEnvelope(cborHex);
  if (decoded.ok !== true) {
    return {
      scriptDataHash: null,
      scriptDataHashSource: null,
      scriptDataHashInvented: false,
      redeemers: [],
      headOutputIndex: null,
      changeOutputIndex: null,
      builtHeadOutputIndex: null,
      builtPayoutOutputIndex: null,
      outputCount: fallbackCount,
      conwayDecoded: false,
    };
  }
  return {
    scriptDataHash: decoded.scriptDataHash,
    scriptDataHashSource: decoded.scriptDataHash ? 'conway-body-key-11' : null,
    scriptDataHashInvented: false,
    redeemers: decoded.redeemers,
    headOutputIndex: decoded.headOutputIndex,
    changeOutputIndex: decoded.changeOutputIndex,
    builtHeadOutputIndex: decoded.headOutputIndex,
    builtPayoutOutputIndex: decoded.payoutOutputIndex,
    outputCount: decoded.outputCount,
    conwayDecoded: true,
  };
}

export function scriptDataHashFromView(shown, envelopeHash = null) {
  if (typeof envelopeHash === 'string' && /^[0-9a-fA-F]{64}$/.test(envelopeHash)) {
    return envelopeHash.toLowerCase();
  }
  const matched = String(shown || '').match(/script\s+data\s+hash:\s*"?([0-9a-fA-F]{64})"?/i);
  return matched ? matched[1].toLowerCase() : null;
}

function formatAssets(assets) {
  if (!Array.isArray(assets) || assets.length === 0) return null;
  const parts = [];
  for (const asset of assets) {
    const quantity = String(asset?.quantity ?? '');
    const policyId = String(asset?.policyId || '');
    const assetName = String(asset?.assetName || '');
    if (!/^[1-9][0-9]*$/.test(quantity) || !HEX28.test(policyId) || !/^[0-9a-f]+$/.test(assetName)) return null;
    parts.push(`${quantity} ${policyId}.${assetName}`);
  }
  return parts.join('+');
}

function plutusScriptFile(path) {
  if (!path || !existsSync(path)) return null;
  try {
    const parsed = JSON.parse(readFileSync(path, 'utf8'));
    if (parsed?.type !== 'PlutusScriptV3' || !/^[0-9a-fA-F]+$/.test(String(parsed.cborHex || ''))) return null;
    return parsed;
  } catch {
    return null;
  }
}

function witnessProvenance(cliPath, requestedUnitOnly) {
  let real = false;
  try {
    real = Boolean(cliPath) && realpathSync(cliPath) === realpathSync(CARDANO_CLI);
  } catch {
    real = false;
  }
  const unitOnly = !real || requestedUnitOnly === true;
  return {
    unitOnly,
    forced: false,
    provenance: unitOnly ? 'unit-io' : 'cardano-cli-witness',
  };
}

export function encodeUnsignedBody(spec, cliPath = CARDANO_CLI) {
  const head = refOf(spec?.head);
  const fee = refOf(spec?.fee);
  const collateral = refOf(spec?.collateral);
  if (!head || !fee || !collateral) return closed('MALFORMED_DATA');
  if (new Set([head, fee, collateral]).size !== 3) return closed('INPUTS_NOT_DISJOINT');
  if (typeof spec?.outputAssets === 'string' && spec.outputAssets.includes('"')) return closed('QUOTED_ASSET_TEXT');
  if (typeof spec?.mintValue === 'string' && spec.mintValue.includes('"')) return closed('QUOTED_ASSET_TEXT');
  const earlySigner = earlyRequiredSigner(spec);
  if (earlySigner) return earlySigner;
  const magic = genesisMagicText(spec);
  if (!magic.ok) return closed(magic.code, { spawned: false, genesisMagicBound: false, submitBlocked: true });
  if (spec.outputLovelace == null || spec.feeLovelace == null) {
    return closed('FEE_UNMEASURED', { spawned: false, submitBlocked: true, protocolMinAda: 'unmeasured', genesisMagicBound: true });
  }
  const outputLovelace = String(spec.outputLovelace);
  const feeLovelace = String(spec.feeLovelace);
  if (!/^[1-9][0-9]*$/.test(outputLovelace) || !/^[1-9][0-9]*$/.test(feeLovelace)) return closed('MALFORMED_DATA');
  if (outputLovelace === '100' || feeLovelace === '100' || outputLovelace === feeLovelace) return closed('QUANTITY_COLLAPSED');
  const inputs = inputValueRows(spec);
  if (inputs.error) return closed(inputs.error, { spawned: false, submitBlocked: true });
  let changeText = null;
  let balance = inputs.rows ? balanceCover(inputs.rows, [outputLovelace], feeLovelace) : null;
  if (spec.requireChange === true) {
    if (!inputs.rows) return closed('CHANGE_UNMEASURED', { spawned: false, submitBlocked: true });
    const change = remainderLovelace(inputs.rows, [outputLovelace], feeLovelace);
    if (!change.ok) return closed(change.code, { spawned: false, submitBlocked: true, balance: change.balance });
    changeText = change.text;
    balance = change.balance;
  }
  if (balance && balance.balanced !== true) return closed('BALANCE_SHORT', { spawned: false, submitBlocked: true, balance });
  const assetText = spec.assets ? formatAssets(spec.assets) : null;
  if (spec.assets && !assetText) return closed('MALFORMED_ASSET');
  const mintText = spec.mintAssets ? formatAssets(spec.mintAssets) : (spec.mintValue ? String(spec.mintValue) : null);
  if (spec.mintAssets && !mintText) return closed('MALFORMED_ASSET');
  const signer = resolveSigner(spec);
  if (signer.error) return signer.error;
  const dir = mkdtempSync(join(spec.workRoot || tmpdir(), 'moriarty-body-'));
  try {
    let addressArgs;
    if (spec.outputScriptPath) {
      if (!plutusScriptFile(spec.outputScriptPath)) return closed('OUTPUT_SCRIPT_REJECTED');
      addressArgs = ['address', 'build', '--payment-script-file', spec.outputScriptPath, '--testnet-magic', magic.text];
    } else {
      writeFileSync(join(dir, 'pay.vkey'), JSON.stringify({
        type: 'PaymentVerificationKeyShelley_ed25519',
        description: 'Encoding fixture public key. Not a signing key and not a wallet.',
        cborHex: signer.vkeyHex,
      }));
      addressArgs = ['address', 'build', '--payment-verification-key-file', join(dir, 'pay.vkey'), '--testnet-magic', magic.text];
    }
    const address = run(cliPath, addressArgs);
    if (address.status !== 0) return closed('BODY_ENCODE_FAILED', { detail: address.stderr });
    const addr = String(address.stdout || '').trim();
    const bodyFile = join(dir, 'body.json');
    const txOut = assetText ? `${addr}+${outputLovelace}+${assetText}` : `${addr}+${outputLovelace}`;
    const args = [
      'conway', 'transaction', 'build-raw',
      '--tx-in', head,
      '--tx-in', fee,
      '--tx-in-collateral', collateral,
      '--tx-out', txOut,
    ];
    args.push(
      '--fee', feeLovelace,
      '--invalid-before', String(spec.invalidBefore ?? 40),
      '--invalid-hereafter', String(spec.invalidHereafter ?? 100),
      '--required-signer-hash', signer.hash,
    );
    if (mintText) {
      const real = isRealCli(cliPath);
      let script = spec.mintScriptPath || null;
      if (!script) {
        if (real) return closed('MINT_SCRIPT_ABSENT');
        script = join(dir, 'mint.marker');
        writeFileSync(script, 'unit-marker-not-a-compiled-cash-policy\n');
      } else if (real && !plutusScriptFile(script)) {
        return closed('MINT_SCRIPT_REJECTED');
      }
      args.push('--mint', mintText, '--mint-script-file', script, '--mint-redeemer-value', '{"int":0}');
      if (real || spec.offlineExecutionUnits) {
        const units = spec.offlineExecutionUnits;
        const memory = units?.memory;
        const steps = units?.steps;
        const suppliedPair = memory === 1 && steps === 1 && units?.measured !== true;
        if (suppliedPair) return closed('SUPPLIED_UNITS_NOT_A_MEASUREMENT', { submitBlocked: true, nativeBudgetMeasured: false });
        const haveUnits = Number.isInteger(memory) && Number.isInteger(steps) && memory >= 0 && steps >= 0;
        if (real && !haveUnits) return closed('OFFLINE_EXECUTION_UNITS_ABSENT');
        if (haveUnits) {
          if (real && (!spec.protocolParamsPath || !existsSync(spec.protocolParamsPath))) return closed('PROTOCOL_PARAMS_ABSENT');
          args.push('--mint-execution-units', `(${memory},${steps})`);
          if (spec.protocolParamsPath && existsSync(spec.protocolParamsPath)) args.push('--protocol-params-file', spec.protocolParamsPath);
        }
      }
    }
    if (Array.isArray(spec.extraTxIns)) {
      const seen = new Set([head, fee, collateral]);
      for (const ref of spec.extraTxIns) {
        const text = String(ref);
        if (seen.has(text)) return closed('INPUTS_NOT_DISJOINT');
        seen.add(text);
        args.push('--tx-in', text);
      }
    }
    if (spec.inlineDatumJson) {
      const datumFile = join(dir, 'inline-datum.json');
      writeFileSync(datumFile, String(spec.inlineDatumJson));
      args.push('--tx-out-inline-datum-file', datumFile);
    }
    if (changeText) args.push('--tx-out', `${addr}+${changeText}`);
    args.push('--out-file', bodyFile);
    const built = run(cliPath, args);
    if (built.status !== 0) return closed('BODY_ENCODE_FAILED', { detail: built.stderr });
    const body = JSON.parse(readFileSync(bodyFile, 'utf8'));
    const txid = run(cliPath, ['conway', 'transaction', 'txid', '--tx-body-file', bodyFile, '--output-text']);
    if (txid.status !== 0 || !HEX32.test(String(txid.stdout || '').trim())) return closed('BODY_ENCODE_FAILED', { detail: txid.stderr });
    const view = run(cliPath, ['debug', 'transaction', 'view', '--tx-body-file', bodyFile]);
    if (view.status !== 0) return closed('BODY_VIEW_FAILED', { detail: view.stderr });
    const shown = view.stdout || '';
    if (!shown.includes(collateral) || !shown.includes(head) || !shown.includes(fee)) return closed('BODY_VIEW_FAILED');
    if (isRealCli(cliPath)) {
      const named = [...(spec.assets || []), ...(spec.mintAssets || [])];
      if (named.some((asset) => !shown.includes(asset.policyId) || !shown.includes(asset.assetName))) {
        return closed('BODY_VIEW_FAILED', { detail: 'asset-not-in-view' });
      }
      if (spec.inlineDatumJson && !shown.includes('constructor')) return closed('BODY_VIEW_FAILED', { detail: 'datum-not-in-view' });
      if (!shown.toLowerCase().includes(signer.hash)) return closed('BODY_VIEW_FAILED', { detail: 'signer-not-in-view' });
    }
    if (spec.outFile) writeFileSync(spec.outFile, JSON.stringify(body));
    const paramsSha = spec.protocolParamsPath && existsSync(spec.protocolParamsPath)
      ? createHash('sha256').update(readFileSync(spec.protocolParamsPath)).digest('hex')
      : null;
    return {
      ok: true,
      code: 'UNSIGNED_BODY',
      txid: String(txid.stdout).trim(),
      cborHex: body.cborHex,
      cborSha256: createHash('sha256').update(body.cborHex).digest('hex'),
      bodyFile: spec.outFile || null,
      type: body.type,
      outputAddress: addr,
      paymentKeyHash: signer.hash,
      seedOwner: spec.outputScriptPath ? null : signer.hash,
      holder: spec.outputScriptPath ? null : signer.hash,
      outputPaymentHash: spec.outputScriptPath ? null : signer.hash,
      signatories: [signer.hash],
      nativeBudgetMeasured: false,
      offlineExecutionUnits: spec.offlineExecutionUnits || null,
      offlineProtocolParamsSha256: paramsSha,
      ledgerExecution: false,
      submitted: false,
      confirmed: false,
      signed: false,
      nativeAcceptance: false,
      collateralInLedgerBody: true,
      scriptContextHasCollateralField: false,
      quantities: {
        outputLovelace,
        feeLovelace,
        kernelCash: '100',
        protocolMinAda: 'unmeasured',
        mintValue: mintText,
      },
      genesisMagicBound: true,
      networkMagic: Number(magic.text),
      ...conwayFields(body.cborHex, args.filter((item) => item === '--tx-out').length),
      changeLovelace: changeText,
      inputValues: inputs.rows,
      balance,
      encodingFixture: signer.encodingFixture,
      runtimePinsCreated: false,
      ordinaryInputs: [head, fee, ...(Array.isArray(spec.extraTxIns) ? spec.extraTxIns.map((ref) => String(ref)) : [])],
      scriptRef: spec.scriptRef ? String(spec.scriptRef) : null,
      mintPolicyCount: mintText ? 1 : 0,
      collateralRef: collateral,
      ordinaryKeyHashes: Array.isArray(spec.ordinaryKeyHashes) ? spec.ordinaryKeyHashes : [],
      collateralKeyHashes: Array.isArray(spec.collateralKeyHashes) ? spec.collateralKeyHashes : [],
    };
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}

export function bindWitness(request = {}, cliPath = CARDANO_CLI) {
  if (!request.signingKeyPath) return closed('WITNESS_KEYS_ABSENT', { constructorReached: false });
  if (!existsSync(request.signingKeyPath)) return closed('WITNESS_KEYS_ABSENT', { constructorReached: false });
  const st = statSync(request.signingKeyPath);
  if (!st.isFile() || (st.mode & 0o777) !== 0o600) {
    return closed('KEY_MODE_REFUSED', {
      constructorReached: false,
      mode: (st.mode & 0o777).toString(8).padStart(4, '0'),
    });
  }
  if (typeof process.getuid === 'function' && st.uid !== process.getuid()) {
    return closed('KEY_NOT_OWNED', { constructorReached: false });
  }
  if (!request.txBodyCborHex && !request.txBodyFile) return closed('WITNESS_BODY_ABSENT', { constructorReached: false });
  if (request.cborSha256) {
    const got = createHash('sha256').update(String(request.txBodyCborHex || '')).digest('hex');
    if (!request.txBodyCborHex || got !== request.cborSha256) return closed('WITNESS_BODY_MISMATCH', { constructorReached: false });
  }
  const magic = genesisMagicText(request);
  if (!magic.ok) return closed(magic.code, { constructorReached: false, genesisMagicBound: false, spawned: false });
  const placeholderBody = String(request.txBodyFile || '').endsWith('unit-body.json');
  const placeholderKey = String(request.signingKeyPath || '').endsWith('unit-payment.skey');
  if (placeholderBody || placeholderKey) return closed('WITNESS_PLACEHOLDER_REFUSED', { constructorReached: false });
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-witness-'));
  try {
    const bodyFile = request.txBodyFile || join(dir, 'tx.body.json');
    if (!request.txBodyFile) {
      writeFileSync(bodyFile, JSON.stringify({
        type: request.txBodyType || 'Tx ConwayEra',
        description: 'Unsigned body for witness construction',
        cborHex: String(request.txBodyCborHex),
      }));
    }
    const outFile = join(dir, 'witness.json');
    const result = run(cliPath, [
      'conway', 'transaction', 'witness',
      '--tx-body-file', bodyFile,
      '--signing-key-file', request.signingKeyPath,
      '--testnet-magic', magic.text,
      '--out-file', outFile,
    ]);
    const provenance = witnessProvenance(cliPath, request.unitOnly);
    if (result.status !== 0 || !existsSync(outFile)) {
      return closed('WITNESS_CONSTRUCTOR_REJECTED', {
        constructorReached: true,
        cliStatus: result.status,
        signed: false,
        ...provenance,
      });
    }
    const signedIsProof = false;
    const witnessBody = JSON.parse(readFileSync(outFile, 'utf8'));
    const witnessCborHex = typeof witnessBody?.cborHex === 'string' ? witnessBody.cborHex : null;
    return {
      ok: true,
      code: provenance.unitOnly ? 'WITNESS_UNIT_ONLY' : 'WITNESS_CONSTRUCTED',
      constructorReached: true,
      cliStatus: 0,
      signed: provenance.unitOnly ? false : true,
      signedIsProof,
      witnessCborHex,
      ledgerExecution: false,
      submitted: false,
      confirmed: false,
      nativeAcceptance: false,
      ...provenance,
      scriptContextHasCollateralField: false,
    };
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}

export function inspectTransportEnvelope({ cborHex, expectedTxId, cliPath = CARDANO_CLI }) {
  if (!/^[0-9a-fA-F]+$/.test(String(cborHex || '')) || String(cborHex).length < 20) return closed('MALFORMED_DATA');
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-transport-'));
  try {
    const file = join(dir, 'tx.json');
    writeFileSync(file, JSON.stringify({
      type: 'Tx ConwayEra',
      description: 'Historical transport fixture. Not a financial-head proof.',
      cborHex: String(cborHex).trim(),
    }));
    const txid = run(cliPath, ['conway', 'transaction', 'txid', '--tx-file', file, '--output-text']);
    if (txid.status !== 0) return closed('TRANSPORT_TXID_FAILED', { detail: txid.stderr });
    const got = String(txid.stdout || '').trim();
    if (expectedTxId && got !== expectedTxId) return closed('TRANSPORT_TXID_MISMATCH', { txid: got, expectedTxId });
    return {
      ok: true,
      code: 'TRANSPORT_BINDING',
      txid: got,
      ledgerExecution: false,
      submitted: false,
      confirmed: false,
      nativeAcceptance: false,
      layer: 'transport-binding',
    };
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}

function writeEncodingVkey(dir, name, hex) {
  const path = join(dir, name);
  writeFileSync(path, JSON.stringify({
    type: 'PaymentVerificationKeyShelley_ed25519',
    description: 'Encoding fixture public key. Not a signing key and not a wallet.',
    cborHex: hex,
  }));
  return path;
}

export function encodeSpendBody(spec, cliPath = CARDANO_CLI) {
  const head = refOf(spec?.head);
  const fee = refOf(spec?.fee);
  const collateral = refOf(spec?.collateral);
  if (!head || !fee || !collateral) return closed('MALFORMED_DATA');
  if (new Set([head, fee, collateral]).size !== 3) return closed('INPUTS_NOT_DISJOINT');
  if (!plutusScriptFile(spec.spendScriptPath)) return closed('SPEND_SCRIPT_REJECTED');
  if (!spec.redeemerJson || typeof spec.redeemerJson !== 'string') return closed('SPEND_REDEEMER_ABSENT');
  const outputs = Array.isArray(spec.outputs) ? spec.outputs : [];
  if (outputs.length !== 2) return closed('SPEND_OUTPUTS_ABSENT');
  const continuing = outputs[0];
  const payout = outputs[1];
  const continuingAssets = formatAssets(continuing.assets || []);
  const payoutAssets = formatAssets(payout.assets || []);
  if (!continuingAssets || !payoutAssets) return closed('MALFORMED_ASSET');
  if (JSON.stringify(continuing.assets) === JSON.stringify(payout.assets)) return closed('CASH_NOT_CONSERVED');
  const magic = genesisMagicText(spec);
  if (!magic.ok) return closed(magic.code, { spawned: false, genesisMagicBound: false, submitBlocked: true });
  const signer = resolveSigner(spec);
  if (signer.error) return signer.error;
  const payoutHex = spec.payoutVkeyHex ? String(spec.payoutVkeyHex).trim().toLowerCase() : '';
  if (!payoutHex) return closed('PAYOUT_VKEY_ABSENT', { spawned: false, encodingFixture: false });
  if (payoutHex === ENCODING_VKEY_HEX || repeatedFixtureMaterial(payoutHex)) {
    return closed('ENCODING_FIXTURE_NOT_A_ROLE', { spawned: false, encodingFixture: true });
  }
  const payoutHash = paymentKeyHashOfVkeyCbor(payoutHex);
  if (!payoutHash || payoutHash === signer.hash) return closed('PAYOUT_KEY_COLLAPSED', { spawned: false });
  const dir = mkdtempSync(join(spec.workRoot || tmpdir(), 'moriarty-spend-'));
  try {
    const real = isRealCli(cliPath);
    const units = spec.offlineExecutionUnits;
    const suppliedPair = units?.memory === 1 && units?.steps === 1 && units?.measured !== true;
    if (suppliedPair) return closed('SUPPLIED_UNITS_NOT_A_MEASUREMENT', { submitBlocked: true, nativeBudgetMeasured: false });
    const haveUnits = Number.isInteger(units?.memory) && Number.isInteger(units?.steps) && units.memory >= 0 && units.steps >= 0;
    if (real && !haveUnits) return closed('OFFLINE_EXECUTION_UNITS_ABSENT');
    if (real && haveUnits && (!spec.protocolParamsPath || !existsSync(spec.protocolParamsPath))) return closed('PROTOCOL_PARAMS_ABSENT');
    const spendInputs = inputValueRows(spec);
    if (spendInputs.error) return closed(spendInputs.error, { spawned: false, submitBlocked: true });
    const spendOutputs = [continuing.lovelace, payout.lovelace].map((value) => canonicalDigit(value)).filter((value) => value != null);
    const feeText = canonicalDigit(spec.feeLovelace);
    let changeText = null;
    let spendBalance = spendInputs.rows && feeText && spendOutputs.length === 2
      ? balanceCover(spendInputs.rows, spendOutputs, feeText)
      : null;
    if (spec.requireChange === true) {
      if (!spendInputs.rows || !feeText || spendOutputs.length !== 2) {
        return closed('CHANGE_UNMEASURED', { spawned: false, submitBlocked: true });
      }
      const change = remainderLovelace(spendInputs.rows, spendOutputs, feeText);
      if (!change.ok) return closed(change.code, { spawned: false, submitBlocked: true, balance: change.balance });
      changeText = change.text;
      spendBalance = change.balance;
    }
    if (spendBalance && spendBalance.balanced !== true) return closed('BALANCE_SHORT', { spawned: false, submitBlocked: true, balance: spendBalance });
    const scriptAddress = run(cliPath, [
      'address', 'build', '--payment-script-file', spec.spendScriptPath, '--testnet-magic', magic.text,
    ]);
    if (scriptAddress.status !== 0) return closed('BODY_ENCODE_FAILED', { detail: scriptAddress.stderr });
    const payoutKey = writeEncodingVkey(dir, 'payout.vkey', payoutHex);
    const payoutAddress = run(cliPath, [
      'address', 'build', '--payment-verification-key-file', payoutKey, '--testnet-magic', magic.text,
    ]);
    if (payoutAddress.status !== 0) return closed('BODY_ENCODE_FAILED', { detail: payoutAddress.stderr });
    const headAddr = String(scriptAddress.stdout || '').trim();
    const payAddr = String(payoutAddress.stdout || '').trim();
    if (!headAddr || !payAddr || headAddr === payAddr) return closed('PAYOUT_ADDRESS_COLLAPSED');
    let changeAddr = null;
    if (changeText) {
      const changeKey = writeEncodingVkey(dir, 'change.vkey', signer.vkeyHex);
      const changeAddress = run(cliPath, [
        'address', 'build', '--payment-verification-key-file', changeKey, '--testnet-magic', magic.text,
      ]);
      changeAddr = String(changeAddress.stdout || '').trim();
      if (changeAddress.status !== 0 || !changeAddr) return closed('BODY_ENCODE_FAILED', { detail: changeAddress.stderr });
    }
    const datumFile = join(dir, 'post-datum.json');
    writeFileSync(datumFile, String(continuing.inlineDatumJson || ''));
    const redeemerFile = join(dir, 'redeemer.json');
    writeFileSync(redeemerFile, spec.redeemerJson);
    const bodyFile = join(dir, 'body.json');
    const args = [
      'conway', 'transaction', 'build-raw',
      '--tx-in', head,
      '--tx-in-script-file', spec.spendScriptPath,
      '--tx-in-redeemer-file', redeemerFile,
    ];
    if (haveUnits) {
      args.push('--tx-in-execution-units', `(${units.memory},${units.steps})`);
      if (spec.protocolParamsPath && existsSync(spec.protocolParamsPath)) args.push('--protocol-params-file', spec.protocolParamsPath);
    }
    args.push(
      '--tx-in', fee,
      '--tx-in-collateral', collateral,
      '--tx-out', `${headAddr}+${continuing.lovelace}+${continuingAssets}`,
      '--tx-out-inline-datum-file', datumFile,
      '--tx-out', `${payAddr}+${payout.lovelace}+${payoutAssets}`,
    );
    if (changeText) args.push('--tx-out', `${changeAddr}+${changeText}`);
    args.push(
      '--fee', String(spec.feeLovelace),
      '--invalid-before', String(spec.invalidBefore),
      '--invalid-hereafter', String(spec.invalidHereafter),
      '--required-signer-hash', signer.hash,
      '--out-file', bodyFile,
    );
    const built = run(cliPath, args);
    if (built.status !== 0) return closed('BODY_ENCODE_FAILED', { detail: built.stderr });
    const body = JSON.parse(readFileSync(bodyFile, 'utf8'));
    if (spec.outFile) writeFileSync(spec.outFile, `${JSON.stringify(body)}\n`);
    const txid = run(cliPath, ['conway', 'transaction', 'txid', '--tx-body-file', bodyFile, '--output-text']);
    if (txid.status !== 0 || !HEX32.test(String(txid.stdout || '').trim())) return closed('BODY_ENCODE_FAILED', { detail: txid.stderr });
    const view = run(cliPath, ['debug', 'transaction', 'view', '--tx-body-file', bodyFile]);
    if (view.status !== 0) return closed('BODY_VIEW_FAILED', { detail: view.stderr });
    const shown = view.stdout || '';
    if (!shown.includes(head) || !shown.includes(fee) || !shown.includes(collateral)) return closed('BODY_VIEW_FAILED');
    if (real) {
      const named = [...(continuing.assets || []), ...(payout.assets || [])];
      if (named.some((asset) => !shown.includes(asset.policyId) || !shown.includes(asset.assetName))) {
        return closed('BODY_VIEW_FAILED', { detail: 'asset-not-in-view' });
      }
      if (!shown.includes('constructor')) return closed('BODY_VIEW_FAILED', { detail: 'datum-not-in-view' });
      if (!shown.toLowerCase().includes(signer.hash)) return closed('BODY_VIEW_FAILED', { detail: 'signer-not-in-view' });
    }
    const paramsSha = spec.protocolParamsPath && existsSync(spec.protocolParamsPath)
      ? createHash('sha256').update(readFileSync(spec.protocolParamsPath)).digest('hex')
      : null;
    return {
      ok: true,
      code: 'UNSIGNED_BODY',
      txid: String(txid.stdout).trim(),
      cborHex: body.cborHex,
      cborSha256: createHash('sha256').update(body.cborHex).digest('hex'),
      type: body.type,
      bodyFile: spec.outFile || null,
      bodyFileSha256: spec.outFile ? createHash('sha256').update(readFileSync(spec.outFile)).digest('hex') : null,
      outputAddress: headAddr,
      payoutAddress: payAddr,
      paymentKeyHash: signer.hash,
      signatories: [signer.hash],
      payoutPaymentHash: payoutHash,
      nativeBudgetMeasured: false,
      offlineExecutionUnits: spec.offlineExecutionUnits || null,
      offlineProtocolParamsSha256: paramsSha,
      ledgerExecution: false,
      submitted: false,
      confirmed: false,
      signed: false,
      signedIsProof: false,
      nativeAcceptance: false,
      kind: spec.kind || null,
      ...conwayFields(body.cborHex, args.filter((item) => item === '--tx-out').length),
      changeLovelace: changeText,
      inputValues: spendInputs.rows,
      balance: spendBalance,
      ordinaryInputs: [head, fee],
      scriptRef: head,
      mintPolicyCount: 0,
      collateralRef: collateral,
      collateralInLedgerBody: true,
      ordinaryKeyHashes: Array.isArray(spec.ordinaryKeyHashes) ? spec.ordinaryKeyHashes : [],
      collateralKeyHashes: Array.isArray(spec.collateralKeyHashes) ? spec.collateralKeyHashes : [],
      invalidBefore: Number(spec.invalidBefore),
      invalidHereafter: Number(spec.invalidHereafter),
      quantities: {
        feeLovelace: String(spec.feeLovelace),
        kernelCash: '100',
        protocolMinAda: 'unmeasured',
        continuingCash: '0',
        payoutCash: '100',
        headQuantity: '1',
      },
    };
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}
