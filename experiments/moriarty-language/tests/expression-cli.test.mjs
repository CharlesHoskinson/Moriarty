import test, { after } from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, writeFileSync, rmSync, symlinkSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import { createExpressionSourceV1 } from '../src/successor/expression-source-v1.ts';
import { formatExpressionSource } from '../src/successor/expression-source-frontend.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';
import { createFinancialExpressionSourceV1 } from '../src/successor/financial-expression-source-v1.ts';
import { formatFinancialExpressionSource } from '../src/successor/financial-expression-source-frontend.ts';

const cli = fileURLToPath(new URL('../src/cli.ts', import.meta.url));
const directory = mkdtempSync(join(tmpdir(), 'moriarty-expression-cli-'));
after(() => rmSync(directory, { recursive: true, force: true }));
let serial = 0;
const file = content => {
  const name = join(directory, `input ${serial++}`);
  writeFileSync(name, content);
  return name;
};
const schema = canonical({ units: [], assets: [], vaults: [], parties: [], recordTypes: {}, enumTypes: {},
  fields: {}, args: {}, observations: {}, operations: {} });
const profile = 'moriarty-expression-source/1';
const source = body => `profile "${profile}"; agreement Demo { action step() { ${body} } }`;
const schemaPath = file(schema);
const invoke = args => {
  const result = spawnSync(process.execPath, [cli, ...args], { encoding: 'utf8', timeout: 5000 });
  assert.equal(result.error, undefined);
  assert.equal(result.signal, null);
  return result;
};
const checkArgs = (input, schemaFile = schemaPath) => ['check', '--profile', profile, '--schema', schemaFile, input];
const formatArgs = input => ['format', '--profile', profile, input];
const simulateArgs = (selectedProfile, selectedSchema, snapshots, input) =>
  ['simulate', '--profile', selectedProfile, '--schema', selectedSchema, '--snapshots', snapshots, input];
const failure = (args, expected, exit = 1) => {
  const result = invoke(args);
  assert.equal(result.status, exit, result.stderr);
  assert.equal(result.stdout, '');
  assert.equal(result.stderr, JSON.stringify(expected) + '\n');
};
const cliFailure = (args, code, input, exit = 2) => failure(args, { status: 'CliRejected', code, input }, exit);

test('expression CLI check equals successful API and leaves source/schema bytes intact', () => {
  const input = source('let label = "é😀"; requires true;');
  const path = file(input);
  const result = invoke(checkArgs(path));
  assert.equal(result.status, 0, result.stderr);
  assert.equal(result.stderr, '');
  assert.equal(result.stdout, JSON.stringify(createExpressionSourceV1(schema).check(input)) + '\n');
  assert.equal(readFileSync(path, 'utf8'), input);
  assert.equal(readFileSync(schemaPath, 'utf8'), schema);
});

test('expression CLI preserves complete API failure records including R1 and UTF-8 spans', () => {
  for (const input of [source('let label="é😀"; requires true or 1;'),
    source('let label="é😀"; let value=access_field(unknown(),"bad field");'),
    source('requires false; requires 1;'), source('requires ;'),
    source('requires true;').replace(profile, 'moriarty-successor-syntax/0')]) {
    failure(checkArgs(file(input)), createExpressionSourceV1(schema).check(input));
  }
});

test('expression CLI format equals API, is idempotent and writes no input files', () => {
  const input = source('let label="é😀";let xs=collection<UInt64,2>(u64(1));requires xs[u64(0)]==u64(1);');
  const path = file(input);
  const result = invoke(formatArgs(path));
  assert.equal(result.status, 0, result.stderr);
  assert.equal(result.stderr, '');
  assert.equal(result.stdout, formatExpressionSource(input));
  assert.equal(invoke(formatArgs(file(result.stdout))).stdout, result.stdout);
  assert.equal(readFileSync(path, 'utf8'), input);
  // Formatting checks syntax; an unbound name does not require a trusted schema.
  assert.equal(invoke(formatArgs(file(source('let x=unknown();')))).status, 0);
});

test('expression CLI format preserves parser code/span with no partial output', () => {
  const input = source('let label="é😀"; requires ;');
  let error;
  try { formatExpressionSource(input); } catch (caught) { error = caught; }
  assert.ok(error);
  failure(formatArgs(file(input)), { status: 'Rejected', code: error.code,
    span: { kind: 'source', start: String(error.start), end: String(error.end) }, nodePath: [], workUsed: '0' });
});

