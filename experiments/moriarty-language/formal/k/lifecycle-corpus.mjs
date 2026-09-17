import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { fileURLToPath, pathToFileURL } from 'node:url';
import path from 'node:path';
import { createFinancialAgreementSourceV5 } from '../../src/successor/financial-agreement-source-v5.ts';
import { createFinancialExpressionContractV4, FINANCIAL_EXPRESSION_CONTRACT_V4 } from '../../src/successor/financial-expression-v1.ts';
import { canonical } from '../../src/successor/expression-wire-v1.ts';

const HERE = fileURLToPath(new URL('.', import.meta.url));
const SOURCE_PATH = path.resolve(HERE, '../../spec/successor/examples/loan-lifecycle.mori');
const EVIDENCE = path.resolve(HERE, '../../../../deliverables/language-to-ledger-2026-09-12');
const SOURCE_SHA = 'a0efd5166fdf27106110c69548d3b3065070e8d759d231296c454d19757bbaaa';
const ACTIONS = ['originate', 'accrue', 'repay', 'settle'];
const FORBIDDEN = ['post', 'financialPost', 'effects', 'descriptors', 'continueSuffix', 'workRemaining'];
const ARGS = {
  originate: { transferId: 'D1', originationId: 'O1' },
  accrue: { accrualId: 'A1', observedTime: '1060', periodIndex: '1' },
  repay: { allocationId: 'R1', nominal: '30', transferId: 'P1' },
  settle: { allocationId: 'R2', transferId: 'P2' },
};
const copy = structuredClone;

// Wire records have unordered keys but ordered arrays. Presence is distinct from
// an own key with undefined, including sparse array slots and extra array keys.
export function compareLifecycleResult(actual, expected) {
  const differences = [];
  const walk = (left, right, route) => {
    if (Object.is(left, right)) return;
    if (left === null || right === null || typeof left !== 'object' || typeof right !== 'object'
      || Array.isArray(left) !== Array.isArray(right)
      || Object.getPrototypeOf(left) !== Object.getPrototypeOf(right)) {
      differences.push(route);
      return;
    }
    for (const key of new Set([...Reflect.ownKeys(left), ...Reflect.ownKeys(right)])) {
      const next = `${route}/${String(key)}`;
      if (!Object.hasOwn(left, key) || !Object.hasOwn(right, key)) differences.push(next);
      else walk(left[key], right[key], next);
    }
  };
  walk(actual, expected, '');
  return { ok: differences.length === 0, differences };
}

// Derive statement diagnostic ranges directly from frozen ASCII source text,
// independently of elaborator/evaluator output. Include the terminating semicolon.
function statementSpan(source, text, from = 0) {
  const start = source.indexOf(text, from);
  if (start < 0) throw new Error(`Missing diagnostic statement: ${text}`);
  return { kind: 'source', start: String(Buffer.byteLength(source.slice(0, start))),
    end: String(Buffer.byteLength(source.slice(0, start + text.length))) };
}

/** Offline Source/Core/independent-oracle agreement only. Factories are injectable
 * for consumer wiring and failure tests; the CLI always uses the real factories. */
