import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import { parseSuccessorSource, SuccessorSyntaxError, SYNTAX_BOUNDS, SYNTAX_PROFILE } from '../src/successor/frontend.ts';
import { formatSuccessorSource } from '../src/successor/format.ts';

const here = dirname(fileURLToPath(import.meta.url));
const examplePath = join(here, '../spec/successor/examples/partial-payment.mori');
const profilePath = join(here, '../spec/successor/syntax-profile.json');
const srcCli = join(here, '../src/successor/syntax-cli.ts');

function runCli(args, options = {}) {
  return spawnSync(process.execPath, [srcCli, ...args], {
    encoding: 'buffer',
    maxBuffer: 2 * 1024 * 1024,
    ...options,
  });
}

function decode(buf) {
  return Buffer.from(buf ?? []).toString('utf8');
}

function probe(body) {
  return `profile "moriarty-successor-syntax/0";\nagreement Probe {\n${body}\n}\n`;
}

function actionProbe(statements) {
  return probe(`  unit USD;\n  action run() {\n${statements}\n  }\n`);
}

function stripSpans(value) {
  if (Array.isArray(value)) return value.map(stripSpans);
  if (value && typeof value === 'object') {
    const out = {};
    for (const [key, item] of Object.entries(value)) {
      if (key === 'span') continue;
      out[key] = stripSpans(item);
    }
    return out;
  }
  return value;
}

function assertSyntaxError(fn, code) {
  let thrown;
  try {
    fn();
  } catch (error) {
    thrown = error;
  }
  assert.ok(thrown instanceof SuccessorSyntaxError, `expected SuccessorSyntaxError, got ${thrown && thrown.name}`);
  if (code) assert.equal(thrown.code, code);
  assert.equal(typeof thrown.start, 'number');
  assert.equal(typeof thrown.end, 'number');
  assert.ok(thrown.start >= 0);
  assert.ok(thrown.end >= thrown.start);
  return thrown;
}

test('public API rejects non-string input without coercion', () => {
  for (const value of [null, undefined, 12, { source: 'x' }, Buffer.from('profile "moriarty-successor-syntax/0";')]) {
    const error = assertSyntaxError(() => parseSuccessorSource(value), 'SOURCE_TYPE');
    assert.equal(error.start, 0);
    assert.equal(error.end, 0);
  }
});

test('example interest-payment source parses, formats idempotently, and preserves AST ignoring spans', () => {
  const source = readFileSync(examplePath, 'utf8');
  assert.match(source, /profile "moriarty-successor-syntax\/0";/);
  const ast = parseSuccessorSource(source);
  assert.equal(ast.tag, 'Program');
  const named = /agreement\s+([A-Za-z][A-Za-z0-9_]*)/.exec(source);
  assert.equal(ast.agreement.name, named[1]);
  assert.equal(ast.profile.value, 'moriarty-successor-syntax/0');
  const kinds = ast.agreement.declarations.map((d) => d.tag);
  assert.ok(kinds.includes('UnitDecl'));
  assert.ok(kinds.includes('PartyDecl'));
  assert.ok(kinds.includes('StateDecl'));
  assert.ok(kinds.includes('ActionDecl'));
  const action = ast.agreement.declarations.find((d) => d.tag === 'ActionDecl');
  assert.ok(action.statements.some((s) => s.tag === 'Requires'));
  assert.ok(action.statements.some((s) => s.tag === 'Next'));
  assert.ok(action.postconditions.some((s) => s.tag === 'Ensures'));
  const formatted = formatSuccessorSource(source);
  assert.equal(formatSuccessorSource(formatted), formatted);
  assert.deepEqual(stripSpans(parseSuccessorSource(formatted)), stripSpans(ast));
});