test('expression CLI format rejects output expansion beyond the bound without partial stdout', () => {
  const body = Array.from({ length: 64 }, (_, i) => `let v${i}="${'a'.repeat(1006)}";`).join('');
  const input = `profile "${profile}";agreement Demo{action step(){${body}}}`;
  assert.equal(Buffer.byteLength(input), 65148);
  assert.throws(() => formatExpressionSource(input), { code: 'FORMAT_BOUND', start: 0, end: 0 });
  failure(formatArgs(file(input)), { status: 'Rejected', code: 'FORMAT_BOUND',
    span: { kind: 'source', start: '0', end: '0' }, nodePath: [], workUsed: '0' });
});

test('expression CLI rejects malformed invocation before reading paths', () => {
  for (const args of [[], ['check'], ['format', '--profile', profile], ['check', '--profile', profile, 'missing'],
    ['evaluate', '--profile', profile, 'missing'], [...formatArgs('missing'), 'extra'],
    ['format', '--profile', profile, '--schema', 'missing', 'missing'],
    ['check', '--schema', 'missing', '--profile', profile, 'missing'],
    ['check', '--profile', profile, '--profile', profile, 'missing'],
    ['format', '--profile', profile, ''], ['format', '--profile', profile, '--bad']]) {
    cliFailure(args, 'CLI_USAGE', 'arguments');
  }
  cliFailure(['format', '--profile', 'unknown', 'missing'], 'CLI_PROFILE', 'arguments');
});

test('expression CLI schema errors equal API after bounded transport', () => {
  const input = source('requires true;');
  for (const text of ['{}', schema + '\n', '{', '\uFEFF' + schema]) {
    failure(checkArgs(file(input), file(text)), createExpressionSourceV1(text).check(input));
  }
});

test('expression CLI bounds distinguish 65536 from 65537 bytes for source/schema', () => {
  const base = source('requires true;');
  const exact = base + ' '.repeat(65536 - Buffer.byteLength(base));
  assert.equal(invoke(checkArgs(file(exact))).status, 0);
  assert.equal(invoke(formatArgs(file(exact))).status, 0);
  for (const args of [checkArgs(file(exact + ' ')), formatArgs(file(exact + ' ')), formatArgs(file('é'.repeat(32768) + 'a'))]) {
    cliFailure(args, 'SOURCE_BOUND', 'source', 1);
  }
  const exactSchema = schema + ' '.repeat(65536 - Buffer.byteLength(schema));
  failure(checkArgs(file(base), file(exactSchema)), createExpressionSourceV1(exactSchema).check(base));
  cliFailure(checkArgs('missing', file(exactSchema + ' ')), 'INPUT_BOUND', 'schema', 1);
});

test('expression CLI rejects invalid UTF-8 before parser and retains BOM for parser', () => {
  for (const args of [formatArgs(file(new Uint8Array([0xc3, 0x28]))), checkArgs(file(new Uint8Array([0xff])))]) {
    cliFailure(args, 'INVALID_UTF8', 'source', 1);
  }
  cliFailure(checkArgs('missing', file(new Uint8Array([0xff]))), 'INVALID_UTF8', 'schema', 1);
  const input = '\uFEFF' + source('requires true;');
  failure(checkArgs(file(input)), createExpressionSourceV1(schema).check(input));
});

test('expression CLI opens only bounded regular files and allows regular symlink targets', () => {
  cliFailure(formatArgs(join(directory, 'missing')), 'CLI_IO', 'source');
  cliFailure(formatArgs(directory), 'CLI_IO', 'source');
  cliFailure(checkArgs('missing', directory), 'CLI_IO', 'schema');
  const target = file(source('requires true;'));
  const link = join(directory, 'regular-link');
  symlinkSync(target, link);
  assert.equal(invoke(checkArgs(link)).status, 0);
  if (process.platform !== 'win32') {
    const fifo = join(directory, 'fifo');
    const made = spawnSync('mkfifo', [fifo], { encoding: 'utf8' });
    assert.equal(made.status, 0, made.stderr);
    cliFailure(formatArgs(fifo), 'CLI_IO', 'source');
    cliFailure(formatArgs('/dev/null'), 'CLI_IO', 'source');
  }
});

test('expression CLI import has no command or stream side effects', () => {
  const result = spawnSync(process.execPath, ['--input-type=module', '-e', 'await import(process.argv[1]);', new URL('../src/cli.ts', import.meta.url).href], { encoding: 'utf8', timeout: 5000 });
  assert.equal(result.status, 0, result.stderr);
  assert.equal(result.stdout, '');
  assert.equal(result.stderr, '');
});

