/** Profile-explicit file adapter for expression-source/1 checking, formatting, and local simulation. */
import * as fs from 'node:fs';
import * as path from 'node:path';
import { fileURLToPath } from 'node:url';
import { createExpressionSourceV1 } from './successor/expression-source-v1.ts';
import { EXPRESSION_SOURCE_PROFILE, formatExpressionSource } from './successor/expression-source-frontend.ts';
import { createFinancialExpressionSourceV1 } from './successor/financial-expression-source-v1.ts';
import { FINANCIAL_EXPRESSION_SOURCE_PROFILE, formatFinancialExpressionSource } from './successor/financial-expression-source-frontend.ts';
import { createFundedFinancialExpressionSourceV1 } from './successor/funded-expression-source-v1.ts';
import { createFinancialAgreementSourceV1 } from './successor/financial-agreement-source-v1.ts';
import { FINANCIAL_AGREEMENT_SOURCE_PROFILE, formatFinancialAgreementSource } from './successor/financial-agreement-source-frontend.ts';
import { createFinancialAgreementSourceV2 } from './successor/financial-agreement-source-v2.ts';
import { FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE, formatFinancialAgreementSourceV2 } from './successor/financial-agreement-source-v2-frontend.ts';
import { SuccessorSyntaxError } from './successor/frontend.ts';
import { parseCanonical } from './successor/expression-wire-v1.ts';

type Input = 'arguments' | 'schema' | 'source' | 'snapshots' | 'repayment-state';
type ProcessIo = { argv: string[]; exitCode: number;
  stdout: { write(value: string): unknown }; stderr: { write(value: string): unknown } };
class CliFailure extends Error {
  readonly code: string;
  readonly input: Input;
  readonly exitCode: number;
  constructor(code: string, input: Input, exitCode = 2) {
    super(code);
    this.code = code;
    this.input = input;
    this.exitCode = exitCode;
  }
}
const LIMIT = 65536;
const SNAPSHOT_LIMIT = 2_000_000;