test('independently derived custody agreement covers asset, const, let, emit and empty lists', () => {
  const source = `profile "moriarty-successor-syntax/0";
agreement CustodyHold {
  unit EUR;
  party depositor;
  party custodian;
  asset vault: Vault<EUR>;
  const limit: Amount<EUR> = amount(50, EUR);
  state held: Amount<EUR> = amount(0, EUR);
  action deposit(value: Amount<EUR>) {
    requires value > amount(0, EUR);
    requires pre.held + value <= limit;
    let total = pre.held + value;
    next.held = total;
    emit Credit { owner: depositor, quantity: value };
    ensures post.held == pre.held + value;
  }
  action noop() {
  }
  action pulse(flag: Flag) {
    emit Tick {};
    ensures true;
  }
}
`;
  const ast = parseSuccessorSource(source);
  assert.equal(ast.agreement.name, 'CustodyHold');
  const names = ast.agreement.declarations.map((d) => d.name);
  assert.deepEqual(names, ['EUR', 'depositor', 'custodian', 'vault', 'limit', 'held', 'deposit', 'noop', 'pulse']);
  const deposit = ast.agreement.declarations.find((d) => d.name === 'deposit');
  assert.equal(deposit.statements[2].tag, 'Let');
  assert.equal(deposit.statements[4].tag, 'Emit');
  assert.equal(deposit.statements[4].type.name, 'Credit');
  const noop = ast.agreement.declarations.find((d) => d.name === 'noop');
  assert.equal(noop.parameters.length, 0);
  assert.equal(noop.statements.length, 0);
  const pulse = ast.agreement.declarations.find((d) => d.name === 'pulse');
  assert.equal(pulse.statements[0].fields.length, 0);
  assert.equal(formatSuccessorSource(formatSuccessorSource(source)), formatSuccessorSource(source));
});

test('not binds looser than comparison and arithmetic is left-associative with tight multiply', () => {
  const source = actionProbe(`    let a = not x == y;\n    let b = 8 - 3 - 2;\n    let c = 1 + 2 * 3;\n    let d = not p and q or r;\n`);
  const ast = parseSuccessorSource(source);
  const action = ast.agreement.declarations.find((d) => d.tag === 'ActionDecl');
  const [notCmp, sub, add, bool] = action.statements.map((s) => s.expression);
  assert.equal(notCmp.tag, 'Unary');
  assert.equal(notCmp.operator, 'not');
  assert.equal(notCmp.operand.tag, 'Comparison');
  assert.equal(notCmp.operand.operator, '==');
  assert.equal(sub.tag, 'Binary');
  assert.equal(sub.operator, '-');
  assert.equal(sub.left.tag, 'Binary');
  assert.equal(sub.left.operator, '-');
  assert.equal(add.tag, 'Binary');
  assert.equal(add.operator, '+');
  assert.equal(add.right.tag, 'Binary');
  assert.equal(add.right.operator, '*');
  assert.equal(bool.tag, 'Binary');
  assert.equal(bool.operator, 'or');
  assert.equal(bool.left.tag, 'Binary');
  assert.equal(bool.left.operator, 'and');
  assert.equal(bool.left.left.tag, 'Unary');
  const formatted = formatSuccessorSource(source);
  assert.deepEqual(stripSpans(parseSuccessorSource(formatted)), stripSpans(ast));
});

test('parenthesized comparison may nest, but unparenthesized chains reject', () => {
  const nested = parseSuccessorSource(actionProbe('    let ok = (a == b) == c;\n'));
  const expr = nested.agreement.declarations.find((d) => d.tag === 'ActionDecl').statements[0].expression;
  assert.equal(expr.tag, 'Comparison');
  assert.equal(expr.left.tag, 'Comparison');
  for (const body of ['    let x = a == b == c;\n', '    let x = a < b < c;\n', '    let x = a <= b > c;\n', '    let x = 1 != 2 == 3;\n']) {
    assertSyntaxError(() => parseSuccessorSource(actionProbe(body)), 'CHAINED_COMPARISON');
  }
});

test('nested generic types parse and empty type argument lists reject', () => {
  const ast = parseSuccessorSource(probe('  asset book: Map<Debt<USD>, Pair<A, B>>;\n'));
  const decl = ast.agreement.declarations[0];
  assert.equal(decl.type.name, 'Map');
  assert.equal(decl.type.arguments[0].name, 'Debt');
  assert.equal(decl.type.arguments[0].arguments[0].name, 'USD');
  assert.equal(decl.type.arguments[1].name, 'Pair');
  assert.equal(decl.type.arguments[1].arguments.length, 2);
  assertSyntaxError(() => parseSuccessorSource(probe('  asset book: Map<>;\n')), 'EMPTY_TYPE_ARGS');
});

test('UTF-8 spans cover multi-byte string contents and comments may contain Unicode', () => {
  const source = `profile "moriarty-successor-syntax/0";\nagreement Cafe {\n  // café\n  const note: Text = "é😀";\n}\n`;
  const ast = parseSuccessorSource(source);
  const decl = ast.agreement.declarations[0];
  const bytes = Buffer.from(source, 'utf8');
  const slice = bytes.subarray(decl.value.span.start, decl.value.span.end).toString('utf8');
  assert.equal(slice, '"é😀"');
  assert.equal(decl.value.decoded, 'é😀');
  assert.equal(decl.value.span.end - decl.value.span.start, Buffer.byteLength('"é😀"', 'utf8'));
});

