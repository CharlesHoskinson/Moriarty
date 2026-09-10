/** Profile-explicit file adapter for expression-source/1 checking and formatting. */
import * as fs from 'node:fs';
import * as path from 'node:path';
import { fileURLToPath } from 'node:url';
import { createExpressionSourceV1 } from './successor/expression-source-v1.ts';
import { EXPRESSION_SOURCE_PROFILE, formatExpressionSource } from './successor/expression-source-frontend.ts';
import { SuccessorSyntaxError } from './successor/frontend.ts';

type Input = 'arguments' | 'schema' | 'source';
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

function argumentsFor(argv: string[]): { command: 'check' | 'format'; source: string; schema?: string } {
  const [command, flag, profile, ...rest] = argv;
  const fileName = (value: string | undefined): value is string =>
    typeof value === 'string' && value.length > 0 && !value.startsWith('--');
  const checking = command === 'check' && rest.length === 3 && rest[0] === '--schema'
    && fileName(rest[1]) && fileName(rest[2]);
  const formatting = command === 'format' && rest.length === 1 && fileName(rest[0]);
  if (flag !== '--profile' || !profile || (!checking && !formatting))
    throw new CliFailure('CLI_USAGE', 'arguments');
  if (profile !== EXPRESSION_SOURCE_PROFILE) throw new CliFailure('CLI_PROFILE', 'arguments');
  return checking ? { command: 'check', schema: rest[1], source: rest[2] }
    : { command: 'format', source: rest[0] };
}

/** Same bounded descriptor/type check as syntax-cli.ts; never reads a FIFO. */
function readText(file: string, input: 'schema' | 'source'): string {
  let fd: number | undefined;
  try {
    fd = fs.openSync(path.resolve(file), fs.constants.O_RDONLY | (fs.constants.O_NONBLOCK ?? 0));
    if (!fs.fstatSync(fd).isFile()) throw new CliFailure('CLI_IO', input);
    const buffer = new Uint8Array(LIMIT + 1);
    let got = 0;
    while (got < buffer.length) {
      const count = fs.readSync(fd, buffer, got, buffer.length - got, got);
      if (count === 0) break;
      if (count < 0) throw new CliFailure('CLI_IO', input);
      got += count;
    }
    if (got > LIMIT) throw new CliFailure(input === 'source' ? 'SOURCE_BOUND' : 'INPUT_BOUND', input, 1);
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
  const schema = args.command === 'check' ? readText(args.schema!, 'schema') : undefined;
  const source = readText(args.source, 'source');
  if (args.command === 'format') {
    const formatted = formatExpressionSource(source);
    io.stdout.write(formatted);
  } else {
    const result = createExpressionSourceV1(schema!).check(source);
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