function argumentsFor(argv: string[]): {
  command: 'check' | 'format' | 'simulate'; profile: string; source: string;
  schema?: string; snapshots?: string; repaymentState?: string; action?: string;
} {
  const [command, flag, profile, ...rest] = argv;
  const fileName = (value: string | undefined): value is string =>
    typeof value === 'string' && value.length > 0 && !value.startsWith('--');
  const checking = command === 'check' && rest.length === 3 && rest[0] === '--schema'
    && fileName(rest[1]) && fileName(rest[2]);
  const checkingBare = command === 'check' && rest.length === 1 && fileName(rest[0]);
  const formatting = command === 'format' && rest.length === 1 && fileName(rest[0]);
  const simulating = command === 'simulate' && rest.length === 5 && rest[0] === '--schema'
    && fileName(rest[1]) && rest[2] === '--snapshots' && fileName(rest[3]) && fileName(rest[4]);
  const funded = command === 'simulate' && rest.length === 7 && rest[0] === '--schema'
    && fileName(rest[1]) && rest[2] === '--snapshots' && fileName(rest[3])
    && rest[4] === '--repayment-state' && fileName(rest[5]) && fileName(rest[6]);
  const agreementSimulate = command === 'simulate' && rest.length === 5 && rest[0] === '--snapshots'
    && fileName(rest[1]) && rest[2] === '--repayment-state' && fileName(rest[3]) && fileName(rest[4]);
  const agreementV2Simulate = command === 'simulate' && rest.length === 7 && rest[0] === '--action'
    && fileName(rest[1]) && rest[2] === '--snapshots' && fileName(rest[3])
    && rest[4] === '--repayment-state' && fileName(rest[5]) && fileName(rest[6]);
  if (flag !== '--profile' || !profile
    || (!checking && !checkingBare && !formatting && !simulating && !funded
      && !agreementSimulate && !agreementV2Simulate))
    throw new CliFailure('CLI_USAGE', 'arguments');
  const known = [
    EXPRESSION_SOURCE_PROFILE, FINANCIAL_EXPRESSION_SOURCE_PROFILE,
    FINANCIAL_AGREEMENT_SOURCE_PROFILE, FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE,
  ];
  if (!known.includes(profile)) throw new CliFailure('CLI_PROFILE', 'arguments');
  if (profile === FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE) {
    if (checking || simulating || funded || agreementSimulate
      || (command === 'simulate' && !agreementV2Simulate)
      || (command === 'check' && !checkingBare)) {
      throw new CliFailure('CLI_USAGE', 'arguments');
    }
  } else if (profile === FINANCIAL_AGREEMENT_SOURCE_PROFILE) {
    if (checking || simulating || funded || agreementV2Simulate
      || (command === 'simulate' && !agreementSimulate)
      || (command === 'check' && !checkingBare)) {
      throw new CliFailure('CLI_USAGE', 'arguments');
    }
  } else if (checkingBare || agreementSimulate || agreementV2Simulate
    || (funded && profile !== FINANCIAL_EXPRESSION_SOURCE_PROFILE)) {
    throw new CliFailure('CLI_USAGE', 'arguments');
  }
  return checking ? { command: 'check', profile, schema: rest[1], source: rest[2] }
    : checkingBare ? { command: 'check', profile, source: rest[0] }
    : funded ? { command: 'simulate', profile, schema: rest[1], snapshots: rest[3], repaymentState: rest[5], source: rest[6] }
    : agreementV2Simulate ? {
      command: 'simulate', profile, action: rest[1], snapshots: rest[3], repaymentState: rest[5], source: rest[6],
    }
    : agreementSimulate ? { command: 'simulate', profile, snapshots: rest[1], repaymentState: rest[3], source: rest[4] }
    : simulating ? { command: 'simulate', profile, schema: rest[1], snapshots: rest[3], source: rest[4] }
    : { command: 'format', profile, source: rest[0] };
}

/** Same bounded descriptor/type check as syntax-cli.ts; never reads a FIFO. */
function readText(file: string, input: 'schema' | 'source' | 'snapshots' | 'repayment-state', limit = LIMIT): string {
  let fd: number | undefined;
  try {
    fd = fs.openSync(path.resolve(file), fs.constants.O_RDONLY | (fs.constants.O_NONBLOCK ?? 0));
    if (!fs.fstatSync(fd).isFile()) throw new CliFailure('CLI_IO', input);
    const buffer = new Uint8Array(limit + 1);
    let got = 0;
    while (got < buffer.length) {
      const count = fs.readSync(fd, buffer, got, buffer.length - got, got);
      if (count === 0) break;
      if (count < 0) throw new CliFailure('CLI_IO', input);
      got += count;
    }
    if (got > limit) throw new CliFailure(input === 'source' ? 'SOURCE_BOUND' : 'INPUT_BOUND', input, 1);
    try {
      return new TextDecoder('utf-8', { fatal: true, ignoreBOM: true }).decode(buffer.subarray(0, got));
    } catch {
      throw new CliFailure('INVALID_UTF8', input, 1);
    }
  } catch (error) {
    if (error instanceof CliFailure) throw error;
    throw new CliFailure('CLI_IO', input);
  } finally {
    if (fd !== undefined) {
      try { fs.closeSync(fd); } catch { /* Closing does not replace the primary result. */ }
    }
  }
}