test('lone surrogates, escaped surrogates and invalid UTF-8 reject', () => {
  assertSyntaxError(() => parseSuccessorSource('profile "\uD800";'), 'INVALID_UTF8');
  assertSyntaxError(() => parseSuccessorSource(probe('  const note: Text = "\\uD800";\n')), 'INVALID_SURROGATE');
  const tmp = join(mkdtempSync(join(tmpdir(), 'mori-syn-')), 'bad.mori');
  writeFileSync(tmp, Buffer.from([0xff, 0xfe, 0x01]));
  const result = runCli(['check-syntax', tmp]);
  assert.notEqual(result.status, 0);
  assert.match(decode(result.stderr), /INVALID_UTF8/);
});

test('leading zeros, plain division, floats and unknown declarations reject', () => {
  assertSyntaxError(() => parseSuccessorSource(actionProbe('    let x = 01;\n')), 'INTEGER_TOKEN');
  assertSyntaxError(() => parseSuccessorSource(actionProbe('    let x = 1 / 2;\n')), 'UNEXPECTED_CHAR');
  assertSyntaxError(() => parseSuccessorSource(actionProbe('    let x = 1.5;\n')));
  assertSyntaxError(() => parseSuccessorSource(probe('  obligation due: Debt<USD>;\n')), 'UNKNOWN_DECLARATION');
  assertSyntaxError(() => parseSuccessorSource(probe('  function helper() {}\n')), 'UNKNOWN_DECLARATION');
  assertSyntaxError(() => parseSuccessorSource(probe('  import other;\n')), 'UNKNOWN_DECLARATION');
});

test('statement after ensures, trailing input, missing agreement and old atomic profile reject', () => {
  assertSyntaxError(() => parseSuccessorSource(actionProbe('    ensures true;\n    requires true;\n')), 'STATEMENT_AFTER_ENSURES');
  assertSyntaxError(() => parseSuccessorSource(probe('  unit USD;\n') + ' leftover'), 'TRAILING_INPUT');
  assertSyntaxError(() => parseSuccessorSource('profile "moriarty-successor-syntax/0";\n'));
  const atomic = 'agreement Generic profile "moriarty-bounded-atomic/1" { lifetime 2; horizon 2000000000; unit USD; }';
  assertSyntaxError(() => parseSuccessorSource(atomic));
  assertSyntaxError(() => parseSuccessorSource('profile "moriarty-bounded-atomic/1";\nagreement X {\n}\n'), 'PROFILE_MISMATCH');
});

test('non-ASCII identifier lookalikes reject while ASCII identifiers stay case-sensitive', () => {
  assertSyntaxError(() => parseSuccessorSource(probe('  unit US𝐷;\n')), 'NON_ASCII_IDENTIFIER');
  const ast = parseSuccessorSource(probe('  unit usd;\n  unit USD;\n'));
  assert.deepEqual(ast.agreement.declarations.map((d) => d.name), ['usd', 'USD']);
});

