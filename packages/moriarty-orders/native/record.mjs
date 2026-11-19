import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {mkdirSync, writeFileSync} from 'node:fs';
import {join} from 'node:path';

export function writeEvidence(directory, name, value) {
  const json = JSON.stringify(value, (_, v) => typeof v === 'bigint' ? String(v) :
    v instanceof Set || v instanceof Map ? [...v] : v, 2);
  writeFileSync(join(directory, name), json + '\n');
}

export function retainedTransactions(directory) {
  mkdirSync(directory, {recursive:true});
  const records = [];
  return record => {
    const {beforeState, afterState, transaction, ...row} = record;
    for (const [name, bytes] of Object.entries({beforeState, afterState, transaction})) {
      const file = String(records.length).padStart(2,'0') + '-' + name + '.bin';
      writeFileSync(join(directory, file), bytes);
      row[name] = {file, sha256:createHash('sha256').update(bytes).digest('hex')};
    }
    records.push(row);
    writeEvidence(directory, 'receipts.json', records);
  };
}

// Evidence names the committed code that produced it. Refuse a modified or untracked
// source tree; generated evidence and factory state are the only permitted changes.
export function sourceRevision(root = new URL('../../../', import.meta.url).pathname) {
  const git = args => execFileSync('git', ['-C', root, ...args], {encoding: 'utf8'});
  const dirty = git(['status', '--porcelain', '--', '.', ':!deliverables/competitor-report',
    ':!qa/sherlock/status.json', ':!qa/sherlock/runs.jsonl']).trim();
  if (dirty && process.env.MORIARTY_EVIDENCE_DRAFT === '1') return null;
  if (dirty) throw Error('Generate evidence from a clean committed tree:\n' + dirty);
  return git(['rev-parse', 'HEAD']).trim();
}

// A draft (MORIARTY_EVIDENCE_DRAFT=1 on a modified tree) records no revision; checkers reject it.
export function generatorRecord(command) {
  const revision = sourceRevision();
  return {revision, draft: revision === null, command, node: process.version, platform: process.platform, architecture: process.arch};
}

export function requireCommittedGenerator(record) {
  if (!record || record.draft !== false || !/^[0-9a-f]{40}$/.test(record.revision ?? '')) {
    throw Error('Evidence was not generated from a clean committed revision');
  }
  return record.revision;
}
