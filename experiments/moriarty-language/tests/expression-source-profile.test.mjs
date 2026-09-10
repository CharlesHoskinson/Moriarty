import test from 'node:test';
import assert from 'node:assert/strict';
import { parseExpressionSource, formatExpressionSource } from '../src/successor/expression-source-frontend.ts';
import { parseSuccessorSource } from '../src/successor/frontend.ts';
import { formatSuccessorSource } from '../src/successor/format.ts';
const old = 'profile "moriarty-successor-syntax/0"; agreement Test { action step() { requires true; } }';
const current = old.replace('moriarty-successor-syntax/0', 'moriarty-expression-source/1');
test('profile separation requires exact source headers at both entry points', () => {
  assert.equal(parseSuccessorSource(old).profile.value, 'moriarty-successor-syntax/0');
  assert.equal(parseExpressionSource(current).profile.value, 'moriarty-expression-source/1');
  assert.throws(() => parseSuccessorSource(current), e => e.code === 'PROFILE_MISMATCH');
  assert.throws(() => parseExpressionSource(old), e => e.code === 'PROFILE_MISMATCH');
});
test('profile formatters retain their respective exact source headers', () => {
  assert.equal(formatSuccessorSource(formatSuccessorSource(old)), formatSuccessorSource(old));
  assert.equal(formatExpressionSource(formatExpressionSource(current)), formatExpressionSource(current));
  assert.throws(() => formatSuccessorSource(current), e => e.code === 'PROFILE_MISMATCH');
  assert.throws(() => formatExpressionSource(old), e => e.code === 'PROFILE_MISMATCH');
});

test('new grammar supports numeric metadata, signed values, generics and brackets only under /1', () => {
  for (const body of ['let x = -1;', 'let x = none<Collection<UInt64,2>>();', 'let x = collection<UInt64,2>()[u64(0)];', 'emit Notice row;']) {
    const next = `profile "moriarty-expression-source/1"; agreement Demo { action step() { ${body} } }`;
    assert.doesNotThrow(() => parseExpressionSource(next));
    assert.throws(() => parseSuccessorSource(next.replace('moriarty-expression-source/1','moriarty-successor-syntax/0')));
  }
});
test('generic calls and comparisons remain distinct and typed emissions reject during parsing', () => {
  assert.doesNotThrow(() => parseExpressionSource(current.replace('requires true;', 'requires a < b; let x = some<Option<UInt64>>(none<UInt64>());')));
  for (const body of ['emit Notice<UInt64> {};', 'let x = record<1> {};', 'let x = record<Row,Other> {};'])
    assert.throws(() => parseExpressionSource(current.replace('requires true;', body)), e => e.code === 'UNEXPECTED_TOKEN');
  assert.throws(() => parseExpressionSource(current.replace('step()', 'step(x: 1)')), e => e.code === 'UNEXPECTED_TOKEN');
});

test('exported lexical catalogs cannot mutate subsequent profile admission', async () => {
  const { GENERIC_PRIMARIES, SOURCE_KEYWORDS } = await import('../src/successor/frontend.ts');
  assert.throws(() => GENERIC_PRIMARIES.pop(), TypeError);
  assert.throws(() => SOURCE_KEYWORDS.push('hello'), TypeError);
  assert.doesNotThrow(() => parseExpressionSource(current.replace('requires true;', 'let x = none<UInt64>();')));
});