test('bounds from syntax-profile.json are enforced before oversized parse results', () => {
  const profile = JSON.parse(readFileSync(profilePath, 'utf8'));
  assert.equal(SYNTAX_PROFILE, profile.profile);
  assert.equal(SYNTAX_BOUNDS.sourceUtf8Bytes, profile.bounds.sourceUtf8Bytes);
  assert.equal(SYNTAX_BOUNDS.identifierAsciiCharacters, profile.bounds.identifierAsciiCharacters);
  assert.equal(SYNTAX_BOUNDS.decodedStringUtf8Bytes, profile.bounds.decodedStringUtf8Bytes);
  assert.equal(SYNTAX_BOUNDS.integerTokenDigits, profile.bounds.integerTokenDigits);
  assert.equal(SYNTAX_BOUNDS.tokenCount, profile.bounds.tokenCount);
  assert.equal(SYNTAX_BOUNDS.astNodes, profile.bounds.astNodes);
  assert.equal(SYNTAX_BOUNDS.nestingDepth, profile.bounds.nestingDepth);
  assert.equal(SYNTAX_BOUNDS.totalDeclarations, profile.bounds.totalDeclarations);
  assert.equal(SYNTAX_BOUNDS.statementsPerAction, profile.bounds.statementsPerAction);
  assert.equal(SYNTAX_BOUNDS.callArguments, profile.bounds.callArguments);

  assertSyntaxError(() => parseSuccessorSource('a'.repeat(profile.bounds.sourceUtf8Bytes + 1)), 'SOURCE_BOUND');
  assertSyntaxError(() => parseSuccessorSource(probe(`  unit ${'A'.repeat(profile.bounds.identifierAsciiCharacters + 1)};\n`)), 'IDENTIFIER_BOUND');
  assertSyntaxError(() => parseSuccessorSource(probe(`  const note: Text = "${'n'.repeat(profile.bounds.decodedStringUtf8Bytes + 1)}";\n`)), 'STRING_BOUND');
  assertSyntaxError(() => parseSuccessorSource(actionProbe(`    let x = ${'1'.repeat(profile.bounds.integerTokenDigits + 1)};\n`)), 'INTEGER_BOUND');

  const units = Array.from({ length: profile.bounds.totalDeclarations + 1 }, (_, i) => `  unit U${i};`).join('\n');
  assertSyntaxError(() => parseSuccessorSource(probe(units)), 'DECLARATION_BOUND');

  const statements = Array.from({ length: profile.bounds.statementsPerAction + 1 }, (_, i) => `    let x${i} = 1;`).join('\n');
  assertSyntaxError(() => parseSuccessorSource(actionProbe(statements)), 'STATEMENT_BOUND');

  const args = Array.from({ length: profile.bounds.callArguments + 1 }, () => '1').join(', ');
  assertSyntaxError(() => parseSuccessorSource(actionProbe(`    let x = f(${args});\n`)), 'ARITY_BOUND');

  const nots = 'not '.repeat(profile.bounds.nestingDepth + 1);
  assertSyntaxError(() => parseSuccessorSource(actionProbe(`    let x = ${nots}true;\n`)), 'NESTING_BOUND');

  const fields = '.f'.repeat(profile.bounds.nestingDepth + 1);
  assertSyntaxError(() => parseSuccessorSource(actionProbe(`    let x = a${fields};\n`)), 'NESTING_BOUND');

  const open = 'Box<'.repeat(profile.bounds.nestingDepth + 1);
  const close = '>'.repeat(profile.bounds.nestingDepth + 1);
  assertSyntaxError(() => parseSuccessorSource(probe(`  asset x: ${open}T${close};\n`)), 'NESTING_BOUND');

  const manyAdds = Array.from({ length: 4500 }, () => '1').join('+');
  const flood = actionProbe(`    let x = ${manyAdds};\n`);
  assert.ok(Buffer.byteLength(flood, 'utf8') < profile.bounds.sourceUtf8Bytes);
  assertSyntaxError(() => parseSuccessorSource(flood), 'TOKEN_BOUND');
});

test('formatter rejects canonical expansion past the source byte bound', () => {
  const declCount = 60;
  const name = `n${'x'.repeat(63)}`;
  const literal = 'a'.repeat(1012);
  assert.equal(name.length, 64);
  assert.equal(literal.length, 1012);
  assert.ok(name.length <= SYNTAX_BOUNDS.identifierAsciiCharacters);
  assert.ok(literal.length <= SYNTAX_BOUNDS.decodedStringUtf8Bytes);
  assert.ok(declCount <= SYNTAX_BOUNDS.totalDeclarations);

  const compactDecl = `const ${name}:Text="${literal}";`;
  assert.equal(compactDecl.length, 5 + 1 + 64 + 1 + 4 + 1 + 1 + 1012 + 1 + 1);
  const prefix = 'profile "moriarty-successor-syntax/0";agreement Expand{';
  const suffix = '}';
  assert.equal(prefix.length, 55);
  const compact = `${prefix}${compactDecl.repeat(declCount)}${suffix}`;
  const compactBytes = prefix.length + compactDecl.length * declCount + suffix.length;
  assert.equal(Buffer.byteLength(compact, 'utf8'), compactBytes);
  assert.ok(compactBytes <= SYNTAX_BOUNDS.sourceUtf8Bytes);

  const spacedDecl = `const ${name}:Text = "${literal}";`;
  assert.equal(spacedDecl.length, compactDecl.length + 2);
  const spacedBytes = prefix.length + spacedDecl.length * declCount + suffix.length;
  assert.equal(spacedBytes, compactBytes + 2 * declCount);
  assert.ok(spacedBytes > SYNTAX_BOUNDS.sourceUtf8Bytes);

  parseSuccessorSource(compact);
  assertSyntaxError(() => formatSuccessorSource(compact), 'FORMAT_BOUND');
});

