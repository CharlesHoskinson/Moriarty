/** Bounded Source/6 formation → purpose-1 image demonstration; content only. */
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

const base = new URL('./', import.meta.url);
const sha256 = bytes => createHash('sha256').update(bytes).digest('hex');
const protectedInputs = JSON.parse(readFileSync(new URL('protected-inputs.json', base), 'utf8'));
// Verify read-only dependencies before importing and executing those bytes.
for (const input of protectedInputs.inputs) {
  const bytes = readFileSync(new URL(input.path, base));
  if (bytes.length !== input.bytes || sha256(bytes) !== input.sha256)
    throw Error(`Frozen read-only input changed: ${input.path}`);
}
const {SOURCE_IMAGE_SUITE, projectSource6Definition, encodeSource6Definition,
  compareSource6DefinitionContent} = await import('./adapter.mjs');
const {transfer, repay} = await import('./fixtures.mjs');
const vectors = JSON.parse(readFileSync(new URL('vectors.json', base), 'utf8'));
const results = [];
for (const [kind, fixture] of [['Transfer', transfer], ['Repay', repay]]) {
  const source = fixture();
  const vector = vectors.find(v => v.expectedFields.operationKind === kind);
  const image = encodeSource6Definition(source);
  if (image.payload.toString('hex') !== vector.payloadHex
    || image.preimage.toString('hex') !== vector.preimageHex || image.digest !== vector.sha256)
    throw Error(`Independent expected bytes mismatch: ${kind}`);
  const file = `${kind.toLowerCase()}.source.mori`;
  writeFileSync(new URL(file, base), source + '\n');
  results.push({kind, sourceFile: file, sourceTextSha256: sha256(Buffer.from(source + '\n')),
    definition: projectSource6Definition(source), purpose: image.purpose,
    payloadBytes: image.payload.length, payloadHex: image.payload.toString('hex'),
    preimageHex: image.preimage.toString('hex'), digest: image.digest,
    comparison: compareSource6DefinitionContent(source, vector.sha256),
    unchangedAfterClaimMutation: encodeSource6Definition(fixture({sourceHash: 'other-source-claim'})).digest === image.digest,
    embeddedSourceClaim: 'claim-source', embeddedPolicyClaim: 'claim-policy'});
}
const record = {status: 'local-source-image-experiment-not-adopted',
  codecRepair: 'proxy-repair-01', suite: SOURCE_IMAGE_SUITE,
  claim: 'parsed-selected-definition-content-only', authenticated: false,
  protectedInputsManifestSha256: sha256(readFileSync(new URL('protected-inputs.json', base))),
  verifiedReadOnlyInputs: protectedInputs.inputs.length, results};
writeFileSync(new URL('result-data.json', base), JSON.stringify(record, null, 2) + '\n');
console.log(JSON.stringify({status: record.status, inputsVerified: record.verifiedReadOnlyInputs,
  outputs: results.map(({kind, digest, payloadBytes}) => ({kind, digest, payloadBytes}))}, null, 2));
