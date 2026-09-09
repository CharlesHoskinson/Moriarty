// Independent GPT-6 audit regression for the compact generic token boundary.
import test from 'node:test';
import assert from 'node:assert/strict';
import { parseSuccessorSource, SuccessorSyntaxError } from '../src/successor/frontend.ts';
import { formatSuccessorSource } from '../src/successor/format.ts';

function compactGeneric(argumentCount) {
  return 'profile "moriarty-successor-syntax/0";agreement X{const a:T<'
    + Array(argumentCount).fill('A').join(',') + '>=1;const b:T<A>=1;}';
}

test('compact type closers reserve their split tokens and preserve formatter closure at the limit', () => {
  // 8190 lexer tokens including EOF, plus two conceptual >= splits: 8192.
  const source = compactGeneric(4083);
  const parsed = parseSuccessorSource(source);
  assert.equal(parsed.agreement.declarations[0].type.arguments.length, 4083);
  const formatted = formatSuccessorSource(source);
  assert.equal(parseSuccessorSource(formatted).agreement.declarations[0].type.arguments.length, 4083);
  assert.equal(formatSuccessorSource(formatted), formatted);
});

test('compact type closers cannot hide tokens beyond the fixed limit', () => {
  // 8192 lexer tokens including EOF, plus two conceptual splits: 8194.
  assert.throws(() => parseSuccessorSource(compactGeneric(4084)), error =>
    error instanceof SuccessorSyntaxError && error.code === 'TOKEN_BOUND');
});
