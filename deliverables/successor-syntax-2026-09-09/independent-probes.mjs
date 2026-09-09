import assert from 'node:assert/strict';
import { pathToFileURL } from 'node:url';
import { writeFileSync, mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';

const root = process.argv[2];
const { parseSuccessorSource } = await import(pathToFileURL(join(root, 'experiments/moriarty-language/src/successor/frontend.ts')));
const { formatSuccessorSource } = await import(pathToFileURL(join(root, 'experiments/moriarty-language/src/successor/format.ts')));
const header = 'profile "moriarty-successor-syntax/0"; agreement Check { ';
const agreement = body => header + body + ' }';
const action = expr => agreement('action examine() { requires ' + expr + '; }');
const results = [];
function check(name, fn) {
  try { fn(); results.push({ name, pass: true }); }
  catch (error) { results.push({ name, pass: false, error: String(error) }); }
}
function rejected(source) {
  let error;
  try { parseSuccessorSource(source); } catch (e) { error = e; }
  assert.ok(error instanceof Error, 'invalid input was accepted');
  assert.ok(typeof error.code === 'string' && error.code.length, 'missing stable diagnostic code');
}

check('second unrelated syntax program', () => parseSuccessorSource(agreement('unit EUR; const threshold: Uint = 17; action inspect() { requires threshold > 3; ensures true; }')));
check('legal nested generics and effect', () => parseSuccessorSource(agreement('asset vault: Container<Shares<Vault>, Amount<USD>>; action route(a: Amount<USD>) { emit Transfer { amount: a, to: recipient }; }')));
check('formatter idempotence on Unicode comments and strings', () => {
  const source = agreement('/* €😀 */ const note: Text = "é😀\\n"; action inspect() { requires not a == b and c or d; }');
  const once = formatSuccessorSource(source);
  assert.equal(formatSuccessorSource(once), once);
  parseSuccessorSource(once);
});
for (const [name, source] of [
  ['wrong profile', 'profile "moriarty-bounded-atomic/1"; agreement X {}'],
  ['missing explicit profile', 'agreement X {}'],
  ['unknown declaration', agreement('obscure thing;')],
  ['trailing agreement', agreement('') + ' agreement Other {}'],
  ['statement after ensures', agreement('action x() { ensures true; let y = 1; }')],
  ['non-ASCII identifier', agreement('unit UЅD;')],
  ['canonical decimal leading zero', action('01 == 1')],
  ['float', action('1.1 > 0')],
  ['exponent', action('1e2 > 0')],
  ['chained comparison', action('a < b < c')],
  ['unary minus absent', action('-1 < 0')],
  ['plain division absent', action('10 / 2 == 5')],
  ['raw lone surrogate', agreement('const x: Text = "\ud800";')],
  ['escaped lone surrogate', agreement('const x: Text = "\\ud800";')],
  ['unfinished block comment', agreement('/* unfinished')],
  ['source bytes bounded', ' '.repeat(65537)],
  ['identifier bounded', agreement('unit ' + 'X'.repeat(65) + ';')],
  ['integer token bounded', action('1'.repeat(79) + ' > 0')],
  ['decoded string bounded', agreement('const x: Text = "' + '€'.repeat(342) + '";')],
  ['type depth bounded', agreement('state x: ' + 'T<'.repeat(66) + 'U' + '>'.repeat(66) + ' = 0;')],
  ['not depth bounded', action('not '.repeat(66) + 'true')],
  ['parenthesis depth bounded', action('('.repeat(66) + 'true' + ')'.repeat(66))],
  ['postfix depth bounded', action('x' + '.x'.repeat(66) + ' == 0')],
  ['call arguments bounded', action('f(' + Array(65).fill('1').join(',') + ') == 0')],
  ['parameters bounded', agreement('action x(' + Array.from({length:65},(_,i)=>'p'+i+':Uint').join(',') + ') {}')],
  ['statements bounded', agreement('action x() {' + 'requires true;'.repeat(257) + '}')],
  ['declarations bounded', agreement(Array.from({length:257},(_,i)=>'unit U'+i+';').join(' '))],
]) check(name, () => rejected(source));

check('hostile non-string input never coerced', () => {
  let coerced = false;
  rejected({ toString() { coerced = true; throw Error('caller code'); } });
  assert.equal(coerced, false);
});

const temporary = mkdtempSync(join(tmpdir(), 'moriarty-syntax-probe-'));
try {
  const path = join(temporary, 'invalid.mori');
  writeFileSync(path, Buffer.from([0xc0, 0xaf]));
  check('CLI invalid UTF8 fails before replacement decoding', () => {
    const r = spawnSync(process.execPath, [join(root, 'experiments/moriarty-language/src/successor/syntax-cli.ts'), 'check-syntax', path], { encoding:'utf8', timeout:10000 });
    assert.notEqual(r.status, 0);
    assert.equal(r.stdout, '');
    assert.match(r.stderr, /utf.?8|encoding|decode/i);
  });
} finally { rmSync(temporary, { recursive:true }); }

console.log(JSON.stringify({ scope:'Independent provisional syntax/API controls only; no semantic/proof/chain claim', passed:results.filter(r=>r.pass).length, failed:results.filter(r=>!r.pass).length, results }, null, 2));
process.exitCode = results.some(r=>!r.pass) ? 1 : 0;
