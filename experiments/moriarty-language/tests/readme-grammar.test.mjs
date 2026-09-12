import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

test('root README embeds the complete canonical agreement-source /3 grammar without drift', () => {
  const readme = readFileSync(new URL('../../../README.md', import.meta.url));
  const grammar = readFileSync(new URL('../spec/successor/financial-agreement-source-v3-grammar.ebnf', import.meta.url));
  const blocks = [...readme.toString('utf8').matchAll(/^```ebnf\n([\s\S]*?)^```$/gm)];
  assert.equal(blocks.length, 1, 'expected exactly one complete EBNF block');
  assert.deepEqual(Buffer.from(blocks[0][1], 'utf8'), grammar);
});
