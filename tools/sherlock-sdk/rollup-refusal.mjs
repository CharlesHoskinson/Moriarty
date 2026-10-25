// Submission errors may contain the complete transaction; retain numeric RPC codes only.
import assert from 'node:assert/strict';
import {writeFileSync} from 'node:fs';
import {join} from 'node:path';

export async function quietSubmission(operation) {
  // The pinned RPC client logs server error.data before rejecting the SDK
  // promise. Each runner owns one submission; restore logging on either exit.
  const saved = Object.fromEntries(['log', 'error', 'warn', 'info', 'debug', 'trace']
    .map(name => [name, console[name]]));
  try {
    for (const name of Object.keys(saved)) console[name] = () => {};
    return await operation();
  } finally { Object.assign(console, saved); }
}

export function submissionRefusal(error) {
  const pending = [[error, 0]], seen = new Set(), codes = new Set();
  while (pending.length && seen.size < 128) {
    const [value, depth] = pending.pop();
    if (!value || depth > 8 || seen.has(value)) continue;
    seen.add(value);
    const message = typeof value === 'string' ? value : value.message;
    if (typeof message === 'string') {
      for (const match of message.matchAll(/\b1010:\s*Invalid Transaction:\s*Custom error:\s*(\d{1,3})\b/g)) {
        const code = Number(match[1]);
        if (code <= 255) codes.add(code);
      }
    }
    if (typeof value !== 'object') continue;
    for (const key of ['cause', 'error', 'failure', 'reason', 'left', 'right']) {
      if (key in value) pending.push([value[key], depth + 1]);
    }
    for (const key of Object.getOwnPropertySymbols(value)) {
      if (/Cause/.test(String(key))) pending.push([value[key], depth + 1]);
    }
  }
  return {kind: codes.size ? 'rpc-invalid-transaction' : 'unclassified', codes: [...codes].sort((a, b) => a - b)};
}

export function recordSubmissionRefusal(error, output, purpose) {
  assert.ok(['deploy', 'acceptValidity', 'settleValidity'].includes(purpose));
  const report = {schema: 'sherlock-public-submission-refusal/1', at: new Date().toISOString(),
    purpose, ...submissionRefusal(error), willRetry: false};
  writeFileSync(join(output, 'refusal.json'), JSON.stringify(report, null, 2) + '\n', {flag: 'wx'});
  console.error('Rollup submission stopped; refusal codes: ' + (report.codes.join(',') || 'unclassified'));
}