test('left-associative expression depth 64 succeeds and 65 fails', () => {
  const sum64 = Array.from({ length: 64 }, () => '1').join('+');
  parseSuccessorSource(actionProbe(`    let x = ${sum64};\n`));
  const sum65 = Array.from({ length: 65 }, () => '1').join('+');
  assertSyntaxError(() => parseSuccessorSource(actionProbe(`    let x = ${sum65};\n`)), 'NESTING_BOUND');
});

test('256 statements total across regular and ensures succeed and 257 fail', () => {
  const lets255 = Array.from({ length: 255 }, (_, i) => `    let x${i} = 1;`).join('\n');
  parseSuccessorSource(actionProbe(`${lets255}\n    ensures true;\n`));
  const lets256 = Array.from({ length: 256 }, (_, i) => `    let x${i} = 1;`).join('\n');
  assertSyntaxError(() => parseSuccessorSource(actionProbe(`${lets256}\n    ensures true;\n`)), 'STATEMENT_BOUND');
  const ensures256 = Array.from({ length: 256 }, () => '    ensures true;').join('\n');
  parseSuccessorSource(actionProbe(`${ensures256}\n`));
  const ensures257 = Array.from({ length: 257 }, () => '    ensures true;').join('\n');
  assertSyntaxError(() => parseSuccessorSource(actionProbe(`${ensures257}\n`)), 'STATEMENT_BOUND');
});

test('token bound includes EOF so 8192 total tokens succeed and 8193 fail', () => {
  const argCount = 44;
  const stmtCount = 87;
  const wrapperTokens = 13;
  const tokensPerStmt = 6 + 2 * argCount;
  const nonEofAccepted = wrapperTokens + stmtCount * tokensPerStmt;
  assert.equal(nonEofAccepted, SYNTAX_BOUNDS.tokenCount - 1);
  assert.equal(nonEofAccepted + 1, SYNTAX_BOUNDS.tokenCount);
  const args = Array.from({ length: argCount }, () => '1').join(',');
  const stmt = `let x=f(${args});`;
  const source = `profile "moriarty-successor-syntax/0";agreement T{action r(){${stmt.repeat(stmtCount)}}}`;
  parseSuccessorSource(source);
  const over = `profile "moriarty-successor-syntax/0";agreement T{action r(){let x=not f(${args});${stmt.repeat(stmtCount - 1)}}}`;
  assertSyntaxError(() => parseSuccessorSource(over), 'TOKEN_BOUND');
});

test('unterminated comments and strings reject, and comments count toward the source bound', () => {
  assertSyntaxError(() => parseSuccessorSource(probe('  /* never ends')), 'UNTERMINATED_COMMENT');
  assertSyntaxError(() => parseSuccessorSource('profile "moriarty-successor-syntax/0";agreement Probe{const note:Text="abc'), 'UNTERMINATED_STRING');
  assertSyntaxError(() => parseSuccessorSource(probe('  const note: Text = "abc')), 'INVALID_STRING');
  const comment = `profile "moriarty-successor-syntax/0";\nagreement X {\n/* ${'x'.repeat(SYNTAX_BOUNDS.sourceUtf8Bytes)} */\n}\n`;
  assertSyntaxError(() => parseSuccessorSource(comment), 'SOURCE_BOUND');
});

test('CLI check-syntax writes AST JSON and format writes idempotent source', () => {
  const checked = runCli(['check-syntax', examplePath]);
  assert.equal(checked.status, 0, decode(checked.stderr));
  const ast = JSON.parse(decode(checked.stdout));
  assert.equal(ast.tag, 'Program');
  const formatted = runCli(['format', examplePath]);
  assert.equal(formatted.status, 0, decode(formatted.stderr));
  const text = decode(formatted.stdout);
  assert.equal(formatSuccessorSource(text), text);
  const again = runCli(['format', examplePath]);
  assert.equal(decode(again.stdout), text);
});

test('CLI rejects malformed arguments without writing files', () => {
  const noArgs = runCli([]);
  assert.notEqual(noArgs.status, 0);
  assert.match(decode(noArgs.stderr), /CLI_USAGE/);
  const unknown = runCli(['compile', examplePath]);
  assert.notEqual(unknown.status, 0);
  assert.match(decode(unknown.stderr), /CLI_USAGE/);
  const missing = runCli(['check-syntax']);
  assert.notEqual(missing.status, 0);
  assert.match(decode(missing.stderr), /CLI_USAGE/);
  const extra = runCli(['check-syntax', examplePath, 'other.mori']);
  assert.notEqual(extra.status, 0);
  assert.match(decode(extra.stderr), /CLI_USAGE/);
  const absent = runCli(['check-syntax', join(here, 'no-such-file.mori')]);
  assert.notEqual(absent.status, 0);
});
