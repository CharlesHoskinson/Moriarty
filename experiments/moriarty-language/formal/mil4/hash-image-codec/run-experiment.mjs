/** Reproduce local content results using explicit current package bytes. */
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync, writeFileSync} from 'node:fs';
import {encodeImage, produceImages, compareContent} from './codec.mjs';

const vectors = JSON.parse(readFileSync(new URL('./vectors.json', import.meta.url)));
const digest = raw => createHash('sha256').update(raw).digest('hex');
// The live design can advance while this experiment stays pinned to repair-02.
const packetPath = '../../../../../deliverables/mil4-k-quint-sprint1-2026-09-29/audits/w-d2h-grok-repair-02-candidate-packet.md';
const packetBytes = readFileSync(new URL(packetPath, import.meta.url));
const packetSha256 = 'daa09426d9e7c64362c0c715b04fff961448d8942441f007f948afb750f551e9';
assert.equal(digest(packetBytes), packetSha256, 'frozen repair-02 packet changed');
const prefix = `## experiments/moriarty-language/formal/mil4/hash-images/SPEC.md\n\nsha256: \`${vectors.specSha256}\`\n\n\`\`\`text\n`;
const suffix = '\n```\n\n## experiments/moriarty-language/formal/mil4/hash-images/DECISION-MATRIX.md';
const sourceBytes = Buffer.from(packetBytes.toString('utf8').split(prefix)[1].split(suffix)[0], 'utf8');
assert.equal(digest(sourceBytes), vectors.specSha256, 'embedded repair-02 SPEC bytes changed');
const modules = vectors.actualModuleInputs.map(input => {
  const bytes = readFileSync(new URL(`../../../src/successor/${input.name}`, import.meta.url));
  assert.equal(bytes.length, input.bytes, 'current package input length changed');
  assert.equal(digest(bytes), input.sha256, 'current package input bytes changed');
  return {role: input.role, bytes};
});
const observations = [];
for (const index of [0, 1]) {
  const input = structuredClone(vectors.vectors[index].inputs);
  input.package.files = modules;
  delete input.package.filesHex;
  const result = produceImages(input.definition, input.package, input.terms);
  const expected = vectors.actualPackages[index];
  assert.equal(result.core.digest, expected.sha256, 'separate Python actual-package digest differs');
  assert.equal(result.core.payload.length, expected.payloadBytes);
  const images = {};
  for (const name of ['source', 'core', 'policy']) images[name] = {
    purpose: result[name].purpose, payloadBytes: result[name].payload.length,
    envelopeBytes: result[name].preimage.length, sha256: result[name].digest,
  };
  const same = compareContent(4, result.policyValue, result.policy.digest);
  const changed = structuredClone(result.policyValue);
  changed.operation.amount = (BigInt(changed.operation.amount) + 1n).toString();
  const different = compareContent(4, changed, result.policy.digest);
  assert.equal(same.matches, true); assert.equal(different.matches, false);
  observations.push({operationKind: input.definition.operationKind, images,
    exactPolicyContentComparison: same, changedAmountContentComparison: different,
    changedAmountSha256: encodeImage(4, changed).digest});
}
const report = {
  status: 'local-executable-experiment-not-adopted', pinnedCandidate: 'hash-images-repair-02',
  specSha256: vectors.specSha256, packetSha256,
  codecRepair: 'proxy-repair-01',
  originalExecutableAuditPacketSha256: '34e34595aaab4094ee4735ce81b511b873f82fe3ad7f89648ebcfecb7757d332',
  currentExecutableIndependentReview: 'pending',
  repair03ContentComparison: JSON.parse(readFileSync(new URL('./repair03-comparison.json', import.meta.url))),
  inputAdmission: 'descriptor values captured once into frozen owned records; byte views copied before producer phases',
  node: process.version, moduleInputs: vectors.actualModuleInputs, observations,
  frozenSyntheticCompleteVectors: vectors.vectors.length * 3,
  frozenActualPackageDigestVectors: vectors.actualPackages.length,
  vectorIndependence: 'different language and serialization implementation; same author; no independent provider review',
  authentication: false, b05b06b07Adopted: false, wd2GateClosed: false,
  limitations: ['closed projected typed values, not a Source parser or full common-domain AST sweep',
    'no artifact decoder or consumer schedule', 'no provider or registry authenticity',
    'no actual-export, dependency-closure, loaded-artifact, toolchain or lowering correspondence proof',
    'no B16 complete-history adapter', 'no signature, native proof or ledger submission'],
};
writeFileSync(new URL('./result-data.json', import.meta.url), JSON.stringify(report, null, 2) + '\n');
console.log(JSON.stringify(report, null, 2));
