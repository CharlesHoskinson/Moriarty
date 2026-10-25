// Manual V3 compilation is pinned here, independently of compactc's skip-ZK manifest.
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';

export function pinnedRollupKeys(types, IntegrityError, record) {
  const pins = structuredClone(record);
  const read = pin => {
    if (!pin) throw new IntegrityError('missing pinned rollup artifact');
    const raw = readFileSync(pin.path);
    if (raw.length !== pin.bytes || createHash('sha256').update(raw).digest('hex') !== pin.sha256)
      throw new IntegrityError('rollup artifact digest mismatch: ' + pin.path);
    return raw;
  };
  const entry = name => {
    if (!['acceptValidity', 'settleValidity'].includes(name)) throw Error('unsupported rollup circuit');
    return pins.provingArtifacts[name];
  };
  return new class extends types.ZKConfigProvider {
    async getProverKey(name) { return types.createProverKey(read(entry(name).prover)); }
    async getZKIR(name) { return types.createZKIR(read(entry(name).ir)); }
    async getVerifierKey(name) { entry(name); return types.createVerifierKey(read(pins.keys[name])); }
  }();
}