export async function runLifecycleCorpus({
  sourceFactory = createFinancialAgreementSourceV5,
  coreFactory = createFinancialExpressionContractV4,
} = {}) {
  const report = { ok: false, mode: 'offline', kExecuted: false, kStatus: 'not-executed',
    wholeSourceChecked: false, sourceSHA256: null, actions: ACTIONS, stages: [], cases: [], failures: [] };
  const compare = (label, actual, expected) => {
    const result = compareLifecycleResult(actual, expected);
    if (!result.ok) report.failures.push({ label, differences: result.differences });
  };
  try {
    const source = readFileSync(SOURCE_PATH, 'utf8');
    report.sourceSHA256 = createHash('sha256').update(source).digest('hex');
    if (report.sourceSHA256 !== SOURCE_SHA) throw new Error('Source changed: independently recount work before using this oracle.');
    // Resolve only within this checkout. Missing or malformed evidence fails the run.
    const templates = JSON.parse(readFileSync(path.join(EVIDENCE, 'design-review/lifecycle-source-expectations.json'), 'utf8'));
    const work = JSON.parse(readFileSync(path.join(EVIDENCE, 'lifecycle/root-work-expectations.json'), 'utf8'));
    if (work.sourceSHA256 !== SOURCE_SHA) throw new Error('Independent work oracle source hash mismatch.');
    compare('oracle actions', templates.stages.map(x => x.action), ACTIONS);
    compare('oracle work seed', work.seed, { remaining: '512', spent: '17', closureReserve: '16' });
    const independent = Object.fromEntries(ACTIONS.map((action, i) => {
      const w = work.actions[action];
      compare(`oracle work ${action}`, [w.total, w.remaining, w.spent],
        [[99, '413', '116'], [65, '348', '181'], [88, '260', '269'], [86, '174', '355']][i]);
      const expected = copy(templates.stages.find(x => x.action === action).expectedResultTemplate);
      expected.financialPost.work = { remaining: w.remaining, spent: w.spent, closureReserve: work.seed.closureReserve };
      expected.workRemaining = w.remaining;
      return [action, expected];
    }));
    const language = sourceFactory();
    const compile = (text) => {
      const checked = language.check(text);
      const elaborated = language.elaborate(text);
      if (checked.judgmentResult !== 'SourceChecked' || elaborated.judgmentResult !== 'SourceElaborated') {
        report.failures.push({ label: 'source check/elaboration', checked, elaborated });
        throw new Error('Whole source check/elaboration failed.');
      }
      compare('checked action set', checked.actions.map(x => x.action), ACTIONS);
      compare('elaborated action set', elaborated.actions.map(x => x.action), ACTIONS);
      return elaborated;
    };
    const elaborated = compile(source);
    report.wholeSourceChecked = true;
    const seed = { post: copy(templates.seed.ordinary), financialPost: copy(templates.seed.financial) };
    seed.financialPost.work = copy(work.seed);
    const predecessors = { source: [copy(seed)], core: [copy(seed)] };
    const execute = (kind, text, compiled, action, predecessor, args, workInitial) => {
      const before = copy(predecessor);
      const snapshot = { Args: copy(args), Obs: {}, Pre: copy(predecessor.post),
        workInitial: workInitial ?? predecessor.financialPost.work.remaining };
      const financialPreState = JSON.stringify(predecessor.financialPost);
      const artifact = compiled.actions.find(x => x.action === action);
      if (!artifact) throw new Error(`Missing elaborated action ${action}`);
      const request = canonical({ contract: FINANCIAL_EXPRESSION_CONTRACT_V4, source: text, core: artifact.core, ...snapshot });
      const schema = canonical(artifact.schema);
      const result = kind === 'source'
        ? language.evaluate(text, action, canonical(snapshot), financialPreState)
        : coreFactory(schema, financialPreState).evaluate(request);
      compare(`${kind} ${action} unchanged input`, predecessor, before);
      return { input: { snapshot, financialPreState, schema, request }, result };
    };
    for (const action of ACTIONS) {
      const stage = { action, expected: independent[action] };
      report.stages.push(stage);
      for (const kind of ['source', 'core']) {
        stage[kind] = execute(kind, source, elaborated, action, predecessors[kind].at(-1), ARGS[action]);
        compare(`${action} ${kind} independent`, stage[kind].result, independent[action]);
      }
      compare(`${action} Source/Core`, stage.source.result, stage.core.result);
      for (const kind of ['source', 'core']) {
        if (stage[kind].result.status !== 'FundedExpressionPrepared') throw new Error(`Incomplete ${kind} lifecycle at ${action}`);
        // Each evaluator consumes only its own actual previous output.
        predecessors[kind].push(stage[kind].result);
      }
    }
    const lateSource = source.replace('ensures post_balance<Cash>("Lender") == amount<Cash>(110);',
      'ensures post_balance<Cash>("Lender") == amount<Cash>(111);');
    const lateCompiled = compile(lateSource);
    const kernel = code => ({ status: 'Rejected', code, actionIndex: 0 });
    const settleStart = source.indexOf('action settle(');
    const cases = [
      { name: 'duplicate-accrual', predecessor: 2, action: 'accrue', args: ARGS.accrue,
        expected: kernel('DUPLICATE'), retry: 'repay' },
      { name: 'repeated-period', predecessor: 2, action: 'accrue', args: { accrualId: 'A2', observedTime: '1120', periodIndex: '1' },
        expected: kernel('PERIOD_SEQUENCE'), retry: 'repay' },
      { name: 'early-period', predecessor: 1, action: 'accrue', args: { ...ARGS.accrue, observedTime: '1059' },
        expected: kernel('PERIOD_NOT_ELIGIBLE'), retry: 'accrue' },
      { name: 'incurred-cap-after-repay', predecessor: 3, action: 'accrue', args: { accrualId: 'A2', observedTime: '1120', periodIndex: '2' },
        expected: kernel('LIABILITY_CAP_EXCEEDED'), retry: 'settle' },
      { name: 'insufficient-funding', predecessor: 2, action: 'repay', args: { transferId: 'PF', allocationId: 'RF', nominal: '111' },
        expected: kernel('INSUFFICIENT_BALANCE'), retry: 'repay', retryArgs: { transferId: 'PF', allocationId: 'RF', nominal: '30' } },
      // Let debt (3), Let magnitude (3), Require/Gt/ReadLocal/LitUInt (4).
      { name: 'settled-source-guard', predecessor: 4, action: 'settle', args: { transferId: 'P3', allocationId: 'R3' },
        expected: { status: 'Rejected', code: 'GUARD_FAILED', workUsed: '10', nodePath: ['2'],
          span: statementSpan(source, 'requires payment > 0;', settleStart) } },
      // All seven prefix statements, two kernel actions, nine suffix statements.
      { name: 'late-false-ensure', predecessor: 3, action: 'settle', args: ARGS.settle, text: lateSource, compiled: lateCompiled,
        expected: { status: 'Rejected', code: 'ENSURES_FAILED', workUsed: '86', nodePath: ['15'],
          span: statementSpan(lateSource, 'ensures post_balance<Cash>("Lender") == amount<Cash>(111);') }, retry: 'settle' },
      { name: 'malformed-unrelated-allowance', predecessor: 1, action: 'accrue', args: ARGS.accrue,
        mutate: pre => { pre.financialPost.allowances[2].remaining = String((1n << 128n) - 1n); },
        expected: { status: 'Rejected', code: 'INVARIANT', actionIndex: null }, retry: 'accrue' },
      { name: 'work-mismatch', predecessor: 1, action: 'accrue', args: ARGS.accrue, workInitial: '412',
        expected: { status: 'Rejected', code: 'WORK_MISMATCH' }, retry: 'accrue' },
    ];
    for (const item of cases) {
      const row = { name: item.name, expected: item.expected, retry: item.retry ?? null };
      for (const kind of ['source', 'core']) {
        const predecessor = predecessors[kind][item.predecessor];
        const attemptPre = copy(predecessor);
        item.mutate?.(attemptPre);
        const run = () => execute(kind, item.text ?? source, item.compiled ?? elaborated, item.action, attemptPre, item.args, item.workInitial);
        row[kind] = run();
        compare(`${item.name} ${kind} independent`, row[kind].result, item.expected);
        compare(`${item.name} ${kind} deterministic rejection`, run().result, row[kind].result);
        for (const key of FORBIDDEN) if (Object.hasOwn(row[kind].result, key)) report.failures.push({ label: `${item.name} ${kind} published ${key}` });
        if (item.retry) {
          const retryArgs = item.retryArgs ?? ARGS[item.retry];
          const expectedRetry = copy(independent[item.retry]);
          if (item.retryArgs) {
            // Independently substitute only the two successful retry identities.
            expectedRetry.financialPost.usedTransferIds = ['D1', 'PF'];
            expectedRetry.financialPost.usedAllocationIds = ['RF'];
            expectedRetry.effects[0].id = 'PF';
            expectedRetry.effects[1].transferId = 'PF';
            expectedRetry.effects[1].allocationId = 'RF';
          }
          row[kind].retry = execute(kind, source, elaborated, item.retry, predecessor, retryArgs);
          compare(`${item.name} ${kind} independent retry`, row[kind].retry.result, expectedRetry);
          compare(`${item.name} ${kind} deterministic retry`,
            execute(kind, source, elaborated, item.retry, predecessor, retryArgs).result, row[kind].retry.result);
        }
      }
      compare(`${item.name} Source/Core`, row.source.result, row.core.result);
      if (item.retry) compare(`${item.name} retry Source/Core`, row.source.retry.result, row.core.retry.result);
      report.cases.push(row);
    }
    if (report.stages.length !== 4 || report.cases.length !== 9) throw new Error('Incomplete corpus.');
    report.ok = report.failures.length === 0;
  } catch (error) {
    report.failures.push({ label: 'corpus failure', message: error instanceof Error ? error.message : String(error) });
  }
  return report;
}

if (process.argv[1] && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) {
  const report = await runLifecycleCorpus();
  console.log(JSON.stringify(report));
  process.exitCode = report.ok ? 0 : 1;
}
