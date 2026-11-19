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
