import { readFileSync } from 'node:fs';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { createFinancialAgreementSourceV5 } from '../src/successor/financial-agreement-source-v5.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';

const read = (name) => readFileSync(new URL('../spec/successor/examples/' + name, import.meta.url), 'utf8');

export function loadLoanLifecycle() {
  const source = read('loan-lifecycle.mori');
  const stateText = read('loan-lifecycle.state.json');
  const snapshotsText = read('loan-lifecycle.snapshots.json');
  return {
    source,
    stateText,
    snapshotsText,
    language: createFinancialAgreementSourceV5(),
  };
}

export function evaluateLoanLifecycleAction(language, source, action, predecessor, args) {
  const snapshot = {
    Args: args,
    Obs: {},
    Pre: predecessor.post,
    workInitial: predecessor.financialPost.work.remaining,
  };
  const snapshotJSON = canonical(snapshot);
  const financialJSON = JSON.stringify(predecessor.financialPost);
  const result = language.evaluate(source, action, snapshotJSON, financialJSON);
  return { action, snapshot, snapshotJSON, financialPre: predecessor.financialPost, financialJSON, result };
}

export function runLoanLifecycleDemo() {
  const { source, stateText, snapshotsText, language } = loadLoanLifecycle();
  const checked = language.check(source);
  const calls = [];
  const originated = language.evaluate(source, 'originate', snapshotsText, stateText);
  calls.push({
    action: 'originate',
    snapshot: JSON.parse(snapshotsText),
    snapshotJSON: snapshotsText,
    financialPre: JSON.parse(stateText),
    financialJSON: stateText,
    result: originated,
  });
  if (originated.status !== 'FundedExpressionPrepared') {
    return { checked, calls, originated, settled: originated };
  }
  const accruedCall = evaluateLoanLifecycleAction(language, source, 'accrue', originated, {
    accrualId: 'A1', observedTime: '1060', periodIndex: '1',
  });
  calls.push(accruedCall);
  if (accruedCall.result.status !== 'FundedExpressionPrepared') {
    return { checked, calls, originated, accrued: accruedCall.result, settled: accruedCall.result };
  }
  const duplicateCall = evaluateLoanLifecycleAction(language, source, 'accrue', accruedCall.result, {
    accrualId: 'A1', observedTime: '1060', periodIndex: '1',
  });
  calls.push(duplicateCall);
  const repaidCall = evaluateLoanLifecycleAction(language, source, 'repay', accruedCall.result, {
    allocationId: 'R1', nominal: '30', transferId: 'P1',
  });
  calls.push(repaidCall);
  if (repaidCall.result.status !== 'FundedExpressionPrepared') {
    return { checked, calls, originated, accrued: accruedCall.result, repaid: repaidCall.result, settled: repaidCall.result };
  }
  const settledCall = evaluateLoanLifecycleAction(language, source, 'settle', repaidCall.result, {
    allocationId: 'R2', transferId: 'P2',
  });
  calls.push(settledCall);
  if (settledCall.result.status === 'FundedExpressionPrepared') {
    const failedSettled = evaluateLoanLifecycleAction(language, source, 'settle', settledCall.result, {
      allocationId: 'R3', transferId: 'P3',
    });
    calls.push(failedSettled);
  }
  return {
    checked,
    calls,
    originated,
    accrued: accruedCall.result,
    duplicate: duplicateCall.result,
    repaid: repaidCall.result,
    settled: settledCall.result,
  };
}

function isDirectInvocation() {
  const invoked = process.argv[1];
  if (typeof invoked !== 'string' || invoked.length === 0) return false;
  return import.meta.url === pathToFileURL(path.resolve(invoked)).href;
}

if (isDirectInvocation()) {
  console.log(JSON.stringify(runLoanLifecycleDemo(), null, 2));
}