test('financial CLI explicit routing equals financial APIs and never infers the profile', () => {
  const financial='moriarty-financial-expression-source/1';
  const text=canonical({...JSON.parse(schema),variantTypes:{}}),schemaFile=file(text),language=createFinancialExpressionSourceV1(text);
  for(const body of ['let x=true ? u256(7) : u256(8);','let x=false ? 7 : u64(8);','let x=some_value(none<UInt128>());']){
    const s=source(body).replace(profile,financial),path=file(s);
    const args=['check','--profile',financial,'--schema',schemaFile,path], expected=language.check(s);
    if('status' in expected) failure(args,expected);
    else {const result=invoke(args);assert.equal(result.status,0,result.stderr);assert.equal(result.stdout,JSON.stringify(expected)+'\n');assert.equal(result.stderr,'');}
    const formatted=invoke(['format','--profile',financial,path]);
    assert.equal(formatted.status,0,formatted.stderr);assert.equal(formatted.stdout,formatFinancialExpressionSource(s));
    failure(checkArgs(path),createExpressionSourceV1(schema).check(s));
  }
  const old=source('requires true;');
  failure(['check','--profile',financial,'--schema',schemaFile,file(old)],language.check(old));
});

test('financial CLI preserves checking and formatting meaning for the published vault quote fixture', () => {
  const financial = 'moriarty-financial-expression-source/1';
  const fixture = fileURLToPath(new URL('../spec/successor/examples/financial-vault-quote.mori', import.meta.url));
  const schemaFixture = fileURLToPath(new URL('../spec/successor/examples/financial-vault-quote.schema.json', import.meta.url));
  const input = readFileSync(fixture, 'utf8');
  const schemaText = readFileSync(schemaFixture, 'utf8');
  const language = createFinancialExpressionSourceV1(schemaText);
  const expected = language.check(input);
  const checked = invoke(['check', '--profile', financial, '--schema', schemaFixture, fixture]);
  assert.equal(checked.status, 0, checked.stderr);
  assert.equal(checked.stderr, '');
  assert.equal(checked.stdout, JSON.stringify(expected) + '\n');

  const formatted = invoke(['format', '--profile', financial, fixture]);
  assert.equal(formatted.status, 0, formatted.stderr);
  assert.equal(formatted.stderr, '');
  assert.equal(formatted.stdout, formatFinancialExpressionSource(input));
  const formattedPath = file(formatted.stdout);
  const rechecked = invoke(['check', '--profile', financial, '--schema', schemaFixture, formattedPath]);
  assert.equal(rechecked.status, 0, rechecked.stderr);
  assert.equal(rechecked.stdout, JSON.stringify(language.check(formatted.stdout)) + '\n');
  assert.equal(invoke(['format', '--profile', financial, formattedPath]).stdout, formatted.stdout);
});

test('simulation CLI returns independently derived local financial and original expression results', () => {
  const financial = 'moriarty-financial-expression-source/1';
  const financialSource = fileURLToPath(new URL('../spec/successor/examples/financial-vault-quote.mori', import.meta.url));
  const financialSchema = fileURLToPath(new URL('../spec/successor/examples/financial-vault-quote.schema.json', import.meta.url));
  const financialSnapshots = fileURLToPath(new URL('../spec/successor/examples/financial-vault-quote.snapshots.json', import.meta.url));
  const financialResult = invoke(simulateArgs(financial, financialSchema, financialSnapshots, financialSource));
  assert.equal(financialResult.status, 0, financialResult.stderr);
  assert.equal(financialResult.stderr, '');
  assert.deepEqual(JSON.parse(financialResult.stdout), {
    judgmentResult: 'SourceSimulated', sourceProfile: financial, pre: { last: '0' }, initialWork: '1000',
    result: { status: 'ExpressionPrepared', post: { last: '1' },
      descriptors: [{ operation: 'Notice', fields: { allocation: '1', offered: '4' } }], workRemaining: '959' },
  });

  const originalSource = fileURLToPath(new URL('../spec/successor/examples/expression-counter.mori', import.meta.url));
  const originalSchema = fileURLToPath(new URL('../spec/successor/examples/expression-counter.schema.json', import.meta.url));
  const originalSnapshots = fileURLToPath(new URL('../spec/successor/examples/expression-counter.snapshots.json', import.meta.url));
  const originalResult = invoke(simulateArgs(profile, originalSchema, originalSnapshots, originalSource));
  assert.equal(originalResult.status, 0, originalResult.stderr);
  assert.equal(originalResult.stderr, '');
  assert.deepEqual(JSON.parse(originalResult.stdout), {
    judgmentResult: 'SourceSimulated', sourceProfile: profile,
    pre: { counter: '10', funds: '100' }, initialWork: '1000',
    result: { status: 'ExpressionPrepared', post: { counter: '12', funds: '100' },
      descriptors: [{ operation: 'QuoteNotice', fields: { amount: '5', count: '2' } }], workRemaining: '960' },
  });
});

