import { readFileSync } from 'node:fs';
import { canonical } from '../../../src/successor/expression-wire-v1.ts';

// Supplemental raw-input packets for native checks. Expected outcomes follow
// explicit byte boundaries; no host-computed validation flags enter K input.
const read = name => JSON.parse(readFileSync(new URL(name, import.meta.url), 'utf8'));
const expressionBase = read('expression-v1.json').cases.find(x => x.id === 'LitText-positive');
const lifecycleBase = read('lifecycle-v1.json').cases.find(x => x.id === 'chain-originate');
const synthetic = { end: '0', kind: 'synthetic', start: '0' };
const rejected = code => ({ status: 'Rejected', code, span: synthetic, nodePath: [], workUsed: '0' });
export const expressionUnicodeCases = [];
for (const [name, character, width] of [['two-byte', 'é', 2], ['three-byte', '中', 3], ['four-byte', '😀', 4]]) {
  for (const extra of [0, 1]) {
    const value = character.repeat(Math.floor(1024 / width)) + 'x'.repeat(1024 % width + extra);
    const request = JSON.parse(expressionBase.request);
    request.core.operands.value = value;
    expressionUnicodeCases.push({ id: `unicode-text-${name}-${1024 + extra}`, schema: expressionBase.schema,
      request: canonical(request), expected: extra ? rejected('TYPE_LITERAL') :
        { judgmentResult: 'ExpressionValue', type: ['Text'], value, workRemaining: '99' } });
  }
  for (const extra of [0, 1]) {
    const request = JSON.parse(expressionBase.request);
    request.source = character.repeat(Math.floor(65536 / width)) + 'x'.repeat(65536 % width + extra);
    expressionUnicodeCases.push({ id: `unicode-source-${name}-${65536 + extra}`, schema: expressionBase.schema,
      request: canonical(request), expected: extra ? rejected('INPUT_BOUND') : expressionBase.expected });
  }
}
export const lifecycleUnicodeCases = [];
for (const extra of [0, 1]) {
  const packet = structuredClone(lifecycleBase.packet);
  const request = JSON.parse(packet.request);
  request.source += '😀'.repeat(Math.floor((65536 - Buffer.byteLength(request.source)) / 4));
  request.source += 'x'.repeat(65536 + extra - Buffer.byteLength(request.source));
  packet.request = canonical(request);
  lifecycleUnicodeCases.push({ id: `unicode-lifecycle-source-${65536 + extra}`, packet,
    expected: extra ? rejected('INPUT_BOUND') : lifecycleBase.expected });
}
// A non-ASCII unknown field remains a schema error while the raw financial
// context fits the byte limit. Inflating UTF-8 bytes would hide that diagnostic.
for (const extra of [0, 1]) {
  const packet = structuredClone(lifecycleBase.packet);
  const state = JSON.parse(packet.financialPreState);
  state['😀'] = '中'.repeat(10000);
  let text = JSON.stringify(state);
  text += ' '.repeat(65536 + extra - Buffer.byteLength(text));
  packet.financialPreState = text;
  lifecycleUnicodeCases.push({ id: `unicode-financial-context-${65536 + extra}`, packet,
    expected: { status: 'Rejected', code: extra ? 'INPUT_BOUND' : 'UNKNOWN_FIELD', ...(extra ? {} : { actionIndex: null }) } });
}

// Direct native function probes distinguish UTF-16 counts from UTF-8 bytes;
// API bounds alone cannot distinguish them because valid UTF-8 is never shorter.
export const unicodeCounterCases = [
  { text: '', utf8: 0, utf16: 0, quotedBytes: 2, quotedUnits: 2 },
  { text: 'é', utf8: 2, utf16: 1, quotedBytes: 4, quotedUnits: 3 },
  { text: '中', utf8: 3, utf16: 1, quotedBytes: 5, quotedUnits: 3 },
  { text: '😀', utf8: 4, utf16: 2, quotedBytes: 6, quotedUnits: 4 },
  { text: 'é中😀\u0000\b\t\n\f\r"\\', utf8: 17, utf16: 12, quotedBytes: 31, quotedUnits: 26 },
  { text: '\u{10ffff}', utf8: 4, utf16: 2, quotedBytes: 6, quotedUnits: 4 },
];
