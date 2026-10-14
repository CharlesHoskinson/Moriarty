/**
 * Evaluate the applied financial_head program with fresh ScriptContexts.
 * Evaluator cpu and memory are not a native budget and not ledger phase-2.
 */
import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { AIKEN, CASH_VALIDATOR } from './apply-head.mjs';
import { createIsolatedCustodyProject } from './compile-custody.mjs';

const CASES = ['MINT_CTX', 'SPEND_CTX', 'WITHDRAW_CTX', 'FORGED_MINT_CTX', 'FORGED_SPEND_CTX', 'FOREIGN_MISMATCH_CTX', 'FOREIGN_MATCH_CTX'];
const PROTOTYPE_POLICY = '9155cf594ceded9188f595f91374d3849f3f39feb8d063958174e6a1';

function run(cmd, args, cwd) {
  return spawnSync(cmd, args, { cwd, encoding: 'utf8', maxBuffer: 64 * 1024 * 1024 });
}

function extractMarked(text) {
  const clean = String(text || '').replace(/\u001b\[[0-9;]*m/g, '');
  const found = {};
  for (let i = 0; i < CASES.length; i += 1) {
    const start = clean.indexOf(CASES[i]);
    if (start < 0) continue;
    const next = CASES.slice(i + 1).map((name) => clean.indexOf(name, start + CASES[i].length)).find((index) => index >= 0);
    const slice = clean.slice(start, next === undefined ? clean.length : next);
    const hexes = slice.match(/[0-9a-fA-F]{64,}/g) || [];
    hexes.sort((a, b) => b.length - a.length);
    if (hexes[0]) found[CASES[i]] = hexes[0].toLowerCase();
  }
  return found;
}

function outerEntry(policy, cash) {
  return `use aiken/cbor.{serialise}
use aiken/primitive/bytearray
use cardano/address.{VerificationKey}
use cardano/script_context.{Minting, ScriptContext, Spending, Withdrawing}
use financial_head.{
  fixture_lender, positive_mint_tx, positive_release, forged_mint_tx, forged_spend,
  foreign_spend_match, foreign_spend_mismatch,
}

fn as_data(value: Data) -> Data { value }

const policy: ByteArray = #"${policy}"
const cash: ByteArray = #"${cash}"

test dump_outer_contexts() fail {
  let mint_tx_pos = positive_mint_tx(policy, cash)
  trace @"MINT_CTX"
  trace bytearray.to_hex(serialise(ScriptContext { transaction: mint_tx_pos, redeemer: 0, info: Minting(policy) }))
  let (head, redeemer, own, spend) = positive_release(policy, cash)
  trace @"SPEND_CTX"
  trace bytearray.to_hex(serialise(ScriptContext { transaction: spend, redeemer: as_data(redeemer), info: Spending { output: own, datum: Some(as_data(head)) } }))
  trace @"WITHDRAW_CTX"
  trace bytearray.to_hex(serialise(ScriptContext { transaction: mint_tx_pos, redeemer: 0, info: Withdrawing(VerificationKey(fixture_lender())) }))
  trace @"FORGED_MINT_CTX"
  trace bytearray.to_hex(serialise(ScriptContext { transaction: forged_mint_tx(policy, cash), redeemer: 0, info: Minting(policy) }))
  let (forged_head, forged_redeemer, forged_own, forged_tx) = forged_spend(policy, cash)
  trace @"FORGED_SPEND_CTX"
  trace bytearray.to_hex(serialise(ScriptContext { transaction: forged_tx, redeemer: as_data(forged_redeemer), info: Spending { output: forged_own, datum: Some(as_data(forged_head)) } }))
  let (mismatch_head, mismatch_redeemer, mismatch_own, mismatch_spend) = foreign_spend_mismatch(policy, cash)
  trace @"FOREIGN_MISMATCH_CTX"
  trace bytearray.to_hex(serialise(ScriptContext { transaction: mismatch_spend, redeemer: as_data(mismatch_redeemer), info: Spending { output: mismatch_own, datum: Some(as_data(mismatch_head)) } }))
  let (match_head, match_redeemer, match_own, match_spend) = foreign_spend_match(policy, cash)
  trace @"FOREIGN_MATCH_CTX"
  trace bytearray.to_hex(serialise(ScriptContext { transaction: match_spend, redeemer: as_data(match_redeemer), info: Spending { output: match_own, datum: Some(as_data(match_head)) } }))
  False
}
`;
}

function readCbor(buf) {
  let i = 0;
  function readInfo(info) {
    if (info < 24) return info;
    if (info === 24) return buf[i++];
    if (info === 25) { const n = buf.readUInt16BE(i); i += 2; return n; }
    if (info === 26) { const n = buf.readUInt32BE(i); i += 4; return n; }
    if (info === 27) {
      const hi = buf.readUInt32BE(i);
      const lo = buf.readUInt32BE(i + 4);
      i += 8;
      if (hi !== 0) throw new Error('uint64');
      return lo;
    }
    if (info === 31) return null;
    throw new Error(`cbor info ${info}`);
  }
  function read() {
    const b = buf[i++];
    if (b === 0xff) return { type: 'break' };
    const major = b >> 5;
    const info = b & 31;
    if (major === 0) return { type: 'int', n: BigInt(readInfo(info)) };
    if (major === 1) return { type: 'int', n: -1n - BigInt(readInfo(info)) };
    if (major === 2) {
      const len = readInfo(info);
      if (len === null) {
        const parts = [];
        while (buf[i] !== 0xff) parts.push(read().hex);
        i += 1;
        return { type: 'bytes', hex: parts.join('') };
      }
      const hex = buf.subarray(i, i + len).toString('hex');
      i += len;
      return { type: 'bytes', hex };
    }
    if (major === 4) {
      const len = readInfo(info);
      const items = [];
      if (len === null) {
        while (buf[i] !== 0xff) items.push(read());
        i += 1;
      } else {
        for (let n = 0; n < len; n += 1) items.push(read());
      }
      return { type: 'list', items };
    }
    if (major === 5) {
      const len = readInfo(info);
      const pairs = [];
      if (len === null) {
        while (buf[i] !== 0xff) pairs.push([read(), read()]);
        i += 1;
      } else {
        for (let n = 0; n < len; n += 1) pairs.push([read(), read()]);
      }
      return { type: 'map', pairs };
    }
    if (major === 6) {
      const tag = readInfo(info);
      const content = read();
      if (content.type !== 'list') throw new Error(`tag ${tag} content ${content.type}`);
      let index;
      if (tag >= 121 && tag <= 127) index = tag - 121;
      else if (tag >= 1280 && tag <= 1400) index = tag - 1280 + 7;
      else throw new Error(`tag ${tag}`);
      return { type: 'constr', index, fields: content.items };
    }
    throw new Error(`cbor major ${major}`);
  }
  const node = read();
  if (i !== buf.length) throw new Error(`unconsumed cbor ${buf.length - i}`);
  return node;
}

function emitData(node) {
  if (node.type === 'int') return `I ${node.n.toString()}`;
  if (node.type === 'bytes') return `B #${node.hex}`;
  if (node.type === 'list') return `List [${node.items.map(emitData).join(', ')}]`;
  if (node.type === 'map') return `Map [${node.pairs.map(([key, value]) => `(${emitData(key)}, ${emitData(value)})`).join(', ')}]`;
  if (node.type === 'constr') return `Constr ${node.index} [${node.fields.map(emitData).join(', ')}]`;
  throw new Error(node.type);
}

function dataTerm(hex) {
  return `(con data (${emitData(readCbor(Buffer.from(hex, 'hex')))}))`;
}

function parseEval(stdout, stderr) {
  try {
    const parsed = JSON.parse(stdout);
    return { result: parsed.result ?? null, cpu: parsed.cpu ?? null, mem: parsed.mem ?? null };
  } catch {
    const costs = String(stderr || '').match(/cpu:\s*(\d+)[\s\S]*?memory:\s*(\d+)/);
    if (costs) return { result: null, cpu: Number(costs[1]), mem: Number(costs[2]) };
    return { result: null, cpu: null, mem: null };
  }
}

export function evaluateOuter({ custodySourcePath, appliedBlueprintPath, outDir, aiken = AIKEN }) {
  mkdirSync(outDir, { recursive: true });
  const applied = JSON.parse(readFileSync(appliedBlueprintPath, 'utf8'));
  const mint = applied.validators.find((item) => item.title.includes('financial_head') && item.title.endsWith('.mint'));
  const cash = applied.validators.find((item) => item.title.includes('cash') && item.title.endsWith('.mint'));
  if (!mint?.compiledCode || !cash?.hash) {
    return { ok: false, code: 'COMPILED_ARTIFACT_ABSENT', ledgerExecution: false, phase2: false, nativeBudgetMeasured: false };
  }
  const policy = mint.hash;
  const cashHash = cash.hash;
  if (!/^[0-9a-f]{56}$/.test(policy) || !/^[0-9a-f]{56}$/.test(cashHash)) {
    return { ok: false, code: 'MALFORMED_ID', ledgerExecution: false, phase2: false };
  }
  const isolated = createIsolatedCustodyProject();
  const summary = {
    ok: false,
    code: 'OUTER_EVAL',
    appliedHash: policy,
    cashHash,
    prototypePolicyReused: policy === PROTOTYPE_POLICY,
    ledgerExecution: false,
    phase2: false,
    nativeBudgetMeasured: false,
    evaluator: true,
    scriptForm: 'hex-text-compiledCode',
    argumentForm: 'con-data-constr',
    cases: {},
  };
  try {
    writeFileSync(join(isolated, 'validators/financial_head.ak'), readFileSync(custodySourcePath));
    writeFileSync(join(isolated, 'validators/cash.ak'), CASH_VALIDATOR);
    writeFileSync(join(isolated, 'validators/outer_entry.ak'), outerEntry(policy, cashHash));
    writeFileSync(join(outDir, 'applied.cbor'), Buffer.from(mint.compiledCode, 'hex'));
    writeFileSync(join(outDir, 'applied.hex'), mint.compiledCode);
    const check = run(aiken, ['check'], isolated);
    writeFileSync(join(outDir, 'aiken-check.stdout'), check.stdout || '');
    writeFileSync(join(outDir, 'aiken-check.stderr'), check.stderr || '');
    const marked = { ...extractMarked(check.stdout), ...extractMarked(check.stderr) };
    try {
      Object.assign(marked, extractMarked(JSON.stringify(JSON.parse(check.stdout))));
    } catch { /* stdout is not JSON */ }
    for (const name of CASES) {
      const dataHex = marked[name];
      if (!dataHex) {
        summary.cases[name] = { ok: false, code: 'OUTER_TRACE_ABSENT', evaluatorCpu: null, evaluatorMem: null };
        continue;
      }
      writeFileSync(join(outDir, `${name}.hex`), `${dataHex}\n`);
      let term;
      try {
        term = dataTerm(dataHex);
      } catch (error) {
        summary.cases[name] = { ok: false, code: 'DATA_TERM_REJECTED', detail: error.message, evaluatorCpu: null, evaluatorMem: null, ledgerExecution: false, phase2: false, nativeBudget: false };
        continue;
      }
      writeFileSync(join(outDir, `${name}.term`), `${term}\n`);
      // This Aiken accepts -c only for hex text. A binary file is a UTF-8 error. (con data #hex) is a parse error.
      const evaluated = run(aiken, ['uplc', 'eval', '-c', join(outDir, 'applied.hex'), term]);
      writeFileSync(join(outDir, `eval-${name}.stdout`), evaluated.stdout || '');
      writeFileSync(join(outDir, `eval-${name}.stderr`), evaluated.stderr || '');
      const parsed = parseEval(evaluated.stdout || '', evaluated.stderr || '');
      const unit = typeof parsed.result === 'string' && parsed.result.includes('(con unit ())');
      summary.cases[name] = {
        ok: evaluated.status === 0,
        cliStatus: evaluated.status,
        unit,
        result: parsed.result,
        evaluatorCpu: parsed.cpu,
        evaluatorMem: parsed.mem,
        evaluatorCrashed: evaluated.status !== 0 && typeof parsed.cpu === 'number',
        inheritedVmLimit: name === 'FOREIGN_MATCH_CTX',
        ledgerExecution: false,
        phase2: false,
        nativeBudget: false,
      };
    }
    summary.traceCount = Object.values(summary.cases).filter((item) => item.code !== 'OUTER_TRACE_ABSENT').length;
    summary.evaluatedCount = Object.values(summary.cases).filter((item) => typeof item.evaluatorCpu === 'number').length;
    summary.ok = summary.traceCount === CASES.length && summary.evaluatedCount === CASES.length && check.status === 0;
    summary.checkStatus = check.status;
    writeFileSync(join(outDir, 'outer-summary.json'), JSON.stringify(summary, null, 2) + '\n');
    return summary;
  } finally {
    rmSync(isolated, { recursive: true, force: true });
  }
}