test('simulation CLI preserves source API rejection records and never prints snapshot state', () => {
  const financial = 'moriarty-financial-expression-source/1';
  const sourcePath = fileURLToPath(new URL('../spec/successor/examples/financial-vault-quote.mori', import.meta.url));
  const schemaPath = fileURLToPath(new URL('../spec/successor/examples/financial-vault-quote.schema.json', import.meta.url));
  const sourceText = readFileSync(sourcePath, 'utf8');
  const schemaText = readFileSync(schemaPath, 'utf8');
  const initial = JSON.parse(readFileSync(new URL('../spec/successor/examples/financial-vault-quote.snapshots.json', import.meta.url), 'utf8'));
  const language = createFinancialExpressionSourceV1(schemaText);
  const cases = [];

  const absent = { ...initial, Args: { ...initial.Args, useSupplied: true } };
  cases.push(canonical(absent));
  cases.push(canonical({ ...initial, workInitial: '40' }));
  const failingEnsure = sourceText.replace('ensures post.last == selected;', 'ensures post.last == deposit;');
  const failingEnsurePath = file(failingEnsure);
  const rollback = language.evaluate(failingEnsure, canonical(initial));
  assert.equal(rollback.status, 'Rejected');
  assert.equal('post' in rollback, false);
  assert.equal('descriptors' in rollback, false);
  failure(simulateArgs(financial, schemaPath, file(canonical(initial)), failingEnsurePath), rollback);

  for (const snapshot of cases) {
    const expected = language.evaluate(sourceText, snapshot);
    assert.equal(expected.status, 'Rejected');
    assert.equal('post' in expected, false);
    assert.equal('descriptors' in expected, false);
    const result = invoke(simulateArgs(financial, schemaPath, file(snapshot), sourcePath));
    assert.equal(result.status, 1, result.stderr);
    assert.equal(result.stdout, '');
    assert.equal(result.stderr, JSON.stringify(expected) + '\n');
    assert.equal(result.stderr.includes('last'), false);
    assert.equal(result.stderr.includes('Notice'), false);
  }
});

test('simulation CLI retains snapshot transport limits and canonical handling', () => {
  const sourcePath = fileURLToPath(new URL('../spec/successor/examples/expression-counter.mori', import.meta.url));
  const schemaPath = fileURLToPath(new URL('../spec/successor/examples/expression-counter.schema.json', import.meta.url));
  const snapshotText = readFileSync(new URL('../spec/successor/examples/expression-counter.snapshots.json', import.meta.url), 'utf8');
  const language = createExpressionSourceV1(readFileSync(schemaPath, 'utf8'));
  for (const text of [snapshotText + '\n', '\uFEFF' + snapshotText,
    '{"Args":{"delta":2},"Obs":{"ready":true},"Pre":{"counter":10,"funds":"100"},"workInitial":"1000"}',
    ' '.repeat(65537)]) {
    const expected = language.evaluate(readFileSync(sourcePath, 'utf8'), text);
    failure(simulateArgs(profile, schemaPath, file(text), sourcePath), expected);
  }
  cliFailure(simulateArgs(profile, schemaPath, file(' '.repeat(2_000_001)), sourcePath), 'INPUT_BOUND', 'snapshots', 1);
  cliFailure(simulateArgs('unknown', 'missing-schema', 'missing-snapshots', 'missing-source'), 'CLI_PROFILE', 'arguments');
  cliFailure(['simulate', '--profile', profile, '--snapshots', 'missing', '--schema', 'missing', 'missing'], 'CLI_USAGE', 'arguments');
  cliFailure(simulateArgs(profile, schemaPath, directory, sourcePath), 'CLI_IO', 'snapshots');
  cliFailure(simulateArgs(profile, schemaPath, file(new Uint8Array([0xff])), sourcePath), 'INVALID_UTF8', 'snapshots', 1);
});
