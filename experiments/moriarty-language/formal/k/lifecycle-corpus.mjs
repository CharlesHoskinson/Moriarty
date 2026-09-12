import { readFileSync, existsSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { fileURLToPath, pathToFileURL } from 'node:url';
import path from 'node:path';
import { createFinancialAgreementSourceV5 } from '../../src/successor/financial-agreement-source-v5.ts';
import { createFinancialExpressionContractV4, FINANCIAL_EXPRESSION_CONTRACT_V4 } from '../../src/successor/financial-expression-v1.ts';
import { canonical } from '../../src/successor/expression-wire-v1.ts';

const HERE = fileURLToPath(new URL('.', import.meta.url));
const SOURCE_PATH = path.resolve(HERE, '../../spec/successor/examples/loan-lifecycle.mori');
const SOURCE_SHA = 'a0efd5166fdf27106110c69548d3b3065070e8d759d231296c454d19757bbaaa';
const MAIN_EVIDENCE = path.resolve(HERE, '../../../../../../deliverables/language-to-ledger-2026-09-12');
const WORKTREE_EVIDENCE = path.resolve(HERE, '../../../../deliverables/language-to-ledger-2026-09-12');

function evidenceFile(...parts) {
  for (const root of [MAIN_EVIDENCE, WORKTREE_EVIDENCE]) {
    const candidate = path.join(root, ...parts);
    if (existsSync(candidate)) return candidate;
  }
  throw new Error('missing evidence ' + parts.join('/'));
}

const WORK = JSON.parse(readFileSync(evidenceFile('lifecycle', 'root-work-expectations.json'), 'utf8'));
const TEMPLATES = JSON.parse(readFileSync(evidenceFile('design-review', 'lifecycle-source-expectations.json'), 'utf8'));
const CASES = existsSync(path.join(MAIN_EVIDENCE, 'design-review', 'lifecycle-k-expectations.json'))
  ? JSON.parse(readFileSync(path.join(MAIN_EVIDENCE, 'design-review', 'lifecycle-k-expectations.json'), 'utf8'))
  : { independentCases: [] };

const ARGS = {
  originate: { transferId: 'D1', originationId: 'O1' },
  accrue: { accrualId: 'A1', observedTime: '1060', periodIndex: '1' },
  repay: { allocationId: 'R1', nominal: '30', transferId: 'P1' },
  settle: { allocationId: 'R2', transferId: 'P2' },
};

function copy(value) {
  return JSON.parse(JSON.stringify(value));
}

function independentStage(action) {
  const stage = TEMPLATES.stages.find((item) => item.action === action);
  const expected = copy(stage.expectedResultTemplate);
  expected.financialPost.work = {
    remaining: WORK.actions[action].remaining,
    spent: WORK.actions[action].spent,
    closureReserve: WORK.seed.closureReserve,
  };
  expected.workRemaining = WORK.actions[action].remaining;
  return expected;
}

export function compareLifecycleResult(actual, expected) {
  const differences = [];
  const walk = (left, right, route) => {
    if (Array.isArray(left) || Array.isArray(right)) {
      if (!Array.isArray(left) || !Array.isArray(right) || left.length !== right.length) {
        differences.push(route);
        return;
      }
      left.forEach((item, index) => walk(item, right[index], route + '/' + index));
      return;
    }
    if (left && right && typeof left === 'object' && typeof right === 'object') {
      const keys = new Set([...Object.keys(left), ...Object.keys(right)]);
      for (const key of keys) walk(left[key], right[key], route + '/' + key);
      return;
    }
    if (left !== right) differences.push(route);
  };
  walk(actual, expected, '');
  return { ok: differences.length === 0, differences };
}

export async function runLifecycleCorpus(options = {}) {
  const mode = options.mode === 'offline' || options.kAdmitted !== true ? 'offline' : 'k';
  const source = readFileSync(SOURCE_PATH, 'utf8');
  const sourceSHA256 = createHash('sha256').update(source).digest('hex');
  if (sourceSHA256 !== SOURCE_SHA) {
    throw new Error('Source changed: independently recount work before using this oracle.');
  }
  const language = createFinancialAgreementSourceV5();
  const elaborated = language.elaborate(source);
  const checked = language.check(source);
  if (elaborated.judgmentResult !== 'SourceElaborated') {
    return { sourceSHA256, wholeSourceChecked: false, kExecuted: false, kStatus: 'not-executed', elaborated, checked };
  }
  if (checked.judgmentResult !== 'SourceChecked') {
    return { sourceSHA256, wholeSourceChecked: false, kExecuted: false, kStatus: 'not-executed', elaborated, checked };
  }
  const actions = elaborated.actions.map((item) => item.action);
  let ordinary = copy(TEMPLATES.seed.ordinary);
  let financial = copy(TEMPLATES.seed.financial);
  financial.work.remaining = WORK.seed.remaining;
  financial.work.spent = WORK.seed.spent;
  financial.work.closureReserve = WORK.seed.closureReserve;
  const sourceResults = [];
  const coreResults = [];
  const packets = [];
  const independent = {};
  for (const action of ['originate', 'accrue', 'repay', 'settle']) {
    const compiled = elaborated.actions.find((item) => item.action === action);
    const snapshot = { Args: ARGS[action], Obs: {}, Pre: ordinary, workInitial: financial.work.remaining };
    const snapshotJSON = canonical(snapshot);
    const financialJSON = JSON.stringify(financial);
    const sourceResult = language.evaluate(source, action, snapshotJSON, financialJSON);
    const request = canonical({
      contract: FINANCIAL_EXPRESSION_CONTRACT_V4,
      source,
      core: compiled.core,
      ...snapshot,
    });
    const schemaJSON = canonical(compiled.schema);
    const coreResult = createFinancialExpressionContractV4(schemaJSON, financialJSON).evaluate(request);
    const expected = independentStage(action);
    independent[action] = expected;
    sourceResults.push(sourceResult);
    coreResults.push(coreResult);
    packets.push({ schema: schemaJSON, request, financialPreState: financialJSON, action });
    if (sourceResult.status !== 'FundedExpressionPrepared') {
      break;
    }
    ordinary = sourceResult.post;
    financial = sourceResult.financialPost;
  }
  return {
    sourceSHA256,
    wholeSourceChecked: true,
    actions,
    sourceResults,
    coreResults,
    packets,
    independent,
    kExecuted: false,
    kStatus: mode === 'offline' ? 'not-executed' : 'not-admitted',
    kResults: [],
    independentCases: CASES.independentCases || [],
    mode,
  };
}

function isDirect() {
  const invoked = process.argv[1];
  if (typeof invoked !== 'string' || invoked.length === 0) return false;
  return import.meta.url === pathToFileURL(path.resolve(invoked)).href;
}

if (isDirect()) {
  const report = await runLifecycleCorpus({ mode: 'offline' });
  const mismatches = [];
  for (const [i, action] of report.actions.entries()) {
    const sourceCmp = compareLifecycleResult(report.sourceResults[i], report.independent[action]);
    const coreCmp = compareLifecycleResult(report.coreResults[i], report.independent[action]);
    if (!sourceCmp.ok || !coreCmp.ok) mismatches.push({ action, source: sourceCmp, core: coreCmp });
  }
  console.log(JSON.stringify({
    kExecuted: report.kExecuted,
    kStatus: report.kStatus,
    sourceSHA256: report.sourceSHA256,
    wholeSourceChecked: report.wholeSourceChecked,
    mismatches,
  }));
  process.exitCode = mismatches.length ? 1 : 0;
}