function run(io: ProcessIo): void {
  const args = argumentsFor(io.argv.slice(2));
  const schema = args.schema !== undefined ? readText(args.schema, 'schema') : undefined;
  const source = readText(args.source, 'source');
  if (args.command === 'format') {
    const formatted = args.profile === FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE
      ? formatFinancialAgreementSourceV2(source)
      : args.profile === FINANCIAL_AGREEMENT_SOURCE_PROFILE
        ? formatFinancialAgreementSource(source)
        : args.profile === FINANCIAL_EXPRESSION_SOURCE_PROFILE
          ? formatFinancialExpressionSource(source) : formatExpressionSource(source);
    io.stdout.write(formatted);
  } else {
    if (args.command === 'simulate') {
      const snapshots = readText(args.snapshots!, 'snapshots', SNAPSHOT_LIMIT);
      if (args.repaymentState !== undefined) {
        const repaymentState = readText(args.repaymentState, 'repayment-state');
        const result = args.profile === FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE
          ? createFinancialAgreementSourceV2().evaluate(source, args.action!, snapshots, repaymentState)
          : args.profile === FINANCIAL_AGREEMENT_SOURCE_PROFILE
            ? createFinancialAgreementSourceV1().evaluate(source, snapshots, repaymentState)
            : createFundedFinancialExpressionSourceV1(schema!).evaluate(source, snapshots, repaymentState);
        if ('status' in result && result.status === 'Rejected') {
          io.stderr.write(JSON.stringify(result) + '\n');
          io.exitCode = 1;
          return;
        }
        const snapshot = parseCanonical(snapshots, SNAPSHOT_LIMIT) as { Pre: unknown; workInitial: string };
        io.stdout.write(JSON.stringify({
          judgmentResult: 'SourceSimulated', sourceProfile: args.profile,
          ...(args.action !== undefined ? { action: args.action } : {}),
          pre: snapshot.Pre, financialPre: JSON.parse(repaymentState), initialWork: snapshot.workInitial, result,
        }) + '\n');
        io.exitCode = 0;
        return;
      }
      const language = args.profile === FINANCIAL_EXPRESSION_SOURCE_PROFILE
        ? createFinancialExpressionSourceV1(schema!) : createExpressionSourceV1(schema!);
      const result = language.evaluate(source, snapshots);
      if ('status' in result && result.status === 'Rejected') {
        io.stderr.write(JSON.stringify(result) + '\n');
        io.exitCode = 1;
        return;
      }
      const snapshot = parseCanonical(snapshots, SNAPSHOT_LIMIT) as { Pre: unknown; workInitial: string };
      io.stdout.write(JSON.stringify({ judgmentResult: 'SourceSimulated', sourceProfile: args.profile,
        pre: snapshot.Pre, initialWork: snapshot.workInitial, result }) + '\n');
      io.exitCode = 0;
      return;
    }
    const result = args.profile === FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE
      ? createFinancialAgreementSourceV2().check(source)
      : args.profile === FINANCIAL_AGREEMENT_SOURCE_PROFILE
        ? createFinancialAgreementSourceV1().check(source)
        : (args.profile === FINANCIAL_EXPRESSION_SOURCE_PROFILE
          ? createFinancialExpressionSourceV1(schema!) : createExpressionSourceV1(schema!)).check(source);
    if ('status' in result) {
      io.stderr.write(JSON.stringify(result) + '\n');
      io.exitCode = 1;
      return;
    }
    io.stdout.write(JSON.stringify(result) + '\n');
  }
  io.exitCode = 0;
}

function report(error: unknown, io: ProcessIo): void {
  if (error instanceof SuccessorSyntaxError) {
    io.stderr.write(JSON.stringify({ status: 'Rejected', code: error.code,
      span: { kind: 'source', start: String(error.start), end: String(error.end) },
      nodePath: [], workUsed: '0' }) + '\n');
    io.exitCode = 1;
    return;
  }
  const failure = error instanceof CliFailure ? error : new CliFailure('CLI_INTERNAL', 'arguments');
  io.stderr.write(JSON.stringify({ status: 'CliRejected', code: failure.code, input: failure.input }) + '\n');
  io.exitCode = failure.exitCode;
}

const entry = process.argv[1];
if (typeof entry === 'string' && path.resolve(entry) === path.resolve(fileURLToPath(import.meta.url))) {
  const io = process as unknown as ProcessIo;
  try { run(io); } catch (error) { report(error, io); }
}
